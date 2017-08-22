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

pub trait AdcExt {
    fn init(&self) -> &Self;
    fn data(&self) -> u16;
    fn start(&self) -> &Self;
    fn stop(&self) -> &Self;
    fn status(&self) -> Isr;
    fn end_of_sequence(&self) -> bool;
    fn clr_end_of_sequence(&self) -> &Self;
    fn end_of_conversion(&self) -> bool;
    fn clr_end_of_conversion(&self) -> &Self;
    fn set_continuous(&self, value: bool) -> &Self;
    fn set_resolution(&self, value: Resolution) -> &Self;
    fn set_channel(&self, value: usize) -> &Self;
    fn clr_channel(&self, value: usize) -> &Self;
    fn clr_channels(&self) -> &Self;
    fn start_single(&self, channel: usize, value: Resolution) -> &Self;
}

impl<T> AdcExt for Periph<T> {
    fn init(&self) -> &Self {
       
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

    fn data(&self) -> u16 {
        self.dr().data().value()
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

    fn clr_end_of_sequence(&self) -> &Self {
        self.with_isr(|r| r.set_eos(1))
    }

    fn end_of_conversion(&self) -> bool {
        self.isr().eoc() != 0
    }

    fn clr_end_of_conversion(&self) -> &Self {
        self.with_isr(|r| r.set_eoc(1))
    }

    fn set_continuous(&self, value: bool) -> &Self {
        self.with_cfgr1(|r| r.set_cont(value))
    }

    fn set_resolution(&self, value: Resolution) -> &Self {
        self.with_cfgr1(|r| r.set_res(value as u32))
    }    

    fn set_channel(&self, channel: usize) -> &Self {
        assert!(channel <= 18, "Channel must be 0..18");
        self.with_chselr(|r| r.set_chsel(channel, 1))
    }

    fn clr_channel(&self, channel: usize) -> &Self {
        assert!(channel <= 18, "Channel must be 0..18");
        self.with_chselr(|r| r.set_chsel(channel, 0))
    }
    fn clr_channels(&self) -> &Self {
        self.set_chselr(|r| r)
    }

    fn start_single(&self, channel: usize, value: Resolution) -> &Self {
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

// impl<P, T> AdcChExt for Channel<P, T> {


//     fn complete(&self) -> bool {
//         self.periph().end_of_conversion()
//     }

//     fn wait(&self) -> &Self {
//         while !self.periph().end_of_conversion() {}
//         self
//     }

//     fn read(&self) -> u16 {
//         self.periph().data()
//     }
// }

impl<P, T> AnalogRead<U12> for Channel<P, T> {
    fn start(&self) -> &Self {
        self.periph().start_single(self.index, Resolution::Bits12);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph().end_of_conversion()
    }

    fn read(&self) -> U12 {
        self.periph().dr().data_12()
    }
}

impl<P, T> AnalogRead<U10> for Channel<P, T> {
    fn start(&self) -> &Self {
        self.periph().start_single(self.index, Resolution::Bits10);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph().end_of_conversion()
    }

    fn read(&self) -> U10 {
        self.periph().dr().data_10()
    }
}

impl<P, T> AnalogRead<U8> for Channel<P, T> {
    fn start(&self) -> &Self {
        self.periph().start_single(self.index, Resolution::Bits8);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph().end_of_conversion()
    }

    fn read(&self) -> U8 {
        self.periph().dr().data_8()
    }
}

impl<P, T> AnalogRead<U6> for Channel<P, T> {
    fn start(&self) -> &Self {
        self.periph().start_single(self.index, Resolution::Bits6);
        self
    }        

    fn is_complete(&self) -> bool {
        self.periph().end_of_conversion()
    }

    fn read(&self) -> U6 {
        self.periph().dr().data_6()
    }
}

impl<P, T> AnalogRead<u8> for Channel<P, T> {
    fn start(&self) -> &Self {
        self.periph().start_single(self.index, Resolution::Bits8);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph().end_of_conversion()
    }

    fn read(&self) -> u8 {
        self.periph().dr().data_8().value()
    }
}