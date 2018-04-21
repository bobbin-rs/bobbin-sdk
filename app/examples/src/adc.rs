use bobbin_hal::delay::*;
use bobbin_hal::analog::*;
use core::fmt;

use core::marker::PhantomData;

pub struct AdcExample<T, OUT: fmt::Write, ADC: AnalogRead<T>, DEL: Delay> {
    out: OUT,
    adc: ADC,
    del: DEL,
    delay_ms: u32,
    _phantom: PhantomData<T>,
}

impl<T, OUT, ADC, DEL> AdcExample<T, OUT, ADC, DEL> 
where
    T: fmt::Display,
    OUT: fmt::Write,
    ADC: AnalogRead<T>,
    DEL: Delay
{
    pub fn new(out: OUT, adc: ADC, del: DEL, delay_ms: u32, _value: T) -> Self {
        Self { out, adc, del, delay_ms, _phantom: PhantomData }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let v: T = self.adc.analog_read();
            let _ = writeln!(self.out, "{}", v);
            self.del.delay_ms(self.delay_ms);
        }
    }
}