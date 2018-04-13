use mcu::tim_gen::*;
use mcu::pin::*;
use clock::*;

pub const TIM: Tim14 = TIM14;
pub const TIM_PRESCALE: u16 = 41999;

pub fn init() {
    TIM.gate_enable();
}

pub fn delay(ms: u32) { 
    TIM.gate_enable();
    let tim_clk = tree().u32_for(TIM);
    TIM
        .set_prescale(((tim_clk / 2000) - 1) as u16)
        .delay((ms << 1) as u16);
}