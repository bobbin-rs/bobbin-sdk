use tim_bas::*;
// use bobbin_common::{Irq, Poll};
use bobbin_hal::enabled::*;
use bobbin_hal::timer::*;

// use core::ptr;
use core::cell::Cell;

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

pub struct TimBasCounter {
    tim: TimBasPeriph,
    count: Cell<u32>,
}

unsafe impl Sync for TimBasCounter {}

impl TimBasCounter {
    pub fn new(tim: TimBasPeriph) -> Self {
        TimBasCounter {
            tim: tim,
            count: Cell::new(0)
        }
    }

    pub fn enable(&self) {
        self.tim.with_dier(|r| r.set_uie(1));
        self.tim.set_enabled(true);

    }

    pub fn disable(&self) {
        self.tim.set_enabled(false);
        self.tim.with_dier(|r| r.set_uie(0));       
    }    

    #[inline]
    pub fn get(&self) -> u32 {
        // unsafe { ptr::read_volatile(self.count.get()) }
        // unsafe { *self.count.get() }
        self.count.get()
    }

    #[inline]
    fn set(&self, value: u32) {
        // unsafe { ptr::write_volatile(self.count.get(), value) }
        // unsafe { *self.count.get() = value; }
        self.count.set(value);
    }

    // #[inline]
    // fn inc(&self, value: u32) {
    //     self.set(self.get().wrapping_add(value))
    // }

}

// impl Poll for TimBasCounter {
//     fn poll(&self) {
//         if self.tim.test_timeout() {
//             self.tim.clr_timeout();
//             self.inc(1);
//         }
//     }
// }

impl Counter<u32> for TimBasCounter {
    fn counter(&self) -> u32 {
        self.get()
    }
}

impl SetCounter<u32> for TimBasCounter {
    fn set_counter(&self, value: u32) -> &Self {
        self.set(value);
        self
    }
}