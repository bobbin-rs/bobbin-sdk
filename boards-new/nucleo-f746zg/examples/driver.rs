#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;
extern crate examples;

use board::console::USART;
use board::ext::dispatch::*;
use board::ext::{Dispatcher, IrqHandler};
use board::common::irq::*;
use board::mcu::irq::*;
use board::mcu::usart::*;

use core::ops::Deref;
use core::cell::UnsafeCell;
// use board::mcu::usart::*;


static mut HANDLER_SLOTS: [Option<IrqHandler>; 2] = [None; 2];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let brd = board::board();

    unsafe {
        Dispatcher::init(&mut HANDLER_SLOTS)
    }
    
    let mut s = SerialDriver::new(USART, brd);
    let _ = s.register();
    // println!("{:?} - {:?}", s.usart, s.irq_handle);
    let mut buf = [0u8; 64];
    let _ = s.write(b"Serial Driver Echo Test\r\n");
    for _i in 0..10 {
        let _ = s.write(b"-");
        // board::delay(100);
    }
    let _ = s.write(b"\r\n");
    loop {
        let n = s.read(&mut buf[..1]).unwrap();
        if n != 0 {
            if buf[0] == 13 {
                let _ = s.write(b"\r\n");
            } else {
                let _ = s.write(&buf[..n]);
                // println!(".");

                // print!("{}", buf[0] as char);
            }
        }
        // board::delay(1000);
    }
}

#[derive(Debug)]
pub enum Error {
    NoRxBuffer,
    NoTxBuffer,
}

pub struct SerialDriver<USART, R>
where    
    USART: 'static + Irq<IrqUsart> + Deref<Target=UsartPeriph>,
    R: 'static + RegisterIrq,
{
    usart: USART,
    rx_desc: Option<Buffer>,
    tx_desc: Option<Buffer>,
    irq_handle: Option<R::Handle>,
    r: R,
}

impl<USART, R> SerialDriver<USART, R> 
where
    USART: 'static + Irq<IrqUsart> + Deref<Target=UsartPeriph>,
    R: 'static + RegisterIrq,
{
    pub fn new(usart: USART, r: R) -> Self {
        Self { usart, rx_desc: None, tx_desc: None, irq_handle: None, r }
    }

    pub fn register(&mut self) -> Result<(), RegisterError> {
        let h = self as *mut Self;
        let irq_number = self.usart.irq_number_for(IRQ_USART);
        // println!("irq_number: {}", irq_number);
        self.irq_handle = Some(self.r.register_irq(irq_number, h)?);
        Ok(())
    }

    pub fn enable(&self) {
        // println!("enabling {}", self.irq_number);
        // self.r.enable_irq(self.irq_number);
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.tx_desc = Some(Buffer::from_slice(buf));
        if let Some(ref desc) = self.tx_desc {
            desc.set_state(State::Busy);
            self.usart.with_cr1(|r| r.set_te(1).set_txeie(1));
            while desc.state() == State::Busy { unsafe { asm!("wfi") } }
            // while desc.state() == State::Busy { }
            // println!("write_done");
            self.usart.with_cr1(|r| r.set_txeie(0));
        }
        self.tx_desc = None;
        return Ok(buf.len())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.rx_desc = Some(Buffer::from_slice(buf));
        if let Some(ref desc) = self.rx_desc {
            desc.set_state(State::Busy);            
            self.usart.with_cr1(|r| r.set_re(1).set_rxneie(1));
            while desc.state() == State::Busy { unsafe { asm!("wfi") } }
            // self.usart.with_cr1(|r| r.set_rxneie(0));
        }
        self.rx_desc = None;
        return Ok(buf.len())
    }

}

