pub use mcu::clock::*;

pub fn init() {
    // 8 MHz External Clock
    // VCO = 432MHz
    // PLL = 216MHz
    // PLLQ = 48MHz
    // SYSCLK = 216MHz
    // AHB = 216MHz
    // APB1 = 54MHz
    // APB2 = 108MHz

    clock_init::enable_pll_hse_bypass_mode(8, 432, 0b00, 9);
}

pub fn tree() -> Tree {
    TREE
}

static mut LSE_HZ: Hz = Hz::from_num(32768);
static mut HSE_HZ: Hz = Hz::from_num(8_000_000);

tree_impl! {
    id: TREE,
    defn: TREE_DEFN: TreeDefn,
    clock_impl: {
        Lsi:     ClkLsi      { lsi() },
        Lse:     ClkLse      { lse() },
        Hsi:     ClkHsi      { hsi() },
        Hse:     ClkHse      { hse() },
        Pllclk:  ClkPllclk   { pllclk() },
        Pll48clk:  ClkPll48clk   { pll48clk() },
        Sysclk:  ClkSysclk   { sysclk() },
        Hclk:    ClkHclk     { hclk() },
        Pclk1:   ClkPclk1    { pclk1() },
        Pclk2:   ClkPclk2    { pclk2() },
        TimPclk1: ClkTimPclk1 { tim_pclk1() },
        TimPclk2: ClkTimPclk2 { tim_pclk2() },
        Systick:  ClkSystick  { systick() },
        Usart1:   ClkUsart1  { usart1() },
        Usart2:   ClkUsart2  { usart2() },
        Usart3:   ClkUsart3  { usart3() },
        Uart4:    ClkUart4   { uart4() },
        Uart5:    ClkUart5   { uart5() },
        Usart6:   ClkUsart6  { usart6() },
        Uart7:    ClkUart7   { uart7() },
        Uart8:    ClkUart8   { uart8() },
        I2c1:     ClkI2c1    { i2c1() },
        I2c2:     ClkI2c2    { i2c2() },
        I2c3:     ClkI2c3    { i2c3() },
        I2c4:     ClkI2c4    { i2c4() },
    }
}

use mcu::rcc::*;
use common::bits::*;

pub fn lsi() -> Hz { LSI_HZ }
pub fn lse() -> Hz { unsafe { LSE_HZ }}
pub fn set_lse(value: Hz) { unsafe { LSE_HZ = value }}
pub fn hsi() -> Hz { HSI_HZ }
pub fn hse() -> Hz { unsafe { HSE_HZ }}
pub fn set_hse(value: Hz) { unsafe { HSE_HZ = value }}


pub fn pllclk() -> Hz {
    let cfgr = RCC.pllcfgr();
    let vco_in = match cfgr.pllsrc() {
        U1::B0 => lsi(),
        U1::B1 => hse(),
    };
    let vco = (vco_in / cfgr.pllm().into_u32()).reduced() * cfgr.plln().into_u32();
    (vco / (2 * (cfgr.pllp().into_u32() + 1))).reduced()
}

pub fn pllq() -> Hz {
    let cfgr = RCC.pllcfgr();
    let vco_in = match cfgr.pllsrc() {
        U1::B0 => hsi(),
        U1::B1 => hse(),
    };
    let vco = (vco_in / cfgr.pllm().into_u32()).reduced() * cfgr.plln().into_u32();
    (vco / cfgr.pllq().into_u32()).reduced()
}    

pub fn pll48clk() -> Hz {
    match RCC.dckcfgr2().ck48msel() {
        U1::B0 => pllq(),
        U1::B1 => unimplemented!(),
    }
}

pub fn sysclk() -> Hz {
    match RCC.cfgr().sws() {
        U2::B00 => hsi(),
        U2::B01 => hse(),
        U2::B10 => pllclk(),
        U2::B11 => panic!("Invalid value for RCC_CFGR[SWS]"),
    }
}

pub fn hclk() -> Hz {
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
    sysclk() >> shift
}    

fn systick() -> Hz {
    hclk() >> 3
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
    hclk() >> shift
}

