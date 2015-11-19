//! FFI bindings for `energymon-msr.h`.

extern crate libc;
extern crate energymon_sys;

pub use energymon_sys::energymon;
use libc::{c_int, uint64_t, c_char, size_t};

extern "C" {
    pub fn energymon_init_msr(em: *mut energymon) -> c_int;

    pub fn energymon_read_total_msr(em: *const energymon) -> uint64_t;

    pub fn energymon_finish_msr(em: *mut energymon) -> c_int;

    pub fn energymon_get_source_msr(buffer: *mut c_char, n: size_t) -> *mut c_char;

    pub fn energymon_get_interval_msr(em: *const energymon) -> uint64_t;

    pub fn energymon_get_msr(em: *mut energymon) -> c_int;
}
