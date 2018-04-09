#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;
extern crate examples;

use board::console::USART;

use board::mcu::dispatch::{HandleException, Exception, Guard};
use board::Dispatcher;
use board::Heap;
use board::common::ring::*;

use board::common::irq::*;
use board::mcu::irq::*;
use board::mcu::usart::*;

// use core::cell::UnsafeCell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    unsafe { Heap::extend(1024) };

    let tx_buf = Heap::slice(0u8, 64);
    let tx_ring = Heap::new(Ring::new(tx_buf));
    let tx_reader = Heap::new(tx_ring.reader());

    let rx_buf = Heap::slice(0u8, 64);
    let rx_ring = Heap::new(Ring::new(rx_buf));
    let rx_writer = Heap::new(rx_ring.writer());
    
    let handler = Heap::new(SerialHandler::new(USART, tx_reader, rx_writer));
    let guard = Dispatcher::register_irq_handler(USART.irq_number_for(IRQ_USART), handler).unwrap();
    let s = SerialDriver::new(guard, tx_ring, rx_ring);

    for _ in 0..100 {
        s.write_all(b"Serial Driver Echo Test\r\n");        
    }
    s.write_all(b"DONE\r\n");


    // let mut s = SerialDriver::new(USART);
    // let mut buf = [0u8; 64];
    // for _i in 0..10 {
    //     let _ = s.write(b"-");
    // }
    // let _ = s.write(b"\r\n");
    // loop {
    //     if let Ok(n) = s.read(&mut buf[..1]) {        
    //         if n != 0 {
    //             if buf[0] == 13 {
    //                 let _ = s.write(b"\r\n");
    //             } else {
    //                 let _ = s.write(&buf[..n]);
    //             }
    //         }
    //     }
    // }
    loop {
        unsafe { asm!("nop")}
    }
}

pub struct SerialDriver {
    guard: Guard<'static, SerialHandler>,
    tx_ring: &'static Ring<'static, u8>,
    rx_ring: &'static Ring<'static, u8>,
}

impl SerialDriver {
    pub fn new(guard: Guard<'static, SerialHandler>, tx_ring: &'static Ring<'static, u8>, rx_ring: &'static Ring<'static, u8>) -> Self {
        Self { guard, tx_ring, rx_ring }
    }

    #[inline]
    fn sleep(&self) {
        unsafe { asm!("
            cpsid i
            wfi
            cpsie i
        ")}
    }    

    pub fn write_all(&self, buf: &[u8]) -> usize {        
        let mut n = 0;
        while n < buf.len() {            
            let sent = self.write(&buf[n..]);
            if sent == 0 {
                self.sleep();
            } else {
                n += sent;
            }
        }
        n
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        let len = self.tx_ring.write(buf);
        self.guard.start_tx();
        len
    }
}

pub struct SerialHandler {
    usart: UsartPeriph,
    tx_reader: &'static Reader<'static, u8>,
    rx_writer: &'static Writer<'static, u8>,
}

impl SerialHandler {
    pub fn new<U: Into<UsartPeriph>>(usart: U, tx_reader: &'static Reader<'static, u8>, rx_writer: &'static Writer<'static, u8>) -> Self {
        let usart = usart.into();
        Self { usart, tx_reader, rx_writer }
    }

    pub fn start_tx(&self) {
        self.usart.with_cr1(|r| r.set_txeie(1));
    }

    pub fn stop_tx(&self) {
        self.usart.with_cr1(|r| r.set_txeie(0));
    }

    pub fn tx(&self, b: u8) {
        self.usart.set_tdr(|r| r.set_tdr(b));
    }
}

impl HandleException for SerialHandler {
    unsafe fn handle_exception(&self, _: Exception) {
        let usart = &self.usart;
        let isr = usart.isr();
        let cr1 = usart.cr1();
        if isr.test_txe() && cr1.test_txeie() {
            if let Some(b) = self.tx_reader.get() {
                self.tx(b);
            } 
            if self.tx_reader.len() == 0 {
                usart.with_cr1(|r| r.set_txeie(0));
            }
        }
        if isr.test_ore() {
            usart.with_icr(|r| r.set_orecf(1));
            // panic!("overrun");
        }
    }
}