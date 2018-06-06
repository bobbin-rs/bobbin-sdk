#![feature(never_type)]
#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f429i as board;
extern crate embedded_hal as hal;

use hal::prelude::*;

use board::mcu::pin::*;
use board::mcu::spi::*;
use board::prelude::*;
use board::bobbin_hal::delay::Delay;
use board::mcu::ext::spi_v1::FrameSize;

// A5 = PB6 = I2C1_SCL
// A4 = PB7 = I2C1_SDA

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> Result<!, Error> {
    board::init().run(|sys| {
        println!("Running L3GD20 SPI");

        let spi = SPI5;
        let spi_miso = PF8;
        let spi_mosi = PF9;
        let spi_sck  = PF7;
        let spi_nss  = PC1;

        spi.gate_enable();
        spi_miso.port().gate_enable();
        spi_mosi.port().gate_enable();
        spi_sck.port().gate_enable();
        spi_nss.port().gate_enable();

        spi_miso.connect_to(spi);
        spi_mosi.connect_to(spi);
        spi_sck.connect_to(spi);

        spi_sck.speed_high().push_pull();
        spi_miso.speed_high().pull_up();
        spi_mosi.speed_high().push_pull();
        spi_nss.mode_output()
            .set_output(true)
            .set_output(false)
            .set_output(true);

        spi.set_config(|cfg| cfg
            .set_frame_size(FrameSize::Bits8)
            .set_master(true)
            .set_cpha(false)
            .set_cpol(false)
            .set_baud_divider(0b011.into())
        );

        spi.set_output_enabled(true).set_enabled(true);

        println!("SPI Configuration Complete");

        let nss_drv = PinDriver::new(spi_nss.into());
        let spi_drv = SpiDriver::new(spi.into());

        let mut dev = L3gd20::new(spi_drv, nss_drv);

        println!("ID:   0x{:02x}", dev.id()?);
        println!("TEMP: 0x{:02x}", dev.temp()?);

        /* Reset then switch to normal mode and enable all three channels */
        dev.cmd_reset()?;
        dev.cmd_mode_normal()?;

        let mut n = 0;
        loop {
            if n == 0 {
                println!("{:>6} {:>6} {:>6}","X","Y","Z");
            }

            // println!("STATUS: {:02x}", dev.reg_read(0x27));
            {
                // let (xl, xh, yl, yh, zl, zh) = (
                //     dev.reg_read(0x28),
                //     dev.reg_read(0x29),
                //     dev.reg_read(0x2a),
                //     dev.reg_read(0x2b),
                //     dev.reg_read(0x2c),
                //     dev.reg_read(0x2d),
                // );
                // let x = (((xh as u16) << 8) | (xl as u16)) as i16;
                // let y = (((yh as u16) << 8) | (yl as u16)) as i16;
                // let z = (((zh as u16) << 8) | (zl as u16)) as i16;
                let (x, y, z) = dev.xyz()?;

                println!("{:6} {:6} {:6}", x, y, z);
            }
            sys.tick().delay_ms(500);
            n = if n == 10 { 0 } else { n + 1};
        }
    })
}

pub struct L3gd20 {
    spi: SpiDriver,
    nss: PinDriver,
}

impl L3gd20 {
    pub fn new(spi: SpiDriver, nss: PinDriver) -> Self {
        Self { spi, nss }
    }

    pub fn reg_write(&mut self, reg: u8, val: u8) -> Result<(), Error> {
        let cmd = [reg, val];
        let mut buf = [0u8, 0u8];
        self.transfer(&cmd, &mut buf)
    }

    pub fn reg_read(&mut self, reg: u8) -> Result<u8, Error> {
        let cmd = [reg | 1 << 7, 0x00];
        let mut buf = [0u8, 0u8];
        self.transfer(&cmd, &mut buf)?;
        Ok(buf[1])
    }

    pub fn transfer(&mut self, src: &[u8], dst: &mut[u8]) -> Result<(), Error> {
        self.nss.set_low();
        self.spi.write(src)?;
        self.spi.transfer(dst)?;
        self.nss.set_high();
        Ok(())
    }

    pub fn cmd_reset(&mut self) -> Result<(), Error> {
        self.reg_write(0x20, 0x00)
    }

    pub fn cmd_mode_normal(&mut self) -> Result<(), Error> {
        self.reg_write(0x20, 0x0f)
    }

    pub fn id(&mut self) -> Result<u8, Error>  {
        self.reg_read(0x0f)
    }

    pub fn temp(&mut self) -> Result<u8, Error> {
        self.reg_read(0x26)
    }

    pub fn xyz(&mut self) -> Result<(i16, i16, i16), Error> {
        Ok((self.x()?, self.y()?, self.z()?))
    }

    pub fn x(&mut self) -> Result<i16, Error> {
        Ok((self.xl()? as u16 | (self.xh()? as u16) << 8) as i16)
    }

    pub fn y(&mut self) -> Result<i16, Error> {
        Ok((self.yl()? as u16 | (self.yh()? as u16) << 8) as i16)
    }

    pub fn z(&mut self) -> Result<i16, Error> {
        Ok((self.zl()? as u16 | (self.zh()? as u16) << 8) as i16)
    }


    pub fn xl(&mut self) -> Result<u8, Error> {
        self.reg_read(0x28)
    }

    pub fn xh(&mut self) -> Result<u8, Error> {
        self.reg_read(0x29)
    }

    pub fn yl(&mut self) -> Result<u8, Error> {
        self.reg_read(0x2a)
    }

    pub fn yh(&mut self) -> Result<u8, Error> {
        self.reg_read(0x2b)
    }

    pub fn zl(&mut self) -> Result<u8, Error> {
        self.reg_read(0x2c)
    }

    pub fn zh(&mut self) -> Result<u8, Error> {
        self.reg_read(0x2d)
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