use std::{ffi::CString, hash::Hash};

use super::ToSimConnect;

pub trait SimUnit: ToSimConnect {}

/* #region Length */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Length {
    Meter,
    Millimeter,
    Centimeter,
    Kilometer,
    NauticalMile,
    Decinmile,
    Inch,
    Foot,
    Yard,
    Decimile,
    Mile,
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Meter => write!(f, "meter"),
            Length::Millimeter => write!(f, "millimeter"),
            Length::Centimeter => write!(f, "centimeter"),
            Length::Kilometer => write!(f, "kilometer"),
            Length::NauticalMile => write!(f, "nmile"),
            Length::Decinmile => write!(f, "decinmile"),
            Length::Inch => write!(f, "inch"),
            Length::Foot => write!(f, "foot"),
            Length::Yard => write!(f, "yard"),
            Length::Decimile => write!(f, "decimile"),
            Length::Mile => write!(f, "mile"),
        }
    }
}

impl ToSimConnect for Length {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for Length {}
/* #endregion */

/* #region Area  */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Area {
    SqIn,
    SqFt,
    SqYd,
    SqMi,
    SqMm,
    SqCm,
    SqM,
    SqKm,
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Area::SqIn => write!(f, "square inch"),
            Area::SqFt => write!(f, "square feet"),
            Area::SqYd => write!(f, "square yard"),
            Area::SqMi => write!(f, "square mile"),
            Area::SqMm => write!(f, "square millimeter"),
            Area::SqCm => write!(f, "square centimeter"),
            Area::SqM => write!(f, "square meter"),
            Area::SqKm => write!(f, "square kilometer"),
        }
    }
}

impl ToSimConnect for Area {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for Area {}
/* #endregion */

/* #region Volume */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Volume {
    Cin,
    Cft,
    Cyd,
    Cmi,
    Cmm,
    Cm,
    Ckm,
    L,
    Gal,
    Qt,
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Volume::Cin => write!(f, "cubic inch"),
            Volume::Cft => write!(f, "cubic foot"),
            Volume::Cyd => write!(f, "cubic yard"),
            Volume::Cmi => write!(f, "cubic mile"),
            Volume::Cmm => write!(f, "cubic millimeter"),
            Volume::Cm => write!(f, "cubic meter"),
            Volume::Ckm => write!(f, "cubic kilometer"),
            Volume::L => write!(f, "liter"),
            Volume::Gal => write!(f, "gallon"),
            Volume::Qt => write!(f, "quart"),
        }
    }
}

impl ToSimConnect for Volume {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for Volume {}
/* #endregion */

/* #region Temp */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Temp {
    Kel,
    Rank,
    F,
    C,
}

impl std::fmt::Display for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Temp::Kel => write!(f, "kelvin"),
            Temp::Rank => write!(f, "rankine"),
            Temp::F => write!(f, "farenheit"),
            Temp::C => write!(f, "celsius"),
        }
    }
}

impl ToSimConnect for Temp {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for Temp {}
/* #endregion */

/* #region Angle */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Angle {
    Rad,
    Round,
    Deg,
    Grad,
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::Rad => write!(f, "radian"),
            Angle::Round => write!(f, "round"),
            Angle::Deg => write!(f, "degree"),
            Angle::Grad => write!(f, "grad"),
        }
    }
}

impl ToSimConnect for Angle {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for Angle {}
/* #endregion */

/* #region GPS */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum GPS {
    DegLat,
    DegLon,
    MetLat,
}

impl std::fmt::Display for GPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GPS::DegLat => write!(f, "degree latitude"),
            GPS::DegLon => write!(f, "degree longitude"),
            GPS::MetLat => write!(f, "meter latitude"),
        }
    }
}

impl ToSimConnect for GPS {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for GPS {}
/* #endregion */

/* #region AngularVelocity */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum AngularVelocity {
    RPS,
    RPM,
    DPS,
}

impl std::fmt::Display for AngularVelocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AngularVelocity::RPS => write!(f, "radian per second"),
            AngularVelocity::RPM => write!(f, "revolution per minute"),
            AngularVelocity::DPS => write!(f, "degree per second"),
        }
    }
}

impl ToSimConnect for AngularVelocity {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for AngularVelocity {}
/* #endregion */

/* #region Speed */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Speed {
    MPS,
    MPM,
    KPH,
    FPS,
    FPM,
    MPH,
    KNT,
    MAC,
}

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Speed::MPS => write!(f, "meter per second"),
            Speed::MPM => write!(f, "meter per minute"),
            Speed::KPH => write!(f, "kilometers per hour"),
            Speed::FPS => write!(f, "feet/second"),
            Speed::FPM => write!(f, "feet/minute"),
            Speed::MPH => write!(f, "mile per hour"),
            Speed::KNT => write!(f, "knot"),
            Speed::MAC => write!(f, "mach"),
        }
    }
}

impl ToSimConnect for Speed {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for Speed {}
/* #endregion */

/* #region Pressure */
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Pressure {
    Pa,
    Kpa,
    MmHg,
    CmHg,
    InHg,
    Bar,
    Atm,
    Psi,
    BoostPsi,
    BoostInHg,
    BoostCmHg,
}

impl std::fmt::Display for Pressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pressure::Pa => write!(f, "pascal"),
            Pressure::Kpa => write!(f, "kilopascal"),
            Pressure::MmHg => write!(f, "millimeter of mercury"),
            Pressure::CmHg => write!(f, "centimeter of mercury"),
            Pressure::InHg => write!(f, "inch of mercury"),
            Pressure::Bar => write!(f, "bar"),
            Pressure::Atm => write!(f, "atmosphere"),
            Pressure::Psi => write!(f, "psi"),
            Pressure::BoostPsi => write!(f, "boost psi"),
            Pressure::BoostInHg => write!(f, "boost inHg"),
            Pressure::BoostCmHg => write!(f, "boost cmHg"),
        }
    }
}

impl ToSimConnect for Pressure {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}

impl SimUnit for Pressure {}

/* #endregion */

pub struct FuelLevels {
    pub center: f32,
    pub left_main: f32,
    pub right_main: f32,
    pub left_aux: f32,
    pub right_aux: f32,
    pub left_tip: f32,
    pub right_tip: f32,
    pub center2: f32,
    pub center3: f32,
    pub external1: f32,
    pub external2: f32,
}
