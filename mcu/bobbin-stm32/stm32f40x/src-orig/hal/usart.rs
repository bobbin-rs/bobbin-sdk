use core::fmt::{self, Write};

use ::chip::usart::*;

pub struct UsartDevice {
    usart: Usart,
}

pub fn device(usart: Usart) -> UsartDevice {
    UsartDevice { usart: usart }
}

impl UsartDevice {
    pub fn into_inner(self) -> Usart {
        self.usart
    }
    
    pub fn enable(&self, brr: u32) {
        // rcc::set_usart_enabled(self.usart, true);
        // rcc::set_usart_clock(self.usart, rcc::UsartClock::Pclk);

        // 29.5.4 - USART baud rate generation
        // In case of oversampling by 16, the equation is: Baud = Fck / USARTDIV
        // In case of oversampling by 8, the equation is: Baud = 2 * Fck / USARTDIV

        // NOTE: USART1 is on APB2, all others on APB1
        // Fck will be 36Mhz for APB1, 72Mhz for APB2

        // Assuming FCK is 36Mhz and 8x oversampling, 115,200 = 2 * 36Mhz / USARTDIV
        // USARTDIV = 2 * 36Mhz / 115,200 = 625

        // let apb_hz = 36_000_000;
        //let brr = 72_000_000 / baud;

        

        let mut u = self.usart;
        unsafe {
            // Disable USART, TX and RX
            //usart.with_cr1(|r| r.set_re(0).set_te(0).set_ue(0));

            u.set_cr1(Cr1(0));    
            u.set_cr2(Cr2(0));    
            u.set_cr3(Cr3(0));
        
            //let brr = (apb_hz / baud_hz) as u16;
            // u.set_brr(Brr(brr as u32));
            u.set_brr(Brr(0)
                .set_div_fraction((brr & 0b1111) as u32)
                .set_div_mantissa(brr as u32 >> 4)
            );    
            u.set_cr1(Cr1(0)
                .set_ue(1)
                .set_re(1)
                .set_te(1)
            );
        }
    }

    pub fn putc(&self, c: u8) {
        let mut u = self.usart;
        unsafe {
            while u.sr().txe() == 0 {}
            u.set_dr(Dr(0).set_dr(c as u32));
        }
    }

    pub fn try_getc(&self) -> Option<u8> {
        let u = self.usart;
        unsafe {        
            if u.sr().rxne() != 0 {
                Some(u.dr().dr() as u8)
            } else {
                None
            }
        }
    }

    pub fn try_putc(&self, c: u8) -> Option<usize> {
        let mut u = self.usart;
        unsafe {       
            if u.sr().txe() != 0 {
                u.set_dr(Dr(0).set_dr(c as u32));
                Some(1)
            } else {
                None
            }            
        }
    }

    pub fn getc(&self) -> u8 {
        let u = self.usart;
        unsafe {
            while u.sr().rxne() == 0 {}
            u.dr().dr() as u8            
        }
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        for b in buf.iter() {
            self.putc(*b)
        }
        buf.len()
    }
}

impl Write for UsartDevice {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}