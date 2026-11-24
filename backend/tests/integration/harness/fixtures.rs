#![cfg(feature = "integration")]

use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

use axum::http::StatusCode;
use backend_my_house::shared::crypto::{hash_otp_code, hash_password, issue_access_token};
use backend_my_house::shared::rbac::Role;
use backend_my_house::shared::session_cookie::REFRESH_TOKEN_COOKIE;
use backend_my_house::shared::storage_key::listing_media_key;
use backend_my_house::shared::token_decoder::TokenClaims;
use backend_my_house::shared::types::{otp_key, AuthChallenge, ListingId, PendingOtp};
use backend_my_house::shared::validation::normalize_email;
use bytes::Bytes;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use super::app::{TestApp, TestRequest, TEST_ACCESS_TTL_SECONDS, TEST_JWT_SECRET};
use super::images;

pub const ADMIN_PASSWORD: &str = "integration-admin-password";
const SIGN_IN_OTP_CODE: &str = "482913";
const COVER_PHOTO_SIDE: u32 = 32;

/// Argon2 is slow by design: hashed once, shared by every admin fixture in the binary.
static ADMIN_PASSWORD_HASH: LazyLock<String> =
    LazyLock::new(|| hash_password(ADMIN_PASSWORD).expect("hash the admin password"));

#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
}

#[derive(Debug, Clone)]
pub struct TestListing {
    pub id: Uuid,
    /// `(media id, storage key)` of the cover photo; `None` for a draft.
    pub cover: Option<(Uuid, String)>,
}

fn unique_email(prefix: &str) -> String {
    format!("{prefix}-{}@test.myhouse.app", Uuid::new_v4().simple())
}

async fn insert_user(pool: &PgPool, prefix: &str, role: &str, is_active: bool) -> Uuid {
    sqlx::query_scalar(
        "INSERT INTO users (email, role, first_name, last_name, phone, is_active)
         VALUES ($1, $2::user_role, 'Test', 'User', '+237690000000', $3)
         RETURNING id",
    )
    .bind(unique_email(prefix))
    .bind(role)
    .bind(is_active)
    .fetch_one(pool)
    .await
    .expect("insert the user fixture")
}

async fn load_user(pool: &PgPool, id: Uuid, role: Role) -> TestUser {
    let email = sqlx::query_scalar("SELECT email FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .expect("read the user fixture back");
    TestUser { id, email, role }
}

pub async fn seeker(pool: &PgPool) -> TestUser {
    let id = insert_user(pool, "seeker", "seeker", true).await;
    load_user(pool, id, Role::Seeker).await
}

/// A seeker whose account an admin suspended.
pub async fn suspended(pool: &PgPool) -> TestUser {
    let id = insert_user(pool, "suspended", "seeker", false).await;
    load_user(pool, id, Role::Seeker).await
}

/// An owner backed by the approved owner request that granted the role.
pub async fn owner(pool: &PgPool) -> TestUser {
    let id = insert_user(pool, "owner", "owner", true).await;
    sqlx::query(
        "INSERT INTO owner_requests (user_id, phone, identity_data, status, reviewed_at)
         VALUES ($1, '+237690000000',
                 '{\"full_name\": \"Test User\", \"id_type\": \"national_id\", \"id_number\": \"CM123456\"}',
                 'approved', NOW())",
    )
    .bind(id)
    .execute(pool)
    .await
    .expect("insert the approved owner request");
    load_user(pool, id, Role::Owner).await
}

/// The admin, who signs in with [`ADMIN_PASSWORD`].
pub async fn admin(pool: &PgPool) -> TestUser {
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (email, role, password_hash) VALUES ($1, 'admin', $2) RETURNING id",
    )
    .bind(unique_email("admin"))
    .bind(ADMIN_PASSWORD_HASH.as_str())
    .fetch_one(pool)
    .await
    .expect("insert the admin fixture");
    load_user(pool, id, Role::Admin).await
}

/// An access token for `user`, signed with the test app's JWT secret.
pub fn token_for(user: &TestUser) -> String {
    issue_access_token(
        TokenClaims {
            user_id: user.id,
            role: user.role,
        },
        TEST_JWT_SECRET.as_bytes(),
        TEST_ACCESS_TTL_SECONDS,
    )
    .expect("sign the access token")
}

