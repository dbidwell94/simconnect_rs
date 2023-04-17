use sim_connect_macros::{SimUnit, ToSimConnect};
use std::{ffi::CString, hash::Hash};

use super::ToSimConnect;

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
    #[string("square inch")]
    SqIn,
    #[string("square feet")]
    SqFt,
    #[string("square yard")]
    SqYd,
    #[string("square mile")]
    SqMi,
    #[string("square millimeter")]
    SqMm,
    #[string("square centimeter")]
    SqCm,
    #[string("square meter")]
    SqM,
    #[string("square kilometer")]
    SqKm,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Volume {
    #[string("cubic inch")]
    Cin,
    #[string("cubit foot")]
    Cft,
    #[string("cubic yard")]
    Cyd,
    #[string("cubic mile")]
    Cmi,
    #[string("cubic millimeter")]
    Cmm,
    #[string("cubic meter")]
    Cm,
    #[string("cubic kilometer")]
    Ckm,
    #[string("liter")]
    L,
    #[string("gallon")]
    Gal,
    #[string("quart")]
    Qt,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Temp {
    #[string("kelvin")]
    Kel,
    #[string("rankine")]
    Rank,
    #[string("farenheit")]
    F,
    #[string("celsius")]
    C,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Angle {
    #[string("radian")]
    Rad,
    #[string("round")]
    Round,
    #[string("degree")]
    Deg,
    #[string("grad")]
    Grad,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum GPS {
    #[string("degree latitude")]
    DegLat,
    #[string("degree longitude")]
    DegLon,
    #[string("meter latitude")]
    MetLat,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum AngularVelocity {
    #[string("radian per second")]
    RPS,
    #[string("revolution per minute")]
    RPM,
    #[string("degree per second")]
    DPS,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Speed {
    #[string("meter per second")]
    MPS,
    #[string("meter per minute")]
    MPM,
    #[string("kilometers per hour")]
    KPH,
    #[string("feet/second")]
    FPS,
    #[string("feet/minute")]
    FPM,
    #[string("mile per hour")]
    MPH,
    #[string("knot")]
    KNT,
    #[string("mach")]
    MAC,
}

#[derive(Hash, PartialEq, Eq, Debug, SimUnit, ToSimConnect)]
pub enum Pressure {
    #[string("pascal")]
    Pa,
    #[string("kilopascal")]
    Kpa,
    #[string("millimeter of mercury")]
    MmHg,
    #[string("centimeter of mercury")]
    CmHg,
    #[string("inch of mercury")]
    InHg,
    #[string("bar")]
    Bar,
    #[string("atmosphere")]
    Atm,
    #[string("psi")]
    Psi,
    #[string("boost psi")]
    BoostPsi,
    #[string("boost inHg")]
    BoostInHg,
    #[string("boost cmHg")]
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
