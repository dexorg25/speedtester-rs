use color_eyre::Report;
use tracing_subscriber::EnvFilter;

use clap::Parser;

use tracing::{debug, info};

mod iperf;
use iperf::{IperfTest, TestRole};

#[derive(Parser)]
struct Config {
    /// Speed to use for test
    #[clap(short)]
    speed: u32,
}

fn main() -> Result<(), Report> {
    setup()?;

    info!("Running a speed test!");

    let mut test = IperfTest::new();

    test.defaults();
    test.set_verbose(1);
    test.set_test_role(TestRole::Client);
    test.set_test_server_hostname("box.witt.me");

    // test.set_test_omit(3);
    test.set_test_duration(10);
    test.set_test_reporter_interval(1.);
    test.set_test_stats_interval(1.);

    test.run_client()?;

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
