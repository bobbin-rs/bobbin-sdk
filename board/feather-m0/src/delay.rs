use mcu::gclk::*;
use mcu::tc::*;

pub const TC: Tc3 = TC3;

pub fn init() {
    GCLK.set_clk(GenericClock::TCC2_TC3, GenericClockGen::GClkGen2);
    TC.gate_enable();
    TC.configure_16bit(Config {
        prescsync: Prescsync::GCLK,
        runstdby: false,
        prescaler: Prescaler::Div1,
        wavegen: Wavegen::NFRQ,
    });
}

// Note: actually ticks at 1.024kHz
pub fn delay(ticks: u16) {
    TC.delay(ticks);
}


impl ::common::hal::delay::Delay for ::FeatherM0 {
    fn delay_ms(&self, ms: u32) {
        // Need to support u32
        delay(ms as u16)
    }
}