use hal::clock::DynamicClock;
use hal::rcc;
use chip::flash;
use chip::pwr;

pub const CLK: DynamicClock = DynamicClock {
    hse_osc: None,
    lse_osc: None,
};

pub fn init() {
    enable_pll_hsi_mode();
}


pub fn enable_pll_hsi_mode() {
    let rcc = rcc::RCC;
    let flash = flash::FLASH;
    let pwr = pwr::PWR;

    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {}

    // // Select HSI as SYSCLK source. 
    // rcc.with_cfgr(|r| r.set_sw(0b00));
    // while RCC.cfgr().sws() != 0b00 {}

    pwr.with_csr1(|r| r.set_vosrdy(1));

    // Set prescalers for AHB, ADC, ABP1, ABP2

    // HPRE = HPRE_DIV_NONE
    // PPRE1 = PPRE_DIV_4
    // PPRE2 = PPRE_DIV_2
    rcc.with_cfgr(|r| r.set_hpre(0).set_ppre1(0b101).set_ppre2(0b100));

    // Configure PLL
    // PLLSRC = HSE
    // M = 8
    // N = 336
    // P = 2
    // Q = 7
    // R = 0
    
    rcc.with_pllcfgr(|r|
        r.set_pllsrc(0)
            .set_pllm(8)
            .set_plln(216)
            .set_pllp(0b00)
            .set_pllq(9)
    );

    // // rcc.with_pllcfgr(|r|
    // //     r.set_pllsrc(1)
    // //         .set_pllq3(0).set_pllq2(1).set_pllq1(1).set_pllq0(1)
    // //         .set_pllp1(0).set_pllp0(0)
    // //         .set_plln8(1).set_plln7(0).set_plln6(1).set_plln5(0).set_plln4(1).set_plln3(0).set_plln2(0).set_plln1(0).set_plln0(0)
    // //         .set_pllm5(0).set_pllm4(0).set_pllm3(1).set_pllm2(0).set_pllm1(0).set_pllm0(0)                                
    // // );

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {}

    // // Configure flash settings.

    flash.with_acr(|r| r.set_icen(1).set_dcen(1).set_latency(5));
    
    // // Select PLL as SYSCLK source.

    rcc.with_cfgr(|r| r.set_sw(0b10));
    while rcc.cfgr().sws() != 0b10 {}    
}