use chip::tim_gen::*;
use hal::tim_gen::*;
use hal::rcc;




// PLL Mode with 8Mhz External Oscillator
//   72Mhz System Clock
//   72Mhz AHB Clock
//   36Mhz APB1 Clock
//   72Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM15 is APB2 Clock = 72MHz

pub fn delay(ms: u32) {    
    let t = TIM15;
    // Clock at 72MHz
    // Divide by 36,000 => 2khz
    // Set auto_reload to ms x 2
    rcc::enable(&t);
    t.delay(ms << 1, 35999);
    rcc::disable(&t);
}