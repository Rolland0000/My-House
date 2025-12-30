use std::sync::{Arc, Mutex};

use crate::configuration::Settings;
use crate::database::Database;
use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get},
};
use tokio::net::TcpListener;
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tower_http::LatencyUnit;

// Health check endpoint that returns HTTP 200 OK status
#[tracing::instrument(name = "Building application router")]
async fn build_app(db_pool: Database) -> Router {
    tracing::debug!("Creating application router with database pool");
    let db = Arc::new(Mutex::new(db_pool));
    
    let app = Router::new()
        .route("/api/health", get(health_check))
        .with_state(db)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO).latency_unit(LatencyUnit::Millis))
        );
    
    tracing::info!("Application router built successfully");
    app
}

#[tracing::instrument(name = "Setting up server", skip(setting))]
pub async fn serve_builder(setting: Settings) -> (Router, TcpListener) {
    tracing::info!("Initializing server with port {}", setting.app_port);
    
    let db_url = setting.database.get_database_url();
    
    let pool = Database::new_connection(&db_url).await.unwrap();
    tracing::info!("Database connection pool established");

    // build our app
    let app = build_app(pool).await;

    // Bind the socket with the address
    let bind_addr = format!("127.0.0.1:{}", setting.app_port);
    tracing::debug!("Attempting to bind to {}", bind_addr);
    
    let listener = TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind address");
    
    tracing::info!("Successfully bound to {}", bind_addr);

    (app, listener)
}

#[tracing::instrument(name = "Health check")]
async fn health_check() -> impl IntoResponse {
    tracing::debug!("Health check endpoint called");
    StatusCode::OK
}
