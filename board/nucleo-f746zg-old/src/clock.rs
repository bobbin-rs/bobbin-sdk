use bobbin_mcu::hz::Hz;
use bobbin_mcu::clock::Clock;

use mcu::clock::*;
// use mcu::ext::rcc::DedicatedClock;
use mcu::ext::clock::{DynamicClock, Osc8m, Osc32k};
// use core::marker::PhantomData;

pub fn init() {
    // 8 MHz External Clock
    // VCO = 432MHz
    // PLL = 216MHz
    // PLLQ = 48MHz
    // SYSCLK = 216MHz
    // AHB = 216MHz
    // APB1 = 54MHz
    // APB2 = 108MHz

    clock_init::enable_pll_hse_bypass_mode(8, 432, 0b00, 9);
}

pub type SystemClockProvider = DynamicClock<Osc8m, Osc32k>;
pub type SystemClock = Clocks<SystemClockProvider>;

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

// pub mod clock_init {
//     use ::mcu::{rcc, flash, pwr};

//     pub fn enable_pll_hse_bypass_mode(m: u32, n: u32, p: u32, q: u32) {
//         let rcc = rcc::RCC;
//         let flash = flash::FLASH;
//         let pwr = pwr::PWR;        

//         // Enable internal high-speed oscillator.
//         rcc.with_cr(|r| r.set_hsion(1));

//         // Wait for HSI Ready
//         while rcc.cr().hsirdy() == 0 {}

//         // Select HSI as SYSCLK source. 
//         rcc.with_cfgr(|r| r.set_sw(0b00));
//         while rcc.cfgr().sws() != 0b00 {}

//         // Enable external high-speed oscillator 8MHz.
//         // rcc.with_cr(|r| r.set_hseon(1));

//         // Enable external source
//         rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(1));
        
//         // Wait for HSE Ready
//         while rcc.cr().hserdy() == 0 {}

//         pwr.with_csr1(|r| r.set_vosrdy(1));

//         // Set prescalers for AHB, ADC, ABP1, ABP2

//         // HPRE = HPRE_DIV_NONE
//         // PPRE1 = PPRE_DIV_4
//         // PPRE2 = PPRE_DIV_2
//         rcc.with_cfgr(|r| r.set_hpre(0).set_ppre1(0b101).set_ppre2(0b100));

//         rcc.with_pllcfgr(|r|
//             r.set_pllsrc(1)
//                 .set_pllm(m)
//                 .set_plln(n)
//                 .set_pllp(p)
//                 .set_pllq(q)
//         );

//         // Enable PLL oscillator and wait for it to stabilize.
//         rcc.with_cr(|r| r.set_pllon(1));


//         // Enable Overdrive

//         rcc.with_apb1enr(|r| r.set_pwren(1));

//         pwr.with_cr1(|r| r.set_oden(1));
//         while !pwr.csr1().test_odrdy() {}

//         pwr.with_cr1(|r| r.set_odswen(1));
//         while !pwr.csr1().test_odswrdy() {}

//         // Wait for PLL Ready
//         while rcc.cr().pllrdy() == 0 {}

//         // Configure flash settings.

//         flash.with_acr(|r| r.set_icen(1).set_dcen(1).set_latency(7));
        
//         // Select PLL as SYSCLK source.

//         rcc.with_cfgr(|r| r.set_sw(0b10));
//         while rcc.cfgr().sws() != 0b10 {}

//         // Disable internal high-speed oscillator.        
//         // rcc.with_cr(|r| r.set_hsion(0));
//     }

//     pub fn enable_pll_hse_mode(m: u32, n: u32, p: u32, q: u32) {
//         let rcc = rcc::RCC;
//         let flash = flash::FLASH;
//         let pwr = pwr::PWR;

//         // Enable internal high-speed oscillator.
//         rcc.with_cr(|r| r.set_hsion(1));

//         // Wait for HSI Ready
//         while rcc.cr().hsirdy() == 0 {}

//         // Select HSI as SYSCLK source. 
//         rcc.with_cfgr(|r| r.set_sw(0b00));
//         while rcc.cfgr().sws() != 0b00 {}

