use core::fmt::{self, Write};

pub use ::chip::uart::*;
use gpio::{PinAltFn};
use sysctl;
use clock;

pub struct UartDevice {
    uart: Uart,
}

pub fn device(uart: Uart) -> UartDevice {    
    UartDevice { uart: uart}
}

impl UartDevice {
    pub fn enable(&self, baud_hz: u32) {
        let baud_div = ((8 * clock::sysclk_hz()) / baud_hz) + 1;
        let baud_int = baud_div / 64;
        let baud_frac = baud_div % 64;

        let mut uart = self.uart;
        sysctl::set_uart_enabled(self.uart, true);
        unsafe {
            uart.with_ctl(|r| r.set_uarten(0));
            uart.with_ibrd(|r| r.set_divint(baud_int));
            uart.with_fbrd(|r| r.set_divfrac(baud_frac));
            uart.with_lcrh(|r| r.set_wlen(0x3).set_fen(1));
            uart.with_ctl(|r| r.set_hse(1).set_rxe(1).set_txe(1));
            uart.with_ctl(|r| r.set_uarten(1));
        }
    }

    pub fn try_getc(&self) -> Option<u8> {
        if !self.rxfe() {
            Some(self.data())
        } else {
            None
        }        
    }

    pub fn putc(&self, c: u8) {
        while self.txff() {}
        self.set_data(c)
    }

    pub fn write(&self, buf: &[u8]) {
        for b in buf.iter() {
            self.putc(*b);
        }
    }

    pub fn data(&self) -> u8 {
        let uart = self.uart;      
        unsafe { uart.dr().data() as u8 }
    }

    pub fn set_data(&self, value: u8) {  
        let mut uart = self.uart;      
        unsafe { uart.set_dr(Dr(0).set_data(value as u32)); }
    }
    
    pub fn rxfe(&self) -> bool {
        unsafe { self.uart.fr().rxfe() != 0 }
    }

    pub fn txff(&self) -> bool {
        unsafe { self.uart.fr().txff() != 0 }
    }    
}

impl Write for UartDevice {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}