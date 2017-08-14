use bobbin_common::bits::*;
pub use bobbin_common::analog::AnalogRead;
pub use chip::adc::*;
pub use super::pcc::PccEnabled;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ClockDivide {
    Div1 = 0x00,
    Div2 = 0x01,
    Div4 = 0x02,
    Div8 = 0x04,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Resolution {
    Bits8 = 0x00,
    Bits12 = 0x01,
    Bits10 = 0x02,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum AltClock {
    Alt1 = 0x00,
    Alt2 = 0x01,
    Alt3 = 0x02,
    Alt4 = 0x03,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum TriggerMode {
    Software = 0x00,
    Hardware = 0x01,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VoltageReference {
    VRef = 0x00,
    VAlt = 0x01,
    Res0 = 0x02,
    Res1 = 0x03,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Average {
    Avg4 = 0x00,
    Avg8 = 0x01,
    Avg16 = 0x02,
    Avg32 = 0x03,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum InputChannel {
    Ad0 = 0x00,
    Ad1 = 0x01,
    Ad2 = 0x02,
    Ad3 = 0x03,
    Ad4 = 0x04,
    Ad5 = 0x05,
    Ad6 = 0x06,
    Ad7 = 0x07,
    Ad8 = 0x08,
    Ad9 = 0x09,
    Ad10 = 0x0a,
    Ad11 = 0x0b,
    Ad12 = 0x0c,
    Ad13 = 0x0d,
    Ad14 = 0x0e,
    Ad15 = 0x0f,
    Ad16 = 0x10,
    Ad17 = 0x11,
    Ad18 = 0x12,
    Ad19 = 0x13,
    Ad20 = 0x14,
    Ad21 = 0x15,
    Ad22 = 0x16,
    Ad23 = 0x17,
    Ad24 = 0x18,
    Ad25 = 0x19,
    Ad26 = 0x1a,
    Ad27 = 0x1b,
    Ad28 = 0x1c,
    Ad29 = 0x1d,
    Ad30 = 0x1e,
    Ad31 = 0x1f,
}


pub trait AdcExt {
    fn init(&self) -> &Self;
    fn is_active(&self) -> bool;
    fn clock_divide(&self) -> ClockDivide;
    fn set_clock_divide(&self, value: ClockDivide) -> &Self;
    fn sample_time(&self) -> u8;
    fn set_sample_time(&self, value: u8) -> &Self;
    fn resolution(&self) -> Resolution;
    fn set_resolution(&self, value: Resolution) -> &Self;
    fn input_clock(&self) -> AltClock;
    fn set_input_clock(&self, value: AltClock) -> &Self;
    fn trigger_mode(&self) -> TriggerMode;
    fn set_trigger_mode(&self, value: TriggerMode) -> &Self;
    fn dma_enabled(&self) -> bool;
    fn set_dma_enabled(&self, value: bool) -> &Self;
    fn voltage_reference(&self) -> VoltageReference;
    fn set_voltage_reference(&self, value: VoltageReference) -> &Self;
    fn continuous_conversion(&self) -> bool;
    fn set_continuous_conversion(&self, value: bool) -> &Self;
    fn calibration_active(&self) -> bool;
    fn set_calibration_active(&self, value: bool) -> &Self;
}

impl<T> AdcExt for Periph<T> {
    fn init(&self) -> &Self {
        // ADC_HAL_SetClockDivide(base, config->clockDivide);
        // ADC_HAL_SetSampleTime(base, config->sampleTime);
        // ADC_HAL_SetResolution(base, config->resolution);
        // ADC_HAL_SetInputClock(base, config->inputClock);
        // ADC_HAL_SetTriggerMode(base, config->trigger);
        // ADC_HAL_SetDMAEnableFlag(base, config->dmaEnable);
        // ADC_HAL_SetVoltageReference(base, config->voltageRef);
        // ADC_HAL_SetContinuousConvFlag(base, config->continuousConvEnable);
        self
    }

    fn is_active(&self) -> bool {
        self.sc2().adact() != 0
    }

    fn clock_divide(&self) -> ClockDivide {
        match self.cfg1().adiv() {
            U2::B00 => ClockDivide::Div1,
            U2::B01 => ClockDivide::Div2,
            U2::B10 => ClockDivide::Div4,
            U2::B11 => ClockDivide::Div8,
        }
    }

    fn set_clock_divide(&self, value: ClockDivide) -> &Self {
        self.with_cfg1(|r| r.set_adiv(value as u32))
    }

    fn sample_time(&self) -> u8 {
        self.cfg2().smplts().value()
    }

    fn set_sample_time(&self, value: u8) -> &Self {
        self.with_cfg2(|r| r.set_smplts(value))
    }

    fn resolution(&self) -> Resolution {
        match self.cfg1().mode() {
            U2::B00 => Resolution::Bits8,
            U2::B01 => Resolution::Bits12,
            U2::B10 => Resolution::Bits10,
            U2::B11 => Resolution::Bits8,
        }
    }

    fn set_resolution(&self, value: Resolution) -> &Self {
        self.with_cfg1(|r| r.set_mode(value as u8))
    }

    fn input_clock(&self) -> AltClock {
        match self.cfg1().adiclk() {
            U2::B00 => AltClock::Alt1,
            U2::B01 => AltClock::Alt2,
            U2::B10 => AltClock::Alt3,
            U2::B11 => AltClock::Alt4,
        }
    }

    fn set_input_clock(&self, value: AltClock) -> &Self {
        self.with_cfg1(|r| r.set_adiclk(value as u8))
    }

    fn trigger_mode(&self) -> TriggerMode {
        match self.sc2().adtrg() {
            U1::B0 => TriggerMode::Software,
            U1::B1 => TriggerMode::Hardware,
        }
    }

    fn set_trigger_mode(&self, value: TriggerMode) -> &Self {
        self.with_sc2(|r| r.set_adtrg(value as u8))
    }

    fn dma_enabled(&self) -> bool {
        self.sc2().dmaen() != 0
    }

    fn set_dma_enabled(&self, value: bool) -> &Self {
        self.with_sc2(|r| r.set_dmaen(value))
    }

    fn voltage_reference(&self) -> VoltageReference {
        match self.sc2().refsel() {
            U2::B00 => VoltageReference::VRef,
            U2::B01 => VoltageReference::VAlt,
            U2::B10 => VoltageReference::Res0,
            U2::B11 => VoltageReference::Res1,
        }
    }

    fn set_voltage_reference(&self, value: VoltageReference) -> &Self {
        self.with_sc2(|r| r.set_refsel(value as u8))
    }

    fn continuous_conversion(&self) -> bool {
        self.sc3().adco() != 0
    }

    fn set_continuous_conversion(&self, value: bool) -> &Self {
        self.with_sc3(|r| r.set_adco(value))
    }

    fn calibration_active(&self) -> bool {
        self.sc3().cal() != 0
    }

    fn set_calibration_active(&self, value: bool) -> &Self {
        self.with_sc3(|r| r.set_cal(value))
    }
}

pub trait AdcChExt {
    fn start(&self) -> &Self;
    fn complete(&self) -> bool;
    fn wait(&self) -> &Self {
        while !self.complete() {}
        self
    }
    fn read(&self) -> u16;
}

impl<P, T> AdcChExt for Channel<P, T> {
    fn start(&self) -> &Self {
        self.periph.with_sc1(0, |r| r.set_adch(self.index as u8));
        self
    }

    fn complete(&self) -> bool {
        self.periph.sc1(0).coco() != 0
    }

    fn read(&self) -> u16 {
        self.periph.r(0).d().into()
    }
}

impl<P, T> AnalogRead<u8> for Channel<P, T> {
    fn analog_read(&self) -> u8 {
        self.periph.set_resolution(Resolution::Bits8);
        self.start().wait();
        self.periph.r(0).d8().into()
    }
}

impl<P, T> AnalogRead<U8> for Channel<P, T> {
    fn analog_read(&self) -> U8 {
        self.periph.set_resolution(Resolution::Bits8);
        self.start().wait();
        self.periph.r(0).d8()
    }
}

impl<P, T> AnalogRead<U10> for Channel<P, T> {
    fn analog_read(&self) -> U10 {
        self.periph.set_resolution(Resolution::Bits10);
        self.start().wait();
        self.periph.r(0).d10()
    }
}

impl<P, T> AnalogRead<U12> for Channel<P, T> {
    fn analog_read(&self) -> U12 {
        self.periph.set_resolution(Resolution::Bits12);
        self.start().wait();
        self.periph.r(0).d12()
    }
}