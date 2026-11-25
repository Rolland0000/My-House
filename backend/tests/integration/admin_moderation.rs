#![cfg(feature = "integration")]
//! Admin moderation and account deletion: route guards, suspension, owner
//! revocation, the back-office tables, and the storage cleanup that precedes
//! every account or listing deletion, driven through the HTTP API.

use std::collections::HashSet;
use std::path::Path;

use axum::http::{Method, StatusCode};
use backend_my_house::shared::crypto::verify_access_token;
use backend_my_house::shared::rbac::Role;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::harness::app::{TestResponse, TEST_JWT_SECRET};
use crate::harness::fixtures::{self, sign_in, token_for, TestUser};
use crate::harness::{images, MultipartPart, TestApp, TestRequest};

const ADMIN_USERS_URI: &str = "/api/v1/admin/users";
const ADMIN_LISTINGS_URI: &str = "/api/v1/admin/listings";
const LISTINGS_URI: &str = "/api/v1/listings";
const UPLOAD_URI: &str = "/api/v1/media/upload";
const AVATAR_URI: &str = "/api/v1/users/me/avatar";
const REFRESH_URI: &str = "/api/v1/auth/refresh";
const PHOTO_SIDE: u32 = 16;
const OWNER_PHONE: &str = "+237690000003";
const PDF_BYTES: &[u8] = b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog >>\nendobj\n%%EOF\n";
const IDENTITY_DATA: &str =
    r#"{"full_name": "Test User", "id_type": "national_id", "id_number": "CM123456"}"#;
/// Axum's default body limit, which applies to every JSON route without its own.
const DEFAULT_BODY_LIMIT_BYTES: usize = 2 * 1024 * 1024;
/// Each round replays the deletion/upload race; the scheduler picks the winner.
const DELETION_RACE_ROUNDS: usize = 8;

fn error_code(response: &TestResponse) -> &str {
    response.body["error"]["code"].as_str().unwrap_or_default()
}