/// Opens a session through OTP verify; returns `(access token, raw refresh token)`.
pub async fn sign_in(app: &TestApp, user: &TestUser) -> (String, String) {
    seed_otp(app, &user.email, SIGN_IN_OTP_CODE).await;
    let verified = app
        .send(
            TestRequest::post("/api/v1/auth/otp/verify")
                .json(json!({ "email": user.email, "code": SIGN_IN_OTP_CODE })),
        )
        .await;
    assert_eq!(verified.status, StatusCode::OK);
    let access_token = verified.body["data"]["access_token"]
        .as_str()
        .expect("the session carries an access token")
        .to_string();
    let raw_refresh_token = verified
        .set_cookie(REFRESH_TOKEN_COOKIE)
        .expect("the session sets the refresh cookie");
    (access_token, raw_refresh_token)
}

/// Stores the OTP `code` for `email` as `POST /auth/otp/request` would, without sending it.
pub async fn seed_otp(app: &TestApp, email: &str, code: &str) {
    let email = normalize_email(email).expect("seed_otp needs a valid email");
    let is_new =
        !sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)")
            .bind(&email)
            .fetch_one(&app.pool)
            .await
            .expect("look the email up");

    let cache = app.state.cache();
    let attempt_window = cache
        .otp_attempts()
        .get_or_insert(email.clone(), Arc::default())
        .await;
    let now = Instant::now();
    let attempts_at_issue = attempt_window.attempts();
    attempt_window.record_code_issued(now);
    cache
        .auth_challenge()
        .insert(
            otp_key(&email),
            AuthChallenge::PendingOtp(PendingOtp {
                code_hash: hash_otp_code(code),
                is_new,
                expires_at: now + Duration::from_secs(app.config().auth.otp_ttl_seconds),
                attempt_window,
                attempts_at_issue,
            }),
        )
        .await;
}

async fn insert_listing(pool: &PgPool, owner: &TestUser, published: bool) -> Uuid {
    sqlx::query_scalar(
        "INSERT INTO listings
            (owner_id, title, description, type, city, neighborhood, price, currency,
             surface_m2, rooms, published_at)
         VALUES ($1, 'Appartement test', 'Description du logement de test.', 'apartment',
                 'Douala', 'Bonapriso', 150000, 'XAF', 80, 3,
                 CASE WHEN $2 THEN NOW() END)
         RETURNING id",
    )
    .bind(owner.id)
    .bind(published)
    .fetch_one(pool)
    .await
    .expect("insert the listing fixture")
}

/// A published listing with one cover photo, whose file exists in the app's storage.
pub async fn published_listing(app: &TestApp, owner: &TestUser) -> TestListing {
    let id = insert_listing(&app.pool, owner, true).await;
    let storage_key = listing_media_key(ListingId::new(id), "jpg");
    let url = app
        .state
        .storage()
        .upload(
            &storage_key,
            Bytes::from(images::jpeg(COVER_PHOTO_SIDE, COVER_PHOTO_SIDE)),
            "image/jpeg",
        )
        .await
        .expect("store the cover photo");
    let media_id = sqlx::query_scalar(
        "INSERT INTO listing_media (listing_id, storage_key, url, is_cover, position)
         VALUES ($1, $2, $3, TRUE, 0)
         RETURNING id",
    )
    .bind(id)
    .bind(&storage_key)
    .bind(url)
    .fetch_one(&app.pool)
    .await
    .expect("insert the cover photo row");

    TestListing {
        id,
        cover: Some((media_id, storage_key)),
    }
}

/// A draft listing (never published) with no photo.
pub async fn draft_listing(pool: &PgPool, owner: &TestUser) -> TestListing {
    let id = insert_listing(pool, owner, false).await;
    TestListing { id, cover: None }
}

/// A published listing with no photo, so not publicly visible.
pub async fn photoless_listing(pool: &PgPool, owner: &TestUser) -> TestListing {
    let id = insert_listing(pool, owner, true).await;
    TestListing { id, cover: None }
}
