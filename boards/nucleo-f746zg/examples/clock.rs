#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::clock::*;
use board::mcu;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Clock Test");

    let tree = tree();

    // let tree = DYN_TREE;

    println!("LSI:       {:>12}", tree.u32_for_id(LSI));
    println!("LSE:       {:>12}", tree.u32_for_id(LSE));
    println!("HSI:       {:>12}", tree.u32_for_id(HSI));
    println!("HSE:       {:>12}", tree.u32_for_id(HSE));
    println!("PLLCLK:    {:>12}", tree.u32_for_id(PLLCLK));
    println!("PLL48CLK:  {:>12}", tree.u32_for_id(PLL48CLK));
    println!("");
    println!("SYSCLK:    {:>12}", tree.u32_for_id(SYSCLK));
    println!("HCLK:      {:>12}", tree.u32_for_id(HCLK));
    println!("PCLK1:     {:>12}", tree.u32_for_id(PCLK1));
    println!("PCLK2:     {:>12}", tree.u32_for_id(PCLK2));
    println!("TIM_PCLK1: {:>12}", tree.u32_for_id(TIM_PCLK1));
    println!("TIM_PCLK2: {:>12}", tree.u32_for_id(TIM_PCLK2));
    println!("SYSTICK:   {:>12}", tree.u32_for_id(SYSTICK));
    println!("");
    println!("USART1:    {:>12}", tree.u32_for(mcu::usart::USART1));
    println!("USART2:    {:>12}", tree.u32_for(mcu::usart::USART2));
    println!("USART3:    {:>12}", tree.u32_for(mcu::usart::USART3));
    println!("UART4:     {:>12}", tree.u32_for(mcu::usart::UART4));
    println!("UART5:     {:>12}", tree.u32_for(mcu::usart::UART5));
    println!("USART6:    {:>12}", tree.u32_for(mcu::usart::USART6));
    println!("UART7:     {:>12}", tree.u32_for(mcu::usart::UART7));
    println!("UART8:     {:>12}", tree.u32_for(mcu::usart::UART8));
    println!("");
    println!("I2C1:      {:>12}", tree.u32_for(mcu::i2c::I2C1));
    println!("I2C2:      {:>12}", tree.u32_for(mcu::i2c::I2C2));
    println!("I2C3:      {:>12}", tree.u32_for(mcu::i2c::I2C3));
    println!("I2C4:      {:>12}", tree.u32_for(mcu::i2c::I2C4));
    println!("");

    // println!("HSI:   {}", tree.hsi().value());
    // println!("OSC:   {}", tree.osc().value());
    // println!("OSC32: {}", tree.osc32().value());

    // println!("");
    // println!("IWDG:  {}", tree.clock_for(&IWDG).value());


    loop {
        println!("Tick");
        board::delay(1000);
    }
}
