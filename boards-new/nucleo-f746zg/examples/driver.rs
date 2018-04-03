#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::console::USART;
use board::mcu::bobbin_common::dispatch::*;
use board::common::irq::*;
use board::mcu::irq::*;
use board::mcu::usart::*;


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

    let s = SerialDriver::new(USART);
    println!("{:?} - {}", s.usart, s.irq_number);

    loop {
        board::delay(1000);
    }
}

pub struct SerialDriver<USART>
where
    USART: 'static + Irq<IrqUsart>,
{
    usart: USART,
    irq_number: u8,
    irq_handle: Option<IrqHandle>,

}

impl<USART> SerialDriver<USART> 
where
    USART: 'static + Irq<IrqUsart>,
{
    pub fn new(usart: USART) -> Self {
        let irq_number = usart.irq_number_for(IRQ_USART);
        Self { usart, irq_number, irq_handle: None }
    }

    pub fn register<R: RegisterIrq>(&mut self, r: R) -> Result<(), RegisterError> {
        self.irq_handle = Some(r.register_irq(self.irq_number, self)?);
        Ok(())
    }

    pub fn enable<E: EnableIrq>(&self, e: E) {
        e.enable_irq(self.irq_number);
    }
}

impl<USART> HandleIrq for SerialDriver<USART>
where
    USART: 'static + Irq<IrqUsart>,
{    
    unsafe fn handle_irq(&mut self, _irq: u8) -> IrqResult {
        IrqResult::Continue
    }
}
