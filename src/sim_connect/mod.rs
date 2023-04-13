use self::{
    recv_data::RecvSimData, sim_events::SystemEvent, sim_units::SimUnit, sim_var_types::SimVarType,
    sim_vars::SimVar,
};

use anyhow::{anyhow, Result as AnyhowResult};
#[cfg(feature = "derive")]
pub use sim_connect_macros;
use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
    sync::{Arc, Mutex, RwLock},
    thread::{self, JoinHandle},
};

#[allow(dead_code)]
mod bindings;
pub mod recv_data;
pub mod sim_events;
pub mod sim_units;
pub mod sim_var_types;
pub mod sim_vars;

/// # Description
/// Auto-implement `StructToSimConnect`.
///
/// # Notes
///
/// - Required enums must be in scope when specifying them in the `#[datum(..)]` attribute
/// - Id's cannot be re-used in the same struct. This will create undefined behaviour
///
/// # Example
///
/// ```
///     use sim_connect_rs::{
///         sim_units::{Length, Speed},
///         sim_var_types::SimVarType,
///         sim_vars::SimVar,
///         StructToSimConnect,
///     };
///
///     #[derive(StructToSimConnect)]
///     struct TestStruct {
///         #[datum(
///             sim_var = "SimVar::AirspeedTrue",
///             sim_unit = "Speed::KNT",
///             data_type = "SimVarType::I32"
///         )]
///         airspeed: i32,
///         #[datum(
///              sim_var = "SimVar::IndicatedAlt",
///              sim_unit = "Length::Foot",
///              data_type = "SimVarType::F32"
///         )]
///         altitude: f32,
///}
/// ```
pub trait StructToSimConnect {
    fn get_fields() -> Vec<SimConnectDatum>;
}

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
struct EventMap {
    data_event_map: HashMap<u32, RecvDataEvent>,
}

pub struct SimConnect {
    handle: std::ptr::NonNull<c_void>,
    type_map: HashMap<String, u32>,
    program_name: String,
    data_event_map: Arc<Mutex<EventMap>>,
    should_quit: Arc<RwLock<bool>>,
    listen_handle: Option<JoinHandle<()>>,
}

impl SimConnect {
    fn get_client_data_name(&self, name: &str) -> String {
        format!("{0}{name}", self.program_name)
    }

    async fn begin_listen_for_events(
        event_map: Arc<Mutex<EventMap>>,
        should_quit: Arc<RwLock<bool>>,
    ) -> AnyhowResult<()> {
        loop {}

        Ok(())
    }

    fn get_struct_name<T: StructToSimConnect>(&self) -> String {
        let struct_name = std::any::type_name::<T>();
        return self.get_client_data_name(struct_name);
    }

    /// Opens a new connection to SimConnect using the program name defined
    /// Program will then automatically start listening for events, caching the latest
    /// of all the unique events. Events can be retrieved by requesting the latest.
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

        let data_event_map: Arc<Mutex<EventMap>> = Default::default();

        let should_quit = Arc::new(RwLock::new(false));

        let cloned_event_map = data_event_map.clone();
        let cloned_should_quit = should_quit.clone();

        let listen_handle = thread::spawn(move || {
            let evt_map = cloned_event_map;
            let should_quit = cloned_should_quit;
        });

        Ok(Self {
            handle: std::ptr::NonNull::new(handle)
                .ok_or_else(|| anyhow!("pointer expected to not be null"))?,
            type_map: HashMap::new(),
            program_name: program_name.to_str().unwrap().to_owned(),
            data_event_map,
            should_quit,
            listen_handle: Some(listen_handle),
        })
    }

    /// Registers the struct's field definitions with SimConnect
    pub fn register_struct<T: StructToSimConnect>(&mut self) -> AnyhowResult<()> {
        let data_name = self.get_struct_name::<T>();

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
    pub fn request_data_on_self_object<T: StructToSimConnect>(&self) -> AnyhowResult<()> {
        let type_name = std::any::type_name::<T>();
        let data_name = self.get_client_data_name(&type_name);
        let object_id = self
            .type_map
            .get(&data_name)
            .ok_or_else(|| anyhow!("{type_name} has not yet been registered"))?;

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
    fn check_events(&self) -> AnyhowResult<Option<RecvDataEvent>> {
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

    pub async fn request_data<T: StructToSimConnect>(&mut self) -> AnyhowResult<T> {
        let data_name = std::any::type_name::<T>();

        // register the struct (no change if already registered)
        self.register_struct::<T>()?;
        self.request_data_on_self_object::<T>()?;

        loop {}

        todo!()
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let mut should_quit = self.should_quit.write().unwrap();
        *should_quit = true;
        if let Some(join_handle) = self.listen_handle.take() {
            join_handle.join();
        }

        unsafe {
            bindings::SimConnect_Close(self.handle.as_ptr());
        }
    }
}
