extern crate libc;
extern crate energymon_sys;

use libc::{c_int, c_ulonglong, c_char, size_t};
use energymon_sys::energymon;

extern "C" {
    pub fn energymon_init_osp(em: *mut energymon) -> c_int;

    pub fn energymon_read_total_osp(em: *const energymon) -> c_ulonglong;

    pub fn energymon_finish_osp(em: *mut energymon) -> c_int;

    pub fn energymon_get_source_osp(buffer: *mut c_char, n: size_t) -> *mut c_char;

    pub fn energymon_get_interval_osp(em: *const energymon) -> c_ulonglong;

    pub fn energymon_get_osp(em: *mut energymon) -> c_int;
}
