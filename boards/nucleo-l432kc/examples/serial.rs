#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::usart::*;
// use board::common::serial::*;
use board::common::ring::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Serial Test");
    // let irq = board::console::USART.irq_usart();
    let tx: board::hal::usart::UsartPeriph = board::console::USART.into();
    let mut buf = [0u8; 16];

    let rb = Ring::new(&mut buf);
   
    let s = UsartDriver::new(tx, rb);
    s.write(b"Hello\r\n");

    loop {
        s.poll();        
    }
}

pub struct UsartDriver<'a> {
    tx: UsartPeriph,
    rb: Ring<'a, u8>,
}

impl<'a> UsartDriver<'a> {
    pub fn new(tx: UsartPeriph, rb: Ring<'a, u8>) -> Self {
        UsartDriver {
            tx: tx,
            rb: rb,
        }
    }

    pub fn poll(&self) {
        if self.tx.can_tx() {
            if let Some(b) = self.rb.dequeue() {
                self.tx.tx(b);
                if self.rb.len() == 0 {
                    self.tx.with_cr1(|r| r.set_txeie(0));
                }
            }
        }
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        let n = self.rb.write(buf);
        self.tx.with_cr1(|r| r.set_txeie(1));
        n
    }
}