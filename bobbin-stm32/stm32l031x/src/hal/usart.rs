use core::fmt::{self, Write};

use ::chip::usart::*;
use rcc;

pub enum Mode {
    Raw,
    Cooked,
}

pub struct Serial {
    pub usart: Usart,
    pub mode: Mode,
}

impl Serial {
    pub fn new(usart: Usart, mode: Mode) -> Self {
        Serial { usart: usart, mode: mode }
    }

    pub fn configure(&mut self, baud_hz: u32, apb_hz: u32) {        
        let mut u = self.usart;
        unsafe {
            rcc::set_usart_enabled(u, true);

            // Set N,8,1            

            u.set_cr1(Cr1(0));

            // Disable hardware flow control
            u.set_cr3(Cr3(0).set_rtse(0).set_ctse(0));
            
            let brr = (apb_hz / baud_hz) as u16;
            u.set_brr(Brr(0)
                .set_div_fraction((brr & 0b1111) as u32)
                .set_div_mantissa(brr as u32 >> 4)
            );
            // let brr = 0x116;
            // u.set_brr(Brr(brr));
            u.set_cr1(Cr1(0).set_te(1).set_re(1).set_ue(1));
        }
    }

    pub fn try_getc(&self) -> Option<u8> {
        unsafe {
            let u = &self.usart;
            if u.isr().rxne() != 0 {
                Some(u.rdr().rdr() as u8)
            } else {
                None
            }
        }
    }

    pub fn try_putc(&mut self, c: u8) -> Option<usize> {
        unsafe {
            let mut u = &mut self.usart;
            if u.isr().txe() != 0 {
                u.set_tdr(Tdr(c as u32));
                Some(1)
            } else {
                None
            }            
        }
    }

    pub fn getc(&self) -> u8 {
        unsafe {
            let u = &self.usart;        
            while u.isr().rxne() == 0 {}
            u.rdr().rdr() as u8            
        }
    }
    

    pub fn putc(&mut self, c: u8) {
        unsafe {
            let mut u = &mut self.usart;
            while u.isr().txe() == 0 {}
            u.set_tdr(Tdr(c as u32));
        }
    }    
    pub fn write(&mut self, buf: &[u8]) {
        for c in buf.iter() {
            self.putc(*c);
        }
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes().iter().cloned() {
            match self.mode {
                Mode::Cooked => if byte == b'\n' { self.putc(b'\r') },
                _ => {},
            }
            self.putc(byte);
        }
        Ok(())
    }
}
