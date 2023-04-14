use std::ffi::CString;

pub mod sim_units;
pub mod sim_vars;
pub mod sim_var_types;
pub mod recv_data;
pub mod sim_events;

pub trait ToSimConnect {
    fn sc_string(&self) -> CString;
}
