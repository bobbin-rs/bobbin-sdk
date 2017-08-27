pub use common::analog::AnalogRead;
use common::bits::*;
use hal::adc::*;
use hal::pcc::*;
use hal::port::*;

// pub const POT0: Ptc14 = PTC14;
// pub const POT0_ADC: Adc0 = ADC0;
// pub const POT0_ADC_CH: Adc0Ch12 = ADC0_CH12;

pub const POT0: Pot0 = Pot0 {};

pub struct Pot0 {}

impl Pot0 {
    fn init(&self) -> &Self {
        self.pin().port.pcc_set_enabled(true);
        self.pin().mode_adc(&self.adc_ch());
        self.adc_ch().periph()
            .pcc_set_clock_source(ClockSource::SPLLDIV2)
            .pcc_set_enabled(true);        
        self
    }
    fn pin(&self) -> Ptc14 {
        PTC14
    }
    fn adc_ch(&self) -> Adc0Ch12 {
        ADC0_CH12
    }
}

impl AnalogRead<U8> for Pot0 {
    fn start(&self) -> &Self {
        <Adc0Ch12 as AnalogRead<U8>>::start(&self.adc_ch());
        self
    }
    fn is_complete(&self) -> bool {
        <Adc0Ch12 as AnalogRead<U8>>::is_complete(&self.adc_ch())
    }
    fn read(&self) -> U8 {
        self.adc_ch().read()
    }
}

impl AnalogRead<U10> for Pot0 {
    fn start(&self) -> &Self {
        <Adc0Ch12 as AnalogRead<U10>>::start(&self.adc_ch());
        self
    }
    fn is_complete(&self) -> bool {
        <Adc0Ch12 as AnalogRead<U10>>::is_complete(&self.adc_ch())
    }
    fn read(&self) -> U10 {
        self.adc_ch().read()
    }
}

pub fn init() {
    POT0.init();    
}