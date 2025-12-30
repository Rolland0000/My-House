use backend::Settings;
use backend::startup::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber with environment filter
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting application");

    let setting = Settings::get_configs().expect("can't read config");
    tracing::info!("Configuration loaded successfully");

    let (app, listener) = serve_builder(setting).await;
    
    let addr = listener.local_addr()?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;
    
    tracing::info!("Application shutting down");
    Ok(())
}
