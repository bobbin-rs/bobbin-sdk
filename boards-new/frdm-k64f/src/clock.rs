pub use mcu::clock::*;

pub fn init() {
    clock_init::init();
}

pub fn tree() -> Tree {
    TREE
}

use common::bits::*;
use mcu::sim::SIM;
use mcu::osc::OSC;
use mcu::mcg::MCG;


static mut EXTAL_HZ: Hz = Hz::from_num(50_000_000);
static mut EXTAL32_HZ: Hz = Hz::from_num(32767);

tree_impl! {
    id: TREE,
    defn: TREE_DEFN: TreeDefn,
    clock_impl: {
        Extal:     ClkExtal      { extal() },
        Extal32:   ClkExtal32    { extal32() },
        Irc48m:    ClkIrc48m     { irc48m() },
        Irc4m:     ClkIrc4m      { irc4m() },
        Irc32k:    ClkIrc32k     { irc32k() },
        Lpo:       ClkLpo        { lpo() },
        

        System:    ClkSystem     { system() },
        Bus:       ClkBus        { bus() },
        Flexbus:   ClkFlexbus    { flexbus() },
        Flash:     ClkFlash      { flash() },
        Mcgirclk:  ClkMcgirclk   { mcgirclk() },
        Erclk32k:  ClkErclk32k   { erclk32k() },
        Oscerclk:  ClkOscerclk   { oscerclk() },
        Systick:   ClkSystick    { systick() },
    }
}

pub fn extal() -> Hz {
    unsafe { EXTAL_HZ }
}

pub fn extal32() -> Hz {
    unsafe { EXTAL32_HZ }
}

pub fn irc48m() -> Hz {
    IRC48M_HZ
}

pub fn irc4m() -> Hz {
    IRC4M_HZ
}

pub fn irc32k() -> Hz {
    IRC32K_HZ
}

pub fn lpo() -> Hz {
    LPO_HZ
}

pub fn core() -> Hz {
    let outdiv1: u32 = SIM.clkdiv1().outdiv1().into();;
    mcgoutclk() / (outdiv1 + 1)
}

pub fn ircclk() -> Hz {
    if MCG.c2().ircs() != 0 {
        let fcrdiv: u32 = MCG.sc().fcrdiv().into();
        irc4m() >> fcrdiv
    } else {
        irc32k()
    }
}

pub fn oscselclk() -> Hz {
    match MCG.c7().oscsel() {
        U2::B00 => oscclk(),
        U2::B01 => rtc32k(),
        U2::B10 => irc48mclk(),
        _ => panic!("Invalid Value"),
    }
}

pub fn mcgffclk() -> Hz {
    if MCG.s().irefst() != 0 {
        irc32k()
    } else {
        oscselclk() / (MCG.c1().frdiv() as u32)
    }
}        

pub fn mcgoutclk() -> Hz {
    match MCG.s().clkst() {
        U2::B00 => mcgfllclk(),
        U2::B01 => ircclk(),
        U2::B10 => oscselclk(),
        U2::B11 => mcgpllclk(),
    }
}        

pub fn mcgfllclk() -> Hz {
    let c4 = MCG.c4();
    let div = MCG.c1().frdiv() as u32;
    if div == 0 {
        return Hz::from(0)
    }
    let div = if MCG.c2().range() == 0 || MCG.c7().oscsel() == 1 {
        div
    } else {
        div << 5
    };
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
    (mcgffclk() / div) * mul
}        

pub fn mcgpllclk() -> Hz {        
    if MCG.s().lock0() != 0 {
        let prdiv0: u32 = MCG.c5().prdiv0().into();
        let vdiv0: u32 = MCG.c6().vdiv0().into();
        let div: u32 = prdiv0 + 1u32;
        let mul: u32 = vdiv0 + 24u32;
        (oscselclk() / div) * mul
    } else {
        Hz::from(0)
    }
    
}        

pub fn irc48mclk() -> Hz {
    if SIM.sopt2().pllfllsel() == 0b11 {
        irc48m()
    } else if MCG.c7().oscsel() == 0b10 {
        irc48m()
    } else {
        Hz::from(0)
    }
}

pub fn oscclk() -> Hz {
    if MCG.c2().erefs() != 0 && MCG.s().oscinit0() != 0 {
        // check if osc is active
        // S[OSCINIT0]
        extal()
    } else {
        extal()
    }
}        

pub fn osc32kclk() -> Hz {
    // Only handling case when external clock is used
    unimplemented!()
}

pub fn rtc32k() -> Hz {
    // RTC_CR[OSCE]
    unimplemented!()
}

pub fn rtc() -> Hz {
    unimplemented!()
}

pub fn system() -> Hz {
    let outdiv1: u32 = SIM.clkdiv1().outdiv1().into();
    mcgoutclk() / (outdiv1 + 1)
}

