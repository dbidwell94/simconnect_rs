use std::ffi::CString;

use super::ToSimConnect;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum SimVar {
    KohlsmanHG,
    IndicatedAlt,
    PlaneLat,
    PlaneLong,
    AirspeedIndicated,
    AirspeedMach,
    AirspeedTrue,
}

impl std::fmt::Display for SimVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimVar::KohlsmanHG => write!(f, "Kohlsman setting hg"),
            SimVar::IndicatedAlt => write!(f, "Indicated Altitude"),
            SimVar::PlaneLat => write!(f, "Plane Latitude"),
            SimVar::PlaneLong => write!(f, "Plane Longitude"),
            SimVar::AirspeedIndicated => write!(f, "Airspeed Indicated"),
            SimVar::AirspeedMach => write!(f, "Airspeed Mach"),
            SimVar::AirspeedTrue => write!(f, "Airspeed True"),
        }
    }
}

impl ToSimConnect for SimVar {
    fn sc_string(&self) -> CString {
        CString::new(format!("{self}")).unwrap()
    }
}
