use super::PROGRAM_NAME;
use anyhow::{anyhow, Result as AnyhowResult};
use std::ffi::{c_void, CStr, CString};

mod bindings;

fn check_err<T>(hr: bindings::HRESULT, to_return: T) -> AnyhowResult<T> {
    if hr != 0 {
        return Err(anyhow!(format!(
            "HRESULT indicates error: 0x{:x}",
            hr as u32
        )));
    }
    return Ok(to_return);
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

        let h_result = unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                program_name.as_ptr(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        };

        check_err(
            h_result,
            Self {
                handle: std::ptr::NonNull::new(handle)
                    .ok_or_else(|| anyhow!("pointer expected to not be null"))?,
            },
        )
    }

    pub fn create_client_data<T: Sized>(&self, data_name: &CStr) -> AnyhowResult<()> {
        let data_size = std::mem::size_of::<T>();
        let data_name = Self::get_client_data_name(data_name)?;
        println!("Data Size Requested: {data_size}");

        let hr = unsafe {
            bindings::SimConnect_MapClientDataNameToID(self.handle.as_ptr(), data_name.as_ptr(), 1)
        };
        check_err(hr, ())?;

        let hr = unsafe {
            bindings::SimConnect_CreateClientData(
                self.handle.as_ptr(),
                1,
                data_size.try_into().unwrap(),
                bindings::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED,
            )
        };
        check_err(hr, ())
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        unsafe {
            bindings::SimConnect_Close(self.handle.as_ptr());
        }
    }
}