pub fn bus() -> Hz {
    let outdiv2: u32 = SIM.clkdiv1().outdiv2().into();
    mcgoutclk() / (outdiv2 + 1)
}

pub fn flexbus() -> Hz {
    let outdiv3: u32 = SIM.clkdiv1().outdiv3().into();
    mcgoutclk() / (outdiv3 + 1)
}    

pub fn flash() -> Hz {
    let outdiv4: u32 = SIM.clkdiv1().outdiv4().into();
    mcgoutclk() / (outdiv4 + 1)
}     

pub fn mcgirclk() -> Hz {
    if MCG.c1().irclken() != 0 {
        ircclk()
    } else {
        Hz::from(0)
    }
}        

pub fn oscerclk() -> Hz {
    if OSC.cr().erclken() != 0 {
        oscclk()
    } else {
        Hz::from(0)
    }
}

pub fn erclk32k() -> Hz {
    unimplemented!()
}   

pub fn systick() -> Hz {
    unimplemented!()
}

pub mod clock_init {
    use mcu::sim::SIM;
    use mcu::osc::OSC;
    use mcu::mcg::{self, MCG};

    pub fn init() {
        // OSC.set_erclk_enable(1);
        OSC.with_cr(|cr| cr.set_erclken(1));
        // SIM.set_clkdiv(0x0, 0x1, 0x2, 0x3);
        SIM.with_clkdiv1(|r| r.set_outdiv1(0x0).set_outdiv2(0x1).set_outdiv3(0x2).set_outdiv4(0x4));

        set_mode_fbe();
        set_mode_pbe();        
        set_mode_pee();
    }

    // External Clock 50MHz
    // Boot Clock Mode (Fast clock boot) = about 21MHz
    //
    // Core / System Clock = 0x0 (Divide by 1)
    // Bus Clock = 0x0 (Divide by 1)
    // Flexbus Clock = 0x1 (Divide by 2)
    // Flash Clock = 0x1 (Divide by 2)
    // MCG in FEI Mode (FLL Engaged Internal)
    //
    // In FEI mode, MCGOUTCLK is derived from the FLL clock (DCOCLK) that is controlled by
    // the 32 kHz Internal Reference Clock (IRC). The FLL loop will lock the DCO frequency to
    // the FLL factor, as selected by C4[DRST_DRS] and C4[DMX32] bits, times the internal 
    // reference frequency. See the C4[DMX32] bit description for more details. In FEI mode,
    // the PLL is disabled in a low-power state unless C5[PLLCLKEN] is set .


    // Standard Clock Mode
    // Core Clock: 120MHz = 0x0 (Divide by 1)
    // System Clock: 120MHz = 0x0 (Divide by 1)
    // Bus Clock: 60MHz = 0x1 (Divide by 2)
    // Flexbus Clock: 40MHz = 0x2 (Divide by 3)
    // Flash Clock: 24MHz = = 0x4 (Divide by 5)


    // From http://www.utasker.com/kinetis/MCG.html
    //
    //    #else                                                                // run from external clock (typical configuration when Ethernet is required)
    //         #define EXTERNAL_CLOCK       50000000                            // this must be 50MHz in order to use Ethernet in RMII mode
    //         #define _EXTERNAL_CLOCK      EXTERNAL_CLOCK
    //         #define CLOCK_DIV            20                                  // input must be divided to 2MHz..4MHz range (/1 to /24)
    //         #define CLOCK_MUL            48                                  // the PLL multiplication factor to achieve operating frequency of 120MHz (x24 to x55 possible)
    //         #define FLEX_CLOCK_DIVIDE    3                                   // 120/3 to give 40MHz
    //         #define FLASH_CLOCK_DIVIDE   5                                   // 120/5 to give 24MHz
    //     #endif
    
