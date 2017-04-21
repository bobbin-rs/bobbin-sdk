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


// pub fn tim2() -> tim_gen::TimGenDevice {
//     rcc::set_tim_gen_enabled(TIM2, true);    
//     tim_gen::device(TIM2)
// }

// pub unsafe fn tim2_unchecked() -> tim_gen::TimGenDevice {
//     tim_gen::device(TIM2)
// }


// pub fn tim3() -> tim_gen::TimGenDevice {
//     rcc::set_tim_gen_enabled(TIM3, true);    
//     tim_gen::device(TIM3)
// }

// pub unsafe fn tim3_unchecked() -> tim_gen::TimGenDevice {
//     tim_gen::device(TIM3)
// }

// pub fn tim4() -> tim_gen::TimGenDevice {
//     rcc::set_tim_gen_enabled(TIM4, true);    
//     tim_gen::device(TIM4)
// }

// pub unsafe fn tim4_unchecked() -> tim_gen::TimGenDevice {
//     tim_gen::device(TIM4)
// }

// pub fn tim15() -> tim_gen::TimGenDevice {
//     rcc::set_tim_gen_enabled(TIM15, true);    
//     tim_gen::device(TIM15)
// }

// pub fn tim15_unchecked() -> tim_gen::TimGenDevice {
//     tim_gen::device(TIM15)
// }

// pub fn tim16() -> tim_gen::TimGenDevice {
//     rcc::set_tim_gen_enabled(TIM16, true);    
//     tim_gen::device(TIM16)
// }

// pub fn tim16_unchecked() -> tim_gen::TimGenDevice {
//     tim_gen::device(TIM16)
// }

// pub fn tim17() -> tim_gen::TimGenDevice {
//     rcc::set_tim_gen_enabled(TIM17, true);    
//     tim_gen::device(TIM17)
// }

// pub fn tim17_unchecked() -> tim_gen::TimGenDevice {
//     tim_gen::device(TIM17)
// }


// PLL Mode with 8Mhz External Oscillator
//   168Mhz System Clock
//   84Mhz AHB Clock
//   42Mhz APB1 Clock
//   84Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM14 is APB1 Clock = 42MHz

pub fn delay(ms: u32) {
    let t = tim14();
    // Clock at 42MHz
    // Divide by 42,000 => 2khz
    t.set_prescaler(41_999);
    t.set_update_event();
    t.clr_update_interrupt_flag();
    // Set auto_reload to ms x 2
    t.set_auto_reload(ms << 1);
    t.set_enabled(true);
    while t.update_interrupt_flag() == false {}
    t.clr_update_interrupt_flag();
    t.set_enabled(false);
}