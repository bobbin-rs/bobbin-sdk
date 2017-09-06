pub use bobbin_common::i2c::*;
pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use ::chip::i2c_v2::*;

use bobbin_common::bits::*;

#[derive(Debug, Default)]
pub struct Config {
    pub cr1: Cr1,
    pub cr2: Cr2,
    pub timingr: Timingr
}

impl Config {
    pub fn set_timing(mut self,
        presc: U4,
        scldel: U4,
        sdadel: U4,
        sclh: U8,
        scll: U8,
    ) -> Self {
        self.timingr = Timingr(0)
            .set_presc(presc)
            .set_scldel(scldel)
            .set_sdadel(sdadel)
            .set_sclh(sclh)
            .set_scll(scll);
        self
    }
}

impl Configure<Config> for I2cPeriph {
    fn config(&self) -> Config {
        Config {
            cr1: Cr1(0),
            cr2: Cr2(0),
            timingr: Timingr(0),
        }
    }
    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_cr1(|_| cfg.cr1)
            .set_cr2(|_| cfg.cr2)
            .set_timingr(|_| cfg.timingr)
    }
}

impl Enabled for I2cPeriph {
    fn enabled(&self) -> bool {
        self.cr1().test_pe()
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr1(|r| r.set_pe(value))        
    }
}

impl WriteAddr<U7> for I2cPeriph {
    fn write_addr(&self, addr: U7, data: &[u8]) -> &Self {
        self.with_cr1(|r| r.set_pe(0));
        self.set_cr2(|r| r
            .set_autoend(1)
            .set_nbytes(data.len())
            .set_sadd(addr.value() << 1 | 0)                
        );
        self.with_cr1(|r| r.set_pe(1));
        self.with_cr2(|r| r.set_start(1));
        for byte in data.iter() {
            while self.isr().txe() == 0 {}
            self.set_txdr(|r| r.set_txdata(*byte));
        }
        while self.isr().stopf() == 0 {}
        self.with_cr1(|r| r.set_pe(0));
        self
    }
}


impl ReadAddr<U7> for I2cPeriph {
    fn read_addr(&self, addr: U7, data: &mut [u8]) -> &Self {
        self.with_cr1(|r| r.set_pe(0));
        self.set_cr2(|r| r
            .set_autoend(1)
            .set_nbytes(data.len())
            .set_sadd(addr.value() << 1 | 1)                
            .set_rd_wrn(1)
        );            
        self.with_cr1(|r| r.set_pe(1));
        self.with_cr2(|r| r.set_start(1));
        for i in 0..data.len() {
            while self.isr().rxne() == 0 {}
            data[i] = self.rxdr().rxdata().value();
        }           
        self.with_cr1(|r| r.set_pe(0));       
        self 
    }
}