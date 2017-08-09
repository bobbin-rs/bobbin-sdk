#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f429zi as board;

use core::marker::{Sync, Send};
use core::cell::Cell;
use board::hal::tim::*;
use board::chip::irq::IrqGuard;

pub const TIM: Tim14 = TIM14;
pub const TIM_PRESCALE: u16 = 41999;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("Timer IRQ Test");
    let h = timer(TIM14);
    h.start(1000, TIM_PRESCALE);
    loop {}

}

pub fn timer<'a, T, I>(t: Periph<T>) -> TimerHandle<'a, T> 
where Periph<T>: RegisterTimHandler + IrqTim<I>
{
    let t = Timer::new(t);
    let g = t.periph.register_tim_handler(&t);
    t.periph.irq_tim().set_enabled(true);
    TimerHandle {
        inner: t,
        _guard: g,
    }
}

pub struct TimerHandle<'a, T> {
    inner: Timer<T>,
    _guard: IrqGuard<'a>,
}

impl<'a, T> TimerHandle<'a, T> {
    fn start(&self, reload: u32, prescaler: u16) {
        self.inner.start(reload, prescaler)
    }
}


unsafe impl<T> Sync for Timer<T> {}
unsafe impl<T> Send for Timer<T> {}

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
            .set_prescale(prescaler)
            .set_update_event()
            .clr_update_interrupt_flag()
            .with_dier(|r| r.set_uie(1))
            .set_auto_reload(reload << 1)
            .set_enabled(true);        
    }    
}

impl<T> HandleTim for Timer<T> {
    fn handle_tim(&self) {
        self.periph.clr_update_interrupt_flag();                
        println!("tick {}", self.counter.get());
        self.counter.set(self.counter.get() + 1);
    }
}