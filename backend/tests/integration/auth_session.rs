#![cfg(feature = "integration")]
//! Sign-in, refresh and sign-out chain, driven through the HTTP API.

use std::time::Duration;

use axum::http::{header, StatusCode};
use backend_my_house::modules::auth::model::NewAccount;
use backend_my_house::modules::auth::repository as auth_repository;
use backend_my_house::shared::crypto::hash_refresh_token;
use backend_my_house::shared::errors::AppError;
use backend_my_house::shared::session_cookie::REFRESH_TOKEN_COOKIE;
use backend_my_house::shared::types::{otp_key, RefreshTokenId};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::harness::app::TestResponse;
use crate::harness::fixtures::{self, sign_in, token_for, ADMIN_PASSWORD};
use crate::harness::{TestApp, TestAppOptions, TestRequest};

const OTP_CODE: &str = "482913";
const WRONG_OTP_CODE: &str = "000000";
const WRONG_ADMIN_PASSWORD: &str = "not-the-admin-password";
/// Wrong passwords after which the admin account locks.
const ADMIN_LOGIN_MAX_ATTEMPTS: usize = 5;
/// Expired refresh tokens older than this are purged on rotation.
const REFRESH_TOKEN_RETENTION_DAYS: i32 = 30;
const NEW_ACCOUNT_REFRESH_TTL_DAYS: i32 = 30;
/// Width of the `refresh_tokens.token_hash` column.
const TOKEN_HASH_MAX_LENGTH: usize = 255;
/// TTL of the production `is_active` cache.
const IS_ACTIVE_CACHE_TTL: Duration = Duration::from_secs(8);
const NEW_USER_EMAIL: &str = "ada@test.myhouse.app";

fn error_code(response: &TestResponse) -> &str {
    response.body["error"]["code"].as_str().unwrap_or_default()
}

fn retry_after_seconds(response: &TestResponse) -> u64 {
    response.headers[header::RETRY_AFTER]
        .to_str()
        .expect("Retry-After is ASCII")
        .parse()
        .expect("Retry-After is a number of seconds")
}

fn refresh_cookie(response: &TestResponse) -> String {
    response
        .set_cookie(REFRESH_TOKEN_COOKIE)
        .expect("the response sets the refresh cookie")
}

async fn verify(app: &TestApp, email: &str, code: &str) -> TestResponse {
    app.send(
        TestRequest::post("/api/v1/auth/otp/verify").json(json!({ "email": email, "code": code })),
    )
    .await
}

async fn register(app: &TestApp, registration_ticket: &str) -> TestResponse {
    app.send(TestRequest::post("/api/v1/auth/register").json(json!({
        "registration_ticket": registration_ticket,
        "first_name": "Ada",
        "last_name": "Lovelace",
        "phone": "+237690000000",
    })))
    .await
}

async fn refresh(app: &TestApp, raw_refresh_token: &str) -> TestResponse {
    app.send(TestRequest::post("/api/v1/auth/refresh").refresh_cookie(raw_refresh_token))
        .await
}

async fn admin_login(app: &TestApp, email: &str, password: &str) -> TestResponse {
    app.send(
        TestRequest::post("/api/v1/admin/auth/login")
            .json(json!({ "email": email, "password": password })),
    )
    .await
}

/// `(id, replaced_by_id, is_revoked)` of the row holding `raw_refresh_token`.
async fn token_row(pool: &PgPool, raw_refresh_token: &str) -> (Uuid, Option<Uuid>, bool) {
    sqlx::query_as(
        "SELECT id, replaced_by_id, revoked_at IS NOT NULL
         FROM refresh_tokens WHERE token_hash = $1",
    )
    .bind(hash_refresh_token(raw_refresh_token))
    .fetch_one(pool)
    .await
    .expect("the refresh token row exists")
}

async fn unrevoked_token_count(pool: &PgPool, user_id: Uuid) -> i64 {
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .expect("count the unrevoked tokens")
}

async fn users_with_email(pool: &PgPool, email: &str) -> Vec<String> {
    sqlx::query_scalar("SELECT email FROM users WHERE lower(btrim(email)) = lower(btrim($1))")
        .bind(email)
        .fetch_all(pool)
        .await
        .expect("look the users up")
}

async fn suspend(pool: &PgPool, user_id: Uuid) {
    sqlx::query("UPDATE users SET is_active = FALSE WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .expect("suspend the user");
}

#[sqlx::test]
async fn verify_for_a_known_email_opens_a_session_with_a_strict_cookie(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    fixtures::seed_otp(&app, &seeker.email, OTP_CODE).await;

    let verified = verify(&app, &seeker.email, OTP_CODE).await;

    assert_eq!(verified.status, StatusCode::OK);
    assert_eq!(verified.body["data"]["is_new_user"], false);
    assert!(verified.body["data"]["access_token"].is_string());
    let cookie = verified
        .headers
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|cookie| cookie.starts_with(&format!("{REFRESH_TOKEN_COOKIE}=")))
        .expect("the session sets the refresh cookie");
    let attributes: Vec<&str> = cookie.split(';').map(str::trim).collect();
    for expected in ["HttpOnly", "Secure", "SameSite=Strict"] {
        assert!(attributes.contains(&expected), "{expected} in {cookie}");
    }
}

