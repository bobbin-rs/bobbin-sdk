use ::chip::adc::*;

pub enum Resolution {
    Bits12 = 0b00,
    Bits10 = 0b01,
    Bits8 =  0b10,
    Bits6 =  0b11,
}

pub fn set_enabled(mut adc: Adc, value: bool)  {
    let value = if value { 1 } else { 0 };
    unsafe {
        adc.with_cr2(|r| r.set_adon(value));
    }
}

pub fn set_sequence_channel(mut adc: Adc, sequence: u8, channel: u8) {
    assert!(sequence > 0 && sequence <= 16, "Sequence must be 1..16");
    assert!(channel < 16, "Channel must b 0..15");
    unsafe {
        let channel = channel as u32;
        match sequence {
            1 => adc.with_sqr3(|r| r.set_sq1(channel)),
            2 => adc.with_sqr3(|r| r.set_sq2(channel)),
            3 => adc.with_sqr3(|r| r.set_sq3(channel)),
            4 => adc.with_sqr3(|r| r.set_sq4(channel)),
            5 => adc.with_sqr3(|r| r.set_sq5(channel)),
            6 => adc.with_sqr3(|r| r.set_sq6(channel)),
            7 => adc.with_sqr2(|r| r.set_sq7(channel)),
            8 => adc.with_sqr2(|r| r.set_sq8(channel)),
            9 => adc.with_sqr2(|r| r.set_sq9(channel)),
            10 => adc.with_sqr2(|r| r.set_sq10(channel)),
            11 => adc.with_sqr2(|r| r.set_sq11(channel)),
            12 => adc.with_sqr2(|r| r.set_sq12(channel)),
            13 => adc.with_sqr1(|r| r.set_sq13(channel)),
            14 => adc.with_sqr1(|r| r.set_sq14(channel)),
            15 => adc.with_sqr1(|r| r.set_sq15(channel)),
            16 => adc.with_sqr1(|r| r.set_sq16(channel)),
            _ => unimplemented!()
        }
    }
}

pub fn set_sequence_length(mut adc: Adc, length: u8) {
    assert!(length > 0 && length <= 16, "length must be 1..16");
    unsafe {
        adc.with_sqr1(|r| r.set_l((length - 1) as u32));
    }
}

pub fn set_start(mut adc: Adc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        adc.with_cr2(|r| r.set_swstart(value));
    }
}

pub fn set_continuous(mut adc: Adc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        adc.with_cr2(|r| r.set_cont(value))
    }
}

pub fn set_resolution(mut adc: Adc, value: Resolution) {
    unsafe {
        adc.with_cr1(|r| r.set_res(value as u32))
    }
}

#[inline]
pub fn complete(adc: Adc) -> bool {
    unsafe {
        adc.sr().eoc() != 0
    }
}

#[inline]
pub fn data(adc: Adc) -> u16 {
    unsafe {
        adc.dr().data() as u16
    }
}