#![cfg(feature = "integration")]
//! Proves each harness helper produces what the flow tests rely on.

use std::time::Duration;

use axum::http::StatusCode;
use backend_my_house::infra::cache::AppCacheProvider;
use backend_my_house::shared::session_cookie::REFRESH_TOKEN_COOKIE;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::harness::app::TEST_PUBLIC_MEDIA_BASE_URL;
use crate::harness::cache::TestCache;
use crate::harness::fixtures::{self, token_for, ADMIN_PASSWORD};
use crate::harness::{images, MultipartPart, TestApp, TestAppOptions, TestRequest};

const OTP_CODE: &str = "482913";
const UPLOAD_SIDE: u32 = 16;

fn stored_file(app: &TestApp, url: &str) -> Vec<u8> {
    let key = url
        .strip_prefix(&format!("{TEST_PUBLIC_MEDIA_BASE_URL}/"))
        .expect("the URL points into the test storage");
    std::fs::read(app.storage_root().join(key)).expect("the uploaded file is on disk")
}

#[sqlx::test]
async fn user_fixtures_satisfy_the_schema_and_carry_their_state(pool: PgPool) {
    let seeker = fixtures::seeker(&pool).await;
    let owner = fixtures::owner(&pool).await;
    let suspended = fixtures::suspended(&pool).await;
    let admin = fixtures::admin(&pool).await;

    let approved: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM owner_requests WHERE user_id = $1 AND status = 'approved'",
    )
    .bind(owner.id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(approved, 1);

    let active: Vec<(Uuid, bool)> =
        sqlx::query_as("SELECT id, is_active FROM users WHERE id = ANY($1)")
            .bind(vec![seeker.id, suspended.id, admin.id])
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(active.len(), 3);
    for (id, is_active) in active {
        assert_eq!(is_active, id != suspended.id);
    }
}

#[sqlx::test]
async fn token_for_is_accepted_and_suspension_is_enforced(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let suspended = fixtures::suspended(&app.pool).await;

    let me = app
        .send(TestRequest::get("/api/v1/users/me").bearer(&token_for(&seeker)))
        .await;
    assert_eq!(me.status, StatusCode::OK);

    let refused = app
        .send(TestRequest::get("/api/v1/users/me").bearer(&token_for(&suspended)))
        .await;
    assert_eq!(refused.status, StatusCode::UNAUTHORIZED);
    assert_eq!(refused.body["error"]["code"], "ACCOUNT_SUSPENDED");
}

#[sqlx::test]
async fn the_real_is_active_cache_can_back_the_app(pool: PgPool) {
    let options = TestAppOptions {
        real_is_active_cache: true,
        ..TestAppOptions::default()
    };
    let app = TestApp::with_options(pool, options);
    let seeker = fixtures::seeker(&app.pool).await;

    let me = app
        .send(TestRequest::get("/api/v1/users/me").bearer(&token_for(&seeker)))
        .await;
    assert_eq!(me.status, StatusCode::OK);
}

#[sqlx::test]
async fn seeded_otp_opens_a_session_whose_cookie_refreshes(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    fixtures::seed_otp(&app, &seeker.email.to_uppercase(), OTP_CODE).await;

    let verified = app
        .send(
            TestRequest::post("/api/v1/auth/otp/verify")
                .json(json!({ "email": seeker.email, "code": OTP_CODE })),
        )
        .await;
    assert_eq!(verified.status, StatusCode::OK);
    assert_eq!(verified.body["data"]["is_new_user"], false);
    assert!(verified.body["data"]["access_token"].is_string());
    let refresh_token = verified
        .set_cookie(REFRESH_TOKEN_COOKIE)
        .expect("the session sets a refresh cookie");

    let refreshed = app
        .send(TestRequest::post("/api/v1/auth/refresh").refresh_cookie(&refresh_token))
        .await;
    assert_eq!(refreshed.status, StatusCode::OK);
    assert!(refreshed.body["data"]["access_token"].is_string());
}

