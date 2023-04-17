use std::env;
use std::path::PathBuf;

#[cfg(feature = "static_link")]
const SDK_PATH: &'static str = r"C:\MSFS SDK\SimConnect SDK\lib\static";
#[cfg(not(feature = "static_link"))]
const SDK_PATH: &'static str = r"C:\MSFS SDK\SimConnect SDK\lib";

#[cfg(not(feature = "static_link"))]
const LINK_LIB_ARGS: &'static str = "dylib=SimConnect";
#[cfg(feature = "static_link")]
const LINK_LIB_ARGS: &'static str = "static=SimConnect";

fn main() {
    let sdk_path = std::env::var("SIMCONNECT_SDK").unwrap_or(SDK_PATH.to_owned());
    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rerun-if-env-changed=SIMCONNECT_SDK");
    println!("cargo:rustc-link-lib={LINK_LIB_ARGS}");
    println!("cargo:rustc-link-search={sdk_path}");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("SimConnect.*")
        .allowlist_type("SIMCONNECT.*")
        .allowlist_var("SIMCONNECT.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
