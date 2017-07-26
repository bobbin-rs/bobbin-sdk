#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;
use board::hal::clock::*;
use board::chip::lpuart::*;
use board::chip::usart::*;
use board::chip::lptim::*;
use board::chip::tim_gen::*;
// use board::chip::tim_adv::*;
// use board::chip::iwdg::*;
// use board::chip::wwdg::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let clk = board::clock::CLK;
    println!("Clock Test");
    println!("Current Source: {:?}", clk.sysclk_src());

    println!("LSI_ON: {} LSI_RDY: {}", clk.lsi_on(), clk.lsi_rdy());
    println!("LSE_ON: {} LSE_RDY: {}", clk.lse_on(), clk.lse_rdy());
    println!("MSI_ON: {} MSI_RDY: {} MSI_RANGE: {:?}", clk.msi_on(), clk.msi_rdy(), clk.msi_range());
    println!("HSI_ON: {} HSI_RDY: {}", clk.hsi16_on(), clk.hsi16_rdy());
    println!("HSE_ON: {} HSE_RDY: {} HSE_BYP: {}", clk.hse_on(), clk.hse_rdy(), clk.hse_bypass());
    println!("PLL_ON: {} PLL_RDY: {} PLL_MUL: {} PLL_DIV: {}", clk.pll_on(), clk.pll_rdy(), clk.pll_mul(), clk.pll_div());

    println!("LSI:      {:?}", clk.lsi());
    println!("LSE:      {:?}", clk.lse());
    println!("MSI:      {:?}", clk.msi());
    println!("HSI:      {:?}", clk.hsi16());
    println!("HSE:      {:?}", clk.hse());
    println!("PLLCLK:   {:?}", clk.pllclk());
    println!("SYSCLK:   {:?}", clk.sysclk());
    println!("HCLK:     {:?}", clk.hclk());
    println!("PCLK1:    {:?}", clk.pclk1());
    println!("PCLK2:    {:?}", clk.pclk2());
    println!("TCLK1:    {:?}", clk.timclk_apb1());
    println!("TCLK2:    {:?}", clk.timclk_apb2());
    println!("SYSTICK:  {:?}", clk.systick());

    println!("LPUART1:   {:?}", clk.clock(&LPUART1));
    println!("USART2:   {:?}", clk.clock(&USART2));
    println!("LPTIM:     {:?}", clk.clock(&LPTIM));
    println!("TIM2:     {:?}", clk.clock(&TIM2));
    println!("TIM21:     {:?}", clk.clock(&TIM21));
    println!("TIM22:     {:?}", clk.clock(&TIM22));

    println!("PRESCALE: {}", (clk.clock(&TIM21).expect("No clock defined") as u32 / 2000 ) as u32 - 1);
    // println!("TIM3:     {:?}", clk.clock(&TIM3));
    // println!("TIM4:     {:?}", clk.clock(&TIM4));
    // println!("IWDG:     {:?}", clk.clock(&IWDG));
    // println!("WWDG:     {:?}", clk.clock(&WWDG));

    // println!("Switching to HSI to PLL");
    // board::delay(10);
    // clk
    //     // To HSI
    //     .set_hsi16_on(true)
    //     .wait_hsi16_rdy()
    //     .set_sysclk_src(SysClockSrc::Hsi16)
    //     .wait_sysclk_rdy()
    //     .set_pll_on(false)

    //     // To PLL
    //     .set_pll_on(true)
    //     .wait_pll_rdy()
    //     .set_sysclk_src(SysClockSrc::Pll)
    //     .wait_sysclk_rdy();

    // println!("Back on PLL");

    println!("Switch to MSI and Update USART");
    board::delay(10);
    clk
        // To MSI
        .set_msi_on(true)
        .wait_msi_rdy()
        .set_sysclk_src(SysClockSrc::Msi)
        .wait_sysclk_rdy()
        .set_pll_on(false)
        .set_hsi16_on(false);
    board::console::reinit();
    println!("Running on MSI, SYSCLK: {:?}", clk.sysclk());
    
    println!("Switch to HSI and Update USART");
    board::delay(10);
    clk
        // To HSI
        .set_hsi16_on(true)
        .wait_hsi16_rdy()
        .set_sysclk_src(SysClockSrc::Hsi16)
        .wait_sysclk_rdy()
        .set_pll_on(false)
        .set_msi_on(false);
    board::console::reinit();
    println!("Running on HSI, SYSCLK: {:?}", clk.sysclk());

    println!("Switch to PLL and Update USART");
    board::delay(10);
    clk
        // To PLL
        .set_pll_on(true)
        .wait_pll_rdy()
        .set_sysclk_src(SysClockSrc::Pll)
        .wait_sysclk_rdy()
        .set_hsi16_on(false);
    board::console::reinit();        
    println!("Running on PLL, SYSCLK: {:?}", clk.sysclk());        

    // if false {
    //     println!("");

    //     println!("LSI: {:?}", clk.lsi());
    //     println!(" turning LSI ON");
    //     clk.set_lsi_on(true);
    //     while !clk.lsi_rdy() {}
    //     println!("LSI: {:?}", clk.lsi());
    //     println!(" turning LSI OFF");
    //     clk.set_lsi_on(false);
    //     while clk.lsi_rdy() {}
    //     println!("LSI: {:?}", clk.lsi());

    //     println!("");    
    //     println!("LSE: {:?}", clk.lse());
    //     println!(" turning LSE ON");
    //     clk.set_lse_on(true);
    //     while !clk.lse_rdy() {}
    //     println!("LSE: {:?}", clk.lse());
    //     println!(" turning LSE OFF");
    //     clk.set_lse_on(false);
    //     while clk.lse_rdy() {}
    //     println!("LSE: {:?}", clk.lse());

    //     println!("");

    //     println!("HSI: {:?}", clk.hsi());
    //     println!(" turning HSI on");
    //     clk.set_hsi_on(true);
    //     while !clk.hsi_rdy() {}
    //     println!("HSI: {:?}", clk.hsi());
    //     println!(" turning HSI off");
    //     clk.set_hsi_on(false);
    //     while clk.hsi_rdy() {}
    //     println!("HSI: {:?}", clk.hsi());
        
    //     println!("");
    //     println!("RTC_EN: {}", clk.rtc_enabled());
    //     println!(" enable RTC");
    //     clk.set_rtc_enabled(true);
    //     println!("RTC_EN: {}", clk.rtc_enabled());
    //     println!(" disable RTC");
    //     clk.set_rtc_enabled(false);
    //     println!("RTC_EN: {}", clk.rtc_enabled());
    // }
    loop {}
}