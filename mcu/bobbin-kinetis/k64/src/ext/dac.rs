pub use periph::dac::*;
pub use bobbin_common::analog::AnalogWrite;
use bobbin_common::bits::*;

impl DacPeriph {
    pub fn write_u12<V: Into<U12>>(&self, data: V) -> &Self {
        let v: u16 = data.into().value();
        self
            .set_datsl(0, |r| r.set_data0(v as u8))
            .set_datsh(0, |r| r.set_data1((v >> 8) as u8));
        self
    }
    pub fn write_u8_left<V: Into<U8>>(&self, data: V) -> &Self {
        let v: u8 = data.into().value();
        self
            .set_datsl(0, |r| r.set_data0(v << 4))
            .set_datsh(0, |r| r.set_data1(v >> 4));
        self
    }    
    pub fn write_u8_right<V: Into<U8>>(&self, data: V) -> &Self {
        let v: u8 = data.into().value();
        self
            .set_datsl(0, |r| r.set_data0(v))
            .set_datsh(0, |r| r.set_data1(0));
        self
    }    
}

impl AnalogWrite<U12> for DacPeriph {
    fn analog_write(&self, data: U12) -> &Self {
        self.write_u12(data)
    }
}

impl AnalogWrite<U8> for DacPeriph {
    fn analog_write(&self, data: U8) -> &Self {
        self.write_u8_left(data)
    }
}