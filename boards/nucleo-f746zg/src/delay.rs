use mcu::tim_gen::*;
use clock::*;

pub const TIM: Tim14 = TIM14;
pub const TIM_PRESCALE: u16 = 41999;

// PLL Mode with 8Mhz External Oscillator
//   168Mhz System Clock
//   168Mhz AHB Clock
//   84Mhz APB1 Clock
//   168Mhz APB2 Clock
// TIM14 is APB1 Clock = 84MHz

// Clock at 84MHz
// Divide by 2KHz = 48,000
// Set auto_reload to ms x 2

pub fn init() {
    TIM.gate_enable();
}

pub fn delay(ms: u32) { 
    TIM.gate_enable();
    let tim_clk: u32 = SystemClock::default().clock_for(TIM).into();
    TIM
        .set_prescale(((tim_clk / 2000) - 1) as u16)
        .delay((ms << 1) as u16);
}

impl ::common::hal::delay::Delay for ::NucleoF746zg {
    fn delay_ms(&self, ms: u32) {
        delay(ms)
    }
}