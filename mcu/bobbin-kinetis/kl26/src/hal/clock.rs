use bobbin_common::bits::*;
use ::chip::osc::*;
use ::chip::mcg::*;
use ::chip::sim::*;
use ::chip::uart0::Uart0;
use ::chip::uart::{Uart1, Uart2};
use ::chip::pit::Pit;
use core::fmt;

static mut SYSCLK_HZ: u32 = 0;

// 8 Mhz Crystal
// SYSCLK = 48Mhz
// BUS Clock = 24MHz

pub fn init() {
    run_48mhz();
}

pub fn run_48mhz() {
    OSC0.with_cr(|r| r.set_erclken(1));
    SIM.with_clkdiv1(|r| r.set_outdiv1(0x1).set_outdiv4(0x1));
    SIM.with_sopt2(|r| r.set_pllfllsel(1));
    set_mode_fbe();
    set_mode_pbe();
    set_mode_pee();
    set_sysclk_hz(48_000_000);
}

pub fn sysclk_hz() -> u32 {
    unsafe { SYSCLK_HZ }
}

pub fn set_sysclk_hz(value: u32) {
    unsafe { SYSCLK_HZ = value }
}

// Multipurpose Clock Generator (MCG)
// See Figure 24-1, Reference Manual Page 370

// Operating Modes must be in sequence FEI -> FBE -> PBE -> PEE
// See 24.4.1, Reference Manual Page 384


// Switch to FBE (FLL Bypassed External) Mode


