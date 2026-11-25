#![cfg(feature = "integration")]
//! Profile writes that touch storage, driven through the HTTP API.

use axum::http::StatusCode;
use sqlx::PgPool;

use crate::harness::app::{TestResponse, TEST_PUBLIC_MEDIA_BASE_URL};
use crate::harness::fixtures::{self, token_for};
use crate::harness::{images, MultipartPart, TestApp, TestRequest};

const AVATAR_URI: &str = "/api/v1/users/me/avatar";
const AVATAR_SIDE: u32 = 16;

async fn upload_avatar(app: &TestApp, token: &str) -> TestResponse {
    app.send(
        TestRequest::post(AVATAR_URI)
            .bearer(token)
            .multipart(vec![MultipartPart::file(
                "file",
                "avatar.png",
                "image/png",
                images::png(AVATAR_SIDE, AVATAR_SIDE),
            )]),
    )
    .await
}

/// Storage key behind an avatar URL returned by the API.
fn avatar_key(response: &TestResponse) -> String {
    let url = response.body["data"]["avatar_url"]
        .as_str()
        .expect("the response carries an avatar URL");
    url.strip_prefix(TEST_PUBLIC_MEDIA_BASE_URL)
        .and_then(|path| path.strip_prefix('/'))
        .expect("the avatar URL is under the public media base")
        .to_string()
}

#[sqlx::test]
async fn replacing_the_avatar_deletes_the_previous_file(pool: PgPool) {
    let app = TestApp::new(pool);
    let seeker = fixtures::seeker(&app.pool).await;
    let token = token_for(&seeker);

    let first = upload_avatar(&app, &token).await;
    assert_eq!(first.status, StatusCode::OK);
    let first_key = avatar_key(&first);
    assert!(app.storage_root().join(&first_key).is_file());

    let second = upload_avatar(&app, &token).await;
    assert_eq!(second.status, StatusCode::OK);
    let second_key = avatar_key(&second);

    assert_ne!(first_key, second_key);
    assert!(!app.storage_root().join(&first_key).exists());
    assert!(app.storage_root().join(&second_key).is_file());
    let remaining = std::fs::read_dir(app.storage_root().join(format!("avatars/{}", seeker.id)))
        .expect("read the avatar directory")
        .count();
    assert_eq!(remaining, 1);
}
