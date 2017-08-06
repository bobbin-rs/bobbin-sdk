use core::fmt::{self, Write};
use chip::usart::*;

pub struct Config {
    pub cr1: Cr1,
    pub cr2: Cr2,
    pub cr3: Cr3,
    pub brr: Brr,
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

pub trait UsartExt {
    fn set_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self;
    fn with_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self;
    fn configure(&self, cfg: Config) -> &Self;
    fn enable(&self) -> &Self;
    fn disable(&self) -> &Self;
    fn putc(&self, c: u8);
    fn try_putc(&self, c: u8) -> Option<usize>;
    fn getc(&self) -> u8;
    fn try_getc(&self) -> Option<u8>;
    fn write(&self, buf: &[u8]) -> usize;
    fn read(&self, buf: &mut [u8]) -> usize;
}

impl<T> UsartExt for Periph<T> {
    fn set_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
        self.configure(f(Config::default()))
    }
    fn with_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
        self.configure(f(Config {
            cr1: self.cr1(),
            cr2: self.cr2(),
            cr3: self.cr3(),
            brr: self.brr(),
        }))
    }
    fn configure(&self, cfg: Config) -> &Self {
        self.set_cr1(cfg.cr1);
        self.set_cr2(cfg.cr2);
        self.set_cr3(cfg.cr3);
        self.set_brr(cfg.brr);
        self
    }
    fn enable(&self) -> &Self {
        self
            .set_cr1(Cr1(0)
                .set_ue(1)
                .set_re(1)
                .set_te(1)
            )        
    }
    fn disable(&self) -> &Self {
        self
            .set_cr1(Cr1(0)
                .set_ue(0)
                .set_re(0)
                .set_te(0)
            )      
    }

    fn putc(&self, c: u8) {
        let u = self;
        while u.isr().txe() == 0 {}
        u.set_tdr(Tdr(0).set_tdr(c as u32));
    }

    fn try_putc(&self, c: u8) -> Option<usize> {
        let u = self;
        if u.isr().txe() != 0 {
            u.set_tdr(Tdr(0).set_tdr(c as u32));
            Some(1)
        } else {
            None
        }            
    }

    fn try_getc(&self) -> Option<u8> {
        let u = self;
        if u.isr().rxne() != 0 {
            Some(u.rdr().rdr().into())
        } else {
            None
        }
    }

    fn getc(&self) -> u8 {
        let u = self;
        while u.isr().rxne() == 0 {}
        u.rdr().rdr().into()         
    }

    fn write(&self, buf: &[u8]) -> usize {
        for b in buf.iter() {
            self.putc(*b)
        }
        buf.len()
    }

    fn read(&self, buf: &mut [u8]) -> usize {
        if buf.len() == 0 { return 0; }
        if let Some(c) = self.try_getc() {
            buf[0] = c;
            1
        } else {
            0
        }
    }
}

impl<T> Write for Periph<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}