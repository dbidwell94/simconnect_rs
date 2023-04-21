use self::sim_events::SystemEvent;
pub use sim_connect_data::ToSimConnect;

use anyhow::{anyhow, Result as AnyhowResult};
use sim_connect_data::{
    recv_data::RecvSystemState, sim_event_args::SimStateArgs, sim_events::SystemEventDataHolder,
    sim_input_events::InputEvent, SimConnectToStruct, StructToSimConnect,
};
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

use sim_connect_sys::bindings;

pub use sim_connect_data::recv_data;
pub use sim_connect_data::sim_event_args;
pub use sim_connect_data::sim_events;
pub use sim_connect_data::sim_units;
pub use sim_connect_data::sim_var_types;
pub use sim_connect_data::sim_vars;

use recv_data::RecvSimData;

use recv_data::RecvDataEvent;

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
    system_event_callback_sender: Sender<(
        SystemEvent,
        Option<Box<dyn Fn(SystemEventDataHolder) + Send + Sync>>,
        bool,
    )>,
    state_request_reciever: Receiver<RecvSystemState>,
    should_quit: Arc<RwLock<bool>>,
    listen_handle: Option<JoinHandle<AnyhowResult<()>>>,
    sender_sender: Sender<(u32, Sender<RecvSimData>)>,
}

impl SimConnect {
    fn get_handle_lock(&self) -> AnyhowResult<MutexGuard<NonNull<c_void>>> {
        self.handle
            .0
            .lock()
            .map_err(|_| anyhow!("SimConnect handle has been poisoned"))
    }

    fn get_client_data_name(&self, name: &str) -> String {
        format!("{0}{name}", self.program_name)
    }

    fn begin_listen_for_events(
        data_sender: Receiver<(u32, Sender<RecvSimData>)>,
        callback_sender: Receiver<(
            SystemEvent,
            Option<Box<dyn Fn(SystemEventDataHolder) + Send + Sync>>,
            bool,
        )>,
        state_sender: Sender<RecvSystemState>,
        should_quit: Arc<RwLock<bool>>,
        handle: ThreadSafeHandle,
        poll_interval: Duration,
    ) -> AnyhowResult<()> {
        let mut should_wait: bool;
        let mut data_map: HashMap<u32, Sender<RecvSimData>> = HashMap::new();
        let mut callback_map: HashMap<SystemEvent, Box<dyn Fn(SystemEventDataHolder)>> =
            HashMap::new();
        loop {
            for (data_id, sender) in data_sender.try_iter() {
                data_map.insert(data_id, sender);
            }
            for (event_type, event_callback, insert) in callback_sender.try_iter() {
                if insert {
                    if let Some(callback) = event_callback {
                        callback_map.insert(event_type, callback);
                    }
                } else {
                    callback_map.remove(&event_type);
                }
            }
            should_wait = true;
            {
                let should_quit = should_quit.as_ref().read().unwrap();
                if *should_quit {
                    break;
                }
            }

            let mut data = std::ptr::null_mut();

            let mut cb_data_size: bindings::DWORD = 0;
            let hr: i32;
            // Get data and unlock ASAP
            {
                let handle = handle
                    .0
                    .lock()
                    .map_err(|_| anyhow!("SimConnect handle has been poisoned"))?;

                hr = unsafe {
                    bindings::SimConnect_GetNextDispatch(
                        handle.as_ptr(),
                        &mut data,
                        &mut cb_data_size,
                    )
                };
            }

            if hr == 0 && cb_data_size > 0 {
                let ptr = std::ptr::NonNull::new(data)
                    .ok_or_else(|| anyhow!("Pointer not expected to be null"))?;
                let data = recv_data::RecvDataEvent::from_pointer(ptr)?;

                match data {
                    RecvDataEvent::SystemState(state) => {
                        state_sender.send(state)?;
                    }
                    RecvDataEvent::Null => {}
                    RecvDataEvent::Open(_) => {}
                    RecvDataEvent::Data(data) => {
                        let data_id = data.get_id();
                        let sender = data_map.get(&data_id);
                        if let Some(sender) = sender {
                            sender
                                .send(data)
                                .expect("Unable to send data across threads.");
                        }
                    }
                    RecvDataEvent::Event(evt_type) => {
                        if let Some(callback) = callback_map.get(&evt_type.system_event) {
                            callback.as_ref()(evt_type);
                        };
                    }
                    RecvDataEvent::Quit => {}
                }

                should_wait = false;
            }

            if should_wait {
                std::thread::sleep(poll_interval);
            }
        }
        Ok(())
    }

