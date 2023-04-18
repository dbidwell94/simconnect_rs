use anyhow::{anyhow, Result as AnyhowResult};
use num_enum::TryFromPrimitive;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::ffi::{c_char, CStr};
use std::mem::transmute;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

use sim_connect_sys::bindings;

use crate::sim_event_args::SimStateArgs;
use crate::sim_events::{SystemEventData, SystemEventDataHolder};

pub trait FromPtr {
    fn from_pointer(data: NonNull<bindings::SIMCONNECT_RECV>) -> AnyhowResult<Self>
    where
        Self: Sized;
}

/* #region RecV Enum */
#[derive(Debug)]
pub enum RecvDataEvent {
    Null,
    Open(RecVOpen),
    Data(RecvSimData),
    Event(SystemEventDataHolder),
    SystemState(RecvSystemState),
    Quit,
}

impl RecvDataEvent {
    pub fn from_pointer(data: NonNull<bindings::SIMCONNECT_RECV>) -> AnyhowResult<Self> {
        let data_id = unsafe { *data.as_ptr() }.dwID as i32;
        Ok(match data_id {
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
                Self::Open(RecVOpen::from_pointer(data)?)
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                Self::Data(RecvSimData::from_pointer(data)?)
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA_BYTYPE => {
                Self::Data(RecvSimData::from_pointer(data)?)
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => Self::Null,
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Self::Quit,
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SYSTEM_STATE => {
                Self::SystemState(RecvSystemState::from_pointer(data)?)
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
                Self::Event(SystemEventDataHolder::from_pointer(data)?)
            }
            _ => Self::Null,
        })
    }
}
/* #endregion */

/* #region RecvOpen */

#[derive(Debug)]
pub struct RecVOpen {
    pub application_name: String,
    pub sim_connect_version: Version,
    pub sim_connect_build_version: Version,
    pub application_version: Version,
    pub application_build_version: Version,
}

impl FromPtr for RecVOpen {
    fn from_pointer(data: NonNull<bindings::SIMCONNECT_RECV>) -> AnyhowResult<Self> {
        let raw_pointer: *mut bindings::SIMCONNECT_RECV_OPEN = unsafe { transmute(data.as_ptr()) };

        let open_data = unsafe { *raw_pointer };

        let name_ptr = &open_data.szApplicationName as *const c_char;
        let name_cstr = unsafe { CStr::from_ptr(name_ptr) };
        let name = name_cstr.to_str()?.to_owned();

        let sim_connect_version = Version::new(
            open_data.dwSimConnectVersionMajor as u64,
            open_data.dwSimConnectVersionMinor as u64,
            0,
        );

        let sim_connect_build_version = Version::new(
            open_data.dwSimConnectBuildMajor as u64,
            open_data.dwSimConnectBuildMinor as u64,
            0,
        );

        let application_version = Version::new(
            open_data.dwApplicationVersionMajor as u64,
            open_data.dwApplicationVersionMinor as u64,
            0,
        );

        let application_build_version = Version::new(
            open_data.dwApplicationBuildMajor as u64,
            open_data.dwApplicationBuildMinor as u64,
            0,
        );

        Ok(Self {
            application_name: name,
            sim_connect_version,
            application_version,
            application_build_version,
            sim_connect_build_version,
        })
    }
}
/* #endregion */

/* #region RecvSimData */
#[derive(Debug)]
pub struct RecvSimData {
    data_pointer: Arc<Mutex<NonNull<bindings::SIMCONNECT_RECV_SIMOBJECT_DATA>>>,
    data_id: u32,
}

impl RecvSimData {
    pub fn to_struct<T: Copy + Clone>(self) -> AnyhowResult<T> {
        let locked = self.data_pointer.lock().unwrap();
        let ptr = unsafe { locked.as_ref() };

        let data = NonNull::new(std::ptr::addr_of!(ptr.dwData) as *mut T)
            .ok_or_else(|| anyhow!("Pointer not expected to be null"))?;

        let data = unsafe { data.as_ref().clone() };

        return Ok(data);
    }

