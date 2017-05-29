use ::chip::flash::*;
use ::chip::rcc::*;

// Main System Clock = 32MHz
// APB2 = 32MHz
// APB1 = 32MHz
// AHB = 32MHz

pub fn init_pll() {
    unsafe {
        // (1) Set one wait state in Latency bit of FLASH_ACR 
        FLASH.with_acr(|r| r.set_latency(1));

        // (2) Check the latency is set
        while FLASH.acr().latency() == 0 {}

        // (3) Switch the clock on HSI16/4 and disable PLL

        RCC.with_cr(|r| r.set_pllon(0).set_hsi16divf(0).set_hsi16on(1));

        // Wait for HSI16 Ready Flag
        while RCC.cr().hsi16rdyf() == 0 {}

        // (4) Set PLLMUL to 16 to get 32MHz on CPU clock, PLLDIV/2

        RCC.with_cfgr(|r| r.set_pllmul(0b0010).set_plldiv(0b10));

        // (5) Enable and switch on PLL 

        RCC.with_cr(|r| r.set_pllon(1));

        // Wait for PLL Ready
        while RCC.cr().pllrdy() == 0 {}

        // Switch to PLL
        RCC.with_cfgr(|r| r.set_sw(0b11));

        // Wait for system clock to use PLL
        while RCC.cfgr().sws() != 0b11 {}
        
    }
    
}