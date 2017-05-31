use chip::tim_gen::*;
use hal::rcc;
use hal::tim_gen::TimGenExt;

pub fn delay(ms: u32) {    
    // Clock at 32MHz
    // Divide by 16,000 => 2khz
    // Set auto_reload to ms x 2
    let t = TIM21;
    rcc::enable(&t);
    t.delay(ms << 1, 15999);
    rcc::disable(&t);
}