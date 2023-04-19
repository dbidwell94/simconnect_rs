use crate::internals::{IterEnum, ToSimConnect};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use sim_connect_macros::{FromStr, InputEvent, IterEnum, ToSimConnect};
use std::ffi::CString;

pub trait InputEvent: ToSimConnect + std::str::FromStr + TryFromPrimitive + Into<u32> {}

#[derive(
    ToSimConnect, Clone, Copy, InputEvent, TryFromPrimitive, IntoPrimitive, IterEnum, FromStr,
)]
#[repr(u32)]
pub enum GearAndBrakes {
    #[string(name = "ANTISKID_BRAKES_TOGGLE")]
    AntiskidBrakeToggled = 0,
    #[string(name = "BRAKES")]
    Brakes = 1,
    #[string(name = "BRAKES_LEFT")]
    BrakesLeft = 2,
    #[string(name = "BRAKES_RIGHT")]
    BrakesRight = 3,
    #[string(name = "GEAR_DOWN")]
    GearDown = 4,
    #[string(name = "GEAR_EMERGENCY_HANDLE_TOGGLE")]
    GearEmergencyHandleToggled = 5,
    #[string(name = "GEAR_PUMP")]
    GearPump = 6,
    #[string(name = "GEAR_UP")]
    GearUp = 7,
    #[string(name = "PARKING_BRAKES")]
    ParkingBrakes = 8,
}

#[derive(
    Clone, Copy, ToSimConnect, InputEvent, TryFromPrimitive, IntoPrimitive, IterEnum, FromStr,
)]
#[repr(u32)]
pub enum Failures {
    #[string(name = "MASTER_CAUTION_ACKNOWLEDGE")]
    MasterCautionAck = 9,
    #[string(name = "MASTER_CAUTION_OFF")]
    MasterCautionOff = 10,
    #[string(name = "MASTER_CAUTION_ON")]
    MasterCautionOn = 11,
    #[string(name = "MASTER_CAUTION_SET")]
    MasterCautionSet = 12,
    #[string(name = "MASTER_CAUTION_TOGGLE")]
    MasterCautionToggle = 13,
    #[string(name = "MASTER_WARNING_ACKNOWLEDGE")]
    MasterWarningAck = 14,
    #[string(name = "MASTER_WARNING_OFF")]
    MasterWarningOff = 15,
    #[string(name = "MASTER_WARNING_ON")]
    MasterWarningOn = 16,
    #[string(name = "MASTER_WARNING_SET")]
    MasterWarningSet = 17,
    #[string(name = "MASTER_WARNING_TOGGLE")]
    MasterWarningToggle = 18,
}