pub fn set_mode_fbe() {        
    let m = MCG;
    // Switch to FBE (FLL Bypassed External) Mode

    // CLKS=2: Encoding 2 — External reference clock is selected.
    // FRDIV=3: If RANGE 0 = 0 , Divide Factor is 8; for all other RANGE 0 values, Divide Factor is 256.
    // IREFS=0: External reference clock is selected.
    // IRCLKEN=1: MCGIRCLK active.
    // IREFSTEN=0: Internal reference clock is disabled in Stop mode.
    //MCG_C1 = (MCG_C1_CLKS(0x02) | MCG_C1_FRDIV(0x03) | MCG_C1_IRCLKEN_MASK);                                                   

    m.set_c1(|r| r.set_clks(0x02).set_frdiv(0x03).set_irclken(1));


    // LOREC0=0: Interrupt request is generated on a loss of OSC0 external reference clock.
    // RANGE0=2: Very high frequency range selected for the crystal oscillator.
    // HGO=0: Configure crystal oscillator for low-power operation.
    // ERFS0=1: External Reference Select - Oscillator requested.
    // LP=0: FLL or PLL is not disabled in bypass modes.
    // IRCS=0: Slow internal reference clock selected.
    //MCG_C2 = (MCG_C2_RANGE0(0x02) | MCG_C2_EREFS0_MASK);                                                   

    m.set_c2(|r| r.set_range0(0x02).set_erefs0(1));

    // DMX32=0: Reference Range 31.25–39.0625 kHz / FLL Factor 640 / DCO Range 20–25 MHz
    // DRST_DRS=0: Encoding 0 — Low range (reset default).
    //MCG_C4 &= (uint8_t)~(uint8_t)((MCG_C4_DMX32_MASK | MCG_C4_DRST_DRS(0x03)));

    m.set_c4(|r| r.set_dmx32(0).set_drst_drs(0x3));

    // PLLCLKEN0=0: MCGPLLCLK is inactive.
    // PLLSTEN0=0: MCGPLLCLK is disabled in any of the Stop modes.
    // PRDIV0=1: Divide Factor 2
    //MCG_C5 = MCG_C5_PRDIV0(0x01);
    m.set_c5(|r| r.set_prdiv0(0x01));

    // LOLIE0=0: No interrupt request is generated on loss of lock.
    // PLLS=0: FLL is selected.
    // CME0=0: External clock monitor is disabled for OSC0.
    // VDIV0=0: Multiply Factor 24
    //MCG_C6 = 0x00U;

    m.set_c6(|r| r);

    /* Wait until the source of the FLL reference clock is the external reference clock. */
    //while((MCG_S & MCG_S_IREFST_MASK) != 0x00U) {}

    while m.s().irefst() != 0x00 {}

    /* Wait until external reference clock is selected as MCG output */
    //while((MCG_S & 0x0CU) != 0x08U) {}

    while m.s().clkst() != 0x2 {}
}
pub fn set_mode_pbe() {
    let o = OSC0;
    let m = MCG;
    
    // Switch to PBE (PLL Bypassed External) Mode  
    /* OSC0_CR: ERCLKEN=1,??=0,EREFSTEN=0,??=0,SC2P=0,SC4P=0,SC8P=0,SC16P=0 */
    //OSC0_CR = OSC_CR_ERCLKEN_MASK;                                                   

    o.with_cr(|cr| cr.set_erclken(1));


    /* MCG_C1: CLKS=2,FRDIV=3,IREFS=0,IRCLKEN=1,IREFSTEN=0 */
    //MCG_C1 = (MCG_C1_CLKS(0x02) | MCG_C1_FRDIV(0x03) | MCG_C1_IRCLKEN_MASK);                                                   
    
    m.set_c1(|r| r.set_clks(0x2).set_frdiv(0x3).set_irefs(0).set_irclken(1).set_irefsten(0));


    /* MCG_C2: LOCRE0=0,??=0,RANGE0=2,HGO0=0,EREFS0=1,LP=0,IRCS=0 */
    //MCG_C2 = (MCG_C2_RANGE0(0x02) | MCG_C2_EREFS0_MASK);                                                   

    m.set_c2(|r| r.set_range0(0x2).set_erefs0(1));

    /* MCG_C5: ??=0,PLLCLKEN0=0,PLLSTEN0=0,PRDIV0=1 */
    //MCG_C5 = MCG_C5_PRDIV0(0x01);                    

    m.set_c5(|r| r.set_prdiv0(0x1));

    /* MCG_C6: LOLIE0=0,PLLS=1,CME0=0,VDIV0=0 */
    //MCG_C6 = MCG_C6_PLLS_MASK;

    m.set_c6(|r| r.set_plls(1));
    
    /* Wait until external reference clock is selected as MCG output */
    //while((MCG_S & 0x0CU) != 0x08U) {}

    while m.s().clkst() != 0x2 {}

    /* Wait until locked */
    //while((MCG_S & MCG_S_LOCK0_MASK) == 0x00U) {}

    while m.s().lock0() == 0x00 {}
}
pub fn set_mode_pee() {
    let o = OSC0;
    let m = MCG;

    // Switch to PBE (PLL Engaged External) Mode
    
    /* OSC0_CR: ERCLKEN=1,??=0,EREFSTEN=0,??=0,SC2P=0,SC4P=0,SC8P=0,SC16P=0 */
    //OSC0_CR = OSC_CR_ERCLKEN_MASK;                                                   

    o.set_cr(|r| r.set_erclken(1));

    /* MCG_C1: CLKS=0,FRDIV=3,IREFS=0,IRCLKEN=1,IREFSTEN=0 */
    //MCG_C1 = (MCG_C1_FRDIV(0x03) | MCG_C1_IRCLKEN_MASK);    

    m.set_c1(|r| r.set_frdiv(0x3).set_irclken(1));

    /* MCG_C2: LOCRE0=0,??=0,RANGE0=2,HGO0=0,EREFS0=1,LP=0,IRCS=0 */
    //MCG_C2 = (MCG_C2_RANGE0(0x02) | MCG_C2_EREFS0_MASK);
    m.set_c2(|r| r.set_range0(0x02).set_erefs0(1));

    /* MCG_C5: ??=0,PLLCLKEN0=0,PLLSTEN0=0,PRDIV0=1 */
    //MCG_C5 = MCG_C5_PRDIV0(0x01);                       
    m.set_c5(|r| r.set_prdiv0(0x01));

    /* MCG_C6: LOLIE0=0,PLLS=1,CME0=0,VDIV0=0 */
    //MCG_C6 = MCG_C6_PLLS_MASK;

    m.set_c6(|r| r.set_plls(1));
    
    /* Wait until output of the PLL is selected */
    //while((MCG_S & 0x0CU) != 0x0CU) {}
    while m.s().clkst() != 0x3 {}
}

pub const IRC4M: Hz = Some(4_000_000);
pub const IRC32K: Hz = Some(32_000);

pub type Hz = Option<u32>;

pub trait ClockTree {
    fn coreclk(&self) -> Hz;
    fn busclk(&self) -> Hz;
    fn mcgirclk(&self) -> Hz;
    fn mcgfllclk(&self) -> Hz;
    fn mcgpllclk(&self) -> Hz;
    fn oscerclk(&self) -> Hz;
    // fn erclk32k(&self) -> Hz;
}

pub struct DynamicClock {
    pub xtal0: Hz,
}

impl DynamicClock {
    pub fn irefclk(&self) -> Hz {
        if MCG.c2().ircs() != 0 {
            IRC4M.map(|v| v >> MCG.sc().fcrdiv().into_u32())
        } else {
            IRC32K
        }
    }

    pub fn oscclk(&self) -> Hz {
        if MCG.s().oscinit0() != 0 {
            self.xtal0
        } else {
            None
        }
    }

