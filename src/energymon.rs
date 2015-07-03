use libc::{c_int, c_double, c_char, c_void};
use std::mem;

/// Typedef for initialization function.
pub type EnergyMonInitFn = extern fn(*mut EnergyMon) -> c_int;
/// Typedef for read function.
pub type EnergyMonReadTotalFn = extern fn(*mut EnergyMon) -> c_double;
/// Typedef for cleanup function.
pub type EnergyMonFinishFn = extern fn(*mut EnergyMon) -> c_int;
/// Typedef for function to get human-readable name.
pub type EnergyMonGetSourceFn = extern fn(*mut c_char) -> *mut c_char;

#[repr(C)]
/// Representation of native C struct `em_impl`.
pub struct EnergyMon {
    /// Native C function type signature that initializes the `EnergyMon`.
    pub finit: EnergyMonInitFn,
    /// Native C function type signature that reads energy data from the `EnergyMon`.
    pub fread: EnergyMonReadTotalFn,
    /// Native C function type signature that cleans up the `EnergyMon`.
    pub ffinish: EnergyMonFinishFn,
    /// Native C function type signature that gets the `EnergyMon`'s human-readable name.
    pub fsource: EnergyMonGetSourceFn,
    /// Native C pointer used by the underlying `EnergyMon` implementation for storing state.
    pub state: *mut c_void,
}

extern {
    /// Native C function that fills in the EnergyMon struct values and may allocate other
    /// resources.
    fn em_impl_get(em: *mut EnergyMon) -> c_int;
}

impl EnergyMon {
    /// Create an `EnergyMon` which represents a C struct.
    pub fn new() -> Result<EnergyMon, &'static str> {
        unsafe {
            let mut em: EnergyMon = mem::uninitialized();
            let result: i32 = em_impl_get(&mut em);
            if result != 0 {
                return Err("Failed to create energymon");
            }
            Ok(em)
        }
    }

    /// Initialize the `EnergyMon` - may allocate more resources and/or start threads.
    pub fn init(&mut self) -> Result<(), &'static str> {
        let result: i32 = (self.finit)(self);
        if result != 0 {
            return Err("Failed to initialize energymon");
        }
        Ok(())
    }

    /// Read the total energy from the `EnergyMon`.
    pub fn read(&mut self) -> f64 {
        (self.fread)(self)
    }

    /// Cleanup the `EnergyMon` - may deallocate associated resources and/or stop threads.
    pub fn finish(&mut self) -> Result<(), &'static str> {
        let result: i32 = (self.ffinish)(self);
        if result != 0 {
            return Err("Failed to cleanup energymon");
        }
        Ok(())
    }

    /// Utility function to get a human-readable name of the `EnergyMon`'s source.
    pub fn source(&mut self) -> Result<String, &'static str> {
        const BUFSIZE: usize = 100;
        let mut buf: [c_char; BUFSIZE] = [0; BUFSIZE];
        let ret: *mut c_char = (self.fsource)(buf.as_mut_ptr());
        if ret.is_null() {
            return Err("Failed to get energymon source");
        }
        let buf: &[u8] = unsafe {
            mem::transmute::<&[c_char], &[u8]>(&buf)
        };
        Ok(String::from_utf8_lossy(buf).into_owned())
    }
}

#[cfg(test)]
mod test {
    use super::EnergyMon;

    #[test]
    fn test_interface() {
        let mut em: EnergyMon = EnergyMon::new().unwrap();
        em.init().unwrap();
        let val = em.read();
        assert!(val >= 0.0);
        let source = em.source().unwrap();
        em.finish().unwrap();
        println!("Read {} from {}", val, source);
    }

}
