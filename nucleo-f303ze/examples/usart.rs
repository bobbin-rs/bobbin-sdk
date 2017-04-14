#![no_std]
#![no_main]

extern crate nucleo_f303ze as board;

use board::hal::rcc;
use board::chip::gpio::GPIOA;
use board::chip::usart::USART2;
use board::driver::gpio;
use board::driver::usart;

use core::fmt::Write;

// USART2
// TX = PA2(AF7)
// RX = PA3(AF7)

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    rcc::set_gpio_enabled(GPIOA, true);
    rcc::set_usart_enabled(USART2, true);
    rcc::set_usart_clock(USART2, rcc::UsartClock::Pclk);

    let tx = gpio::pin(GPIOA, 2).into_altfn(gpio::AltFn::AF7);
    let rx = gpio::pin(GPIOA, 3).into_altfn(gpio::AltFn::AF7);

    let mut u = usart::device(USART2, tx, rx);
    u.enable(36_000_000 / 115_200);

    let mut i = 0;
    loop {
        write!(u, "Hello, World: {}\r\n", i).unwrap();
        i += 1;
        board::delay(1000);
    }
}