    pub fn mcgoutclk(&self) -> Hz {
        match MCG.s().clkst() {
            U2::B00 => self.mcgfllclk(),
            U2::B01 => self.irefclk(),
            U2::B10 => self.oscclk(),
            U2::B11 => self.mcgpllclk(),
        }
    }
}

impl ClockTree for DynamicClock {
    fn coreclk(&self) -> Hz {
        self.mcgoutclk().map(|v| v / (1 + SIM.clkdiv1().outdiv1().into_u32()))
    }

    fn mcgirclk(&self) -> Hz {
        if MCG.c1().irclken() != 0 {
            self.irefclk()
        } else {
            None
        }
    }

    fn busclk(&self)-> Hz {
        self.coreclk().map(|v| v / ( 1 + SIM.clkdiv1().outdiv4().into_u32()))
    }
    fn mcgfllclk(&self) -> Hz {
        let c1 = MCG.c1();
        
        let fllref = if c1.irefs() != 0 {
            IRC32K
        } else {
            self.oscclk()
        };
        let c2 = MCG.c2();
        let c4 = MCG.c4();            
        let mul = match (c4.drst_drs(), c4.dmx32()) {
            (U2::B00, U1::B0) => 640,
            (U2::B00, U1::B1) => 732,
            (U2::B01, U1::B0) => 1280,
            (U2::B01, U1::B1) => 1464,
            (U2::B10, U1::B0) => 1920,
            (U2::B10, U1::B1) => 2197,
            (U2::B11, U1::B0) => 2560,
            (U2::B11, U1::B1) => 2929,
        };

        let div = if c2.range0() == 0 {
            1 << c1.frdiv().value()
        } else {
            match c1.frdiv() {
                U3::B000 => 32,
                U3::B001 => 64,
                U3::B010 => 128,
                U3::B011 => 256,
                U3::B100 => 512,
                U3::B101 => 1024,
                U3::B110 => 1280,
                U3::B111 => 1536,
            }
        };
        fllref.map(|v| (v / div) * mul)
        // if MCG.c7().oscsel() != 0 {
            
        // } else {
        //     IRC32K.map(|v| (v / div) * mul)
        // }
        // }
    }
    fn mcgpllclk(&self) -> Hz {
        let c5 = MCG.c5();
        let c6 = MCG.c6();
        if MCG.s().lock0() == 0 { return None }
        let mul = c6.vdiv0().into_u32() + 24;
        let div = c5.prdiv0().into_u32() + 1;
        self.oscclk().map(|v| v * mul / div)
    }

    fn oscerclk(&self) -> Hz {
        if OSC0.cr().erclken() != 0 {
            self.oscclk()
        } else {
            None
        }
    }
    // fn erclk32k(&self) -> Hz {
    //     unimplemented!()
    // }
}

impl fmt::Debug for DynamicClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DynamicCLock")?;
        write!(f, " IREFCLK={:?}", self.irefclk())?;
        write!(f, " OSCCLK={:?}", self.oscclk())?;
        write!(f, " MCGOUTCLK={:?}", self.mcgoutclk())?;
        
        // write!(f, " CORE={:?}", self.coreclk())?;
        // write!(f, " BUS={:?}", self.busclk())?;
        write!(f, " MCGIRCLK={:?}", self.mcgirclk())?;
        // write!(f, " MCGFLLCLK={:?}", self.mcgfllclk())?;
        write!(f, " MCGPLLCLK={:?}", self.mcgpllclk())?;
        // write!(f, " OSCERCLK={:?}", self.oscerclk())?;
        write!(f, "]")?;
        Ok(())
    }
}

pub trait Clock<T: ClockTree> {
    fn clock(&self, t: &T) -> Hz;
}

impl<T: ClockTree> Clock<T> for Uart0 {
    fn clock(&self, t: &T) -> Hz {
        let sopt2 = SIM.sopt2();
        match sopt2.uart0src() {
            U2::B00 => None,
            U2::B01 => if sopt2.pllfllsel() == 0 {
                t.mcgfllclk()
            } else {
                t.mcgpllclk().map(|v| v >> 1)
            },
            U2::B10 => t.oscerclk(),
            U2::B11 => t.mcgirclk(),
        }
    }
}

impl<T: ClockTree> Clock<T> for Uart1 {
    fn clock(&self, t: &T) -> Hz {
        t.busclk()
    }
}


impl<T: ClockTree> Clock<T> for Uart2 {
    fn clock(&self, t: &T) -> Hz {
        t.busclk()
    }
}

impl<T: ClockTree> Clock<T> for Pit {
    fn clock(&self, t: &T) -> Hz {
        t.busclk()
    }
}