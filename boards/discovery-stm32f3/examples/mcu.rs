#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f3 as board;

use board::common::board::*;
use board::common::mcu::*;
use board::common::periph::*;
use board::mcu::gpio;
use board::mcu::wwdg;
use board::mcu::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();

    let brd = board::board();
    let mcu = brd.mcu();

    println!("Board Test");
    println!("Board: {}", brd.id());
    println!("MCU:   {}", mcu.id());
    
    // Get Instance

    let gpioa = mcu.gpioa();

    println!("GPIOA is {} of {}", gpioa.index(), <Stm32f74x as GetPeriphInstance<gpio::GpioPeriph>>::get_periph_instance_count(&mcu));

    println!("GPIOA_MODER: {:?}", gpioa.moder());

    // Get Instance by Type

    let gpioa: gpio::Gpioa = mcu.get();
    println!("GPIOA_MODER: {:?}", gpioa.moder());

    // Get Periph by Type and Index

    let gpioa: Option<gpio::GpioPeriph> = mcu.get_periph_instance(0);
    if let Some(gpioa) = gpioa {
        println!("GPIOA_MODER: {:?}", gpioa.moder());    
    }

    // Get Periph by Type

    let wwdg: wwdg::WwdgPeriph = mcu.get_periph();
    println!("WWDG_CFR: {:?}", wwdg.cfr());

    use_wwdg(mcu);
    
    loop {
        println!("Tick...");
        board::delay(500);
    }
}

fn use_wwdg<M: GetPeriph<wwdg::WwdgPeriph>>(mcu: M) {
    let w: wwdg::WwdgPeriph = mcu.get_periph();
    println!("WWDG_CFG: {:?}", w.cfr());
}