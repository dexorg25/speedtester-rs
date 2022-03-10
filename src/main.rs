use color_eyre::Report;
use tracing_subscriber::EnvFilter;

use clap::Parser;

use tracing::info;

mod iperf;
use iperf::IperfTest;

use crate::iperf::TestResults;

#[derive(Parser)]
struct Config {
    /// Interval at which to run tests (in minutes)
    #[clap(short, default_value = "5")]
    interval: u32,
}

fn main() -> Result<(), Report> {
    setup()?;

    info!("Running a speed test!");

    // I'll use this eventually to set up the timing loop
    let _args = Config::parse();

    // Construct argv for iperf with argument 0 cribbed from our argv[0], and others to define the test to run
    let mut test = IperfTest::new_from_arguments([&std::env::args().next().unwrap(), "-c", "box.witt.me", "-u", "--logfile", "/dev/null"])?;

    // Only keep the ending stats around
    let TestResults { end, .. } = test.run_client()?;

    let iperf::IperfEndStream::udp { lost_percent, .. } = end.streams[0];
    info!("Speed test done, upstream packet loss is {lost_percent}%");

    todo!("Send that figure up to timescaledb")
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
