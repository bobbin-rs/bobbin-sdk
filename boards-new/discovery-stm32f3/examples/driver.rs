#![no_std]
#![no_main]
#![feature(asm)]

extern crate discovery_stm32f3 as board;
extern crate examples;

use board::console::USART;

use board::mcu::dispatch::{HandleException, Exception};
use board::Dispatcher;

use board::common::irq::*;
use board::mcu::irq::*;
use board::mcu::usart::*;

use core::cell::UnsafeCell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let mut s = SerialDriver::new(USART);
    let mut buf = [0u8; 64];
    let _ = s.write(b"Serial Driver Echo Test\r\n");
    for _i in 0..10 {
        let _ = s.write(b"-");
        // board::delay(100);
    }
    let _ = s.write(b"\r\n");
    loop {
        if let Ok(n) = s.read(&mut buf[..1]) {        
            if n != 0 {
                if buf[0] == 13 {
                    let _ = s.write(b"\r\n");
                } else {
                    let _ = s.write(&buf[..n]);
                }
            }
        }
        // board::delay(1000);
    }
}

#[derive(Debug)]
pub enum Error {
    NoIrqSlots
}

pub struct SerialDriver
{
    usart: UsartPeriph,
    irq_number: u8,
}

impl SerialDriver
{
    pub fn new<USART: Irq<IrqUsart> + Into<UsartPeriph>>(usart: USART) -> Self {
        let usart = usart.into();
        let irq_number = USART.irq_number_for(IRQ_USART);
        Self { usart, irq_number }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let handler = TxHandler::new(self.usart, buf);
        let guard = Dispatcher::register_irq_handler(self.irq_number, &handler);
        if let Ok(guard) = guard {
            Ok(guard.run())
        } else {
            Err(Error::NoIrqSlots)
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let handler = RxHandler::new(self.usart, buf);
        let guard = Dispatcher::register_irq_handler(self.irq_number, &handler);        
        if let Ok(guard) = guard {
            Ok(guard.run())
        } else {
            Err(Error::NoIrqSlots)
        }
    }
}

pub struct TxHandler {
    usart: UsartPeriph,
    ptr: *const u8,
    len: usize,
    pos: VIndex,
}

impl TxHandler {
    pub fn new(usart: UsartPeriph, buf: &[u8]) -> Self {
        let ptr = buf.as_ptr();
        let len = buf.len();
        TxHandler { usart, ptr, len, pos: VIndex::new(0) }
    }

    #[inline]
    fn sleep(&self) {
        unsafe { asm!("
            cpsid i
            wfi
            cpsie i
        ")}
    }

    pub fn run(&self) -> usize {
        self.start();
        while !self.done() { self.sleep() }
        self.pos.get()
    }

    pub fn start(&self) {
        self.usart.with_cr1(|r| r.set_txeie(1));
    }

    #[inline]
    pub fn done(&self) -> bool {
        self.pos.get() == self.len
    }
}

impl HandleException for TxHandler
{    
    unsafe fn handle_exception(&self, _: Exception) {
        let usart = &self.usart;
        let isr = usart.isr();
        let cr1 = usart.cr1();
        if isr.test_txe() && cr1.test_txeie() {
            assert!(self.pos.get() < self.len);
            usart.set_tdr(|r| r.set_tdr( *self.ptr.offset(self.pos.post_incr(1) as isize) ));
            if self.done() {
                usart.with_cr1(|r| r.set_txeie(0));
            }
        }
        if isr.test_ore() {
            usart.with_icr(|r| r.set_orecf(1));
            // panic!("overrun");
        }
    }
}

pub struct RxHandler {
    usart: UsartPeriph,
    ptr: *mut u8,
    len: usize,
    pos: VIndex,
}

impl RxHandler {
    pub fn new(usart: UsartPeriph, buf: &mut [u8]) -> Self {
        let ptr = buf.as_mut_ptr();
        let len = buf.len();
        RxHandler { usart, ptr, len, pos: VIndex::new(0) }
    }

    #[inline]
    fn sleep(&self) {
        unsafe { asm!("
            cpsid i
            wfi
            cpsie i
        ")}
    }

    pub fn run(&self) -> usize {
        self.start();
        while !self.done() { self.sleep() }
        self.pos.get()
    }

    pub fn start(&self) {
        self.usart.with_cr1(|r| r.set_rxneie(1));   
    }

    #[inline]
    pub fn done(&self) -> bool {
        self.pos.get() == self.len
    }
}

impl HandleException for RxHandler
{    
    unsafe fn handle_exception(&self, _: Exception) {
        let usart = &self.usart;
        let isr = usart.isr();
        let cr1 = usart.cr1();

        if isr.test_rxne() && cr1.test_rxneie() {
            assert!(self.pos.get() < self.len);
            let b: u8 = usart.rdr().rdr().value() as u8;
            *self.ptr.offset(self.pos.post_incr(1) as isize) = b;                
            if self.done() {                
                usart.with_cr1(|r| r.set_rxneie(0));                    
            }
        }
        if isr.test_ore() {
            usart.with_icr(|r| r.set_orecf(1));
            // panic!("overrun");
        }
    }
}



pub struct VIndex(UnsafeCell<usize>);

impl VIndex {
    pub fn new(value: usize) -> Self {
        VIndex(UnsafeCell::new(value))
    }
    #[inline]    
    pub fn get(&self) -> usize {
        unsafe { core::ptr::read_volatile(self.0.get()) }        
    }
    #[inline]
    pub fn set(&self, value: usize) {
        unsafe { core::ptr::write_volatile(self.0.get(), value) }        
    }    
    #[inline]
    pub fn post_incr(&self, value: usize) -> usize {        
        let v = self.get();
        self.set(v + value);
        v
    }    
}