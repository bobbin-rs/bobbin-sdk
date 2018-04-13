#![no_std]
#![no_main]

extern crate frdm_k64f as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::gpio::*;
use board::mcu::spi::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();

    let spi = SPI0;

    let spi_sck = PTD1;  // D13
    let spi_miso = PTD3; // D12
    let spi_mosi = PTD2; // D11
    let spi_nss_pin = PTD0; // D10
    let spi_nss = PD0;

    spi.gate_enable();
    spi_sck.port().gate_enable();
    spi_miso.port().gate_enable();
    spi_mosi.port().gate_enable();
    spi_nss_pin.port().gate_enable();    

    spi_sck.connect_to(spi);
    spi_miso.connect_to(spi);
    spi_mosi.connect_to(spi);
    spi_nss_pin.set_mux_gpio();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_nss.set_dir_output().set_output(false);

    spi.init(0b1000, 0b00);

    let nss_drv = PinDriver::new(spi_nss.into());
    let spi_drv = SpiDriver::new(spi.into());
    let mut app = examples::mb85rs::Example::new(brd.console(), spi_drv, nss_drv);
    let _ = app.run();
   
    loop {}

}

pub struct PinDriver { pin: GpioCh }
impl PinDriver {
    pub fn new(pin: GpioCh) -> Self {
        Self { pin }
    }
}

impl hal::digital::OutputPin for PinDriver {
    fn is_high(&self) -> bool {
        self.pin.output()
    }

    fn is_low(&self) -> bool {
        !self.pin.output()
    }

    fn set_low(&mut self) { self.pin.set_output(false); }
    fn set_high(&mut self) { self.pin.set_output(true); }
}


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
        self.spi.write(words);
        while !self.spi.sr().test_tcf() {}
        self.spi.with_sr(|r| r.set_tcf(1));
        Ok(())
    }
}

impl hal::blocking::spi::Transfer<u8> for SpiDriver {
    type Error = Error;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.spi.transfer(words);
        while !self.spi.sr().test_tcf() {}
        self.spi.with_sr(|r| r.set_tcf(1));
        Ok(words)
    }
}