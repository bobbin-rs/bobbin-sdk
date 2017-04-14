use chip::tim_gen::TIM15;
use hal::rcc;
use driver::tim_gen;

pub fn init() {
    rcc::set_tim_gen_enabled(TIM15, true);    
}

// PLL Mode with 8Mhz External Oscillator
//   72Mhz System Clock
//   72Mhz AHB Clock
//   36Mhz APB1 Clock
//   72Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM15 is APB2 Clock = 72MHz

pub fn delay(ms: u32) {
    let t = tim_gen::device(TIM15);
    // Clock at 72MHz
    // Divide by 36,000 => 2khz
    t.set_prescaler(35999);
    t.set_update_event();
    t.clr_update_interrupt_flag();
    // Set auto_reload to ms x 2
    t.set_auto_reload(ms << 1);
    t.set_enabled(true);
    while t.update_interrupt_flag() == false {}
    t.clr_update_interrupt_flag();
    t.set_enabled(false);
}