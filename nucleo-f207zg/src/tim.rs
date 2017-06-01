use hal::tim::*;

pub const TIM: Tim14 = TIM14;
pub const TIM_PRESCALE: u16 = 29_999;

// PLL Mode with 8Mhz External Oscillator
//   120Mhz System Clock
//   60Mhz AHB Clock
//   30Mhz APB1 Clock
//   60Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM14 is APB1 Clock = 42MHz

// Clock at 30MHz
// Divide by 30,000 => 2khz
// Set auto_reload to ms x 2

pub fn init() {
    TIM.rcc_enable();
}

pub fn delay(ms: u32) {    
    TIM.delay(ms << 1, TIM_PRESCALE);    
}