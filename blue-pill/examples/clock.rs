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
    loop {}
}

use board::chip::rcc::RCC;

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
        match RCC.cr().hserdy() {
            0b1 => HSI,
            0b0 => None,
            _ => unimplemented!(),
        }
    }

    pub fn hse(&self) -> Hz {
        match RCC.cr().hserdy() {
            0b1 => self.hse_osc,
            0b0 => None,
            _ => unimplemented!(),
        }
    }

    pub fn lsi(&self) -> Hz { 
        match RCC.csr().lsirdy() {
            0b1 => LSI,
            0b0 => None,
            _ => unimplemented!(),
        }
    }

    pub fn lse(&self) -> Hz {
        match RCC.bdcr().lserdy() {
            0b1 => LSE,
            0b0 => None,
            _ => unimplemented!(),
        }
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
    
    pub fn pllclk(&self) -> Hz {
        let cfgr = RCC.cfgr();
        match cfgr.pllsrc() {
            0b0 => self.hsi().map(|v| v >> 1),
            0b1 => {
                match cfgr.pllxtpre() {
                    0b0 => self.hse(),
                    0b1 => self.hse().map(|v| v >> 1),
                    _ => unimplemented!(),
                }    
            },
            _ => unimplemented!(),
        }.map(|v| v * self.pll_mul())        
    }

    pub fn sysclk_src(&self) -> SysClock {
        match RCC.cfgr().sws() {
            0b00 => SysClock::Hsi,
            0b01 => SysClock::Hse,
            0b10 => SysClock::Pll,
            _ => unimplemented!()
        }
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