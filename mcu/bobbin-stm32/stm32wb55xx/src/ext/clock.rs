use bobbin_mcu::hz::Hz;
use bobbin_mcu::clock::{Clock, ClockFor, ClockSource};
use bobbin_bits::*;

use ext::rcc::*;
use clock::*;
use rcc::{self, RCC};

macro_rules! impl_usart_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id(&self) -> Hz {
            match $periph.clock_source() {
                UsartClock::Pclk => self.$default(),
                UsartClock::Sysclk => self.sysclk(),
                UsartClock::Hsi16 => self.hsi16(),
                UsartClock::Lse => self.lse(),
            }
        }
    };
}


macro_rules! impl_i2c_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id(&self) -> Hz {
            match $periph.clock_source() {
                I2cClock::Pclk => self.$default(),
                I2cClock::Sysclk => self.sysclk(),
                I2cClock::Hsi16 => self.hsi16(),
            }
        }
    };
}

macro_rules! impl_lptim_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id(&self) -> Hz {
            match $periph.clock_source() {
                LptimClock::Pclk => self.$default(),
                LptimClock::Lsi => self.lsi1(),
                LptimClock::Hsi16 => self.hsi16(),
                LptimClock::Lse => self.lse(),
            }
        }
    };
}

fn pll_p(val: U5) -> u32 {
    let val = val.into_u32();
    if val == 0 {
        unreachable!();
    }

    val + 1
}

fn pll_rq(val: U3) -> u32 {
    match val {
        U3::B000 => unimplemented!(),
        U3::B001 => 2,
        U3::B010 => 3,
        U3::B011 => 4,
        U3::B100 => 5,
        U3::B101 => 6,
        U3::B110 => 7,
        U3::B111 => 8,
    }
}

fn pllm(cfgr: rcc::Pllcfgr) -> u32 {
    1 + cfgr.pllm().into_u32()
}

fn saim(_cfgr: rcc::Pllsai1cfgr) -> u32 {
    1
}

macro_rules! pll_clk {
    ($id:ident, $cfg:ident, $divreg:ident, $mulfun:ident, $divfun:ident) => {
        fn $id(&self) -> Hz {
            let cfgr = RCC.$cfg();

            let plln = cfgr.plln().into_u32();
            let pllm = $mulfun(cfgr);
            let divisor = $divfun(cfgr.$divreg());

            // PLLSCFGR.PLLSRC selects clock source for both for PLL and PLLSAI1
            let v = match RCC.pllcfgr().pllsrc() {
                U2::B00 => Hz::from(0),
                U2::B01 => self.msi(),
                U2::B10 => self.hsi16(),
                U2::B11 => self.hse(),
            };

            let vco_clock = v * (plln / pllm);
            let freq = vco_clock / divisor;

            freq
        }
    }
}

#[derive(Default)]
pub struct Osc32m {}
impl Clock for Osc32m {
    fn hz() -> Hz { Hz::from_num(32_000_000) }
}

#[derive(Default)]
pub struct Osc32k {}
impl Clock for Osc32k {
    fn hz() -> Hz { Hz::from_num(32768) }
}

#[derive(Default)]
pub struct DynamicClock<OSC: Clock, OSC32: Clock>(OSC, OSC32);

impl<OSC: Clock, OSC32: Clock> DynamicClock<OSC, OSC32> {
    fn msi(&self) -> Hz {
        Hz::from(match RCC.cr().msirange() {
            U4::B0000 => 100_000,
            U4::B0001 => 200_000,
            U4::B0010 => 400_000,
            U4::B0011 => 800_000,
            U4::B0100 => 1_000_000,
            U4::B0101 => 2_000_000,
            U4::B0110 => 4_000_000, // reset value
            U4::B0111 => 8_000_000,
            U4::B1000 => 16_000_000,
            U4::B1001 => 24_000_000,
            U4::B1010 => 32_000_000,
            U4::B1011 => 48_000_000,
            _ => 0,
        })
    }
}

