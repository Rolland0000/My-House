#![cfg(feature = "integration")]
//! Listing and photo lifecycle: creation, uploads, cover, order, publication
//! and deletion, driven through the HTTP API, concurrent requests included.

use std::collections::HashSet;
use std::path::PathBuf;

use axum::http::{Method, StatusCode};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::harness::app::TestResponse;
use crate::harness::fixtures::{self, token_for};
use crate::harness::{images, MultipartPart, TestApp, TestRequest};

const LISTINGS_URI: &str = "/api/v1/listings";
const UPLOAD_URI: &str = "/api/v1/media/upload";
const MAX_PHOTOS: usize = 5;
const PHOTO_SIDE: u32 = 16;
const EXIF_ORIENTATION: u16 = 6;
/// Each round replays the publish/delete race; the scheduler picks the winner.
const PUBLISH_RACE_ROUNDS: usize = 8;

fn error_code(response: &TestResponse) -> &str {
    response.body["error"]["code"].as_str().unwrap_or_default()
}

fn data_id(response: &TestResponse) -> Uuid {
    response.body["data"]["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("the response carries an id")
}

fn listing_body() -> Value {
    json!({
        "title": "Studio meublé à Akwa",
        "description": "Studio lumineux proche du marché, eau et électricité incluses.",
        "type": "studio",
        "price": 85000,
        "city": "Douala",
        "neighborhood": "Akwa",
        "surface_m2": 30,
        "rooms": 1
    })
}

async fn create_listing(app: &TestApp, token: &str) -> TestResponse {
    app.send(
        TestRequest::post(LISTINGS_URI)
            .bearer(token)
            .json(listing_body()),
    )
    .await
}

async fn upload_file(
    app: &TestApp,
    token: &str,
    listing_id: Uuid,
    filename: &str,
    bytes: Vec<u8>,
) -> TestResponse {
    app.send(TestRequest::post(UPLOAD_URI).bearer(token).multipart(vec![
        MultipartPart::text("listing_id", &listing_id.to_string()),
        MultipartPart::file("file", filename, "image/jpeg", bytes),
    ]))
    .await
}

async fn upload_jpeg(app: &TestApp, token: &str, listing_id: Uuid) -> TestResponse {
    upload_file(
        app,
        token,
        listing_id,
        "photo.jpg",
        images::jpeg(PHOTO_SIDE, PHOTO_SIDE),
    )
    .await
}

/// Uploads `count` photos one after the other and returns their ids in upload order.
async fn upload_photos(app: &TestApp, token: &str, listing_id: Uuid, count: usize) -> Vec<Uuid> {
    let mut ids = Vec::with_capacity(count);
    for _ in 0..count {
        let uploaded = upload_jpeg(app, token, listing_id).await;
        assert_eq!(uploaded.status, StatusCode::CREATED);
        ids.push(data_id(&uploaded));
    }
    ids
}

async fn publish(app: &TestApp, token: &str, listing_id: Uuid) -> TestResponse {
    app.send(TestRequest::post(&format!("{LISTINGS_URI}/{listing_id}/publish")).bearer(token))
        .await
}

async fn delete_media(app: &TestApp, token: &str, media_id: Uuid) -> TestResponse {
    app.send(TestRequest::new(Method::DELETE, &format!("/api/v1/media/{media_id}")).bearer(token))
        .await
}

async fn reorder(app: &TestApp, token: &str, listing_id: Uuid, media_ids: &[Uuid]) -> TestResponse {
    app.send(
        TestRequest::new(
            Method::PUT,
            &format!("{LISTINGS_URI}/{listing_id}/media/order"),
        )
        .bearer(token)
        .json(json!({ "media_ids": media_ids })),
    )
    .await
}

/// `(id, is_cover, position)` of every photo of the listing, by position then id.
async fn media_rows(pool: &PgPool, listing_id: Uuid) -> Vec<(Uuid, bool, i16)> {
    sqlx::query_as(
        "SELECT id, is_cover, position FROM listing_media
         WHERE listing_id = $1 ORDER BY position, id",
    )
    .bind(listing_id)
    .fetch_all(pool)
    .await
    .expect("read the listing photos")
}

async fn is_published(pool: &PgPool, listing_id: Uuid) -> bool {
    sqlx::query_scalar("SELECT published_at IS NOT NULL FROM listings WHERE id = $1")
        .bind(listing_id)
        .fetch_one(pool)
        .await
        .expect("read the listing publication state")
}

/// Files stored under the listing's `listings/{id}/` storage prefix.
fn listing_files(app: &TestApp, listing_id: Uuid) -> Vec<PathBuf> {
    let dir = app
        .storage_root()
        .join("listings")
        .join(listing_id.to_string());
    if !dir.exists() {
        return Vec::new();
    }
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("read the listing storage directory")
        .map(|entry| entry.expect("read a storage entry").path())
        .filter(|path| path.is_file())
        .collect();
    files.sort();
    files
}

async fn feed_ids(app: &TestApp) -> Vec<Uuid> {
    let feed = app.send(TestRequest::get(LISTINGS_URI)).await;
    assert_eq!(feed.status, StatusCode::OK);
    feed.body["data"]
        .as_array()
        .expect("the feed is a list")
        .iter()
        .filter_map(|item| item["id"].as_str().and_then(|id| Uuid::parse_str(id).ok()))
        .collect()
}

async fn assert_not_public(app: &TestApp, listing_id: Uuid) {
    assert!(!feed_ids(app).await.contains(&listing_id));
    let anonymous = app
        .send(TestRequest::get(&format!("{LISTINGS_URI}/{listing_id}")))
        .await;
    assert_eq!(anonymous.status, StatusCode::NOT_FOUND);
    assert_eq!(error_code(&anonymous), "LISTING_NOT_FOUND");
}

#[sqlx::test]
async fn an_owner_creates_a_draft_that_only_its_owner_sees(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let token = token_for(&owner);

    let created = create_listing(&app, &token).await;
    assert_eq!(created.status, StatusCode::CREATED);
    assert!(created.body["data"]["published_at"].is_null());
    let listing_id = data_id(&created);

    assert_not_public(&app, listing_id).await;
    let own = app
        .send(TestRequest::get(&format!("{LISTINGS_URI}/{listing_id}")).bearer(&token))
        .await;
    assert_eq!(own.status, StatusCode::OK);
    assert_eq!(data_id(&own), listing_id);
    assert!(own.body["data"]["published_at"].is_null());
}

#[sqlx::test]
async fn a_seeker_cannot_create_and_another_owner_gets_the_unknown_listing_404(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let owner = fixtures::owner(&app.pool).await;
    let intruder = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;
    let intruder_token = token_for(&intruder);

    let refused = create_listing(&app, &token_for(&seeker)).await;
    assert_eq!(refused.status, StatusCode::FORBIDDEN);
    assert_eq!(error_code(&refused), "FORBIDDEN");
    let seeker_listings: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM listings WHERE owner_id = $1")
            .bind(seeker.id)
            .fetch_one(&app.pool)
            .await
            .unwrap();
    assert_eq!(seeker_listings, 0);

    let writes: [(&str, Method, &str, Option<Value>); 4] = [
        ("edit", Method::PUT, "", Some(listing_body())),
        (
            "status",
            Method::PATCH,
            "/status",
            Some(json!({ "status": "unavailable" })),
        ),
        ("publish", Method::POST, "/publish", None),
        ("delete", Method::DELETE, "", None),
    ];
    for (case, method, suffix, body) in writes {
        let mut responses = Vec::new();
        for target in [listing.id, Uuid::new_v4()] {
            let mut request =
                TestRequest::new(method.clone(), &format!("{LISTINGS_URI}/{target}{suffix}"))
                    .bearer(&intruder_token);
            if let Some(body) = &body {
                request = request.json(body.clone());
            }
            responses.push(app.send(request).await);
        }
        let (foreign, unknown) = (&responses[0], &responses[1]);
        assert_eq!(foreign.status, StatusCode::NOT_FOUND, "{case}");
        assert_eq!(error_code(foreign), "LISTING_NOT_FOUND", "{case}");
        assert_eq!(foreign.body, unknown.body, "{case}");
    }

    let (title, status, published): (String, String, bool) = sqlx::query_as(
        "SELECT title, status::text, published_at IS NOT NULL FROM listings WHERE id = $1",
    )
    .bind(listing.id)
    .fetch_one(&app.pool)
    .await
    .expect("the listing still exists");
    assert_eq!(
        (title.as_str(), status.as_str(), published),
        ("Appartement test", "available", false)
    );
}

#[sqlx::test]
async fn publishing_without_a_photo_is_refused_and_keeps_the_draft(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;

    let refused = publish(&app, &token_for(&owner), listing.id).await;
    assert_eq!(refused.status, StatusCode::CONFLICT);
    assert_eq!(error_code(&refused), "PHOTO_REQUIRED");
    assert!(!is_published(&app.pool, listing.id).await);
}

#[sqlx::test]
async fn the_first_upload_is_the_cover_and_a_sixth_is_refused(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;
    let token = token_for(&owner);

    for index in 0..MAX_PHOTOS {
        let uploaded = upload_jpeg(&app, &token, listing.id).await;
        assert_eq!(uploaded.status, StatusCode::CREATED);
        assert_eq!(
            uploaded.body["data"]["is_cover"],
            index == 0,
            "photo {index}"
        );
        assert_eq!(uploaded.body["data"]["position"], index, "photo {index}");
    }

    let sixth = upload_jpeg(&app, &token, listing.id).await;
    assert_eq!(sixth.status, StatusCode::CONFLICT);
    assert_eq!(error_code(&sixth), "MEDIA_QUOTA_EXCEEDED");
    assert_eq!(media_rows(&app.pool, listing.id).await.len(), MAX_PHOTOS);
    assert_eq!(listing_files(&app, listing.id).len(), MAX_PHOTOS);
}

#[sqlx::test]
async fn six_concurrent_uploads_end_with_five_photos_and_one_cover(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;
    let token = token_for(&owner);

    let (first, second, third, fourth, fifth, sixth) = tokio::join!(
        upload_jpeg(&app, &token, listing.id),
        upload_jpeg(&app, &token, listing.id),
        upload_jpeg(&app, &token, listing.id),
        upload_jpeg(&app, &token, listing.id),
        upload_jpeg(&app, &token, listing.id),
        upload_jpeg(&app, &token, listing.id),
    );
    let responses = [first, second, third, fourth, fifth, sixth];
    let created = responses
        .iter()
        .filter(|response| response.status == StatusCode::CREATED)
        .count();
    let refused: Vec<&TestResponse> = responses
        .iter()
        .filter(|response| response.status != StatusCode::CREATED)
        .collect();
    assert_eq!(created, MAX_PHOTOS);
    assert_eq!(refused.len(), 1);
    assert_eq!(refused[0].status, StatusCode::CONFLICT);
    assert_eq!(error_code(refused[0]), "MEDIA_QUOTA_EXCEEDED");

    let rows = media_rows(&app.pool, listing.id).await;
    assert_eq!(rows.len(), MAX_PHOTOS);
    assert_eq!(rows.iter().filter(|(_, is_cover, _)| *is_cover).count(), 1);
    let positions: HashSet<i16> = rows.iter().map(|(_, _, position)| *position).collect();
    assert_eq!(positions.len(), MAX_PHOTOS);
    // The refused upload deletes the file it wrote before losing the locked recheck.
    assert_eq!(listing_files(&app, listing.id).len(), MAX_PHOTOS);
}

#[sqlx::test]
async fn a_non_image_named_as_jpeg_is_refused_and_stores_nothing(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;

    let refused = upload_file(
        &app,
        &token_for(&owner),
        listing.id,
        "photo.jpg",
        b"plain text pretending to be a JPEG photo".to_vec(),
    )
    .await;
    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(error_code(&refused), "INVALID_FILE");
    assert!(media_rows(&app.pool, listing.id).await.is_empty());
    assert!(listing_files(&app, listing.id).is_empty());
}

#[sqlx::test]
async fn an_uploaded_jpeg_is_stored_without_exif_and_still_decodes(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;
    let original = images::jpeg_with_exif(PHOTO_SIDE, PHOTO_SIDE, EXIF_ORIENTATION);
    assert!(original.windows(4).any(|window| window == b"Exif"));

    let uploaded = upload_file(&app, &token_for(&owner), listing.id, "photo.jpg", original).await;
    assert_eq!(uploaded.status, StatusCode::CREATED);

    let files = listing_files(&app, listing.id);
    assert_eq!(files.len(), 1);
    let stored = std::fs::read(&files[0]).expect("read the stored photo");
    assert!(!stored.windows(4).any(|window| window == b"Exif"));
    assert!(!stored.windows(11).any(|window| window == b"GPSLatitude"));
    image::load_from_memory(&stored).expect("the stored photo decodes");
}

#[sqlx::test]
async fn the_cover_cannot_be_deleted_while_others_remain_and_moves_with_patch(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;
    let token = token_for(&owner);
    let ids = upload_photos(&app, &token, listing.id, 3).await;
    let (old_cover, new_cover) = (ids[0], ids[2]);

    let refused = delete_media(&app, &token, old_cover).await;
    assert_eq!(refused.status, StatusCode::CONFLICT);
    assert_eq!(error_code(&refused), "COVER_PHOTO_REQUIRED");
    assert_eq!(media_rows(&app.pool, listing.id).await.len(), 3);
    assert_eq!(listing_files(&app, listing.id).len(), 3);

    let promoted = app
        .send(
            TestRequest::new(
                Method::PATCH,
                &format!("{LISTINGS_URI}/{}/cover", listing.id),
            )
            .bearer(&token)
            .json(json!({ "media_id": new_cover })),
        )
        .await;
    assert_eq!(promoted.status, StatusCode::OK);
    assert_eq!(data_id(&promoted), new_cover);
    assert_eq!(promoted.body["data"]["is_cover"], true);
    let covers: Vec<Uuid> = media_rows(&app.pool, listing.id)
        .await
        .into_iter()
        .filter(|(_, is_cover, _)| *is_cover)
        .map(|(id, _, _)| id)
        .collect();
    assert_eq!(covers, [new_cover]);

    let deleted = delete_media(&app, &token, old_cover).await;
    assert_eq!(deleted.status, StatusCode::NO_CONTENT);
    assert_eq!(listing_files(&app, listing.id).len(), 2);
}

#[sqlx::test]
async fn a_stale_or_incomplete_order_is_a_conflict_and_changes_nothing(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;
    let token = token_for(&owner);
    let ids = upload_photos(&app, &token, listing.id, 3).await;
    let deleted = delete_media(&app, &token, ids[2]).await;
    assert_eq!(deleted.status, StatusCode::NO_CONTENT);
    let before = media_rows(&app.pool, listing.id).await;

    let stale = [ids[1], ids[0], ids[2]];
    let incomplete = [ids[1]];
    for (case, sent) in [("stale", &stale[..]), ("incomplete", &incomplete[..])] {
        let refused = reorder(&app, &token, listing.id, sent).await;
        assert_eq!(refused.status, StatusCode::CONFLICT, "{case}");
        assert_eq!(error_code(&refused), "MEDIA_ORDER_CONFLICT", "{case}");
        assert_eq!(media_rows(&app.pool, listing.id).await, before, "{case}");
    }
}

#[sqlx::test]
async fn two_concurrent_reorders_leave_distinct_positions_without_gaps(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let listing = fixtures::draft_listing(&app.pool, &owner).await;
    let token = token_for(&owner);
    let mut ids = upload_photos(&app, &token, listing.id, MAX_PHOTOS).await;
    // Deleting a middle photo leaves a gap in the positions before the reorders.
    let removed = ids.remove(2);
    assert_eq!(
        delete_media(&app, &token, removed).await.status,
        StatusCode::NO_CONTENT
    );

    let reversed: Vec<Uuid> = ids.iter().rev().copied().collect();
    let mut rotated = ids.clone();
    rotated.rotate_left(1);
    let (first, second) = tokio::join!(
        reorder(&app, &token, listing.id, &reversed),
        reorder(&app, &token, listing.id, &rotated),
    );
    assert_eq!(first.status, StatusCode::OK);
    assert_eq!(second.status, StatusCode::OK);

    let stored: Vec<Uuid> = media_rows(&app.pool, listing.id)
        .await
        .into_iter()
        .enumerate()
        .map(|(index, (id, _, position))| {
            assert_eq!(usize::try_from(position).ok(), Some(index));
            id
        })
        .collect();
    assert!(
        stored == reversed || stored == rotated,
        "the final order is one of the two sent: {stored:?}"
    );
}

#[sqlx::test]
async fn publish_racing_the_last_photo_deletion_never_exposes_a_photoless_listing(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let token = token_for(&owner);

    for round in 0..PUBLISH_RACE_ROUNDS {
        let listing = fixtures::draft_listing(&app.pool, &owner).await;
        let photo = upload_photos(&app, &token, listing.id, 1).await[0];

        let (published, deleted) = tokio::join!(
            publish(&app, &token, listing.id),
            delete_media(&app, &token, photo),
        );
        assert_eq!(deleted.status, StatusCode::NO_CONTENT, "round {round}");
        assert!(media_rows(&app.pool, listing.id).await.is_empty());

        // Publishing first leaves a published listing without a photo, which the
        // visibility rule hides; deleting first makes the publish fail.
        match published.status {
            StatusCode::OK => assert!(is_published(&app.pool, listing.id).await),
            StatusCode::CONFLICT => {
                assert_eq!(error_code(&published), "PHOTO_REQUIRED", "round {round}");
                assert!(!is_published(&app.pool, listing.id).await);
            }
            other => panic!("round {round}: unexpected publish status {other}"),
        }
        assert_not_public(&app, listing.id).await;
    }
}

#[sqlx::test]
async fn deleting_a_listing_removes_its_rows_and_its_files(pool: PgPool) {
    let app = TestApp::new(pool);
    let owner = fixtures::owner(&app.pool).await;
    let token = token_for(&owner);
    let created = create_listing(&app, &token).await;
    assert_eq!(created.status, StatusCode::CREATED);
    let listing_id = data_id(&created);
    upload_photos(&app, &token, listing_id, 3).await;
    assert_eq!(listing_files(&app, listing_id).len(), 3);

    let deleted = app
        .send(
            TestRequest::new(Method::DELETE, &format!("{LISTINGS_URI}/{listing_id}"))
                .bearer(&token),
        )
        .await;
    assert_eq!(deleted.status, StatusCode::NO_CONTENT);

    let (listings, photos): (i64, i64) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM listings WHERE id = $1),
                (SELECT COUNT(*) FROM listing_media WHERE listing_id = $1)",
    )
    .bind(listing_id)
    .fetch_one(&app.pool)
    .await
    .unwrap();
    assert_eq!((listings, photos), (0, 0));
    assert!(listing_files(&app, listing_id).is_empty());
}
