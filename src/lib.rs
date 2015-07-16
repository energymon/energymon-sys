extern crate libc;

mod wrapper;

pub mod energymon;
pub mod singleton;

use libc::{c_int, c_ulonglong, c_char, c_void};

/// Typedef for initialization function.
pub type EnergyMonInitFn = extern fn(*mut EMImpl) -> c_int;
/// Typedef for read function.
pub type EnergyMonReadTotalFn = extern fn(*const EMImpl) -> c_ulonglong;
/// Typedef for cleanup function.
pub type EnergyMonFinishFn = extern fn(*mut EMImpl) -> c_int;
/// Typedef for function to get human-readable name.
pub type EnergyMonGetSourceFn = extern fn(*mut c_char) -> *mut c_char;
/// Typedef for function to get refresh interval.
pub type EnergyMonGetIntervalFn = extern fn(*const EMImpl) -> c_ulonglong;

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
/// Representation of native C struct `em_impl`.
pub struct EMImpl {
    /// Native C function type signature that initializes the `EMImpl`.
    pub finit: EnergyMonInitFn,
    /// Native C function type signature that reads energy data from the `EMImpl`.
    pub fread: EnergyMonReadTotalFn,
    /// Native C function type signature that cleans up the `EMImpl`.
    pub ffinish: EnergyMonFinishFn,
    /// Native C function type signature that gets the `EMImpl`'s human-readable name.
    pub fsource: EnergyMonGetSourceFn,
    /// Native C function type signature that gets the `EMImpl`'s refresh interval.
    pub finterval: EnergyMonGetIntervalFn,
    /// Native C pointer used by the underlying `EMImpl` implementation for storing state.
    pub state: *mut c_void,
}

extern "C" {
    /// Native C function that fills in the EMImpl struct values and may allocate other resources.
    pub fn em_impl_get(em: *mut EMImpl) -> c_int;
}
