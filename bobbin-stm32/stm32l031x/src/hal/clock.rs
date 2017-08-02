pub use bobbin_common::clock::*;

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
            0b000 => Some(65536),
            0b001 => Some(131072),
            0b010 => Some(262144),
            0b011 => Some(524288),
            0b100 => Some(1048000),
            0b101 => Some(2097000),
            0b110 => Some(4194000),
            _ => unimplemented!()
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
            0b0000 => 3,
            0b0001 => 4,
            0b0010 => 6,
            0b0011 => 8,
            0b0100 => 12,
            0b0101 => 16,
            0b0110 => 24,
            0b0111 => 32,
            0b1000 => 48,
            _ => unimplemented!(),
        };
        let pll_div = match RCC.cfgr().plldiv() {
            0b01 => 2,
            0b10 => 3,
            0b11 => 4,
            _ => unimplemented!(),
        };

        match cfgr.pllsrc() {
            0b0 => self.hsi16(),
            0b1 => self.hse(),
            _ => unimplemented!(),
        }.map(|v| v * pll_mul / pll_div)
    }

    fn sysclk(&self) -> Hz {
        match RCC.cfgr().sws() {
            0b00 => self.msi(),
            0b01 => self.hsi16(),
            0b10 => self.hse(),
            0b11 => self.pllclk(),
            _ => unimplemented!(),
        }
    }

    fn hclk(&self) -> Hz {
        let shift = match RCC.cfgr().hpre() {
            0b0000 ... 0b111 => 0,
            0b1000 => 1,
            0b1001 => 2,
            0b1010 => 3,
            0b1011 => 4,
            // NOTE: Divide by 32 is skipped
            0b1100 => 6,
            0b1101 => 7,
            0b1110 => 8,
            0b1111 => 9,
            _ => unimplemented!(),
        };
        self.sysclk().map(|v| v >> shift)
    }

    fn pclk1(&self) -> Hz {
        let shift = match RCC.cfgr().ppre1() {
            0b000 ... 0b011 => 0,
            0b100 => 1,
            0b101 => 2,
            0b110 => 3,
            0b111 => 4,
            _ => unimplemented!(),
        };
        self.hclk().map(|v| v >> shift)
    }

    fn tim_pclk1(&self) -> Hz {
        match RCC.cfgr().ppre1() {
            0b000 ... 0b011 => self.pclk1(),
            _ => self.pclk1().map(|v| v << 1),
        }
    }

    fn pclk2(&self) -> Hz {
        let shift = match RCC.cfgr().ppre2() {
            0b000 ... 0b011 => 0,
            0b100 => 1,
            0b101 => 2,
            0b110 => 3,
            0b111 => 4,
            _ => unimplemented!(),
        };
        self.hclk().map(|v| v >> shift)
    }

    fn tim_pclk2(&self) -> Hz {
        match RCC.cfgr().ppre2() {
            0b000 ... 0b011 => self.pclk2(),
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
            0b00 => t.pclk1(),
            0b01 => t.sysclk(),
            0b10 => t.hsi16(),
            0b11 => t.lse(),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Usart2 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().usart2sel() {
            0b00 => t.pclk1(),
            0b01 => t.sysclk(),
            0b10 => t.hsi16(),
            0b11 => t.lse(),
            _ => unimplemented!(),
        }
    }
}


impl<T: ClockTree> Clock<T> for Lptim {
    fn clock(&self, t: &T) -> Hz {
        match RCC.ccipr().lptim1sel() {
            0b00 => t.pclk1(),
            0b01 => t.lsi(),
            0b10 => t.hsi16(),
            0b11 => t.lse(),
            _ => unimplemented!(),
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