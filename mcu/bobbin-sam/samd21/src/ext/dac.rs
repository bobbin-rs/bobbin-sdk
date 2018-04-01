pub use periph::dac::*;
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
}