fn freq_prescaler_lookup(val: U4) -> u32 {
    match val {
//        U4::B0001 => 3,
//        U4::B0010 => 5,
//        U4::B0101 => 6,
//        U4::B0110 => 10,
        U4::B1000 => 1,
        U4::B1001 => 2,
        U4::B1010 => 3,
        U4::B1011 => 4,
        U4::B0111 => 5,
        U4::B1100 => 6,
        U4::B1101 => 7,
        U4::B1110 => 8,
        U4::B1111 => 9,

        _ => 0, // Other values indicate that SYSCLK is not divided
    }
}

impl<OSC: Clock, OSC32: Clock> ClockProvider for DynamicClock<OSC, OSC32> {
    type Osc = OSC;
    type Osc32 = OSC32;

    pll_clk!(pllclk, pllcfgr, pllr, pllm, pll_rq);
    pll_clk!(pllsai1rclk, pllsai1cfgr, pllr, saim, pll_rq);
    pll_clk!(pllsai1qclk, pllsai1cfgr, pllq, saim, pll_rq);
    pll_clk!(pllsai1pclk, pllsai1cfgr, pllp, saim, pll_p);

    fn sysclk(&self) -> Hz {
        match RCC.cfgr().sws() {
            U2::B00 => self.msi(),
            U2::B01 => self.hsi16(),
            U2::B10 => self.hse(),
            U2::B11 => self.pllclk(),
        }
    }

    fn hclk1(&self) -> Hz {
        // RM0434 section 8.4.3
        self.sysclk() >> freq_prescaler_lookup(RCC.cfgr().hpre())
    }

    fn hclk2(&self) -> Hz {
        // RM0434 section 8.4.34 (RCC extended clock recovery register),
        // HCLK2 prescaler (CPU2)
        self.sysclk() >> freq_prescaler_lookup(RCC.extcfgr().c2hpre())
    }

    fn hclk4(&self) -> Hz {
        // RM0434 section 8.4.34 (RCC extended clock recovery register),
        // HCLK4 shared prescaler (AHB4, Flash memory and SRAM2)
        self.sysclk() >> freq_prescaler_lookup(RCC.extcfgr().shdhpre())
    }

    fn systick(&self) -> Hz {
        // RM0434, section 8.2 (Clocks)
        // The RCC feeds the CPU1 System Timer (SysTick) external clock with the
        // AHB clock (HCLK1) divided by 8 (hence the >> 3 shift)
        self.hclk1() >> 3
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
        self.hclk1() >> shift
    }

    fn tim_pclk1(&self) -> Hz {
        match RCC.cfgr().ppre1() {
            v if (v as u8) < 0b100  => self.pclk1(),
            _ => self.pclk1() << 1,
        }
    }

    fn pclk2(&self) -> Hz {
        let shift = match RCC.cfgr().ppre2() {
            v if (v as u8) < 0b100  => 0,
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
            _ => 0,
        };
        self.hclk2() >> shift
    }

    fn tim_pclk2(&self) -> Hz {
        match RCC.cfgr().ppre2() {
            v if (v as u8) < 0b100  => self.pclk2(),
            _ => self.pclk2() << 1,
        }
    }

    fn lsi(&self) -> Hz {
        // RM0434 page 215
        // LSI2 is used as clock source for LSI by-default, if switched on.
        // LSI1 is selected as clock source if LSI2 is switched off and LSI1 is switched on.
        if RCC.csr().lsi2on() == U1::B1 {
            self.lsi2()
        } else {
            if RCC.csr().lsi1on() == U1::B1 {
                self.lsi1()
            } else {
                Hz::from(0)
            }
        }
    }

    fn rtcclk(&self) -> Hz {
        // RM0434, page 265
        match RCC.bdcr().rtcsel() {
            U2::B00 => Hz::from(0),
            U2::B01 => self.lse(),
            U2::B10 => self.lsi(),
            U2::B11 => self.hse() >> 5, // HSE oscillator clock divided by 32 used as RTC clock
        }
    }

    fn rng(&self) -> Hz {
        // RM0434, page 262
        match RCC.ccipr().rngsel() {
            U2::B00 => self.clk48(),
            U2::B01 => self.lsi(),
            U2::B10 => self.lse(),
            U2::B11 => unreachable!(),
        }
    }

    fn adc(&self) -> Hz {
        // RM0434, page 262
        match RCC.ccipr().adcsel() {
            U2::B00 => Hz::from(0),
            U2::B01 => self.pllsai1rclk(),
            U2::B10 => self.pllclk(),
            U2::B11 => self.sysclk(),
        }
    }

