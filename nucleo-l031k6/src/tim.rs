use clock::CLK;
use hal::clock::Clock;
use hal::tim::*;

pub const TIM: Tim21 = TIM21;
pub const TIM_PRESCALE: u16 = 15999;

// Clock at 32MHz
// Divide by 16,000 => 2khz
// Set auto_reload to ms x 2

pub fn init() {
    TIM.rcc_enable();
}

pub fn delay(ms: u16) {    
    TIM.delay((ms << 1) as u16, ((TIM.clock(&CLK).unwrap() / 2000) - 1) as u16);
}