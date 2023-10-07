use api::TestReservation;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Extension, Json, Router, Server,
};
use color_eyre::{eyre::eyre, Report};

use clap::Parser;
use sqlx::{postgres::PgQueryResult, query, Pool, Postgres};
use std::{
    collections::HashSet,
    error::Error,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use tokio::{sync::OwnedSemaphorePermit, task::JoinHandle};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

// Constants
const MAX_CONCURRENT_TESTS: usize = 10;

#[derive(Clone)]
pub struct State {
    active_ports: Arc<Mutex<HashSet<u16>>>,
    test_counter: Arc<Semaphore>,
    db_pool: Pool<Postgres>,
}

impl State {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self {
            active_ports: Arc::new(Mutex::new(HashSet::with_capacity(MAX_CONCURRENT_TESTS))),
            test_counter: Arc::new(Semaphore::new(MAX_CONCURRENT_TESTS)),
            db_pool,
        }
    }
}

#[axum_macros::debug_handler]
pub async fn new_test(
    Extension(state): Extension<Arc<State>>,
    req: Request<Body>,
) -> axum::response::Result<Json<TestReservation>, StatusCode> {
    // Extract auth token from request header
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Acquire a test permit (this also enforces concurrency limit, auth, and any other rules)
    let test_permit = TestPermit::new(&state, token).await.map_err(|e| match e {
        TestPermitError::Unauthorized => {
            info!("Unauthorized User!");
            StatusCode::UNAUTHORIZED
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    let port = test_permit.port_number;

    // Consume the permit, and save the handle to the test's results
    // iperf server returns results only after the test is complete and it has shut down
    let report_future = test_permit.execute_test();

    // The client must give the server some time to initialize...

    // In the meantime hand the future for the report to async worker for eventual insertion to the DB
    // tokio::spawn(async move {
    //     match report_future.await {
    //         Ok(result) => match result {
    //             Ok(report) => {
    //                 // Not much can be done about this from here, just log it
    //                 if let Err(e) = insert_new_test(report, &state.db_pool).await {
    //                     error!("DB error {e}");
    //                 }
    //             }
    //             Err(e) => {
    //                 error!("Iperf execution error: '{e:?}'");
    //             }
    //         },
    //         Err(e) => {
    //             error!("Join Error: '{e}'");
    //         }
    //     }
    // });

    // This assumes that the server launch passed, and further that the server runs correctly
    // given the commands listed above, there will be issues if any of those fail to hold true

    // when test server has been started reply to request with the port number
    Ok(Json(TestReservation { port_number: port }))
}

/// RAII based test permit to manage concurrency limit, and port collision mechanism
#[derive(Debug)]
struct TestPermit {
    _inner_permit: OwnedSemaphorePermit,
    ports: Arc<Mutex<HashSet<u16>>>,
    port_number: u16,
    port_listener: Option<TcpListener>,
    client_id: Uuid,
}

#[derive(Debug)]
pub enum TestPermitError {
    Unauthorized,
    DbError(Box<dyn Error>),
    MutexPoisoned,
    SemaphorePoisoned,
    IoError(Box<dyn Error>),
}

impl TestPermit {
    // Get a random port from the pool that's not already in use, and return it with the semaphore guard
    async fn new(state: &State, token: &str) -> Result<Self, TestPermitError> {
        // perform authentication by searching for a matching user record
        // If no matching record is found then that is considered an auth failure
        // If some other DB error happened, preserve it for logging if desired. Caller should error 500
        let record = query!(
            "SELECT * FROM registered_clients WHERE client_token = $1;",
            token
        )
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => TestPermitError::Unauthorized,
            _ => TestPermitError::DbError(Box::new(e)),
        })?
        .ok_or(TestPermitError::Unauthorized)?;

        // Get a port
        let (port_number, port_listener) = {
            match tokio::net::TcpListener::bind(("0.0.0.0", 0)).await {
                Ok(listener) => {
                    let port = listener
                        .local_addr()
                        .map_err(|e| {
                            error!("Failed to get addr: error '{e}'");
                            TestPermitError::IoError(Box::new(e))
                        })?
                        .port();
                    Ok((port, listener))
                }
                Err(e) => {
                    // Complain if we cannot bind
                    error!("Port test failed with {e}");
                    Err(TestPermitError::IoError(Box::new(e)))
                }
            }
        }?;
        // Iperf concurrency limit is ensured here
        Ok(Self {
            _inner_permit: state
                .test_counter
                .clone()
                .acquire_owned()
                .await
                .map_err(|_| TestPermitError::SemaphorePoisoned)?,
            ports: state.active_ports.clone(),
            port_number,
            port_listener: Some(port_listener),
            client_id: record.id,
        })
    }

    /// Fork a test onto a thread, which will drop the current permit when the test server exits
    /// It is the callers responsibility to eventually do something with the join handle, and it's contained results
    fn execute_test(mut self) -> JoinHandle<Result<(), color_eyre::Report>> {
        // Simply spin up iperf in a blocking thread. Once done, return results to join handle
        tokio::task::spawn_blocking(move || {
            // sync code here
            debug!("drop the TCP Listener to release the port");
            self.port_listener.take();
            debug!("Start iperf server");
            // Ok((
            //     self.client_id,
            //     iperf3_cli::test_udp_server(self.port_number),
            // ))
            Err(eyre!("unimplemented"))
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
