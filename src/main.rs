use color_eyre::{eyre::eyre, Report};
use tracing_subscriber::EnvFilter;

use clap::Parser;

use tracing::{error, info};

mod iperf;
use iperf::IperfTest;

use crate::iperf::TestResults;

use postgres::{Client, NoTls};

use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
struct Config {
    /// Interval at which to run tests (in minutes)
    #[clap(short, default_value = "5", env = "INTERVAL")]
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

    info!("Running a speed test!");

    // I'll use this eventually to set up the timing loop
    let args = Config::parse();

    // Assume the test takes 10 seconds, and the DB update takes no time.
    // THis is the interval to sleep to
    let interval_seconds = args.interval * 60. - 10.;

    if interval_seconds <= 0. {
        return Err(eyre!("Time interval MUST be greater than 10 seconds!"));
    }

    let configstr = format!(
        "host={} user={} password={} dbname=misc_stats",
        args.dbhost, args.dbuser, args.dbpass
    );

    let mut client = Client::connect(&configstr, NoTls)?;

    // Make the table if it isn't there
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS packet_loss_ts (
            ts      timestamptz PRIMARY KEY DEFAULT NOW(),
            loss    float8
        )
    ",
    )?;

    // Run the test in a loop with a rate limit and a database insertion
    loop {
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
            Ok(TestResults { end, .. }) => {
                // Perform the test and print the result
                let iperf::IperfEndStream::udp { lost_percent, .. } = end.streams[0];
                info!("Speed test done, upstream packet loss is {lost_percent}%");

                // Send the result to a database (this will panic if the client is disconnected? that may not be a problem)
                client.execute(
                    "INSERT INTO packet_loss_ts VALUES (now(), $1)",
                    &[&lost_percent],
                )?;
            }
            Err(e) => {
                error!("Test error with {e}");
            }
        }

        // Sleep the thread until it's time to run again
        sleep(Duration::from_secs_f32(interval_seconds));
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
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
