pub use mcu::clock::*;
pub use mcu::ext::clock::{DynamicClock, clock_init};

pub fn init() {
    clock_init::init_pll()
}

// Main System Clock = 80 MHz
// APB2 = 80 MHz
// APB1 = 80 MHz
// AHB = 80 MHz
// 
// HSI @ 16 MHz
// VCO @ 160 MHz (M = 1, N = 10)
// PLL @ 80 Mhz (R = 2)
// FLASH: 4 wait states

pub type SystemClockProvider = DynamicClock<Osc8m, Osc32k>;
pub type SystemClocks = Clocks<SystemClockProvider>;

#[derive(Default)]
pub struct Osc8m {}
impl Clock for Osc8m {
    fn hz() -> Hz { Hz::from_num(8_000_000) }
}

#[derive(Default)]
pub struct Osc32k {}
impl Clock for Osc32k {
    fn hz() -> Hz { Hz::from_num(32768) }
}
