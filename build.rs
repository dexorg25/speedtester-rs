use autotools::Config;
use color_eyre::Report;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    // Compile iperf as a static lib, could probably pass some parameters to prune this
    let dst = Config::new("extern/iperf")
        .disable_shared()
        .enable_static()
        .config_option("with-ssl", None)
        .build();

    // Explicitly search library directory
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Add static library to link list
    println!("cargo:rustc-link-lib=static=iperf");

    Ok(())
}