    pub fn set_mode_fbe() {
        // Switch to FBE (FLL Bypassed External) Mode

        // CLKS=2: Encoding 2 — External reference clock is selected.
        // FRDIV=3: If RANGE 0 = 0 , Divide Factor is 8; for all other RANGE 0 values, Divide Factor is 256.
        // IREFS=0: External reference clock is selected.
        // IRCLKEN=1: MCGIRCLK active.
        // IREFSTEN=0: Internal reference clock is disabled in Stop mode.
        //MCG_C1 = (MCG_C1_CLKS(0x02) | MCG_C1_FRDIV(0x03) | MCG_C1_IRCLKEN_MASK);                                                   

        MCG.set_c1(|r| r.set_clks(0x02).set_frdiv(0x03).set_irclken(1));

        // LOREC0=0: Interrupt request is generated on a loss of OSC0 external reference clock.
        // RANGE0=2: Very high frequency range selected for the crystal oscillator.
        // HGO=0: Configure crystal oscillator for low-power operation.
        // ERFS0=1: External Reference Select - Oscillator requested.
        // LP=0: FLL or PLL is not disabled in bypass modes.
        // IRCS=0: Slow internal reference clock selected.
        //MCG_C2 = (MCG_C2_RANGE0(0x02) | MCG_C2_EREFS0_MASK);                                                   

        MCG.set_c2(|r| r.set_range(0x02).set_erefs(1));

        // DMX32=0: Reference Range 31.25–39.0625 kHz / FLL Factor 640 / DCO Range 20–25 MHz
        // DRST_DRS=0: Encoding 0 — Low range (reset default).
        //MCG_C4 &= (uint8_t)~(uint8_t)((MCG_C4_DMX32_MASK | MCG_C4_DRST_DRS(0x03)));

        MCG.set_c4(|r| r.set_dmx32(0).set_drst_drs(0x3));

        // PLLCLKEN0=0: MCGPLLCLK is inactive.
        // PLLSTEN0=0: MCGPLLCLK is disabled in any of the Stop modes.
        // PRDIV0=1: Divide Factor 2
        //MCG_C5 = MCG_C5_PRDIV0(0x01);
        MCG.set_c5(|r| r.set_prdiv0(0x01));

        // LOLIE0=0: No interrupt request is generated on loss of lock.
        // PLLS=0: FLL is selected.
        // CME0=0: External clock monitor is disabled for OSC0.
        // VDIV0=0: Multiply Factor 24
        //MCG_C6 = 0x00U;

        MCG.set_c6(|r| r);

        /* Wait until the source of the FLL reference clock is the external reference clock. */
        //while((MCG_S & MCG_S_IREFST_MASK) != 0x00U) {}

        while MCG.s().irefst() != 0 {}

        /* Wait until external reference clock is selected as MCG output */
        //while((MCG_S & 0x0CU) != 0x08U) {}

        while MCG.s().clkst() != 0x2 {}
    }


    pub fn set_mode_pbe() {
        let o = OSC;
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

        m.set_c2(|r| r.set_range(0x2).set_erefs(1));

        /* MCG_C5: ??=0,PLLCLKEN0=0,PLLSTEN0=0,PRDIV0=1 */
        //MCG_C5 = MCG_C5_PRDIV0(0x01);                    

        m.set_c5(|r| r.set_prdiv0(0x13));

        /* MCG_C6: LOLIE0=0,PLLS=1,CME0=0,VDIV0=0 */
        //MCG_C6 = MCG_C6_PLLS_MASK;

        m.set_c6(|_| mcg::C6(0x18).set_plls(1));
        
        /* Wait until external reference clock is selected as MCG output */
        //while((MCG_S & 0x0CU) != 0x08U) {}

        while m.s().clkst() != 0x2 {}

        /* Wait until locked */
        //while((MCG_S & MCG_S_LOCK0_MASK) == 0x00U) {}

        while m.s().lock0() == 0 {}    
    }

    pub fn set_mode_pee() {
        let o = OSC;
        let m = MCG;

        // Switch to PBE (PLL Engaged External) Mode
        
        /* OSC0_CR: ERCLKEN=1,??=0,EREFSTEN=0,??=0,SC2P=0,SC4P=0,SC8P=0,SC16P=0 */
        // OSC0_CR = OSC_CR_ERCLKEN_MASK;                                                   

        o.set_cr(|r| r.set_erclken(1));

        /* MCG_C1: CLKS=0,FRDIV=3,IREFS=0,IRCLKEN=1,IREFSTEN=0 */
        // MCG_C1 = (MCG_C1_FRDIV(0x03) | MCG_C1_IRCLKEN_MASK);    

        m.set_c1(|r| r.set_frdiv(0x3).set_irclken(1));


        /* MCG_C2: LOCRE0=0,??=0,RANGE0=2,HGO0=0,EREFS0=1,LP=0,IRCS=0 */
        // MCG_C2 = (MCG_C2_RANGE0(0x02) | MCG_C2_EREFS0_MASK);

        // m.set_c2(mcg::C2(0).set_range(0x02).set_erefs(1));        
        // *** NOTE: External Reference must be set to use Ethernet ***
        m.set_c2(|r| r.set_range(0x02).set_erefs(0));

        /* MCG_C5: ??=0,PLLCLKEN0=0,PLLSTEN0=0,PRDIV0=1 */
        // MCG_C5 = MCG_C5_PRDIV0(0x01);                       
        
        m.set_c5(|r| r.set_prdiv0(0x13));

        /* MCG_C6: LOLIE0=0,PLLS=1,CME0=0,VDIV0=0 */
        // MCG_C6 = MCG_C6_PLLS_MASK;

        m.set_c6(|_| mcg::C6(0x18).set_plls(1));
        
        /* Wait until output of the PLL is selected */
        // while((MCG_S & 0x0CU) != 0x0CU) {}
        while m.s().clkst() != 0x3 {}    
    }
}