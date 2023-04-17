use num_enum::TryFromPrimitive;
use sim_connect_macros::ToSimConnect;
use std::ffi::CString;

use super::ToSimConnect;

#[derive(Debug, Copy, Clone, TryFromPrimitive, ToSimConnect)]
#[repr(u32)]
pub enum SystemEvent {
    #[string("1sec")]
    OneSec,
    #[string("4sec")]
    FourSec,
    #[string("6Hz")]
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

