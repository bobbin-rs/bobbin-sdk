use hal::tim::*;
use clock::CLK;
use hal::clock::Clock;
pub const TIM: Tim15 = TIM15;
pub const TIM_PRESCALE: u16 = 35999;

// PLL Mode with 8Mhz External Oscillator
//   72Mhz System Clock
//   72Mhz AHB Clock
//   36Mhz APB1 Clock
//   72Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM15 is APB2 Clock = 72MHz

// Clock at 72MHz
// Divide by 36,000 => 2khz
// Set auto_reload to ms x 2

pub fn init() {
    TIM.rcc_enable();
}

pub fn delay(ms: u32) {    
    TIM.delay(ms << 1, TIM.clock(&CLK).unwrap() as u16);
}