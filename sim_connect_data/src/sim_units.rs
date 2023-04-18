use sim_connect_macros::{SimUnit, ToSimConnect};
use std::{ffi::CString, hash::Hash};

use super::internals::ToSimConnect;

pub trait SimUnit: ToSimConnect {}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
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

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Area {
    #[string(name = "square inch")]
    SqIn,
    #[string(name = "square feet")]
    SqFt,
    #[string(name = "square yard")]
    SqYd,
    #[string(name = "square mile")]
    SqMi,
    #[string(name = "square millimeter")]
    SqMm,
    #[string(name = "square centimeter")]
    SqCm,
    #[string(name = "square meter")]
    SqM,
    #[string(name = "square kilometer")]
    SqKm,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Volume {
    #[string(name = "cubic inch")]
    Cin,
    #[string(name = "cubit foot")]
    Cft,
    #[string(name = "cubic yard")]
    Cyd,
    #[string(name = "cubic mile")]
    Cmi,
    #[string(name = "cubic millimeter")]
    Cmm,
    #[string(name = "cubic meter")]
    Cm,
    #[string(name = "cubic kilometer")]
    Ckm,
    #[string(name = "liter")]
    L,
    #[string(name = "gallon")]
    Gal,
    #[string(name = "quart")]
    Qt,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Temp {
    #[string(name = "kelvin")]
    Kel,
    #[string(name = "rankine")]
    Rank,
    #[string(name = "farenheit")]
    F,
    #[string(name = "celsius")]
    C,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Angle {
    #[string(name = "radian")]
    Rad,
    #[string(name = "round")]
    Round,
    #[string(name = "degree")]
    Deg,
    #[string(name = "grad")]
    Grad,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum GPS {
    #[string(name = "degree latitude")]
    DegLat,
    #[string(name = "degree longitude")]
    DegLon,
    #[string(name = "meter latitude")]
    MetLat,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum AngularVelocity {
    #[string(name = "radian per second")]
    RPS,
    #[string(name = "revolution per minute")]
    RPM,
    #[string(name = "degree per second")]
    DPS,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Speed {
    #[string(name = "meter per second")]
    MPS,
    #[string(name = "meter per minute")]
    MPM,
    #[string(name = "kilometers per hour")]
    KPH,
    #[string(name = "feet/second")]
    FPS,
    #[string(name = "feet/minute")]
    FPM,
    #[string(name = "mile per hour")]
    MPH,
    #[string(name = "knot")]
    KNT,
    #[string(name = "mach")]
    MAC,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Pressure {
    #[string(name = "pascal")]
    Pa,
    #[string(name = "kilopascal")]
    Kpa,
    #[string(name = "millimeter of mercury")]
    MmHg,
    #[string(name = "centimeter of mercury")]
    CmHg,
    #[string(name = "inch of mercury")]
    InHg,
    #[string(name = "bar")]
    Bar,
    #[string(name = "atmosphere")]
    Atm,
    #[string(name = "psi")]
    Psi,
    #[string(name = "boost psi")]
    BoostPsi,
    #[string(name = "boost inHg")]
    BoostInHg,
    #[string(name = "boost cmHg")]
    BoostCmHg,
}

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
