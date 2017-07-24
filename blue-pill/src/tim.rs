use hal::tim::*;

pub const TIM: Tim4 = TIM4;
pub const TIM_PRESCALE: u16 = 35999;

// PLL Mode with 8Mhz external clock
// System = 72Mhz
// AHB = 72Mhz (Divide by 1)
// APB1 = 36Mhz (Divide by 2)
// APB2 = 72Mhz (Divide by 1)

// TIM4 is APB1 Clock = 36MHz

// Clock at 36MHz
// Divide by 36,000 => 1khz
// Set auto_reload to ms

pub fn init() {
    TIM.rcc_enable();
}

pub fn delay(ms: u32) {    
    TIM.delay(ms << 1, TIM_PRESCALE);    
}