use ::chip::rcc::{self, RCC};
use ::chip::pwr::PWR;
use ::chip::flash;
use ::chip::usart::*;
use ::chip::tim_gen::*;
use ::chip::tim_adv::*;
use ::chip::iwdg::*;
use ::chip::wwdg::*;

// 8Mhz external clock
// System = 72Mhz
// AHB = 72Mhz (Divide by 1)
// APB1 = 36Mhz (Divide by 2)
// APB2 = 72Mhz (Divide by 1)

pub fn enable_pll_external_mode() {
    let rcc = rcc::RCC;
    let flash = flash::FLASH;
    
    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {}

    // Select HSI as SYSCLK source. 
    rcc.with_cfgr(|r| r.set_sw(0));

    // Enable external high-speed oscillator 8MHz.
    rcc.with_cr(|r| r.set_hseon(1));

    // Wait for HSE Ready
    while rcc.cr().hserdy() == 0 {}

    // Set prescalers for AHB, ADC, ABP1, ABP2

    // HPRE = HPRE_DIV_NONE
    // PPRE1 = PPRE_DIV_2 (0b100)
    // PPRE2 = PPRE_DIV_NONE (0b000)
    // PLLSRC = PREDIV1 (0b1)
    // PLLMUL = x9 (0b0111)
    // MCO = PLL/2 (0b0111)
    rcc.with_cfgr(|r| 
        r.set_hpre(0)
            .set_ppre1(0b100)
            .set_ppre2(0b000)
            .set_pllsrc(1)
            .set_pllmul(0b0111)
    );

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {}

    // Configure flash settings.
    // PRFTBE = enabled (0b1)
    // LATENCY = 2 Wait States (0b010)
    flash.with_acr(|r| r.set_latency(0b010));
    
    // Select PLL as SYSCLK source.

    rcc.with_cfgr(|r| r.set_sw(0b10));
    
    // Wait for PLL to be selected
    while rcc.cfgr().sws() != 0b10 {}
    
    // Disable internal high-speed oscillator.        
    rcc.with_cr(|r| r.set_hsion(0));
}


pub type Hz = Option<u32>;

pub const LSI: Hz = Some(40_000);
pub const LSE: Hz = Some(32768);
pub const HSI: Hz = Some(8_000_000);

#[derive(Debug, PartialEq)]
pub enum SysClockSrc {
    Hsi = 0b00,
    Hse = 0b01,
    Pll = 0b10,
}

#[derive(Debug, PartialEq)]
pub enum PllSrc {
    HsiDiv2 = 0b00,
    Hse = 0b01,
}

#[derive(Debug, PartialEq)]
pub enum RtcSel {
    None = 0b00,
    Lse = 0b01,
    Lsi = 0b10,
    HseDiv128 = 0b11,
}

#[derive(Debug, PartialEq)]
pub enum HPre {
    Div1 = 0b0000,
    Div2 = 0b1000,
    Div4 = 0b1001,
    Div8 = 0b1010,
    Div16 = 0b1011,
    // Note: Divide by 32 is skipped
    Div64 = 0b1100,
    Div128 = 0b1101,
    Div256 = 0b1110,
    Div512 = 0b1111,
}

#[derive(Debug, PartialEq)]
pub enum PPre1 {
    Div1 = 0b000,
    Div2 = 0b100,
    Div4 = 0b101,
    Div8 = 0b110,
    Div16 = 0b111,
}

#[derive(Debug, PartialEq)]
pub enum PPre2 {
    Div1 = 0b000,
    Div2 = 0b100,
    Div4 = 0b101,
    Div8 = 0b110,
    Div16 = 0b111,
}

pub struct ClockTree {
    pub hse_osc: Hz,
}

impl ClockTree {
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

