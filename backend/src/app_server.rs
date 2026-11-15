use std::net::SocketAddr;
use std::pin::pin;
use std::time::Duration;

use axum::extract::{ConnectInfo, Request};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper_util::rt::{TokioIo, TokioTimer};
use hyper_util::server::graceful::GracefulShutdown;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceExt;
use tracing::{debug, info, warn};

use crate::app_state::AppState;
use crate::route::build_router;

/// A client must send its complete request headers within this delay, idle
/// keep-alive connections included. `axum::serve` gives hyper no timer, which
/// leaves this timeout inactive and lets slow clients hold sockets forever.
const HEADER_READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Longest wait for in-flight requests once shutdown starts, so one slow
/// connection cannot block a redeploy.
const SHUTDOWN_DRAIN_TIMEOUT: Duration = Duration::from_secs(30);

/// Pause after a failed `accept` (e.g. file descriptor exhaustion), as
/// `axum::serve` does, instead of spinning on the error.
const ACCEPT_ERROR_BACKOFF: Duration = Duration::from_secs(1);

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
    /// Same behaviour as `axum::serve` with connect info (HTTP/1, peer address
    /// for the rate limiter), plus a header read timeout. On SIGINT or SIGTERM
    /// the server stops accepting connections and drains in-flight requests
    /// for at most [`SHUTDOWN_DRAIN_TIMEOUT`].
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = build_router(self.state);

        let listener = TcpListener::bind(self.addr).await?;
        info!("MyHouse backend listening on {}", self.addr);

        let mut http = http1::Builder::new();
        http.timer(TokioTimer::new())
            .header_read_timeout(HEADER_READ_TIMEOUT);

        let graceful = GracefulShutdown::new();
        let mut shutdown = pin!(shutdown_signal());

        loop {
            let (stream, peer) = tokio::select! {
                accepted = listener.accept() => match accepted {
                    Ok(connection) => connection,
                    Err(error) => {
                        warn!(error = %error, "failed to accept connection");
                        tokio::time::sleep(ACCEPT_ERROR_BACKOFF).await;
                        continue;
                    }
                },
                () = &mut shutdown => break,
            };

            // What `into_make_service_with_connect_info` did: the rate-limit
            // middleware reads the peer address from `ConnectInfo`.
            let service = app
                .clone()
                .map_request(move |mut request: Request<Incoming>| {
                    request.extensions_mut().insert(ConnectInfo(peer));
                    request
                });
            let connection =
                http.serve_connection(TokioIo::new(stream), TowerToHyperService::new(service));
            let connection = graceful.watch(connection);

            tokio::spawn(async move {
                if let Err(error) = connection.await {
                    debug!(error = %error, "connection closed with an error");
                }
            });
        }

        drop(listener);
        if tokio::time::timeout(SHUTDOWN_DRAIN_TIMEOUT, graceful.shutdown())
            .await
            .is_err()
        {
            warn!("shutdown drain timed out; closing the remaining connections");
        }

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
