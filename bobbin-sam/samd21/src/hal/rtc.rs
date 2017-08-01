pub use ::chip::rtc::*;
use ::chip::gclk;
use ::chip::irq;
use ::hal::pm;
use cortex_core::hal::nvic;

pub type Handler = unsafe extern "C" fn();

pub enum Prescaler {
    Div1 = 0x0,
    Div2 = 0x1,
    Div4 = 0x2,
    Div8 = 0x3,
    Div16 = 0x4,
    Div32 = 0x5,
    Div64 = 0x6,
    Div128 = 0x7,
    Div256 = 0x8,
    Div512 = 0x9,
    Div1024 = 0xA,
}

pub fn set_handler(_rtc: Rtc, handler: Option<Handler>) {
    irq::IRQ_RTC.set_handler(handler);
}

pub fn set_handler_enabled(_rtc: Rtc, value: bool) {
    nvic::set_enabled(irq::IRQ_RTC.0, value);
}

pub fn init_mode0(rtc: Rtc, prescaler: Prescaler) {
    unsafe {
        let mut m0 = rtc.mode0();

        pm::set_rtc_enabled(RTC, true);

        // Set GCLK_GEN01 (XOSC32K) as source for RTC

        gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
            .set_id(0x04)
            .set_gen(0x1)
            .set_clken(1)
        );

        m0.with_ctrl(|r| r.set_enable(0));
        while m0.status().syncbusy() != 0 {}
        
        m0.with_ctrl(|r| r.set_swrst(1));
        while m0.status().syncbusy() != 0 {}

        m0.set_ctrl(mode0::Ctrl(0)
            .set_mode(0)
            .set_prescaler(prescaler as u16)                
        );
    }
}

pub fn mode(rtc: Rtc) -> u16 {
    unsafe {
        let m0 = rtc.mode0();
        m0.ctrl().mode()
    }
}

pub fn enabled_mode0(rtc: Rtc) -> bool {
    unsafe {
        let m0 = rtc.mode0();
        m0.ctrl().enable() != 0
    }
}

pub fn set_enabled_mode0(rtc: Rtc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        let mut m0 = rtc.mode0();
        m0.with_ctrl(|r| r.set_enable(value));
        while m0.status().syncbusy() != 0 {}
    }
}

pub fn count_mode0(rtc: Rtc) -> u32 {
    unsafe {
        let m0 = rtc.mode0();
        m0.count().count()
    }
}

pub fn set_count_mode0(rtc: Rtc, value: u32) {
    unsafe {
        let mut m0 = rtc.mode0();
        m0.set_count(mode0::Count(0).set_count(value))
    }
}

pub fn set_comp_mode0(rtc: Rtc, index: usize, value: u32) {
    unsafe {
        let mut m0 = rtc.mode0();
        m0.set_comp(index, mode0::Comp(0).set_comp(value))            
    }
}

pub fn set_matchclr_mode0(rtc: Rtc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        let mut m0 = rtc.mode0();
        m0.with_ctrl(|r| r.set_matchclr(value));
    }
}

pub fn ovf_mode0(rtc: Rtc) -> bool {
    unsafe {
        let m0 = rtc.mode0();
        m0.intflag().ovf() != 0
    }
}

pub fn clr_ovf0_mode0(rtc: Rtc) {
    unsafe {
        let mut m0 = rtc.mode0();
        m0.set_intflag(mode0::Intflag(0).set_ovf(1));
    }
}    

pub fn cmp0_mode0(rtc: Rtc) -> bool {
    unsafe {
        let m0 = rtc.mode0();
        m0.intflag().cmp0() != 0
    }
}

pub fn clr_cmp0_mode0(rtc: Rtc) {
    unsafe {
        let mut m0 = rtc.mode0();
        m0.set_intflag(mode0::Intflag(0).set_cmp0(1));
    }
}

pub fn set_inten_cmp0_mode0(rtc: Rtc) {
    unsafe {
        let mut m0 = rtc.mode0();
        m0.set_intenset(mode0::Intenset(0).set_cmp0(1))
    }
}

pub fn clr_inten_cmp0_mode0(rtc: Rtc) {
    unsafe {
        let mut m0 = rtc.mode0();
        m0.set_intenclr(mode0::Intenclr(0).set_cmp0(1))
    }
}


pub fn init_mode2(rtc: Rtc) {
    unsafe {
        let mut m2 = rtc.mode2();

        pm::set_rtc_enabled(RTC, true);

        // Set GCLK_GEN02 (1khz) as source for RTC

        gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
            .set_id(0x04)
            .set_gen(0x2)
            .set_clken(1)
        );

        m2.with_ctrl(|r| r.set_enable(0));
        while m2.status().syncbusy() != 0 {}
        
        m2.with_ctrl(|r| r.set_swrst(1));
        while m2.status().syncbusy() != 0 {}

        m2.set_ctrl(mode2::Ctrl(0)
            .set_mode(2)
            .set_clkrep(0)
            .set_prescaler(Prescaler::Div1024 as u16)
        );
    }
}

pub fn clock_mode2(rtc: Rtc) -> mode2::Clock {
    unsafe {
        let m2 = rtc.mode2();
        m2.clock()
    }
}

pub fn set_clock_mode2(rtc: Rtc, clk: mode2::Clock) {
    unsafe {
        let mut m2 = rtc.mode2();
        m2.set_clock(clk);
    }
}

pub fn set_alarm_mode2(rtc: Rtc, index: usize, alarm: mode2::Alarm) {
    unsafe {
        let mut m2 = rtc.mode2();
        m2.set_alarm(index, alarm);
    }
}

pub fn set_alarm_mask_mode2(rtc: Rtc, index: usize, mask: u8) {
    unsafe {
        let mut m2 = rtc.mode2();
        m2.set_mask(index, mode2::Mask(0).set_sel(mask));
    }
}

pub fn alarm0_mode2(rtc: Rtc) -> bool {
    unsafe {
        let m2 = rtc.mode2();
        m2.intflag().alarm0() != 0
    }
}

pub fn clr_alarm0_mode2(rtc: Rtc) {
    unsafe {
        let mut m2 = rtc.mode2();
        m2.set_intflag(mode2::Intflag(0).set_alarm0(1));
    }
}


pub fn set_inten_alarm0_mode2(rtc: Rtc) {
    unsafe {
        let mut m2 = rtc.mode2();
        m2.set_intenset(mode2::Intenset(0).set_alarm0(1))
    }
}

pub fn clr_inten_alarm0_mode2(rtc: Rtc) {
    unsafe {
        let mut m2 = rtc.mode2();
        m2.set_intenclr(mode2::Intenclr(0).set_alarm0(1))
    }
}
