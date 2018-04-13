use hal::tim::*;

pub const TIM: Tim14 = TIM14;
pub const TIM_PRESCALE: u16 = 41_999;


// PLL Mode with 25Mhz External Oscillator
//   168Mhz System Clock
//   84Mhz AHB Clock
//   42Mhz APB1 Clock
//   84Mhz APB2 Clock
//   9Mhz SysTick clock
// TIM14 is APB1 Clock = 42MHz

// Clock at 42MHz
// Divide by 42,000 => 2khz
// Set auto_reload to ms x 2

pub fn init() {
    TIM.rcc_enable();
}

pub fn delay(ms: u32) {    
    TIM
        .set_prescale(TIM_PRESCALE)
        .delay((ms << 1) as u16));   
}



// pub fn delay(ms: u32) {
//     let t = tim14();
//     // Clock at 42MHz
//     // Divide by 42,000 => 2khz
//     t.set_prescaler(41_999);
//     t.set_update_event();
//     t.clr_update_interrupt_flag();
//     // Set auto_reload to ms x 2
//     t.set_auto_reload(ms << 1);
//     t.set_enabled(true);
//     while t.update_interrupt_flag() == false {}
//     t.clr_update_interrupt_flag();
//     t.set_enabled(false);
// }