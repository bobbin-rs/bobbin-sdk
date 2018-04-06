#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;
extern crate examples;

use board::console::USART;
use board::ext::dispatch::*;
use board::ext::{Dispatcher, IrqHandler, IrqGuard};
use board::common::irq::*;
use board::mcu::irq::*;
use board::mcu::usart::*;

use core::cell::UnsafeCell;
// use board::mcu::usart::*;

static mut HANDLER_SLOTS: [Option<IrqHandler>; 2] = [None; 2];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    unsafe {
        Dispatcher::init(&mut HANDLER_SLOTS)
    }
    let mut h = SerialHandler::new(USART);
    let mut s = SerialDriver::new(USART, &mut h);    
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

pub struct SerialDriver<'a>
{
    usart: UsartPeriph,
    guard: IrqGuard<'a, SerialHandler>,
}

impl<'a> SerialDriver<'a> 
{
    pub fn new<USART: Irq<IrqUsart> + Into<UsartPeriph>>(usart: USART, handler: &'a mut SerialHandler) -> Self {
        let guard = Dispatcher::register_irq_handler(USART.irq_number_for(IRQ_USART), handler).unwrap();
        let usart = usart.into();
        Self { usart, guard }
    }

    pub fn irq(&self) -> u8 {
        self.guard.irq()
    }

    fn tx_desc(&mut self) -> &mut Option<Buffer> {
        self.guard.tx_desc()
    }
    
    fn rx_desc(&mut self) -> &mut Option<Buffer> {
        self.guard.rx_desc()
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        *self.tx_desc() = Some(Buffer::from_slice(buf));
        self.tx_desc().as_mut().unwrap().set_state(State::Busy);
        self.usart.with_cr1(|r| r.set_te(1).set_txeie(1));
        if let &mut Some(ref desc) = self.tx_desc() {
            while desc.state() == State::Busy { unsafe { asm!("nop") } }
        }
        self.usart.with_cr1(|r| r.set_txeie(0));
        *self.tx_desc() = None;
        return Ok(buf.len())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        *self.rx_desc() = Some(Buffer::from_slice(buf));
        self.rx_desc().as_mut().unwrap().set_state(State::Busy);            
        self.usart.with_cr1(|r| r.set_re(1).set_rxneie(1));
        if let &mut Some(ref desc) = self.rx_desc() {
            while desc.state() == State::Busy { unsafe { asm!("wfi") } }
        }
        self.usart.with_cr1(|r| r.set_rxneie(0));
        *self.rx_desc() = None;
        return Ok(buf.len())
    }

}

pub struct SerialHandler
{
    usart: UsartPeriph,
}

impl SerialHandler 
{
    pub fn new<USART: Into<UsartPeriph>>(usart: USART) -> Self {
        Self { usart: usart.into() }
    }

    fn tx_desc(&self) -> &mut Option<Buffer> {
        static mut TX_DESC: Option<Buffer> = None;
        unsafe { &mut TX_DESC }
    }

    fn rx_desc(&self) -> &mut Option<Buffer> {
        static mut RX_DESC: Option<Buffer> = None;
        unsafe { &mut RX_DESC }
    }    
}
impl HandleIrq for SerialHandler
{    
    unsafe fn handle_irq(&self, _irq: u8) -> IrqResult {
        let usart = &self.usart;
        let isr = usart.isr();
        let cr1 = usart.cr1();
        // println!("irq: {:p}", self);
        // println!("irq: {}", irq);
        // println!("  ISR: {:?}", isr);
        // println!("  CR1: {:?}", cr1);
        if isr.test_txe() && cr1.test_txeie() {
            if let &mut Some(ref mut tx_desc) = self.tx_desc() {
                let b: u8 = tx_desc.read();
                usart.set_tdr(|r| r.set_tdr(b));
                if tx_desc.pos() == tx_desc.len() {
                    tx_desc.set_state(State::Done);
                    usart.with_cr1(|r| r.set_txeie(0));
                }
            }
        }
        if isr.test_rxne() && cr1.test_rxneie() {
            if let &mut Some(ref mut rx_desc) = self.rx_desc() {
                let b: u8 = usart.rdr().rdr().value() as u8;
                rx_desc.write(b);
                if rx_desc.pos() == rx_desc.len() {
                    rx_desc.set_state(State::Done);
                    usart.with_cr1(|r| r.set_rxneie(0));                    
                }
            }
        }
        if isr.test_ore() {
            usart.with_icr(|r| r.set_orecf(1));
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

    #[inline]
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