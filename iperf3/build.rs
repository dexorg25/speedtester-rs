use autotools::Config;

#[allow(clippy::unwrap_used)]
fn main() {
    // Openssl include dir passed as env
    let ssl_include = std::env::var_os("DEP_OPENSSL_INCLUDE").unwrap();

    std::env::set_var("LDFLAGS", format!("-L{}", ssl_include.to_str().unwrap()));

    // Compile iperf as a static lib, could probably pass some parameters to prune this
    let dst = Config::new("iperf")
        .enable_static()
        // .config_option("with-ssl", None)
        .fast_build(true)
        .build();

    // Explicitly search library directory
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Add static library to link list
    println!("cargo:rustc-link-lib=static=iperf");
}
