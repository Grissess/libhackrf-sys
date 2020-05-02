extern crate bindgen;
extern crate pkg_config;

fn main() {
    pkg_config::Config::new()
        .atleast_version("0.5")
        .probe("libhackrf")
        .unwrap();

    println!("cargo:rerun-if-changed=wrapper.h");

    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
                .join("bindings.rs")
        )
        .unwrap();
}
