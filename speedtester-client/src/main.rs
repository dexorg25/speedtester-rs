use color_eyre::{Report, Result};
use tracing_subscriber::EnvFilter;

use clap::Parser;

use tracing::{debug, error};

use core::fmt;
use speedtester_server::{TestRequest, TestReservation};
use std::thread::sleep;
use std::time::Duration;

use reqwest::Client;
use reqwest::StatusCode;

#[derive(Parser)]
struct Config {
    // URL for test host API (reused for iperf endpoint too)
    #[clap(env = "TEST_HOST")]
    test_host: String,

    /// Authentication token passed in header to access service
    #[clap(env = "API_TOKEN")]
    api_token: String,

    /// Interval at which to run tests (in seconds)
    #[clap(short, default_value = "1", env = "INTERVAL")]
    interval: f32,
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

    // Argument parsing defined by `Config`
    let args = Config::parse();

    let test_host = args.test_host;

    let test_url = test_host + "/api/v1/newtest";

    // Construct HTTP client once for reuse between tests
    debug!("Create HTTP client");
    let http_client = std::sync::Arc::new(Client::new());

    let mut sleep_timer = tokio::time::interval(Duration::from_secs_f32(args.interval));

    // test driver loop
    loop {
        // Sleep the thread until it's time to run again
        sleep_timer.tick().await;

        debug!("Spawning a test");
        match execute_test(http_client.clone(), &test_url, &args.api_token).await {
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
    api_token: &str,
) -> Result<(), SpeedtesterError> {
    let payload: TestRequest = TestRequest {
        client_name: "Hello!".into(),
    };
    debug!("Request test reservation");
    let http_resp = client
        .post(test_host)
        .header("Content-Type", "application/json")
        .header("Authorization", api_token)
        .json(&payload)
        .send()
        .await?;

    // Get the host out of the test_host URL
    if let Some(iperf_host) = http_resp.url().host() {
        let iperf_host = iperf_host.to_string();
        if http_resp.status() == StatusCode::OK {
            let resp: TestReservation = http_resp.json().await?;
            debug!(iperf_host, resp.port_number);

            // Wait one second here for test host to get it's server up, then call iperf3 client
            sleep(Duration::from_secs(1));

            // After iperf is launched this app is no longer concerned with it's operation, the output is not checked (although the status is)
            let res = tokio::process::Command::new("iperf3")
                .args(&["-c", &iperf_host, "-p", &resp.port_number.to_string(), "-u"])
                .output()
                .await?;

            let mut test = iperf3::IperfTest::new()
                .map_err(|_| SpeedtesterError::IperfFail("allocation error".to_string()))?;

            test.set_server_port(resp.port_number.try_into().map_err(|_| {
                SpeedtesterError::IperfFail("Illegal port number passed".to_string())
            })?);
            test.set_test_server_hostname(&iperf_host);
            test.set_protocol

            if res.status.success() {
                Ok(())
            } else {
                error!("iperf exited nonzero");
                let output = String::from_utf8_lossy(&res.stdout).to_string();
                Err(SpeedtesterError::IperfFail(output))
            }
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
    dotenv::dotenv().ok();

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
