// use chip::flash::FLASH;
use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    hse_osc: Some(8_000_000),
    lse_osc: Some(32767),
};

// Main System Clock = 32MHz
// APB2 = 32MHz
// APB1 = 32MHz
// AHB = 32MHz

pub fn init() {
    clock::init_pll();
    // // (1) Set one wait state in Latency bit of FLASH_ACR 
    // FLASH.with_acr(|r| r.set_latency(1));

    // // (2) Check the latency is set
    // while FLASH.acr().latency() == 0 {}

    // CLK
    //     .set_pll_on(false)
    //     // Set up HSI16
    //     .set_hsi16_div(false)
    //     .set_hsi16_on(true)
    //     .wait_hsi16_rdy()
    //     // Set up PLL - Multiply 6 / Divide by 2
    //     .set_pll_mul(6)
    //     .set_pll_div(3)
    //     .set_pll_on(true)
    //     .wait_pll_rdy()
    //     .set_sysclk_src(clock::SysClockSrc::Pll)
    //     .wait_sysclk_rdy()
    //     .set_msi_on(false);
}