use color_eyre::{Report, Result};
use iperf3_cli::reports::IperfError;
use iperf3_cli::reports::TestResults;
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

use clap::Parser;

use api::TestReservation;
use core::fmt;
use iperf3_cli as iperf3;
use std::thread::sleep;
use std::{
    io::{Read, Write},
    path::Path,
    time::Duration,
};
use tracing::{debug, error};

use reqwest::Client;
use reqwest::StatusCode;

use uuid::Uuid;

const DEFAULT_CONFIG_PATH: &str = "speedtester_config.toml";

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    /// URL for test host API (reused for iperf endpoint too)
    test_host: String,

    /// Interval at which to run tests (in seconds)
    interval: f32,

    /// Client UUID (new one will be generated if config file is missing)
    client_uuid: Uuid,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            test_host: "http://localhost:8080".to_string(),
            interval: 1.,
            client_uuid: Uuid::new_v4(),
        }
}
}

#[derive(Debug)]
enum SpeedtesterError {
    TestHost(String),
    Io(std::io::Error),
    Parse(serde_json::Error),
    Http(reqwest::Error),
    Config(String),
    IperfFail(String),
}

impl From<std::io::Error> for SpeedtesterError {
    fn from(src: std::io::Error) -> Self {
        Self::Io(src)
    }
}

impl From<serde_json::Error> for SpeedtesterError {
    fn from(src: serde_json::Error) -> Self {
        Self::Parse(src)
    }
}

impl From<reqwest::Error> for SpeedtesterError {
    fn from(src: reqwest::Error) -> Self {
        Self::Http(src)
    }
}

impl fmt::Display for SpeedtesterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TestHost(err) => write!(f, "Test Host replied with '{err}'"),
            Self::Io(err) => write!(f, "IO Error: '{err}'"),
            Self::Parse(err) => write!(f, "Parser Error: '{err}'"),
            Self::Http(err) => write!(f, "HTTP Request Error: '{err}'"),
            Self::Config(err) => write!(f, "Configuration Error: '{err}'"),
            Self::IperfFail(_err) => write!(f, "Iperf command failed to complete"),
        }
    }
}

impl std::error::Error for SpeedtesterError {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    setup()?;

    // Configure from a file (create with default values if missing)
    let config = {
        // Path to configuration file
        let config_path = Path::new(DEFAULT_CONFIG_PATH);

        let mut config_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(config_path)?;

        let mut contents = String::new();
        config_file.read_to_string(&mut contents)?;

        // If the file is empty and/or doesn't exist, create a new one with default values
        if contents.is_empty() {
            let config = Config::default();
            config_file.write_all(toml::to_string_pretty(&config)?.as_bytes())?;
            config
        } else {
            toml::from_str::<Config>(&contents)?
        }
    };

    println!("Config: {:?}", config);

    let test_host = config.test_host;

    let test_url = test_host + "/api/v1/newtest";

    // Construct HTTP client once for reuse between tests
    debug!("Create HTTP client");
    let http_client = std::sync::Arc::new(Client::new());

    let mut sleep_timer = tokio::time::interval(Duration::from_secs_f32(config.interval));

    // test driver loop
    loop {
        // Sleep the thread until it's time to run again
        sleep_timer.tick().await;

        debug!("Spawning a test");
        match execute_test(http_client.clone(), &test_url).await {
            Ok(_) => {
                // Given the test passed, there isn't anything to do on this end. Server now handles reporting
            }

            // There are several reasons this might fail, some can be resolved with a retry but other errors will not resolve over time
            // so the app should stop trying
            Err(e) => {
                use SpeedtesterError::{Config, Io};
                match e {
                    Io(_) | Config(_) => {
                        error!("Unrecoverable Error, aborting");
                        break Err(e.into());
                    }
                    _ => {
                        error!("generic recoverable error '{e}', retry");
                    }
                }
            }
        }
    }
}

/// Execute an iperf test against a remote endpoint running the `test_host` binary on the passed-in URL
/// The return will be Ok(..) if:
///  - The remote host accepted our test request
///  - The local iperf invocation returned _something_
///  - Whatever the local iperf tool returned parsed without error
///
/// Important note here, that an Ok return does not necessarily mean that the test succeeded, only that it was very likely to. The iperf test report should be parsed for status
async fn execute_test(
    client: std::sync::Arc<Client>,
    test_host: &str,
) -> Result<(), SpeedtesterError> {
    debug!("Request test reservation");
    let http_resp = client
        .post(test_host)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    // Get the host out of the test_host URL
    if let Some(iperf_host) = http_resp.url().host() {
        let iperf_host = iperf_host.to_string();
        if http_resp.status() == StatusCode::OK {
            let resp: TestReservation = http_resp.json().await?;
            debug!(iperf_host, resp.port_number);

            let result = tokio::task::spawn_blocking(move || -> Result<TestResults, IperfError> {
                // Wait one second here for test host to get it's server up, then call iperf3 client
                sleep(Duration::from_secs(1));

                // Run the client, parsing and returning result struct
                Ok(iperf3::test_udp_client(&iperf_host, resp.port_number))
            });

            (result.await).map_or_else(
                |_| {
                    Err(SpeedtesterError::IperfFail(
                        "Iperf thread panic!".to_owned(),
                    ))
                },
                |res| match res {
                    Ok(_) => Ok(()),
                    Err(e) => Err(SpeedtesterError::IperfFail(format!("{e}"))),
                },
            )
        } else {
            error!(test_host_response = ?http_resp);
            Err(SpeedtesterError::TestHost(http_resp.text().await?))
        }
    } else {
        Err(SpeedtesterError::Config("URL Has no valid host".into()))
    }
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
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
