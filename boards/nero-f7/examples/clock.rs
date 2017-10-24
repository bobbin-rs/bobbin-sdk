#![no_std]
#![no_main]

#[macro_use]
extern crate nero_f7 as board;

use board::hal::clock::Clock;
use board::clock::CLK;

use board::chip::usart_f24::*;
use board::chip::tim_bas::*;
use board::chip::tim_gen::*;
use board::chip::tim_adv::*;

use board::chip::rcc::RCC;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Clock");
    println!("{:?}", CLK);
    println!("USART1: {:?}", USART1.clock(&CLK));
    println!("USART2: {:?}", USART2.clock(&CLK));
    println!("USART3: {:?}", USART3.clock(&CLK));
    println!("UART4: {:?}", UART4.clock(&CLK));
    println!("UART5: {:?}", UART5.clock(&CLK));
    println!("USART6: {:?}", USART6.clock(&CLK));
    println!("UART7: {:?}", UART7.clock(&CLK));
    println!("UART8: {:?}", UART8.clock(&CLK));
    println!("");
    println!("TIM1: {:?}", TIM1.clock(&CLK));
    println!("TIM2: {:?}", TIM2.clock(&CLK));
    println!("TIM3: {:?}", TIM3.clock(&CLK));
    println!("TIM4: {:?}", TIM4.clock(&CLK));
    println!("TIM5: {:?}", TIM5.clock(&CLK));
    println!("TIM6: {:?}", TIM6.clock(&CLK));
    println!("TIM7: {:?}", TIM7.clock(&CLK));
    println!("TIM8: {:?}", TIM8.clock(&CLK));
    println!("TIM9: {:?}", TIM9.clock(&CLK));
    println!("TIM10: {:?}", TIM10.clock(&CLK));
    println!("TIM11: {:?}", TIM11.clock(&CLK));
    println!("TIM12: {:?}", TIM12.clock(&CLK));
    println!("TIM13: {:?}", TIM13.clock(&CLK));
    println!("TIM14: {:?}", TIM14.clock(&CLK));
    loop {
        println!("Switching to HSE");
        board::delay(10);
        board::console::disable();
        RCC.with_cfgr(|r| r.set_sw(0b01));
        while RCC.cfgr().sws() != 0b01 {};
        board::console::enable();
        println!("Running on HSE");

        board::delay(1000);

        println!("Switching to PlL");
        board::delay(10);
        board::console::disable();
        RCC.with_cfgr(|r| r.set_sw(0b10));
        while RCC.cfgr().sws() != 0b10 {};
        board::console::enable();
        println!("Running on PLL");

        board::delay(1000);

    }

}
