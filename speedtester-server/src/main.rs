use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::post,
    Extension, Json, Router, Server,
};
use color_eyre::{eyre::eyre, Report};
use iperf3::TestResults;
use rand::prelude::*;
use speedtester_server::{TestRequest, TestReservation};

use clap::Parser;
use iperf3::IperfTest;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use std::{
    collections::HashSet,
    error::Error,
    marker::Send,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::Semaphore;
use tokio::{sync::OwnedSemaphorePermit, task::JoinHandle};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, log::warn};
use tracing_subscriber::EnvFilter;

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
    async fn new(
        sem: Arc<Semaphore>,
        ports: Arc<Mutex<HashSet<u16>>>,
    ) -> Result<Self, Box<dyn Error>> {
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
    /// It is the callers responsibility to eventually do something with the join handle, and it's contained results
    fn execute_test(self) -> JoinHandle<Result<TestResults, Box<dyn Error + Send + Sync>>> {
        // Simply spin up iperf in a blocking thread. Once done, return results to join handle
        tokio::task::spawn_blocking(move || {
            // sync code here
            debug!("Start iperf server");
            // let mut server = IperfTest::new_from_arguments([
            //     "-s",
            //     "-p",
            //     &self.port_number.to_string(),
            //     "-1",
            //     "--idle-timeout",
            //     "10",
            // ])?;

            let mut server = IperfTest::new()?;

            server.set_role(&iperf3::TestRole::Server);
            server.set_server_port(self.port_number.into());
            server.set_one_off(true);
            server.set_idle_timeout(std::time::Duration::from_secs(10));
            server.set_json_output(true);
            server.set_log_file(std::path::Path::new("/dev/null"));

            // TODO: an iperf sentinel worker that through some scheme notifies us when iperf is up and ready to receive connections

            let test = server.run_server()?;

            Ok(test)
        })
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

    #[clap(default_value = "[::]:8080", env = "HOST_ADDR")]
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
                .layer(Extension(state))
                .layer(axum::middleware::from_fn(authenticate)),
        );

    info!("Listening on {}", cfg.api_address);
    Server::bind(&cfg.api_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Authenticate against the state in the DB, using passed token header
async fn authenticate<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode>
where
    B: Send + Sync,
{
    if let Some(token) = req.headers().get("Authorization") {
        match token.to_str() {
            Ok(token) => {
                warn!("Fake auth passes against token: {token}");
                //TODO: fetch the client object by token from DB and then give it to the next layer
                Ok(next.run(req).await)
            }
            Err(e) => {
                error!("Auth token contained non-ascii chars: '{e}'");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
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

    // Consume the permit, and save the handle to the test's results
    // iperf server returns results only after the test is complete and it has shut down
    let report_future = test_permit.execute_test();

    // The client must give the server some time to initialize...

    // In the meantime hand the future for the report to async worker for eventual insertion to the DB
    tokio::spawn(async move {
        match report_future.await {
            Ok(result) => match result {
                Ok(report) => {
                    // Not much can be done about this from here, just log it
                    if let Err(e) = insert_new_test(report, &state.db_pool).await {
                        error!("DB error {e}");
                    }
                }
                Err(e) => {
                    error!("Iperf execution error: '{e:?}'");
                }
            },
            Err(e) => {
                error!("Join Error: '{e}'");
            }
        }
    });

    // This assumes that the server launch passed, and further that the server runs correctly
    // given the commands listed above, there will be issues if any of those fail to hold true

    // when test server has been started reply to request with the port number
    Ok(Json(TestReservation { port_number: port }))
}

async fn insert_new_test(
    test: TestResults,
    db: &Pool<Postgres>,
) -> Result<PgQueryResult, Box<dyn Error>> {
    let test = serde_json::to_value(test)?;
    sqlx::query!("INSERT INTO packet_loss_tests (test) VALUES ($1)", test)
        .execute(db)
        .await
        .map_err(std::convert::Into::into)
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
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    Ok(())
}
