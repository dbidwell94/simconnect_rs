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
    #[string("Category")]
    AirplaneCategory,
    Realism,
    #[string("Sim Disabled")]
    SimDisabled,
    #[string("Sim On Ground")]
    SimOnGround,
    #[string("Barometer Pressure")]
    BarometerPressure,
    #[string("Fuel Dump Active")]
    FuelDumpActive,
    #[string("Fuel Left Capacity")]
    FuelLeftCapacity,
    #[string("Fuel Left Quantity")]
    FuelLeftQuantity,
    #[string("Fuel Right Capacity")]
    FuelRightCapacity,
    #[string("Fuel Right Quantity")]
    FuelRightQuantity,
    #[string("Fuel Total Quantity")]
    FuelTotalQuantity,
    #[string("Fuel Total Capacity")]
    FuelTotalCapacity,
    #[string("Fuel Total Quantity Weight")]
    FuelTotalQuantityWeight,
    #[string("Unlimited Fuel")]
    IsUnlimitedFuelSet,
    #[string("Autobrakes Active")]
    AutobrakesActive,
    #[string("Antiskid Brakes Active")]
    AntiskidBrakesActive,
    #[string("Brake Parking Position")]
    ParkingBrakeEnabled,
    #[string("Light Strobe")]
    LightStobeEnabled,
    #[string("Light Landing")]
    LightLandingEnabled,
    #[string("Light Taxi")]
    LightTaxiEnabled,
    #[string("Light Beacon")]
    LightBeaconEnabled,
    #[string("Light Nav")]
    LightNavEnabled,
    #[string("Light Logo")]
    LightLogoEnabled,
    #[string("Light Wing")]
    LightWingEnabled,
}
