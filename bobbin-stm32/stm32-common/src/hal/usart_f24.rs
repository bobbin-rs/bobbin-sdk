use core::fmt::{self, Write};
use chip::usart_f24::*;

pub trait UsartExt {
    fn enable(&self, brr: u32) -> &Self;
    fn disable(&self) -> &Self;
    fn putc(&self, c: u8);
    fn try_putc(&self, c: u8) -> Option<usize>;
    fn getc(&self) -> u8;
    fn try_getc(&self) -> Option<u8>;
    fn write(&self, buf: &[u8]) -> usize;
}

impl<T> UsartExt for Periph<T> {
    fn enable(&self, brr: u32) -> &Self {
        // 29.5.4 - USART baud rate generation
        // In case of oversampling by 16, the equation is: Baud = Fck / USARTDIV
        // In case of oversampling by 8, the equation is: Baud = 2 * Fck / USARTDIV

        // NOTE: USART1 is on APB2, all others on APB1
        // Fck will be 36Mhz for APB1, 72Mhz for APB2

        // Assuming FCK is 36Mhz and 8x oversampling, 115,200 = 2 * 36Mhz / USARTDIV
        // USARTDIV = 2 * 36Mhz / 115,200 = 625
        // let brr = (apb_hz / baud_hz) as u16;

        self
            .set_cr1(|r| r)
            .set_cr2(|r| r)
            .set_cr3(|r| r)
            .set_brr(|r| r
                .set_div_fraction((brr & 0b1111) as u32)
                .set_div_mantissa(brr as u32 >> 4)
            )    
            .set_cr1(|r| r
                .set_ue(1)
                .set_re(1)
                .set_te(1)
            )        
    }

    fn disable(&self) -> &Self {
        self.with_cr1(|r| r.set_ue(0).set_re(0).set_te(0))
    }

    fn putc(&self, c: u8) {
        let u = self;
        while u.sr().txe() == 0 {}
        u.set_dr(|r| r.set_dr(c as u32));    
    }

    fn try_putc(&self, c: u8) -> Option<usize> {
        let u = self;
        if u.sr().txe() != 0 {
            u.set_dr(|r| r.set_dr(c as u32));
            Some(1)
        } else {
            None
        }          
    }

    fn try_getc(&self) -> Option<u8> {
        let u = self;
        if u.sr().rxne() != 0 {
            Some(u.dr().dr().into())
        } else {
            None
        }  
    }

    fn getc(&self) -> u8 {
        let u = self;
        while u.sr().rxne() == 0 {}
        u.dr().dr().into()
    }

    fn write(&self, buf: &[u8]) -> usize {
        for b in buf.iter() {
            self.putc(*b)
        }
        buf.len()
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