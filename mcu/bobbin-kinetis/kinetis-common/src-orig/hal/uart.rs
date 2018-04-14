pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::hal::serial::*;


use bobbin_common::{Irq, Poll};
use bobbin_common::sys::ring::Ring;
use bobbin_cortexm::hal::scb::SCB;
use bobbin_cortexm::hal::nvic;
use bobbin_cortexm::wfi;

use core::marker::PhantomData;
use core::fmt;


use chip::uart::*;

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

impl Configure<Config> for UartPeriph {
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

impl Enabled for UartPeriph {
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

impl SerialTx<u8> for UartPeriph {    
    fn can_tx(&self) -> bool {
        self.s1().tdre() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_d(|r| r.set_rt(c))
    }
}

impl SerialRx<u8> for UartPeriph {
    fn can_rx(&self) -> bool {
        self.s1().rdrf() != 0
    }

    fn rx(&self) -> u8 {
        self.d().rt().value()
    }
}

pub struct UartDriver<'a> {
    uart: UartPeriph,
    tx: Ring<'a, u8>,
    rx: Ring<'a, u8>,
    _phantom: PhantomData<&'a mut [u8]>,
}

unsafe impl<'a> Sync for UartDriver<'a> {}
unsafe impl<'a> Send for UartDriver<'a> {}

impl<'a> UartDriver<'a> {
    pub fn new<P: Into<UartPeriph>>(uart: P, tx_buf: &'a mut [u8], rx_buf: &'a mut [u8]) -> Self {
        UartDriver { 
            uart: uart.into(),
            tx: Ring::new(tx_buf),
            rx: Ring::new(rx_buf),
            _phantom: PhantomData,
        }
    }

    pub fn enable_irq<I: Irq>(&self, irq: &I) {        
        nvic::set_enabled(irq.irq_num() as usize, false);
        SCB.set_irq_handler(irq.irq_num() as usize, None);
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn tx_len(&self) -> usize {
        self.tx.len()
    }

    pub fn rx_len(&self) -> usize {
        self.rx.len()
    }

    pub fn read(&self, buf: &mut [u8]) {
        while self.rx.len() < buf.len() {
            wfi()
        }
        self.rx.read(buf);
    }

    pub fn read_byte(&self) -> u8 {
        while self.rx.len() == 0 {
            wfi()
        }
        self.rx.dequeue().unwrap()
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        let v = self.tx.write(buf);
        self.tx_enable();
        v
    }

    pub fn tx_enable(&self) {
        self.uart.with_c2(|r| r.set_tie(true).set_te(true));
    }
    
    pub fn tx_disable(&self) {        
        self.uart.with_c2(|r| r.set_tie(false).set_te(false));
    }

    pub fn rx_enable(&self) {        
        self.uart.with_c2(|r| r.set_rie(true).set_re(true));
    }
    
    pub fn rx_disable(&self) {        
        self.uart.with_c2(|r| r.set_rie(false).set_re(false));
    }    
}

impl<'a> Poll for UartDriver<'a> {
    fn poll(&self) {       
        let s1 = self.uart.s1();
        
        if s1.test_tdre() {
            if let Some(b) = self.tx.dequeue() {
                self.uart.set_d(|r| r.set_rt(b));
            } else {
                self.tx_disable();
            }
        }
        if s1.test_rdrf() {
            self.rx.enqueue(self.uart.d().rt().value());
        }
    }
}

impl<'a> fmt::Write for UartDriver<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {        
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                self.tx.enqueue(b'\r');
            }
            self.tx.enqueue(byte);
        }
        self.tx_enable();
        Ok(())
    }
}
