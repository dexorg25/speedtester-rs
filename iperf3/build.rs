fn _find_libiperf() {
    //TODO: Fix this up to look for iperf instead of just hacking it for me
    println!("cargo:rustc-link-lib=static=iperf");
    println!("cargo:rustc-link-lib=static=sctp");
}

#[allow(clippy::unwrap_used)]
fn main() {
    // Openssl included with pkg config (fallback on vendoring from openssl-src)
    pkg_config::Config::new()
        .statik(true)
        // .atleast_version("3.0.0")
        .probe("openssl")
        .map_or_else(
            |_| {
                // Build it from source and then emit messages to link the built lib
                let artifacts = openssl_src::Build::new().build();
                artifacts.print_cargo_metadata();
            },
            |_lib| {
                // pkg-config crate emits the appropriate directives for cargo on stdout
            },
        );

    // Try to find iperf lib and if not present, and features allow, build it
    // if find_libiperf().is_err() {
    #[cfg(feature = "allow_vendoring")]
    {
        iperf3_src::vendor_iperf();
    }
    // }
}