#[sqlx::test]
async fn verify_for_an_unknown_email_returns_a_ticket_and_creates_no_user(pool: PgPool) {
    let app = TestApp::new(pool);
    fixtures::seed_otp(&app, NEW_USER_EMAIL, OTP_CODE).await;

    let verified = verify(&app, NEW_USER_EMAIL, OTP_CODE).await;

    assert_eq!(verified.status, StatusCode::OK);
    assert_eq!(verified.body["data"]["is_new_user"], true);
    assert!(verified.body["data"]["registration_ticket"].is_string());
    assert!(verified.body["data"]["access_token"].is_null());
    assert!(verified.set_cookie(REFRESH_TOKEN_COOKIE).is_none());
    assert!(users_with_email(&app.pool, NEW_USER_EMAIL).await.is_empty());
}

#[sqlx::test]
async fn register_consumes_its_ticket_once(pool: PgPool) {
    let app = TestApp::new(pool);
    fixtures::seed_otp(&app, NEW_USER_EMAIL, OTP_CODE).await;
    let verified = verify(&app, NEW_USER_EMAIL, OTP_CODE).await;
    let ticket = verified.body["data"]["registration_ticket"]
        .as_str()
        .expect("registration ticket")
        .to_string();

    let first = register(&app, &ticket).await;
    assert_eq!(first.status, StatusCode::OK);
    assert!(first.body["data"]["access_token"].is_string());
    assert!(first.set_cookie(REFRESH_TOKEN_COOKIE).is_some());

    let replay = register(&app, &ticket).await;
    assert_eq!(replay.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error_code(&replay), "OTP_INVALID");
    assert_eq!(users_with_email(&app.pool, NEW_USER_EMAIL).await.len(), 1);
}

/// Both tickets are issued while the email is still unknown, so the second
/// registration reaches the insert and meets the first account.
#[sqlx::test]
async fn register_refuses_an_email_differing_only_in_case_or_spaces(pool: PgPool) {
    let app = TestApp::new(pool);
    let variant = format!("  {}  ", NEW_USER_EMAIL.to_uppercase());
    let mut tickets = Vec::new();
    for email in [NEW_USER_EMAIL, variant.as_str()] {
        fixtures::seed_otp(&app, NEW_USER_EMAIL, OTP_CODE).await;
        let verified = verify(&app, email, OTP_CODE).await;
        assert_eq!(verified.status, StatusCode::OK, "verify {email:?}");
        let ticket = verified.body["data"]["registration_ticket"]
            .as_str()
            .expect("registration ticket")
            .to_string();
        tickets.push(ticket);
    }

    assert_eq!(register(&app, &tickets[0]).await.status, StatusCode::OK);
    let duplicate = register(&app, &tickets[1]).await;

    assert_eq!(duplicate.status, StatusCode::CONFLICT);
    assert_eq!(error_code(&duplicate), "EMAIL_ALREADY_EXISTS");
    assert_eq!(
        users_with_email(&app.pool, &variant).await,
        vec![NEW_USER_EMAIL.to_string()]
    );
}

#[sqlx::test]
async fn a_failing_refresh_token_insert_rolls_the_new_user_back(pool: PgPool) {
    // Too long for `refresh_tokens.token_hash`, so only the second insert fails.
    let oversized_token_hash = "h".repeat(TOKEN_HASH_MAX_LENGTH + 1);

    let created = auth_repository::create_account(
        &pool,
        NewAccount {
            email: NEW_USER_EMAIL,
            first_name: Some("Ada"),
            last_name: "Lovelace",
            phone: "+237690000000",
            refresh_token_hash: &oversized_token_hash,
            refresh_ttl_days: NEW_ACCOUNT_REFRESH_TTL_DAYS,
        },
    )
    .await;

    assert!(matches!(created, Err(AppError::Database(_))), "{created:?}");
    assert!(users_with_email(&pool, NEW_USER_EMAIL).await.is_empty());
    let token_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM refresh_tokens")
        .fetch_one(&pool)
        .await
        .expect("count the refresh tokens");
    assert_eq!(token_count, 0);
}

