#![no_std]
#![no_main]

extern crate nucleo_f746zg as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::spi::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();

    let spi = SPI1;

    let spi_sck = PA5;  // D13
    let spi_miso = PA6; // D12
    let spi_mosi = PA7; // D11
    let spi_nss = PD14; // D10

    spi.gate_enable();
    spi_sck.port().gate_enable();
    spi_miso.port().gate_enable();
    spi_mosi.port().gate_enable();
    spi_nss.port().gate_enable();    

    spi_sck.connect_to(spi);
    spi_miso.connect_to(spi);
    spi_mosi.connect_to(spi);

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_sck.speed_high().push_pull();
    spi_miso.speed_high().pull_up();
    spi_mosi.speed_high().push_pull();
    spi_nss.mode_output().set_output(true).set_output(false).set_output(true);

    spi.set_config(|cfg| cfg
        .set_data_size(DataSize::Bits8)
        .set_master(true)
        .set_baud_divider(0b011.into())
    );
    spi.with_cr1(|r| r.set_cpha(0).set_cpol(0));
    spi.with_cr2(|r| r.set_frxth(1));
    spi.set_output_enabled(true).set_enabled(true);


    let nss_drv = PinDriver::new(spi_nss.into());
    let spi_drv = SpiDriver::new(spi.into());
    let mut app = examples::mb85rs::Example::new(brd.console(), spi_drv, nss_drv);
    let _ = app.run();
   
    loop {}

}

pub struct PinDriver { pin: GpioPin }
impl PinDriver {
    pub fn new(pin: GpioPin) -> Self {
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

// pub trait Transfer<W> {
//     type Error;
//     fn transfer<'w>(
//         &mut self, 
//         words: &'w mut [W]
//     ) -> Result<&'w [W], Self::Error>;
// }

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