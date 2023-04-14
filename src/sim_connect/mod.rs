use self::{
    sim_events::SystemEvent, sim_units::SimUnit, sim_var_types::SimVarType, sim_vars::SimVar,
};

use anyhow::{anyhow, Result as AnyhowResult};
#[cfg(feature = "derive")]
pub use sim_connect_macros;
use std::{
    collections::HashMap,
    ffi::{c_void, CString},
    ptr::NonNull,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, MutexGuard, RwLock,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

#[allow(dead_code)]
mod bindings;
pub mod recv_data;
pub mod sim_events;
pub mod sim_units;
pub mod sim_var_types;
pub mod sim_vars;

use recv_data::RecvSimData;

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

pub struct SimConnect {
    handle: ThreadSafeHandle,
    type_map: HashMap<String, u32>,
    program_name: String,
    data_event_map: HashMap<u32, Receiver<RecvSimData>>,
    should_quit: Arc<RwLock<bool>>,
    listen_handle: Option<JoinHandle<AnyhowResult<()>>>,
    sender_sender: Sender<(u32, Sender<RecvSimData>)>,
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
        data_sender: Receiver<(u32, Sender<RecvSimData>)>,
        should_quit: Arc<RwLock<bool>>,
        handle: ThreadSafeHandle,
        poll_interval: Duration,
    ) -> AnyhowResult<()> {
        let mut should_wait: bool;
        let mut data_map: HashMap<u32, Sender<RecvSimData>> = HashMap::new();
        loop {
            for (data_id, sender) in data_sender.try_iter() {
                data_map.insert(data_id, sender);
            }
            should_wait = true;
            {
                let should_quit = should_quit.as_ref().read().unwrap();
                if *should_quit {
                    break;
                }
            }

            let mut data = std::ptr::null_mut();

            let mut cb_data: bindings::DWORD = 0;
            let hr: i32;
            // Get data and unlock ASAP
            {
                let handle = handle
                    .0
                    .lock()
                    .or_else(|_| Err(anyhow!("SimConnect handle has been poisoned")))?;

                hr = unsafe {
                    bindings::SimConnect_GetNextDispatch(handle.as_ptr(), &mut data, &mut cb_data)
                };
            }

            if hr == 0 && cb_data > 0 {
                let ptr = std::ptr::NonNull::new(data)
                    .ok_or_else(|| anyhow!("Pointer not expected to be null"))?;
                let data = recv_data::RecvDataEvent::from_pointer(ptr)?;

                if let RecvDataEvent::Data(data) = data {
                    let data_id = data.get_id();
                    let sender = data_map.get(&data_id);
                    if let Some(sender) = sender {
                        sender
                            .send(data)
                            .expect("Unable to send data across threads.");
                    }
                }

                should_wait = false;
            }

            if should_wait {
                std::thread::sleep(poll_interval);
            }
        }
        Ok(())
    }

    fn get_struct_name<T: StructToSimConnect + Copy>(&self) -> String {
        let struct_name = std::any::type_name::<T>();
        return self.get_client_data_name(struct_name);
    }

    /// Opens a new connection to SimConnect using the program name defined
    /// Program will then automatically start listening for events, caching the latest
    /// of all the unique events. Events can be retrieved by requesting the latest.
    ///
    /// # Parameters
    ///
    /// - program_name -> The name which to register the program in MSFS SimConnect
    ///     - Will return `Err(_)` if `program_name` contains a null-terminated string
    /// - poll_interval -> How often should the SimConnect wrapper check for data with MSFS SimConnect
    ///     - Defaults to 1 sec
    ///     - This value is ignored if data is recieved, as data will be checked for again immediately afterwards
    ///     - Note: if duration is too long, some functions might take longer to return data
    ///
    /// # Example
    ///
    /// ```
    /// // This will poll MSFS SimConnect every .5 seconds for data
    /// let sc = SimConnect::open("My Awesome Application", Some(Duration::from_millis(500)));
    /// ```
    pub fn open(program_name: &str, mut poll_interval: Option<Duration>) -> AnyhowResult<Self> {
        if let None = poll_interval {
            poll_interval = Some(Duration::from_secs(1));
        }
        let mut handle = std::ptr::null_mut() as bindings::HANDLE;
        let program_name = CString::new(program_name)?;

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

        let should_quit = Arc::new(RwLock::new(false));

        let cloned_should_quit = should_quit.clone();
        let cloned_handle = handle.clone();

        let (sx, rc) = channel();

        let listen_handle: JoinHandle<AnyhowResult<()>> = thread::spawn(move || {
            let should_quit = cloned_should_quit;
            let handle = cloned_handle;

            Self::begin_listen_for_events(rc, should_quit, handle, poll_interval.unwrap())?;
            Ok(())
        });

        Ok(Self {
            handle,
            type_map: HashMap::new(),
            program_name: program_name.to_str().unwrap().to_owned(),
            data_event_map: HashMap::new(),
            should_quit,
            listen_handle: Some(listen_handle),
            sender_sender: sx,
        })
    }

    /// Registers the struct's field definitions with SimConnect
    pub fn register_struct<T: StructToSimConnect + Copy>(&mut self) -> AnyhowResult<()> {
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

        let (sx, rc) = channel();

        self.sender_sender
            .send((new_data_id, sx))
            .expect("Program encountered a fatal error sending data across threads");
        self.data_event_map.insert(new_data_id, rc);
        Ok(())
    }

    /// Can ONLY be called after a call to `register_struct` has been called.
    /// Data will be found in the `check_events` function call.
    ///
    /// This function will return an error if the struct has not yet been registered
    /// with SimConnect
    fn request_data_on_self_object<T: StructToSimConnect + Copy>(&self) -> AnyhowResult<()> {
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

    /// Check weather or not SimConnect has data on a specified struct
    pub fn has_data<T: StructToSimConnect + Copy>(&self) -> AnyhowResult<bool> {
        let data_name = self.get_struct_name::<T>();

        let data_id = self.type_map.get(&data_name);

        if let None = data_id {
            return Ok(false);
        }
        let data_id = data_id.unwrap();
        if let Some(k) = self.data_event_map.get(data_id) {
            if let Some(_) = k.try_iter().peekable().peek() {
                return Ok(true);
            }
            return Ok(false);
        }
        Ok(false)
    }

    /// Gets data on a sim object. Calls `register_struct` if it hasn't already been called.
    /// If it hasn't been called, chances are this function will return None as SimConnect needs
    /// time to process the data. It is recommend that you check for data using the `has_data` call
    pub async fn get_latest_data<T: StructToSimConnect + Copy>(&mut self) -> AnyhowResult<T> {
        self.register_struct::<T>()?;
        let data_name = self.get_struct_name::<T>();
        let data_id = self.type_map.get(&data_name);
        let data_id = data_id.unwrap();

        self.request_data_on_self_object::<T>()?;

        let recv = self
            .data_event_map
            .get(data_id)
            .ok_or_else(|| anyhow!("data_event_map not expected to be empty"))?;

        let mut data = recv.try_iter().map(|d| d.to_struct()).last();
        if let None = data {
            data = Some(
                recv.recv()
                    .or_else(|_| {
                        Err(anyhow!(
                            "Thread has been closed and channel no longer available"
                        ))
                    })?
                    .to_struct(),
            )
        }
        let data = data.unwrap()?;
        Ok(data)
    }
}

unsafe impl Send for SimConnect {}

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
