#![cfg(feature = "integration")]
//! Owner onboarding: submission, status, admin review and document access,
//! driven through the HTTP API.

use std::path::Path;

use axum::http::{header, Method, StatusCode};
use backend_my_house::shared::crypto::verify_access_token;
use backend_my_house::shared::rbac::Role;
use backend_my_house::shared::session_cookie::REFRESH_TOKEN_COOKIE;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::harness::app::{TestResponse, TEST_JWT_SECRET};
use crate::harness::fixtures::{self, sign_in, token_for};
use crate::harness::{images, MultipartPart, TestApp, TestRequest};

const SUBMIT_URI: &str = "/api/v1/owner-requests";
const STATUS_URI: &str = "/api/v1/users/me/owner-request";
const STORAGE_PREFIX: &str = "owner-requests";
const APPLICANT_PHONE: &str = "+237690000001";
const IDENTITY_DATA: &str =
    r#"{"full_name": "Ada Lovelace", "id_type": "national_id", "id_number": "CM987654"}"#;
const REJECTION_NOTE: &str = "The ID photo is blurry.";
const FRONT_SIDE: (u32, u32) = (32, 32);
const BACK_SIDE: (u32, u32) = (48, 32);

fn error_code(response: &TestResponse) -> &str {
    response.body["error"]["code"].as_str().unwrap_or_default()
}

fn pdf_bytes() -> Vec<u8> {
    b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog >>\nendobj\n%%EOF\n".to_vec()
}

fn document(filename: &str, content_type: &str, data: Vec<u8>) -> MultipartPart {
    MultipartPart::file("documents", filename, content_type, data)
}

fn pdf_document() -> MultipartPart {
    document("id-card.pdf", "application/pdf", pdf_bytes())
}

fn front_image() -> Vec<u8> {
    images::jpeg(FRONT_SIDE.0, FRONT_SIDE.1)
}

fn back_image() -> Vec<u8> {
    images::jpeg(BACK_SIDE.0, BACK_SIDE.1)
}

fn image_pair() -> Vec<MultipartPart> {
    vec![
        document("front.jpg", "image/jpeg", front_image()),
        document("back.jpg", "image/jpeg", back_image()),
    ]
}

async fn submit(app: &TestApp, access_token: &str, documents: Vec<MultipartPart>) -> TestResponse {
    let mut parts = vec![
        MultipartPart::text("phone", APPLICANT_PHONE),
        MultipartPart::text("identity_data", IDENTITY_DATA),
    ];
    parts.extend(documents);
    app.send(
        TestRequest::post(SUBMIT_URI)
            .bearer(access_token)
            .multipart(parts),
    )
    .await
}

/// Submits a PDF and returns the created request's id.
async fn submit_pdf(app: &TestApp, access_token: &str) -> Uuid {
    let submitted = submit(app, access_token, vec![pdf_document()]).await;
    assert_eq!(submitted.status, StatusCode::CREATED);
    response_id(&submitted)
}

