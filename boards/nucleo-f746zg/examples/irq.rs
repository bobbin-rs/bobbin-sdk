#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::hal::tim::*;
use board::common::Poll;

pub struct System {
    tick: TickDriver,
    tick_irq: irq::IrqHandle,
}

impl System {
    fn start(&self) {
        self.tick_irq.enable();
        self.tick.enable();
    }

    fn stop(&self) {
        self.tick.disable();
        self.tick_irq.disable();
    }
}

static mut SYS: Option<System> = None;

fn initialize<F: FnOnce() -> System>(f: F) {
    unsafe {
        SYS = Some(f());
    }
}

fn run<F: FnOnce(&mut System)>(f: F) -> ! {
    let sys = unsafe { SYS.as_mut().unwrap() };
    sys.start();
    f(sys);
    sys.stop();
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("IRQ Test");

    initialize(|| {
        pub const TIM: Tim11 = TIM11;
        pub const TIM_PRESCALE: u16 = 41999;

        let irq_tim = TIM.irq_tim();

        TIM.rcc_enable();
        TIM.set_prescale(TIM_PRESCALE)
                .set_update_event()
                .clr_update_interrupt_flag()
                .with_dier(|r| r.set_uie(1))
                .set_auto_reload(100 << 1);

        let t = TickDriver::new(TIM);
        let h = irq::register_handler(&irq_tim, &t);
        
        System {
            tick: t,
            tick_irq: h,
        }
    });

    run(|sys| {
        println!("Timer Activated");
        for _ in 0..2 {
            println!("Enable IRQ");    
            sys.tick_irq.enable();
            board::delay(500);
            println!("Disable IRQ");
            sys.tick_irq.disable();
            board::delay(500);
        }
        println!("Done");
    })
}


use core::cell::Cell;

pub struct TickDriver {
    tim: TimGenPeriph,
    counter: Cell<usize>,
}

impl TickDriver {
    pub fn new<T: Into<TimGenPeriph>>(t: T) -> Self {
        TickDriver {
            tim: t.into(),
            counter: Cell::new(0),
        }        
    }

    pub fn enable(&self) {        
        self.tim.set_enabled(true);
    }

    pub fn disable(&self) {        
        self.tim.set_enabled(false);
    }
}
impl Poll for TickDriver {
    fn poll(&self) {
        self.tim.clr_update_interrupt_flag();
        println!("Tick {}", self.counter.get());
        self.counter.set(self.counter.get() + 1);        
    }
}

unsafe impl Send for TickDriver {}
unsafe impl Sync for TickDriver {}


pub mod irq {    
    use core::mem;    
    use ::board::common::{Irq, Poll};
    use ::board::cortexm::hal::nvic;
    use ::board::cortexm::hal::scb::SCB;

    pub trait RegisterIrq : Send + Sync + Sized + Poll {
        fn register_irq<I: Irq>(&self, irq: &I) -> IrqHandle {
            let irq_num = irq.irq_num();
            SCB.set_irq_handler(irq_num as usize, Some(irq.wrap(self)));
            IrqHandle(irq_num)
        }
    }

    pub fn register_handler<I: Irq, H: Send + Sync + Poll>(irq: &I, handler: &H) -> IrqHandle 
    {
        let irq_num = irq.irq_num();
        SCB.set_irq_handler(irq_num as usize, Some(irq.wrap(handler)));
        IrqHandle(irq_num)
    }

    pub struct IrqHandle(u8);

    impl IrqHandle {
        pub fn enabled(&self) -> bool {
            nvic::enabled(self.0 as usize)
        }

        pub fn set_enabled(&self, value: bool) {
            nvic::set_enabled(self.0 as usize, value);
        }

        pub fn enable(&self) {
            self.set_enabled(true);
        }

        pub fn disable(&self) {
            self.set_enabled(false);
        }

        pub fn unregister(self) {            
            self.disable();
            SCB.set_irq_handler(self.0 as usize, None);
            mem::forget(self)
        }
    }

    impl Drop for IrqHandle {
        fn drop(&mut self) {
            panic!("Attempted to drop registered handler for irq {}", self.0);
        }
    }
}