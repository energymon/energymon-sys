//! FFI bindings for `energymon.h`.

#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, uint64_t, c_char, c_void, size_t};

/// Typedef for initialization function.
pub type energymon_init = extern fn(*mut energymon) -> c_int;
/// Typedef for read function.
pub type energymon_read_total = extern fn(*const energymon) -> uint64_t;
/// Typedef for cleanup function.
pub type energymon_finish = extern fn(*mut energymon) -> c_int;
/// Typedef for function to get human-readable name.
pub type energymon_get_source = extern fn(*mut c_char, size_t) -> *mut c_char;
/// Typedef for function to get refresh interval.
pub type energymon_get_interval = extern fn(*const energymon) -> uint64_t;
/// Typedef for function to get read precision.
pub type energymon_get_precision = extern fn(*const energymon) -> uint64_t;
/// Typedef for function to get refresh interval.
pub type energymon_is_exclusive = extern fn() -> c_int;

#[repr(C)]
/// Representation of native C struct `energymon`.
pub struct energymon {
    /// Native C function type signature that initializes the `energymon`.
    pub finit: energymon_init,
    /// Native C function type signature that reads energy data from the `energymon`.
    pub fread: energymon_read_total,
    /// Native C function type signature that cleans up the `energymon`.
    pub ffinish: energymon_finish,
    /// Native C function type signature that gets the `energymon`'s human-readable name.
    pub fsource: energymon_get_source,
    /// Native C function type signature that gets the `energymon`'s refresh interval.
    pub finterval: energymon_get_interval,
    /// Native C function type signature that gets the `energymon`'s precision.
    pub fprecision: energymon_get_precision,
    /// Native C function type signature that gets the `energymon`'s exclusiveness requirement.
    pub fexclusive: energymon_is_exclusive,
    /// Native C pointer used by the underlying `energymon` implementation for storing state.
    pub state: *mut c_void,
}
