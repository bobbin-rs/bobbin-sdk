use bobbin_common::bits::*;
pub use bobbin_common::analog::AnalogRead;
pub use ::chip::adc_l0::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Resolution {
    Bits12 = 0b00,
    Bits10 = 0b01,
    Bits8 =  0b10,
    Bits6 =  0b11,
}

impl AdcPeriph {
    pub fn init(&self) -> &Self {
       
        self.with_cr(|r| r.set_aden(0));
        while self.isr().adrdy() != 0 {}

        // Enable Analog Voltage Regulator
        self.with_cr(|r| r.set_advregen(0b00));
        self.with_cr(|r| r.set_advregen(0b01));
        
        // Calibrate
        self.with_cr(|r| r.set_adcal(1));
        while self.cr().adcal() != 0 {}

        // Enable ADC
        self.with_cr(|r| r.set_aden(1));

        // Wait until ADC is ready
        while self.isr().adrdy() == 0 {}
        self
    }

    pub fn data(&self) -> u16 {
        self.dr().data().value()
    }

    pub fn start(&self) -> &Self {
        self.with_cr(|r| r.set_adstart(1))
    }

    pub fn stop(&self) -> &Self {
        self.with_cr(|r| r.set_adstp(1))
    }    

    pub fn status(&self) -> Isr {
        self.isr()
    }

    pub fn end_of_sequence(&self) -> bool {
        self.isr().eos() != 0
    }    

    pub fn clr_end_of_sequence(&self) -> &Self {
        self.with_isr(|r| r.set_eos(1))
    }

    pub fn end_of_conversion(&self) -> bool {
        self.isr().eoc() != 0
    }

    pub fn clr_end_of_conversion(&self) -> &Self {
        self.with_isr(|r| r.set_eoc(1))
    }

    pub fn set_continuous(&self, value: bool) -> &Self {
        self.with_cfgr1(|r| r.set_cont(value))
    }

    pub fn set_resolution(&self, value: Resolution) -> &Self {
        self.with_cfgr1(|r| r.set_res(value as u32))
    }    

    pub fn set_channel(&self, channel: usize) -> &Self {
        assert!(channel <= 18, "Channel must be 0..18");
        self.with_chselr(|r| r.set_chsel(channel, 1))
    }

    pub fn clr_channel(&self, channel: usize) -> &Self {
        assert!(channel <= 18, "Channel must be 0..18");
        self.with_chselr(|r| r.set_chsel(channel, 0))
    }
    pub fn clr_channels(&self) -> &Self {
        self.set_chselr(|r| r)
    }

    pub fn start_single(&self, channel: usize, value: Resolution) -> &Self {
        self
            .set_resolution(value)
            .clr_channels()
            .set_channel(channel)
            .clr_end_of_conversion()
            .start()
    }    
}

// pub trait AdcChExt {
//     fn start(&self) -> &Self;
//     fn complete(&self) -> bool;
//     fn wait(&self) -> &Self;
//     fn read(&self) -> u16;
// }

// impl AdcChExt for AdcCh {


//     fn complete(&self) -> bool {
//         self.periph.end_of_conversion()
//     }

//     fn wait(&self) -> &Self {
//         while !self.periph.end_of_conversion() {}
//         self
//     }

//     fn read(&self) -> u16 {
//         self.periph.data()
//     }
// }

impl AnalogRead<U12> for AdcCh {
    fn start(&self) -> &Self {
        self.periph.start_single(self.index, Resolution::Bits12);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph.end_of_conversion()
    }

    fn read(&self) -> U12 {
        self.periph.dr().data_12()
    }
}

impl AnalogRead<U10> for AdcCh {
    fn start(&self) -> &Self {
        self.periph.start_single(self.index, Resolution::Bits10);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph.end_of_conversion()
    }

    fn read(&self) -> U10 {
        self.periph.dr().data_10()
    }
}

impl AnalogRead<U8> for AdcCh {
    fn start(&self) -> &Self {
        self.periph.start_single(self.index, Resolution::Bits8);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph.end_of_conversion()
    }

    fn read(&self) -> U8 {
        self.periph.dr().data_8()
    }
}

impl AnalogRead<U6> for AdcCh {
    fn start(&self) -> &Self {
        self.periph.start_single(self.index, Resolution::Bits6);
        self
    }        

    fn is_complete(&self) -> bool {
        self.periph.end_of_conversion()
    }

    fn read(&self) -> U6 {
        self.periph.dr().data_6()
    }
}

impl AnalogRead<u8> for AdcCh {
    fn start(&self) -> &Self {
        self.periph.start_single(self.index, Resolution::Bits8);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph.end_of_conversion()
    }

    fn read(&self) -> u8 {
        self.periph.dr().data_8().value()
    }
}