mod internals;
pub mod recv_data;
pub mod sim_event_args;
pub mod sim_events;
pub mod sim_input_events;
pub mod sim_units;
pub mod sim_var_types;
pub mod sim_vars;

use std::ptr::NonNull;

pub use internals::ToSimConnect;
use sim_units::SimUnit;
use sim_var_types::SimVarType;
use sim_vars::SimVar;

/// # Description
/// Auto-implement `StructToSimConnect`.
///
/// # Notes
///
/// - Required enums must be in scope when specifying them in the `#[datum(..)]` attribute
/// - Id's cannot be re-used in the same struct. This will create undefined behaviour
/// - Your data type will be automatically converted if the data type is supported. Current supported data types are:
///     - i32
///     - i64
///     - f32
///     - f64
///     - String
///
/// # Example
///
/// ```
///     use sim_connect_rs::{
///         sim_units::{Length, Speed},
///         sim_vars::SimVar,
///         StructToSimConnect,
///     };
///
///     #[derive(StructToSimConnect)]
///     struct TestStruct {
///         #[datum(
///             sim_var = "SimVar::AirspeedTrue",
///             sim_unit = "Speed::KNT",
///         )]
///         airspeed: i32,
///         #[datum(
///              sim_var = "SimVar::IndicatedAlt",
///              sim_unit = "Length::Foot",
///         )]
///         altitude: f32,
///}
/// ```
pub trait StructToSimConnect: Clone + Sized {
    fn get_fields() -> Vec<SimConnectDatum>;
}

pub trait SimConnectToStruct: StructToSimConnect {
    type Error;
    type ReturnType;
    unsafe fn parse_struct(pointer: NonNull<u32>) -> Result<Self::ReturnType, Self::Error>;
}

pub struct SimConnectDatum {
    pub id: u32,
    pub sim_var: SimVar,
    pub sim_unit: Option<Box<dyn SimUnit>>,
    pub data_type: SimVarType,
}
