pub use bobbin_common::crc::*;
pub use bobbin_common::configure::*;
pub use ::chip::crc::*;

impl CrcMode16 for CrcPeriph {
    fn mode_16(&self) -> &Self {
        self.with_ctrl(|r| r.set_tcrc(0))
    }
}

impl CrcMode32 for CrcPeriph {
    fn mode_32(&self) -> &Self {
        self.with_ctrl(|r| r.set_tcrc(1))
    }
}

impl CrcRead<u8> for CrcPeriph {
    fn read(&self) -> u8 {
        self.datall().0
    }
}

impl CrcWrite<u8> for CrcPeriph {
    fn write(&self, value: u8) -> &Self {
        self.set_datall(|_| Datall(value))
    }
}

impl CrcRead<u16> for CrcPeriph {
    fn read(&self) -> u16 {
        let d = self.data();
        d.lu().into_u16() | d.ll().into_u16() << 8
    }
}

impl CrcWrite<u16> for CrcPeriph {
    fn write(&self, value: u16) -> &Self {
        self.set_datal(|_| Datal(value))
    }
}

impl CrcRead<u32> for CrcPeriph  {
    fn read(&self) -> u32 {
        self.data().0 as u32
    }
}

impl CrcWrite<u32> for CrcPeriph  {
    fn write(&self, value: u32) -> &Self {
        self.set_data(|_| Data(value))
    }
}

impl<'a> CrcWrite<&'a [u8]> for CrcPeriph  {
    fn write(&self, value: &[u8]) -> &Self {
        for c in value.iter() {
            self.write(*c);
        }
        self
    }
}

impl CrcPoly<u16> for CrcPeriph  {
    fn poly(&self) -> u16 {
        self.gpoly().low().into()
    }
    fn set_poly(&self, value: u16) -> &Self {
        self.set_gpoly(|r| r.set_low(value))
    }
}

impl CrcPoly<u32> for CrcPeriph  {
    fn poly(&self) -> u32 {
        self.gpoly().0
    }
    fn set_poly(&self, value: u32) -> &Self {
        self.set_gpoly(|_| Gpoly(value))
    }
}

impl CrcInit<u16> for CrcPeriph  {
    fn init(&self, value: u16) -> &Self {
        self
            .with_ctrl(|r| r.set_was(1))
            .write(value)
            .with_ctrl(|r| r.set_was(0))
    }
}

impl CrcInit<u32> for CrcPeriph  {
    fn init(&self, value: u32) -> &Self {
        self
            .with_ctrl(|r| r.set_was(1))
            .write(value)
            .with_ctrl(|r| r.set_was(0))
    }
}