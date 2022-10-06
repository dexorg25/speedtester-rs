use std::env;
use std::path::PathBuf;

use autotools::Config;

fn vendor_iperf() -> PathBuf {
    // Compile iperf as a static lib, could probably pass some parameters to prune this
    let dst = Config::new("iperf")
        .enable_static()
        .config_option("with-ssl", None)
        .fast_build(true)
        .build();

    // Explicitly search library directory
    // println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Add static library+deps to link list
    // println!("cargo:rustc-link-lib=static=iperf");
    dst
}
#[allow(clippy::unwrap_used)]
fn main() {
    // Openssl included with pkg config (fallback on vendoring from openssl-src)
    if let Ok(_lib) = pkg_config::Config::new()
        .statik(true)
        .atleast_version("3.0.0")
        .probe("openssl")
    {
        // pkg-config crate emits the appropriate directives for cargo on stdout
    } else {
        #[cfg(feature = "allow_vendoring")]
        {
            // Build it from source and then emit messages to link the built lib
            let artifacts = openssl_src::Build::new().build();
            artifacts.print_cargo_metadata();
        }
        #[cfg(not(feature = "allow_vendoring"))]
        {
            panic!("Failed to find OpenSSL. Enable feature allow_vendoring to allow building from source with openssl-src")
        }
    };

    // Generate bindings from comitted iperf_api header
    // Keeping this around rather than comitting bindings, as I might want to include more things later
    let bindings = bindgen::builder();
    bindings
        .header("src/iperf_api.h")
        .allowlist_function("iperf_*|set_protocol|get_protocol")
        .allowlist_type("iperf_*")
        .blocklist_function("iperf_(send|recv|get_test_outfile)")
        .blocklist_type("FILE|fd_set|__sbuf|__sFILE|__sFILEX")
        .clang_arg("-D HAVE_STDINT_H")
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("iperf_bindings.rs"))
        .unwrap();
}
