#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::common::serial::*;
use core::cell::Cell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Serial Test");

    let tx: board::hal::usart::UsartPeriph = board::console::USART.into();
    let mut buf = [0u8; 16];
    
    let mut s = SerialDriver::new(tx, &mut buf);
    s.write(b"Hello");

    loop {
        s.poll();        
    }
}

pub struct SerialDriver<'a, T: SerialTx<u8>> {
    tx: T,
    tx_buf: &'a mut [u8],
    tx_head: Cell<usize>,
    tx_tail: Cell<usize>,
}

impl<'a, T: SerialTx<u8>> SerialDriver<'a, T> {
    pub fn new(tx: T, tx_buf: &'a mut [u8]) -> Self {
        SerialDriver {
            tx: tx,
            tx_buf: tx_buf,
            tx_head: Cell::new(0),
            tx_tail: Cell::new(0),
        }
    }

    pub fn tx_phy(&self, value: usize) -> usize {
        value % self.tx_len()
    }

    pub fn tx_len(&self) -> usize {
        self.tx_buf.len()
    }

    pub fn tx_enq(&mut self, b: u8) {
        self.tx_buf[self.tx_phy(self.tx_head.get())] = b;
        self.tx_head.set(self.tx_head.get().wrapping_add(1));
    }

    pub fn tx_deq(&mut self) -> Option<u8> {
        if self.tx_tail == self.tx_head {
            None
        } else {
            let b = self.tx_buf[self.tx_phy(self.tx_tail.get())];
            self.tx_tail.set(self.tx_tail.get().wrapping_add(1));
            Some(b)
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> usize {
        for b in buf.iter() {
            self.tx_enq(*b);
        }
        buf.len()
    }
    pub fn poll(&mut self) {
        if self.tx.can_tx() {
            if let Some(b) = self.tx_deq() {
                self.tx.tx(b);
            }
        }
    }
}

