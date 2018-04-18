pub use rcc::*;
use bobbin_bits::*;
use bobbin_mcu::clock::ClockSource;

#[repr(u8)]
pub enum UsartClock {
    Pclk = 0b00,
    Sysclk = 0b01,
    Hsi16 = 0b10,
    Lse = 0b11,
}

impl From<U2> for UsartClock {
    fn from(other: U2) -> UsartClock {
        match other {
            U2::B00 => UsartClock::Pclk,
            U2::B01 => UsartClock::Sysclk,
            U2::B10 => UsartClock::Hsi16,
            U2::B11 => UsartClock::Lse,
        }
    }
}
impl From<UsartClock> for U2 {
    fn from(other: UsartClock) -> U2 {
        match other {
            UsartClock::Pclk => U2::B00,
            UsartClock::Sysclk => U2::B01,
            UsartClock::Hsi16 => U2::B10,
            UsartClock::Lse => U2::B11,
        }
    }
}

macro_rules! impl_usart_clocksource {
    ($periph:path, $getter:ident, $setter:ident) => {
        impl ClockSource<UsartClock> for $periph {
            fn clock_source(&self) -> UsartClock {
                ::rcc::RCC.ccipr().$getter().into()
            }
            fn set_clock_source(&self, clk: UsartClock) -> &Self {
                ::rcc::RCC.with_ccipr(|r| r.$setter(clk));
                self
            }
        }        
    };
}

impl_usart_clocksource!(::usart::Usart1, usart1sel, set_usart1sel);
impl_usart_clocksource!(::usart::Usart2, usart2sel, set_usart2sel);
impl_usart_clocksource!(::usart::Usart3, usart3sel, set_usart3sel);
impl_usart_clocksource!(::lpuart::Lpuart1, lpuart1sel, set_lpuart1sel);

#[repr(u8)]
pub enum I2cClock {
    Pclk = 0b00,
    Sysclk = 0b01,
    Hsi16 = 0b10,
}

impl From<U2> for I2cClock {
    fn from(other: U2) -> I2cClock {
        match other {
            U2::B00 => I2cClock::Pclk,
            U2::B01 => I2cClock::Sysclk,
            U2::B10 => I2cClock::Hsi16,
            U2::B11 => panic!("Invalid Clock Source")
        }
    }
}
impl From<I2cClock> for U2 {
    fn from(other: I2cClock) -> U2 {
        match other {
            I2cClock::Pclk => U2::B00,
            I2cClock::Sysclk => U2::B01,
            I2cClock::Hsi16 => U2::B10,
        }
    }
}

macro_rules! impl_i2c_clocksource {
    ($periph:path, $getter:ident, $setter:ident) => {
        impl ClockSource<I2cClock> for $periph {
            fn clock_source(&self) -> I2cClock {
                ::rcc::RCC.ccipr().$getter().into()
            }
            fn set_clock_source(&self, clk: I2cClock) -> &Self {
                ::rcc::RCC.with_ccipr(|r| r.$setter(clk));
                self
            }
        }        
    };
}

impl_i2c_clocksource!(::i2c::I2c1, i2c1sel, set_i2c1sel);
impl_i2c_clocksource!(::i2c::I2c2, i2c2sel, set_i2c2sel);
impl_i2c_clocksource!(::i2c::I2c3, i2c3sel, set_i2c3sel);

#[repr(u8)]
pub enum LptimClock {
    Pclk = 0b00,
    Lsi = 0b01,
    Hsi16 = 0b10,
    Lse = 0b11,
}

impl From<U2> for LptimClock {
    fn from(other: U2) -> LptimClock {
        match other {
            U2::B00 => LptimClock::Pclk,
            U2::B01 => LptimClock::Lsi,
            U2::B10 => LptimClock::Hsi16,
            U2::B11 => LptimClock::Lse,
        }
    }
}
impl From<LptimClock> for U2 {
    fn from(other: LptimClock) -> U2 {
        match other {
            LptimClock::Pclk => U2::B00,
            LptimClock::Lsi => U2::B01,
            LptimClock::Hsi16 => U2::B10,
            LptimClock::Lse => U2::B11,
        }
    }
}
macro_rules! impl_lptim_clocksource {
    ($periph:path, $getter:ident, $setter:ident) => {
        impl ClockSource<LptimClock> for $periph {
            fn clock_source(&self) -> LptimClock {
                ::rcc::RCC.ccipr().$getter().into()
            }
            fn set_clock_source(&self, clk: LptimClock) -> &Self {
                ::rcc::RCC.with_ccipr(|r| r.$setter(clk));
                self
            }
        }        
    };
}

impl_lptim_clocksource!(::lptim::Lptim1, lptim1sel, set_lptim1sel);
impl_lptim_clocksource!(::lptim::Lptim2, lptim2sel, set_lptim2sel);

