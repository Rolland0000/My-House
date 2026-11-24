#![cfg(feature = "integration")]
//! Contact reveal: the visibility conditions on the SQL side, the choice of the
//! secondary number, and the per-user cap (3 in the test config).

use axum::http::{header, StatusCode};
use sqlx::PgPool;
use uuid::Uuid;

use crate::harness::app::TestResponse;
use crate::harness::fixtures::{self, token_for, TestUser};
use crate::harness::{TestApp, TestRequest};

const OWNER_PHONE: &str = "+237690000111";
const OLDER_SECONDARY_PHONE: &str = "+237690000222";
const LATEST_SECONDARY_PHONE: &str = "+237690000333";
const REJECTED_SECONDARY_PHONE: &str = "+237690000444";

fn contact_uri(listing_id: Uuid) -> String {
    format!("/api/v1/listings/{listing_id}/contact")
}

async fn reveal(app: &TestApp, token: &str, listing_id: Uuid) -> TestResponse {
    app.send(TestRequest::get(&contact_uri(listing_id)).bearer(token))
        .await
}

async fn set_phone(pool: &PgPool, user: &TestUser, phone: &str) {
    sqlx::query("UPDATE users SET phone = $1 WHERE id = $2")
        .bind(phone)
        .bind(user.id)
        .execute(pool)
        .await
        .expect("set the user's phone");
}

/// Adds an owner request for `owner`, reviewed `days_ago` days back.
async fn insert_reviewed_request(
    pool: &PgPool,
    owner: &TestUser,
    status: &str,
    secondary_phone: &str,
    days_ago: i32,
) {
    sqlx::query(
        "INSERT INTO owner_requests
            (user_id, phone, secondary_phone, identity_data, status, reviewed_at)
         VALUES ($1, '+237690000000', $2,
                 '{\"full_name\": \"Test User\", \"id_type\": \"national_id\", \"id_number\": \"CM123456\"}',
                 $3::owner_request_status, NOW() - make_interval(days => $4))",
    )
    .bind(owner.id)
    .bind(secondary_phone)
    .bind(status)
    .bind(days_ago)
    .execute(pool)
    .await
    .expect("insert the owner request");
}

#[sqlx::test]
async fn requires_a_token(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::published_listing(&app, &owner).await;

    let response = app.send(TestRequest::get(&contact_uri(listing.id))).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn a_seeker_gets_the_owners_phone_on_a_public_listing(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    set_phone(&app.pool, &owner, OWNER_PHONE).await;
    let listing = fixtures::published_listing(&app, &owner).await;
    let token = token_for(&fixtures::seeker(&app.pool).await);

    let response = reveal(&app, &token, listing.id).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body["data"]["phone"], OWNER_PHONE);
    assert!(response.body["data"]["secondary_phone"].is_null());
}

#[sqlx::test]
async fn an_unavailable_public_listing_can_still_be_contacted(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::published_listing(&app, &owner).await;
    sqlx::query("UPDATE listings SET status = 'unavailable' WHERE id = $1")
        .bind(listing.id)
        .execute(&app.pool)
        .await
        .expect("mark the listing unavailable");
    let token = token_for(&fixtures::seeker(&app.pool).await);

    let response = reveal(&app, &token, listing.id).await;

    assert_eq!(response.status, StatusCode::OK);
}

#[sqlx::test]
async fn listings_outside_public_view_return_not_found(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let draft = fixtures::draft_listing(&app.pool, &owner).await;
    let photoless = fixtures::photoless_listing(&app.pool, &owner).await;
    let suspended_owner = fixtures::owner(&app.pool).await;
    let suspended_owners_listing = fixtures::published_listing(&app, &suspended_owner).await;
    sqlx::query("UPDATE users SET is_active = FALSE WHERE id = $1")
        .bind(suspended_owner.id)
        .execute(&app.pool)
        .await
        .expect("suspend the owner");
    let token = token_for(&fixtures::seeker(&app.pool).await);

    for (case, listing_id) in [
        ("draft", draft.id),
        ("no photo", photoless.id),
        ("suspended owner", suspended_owners_listing.id),
    ] {
        let response = reveal(&app, &token, listing_id).await;
        assert_eq!(response.status, StatusCode::NOT_FOUND, "{case}");
    }
}

#[sqlx::test]
async fn secondary_phone_comes_from_the_latest_approved_request(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    sqlx::query(
        "UPDATE owner_requests SET reviewed_at = NOW() - INTERVAL '30 days' WHERE user_id = $1",
    )
    .bind(owner.id)
    .execute(&app.pool)
    .await
    .expect("age the fixture's approved request");
    insert_reviewed_request(&app.pool, &owner, "approved", OLDER_SECONDARY_PHONE, 20).await;
    insert_reviewed_request(&app.pool, &owner, "approved", LATEST_SECONDARY_PHONE, 10).await;
    insert_reviewed_request(&app.pool, &owner, "rejected", REJECTED_SECONDARY_PHONE, 1).await;
    let listing = fixtures::published_listing(&app, &owner).await;
    let token = token_for(&fixtures::seeker(&app.pool).await);

    let response = reveal(&app, &token, listing.id).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(
        response.body["data"]["secondary_phone"],
        LATEST_SECONDARY_PHONE
    );
}

#[sqlx::test]
async fn the_cap_counts_distinct_public_listings_only(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let draft = fixtures::draft_listing(&app.pool, &owner).await;
    let cap = app.config().rate_limit.contact_reveal_max_per_window as usize;
    let mut listings = Vec::new();
    for _ in 0..=cap {
        listings.push(fixtures::published_listing(&app, &owner).await.id);
    }
    let token = token_for(&fixtures::seeker(&app.pool).await);

    let not_found = reveal(&app, &token, draft.id).await;
    assert_eq!(not_found.status, StatusCode::NOT_FOUND);

    for (index, listing_id) in listings[..cap].iter().enumerate() {
        let response = reveal(&app, &token, *listing_id).await;
        assert_eq!(response.status, StatusCode::OK, "listing {index}");
    }

    let again = reveal(&app, &token, listings[0]).await;
    assert_eq!(again.status, StatusCode::OK, "a listing already revealed");

    let over_cap = reveal(&app, &token, listings[cap]).await;
    assert_eq!(over_cap.status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(over_cap.body["error"]["code"], "CONTACT_RATE_LIMITED");
    let retry_after: u64 = over_cap
        .headers
        .get(header::RETRY_AFTER)
        .expect("the 429 carries Retry-After")
        .to_str()
        .expect("Retry-After is ASCII")
        .parse()
        .expect("Retry-After is a number of seconds");
    let window_seconds = app.config().rate_limit.contact_reveal_window_seconds;
    assert!((1..=window_seconds).contains(&retry_after));
}
