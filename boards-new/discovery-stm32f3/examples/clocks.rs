#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f3 as board;

use board::clock::*;
use board::mcu::usart;
use board::mcu::i2c;
use board::mcu::tim_bas::*;
use board::mcu::tim_gen::*;
use board::mcu::tim_adv::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("Clocks");
    println!("lsi:       {:>12}", Clk::lsi().as_u32());
    println!("lse:       {:>12}", Clk::lse().as_u32());
    println!("hsi:       {:>12}", Clk::hsi().as_u32());
    println!("hse:       {:>12}", Clk::hse().as_u32());
    println!("pllclk:    {:>12}", Clk::pllclk().as_u32());
    println!("sysclk:    {:>12}", Clk::sysclk().as_u32());
    println!("systick:   {:>12}", Clk::systick_hz().as_u32());
    println!("");
    println!("USART1:    {:>12}", Tree::clock_for(usart::USART1).as_u32());    
    println!("USART2:    {:>12}", Tree::clock_for(usart::USART2).as_u32());
    println!("USART3:    {:>12}", Tree::clock_for(usart::USART3).as_u32());
    println!("UART4:     {:>12}", Tree::clock_for(usart::UART4).as_u32());
    println!("UART5:     {:>12}", Tree::clock_for(usart::UART5).as_u32());
    println!("");
    println!("I2C1:      {:>12}", Tree::clock_for(i2c::I2C1).as_u32());
    println!("I2C2:      {:>12}", Tree::clock_for(i2c::I2C2).as_u32());
    println!("I2C3:      {:>12}", Tree::clock_for(i2c::I2C3).as_u32());
    println!("");
    println!("TIM1:      {:>12}", Tree::clock_for(TIM1).as_u32());
    println!("TIM2:      {:>12}", Tree::clock_for(TIM2).as_u32());
    println!("TIM3:      {:>12}", Tree::clock_for(TIM3).as_u32());
    println!("TIM4:      {:>12}", Tree::clock_for(TIM4).as_u32());
    println!("TIM6:      {:>12}", Tree::clock_for(TIM6).as_u32());
    println!("TIM7:      {:>12}", Tree::clock_for(TIM7).as_u32());
    println!("TIM8:      {:>12}", Tree::clock_for(TIM8).as_u32());
    loop {}
}
