#![no_std]
#![no_main]
#![feature(asm)]

extern crate frdm_k64f as board;
extern crate examples;

use board::console::UART;

use board::mcu::dispatch::{HandleException, Exception, Guard};
use board::Dispatcher;
use board::Heap;
use board::common::ring::*;

use board::common::irq::*;
use board::mcu::irq::*;
use board::mcu::uart::*;

// use core::cell::UnsafeCell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    unsafe { Heap::extend(1024) };
    let mut s = SerialDriver::new();

    s.write_all(b"Serial Driver Echo Test\r\n");        

    let mut buf = [0u8; 64];
    loop {
        let n = s.read(&mut buf);
        if n > 0 {
            for b in &buf[..n] {
                if *b == 13 {
                    s.write_all(b"\r\n");
                } else {
                    s.write_all(&[*b]);
                }
            }
        }
        s.sleep();
    }
}

pub struct Config {
    pub tx_len: usize,
    pub rx_len: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tx_len: 64,
            rx_len: 64,
        }
    }
}

pub struct SerialDriver {
    guard: Guard<'static, SerialHandler>,
    tx_ring: &'static Ring<'static, u8>,
    rx_ring: &'static Ring<'static, u8>,
}

impl SerialDriver {
    pub fn new() -> Self {
        Self::new_with_config(Config::default())
    }

    pub fn new_with_config(cfg: Config) -> Self {
        let tx_buf = Heap::slice(0u8, cfg.tx_len);
        let tx_ring = Heap::new(Ring::new(tx_buf));
        let tx_reader = Heap::new(tx_ring.reader());

        let rx_buf = Heap::slice(0u8, cfg.rx_len);
        let rx_ring = Heap::new(Ring::new(rx_buf));
        let rx_writer = Heap::new(rx_ring.writer());
        
        let handler = Heap::new(SerialHandler::new(UART, tx_reader, rx_writer));
        let guard = Dispatcher::register_irq_handler(UART.irq_number_for(IRQ_UART), handler).unwrap();        
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

    pub fn write_all(&mut self, buf: &[u8]) -> usize {        
        let mut n = 0;
        while n < buf.len() {          
            let sent = self.write(&buf[n..]);
            if sent == 0 {
                // self.sleep();
            } else {
                n += sent;
            }
        }
        n
    }

    pub fn write(&mut self, buf: &[u8]) -> usize {
        let len = self.tx_ring.write(buf);
        self.guard.tx_start();
        len
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let len = self.rx_ring.read(buf);
        if len > 0 {
        }
        self.guard.rx_start();
        len
    }    
}

pub struct SerialHandler {
    uart: UartPeriph,
    reader: &'static Reader<'static, u8>,
    writer: &'static Writer<'static, u8>,
}

impl SerialHandler {
    pub fn new<U: Into<UartPeriph>>(uart: U, reader: &'static Reader<'static, u8>, writer: &'static Writer<'static, u8>) -> Self {
        let uart = uart.into();
        Self { uart, reader, writer }
    }

    pub fn tx_start(&self) {
        self.uart.with_c2(|r| r.set_tie(1));
    }

    pub fn tx_stop(&self) {
        self.uart.with_c2(|r| r.set_tie(0));
    }

    pub fn tx(&self, c: u8) {
        self.uart.set_d(|r| r.set_rt(c));
    }

    pub fn tx_ready(&self) -> bool {
        self.uart.s1().test_tdre()
    }

    pub fn rx_start(&self) {
        self.uart.with_c2(|r| r.set_rie(1));
    }

    pub fn rx_stop(&self) {
        self.uart.with_c2(|r| r.set_rie(0));
    }

    pub fn rx_ready(&self) -> bool {
        self.uart.s1().test_rdrf()
    }

    pub fn rx(&self) -> u8 {
        self.uart.d().rt().value()
    }    
}

impl HandleException for SerialHandler {
    unsafe fn handle_exception(&self, _: Exception) {
        if self.tx_ready() {
            if let Some(b) = self.reader.get() {
                self.tx(b);
            } 
            if self.reader.len() == 0 {
                self.tx_stop();
            }
        }
        if self.rx_ready() {
            if let Some(elt) = self.writer.head_elt() {                
                *elt = self.rx();
                self.writer.incr_head();
            }
            if self.writer.rem() == 0 {
                self.rx_stop();
            }
        }        
    }
}