use axum::{routing::post, Extension, Json, Router, Server};
use color_eyre::{Report, Result};
use rand::prelude::*;
use speedtester_rs::api::{TestRequest, TestReservation};

use std::{
    collections::HashSet,
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
// new test
//- spawn thread
//- create iperf server ctx for one off
//- reply to req with test info
// ensure test is cleaned up

// Constants
const MAX_CONCURRENT_TESTS: usize = 10;

struct State {
    active_ports: Arc<Mutex<HashSet<u16>>>,
    test_counter: Arc<Semaphore>,
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
    async fn new(sem: Arc<Semaphore>, ports: Arc<Mutex<HashSet<u16>>>) -> Self {
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
                    let mut ports_guard = ports.lock().unwrap();
                    if !ports_guard.contains(&port) {
                        ports_guard.insert(port);
                        break port;
                    }
                }
            }
        };
        // Iperf concurrency limit is ensured here
        Self {
            _inner_permit: sem.acquire_owned().await.unwrap(),
            ports,
            port_number,
        }
    }

    /// Fork a test onto a thread, which will drop the current permit when the test server exits
    async fn execute_test(self) {
        tokio::spawn(async move {
            // Start as server, one-off, with port, JSON output (in case client wants it maybe), and timeout if no one dials in in 10 seconds
            let res = tokio::process::Command::new("iperf3")
                .args(&[
                    "-s",
                    "-p",
                    &self.port_number.to_string(),
                    "-1",
                    "--idle-timeout",
                    "10",
                ])
                .output()
                .await
                .unwrap(); // this only fails if the command failed to spawn for some reason, and the application should exit

            if !res.status.success() {
                let out_str = String::from_utf8_lossy(&res.stdout);
                error!("Iperf server exited '{}', output follows", res.status);
                error!("{out_str}");
            } else {
                debug!("Iperf server exited successfully!, output follows");
                debug!("{}", String::from_utf8_lossy(&res.stdout));
            }
        });
    }
}

impl Drop for TestPermit {
    fn drop(&mut self) {
        if !self
            .ports
            .lock()
            .expect("TestPermit mutex poisoned")
            .remove(&self.port_number)
        {
            error!(
                "Attempted to drop port {} but it was not in the list?",
                self.port_number
            );
        }
    }
}

#[derive(Parser)]
struct Config {
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
    });

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
        .await
        .unwrap();

    Ok(())
}

#[axum_macros::debug_handler]
async fn new_test(
    Json(request): Json<TestRequest>,
    Extension(state): Extension<Arc<State>>,
) -> axum::response::Result<Json<TestReservation>, &'static str> {
    debug!("Client info: {request:#?}");

    // Acquire a test permit (this also ensures ports do not collide)
    let test_permit = TestPermit::new(state.test_counter.clone(), state.active_ports.clone()).await;
    let port = test_permit.port_number;

    // Spawn iperf server into a thread
    test_permit.execute_test().await;

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
            match e.kind() {
                // std::io::ErrorKind::AddrInUse => todo!(),
                std::io::ErrorKind::AlreadyExists => true,
                _ => {
                    error!("Port test failed with {e}");
                    true
                }
            }
        }
    }
}

async fn _authenticate() {}

fn setup() -> Result<(), Report> {
    // Load environment from .env if present for dev convenience
    dotenv::dotenv().ok();

    // if std::env::var("RUST_LIB_BACKTRACE").is_err() {
    //     std::env::set_var("RUST_LIB_BACKTRACE", "1")
    // }
    color_eyre::install()?;

    // For now, debug at top level and info for all other modules and crates. Will change to warning later
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
