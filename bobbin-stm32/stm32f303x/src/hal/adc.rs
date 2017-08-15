pub use chip::adc::*;
pub use chip::c_adc;
pub use super::rcc::RccEnabled;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Resolution {
    Bits12 = 0b00,
    Bits10 = 0b01,
    Bits8 =  0b10,
    Bits6 =  0b11,
}

pub trait AdcExt {
    fn init(&self) -> &Self;
    fn data(&self) -> u16;
    fn start(&self) -> &Self;
    fn stop(&self) -> &Self;
    fn status(&self) -> Isr;
    fn end_of_sequence(&self) -> bool;
    fn end_of_conversion(&self) -> bool;
    fn set_continuous(&self, value: bool) -> &Self;
    fn set_resolution(&self, value: Resolution) -> &Self;
    fn sequence_length(&self) -> usize;
    fn set_sequence_length(&self, length: usize) -> &Self;
    fn set_sequence_channel(&self, sequence: usize, channel: usize) -> &Self;
    fn set_sequence(&self, channels: &[usize]) -> &Self;
    fn read_sequence(&self, out: &mut[u16]) -> &Self;
}

impl<T> AdcExt for Periph<T> {
    // fn c_adc(&self) -> CAdc {
    //     match self.adc {
    //         ADC1 => C_ADC12,
    //         ADC2 => C_ADC12,
    //         ADC3 => C_ADC34,
    //         ADC4 => C_ADC34,
    //         _ => unimplemented!()
    //     }
    // }        
    fn init(&self) -> &Self {
        
        self.with_cr(|r| r.set_aden(0));
        while self.isr().adrdy() != 0 {}

        // Enable Analog Voltage Regulator
        self.with_cr(|r| r.set_advregen(0b00));
        self.with_cr(|r| r.set_advregen(0b01));
        for _ in 0..10000 { 
            unsafe { asm!("nop") }
        }

        // Use HCLK/1 Clock Mode
        c_adc::C_ADC12.with_ccr(|r| r.set_ckmode(0b01));

        // Calibrate
        self.with_cr(|r| r.set_adcaldif(0));
        self.with_cr(|r| r.set_adcal(1));
        while self.cr().adcal() != 0 {}

        // Enable ADC
        self.with_cr(|r| r.set_aden(1));

        // Wait until ADC is ready
        let mut n = 10_000_000;
        while self.isr().adrdy() == 0 {
            if n == 0 {
                panic!("ADC Timeout - CR {:?} CFGR {:?} ISR {:?}", self.cr(), self.cfgr(), self.isr());
            }
            n -= 1;
        }
        self
    }

    fn data(&self) -> u16 {
        self.dr().regulardata().value()
    }

    fn start(&self) -> &Self {
        self.with_cr(|r| r.set_adstart(1))
    }

    fn stop(&self) -> &Self {
        self.with_cr(|r| r.set_adstp(1))
    }    

    fn status(&self) -> Isr {
        self.isr()
    }

    fn end_of_sequence(&self) -> bool {
        self.isr().eos() != 0
    }    

    fn end_of_conversion(&self) -> bool {
        self.isr().eoc() != 0
    }

    fn set_continuous(&self, value: bool) -> &Self {
        self.with_cfgr(|r| r.set_cont(value))
    }

    fn set_resolution(&self, value: Resolution) -> &Self {
        self.with_cfgr(|r| r.set_res(value as u32))
    }    

    fn sequence_length(&self) -> usize {
        self.sqr1().l().into_usize() + 1
    }

    fn set_sequence_length(&self, length: usize) -> &Self {
        assert!(length > 0 && length <= 16, "length must be 1..16");
        self.with_sqr1(|r| r.set_l((length - 1) as u32))
    }

    fn set_sequence_channel(&self, sequence: usize, channel: usize) -> &Self {
        assert!(sequence > 0 && sequence <= 16, "Sequence must be 1..16");
        assert!(channel > 0 && channel <= 18, "Channel must be 1..18");
        let channel = channel as u32;
        match sequence {
            1 => self.with_sqr1(|r| r.set_sq1(channel)),
            2 => self.with_sqr1(|r| r.set_sq2(channel)),
            3 => self.with_sqr1(|r| r.set_sq3(channel)),
            4 => self.with_sqr1(|r| r.set_sq4(channel)),
            5 => self.with_sqr2(|r| r.set_sq5(channel)),
            6 => self.with_sqr2(|r| r.set_sq6(channel)),
            7 => self.with_sqr2(|r| r.set_sq7(channel)),
            8 => self.with_sqr2(|r| r.set_sq8(channel)),
            9 => self.with_sqr2(|r| r.set_sq9(channel)),
            10 => self.with_sqr3(|r| r.set_sq10(channel)),
            11 => self.with_sqr3(|r| r.set_sq11(channel)),
            12 => self.with_sqr3(|r| r.set_sq12(channel)),
            13 => self.with_sqr3(|r| r.set_sq13(channel)),
            14 => self.with_sqr3(|r| r.set_sq14(channel)),
            15 => self.with_sqr4(|r| r.set_sq15(channel)),
            16 => self.with_sqr4(|r| r.set_sq16(channel)),
            _ => unimplemented!()
        }
    }

    fn set_sequence(&self, channels: &[usize]) -> &Self {
        for (i, c) in channels.iter().enumerate() {
            self.set_sequence_channel(i + 1, *c);
        }
        self.set_sequence_length(channels.len())        
    }

    fn read_sequence(&self, out: &mut [u16]) -> &Self {
        assert!(out.len() == self.sequence_length());
        self.start();
        for c in out.iter_mut() {
            while !self.end_of_conversion() {}
            *c = self.data();
        }
        self
    }
}

pub trait AdcChExt {
    fn read(&self) -> u16;
}

impl<P, T> AdcChExt for Channel<P, T> {
    fn read(&self) -> u16 {
        let mut out = [0u16];
        self.periph()
            .set_sequence(&[self.index()])
            .read_sequence(&mut out);
        out[0]            
    }
}