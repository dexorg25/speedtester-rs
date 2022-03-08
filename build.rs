use std::env;
use std::path::PathBuf;

use autotools::Config;
use bindgen::Builder;
use color_eyre::{eyre::eyre, Report};

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    // Compile iperf as a static lib, could probably pass some parameters to prune this
    let dst = Config::new("extern/iperf")
        .disable_shared()
        .enable_static()
        .config_option("without-openssl", None) // disable ssl until I figure out how to link it in
        .build();

    // Explicitly search library directory
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Add static library to link list
    println!("cargo:rustc-link-lib=static=iperf");

    // This depends on openssl, link it
    // println!("cargo:rustc-link-lib=ssl"); // commented out for simplicity, idk exactly how to link this

    // Generate bindings for iperf API
    let bindings = Builder::default()
        .header("extern/iperf/src/iperf_api.h")
        .clang_arg("-DHAVE_STDINT_H") // Ensure API header has the defines it needs
        .generate()
        .map_err(|_| eyre!("Unable to generate bindings"))?;

    // Write the bindings to the $OUT_DIR/iperf_bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("iperf_bindings.rs"))
        .map_err(|_| eyre!("Couldn't write bindings!"))?;

    Ok(())
}
