use std::mem;
use std::sync::{Arc, Once, ONCE_INIT};
use std::cell::Cell;
use emimpl::EMImpl;

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
            if EM.is_null() {
                None
            } else {
                Some((*EM).clone())
            }
        }
    }

    pub fn read(&self) -> u64 {
        self.em.get().read()
    }

    pub fn source(&self) -> String {
        self.em.get().source()
    }

    pub fn interval(&self) -> u64 {
        self.em.get().interval()
    }

    pub unsafe fn destroy(&self) {
        println!("Finishing singleton energy monitor");
        let _ = self.em.get().finish();
    }
}

#[cfg(test)]
mod test {
    use super::SingletonEnergyMon;

    #[test]
    fn test_singleton() {
        for _ in 0..2 {
            let sem: SingletonEnergyMon = SingletonEnergyMon::instance().unwrap();
            let val = sem.read();
            println!("Read {} from {} singleton with refresh interval {}", val, sem.source(),
                     sem.interval());
        }
        unsafe {
            SingletonEnergyMon::instance().unwrap().destroy();
        }
    }

}
