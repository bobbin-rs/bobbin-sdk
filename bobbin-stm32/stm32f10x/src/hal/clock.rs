use bobbin_common::bits::*;
use ::chip::rcc::{self, RCC, En};
use ::chip::flash;
use ::chip::usart::*;
use ::chip::tim_gen::*;
use ::chip::tim_adv::*;
// use ::chip::iwdg::*;
// use ::chip::wwdg::*;
use ::chip::i2c::*;
use ::chip::spi::*;
use core::fmt;
pub type Hz = Option<u32>;


// 8Mhz external clock
// System = 72Mhz
// AHB = 72Mhz (Divide by 1)
// APB1 = 36Mhz (Divide by 2)
// APB2 = 72Mhz (Divide by 1)

pub fn enable_pll_external_mode() {
    let rcc = rcc::RCC;
    let flash = flash::FLASH;

    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {}

    // Select HSI as SYSCLK source. 
    rcc.with_cfgr(|r| r.set_sw(0b00));

    while rcc.cfgr().sws() != 0b00 {}

    // Enable external high-speed oscillator 8MHz.    
    rcc.with_cr(|r| r.set_hseon(1));

    // Wait for HSE Ready
    while rcc.cr().hserdy() == 0 {}

    // Set prescalers for AHB, ADC, ABP1, ABP2

    // HPRE = HPRE_DIV_NONE
    // PPRE1 = PPRE_DIV_2 (0b100)
    // PPRE2 = PPRE_DIV_NONE (0b000)
    // PLLSRC = PREDIV1 (0b1)
    // PLLMUL = x9 (0b0111)
    // MCO = PLL/2 (0b0111)
    rcc.with_cfgr(|r| 
        r.set_hpre(0)
            .set_ppre1(0b100)
            .set_ppre2(0b000)
            .set_pllsrc(1)
            .set_pllmul(0b0111)
    );

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {}

    // Configure flash settings.
    // PRFTBE = enabled (0b1)
    // LATENCY = 2 Wait States (0b010)

    flash.with_acr(|r| r.set_latency(0b010));
    
    // Select PLL as SYSCLK source.

    rcc.with_cfgr(|r| r.set_sw(0b10));
        
    // Wait for PLL to be selected
    while rcc.cfgr().sws() != 0b10 {}
    
    // Disable internal high-speed oscillator.        
    rcc.with_cr(|r| r.set_hsion(0));
}

pub const LSI: Hz = Some(40_000);
pub const HSI: Hz = Some(8_000_000);

#[derive(Debug, PartialEq)]
pub enum SysClockSrc {
    Hsi = 0b00,
    Hse = 0b01,
    Pll = 0b10,
}

#[derive(Debug, PartialEq)]
pub enum PllSrc {
    HsiDiv2 = 0b00,
    Hse = 0b01,
}

#[derive(Debug, PartialEq)]
pub enum RtcSel {
    None = 0b00,
    Lse = 0b01,
    Lsi = 0b10,
    HseDiv128 = 0b11,
}

#[derive(Debug, PartialEq)]
pub enum HPre {
    Div1 = 0b0000,
    Div2 = 0b1000,
    Div4 = 0b1001,
    Div8 = 0b1010,
    Div16 = 0b1011,
    // Note: Divide by 32 is skipped
    Div64 = 0b1100,
    Div128 = 0b1101,
    Div256 = 0b1110,
    Div512 = 0b1111,
}

#[derive(Debug, PartialEq)]
pub enum PPre1 {
    Div1 = 0b000,
    Div2 = 0b100,
    Div4 = 0b101,
    Div8 = 0b110,
    Div16 = 0b111,
}

#[derive(Debug, PartialEq)]
pub enum PPre2 {
    Div1 = 0b000,
    Div2 = 0b100,
    Div4 = 0b101,
    Div8 = 0b110,
    Div16 = 0b111,
}

pub trait ClockTree {
    fn lsi(&self) -> Hz { LSI }
    fn hsi(&self) -> Hz { HSI }
    fn lse(&self) -> Hz;
    fn hse(&self) -> Hz;
    fn pllclk(&self) -> Hz;
    fn sysclk(&self) -> Hz;
    fn hclk(&self) -> Hz;
    fn systick(&self) -> Hz;
    fn pclk1(&self) -> Hz;
    fn tim_pclk1(&self) -> Hz;
    fn pclk2(&self) -> Hz;
    fn tim_pclk2(&self) -> Hz;
    fn rtcclk(&self) -> Hz;
    fn adcclk(&self) -> Hz;
}

pub trait Clock<T: ClockTree> {
    fn clock(&self, t: &T) -> Hz;
}

pub struct DynamicClock {
    pub hse_osc: Hz,
    pub lse_osc: Hz,
}

impl ClockTree for DynamicClock {
    fn hsi(&self) -> Hz { 
        HSI
    }

    fn hse(&self) -> Hz { 
        self.hse_osc
    }

    fn lsi(&self) -> Hz { 
        LSI
    }

    fn lse(&self) -> Hz {
        self.lse_osc
    }

    fn pllclk(&self) -> Hz {
        let pll_src = match RCC.cfgr().pllsrc() {
            U1::B0 => self.hsi().map(|v| v >> 1),
            U1::B1 => self.hse()
        };
        let pll_mul =  match RCC.cfgr().pllmul() {
            U4::B0000 => 2,
            U4::B0001 => 3,
            U4::B0010 => 4,
            U4::B0011 => 5,
            U4::B0100 => 6,
            U4::B0101 => 7,
            U4::B0110 => 8,
            U4::B0111 => 9,
            U4::B1000 => 10,
            U4::B1001 => 11,
            U4::B1010 => 12,
            U4::B1011 => 13,
            U4::B1100 => 14,
            U4::B1101 => 15,
            U4::B1110 => 16,
            U4::B1111 => 16,
        };
        pll_src.map(|v| v * pll_mul)
    }

