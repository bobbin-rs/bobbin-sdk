use ::rcc;
use ::flash;

pub fn enable_pll_external_mode() {
    let rcc = rcc::RCC;
    let flash = flash::FLASH;
    //let mut pwr = pwr::PWR;

    // Configure flash settings.
    // Prefetch Buffer Enabled + Two Wait States
    flash.with_acr(|r| r.set_prftbe(1).set_latency(0b010));

    // Configure Prescalers

    // AHB (HCLK)  = SYSCLK
    // APB1 = HCLK / 2
    // APB2 = HCLK
    rcc.with_cfgr(|r| r.set_hpre(0b000).set_ppre1(0b100).set_ppre2(0b000));

    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {}

    // Select HSI as SYSCLK source. 
    rcc.with_cfgr(|r| r.set_sw(0b00));

    // Enable external high-speed clock 8MHz.
    rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(1));
    
    // Wait for HSE Ready
    while rcc.cr().hserdy() == 0 {}

    // Configure PLL
    // PLLSRC = HSE
    // PREDIV = 1
    // MUL = 9

    rcc.with_cfgr(|r| r.set_pllsrc(0b10).set_pllmul(0b111));

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {}
    
    // Select PLL as SYSCLK source.        
    rcc.with_cfgr(|r| r.set_sw(0b10));
    
    // Wait for PLL to be selected

    while rcc.cfgr().sws() != 0b10 {}
    
    // Enabled SYCLK output on MCO pin
    //rcc.with_cfgr(|r| r.set_mco(0b101));
    
    // Disable internal high-speed oscillator.        
    // rcc.with_cr(|r| r.set_hsion(0));
}