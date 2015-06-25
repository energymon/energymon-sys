use libc::{c_int, c_double, c_char};

pub type EnergyMonInitFn = unsafe extern fn() -> c_int;
pub type EnergyMonReadTotalFn = unsafe extern fn() -> c_double;
pub type EnergyMonFinishFn = unsafe extern fn() -> c_int;
pub type EnergyMonGetSourceFn = unsafe extern fn(*mut c_char) -> *mut c_char;

#[repr(C)]
pub struct EnergyMon {
    pub finit: EnergyMonInitFn,
    pub fread: EnergyMonReadTotalFn,
    pub ffinish: EnergyMonFinishFn,
    pub fsource: EnergyMonGetSourceFn,
}
