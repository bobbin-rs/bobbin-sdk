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

impl CrcReadWrite<u8> for CrcPeriph {
    fn read(&self) -> u8 {
        self.data().0 as u8
    }
    fn write(&self, value: u8) -> &Self {
        self.set_data(|_| Data(value as u32))
    }
}


impl CrcReadWrite<u16> for CrcPeriph {
    fn read(&self) -> u16 {
        self.data().0 as u16
    }
    fn write(&self, value: u16) -> &Self {
        self.set_data(|_| Data(value as u32))
    }
}

impl CrcReadWrite<u32> for CrcPeriph  {
    fn read(&self) -> u32 {
        self.data().0 as u32
    }
    fn write(&self, value: u32) -> &Self {
        self.set_data(|_| Data(value))
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