#![no_std]
#![no_main]

extern crate feather_m0 as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::sercom::*;
// use board::mcu::spi::*;

// SPI = SERCOM1
// SCK = PA17 / SERCOM1_PAD1
// MISO = PA19 / SERCOM1_PAD3
// MOSI = PA16 / SERCOM1_PAD0
// SS = PA18 / SERCOM1_PAD2

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();

    let spi = SERCOM1;

    let spi_sck = PA17;  // D13
    let spi_miso = PA19; // D12
    let spi_mosi = PA16; // D11    
    let spi_nss = PA18;

    spi.gate_enable();
    spi_sck.port().gate_enable();
    spi_miso.port().gate_enable();
    spi_mosi.port().gate_enable();
    spi_nss.port().gate_enable();    

    spi_sck.connect_to(spi);
    spi_miso.connect_to(spi);
    spi_mosi.connect_to(spi);
    spi_nss.set_mode_output().set_output(false);

    spi.init_spi(47, 0x0, 0x3);
    spi.set_enabled(true);
    spi.set_rxen(true);    

    let nss_drv = PinDriver::new(spi_nss.into());
    let spi_drv = SpiDriver::new(spi.into());
    let mut app = examples::mb85rs::Example::new(brd.console(), spi_drv, nss_drv);
    let _ = app.run();
   
    loop {}

}

pub struct PinDriver { pin: PortPin }
impl PinDriver {
    pub fn new(pin: PortPin) -> Self {
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

pub struct SpiDriver { spi: SercomPeriph }

impl SpiDriver {
    pub fn new(spi: SercomPeriph) -> Self {
        Self { spi }
    }
}

impl hal::blocking::spi::Write<u8> for SpiDriver {
    type Error = Error;
    fn write<'w>(&mut self, words: &'w [u8]) -> Result<(), Self::Error> {
        for i in 0..words.len() {
            self.spi.xfer(words[i]);
        }
        Ok(())
    }
}

impl hal::blocking::spi::Transfer<u8> for SpiDriver {
    type Error = Error;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        for i in 0..words.len() {
            words[i] = self.spi.xfer(words[i]);
        }
        Ok(words)
    }
}