fn tim_pclk1() -> Hz {
    match RCC.cfgr().ppre1() {
        U3::B000 | U3::B001 | U3::B010 | U3::B011 => pclk1(),
        _ => pclk1() << 1,        
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
    hclk() >> shift
}

fn tim_pclk2() -> Hz {
    match RCC.cfgr().ppre2() {
        U3::B000 | U3::B001 | U3::B010 | U3::B011 => pclk2(),
        _ => pclk2() << 1,
    }
}    

macro_rules! impl_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id() -> Hz {
            match $periph.clock_source() {
                DedicatedClock::Pclk => $default(),
                DedicatedClock::Sysclk => sysclk(),
                DedicatedClock::Hsi => hsi(),
                DedicatedClock::Lse => lse(),
            }
        }        
    };
}

impl_clock_source!(::mcu::usart::USART1, usart1, pclk2);
impl_clock_source!(::mcu::usart::USART2, usart2, pclk1);
impl_clock_source!(::mcu::usart::USART3, usart3, pclk1);
impl_clock_source!(::mcu::usart::UART4, uart4, pclk1);
impl_clock_source!(::mcu::usart::UART5, uart5, pclk1);
impl_clock_source!(::mcu::usart::USART6, usart6, pclk2);
impl_clock_source!(::mcu::usart::UART7, uart7, pclk1);
impl_clock_source!(::mcu::usart::UART8, uart8, pclk1);


impl_clock_source!(::mcu::i2c::I2C1, i2c1, pclk1);
impl_clock_source!(::mcu::i2c::I2C2, i2c2, pclk1);
impl_clock_source!(::mcu::i2c::I2C3, i2c3, pclk1);
impl_clock_source!(::mcu::i2c::I2C4, i2c4, pclk1);

// fn usart1() -> Hz {
//     match ::mcu::usart::USART1.clock_source() {
//         DedicatedClock::Pclk => pclk2(),
//         DedicatedClock::Sysclk => sysclk(),
//         DedicatedClock::Hsi => hsi(),
//         DedicatedClock::Lse => lse(),
//     }
// }


pub mod clock_init {
    use ::mcu::{rcc, flash, pwr};

    pub fn enable_pll_external_mode() {
        enable_pll_hse_bypass_mode(8, 336, 0, 7)
    }

    pub fn enable_pll_hse_bypass_mode(m: u32, n: u32, p: u32, q: u32) {
        let rcc = rcc::RCC;
        let flash = flash::FLASH;
        let pwr = pwr::PWR;        

        // Enable internal high-speed oscillator.
        rcc.with_cr(|r| r.set_hsion(1));

        // Wait for HSI Ready
        while rcc.cr().hsirdy() == 0 {}

        // Select HSI as SYSCLK source. 
        rcc.with_cfgr(|r| r.set_sw(0b00));
        while rcc.cfgr().sws() != 0b00 {}

        // Enable external high-speed oscillator 8MHz.
        // rcc.with_cr(|r| r.set_hseon(1));

        // Enable external source
        rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(1));
        
        // Wait for HSE Ready
        while rcc.cr().hserdy() == 0 {}

        pwr.with_csr1(|r| r.set_vosrdy(1));

        // Set prescalers for AHB, ADC, ABP1, ABP2

        // HPRE = HPRE_DIV_NONE
        // PPRE1 = PPRE_DIV_4
        // PPRE2 = PPRE_DIV_2
        rcc.with_cfgr(|r| r.set_hpre(0).set_ppre1(0b101).set_ppre2(0b100));

        rcc.with_pllcfgr(|r|
            r.set_pllsrc(1)
                .set_pllm(m)
                .set_plln(n)
                .set_pllp(p)
                .set_pllq(q)
        );

        // Enable PLL oscillator and wait for it to stabilize.
        rcc.with_cr(|r| r.set_pllon(1));


        // Enable Overdrive

        rcc.with_apb1enr(|r| r.set_pwren(1));

        pwr.with_cr1(|r| r.set_oden(1));
        while !pwr.csr1().test_odrdy() {}

        pwr.with_cr1(|r| r.set_odswen(1));
        while !pwr.csr1().test_odswrdy() {}

        // Wait for PLL Ready
        while rcc.cr().pllrdy() == 0 {}

        // Configure flash settings.

        flash.with_acr(|r| r.set_icen(1).set_dcen(1).set_latency(7));
        
        // Select PLL as SYSCLK source.

        rcc.with_cfgr(|r| r.set_sw(0b10));
        while rcc.cfgr().sws() != 0b10 {}

