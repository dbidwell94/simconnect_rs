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
        .allowlist_function("SimConnect_SubscribeToSystemEvent")
        .allowlist_function("SimConnect_UnsubscribeFromSystemEvent")
        .allowlist_function("SimConnect_RequestDataOnSimObject.*")
        .allowlist_function("SimConnect_GetNextDispatch")
        .allowlist_function("SimConnect_AddToDataDefinition")
        .allowlist_type("SIMCONNECT_REC")
        .allowlist_type("SIMCONNECT_RECV_.*")
        .allowlist_var("SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED")
        .allowlist_var("SIMCONNECT_RECV_ID_.*")
        .allowlist_var("SIMCONNECT_OBJECT_ID_USER")
        .allowlist_var("SIMCONNECT_PERIOD.*")
        .allowlist_var("SIMCONNECT_DATA_REQUEST.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    std::fs::copy("./SimConnect.dll", out_path.join("SimConnect.dll"))
        .expect("Unable to copy SimConnect.dll to output directory");
}
