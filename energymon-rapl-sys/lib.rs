//! FFI bindings for `energymon-rapl.h`.

extern crate libc;
extern crate energymon_sys;

pub use energymon_sys::energymon;
use libc::{c_int, uint64_t, c_char, size_t};

extern "C" {
    pub fn energymon_init_rapl(em: *mut energymon) -> c_int;

    pub fn energymon_read_total_rapl(em: *const energymon) -> uint64_t;

    pub fn energymon_finish_rapl(em: *mut energymon) -> c_int;

    pub fn energymon_get_source_rapl(buffer: *mut c_char, n: size_t) -> *mut c_char;

    pub fn energymon_get_interval_rapl(em: *const energymon) -> uint64_t;

    pub fn energymon_get_rapl(em: *mut energymon) -> c_int;
}