    fn get_struct_name<T: StructToSimConnect>(&self) -> String {
        let struct_name = std::any::type_name::<T>();
        self.get_client_data_name(struct_name)
    }

    /// Can ONLY be called after a call to `register_struct` has been called.
    /// Data will be found in the `check_events` function call.
    ///
    /// This function will return an error if the struct has not yet been registered
    /// with SimConnect
    fn request_data_on_self_object<T: StructToSimConnect>(&self) -> AnyhowResult<()> {
        let type_name = std::any::type_name::<T>();
        let data_name = self.get_client_data_name(type_name);
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
                    *object_id,
                    0,
                    bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_USER,
                )
            });
        }

        Ok(())
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
        if poll_interval.is_none() {
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
        let (evt_sx, evt_rcv) = channel();
        let (state_sx, state_rcv) = channel();

        let listen_handle: JoinHandle<AnyhowResult<()>> = thread::spawn(move || {
            let should_quit = cloned_should_quit;
            let handle = cloned_handle;

            Self::begin_listen_for_events(
                rc,
                evt_rcv,
                state_sx,
                should_quit,
                handle,
                poll_interval.unwrap(),
            )?;
            Ok(())
        });

        Ok(Self {
            handle,
            type_map: HashMap::new(),
            program_name: program_name.to_str().unwrap().to_owned(),
            data_event_map: HashMap::new(),
            system_event_callback_sender: evt_sx,
            state_request_reciever: state_rcv,
            should_quit,
            listen_handle: Some(listen_handle),
            sender_sender: sx,
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
                // let sim_unit = field
                //     .sim_unit
                //     .map(|unit| unsafe { *unit.sc_string().as_ptr() as *const i8 })
                //     .unwrap_or(std::ptr::null());

                check_hr!(unsafe {
                    bindings::SimConnect_AddToDataDefinition(
                        handle.as_ptr(),
                        new_data_id,
                        field.sim_var.sc_string().as_ptr(),
                        field
                            .sim_unit
                            .map(|unit| unit.sc_string())
                            .or(Some(CString::new("").unwrap()))
                            .unwrap()
                            .as_ptr(),
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

    /* #region request_system_state */
    #[cfg(feature = "async")]
    /// Requests current information about the system state.
    pub async fn request_system_state(
        &mut self,
        state_request: SimStateArgs,
    ) -> AnyhowResult<RecvSystemState> {
        {
            let handle = self.get_handle_lock()?;
            let request_id: u32 = state_request.into();

            check_hr!(unsafe {
                bindings::SimConnect_RequestSystemState(
                    handle.as_ptr(),
                    request_id,
                    state_request.sc_string().as_ptr(),
                )
            });
        }

        let possible_found = self.state_request_reciever.try_iter().last();
        if let None = possible_found {
            let data = self.state_request_reciever.recv().or_else(|_| {
                Err(anyhow!(
                    "Thread has been closed and channel no longer available"
                ))
            })?;
            return Ok(data);
        }

        Ok(possible_found.unwrap())
    }

    #[cfg(not(feature = "async"))]
    /// Requests current information about the system state.
    pub fn request_system_state(
        &mut self,
        state_request: SimStateArgs,
    ) -> AnyhowResult<RecvSystemState> {
        {
            let handle = self.get_handle_lock()?;
            let request_id: u32 = state_request.into();

            check_hr!(unsafe {
                bindings::SimConnect_RequestSystemState(
                    handle.as_ptr(),
                    request_id,
                    state_request.sc_string().as_ptr(),
                )
            });
        }

        let possible_found = self.state_request_reciever.try_iter().last();
        if possible_found.is_none() {
            let data = self
                .state_request_reciever
                .recv()
                .map_err(|_| anyhow!("Thread has been closed and channel no longer available"))?;
            return Ok(data);
        }

        Ok(possible_found.unwrap())
    }

    /* #endregion */

    /* #region system_event */
    /// Requests a subscription to a system event. Events can be checked by using the
    /// `check_events` function
    pub fn subscribe_to_system_event(
        &mut self,
        event: SystemEvent,
        callback: impl Fn(SystemEventDataHolder) + Send + Sync + 'static,
    ) -> AnyhowResult<()> {
        let event_id: u32 = event.into();

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

        self.system_event_callback_sender
            .send((event, Some(Box::new(callback)), true))
            .map_err(|_| anyhow!("Unable to send function to the listener"))?;

        Ok(())
    }

    pub fn unsubscribe_from_system_event(&mut self, event: SystemEvent) -> AnyhowResult<()> {
        let evt_id: u32 = event.into();
        let handle = self.get_handle_lock()?;

        check_hr!(unsafe {
            bindings::SimConnect_UnsubscribeFromSystemEvent(handle.as_ptr(), evt_id)
        });

        self.system_event_callback_sender
            .send((event, None, false))
            .map_err(|_| anyhow!("Unable to unsubscribe from system event"))?;

        Ok(())
    }

    /* #endregion */

    /* #region input_event */
    /// Request subscription to an input event. Input events are located in the `sim_connect_rs::sim_connect_data::sim_input_events` package
    pub fn subscribe_to_input_event(&mut self, input_event: impl InputEvent) -> AnyhowResult<()> {
        let _event = Box::new(input_event);
        Ok(())
    }
    /* #endregion */

    /* #region get_latest_data */
    #[cfg(feature = "async")]
    /// Gets data on a sim object. Calls `register_struct` if it hasn't already been called.
    /// If it hasn't been called, chances are this function will return None as SimConnect needs
    /// time to process the data.
    pub async fn get_latest_data<T: SimConnectToStruct>(&mut self) -> AnyhowResult<T::ReturnType> {
        self.register_struct::<T>()?;
        let data_name = self.get_struct_name::<T>();
        let data_id = self.type_map.get(&data_name);
        let data_id = data_id.unwrap();

        self.request_data_on_self_object::<T>()?;

        let recv = self
            .data_event_map
            .get(data_id)
            .ok_or_else(|| anyhow!("data_event_map not expected to be empty"))?;

        let mut data = recv.try_iter().last();
        if let None = data {
            data = Some(recv.recv().or_else(|_| {
                Err(anyhow!(
                    "Thread has been closed and channel no longer available"
                ))
            })?)
        }
        let data = data.unwrap();
        let s = unsafe { T::parse_struct(data.get_pointer()) }
            .map_err(|_| anyhow!("Unable to parse struct"))?;
        Ok(s)
    }

    #[cfg(not(feature = "async"))]
    /// Gets data on a sim object. Calls `register_struct` if it hasn't already been called.
    /// If it hasn't been called, chances are this function will return None as SimConnect needs
    /// time to process the data.
    pub fn get_latest_data<T: SimConnectToStruct>(&mut self) -> AnyhowResult<T> {
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
        if data.is_none() {
            data = Some(
                recv.recv()
                    .map_err(|_| anyhow!("Thread has been closed and channel no longer available"))?
                    .to_struct(),
            )
        }
        let data = data.unwrap()?;
        Ok(data)
    }

    /* #endregion */
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
