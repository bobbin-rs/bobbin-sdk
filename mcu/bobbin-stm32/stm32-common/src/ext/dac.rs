pub use periph::dac::*;
pub use bobbin_hal::analog::AnalogWrite;
pub use bobbin_common::enabled::Enabled;
use bobbin_common::bits::*;

impl DacPeriph {
    pub fn set_data_u12<V1: Into<U12>, V2: Into<U12>>(&self, data_1: V1, data_2: V2) -> &Self {
        self.set_dhr12rd(|r| r.set_daccdhr(0, data_1).set_daccdhr(1, data_2))
    }

    pub fn set_data_u8<V1: Into<U8>, V2: Into<U8>>(&self, data_1: V1, data_2: V2) -> &Self {
        self.set_dhr8rd(|r| r.set_daccdhr(0, data_1).set_daccdhr(1, data_2))
    }
}


impl Enabled for DacCh {
    fn enabled(&self) -> bool {
        self.periph.cr().test_en(self.index)
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.periph.with_cr(|r| r.set_en(self.index, value));
        self
    }
}

impl DacCh {
    pub fn set_data_u12<V: Into<U12>>(&self, data: V) -> &Self {
        self.periph.set_dhr12r(self.index, |r| r.set_daccdhr(data));
        self
    }
    pub fn set_data_u8<V: Into<U8>>(&self, data: V) -> &Self {
        self.periph.set_dhr8r(self.index, |r| r.set_daccdhr(data));
        self
    }    
}

impl AnalogWrite<U12> for DacCh {
    fn analog_write(&self, data: U12) -> &Self {
        self.set_data_u12(data)
    }
}

impl AnalogWrite<U8> for DacCh {
    fn analog_write(&self, data: U8) -> &Self {
        self.set_data_u8(data)
    }
}

impl AnalogWrite<u8> for DacCh {
    fn analog_write(&self, data: u8) -> &Self {
        self.set_data_u8(data)
    }
}