pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::serial::*;
use core::fmt::{self, Write};
use chip::usart::*;

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
    // 29.5.4 - USART baud rate generation
    // In case of oversampling by 16, the equation is: Baud = Fck / USARTDIV
    // In case of oversampling by 8, the equation is: Baud = 2 * Fck / USARTDIV

    // NOTE: USART1 is on APB2, all others on APB1
    // Fck will be 36Mhz for APB1, 72Mhz for APB2

    // Assuming FCK is 36Mhz and 8x oversampling, 115,200 = 2 * 36Mhz / USARTDIV
    // USARTDIV = 2 * 36Mhz / 115,200 = 625
    // let brr = (apb_hz / baud_hz) as u16;    
    pub fn set_baud(self, baud: u32, clock: u32) -> Self {
        self.set_brr(clock / baud)
    }

    pub fn set_brr(mut self, brr: u32) -> Self {
        self.brr = Brr(0)
            .set_div_fraction((brr & 0b1111) as u32)
            .set_div_mantissa(brr as u32 >> 4);
        self            
    }
}

impl Configure<Config> for UsartPeriph {
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

impl Enabled for UsartPeriph {
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


impl Write for UsartPeriph {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}


impl SerialTx<u8> for UsartPeriph {    
    fn can_tx(&self) -> bool {
        self.isr().txe() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_tdr(|r| r.set_tdr(c))
    }
}

impl SerialRx<u8> for UsartPeriph {
    fn can_rx(&self) -> bool {
        self.isr().rxne() != 0
    }

    fn rx(&self) -> u8 {
        self.rdr().rdr().value() as u8
    }
}