use self::sim_events::SystemEvent;

use super::PROGRAM_NAME;
use anyhow::{anyhow, Result as AnyhowResult};
use std::ffi::{c_void, CStr, CString};

mod bindings;
pub mod sim_events;
pub mod sim_vars;

pub trait ToSimConnect {
    fn sc_string(&self) -> CString;
}

macro_rules! check_hr {
    ($hr: expr) => {
        let hr = $hr;
        if hr != 0 {
            return Err(anyhow!(format!(
                "HRESULT indicates error: 0x{:x}",
                hr as u32
            )));
        }
    };
}

pub struct SimConnect {
    handle: std::ptr::NonNull<c_void>,
}

impl SimConnect {
    fn get_client_data_name(name: &CStr) -> AnyhowResult<CString> {
        let string = name.to_str()?;

        Ok(CString::new(format!("{PROGRAM_NAME}{string}"))?)
    }

    pub fn open(program_name: &CStr) -> AnyhowResult<Self> {
        let mut handle = std::ptr::null_mut() as bindings::HANDLE;

        check_hr!(unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                program_name.as_ptr(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        });

        Ok(Self {
            handle: std::ptr::NonNull::new(handle)
                .ok_or_else(|| anyhow!("pointer expected to not be null"))?,
        })
    }

    pub fn register_struct<T: Sized>(&self, data_name: &CStr) -> AnyhowResult<()> {
        let data_size = std::mem::size_of::<T>();
        let data_name = Self::get_client_data_name(data_name)?;
        println!("Data Size Requested: {data_size}");

        let hr = unsafe {
            bindings::SimConnect_MapClientDataNameToID(self.handle.as_ptr(), data_name.as_ptr(), 1)
        };

        check_hr!(hr);

        check_hr!(unsafe {
            bindings::SimConnect_CreateClientData(
                self.handle.as_ptr(),
                1,
                data_size.try_into().unwrap(),
                bindings::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED,
            )
        });
        Ok(())
    }

    pub fn subscribe_to_system_event(&self, event: SystemEvent) -> AnyhowResult<()> {
        println!("Requesting subscription to event: {event}");
        let event_id = event as u32;
        check_hr!(unsafe {
            bindings::SimConnect_SubscribeToSystemEvent(
                self.handle.as_ptr(),
                event_id,
                event.sc_string().as_ptr(),
            )
        });

        Ok(())
    }

    pub fn check_events(&self) -> AnyhowResult<()> {
        extern "C" fn callback(
            data: *mut bindings::SIMCONNECT_RECV,
            cb_data: bindings::DWORD,
            context: *mut c_void,
        ) {
            let data_id = unsafe { *data }.dwID as i32;
            match data_id {
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
                    let event = unsafe { *(data as *const bindings::SIMCONNECT_RECV_OPEN) };
                    let major_version = event.dwApplicationBuildMajor;
                    let minor_version = event.dwApplicationBuildMinor;

                    let application_name = String::from_utf8(
                        unsafe {
                            std::mem::transmute::<[i8; 256], [u8; 256]>(event.szApplicationName)
                        }
                        .to_vec(),
                    )
                    .unwrap();

                    println!(
                        "Connection Opened to SimConnect: {application_name}@{0}.{1}",
                        major_version, minor_version
                    );
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
                    let event = unsafe { *(data as *mut bindings::SIMCONNECT_RECV_EVENT) };
                }
                id => println!("unknown id: {id}"),
            }
        }

        check_hr!(unsafe {
            bindings::SimConnect_CallDispatch(
                self.handle.as_ptr(),
                Some(callback),
                std::ptr::null_mut(),
            )
        });

        Ok(())
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        unsafe {
            bindings::SimConnect_Close(self.handle.as_ptr());
        }
    }
}
