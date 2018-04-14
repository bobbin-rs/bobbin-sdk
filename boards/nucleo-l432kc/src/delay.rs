use mcu::tim_gen::*;

pub const TIM: Tim16 = TIM16;
pub const TIM_CLK: u32 = 80_000_000;
pub const TIM_PRESCALE: u16 = 15999;

// Clock at 4MHz
// Divide by 16,000 => 2khz
// Set auto_reload to ms x 2

pub fn init() {
}

pub fn delay(ms: u16) {    
    // TIM.rcc_enable().set_prescale((TIM.clock(&CLK).unwrap() / 2000) as u16);
    TIM.gate_enable();
    TIM.set_prescale((TIM_CLK / 2000) as u16);
    TIM.delay(ms << 1);
}

impl ::common::hal::delay::Delay for ::NucleoL432kc {
    fn delay_ms(&self, ms: u32) {
        // NOTE: Need to make delay support u32
        delay(ms as u16)
    }
}