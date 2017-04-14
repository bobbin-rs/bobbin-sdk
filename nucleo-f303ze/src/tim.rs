use chip::tim_gen::{TIM2, TIM3, TIM4, TIM15, TIM16, TIM17};
use hal::rcc;
use driver::tim_gen;

pub fn tim2() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM2, true);    
    tim_gen::device(TIM2)
}

pub unsafe fn tim2_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM2)
}


pub fn tim3() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM3, true);    
    tim_gen::device(TIM3)
}

pub unsafe fn tim3_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM3)
}

pub fn tim4() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM4, true);    
    tim_gen::device(TIM4)
}

pub unsafe fn tim4_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM4)
}

pub fn tim15() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM15, true);    
    tim_gen::device(TIM15)
}

pub fn tim15_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM15)
}

pub fn tim16() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM16, true);    
    tim_gen::device(TIM16)
}

pub fn tim16_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM16)
}

pub fn tim17() -> tim_gen::TimGenDevice {
    rcc::set_tim_gen_enabled(TIM17, true);    
    tim_gen::device(TIM17)
}

pub fn tim17_unchecked() -> tim_gen::TimGenDevice {
    tim_gen::device(TIM17)
}


// PLL Mode with 8Mhz External Oscillator
//   72Mhz System Clock
//   72Mhz AHB Clock
//   36Mhz APB1 Clock
//   72Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM15 is APB2 Clock = 72MHz

pub fn delay(ms: u32) {
    let t = tim15();
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