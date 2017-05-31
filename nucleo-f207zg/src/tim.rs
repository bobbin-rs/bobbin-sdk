use hal::tim::*;
use hal::rcc;

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
pub fn delay(ms: u32) {    
    let t = TIM14;
    rcc::enable(&t);
    t.delay(ms << 1, 29_999);
    rcc::disable(&t);
}

