use anyhow::{anyhow, Result as AnyhowResult};
use semver::Version;
use std::mem::transmute;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

use super::bindings;

trait FromPtr {
    fn from_pointer(data: NonNull<bindings::SIMCONNECT_RECV>) -> AnyhowResult<Self>
    where
        Self: Sized;
}

/* #region RecV Enum */
#[derive(Debug)]
pub enum RecvDataEvent {
    Null,
    Open(RecVOpen),
    Event,
    Data(RecvSimData),
    Quit,
    Exception,
    AirportList,
}

impl RecvDataEvent {
    pub fn from_pointer(data: NonNull<bindings::SIMCONNECT_RECV>) -> AnyhowResult<Self> {
        let data_id = unsafe { *data.as_ptr() }.dwID as i32;
        Ok(match data_id {
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
                Self::Open(RecVOpen::from_pointer(data)?)
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => Self::Event,
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                Self::Data(RecvSimData::from_pointer(data)?)
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA_BYTYPE => {
                Self::Data(RecvSimData::from_pointer(data)?)
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => Self::Null,
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => Self::Exception,
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Self::Quit,
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => Self::AirportList,
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
        let name = String::from_utf8(
            unsafe { transmute::<[i8; 256], [u8; 256]>(open_data.szApplicationName) }.to_vec(),
        )?;

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
    data_id: u32
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

        let data_id = unsafe {*raw_ptr}.dwDefineID;

        let ptr =
            NonNull::new(raw_ptr).ok_or_else(|| anyhow::anyhow!("Unexpected empty pointer"))?;
        Ok(Self {
            data_pointer: Arc::new(Mutex::new(ptr)),
            data_id
        })
    }
}

unsafe impl Send for RecvSimData {}

/* #endregion */
