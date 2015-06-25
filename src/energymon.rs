use libc::{c_int, c_double, c_char};

pub type EnergyMonInitFn = unsafe extern fn() -> i32;
pub type EnergyMonReadTotalFn = unsafe extern fn() -> c_double;
pub type EnergyMonFinishFn = unsafe extern fn() -> i32;
pub type EnergyMonGetSourceFn = unsafe extern fn(*mut c_char) -> *mut c_char;

extern {
    fn em_impl_get(ptr: *mut EnergyMonC) -> c_int;
    fn em_init() -> i32;
    fn em_read_total() -> c_double;
    fn em_finish() -> i32;
    fn em_get_source(buffer: *mut c_char) -> *mut c_char;
}

#[repr(C)]
pub struct EnergyMonC {
    pub finit: EnergyMonInitFn,
    pub fread: EnergyMonReadTotalFn,
    pub ffinish: EnergyMonFinishFn,
    pub fsource: EnergyMonGetSourceFn,
}

impl Default for EnergyMonC {
    fn default() -> EnergyMonC {
        EnergyMonC {
            finit: em_init,
            fread: em_read_total,
            ffinish: em_finish,
            fsource: em_get_source,
        }
    }
}

pub struct EnergyMon {
    pub em: EnergyMonC,
}

impl EnergyMon {
    pub fn new() -> Result<EnergyMon, String> {
        let mut em = EnergyMonC::default();
        let result = unsafe {
            em_impl_get(&mut em)
        };
        if result != 0 {
            return Err("Failed to initialize energy reader".to_string());
        }
        Ok(EnergyMon { em: em, })
    }
}
