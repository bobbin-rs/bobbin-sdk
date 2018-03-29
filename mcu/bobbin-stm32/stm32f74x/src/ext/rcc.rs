pub use periph::rcc::*;
pub use bobbin_common::clock::ClockSource;
pub use bobbin_common::bits::U2;

#[repr(u8)]
pub enum DedicatedClock {
    Pclk = 0b00,
    Sysclk = 0b01,
    Hsi = 0b10,
    Lse = 0b11,
}

impl From<U2> for DedicatedClock {
    fn from(other: U2) -> DedicatedClock {
        match other {
            U2::B00 => DedicatedClock::Pclk,
            U2::B01 => DedicatedClock::Sysclk,
            U2::B10 => DedicatedClock::Hsi,
            U2::B11 => DedicatedClock::Lse,
        }
    }
}
impl From<DedicatedClock> for U2 {
    fn from(other: DedicatedClock) -> U2 {
        match other {
            DedicatedClock::Pclk => U2::B00,
            DedicatedClock::Sysclk => U2::B01,
            DedicatedClock::Hsi => U2::B10,
            DedicatedClock::Lse => U2::B11,
        }
    }
}

macro_rules! impl_clocksource {
    ($periph:path, $getter:ident, $setter:ident) => {
        impl ClockSource<DedicatedClock> for $periph {
            fn clock_source(&self) -> DedicatedClock {
                ::rcc::RCC.dckcfgr2().$getter().into()
            }
            fn set_clock_source(&self, clk: DedicatedClock) -> &Self {
                ::rcc::RCC.with_dckcfgr2(|r| r.$setter(clk));
                self
            }
        }        
    };
}

impl_clocksource!(::usart::Usart1, usart1sel, set_usart1sel);
impl_clocksource!(::usart::Usart2, usart2sel, set_usart2sel);
impl_clocksource!(::usart::Usart3, usart3sel, set_usart3sel);
impl_clocksource!(::usart::Uart4, uart4sel, set_uart4sel);
impl_clocksource!(::usart::Uart5, uart5sel, set_uart5sel);
impl_clocksource!(::usart::Usart6, usart6sel, set_usart6sel);
impl_clocksource!(::usart::Uart7, uart7sel, set_uart7sel);
impl_clocksource!(::usart::Uart8, uart8sel, set_uart8sel);

impl_clocksource!(::i2c::I2c1, i2c1sel, set_i2c1sel);
impl_clocksource!(::i2c::I2c2, i2c2sel, set_i2c2sel);
impl_clocksource!(::i2c::I2c3, i2c3sel, set_i2c3sel);
impl_clocksource!(::i2c::I2c4, i2c4sel, set_i2c4sel);

