#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f302r8 as board;

use board::chip::rcc::RCC;
use board::hal::iwdg::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running IWDG Test");
    println!("RCC_CSR: {:?}", board::chip::rcc::RCC.csr());
    RCC.with_csr(|r| r.set_rmvf(1));

    board::delay(1000);

    // IWDG runs from LSI, approx 37khz
    // Use prescaler Div32, about 1ms per tick

    IWDG.configure(Config {
        prescaler: Prescaler::Div32,
        reload: 2000,
        window: 2000,
    });
    let mut i = 0;
    loop {
        if i < 5 {
            println!("refresh {}", i);
            IWDG.refresh();
            i += 1;
        }
        board::delay(1000);
    }
}

