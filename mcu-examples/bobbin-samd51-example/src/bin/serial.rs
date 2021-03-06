#![no_std]
#![no_main]

extern crate panic_abort;
use cortex_m_rt::entry;
use cortex_m::asm;

use samd51::bobbin_mcu::prelude::*;
use samd51::bobbin_hal::prelude::*;
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

    loop {
        for _ in 0..1_000_000 {
            asm::nop();
        }
        PA23.toggle_output();
        SERCOM5.write("Hello, World\n".as_bytes());
    }
}
