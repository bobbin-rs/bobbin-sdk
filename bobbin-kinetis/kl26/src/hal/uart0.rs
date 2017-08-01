use core::fmt::{self, Write};

pub use ::chip::uart0::*;
use port::*;


pub struct Uart0Device {
    uart: Uart0,
    _rx: PinAltFn,
    _tx: PinAltFn,
    bd: u16,
}

pub fn device(uart: Uart0, rx: PinAltFn, tx: PinAltFn, bd: u16) -> Uart0Device {
    Uart0Device { uart: uart, _rx: rx, _tx: tx, bd: bd }
}

impl Uart0Device {    
    pub fn enable(&self) {
        let mut u = self.uart;
        let baud_divider = self.bd;
        unsafe {
            u.set_c1(C1(0));
            //Disable TX and Receive
            u.set_c2(C2(0));
            // Set baud divider
            u.set_bdh(Bdh(0).set_sbr((baud_divider >> 8) as u8));
            u.set_bdl(Bdl(0).set_sbr(baud_divider as u8));
            
            // Oversampling 4x
            u.set_c4(C4(0).set_osr(0x3));
            u.set_c3(C3(0));                       
            u.set_c5(C5(0));
            // Enable Transmit and Receive
            u.set_c2(C2(0).set_te(1).set_re(1));
        }
    }

    pub fn set_rx_enabled(&self, value: bool) {        
        let mut uart = self.uart;
        let value = if value { 1 } else { 0 };
        unsafe { uart.with_c2(|c2| c2.set_re(value)); }
    }

    pub fn set_tx_enabled(&self, value: bool) {
        let mut uart = self.uart;
        let value = if value { 1 } else { 0 };
        unsafe { uart.with_c2(|c2| c2.set_te(value)); }
    }        

    pub fn tx_empty(&self) -> bool {
        unsafe { self.uart.s1().tdre() != 0 }
    }

    pub fn tx_complete(&self) -> bool {
        unsafe { self.uart.s1().tc() != 0 }
    }

    pub fn rx_full(&self) -> bool {
        unsafe { self.uart.s1().rdrf() != 0 }
    }

    pub fn idle(&self) -> bool {
        unsafe { self.uart.s1().idle() != 0 }
    }        

    pub fn try_getc(&self) -> Option<u8> {
        let uart = self.uart;
        unsafe {
            if uart.s1().rdrf() != 0 {
                Some(uart.d().rt())
            } else {
                None
            }
        }
    }

    pub fn getc(&self) -> u8 {
        let uart = self.uart;
        unsafe {                           
            while uart.s1().rdrf() == 0 {}
            uart.d().rt()
        }
        
    }

    pub fn putc(&self, value: u8) {            
        let mut uart = self.uart;
        unsafe { 
            while uart.s1().tdre() == 0 {}
            uart.set_d(D(0).set_rt(value));
        }
    }

    pub fn write(&self, data: &[u8]) {
        for i in 0..data.len() {
            self.putc(data[i]);
        }
    }    
}

impl Write for Uart0Device {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}