pub use periph::dac::*;
use bobbin_common::bits::*;

impl DacPeriph {
    pub fn set_data_12<V1: Into<U12>, V2: Into<U12>>(&self, data_1: V1, data_2: V2) -> &Self {
        self.set_dhr12rd(|r| r.set_daccdhr(0, data_1).set_daccdhr(1, data_2))
    }

    pub fn set_data_8<V1: Into<U8>, V2: Into<U8>>(&self, data_1: V1, data_2: V2) -> &Self {
        self.set_dhr8rd(|r| r.set_daccdhr(0, data_1).set_daccdhr(1, data_2))
    }
}

impl DacCh {
    pub fn set_data_12<V: Into<U12>>(&self, data: V) -> &Self {
        self.periph.set_dhr12r(self.index, |r| r.set_daccdhr(data));
        self
    }
    pub fn set_data_8<V: Into<U8>>(&self, data: V) -> &Self {
        self.periph.set_dhr8r(self.index, |r| r.set_daccdhr(data));
        self
    }    
}