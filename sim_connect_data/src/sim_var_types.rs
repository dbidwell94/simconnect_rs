use num_enum::FromPrimitive;

pub trait IntoSimVarType {
    fn into_sim_var() -> SimVarType;
}

use sim_connect_sys::bindings;

#[derive(Debug, FromPrimitive)]
#[repr(i32)]
pub enum SimVarType {
    #[default]
    Invalid = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INVALID,
    I32 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT32,
    I64 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT64,
    F32 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT32,
    F64 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
    String8 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING8,
    String32 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING32,
    String64 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING64,
    String128 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING128,
    String256 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING256,
    String260 = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING260,
    StringV = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRINGV,
    InitPos = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INITPOSITION,
    MarkerState = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_MARKERSTATE,
    Waypoint = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_WAYPOINT,
    LatLongAlt = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_LATLONALT,
    XYZ = bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_XYZ,
}

impl IntoSimVarType for i32 {
    fn into_sim_var() -> SimVarType {
        SimVarType::I32
    }
}

impl IntoSimVarType for i64 {
    fn into_sim_var() -> SimVarType {
        SimVarType::I64
    }
}

impl IntoSimVarType for f32 {
    fn into_sim_var() -> SimVarType {
        SimVarType::F32
    }
}

impl IntoSimVarType for f64 {
    fn into_sim_var() -> SimVarType {
        SimVarType::F64
    }
}

impl IntoSimVarType for String {
    fn into_sim_var() -> SimVarType {
        SimVarType::StringV
    }
}