    pub fn get_id(&self) -> u32 {
        self.data_id
    }
}

impl FromPtr for RecvSimData {
    fn from_pointer(data: NonNull<bindings::SIMCONNECT_RECV>) -> AnyhowResult<Self>
    where
        Self: Sized,
    {
        let raw_ptr: *mut bindings::SIMCONNECT_RECV_SIMOBJECT_DATA =
            unsafe { transmute(data.as_ptr()) };

        let data_id = unsafe { *raw_ptr }.dwDefineID;

        let ptr =
            NonNull::new(raw_ptr).ok_or_else(|| anyhow::anyhow!("Unexpected empty pointer"))?;
        Ok(Self {
            data_pointer: Arc::new(Mutex::new(ptr)),
            data_id,
        })
    }
}

unsafe impl Send for RecvSimData {}

/* #endregion */

/* #region RecvSystemEvent */
#[derive(Debug)]
pub struct RecvSystemEvent(SystemEventData);

impl FromPtr for RecvSystemEvent {
    fn from_pointer(data: NonNull<bindings::SIMCONNECT_RECV>) -> AnyhowResult<Self>
    where
        Self: Sized,
    {
        Ok(Self(SystemEventData::from_pointer(data)?))
    }
}

/* #endregion */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecvSystemState {
    pub state_arg: SimStateArgs,
    pub aircraft_loaded: Option<String>,
    pub dialog_mode: Option<bool>,
    pub flight_loaded: Option<String>,
    pub flight_plan: Option<String>,
    pub sim: Option<bool>,
}

unsafe impl Sync for RecvSystemState {}

unsafe impl Send for RecvSystemState {}

impl FromPtr for RecvSystemState {
    fn from_pointer(data: NonNull<bindings::SIMCONNECT_RECV>) -> AnyhowResult<Self>
    where
        Self: Sized,
    {
        let raw_ptr: *mut bindings::SIMCONNECT_RECV_SYSTEM_STATE =
            unsafe { transmute(data.as_ptr()) };

        let system_state = unsafe { *raw_ptr };
        let arg_type = SimStateArgs::try_from_primitive(system_state.dwRequestID)?;
        match arg_type {
            SimStateArgs::AircraftLoaded => {
                let name = unsafe { CStr::from_ptr(&system_state.szString as *const i8) };
                return Ok(Self {
                    aircraft_loaded: Some(name.to_str()?.to_owned()),
                    dialog_mode: None,
                    flight_loaded: None,
                    flight_plan: None,
                    sim: None,
                    state_arg: arg_type,
                });
            }
            SimStateArgs::DialogMode => {
                let dialog_mode_bool: bool = system_state.dwInteger != 0;
                return Ok(Self {
                    aircraft_loaded: None,
                    dialog_mode: Some(dialog_mode_bool),
                    flight_loaded: None,
                    flight_plan: None,
                    sim: None,
                    state_arg: arg_type,
                });
            }
            SimStateArgs::FlightLoaded => {
                let flight_name = unsafe { CStr::from_ptr(&system_state.szString as *const i8) };
                return Ok(Self {
                    aircraft_loaded: None,
                    dialog_mode: None,
                    flight_loaded: Some(flight_name.to_str()?.to_owned()),
                    flight_plan: None,
                    sim: None,
                    state_arg: arg_type,
                });
            }
            SimStateArgs::FlightPlan => {
                let flight_plan_str =
                    unsafe { CStr::from_ptr(&system_state.szString as *const i8) };
                return Ok(Self {
                    aircraft_loaded: None,
                    dialog_mode: None,
                    flight_loaded: None,
                    flight_plan: Some(flight_plan_str.to_str()?.to_owned()),
                    sim: None,
                    state_arg: arg_type,
                });
            }
            SimStateArgs::Sim => {
                let sim_bool: bool = system_state.dwInteger != 0;
                return Ok(Self {
                    aircraft_loaded: None,
                    dialog_mode: None,
                    flight_loaded: None,
                    flight_plan: None,
                    sim: Some(sim_bool),
                    state_arg: arg_type,
                });
            }
        }
    }
}