#[sqlx::test]
async fn the_right_code_fails_once_the_attempt_budget_is_spent(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    fixtures::seed_otp(&app, &seeker.email, OTP_CODE).await;

    for attempt in 1..=app.config().auth.otp_max_attempts {
        let wrong = verify(&app, &seeker.email, WRONG_OTP_CODE).await;
        assert_eq!(wrong.status, StatusCode::UNAUTHORIZED, "attempt {attempt}");
        assert_eq!(error_code(&wrong), "OTP_INVALID", "attempt {attempt}");
    }
    let right = verify(&app, &seeker.email, OTP_CODE).await;

    assert_eq!(right.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error_code(&right), "OTP_INVALID");
}

/// `seed_otp` stands in for the two code requests: it records the issue time
/// as `otp/request` does, without the base rate-limit entry that would
/// otherwise answer every later request with the base cooldown.
#[sqlx::test]
async fn failed_attempts_survive_a_new_code_and_lengthen_the_cooldown(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let auth = &app.config().auth;

    for code_number in 1..=2 {
        fixtures::seed_otp(&app, &seeker.email, OTP_CODE).await;
        for _ in 0..auth.otp_max_attempts {
            let wrong = verify(&app, &seeker.email, WRONG_OTP_CODE).await;
            assert_eq!(wrong.status, StatusCode::UNAUTHORIZED, "code {code_number}");
        }
    }
    let requested = app
        .send(TestRequest::post("/api/v1/auth/otp/request").json(json!({ "email": seeker.email })))
        .await;

    assert_eq!(requested.status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(error_code(&requested), "OTP_RATE_LIMITED");
    // Above twice the base: the second code's failures counted on top of the first's.
    assert!(retry_after_seconds(&requested) > 2 * auth.otp_rate_limit_seconds);
}

#[sqlx::test]
async fn otp_request_for_the_admin_email_issues_no_code(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin = fixtures::admin(&app.pool).await;

    let requested = app
        .send(TestRequest::post("/api/v1/auth/otp/request").json(json!({ "email": admin.email })))
        .await;

    assert_eq!(requested.status, StatusCode::OK);
    let challenge = app
        .state
        .cache()
        .auth_challenge()
        .get(&otp_key(&admin.email))
        .await;
    assert!(challenge.is_none());
}

#[sqlx::test]
async fn refresh_rotates_and_chains_the_old_token_to_the_new_one(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let (_, old_token) = sign_in(&app, &seeker).await;

    let refreshed = refresh(&app, &old_token).await;

    assert_eq!(refreshed.status, StatusCode::OK);
    assert!(refreshed.body["data"]["access_token"].is_string());
    let new_token = refresh_cookie(&refreshed);
    assert_ne!(new_token, old_token);
    let (new_id, _, new_revoked) = token_row(&app.pool, &new_token).await;
    let (_, replaced_by_id, old_revoked) = token_row(&app.pool, &old_token).await;
    assert!(old_revoked);
    assert_eq!(replaced_by_id, Some(new_id));
    assert!(!new_revoked);
}

#[sqlx::test]
async fn replaying_a_revoked_token_after_the_grace_window_revokes_every_token(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let (_, old_token) = sign_in(&app, &seeker).await;
    // A second, independent session of the same user.
    sign_in(&app, &seeker).await;
    assert_eq!(refresh(&app, &old_token).await.status, StatusCode::OK);
    let (old_id, _, _) = token_row(&app.pool, &old_token).await;
    // What the grace window's expiry does.
    app.state
        .cache()
        .refresh_replay()
        .invalidate(&RefreshTokenId::new(old_id))
        .await;

    let replay = refresh(&app, &old_token).await;

    assert_eq!(replay.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error_code(&replay), "UNAUTHORIZED");
    assert_eq!(unrevoked_token_count(&app.pool, seeker.id).await, 0);
}

#[sqlx::test]
async fn two_refreshes_with_one_token_inside_the_grace_window_both_succeed(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let (_, old_token) = sign_in(&app, &seeker).await;

    let first = refresh(&app, &old_token).await;
    let second = refresh(&app, &old_token).await;

    assert_eq!(first.status, StatusCode::OK);
    assert_eq!(second.status, StatusCode::OK);
    assert!(second.body["data"]["access_token"].is_string());
    assert_eq!(refresh_cookie(&second), refresh_cookie(&first));
    assert_eq!(unrevoked_token_count(&app.pool, seeker.id).await, 1);
}

#[sqlx::test]
async fn after_logout_the_refresh_token_is_refused_even_inside_the_grace_window(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let (_, rotated_token) = sign_in(&app, &seeker).await;
    let refreshed = refresh(&app, &rotated_token).await;
    let current_token = refresh_cookie(&refreshed);
    let access_token = refreshed.body["data"]["access_token"]
        .as_str()
        .expect("access token")
        .to_string();

    let logout = app
        .send(
            TestRequest::post("/api/v1/auth/logout")
                .bearer(&access_token)
                .refresh_cookie(&current_token),
        )
        .await;
    assert_eq!(logout.status, StatusCode::OK);

    // Still inside its grace window, but its replacement is now revoked.
    let grace_replay = refresh(&app, &rotated_token).await;
    assert_eq!(grace_replay.status, StatusCode::UNAUTHORIZED);
    let current_replay = refresh(&app, &current_token).await;
    assert_eq!(current_replay.status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn rotation_deletes_tokens_expired_beyond_the_retention(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let (_, token) = sign_in(&app, &seeker).await;
    let stale = ("stale-token-hash", REFRESH_TOKEN_RETENTION_DAYS + 1);
    let retained = ("retained-token-hash", REFRESH_TOKEN_RETENTION_DAYS - 1);
    for (token_hash, expired_days_ago) in [stale, retained] {
        sqlx::query(
            "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
             VALUES ($1, $2, NOW() - make_interval(days => $3))",
        )
        .bind(seeker.id)
        .bind(token_hash)
        .bind(expired_days_ago)
        .execute(&app.pool)
        .await
        .expect("insert an expired token");
    }

    assert_eq!(refresh(&app, &token).await.status, StatusCode::OK);

    let remaining: Vec<String> =
        sqlx::query_scalar("SELECT token_hash FROM refresh_tokens WHERE token_hash = ANY($1)")
            .bind(vec![stale.0, retained.0])
            .fetch_all(&app.pool)
            .await
            .expect("read the expired tokens back");
    assert_eq!(remaining, vec![retained.0.to_string()]);
}

#[sqlx::test]
async fn a_suspended_user_is_refused_at_verify(pool: PgPool) {
    let app = TestApp::new(pool);
    let suspended = fixtures::suspended(&app.pool).await;
    fixtures::seed_otp(&app, &suspended.email, OTP_CODE).await;

    let verified = verify(&app, &suspended.email, OTP_CODE).await;

    assert_eq!(verified.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error_code(&verified), "ACCOUNT_SUSPENDED");
}

#[sqlx::test]
async fn a_suspended_user_is_refused_at_refresh(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let (_, token) = sign_in(&app, &seeker).await;
    suspend(&app.pool, seeker.id).await;

    let refreshed = refresh(&app, &token).await;

    assert_eq!(refreshed.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error_code(&refreshed), "ACCOUNT_SUSPENDED");
}

#[sqlx::test]
async fn a_valid_access_token_is_refused_within_the_is_active_ttl_of_a_suspension(pool: PgPool) {
    let options = TestAppOptions {
        real_is_active_cache: true,
        ..TestAppOptions::default()
    };
    let app = TestApp::with_options(pool, options);
    let seeker = fixtures::seeker(&app.pool).await;
    let access_token = token_for(&seeker);
    let me = || TestRequest::get("/api/v1/users/me").bearer(&access_token);

    // `is_active = true` is cached before this response returns, so the entry
    // has expired once the TTL has elapsed from here.
    assert_eq!(app.send(me()).await.status, StatusCode::OK);
    suspend(&app.pool, seeker.id).await;
    tokio::time::sleep(IS_ACTIVE_CACHE_TTL).await;

    let refused = app.send(me()).await;

    assert_eq!(refused.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error_code(&refused), "ACCOUNT_SUSPENDED");
}

#[sqlx::test]
async fn admin_login_with_the_right_password_opens_a_session(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin = fixtures::admin(&app.pool).await;

    let login = admin_login(&app, &admin.email, ADMIN_PASSWORD).await;

    assert_eq!(login.status, StatusCode::OK);
    assert!(login.body["data"]["access_token"].is_string());
    let (_, _, revoked) = token_row(&app.pool, &refresh_cookie(&login)).await;
    assert!(!revoked);
}

#[sqlx::test]
async fn admin_login_locks_after_five_wrong_passwords_even_for_the_right_one(pool: PgPool) {
    let app = TestApp::new(pool);
    let admin = fixtures::admin(&app.pool).await;

    for attempt in 1..=ADMIN_LOGIN_MAX_ATTEMPTS {
        let wrong = admin_login(&app, &admin.email, WRONG_ADMIN_PASSWORD).await;
        assert_eq!(wrong.status, StatusCode::UNAUTHORIZED, "attempt {attempt}");
        assert_eq!(
            error_code(&wrong),
            "INVALID_CREDENTIALS",
            "attempt {attempt}"
        );
    }
    let locked = admin_login(&app, &admin.email, ADMIN_PASSWORD).await;

    assert_eq!(locked.status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(error_code(&locked), "ADMIN_LOGIN_LOCKED");
    assert!(retry_after_seconds(&locked) > 0);
    assert!(locked.set_cookie(REFRESH_TOKEN_COOKIE).is_none());
}
