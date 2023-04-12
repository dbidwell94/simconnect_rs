use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rustc-link-lib=SimConnect");
    println!(r#"cargo:rustc-link-search=C:\MSFS SDK\SimConnect SDK\lib"#);

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("SimConnect_Open")
        .allowlist_function("SimConnect_Close")
        .allowlist_function("SimConnect_CreateClientData")
        .allowlist_function("SimConnect_MapClientDataNameToID")
        .allowlist_function("SimConnect_CallDispatch")
        .allowlist_var("SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    std::fs::copy("./SimConnect.dll", out_path.join("SimConnect.dll"))
        .expect("Unable to copy SimConnect.dll to output directory");
}