fn data_id(response: &TestResponse) -> Uuid {
    response.body["data"]["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("the response carries an id")
}

fn set_of(ids: &[Uuid]) -> HashSet<Uuid> {
    ids.iter().copied().collect()
}

/// Asserts the standard error envelope: `code`, `message` and the HTTP `status`.
fn assert_error_envelope(response: &TestResponse, status: StatusCode, code: &str) {
    assert_eq!(response.status, status);
    let error = &response.body["error"];
    assert_eq!(error["code"], code);
    assert!(error["message"].is_string(), "message in {error}");
    assert_eq!(error["status"], status.as_u16());
}

/// Files stored under the storage `prefix` (e.g. `avatars/{user_id}`).
fn file_count(app: &TestApp, prefix: &str) -> usize {
    fn count(dir: &Path) -> usize {
        if !dir.exists() {
            return 0;
        }
        std::fs::read_dir(dir)
            .expect("read a storage directory")
            .map(|entry| entry.expect("read a storage entry").path())
            .map(|path| if path.is_dir() { count(&path) } else { 1 })
            .sum()
    }
    count(&app.storage_root().join(prefix))
}

async fn set_account_active(
    app: &TestApp,
    admin_token: &str,
    user_id: Uuid,
    is_active: bool,
) -> TestResponse {
    app.send(
        TestRequest::new(Method::PATCH, &format!("{ADMIN_USERS_URI}/{user_id}"))
            .bearer(admin_token)
            .json(json!({ "is_active": is_active })),
    )
    .await
}

async fn revoke_owner(app: &TestApp, admin_token: &str, user_id: Uuid) -> TestResponse {
    app.send(
        TestRequest::post(&format!("{ADMIN_USERS_URI}/{user_id}/revoke-owner")).bearer(admin_token),
    )
    .await
}

async fn refresh(app: &TestApp, raw_refresh_token: &str) -> TestResponse {
    app.send(TestRequest::post(REFRESH_URI).refresh_cookie(raw_refresh_token))
        .await
}

async fn upload_photo(app: &TestApp, token: &str, listing_id: Uuid) -> TestResponse {
    app.send(TestRequest::post(UPLOAD_URI).bearer(token).multipart(vec![
        MultipartPart::text("listing_id", &listing_id.to_string()),
        MultipartPart::file(
            "file",
            "photo.jpg",
            "image/jpeg",
            images::jpeg(PHOTO_SIDE, PHOTO_SIDE),
        ),
    ]))
    .await
}

async fn upload_avatar(app: &TestApp, token: &str) -> TestResponse {
    app.send(
        TestRequest::post(AVATAR_URI)
            .bearer(token)
            .multipart(vec![MultipartPart::file(
                "file",
                "avatar.png",
                "image/png",
                images::png(PHOTO_SIDE, PHOTO_SIDE),
            )]),
    )
    .await
}

/// Ids of `uri`'s rows, checked against the page `total`.
async fn id_set(app: &TestApp, admin_token: &str, uri: &str) -> HashSet<Uuid> {
    let response = app.send(TestRequest::get(uri).bearer(admin_token)).await;
    assert_eq!(response.status, StatusCode::OK, "GET {uri}");
    let ids: Vec<Uuid> = response.body["data"]
        .as_array()
        .expect("the page is a list")
        .iter()
        .map(|item| {
            item["id"]
                .as_str()
                .and_then(|id| Uuid::parse_str(id).ok())
                .expect("each item carries an id")
        })
        .collect();
    assert_eq!(
        response.body["pagination"]["total"].as_u64(),
        Some(ids.len() as u64),
        "GET {uri} total"
    );
    set_of(&ids)
}

async fn count_rows(pool: &PgPool, sql: &'static str, id: Uuid) -> i64 {
    sqlx::query_scalar(sql)
        .bind(id)
        .fetch_one(pool)
        .await
        .expect("count the rows")
}

#[sqlx::test]
async fn every_admin_route_requires_an_admin(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker_token = token_for(&fixtures::seeker(&app.pool).await);
    let owner_token = token_for(&fixtures::owner(&app.pool).await);
    let id = Uuid::new_v4();
    let doc_id = Uuid::new_v4();
    // Valid ids and bodies, so the role check is what refuses the request.
    let routes = [
        (
            Method::GET,
            "/api/v1/admin/owner-requests".to_string(),
            None,
        ),
        (
            Method::GET,
            format!("/api/v1/admin/owner-requests/{id}"),
            None,
        ),
        (
            Method::GET,
            format!("/api/v1/admin/owner-requests/{id}/documents/{doc_id}"),
            None,
        ),
        (
            Method::PATCH,
            format!("/api/v1/admin/owner-requests/{id}"),
            Some(json!({ "status": "approved" })),
        ),
        (Method::GET, ADMIN_LISTINGS_URI.to_string(), None),
        (Method::DELETE, format!("{ADMIN_LISTINGS_URI}/{id}"), None),
        (Method::GET, ADMIN_USERS_URI.to_string(), None),
        (
            Method::PATCH,
            format!("{ADMIN_USERS_URI}/{id}"),
            Some(json!({ "is_active": false })),
        ),
        (Method::DELETE, format!("{ADMIN_USERS_URI}/{id}"), None),
        (
            Method::POST,
            format!("{ADMIN_USERS_URI}/{id}/revoke-owner"),
            None,
        ),
    ];

    for (method, uri, body) in routes {
        let request = |token: Option<&str>| {
            let mut request = TestRequest::new(method.clone(), &uri);
            if let Some(token) = token {
                request = request.bearer(token);
            }
            match &body {
                Some(body) => request.json(body.clone()),
                None => request,
            }
        };

        let anonymous = app.send(request(None)).await;
        assert_eq!(anonymous.status, StatusCode::UNAUTHORIZED, "{method} {uri}");
        for token in [&seeker_token, &owner_token] {
            let refused = app.send(request(Some(token))).await;
            assert_eq!(refused.status, StatusCode::FORBIDDEN, "{method} {uri}");
            assert_eq!(error_code(&refused), "FORBIDDEN", "{method} {uri}");
        }
    }
}

#[sqlx::test]
async fn admin_accounts_are_protected_from_moderation(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin = fixtures::admin(&app.pool).await;
    let admin_token = token_for(&admin);

    let suspended = set_account_active(&app, &admin_token, admin.id, false).await;
    let revoked = revoke_owner(&app, &admin_token, admin.id).await;
    let deleted = app
        .send(
            TestRequest::new(Method::DELETE, &format!("{ADMIN_USERS_URI}/{}", admin.id))
                .bearer(&admin_token),
        )
        .await;

    for response in [&suspended, &revoked, &deleted] {
        assert_eq!(response.status, StatusCode::FORBIDDEN);
        assert_eq!(error_code(response), "ADMIN_ACCOUNT_PROTECTED");
    }
    let (role, is_active): (String, bool) =
        sqlx::query_as("SELECT role::text, is_active FROM users WHERE id = $1")
            .bind(admin.id)
            .fetch_one(&app.pool)
            .await
            .expect("the admin row is still there");
    assert_eq!((role.as_str(), is_active), ("admin", true));
}

#[sqlx::test]
async fn suspension_refuses_the_refresh_cookie_and_reactivation_deletes_it(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin_token = token_for(&fixtures::admin(&app.pool).await);
    let seeker = fixtures::seeker(&app.pool).await;
    let (_, cookie) = sign_in(&app, &seeker).await;

    let suspended = set_account_active(&app, &admin_token, seeker.id, false).await;
    assert_eq!(suspended.status, StatusCode::OK);
    let refused = refresh(&app, &cookie).await;
    assert_eq!(refused.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error_code(&refused), "ACCOUNT_SUSPENDED");

    let reactivated = set_account_active(&app, &admin_token, seeker.id, true).await;
    assert_eq!(reactivated.status, StatusCode::OK);
    let stale = refresh(&app, &cookie).await;
    assert_eq!(stale.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error_code(&stale), "UNAUTHORIZED");
    let tokens = count_rows(
        &app.pool,
        "SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1",
        seeker.id,
    )
    .await;
    assert_eq!(tokens, 0);

    let (_, fresh_cookie) = sign_in(&app, &seeker).await;
    assert_eq!(refresh(&app, &fresh_cookie).await.status, StatusCode::OK);
}

#[sqlx::test]
async fn revoking_an_owner_demotes_hides_and_refreshes_as_seeker(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin_token = token_for(&fixtures::admin(&app.pool).await);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::published_listing(&app, &owner).await;
    let (_, cookie) = sign_in(&app, &owner).await;
    let public_uri = format!("{ADMIN_LISTINGS_URI}?visibility=public");
    let hidden_uri = format!("{ADMIN_LISTINGS_URI}?visibility=hidden");
    assert_eq!(
        id_set(&app, &admin_token, &public_uri).await,
        set_of(&[listing.id])
    );

    let revoked = revoke_owner(&app, &admin_token, owner.id).await;

    assert_eq!(revoked.status, StatusCode::OK);
    assert_eq!(revoked.body["data"]["role"], "seeker");
    let stored_role: String = sqlx::query_scalar("SELECT role::text FROM users WHERE id = $1")
        .bind(owner.id)
        .fetch_one(&app.pool)
        .await
        .expect("read the role back");
    assert_eq!(stored_role, "seeker");
    assert!(id_set(&app, &admin_token, &public_uri).await.is_empty());
    assert_eq!(
        id_set(&app, &admin_token, &hidden_uri).await,
        set_of(&[listing.id])
    );

    let refreshed = refresh(&app, &cookie).await;
    assert_eq!(refreshed.status, StatusCode::OK);
    let access_token = refreshed.body["data"]["access_token"]
        .as_str()
        .expect("the refresh returns an access token");
    let claims = verify_access_token(access_token, TEST_JWT_SECRET.as_bytes())
        .expect("the refreshed token verifies");
    assert_eq!(claims.role, Role::Seeker);

    let again = revoke_owner(&app, &admin_token, owner.id).await;
    assert_eq!(again.status, StatusCode::CONFLICT);
    assert_eq!(error_code(&again), "NOT_AN_OWNER");
}

#[sqlx::test]
async fn the_admin_listing_table_filters_by_status_visibility_and_owner(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin_token = token_for(&fixtures::admin(&app.pool).await);
    let owner = fixtures::owner(&app.pool).await;
    let suspended_owner = fixtures::owner(&app.pool).await;

    let draft = fixtures::draft_listing(&app.pool, &owner).await.id;
    let photoless = fixtures::photoless_listing(&app.pool, &owner).await.id;
    let public = fixtures::published_listing(&app, &owner).await.id;
    let unavailable = fixtures::published_listing(&app, &owner).await.id;
    let hidden = fixtures::published_listing(&app, &suspended_owner).await.id;
    let marked = app
        .send(
            TestRequest::new(
                Method::PATCH,
                &format!("{LISTINGS_URI}/{unavailable}/status"),
            )
            .bearer(&token_for(&owner))
            .json(json!({ "status": "unavailable" })),
        )
        .await;
    assert_eq!(marked.status, StatusCode::OK);
    let suspended = set_account_active(&app, &admin_token, suspended_owner.id, false).await;
    assert_eq!(suspended.status, StatusCode::OK);

    let expectations = [
        ("visibility=draft".to_string(), vec![draft]),
        ("visibility=no_photo".to_string(), vec![photoless]),
        ("visibility=public".to_string(), vec![public, unavailable]),
        ("visibility=hidden".to_string(), vec![hidden]),
        ("status=unavailable".to_string(), vec![unavailable]),
        (
            "status=available".to_string(),
            vec![draft, photoless, public, hidden],
        ),
        (
            format!("owner_id={}", owner.id),
            vec![draft, photoless, public, unavailable],
        ),
        (format!("owner_id={}", suspended_owner.id), vec![hidden]),
        (
            format!("owner_id={}&visibility=public&status=available", owner.id),
            vec![public],
        ),
    ];
    for (query, expected) in expectations {
        assert_eq!(
            id_set(&app, &admin_token, &format!("{ADMIN_LISTINGS_URI}?{query}")).await,
            set_of(&expected),
            "{query}"
        );
    }
}

#[sqlx::test]
async fn the_admin_user_table_filters_by_role_state_and_q(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin = fixtures::admin(&app.pool).await;
    let admin_token = token_for(&admin);
    let seeker = fixtures::seeker(&app.pool).await;
    let owner = fixtures::owner(&app.pool).await;
    let suspended = fixtures::suspended(&app.pool).await;
    sqlx::query("UPDATE users SET first_name = 'Hélène', last_name = 'Mbarga' WHERE id = $1")
        .bind(owner.id)
        .execute(&app.pool)
        .await
        .expect("rename the owner");
    let seeker_email_part = seeker
        .email
        .split('@')
        .next()
        .expect("the email has a local part")
        .to_uppercase();

    let expectations = [
        ("role=seeker".to_string(), vec![seeker.id, suspended.id]),
        ("role=owner".to_string(), vec![owner.id]),
        ("role=admin".to_string(), vec![admin.id]),
        (
            "state=active".to_string(),
            vec![admin.id, seeker.id, owner.id],
        ),
        ("state=suspended".to_string(), vec![suspended.id]),
        ("role=seeker&state=active".to_string(), vec![seeker.id]),
        ("q=helene".to_string(), vec![owner.id]),
        ("q=MBARG".to_string(), vec![owner.id]),
        (format!("q={seeker_email_part}"), vec![seeker.id]),
    ];
    for (query, expected) in expectations {
        assert_eq!(
            id_set(&app, &admin_token, &format!("{ADMIN_USERS_URI}?{query}")).await,
            set_of(&expected),
            "{query}"
        );
    }
}

#[sqlx::test]
async fn admin_deletion_of_a_listing_removes_its_rows_and_its_files(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin_token = token_for(&fixtures::admin(&app.pool).await);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::published_listing(&app, &owner).await.id;
    let owner_token = token_for(&owner);
    for _ in 0..2 {
        assert_eq!(
            upload_photo(&app, &owner_token, listing).await.status,
            StatusCode::CREATED
        );
    }
    let listing_prefix = format!("listings/{listing}");
    assert_eq!(file_count(&app, &listing_prefix), 3);

    let deleted = app
        .send(
            TestRequest::new(Method::DELETE, &format!("{ADMIN_LISTINGS_URI}/{listing}"))
                .bearer(&admin_token),
        )
        .await;

    assert_eq!(deleted.status, StatusCode::NO_CONTENT);
    let listings = count_rows(
        &app.pool,
        "SELECT COUNT(*) FROM listings WHERE id = $1",
        listing,
    )
    .await;
    let photos = count_rows(
        &app.pool,
        "SELECT COUNT(*) FROM listing_media WHERE listing_id = $1",
        listing,
    )
    .await;
    assert_eq!((listings, photos), (0, 0));
    assert_eq!(file_count(&app, &listing_prefix), 0);
}

/// Who deletes the account in [`account_deletion_removes_every_file_then_the_rows`].
enum Deleter {
    Admin,
    AccountHolder,
}

/// Builds an owner with an avatar, an approved request with a document and a
/// listing with photos, all through the API, then deletes the account.
async fn account_deletion_removes_every_file_then_the_rows(pool: PgPool, deleter: Deleter) {
    let app = TestApp::new(pool);
    let admin_token = token_for(&fixtures::admin(&app.pool).await);
    let seeker = fixtures::seeker(&app.pool).await;
    sign_in(&app, &seeker).await;
    assert_eq!(
        upload_avatar(&app, &token_for(&seeker)).await.status,
        StatusCode::OK
    );
    let submitted = app
        .send(
            TestRequest::post("/api/v1/owner-requests")
                .bearer(&token_for(&seeker))
                .multipart(vec![
                    MultipartPart::text("phone", OWNER_PHONE),
                    MultipartPart::text("identity_data", IDENTITY_DATA),
                    MultipartPart::file(
                        "documents",
                        "id-card.pdf",
                        "application/pdf",
                        PDF_BYTES.to_vec(),
                    ),
                ]),
        )
        .await;
    assert_eq!(submitted.status, StatusCode::CREATED);
    let request_id = data_id(&submitted);
    let approved = app
        .send(
            TestRequest::new(
                Method::PATCH,
                &format!("/api/v1/admin/owner-requests/{request_id}"),
            )
            .bearer(&admin_token)
            .json(json!({ "status": "approved" })),
        )
        .await;
    assert_eq!(approved.status, StatusCode::OK);
    let owner = TestUser {
        role: Role::Owner,
        ..seeker
    };
    let owner_token = token_for(&owner);
    let created = app
        .send(
            TestRequest::post(LISTINGS_URI)
                .bearer(&owner_token)
                .json(json!({
                    "title": "Studio meublé à Akwa",
                    "description": "Studio lumineux proche du marché, eau et électricité incluses.",
                    "type": "studio",
                    "price": 85000,
                    "city": "Douala",
                    "neighborhood": "Akwa",
                    "surface_m2": 30,
                    "rooms": 1
                })),
        )
        .await;
    assert_eq!(created.status, StatusCode::CREATED);
    let listing = data_id(&created);
    for _ in 0..2 {
        assert_eq!(
            upload_photo(&app, &owner_token, listing).await.status,
            StatusCode::CREATED
        );
    }
    let prefixes = [
        format!("avatars/{}", owner.id),
        format!("listings/{listing}"),
        format!("owner-requests/{request_id}"),
    ];
    let before: Vec<usize> = prefixes.iter().map(|p| file_count(&app, p)).collect();
    assert_eq!(before, vec![1, 2, 1]);

    match deleter {
        Deleter::Admin => {
            let deleted = app
                .send(
                    TestRequest::new(Method::DELETE, &format!("{ADMIN_USERS_URI}/{}", owner.id))
                        .bearer(&admin_token),
                )
                .await;
            assert_eq!(deleted.status, StatusCode::NO_CONTENT);
        }
        Deleter::AccountHolder => {
            let deleted = app
                .send(TestRequest::new(Method::DELETE, "/api/v1/users/me").bearer(&owner_token))
                .await;
            assert_eq!(deleted.status, StatusCode::OK);
        }
    }

    for prefix in &prefixes {
        assert_eq!(file_count(&app, prefix), 0, "{prefix}");
    }
    let rows: (i64, i64, i64, i64, i64) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM users WHERE id = $1),
                (SELECT COUNT(*) FROM listings WHERE owner_id = $1),
                (SELECT COUNT(*) FROM listing_media WHERE listing_id = $2),
                (SELECT COUNT(*) FROM owner_requests WHERE user_id = $1),
                (SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1)",
    )
    .bind(owner.id)
    .bind(listing)
    .fetch_one(&app.pool)
    .await
    .expect("count the account rows");
    assert_eq!(rows, (0, 0, 0, 0, 0));
}

