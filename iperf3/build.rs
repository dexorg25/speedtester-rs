use autotools::Config;

#[allow(clippy::unwrap_used)]
fn main() {
    // Openssl included with pkg config
    system_deps::Config::new().probe().unwrap();

    // Compile iperf as a static lib, could probably pass some parameters to prune this
    let dst = Config::new("iperf")
        .enable_static()
        .config_option("with-ssl", None)
        .fast_build(true)
        .build();

    // Explicitly search library directory
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Add static library+deps to link list
    println!("cargo:rustc-link-lib=static=iperf");
}
