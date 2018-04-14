use bobbin_common::bits::*;
pub use bobbin_common::hal::analog::AnalogRead;
pub use chip::adc::*;
pub use super::pm::PmEnabled;

pub use ::chip::adc::*;
use ::hal::gclk;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Resolution {
    Bits12 = 0x0,
    Bits16 = 0x1,
    Bits10 = 0x2,
    Bits8 = 0x3,
}

pub fn init() {
    // Enable ADC Bus Clock
    ADC.pm_set_enabled(true);
    
    while gclk::GCLK.status().syncbusy() != 0 {}

    // Set GCLK_GEN0 as source for ADC
    gclk::GCLK.set_clkctrl(|r| r
        .set_id(0x1e) // GCLK_ADC
        .set_gen(0x0)
        .set_clken(true)
    );    

    wait_busy();

    // Set prescaler to Divide_by_512
    // set Resolution 10 bit
    ADC.set_ctrlb(|r| r.set_prescaler(0x7).set_ressel(0x2));

    // Set max sampling time
    ADC.set_sampctrl(|r| r.set_samplen(0x3f));

    wait_busy();

    // No Negative input (Internal Ground)
    ADC.set_inputctrl(|r| r.set_muxneg(0x18));

    // 1 sample only (no oversampling nor averaging)
    // Adjusting result by 0
    ADC.set_avgctrl(|r| r.set_samplenum(0x0).set_adjres(0x0));

    // Set Analog reference
    // 1x gain
    // ADC.with_inputctrl(|r| r.set_gain(0x0));
    // Gain DIV/2
    ADC.with_inputctrl(|r| r.set_gain(0xf));
    // 1/2 VDDANA
    ADC.set_refctrl(|r| r.set_refsel(0x2));
    // Use VREFA
    //ADC.set_refctrl(adc::Refctrl(0).set_refsel(0x3));

    //enable();
}

pub fn wait_busy() {
    while ADC.status().syncbusy() != 0 {}
}

pub fn enable() {
    ADC.with_ctrla(|r| r.set_enable(false));
    ADC.with_ctrla(|r| r.set_swrst(true));
    while ADC.ctrla().swrst() != 0 {}      
    while ADC.status().syncbusy() != 0 {}  
    ADC.with_ctrla(|r| r.set_enable(true));
}

pub fn disable() {
    ADC.with_ctrla(|r| r.set_enable(true));
}

pub fn read(channel: u8) -> u16 {
    // Set input channel
    wait_busy();        
    ADC.with_inputctrl(|r| r.set_muxpos(channel as u32));
            
    // Enable ADC
    wait_busy();
    ADC.with_ctrla(|r| r.set_enable(true));

    

    // Start Conversion
    wait_busy();
    ADC.set_swtrig(|r| r.set_start(true));

    // Clear the Data Ready flag
    ADC.set_intflag(|r| r.set_resrdy(true));

    // Start conversion again, since the first conversion after the reference is changed must not be used.
    wait_busy();
    ADC.set_swtrig(|r| r.set_start(true));

    // Wait for conversion to complete
    while ADC.intflag().resrdy() == 0 {}

    let result = ADC.result();

    // Disable ADC
    wait_busy();
    ADC.with_ctrla(|r| r.set_enable(false));

    result.result().value()
}

impl AdcPeriph {
    pub fn wait_busy(&self) -> &Self {
        while self.status().syncbusy() != 0 {}
        self
    }
    pub fn enabled(&self) -> bool {
        self.ctrla().enable() != 0
    }
    pub fn set_enabled(&self, value: bool) -> &Self {
        self.with_ctrla(|r| r.set_enable(value))
    }
    pub fn resolution(&self) -> Resolution {
        match self.ctrlb().ressel() {
            U2::B00 => Resolution::Bits12,
            U2::B01 => Resolution::Bits16,
            U2::B10 => Resolution::Bits10,
            U2::B11 => Resolution::Bits8,
        }
    }

    pub fn set_resolution(&self, value: Resolution) -> &Self {
        self.with_ctrlb(|r| r.set_ressel(value as u8))
    }
    pub fn muxpos(&self) -> U5 {
        self.inputctrl().muxpos()
    }
    pub fn set_muxpos(&self, value: U5) -> &Self {
        self.with_inputctrl(|r| r.set_muxpos(value))
    }
    pub fn muxneg(&self) -> U5 {
        self.inputctrl().muxneg()
    }
    pub fn set_muxneg(&self, value: U5) -> &Self {
        self.with_inputctrl(|r| r.set_muxneg(value))
    }
    pub fn result_ready(&self) -> bool {
        self.intflag().resrdy() != 0
    }
    pub fn clr_result_ready(&self) -> &Self {
        self.set_intflag(|r| r.set_resrdy(1))
    }
    pub fn wait_result_ready(&self) -> &Self {
        while !self.result_ready() {}
        self
    }
    pub fn trigger(&self) -> &Self {
        self.set_swtrig(|r| r.set_start(1))
    }
    pub fn result_16(&self) -> U16 {
        self.result().result_16()
    }
    pub fn result_12(&self) -> U12 {
        self.result().result_12()
    }
    pub fn result_10(&self) -> U10 {
        self.result().result_10()
    }
    pub fn result_8(&self) -> U8 {
        self.result().result_8()
    }
}

macro_rules! impl_analog_read {
    ($t:ty, $res:expr, $meth:ident) => (
        impl AnalogRead<$t> for AdcCh {
            fn start(&self) -> &Self {
                // Clear the Data Ready flag
                self.periph
                    .set_enabled(false)
                    .set_resolution($res)
                    .set_muxpos(self.index.into())
                    .set_enabled(true)
                    .clr_result_ready()
                    .trigger();
                self
            }

            fn is_complete(&self) -> bool {
                self.periph.result_ready()
            }

            fn read(&self) -> $t {
                self.periph.$meth()
            }
        }
    )
}

impl_analog_read!(U8, Resolution::Bits8, result_8);
impl_analog_read!(U10, Resolution::Bits10, result_10);
impl_analog_read!(U12, Resolution::Bits12, result_12);
