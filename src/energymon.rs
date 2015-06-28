use libc::{c_int, c_double, c_char, c_void};

pub type EnergyMonInitFn = unsafe extern fn(em: *mut EnergyMon) -> c_int;
pub type EnergyMonReadTotalFn = unsafe extern fn(em: *mut EnergyMon) -> c_double;
pub type EnergyMonFinishFn = unsafe extern fn(em: *mut EnergyMon) -> c_int;
pub type EnergyMonGetSourceFn = unsafe extern fn(*mut c_char) -> *mut c_char;

#[repr(C)]
pub struct EnergyMon {
    pub finit: EnergyMonInitFn,
    pub fread: EnergyMonReadTotalFn,
    pub ffinish: EnergyMonFinishFn,
    pub fsource: EnergyMonGetSourceFn,
    pub state: *mut c_void,
}
