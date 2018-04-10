pub use mcu::clock::*;
pub use mcu::systick_ext::SystickHz;
use mcu::rcc::*;
use mcu::{rcc, tim_adv, tim_gen, i2c, usart};
use ::common::bits::*;
pub use ::common::clock::ClockSource;

pub fn init() {
}

pub type Clk = DynamicClock<Osc8m, Osc32k>;
pub type Tree = ClockTree<Clk>;


pub struct Osc8m {}
impl Clock for Osc8m {
    fn hz() -> Hz { Hz::from_num(8_000_000) }
}

pub struct Osc32k {}
impl Clock for Osc32k {
    fn hz() -> Hz { Hz::from_num(32768) }
}

pub struct DynamicClock<OSC: Clock, OSC32: Clock>(OSC, OSC32);

impl<OSC: Clock, OSC32: Clock> Clocks for DynamicClock<OSC, OSC32> {
    type Osc = OSC;
    type Osc32 = OSC32;

    fn pllclk() -> Hz {
        let cfgr = RCC.cfgr();
        let cfgr2 = RCC.cfgr2();
        let prediv = cfgr2.prediv().into_u32() + 1;
        let pllmul = match cfgr.pllmul() {
            U4::B1111 => 16,
            m @ _ => m.into_u32() + 2,
        };
        match cfgr.pllsrc() {
            U2::B00 => Self::hsi() >> 1,
            U2::B01 => Self::hsi() * pllmul / prediv,
            U2::B10 => Self::hse()* pllmul / prediv,
            U2::B11 => panic!("Invalid value for RCC_CFGR[PLLSRC]")
        }
    }

    fn sysclk() -> Hz {
        match RCC.cfgr().sws() {
            U2::B00 => Self::hsi(),
            U2::B01 => Self::hse(),
            U2::B10 => Self::pllclk(),
            U2::B11 => panic!("Invalid value for RCC_CFGR[SWS]"),
        }
    }

    fn hclk() -> Hz {
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
        Self::sysclk() >> shift
    }

    fn systick() -> Hz {
        Self::hclk() >> 3
    }

    fn pclk1() -> Hz {
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
        Self::hclk() >> shift
    }

