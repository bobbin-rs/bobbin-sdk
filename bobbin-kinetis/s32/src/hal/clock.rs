use chip::scg::{self, SCG};
use chip::pcc::PCC;
use chip::lpuart::*;
use chip::lpit::*;
use chip::ftm::*;
use core::fmt;

// 8 MHz Crystal on PTB6 / PTB7
//   into SOSC

//
// Target clock configuration is normal RUN mode
// VCO_CLK = 320MHz, SPLL_CLK = 160MHz
//   CORE_CLK = 80Mhz
//   SYS_CLK = 80MHz
//   BUS_CLK = 40MHz
//   FLASH_CLK = 26.67Mhz

// SCG Configuration
//    SCG_SOSCCFG[RANGE] = 0b11 (High Frequency Range)
//    

// Transitions
//   RESET = FIRC
//   FIRC => SOSC
//   SOSC => SYSPLL

// System Oscillator Clock (SOSC) mode is entered when all the following conditions occur:
// • RUN MODE: 0001 is written to RCCR[SCS].
// HSRUN MODE: 0001 is written to HCCR[SCS].
// • SOSCEN = 1
// • SOSCVLD = 1
// In SOSC mode, SCGCLKOUT and system clocks are derived from the external System Oscillator Clock (SOSC).

// Sys PLL (SPLL) mode is entered when all the following conditions occur:
// • RUN MODE: 0110 is written to RCCR[SCS].
// HSRUN MODE: 0110 is written to HCCR[SCS].
// • SPLLEN = 1
// • SPLLVLD = 1

// In SPLL mode, the SCGCLKOUT and system clocks are derived from the output of PLL which is controlled 
// by the System Oscillator (SOSC) clock. The selected PLL clock frequency locks to a multiplication factor, 
// as specified by its corresponding SCG_SPLLCFG[MULT], times the selected PLL reference frequency. 
// The PLL's programmable reference divider must be configured to produce a valid PLL reference clock. 
// This divide value is defined by the SCG_SPLLCFG[PREDIV] bits.

pub fn init() {

    // Setup SOSC

    // Set SOSC Output Dividers
    SCG.set_soscdiv(scg::Soscdiv(0)
        .set_soscdiv1(0b001) // Divide by 1
        .set_soscdiv2(0b001) // Divide by 1
    );

    // Set SOSC Configuration
    SCG.set_sosccfg(scg::Sosccfg(0)
        .set_range(0b11) // High Speed Range
        .set_hgo(0) // Low Gain
        .set_erefs(1) // Use Internal Reference Clock
    );

    // Enable SOSC
    SCG.set_sosccsr(scg::Sosccsr(0)
        .set_soscen(1)
    );

    // Wait for SOSC Valid
    while SCG.sosccsr().soscvld() == 0 {}

    // Setup SPLL

    // Disable PLL
    SCG.with_spllcsr(|r| r.set_spllen(0));

    // Set PLL Output Dividers
    SCG.set_splldiv(scg::Splldiv(0)
        .set_splldiv1(0b010) // Divide by 2
        .set_splldiv2(0b011) // Divide by 4
    );

    // Set PLL Configuration
    SCG.set_spllcfg(scg::Spllcfg(0)
        .set_prediv(0b000) // Divide by 1
        .set_mult(0b11000) // Multiply by 40
    );

    // Enable PLL
    SCG.with_spllcsr(|r| r.set_spllen(1));

    // Wait for PLL Valid
    while SCG.spllcsr().spllvld() == 0 {}

    // Switch to SPLL and set multipliers
    SCG.set_rccr(scg::Rccr(0)
        .set_scs(0b0110)
        .set_divcore(0b0001)
        .set_divbus(0b0001)
        .set_divslow(0b0010)
    );
    // Wait for clock source to be set
    while SCG.csr().scs() != 0b0110 {}   
}

pub const LPO: Hz = Some(128_000);
pub const SIRC: Hz = Some(8_000_000);
pub const FIRC: Hz = Some(48_000_000);

pub type Hz = Option<u32>;

pub trait ClockTree {
    fn prediv_sys_clk(&self) -> Hz;
    fn core_clk(&self) -> Hz;
    fn sys_clk(&self) -> Hz;
    fn bus_clk(&self) -> Hz;
    fn flash_clk(&self) -> Hz;
    fn splldiv1_clk(&self) -> Hz;    
    fn splldiv2_clk(&self) -> Hz;    
    fn fircdiv1_clk(&self) -> Hz;    
    fn fircdiv2_clk(&self) -> Hz;    
    fn sircdiv1_clk(&self) -> Hz;    
    fn sircdiv2_clk(&self) -> Hz;    
    fn soscdiv1_clk(&self) -> Hz;    
    fn soscdiv2_clk(&self) -> Hz;    
    // fn lpo_clk(&self) -> Hz;
    // fn rtc_clkout(&self) -> Hz;
}

pub struct DynamicClock {
    pub xosc: Hz,
    pub xrtc: Hz,
}

impl DynamicClock {
    fn sirc_clk(&self) -> Hz { SIRC }
    fn firc_clk(&self) -> Hz { FIRC }
    fn spll_clk(&self) -> Hz {
        let spllcfg = SCG.spllcfg();
        let div = spllcfg.prediv() + 1;
        let mul = spllcfg.mult() + 16;
        self.sosc_clk().map(|v| (v * mul / div) >> 1)
    }
    fn sosc_clk(&self) -> Hz {
        self.xosc
    }    
}

