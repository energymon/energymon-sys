//! FFI bindings for `energymon-default.h`.

extern crate libc;
extern crate energymon_sys;

pub use energymon_sys::energymon;
use libc::{c_int};

extern "C" {
    /// Native C function that fills in the energymon struct values.
    pub fn energymon_get_default(em: *mut energymon) -> c_int;
}
