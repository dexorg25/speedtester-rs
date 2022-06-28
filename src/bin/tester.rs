use color_eyre::{Report, Result};
use hyper::Body;
use tracing_subscriber::EnvFilter;

use clap::Parser;

use tracing::{debug, error, info, warn};

use speedtester_rs::api::{TestRequest, TestReservation};
use std::thread::sleep;
use std::time::{Duration, Instant};

use axum::http::Request;
use hyper_tls::HttpsConnector;

#[derive(Parser)]
struct Config {
    #[clap(env = "TEST_HOST")]
    test_host: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    use axum::http;
    setup()?;

    // Argument parsing defined by `Config`
    let args = Config::parse();

    let test_host = args.test_host;

    let payload: TestRequest = TestRequest {
        client_name: "Hello From here!".into(),
    };
    let payload = serde_json::to_string(&payload)?;
    info!("Sending req body: {payload}");
    let req = Request::builder()
        .method(http::Method::POST)
        .uri(&test_host)
        .header("Content-Type", "application/json")
        .body(Body::from(payload))?;

    let client = hyper::Client::builder().build(HttpsConnector::new());

    let resp = client.request(req).await?;

    //TODO! check status before attempting to unwrap the body
    let resp = hyper::body::to_bytes(resp.into_body()).await?;
    let resp: TestReservation = serde_json::from_slice(&resp)?;

    let mut handle = tokio::process::Command::new("iperf3")
        .args(&["-c", "localhost", "-p", &resp.port_number.to_string(), "-J"])
        .spawn()?;

    handle.wait().await?;

    Ok(())
}

// fn connect_db(
//     host: &str,
//     port: u16,
//     database: &str,
//     user: &str,
//     pass: &str,
// ) -> Result<postgres::Client, postgres::Error> {
//     let mut ret = postgres::Config::new()
//         .application_name("speedtester-rs")
//         .host(host)
//         .port(port)
//         .user(user)
//         .password(pass)
//         .dbname(database)
//         .connect_timeout(Duration::from_secs(60))
//         .connect(postgres::NoTls)?;

//     info!("Connected to database {database} at {user}@{host}!");

//     // Make the tables if not there
//     ret.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS packet_loss_ts (
//             ts      timestamptz PRIMARY KEY DEFAULT NOW(),
//             loss    float8
//         );
//         CREATE TABLE IF NOT EXISTS packet_loss_tests (
//             ts      timestamptz PRIMARY KEY DEFAULT NOW(),
//             test    jsonb NOT NULL
//         )
//     ",
//     )?;

//     Ok(ret)
// }

// fn old_main() -> Result<(), Report> {
//     setup()?;

//     // Argument parsing defined by `Config`
//     let args = Config::parse();

//     info!(
//         "Starting speedtester loop on {}-second interval!",
//         args.interval * 60.
//     );

//     if args.interval * 60. <= 10. {
//         warn!("The test usually takes 10-11 seconds, the current configuration will have no dead time!!");
//     }

//     // Track the number of retries
//     let mut db_retry_counter: u64 = 0;

//     // DB connect retry loop
//     loop {
//         // On successful connect, reset retry counter, else decrement
//         match connect_db(
//             &args.dbhost,
//             args.dbport,
//             &args.database,
//             &args.dbuser,
//             &args.dbpass,
//         ) {
//             Ok(mut client) => {
//                 db_retry_counter = 0;

//                 // Initialize test timer
//                 let mut last_test_time = Instant::now();

//                 // test loop, while DB client is valid
//                 loop {
//                     match client.is_valid(Duration::from_secs(5)) {
//                         Ok(()) => {
//                             // Schedule next test from last_test_time (which should theoretically be about Instant::now() unless we fell behind)
//                             let next_test_time =
//                                 last_test_time + Duration::from_secs_f32(args.interval * 60.);

//                             // Construct argv for iperf with argument 0 cribbed from our argv[0], and others to define the test to run
//                             let mut test = IperfTest::new_from_arguments([
//                                 &args().next().unwrap(),
//                                 "-c",
//                                 &args.iperf_host,
//                                 "-p",
//                                 &args.iperf_port.to_string(),
//                                 "-u",
//                                 "--logfile",
//                                 "/dev/null",
//                             ])?;

//                             // Only keep the ending stats around
//                             match test.run_client() {
//                                 Ok(results) => {
//                                     let end = &results.end;

//                                     // Perform the test and print the result
//                                     let IperfEndStream::udp { lost_percent, .. } = &end.streams[0];
//                                     debug!(
//                                         "Speed test done, upstream packet loss is {lost_percent}%"
//                                     );

//                                     // Send the result to a database (this will panic if the client is disconnected? that may not be a problem)
//                                     client.execute(
//                                         "INSERT INTO packet_loss_ts (loss) VALUES ($1)",
//                                         &[&lost_percent],
//                                     )?;

//                                     // Upload the full log too
//                                     client.execute(
//                                         "INSERT INTO packet_loss_tests (test) VALUES ($1)",
//                                         &[&postgres::types::Json(results)],
//                                     )?;
//                                 }
//                                 Err(e) => {
//                                     error!("iPerf error {e}");

//                                     // Insert a null to signify a failed test
//                                     client.execute(
//                                         "INSERT INTO packet_loss_ts (loss) VALUES (NULL)",
//                                         &[],
//                                     )?;
//                                 }
//                             }

//                             // Sleep if the next scheduled test is in the future
//                             let now = Instant::now();
//                             if now < next_test_time {
//                                 // Sleep the thread until it's time to run again
//                                 sleep(next_test_time - now);
//                             } // else continue the loop immediately

//                             // last_test_time should be about Instant::now() after wake (or earlier), roll it over to last_test_time
//                             last_test_time = next_test_time;
//                         }
//                         Err(e) => {
//                             error!("Invalid client: {e}, retry connection...");
//                             break; // Raise this to the DB reconnect loop, since most reasons a client would be invalid require a reconnect
//                         }
//                     }
//                 }
//             }
//             Err(e) => {
//                 db_retry_counter += 1;
//                 error!("Failed to connect to database with error {e}, retrying for the {db_retry_counter}th time...");
//                 sleep(Duration::from_secs(1)); // throttle retries some, this should keep this counter from ever rolling
//             }
//         }
//     }
// }

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
