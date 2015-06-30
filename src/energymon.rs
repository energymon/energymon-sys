use libc::{c_int, c_double, c_char, c_void};
use std::ffi::CStr;
use std::mem;

pub type EnergyMonInitFn = unsafe extern fn(em: *mut EnergyMon) -> c_int;
pub type EnergyMonReadTotalFn = unsafe extern fn(em: *mut EnergyMon) -> c_double;
pub type EnergyMonFinishFn = unsafe extern fn(em: *mut EnergyMon) -> c_int;
pub type EnergyMonGetSourceFn = unsafe extern fn(*mut c_char) -> *mut c_char;

#[repr(C)]
pub struct EnergyMon {
    pub finit: EnergyMonInitFn,
    pub fread: EnergyMonReadTotalFn,
    pub ffinish: EnergyMonFinishFn,
    pub fsource: EnergyMonGetSourceFn,
    pub state: *mut c_void,
}

extern {
    fn em_impl_get(em: *mut EnergyMon) -> c_int;
}

impl EnergyMon {
    pub fn new() -> Result<EnergyMon, String> {
        unsafe {
            let mut em: EnergyMon = mem::uninitialized();
            let result = em_impl_get(&mut em);
            if result != 0 {
                return Err("Failed to initialize energymon".to_string());
            }
            Ok(em)
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let result: i32 = unsafe {
            (self.finit)(self)
        };
        if result != 0 {
            return Err("Failed to initialize energymon".to_string());
        }
        Ok(())
    }

    pub fn read(&mut self) -> f64 {
        unsafe {
            (self.fread)(self)
        }
        
    }

    pub fn finish(&mut self) -> Result<(), String> {
        let result: i32 = unsafe {
            (self.ffinish)(self)
        };
        if result != 0 {
            return Err("Failed to initialize energymon".to_string());
        }
        Ok(())
    }

    pub fn source(&mut self) -> Result<String, String> {
        let mut buf = [0; 100];
        unsafe {
            let ret = (self.fsource)(buf.as_mut_ptr());
            if ret.is_null() {
                return Err("Failed to get energymon source".to_string());
            }
            Ok(String::from_utf8_lossy(CStr::from_ptr(ret).to_bytes()).into_owned())
        }
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
