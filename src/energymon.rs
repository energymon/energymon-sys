use libc::{c_int, c_longlong, c_char, c_void};
use std::mem;
use std::sync::{Arc, Once, ONCE_INIT};
use std::cell::Cell;

/// Typedef for initialization function.
type EnergyMonInitFn = extern fn(*mut EMImpl) -> c_int;
/// Typedef for read function.
type EnergyMonReadTotalFn = extern fn(*mut EMImpl) -> c_longlong;
/// Typedef for cleanup function.
type EnergyMonFinishFn = extern fn(*mut EMImpl) -> c_int;
/// Typedef for function to get human-readable name.
type EnergyMonGetSourceFn = extern fn(*mut c_char) -> *mut c_char;

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
/// Representation of native C struct `em_impl`.
struct EMImpl {
    /// Native C function type signature that initializes the `EMImpl`.
    finit: EnergyMonInitFn,
    /// Native C function type signature that reads energy data from the `EMImpl`.
    fread: EnergyMonReadTotalFn,
    /// Native C function type signature that cleans up the `EMImpl`.
    ffinish: EnergyMonFinishFn,
    /// Native C function type signature that gets the `EMImpl`'s human-readable name.
    fsource: EnergyMonGetSourceFn,
    /// Native C pointer used by the underlying `EMImpl` implementation for storing state.
    state: *mut c_void,
}

extern {
    /// Native C function that fills in the EMImpl struct values and may allocate other resources.
    fn em_impl_get(em: *mut EMImpl) -> c_int;
}

impl EMImpl {
    /// Create an `EMImpl` which represents a C struct.
    fn new() -> Result<EMImpl, &'static str> {
        unsafe {
            let mut em: EMImpl = mem::uninitialized();
            match em_impl_get(&mut em) {
                0 => Ok(em),
                _ => Err("Failed to create EMImpl"),
            }
        }
    }

    /// Initialize the `EMImpl` - may allocate more resources and/or start threads.
    fn init(&mut self) -> Result<(), &'static str> {
        match (self.finit)(self) {
            0 => Ok(()),
            _ => Err("Failed to initialize EMImpl"),
        }
    }

    /// Read the total energy from the `EMImpl`.
    fn read(&mut self) -> i64 {
        (self.fread)(self)
    }

    /// Cleanup the `EMImpl` - may deallocate associated resources and/or stop threads.
    fn finish(&mut self) -> Result<(), &'static str> {
        match (self.ffinish)(self) {
            0 => Ok(()),
            _ => Err("Failed to cleanup EMImpl"),
        }
    }

    /// Utility function to get a human-readable name of the `EMImpl`'s source.
    fn source(&mut self) -> String {
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
}

/// A basic energy monitor.
pub struct EnergyMon {
    /// The native C struct.
    em: EMImpl,
}

impl EnergyMon {
    /// Create and initialize an `EnergyMon`.
    pub fn new() -> Result<EnergyMon, &'static str> {
        println!("Initializing energy monitor");
        let mut em = try!{EMImpl::new()};
        try!{em.init()};
        Ok(EnergyMon { em: em })
    }

    /// Read the total energy from the `EnergyMon`.
    pub fn read(&mut self) -> i64 {
        self.em.read()
    }

    /// Get a human-readable name of the `EnergyMon`'s source.
    pub fn source(&mut self) -> String {
        self.em.source()
    }
}

impl Drop for EnergyMon {
    fn drop(&mut self) {
        println!("Finishing energy monitor");
        let _ = self.em.finish();
    }
}

#[derive(Clone)]
pub struct SingletonEnergyMon {
    em: Arc<Cell<EMImpl>>,
}

impl SingletonEnergyMon {
    pub fn instance() -> Option<SingletonEnergyMon> {
        static mut EM: *const SingletonEnergyMon = 0 as *const SingletonEnergyMon;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                println!("Initializing singleton energy monitor");
                let mut em = match EMImpl::new() {
                    Ok(e) => e,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    },
                };
                match em.init() {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{}", e);
                        let _ = em.finish();
                        return;
                    },
                }
                let sem: SingletonEnergyMon = SingletonEnergyMon {
                    em: Arc::new(Cell::new(em))
                };
                // put it on the heap
                EM = mem::transmute(Box::new(sem));
            });
            Some((*EM).clone())
        }
    }

    pub fn read(&self) -> i64 {
        self.em.get().read()
    }

    pub fn source(&self) -> String {
        self.em.get().source()
    }

    pub unsafe fn destroy(&self) {
        println!("Finishing singleton energy monitor");
        let _ = self.em.get().finish();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interface() {
        let mut em: EnergyMon = EnergyMon::new().unwrap();
        let val = em.read();
        assert!(val >= 0);
        println!("Read {} from {}", val, em.source());
    }

    #[test]
    fn test_singleton() {
        for _ in 0..2 {
            let sem: SingletonEnergyMon = SingletonEnergyMon::instance().unwrap();
            let val = sem.read();
            println!("Read {} from {} singleton", val, sem.source());
        }
        unsafe {
            SingletonEnergyMon::instance().unwrap().destroy();
        }
    }

}
