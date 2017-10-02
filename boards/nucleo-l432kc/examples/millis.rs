#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::cortexm::wfi;
use board::clock::CLK;
use board::common::clock::Millis;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Millis Driver Test");    
    let t = SystickCounter::init(&CLK);
    t.enable();
    let mut n = 1000;
    let mut c = 0;
    loop {            
        let ms = t.millis();            
        if ms >= n {
            println!("{}, {}", ms, c);
            n = n + 1000;
        }        
        wfi();
        c += 1;
    }
}

use board::common::clock::Systick;
use board::common::enabled::*;
use board::hal::systick::*;
use core::cell::UnsafeCell;
use core::ptr;

pub struct SystickCounter {
    counter: UnsafeCell<u32>,
}

impl SystickCounter {
    pub fn init(clk: &Systick) -> Self {
        let reload_value = (clk.systick().unwrap() / 1000) - 1;
        SYSTICK.set_enabled(false);
        SYSTICK.set_reload_value(reload_value);
        SYSTICK.set_current_value(reload_value);
        SystickCounter {
            counter: UnsafeCell::new(0)
        }
    }
    #[inline(always)]
    pub fn get(&self) -> u32 {
        unsafe { ptr::read_volatile(self.counter.get()) }
    }

    #[inline(always)]
    pub fn set(&self, value: u32) {
        unsafe { 
            ptr::write_volatile(self.counter.get(), value)
        }
    }
}

impl HandleSystick for SystickCounter {
    fn handle_systick(&self) {
        if SYSTICK.test_timeout() {
            self.set(self.get().wrapping_add(1));
        }
    }
}

impl Enabled for SystickCounter {
    fn enabled(&self) -> bool {
        SYSTICK.enabled()
    }

    fn set_enabled(&self, value: bool) -> &Self {
        SYSTICK.set_handler(self);
        SYSTICK.set_tick_interrupt(true);        
        SYSTICK.set_enabled(value);
        self
    }
}

impl Millis for SystickCounter {
    #[inline(always)]
    fn millis(&self) -> u32 {
        self.get()
    }
}
