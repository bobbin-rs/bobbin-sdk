use hal::tim::*;
use hal::clock::Clock;
use clock::CLK;

pub const TIM: Tim14 = TIM14;

// PLL Mode with 8Mhz External Oscillator
//   120Mhz System Clock
//   60Mhz AHB Clock
//   30Mhz APB1 Clock
//   60Mhz APB2 Clock
//   9Mhz SysTick clock

// Clock at 30MHz
// Divide by 30,000 => 2khz
// Set auto_reload to ms x 2

pub fn init() {
    TIM.rcc_enable();
}

pub fn delay(ms: u32) {    
    TIM.delay(ms << 1, ((TIM.clock(&CLK).unwrap() / 2000) - 1) as u16);        
}