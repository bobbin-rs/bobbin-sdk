#![no_std]
#![no_main]

#[macro_use]
extern crate blue_pill as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let clk = Clock { hse_osc: Some(8_000_000)};
    println!("Clock Test");
    println!("Current Source: {:?}", clk.sysclk_src());

    println!("LSI_ON: {} LSI_RDY: {}", clk.lsi_on(), clk.lsi_rdy());
    println!("LSE_ON: {} LSE_RDY: {}", clk.lse_on(), clk.lse_rdy());
    println!("HSI_ON: {} HSI_RDY: {}", clk.hsi_on(), clk.hsi_rdy());
    println!("HSE_ON: {} HSE_RDY: {}", clk.hse_on(), clk.hse_rdy());
    println!("PLL_ON: {} PLL_RDY: {}", clk.pll_on(), clk.pll_rdy());

    println!("LSI:      {:?}", clk.lsi());
    println!("LSE:      {:?}", clk.lse());
    println!("HSI:      {:?}", clk.hsi());
    println!("HSE:      {:?}", clk.hse());
    println!("PLLCLK:   {:?}", clk.pllclk());
    println!("SYSCLK:   {:?}", clk.sysclk());
    println!("HCLK:     {:?}", clk.hclk());
    println!("PCLK1:    {:?}", clk.pclk1());
    println!("PCLK2:    {:?}", clk.pclk2());
    println!("TCLK1:    {:?}", clk.timclk_apb1());
    println!("TCLK2:    {:?}", clk.timclk_apb2());
    println!("ADCCLK:   {:?}", clk.adcclk());
    println!("SYSTICK:  {:?}", clk.systick());
    println!("RTCCLK:   {:?}", clk.rtcclk());
    println!("FLITFCLK: {:?}", clk.flitfclk());
    println!("IWDGCLK:  {:?}", clk.iwdgclk());

    println!("");

    println!("LSI: {:?}", clk.lsi());
    println!(" turning LSI ON");
    clk.set_lsi_on(true);
    while !clk.lsi_rdy() {}
    println!("LSI: {:?}", clk.lsi());
    println!(" turning LSI OFF");
    clk.set_lsi_on(false);
    while clk.lsi_rdy() {}
    println!("LSI: {:?}", clk.lsi());

    println!("");    
    println!("LSE: {:?}", clk.lse());
    println!(" turning LSE ON");
    clk.set_lse_on(true);
    while !clk.lse_rdy() {}
    println!("LSE: {:?}", clk.lse());
    println!(" turning LSE OFF");
    clk.set_lse_on(false);
    while clk.lse_rdy() {}
    println!("LSE: {:?}", clk.lse());

    println!("");

    println!("HSI: {:?}", clk.hsi());
    println!(" turning HSI on");
    clk.set_hsi_on(true);
    while !clk.hsi_rdy() {}
    println!("HSI: {:?}", clk.hsi());
    println!(" turning HSI off");
    clk.set_hsi_on(false);
    while clk.hsi_rdy() {}
    println!("HSI: {:?}", clk.hsi());
    
    println!("");
    println!("RTC_EN: {}", clk.rtc_enabled());
    println!(" enable RTC");
    clk.set_rtc_enabled(true);
    println!("RTC_EN: {}", clk.rtc_enabled());
    println!(" disable RTC");
    clk.set_rtc_enabled(false);
    println!("RTC_EN: {}", clk.rtc_enabled());
    loop {}
}

use board::chip::rcc::RCC;
use board::chip::pwr::PWR;

pub type Hz = Option<u32>;

pub const LSI: Hz = Some(40_000);
pub const LSE: Hz = Some(32768);
pub const HSI: Hz = Some(8_000_000);

#[derive(Debug, PartialEq)]
pub enum SysClock {
    Hsi = 0b00,
    Hse = 0b01,
    Pll = 0b10,
}

pub struct Clock {
    hse_osc: Hz,
}

impl Clock {
    pub fn hsi(&self) -> Hz { 
        if self.hsi_rdy() { HSI } else { None } 
    }

