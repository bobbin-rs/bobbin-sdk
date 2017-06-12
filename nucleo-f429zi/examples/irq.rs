#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f429zi as board;

use core::cell::Cell;

use board::chip::irq::*;
use board::hal::tim::*;

pub const TIM: Tim14 = TIM14;
pub const TIM_PRESCALE: u16 = 41999;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("Timer IRQ Test");
    let t = Timer::new(TIM14);
    let tim_irq = IRQ_TIM8_TRG_COM_TIM14;

    let _g = tim_irq.register_handler(&t);
    board::hal::nvic::set_enabled(tim_irq.index(), true);
    t.start(1000, TIM_PRESCALE);
    loop {}

}

pub struct Timer<T> {
    periph: Periph<T>,
    counter: Cell<usize>,
}

impl<T> Timer<T> {
    fn new(periph: Periph<T>) -> Self {
        Timer { periph: periph, counter: Cell::new(0) } 
    }
    fn start(&self, reload: u32, prescaler: u16) {
        self.periph
            .set_prescaler(prescaler)
            .set_update_event()
            .clr_update_interrupt_flag()
            .with_dier(|r| r.set_uie(1))
            .set_auto_reload(reload << 1)
            .set_enabled(true);        
    }    
}

impl<T> HandleInterrupt for Timer<T> {
    fn handle_interrupt(&self) {
        self.periph.clr_update_interrupt_flag();                
        println!("tick {}", self.counter.get());
        self.counter.set(self.counter.get() + 1);
    }
}