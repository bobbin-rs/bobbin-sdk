pub use common::analog::AnalogRead;
use hal::adc::*;
use hal::pcc::*;
use hal::port::*;
use hal::gpio::*;

// pub const POT0: Ptc14 = PTC14;
pub const POT0: Adc0Ch12 = ADC0_CH12;
pub const POT0_ADC: Adc0 = ADC0;
pub const POT0_PT: Ptc14 = PTC14;
pub const POT0_PIN: Pc14 = PC14;

pub struct Pot0 {}

pub fn init() {
    PORTC.pcc_set_enabled(true);
    POT0_PT.mode_adc(&POT0);
    POT0_ADC
        .pcc_set_clock_source(ClockSource::SPLLDIV2)
        .pcc_set_enabled(true);        
}