use bobbin_hal::delay::*;
use bobbin_hal::analog::*;

pub struct DacExample<DAC: AnalogWrite<u8>, DEL: Delay> {
    dac: DAC,
    del: DEL,
    delay_ms: u32,
}

impl<DAC, DEL> DacExample<DAC, DEL> 
where
    DAC: AnalogWrite<u8>,
    DEL: Delay
{
    pub fn new(dac: DAC, del: DEL, delay_ms: u32) -> Self {
        Self { dac, del, delay_ms }
    }

    pub fn run(&mut self) -> ! {
        let mut v: u8 = 16;
        let s: u8 = 4;
        let mut d: bool = true;
        loop {
            self.dac.analog_write(v);
            if d {
                v += s;
                if v == 240 {
                    d = !d;
                }
                
            } else {
                v -= s;
                if v == 16 {
                    d = !d;
                }
            }
            self.del.delay_ms(self.delay_ms);
        }        
    }
}