        // Disable internal high-speed oscillator.        
        // rcc.with_cr(|r| r.set_hsion(0));
    }

    pub fn enable_pll_hse_mode(m: u32, n: u32, p: u32, q: u32) {
        let rcc = rcc::RCC;
        let flash = flash::FLASH;
        let pwr = pwr::PWR;

        // Enable internal high-speed oscillator.
        rcc.with_cr(|r| r.set_hsion(1));

        // Wait for HSI Ready
        while rcc.cr().hsirdy() == 0 {}

        // Select HSI as SYSCLK source. 
        rcc.with_cfgr(|r| r.set_sw(0b00));
        while rcc.cfgr().sws() != 0b00 {}

        // Enable external high-speed oscillator 8MHz.
        // rcc.with_cr(|r| r.set_hseon(1));

        // Enable external source
        rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(0));
        
        // Wait for HSE Ready
        while rcc.cr().hserdy() == 0 {}

        pwr.with_csr1(|r| r.set_vosrdy(1));

        // Set prescalers for AHB, ADC, ABP1, ABP2

        // HPRE = HPRE_DIV_NONE
        // PPRE1 = PPRE_DIV_4
        // PPRE2 = PPRE_DIV_2
        rcc.with_cfgr(|r| r.set_hpre(0).set_ppre1(0b101).set_ppre2(0b100));
        
        rcc.with_pllcfgr(|r|
            r.set_pllsrc(1)
                .set_pllm(m)
                .set_plln(n)
                .set_pllp(p)
                .set_pllq(q)
        );

        // Enable PLL oscillator and wait for it to stabilize.
        rcc.with_cr(|r| r.set_pllon(1));
        
        // Wait for PLL Ready
        while rcc.cr().pllrdy() == 0 {}

        // Configure flash settings.

        flash.with_acr(|r| r.set_icen(1).set_dcen(1).set_latency(7));
        
        // Select PLL as SYSCLK source.

        rcc.with_cfgr(|r| r.set_sw(0b10));
        while rcc.cfgr().sws() != 0b10 {}
        
        // // Disable internal high-speed oscillator.        
        // rcc.with_cr(|r| r.set_hsion(0));
    }

    pub fn enable_pll_hsi_mode() {
        let rcc = rcc::RCC;
        let flash = flash::FLASH;
        let pwr = pwr::PWR;

        // Enable internal high-speed oscillator.
        rcc.with_cr(|r| r.set_hsion(1));

        // Wait for HSI Ready
        while rcc.cr().hsirdy() == 0 {}

        // // Select HSI as SYSCLK source. 
        // rcc.with_cfgr(|r| r.set_sw(0b00));
        // while RCC.cfgr().sws() != 0b00 {}

        pwr.with_csr1(|r| r.set_vosrdy(1));

        // Set prescalers for AHB, ADC, ABP1, ABP2

        // HPRE = HPRE_DIV_NONE
        // PPRE1 = PPRE_DIV_4
        // PPRE2 = PPRE_DIV_2
        rcc.with_cfgr(|r| r.set_hpre(0).set_ppre1(0b101).set_ppre2(0b100));

        // Configure PLL
        // PLLSRC = HSE
        // M = 8
        // N = 336
        // P = 2
        // Q = 7
        // R = 0
        
        rcc.with_pllcfgr(|r|
            r.set_pllsrc(0)
                .set_pllm(8)
                .set_plln(432)
                .set_pllp(2)
                .set_pllq(9)
        );

        // // rcc.with_pllcfgr(|r|
        // //     r.set_pllsrc(1)
        // //         .set_pllq3(0).set_pllq2(1).set_pllq1(1).set_pllq0(1)
        // //         .set_pllp1(0).set_pllp0(0)
        // //         .set_plln8(1).set_plln7(0).set_plln6(1).set_plln5(0).set_plln4(1).set_plln3(0).set_plln2(0).set_plln1(0).set_plln0(0)
        // //         .set_pllm5(0).set_pllm4(0).set_pllm3(1).set_pllm2(0).set_pllm1(0).set_pllm0(0)                                
        // // );

        // Enable PLL oscillator and wait for it to stabilize.
        rcc.with_cr(|r| r.set_pllon(1));
        
        // Wait for PLL Ready
        while rcc.cr().pllrdy() == 0 {}

        // // Configure flash settings.

        flash.with_acr(|r| r.set_icen(1).set_dcen(1).set_latency(5));
        
        // // Select PLL as SYSCLK source.

        rcc.with_cfgr(|r| r.set_sw(0b10));
        while rcc.cfgr().sws() != 0b10 {}    
    }
}