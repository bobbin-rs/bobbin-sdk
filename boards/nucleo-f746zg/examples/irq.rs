#![no_std]
#![no_main]
#![feature(const_fn)]
#[macro_use]
extern crate nucleo_f746zg as board;

use board::mcu::tim_gen::*;
use board::clock::*;
use board::common::gate::*;

static mut TICK_HANDLER: TickHandler<Tim11> = TickHandler::new(TIM11);

pub const TIM: Tim11 = TIM11;

fn tick_handler() -> &'static mut TickHandler<Tim11> {
    unsafe { &mut TICK_HANDLER }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("IRQ Test");

    let tim_clk = tree().u32_for(TIM) / 2000;
    tick_handler().enable(tim_clk as u16, 2000 << 1);
    
    let mut c = tick_handler().count();
    loop {
        let count = tick_handler().count();
        if count != c {
            println!("tick {}", count);
            c = count;
        }
    }
}

use core::ops::Deref;
use board::mcu::nvic;
use board::mcu::irq::*;

struct TickHandler<T>
where
    T: 'static + GateEn + Irq<IrqTim> + Deref<Target=TimGenPeriph>
 {
    tim: T,
    count: u32,
}

impl<T> TickHandler<T> 
where
    T: 'static + GateEn + Irq<IrqTim> + Deref<Target=TimGenPeriph>
{
    pub const fn new(tim: T) -> Self {
        TickHandler { tim, count: 0 }
    }    

    pub fn enable(&self, prescale: u16, reload: u32) {    
        nvic::set_enabled(self.tim.irq_number_for(IRQ_TIM) as usize, true);
        self.tim.gate_enable();
        self.tim.set_prescale(prescale)
                .set_update_event()
                .clr_update_interrupt_flag()
                .with_dier(|r| r.set_uie(1))
                .set_auto_reload(reload); 
        self.tim.set_enabled(true);           
    }

    pub fn count(&self) -> u32 {
        unsafe { core::ptr::read_volatile(&self.count) }
    }

    fn handle_irq(&mut self) {
        self.tim.clr_update_interrupt_flag();
        self.count += 1;
    }
}


#[no_mangle]
pub extern "C" fn IRQ_26_HANDLER() {
    tick_handler().handle_irq()
}