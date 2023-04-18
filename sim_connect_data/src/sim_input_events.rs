use crate::ToSimConnect;
use sim_connect_macros::{InputEvent, ToSimConnect};
use std::ffi::CString;

pub trait InputEvent {}

#[derive(ToSimConnect, Clone, Copy, InputEvent)]
pub enum GearAndBrakes {
    #[string(name = "ANTISKID_BRAKES_TOGGLE")]
    AntiskidBrakeToggled,
    #[string(name = "BRAKES")]
    Brakes,
    #[string(name = "BRAKES_LEFT")]
    BrakesLeft,
    #[string(name = "BRAKES_RIGHT")]
    BrakesRight,
    #[string(name = "GEAR_DOWN")]
    GearDown,
    #[string(name = "GEAR_EMERGENCY_HANDLE_TOGGLE")]
    GearEmergencyHandleToggled,
    #[string(name = "GEAR_PUMP")]
    GearPump,
    #[string(name = "GEAR_UP")]
    GearUp,
    #[string(name = "PARKING_BRAKES")]
    ParkingBrakes,
}

#[derive(Clone, Copy, ToSimConnect, InputEvent)]
pub enum Failures {
    #[string(name = "MASTER_CAUTION_ACKNOWLEDGE")]
    MasterCautionAck,
    #[string(name = "MASTER_CAUTION_OFF")]
    MasterCautionOff,
    #[string(name = "MASTER_CAUTION_ON")]
    MasterCautionOn,
    #[string(name = "MASTER_CAUTION_SET")]
    MasterCautionSet,
    #[string(name = "MASTER_CAUTION_TOGGLE")]
    MasterCautionToggle,
    #[string(name = "MASTER_WARNING_ACKNOWLEDGE")]
    MasterWarningAck,
    #[string(name = "MASTER_WARNING_OFF")]
    MasterWarningOff,
    #[string(name = "MASTER_WARNING_ON")]
    MasterWarningOn,
    #[string(name = "MASTER_WARNING_SET")]
    MasterWarningSet,
    #[string(name = "MASTER_WARNING_TOGGLE")]
    MasterWarningToggle,
}
