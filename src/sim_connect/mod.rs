use self::{
    sim_events::SystemEvent, sim_units::SimUnit, sim_var_types::SimVarType, sim_vars::SimVar,
};

use anyhow::{anyhow, Result as AnyhowResult};
use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
};

mod bindings;
pub mod recv_data;
pub mod sim_events;
pub mod sim_units;
pub mod sim_var_types;
pub mod sim_vars;
mod macros;

use recv_data::RecvDataEvent;

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

pub trait SimConnectToStruct {
    fn create_instance<'a>(bytes: &'a [u32]) -> AnyhowResult<Self>
    where
        Self: Sized;
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
    type_map: HashMap<CString, u32>,
    program_name: String,
}

impl SimConnect {
    fn get_client_data_name(&self, name: &CStr) -> AnyhowResult<CString> {
        let string = name.to_str()?;

        Ok(CString::new(format!("{0}{string}", self.program_name))?)
    }

    /// Opens a new connection to SimConnect using the program name defined
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
            program_name: program_name.to_str().unwrap().to_owned(),
        })
    }

    /// Registers the struct's field definitions with SimConnect
    pub fn register_struct<T: ToSimConnectStruct>(&mut self) -> AnyhowResult<()> {
        let raw_name = CString::new(std::any::type_name::<T>()).unwrap();
        println!(
            "registering {0} with SimConnect",
            raw_name.to_str().unwrap()
        );
        let data_name = self.get_client_data_name(&raw_name)?;

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

    /// Can ONLY be called after a call to `register_struct` has been called.
    /// Data will be found in the `check_events` function call.
    ///
    /// This function will return an error if the struct has not yet been registered
    /// with SimConnect
    pub fn request_data_on_self_object<T: ToSimConnectStruct>(&self) -> AnyhowResult<()> {
        let type_name = CString::new(std::any::type_name::<T>()).unwrap();
        let data_name = self.get_client_data_name(&type_name)?;
        let object_id = self.type_map.get(&data_name).ok_or_else(|| {
            anyhow!(
                "{0} has not yet been registered",
                type_name.to_str().unwrap()
            )
        })?;

        check_hr!(unsafe {
            bindings::SimConnect_RequestDataOnSimObjectType(
                self.handle.as_ptr(),
                0,
                object_id.clone(),
                0,
                bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_USER,
            )
        });

        Ok(())
    }

    /// Requests a subscription to a system event. Events can be checked by using the
    /// `check_events` function
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

    /// Checks for new system events. There is no guarantee that an event is waiting to be
    /// retrieved from SimConnect. If nothing is available, returns `None`.
    pub fn check_events(&self) -> AnyhowResult<Option<RecvDataEvent>> {
        let mut data = std::ptr::null_mut();

        let mut cb_data: bindings::DWORD = 0;

        let hr = unsafe {
            bindings::SimConnect_GetNextDispatch(self.handle.as_ptr(), &mut data, &mut cb_data)
        };

        if hr == 0 && cb_data > 0 {
            let ptr = std::ptr::NonNull::new(data)
                .ok_or_else(|| anyhow!("Pointer not expected to be null"))?;
            let data = recv_data::RecvDataEvent::from_pointer(ptr)?;

            return Ok(Some(data));
        }
        return Ok(None);
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        unsafe {
            bindings::SimConnect_Close(self.handle.as_ptr());
        }
    }
}
