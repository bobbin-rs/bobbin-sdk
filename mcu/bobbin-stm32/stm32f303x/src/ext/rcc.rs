pub use periph::rcc::*;
pub use bobbin_common::clock::ClockSource;
use mcu::{rcc, tim_adv, tim_gen, i2c, usart};
use bobbin_common::bits::*;


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
