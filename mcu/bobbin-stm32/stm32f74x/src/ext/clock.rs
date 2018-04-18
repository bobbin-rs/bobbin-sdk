// use ::clock::*;
use ext::systick::SystickHz;
use bobbin_mcu::hz::Hz;
use bobbin_mcu::clock::{Clock, ClockSource};
use bobbin_bits::*;

use ext::rcc::DedicatedClock;

use rcc::RCC;
use clock::ClockProvider;

#[derive(Default)]
pub struct DynamicClock<OSC: Clock, OSC32: Clock>(OSC, OSC32);

impl<OSC: Clock, OSC32: Clock> DynamicClock<OSC, OSC32> {
    fn pllq(&self) -> Hz {
        let cfgr = RCC.pllcfgr();
        let vco_in = match cfgr.pllsrc() {
            U1::B0 => self.hsi(),
            U1::B1 => self.hse(),
        };
        let vco = (vco_in / cfgr.pllm().into_u32()).normalized() * cfgr.plln().into_u32();
        (vco / cfgr.pllq().into_u32()).normalized()
    }    
}

macro_rules! impl_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id(&self) -> Hz {
            match $periph.clock_source() {
                DedicatedClock::Pclk => self.$default(),
                DedicatedClock::Sysclk => self.sysclk(),
                DedicatedClock::Hsi => self.hsi(),
                DedicatedClock::Lse => self.lse(),
            }
        }        
    };
}

impl<OSC: Clock, OSC32: Clock> ClockProvider for DynamicClock<OSC, OSC32> {
    type Osc = OSC;
    type Osc32 = OSC32;
    fn pllclk(&self) -> Hz {
        let cfgr = RCC.pllcfgr();
        let vco_in = match cfgr.pllsrc() {
            U1::B0 => self.lsi(),
            U1::B1 => self.hse(),
        };
        let vco = (vco_in / cfgr.pllm().into_u32()).normalized();
        (vco  * cfgr.plln().into_u32() / (2 * (cfgr.pllp().into_u32() + 1))).normalized()
    }



    fn pll48clk(&self) -> Hz {
        match RCC.dckcfgr2().ck48msel() {
            U1::B0 => self.pllq(),
            U1::B1 => unimplemented!(),
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
        self.sysclk() >> shift
    }    

    fn systick(&self) -> Hz {
        self.hclk() >> 3
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
        self.hclk() >> shift
    }

    fn tim_pclk1(&self) -> Hz {
        match RCC.cfgr().ppre1() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => self.pclk1(),
            _ => self.pclk1() << 1,        
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
        self.hclk() >> shift
    }

    fn tim_pclk2(&self) -> Hz {
        match RCC.cfgr().ppre2() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => self.pclk2(),
            _ => self.pclk2() << 1,
        }
    }    

    impl_clock_source!(::usart::USART1, usart1, pclk2);
    impl_clock_source!(::usart::USART2, usart2, pclk1);
    impl_clock_source!(::usart::USART3, usart3, pclk1);
    impl_clock_source!(::usart::UART4, uart4, pclk1);
    impl_clock_source!(::usart::UART5, uart5, pclk1);
    impl_clock_source!(::usart::USART6, usart6, pclk2);
    impl_clock_source!(::usart::UART7, uart7, pclk1);
    impl_clock_source!(::usart::UART8, uart8, pclk1);

    impl_clock_source!(::i2c::I2C1, i2c1, pclk1);
    impl_clock_source!(::i2c::I2C2, i2c2, pclk1);
    impl_clock_source!(::i2c::I2C3, i2c3, pclk1);
    impl_clock_source!(::i2c::I2C4, i2c4, pclk1);
    
}

impl<OSC: Clock, OSC32: Clock> SystickHz for DynamicClock<OSC, OSC32> {
    fn systick_hz(&self) -> Hz {
        self.systick()
    }
}