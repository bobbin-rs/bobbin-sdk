use bobbin_bits::*;
use bobbin_mcu::hz::Hz;
use bobbin_mcu::clock::{Clock, ClockSource};

use clock::*;
use ext::systick::SystickHz;
use rcc::*;
use ext::rcc::*;
use {tim_adv, tim_gen, i2c, usart};


use rcc;
use flash;

#[derive(Default)]
pub struct Osc8m {}
impl Clock for Osc8m {
    fn hz() -> Hz { Hz::from_num(8_000_000) }
}

#[derive(Default)]
pub struct Osc32k {}
impl Clock for Osc32k {
    fn hz() -> Hz { Hz::from_num(32768) }
}

macro_rules! impl_usart_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id(&self) -> Hz {
            match $periph.clock_source() {
                UsartClock::Pclk => self.$default(),
                UsartClock::Sysclk => self.sysclk(),
                UsartClock::Lse => self.lse(),
                UsartClock::Hsi => self.hsi(),
            }
        }        
    };
}


macro_rules! impl_i2c_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id(&self) -> Hz {
            match $periph.clock_source() {
                I2cClock::Hsi => self.hsi(),
                I2cClock::Sysclk => self.sysclk(),
            }
        }        
    };
}

macro_rules! impl_tim_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id(&self) -> Hz {
            match $periph.clock_source() {
                TimClock::Pclk => self.$default(),
                TimClock::Pll => self.pllclk(),
            }
        }        
    };
}

#[derive(Default)]
pub struct DynamicClock<OSC: Clock, OSC32: Clock>(OSC, OSC32);

impl<OSC: Clock, OSC32: Clock> ClockProvider for DynamicClock<OSC, OSC32> {
    type Osc = OSC;
    type Osc32 = OSC32;

    fn pllclk(&self) -> Hz {
        let cfgr = RCC.cfgr();
        let cfgr2 = RCC.cfgr2();
        let prediv = cfgr2.prediv().into_u32() + 1;
        let pllmul = match cfgr.pllmul() {
            U4::B1111 => 16,
            m @ _ => m.into_u32() + 2,
        };
        match cfgr.pllsrc() {
            U2::B00 => self.hsi() >> 1,
            U2::B01 => self.hsi() * pllmul / prediv,
            U2::B10 => self.hse()* pllmul / prediv,
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

    fn adc12(&self) -> Hz {
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
        self.pllclk() / div
    }

    fn adc34(&self) -> Hz {
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
        self.pllclk() / div
    }        
    impl_usart_clock_source!(usart::USART1, usart1, pclk2);
    impl_usart_clock_source!(usart::USART2, usart2, pclk1);
    impl_usart_clock_source!(usart::USART3, usart3, pclk1);
    impl_usart_clock_source!(usart::UART4, uart4, pclk1);
    impl_usart_clock_source!(usart::UART5, uart5, pclk1);

    impl_i2c_clock_source!(i2c::I2C1, i2c1, hsi);
    impl_i2c_clock_source!(i2c::I2C2, i2c2, hsi);
    impl_i2c_clock_source!(i2c::I2C3, i2c3, hsi);

    impl_tim_clock_source!(tim_adv::TIM1, tim1, pclk2);
    impl_tim_clock_source!(tim_gen::TIM2, tim2, pclk2);
    impl_tim_clock_source!(tim_gen::TIM3, tim3, pclk2);
    impl_tim_clock_source!(tim_gen::TIM4, tim4, pclk2);
    impl_tim_clock_source!(tim_adv::TIM8, tim8, pclk2);
    impl_tim_clock_source!(tim_gen::TIM15, tim15, pclk2);
    impl_tim_clock_source!(tim_gen::TIM16, tim16, pclk2);
    impl_tim_clock_source!(tim_adv::TIM20, tim20, pclk2);
}

impl<OSC: Clock, OSC32: Clock> SystickHz for DynamicClock<OSC, OSC32> {
    fn systick_hz(&self) -> Hz {
        self.systick()
    }
}


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
    // rcc.with_cr(|r| r.set_hsion(0));
}
