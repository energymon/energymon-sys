//! FFI bindings for `energymon-shmem.h`.

extern crate libc;
extern crate energymon_sys;

pub use energymon_sys::energymon;
use libc::{c_int, uint64_t, c_char, size_t};

#[repr(C)]
/// Representation of native C struct `energymon_shmem`.
pub struct energymon_shmem {
    pub interval_us: uint64_t,
    pub precision_uj: uint64_t,
    pub energy_uj: uint64_t,
}

extern "C" {
    pub fn energymon_init_shmem(em: *mut energymon) -> c_int;

    pub fn energymon_read_total_shmem(em: *const energymon) -> uint64_t;

    pub fn energymon_finish_shmem(em: *mut energymon) -> c_int;

    pub fn energymon_get_source_shmem(buffer: *mut c_char, n: size_t) -> *mut c_char;

    pub fn energymon_get_interval_shmem(em: *const energymon) -> uint64_t;

    pub fn energymon_get_precision_shmem(em: *const energymon) -> uint64_t;

    pub fn energymon_is_exclusive_shmem() -> c_int;

    pub fn energymon_get_shmem(em: *mut energymon) -> c_int;
}
