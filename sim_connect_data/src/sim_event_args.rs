use std::{mem::transmute, str::FromStr};

use crate::internals::{IterEnum, ToSimConnect};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use sim_connect_macros::{IterEnum, ToSimConnect};
use sim_connect_sys::bindings;
use std::ffi::CString;

use crate::recv_data::FromPtr;

#[derive(TryFromPrimitive, Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(i32)]
#[serde(rename = "camelCase")]
pub enum SimObjectType {
    User = bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_USER,
    All = bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_ALL,
    Aircraft = bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_AIRCRAFT,
    Helicopter = bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_HELICOPTER,
    Boat = bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_BOAT,
    Ground = bindings::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_GROUND,
}

impl FromPtr for SimObjectType {
    fn from_pointer(data: std::ptr::NonNull<bindings::SIMCONNECT_RECV>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let add_or_remove: *mut bindings::SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE =
            unsafe { transmute(data.as_ptr()) };

        let obj_type = unsafe { *add_or_remove }.eObjType;

        Ok(Self::try_from_primitive(obj_type)?)
    }
}

#[derive(TryFromPrimitive, Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u32)]
#[serde(rename = "camelCase")]
pub enum SimViewType {
    Cockpit2D = bindings::SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_2D,
    CockpitVirtual = bindings::SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_VIRTUAL,
    Ortho = bindings::SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_ORTHOGONAL,
}

#[derive(
    TryFromPrimitive,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    ToSimConnect,
    IntoPrimitive,
    IterEnum,
)]
#[repr(u32)]
#[serde(rename = "camelCase")]
pub enum SimStateArgs {
    AircraftLoaded,
    DialogMode,
    FlightLoaded,
    FlightPlan,
    Sim,
}

impl FromStr for SimStateArgs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variants = Self::iter_enum();
        let lower_str = s.to_lowercase();

        for item in variants {
            let lower_item = item.to_string().to_lowercase();
            if lower_item == lower_str {
                return Ok(item);
            }
        }
        Err(anyhow::anyhow!("Unable to convert {s} into SimStateArgs"))
    }
}
