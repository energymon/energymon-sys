extern crate libc;
extern crate energymon_sys;

use libc::{c_int, c_ulonglong, c_char, size_t};
use energymon_sys::energymon;

extern "C" {
    pub fn energymon_init_msr(em: *mut energymon) -> c_int;

    pub fn energymon_read_total_msr(em: *const energymon) -> c_ulonglong;

    pub fn energymon_finish_msr(em: *mut energymon) -> c_int;

    pub fn energymon_get_source_msr(buffer: *mut c_char, n: size_t) -> *mut c_char;

    pub fn energymon_get_interval_msr(em: *const energymon) -> c_ulonglong;

    pub fn energymon_get_msr(em: *mut energymon) -> c_int;
}
