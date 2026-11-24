#![cfg(feature = "integration")]
//! Public discovery: the visibility rule, filters, relevance and totals shared by
//! `GET /listings` and `GET /search`, driven through the HTTP API.

use std::collections::HashSet;

use axum::http::{Method, StatusCode};
use backend_my_house::shared::rbac::Role;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::harness::app::TestResponse;
use crate::harness::fixtures::{self, token_for, TestUser};
use crate::harness::{images, MultipartPart, TestApp, TestRequest};

const FEED_URI: &str = "/api/v1/listings";
const SEARCH_URI: &str = "/api/v1/search";
const UPLOAD_URI: &str = "/api/v1/media/upload";
const PHOTO_SIDE: u32 = 16;
const OWNER_PHONE: &str = "+237690000002";
const PDF_BYTES: &[u8] = b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog >>\nendobj\n%%EOF\n";
const IDENTITY_DATA: &str =
    r#"{"full_name": "Test User", "id_type": "national_id", "id_number": "CM123456"}"#;
/// Small enough that the combined-filter case spans several pages.
const SMALL_PAGE: u32 = 2;

fn error_code(response: &TestResponse) -> &str {
    response.body["error"]["code"].as_str().unwrap_or_default()
}

fn data_id(response: &TestResponse) -> Uuid {
    response.body["data"]["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("the response carries an id")
}

/// A listing body whose default words match none of the keywords searched below.
fn listing_body(overrides: Value) -> Value {
    let mut body = json!({
        "title": "Logement lumineux et calme",
        "description": "Logement bien situé, eau et électricité incluses, proche du marché.",
        "type": "apartment",
        "price": 100000,
        "city": "Douala",
        "neighborhood": "Akwa",
        "surface_m2": 40,
        "rooms": 2
    });
    let fields = body.as_object_mut().expect("the body is an object");
    for (key, value) in overrides.as_object().expect("overrides are an object") {
        fields.insert(key.clone(), value.clone());
    }
    body
}

async fn create_draft(app: &TestApp, token: &str, overrides: Value) -> Uuid {
    let created = app
        .send(
            TestRequest::post(FEED_URI)
                .bearer(token)
                .json(listing_body(overrides)),
        )
        .await;
    assert_eq!(created.status, StatusCode::CREATED);
    data_id(&created)
}

async fn upload_photo(app: &TestApp, token: &str, listing_id: Uuid) -> Uuid {
    let uploaded = app
        .send(TestRequest::post(UPLOAD_URI).bearer(token).multipart(vec![
            MultipartPart::text("listing_id", &listing_id.to_string()),
            MultipartPart::file(
                "file",
                "photo.jpg",
                "image/jpeg",
                images::jpeg(PHOTO_SIDE, PHOTO_SIDE),
            ),
        ]))
        .await;
    assert_eq!(uploaded.status, StatusCode::CREATED);
    data_id(&uploaded)
}

async fn publish(app: &TestApp, token: &str, listing_id: Uuid) {
    let published = app
        .send(TestRequest::post(&format!("{FEED_URI}/{listing_id}/publish")).bearer(token))
        .await;
    assert_eq!(published.status, StatusCode::OK);
}

/// Creates, illustrates and publishes a listing through the API; returns its id.
async fn public_listing(app: &TestApp, token: &str, overrides: Value) -> Uuid {
    let listing_id = create_draft(app, token, overrides).await;
    upload_photo(app, token, listing_id).await;
    publish(app, token, listing_id).await;
    listing_id
}

/// Ids in response order, and the `total` of the pagination envelope.
async fn page(app: &TestApp, uri: &str) -> (Vec<Uuid>, u64) {
    let response = app.send(TestRequest::get(uri)).await;
    assert_eq!(response.status, StatusCode::OK, "GET {uri}");
    let ids = response.body["data"]
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
    let total = response.body["pagination"]["total"]
        .as_u64()
        .expect("the page carries a total");
    (ids, total)
}

/// The rows of `uri` as a set, checked against its `total`.
async fn id_set(app: &TestApp, uri: &str) -> HashSet<Uuid> {
    let (ids, total) = page(app, uri).await;
    let set: HashSet<Uuid> = ids.iter().copied().collect();
    assert_eq!(set.len(), ids.len(), "GET {uri} repeats a row");
    assert_eq!(total, ids.len() as u64, "GET {uri} total");
    set
}

fn set_of(ids: &[Uuid]) -> HashSet<Uuid> {
    ids.iter().copied().collect()
}

async fn detail_status(app: &TestApp, listing_id: Uuid) -> StatusCode {
    app.send(TestRequest::get(&format!("{FEED_URI}/{listing_id}")))
        .await
        .status
}

/// Checks that `listing_id` is in the feed, in a search on `keyword` and opens anonymously,
/// or that it is absent from all three.
async fn assert_public(app: &TestApp, listing_id: Uuid, keyword: &str, expected: bool) {
    let in_feed = id_set(app, FEED_URI).await.contains(&listing_id);
    let in_search = id_set(app, &format!("{SEARCH_URI}?q={keyword}"))
        .await
        .contains(&listing_id);
    let detail = detail_status(app, listing_id).await;
    let expected_detail = if expected {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    };
    assert_eq!(
        (in_feed, in_search, detail),
        (expected, expected, expected_detail)
    );
}

async fn set_account_active(app: &TestApp, admin_token: &str, user_id: Uuid, is_active: bool) {
    let updated = app
        .send(
            TestRequest::new(Method::PATCH, &format!("/api/v1/admin/users/{user_id}"))
                .bearer(admin_token)
                .json(json!({ "is_active": is_active })),
        )
        .await;
    assert_eq!(updated.status, StatusCode::OK);
}

async fn rename(app: &TestApp, token: &str, first_name: &str) {
    let renamed = app
        .send(
            TestRequest::new(Method::PUT, "/api/v1/users/me")
                .bearer(token)
                .json(json!({
                    "first_name": first_name,
                    "last_name": "Nkotto",
                    "phone": OWNER_PHONE
                })),
        )
        .await;
    assert_eq!(renamed.status, StatusCode::OK);
}

#[sqlx::test]
async fn only_published_listings_with_a_photo_are_public(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let token = token_for(&owner);
    let title = json!({ "title": "Villa à Kribi face à la mer" });

    let visible = public_listing(&app, &token, title.clone()).await;
    let draft = create_draft(&app, &token, title.clone()).await;
    upload_photo(&app, &token, draft).await;
    let photoless = create_draft(&app, &token, title).await;
    let last_photo = upload_photo(&app, &token, photoless).await;
    publish(&app, &token, photoless).await;
    let deleted = app
        .send(
            TestRequest::new(Method::DELETE, &format!("/api/v1/media/{last_photo}")).bearer(&token),
        )
        .await;
    assert_eq!(deleted.status, StatusCode::NO_CONTENT);

    assert_eq!(id_set(&app, FEED_URI).await, set_of(&[visible]));
    assert_eq!(
        id_set(&app, &format!("{SEARCH_URI}?q=kribi")).await,
        set_of(&[visible])
    );
    assert_eq!(detail_status(&app, visible).await, StatusCode::OK);
    assert_eq!(detail_status(&app, draft).await, StatusCode::NOT_FOUND);
    assert_eq!(detail_status(&app, photoless).await, StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn a_suspended_owners_listings_leave_public_view_until_reactivation(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let admin_token = token_for(&fixtures::admin(&app.pool).await);
    let listing = public_listing(
        &app,
        &token_for(&owner),
        json!({ "title": "Duplex à Limbé" }),
    )
    .await;
    assert_public(&app, listing, "limbe", true).await;

    set_account_active(&app, &admin_token, owner.id, false).await;
    assert_public(&app, listing, "limbe", false).await;

    set_account_active(&app, &admin_token, owner.id, true).await;
    assert_public(&app, listing, "limbe", true).await;
}

#[sqlx::test]
async fn a_demoted_owners_listings_leave_public_view_until_re_approval(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let admin_token = token_for(&fixtures::admin(&app.pool).await);
    let listing = public_listing(
        &app,
        &token_for(&owner),
        json!({ "title": "Chalet à Buea" }),
    )
    .await;
    assert_public(&app, listing, "buea", true).await;

    let revoked = app
        .send(
            TestRequest::post(&format!("/api/v1/admin/users/{}/revoke-owner", owner.id))
                .bearer(&admin_token),
        )
        .await;
    assert_eq!(revoked.status, StatusCode::OK);
    assert_public(&app, listing, "buea", false).await;

    let demoted = TestUser {
        role: Role::Seeker,
        ..owner
    };
    let submitted = app
        .send(
            TestRequest::post("/api/v1/owner-requests")
                .bearer(&token_for(&demoted))
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
    let approved = app
        .send(
            TestRequest::new(
                Method::PATCH,
                &format!("/api/v1/admin/owner-requests/{}", data_id(&submitted)),
            )
            .bearer(&admin_token)
            .json(json!({ "status": "approved" })),
        )
        .await;
    assert_eq!(approved.status, StatusCode::OK);
    assert_public(&app, listing, "buea", true).await;
}

#[sqlx::test]
async fn place_filters_ignore_case_and_accents(pool: PgPool) {
    let app = TestApp::new(pool);
    let token = token_for(&fixtures::owner(&app.pool).await);
    let bonamoussadi = public_listing(
        &app,
        &token,
        json!({ "city": "Douala", "neighborhood": "Bonamoussadi" }),
    )
    .await;
    let akwa = public_listing(
        &app,
        &token,
        json!({ "city": "Douala", "neighborhood": "Akwa" }),
    )
    .await;
    let bastos = public_listing(
        &app,
        &token,
        json!({ "city": "Yaoundé", "neighborhood": "Bastos" }),
    )
    .await;

    let cases = [
        ("city=Douala", set_of(&[bonamoussadi, akwa])),
        ("city=douala", set_of(&[bonamoussadi, akwa])),
        ("neighborhood=Bonamoussadi", set_of(&[bonamoussadi])),
        ("neighborhood=bonamoussadi", set_of(&[bonamoussadi])),
        ("city=Yaound%C3%A9", set_of(&[bastos])),
        ("city=yaounde", set_of(&[bastos])),
        ("city=YAOUNDE", set_of(&[bastos])),
    ];
    for (filter, expected) in cases {
        for base in [FEED_URI, SEARCH_URI] {
            let uri = format!("{base}?{filter}");
            assert_eq!(id_set(&app, &uri).await, expected, "GET {uri}");
        }
    }
}

#[sqlx::test]
async fn a_title_match_ranks_above_a_description_only_match(pool: PgPool) {
    let app = TestApp::new(pool);
    let token = token_for(&fixtures::owner(&app.pool).await);
    // Created first, so only relevance can put it ahead of the newer listing.
    let in_title =
        public_listing(&app, &token, json!({ "title": "Appartement avec piscine" })).await;
    let in_description = public_listing(
        &app,
        &token,
        json!({ "description": "Résidence sécurisée avec une grande piscine commune." }),
    )
    .await;

    let (feed, _) = page(&app, FEED_URI).await;
    assert_eq!(feed, vec![in_description, in_title]);
    let (ranked, total) = page(&app, &format!("{SEARCH_URI}?q=piscine")).await;
    assert_eq!(ranked, vec![in_title, in_description]);
    assert_eq!(total, 2);
}

#[sqlx::test]
async fn an_english_type_label_finds_listings_of_that_type(pool: PgPool) {
    let app = TestApp::new(pool);
    let token = token_for(&fixtures::owner(&app.pool).await);
    let apartment = public_listing(&app, &token, json!({ "type": "apartment" })).await;
    let house = public_listing(&app, &token, json!({ "type": "house" })).await;
    let room = public_listing(&app, &token, json!({ "type": "room" })).await;

    for (keyword, expected) in [("apartment", apartment), ("house", house), ("room", room)] {
        assert_eq!(
            id_set(&app, &format!("{SEARCH_URI}?q={keyword}")).await,
            set_of(&[expected]),
            "q={keyword}"
        );
    }
}

#[sqlx::test]
async fn an_empty_q_returns_the_feed_rows_and_total(pool: PgPool) {
    let app = TestApp::new(pool);
    let token = token_for(&fixtures::owner(&app.pool).await);
    for _ in 0..3 {
        public_listing(&app, &token, json!({})).await;
    }
    create_draft(&app, &token, json!({})).await;

    let feed = page(&app, FEED_URI).await;
    assert_eq!(feed.1, 3);
    for blank in ["", "%20%20"] {
        assert_eq!(
            page(&app, &format!("{SEARCH_URI}?q={blank}")).await,
            feed,
            "q={blank:?}"
        );
    }
}

#[sqlx::test]
async fn with_keywords_and_filters_total_matches_the_rows_across_pages(pool: PgPool) {
    let app = TestApp::new(pool);
    let token = token_for(&fixtures::owner(&app.pool).await);
    let mut expected = Vec::new();
    for _ in 0..3 {
        expected
            .push(public_listing(&app, &token, json!({ "title": "Studio avec terrasse" })).await);
    }
    for _ in 0..2 {
        expected.push(
            public_listing(
                &app,
                &token,
                json!({ "description": "Logement rénové avec une terrasse ombragée." }),
            )
            .await,
        );
    }
    // Each misses exactly one condition.
    public_listing(
        &app,
        &token,
        json!({ "title": "Maison avec terrasse", "city": "Kribi" }),
    )
    .await;
    public_listing(
        &app,
        &token,
        json!({ "title": "Villa avec terrasse", "price": 900000 }),
    )
    .await;
    public_listing(&app, &token, json!({})).await;

    let query =
        format!("{SEARCH_URI}?q=terrasse&city=douala&price_max=200000&per_page={SMALL_PAGE}");
    let (_, total) = page(&app, &query).await;
    assert_eq!(total, expected.len() as u64);

    let mut read = Vec::new();
    let page_count = total.div_ceil(u64::from(SMALL_PAGE));
    for page_number in 1..=page_count {
        let (ids, page_total) = page(&app, &format!("{query}&page={page_number}")).await;
        assert_eq!(page_total, total, "page {page_number}");
        read.extend(ids);
    }
    let (beyond, _) = page(&app, &format!("{query}&page={}", page_count + 1)).await;
    assert!(beyond.is_empty());

    assert_eq!(read.len() as u64, total);
    assert_eq!(set_of(&read), set_of(&expected));
}

#[sqlx::test]
async fn repeated_type_values_combine_as_or(pool: PgPool) {
    let app = TestApp::new(pool);
    let token = token_for(&fixtures::owner(&app.pool).await);
    let apartment = public_listing(&app, &token, json!({ "type": "apartment" })).await;
    let studio = public_listing(&app, &token, json!({ "type": "studio" })).await;
    let house = public_listing(&app, &token, json!({ "type": "house" })).await;

    for base in [FEED_URI, SEARCH_URI] {
        assert_eq!(
            id_set(&app, &format!("{base}?type=apartment&type=studio")).await,
            set_of(&[apartment, studio]),
            "{base}"
        );
        assert_eq!(
            id_set(&app, &format!("{base}?type=house")).await,
            set_of(&[house]),
            "{base}"
        );
    }
}

#[sqlx::test]
async fn invalid_price_bounds_are_refused(pool: PgPool) {
    let app = TestApp::new(pool);

    for bounds in [
        "price_min=-1",
        "price_max=-1",
        "price_min=200000&price_max=100000",
    ] {
        let response = app
            .send(TestRequest::get(&format!("{SEARCH_URI}?{bounds}")))
            .await;
        assert_eq!(response.status, StatusCode::BAD_REQUEST, "{bounds}");
        assert_eq!(error_code(&response), "INVALID_QUERY_PARAM", "{bounds}");
    }
    let equal = app
        .send(TestRequest::get(&format!(
            "{SEARCH_URI}?price_min=100000&price_max=100000"
        )))
        .await;
    assert_eq!(equal.status, StatusCode::OK);
}

#[sqlx::test]
async fn renaming_the_owner_moves_their_listings_to_the_new_name(pool: PgPool) {
    let app = TestApp::new(pool);
    let token = token_for(&fixtures::owner(&app.pool).await);
    let listing = public_listing(&app, &token, json!({})).await;

    rename(&app, &token, "Ekambi").await;
    assert_eq!(
        id_set(&app, &format!("{SEARCH_URI}?q=ekambi")).await,
        set_of(&[listing])
    );

    rename(&app, &token, "Fotso").await;
    assert_eq!(
        id_set(&app, &format!("{SEARCH_URI}?q=fotso")).await,
        set_of(&[listing])
    );
    assert!(id_set(&app, &format!("{SEARCH_URI}?q=ekambi"))
        .await
        .is_empty());
}
