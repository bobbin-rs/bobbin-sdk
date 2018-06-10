#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f429i as board;

use board::prelude::*;
use board::mcu::usart;
use board::mcu::i2c;
use board::mcu::tim_bas::*;
use board::mcu::tim_gen::*;
use board::mcu::tim_adv::*;
use board::mcu::clock::ClockProvider;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let sys = board::init();

    let clocks = sys.clk();

    println!("Clocks");
    println!("lsi:       {:>12}", clocks.lsi().as_u32());
    println!("lse:       {:>12}", clocks.lse().as_u32());
    println!("hsi:       {:>12}", clocks.hsi().as_u32());
    println!("hse:       {:>12}", clocks.hse().as_u32());
    println!("pllclk:    {:>12}", clocks.pllclk().as_u32());
    println!("sysclk:    {:>12}", clocks.sysclk().as_u32());
    println!("");
    println!("USART1:    {:>12}", clocks.clock_for(usart::USART1).as_u32());
    println!("USART2:    {:>12}", clocks.clock_for(usart::USART2).as_u32());
    println!("USART3:    {:>12}", clocks.clock_for(usart::USART3).as_u32());
    println!("UART4:     {:>12}", clocks.clock_for(usart::UART4).as_u32());
    println!("UART5:     {:>12}", clocks.clock_for(usart::UART5).as_u32());
    println!("USART6:    {:>12}", clocks.clock_for(usart::USART6).as_u32());
    println!("UART7:     {:>12}", clocks.clock_for(usart::UART7).as_u32());
    println!("UART8:     {:>12}", clocks.clock_for(usart::UART8).as_u32());
    println!("");
    println!("I2C1:      {:>12}", clocks.clock_for(i2c::I2C1).as_u32());
    println!("I2C2:      {:>12}", clocks.clock_for(i2c::I2C2).as_u32());
    println!("I2C3:      {:>12}", clocks.clock_for(i2c::I2C3).as_u32());
    println!("");
    println!("TIM1:      {:>12}", clocks.clock_for(TIM1).as_u32());
    println!("TIM2:      {:>12}", clocks.clock_for(TIM2).as_u32());
    println!("TIM3:      {:>12}", clocks.clock_for(TIM3).as_u32());
    println!("TIM4:      {:>12}", clocks.clock_for(TIM4).as_u32());
    println!("TIM5:      {:>12}", clocks.clock_for(TIM5).as_u32());
    println!("TIM6:      {:>12}", clocks.clock_for(TIM6).as_u32());
    println!("TIM7:      {:>12}", clocks.clock_for(TIM7).as_u32());
    println!("TIM8:      {:>12}", clocks.clock_for(TIM8).as_u32());
    println!("TIM9:      {:>12}", clocks.clock_for(TIM9).as_u32());
    println!("TIM10:     {:>12}", clocks.clock_for(TIM10).as_u32());
    println!("TIM11:     {:>12}", clocks.clock_for(TIM11).as_u32());
    println!("TIM12:     {:>12}", clocks.clock_for(TIM12).as_u32());
    loop {}
}
