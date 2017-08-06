use bobbin_common::bits::*;

use ::chip::rcc::RCC;
// use ::chip::pwr::PWR;
use ::chip::flash::FLASH;
use ::chip::usart::*;
use ::chip::lpuart::*;
use ::chip::lptim::*;
use ::chip::tim_gen::*;

use core::fmt;

// Main System Clock = 32MHz
// APB2 = 32MHz
// APB1 = 32MHz
// AHB = 32MHz

pub fn init_pll() {
    // (1) Set one wait state in Latency bit of FLASH_ACR 
    FLASH.with_acr(|r| r.set_latency(1));

    // (2) Check the latency is set
    while FLASH.acr().latency() == 0 {}

    // (3) Switch the clock on HSI16/4 and disable PLL

    RCC.with_cr(|r| r.set_pllon(0).set_hsi16divf(0).set_hsi16on(1));

    // Wait for HSI16 Ready Flag
    while RCC.cr().hsi16rdyf() == 0 {}

    // (4) Set PLLMUL to 16 to get 32MHz on CPU clock, PLLDIV/2

    RCC.with_cfgr(|r| r.set_pllmul(0b0010).set_plldiv(0b10));

    // (5) Enable and switch on PLL 

    RCC.with_cr(|r| r.set_pllon(1));

    // Wait for PLL Ready
    while RCC.cr().pllrdy() == 0 {}

    // Switch to PLL
    RCC.with_cfgr(|r| r.set_sw(0b11));

    // Wait for system clock to use PLL
    while RCC.cfgr().sws() != 0b11 {}
}

pub const HSI16: Hz = Some(16_000_000);
pub const LSI: Hz = Some(37_000);

pub type Hz = Option<u32>;

pub trait ClockTree {
    fn hsi16(&self) -> Hz;
    fn lsi(&self) -> Hz;
    fn msi(&self) -> Hz;
    fn hse(&self) -> Hz;
    fn lse(&self) -> Hz;
    fn pllclk(&self) -> Hz;
    fn sysclk(&self) -> Hz;
    fn hclk(&self) -> Hz;
    fn systick(&self) -> Hz;
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
    fn hsi16(&self) -> Hz {
        if RCC.cr().hsi16divf() != 0 {
            HSI16.map(|v| v >> 2)
        } else {
            HSI16
        }
    }

    fn lsi(&self) -> Hz { 
        LSI 
    }

    fn msi(&self) -> Hz {
        match RCC.icscr().msirange() {
            B3::B000 => Some(65536),
            B3::B001 => Some(131072),
            B3::B010 => Some(262144),
            B3::B011 => Some(524288),
            B3::B100 => Some(1048000),
            B3::B101 => Some(2097000),
            B3::B110 => Some(4194000),
            _ => panic!("Invalid Value"),
        }
    }

    fn lse(&self) -> Hz {
        self.lse_osc
    }

    fn hse(&self) -> Hz {
        self.hse_osc
    }

    fn pllclk(&self) -> Hz {
        let cfgr = RCC.cfgr();

        let pll_mul = match RCC.cfgr().pllmul() {
            B4::B0000 => 3,
            B4::B0001 => 4,
            B4::B0010 => 6,
            B4::B0011 => 8,
            B4::B0100 => 12,
            B4::B0101 => 16,
            B4::B0110 => 24,
            B4::B0111 => 32,
            B4::B1000 => 48,
            _ => panic!("Invalid Value"),
        };
        let pll_div = match RCC.cfgr().plldiv() {
            B2::B01 => 2,
            B2::B10 => 3,
            B2::B11 => 4,
            _ => panic!("Invalid Value"),
        };

        match cfgr.pllsrc() {
            B1::B0 => self.hsi16(),
            B1::B1 => self.hse(),
        }.map(|v| v * pll_mul / pll_div)
    }

    fn sysclk(&self) -> Hz {
        match RCC.cfgr().sws() {
            B2::B00 => self.msi(),
            B2::B01 => self.hsi16(),
            B2::B10 => self.hse(),
            B2::B11 => self.pllclk(),
        }
    }

    fn hclk(&self) -> Hz {
        let shift = match RCC.cfgr().hpre() {
            B4::B0000 => 0,
            B4::B0001 => 0,
            B4::B0010 => 0,
            B4::B0011 => 0,
            B4::B0100 => 0,
            B4::B0101 => 0,
            B4::B0110 => 0,
            B4::B0111 => 0,
            B4::B1000 => 1,
            B4::B1001 => 2,
            B4::B1010 => 3,
            B4::B1011 => 4,
            // NOTE: Divide by 32 is skipped
            B4::B1100 => 6,
            B4::B1101 => 7,
            B4::B1110 => 8,
            B4::B1111 => 9,
        };
        self.sysclk().map(|v| v >> shift)
    }

    fn systick(&self) -> Hz {
        self.hclk().map(|v| v >> 3)
    }

    fn pclk1(&self) -> Hz {
        let shift = match RCC.cfgr().ppre1() {
            B3::B000 => 0,
            B3::B001 => 0,
            B3::B010 => 0,
            B3::B011 => 0,
            B3::B100 => 1,
            B3::B101 => 2,
            B3::B110 => 3,
            B3::B111 => 4,
        };
        self.hclk().map(|v| v >> shift)
    }

    fn tim_pclk1(&self) -> Hz {
        match RCC.cfgr().ppre1() {
            v if (v as u8) < 0b100  => self.pclk1(),
            _ => self.pclk1().map(|v| v << 1),
        }
    }

    fn pclk2(&self) -> Hz {
        let shift = match RCC.cfgr().ppre2() {
            v if (v as u8) < 0b100  => 0,
            B3::B100 => 1,
            B3::B101 => 2,
            B3::B110 => 3,
            B3::B111 => 4,
            _ => unimplemented!(),
        };
        self.hclk().map(|v| v >> shift)
    }

    fn tim_pclk2(&self) -> Hz {
        match RCC.cfgr().ppre2() {
            v if (v as u8) < 0b100  => self.pclk2(),
            _ => self.pclk2().map(|v| v << 1),
        }
    }    
}

impl fmt::Debug for DynamicClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DynamicCLock")?;
        write!(f, " LSI={:?}", self.lsi())?;
        write!(f, " HSI16={:?}", self.hsi16())?;
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
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().lpuart1sel() {
            B2::B00 => t.pclk1(),
            B2::B01 => t.sysclk(),
            B2::B10 => t.hsi16(),
            B2::B11 => t.lse(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Usart2 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().usart2sel() {
            B2::B00 => t.pclk1(),
            B2::B01 => t.sysclk(),
            B2::B10 => t.hsi16(),
            B2::B11 => t.lse(),
        }
    }
}


impl<T: ClockTree> Clock<T> for Lptim {
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().lptim1sel() {
            B2::B00 => t.pclk1(),
            B2::B01 => t.lsi(),
            B2::B10 => t.hsi16(),
            B2::B11 => t.lse(),
        }
    }
}


impl<T: ClockTree> Clock<T> for Tim2 {
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk1()
    }
}

impl<T: ClockTree> Clock<T> for Tim21 {
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk2()
    }
}

impl<T: ClockTree> Clock<T> for Tim22 {
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk2()
    }
}