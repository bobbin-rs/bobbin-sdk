#![feature(never_type)]
#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f429i as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::spi::*;
use board::prelude::*;
use examples::lcd::{Ili9341, Error as LcdError, Orientation};
use board::bobbin_hal::delay::Delay;
use board::mcu::ext::spi_v1::FrameSize;

use core::iter::repeat;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let err = run();
    println!("!!!{:?}", err);

    loop {}
}


fn run() -> Result<!, LcdError<Error>> {
    board::init().run(|brd| {
        println!("Start!");
        let lcd_csx = PC2;
        let lcd_dcx = PD13;
        let lcd_scl = PF7;
        let lcd_miso = PF8;
        let lcd_mosi = PF9;
        let lcd_reset = PA3;
        let lcd_spi = SPI5;

        lcd_spi.gate_enable();
        lcd_csx.port().gate_enable();
        lcd_dcx.port().gate_enable();
        lcd_scl.port().gate_enable();
        lcd_miso.port().gate_enable();
        lcd_mosi.port().gate_enable();
        lcd_reset.port().gate_enable();

        lcd_csx.mode_output()
            .set_output(true)
            .set_output(false)
            .set_output(true);

        lcd_dcx.mode_output();
        lcd_reset.mode_output();

        lcd_scl.connect_to(lcd_spi);
        lcd_miso.connect_to(lcd_spi);
        lcd_mosi.connect_to(lcd_spi);

        lcd_spi.set_config(|cfg| cfg
            .set_frame_size(FrameSize::Bits8)
            .set_master(true)
            .set_cpha(false)
            .set_cpol(false)
            .set_msb_first()
            .set_baud_divider(1.into())
        );

        lcd_spi.set_output_enabled(true)
               .set_enabled(true);

        let mut delay = DelayTimer::new(&brd);
        let ldc_cs_drv = PinDriver::new(lcd_csx.into());
        let ldc_ds_drv = PinDriver::new(lcd_dcx.into());
        let ldc_reset_drv = PinDriver::new(lcd_reset.into());
        let ldc_spi_drv = SpiDriver::new(lcd_spi.into());

        let mut lcd = Ili9341::new(ldc_spi_drv, ldc_cs_drv, ldc_ds_drv, ldc_reset_drv, &mut delay)?;

        lcd.set_orientation(Orientation::Landscape)?;

        loop {
            lcd.draw_iter(0, 0, 320, 240, repeat(0xFF00).take(240 * 320))?;
            brd.tick().delay_ms(500);

            lcd.draw_iter(0, 0, 320, 240, repeat(0xF0F0).take(240 * 320))?;
            brd.tick().delay_ms(500);
        }
    })
}

pub struct DelayTimer<'a>(&'a System<board::Board>);

impl<'a> DelayTimer<'a> {
    pub fn new(brd: &'a System<board::Board>) -> Self { DelayTimer(brd) }
}

impl<'a> hal::blocking::delay::DelayMs<u16> for DelayTimer<'a> {
    fn delay_ms(&mut self, ms: u16) {
        self.0.tick().delay_ms(ms.into());
    }
}

pub struct PinDriver { pin: GpioPin }
impl PinDriver {
    pub fn new(pin: GpioPin) -> Self {
        Self { pin }
    }
}

impl hal::digital::OutputPin for PinDriver {
    fn set_low(&mut self) { self.pin.set_output(false); }
    fn set_high(&mut self) { self.pin.set_output(true); }
}

#[derive(Debug)]
pub struct Error;

pub struct SpiDriver { spi: SpiPeriph }

impl SpiDriver {
    pub fn new(spi: SpiPeriph) -> Self {
        Self { spi }
    }
}

impl hal::blocking::spi::Write<u8> for SpiDriver {
    type Error = Error;
    fn write<'w>(&mut self, words: &'w [u8]) -> Result<(), Self::Error> {
        self.spi.transfer(words, &mut []);
        Ok(())
    }
}

impl hal::blocking::spi::Transfer<u8> for SpiDriver {
    type Error = Error;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.spi.transfer(&[], words);
        Ok(words)
    }
}
