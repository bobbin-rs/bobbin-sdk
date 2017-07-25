use ::chip::rcc;
use ::chip::flash;

// 8Mhz external clock
// System = 72Mhz
// AHB = 72Mhz (Divide by 1)
// APB1 = 36Mhz (Divide by 2)
// APB2 = 72Mhz (Divide by 1)

pub fn enable_pll_external_mode() {
    let rcc = rcc::RCC;
    let flash = flash::FLASH;
    
    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {}

    // Select HSI as SYSCLK source. 
    rcc.with_cfgr(|r| r.set_sw(0));

    // Enable external high-speed oscillator 8MHz.
    rcc.with_cr(|r| r.set_hseon(1));

    // Wait for HSE Ready
    while rcc.cr().hserdy() == 0 {}

    // Set prescalers for AHB, ADC, ABP1, ABP2

    // HPRE = HPRE_DIV_NONE
    // PPRE1 = PPRE_DIV_2 (0b100)
    // PPRE2 = PPRE_DIV_NONE (0b000)
    // PLLSRC = PREDIV1 (0b1)
    // PLLMUL = x9 (0b0111)
    // MCO = PLL/2 (0b0111)
    rcc.with_cfgr(|r| 
        r.set_hpre(0)
            .set_ppre1(0b100)
            .set_ppre2(0b000)
            .set_pllsrc(1)
            .set_pllmul(0b0111)
    );

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {}

    // Configure flash settings.
    // PRFTBE = enabled (0b1)
    // LATENCY = 2 Wait States (0b010)
    flash.with_acr(|r| r.set_latency(0b010));
    
    // Select PLL as SYSCLK source.

    rcc.with_cfgr(|r| r.set_sw(0b10));
    
    // Wait for PLL to be selected
    while rcc.cfgr().sws() != 0b10 {}
    
    // Disable internal high-speed oscillator.        
    rcc.with_cr(|r| r.set_hsion(0));
}