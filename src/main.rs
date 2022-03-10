use color_eyre::Report;
use tracing_subscriber::EnvFilter;

use clap::Parser;

use tracing::{debug, info};

mod iperf;
use iperf::IperfTest;

use crate::iperf::TestResults;

#[derive(Parser)]
struct Config {
    /// Speed to use for test
    #[clap(short)]
    speed: u32,

    /// Interval at which to run tests (in minutes)
    #[clap(short)]
    interval: u32,
}

fn main() -> Result<(), Report> {
    setup()?;

    info!("Running a speed test!");

    let args = std::env::args();

    let mut test = IperfTest::new_from_arguments(args)?;

    // test.set_verbose(false);
    // test.set_test_role(TestRole::Client);
    // test.set_test_server_hostname("box.witt.me");

    // test.set_test_duration(10);
    // test.set_test_reporter_interval(1.);
    // test.set_test_stats_interval(1.);

    let output = test.run_client()?;

    info!(
        "Speed test done, output is {:#?}",
        serde_json::to_string(&output)?
    );
    Ok(())
}

fn setup() -> Result<(), Report> {
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
