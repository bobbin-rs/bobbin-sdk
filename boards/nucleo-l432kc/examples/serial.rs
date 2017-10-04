#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::usart::*;
use board::common::{Irq, Poll};
use board::console::USART;
// use board::common::serial::*;
use board::common::ring::*;
use core::fmt::{self, Write};

use board::hal::nvic;
use board::hal::scb::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Serial Test");
    let irq = board::console::USART.irq_usart();
    let mut tx_buf = [0u8; 16];
    let mut rx_buf = [0u8; 16];
    let mut s = UsartDriver::new(USART, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&irq);

    write!(s, "Hello, World!\r\n").unwrap();
    s.enable_rx();


    let mut buf = [0u8; 16];
    loop {
        let n = s.read(&mut buf);
        if n > 0 {
            for b in (&buf[..n]).iter() {
                if *b == b'\r' {
                    s.write(b"\r\n");
                } else {
                    s.write(&[*b]);
                }
            }
        }
    }
}

pub struct UsartDriver<'a> {    
    usart: UsartPeriph,
    tx: Ring<'a, u8>,
    rx: Ring<'a, u8>,    
    irq_num: Option<u8>,
}

impl<'a> UsartDriver<'a> {
    pub fn new<U: Into<UsartPeriph>>(usart: U, tx_buf: &mut [u8], rx_buf: &mut [u8]) -> Self {
        let tx = Ring::new(tx_buf);
        let rx = Ring::new(rx_buf);
   
        UsartDriver {
            usart: usart.into(),
            tx: tx,
            rx: rx,
            irq_num: None,
        }
    }

    pub fn enable_irq<I: Irq>(&mut self, irq: &I) {
        self.irq_num = Some(irq.irq_num());
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn enable_rx(&self) {
        self.usart.with_cr1(|r| r.set_rxneie(1));
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        let n = self.tx.write(buf);
        if n > 0 {
            self.usart.with_cr1(|r| r.set_txeie(1));
        }
        n
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {
        self.rx.read(buf)
    }
}

impl<'a> Drop for UsartDriver<'a> {
    fn drop(&mut self) {
        if let Some(irq_num) = self.irq_num {
            nvic::set_enabled(irq_num as usize, false);
            SCB.set_irq_handler(irq_num as usize, None);
        }
    }
}

impl<'a> Poll for UsartDriver<'a> {
    fn poll(&self) {
        if self.usart.can_tx() {
            if let Some(b) = self.tx.dequeue() {
                self.usart.tx(b);
                if self.tx.is_empty() {
                    self.usart.with_cr1(|r| r.set_txeie(0));
                }
            }
        }
        if self.usart.can_rx() {
            let b = self.usart.rx();
            if !self.rx.is_full() {
                self.rx.enqueue(b);
            }
        }
    }

}

impl<'a> Write for UsartDriver<'a> {
    fn write_str(&mut self, buf: &str) -> fmt::Result {
        let buf = buf.as_bytes();
        let mut n = 0;
        loop {
            if n == buf.len() { return Ok(())}
            n += self.write(&buf[n..])
        }
    }
}