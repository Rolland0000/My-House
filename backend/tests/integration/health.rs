#![cfg(feature = "integration")]

use axum::http::StatusCode;
use sqlx::PgPool;

use crate::harness::{TestApp, TestRequest};

#[sqlx::test]
async fn health_reports_a_reachable_database(pool: PgPool) {
    let app = TestApp::new(pool);

    let response = app.send(TestRequest::get("/health")).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body["status"], "ok");
}
