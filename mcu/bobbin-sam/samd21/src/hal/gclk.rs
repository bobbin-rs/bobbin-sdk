#![allow(non_camel_case_types)]

pub use ::chip::gclk::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClockSource {
    XOSC = 0x00,
    GCLKIN = 0x01,
    GCLKGEN1 = 0x02,
    OSCULP32K = 0x03,
    OSC32K = 0x04,
    XOSC32K = 0x05,
    OSC8M = 0x06,
    DFLL48M = 0x07,
    FDPLL96M = 0x08,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GenericClockGen {
    GClkGen0 = 0x0,
    GClkGen1 = 0x1,
    GClkGen2 = 0x2,
    GClkGen3 = 0x3,
    GClkGen4 = 0x4,
    GClkGen5 = 0x5,
    GClkGen6 = 0x6,
    GClkGen7 = 0x7,
    GClkGen8 = 0x8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GenericClock {
    DFLL48M_REF = 0x00,
    DPLL = 0x01,
    DPLL_32K = 0x02,
    WDT = 0x03,
    RTC = 0x04,
    EIC = 0x05,
    USB = 0x06,
    EVSYS_CHANNEL_0 = 0x07,
    EVSYS_CHANNEL_1 = 0x08,
    EVSYS_CHANNEL_2 = 0x09,
    EVSYS_CHANNEL_3 = 0x0a,
    EVSYS_CHANNEL_4 = 0x0b,
    EVSYS_CHANNEL_5 = 0x0c,
    EVSYS_CHANNEL_6 = 0x0d,
    EVSYS_CHANNEL_7 = 0x0e,
    EVSYS_CHANNEL_8 = 0x0f,
    EVSYS_CHANNEL_9 = 0x10,
    EVSYS_CHANNEL_10 = 0x11,
    EVSYS_CHANNEL_11 = 0x12,
    SERCOMx_SLOW = 0x13,
    SERCOM0_CORE = 0x14,
    SERCOM1_CORE = 0x15,
    SERCOM2_CORE = 0x16,
    SERCOM3_CORE = 0x17,
    SERCOM4_CORE = 0x18,
    SERCOM5_CORE = 0x19,
    TCC0_TCC1 = 0x1a,
    TCC2_TC3 = 0x1b,
    TC4_TC5 = 0x1c,
    TC6_TC7 = 0x1d,
    ADC = 0x1e,
    AC_DIG = 0x1f,
    AC_ANA = 0x21,
    DAC = 0x22,
    PTC = 0x24,
    I2S_0 = 0x25,
    I2S_1 = 0x26,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ClockConfig {
    write_lock: bool,
    clk_gen: GenericClockGen,
    enabled: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ClockGenConfig {
    run_in_standby: bool,
    divide_selection: DivideSelection,
    output_enable: bool,
    output_off_value: bool,
    improve_duty_cycle: bool,
    src: ClockSource,
    enabled: bool,
    division_factor: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DivideSelection {
    Divide, // Divide by DivFactor
    DivideExp, // Divide by 2^(DivFactor + 1)
}

pub fn reset() {
    GCLK.set_ctrl(|r| r.set_swrst(1));
    while GCLK.ctrl().swrst() != 0 {}
}

pub fn configure_clk(clk: GenericClock, cfg: ClockConfig) {    
    GCLK.set_clkctrl(|r| r
        .set_id(clk as u16)
        .set_wrtlock(bool2u16(cfg.write_lock))
        .set_gen(cfg.clk_gen as u16)
        .set_clken(bool2u16(cfg.enabled)));
    while GCLK.status().syncbusy() != 0 {}
}

/// Configure and enable a Generic Clock
pub fn set_clk(clk: GenericClock, clk_gen: GenericClockGen) {
    GCLK.set_clkctrl(|r| r
        .set_id(clk as u16)
        .set_gen(clk_gen as u16)
        .set_clken(1));
    while GCLK.status().syncbusy() != 0 {}            
}

pub fn configure_clkgen(clk_gen: GenericClockGen, cfg: ClockGenConfig) {
    GCLK.set_gendiv(|r| r.set_id(clk_gen as u32).set_div(cfg.division_factor as u32));
    GCLK.set_genctrl(|r| r
        .set_id(clk_gen as u32)
        .set_runstdby(bool2u32(cfg.run_in_standby))
        .set_divsel(cfg.divide_selection as u32)
        .set_oe(bool2u32(cfg.output_enable))
        .set_oov(bool2u32(cfg.output_off_value))
        .set_idc(bool2u32(cfg.improve_duty_cycle))
        .set_genen(bool2u32(cfg.enabled))
        .set_src(cfg.src as u32)
    );        
    while GCLK.status().syncbusy() != 0 {}
}

/// Configure and enable a Generic Clock using defaults
pub fn set_clockgen(clk_gen: GenericClockGen, clk_src: ClockSource, div_selection: DivideSelection, div_factor: u32) {
    GCLK.set_gendiv(|r| r.set_id(clk_gen as u32).set_div(div_factor as u32));
    GCLK.set_genctrl(|r| r
        .set_id(clk_gen as u32)
        .set_divsel(div_selection as u32)
        .set_src(clk_src as u32)
    );
}

fn bool2u16(value: bool) -> u16 {
    if value { 1 } else { 0 }
}

fn bool2u32(value: bool) -> u32 {
    if value { 1 } else { 0 }
}