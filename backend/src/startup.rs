use std::sync::Arc;

use crate::configuration::Settings;
use crate::database::Database;
use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get},
};
use tokio::net::TcpListener;

async fn build_app(db_pool: Database) -> Router {
    let db = Arc::new(db_pool);
    Router::new()
        .route("api/health", get(health_check))
        .with_state(db)
}


pub async fn serve_builder(setting: Settings) -> (Router, TcpListener) {
    let db_url = setting.database.get_database_url();
    let pool = Database::new_connection(&db_url).await.unwrap();

    // build our app
    let app = build_app(pool).await;

    // Bind the socket with the address
    let listener = TcpListener::bind(format!("127.0.0.1:{}", setting.app_port))
        .await
        .expect("Failed to bind address");

    (app, listener)
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
