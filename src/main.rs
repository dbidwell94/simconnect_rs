mod sim_connect;
use std::{ffi::CString, thread, time::Duration};

use anyhow::{anyhow, Result as AnyhowResult};
use sim_connect::{
    recv_data::RecvDataEvent, sim_events::SystemEvent, sim_units::Length,
    sim_var_types::SimVarType, sim_vars::SimVar, SimConnect, SimConnectDatum, SimConnectToStruct,
    ToSimConnectStruct,
};

use crate::sim_connect::sim_units::{Speed, Angle};

pub const PROGRAM_NAME: &'static str = "MSFS_EconoWorld";

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct TestStruct {
    pub air_speed: f32,
    pub altitude: f32,
    pub latitude: f32,
    pub longitude: f32,
}

impl ToSimConnectStruct for TestStruct {
    fn get_fields() -> Vec<SimConnectDatum> {
        vec![
            SimConnectDatum {
                id: 1,
                sim_var: SimVar::AirspeedMach,
                sim_unit: Box::new(Speed::KNT),
                data_type: SimVarType::F32,
            },
            SimConnectDatum {
                id: 2,
                sim_var: SimVar::IndicatedAlt,
                sim_unit: Box::new(Length::Foot),
                data_type: SimVarType::F32,
            },
            SimConnectDatum {
                id: 3,
                sim_var: SimVar::PlaneLat,
                sim_unit: Box::new(Angle::Deg),
                data_type: SimVarType::F32,
            },
            SimConnectDatum {
                id: 4,
                sim_var: SimVar::PlaneLong,
                sim_unit: Box::new(Angle::Deg),
                data_type: SimVarType::F32,
            },
        ]
    }
}

fn main() -> AnyhowResult<()> {
    let mut sim = SimConnect::open(CString::new(PROGRAM_NAME).unwrap().as_c_str())?;
    sim.subscribe_to_system_event(SystemEvent::FourSec)?;
    sim.register_struct::<TestStruct>()?;
    sim.request_data_on_self_object::<TestStruct>()?;
    let mut got_data = false;

    loop {
        let data = sim.check_events()?;

        if let Some(data) = data {
            if let RecvDataEvent::Data(data) = data {
                let data = data.to_struct::<TestStruct>()?;
                println!("Data recieved: {data:?}");
                got_data = true;
            }
        }
        if got_data {
            got_data = false;
            sim.request_data_on_self_object::<TestStruct>()?;
        }

        thread::sleep(Duration::new(1, 0));
    }

    Ok(())
}
