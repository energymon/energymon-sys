//! FFI bindings for `energymon-wattsup.h`.

extern crate libc;
extern crate energymon_sys;

pub use energymon_sys::energymon;
use libc::{c_int, uint64_t, c_char, size_t};

extern "C" {
    pub fn energymon_init_wattsup(em: *mut energymon) -> c_int;

    pub fn energymon_read_total_wattsup(em: *const energymon) -> uint64_t;

    pub fn energymon_finish_wattsup(em: *mut energymon) -> c_int;

    pub fn energymon_get_source_wattsup(buffer: *mut c_char, n: size_t) -> *mut c_char;

    pub fn energymon_get_interval_wattsup(em: *const energymon) -> uint64_t;

    pub fn energymon_get_precision_wattsup(em: *const energymon) -> uint64_t;

    pub fn energymon_is_exclusive_wattsup() -> c_int;

    pub fn energymon_get_wattsup(em: *mut energymon) -> c_int;
}