    fn clk48(&self) -> Hz {
        // RM0434, page 262
        match RCC.ccipr().clk48sel() {
            U2::B00 => self.hsi48(),
            U2::B01 => self.pllsai1qclk(),
            U2::B10 => self.pllclk(),
            U2::B11 => self.msi(),
        }
    }

    fn sai1(&self) -> Hz {
        match RCC.ccipr().sai1sel() {
            U2::B00 => self.pllsai1pclk(),
            U2::B01 => self.pllclk(),
            U2::B10 => self.hsi16(),
            U2::B11 => unimplemented!(), // External clock frequency is not specified
        }
    }

    impl_usart_clock_source!(::usart::USART1, usart1, pclk1);
    impl_usart_clock_source!(::lpuart::LPUART1, lpuart1, pclk1);

    impl_i2c_clock_source!(::i2c::I2C1, i2c1, pclk1);
    impl_i2c_clock_source!(::i2c::I2C3, i2c3, pclk1);

    impl_lptim_clock_source!(::lptim::LPTIM1, lptim1, pclk2);
    impl_lptim_clock_source!(::lptim::LPTIM2, lptim2, pclk2);
}

impl<CP> ClockFor<::systick::Systick> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::systick::Systick) -> Hz { self.systick() }
}

pub mod clock_init {
    use rcc::RCC;
    use pwr::PWR;
    use flash::FLASH;


    #[inline(never)]
    pub fn init_pll() {
        // Set one wait state in Latency bit of FLASH_ACR
        FLASH.with_acr(|r| r.set_latency(3));

        // Check the latency is set
        while FLASH.acr().latency() != 3 {}

        // Enable HSE and disable PLL
        RCC.with_cr(|r| r.set_pllon(0).set_hseon(1));

        // Wait for HSE Ready Flag
        while RCC.cr().hserdy() == 0 {}

        // Enable HSI
        RCC.with_cr(|r| r.set_hsion(1));

        // Wait for HSI Ready Flag
        while RCC.cr().hsirdy() == 0 {}

        // Enable HSI48
        RCC.with_crrcr(|r| r.set_hsi48on(1));

        // Wait for HSI48 Ready Flag
        while RCC.crrcr().hsi48rdy() == 0 {}

        // Configure PLL
        // 32 / 2 * 8 / 2 = 64 MHz
        RCC.with_pllcfgr(|r| r
            .set_pllsrc(0b11) // HSE 32 MHz as input source
            .set_pllm(0b001)  // divide by 2
            .set_plln(8)      // multiply by 8
            .set_pllr(0x1)    // divide by 2
            .set_pllren(1)    // enable output
        );

        // (5) Enable and switch on PLL
        RCC.with_cr(|r| r.set_pllon(1));

        // Wait for PLL Ready
        while RCC.cr().pllrdy() == 0 {}

        // Set CPU2 AHB prescaler to /2
        RCC.with_extcfgr(|r| r.set_c2hpre(2));

        // Switch to PLL
        RCC.with_cfgr(|r| r.set_sw(0b11));

        // Wait for system clock to use PLL
        while RCC.cfgr().sws() != 0b11 {}

        // Set AHB4 (shared) prescaler
        RCC.with_extcfgr(|r| r.set_shdhpre(1));

        // Set APB1 prescaler
        RCC.with_cfgr(|r| r.set_ppre1(1));

        // Set APB2 prescaler
        RCC.with_cfgr(|r| r.set_ppre2(1));

        // Enable backup domain access
        PWR.with_cr1(|cr1| cr1.set_dbp(1));

        // Reset backup domain
        RCC.with_bdcr(|bdcr| bdcr.set_bdrst(1));
        RCC.with_bdcr(|bdcr| bdcr.set_bdrst(0));

        // Set LSE drive capability to LOW
        RCC.with_bdcr(|bdcr| bdcr.set_lsedrv(0));

        // Enable LSE
        RCC.with_bdcr(|bdcr| bdcr.set_lseon(1));

        // Wait till LSE is ready
        while RCC.bdcr().lserdy() == 0 {}
    }
}
