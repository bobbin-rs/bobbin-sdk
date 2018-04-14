#![no_std]
#![no_main]
#![feature(asm, nll)]

extern crate nucleo_l432kc as board;
extern crate examples;

use board::console::USART;

use board::mcu::dispatch::{HandleException, Exception, Guard};
use board::System;
use board::common::sys::ring::*;

use board::common::irq::*;
use board::mcu::irq::*;
use board::mcu::usart::*;

// use core::cell::UnsafeCell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();

    unsafe { sys.heap_mut().extend(1024) };
    let mut s = SerialDriver::new(&mut sys, USART);

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
    pub fn new<U: Into<UsartPeriph> + Irq<IrqUsart>>(sys: &mut System, usart: U) -> Self {
        Self::new_with_config(sys, usart, Config::default())
    }

    pub fn new_with_config<U: Into<UsartPeriph> + Irq<IrqUsart>>(sys: &mut System, usart: U, cfg: Config) -> Self {
        let heap = sys.heap_mut();
        let tx_buf = heap.slice(0u8, cfg.tx_len);
        let tx_ring = heap.new(Ring::new(tx_buf));
        let tx_reader = heap.new(tx_ring.reader());

        let rx_buf = heap.slice(0u8, cfg.rx_len);
        let rx_ring = heap.new(Ring::new(rx_buf));
        let rx_writer = heap.new(rx_ring.writer());
        
        let irq_number = usart.irq_number_for(IRQ_USART);
        let handler = heap.new(SerialHandler::new(usart, tx_reader, rx_writer));
        let guard = sys.dispatcher_mut().register_irq_handler(irq_number, handler).unwrap();        
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
                self.sleep();
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
        self.guard.rx_start();
        len
    }    
}

pub struct SerialHandler {
    usart: UsartPeriph,
    reader: &'static Reader<'static, u8>,
    writer: &'static Writer<'static, u8>,
}

impl SerialHandler {
    pub fn new<U: Into<UsartPeriph>>(usart: U, reader: &'static Reader<'static, u8>, writer: &'static Writer<'static, u8>) -> Self {
        let usart = usart.into();
        Self { usart, reader, writer }
    }

    pub fn tx_start(&self) {
        self.usart.enable_tx_irq();
    }

    pub fn tx_stop(&self) {
        self.usart.disable_tx_irq();
    }

    pub fn rx_start(&self) {
        self.usart.enable_rx_irq();
    }

    pub fn rx_stop(&self) {
        self.usart.disable_rx_irq();
    }
}

impl HandleException for SerialHandler {
    unsafe fn handle_exception(&self, _: Exception) {
        if self.usart.can_tx() {
            if let Some(b) = self.reader.get() {
                self.usart.tx(b);
            } else {
                self.tx_stop();
            }
        }
        if self.usart.can_rx() {
            if self.writer.rem() > 0 {
                self.writer.put(self.usart.rx());
            } else {
                self.rx_stop();
            }
        }        
        if self.usart.isr().test_ore() {
            self.usart.with_icr(|r| r.set_orecf(1));
        }
    }
}