pub use ::clock::*;

use mcu::rcc::*;
use bobbin_common::bits::*;

macro_rules! impl_usart_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id() -> Hz {
            match $periph.clock_source() {
                UsartClock::Pclk => Self::$default(),
                UsartClock::Sysclk => Self::sysclk(),
                UsartClock::Hsi16 => Self::hsi16(),
                UsartClock::Lse => Self::lse(),
            }
        }        
    };
}


macro_rules! impl_i2c_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id() -> Hz {
            match $periph.clock_source() {
                I2cClock::Pclk => Self::$default(),
                I2cClock::Sysclk => Self::sysclk(),
                I2cClock::Hsi16 => Self::hsi16(),
            }
        }        
    };
}

macro_rules! impl_lptim_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id() -> Hz {
            match $periph.clock_source() {
                LptimClock::Pclk => Self::$default(),
                LptimClock::Lsi => Self::lsi(),
                LptimClock::Hsi16 => Self::hsi16(),
                LptimClock::Lse => Self::lse(),
            }
        }        
    };
}


pub struct DynamicClock<OSC: Clock, OSC32: Clock>(OSC, OSC32);

impl<OSC: Clock, OSC32: Clock> DynamicClock<OSC, OSC32> {
    fn msi() -> Hz {
        Hz::from(match RCC.cr().msirange() {
            U4::B0000 => 100_000,
            U4::B0001 => 200_000,
            U4::B0010 => 400_000,
            U4::B0011 => 800_000,
            U4::B0100 => 1_000_000,
            U4::B0101 => 2_000_000,
            U4::B0110 => 4_000_000,
            U4::B0111 => 8_000_000,
            U4::B1000 => 16_000_000,
            U4::B1001 => 24_000_000,
            U4::B1010 => 32_000_000,
            U4::B1011 => 48_000_000,
            _ => 0,
        })
    }    
}

impl<OSC: Clock, OSC32: Clock> Clocks for DynamicClock<OSC, OSC32> {
    type Osc = OSC;
    type Osc32 = OSC32;



    fn pllclk() -> Hz {
        let cfgr = RCC.pllcfgr();
        let plln = cfgr.plln().into_u32();
        let pllm = 1 + cfgr.pllm().into_u32();
        let pllr = match cfgr.pllr() {
            U2::B00 => 2,
            U2::B01 => 4,
            U2::B10 => 6,
            U2::B11 => 8,
        };
        let v = match cfgr.pllsrc() {
            U2::B00 => Hz::from(0),
            U2::B01 => Self::msi(),
            U2::B10 => Self::hsi16(),
            U2::B11 => Self::hse(),
        };
        (v * plln / pllm) / pllr
    }

    fn sysclk() -> Hz {
        match RCC.cfgr().sws() {
            U2::B00 => Self::msi(),
            U2::B01 => Self::hsi16(),
            U2::B10 => Self::hse(),
            U2::B11 => Self::pllclk(),
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
        Self::hclk()  >> shift
    }

    fn tim_pclk1() -> Hz {
        match RCC.cfgr().ppre1() {
            v if (v as u8) < 0b100  => Self::pclk1(),
            _ => Self::pclk1() << 1,
        }
    }

    fn pclk2() -> Hz {
        let shift = match RCC.cfgr().ppre2() {
            v if (v as u8) < 0b100  => 0,
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
            _ => 0,
        };
        Self::hclk() >> shift
    }

    fn tim_pclk2() -> Hz {
        match RCC.cfgr().ppre2() {
            v if (v as u8) < 0b100  => Self::pclk2(),
            _ => Self::pclk2() << 1,
        }
    }    

    impl_usart_clock_source!(::mcu::usart::USART1, usart1, pclk1);
    impl_usart_clock_source!(::mcu::usart::USART2, usart2, pclk1);
    impl_usart_clock_source!(::mcu::usart::USART3, usart3, pclk1);
    impl_usart_clock_source!(::mcu::lpuart::LPUART1, lpuart1, pclk1);

    impl_i2c_clock_source!(::mcu::i2c::I2C1, i2c1, pclk1);
    impl_i2c_clock_source!(::mcu::i2c::I2C2, i2c2, pclk1);
    impl_i2c_clock_source!(::mcu::i2c::I2C3, i2c3, pclk1);

    impl_lptim_clock_source!(::mcu::lptim::LPTIM1, lptim1, pclk2);
    impl_lptim_clock_source!(::mcu::lptim::LPTIM2, lptim2, pclk2);   
}



pub mod clock_init {
    use mcu::rcc::RCC;
    use mcu::flash::FLASH;

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
}