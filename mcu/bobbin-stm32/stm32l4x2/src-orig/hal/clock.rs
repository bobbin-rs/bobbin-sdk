use bobbin_common::bits::*;
pub use bobbin_common::clock::*;

use ::chip::rcc::RCC;
// use ::chip::pwr::PWR;
use ::chip::flash::FLASH;
use ::chip::usart::*;
use ::chip::lpuart::*;
use ::chip::lptim::*;
use ::chip::tim_adv::*;
use ::chip::tim_bas::*;
use ::chip::tim_gen::*;
use ::chip::i2c::*;
use ::chip::spi::*;

use core::fmt;

// Main System Clock = 80 MHz
// APB2 = 80 MHz
// APB1 = 80 MHz
// AHB = 80 MHz
// 
// HSI @ 16 MHz
// VCO @ 160 MHz (M = 1, N = 10)
// PLL @ 80 Mhz (R = 2)
// FLASH: 4 wait states


pub fn init_pll() {
    // (1) Set one wait state in Latency bit of FLASH_ACR 
    FLASH.with_acr(|r| r.set_latency(4));

    // (2) Check the latency is set
    while FLASH.acr().latency() != 4 {}

    // (3) Switch the clock on HSI16/4 and disable PLL

    RCC.with_cr(|r| r.set_pllon(0).set_hsion(1));

    // Wait for HSI16 Ready Flag
    while RCC.cr().hsirdy() == 0 {}


    RCC.with_pllcfgr(|r| r.set_pllsrc(0b10).set_pllm(0x0).set_plln(0xa).set_pllr(0x0).set_pllren(1));

    // (5) Enable and switch on PLL 

    RCC.with_cr(|r| r.set_pllon(1));

    // Wait for PLL Ready
    while RCC.cr().pllrdy() == 0 {}

    // Switch to PLL
    RCC.with_cfgr(|r| r.set_sw(0b11));

    // Wait for system clock to use PLL
    while RCC.cfgr().sws() != 0b11 {}
}

pub const HSI: Hz = Some(16_000_000);
pub const LSI: Hz = Some(32_000);

pub trait ClockTree {
    fn hsi16(&self) -> Hz;
    fn lsi(&self) -> Hz;
    fn msi(&self) -> Hz;
    fn hse(&self) -> Hz;
    fn lse(&self) -> Hz;
    fn pllclk(&self) -> Hz;
    fn sysclk(&self) -> Hz;
    fn hclk(&self) -> Hz;
    fn pclk1(&self) -> Hz;
    fn tim_pclk1(&self) -> Hz;
    fn pclk2(&self) -> Hz;
    fn tim_pclk2(&self) -> Hz;
}

pub trait Clock<T: ClockTree> {
    fn clock(&self, t: &T) -> Hz;
}

pub struct DynamicClock {
    pub hse_osc: Hz,
    pub lse_osc: Hz,
}

impl ClockTree for DynamicClock {
    #[inline]
    fn hsi16(&self) -> Hz {
        HSI
    }

    #[inline]
    fn lsi(&self) -> Hz { 
        LSI 
    }

    #[inline]
    fn msi(&self) -> Hz {
        match RCC.cr().msirange() {
            U4::B0000 => Some(100_000),
            U4::B0001 => Some(200_000),
            U4::B0010 => Some(400_000),
            U4::B0011 => Some(800_000),
            U4::B0100 => Some(1_000_000),
            U4::B0101 => Some(2_000_000),
            U4::B0110 => Some(4_000_000),
            U4::B0111 => Some(8_000_000),
            U4::B1000 => Some(16_000_000),
            U4::B1001 => Some(24_000_000),
            U4::B1010 => Some(32_000_000),
            U4::B1011 => Some(48_000_000),
            _ => panic!("Invalid Value"),
        }
    }

    #[inline]
    fn lse(&self) -> Hz {
        self.lse_osc
    }

    #[inline]
    fn hse(&self) -> Hz {
        self.hse_osc
    }

    #[inline]
    fn pllclk(&self) -> Hz {
        let cfgr = RCC.pllcfgr();
        let plln = cfgr.plln().into_u32();
        let pllm = 1 + cfgr.pllm().into_u32();
        let pllr = match cfgr.pllr() {
            U2::B00 => 2,
            U2::B01 => 4,
            U2::B10 => 6,
            U2::B11 => 8,
        };
        match cfgr.pllsrc() {
            U2::B00 => None,
            U2::B01 => self.msi(),
            U2::B10 => self.hsi16(),
            U2::B11 => self.hse(),
        }.map(|v| (v * plln / pllm) / pllr)
    }

