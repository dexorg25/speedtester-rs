use axum::{routing::post, Extension, Router, Server};
use color_eyre::Report;

use clap::Parser;
use sqlx::PgPool;
use std::net::SocketAddr;

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod transport_tests;
use transport_tests::packet_loss;

#[derive(Parser)]
struct Config {
    #[arg(env = "DATABASE_URL")]
    db_url: String,

    #[arg(default_value = "[::]:8080", env = "HOST_ADDR")]
    api_address: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    let cfg = Config::parse();

    let db_pool = PgPool::connect(&cfg.db_url).await?;

    // Setup application state
    let state = packet_loss::State::new(db_pool.clone());

    // Apply migrations
    sqlx::migrate!().run(&db_pool).await?;

    //No auth layer used, wireguard secures it
    let app = Router::new()
        .route("/api/v1/newtest", post(packet_loss::new_test))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(state)),
        );

    info!("Listening on {}", cfg.api_address);
    Server::bind(&cfg.api_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn setup() -> Result<(), Report> {
    // Load environment from .env if present for dev convenience
    dotenvy::dotenv().ok();

    // if std::env::var("RUST_LIB_BACKTRACE").is_err() {
    //     std::env::set_var("RUST_LIB_BACKTRACE", "1")
    // }
    color_eyre::install()?;

    // For now, debug at top level and info for all other modules and crates. Will change to warning later
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    Ok(())
}
