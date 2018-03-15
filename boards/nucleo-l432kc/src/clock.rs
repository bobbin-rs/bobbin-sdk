// use common::bits::*;
pub use common::clock::*;

use mcu::rcc::RCC;
// use ::chip::pwr::PWR;
use mcu::flash::FLASH;
// use ::chip::usart::*;
// use ::chip::lpuart::*;
// use ::chip::lptim::*;
// use ::chip::tim_adv::*;
// use ::chip::tim_bas::*;
// use ::chip::tim_gen::*;
// use ::chip::i2c::*;
// use ::chip::spi::*;

// use core::fmt;

pub fn init() {
    init_pll()

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


pub fn init_pll() {
    // (1) Set one wait state in Latency bit of FLASH_ACR 
    FLASH.with_acr(|r| r.set_latency(4));

    // (2) Check the latency is set
    while FLASH.acr().latency() != 4 {}

    // (3) Switch the clock on HSI16/4 and disable PLL

    RCC.with_cr(|r| r.set_pllon(0).set_hsion(1));

    // Wait for HSI16 Ready Flag
    while RCC.cr().hsirdy() == 0 {}


    RCC.with_pllcfgr(|r| r.set_pllsrc(0b10).set_pllm(0x0).set_plln(0xa).set_pllr(0x0).set_pllren(1));

    // (5) Enable and switch on PLL 

    RCC.with_cr(|r| r.set_pllon(1));

    // Wait for PLL Ready
    while RCC.cr().pllrdy() == 0 {}

    // Switch to PLL
    RCC.with_cfgr(|r| r.set_sw(0b11));

    // Wait for system clock to use PLL
    while RCC.cfgr().sws() != 0b11 {}
}
