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

    /* #region Fuel Systems */
    #[string(name = "Fuel Dump Active")]
    FuelDumpActive,

    /* #region Fuel Tanks */
    #[string(name = "FUEL TANK CENTER LEVEL")]
    FuelTankCenterLevel,
    #[string(name = "FUEL TANK CENTER2 LEVEL")]
    FuelTankCenter2Level,
    #[string(name = "FUEL TANK CENTER3 LEVEL")]
    FuelTankCenter3Level,
    #[string(name = "FUEL TANK EXTERNAL1 LEVEL")]
    FuelTankExternal1Level,
    #[string(name = "FUEL TANK EXTERNAL2 LEVEL")]
    FuelTankExternal2Level,
    #[string(name = "FUEL TANK LEFT AUX LEVEL")]
    FuelTankLeftAuxLevel,
    #[string(name = "FUEL TANK LEFT MAIN LEVEL")]
    FuelTankLeftMainLevel,
    #[string(name = "FUEL TANK LEFT TIP LEVEL")]
    FuelTankLeftTipLevel,
    #[string(name = "FUEL TANK RIGHT AUX LEVEL")]
    FuelTankRightAuxLevel,
    #[string(name = "FUEL TANK RIGHT MAIN LEVEL")]
    FuelTankRightMainLevel,
    #[string(name = "FUEL TANK RIGHT TIP LEVEL")]
    FuelTankRightTipLevel,
    /* #endregion */
    #[string(name = "Fuel Total Quantity")]
    FuelTotalQuantity,
    #[string(name = "Fuel Total Capacity")]
    FuelTotalCapacity,
    #[string(name = "Fuel Total Quantity Weight")]
    FuelTotalQuantityWeight,
    #[string(name = "Unlimited Fuel")]
    IsUnlimitedFuelSet,

    /* #endregion */
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
    Title,
    #[string(name = "Vertical Speed")]
    VerticalSpeed,
    #[string(name = "ZULU TIME")]
    ZuluTime,
}
