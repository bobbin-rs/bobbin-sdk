#![no_std]
#![no_main]

#[macro_use]
extern crate blue_pill as board;
use board::hal::clock::*;
use board::chip::usart::*;
use board::chip::tim_gen::*;
use board::chip::tim_adv::*;
use board::chip::iwdg::*;
use board::chip::wwdg::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let clk = board::clock::CLK;
    println!("Clock Test");
    println!("Current Source: {:?}", clk.sysclk_src());

    println!("LSI_ON: {} LSI_RDY: {}", clk.lsi_on(), clk.lsi_rdy());
    println!("LSE_ON: {} LSE_RDY: {}", clk.lse_on(), clk.lse_rdy());
    println!("HSI_ON: {} HSI_RDY: {}", clk.hsi_on(), clk.hsi_rdy());
    println!("HSE_ON: {} HSE_RDY: {}", clk.hse_on(), clk.hse_rdy());
    println!("PLL_ON: {} PLL_RDY: {}", clk.pll_on(), clk.pll_rdy());

    println!("LSI:      {:?}", clk.lsi());
    println!("LSE:      {:?}", clk.lse());
    println!("HSI:      {:?}", clk.hsi());
    println!("HSE:      {:?}", clk.hse());
    println!("PLLCLK:   {:?}", clk.pllclk());
    println!("SYSCLK:   {:?}", clk.sysclk());
    println!("HCLK:     {:?}", clk.hclk());
    println!("PCLK1:    {:?}", clk.pclk1());
    println!("PCLK2:    {:?}", clk.pclk2());
    println!("TCLK1:    {:?}", clk.timclk_apb1());
    println!("TCLK2:    {:?}", clk.timclk_apb2());
    println!("ADCCLK:   {:?}", clk.adcclk());
    println!("SYSTICK:  {:?}", clk.systick());
    println!("RTCCLK:   {:?}", clk.rtcclk());
    println!("FLITFCLK: {:?}", clk.flitfclk());
    println!("IWDGCLK:  {:?}", clk.iwdgclk());

    println!("USART1:   {:?}", clk.clock(&USART1));
    println!("USART2:   {:?}", clk.clock(&USART2));
    println!("USART3:   {:?}", clk.clock(&USART3));
    println!("TIM1:     {:?}", clk.clock(&TIM1));
    println!("TIM2:     {:?}", clk.clock(&TIM2));
    println!("TIM3:     {:?}", clk.clock(&TIM3));
    println!("TIM4:     {:?}", clk.clock(&TIM4));
    println!("IWDG:     {:?}", clk.clock(&IWDG));
    println!("WWDG:     {:?}", clk.clock(&WWDG));

    println!("Switching to HSI to HSE to PLL");
    board::delay(10);
    clk
        // To HSI
        .set_hsi_on(true)
        .wait_hsi_rdy()
        .set_sysclk_src(SysClockSrc::Hsi)
        .wait_sysclk_rdy()
        .set_pll_on(false)
        .set_hse_on(false)

        // To HSE
        .set_hse_on(true)
        .wait_hse_rdy()
        .set_sysclk_src(SysClockSrc::Hse)
        .wait_sysclk_rdy()
        .set_hsi_on(false)

        // To PLL
        .set_pll_on(true)
        .wait_pll_rdy()
        .set_sysclk_src(SysClockSrc::Pll)
        .wait_sysclk_rdy();

    println!("Back on PLL");

    if false {
        println!("");

        println!("LSI: {:?}", clk.lsi());
        println!(" turning LSI ON");
        clk.set_lsi_on(true);
        while !clk.lsi_rdy() {}
        println!("LSI: {:?}", clk.lsi());
        println!(" turning LSI OFF");
        clk.set_lsi_on(false);
        while clk.lsi_rdy() {}
        println!("LSI: {:?}", clk.lsi());

        println!("");    
        println!("LSE: {:?}", clk.lse());
        println!(" turning LSE ON");
        clk.set_lse_on(true);
        while !clk.lse_rdy() {}
        println!("LSE: {:?}", clk.lse());
        println!(" turning LSE OFF");
        clk.set_lse_on(false);
        while clk.lse_rdy() {}
        println!("LSE: {:?}", clk.lse());

        println!("");

        println!("HSI: {:?}", clk.hsi());
        println!(" turning HSI on");
        clk.set_hsi_on(true);
        while !clk.hsi_rdy() {}
        println!("HSI: {:?}", clk.hsi());
        println!(" turning HSI off");
        clk.set_hsi_on(false);
        while clk.hsi_rdy() {}
        println!("HSI: {:?}", clk.hsi());
        
        println!("");
        println!("RTC_EN: {}", clk.rtc_enabled());
        println!(" enable RTC");
        clk.set_rtc_enabled(true);
        println!("RTC_EN: {}", clk.rtc_enabled());
        println!(" disable RTC");
        clk.set_rtc_enabled(false);
        println!("RTC_EN: {}", clk.rtc_enabled());
    }
    loop {}
}