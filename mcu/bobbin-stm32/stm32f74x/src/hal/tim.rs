use ::chip::tim::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Channel {
    Ch1,
    Ch2,
    Ch3,
    Ch4,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OcMode {
    Frozen =        0b000,
    Active =        0b001,
    Inactive =      0b010,
    Toggle =        0b011,
    ForceInactive = 0b100,
    ForceActive =   0b101,
    Pwm1 =          0b110,
    Pwm2 =          0b111,
}

pub fn set_enabled(mut tim: Tim, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        tim.with_cr1(|r| r.set_cen(value))
    }
}

pub fn set_period(mut tim: Tim, value: u32) {
    unsafe {
        tim.set_arr(Arr(value));
    }
}

pub fn set_oc_value(mut tim: Tim, ch: Channel, value: u32) {
    use self::Channel::*;
    unsafe {
        match ch {
            Ch1 => tim.set_ccr1(Ccr1(value)),
            Ch2 => tim.set_ccr2(Ccr2(value)),
            Ch3 => tim.set_ccr3(Ccr3(value)),
            Ch4 => tim.set_ccr4(Ccr4(value)),
        }
    }
}

pub fn set_oc_output(mut tim: Tim, ch: Channel, value: bool) {
    use self::Channel::*;
    let value = if value { 1 } else { 0 };
    unsafe {
        match ch {
            Ch1 => tim.with_ccer(|r| r.set_cc1e(value)),
            Ch2 => tim.with_ccer(|r| r.set_cc2e(value)),
            Ch3 => tim.with_ccer(|r| r.set_cc3e(value)),
            Ch4 => tim.with_ccer(|r| r.set_cc4e(value)),
        }        
    }
}

pub fn set_oc_mode(mut tim: Tim, ch: Channel, mode: OcMode) {
    use self::Channel::*;
    unsafe {
        match ch {
            Ch1 => tim.with_ccmr1_output(|r| r.set_cc1s(0).set_oc1m(mode as u32)),
            Ch2 => tim.with_ccmr1_output(|r| r.set_cc2s(0).set_oc2m(mode as u32)),
            Ch3 => tim.with_ccmr2_output(|r| r.set_cc3s(0).set_oc3m(mode as u32)),
            Ch4 => tim.with_ccmr2_output(|r| r.set_cc4s(0).set_oc4m(mode as u32)),
        }
    }
}