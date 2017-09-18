use clock::CLK;
use hal::clock::Clock;
use hal::tim::*;

pub const TIM: Tim16 = TIM16;
pub const TIM_PRESCALE: u16 = 15999;

// Clock at 4MHz
// Divide by 16,000 => 2khz
// Set auto_reload to ms x 2

pub fn init() {
}

pub fn delay(ms: u16) {    
    TIM.rcc_enable().set_prescale((TIM.clock(&CLK).unwrap() / 2000) as u16);
    TIM.delay(ms << 1);
}