    pub fn hsi_rdy(&self) -> bool {
        match RCC.cr().hsirdy() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn hsi_on(&self) -> bool {
        match RCC.cr().hsion() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn set_hsi_on(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        RCC.with_cr(|r| r.set_hsion(value) );
        self
    }

    pub fn hse(&self) -> Hz { 
        if self.hse_rdy() { self.hse_osc } else { None } 
    }

    pub fn hse_rdy(&self) -> bool {
        match RCC.cr().hserdy() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn hse_on(&self) -> bool {
        match RCC.cr().hseon() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn set_hse_on(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        RCC.with_cr(|r| r.set_hseon(value) );
        self
    }


    pub fn lsi(&self) -> Hz { 
        if self.lsi_rdy() {
            LSI
        } else {
            None
        }
    }

    pub fn lsi_rdy(&self) -> bool {
        match RCC.csr().lsirdy() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn lsi_on(&self) -> bool {
        match RCC.csr().lsion() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn set_lsi_on(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        RCC.with_csr(|r| r.set_lsion(value) );
        self
    }

    pub fn lse(&self) -> Hz {
        if self.lse_rdy() { LSE } else { None }
    }

    pub fn lse_rdy(&self) -> bool {
        match RCC.bdcr().lserdy() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn lse_on(&self) -> bool {
        match RCC.bdcr().lseon() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn set_lse_on(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let p = RCC.apb1enr().pwren();
        RCC.with_apb1enr(|r| r.set_pwren(1));
        PWR.with_cr(|r| r.set_dbp(1));
        RCC.with_bdcr(|r| r.set_lseon(value) );
        PWR.with_cr(|r| r.set_dbp(0));
        RCC.with_apb1enr(|r| r.set_pwren(p));
        self
    }    

    pub fn pll_mul(&self) -> u32 {
        match RCC.cfgr().pllmul() {
            0b0000 => 2,
            0b0001 => 3,
            0b0010 => 4,
            0b0011 => 5,
            0b0100 => 6,
            0b0101 => 7,
            0b0110 => 8,
            0b0111 => 9,
            0b1000 => 10,
            0b1001 => 11,
            0b1010 => 12,
            0b1011 => 13,
            0b1100 => 14,
            0b1101 => 15,
            0b1110 => 16,
            0b1111 => 16,
            _ => unimplemented!(),
        }
    }

    pub fn pll_rdy(&self) -> bool {
        match RCC.cr().pllrdy() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn pll_on(&self) -> bool {
        match RCC.cr().pllon() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn set_pll_on(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        RCC.with_cr(|r| r.set_pllon(value) );
        self
    }

    pub fn pllclk(&self) -> Hz {
        match RCC.cr().pllrdy() {
            0b0 => None,
            0b1 => match RCC.cfgr().pllsrc() {
                0b0 => self.hsi().map(|v| v >> 1),
                0b1 => {
                    match RCC.cfgr().pllxtpre() {
                        0b0 => self.hse(),
                        0b1 => self.hse().map(|v| v >> 1),
                        _ => unimplemented!(),
                    }    
                },
                _ => unimplemented!(),
            }.map(|v| v * self.pll_mul()),
            _ => unimplemented!(),
        }
    }

    pub fn sysclk_src(&self) -> SysClock {
        match RCC.cfgr().sws() {
            0b00 => SysClock::Hsi,
            0b01 => SysClock::Hse,
            0b10 => SysClock::Pll,
            _ => unimplemented!()
        }
    }

    pub fn set_sysclk_src(&self, value: SysClock) -> &Self {
        RCC.with_cfgr(|r| r.set_sw(value as u32));
        self
    }

    pub fn sysclk(&self) -> Hz {
        match self.sysclk_src() {
            SysClock::Hsi => self.hsi(),
            SysClock::Hse => self.hse(),
            SysClock::Pll => self.pllclk(),
        }
    }

    pub fn hclk(&self) -> Hz {
        let hclk_shift = match RCC.cfgr().hpre() {
            0b0000 ... 0b0111 => 0,
            0b1000 => 1,
            0b1001 => 2,
            0b1010 => 3,
            0b1011 => 4,
            0b1100 => 5,
            0b1101 => 6,
            0b1110 => 7,
            0b1111 => 8,
            _ => unimplemented!(),
        };
        self.sysclk().map(|v| v >> hclk_shift)
    }

    pub fn pclk1(&self) -> Hz {
        let ppre1_shift = match RCC.cfgr().ppre1() {
            0b000 ... 0b011 => 0,
            0b100 => 1,
            0b101 => 2,
            0b110 => 3,
            0b111 => 4,
            _ => unimplemented!(),
        };
        self.hclk().map(|v| v >> ppre1_shift)
    }

    pub fn pclk2(&self) -> Hz {
        let ppre2_shift = match RCC.cfgr().ppre2() {
            0b000 ... 0b011 => 0,
            0b100 => 1,
            0b101 => 2,
            0b110 => 3,
            0b111 => 4,
            _ => unimplemented!(),
        };
        self.hclk().map(|v| v >> ppre2_shift)
    }

    pub fn timclk_apb1(&self) -> Hz {
        let timclk_shift = match RCC.cfgr().ppre1() {
            0b000 ... 0b011 => 0,
            0b100 ... 0b111 => 1,
            _ => unimplemented!(),
        };
        self.pclk1().map(|v| v << timclk_shift)
    }

    pub fn timclk_apb2(&self) -> Hz {
        let timclk_shift = match RCC.cfgr().ppre2() {
            0b000 ... 0b011 => 0,
            0b100 ... 0b111 => 1,
            _ => unimplemented!(),
        };
        self.pclk2().map(|v| v << timclk_shift)
    }

    pub fn adcclk(&self) -> Hz {
        let adc_div = match RCC.cfgr().adcpre() {
            0b00 => 2,
            0b01 => 4,
            0b10 => 6,
            0b11 => 8,
            _ => unimplemented!(),
        };
        self.pclk2().map(|v| v >> adc_div)
    }

    pub fn systick(&self) -> Hz {
        self.hclk().map(|v| v >> 3)
    }

    pub fn rtc_enabled(&self) -> bool {
        match RCC.bdcr().rtcen() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn set_rtc_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let p = RCC.apb1enr().pwren();
        RCC.with_apb1enr(|r| r.set_pwren(1));
        PWR.with_cr(|r| r.set_dbp(1));
        RCC.with_bdcr(|r| r.set_rtcen(value) );
        PWR.with_cr(|r| r.set_dbp(0));
        RCC.with_apb1enr(|r| r.set_pwren(p));
        self
    }

    pub fn rtcclk(&self) -> Hz {
        match RCC.bdcr().rtcsel() {
            0b00 => None,
            0b01 => self.lse(),
            0b10 => self.lsi(),
            0b11 => self.hse().map(|v| v >> 7),
            _ => unimplemented!(),
        }
    }

    pub fn iwdgclk(&self) -> Hz {
        self.lsi()
    }

    pub fn flitfclk(&self) -> Hz {
        self.hsi()
    }
}