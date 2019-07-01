#![no_std]

use cortex_m::asm;
use cortex_m::interrupt;
use core::panic::PanicInfo;
pub use samd51::bobbin_sys::{print, println};
pub use samd51 as mcu;
use samd51::bobbin_mcu::prelude::*;
use samd51::bobbin_hal::prelude::*;
use samd51::bobbin_sys::prelude::*;

use samd51::ext::clock;
use samd51::port::PORTB;
use samd51::sercom::SERCOM5;
use samd51::pin::{Pa23, PA23};
use samd51::gclk::{self, GCLK};

pub const LED0: Pa23 = PA23;

pub fn init() {
    clock::run_120mhz();
    for _ in 0..1000 {
        asm::nop();
    }

    // Enable LED Output
    PA23.set_output(true);

    // Set PinMux to 0x2 (C)
    PORTB.with_pmux(8, |r| r.set_pmuxe(0x2).set_pmuxo(0x2));
    // Enable PinMux
    PORTB.with_pincfg(16, |r| r.set_pmuxen(1));
    PORTB.with_pincfg(17, |r| r.set_pmuxen(1));



    // Use clockgen 0 for SERCOM5

    GCLK.write_pchctrl(35, gclk::Pchctrl(0)
        .set_gen(0x0)
        .set_chen(1)
    );

    SERCOM5.gate_enable();

    let cfg = samd51::ext::sercom::Config::default()
        .set_mode_usart_int()
        .set_baud_clock(115_200, 120_000_000)
        .set_rxpo(0x1)
        .set_txpo(0x0);
    SERCOM5.configure(cfg);
    SERCOM5.set_enabled(true);
    Console::set(Console::new(SERCOM5.as_periph(), ConsoleMode::Cooked));
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();
    println!("{}", info);
    loop {}
}