//! FFI bindings for `energymon-odroid-ioctl.h`.

extern crate libc;
extern crate energymon_sys;

pub use energymon_sys::energymon;
use libc::{c_int, uint64_t, c_char, size_t};

extern "C" {
    pub fn energymon_init_odroid_ioctl(em: *mut energymon) -> c_int;

    pub fn energymon_read_total_odroid_ioctl(em: *const energymon) -> uint64_t;

    pub fn energymon_finish_odroid_ioctl(em: *mut energymon) -> c_int;

    pub fn energymon_get_source_odroid_ioctl(buffer: *mut c_char, n: size_t) -> *mut c_char;

    pub fn energymon_get_interval_odroid_ioctl(em: *const energymon) -> uint64_t;

    pub fn energymon_get_odroid_ioctl(em: *mut energymon) -> c_int;
}
