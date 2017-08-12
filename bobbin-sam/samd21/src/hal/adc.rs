pub use bobbin_common::analog::AnalogRead;
pub use chip::adc::*;
pub use super::pm::PmEnabled;

pub use ::chip::adc::*;
use ::hal::gclk;

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

impl<P, T> AnalogRead<u16> for Channel<P, T> {
    fn analog_read(&self) -> u16 {
        read(self.index() as u8)
    }
}