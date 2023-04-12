mod sim_connect;
use std::{ffi::CString, thread, time::Duration};

use anyhow::Result as AnyhowResult;
use sim_connect::{SimConnect, sim_events::SystemEvent};

pub const PROGRAM_NAME: &'static str = "MSFS_EconoWorld";

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    let sim = SimConnect::open(CString::new(PROGRAM_NAME).unwrap().as_c_str())?;
    sim.subscribe_to_system_event(SystemEvent::FourSec)?;

    loop {
        sim.check_events()?;
        thread::sleep(Duration::new(1, 0));
    }

    Ok(())
}
