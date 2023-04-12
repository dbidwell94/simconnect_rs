use self::{
    sim_events::SystemEvent, sim_units::SimUnit, sim_var_types::SimVarType, sim_vars::SimVar,
};

use super::PROGRAM_NAME;
use anyhow::{anyhow, Result as AnyhowResult};
use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
};

mod bindings;
pub mod sim_events;
pub mod sim_units;
pub mod sim_var_types;
pub mod sim_vars;

pub struct SimConnectDatum {
    pub id: u32,
    pub sim_var: SimVar,
    pub sim_unit: Box<dyn SimUnit>,
    pub data_type: SimVarType,
}

pub trait ToSimConnect {
    fn sc_string(&self) -> CString;
}

pub trait ToSimConnectStruct {
    fn get_fields() -> Vec<SimConnectDatum>;
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

#[derive(Default)]
struct Test {
    item: u32,
}

pub struct SimConnect {
    handle: std::ptr::NonNull<c_void>,
    type_map: HashMap<CString, u32>,
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
            type_map: HashMap::new(),
        })
    }

    pub fn register_struct<T: Default + ToSimConnectStruct>(&mut self) -> AnyhowResult<()> {
        let raw_name = CString::new(std::any::type_name::<T>()).unwrap();
        let data_name = Self::get_client_data_name(&raw_name)?;

        let new_data_id = self.type_map.len() as u32;

        if self.type_map.contains_key(&data_name) {
            return Ok(());
        }

        let fields = T::get_fields();

        for field in fields {
            check_hr!(unsafe {
                bindings::SimConnect_AddToDataDefinition(
                    self.handle.as_ptr(),
                    new_data_id,
                    field.sim_var.sc_string().as_ptr(),
                    field.sim_unit.sc_string().as_ptr(),
                    field.data_type as i32,
                    0.0,
                    field.id,
                )
            });
        }

        self.type_map.insert(data_name, new_data_id);

        Ok(())
    }

    pub fn request_data<T: Sized + Default>(&self) -> AnyhowResult<T> {
        let obj: T = T::default();
        Ok(obj)
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
        let mut data = std::ptr::null_mut();

        let mut cb_data: bindings::DWORD = 0;

        let hr = unsafe {
            bindings::SimConnect_GetNextDispatch(self.handle.as_ptr(), &mut data, &mut cb_data)
        };

        println!("Callback data size: {cb_data}");

        if hr != 0 {
            return Ok(());
        }

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