#[sqlx::test]
async fn admin_account_deletion_removes_every_file_then_the_rows(pool: PgPool) {
    account_deletion_removes_every_file_then_the_rows(pool, Deleter::Admin).await;
}

#[sqlx::test]
async fn self_account_deletion_removes_every_file_then_the_rows(pool: PgPool) {
    account_deletion_removes_every_file_then_the_rows(pool, Deleter::AccountHolder).await;
}

#[sqlx::test]
async fn uploads_racing_account_deletion_leave_neither_a_row_nor_a_file(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin_token = token_for(&fixtures::admin(&app.pool).await);

    for round in 0..DELETION_RACE_ROUNDS {
        let owner = fixtures::owner(&app.pool).await;
        let listing = fixtures::draft_listing(&app.pool, &owner).await.id;
        let owner_token = token_for(&owner);
        // Caches `is_active` so both uploads get past authentication.
        assert_eq!(
            app.send(TestRequest::get("/api/v1/users/me").bearer(&owner_token))
                .await
                .status,
            StatusCode::OK
        );

        let (deleted, photo, avatar) = tokio::join!(
            app.send(
                TestRequest::new(Method::DELETE, &format!("{ADMIN_USERS_URI}/{}", owner.id))
                    .bearer(&admin_token),
            ),
            upload_photo(&app, &owner_token, listing),
            upload_avatar(&app, &owner_token),
        );

        assert_eq!(deleted.status, StatusCode::NO_CONTENT, "round {round}");
        assert!(
            matches!(
                photo.status,
                StatusCode::CREATED | StatusCode::UNAUTHORIZED | StatusCode::NOT_FOUND
            ),
            "round {round}: photo upload {}",
            photo.status
        );
        assert!(
            matches!(
                avatar.status,
                StatusCode::OK | StatusCode::UNAUTHORIZED | StatusCode::NOT_FOUND
            ),
            "round {round}: avatar upload {}",
            avatar.status
        );
        let rows: (i64, i64, i64) = sqlx::query_as(
            "SELECT (SELECT COUNT(*) FROM users WHERE id = $1),
                    (SELECT COUNT(*) FROM listings WHERE id = $2),
                    (SELECT COUNT(*) FROM listing_media WHERE listing_id = $2)",
        )
        .bind(owner.id)
        .bind(listing)
        .fetch_one(&app.pool)
        .await
        .expect("count the account rows");
        assert_eq!(rows, (0, 0, 0), "round {round}");
        assert_eq!(
            file_count(&app, &format!("listings/{listing}")),
            0,
            "round {round}"
        );
        assert_eq!(
            file_count(&app, &format!("avatars/{}", owner.id)),
            0,
            "round {round}"
        );
    }
}

