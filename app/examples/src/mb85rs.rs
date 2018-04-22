//! This example shows usage of the MB85RS FRAM device using SPI and the embedded-hal API.
use embedded_hal::blocking::spi;
use embedded_hal::digital;
use core::fmt;

pub struct Example<OUT, SPI, NSS> { out: OUT, fram: Mb85rs<SPI, NSS> }

impl<OUT, SPI, NSS, E> Example<OUT, SPI, NSS>
where 
    OUT: fmt::Write,
    SPI: spi::Transfer<u8, Error=E> + spi::Write<u8, Error=E>,
    NSS: digital::OutputPin,    
{
    pub fn new(out: OUT, spi: SPI, nss: NSS) -> Self {
        Self { out, fram: Mb85rs::new(spi, nss) }
    }

    pub fn run(&mut self) -> Result<(), E> {
        let mut buf = [0u8; 3];
        self.fram.device_id(&mut buf)?;
        let _ = write!(self.out, "ID: ");
        for i in 0..buf.len() {
            let _ = write!(self.out, "{:02x} ", buf[i]);
        }
        let _ = writeln!(self.out, "");

        {
            let mut out_buf = [0u8; 0x40];
            let mut in_buf = [0u8; 0x40];
            for i in 0..0x40 {
                out_buf[i] = i as u8;
            }    
            self.fram.write(0x00, &out_buf)?;
            self.fram.read(0x00, &mut in_buf)?;
            let _ = self.dump(&in_buf);
        }

        {
            let out_buf = [0u8; 0x40];
            let mut in_buf = [0u8; 0x40];
            self.fram.write(0x00, &out_buf)?;
            self.fram.read(0x00, &mut in_buf)?;
            let _ = self.dump(&in_buf);
        }    
        Ok(())
    }

    pub fn dump(&mut self, buf: &[u8]) -> fmt::Result {
        for i in 0..buf.len() {
            if i % 16 == 0 {
                if i > 0 {
                    writeln!(self.out, "")?;
                }
                write!(self.out, "{:04x}:", i)?;
            }
            if i % 8 == 0 {
                write!(self.out, " ")?;
            }
            write!(self.out, " {:02x}", buf[i])?;
        }
        writeln!(self.out, "")?;
        Ok(())
    }
}


pub const WREN: u8 = 0b0000_0110;
pub const WRDI: u8 = 0b0000_0100;
pub const RDSR: u8 = 0b0000_0101;
pub const WRSR: u8 = 0b0000_0001;
pub const READ: u8 = 0b0000_0011;
pub const WRITE: u8 = 0b0000_0010;
pub const RDID: u8 = 0b1001_1111;

pub struct Mb85rs<SPI, NSS> {
    spi: SPI,
    nss: NSS,
}

impl<SPI, NSS, E> Mb85rs<SPI, NSS> 
where 
    SPI: spi::Transfer<u8, Error=E> + spi::Write<u8, Error=E>,
    NSS: digital::OutputPin,    
{
    pub fn new(spi: SPI, nss: NSS) -> Self {
        Self { spi, nss }
    }

    pub fn device_id(&mut self, buf: &mut [u8]) -> Result<(), E> {
        self.nss.set_low();
        self.spi.write(&[RDID])?;
        self.spi.transfer(buf)?;
        self.nss.set_high();
        Ok(())
    }

    pub fn write(&mut self, addr: u16, buf: &[u8])  -> Result<(), E> {
        self.nss.set_low();
        self.spi.write(&[WREN])?;
        self.nss.set_high();
        self.nss.set_low();
        self.spi.write(&[WRITE, (addr >> 8) as u8, addr as u8])?;
        self.spi.write(buf)?;
        self.nss.set_high();
        Ok(())    
    }

    pub fn read(&mut self, addr: u16, buf: &mut [u8])  -> Result<(), E> {
        self.nss.set_low();
        self.spi.write(&[READ, (addr >> 8) as u8, addr as u8])?;
        self.spi.transfer(buf)?;
        self.nss.set_high();
        Ok(())    
    }
}