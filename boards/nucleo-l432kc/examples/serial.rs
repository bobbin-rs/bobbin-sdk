#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::common::serial::*;
use board::common::ring::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Serial Test");

    let tx: board::hal::usart::UsartPeriph = board::console::USART.into();
    let mut buf = [0u8; 16];

    let rb = Ring::new(&mut buf);
    let (r, w) = rb.pair();
   
    let mut s = SerialDriver::new(tx, r);
    w.write(b"Hello\r\n");

    loop {
        s.poll();        
    }
}

pub struct SerialDriver<'a, T: SerialTx<u8>> {
    tx: T,
    rb: RingReader<'a, u8>,
}

impl<'a, T: SerialTx<u8>> SerialDriver<'a, T> {
    pub fn new(tx: T, rb: RingReader<'a, u8>) -> Self {
        SerialDriver {
            tx: tx,
            rb: rb,
        }
    }

    pub fn poll(&mut self) {
        if self.tx.can_tx() {
            if let Some(b) = self.rb.dequeue() {
                self.tx.tx(b);
            }
        }
    }
}