impl<USART, R> HandleIrq for SerialDriver<USART, R>
where
    USART: 'static + Irq<IrqUsart> + Deref<Target=UsartPeriph>,
    R: 'static + RegisterIrq,
{    
    unsafe fn handle_irq(&mut self, _irq: u8) -> IrqResult {
        let isr = self.usart.isr();
        let cr1 = self.usart.cr1();
        // println!("irq: {}", irq);
        // println!("  ISR: {:?}", isr);
        // println!("  CR1: {:?}", cr1);
        if isr.test_txe() && cr1.test_txeie() {
            if let Some(ref mut tx_desc) = self.tx_desc {
                let b: u8 = tx_desc.read();
                self.usart.set_tdr(|r| r.set_tdr(b));
                if tx_desc.pos() == tx_desc.len() {
                    tx_desc.set_state(State::Done);
                    self.usart.with_cr1(|r| r.set_txeie(0));
                }
            }
        }
        if isr.test_rxne() && cr1.test_rxneie() {
            if let Some(ref mut rx_desc) = self.rx_desc {
                let b: u8 = self.usart.rdr().rdr().value() as u8;
                rx_desc.write(b);
                if rx_desc.pos() == rx_desc.len() {
                    rx_desc.set_state(State::Done);
                    self.usart.with_cr1(|r| r.set_rxneie(0));                    
                }
            }
        }
        if isr.test_ore() {
            self.usart.with_icr(|r| r.set_orecf(1));
        }
        IrqResult::Continue
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Idle,
    Busy,
    Done,
}

pub struct Buffer {
    ptr: *mut u8,    
    pos: UnsafeCell<usize>,
    len: UnsafeCell<usize>,
    state: UnsafeCell<State>,
}

impl Buffer {    
    pub fn new(ptr: *mut u8, pos: usize, len: usize) -> Buffer {
        Buffer { 
            ptr, 
            pos: UnsafeCell::new(pos), 
            len: UnsafeCell::new(len), 
            state: UnsafeCell::new(State::Idle),
        }
    }

    pub fn from_slice(s: &[u8]) -> Buffer {
        Buffer::new(s.as_ptr() as *mut u8, 0, s.len())
    }

    pub fn from_mut_slice(s: &mut [u8]) -> Buffer {
        Buffer::new(s.as_mut_ptr(), 0, s.len())
    }

    pub fn len(&self) -> usize { 
        unsafe { core::ptr::read_volatile(self.len.get()) }
    }

    pub fn pos(&self) -> usize {
        unsafe { core::ptr::read_volatile(self.pos.get()) }
    }

    pub fn set_pos(&self, value: usize) {
        unsafe { core::ptr::write_volatile(self.pos.get(), value) }        
    }

    pub fn incr_pos(&self, value: usize) {
        self.set_pos(self.pos() + value);
    }

    pub fn state(&self) -> State {
        unsafe { core::ptr::read_volatile(self.state.get()) }
    }

    pub fn set_state(&self, value: State) {
        unsafe { core::ptr::write_volatile(self.state.get(), value) }        
    }

    pub fn reset(&mut self) {
        self.set_pos(0);
    }

    pub fn done(&self) -> bool {
        self.len() == self.pos()
    }

    pub fn read<T>(&mut self) -> T {
        assert!(self.pos() + core::mem::size_of::<T>() <= self.len());
        let v = unsafe { core::ptr::read_volatile(self.ptr.offset(self.pos() as isize) as *const T) };
        self.incr_pos(core::mem::size_of::<T>());
        v
    }

    pub fn write<T>(&mut self, v: T) {
        assert!(self.pos() + core::mem::size_of::<T>() <= self.len());
        unsafe { 
            core::ptr::write_volatile(self.ptr.offset(self.pos() as isize) as *mut T, v);
        }
        self.incr_pos(core::mem::size_of::<T>());
    }

    pub fn slice_to_pos(&self) -> &[u8] {
        let range = ..self.pos();
        &self.as_ref()[range]
    }

    pub fn slice_from_pos(&mut self) -> &mut [u8] {
        let range = self.pos()..;
        &mut self.as_mut()[range]
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { ::core::slice::from_raw_parts(self.ptr, self.len()) }
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { ::core::slice::from_raw_parts_mut(self.ptr, self.len()) }
    }
}