use super::*;

/// A basic energy monitor.
pub struct EnergyMon {
    /// The native C struct.
    em: em_impl,
}

impl EnergyMon {
    /// Create and initialize an `EnergyMon`.
    pub fn new() -> Result<EnergyMon, &'static str> {
        let mut em = try!{em_impl::new()};
        try!{em.init()};
        Ok(EnergyMon { em: em })
    }

    /// Read the total energy from the `EnergyMon`.
    pub fn read(&self) -> u64 {
        self.em.read()
    }

    /// Get a human-readable name of the `EnergyMon`'s source.
    pub fn source(&mut self) -> String {
        self.em.source()
    }

    /// Get the refresh interval for the `EnergyMon`.
    pub fn interval(&self) -> u64 {
        self.em.interval()
    }
}

impl Drop for EnergyMon {
    /// Cleanup the `EnergyMon`.
    fn drop(&mut self) {
        let _ = self.em.finish();
    }
}

#[cfg(test)]
mod test {
    use super::EnergyMon;

    #[test]
    fn test_interface() {
        let mut em: EnergyMon = EnergyMon::new().unwrap();
        let val = em.read();
        println!("Read {} from {} with refresh interval {}", val, em.source(), em.interval());
    }

}
