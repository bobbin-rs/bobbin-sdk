use chip::tim_gen::{TIM2, TIM21, TIM22};
use hal::rcc;
use driver::tim_gen;

pub fn tim2() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM2, true);    
    tim_gen::device(TIM2)
}

pub fn tim2_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM2)
}

pub fn tim21() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM21, true);    
    tim_gen::device(TIM21)
}

pub fn tim21_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM21)
}

pub fn tim22() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM22, true);    
    tim_gen::device(TIM22)
}

pub fn tim22_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM22)
}

pub fn delay(ms: u32) {
    let t = tim21();
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
}