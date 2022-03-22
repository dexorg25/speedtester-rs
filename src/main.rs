use color_eyre::{eyre::eyre, Report};
use tracing_subscriber::EnvFilter;

use clap::Parser;

use tracing::{error, info, warn};

mod iperf;
use iperf::IperfTest;

use crate::iperf::TestResults;

use postgres::{types::Type, Client, NoTls};

use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Parser)]
struct Config {
    /// Interval at which to run tests (in minutes)
    #[clap(short, default_value = "1", env = "INTERVAL")]
    interval: f32,

    /// Database address
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
    dbport: u32,
}

fn main() -> Result<(), Report> {
    setup()?;

    // I'll use this eventually to set up the timing loop
    let args = Config::parse();

    info!(
        "Starting speedtester loop on {}-second interval!",
        args.interval * 60.
    );

    if args.interval * 60. <= 10. {
        warn!("The test usually takes 10-11 seconds, the current configuration will have no dead time!!");
    }

    let configstr = format!(
        "host={} user={} password={} dbname=misc_stats",
        args.dbhost, args.dbuser, args.dbpass
    );

    let mut client = Client::connect(&configstr, NoTls)?;

    // Make the tables if not there
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS packet_loss_ts (
            ts      timestamptz PRIMARY KEY DEFAULT NOW(),
            loss    float8
        );
        CREATE TABLE IF NOT EXISTS packet_loss_tests (
            ts      timestamptz PRIMARY KEY DEFAULT NOW(),
            test    jsonb NOT NULL
        )
    ",
    )?;

    // prepare statements for logging
    let save_loss = client.prepare_typed(
        "INSERT INTO packet_loss_ts (loss) VALUES ($1)",
        &[Type::FLOAT8],
    )?;
    let save_output = client.prepare_typed(
        "INSERT INTO packet_loss_tests (test) VALUES ($1)",
        &[Type::JSONB],
    )?;

    // Initialize test timer
    let mut last_test_time = Instant::now();

    // Run the test in a loop with a rate limit and a database insertion
    loop {
        // Schedule next test from last_test_time (which should theoretically be about Instant::now() unless we fell behind)
        let next_test_time = last_test_time + Duration::from_secs_f32(args.interval * 60.);

        // Construct argv for iperf with argument 0 cribbed from our argv[0], and others to define the test to run
        let mut test = IperfTest::new_from_arguments([
            &std::env::args().next().unwrap(),
            "-c",
            "box.witt.me",
            "-u",
            "--logfile",
            "/dev/null",
        ])?;

        // Only keep the ending stats around
        match test.run_client() {
            Ok(results) => {
                let end = &results.end;

                // Perform the test and print the result
                let iperf::IperfEndStream::udp { lost_percent, .. } = &end.streams[0];
                info!("Speed test done, upstream packet loss is {lost_percent}%");

                // Send the result to a database (this will panic if the client is disconnected? that may not be a problem)
                client.execute(&save_loss, &[&lost_percent])?;

                // Upload the full log too
                client.execute(&save_output, &[&postgres::types::Json(results)])?;
            }
            Err(e) => {
                error!("Test error with {e}");

                // Insert a null to signify a failed test
                client.execute("INSERT INTO packet_loss_ts (loss) VALUES (NULL)", &[])?;
            }
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

fn setup() -> Result<(), Report> {
    // Load environment from .env if present for dev convenience
    dotenv::dotenv().ok();

    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
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
