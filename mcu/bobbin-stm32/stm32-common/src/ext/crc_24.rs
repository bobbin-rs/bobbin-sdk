pub use periph::crc_24::*;

impl CrcPeriph {
    pub fn reset(&self) -> &Self {    
        self.set_cr(|r| r.set_reset(1))
    }

    pub fn write(&self, value: u32) -> &Self {
        self.set_dr(|r| r.set_dr(value))
    }

    pub fn read(&self) -> u32 {
        self.dr().dr().into()
    }
}