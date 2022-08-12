use color_eyre::{Report, Result};
use speedtester_rs::iperf_reports::TestResults;
use tracing_subscriber::EnvFilter;

use clap::Parser;

use tracing::{debug, error, info, warn};

use core::fmt;
use speedtester_rs::api::{TestRequest, TestReservation};
use std::thread::sleep;
use std::time::{Duration, Instant};

use reqwest::Client;
use reqwest::StatusCode;

#[derive(Parser)]
struct Config {
    #[clap(env = "TEST_HOST")]
    test_host: String,
    /// Interval at which to run tests (in m&inutes)
    #[clap(short, default_value = "1", env = "INTERVAL")]
    interval: f32,

    /// Database name
    #[clap(env = "DBNAME")]
    database: String,

    /// Database Host  
    #[clap(env = "DBHOST")]
    dbhost: String,

    /// Database User
    #[clap(env = "DBUSER")]
    dbuser: String,

    /// Database password
    #[clap(default_value = "", env = "DBPASS")]
    dbpass: String,

    /// Database port
    #[clap(default_value = "5432", env = "DBPORT")]
    dbport: u16,

    /// Maximum DB retries
    #[clap(default_value = "0", env = "MAX_DB_RETRIES")]
    max_db_retries: u32,
}

#[derive(Debug, Clone)]
enum SpeedtesterError {
    TestHostError(String),
}

impl fmt::Display for SpeedtesterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TestHostError(err) => write!(f, "Test Host replied with '{err}'"),
        }
    }
}

impl std::error::Error for SpeedtesterError {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    setup()?;

    // Argument parsing defined by `Config`
    let args = Config::parse();

    let test_host = args.test_host;

    let test_url = test_host + "/api/v1/newtest";
    let mut db_retry_counter: u32 = 0;

    // Construct HTTP client once for reuse between tests
    debug!("Create HTTP client");
    let http_client = Client::new();

    loop {
        // On successful connect, reset retry counter, else decrement
        match connect_db(
            &args.dbhost,
            args.dbport,
            &args.database,
            &args.dbuser,
            &args.dbpass,
        )
        .await
        {
            Ok(client) => {
                db_retry_counter = 0;

                // Initialize test timer
                let mut last_test_time = Instant::now();

                // test driver loop
                loop {
                    // Schedule next test from last_test_time (which should theoretically be about Instant::now() unless we fell behind)
                    let next_test_time =
                        last_test_time + Duration::from_secs_f32(args.interval * 60.);

                    // Execute the test, and if the process passed save the result
                    match execute_test(&http_client, &test_url).await {
                        Ok(report) => {
                            // Upload the full log, this binary is not concerned with much of anything in it beyond validation of the general structure
                            if client
                                .execute(
                                    "INSERT INTO packet_loss_tests_v2 (test) VALUES ($1)",
                                    &[&tokio_postgres::types::Json(report)],
                                )
                                .await
                                .is_err()
                            {
                                break; // if the INSERT fails then reconnect
                            }
                        }
                        Err(e) => error!("Test failed to start with error {e}"),
                    }

                    // Sleep if the next scheduled test is in the future
                    let now = Instant::now();
                    if now < next_test_time {
                        // Sleep the thread until it's time to run again
                        sleep(next_test_time - now);
                    } // else continue the loop immediately

                    // last_test_time should be about Instant::now() after wake (or earlier), roll it over to last_test_time
                    last_test_time = next_test_time;
                }
            }
            Err(e) => {
                // Retry if retries are 0 or we have not exceeded the total
                if (args.max_db_retries == 0) || (db_retry_counter <= args.max_db_retries) {
                    db_retry_counter += 1;
                    error!("Failed to connect to database with error {e}, retrying for the {db_retry_counter}th time...");
                    sleep(Duration::from_secs(1)); // throttle retries some, this should keep this counter from ever rolling
                } else {
                    error!("DB Max Retries Exceeded");
                    break;
                }
            }
        }
    }

    error!("DB Connect failed");
    Ok(())
}

/// Execute an iperf test against a remote endpoint running the test_host binary on the passed-in URL
/// The return will be Ok(..) if:
///  - The remote host accepted our test request
///  - The local iperf invocation returned _something_
///  - Whatever the local iperf tool returned parsed without error
///
/// Important note here, that an Ok return does not necessarily mean that the test succeeded, only that it was very likely to. The iperf test report should be parsed for status
async fn execute_test(
    client: &Client,
    test_host: &str,
) -> Result<TestResults, Box<dyn std::error::Error>> {
    let payload: TestRequest = TestRequest {
        client_name: "Hello From here!".into(),
    };
    info!("Sending req body: {payload:?}");
    let resp = client
        .post(test_host)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if resp.status() == StatusCode::OK {
        let resp: TestReservation = resp.json().await?;

        debug!("Executing test to localhost at port {}", resp.port_number);

        let handle = tokio::process::Command::new("iperf3")
            .args(&["-c", "localhost", "-p", &resp.port_number.to_string(), "-J"])
            .output()
            .await?;

        let test_report: TestResults = serde_json::from_slice(&handle.stdout).map_err(|e| {
            let out_str = String::from_utf8_lossy(&handle.stdout);
            error!("Failed to parse input report: '{out_str}'");
            e
        })?;

        // Complain about errors, although the full report should be uploaded anyway
        if let Some(msg) = &test_report.error {
            warn!("Iperf test failed with error {}", msg);
        }

        debug!(iperf_report = ?test_report);

        Ok(test_report)
    } else {
        error!(test_host_response = ?resp);
        Err(Box::new(SpeedtesterError::TestHostError(
            resp.text().await?,
        )))
    }
}

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
async fn connect_db(
    host: &str,
    port: u16,
    database: &str,
    user: &str,
    pass: &str,
) -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    let (client, _socket) = tokio_postgres::Config::new()
        .application_name("speedtester-rs")
        .host(host)
        .port(port)
        .user(user)
        .password(pass)
        .dbname(database)
        .connect_timeout(Duration::from_secs(60))
        .connect(tokio_postgres::NoTls)
        .await?;

    info!("Connected to database {database} at {user}@{host}!");

    // Make the tables if not there
    //debug!("Create tables if not exist");
    //client
    //.batch_execute(
    //"
    //CREATE TABLE IF NOT EXISTS packet_loss_ts (
    //ts      timestamptz PRIMARY KEY DEFAULT NOW(),
    //loss    float8
    //);
    //CREATE TABLE IF NOT EXISTS packet_loss_tests (
    //ts      timestamptz PRIMARY KEY DEFAULT NOW(),
    //test    jsonb NOT NULL
    //);
    //CREATE TABLE IF NOT EXISTS packet_loss_tests_v2 (
    //ts      timestamptz PRIMARY KEY DEFAULT NOW(),
    //client_id INET NOT NULL DEFAULT inet_client_addr(),
    //test    jsonb NOT NULL
    //);
    //",
    //)
    //.await?;

    debug!("Creeated tables (or they existed)");

    Ok(client)
}
