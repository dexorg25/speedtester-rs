use autotools::Config;

fn main() {
    // Compile iperf as a static lib, could probably pass some parameters to prune this
    let mut cfg = Config::new("extern/iperf");
    cfg.disable_shared()
        .enable_static()
        .config_option("with-ssl", None)
        .fast_build(true);

    let dst = cfg.build();

    // Explicitly search library directory
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Add iperf library to lib target
    println!("cargo:rustc-link-lib=static=iperf");

    // dependence on openssl libcrypto
    println!("cargo:rustc-link-lib=dylib=crypto");
}
