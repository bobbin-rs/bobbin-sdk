use ::chip::mpu::*;

// MPU_TYPE

pub fn iregion() -> u8 {
    unsafe {
        MPU.mpu_type().iregion() as u8
    }
}

pub fn dregion() -> u8 {
    unsafe {
        MPU.mpu_type().dregion() as u8
    }
}

pub fn separate() -> bool {
    unsafe {
        MPU.mpu_type().separate() != 0
    }
}

// MPU_CTRL

pub fn privdefena() -> bool {
    unsafe {
        MPU.mpu_ctrl().privdefena() != 0
    }    
}

pub fn set_privdefena(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        MPU.with_mpu_ctrl(|r| r.set_privdefena(value))
    }
}

pub fn hfnmiena() -> bool {
    unsafe {
        MPU.mpu_ctrl().hfnmiena() != 0
    }    
}

pub fn set_hfnmiena(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        MPU.with_mpu_ctrl(|r| r.set_hfnmiena(value))
    }
}

pub fn enable() -> bool {
    unsafe {
        MPU.mpu_ctrl().enable() != 0
    }    
}

pub fn set_enable(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        MPU.with_mpu_ctrl(|r| r.set_enable(value))
    }
}

// MPU_RNR

pub fn region_number() -> u8 {
    unsafe {
        MPU.mpu_rnr().region() as u8
    }
}

pub fn set_region_number(value: u8) {
    unsafe {
        MPU.set_mpu_rnr(MpuRnr(0).set_region(value as u32))
    }
}

// MPU_RBAR

pub fn region_base_address() -> u32 {
    unsafe {
        MPU.mpu_rbar().addr()
    }
}

pub fn set_region_base_address(value: u32) {
    unsafe {
        MPU.with_mpu_rbar(|r| r.set_addr(value))
    }
}

pub fn region_base_address_valid() -> bool {
    unsafe {
        MPU.mpu_rbar().valid() != 0
    }    
}

pub fn set_region_base_address_valid(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        MPU.with_mpu_rbar(|r| r.set_valid(value))
    }
}

pub fn region_base_address_region() -> u8 {
    unsafe {
        MPU.mpu_rbar().region() as u8
    }    
}

pub fn set_region_base_address_region(value: u8) {
    unsafe {
        MPU.with_mpu_rbar(|r| r.set_region(value as u32))
    }
}

// MPU_RASR

pub fn region_xn() -> bool {
    unsafe {
        MPU.mpu_rasr().xn() != 0
    }
}

pub fn set_region_xn(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        MPU.with_mpu_rasr(|r| r.set_xn(value))
    }
}

pub fn region_ap() -> u8 {
    unsafe {
        MPU.mpu_rasr().ap() as u8
    }    
}

pub fn set_region_ap(value: u8) {
    unsafe {
        MPU.with_mpu_rasr(|r| r.set_ap(value as u32))
    }
}

pub fn region_tex() -> u8 {
    unsafe {
        MPU.mpu_rasr().tex() as u8
    }    
}

pub fn set_region_tex(value: u8) {
    unsafe {
        MPU.with_mpu_rasr(|r| r.set_tex(value as u32))
    }
}

pub fn region_c() -> bool {
    unsafe {
        MPU.mpu_rasr().c() != 0
    }
}

pub fn set_region_c(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        MPU.with_mpu_rasr(|r| r.set_c(value))
    }
}

pub fn region_b() -> bool {
    unsafe {
        MPU.mpu_rasr().b() != 0
    }
}

pub fn set_region_b(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        MPU.with_mpu_rasr(|r| r.set_b(value))
    }
}

pub fn region_srd() -> u8 {
    unsafe {
        MPU.mpu_rasr().srd() as u8
    }    
}

pub fn set_region_srd(value: u8) {
    unsafe {
        MPU.with_mpu_rasr(|r| r.set_srd(value as u32))
    }
}

pub fn region_size() -> u8 {
    unsafe {
        MPU.mpu_rasr().size() as u8
    }    
}

pub fn set_region_size(value: u8) {
    unsafe {
        MPU.with_mpu_rasr(|r| r.set_size(value as u32))
    }
}

pub fn region_enable() -> bool {
    unsafe {
        MPU.mpu_rasr().enable() != 0
    }
}

pub fn set_region_enable(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        MPU.with_mpu_rasr(|r| r.set_enable(value))
    }
}