#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::console::USART;
use board::ext::dispatch::*;
use board::ext::{Dispatcher, IrqHandler};
use board::common::irq::*;
use board::mcu::irq::*;
// use board::mcu::usart::*;


static mut HANDLER_SLOTS: [Option<IrqHandler>; 2] = [None; 2];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let brd = board::board();
    println!("Driver Test");

    unsafe {
        Dispatcher::init(&mut HANDLER_SLOTS)
    }
    
    // let irq_num = USART.irq_number_for(IRQ_USART);
    // println!("irq number for {:?} = {}", USART, irq_num);

    let mut s = SerialDriver::new(USART, brd);
    println!("{:?} - {}", s.usart, s.irq_number);
    let _ = s.register();
    loop {
        board::delay(1000);
    }
}

pub struct SerialDriver<USART, R>
where    
    USART: 'static + Irq<IrqUsart>,
    R: 'static + RegisterIrq,
{
    usart: USART,
    irq_number: u8,
    irq_handle: Option<R::Handle>,
    r: R,
}

impl<USART, R> SerialDriver<USART, R> 
where
    USART: 'static + Irq<IrqUsart>,
    R: 'static + RegisterIrq,
{
    pub fn new(usart: USART, r: R) -> Self {
        let irq_number = usart.irq_number_for(IRQ_USART);
        Self { usart, irq_number, irq_handle: None, r }
    }

    pub fn register(&mut self) -> Result<(), RegisterError> {
        let h = self as *mut Self;
        self.irq_handle = Some(self.r.register_irq(self.irq_number, h)?);
        Ok(())
    }

    pub fn enable(&self) {
        // println!("enabling {}", self.irq_number);
        // self.r.enable_irq(self.irq_number);
    }
}

impl<USART, R> HandleIrq for SerialDriver<USART, R>
where
    USART: 'static + Irq<IrqUsart>,
    R: 'static + RegisterIrq,
{    
    unsafe fn handle_irq(&mut self, _irq: u8) -> IrqResult {
        IrqResult::Continue
    }
}
