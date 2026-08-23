use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

use crate::app_state::AppState;
use crate::route::build_router;

/// Encapsulates the Axum HTTP server lifecycle: binding, routing, and
/// graceful shutdown on SIGINT / SIGTERM.
pub struct AppServer {
    state: AppState,
    addr: SocketAddr,
}

impl AppServer {
    pub fn new(state: AppState, addr: SocketAddr) -> Self {
        Self { state, addr }
    }

    /// Starts the HTTP server and blocks until a shutdown signal is received.
    ///
    /// On SIGINT (Ctrl-C) or SIGTERM the server stops accepting new
    /// connections and drains in-flight requests before returning.
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = build_router(self.state);

        let listener = TcpListener::bind(self.addr).await?;
        info!("MyHouse backend listening on {}", self.addr);

        // `with_connect_info` exposes the real peer `SocketAddr` to
        // extractors (e.g. the rate-limit middleware's `ConnectInfo`).
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await?;

        info!("Server shut down gracefully");
        Ok(())
    }
}

/// Waits for either SIGINT (Ctrl-C) or SIGTERM, whichever comes first.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install SIGINT handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => info!("Received SIGINT, shutting down…"),
        () = terminate => info!("Received SIGTERM, shutting down…"),
    }
}