fn response_id(response: &TestResponse) -> Uuid {
    response.body["data"]["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("the response carries the request id")
}

async fn review(app: &TestApp, admin_token: &str, request_id: Uuid, body: Value) -> TestResponse {
    app.send(
        TestRequest::new(
            Method::PATCH,
            &format!("/api/v1/admin/owner-requests/{request_id}"),
        )
        .bearer(admin_token)
        .json(body),
    )
    .await
}

async fn status_of(app: &TestApp, access_token: &str) -> Value {
    let response = app
        .send(TestRequest::get(STATUS_URI).bearer(access_token))
        .await;
    assert_eq!(response.status, StatusCode::OK);
    response.body["data"].clone()
}

/// Statuses of the user's requests, sorted.
async fn request_statuses(pool: &PgPool, user_id: Uuid) -> Vec<String> {
    sqlx::query_scalar(
        "SELECT status::text FROM owner_requests WHERE user_id = $1 ORDER BY status::text",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .expect("read the owner requests")
}

async fn user_role(pool: &PgPool, user_id: Uuid) -> String {
    sqlx::query_scalar("SELECT role::text FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .expect("read the user role")
}

/// Every stored file as `(storage key, bytes)`, sorted by key.
fn stored_files(app: &TestApp) -> Vec<(String, Vec<u8>)> {
    fn walk(root: &Path, dir: &Path, files: &mut Vec<(String, Vec<u8>)>) {
        for entry in std::fs::read_dir(dir).expect("read the storage directory") {
            let path = entry.expect("read a storage entry").path();
            if path.is_dir() {
                walk(root, &path, files);
            } else {
                let key = path
                    .strip_prefix(root)
                    .expect("the file is under the storage root")
                    .components()
                    .map(|part| part.as_os_str().to_string_lossy())
                    .collect::<Vec<_>>()
                    .join("/");
                files.push((key, std::fs::read(&path).expect("read a stored file")));
            }
        }
    }
    let mut files = Vec::new();
    walk(app.storage_root(), app.storage_root(), &mut files);
    files.sort();
    files
}

/// Asserts the storage holds exactly `expected` (in any order), each under the request's folder.
fn assert_stored_documents(app: &TestApp, request_id: Uuid, expected: &[Vec<u8>]) {
    let files = stored_files(app);
    let folder = format!("{STORAGE_PREFIX}/{request_id}/");
    for (key, _) in &files {
        assert!(key.starts_with(&folder), "{key} is outside {folder}");
    }
    let mut stored: Vec<&Vec<u8>> = files.iter().map(|(_, bytes)| bytes).collect();
    let mut expected: Vec<&Vec<u8>> = expected.iter().collect();
    stored.sort();
    expected.sort();
    assert_eq!(stored, expected);
}

/// `(doc_id, side)` of each document, read through the admin detail endpoint.
async fn admin_documents(
    app: &TestApp,
    admin_token: &str,
    request_id: Uuid,
) -> Vec<(Uuid, Option<String>)> {
    let detail = app
        .send(
            TestRequest::get(&format!("/api/v1/admin/owner-requests/{request_id}"))
                .bearer(admin_token),
        )
        .await;
    assert_eq!(detail.status, StatusCode::OK);
    detail.body["data"]["documents"]
        .as_array()
        .expect("the detail lists the documents")
        .iter()
        .map(|document| {
            let doc_id = document["doc_id"]
                .as_str()
                .and_then(|id| Uuid::parse_str(id).ok())
                .expect("each document has an id");
            (doc_id, document["side"].as_str().map(str::to_string))
        })
        .collect()
}

#[sqlx::test]
async fn a_pdf_submission_creates_one_pending_request_and_stores_the_pdf(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;

    let submitted = submit(&app, &token_for(&seeker), vec![pdf_document()]).await;

    assert_eq!(submitted.status, StatusCode::CREATED);
    assert_eq!(submitted.body["data"]["status"], "pending");
    assert_eq!(
        request_statuses(&app.pool, seeker.id).await,
        vec!["pending"]
    );
    assert_stored_documents(&app, response_id(&submitted), &[pdf_bytes()]);
}

#[sqlx::test]
async fn a_two_image_submission_stores_both_sides_as_front_then_back(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let admin = fixtures::admin(&app.pool).await;

    let submitted = submit(&app, &token_for(&seeker), image_pair()).await;

    assert_eq!(submitted.status, StatusCode::CREATED);
    assert_eq!(submitted.body["data"]["status"], "pending");
    assert_eq!(
        request_statuses(&app.pool, seeker.id).await,
        vec!["pending"]
    );
    let request_id = response_id(&submitted);
    assert_stored_documents(&app, request_id, &[front_image(), back_image()]);
    let sides: Vec<Option<String>> = admin_documents(&app, &token_for(&admin), request_id)
        .await
        .into_iter()
        .map(|(_, side)| side)
        .collect();
    assert_eq!(
        sides,
        vec![Some("front".to_string()), Some("back".to_string())]
    );
}

#[sqlx::test]
async fn an_invalid_document_is_refused_and_leaves_no_row_and_no_file(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let cases = [
        (
            "text sent as a PDF",
            vec![document(
                "id-card.pdf",
                "application/pdf",
                b"not a pdf at all".to_vec(),
            )],
        ),
        (
            "a single image",
            vec![document("front.jpg", "image/jpeg", front_image())],
        ),
        // The valid front must not be stored while the back is rejected.
        (
            "a valid front with a back that is not an image",
            vec![
                document("front.jpg", "image/jpeg", front_image()),
                document("back.jpg", "image/jpeg", b"not an image".to_vec()),
            ],
        ),
    ];

    for (case, documents) in cases {
        let submitted = submit(&app, &token_for(&seeker), documents).await;

        assert_eq!(submitted.status, StatusCode::UNPROCESSABLE_ENTITY, "{case}");
        assert_eq!(error_code(&submitted), "INVALID_DOCUMENT", "{case}");
        assert!(
            request_statuses(&app.pool, seeker.id).await.is_empty(),
            "{case}"
        );
        assert!(stored_files(&app).is_empty(), "{case}");
    }
}

#[sqlx::test]
async fn a_second_submission_while_one_is_pending_is_refused(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let access_token = token_for(&seeker);
    let request_id = submit_pdf(&app, &access_token).await;

    let second = submit(&app, &access_token, image_pair()).await;

    assert_eq!(second.status, StatusCode::CONFLICT);
    assert_eq!(error_code(&second), "OWNER_REQUEST_ALREADY_PENDING");
    assert_eq!(
        request_statuses(&app.pool, seeker.id).await,
        vec!["pending"]
    );
    assert_stored_documents(&app, request_id, &[pdf_bytes()]);
}

#[sqlx::test]
async fn the_status_endpoint_returns_null_then_the_pending_request(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let access_token = token_for(&seeker);

    assert!(status_of(&app, &access_token).await.is_null());

    let request_id = submit_pdf(&app, &access_token).await;
    let current = status_of(&app, &access_token).await;

    assert_eq!(current["id"], request_id.to_string());
    assert_eq!(current["status"], "pending");
    assert!(current["reviewed_at"].is_null());
    assert!(current["admin_note"].is_null());
}

#[sqlx::test]
async fn approval_makes_the_user_an_owner_and_the_next_refresh_carries_the_role(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let admin = fixtures::admin(&app.pool).await;
    let (access_token, raw_refresh_token) = sign_in(&app, &seeker).await;
    let request_id = submit_pdf(&app, &access_token).await;

    let approved = review(
        &app,
        &token_for(&admin),
        request_id,
        json!({ "status": "approved" }),
    )
    .await;

    assert_eq!(approved.status, StatusCode::OK);
    assert_eq!(approved.body["data"]["status"], "approved");
    assert_eq!(user_role(&app.pool, seeker.id).await, "owner");
    assert_eq!(status_of(&app, &access_token).await["status"], "approved");

    let refreshed = app
        .send(TestRequest::post("/api/v1/auth/refresh").refresh_cookie(&raw_refresh_token))
        .await;
    assert_eq!(refreshed.status, StatusCode::OK);
    assert!(refreshed.set_cookie(REFRESH_TOKEN_COOKIE).is_some());
    let new_access_token = refreshed.body["data"]["access_token"]
        .as_str()
        .expect("the refresh issues an access token");
    let claims = verify_access_token(new_access_token, TEST_JWT_SECRET.as_bytes())
        .expect("the refreshed access token is valid");
    assert_eq!(claims.user_id, seeker.id);
    assert_eq!(claims.role, Role::Owner);
}

#[sqlx::test]
async fn rejection_keeps_the_seeker_role_and_allows_a_new_submission(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let admin = fixtures::admin(&app.pool).await;
    let access_token = token_for(&seeker);
    let request_id = submit_pdf(&app, &access_token).await;

    let rejected = review(
        &app,
        &token_for(&admin),
        request_id,
        json!({ "status": "rejected", "admin_note": REJECTION_NOTE }),
    )
    .await;

    assert_eq!(rejected.status, StatusCode::OK);
    assert_eq!(user_role(&app.pool, seeker.id).await, "seeker");
    let current = status_of(&app, &access_token).await;
    assert_eq!(current["status"], "rejected");
    assert_eq!(current["admin_note"], REJECTION_NOTE);
    assert!(current["reviewed_at"].is_string());

    let resubmitted = submit(&app, &access_token, image_pair()).await;

    assert_eq!(resubmitted.status, StatusCode::CREATED);
    assert_eq!(
        request_statuses(&app.pool, seeker.id).await,
        vec!["pending", "rejected"]
    );
    assert_eq!(
        status_of(&app, &access_token).await["id"],
        response_id(&resubmitted).to_string()
    );
}

#[sqlx::test]
async fn reviewing_an_already_reviewed_request_is_refused_and_changes_nothing(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let admin = fixtures::admin(&app.pool).await;
    let admin_token = token_for(&admin);
    let request_id = submit_pdf(&app, &token_for(&seeker)).await;
    let approve = json!({ "status": "approved" });
    assert_eq!(
        review(&app, &admin_token, request_id, approve.clone())
            .await
            .status,
        StatusCode::OK
    );

    let reject = json!({ "status": "rejected", "admin_note": REJECTION_NOTE });
    for decision in [approve, reject] {
        let second = review(&app, &admin_token, request_id, decision.clone()).await;

        assert_eq!(second.status, StatusCode::CONFLICT, "{decision}");
        assert_eq!(
            error_code(&second),
            "OWNER_REQUEST_ALREADY_REVIEWED",
            "{decision}"
        );
    }
    let (status, admin_note): (String, Option<String>) =
        sqlx::query_as("SELECT status::text, admin_note FROM owner_requests WHERE id = $1")
            .bind(request_id)
            .fetch_one(&app.pool)
            .await
            .expect("read the request back");
    assert_eq!(status, "approved");
    assert_eq!(admin_note, None);
    assert_eq!(user_role(&app.pool, seeker.id).await, "owner");
}

#[sqlx::test]
async fn only_the_admin_can_download_a_document(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let owner = fixtures::owner(&app.pool).await;
    let admin = fixtures::admin(&app.pool).await;
    let admin_token = token_for(&admin);
    let submitted = submit(&app, &token_for(&seeker), image_pair()).await;
    assert_eq!(submitted.status, StatusCode::CREATED);
    let request_id = response_id(&submitted);
    let documents = admin_documents(&app, &admin_token, request_id).await;
    assert_eq!(documents.len(), 2);

    for (doc_id, side) in documents {
        let expected_bytes = match side.as_deref() {
            Some("front") => front_image(),
            Some("back") => back_image(),
            other => panic!("unexpected document side {other:?}"),
        };
        let uri = format!("/api/v1/admin/owner-requests/{request_id}/documents/{doc_id}");

        let downloaded = app.send(TestRequest::get(&uri).bearer(&admin_token)).await;
        assert_eq!(downloaded.status, StatusCode::OK, "{side:?}");
        assert_eq!(
            downloaded.headers[header::CONTENT_TYPE],
            "image/jpeg",
            "{side:?}"
        );
        assert_eq!(downloaded.raw_body, expected_bytes, "{side:?}");

        for (caller, token) in [("seeker", token_for(&seeker)), ("owner", token_for(&owner))] {
            let refused = app.send(TestRequest::get(&uri).bearer(&token)).await;
            assert_eq!(refused.status, StatusCode::FORBIDDEN, "{caller} {side:?}");
            assert_eq!(error_code(&refused), "FORBIDDEN", "{caller} {side:?}");
            assert!(
                refused.headers.get(header::CONTENT_DISPOSITION).is_none(),
                "{caller} {side:?}"
            );
        }
    }
}
