pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_hal::serial::*;
use core::fmt::{self, Write};
pub use periph::lpuart::*;

use bobbin_common::bits::*;

#[derive(Debug)]
pub struct Config {
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    brr: Brr,
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            cr1: Cr1(0),
            cr2: Cr2(0),
            cr3: Cr3(0),
            brr: Brr(0),
        }
    }
}

impl Config {
    // 25.4.4 LPUART baud rate generation
    // The baud rate for the receiver and transmitter (Rx and Tx) are both set to the same value as programmed in the LPUART_BRR register.
    // The baud rate for the receiver and transmitter (Rx and Tx) are both set to the same value as programmed in the LPUART_BRR register.
    // Baud = (256 x f_ck) / LPUARTDIV
    // LPUARTDIV is coded on the LPUART_BRR register.

    pub fn set_baud(mut self, baud: u32, clock: u32) -> Self {
        let divider: u32 = (64 * clock) / (baud / 4);
        self.brr = Brr(0).set_brr(divider);
        self
    }

    pub fn set_brr<V: Into<U20>>(mut self, brr: U20) -> Self {
        self.brr = Brr(0).set_brr(brr);
        self            
    }
}

impl Configure<Config> for LpuartPeriph {
    fn config(&self) -> Config {
        Config {
            cr1: self.cr1(),
            cr2: self.cr2(),
            cr3: self.cr3(),
            brr: self.brr(),
        }
    }


    fn configure(&self, cfg: Config) -> &Self {        
        self.set_cr1(|_| cfg.cr1);
        self.set_cr2(|_| cfg.cr2);
        self.set_cr3(|_| cfg.cr3);
        self.set_brr(|_| cfg.brr);
        self
    }
}

impl Enabled for LpuartPeriph {
    fn enabled(&self) -> bool {
        self.cr1().test_ue()
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self
            .set_cr1(|r| r
                .set_ue(value)
                .set_re(value)
                .set_te(value)
            )        
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
        self.isr().test_txe()
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_tdr(|r| r.set_tdr(c))
    }
}

impl SerialRx<u8> for LpuartPeriph {
    fn can_rx(&self) -> bool {
        self.isr().test_rxne()
    }

    fn rx(&self) -> u8 {
        self.rdr().rdr().value() as u8
    }
}