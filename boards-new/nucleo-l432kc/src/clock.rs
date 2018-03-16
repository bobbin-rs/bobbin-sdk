pub use mcu::clock::*;

pub fn init() {
    clock_init::init_pll()
}

// Main System Clock = 80 MHz
// APB2 = 80 MHz
// APB1 = 80 MHz
// AHB = 80 MHz
// 
// HSI @ 16 MHz
// VCO @ 160 MHz (M = 1, N = 10)
// PLL @ 80 Mhz (R = 2)
// FLASH: 4 wait states

static mut LSE_HZ: Hz = Hz::from_num(32768);
static mut HSE_HZ: Hz = Hz::from_num(8_000_000);

tree_impl! {
    id: TREE,
    defn: TREE_DEFN: TreeDefn,
    clock_impl: {
        Lsi:     ClkLsi      { lsi() },
        Lse:     ClkLse      { lse() },
        Msi:     ClkMsi      { msi() },
        Hsi16:   ClkHsi16    { hsi16() },
        Hse:     ClkHse      { hse() },
        Pllclk:  ClkPllclk   { pllclk() },        
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
        Lpuart1:   ClkLpuart1     { lpuart1() },
        I2c1:     ClkI2c1    { i2c1() },
        I2c2:     ClkI2c2    { i2c2() },
        I2c3:     ClkI2c3    { i2c3() },        
        Lptim1:   ClkLptim1     { lptim1() },
        Lptim2:   ClkLptim2     { lptim2() },
    }
}

use mcu::rcc::*;
use common::bits::*;

pub fn lsi() -> Hz { LSI_HZ }
pub fn lse() -> Hz { unsafe { LSE_HZ }}
pub fn set_lse(value: Hz) { unsafe { LSE_HZ = value }}

pub fn msi() -> Hz {
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
        _ => panic!("Invalid Value"),
    })
}

pub fn hsi16() -> Hz { HSI16_HZ }
pub fn hse() -> Hz { unsafe { HSE_HZ }}
pub fn set_hse(value: Hz) { unsafe { HSE_HZ = value }}

pub fn pllclk() -> Hz {
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
        U2::B01 => msi(),
        U2::B10 => hsi16(),
        U2::B11 => hse(),
    };
    (v * plln / pllm) / pllr
}

pub fn sysclk() -> Hz {
    match RCC.cfgr().sws() {
        U2::B00 => msi(),
        U2::B01 => hsi16(),
        U2::B10 => hse(),
        U2::B11 => pllclk(),
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

pub fn pclk1() -> Hz {
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
    hclk()  >> shift
}

pub fn tim_pclk1() -> Hz {
    match RCC.cfgr().ppre1() {
        v if (v as u8) < 0b100  => pclk1(),
        _ => pclk1() << 1,
    }
}

pub fn pclk2() -> Hz {
    let shift = match RCC.cfgr().ppre2() {
        v if (v as u8) < 0b100  => 0,
        U3::B100 => 1,
        U3::B101 => 2,
        U3::B110 => 3,
        U3::B111 => 4,
        _ => unimplemented!(),
    };
    hclk() >> shift
}

pub fn tim_pclk2() -> Hz {
    match RCC.cfgr().ppre2() {
        v if (v as u8) < 0b100  => pclk2(),
        _ => pclk2() << 1,
    }
}    


macro_rules! impl_usart_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id() -> Hz {
            match $periph.clock_source() {
                UsartClock::Pclk => $default(),
                UsartClock::Sysclk => sysclk(),
                UsartClock::Hsi16 => hsi16(),
                UsartClock::Lse => lse(),
            }
        }        
    };
}

impl_usart_clock_source!(::mcu::usart::USART1, usart1, pclk1);
impl_usart_clock_source!(::mcu::usart::USART2, usart2, pclk1);
impl_usart_clock_source!(::mcu::usart::USART3, usart3, pclk1);
impl_usart_clock_source!(::mcu::lpuart::LPUART1, lpuart1, pclk1);



macro_rules! impl_i2c_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id() -> Hz {
            match $periph.clock_source() {
                I2cClock::Pclk => $default(),
                I2cClock::Sysclk => sysclk(),
                I2cClock::Hsi16 => hsi16(),
            }
        }        
    };
}

impl_i2c_clock_source!(::mcu::i2c::I2C1, i2c1, pclk1);
impl_i2c_clock_source!(::mcu::i2c::I2C2, i2c2, pclk1);
impl_i2c_clock_source!(::mcu::i2c::I2C3, i2c3, pclk1);

macro_rules! impl_lptim_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id() -> Hz {
            match $periph.clock_source() {
                LptimClock::Pclk => $default(),
                LptimClock::Lsi => lsi(),
                LptimClock::Hsi16 => hsi16(),
                LptimClock::Lse => lse(),
            }
        }        
    };
}

impl_lptim_clock_source!(::mcu::lptim::LPTIM1, lptim1, pclk2);
impl_lptim_clock_source!(::mcu::lptim::LPTIM2, lptim2, pclk2);


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