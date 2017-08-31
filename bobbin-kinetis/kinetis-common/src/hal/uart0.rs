pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::serial::*;

use chip::uart0::*;

#[derive(Debug, Default)]
pub struct Config {
    pub bdh: Bdh,
    pub bdl: Bdl,
}

impl Config {
    pub fn set_baud_divisor(mut self, baud_divisor: u16) -> Self {
        self.bdh = self.bdh.set_sbr((baud_divisor >> 8) as u8);
        self.bdl = self.bdl.set_sbr(baud_divisor as u8);
        self
    }
}

impl Configure<Config> for Uart0Periph {
    fn config(&self) -> Config {
        Config {
            bdh: self.bdh(),
            bdl: self.bdl(),
        }
    }
    fn configure(&self, cfg: Config) -> &Self {
        self
            .disable()
            .set_bdh(|_| cfg.bdh)
            .set_bdl(|_| cfg.bdl)
    }
}

impl Enabled for Uart0Periph {
    fn enabled(&self) -> bool {
        let c2 = self.c2();
        c2.te() != 0 || c2.re() != 0
    }

    fn set_enabled(&self, value: bool) -> &Self {
        if value {
            self.with_c2(|r| r.set_te(1).set_re(1))
        } else {
            self.with_c2(|r| r.set_te(0).set_re(0))
        }
    }
}

impl SerialTx<u8> for Uart0Periph {    
    fn can_tx(&self) -> bool {
        self.s1().tdre() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_d(|r| r.set_rt(c))
    }
}

impl SerialRx<u8> for Uart0Periph {
    fn can_rx(&self) -> bool {
        self.s1().rdrf() != 0
    }

    fn rx(&self) -> u8 {
        self.d().rt().value()
    }
}