#[sqlx::test]
async fn seeded_otp_for_an_unknown_email_asks_for_registration(pool: PgPool) {
    let app = TestApp::new(pool);
    let email = "newcomer@test.myhouse.app";
    fixtures::seed_otp(&app, email, OTP_CODE).await;

    let verified = app
        .send(
            TestRequest::post("/api/v1/auth/otp/verify")
                .json(json!({ "email": email, "code": OTP_CODE })),
        )
        .await;
    assert_eq!(verified.status, StatusCode::OK);
    assert_eq!(verified.body["data"]["is_new_user"], true);
}

#[sqlx::test]
async fn the_admin_fixture_signs_in_with_its_password(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin = fixtures::admin(&app.pool).await;

    let login = app
        .send(
            TestRequest::post("/api/v1/admin/auth/login")
                .json(json!({ "email": admin.email, "password": ADMIN_PASSWORD })),
        )
        .await;
    assert_eq!(login.status, StatusCode::OK);
    assert!(login.set_cookie(REFRESH_TOKEN_COOKIE).is_some());
}

#[sqlx::test]
async fn listing_fixtures_match_their_publication_state(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let published = fixtures::published_listing(&app, &owner).await;
    let draft = fixtures::draft_listing(&app.pool, &owner).await;

    let (_, cover_key) = published.cover.as_ref().expect("published has a cover");
    assert!(app.storage_root().join(cover_key).is_file());
    assert!(draft.cover.is_none());

    let public = app
        .send(TestRequest::get(&format!(
            "/api/v1/listings/{}",
            published.id
        )))
        .await;
    assert_eq!(public.status, StatusCode::OK);
    let hidden = app
        .send(TestRequest::get(&format!("/api/v1/listings/{}", draft.id)))
        .await;
    assert_eq!(hidden.status, StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn generated_images_upload_through_multipart(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let draft = fixtures::draft_listing(&app.pool, &owner).await;
    let token = token_for(&owner);

    let uploads = [
        (
            "photo.png",
            "image/png",
            images::png(UPLOAD_SIDE, UPLOAD_SIDE),
        ),
        (
            "photo.jpg",
            "image/jpeg",
            images::jpeg_with_exif(UPLOAD_SIDE, UPLOAD_SIDE, 6),
        ),
    ];
    for (filename, content_type, bytes) in uploads {
        let response = app
            .send(
                TestRequest::post("/api/v1/media/upload")
                    .bearer(&token)
                    .multipart(vec![
                        MultipartPart::text("listing_id", &draft.id.to_string()),
                        MultipartPart::file("file", filename, content_type, bytes),
                    ]),
            )
            .await;
        assert_eq!(response.status, StatusCode::CREATED, "{filename}");
        let url = response.body["data"]["url"].as_str().expect("media url");
        image::load_from_memory(&stored_file(&app, url)).expect("the stored file decodes");
    }
}

#[test]
fn jpeg_with_exif_decodes_and_carries_the_segment() {
    let bytes = images::jpeg_with_exif(UPLOAD_SIDE, UPLOAD_SIDE, 6);
    assert!(bytes.windows(4).any(|window| window == b"Exif"));
    let decoded = image::load_from_memory(&bytes).expect("decodable JPEG");
    assert_eq!(
        (decoded.width(), decoded.height()),
        (UPLOAD_SIDE, UPLOAD_SIDE)
    );
}

#[tokio::test]
async fn test_cache_honours_its_ttl() {
    let forever = TestCache::<u8, u8>::new(None);
    let short = TestCache::<u8, u8>::new(Some(Duration::from_millis(20)));
    forever.insert(1, 1).await;
    short.insert(1, 1).await;

    tokio::time::sleep(Duration::from_millis(40)).await;

    assert_eq!(forever.get(&1).await, Some(1));
    assert_eq!(short.get(&1).await, None);
    assert_eq!(short.get_or_insert(1, 2).await, 2);
}
