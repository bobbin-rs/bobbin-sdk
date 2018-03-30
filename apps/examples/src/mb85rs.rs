use embedded_hal::blocking::i2c;

use core::fmt;

pub const ADDR: u8 = 0b1010_000;

pub struct Example<OUT, I2C> { out: OUT, fram: Mb85rs64<I2C> }

impl<OUT, I2C> Example<OUT, I2C>
where 
    OUT: fmt::Write,
    I2C: i2c::WriteRead,
{
    pub fn new(out: OUT, i2c: I2C) -> Self {
        Self { out, fram: Mb85rs64::new(i2c, ADDR) }
    }

    pub fn run(&mut self) -> Result<(), I2C::Error> {
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

pub const RESERVED_SLAVE_ID: u8 = 0x7c;

pub struct Mb85rs64<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C> Mb85rs64<I2C> 
where 
    I2C: i2c::WriteRead,
{
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr}
    }

    pub fn device_id(&mut self, buf: &mut [u8]) -> Result<(), I2C::Error> {
        self.i2c.write_read(RESERVED_SLAVE_ID, &[self.addr << 1], buf)
    }

    pub fn write(&mut self, addr: u16, buf: &[u8])  -> Result<(), I2C::Error> {
        self.i2c.write_read(self.addr, &[(addr >> 8) as u8, addr as u8], &mut[])?;
        self.i2c.write_read(self.addr, buf, &mut[])
    }

    pub fn read(&mut self, addr: u16, buf: &mut [u8])  -> Result<(), I2C::Error> {
        self.i2c.write_read(self.addr, &[(addr >> 8) as u8, addr as u8], buf)
    }
}