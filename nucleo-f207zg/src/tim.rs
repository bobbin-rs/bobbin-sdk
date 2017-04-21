use chip::tim_gen::{TIM2, TIM3, TIM4, TIM5, TIM9, TIM10, TIM11, TIM12, TIM13, TIM14};
use hal::rcc;
use hal::tim_gen;

macro_rules! timgen_def {
    ($id:ident, $id_un:ident, $dev:expr) => {
        pub fn $id() -> tim_gen::TimGenDevice {
            rcc::set_tim_gen_enabled($dev, true);    
            tim_gen::device($dev)
        }

        pub unsafe fn $id_un() -> tim_gen::TimGenDevice {
            tim_gen::device($dev)
        }
    }
}

timgen_def!(tim2, tim2_unchecked, TIM2);
timgen_def!(tim3, tim3_unchecked, TIM3);
timgen_def!(tim4, tim4_unchecked, TIM4);
timgen_def!(tim5, tim5_unchecked, TIM5);
timgen_def!(tim9, tim9_unchecked, TIM9);
timgen_def!(tim10, tim10_unchecked, TIM10);
timgen_def!(tim11, tim11_unchecked, TIM11);
timgen_def!(tim12, tim12_unchecked, TIM12);
timgen_def!(tim13, tim13_unchecked, TIM13);
timgen_def!(tim14, tim14_unchecked, TIM14);


// PLL Mode with 8Mhz External Oscillator
//   120Mhz System Clock
//   60Mhz AHB Clock
//   30Mhz APB1 Clock
//   60Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM14 is APB1 Clock = 42MHz

pub fn delay(ms: u32) {
    let t = tim14();
    // Clock at 30MHz
    // Divide by 30,000 => 2khz
    t.set_prescaler(29_999);
    t.set_update_event();
    t.clr_update_interrupt_flag();
    // Set auto_reload to ms x 2
    t.set_auto_reload(ms << 1);
    t.set_enabled(true);
    while t.update_interrupt_flag() == false {}
    t.clr_update_interrupt_flag();
    t.set_enabled(false);
}