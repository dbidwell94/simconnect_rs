use std::ffi::CString;

use super::ToSimConnect;

#[derive(Debug, Copy, Clone, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum SystemEvent {
    OneSec,
    FourSec,
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

impl std::fmt::Display for SystemEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneSec => write!(f, "1sec"),
            Self::FourSec => write!(f, "4sec"),
            Self::SixHz => write!(f, "6Hz"),
            _ => write!(f, "{self:?}"),
        }
    }
}

impl ToSimConnect for SystemEvent {
    fn sc_string(&self) -> std::ffi::CString {
        CString::new(format!("{self}")).unwrap()
    }
}
