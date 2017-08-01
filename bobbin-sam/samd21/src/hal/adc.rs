#![allow(non_camel_case_types)]

pub use ::chip::adc::*;

pub struct AdcDevice {
    adc: Adc,
}

pub enum Resolution {
    Bits12 = 0x00,
    Bits16 = 0x01,
    Bits10 = 0x02,
    Bits8 = 0x03,
}

pub enum Reference {
    INT1V = 0x0,
    INTVCC0 = 0x1,
    INTVCC1 = 0x2,
    VREFA = 0x3,
    VREFB = 0x4,
}

pub enum Gain {
    Gain1x = 0x0,
    Gain2x = 0x1,
    Gain4x = 0x2,
    Gain8x = 0x3,
    Gain16x = 0x4,
    Div2 = 0xf,
}

pub enum MuxNeg {
    Pin0 = 0x00,
    Pin1 = 0x01,
    Pin2 = 0x02,
    Pin3 = 0x03,
    Pin4 = 0x04,
    Pin5 = 0x05,
    Pin6 = 0x06,
    Pin7 = 0x07,
    Gnd = 0x18,
    IoGnd = 0x19,
}

pub enum MuxPos {
    Pin0 = 0x00,
    Pin1 = 0x01,
    Pin2 = 0x02,
    Pin3 = 0x03,
    Pin4 = 0x04,
    Pin5 = 0x05,
    Pin6 = 0x06,
    Pin7 = 0x07,
    Pin8 = 0x08,
    Pin9 = 0x09,
    Pin10 = 0x0a,
    Pin11 = 0x0b,
    Pin12 = 0x0c,
    Pin13 = 0x0d,
    Pin14 = 0x0e,
    Pin15 = 0x0f,
    Pin16 = 0x10,
    Pin17 = 0x11,
    Pin18 = 0x12,
    Pin19 = 0x13,
    Temp = 0x18,
    Bandgap = 0x19,
    ScaledCoreVcc = 0x1a,
    ScaledIoVcc = 0x1b,
    Dac = 0x1c,    
}


pub struct Config {
    pub reference: Reference,
    pub muxneg: MuxNeg,
    pub muxpos: MuxPos,
    pub inputscan: u8,
    pub gain: Gain,
}

pub fn configure(mut adc: Adc, cfg: Config) -> AdcDevice {       
    unsafe {
        adc.with_ctrla(|r| r.set_enable(0));
        while adc.status().syncbusy() != 0 {}
        adc.set_ctrla(Ctrla(0).set_swrst(1));
        while adc.ctrla().swrst() != 0 {}
        //adc.with_ctrlb(|r| r.set_ressel(0x3));
        adc.with_ctrlb(|r| r.set_prescaler(0x7).set_ressel(Resolution::Bits12 as u16));
        adc.set_sampctrl(Sampctrl(0).set_samplen(0x3f));
        adc.set_refctrl(Refctrl(0).set_refsel(cfg.reference as u8));
        while adc.status().syncbusy() != 0 {}

        adc.set_inputctrl(Inputctrl(0)
            .set_muxneg(cfg.muxneg as u32)
            .set_muxpos(cfg.muxpos as u32)
            .set_inputscan(cfg.inputscan as u32)
        );

    }
    AdcDevice { adc: adc }
}

impl AdcDevice {
    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            let mut adc = self.adc;
            adc.with_ctrla(|r| r.set_enable(value));
            while self.adc.status().syncbusy() != 0 {}
        }
    }


    pub fn reset(&self) {
        self.set_enabled(false);
        unsafe {
            let mut adc = self.adc;
            adc.set_ctrla(Ctrla(0).set_swrst(1));
            while self.adc.ctrla().swrst() != 0 {}
        }
    }

    pub fn start(&self) {
        unsafe {
            let mut adc = self.adc;            
            adc.set_swtrig(Swtrig(0).set_start(1));
        }
    }

    pub fn result(&self) -> u16 {
        unsafe {
            self.adc.result().result()
        }
    }

    pub fn intflag(&self) -> Intflag {
        unsafe {
            self.adc.intflag()
        }
    }
    pub fn clr_resready(&self) {
        unsafe {
            let mut adc = self.adc;
            adc.set_intflag(Intflag(0).set_resrdy(1));
        }
    }
}