//         // Enable external high-speed oscillator 8MHz.
//         // rcc.with_cr(|r| r.set_hseon(1));

//         // Enable external source
//         rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(0));
        
//         // Wait for HSE Ready
//         while rcc.cr().hserdy() == 0 {}

//         pwr.with_csr1(|r| r.set_vosrdy(1));

//         // Set prescalers for AHB, ADC, ABP1, ABP2

//         // HPRE = HPRE_DIV_NONE
//         // PPRE1 = PPRE_DIV_4
//         // PPRE2 = PPRE_DIV_2
//         rcc.with_cfgr(|r| r.set_hpre(0).set_ppre1(0b101).set_ppre2(0b100));
        
//         rcc.with_pllcfgr(|r|
//             r.set_pllsrc(1)
//                 .set_pllm(m)
//                 .set_plln(n)
//                 .set_pllp(p)
//                 .set_pllq(q)
//         );

//         // Enable PLL oscillator and wait for it to stabilize.
//         rcc.with_cr(|r| r.set_pllon(1));
        
//         // Wait for PLL Ready
//         while rcc.cr().pllrdy() == 0 {}

//         // Configure flash settings.

//         flash.with_acr(|r| r.set_icen(1).set_dcen(1).set_latency(7));
        
//         // Select PLL as SYSCLK source.

//         rcc.with_cfgr(|r| r.set_sw(0b10));
//         while rcc.cfgr().sws() != 0b10 {}
        
//         // // Disable internal high-speed oscillator.        
//         // rcc.with_cr(|r| r.set_hsion(0));
//     }

//     pub fn enable_pll_hsi_mode() {
//         let rcc = rcc::RCC;
//         let flash = flash::FLASH;
//         let pwr = pwr::PWR;

//         // Enable internal high-speed oscillator.
//         rcc.with_cr(|r| r.set_hsion(1));

//         // Wait for HSI Ready
//         while rcc.cr().hsirdy() == 0 {}

//         // // Select HSI as SYSCLK source. 
//         // rcc.with_cfgr(|r| r.set_sw(0b00));
//         // while RCC.cfgr().sws() != 0b00 {}

//         pwr.with_csr1(|r| r.set_vosrdy(1));

//         // Set prescalers for AHB, ADC, ABP1, ABP2

//         // HPRE = HPRE_DIV_NONE
//         // PPRE1 = PPRE_DIV_4
//         // PPRE2 = PPRE_DIV_2
//         rcc.with_cfgr(|r| r.set_hpre(0).set_ppre1(0b101).set_ppre2(0b100));

//         // Configure PLL
//         // PLLSRC = HSE
//         // M = 8
//         // N = 336
//         // P = 2
//         // Q = 7
//         // R = 0
        
//         rcc.with_pllcfgr(|r|
//             r.set_pllsrc(0)
//                 .set_pllm(8)
//                 .set_plln(432)
//                 .set_pllp(2)
//                 .set_pllq(9)
//         );

//         // // rcc.with_pllcfgr(|r|
//         // //     r.set_pllsrc(1)
//         // //         .set_pllq3(0).set_pllq2(1).set_pllq1(1).set_pllq0(1)
//         // //         .set_pllp1(0).set_pllp0(0)
//         // //         .set_plln8(1).set_plln7(0).set_plln6(1).set_plln5(0).set_plln4(1).set_plln3(0).set_plln2(0).set_plln1(0).set_plln0(0)
//         // //         .set_pllm5(0).set_pllm4(0).set_pllm3(1).set_pllm2(0).set_pllm1(0).set_pllm0(0)                                
//         // // );

//         // Enable PLL oscillator and wait for it to stabilize.
//         rcc.with_cr(|r| r.set_pllon(1));
        
//         // Wait for PLL Ready
//         while rcc.cr().pllrdy() == 0 {}

//         // // Configure flash settings.

//         flash.with_acr(|r| r.set_icen(1).set_dcen(1).set_latency(5));
        
//         // // Select PLL as SYSCLK source.

//         rcc.with_cfgr(|r| r.set_sw(0b10));
//         while rcc.cfgr().sws() != 0b10 {}    
//     }
// }