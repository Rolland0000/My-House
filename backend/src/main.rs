use std::net::SocketAddr;

use backend_my_house::{app_server::AppServer, app_state::AppState};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialise structured logging — level driven by RUST_LOG env var (default: info).
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let server = AppServer::new(state, addr);
    if let Err(e) = server.run().await {
        eprintln!("Server failed: {e}");
        std::process::exit(1);
    }
}
