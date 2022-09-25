use axum::{http::StatusCode, routing::post, Extension, Json, Router, Server};
use color_eyre::{eyre::eyre, Report, Result};
use iperf3::TestResults;
use rand::prelude::*;
use speedtester_server::{TestRequest, TestReservation};

use sqlx::{Pool, Postgres};
use std::{
    collections::HashSet,
    future::Future,
    marker::Send,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::OwnedSemaphorePermit;
use tokio::sync::Semaphore;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

use clap::Parser;

// Constants
const MAX_CONCURRENT_TESTS: usize = 10;

struct State {
    active_ports: Arc<Mutex<HashSet<u16>>>,
    test_counter: Arc<Semaphore>,
    db_pool: Pool<Postgres>,
}

/// RAII based test permit to manage concurrency limit, and port collision mechanism
#[derive(Debug)]
struct TestPermit {
    _inner_permit: OwnedSemaphorePermit,
    ports: Arc<Mutex<HashSet<u16>>>,
    port_number: u16,
}

impl TestPermit {
    // Get a random port from the pool that's not already in use, and return it with the semaphore guard
    async fn new(sem: Arc<Semaphore>, ports: Arc<Mutex<HashSet<u16>>>) -> Result<Self> {
        // Small loop to generate a new, free port for iperf to bind to
        let port_number = {
            loop {
                // Would share this but it's not cool to hold across .await??? I need a nicer random
                let port = thread_rng().gen_range(5000..6000);
                // Only try to use a port if it passes the in_use test first
                // This test is necessary, if the RNG returns the same number twice in a row, and two permit requests come in for the same port
                //   (say this runs something else after one of these awaits and that something else is another test request)
                //   then if there were no hash set check, two test requests could be created with the same port number and one of the iperfs might fail
                if !port_in_use(port).await {
                    let mut ports_guard = ports
                        .lock()
                        .map_err(|_| eyre!("Other worker panicked with mutex"))?;
                    if !ports_guard.contains(&port) {
                        ports_guard.insert(port);
                        break port;
                    }
                }
            }
        };
        // Iperf concurrency limit is ensured here
        Ok(Self {
            _inner_permit: sem
                .acquire_owned()
                .await
                .map_err(|_| eyre!("Semaphore closed but requests still being serviced"))?,
            ports,
            port_number,
        })
    }

    /// Fork a test onto a thread, which will drop the current permit when the test server exits
    async fn execute_test<T: Future<Output = ()> + Send>(
        self,
        output: impl FnOnce(TestResults) -> T + Send + 'static,
    ) {
        tokio::spawn(async move {
            debug!("Start iperf server");
            // Start as server, one-off, with port, JSON output (in case client wants it maybe), and timeout if no one dials in in 10 seconds
            match tokio::process::Command::new("iperf3")
                .args(&[
                    "-s",
                    "-p",
                    &self.port_number.to_string(),
                    "-1",
                    "--idle-timeout",
                    "10",
                    "-J",
                ])
                .output()
                .await
            {
                Ok(iperf_result) => {
                    if iperf_result.status.success() {
                        match serde_json::from_slice(&iperf_result.stdout) {
                            Ok(report) => {
                                debug!("Call test result handler");
                                output(report).await;
                            }
                            Err(parse_error) => {
                                let parse_error = parse_error.to_string();
                                error!(parse_error);
                                debug!(
                                    failed_parse =
                                        String::from_utf8_lossy(&iperf_result.stdout).to_string(),
                                );
                            }
                        }
                    } else {
                        let out_str = String::from_utf8_lossy(&iperf_result.stdout);
                        iperf_result.status.code().map_or_else(
                            || {
                                error!("Iperf killed by signal");
                            },
                            |exit_code| {
                                error!(exit_code, output = out_str.to_string());
                            },
                        );
                    }
                }
                Err(e) => error!(exec_error = e.to_string()),
            }
        });
    }
}

impl Drop for TestPermit {
    fn drop(&mut self) {
        if let Ok(mut ports) = self.ports.lock() {
            if !ports.remove(&self.port_number) {
                error!(
                    "Attempted to drop port {} but it was not in the list?",
                    self.port_number
                );
            }
        } else {
            error!("Ports mutex held through panic");
        }
    }
}

#[derive(Parser)]
struct Config {
    #[clap(env = "DATABASE_URL")]
    db_url: String,

    #[clap(default_value = "0.0.0.0:8080", env = "HOST_ADDR")]
    api_address: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    let cfg = Config::parse();

    // Setup application state
    let state = Arc::new(State {
        active_ports: Arc::new(Mutex::new(HashSet::with_capacity(MAX_CONCURRENT_TESTS))),
        test_counter: Arc::new(Semaphore::new(MAX_CONCURRENT_TESTS)),
        db_pool: sqlx::postgres::PgPool::connect(&cfg.db_url).await?,
    });

    // Apply migrations
    sqlx::migrate!().run(&state.db_pool).await?;

    //TODO: Auth layer
    let app = Router::new()
        .route("/api/v1/newtest", post(new_test))
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

#[axum_macros::debug_handler]
async fn new_test(
    Json(request): Json<TestRequest>,
    Extension(state): Extension<Arc<State>>,
) -> axum::response::Result<Json<TestReservation>, StatusCode> {
    debug!("Client info: {request:#?}");

    // Acquire a test permit (this also ensures ports do not collide)
    let test_permit = TestPermit::new(state.test_counter.clone(), state.active_ports.clone())
        .await
        .map_err(|e| {
            error!("Creating test permit failed with error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let port = test_permit.port_number;

    // Spawns iperf server into a thread with callback to act on the test report
    test_permit
        .execute_test(|r| async move {
            if let Ok(report) = serde_json::to_value(r) {
                match sqlx::query!("INSERT INTO packet_loss_tests (test) VALUES ($1)", report)
                    .execute(&state.db_pool)
                    .await
                {
                    Ok(_) => {}
                    Err(query_error) => {
                        error!("{query_error}");
                    }
                }
            }
        })
        .await;

    // The client must give the server some time to initialize...

    // This assumes that the server launch passed, and further that the server runs correctly
    // given the commands listed above, there will be issues if any of those fail to hold true

    // when test server has been started reply to request with the port number
    Ok(Json(TestReservation { port_number: port }))
}

/// Check if a port is in use by attempting to (and immediately releasing) a bind to said port
async fn port_in_use(port: u16) -> bool {
    match tokio::net::TcpListener::bind(("0.0.0.0", port)).await {
        Ok(_) => false,
        Err(e) => {
            // Complain if it is not the error we expect
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                error!("Port test failed with {e}");
            }
            true
        }
    }
}

//async fn _authenticate() {}

fn setup() -> Result<(), Report> {
    // Load environment from .env if present for dev convenience
    dotenv::dotenv().ok();

    // if std::env::var("RUST_LIB_BACKTRACE").is_err() {
    //     std::env::set_var("RUST_LIB_BACKTRACE", "1")
    // }
    color_eyre::install()?;

    // For now, debug at top level and info for all other modules and crates. Will change to warning later
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