impl ClockTree for DynamicClock {
    fn prediv_sys_clk(&self) -> Hz {
        match SCG.csr().scs() {
            0b0001 => self.sosc_clk(),
            0b0010 => self.sirc_clk(),
            0b0011 => self.firc_clk(),
            0b0110 => self.spll_clk(),
            _ => unimplemented!(),
        }
    }
    fn core_clk(&self) -> Hz {
        self.prediv_sys_clk().map(|v| v / (1 + SCG.csr().divcore()))
    }

    fn sys_clk(&self) -> Hz {
        self.core_clk()
    }

    fn bus_clk(&self) -> Hz {
        self.core_clk().map(|v| v / (1 + SCG.csr().divbus()))
    }

    fn flash_clk(&self) -> Hz {
        self.core_clk().map(|v| v / (1 + SCG.csr().divslow()))
    }
    
    fn splldiv1_clk(&self) -> Hz {
        match SCG.splldiv().splldiv1() {
            0 => None,
            div @ _ => self.spll_clk().map(|v| v >> (div - 1))
        }    
    }

    fn splldiv2_clk(&self) -> Hz {
        match SCG.splldiv().splldiv2() {
            0 => None,
            div @ _ => self.spll_clk().map(|v| v >> (div - 1))
        }    
    }    

    fn fircdiv1_clk(&self) -> Hz {
        match SCG.fircdiv().fircdiv1() {
            0 => None,
            div @ _ => self.firc_clk().map(|v| v >> (div - 1))
        }    
    }

    fn fircdiv2_clk(&self) -> Hz {
        match SCG.fircdiv().fircdiv2() {
            0 => None,
            div @ _ => self.firc_clk().map(|v| v >> (div - 1))
        }     
    }        

    fn sircdiv1_clk(&self) -> Hz {
        match SCG.sircdiv().sircdiv1() {
            0 => None,
            div @ _ => self.sirc_clk().map(|v| v >> (div - 1))
        }    
    }

    fn sircdiv2_clk(&self) -> Hz {
        match SCG.sircdiv().sircdiv2() {
            0 => None,
            div @ _ => self.sirc_clk().map(|v| v >> (div - 1))
        }    
    }     

    fn soscdiv1_clk(&self) -> Hz {
        match SCG.soscdiv().soscdiv1() {
            0 => None,
            div @ _ => self.sosc_clk().map(|v| v >> (div - 1))
        }    
    }

    fn soscdiv2_clk(&self) -> Hz {
        match SCG.soscdiv().soscdiv2() {
            0 => None,
            div @ _ => self.sosc_clk().map(|v| v >> (div - 1))
        }    
    }

    // fn lpo_clk(&self) -> Hz {
    //     match SIM.lpoclks().lpoclksel() {
    //         0b00 => Some(128_000_000),
    //         0b01 => unimplemented!(),
    //         0b10 => Some(32_000_000),
    //         0b11 => Some(1_000),
    //         _ => unimplemented!(),
    //     }
    // }

    // fn rtc_clk(&self) -> Hz {
    // }
}


impl fmt::Debug for DynamicClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DynamicCLock")?;
        write!(f, " SIRC_CLK={:?}", self.sirc_clk())?;
        write!(f, " FIRC_CLK={:?}", self.firc_clk())?;
        write!(f, " SOSC_CLK={:?}", self.sosc_clk())?;
        write!(f, " SPLL_CLK={:?}", self.spll_clk())?;        
        write!(f, " PREDIV_SYS={:?}", self.prediv_sys_clk())?;
        write!(f, " CORE={:?}", self.core_clk())?;
        write!(f, " SYS={:?}", self.sys_clk())?;
        write!(f, " BUS={:?}", self.bus_clk())?;
        write!(f, " FLASH={:?}", self.flash_clk())?;
        write!(f, " SPLLDIV1={:?}", self.splldiv1_clk())?;
        write!(f, " SPLLDIV2={:?}", self.splldiv2_clk())?;
        write!(f, " FIRCDIV1={:?}", self.fircdiv1_clk())?;
        write!(f, " FIRCDIV2={:?}", self.fircdiv2_clk())?;
        write!(f, " SIRCDIV1={:?}", self.sircdiv1_clk())?;
        write!(f, " SIRCDIV2={:?}", self.sircdiv2_clk())?;
        write!(f, " SOSCDIV1={:?}", self.soscdiv1_clk())?;
        write!(f, " SOSCDIV2={:?}", self.soscdiv2_clk())?;
        write!(f, "]")?;
        Ok(())
    }
}

pub trait Clock<T: ClockTree> {
    fn clock(&self, t: &T) -> Hz;
}

macro_rules! impl_clock {
    ($t:ty, $id:ident) => (
        impl<T: ClockTree> Clock<T> for $t {
            fn clock(&self, t: &T) -> Hz {
                let cfg = PCC.$id();
                if cfg.cgc() == 0 {
                    None
                } else {
                    match cfg.pcs() {
                        0b000 => None,
                        0b001 => t.soscdiv2_clk(),
                        0b010 => t.sircdiv2_clk(),
                        0b011 => t.fircdiv2_clk(),
                        0b110 => t.splldiv2_clk(),
                        _ => unimplemented!(),
                    }
                }
            }
        }
    )
}

impl_clock!(Lpuart0, lpuart0);
impl_clock!(Lpuart1, lpuart1);
impl_clock!(Lpuart2, lpuart2);
impl_clock!(Lpit0, lpit);