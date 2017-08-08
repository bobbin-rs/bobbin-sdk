use ::chip::mpu::*;

// MPU_TYPE

pub fn iregion() -> u8 {
    MPU.mpu_type().iregion().into()
}

pub fn dregion() -> u8 {
    MPU.mpu_type().dregion().into()
}

pub fn separate() -> bool {
    MPU.mpu_type().separate() != 0
}

// MPU_CTRL

pub fn privdefena() -> bool {
    MPU.mpu_ctrl().privdefena() != 0
}

pub fn set_privdefena(value: bool) {
    MPU.with_mpu_ctrl(|r| r.set_privdefena(value));
}

pub fn hfnmiena() -> bool {
    MPU.mpu_ctrl().hfnmiena() != 0
}

pub fn set_hfnmiena(value: bool) {
    MPU.with_mpu_ctrl(|r| r.set_hfnmiena(value));
}

pub fn enable() -> bool {
    MPU.mpu_ctrl().enable() != 0
}

pub fn set_enable(value: bool) {
    MPU.with_mpu_ctrl(|r| r.set_enable(value));
}

// MPU_RNR

pub fn region_number() -> u8 {
    MPU.mpu_rnr().region().into()
}

pub fn set_region_number(value: u8) {
    MPU.set_mpu_rnr(|r| r.set_region(value));
}

// MPU_RBAR

pub fn region_base_address() -> u32 {
    MPU.mpu_rbar().addr().into()
}

pub fn set_region_base_address(value: u32) {
    MPU.with_mpu_rbar(|r| r.set_addr(value));
}

pub fn region_base_address_valid() -> bool {
    MPU.mpu_rbar().valid() != 0
}

pub fn set_region_base_address_valid(value: bool) {
    let value = if value { 1 } else { 0 };
    MPU.with_mpu_rbar(|r| r.set_valid(value));
}

pub fn region_base_address_region() -> u8 {
    MPU.mpu_rbar().region().into()
}

pub fn set_region_base_address_region(value: u8) {
    MPU.with_mpu_rbar(|r| r.set_region(value));
}

// MPU_RASR

pub fn region_xn() -> bool {
    MPU.mpu_rasr().xn() != 0
}

pub fn set_region_xn(value: bool) {
    MPU.with_mpu_rasr(|r| r.set_xn(value));
}

pub fn region_ap() -> u8 {
    MPU.mpu_rasr().ap().into()
}

pub fn set_region_ap(value: u8) {
    MPU.with_mpu_rasr(|r| r.set_ap(value));
}

pub fn region_tex() -> u8 {
    MPU.mpu_rasr().tex().into()
}

pub fn set_region_tex(value: u8) {
    MPU.with_mpu_rasr(|r| r.set_tex(value));
}

pub fn region_c() -> bool {
    MPU.mpu_rasr().c() != 0
}

pub fn set_region_c(value: bool) {
    MPU.with_mpu_rasr(|r| r.set_c(value));
}

pub fn region_b() -> bool {
    MPU.mpu_rasr().b() != 0
}

pub fn set_region_b(value: bool) {
    MPU.with_mpu_rasr(|r| r.set_b(value));
}

pub fn region_srd() -> u8 {
    MPU.mpu_rasr().srd().into()
}

pub fn set_region_srd(value: u8) {
    MPU.with_mpu_rasr(|r| r.set_srd(value));
}

pub fn region_size() -> u8 {
    MPU.mpu_rasr().size().into()
}

pub fn set_region_size(value: u8) {
    MPU.with_mpu_rasr(|r| r.set_size(value));
}

pub fn region_enable() -> bool {
    MPU.mpu_rasr().enable() != 0
}

pub fn set_region_enable(value: bool) {
    MPU.with_mpu_rasr(|r| r.set_enable(value));
}