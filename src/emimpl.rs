use libc::{c_int, c_ulonglong, c_char, c_void};
use std::mem;

/// Typedef for initialization function.
type EnergyMonInitFn = extern fn(*mut EMImpl) -> c_int;
/// Typedef for read function.
type EnergyMonReadTotalFn = extern fn(*const EMImpl) -> c_ulonglong;
/// Typedef for cleanup function.
type EnergyMonFinishFn = extern fn(*mut EMImpl) -> c_int;
/// Typedef for function to get human-readable name.
type EnergyMonGetSourceFn = extern fn(*mut c_char) -> *mut c_char;
/// Typedef for function to get refresh interval.
type EnergyMonGetIntervalFn = extern fn(*const EMImpl) -> c_ulonglong;

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
/// Representation of native C struct `em_impl`.
pub struct EMImpl {
    /// Native C function type signature that initializes the `EMImpl`.
    finit: EnergyMonInitFn,
    /// Native C function type signature that reads energy data from the `EMImpl`.
    fread: EnergyMonReadTotalFn,
    /// Native C function type signature that cleans up the `EMImpl`.
    ffinish: EnergyMonFinishFn,
    /// Native C function type signature that gets the `EMImpl`'s human-readable name.
    fsource: EnergyMonGetSourceFn,
    /// Native C function type signature that gets the `EMImpl`'s refresh interval.
    finterval: EnergyMonGetIntervalFn,
    /// Native C pointer used by the underlying `EMImpl` implementation for storing state.
    state: *mut c_void,
}

extern {
    /// Native C function that fills in the EMImpl struct values and may allocate other resources.
    fn em_impl_get(em: *mut EMImpl) -> c_int;
}

impl EMImpl {
    /// Create an `EMImpl` which represents a C struct.
    pub fn new() -> Result<EMImpl, &'static str> {
        unsafe {
            let mut em: EMImpl = mem::uninitialized();
            match em_impl_get(&mut em) {
                0 => Ok(em),
                _ => Err("Failed to create EMImpl"),
            }
        }
    }

    /// Initialize the `EMImpl` - may allocate more resources and/or start threads.
    pub fn init(&mut self) -> Result<(), &'static str> {
        match (self.finit)(self) {
            0 => Ok(()),
            _ => Err("Failed to initialize EMImpl"),
        }
    }

    /// Read the total energy from the `EMImpl`.
    pub fn read(&self) -> u64 {
        (self.fread)(self)
    }

    /// Cleanup the `EMImpl` - may deallocate associated resources and/or stop threads.
    pub fn finish(&mut self) -> Result<(), &'static str> {
        match (self.ffinish)(self) {
            0 => Ok(()),
            _ => Err("Failed to cleanup EMImpl"),
        }
    }

    /// Utility function to get a human-readable name of the `EMImpl`'s source.
    pub fn source(&mut self) -> String {
        const BUFSIZE: usize = 100;
        let mut buf: [c_char; BUFSIZE] = [0; BUFSIZE];
        let ret: *mut c_char = (self.fsource)(buf.as_mut_ptr());
        if ret.is_null() {
            return "UNKNOWN".to_owned();
        }
        let buf: &[u8] = unsafe {
            mem::transmute::<&[c_char], &[u8]>(&buf)
        };
        String::from_utf8_lossy(buf).into_owned()
    }

    /// Get the refresh interval from the `EMImpl`.
    pub fn interval(&self) -> u64 {
        (self.finterval)(self)
    }
}
