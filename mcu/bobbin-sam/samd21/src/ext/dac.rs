pub use periph::dac::*;
pub use bobbin_common::bits::*;
pub use bobbin_common::analog::AnalogWrite;

use gclk;

impl DacPeriph {
    pub fn init(&self) -> &Self {
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Set GCLK_GEN0 as source for ADC
        gclk::GCLK.set_clkctrl(|r| r
            // NOTE - Datasheet claims GCLK_DAC is 0x23
            .set_id(0x21) // GCLK_DAC
            .set_gen(0x0)
            .set_clken(true)
        );    
        self.set_ctrlb(|r| r.set_refsel(0x01).set_eoen(1));
        self.wait_busy();        
        self
    }

    pub fn enable(&self) -> &Self {
        self.set_ctrla(|r| r.set_enable(1));
        self.wait_busy();
        self
    }

    pub fn wait_busy(&self) {
        while self.status().syncbusy() != 0 {}
    }

    pub fn write_u10<V: Into<U10>>(&self, data: V) -> &Self {        
        self.set_data(|r| r.set_data(data.into().value()));
        self
    }
}

impl AnalogWrite<U10> for DacPeriph {
    fn analog_write(&self, value: U10) -> &Self {
        self.write_u10(value)
    }
}

impl AnalogWrite<U8> for DacPeriph {
    fn analog_write(&self, data: U8) -> &Self {
        self.analog_write(data.value())
    }
}

impl AnalogWrite<u8> for DacPeriph {
    fn analog_write(&self, data: u8) -> &Self {
        self.write_u10((data as u16) << 2)
    }
}