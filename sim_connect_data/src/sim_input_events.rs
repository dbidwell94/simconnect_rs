use crate::ToSimConnect;
use sim_connect_macros::{InputEvent, ToSimConnect};
use std::ffi::CString;

pub trait InputEvent {}

#[derive(ToSimConnect, Clone, Copy, InputEvent)]
pub enum GearAndBrakes {
    #[string("ANTISKID_BRAKES_TOGGLE")]
    AntiskidBrakeToggled,
    #[string("BRAKES")]
    Brakes,
    #[string("BRAKES_LEFT")]
    BrakesLeft,
    #[string("BRAKES_RIGHT")]
    BrakesRight,
    #[string("GEAR_DOWN")]
    GearDown,
    #[string("GEAR_EMERGENCY_HANDLE_TOGGLE")]
    GearEmergencyHandleToggled,
    #[string("GEAR_PUMP")]
    GearPump,
    #[string("GEAR_UP")]
    GearUp,
    #[string("PARKING_BRAKES")]
    ParkingBrakes,
}

#[derive(Clone, Copy, ToSimConnect, InputEvent)]
pub enum Failures {
    #[string("MASTER_CAUTION_ACKNOWLEDGE")]
    MasterCautionAck,
    #[string("MASTER_CAUTION_OFF")]
    MasterCautionOff,
    #[string("MASTER_CAUTION_ON")]
    MasterCautionOn,
    #[string("MASTER_CAUTION_SET")]
    MasterCautionSet,
    #[string("MASTER_CAUTION_TOGGLE")]
    MasterCautionToggle,
    #[string("MASTER_WARNING_ACKNOWLEDGE")]
    MasterWarningAck,
    #[string("MASTER_WARNING_OFF")]
    MasterWarningOff,
    #[string("MASTER_WARNING_ON")]
    MasterWarningOn,
    #[string("MASTER_WARNING_SET")]
    MasterWarningSet,
    #[string("MASTER_WARNING_TOGGLE")]
    MasterWarningToggle,
}
