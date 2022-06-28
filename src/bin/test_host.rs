use axum::{
    extract::{Form, Path},
    http::{Response, StatusCode},
    routing::post,
    Extension, Json, Router, Server,
};
use color_eyre::{Report, Result};
use speedtester_rs::api::{TestRequest, TestReservation};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
// rest api

// new test
//- spawn thread
//- create iperf server ctx for one off
//- reply to req with test info
// ensure test is cleaned up

#[derive(Clone)]
struct State {}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    // Setup application state
    // let mut active_ports = Arc::new(Mutex::new(HashSet::new()));

    let app = Router::new()
        .route("/api/v1/newtest", post(new_test))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(State {})),
        );

    info!("Listening on 0.0.0.0:8000");
    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[axum_macros::debug_handler]
async fn new_test(
    Json(request): Json<TestRequest>,
) -> axum::response::Result<Json<TestReservation>, &'static str> {
    info!("Client info: {request:#?}");

    let port: u16 = 1;

    // Spawn iperf server into a thread and wait for it to come up
    let handle = tokio::spawn(spawn_server(1234))
        .await
        .expect("Iperf server start failed!!!");

    // TODO: FInd a way to catch if the server fails to initialize, even if the exec call passes

    // when it is up reply to request with the port number
    Ok(Json(TestReservation { port_number: 1234 }))
}

// Spawn an iperf server and return a handle to it's thread
async fn spawn_server(port: u16) -> Result<tokio::process::Child> {
    // Start as server, one-off, with port, JSON output (in case client wants it maybe), and timeout if no one dials in in 10 seconds
    tokio::process::Command::new("iperf3")
        .args(&[
            "-s",
            "-p",
            &port.to_string(),
            "-1",
            "-J",
            "--idle-timeout",
            "10",
        ])
        .spawn()
        .map_err(|e| e.into())
}

fn setup() -> Result<(), Report> {
    // Load environment from .env if present for dev convenience
    dotenv::dotenv().ok();

    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    // For now, debug at top level and info for all other modules and crates. Will change to warning later
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
