#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::chip::rcc::RCC;
// use board::hal::clock::*;
// use board::chip::lpuart::*;
// use board::chip::usart::*;
// use board::chip::lptim::*;
// use board::chip::tim_gen::*;
// use board::chip::tim_adv::*;
// use board::chip::iwdg::*;
// use board::chip::wwdg::*;

use board::common::bits::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let clk = board::clock::CLK;
    println!("Clock Test");
    println!("{:?}", clk);
    // Enable HSI
    RCC.with_cr(|r| r.set_hsion(true));
    while !RCC.cr().test_hsirdy() {}
    loop {
        println!("Switching to HSI");
        board::delay(50);
        board::console::disable();        
        // Select HSI as SYSCLK source.        
        RCC.with_cfgr(|r| r.set_sw(U2::B01));
        // Wait for HSI to be selected
        while RCC.cfgr().sws() != U2::B01 {}
        board::console::enable();
        println!("Running on HSI");

        board::delay(1000);

        println!("Switching to MSI");
        board::delay(50);
        board::console::disable();
        // Select MSI as SYSCLK source.        
        RCC.with_cfgr(|r| r.set_sw(U2::B00));
        // Wait for MSI to be selected
        while RCC.cfgr().sws() != U2::B00 {}
        board::console::enable();
        println!("Running on MSI");

        board::delay(1000);        
    }

    //     println!("Switching to PLL");
    //     board::delay(50);
    //     board::console::disable();
        
    //     // Select PLL as SYSCLK source.        
    //     RCC.with_cfgr(|r| r.set_sw(U2::B11));
    //     while RCC.cfgr().sws() != U2::B11 {}        

    //     board::console::enable();
    //     println!("Running on PLL");        
    //     board::delay(1000);
    // }
}