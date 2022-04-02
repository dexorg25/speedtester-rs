use color_eyre::Report;
use speedtester_rs::IperfTest;
/// A test to construct a server and client and point them st each other
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn end_to_end() -> Result<(), Report> {
    color_eyre::install()?;

    // Construct a server for a oneshot run, pop it into a thread
    let server = thread::spawn(|| {
        // Configure as server, port 9999 on loopback (should be free...)
        let mut test_server = IperfTest::new_from_arguments([
            "bin",
            "-s",
            "-p",
            "9999",
            "-1",
            "--logfile",
            "/dev/null",
        ])
        .expect("Could not create server");

        test_server.run_server().unwrap();
    });

    // Client also runs in a thread
    let client = thread::spawn(|| {
        // Give the server a moment to start up
        sleep(Duration::from_millis(1000));

        // construct a client pointed at that server, run synchronous on this thread
        let mut test_client = IperfTest::new_from_arguments([
            "bin",
            "-u",
            "-c",
            "127.0.0.1",
            "-p",
            "9999",
            "-t",
            "1",
            "--logfile",
            "/dev/null",
        ])
        .unwrap();

        // execute the client (validates output internally)
        test_client.run_client().unwrap();
    });

    // join the server thread (should exit pretty quick)
    client.join().unwrap();
    server.join().unwrap();

    Ok(())
}
