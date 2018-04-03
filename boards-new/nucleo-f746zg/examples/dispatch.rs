#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::mcu::systick;

use board::mcu::bobbin_common::dispatch::*;
static mut HANDLER_SLOTS: [Option<IrqHandler>; 2] = [None; 2];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let brd = board::board();
    println!("Dispatch Test");

    unsafe {
        Dispatcher::init(&mut HANDLER_SLOTS)
    }

    let reload_value = (216_000_000 / 8000) - 1;

    let mut t = TickHandler::new();    
    let _ = t.register(brd);
    t.configure(reload_value);
    t.enable();
    
    loop {
        println!("tick: {}", t.count);
        board::delay(1000);
    }
}

use systick::SYSTICK;

pub struct TickHandler {
    count: u32,
    irq_handle: Option<IrqHandle>,
}

impl TickHandler {
    pub fn new() -> Self {
        Self { count: 0, irq_handle: None }
    }

    pub fn register<R: RegisterExc>(&mut self, reg: R) -> Result<(), RegisterError> {
        self.irq_handle = Some(reg.register_exc(15, self)?);
        Ok(())
    }

    pub fn configure(&mut self, reload_value: u32) {
        SYSTICK.set_reload_value(reload_value);
        SYSTICK.set_current_value(reload_value);
    }

    pub fn enable(&mut self){
        SYSTICK.set_tick_interrupt(true);
        SYSTICK.set_enabled(true);        
    }
}

impl HandleIrq for TickHandler {
    unsafe fn handle_irq(&mut self, _irq: u8) -> IrqResult {
        self.count += 1;
        IrqResult::Continue
    }
}