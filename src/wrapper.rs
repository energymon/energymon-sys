use super::*;
use libc::{c_char};
use std::mem;

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