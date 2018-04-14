pub use bobbin_common::hal::i2c::*;
pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use ::chip::i2c_v1::*;

use bobbin_common::bits::*;

#[derive(Debug, Default)]
pub struct Config {
    pub cr1: Cr1,
    pub cr2: Cr2,
}

impl Config {
}

impl Configure<Config> for I2cPeriph {
    fn config(&self) -> Config {
        Config {
            cr1: Cr1(0),
            cr2: Cr2(0),
        }
    }
    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_cr1(|_| cfg.cr1)
            .set_cr2(|_| cfg.cr2)
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

// NOTE: Seems to always produce double byte reads and writes. Needs Review.

impl<A: Into<U7>> I2cTransfer<A> for I2cPeriph {
    fn transfer(&self, addr: A, cmd: &[u8], data: &mut[u8]) -> &Self {
        let i2c = self;
        let addr = addr.into();
        self.set_enabled(true);
        // Wait until BUSY is clear
        while i2c.sr2().busy() != 0 {}

        // Set START
        i2c.with_cr1(|r| r.set_start(1));

        // EV5: SB=1, cleared by reading SR1 followed by writing to DR

        // Wait until START is set
        while i2c.sr1().sb() == 0 {}

        // Send Slave Address
        i2c.set_dr(|_| Dr(0).set_dr(addr.value() << 1));

        // EV6: ADDR=1, cleared by reading SR1 followed by reading SR2

        // Wait until ADDR is set
        while i2c.sr1().addr() == 0 {}

        // Clear ADDR
        i2c.with_sr1(|r| r.set_addr(0));

        // Read SR2
        let _ = i2c.sr2();

        let mut i = 0;

        while i < cmd.len() {
            let mut b = cmd[i];
            // Wait until TXE is set
            while i2c.sr1().txe() == 0 {}

            // Write data to DR
            i2c.set_dr(|_| Dr(0).set_dr(b));
            i += 1;

            if i < cmd.len() {
                b = cmd[i];
                // Write data to DR
                i2c.set_dr(|_| Dr(0).set_dr(b));
                i += 1;
            }

            // Wait until BTF is set
            while i2c.sr1().btf() == 0 {}
        }    

        // Set START
        i2c.with_cr1(|r| r.set_start(1));

        // EV5: SB=1, cleared by reading SR1 followed by writing to DR

        // Wait until START is set
        while i2c.sr1().sb() == 0 {}

        // Send Slave Address
        i2c.set_dr(|_| Dr(0).set_dr((addr.value() << 1) | 1));

        // EV6: ADDR=1, cleared by reading SR1 followed by reading SR2

        // Wait until ADDR is set
        while i2c.sr1().addr() == 0 {}

        // Clear ADDR
        i2c.with_sr1(|r| r.set_addr(0));

        // Read SR2
        let _ = i2c.sr2();

        let mut i = 0;        
        while i < data.len() {
            // if last packet, clear ACK else set ACK
            if i == data.len() - 1 {
                i2c.with_cr1(|r| r.set_ack(0));
            } else {
                i2c.with_cr1(|r| r.set_ack(1));
            }
            
            // Wait until RXnE
            while i2c.sr1().rxne() == 0 {}
            data[i] = i2c.dr().dr().value();
            i += 1;
        }

        i2c.with_cr1(|r| r.set_stop(1)); 
        self.set_enabled(false);
        self
    }    
}