#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::common::mcu::*;
use board::mcu::gpio;
use board::mcu::wwdg;
use board::mcu::MCU;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("MCU Test");

    // Get Instance

    let gpioa = MCU.gpioa();
    println!("GPIOA_MODER: {:?}", gpioa.moder());

    // Get Instance by Type

    let gpioa: gpio::Gpioa = MCU.get();
    println!("GPIOA_MODER: {:?}", gpioa.moder());

    // Get Periph by Type and Index

    let gpioa: Option<gpio::GpioPeriph> = MCU.get_periph_instance(0);
    if let Some(gpioa) = gpioa {
        println!("GPIOA_MODER: {:?}", gpioa.moder());    
    }

    // Get Periph by Type

    let wwdg: wwdg::WwdgPeriph = MCU.get_periph();
    println!("WWDG_CFR: {:?}", wwdg.cfr());

    
    loop {
        println!("Tick...");
        board::delay(500);
    }
}
