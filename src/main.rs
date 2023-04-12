mod sim_connect;
use std::{ffi::CString, thread, time::Duration};

use anyhow::Result as AnyhowResult;
use sim_connect::SimConnect;

pub const PROGRAM_NAME: &'static str = "MSFS_EconoWorld";

struct TestDataStruct {
    name: u8,
}

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    let sim = SimConnect::open(CString::new(PROGRAM_NAME).unwrap().as_c_str())?;
    sim.create_client_data::<TestDataStruct>(CString::new("Test_Data_Struct").unwrap().as_c_str())?;

    thread::sleep(Duration::new(30, 0));

    Ok(())
}
