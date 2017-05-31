use chip::tim_gen::*;
use hal::rcc;
use hal::tim_gen::TimGenExt;

pub fn delay(ms: u32) {
    rcc::enable(&TIM21);
    let t = TIM21;
    // Clock at 32MHz
    // Divide by 16,000 => 2khz
    t.set_prescaler(15999);
    t.set_update_event();
    t.clr_update_interrupt_flag();
    // Set auto_reload to ms x 2
    t.set_auto_reload(ms << 1);
    t.set_enabled(true);
    while t.update_interrupt_flag() == false {}
    t.clr_update_interrupt_flag();
    t.set_enabled(false);
    rcc::disable(&TIM21);
}