    fn tim_pclk1() -> Hz {
        match RCC.cfgr().ppre1() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => Self::pclk1(),
            _ => Self::pclk1() << 1,
        }
    }

    fn pclk2() -> Hz {
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
        Self::hclk() >> shift
    }

    fn tim_pclk2() -> Hz {
        match RCC.cfgr().ppre2() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => Self::pclk2(),
            _ => Self::pclk2() << 1,
        }
    }  

    fn adc12() -> Hz {
        let div = match RCC.cfgr2().adc12pres() as u8 {
            0b00000 ... 0b01111 => return Hz::from_num(0),
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
        Self::pllclk() / div
    }

    fn adc34() -> Hz {
        let div = match RCC.cfgr2().adc34pres() as u8 {
            0b00000 ... 0b01111 => return Hz::from_num(0),
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
        Self::pllclk() / div
    }        
}

#[repr(u8)]
pub enum UsartClock {
    Pclk = 0b00,
    Sysclk = 0b01,
    Lse = 0b10,
    Hsi = 0b11,
}

impl From<U2> for UsartClock {
    fn from(other: U2) -> UsartClock {
        match other {
            U2::B00 => UsartClock::Pclk,
            U2::B01 => UsartClock::Sysclk,
            U2::B10 => UsartClock::Lse,
            U2::B11 => UsartClock::Hsi,
        }
    }
}
impl From<UsartClock> for U2 {
    fn from(other: UsartClock) -> U2 {
        match other {
            UsartClock::Pclk => U2::B00,
            UsartClock::Sysclk => U2::B01,
            UsartClock::Lse => U2::B10,
            UsartClock::Hsi => U2::B11,
        }
    }
}

macro_rules! impl_usart_clocksource {
    ($periph:path, $getter:ident, $setter:ident) => {
        impl ClockSource<UsartClock> for $periph {
            fn clock_source(&self) -> UsartClock {
                rcc::RCC.cfgr3().$getter().into()
            }
            fn set_clock_source(&self, clk: UsartClock) -> &Self {
                rcc::RCC.with_cfgr3(|r| r.$setter(clk));
                self
            }
        }        
    };
}

impl_usart_clocksource!(usart::Usart1, usart1sw, set_usart1sw);
impl_usart_clocksource!(usart::Usart2, usart2sw, set_usart2sw);
impl_usart_clocksource!(usart::Usart3, usart3sw, set_usart3sw);
impl_usart_clocksource!(usart::Uart4, uart4sw, set_uart4sw);
impl_usart_clocksource!(usart::Uart5, uart5sw, set_uart5sw);

#[repr(u8)]
pub enum I2cClock {
    Hsi = 0b0,
    Sysclk = 0b1,
}

impl From<U1> for I2cClock {
    fn from(other: U1) -> I2cClock {
        match other {
            U1::B0 => I2cClock::Hsi,
            U1::B1 => I2cClock::Sysclk,
        }
    }
}
impl From<I2cClock> for U1 {
    fn from(other: I2cClock) -> U1 {
        match other {
            I2cClock::Hsi => U1::B0,
            I2cClock::Sysclk => U1::B1,
        }
    }
}


macro_rules! impl_i2c_clocksource {
    ($periph:path, $getter:ident, $setter:ident) => {
        impl ClockSource<I2cClock> for $periph {
            fn clock_source(&self) -> I2cClock {
                rcc::RCC.cfgr3().$getter().into()
            }
            fn set_clock_source(&self, clk: I2cClock) -> &Self {
                rcc::RCC.with_cfgr3(|r| r.$setter(clk));
                self
            }
        }        
    };
}

impl_i2c_clocksource!(i2c::I2c1, i2c1sw, set_i2c1sw);
impl_i2c_clocksource!(i2c::I2c2, i2c2sw, set_i2c2sw);
impl_i2c_clocksource!(i2c::I2c3, i2c3sw, set_i2c3sw);

#[repr(u8)]
pub enum TimClock {
    Pclk = 0b0,
    Pll = 0b1,
}


impl From<U1> for TimClock {
    fn from(other: U1) -> TimClock {
        match other {
            U1::B0 => TimClock::Pclk,
            U1::B1 => TimClock::Pll,
        }
    }
}
impl From<TimClock> for U1 {
    fn from(other: TimClock) -> U1 {
        match other {
            TimClock::Pclk => U1::B0,
            TimClock::Pll => U1::B1,
        }
    }
}


macro_rules! impl_tim_clocksource {
    ($periph:path, $getter:ident, $setter:ident) => {
        impl ClockSource<TimClock> for $periph {
            fn clock_source(&self) -> TimClock {
                rcc::RCC.cfgr3().$getter().into()
            }
            fn set_clock_source(&self, clk: TimClock) -> &Self {
                rcc::RCC.with_cfgr3(|r| r.$setter(clk));
                self
            }
        }        
    };
}

impl_tim_clocksource!(tim_adv::Tim1, tim1sw, set_tim1sw);
impl_tim_clocksource!(tim_gen::Tim2, tim2sw, set_tim2sw);
impl_tim_clocksource!(tim_gen::Tim3, tim34sw, set_tim34sw);
impl_tim_clocksource!(tim_gen::Tim4, tim34sw, set_tim34sw);
impl_tim_clocksource!(tim_adv::Tim8, tim8sw, set_tim8sw);
impl_tim_clocksource!(tim_gen::Tim15, tim15sw, set_tim15sw);
impl_tim_clocksource!(tim_gen::Tim16, tim16sw, set_tim16sw);
impl_tim_clocksource!(tim_adv::Tim20, tim20sw, set_tim20sw);
