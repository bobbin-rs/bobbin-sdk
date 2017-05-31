use hal::tim::*;
use hal::rcc;

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
pub fn delay(ms: u32) {    
    let t = TIM14;
    rcc::enable(&t);
    t.delay(ms << 1, 41999);
    rcc::disable(&t);
}
