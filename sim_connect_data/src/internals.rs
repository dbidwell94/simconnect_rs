use std::ffi::CString;

pub trait IterEnum {
    type Item;
    fn iter_enum() -> std::vec::IntoIter<Self::Item>;
}

pub trait ToSimConnect {
    fn sc_string(&self) -> CString;
}
