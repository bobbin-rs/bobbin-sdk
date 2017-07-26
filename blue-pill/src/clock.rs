use hal::clock::*;
use chip::flash::FLASH;

pub const CLK: ClockTree = ClockTree { hse_osc: Some(8_000_000) };

pub fn init() {    
    // 8Mhz external clock
    // SYSCLK = PLLCLK = HSE x 9
    // System = 72Mhz
    // AHB = 72Mhz (Divide by 1)
    // APB1 = 36Mhz (Divide by 2)
    // APB2 = 72Mhz (Divide by 1)
    // Flash = 2 wait states

    // LATENCY = 2 Wait States (0b010)
    FLASH.with_acr(|r| r.set_latency(0b010));

    CLK
        // Switch to HSI
        .set_hsi_on(true)
        .wait_hsi_rdy()
        // Enable HSE
        .set_hse_on(true)
        .wait_hse_rdy()
        // Setup PLL
        .set_pll_src(PllSrc::Hse)
        .set_pll_mul(9)        
        // Setup Dividers
        .set_hclk_pre(HPre::Div1)
        .set_pclk1_pre(PPre1::Div2)
        .set_pclk2_pre(PPre2::Div1)
        // Enable PLL
        .set_pll_on(true)
        .wait_pll_rdy()
        // Switch to PLL
        .set_sysclk_src(SysClockSrc::Pll)
        .wait_sysclk_rdy();

}