#[sqlx::test]
async fn a_malformed_path_and_an_oversized_body_use_the_error_envelope(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin_token = token_for(&fixtures::admin(&app.pool).await);
    let seeker = fixtures::seeker(&app.pool).await;

    for (method, uri) in [
        (Method::GET, "/api/v1/admin/owner-requests/not-a-uuid"),
        (Method::DELETE, "/api/v1/admin/listings/not-a-uuid"),
        (Method::POST, "/api/v1/admin/users/not-a-uuid/revoke-owner"),
    ] {
        let malformed = app
            .send(TestRequest::new(method, uri).bearer(&admin_token))
            .await;
        assert_error_envelope(&malformed, StatusCode::BAD_REQUEST, "INVALID_PATH_PARAM");
    }

    let padding = "x".repeat(DEFAULT_BODY_LIMIT_BYTES);
    let oversized = app
        .send(
            TestRequest::new(Method::PATCH, &format!("{ADMIN_USERS_URI}/{}", seeker.id))
                .bearer(&admin_token)
                .json(json!({ "is_active": false, "padding": padding })),
        )
        .await;
    assert_error_envelope(
        &oversized,
        StatusCode::PAYLOAD_TOO_LARGE,
        "PAYLOAD_TOO_LARGE",
    );
    let is_active: bool = sqlx::query_scalar("SELECT is_active FROM users WHERE id = $1")
        .bind(seeker.id)
        .fetch_one(&app.pool)
        .await
        .expect("read the account state back");
    assert!(is_active);
}
