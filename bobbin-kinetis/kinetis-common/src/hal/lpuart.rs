pub use bobbin_common::configure::*;
pub use bobbin_common::reset::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::serial::*;

use bobbin_common::bits::*;
use core::fmt::{self, Write};

use chip::lpuart::*;

#[derive(Debug)]
pub struct Config {
    baud: Baud,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Config {
            baud: Baud(0).set_osr(0b1111)
        }
    }
}

impl Config {
    pub fn set_baud_divisor(mut self, baud_divisor: U13) -> Self {
        self.baud = self.baud.set_sbr(baud_divisor);
        self
    }

    pub fn set_osr(mut self, osr: U5) -> Self {
        self.baud = self.baud.set_osr(osr);
        self
    }
}

impl Configure<Config> for LpuartPeriph {
    fn config(&self) -> Config {
        Config {
            baud: self.baud(),
        }
    }

    fn configure(&self, cfg: Config) -> &Self {
        self
            .disable()
            .set_baud(|_| cfg.baud)
    }
}

impl Enabled for LpuartPeriph {
    fn enabled(&self) -> bool {
        let ctrl = self.ctrl();
        ctrl.te() != 0 || ctrl.re() != 0
    }

    fn set_enabled(&self, value: bool) -> &Self {
        if value {
            self.with_ctrl(|r| r.set_te(1).set_re(1))
        } else {
            self.with_ctrl(|r| r.set_te(0).set_re(0))
        }
    }
}

impl Reset for LpuartPeriph {
    fn reset(&self) -> &Self {
        self.set_global(|r| r.set_rst(1));
        self.set_global(|r| r.set_rst(0));
        self
    }
}

impl Write for LpuartPeriph {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}

impl SerialTx<u8> for LpuartPeriph {    
    fn can_tx(&self) -> bool {
        self.stat().tdre() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_data(|r| r.set_rt(c))
    }
}

impl SerialRx<u8> for LpuartPeriph {
    fn can_rx(&self) -> bool {
        self.stat().rdrf() != 0
    }

    fn rx(&self) -> u8 {
        self.data().rt().into()        
    }
}