#![no_std]
#![no_main]

extern crate panic_abort;
use cortex_m_rt::entry;
use cortex_m::asm;

use samd51::bobbin_mcu::prelude::*;
use samd51::bobbin_hal::prelude::*;
use samd51::bobbin_sys::prelude::*;
use samd51::bobbin_sys::{print, println};

use samd51::port::PORTB;
use samd51::sercom::SERCOM5;
use samd51::pin::PA23;
use samd51::gclk;

// LED Pin D13 = a23

// Serial RX / TX
// SERCOM5
// d0 = PB17 = PinMux C (0x2) = PMUX[16 + 8].PMUX0
// d1 = PB16 = PinMux C (0x2) = PMUX[16 + 8].PMUXE
// rxpo = 0x1 - receive on SERCOM5 PAD[1]
// txpo = 0x0 - transmit on SERCOM5 PAD[0]

// use gclk0

// Clock Notes
//   from https://github.com/atsamd-rs/atsamd/blob/master/hal/src/clock51.rs
//
// Set Flash to Half Auto Wait State
//    - NVMCTRLA.CTRLA.RWS = 0x1 (Half Auto Wait State)
//
// Enable GCLK APB
//    - MCLK.APBAMASK |= GCLK

// Set GCLK5 to 2MHz
//    - SRC = 0x06 (DFLL @ 48MHz)
//    - GENEN = 1
//    - DIV = 24 (divide by 24) => 2MHz
//
// Set DPLL0 (GCLK1) to use GCLK5 as source
//   - CHEN = 1
//   - GEN = 0x5 (GCLK5)
//
// Configure DPLL0 to output 120MHz (60 * 2MHz)
//   - DPLLRATIO0.LDR = 59 (multiply by 60)
//   - DPLLRATIO0.LDRFRAC = 0
//   - DPLLCTRLB.REFCLK = 0 (GCLK5)
//   - DPLLCTRLA.ENABLE = 1
//   - DPLLCTRLA.ONDEMAND = 0
// Set GCLK0 to use DPLL0
//   - SRC = 0x07 (DPLL0)
//   - DIV = 1
//   - OE = 1
//   - GENEN = 1
//
//  Set MCLK CPU Divider to 1 (120MHz)
//   - MCLK.CPUDIV = 1 (Divide by 1)

#[entry]
fn main() -> ! {
    // Enable LED Output
    PA23.set_output(true);

    // Set PinMux to 0x2 (C)
    PORTB.with_pmux(8, |r| r.set_pmuxe(0x2).set_pmuxo(0x2));
    // Enable PinMux
    PORTB.with_pincfg(16, |r| r.set_pmuxen(1));
    PORTB.with_pincfg(17, |r| r.set_pmuxen(1));

    gclk::GCLK.write_pchctrl(35, gclk::Pchctrl(0)
        .set_gen(0x0)
        .set_chen(1)
    );

    SERCOM5.gate_enable();

    let cfg = samd51::ext::sercom::Config::default()
        .set_mode(samd51::ext::sercom::Mode::UsartInternal)
        .set_baud_clock(115_200, 48_000_000)
        .set_rxpo(0x1)
        .set_txpo(0x0);
    SERCOM5.configure(cfg);
    SERCOM5.set_enabled(true);
    Console::set(Console::new(SERCOM5.as_periph(), ConsoleMode::Cooked));

    let mut i = 0u32;
    loop {
        for _ in 0..1_000_000 {
            asm::nop();
        }
        PA23.toggle_output();
        println!("Hello, World {}", i);
        i = i.wrapping_add(1);
    }
}
