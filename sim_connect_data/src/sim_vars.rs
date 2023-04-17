use sim_connect_macros::ToSimConnect;
use std::ffi::CString;

use super::ToSimConnect;

#[derive(Hash, PartialEq, Eq, Debug, ToSimConnect)]
pub enum SimVar {
    #[string("Kohlsman Setting hg")]
    KohlsmanHG,
    #[string("Indicated Altitude")]
    IndicatedAlt,
    #[string("Plane Latitude")]
    PlaneLat,
    #[string("Plane Longitude")]
    PlaneLong,
    #[string("Airspeed Indicated")]
    AirspeedIndicated,
    #[string("Airspeed Mach")]
    AirspeedMach,
    #[string("Airspeed True")]
    AirspeedTrue,
}
