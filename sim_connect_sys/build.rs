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
    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rustc-link-lib={LINK_LIB_ARGS}");
    println!("cargo:rustc-link-search={}", std::env::var("SIMCONNECT_SDK").unwrap_or(SDK_PATH.to_owned()));

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("SimConnect_Open")
        .allowlist_function("SimConnect_Close")
        .allowlist_function("SimConnect_SubscribeToSystemEvent")
        .allowlist_function("SimConnect_UnsubscribeFromSystemEvent")
        .allowlist_function("SimConnect_RequestDataOnSimObjectType")
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

    #[cfg(not(feature = "static_link"))]
    std::fs::copy(format!("{SDK_PATH}/SimConnect.dll"), out_path.join("SimConnect.dll"))
        .expect("Unable to copy SimConnect.dll to output directory");
}
