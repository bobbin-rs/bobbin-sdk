#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f303ze as board;

use board::chip::usart::*;
use board::chip::tim_gen::*;
use board::chip::tim_adv::*;
use board::chip::tim_bas::*;
use board::hal::clock::Clock;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Clock");
    let clk = board::clock::CLK;
    println!("{:?}", clk);
    println!("USART1: {:?}", USART1.clock(&clk));
    println!("USART2: {:?}", USART2.clock(&clk));
    println!("USART3: {:?}", USART3.clock(&clk));
    println!("UART4: {:?}", UART4.clock(&clk));
    println!("UART5: {:?}", UART5.clock(&clk));
    println!("TIM1: {:?}", TIM1.clock(&clk));        
    println!("TIM2: {:?}", TIM2.clock(&clk));        
    println!("TIM3: {:?}", TIM3.clock(&clk));        
    println!("TIM4: {:?}", TIM4.clock(&clk));        
    println!("TIM6: {:?}", TIM6.clock(&clk));        
    println!("TIM7: {:?}", TIM7.clock(&clk));        
    println!("TIM8: {:?}", TIM8.clock(&clk));        
    println!("TIM15: {:?}", TIM15.clock(&clk));        
    println!("TIM16: {:?}", TIM16.clock(&clk));        
    println!("TIM17: {:?}", TIM17.clock(&clk));        
    println!("TIM20: {:?}", TIM20.clock(&clk));        

    loop {}
}
