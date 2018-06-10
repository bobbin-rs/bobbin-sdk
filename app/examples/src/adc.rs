//! This example reads an ADC periodically and outputs the value to the console.

use bobbin_hal::delay::*;
use bobbin_hal::analog::*;
use core::fmt;

use core::marker::PhantomData;

pub struct AdcExample<T, ADC: AnalogRead<T>, DEL: Delay> {
    adc: ADC,
    del: DEL,
    delay_ms: u32,
    _phantom: PhantomData<T>,
}

impl<T, ADC, DEL> AdcExample<T, ADC, DEL>
where
    T: fmt::Display,
    ADC: AnalogRead<T>,
    DEL: Delay
{
    pub fn new(adc: ADC, del: DEL, delay_ms: u32, _value: T) -> Self {
        Self { adc, del, delay_ms, _phantom: PhantomData }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let v: T = self.adc.analog_read();
            println!("!{}", v);
            self.del.delay_ms(self.delay_ms);
        }
    }
}