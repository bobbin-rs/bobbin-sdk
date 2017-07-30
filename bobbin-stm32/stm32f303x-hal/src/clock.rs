use ::chip::rcc::{self, RCC};
use ::chip::flash;
use ::chip::usart::*;
use ::chip::tim_gen::*;
use ::chip::tim_adv::*;
use ::chip::tim_bas::*;

use core::fmt;

//use ::chip::pwr;

// Clock Configuration on Reset
//   System Clock = HSI = 8Mhz

// PLL Mode with 8Mhz External Oscillator
//   72Mhz System Clock
//   72Mhz AHB Clock
//   36Mhz APB1 Clock
//   72Mhz APB2 Clock
//   9Mhz SysTick clock

pub fn enable_pll_external_mode() {
    let rcc = rcc::RCC;
    let flash = flash::FLASH;
    //let mut pwr = pwr::PWR;

    // Configure flash settings.
    // Prefetch Buffer Enabled + Two Wait States
    flash.with_acr(|r| r.set_prftbe(1).set_latency(0b010));

    // Configure Prescalers

    // AHB (HCLK)  = SYSCLK
    // APB1 = HCLK / 2
    // APB2 = HCLK
    rcc.with_cfgr(|r| r.set_hpre(0b000).set_ppre1(0b100).set_ppre2(0b000));

    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {}

    // Select HSI as SYSCLK source. 
    rcc.with_cfgr(|r| r.set_sw(0b00));

    // Enable external high-speed clock 8MHz.
    rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(1));
    
    // Wait for HSE Ready
    while rcc.cr().hserdy() == 0 {}

    // Configure PLL
    // PLLSRC = HSE
    // PREDIV = 1
    // MUL = 9

    rcc.with_cfgr(|r| r.set_pllsrc(0b10).set_pllmul(0b111));

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {}
    
    // Select PLL as SYSCLK source.        
    rcc.with_cfgr(|r| r.set_sw(0b10));
    
    // Wait for PLL to be selected

    while rcc.cfgr().sws() != 0b10 {}
    
    // Enabled SYCLK output on MCO pin
    //rcc.with_cfgr(|r| r.set_mco(0b101));
    
    // Disable internal high-speed oscillator.        
    rcc.with_cr(|r| r.set_hsion(0));
}

pub fn hsi_enabled() -> bool {
    rcc::RCC.cr().hsion() != 0
}

pub fn set_hsi_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    rcc::RCC.with_cr(|r| r.set_hsion(value));
    if value == 1 {
        while rcc::RCC.cr().hsirdy() == 0 {}
    }
}

pub const HSI: Hz = Some(8_000_000);
pub const LSI: Hz = Some(49_000);

pub type Hz = Option<u32>;

pub trait ClockTree {
    fn hsi(&self) -> Hz { HSI }
    fn lsi(&self) -> Hz { LSI }
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
    fn hse(&self) -> Hz {
        self.hse_osc
    }

    fn lse(&self) -> Hz {
        self.lse_osc
    }

    fn pllclk(&self) -> Hz {
        let cfgr = RCC.cfgr();
        let cfgr2 = RCC.cfgr2();
        let prediv = cfgr2.prediv() + 1;
        let pllmul = match cfgr.pllmul() {
            0b1111 => 16,
            m @ _ => m + 2,
        };
        match cfgr.pllsrc() {
            0b00 => self.hsi().map(|v| v >> 1),
            0b01 => self.hsi().map(|v| v * pllmul / prediv),
            0b10 => self.hse().map(|v| v * pllmul / prediv),
            _ => unimplemented!(),
        }
    }

    fn sysclk(&self) -> Hz {
        match RCC.cfgr().sws() {
            0b00 => self.hsi(),
            0b01 => self.hse(),
            0b10 => self.pllclk(),
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

    fn systick(&self) -> Hz {
        self.hclk().map(|v| v >> 3)
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

impl fmt::Debug for DynamicClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DynamicCLock")?;
        write!(f, " LSI={:?}", self.lsi())?;
        write!(f, " HSI={:?}", self.hsi())?;
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

impl<T: ClockTree> Clock<T> for Usart1 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().usart1sw() {
            0b00 => t.pclk2(),
            0b01 => t.sysclk(),
            0b10 => t.lse(),
            0b11 => t.hsi(),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Usart2 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().usart2sw() {
            0b00 => t.pclk1(),
            0b01 => t.sysclk(),
            0b10 => t.lse(),
            0b11 => t.hsi(),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Usart3 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().usart3sw() {
            0b00 => t.pclk1(),
            0b01 => t.sysclk(),
            0b10 => t.lse(),
            0b11 => t.hsi(),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Uart4 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().uart4sw() {
            0b00 => t.pclk1(),
            0b01 => t.sysclk(),
            0b10 => t.lse(),
            0b11 => t.hsi(),
            _ => unimplemented!(),
        }
    }
}


impl<T: ClockTree> Clock<T> for Uart5 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().uart5sw() {
            0b00 => t.pclk1(),
            0b01 => t.sysclk(),
            0b10 => t.lse(),
            0b11 => t.hsi(),
            _ => unimplemented!(),
        }
    }
}

// NOTE: Clock Tree Diagram differs from RCC_CFGR3 description,
// using clock tree as reference

impl<T: ClockTree> Clock<T> for Tim1 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim1sw() {
            0b00 => t.tim_pclk2(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}


impl<T: ClockTree> Clock<T> for Tim2 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim2sw() {
            0b00 => t.tim_pclk1(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}


impl<T: ClockTree> Clock<T> for Tim3 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim34sw() {
            0b00 => t.tim_pclk1(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim4 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim34sw() {
            0b00 => t.tim_pclk1(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim6 {
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk1()
    }
}

impl<T: ClockTree> Clock<T> for Tim7 {
    fn clock(&self, t: &T) -> Hz {
        t.tim_pclk1()
    }
}

impl<T: ClockTree> Clock<T> for Tim8 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim8sw() {
            0b00 => t.tim_pclk2(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim15 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim15sw() {
            0b00 => t.tim_pclk2(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim16 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim16sw() {
            0b00 => t.tim_pclk2(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim17 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim17sw() {
            0b00 => t.tim_pclk2(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim20 {
    fn clock(&self, t: &T) -> Hz {
        match RCC.cfgr3().tim20sw() {
            0b00 => t.tim_pclk2(),
            0b01 => t.pllclk().map(|v| v << 1),
            _ => unimplemented!(),
        }
    }
}