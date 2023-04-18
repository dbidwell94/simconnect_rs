use num_enum::{IntoPrimitive, TryFromPrimitive};
use sim_connect_macros::{IterEnum, ToSimConnect};
use sim_connect_sys::bindings;
use std::{
    ffi::{CStr, CString},
    mem::transmute,
    str::FromStr,
};

use crate::{
    recv_data::FromPtr,
    sim_event_args::{SimObjectType, SimViewType},
    IterEnum,
};
use serde::{Deserialize, Serialize};

use super::ToSimConnect;

#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    ToSimConnect,
    Hash,
    PartialEq,
    Eq,
    IntoPrimitive,
    Serialize,
    Deserialize,
    IterEnum,
)]
#[repr(u32)]
#[serde(rename = "camelCase")]
pub enum SystemEvent {
    #[string(name = "1sec")]
    OneSec,
    #[string(name = "4sec")]
    FourSec,
    #[string(name = "6Hz")]
    SixHz,
    AircraftLoaded,
    Crashed,
    CrashReset,
    FlightLoaded,
    FlightSaved,
    FlightPlanActivated,
    FlightPlanDeactivated,
    Frame,
    ObjectAdded,
    ObjectRemoved,
    Pause,
    PauseEX1,
    Paused,
    PauseFrame,
    PositionChanged,
    Sim,
    SimStart,
    SimStop,
    Sound,
    Unpaused,
    View,
}

impl FromStr for SystemEvent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let self_iter = Self::iter_enum();
        let lower_s = s.to_lowercase();

        for item in self_iter {
            let lower_item = item.to_string().to_lowercase();
            if lower_s == lower_item {
                return Ok(item);
            }
        }

        Err(anyhow::anyhow!("Unable to serialize {s} to SystemEvent"))
    }
}

#[derive(Debug)]
pub struct SystemEventDataHolder {
    pub system_event: SystemEvent,
    pub event_data: SystemEventData,
}

impl FromPtr for SystemEventDataHolder {
    fn from_pointer(data: std::ptr::NonNull<bindings::SIMCONNECT_RECV>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let raw_ptr: *mut bindings::SIMCONNECT_RECV_EVENT = unsafe { transmute(data.as_ptr()) };
        let event: SystemEvent = SystemEvent::try_from_primitive(unsafe { *raw_ptr }.uEventID)?;
        let event_data = SystemEventData::from_pointer(data)?;
        Ok(Self {
            event_data,
            system_event: event,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "camelCase")]
pub enum SystemEventData {
    OneSec,
    FourSec,
    SixHz,
    AircraftLoaded(String),
    Crashed,
    CrashReset,
    FlightLoaded(String),
    FlightSaved(String),
    FlightPlanActivated(String),
    FlightPlanDeactivated,
    Frame,
    ObjectAdded(SimObjectType),
    ObjectRemoved(SimObjectType),
    Pause,
    PauseEX1,
    Paused,
    PauseFrame,
    PositionChanged,
    Sim(bool),
    SimStart,
    SimStop,
    Sound,
    Unpaused,
    View(SimViewType),
}

fn extract_name_from_filename(
    name_ptr: *mut bindings::SIMCONNECT_RECV_EVENT_FILENAME,
) -> anyhow::Result<String> {
    let evt = unsafe { *name_ptr };
    let str_ref = &evt.szFileName as *const i8;
    Ok(unsafe { CStr::from_ptr(str_ref) }.to_str()?.to_owned())
}

impl FromPtr for SystemEventData {
    fn from_pointer(
        data: std::ptr::NonNull<sim_connect_sys::bindings::SIMCONNECT_RECV>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let raw_ptr: *mut bindings::SIMCONNECT_RECV_EVENT = unsafe { transmute(data.as_ptr()) };
        let event: SystemEvent = SystemEvent::try_from_primitive(unsafe { *raw_ptr }.uEventID)?;

        Ok(match event {
            SystemEvent::OneSec => Self::OneSec,
            SystemEvent::FourSec => Self::FourSec,
            SystemEvent::SixHz => Self::SixHz,
            SystemEvent::AircraftLoaded => {
                let evt_filename: *mut bindings::SIMCONNECT_RECV_EVENT_FILENAME =
                    unsafe { transmute(raw_ptr) };
                Self::AircraftLoaded(extract_name_from_filename(evt_filename)?)
            }
            SystemEvent::Crashed => Self::Crashed,
            SystemEvent::CrashReset => Self::CrashReset,
            SystemEvent::FlightLoaded => {
                let evt_filename: *mut bindings::SIMCONNECT_RECV_EVENT_FILENAME =
                    unsafe { transmute(raw_ptr) };

                Self::FlightLoaded(extract_name_from_filename(evt_filename)?)
            }
            SystemEvent::FlightSaved => {
                let evt_filename: *mut bindings::SIMCONNECT_RECV_EVENT_FILENAME =
                    unsafe { transmute(raw_ptr) };

                Self::FlightSaved(extract_name_from_filename(evt_filename)?)
            }
            SystemEvent::FlightPlanActivated => {
                let evt_filename: *mut bindings::SIMCONNECT_RECV_EVENT_FILENAME =
                    unsafe { transmute(raw_ptr) };

                Self::FlightPlanActivated(extract_name_from_filename(evt_filename)?)
            }
            SystemEvent::FlightPlanDeactivated => Self::FlightPlanDeactivated,
            SystemEvent::Frame => Self::Frame,
            SystemEvent::ObjectAdded => Self::ObjectAdded(SimObjectType::from_pointer(data)?),
            SystemEvent::ObjectRemoved => Self::ObjectRemoved(SimObjectType::from_pointer(data)?),
            SystemEvent::Pause => Self::Pause,
            SystemEvent::PauseEX1 => Self::PauseEX1,
            SystemEvent::Paused => Self::Paused,
            SystemEvent::PauseFrame => Self::PauseFrame,
            SystemEvent::PositionChanged => Self::PositionChanged,
            SystemEvent::Sim => {
                let state = unsafe { *raw_ptr }.dwData != 0;
                Self::Sim(state)
            }
            SystemEvent::SimStart => Self::SimStart,
            SystemEvent::SimStop => Self::SimStop,
            SystemEvent::Sound => Self::Sound,
            SystemEvent::Unpaused => Self::Unpaused,
            SystemEvent::View => {
                let view = SimViewType::try_from_primitive(unsafe { *raw_ptr }.dwData)?;
                Self::View(view)
            }
        })
    }
}
