pub use chip::tim_bas::*;
pub use bobbin_common::{IrqNum, WrapHandler, HandleIrq};
pub use bobbin_common::enabled::*;
pub use bobbin_common::timer::*;
pub use core::ops::Deref;

use ::hal::nvic;

use core::ptr;
use core::cell::UnsafeCell;

impl TimBasPeriph {
    pub fn one_pulse_mode(&self) -> bool {
        self.cr1().opm() != 0
    }
    pub fn set_one_pulse_mode(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_cr1(|r| r.set_opm(value));
        self
    }
}

impl Start<u16> for TimBasPeriph {
    fn start(&self, period: u16) -> &Self {
        self.start_up(period)
    }
}

impl StartOnce<u16> for TimBasPeriph {
    fn start_once(&self, period: u16) -> &Self {
        self.start_up_once(period)
    }
}

impl StartUp<u16> for TimBasPeriph {
    fn start_up(&self, period: u16) -> &Self {
        self
            .disable()
            .set_one_pulse_mode(false)
            .set_period(period)
            .clr_timeout()
            .enable()
    }
}


impl StartUpOnce<u16> for TimBasPeriph {
    fn start_up_once(&self, period: u16) -> &Self {
        self
            .disable()
            .set_one_pulse_mode(true)
            .set_period(period)
            .clr_timeout()
            .enable()
    }
}




impl Enabled for TimBasPeriph {
    fn enabled(&self) -> bool {
        self.cr1().cen() != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_cr1(|r| r.set_cen(value));
        self
    }
}


impl Timeout for TimBasPeriph {
    fn test_timeout(&self) -> bool {
        self.sr().uif() != 0
    }
    fn clr_timeout(&self) -> &Self {
        self.set_sr(|r| r.set_uif(0))
    }
}

impl Prescale<u16> for TimBasPeriph {
    fn prescale(&self) -> u16 {
        self.psc().psc().into()
    }
}

impl SetPrescale<u16> for TimBasPeriph {
    fn set_prescale(&self, value: u16) -> &Self {
        self.set_psc(|r| r.set_psc(value))        
    }
}

impl Period<u16> for TimBasPeriph {
    fn period(&self) -> u16 {
        self.arr().arr().into()
    }
}

impl SetPeriod<u16> for TimBasPeriph {
    fn set_period(&self, value: u16) -> &Self {
        self.set_arr(|r| r.set_arr(value))        
    }
}

impl Counter<u16> for TimBasPeriph {
    fn counter(&self) -> u16 {
        self.cnt().cnt().into()
    }
}

impl SetCounter<u16> for TimBasPeriph {
    fn set_counter(&self, value: u16) -> &Self {
        self.set_cnt(|r| r.set_cnt(value))        
    }
}


pub struct TimBasCounter<I: Sync + Send + WrapHandler + IrqNum, T: Sync + Send + Deref<Target=TimBasPeriph>> {
    irq: I,
    tim: T,
    count: UnsafeCell<u32>,
}

unsafe impl<I: Sync + Send + WrapHandler + IrqNum, T: Sync + Send + Deref<Target=TimBasPeriph>> Sync for TimBasCounter<I, T> {}

impl<I: Sync + Send + WrapHandler + IrqNum, T: Sync + Send + Deref<Target=TimBasPeriph>> TimBasCounter<I, T> {
    pub fn new(irq: I, tim: T) -> Self {
        TimBasCounter {
            irq: irq,
            tim: tim,
            count: UnsafeCell::new(0)
        }
    }

    #[inline]
    pub fn tim(&self) -> &TimBasPeriph {
        self.tim.deref()
    }

    pub fn enable(&self) {
        self.irq.wrap_handler(self); 
        nvic::set_enabled(self.irq.irq_num() as usize, true);
        self.tim().with_dier(|r| r.set_uie(1));
        self.tim().set_enabled(true);

    }

    pub fn disable(&self) {
        self.tim().set_enabled(false);
        self.tim().with_dier(|r| r.set_uie(0));       
        nvic::set_enabled(self.irq.irq_num() as usize, false); 
    }    

    #[inline]
    pub fn get(&self) -> u32 {
        unsafe { ptr::read_volatile(self.count.get()) }
    }

    #[inline]
    pub fn set(&self, value: u32) {
        unsafe { ptr::write_volatile(self.count.get(), value) }
    }

    #[inline]
    pub fn incr(&self, value: u32) {
        self.set(self.get().wrapping_add(value))
    }

}

impl<I: Sync + Send + WrapHandler + IrqNum, T: Sync + Send + Deref<Target=TimBasPeriph>> HandleIrq for TimBasCounter<I, T> {
    fn handle_irq(&self) {
        if self.tim().test_timeout() {
            self.tim().clr_timeout();
            self.incr(1);
        }
    }
}

impl<I: Sync + Send + WrapHandler + IrqNum, T: Sync + Send + Deref<Target=TimBasPeriph>> Counter<u32> for TimBasCounter<I, T> {
    fn counter(&self) -> u32 {
        self.get()
    }
}

impl<I: Sync + Send + WrapHandler + IrqNum, T: Sync + Send + Deref<Target=TimBasPeriph>> SetCounter<u32> for TimBasCounter<I, T> {
    fn set_counter(&self, value: u32) -> &Self {
        self.set(value);
        self
    }
}