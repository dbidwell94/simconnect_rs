use sim_connect_macros::ToSimConnect;
use std::ffi::CString;

use super::internals::ToSimConnect;

#[derive(Hash, PartialEq, Eq, Debug, ToSimConnect)]
pub enum SimVar {
    #[string(name = "Kohlsman Setting hg")]
    KohlsmanHG,
    #[string(name = "Indicated Altitude")]
    IndicatedAlt,
    #[string(name = "Plane Latitude")]
    PlaneLat,
    #[string(name = "Plane Longitude")]
    PlaneLong,
    #[string(name = "Airspeed Indicated")]
    AirspeedIndicated,
    #[string(name = "Airspeed Mach")]
    AirspeedMach,
    #[string(name = "Airspeed True")]
    AirspeedTrue,
    #[string(name = "Category")]
    AirplaneCategory,
    Realism,
    #[string(name = "Sim Disabled")]
    SimDisabled,
    #[string(name = "Sim On Ground")]
    SimOnGround,
    #[string(name = "Barometer Pressure")]
    BarometerPressure,
    #[string(name = "Fuel Dump Active")]
    FuelDumpActive,
    #[string(name = "Fuel Left Capacity")]
    FuelLeftCapacity,
    #[string(name = "Fuel Left Quantity")]
    FuelLeftQuantity,
    #[string(name = "Fuel Right Capacity")]
    FuelRightCapacity,
    #[string(name = "Fuel Right Quantity")]
    FuelRightQuantity,
    #[string(name = "Fuel Total Quantity")]
    FuelTotalQuantity,
    #[string(name = "Fuel Total Capacity")]
    FuelTotalCapacity,
    #[string(name = "Fuel Total Quantity Weight")]
    FuelTotalQuantityWeight,
    #[string(name = "Unlimited Fuel")]
    IsUnlimitedFuelSet,
    #[string(name = "Autobrakes Active")]
    AutobrakesActive,
    #[string(name = "Antiskid Brakes Active")]
    AntiskidBrakesActive,
    #[string(name = "Brake Parking Position")]
    ParkingBrakeEnabled,
    #[string(name = "Light Strobe")]
    LightStobeEnabled,
    #[string(name = "Light Landing")]
    LightLandingEnabled,
    #[string(name = "Light Taxi")]
    LightTaxiEnabled,
    #[string(name = "Light Beacon")]
    LightBeaconEnabled,
    #[string(name = "Light Nav")]
    LightNavEnabled,
    #[string(name = "Light Logo")]
    LightLogoEnabled,
    #[string(name = "Light Wing")]
    LightWingEnabled,
}