    pub fn wait_hsi_rdy(&self) -> &Self {
        while !self.hsi_rdy() {}
        self
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

    pub fn wait_hse_rdy(&self) -> &Self {
        while !self.hse_rdy() {}
        self
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

    pub fn wait_lsi_rdy(&self) -> &Self {
        while !self.lsi_rdy() {}
        self
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

    pub fn wait_lse_rdy(&self) -> &Self {
        while !self.lse_rdy() {}
        self
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

    pub fn set_pll_mul(&self, value: u32) -> &Self {
        let value = match value {
            2 => 0b0000,
            3 => 0b0001,
            4 => 0b0010,
            5 => 0b0011,
            6 => 0b0100,
            7 => 0b0101,
            8 => 0b0110,
            9 => 0b0111,
            10 => 0b1000,
            11 => 0b1001,
            12 => 0b1010,
            13 => 0b1011,
            14 => 0b1100,
            15 => 0b1101,
            16 => 0b1110,
            _ => panic!("Unsupported PLL Multiplier"),
        };
        RCC.with_cfgr(|r| r.set_pllmul(value));
        self
    }

    pub fn pll_rdy(&self) -> bool {
        match RCC.cr().pllrdy() {
            0b0 => false,
            0b1 => true,
            _ => unimplemented!(),
        }
    }

    pub fn wait_pll_rdy(&self) -> &Self {
        while !self.pll_rdy() {}
        self
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

    pub fn sysclk_src(&self) -> SysClockSrc {
        match RCC.cfgr().sws() {
            0b00 => SysClockSrc::Hsi,
            0b01 => SysClockSrc::Hse,
            0b10 => SysClockSrc::Pll,
            _ => unimplemented!()
        }
    }

    pub fn set_sysclk_src(&self, value: SysClockSrc) -> &Self {
        RCC.with_cfgr(|r| r.set_sw(value as u32));
        self
    }

    pub fn wait_sysclk_rdy(&self) -> &Self {
        loop {
            let cfgr = RCC.cfgr();
            if cfgr.sws() == cfgr.sw() {
                return self
            }
        }
    }


    pub fn sysclk(&self) -> Hz {
        match self.sysclk_src() {
            SysClockSrc::Hsi => self.hsi(),
            SysClockSrc::Hse => self.hse(),
            SysClockSrc::Pll => self.pllclk(),
        }
    }

    pub fn hclk_pre(&self) -> HPre {
        match RCC.cfgr().hpre() {
            0b0000 ... 0b0111 => HPre::Div1,
            0b1000 => HPre::Div2,
            0b1001 => HPre::Div4,
            0b1010 => HPre::Div8,
            0b1011 => HPre::Div16,
            0b1100 => HPre::Div64,
            0b1101 => HPre::Div128,
            0b1110 => HPre::Div256,
            0b1111 => HPre::Div512,
            _ => unimplemented!(),            
        }        
    }

    pub fn set_hclk_pre(&self, value: HPre) -> &Self {
        RCC.with_cfgr(|r| r.set_hpre(value as u32));
        self
    }    

    pub fn hclk(&self) -> Hz {
        // Note: Divide by 32 is skipped
        let hclk_shift = match RCC.cfgr().hpre() {
            0b0000 ... 0b0111 => 0,
            0b1000 => 1,
            0b1001 => 2,
            0b1010 => 3,
            0b1011 => 4,
            0b1100 => 6,
            0b1101 => 7,
            0b1110 => 8,
            0b1111 => 9,
            _ => unimplemented!(),
        };
        self.sysclk().map(|v| v >> hclk_shift)
    }

    pub fn pclk1_pre(&self) -> PPre1 {
        match RCC.cfgr().ppre1() {
            0b000 ... 0b011 => PPre1::Div1,
            0b100 => PPre1::Div2,
            0b101 => PPre1::Div4,
            0b110 => PPre1::Div8,
            0b111 => PPre1::Div16,
            _ => unimplemented!(),            
        }        
    }

    pub fn set_pclk1_pre(&self, value: PPre1) -> &Self {
        RCC.with_cfgr(|r| r.set_ppre1(value as u32));
        self
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

    pub fn pclk2_pre(&self) -> PPre2 {
        match RCC.cfgr().ppre2() {
            0b000 ... 0b011 => PPre2::Div1,
            0b100 => PPre2::Div2,
            0b101 => PPre2::Div4,
            0b110 => PPre2::Div8,
            0b111 => PPre2::Div16,
            _ => unimplemented!(),            
        }        
    }

    pub fn set_pclk2_pre(&self, value: PPre1) -> &Self {
        RCC.with_cfgr(|r| r.set_ppre2(value as u32));
        self
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

    pub fn rtcclk_sel(&self) -> RtcSel {
        match RCC.bdcr().rtcsel() {
            0b00 => RtcSel::None,
            0b01 => RtcSel::Lse,
            0b10 => RtcSel::Lsi,
            0b11 => RtcSel::HseDiv128,
            _ => unimplemented!(),
        }
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

    pub fn clock<P: Clock>(&self, p: &P) -> Hz {
        p.clock(self)
    }
}

pub trait Clock {
    fn clock(&self, clk: &ClockTree) -> Hz;
}

impl Clock for Usart1 {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.pclk2()
    }
}

impl Clock for Usart2 {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.pclk1()
    }
}

impl Clock for Usart3 {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.pclk1()
    }
}

impl Clock for Tim1 {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.timclk_apb2()
    }
}

impl Clock for Tim2 {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.timclk_apb1()
    }
}

impl Clock for Tim3 {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.timclk_apb1()
    }
}

impl Clock for Tim4 {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.timclk_apb1()
    }
}

impl Clock for Iwdg {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.iwdgclk()
    }
}

impl Clock for Wwdg {
    fn clock(&self, clk: &ClockTree) -> Hz {
        clk.pclk1()
    }
}