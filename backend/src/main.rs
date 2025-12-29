use backend::Settings;
use backend::startup::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let setting = Settings::get_configs().expect("can't read config");

    let (app, listener) = serve_builder(setting).await;

    axum::serve(listener, app).await?;
    Ok(())
}
