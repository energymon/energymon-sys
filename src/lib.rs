#![allow(non_camel_case_types)]

extern crate libc;

mod wrapper;

pub mod energymon;
pub mod singleton;

use libc::{c_int, c_ulonglong, c_char, c_void};

/// Typedef for initialization function.
pub type em_init_func = extern fn(*mut em_impl) -> c_int;
/// Typedef for read function.
pub type em_read_total_func = extern fn(*const em_impl) -> c_ulonglong;
/// Typedef for cleanup function.
pub type em_finish_func = extern fn(*mut em_impl) -> c_int;
/// Typedef for function to get human-readable name.
pub type em_get_source_func = extern fn(*mut c_char) -> *mut c_char;
/// Typedef for function to get refresh interval.
pub type em_get_interval_func = extern fn(*const em_impl) -> c_ulonglong;

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
/// Representation of native C struct `em_impl`.
pub struct em_impl {
    /// Native C function type signature that initializes the `em_impl`.
    pub finit: em_init_func,
    /// Native C function type signature that reads energy data from the `em_impl`.
    pub fread: em_read_total_func,
    /// Native C function type signature that cleans up the `em_impl`.
    pub ffinish: em_finish_func,
    /// Native C function type signature that gets the `em_impl`'s human-readable name.
    pub fsource: em_get_source_func,
    /// Native C function type signature that gets the `em_impl`'s refresh interval.
    pub finterval: em_get_interval_func,
    /// Native C pointer used by the underlying `em_impl` implementation for storing state.
    pub state: *mut c_void,
}

extern "C" {
    /// Native C function that fills in the em_impl struct values and may allocate other resources.
    pub fn em_impl_get(em: *mut em_impl) -> c_int;
}