    fn sysclk(&self) -> Hz {
        match RCC.cfgr().sws() {
            U2::B00 => self.hsi(),
            U2::B01 => self.hse(),
            U2::B10 => self.pllclk(),
            U2::B11 => panic!("Invalid value for RCC_CFGR[SWS]"),            
        }
    }

    fn hclk(&self) -> Hz {
        // Note: Divide by 32 is skipped
        let hclk_shift = match RCC.cfgr().hpre() {
            U4::B0000 => 0,
            U4::B0001 => 0,
            U4::B0010 => 0,
            U4::B0011 => 0,
            U4::B0100 => 0,
            U4::B0101 => 0,
            U4::B0110 => 0,
            U4::B0111 => 0,
            U4::B1000 => 1,
            U4::B1001 => 2,
            U4::B1010 => 3,
            U4::B1011 => 4,
            // NOTE: Divide by 32 is skipped
            U4::B1100 => 6,
            U4::B1101 => 7,
            U4::B1110 => 8,
            U4::B1111 => 9,
        };
        self.sysclk().map(|v| v >> hclk_shift)
    }

    fn pclk1(&self) -> Hz {
        let shift = match RCC.cfgr().ppre1() {
            U3::B000 => 0,
            U3::B001 => 0,
            U3::B010 => 0,
            U3::B011 => 0,            
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
        };
        self.hclk().map(|v| v >> shift)
    }

    fn tim_pclk1(&self) -> Hz {
        match RCC.cfgr().ppre1() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => self.pclk1(),
            _ => self.pclk1().map(|v| v << 1),
        }
    }

    fn pclk2(&self) -> Hz {
        let shift = match RCC.cfgr().ppre2() {
            U3::B000 => 0,
            U3::B001 => 0,
            U3::B010 => 0,
            U3::B011 => 0,            
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
        };
        self.hclk().map(|v| v >> shift)
    }

    fn tim_pclk2(&self) -> Hz {
        match RCC.cfgr().ppre2() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => self.pclk2(),
            _ => self.pclk2().map(|v| v << 1),
        }
    }

    fn adcclk(&self) -> Hz {
        let adc_div = match RCC.cfgr().adcpre() {
            U2::B00 => 2,
            U2::B01 => 4,
            U2::B10 => 6,
            U2::B11 => 8,            
        };
        self.pclk2().map(|v| v / adc_div)
    }

    fn systick(&self) -> Hz {
        self.hclk().map(|v| v >> 3)
    }

    fn rtcclk(&self) -> Hz {
        match RCC.bdcr().rtcsel() {
            U2::B00 => None,
            U2::B01 => self.lse(),
            U2::B10 => self.lsi(),
            U2::B11 => self.hse().map(|v| v >> 7),            
        }
    }
}

impl fmt::Debug for DynamicClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DynamicCLock")?;
        write!(f, " LSI={:?}", self.lsi())?;
        write!(f, " HSI={:?}", self.hsi())?;
        write!(f, " LSE={:?}", self.lse())?;
        write!(f, " HSE={:?}", self.hse())?;
        write!(f, " PLLCLK={:?}", self.pllclk())?;
        write!(f, " SYSCLK={:?}", self.sysclk())?;
        write!(f, " SYSTICK={:?}", self.systick())?;
        write!(f, " HCLK={:?}", self.hclk())?;
        write!(f, " PCLK1={:?}", self.pclk1())?;
        write!(f, " TIM_PCLK1={:?}", self.tim_pclk1())?;
        write!(f, " PCLK2={:?}", self.pclk2())?;
        write!(f, " TIM_PCLK2={:?}", self.tim_pclk2())?;
        write!(f, " PCLK2={:?}", self.pclk2())?;
        write!(f, " TIM_PCLK2={:?}", self.tim_pclk2())?;
        write!(f, " ADCCLK={:?}", self.adcclk())?;
        write!(f, "]")?;
        Ok(())
    }
}

macro_rules! impl_clock {
    ($ty:ty, $id:ident) => (
        impl<T: ClockTree> Clock<T> for $ty {
            fn clock(&self, t: &T) -> Hz {
                if self.en() != 0 {
                    t.$id()
                } else {
                    None
                }
            }
        }
    )
}


impl_clock!(Usart1, pclk2);
impl_clock!(Usart2, pclk1);
impl_clock!(Usart3, pclk1);


impl_clock!(Tim1, tim_pclk2);
impl_clock!(Tim2, tim_pclk1);
impl_clock!(Tim3, tim_pclk1);
impl_clock!(Tim4, tim_pclk1);
// impl_clock!(Tim5, tim_pclk1);
// impl_clock!(Tim6, tim_pclk1);
// impl_clock!(Tim7, tim_pclk1);
// impl_clock!(Tim8, tim_pclk2);
// impl_clock!(Tim9, tim_pclk2);
// impl_clock!(Tim10, tim_pclk2);
// impl_clock!(Tim11, tim_pclk2);
// impl_clock!(Tim12, tim_pclk1);
// impl_clock!(Tim13, tim_pclk1);
// impl_clock!(Tim14, tim_pclk1);


// impl_clock!(Iwdg, lsi);
impl_clock!(I2c1, pclk1);
impl_clock!(I2c2, pclk1);
impl_clock!(Spi1, pclk2);
impl_clock!(Spi2, pclk1);
impl_clock!(Spi3, pclk1);
