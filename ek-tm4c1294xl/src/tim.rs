use hal::timer::*;

pub const TIM: Timer0 = TIMER0;
pub const TIM_CLK: u32 = 120_000;

pub fn init() {
    TIM.sysctl_set_enabled(true);
}

pub fn delay(ms: u32) {
    TIM.delay(ms * TIM_CLK);
}