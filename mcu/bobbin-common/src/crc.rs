pub const CRC_32: u32 = 0x04C11DB7;
pub const CRC_32_START: u32 = 0xFFFFFFFF;
pub const CRC_16: u16 = 0x8005;
pub const CRC_16_START: u16 = 0x0000;

pub trait CrcMode16 {
    fn mode_16(&self) -> &Self;
}

pub trait CrcMode32 {
    fn mode_32(&self) -> &Self;
}

pub trait CrcRead<T> {
    fn read(&self) -> T;
}

pub trait CrcWrite<T> {
    fn write(&self, value: T) -> &Self;
}


pub trait CrcPoly<T> {
    fn poly(&self) -> T;
    fn set_poly(&self, value: T) -> &Self;
}

pub trait CrcInit<T> {
    fn init(&self, value: T) -> &Self;
}