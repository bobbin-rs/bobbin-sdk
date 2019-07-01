#![no_std]
#![no_main]

use cortex_m_rt::entry;
use bobbin_samd51_example::*;
use mcu::bobbin_hal::prelude::*;
use mcu::bobbin_mcu::prelude::*;
use mcu::bobbin_hal::timer::Delay;
use mcu::tc::*;
use mcu::gclk::*;
use mcu::ext::tc::*;


#[entry]
fn main() -> ! {
    init();

    // Enable peripheral gate
    TC0.gate_enable();

    // Enable generic clock using ClockGen 5
    GCLK.set_pchctrl(0x9, |r| r.set_chen(1).set_gen(5));

    // Configure timer to use prescaler, divide by 1024
    // Timer base is 1953.125Hz

    let cfg = Config {
        wavegen: Wavegen::NPWM,
        prescaler: Prescaler::Div1024,
        runstdby: false,
        prescsync: Prescsync::PRESC,
    };
    TC0.configure_16bit(cfg);
    let mut i = 0u32;
    loop {
        LED0.toggle_output();
        println!("Hello, World {}", i);
        i = i.wrapping_add(1);
        TC0.delay(1953);
    }
}
