use bobbin_common::bits::*;
use ::chip::rcc::{self, RCC};
use ::chip::flash;
use ::chip::usart::*;
use ::chip::tim_gen::*;
use ::chip::tim_adv::*;
use ::chip::tim_bas::*;
use ::chip::adc::*;
use ::chip::rcc::En;

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
    fn adc12(&self) -> Hz;
    fn adc34(&self) -> Hz;
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
        let prediv = cfgr2.prediv().into_u32() + 1;
        let pllmul = match cfgr.pllmul() {
            U4::B1111 => 16,
            m @ _ => m.into_u32() + 2,
        };
        match cfgr.pllsrc() {
            U2::B00 => self.hsi().map(|v| v >> 1),
            U2::B01 => self.hsi().map(|v| v * pllmul / prediv),
            U2::B10 => self.hse().map(|v| v * pllmul / prediv),
            U2::B11 => panic!("Invalid value for RCC_CFGR[PLLSRC]")
        }
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

    fn systick(&self) -> Hz {
        self.hclk().map(|v| v >> 3)
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

    fn adc12(&self) -> Hz {
        let div = match RCC.cfgr2().adc12pres() as u8 {
            0b00000 ... 0b01111 => return None,
            0b10000 => 1,
            0b10001 => 2,
            0b10010 => 4,
            0b10011 => 6,
            0b10100 => 8,
            0b10101 => 10,
            0b10110 => 12,
            0b10111 => 16,
            0b11000 => 32,
            0b11001 => 64,
            0b11010 => 128,
            0b11011 => 256,
            _ => 256,
        };
        self.pllclk().map(|v| v / div)
    }

    fn adc34(&self) -> Hz {
        let div = match RCC.cfgr2().adc34pres() as u8 {
            0b00000 ... 0b01111 => return None,
            0b10000 => 1,
            0b10001 => 2,
            0b10010 => 4,
            0b10011 => 6,
            0b10100 => 8,
            0b10101 => 10,
            0b10110 => 12,
            0b10111 => 16,
            0b11000 => 32,
            0b11001 => 64,
            0b11010 => 128,
            0b11011 => 256,
            _ => 256,
        };
        self.pllclk().map(|v| v / div)
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
        write!(f, " ADC12={:?}", self.adc12())?;
        write!(f, " ADC34={:?}", self.adc34())?;
        write!(f, "]")?;
        Ok(())
    }
}

impl<T: ClockTree> Clock<T> for Usart1 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().usart1sw() {
                U2::B00 => t.pclk2(),
                U2::B01 => t.sysclk(),
                U2::B10 => t.lse(),
                U2::B11 => t.hsi(),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Usart2 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().usart2sw() {
                U2::B00 => t.pclk1(),
                U2::B01 => t.sysclk(),
                U2::B10 => t.lse(),
                U2::B11 => t.hsi(),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Usart3 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().usart3sw() {
                U2::B00 => t.pclk1(),
                U2::B01 => t.sysclk(),
                U2::B10 => t.lse(),
                U2::B11 => t.hsi(),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Uart4 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().uart4sw() {
                U2::B00 => t.pclk1(),
                U2::B01 => t.sysclk(),
                U2::B10 => t.lse(),
                U2::B11 => t.hsi(),
            }
        } else {
            None
        }
    }
}


impl<T: ClockTree> Clock<T> for Uart5 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().uart5sw() {
                U2::B00 => t.pclk1(),
                U2::B01 => t.sysclk(),
                U2::B10 => t.lse(),
                U2::B11 => t.hsi(),
            }
        } else {
            None
        }
    }
}

// NOTE: Clock Tree Diagram differs from RCC_CFGR3 description,
// using clock tree as reference

impl<T: ClockTree> Clock<T> for Tim1 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().tim1sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}


impl<T: ClockTree> Clock<T> for Tim2 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {        
            match RCC.cfgr3().tim2sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}


impl<T: ClockTree> Clock<T> for Tim3 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().tim34sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim4 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().tim34sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim6 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            t.tim_pclk1()
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim7 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            t.tim_pclk1()
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim8 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().tim8sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim15 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().tim15sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim16 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().tim16sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim17 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {
            match RCC.cfgr3().tim17sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Tim20 {
    fn clock(&self, t: &T) -> Hz {
        if self.en() != 0 {        
            match RCC.cfgr3().tim20sw() {
                U1::B0 => t.tim_pclk2(),
                U1::B1 => t.pllclk().map(|v| v << 1),
            }
        } else {
            None
        }
    }
}

impl<T: ClockTree> Clock<T> for Adc1 {
    fn clock(&self, t: &T) -> Hz {
        t.adc12()
    }
}

impl<T: ClockTree> Clock<T> for Adc2 {
    fn clock(&self, t: &T) -> Hz {
        t.adc12()
    }
}
impl<T: ClockTree> Clock<T> for Adc3 {
    fn clock(&self, t: &T) -> Hz {
        t.adc34()
    }
}
impl<T: ClockTree> Clock<T> for Adc4 {
    fn clock(&self, t: &T) -> Hz {
        t.adc34()
    }
}