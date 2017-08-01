use ::chip::dac::*;

pub fn set_ch1_enabled(mut dac: Dac, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        dac.with_cr(|r| r.set_en1(value));
    }
}

pub fn set_ch2_enabled(mut dac: Dac, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        dac.with_cr(|r| r.set_en2(value));
    }
}

pub fn set_ch1_trigger(mut dac: Dac, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        dac.set_swtrigr(Swtrigr(0).set_swtrig1(value));
    }    
}

pub fn set_ch2_trigger(mut dac: Dac, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        dac.set_swtrigr(Swtrigr(0).set_swtrig2(value));
    }    
}

pub fn write_ch1_8(mut dac: Dac, value: u8) {
    unsafe {
        dac.set_dhr8r1(Dhr8r1(0).set_dacc1dhr(value as u32))
    }
}

pub fn write_ch2_8(mut dac: Dac, value: u8) {
    unsafe {
        dac.set_dhr8r2(Dhr8r2(0).set_dacc2dhr(value as u32))
    }
}

pub fn read_ch1(dac: Dac) -> u16 {
    unsafe {
        dac.dor1().dacc1dor() as u16
    }
}

pub fn read_ch2(dac: Dac) -> u16 {
    unsafe {
        dac.dor2().dacc2dor() as u16
    }    
}