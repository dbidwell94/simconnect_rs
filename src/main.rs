mod sim_connect;
use std::{ffi::CString, thread, time::Duration};

use anyhow::Result as AnyhowResult;
use sim_connect::{
    sim_events::SystemEvent,
    sim_units::{Angle, Length, Pressure},
    sim_var_types::SimVarType,
    sim_vars::SimVar,
    SimConnect, SimConnectDatum, ToSimConnectStruct,
};

pub const PROGRAM_NAME: &'static str = "MSFS_EconoWorld";

#[derive(Default)]
pub struct TestStruct {
    pub kohlsmann: f32,
    pub altitude: i32,
    pub latitude: f32,
    pub longitude: f32,
}

impl ToSimConnectStruct for TestStruct {
    fn get_fields() -> Vec<SimConnectDatum> {
        vec![
            SimConnectDatum {
                id: 0,
                sim_var: SimVar::KohlsmanHG,
                sim_unit: Box::new(Pressure::InHg),
                data_type: SimVarType::F32,
            },
            SimConnectDatum {
                id: 1,
                sim_var: SimVar::IndicatedAlt,
                sim_unit: Box::new(Length::Foot),
                data_type: SimVarType::I32,
            },
            SimConnectDatum {
                id: 2,
                sim_var: SimVar::PlaneLat,
                sim_unit: Box::new(Angle::Deg),
                data_type: SimVarType::F32,
            },
            SimConnectDatum {
                id: 3,
                sim_var: SimVar::PlaneLong,
                sim_unit: Box::new(Angle::Deg),
                data_type: SimVarType::F32,
            },
        ]
    }
}

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    let mut sim = SimConnect::open(CString::new(PROGRAM_NAME).unwrap().as_c_str())?;
    sim.subscribe_to_system_event(SystemEvent::FourSec)?;
    sim.register_struct::<TestStruct>()?;

    loop {
        sim.check_events()?;
        thread::sleep(Duration::new(1, 0));
    }

    Ok(())
}
