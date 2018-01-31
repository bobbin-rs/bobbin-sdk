use hal::tim::*;
use hal::clock::Clock;
use clock::CLK;

pub const TIM: Tim15 = TIM15;
pub const TIM_PRESCALE: u16 = 41_999;


// PLL Mode with 8Mhz External Oscillator
//   168Mhz System Clock
//   84Mhz AHB Clock
//   42Mhz APB1 Clock
//   84Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM14 is APB1 Clock = 42MHz

// Clock at 42MHz
// Divide by 42,000 => 2khz
// Set auto_reload to ms x 2

pub fn init() {
    TIM.rcc_enable();
}

pub fn delay(ms: u32) {    
    TIM
        .set_prescale((TIM.clock(&CLK).unwrap() / 2000) as u16)
        .delay((ms << 1) as u16);
}