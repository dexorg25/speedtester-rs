use autotools::Config;

fn main() {
    // Compile iperf as a static lib, could probably pass some parameters to prune this
    let dst = Config::new("iperf")
        .disable_shared()
        .enable_static()
        // .config_option("with-ssl", None)
        .fast_build(true)
        .build();

    // Explicitly search library directory
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Add static library to link list
    println!("cargo:rustc-link-lib=static=iperf");
}
