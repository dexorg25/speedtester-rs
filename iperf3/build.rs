use std::env;
use std::path::PathBuf;

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

    // Generate bindings
    let bindings = bindgen::builder();
    bindings
        .header("iperf/src/iperf_api.h")
        .allowlist_function("iperf_*|set_protocol|get_protocol")
        .allowlist_type("iperf_*")
        .allowlist_var("SOCK_STREAM|SOCK_DATAGRAM")
        .blocklist_function("iperf_(send|recv|get_test_outfile)")
        .blocklist_type("FILE|fd_set|__sbuf|__sFILE|__sFILEX")
        .clang_arg("-D HAVE_STDINT_H")
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("iperf_bindings.rs"))
        .unwrap();
}
