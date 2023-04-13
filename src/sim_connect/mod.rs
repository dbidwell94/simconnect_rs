use self::{
    sim_events::SystemEvent, sim_units::SimUnit, sim_var_types::SimVarType, sim_vars::SimVar,
};

use anyhow::{anyhow, Result as AnyhowResult};
#[cfg(feature = "derive")]
pub use sim_connect_macros;
use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
    ptr::NonNull,
    sync::{Arc, Mutex, MutexGuard, RwLock},
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

struct ThreadSafeHandle(Arc<Mutex<std::ptr::NonNull<c_void>>>);

impl Clone for ThreadSafeHandle {
    fn clone(&self) -> Self {
        let cloned_arc = self.0.clone();
        Self(cloned_arc)
    }
}

unsafe impl Send for ThreadSafeHandle {}

#[derive(Default)]
struct EventMap {
    data_event_map: HashMap<u32, RecvDataEvent>,
}

pub struct SimConnect {
    handle: ThreadSafeHandle,
    type_map: HashMap<String, u32>,
    program_name: String,
    data_event_map: Arc<Mutex<EventMap>>,
    should_quit: Arc<RwLock<bool>>,
    listen_handle: Option<JoinHandle<AnyhowResult<()>>>,
}

impl SimConnect {
    fn get_handle_lock(&self) -> AnyhowResult<MutexGuard<NonNull<c_void>>> {
        Ok(self
            .handle
            .0
            .lock()
            .or_else(|_| Err(anyhow!("SimConnect handle has been poisoned")))?)
    }

    fn get_client_data_name(&self, name: &str) -> String {
        format!("{0}{name}", self.program_name)
    }

    fn begin_listen_for_events(
        event_map: Arc<Mutex<EventMap>>,
        should_quit: Arc<RwLock<bool>>,
    ) -> AnyhowResult<()> {
        loop {
            let hr = unsafe {};
        }

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

        let handle = ThreadSafeHandle(Arc::new(Mutex::new(
            std::ptr::NonNull::new(handle)
                .ok_or_else(|| anyhow!("pointer expected to not be null"))?,
        )));

        let data_event_map: Arc<Mutex<EventMap>> = Default::default();

        let should_quit = Arc::new(RwLock::new(false));

        let cloned_event_map = data_event_map.clone();
        let cloned_should_quit = should_quit.clone();
        let cloned_handle = handle.clone();

        let listen_handle: JoinHandle<AnyhowResult<()>> = thread::spawn(move || {
            let evt_map = cloned_event_map;
            let should_quit = cloned_should_quit;
            let handle = cloned_handle;

            Self::begin_listen_for_events(evt_map, should_quit)?;
            Ok(())
        });

        Ok(Self {
            handle,
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

        {
            let handle_lock = self.get_handle_lock()?;
            let handle = *handle_lock;

            for field in fields {
                check_hr!(unsafe {
                    bindings::SimConnect_AddToDataDefinition(
                        handle.as_ptr(),
                        new_data_id,
                        field.sim_var.sc_string().as_ptr(),
                        field.sim_unit.sc_string().as_ptr(),
                        field.data_type as i32,
                        0.0,
                        field.id,
                    )
                });
            }
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

        {
            let handle_lock = self.get_handle_lock()?;
            let handle = *handle_lock;

            check_hr!(unsafe {
                bindings::SimConnect_RequestDataOnSimObjectType(
                    handle.as_ptr(),
                    0,
                    object_id.clone(),
                    0,
                    bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_USER,
                )
            });
        }

        Ok(())
    }

    /// Requests a subscription to a system event. Events can be checked by using the
    /// `check_events` function
    pub fn subscribe_to_system_event(&self, event: SystemEvent) -> AnyhowResult<()> {
        let event_id = event as u32;

        {
            let handle = self.get_handle_lock()?;

            check_hr!(unsafe {
                bindings::SimConnect_SubscribeToSystemEvent(
                    handle.as_ptr(),
                    event_id,
                    event.sc_string().as_ptr(),
                )
            });
        }

        Ok(())
    }

    /// Checks for new system events. There is no guarantee that an event is waiting to be
    /// retrieved from SimConnect. If nothing is available, returns `None`.
    fn check_events(&self) -> AnyhowResult<Option<RecvDataEvent>> {
        let mut data = std::ptr::null_mut();

        let mut cb_data: bindings::DWORD = 0;
        let hr: i32;
        {
            let handle = self.get_handle_lock()?;

            hr = unsafe {
                bindings::SimConnect_GetNextDispatch(handle.as_ptr(), &mut data, &mut cb_data)
            };
        }

        if hr == 0 && cb_data > 0 {
            let ptr = std::ptr::NonNull::new(data)
                .ok_or_else(|| anyhow!("Pointer not expected to be null"))?;
            let data = recv_data::RecvDataEvent::from_pointer(ptr)?;

            return Ok(Some(data));
        }
        return Ok(None);
    }

    /// Check weather or not SimConnect has data on a specified struct
    pub fn has_data<T: StructToSimConnect>(&self) -> AnyhowResult<bool> {
        let data_name = self.get_struct_name::<T>();
        let evt_map_lock = self
            .data_event_map
            .lock()
            .or_else(|_| Err(anyhow!("The data event map lock has been poisoned")))?;

        let data_id = self.type_map.get(&data_name);

        if let Some(data_id) = data_id {
            return Ok(evt_map_lock.data_event_map.contains_key(data_id));
        }
        Ok(false)
    }

    /// Gets data on a sim object. Calls `register_struct` if it hasn't already been called.
    /// If it hasn't been called, chances are this function will return None as SimConnect needs
    /// time to process the data. It is recommend that you check for data using the `has_data` call
    pub fn get_data<T: StructToSimConnect>(&mut self) -> AnyhowResult<Option<T>> {
        self.register_struct::<T>()?;
        let data_name = self.get_struct_name::<T>();
        let data_id = self.type_map.get(&data_name);
        if let None = data_id {
            return Ok(None);
        };
        let data_id = data_id.unwrap();

        let found_data: Option<RecvDataEvent>;

        // Lock the data and remove lock as quick as possible
        {
            let mut data_map_lock = self
                .data_event_map
                .lock()
                .or_else(|_| Err(anyhow!("Data map lock has been poisoned")))?;

            found_data = data_map_lock.data_event_map.remove(data_id);
        }
        if let None = found_data {
            return Ok(None);
        }
        let found_data = found_data.unwrap();

        todo!()
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let mut should_quit = self.should_quit.write().unwrap();
        *should_quit = true;
        if let Some(join_handle) = self.listen_handle.take() {
            let _ = join_handle.join();
        }
        let handle = self.get_handle_lock().unwrap();

        unsafe {
            bindings::SimConnect_Close(handle.as_ptr());
        }
    }
}
