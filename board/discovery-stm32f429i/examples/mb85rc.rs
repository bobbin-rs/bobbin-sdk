#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f429i as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::i2c::*;
use board::prelude::*;

// A5 = PB6 = I2C1_SCL
// A4 = PB7 = I2C1_SDA

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|brd| {
        println!("Running I2C");

        let i2c = I2C3;
        let i2c_scl = PA8; // D15
        let i2c_sda = PC9; // D14
        let int = PA15;

        i2c.gate_enable();
        i2c_scl.port().gate_enable();
        i2c_sda.port().gate_enable();
        int.port().gate_enable();

        i2c_scl.connect_to(i2c);
        i2c_sda.connect_to(i2c);

        i2c_scl.open_drain();
        i2c_sda.open_drain();

        println!("# Configuring I2C");

        // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
        i2c.set_enabled(false);
        i2c.set_timing(|r| r
            .set_presc(0x0)
            .set_scldel(0x3)
            .set_sdadel(0x0)
            .set_sclh(0xF)
            .set_scll(0x12)
        );

        loop{}
    })
}