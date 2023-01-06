use anyhow::Error;

mod app;
mod ops;
mod typescript;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting logging");

    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("rust_js", log::LevelFilter::Debug)
        .init();

    log::info!("Starting app");

    let mut app = app::App::new().await?;

    app.start().await
}
