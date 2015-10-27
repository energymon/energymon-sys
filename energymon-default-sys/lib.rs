//! FFI bindings for `energymon-default.h`.

extern crate libc;
extern crate energymon_sys;

use libc::{c_int};
use energymon_sys::energymon;

extern "C" {
    /// Native C function that fills in the energymon struct values.
    pub fn energymon_get_default(em: *mut energymon) -> c_int;
}