    #[inline]
    fn sysclk(&self) -> Hz {
        match RCC.cfgr().sws() {
            U2::B00 => self.msi(),
            U2::B01 => self.hsi16(),
            U2::B10 => self.hse(),
            U2::B11 => self.pllclk(),
        }
    }

    #[inline]
    fn hclk(&self) -> Hz {
        let shift = match RCC.cfgr().hpre() {
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
        self.sysclk().map(|v| v >> shift)
    }

    #[inline]
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

    #[inline]
    fn tim_pclk1(&self) -> Hz {
        match RCC.cfgr().ppre1() {
            v if (v as u8) < 0b100  => self.pclk1(),
            _ => self.pclk1().map(|v| v << 1),
        }
    }

    #[inline]
    fn pclk2(&self) -> Hz {
        let shift = match RCC.cfgr().ppre2() {
            v if (v as u8) < 0b100  => 0,
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
            _ => unimplemented!(),
        };
        self.hclk().map(|v| v >> shift)
    }

    #[inline]
    fn tim_pclk2(&self) -> Hz {
        match RCC.cfgr().ppre2() {
            v if (v as u8) < 0b100  => self.pclk2(),
            _ => self.pclk2().map(|v| v << 1),
        }
    }    
}

impl Systick for DynamicClock {
    fn systick(&self) -> Hz {
        self.hclk().map(|v| v >> 3)
    }    
}

impl fmt::Debug for DynamicClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DynamicCLock")?;
        write!(f, " LSI={:?}", self.lsi())?;
        write!(f, " HSI={:?}", self.hsi16())?;
        write!(f, " MSI={:?}", self.msi())?;
        write!(f, " LSE={:?}", self.lse())?;
        write!(f, " HSE={:?}", self.hse())?;
        write!(f, " PLLCLK={:?}", self.pllclk())?;
        write!(f, " SYSCLK={:?}", self.sysclk())?;
        write!(f, " HCLK={:?}", self.hclk())?;
        write!(f, " PCLK1={:?}", self.pclk1())?;
        write!(f, " TIM_PCLK1={:?}", self.tim_pclk1())?;
        write!(f, " PCLK2={:?}", self.pclk2())?;
        write!(f, " TIM_PCLK2={:?}", self.tim_pclk2())?;
        write!(f, "]")?;
        Ok(())
    }
}

impl<T: ClockTree> Clock<T> for Lpuart1 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().lpuart1sel() {
            U2::B00 => t.pclk1(),
            U2::B01 => t.sysclk(),
            U2::B10 => t.hsi16(),
            U2::B11 => t.lse(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Usart2 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().usart2sel() {
            U2::B00 => t.pclk1(),
            U2::B01 => t.sysclk(),
            U2::B10 => t.hsi16(),
            U2::B11 => t.lse(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Usart3 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().usart3sel() {
            U2::B00 => t.pclk1(),
            U2::B01 => t.sysclk(),
            U2::B10 => t.hsi16(),
            U2::B11 => t.lse(),
        }
    }
}


impl<T: ClockTree> Clock<T> for Lptim1 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().lptim1sel() {
            U2::B00 => t.pclk1(),
            U2::B01 => t.lsi(),
            U2::B10 => t.hsi16(),
            U2::B11 => t.lse(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim1 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk2()
    }
}

impl<T: ClockTree> Clock<T> for Tim2 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk1()
    }
}

impl<T: ClockTree> Clock<T> for Tim3 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk1()
    }
}

impl<T: ClockTree> Clock<T> for Tim6 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk1()
    }
}

impl<T: ClockTree> Clock<T> for Tim7 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk1()
    }
}



impl<T: ClockTree> Clock<T> for Tim15 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk2()
    }
}

impl<T: ClockTree> Clock<T> for Tim16 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk2()
    }
}

impl<T: ClockTree> Clock<T> for I2c1 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.pclk1()
    }
}

impl<T: ClockTree> Clock<T> for Spi1 {
    #[inline]
    fn clock(&self, t: &T) -> Hz {
        t.pclk2()
    }
}