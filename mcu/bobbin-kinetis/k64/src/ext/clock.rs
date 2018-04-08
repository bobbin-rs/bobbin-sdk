pub use ::clock::*;
use bobbin_common::bits::*;
use mcu::sim::SIM;
use mcu::osc::OSC;
use mcu::mcg::MCG;

pub struct DynamicClock<EXTAL: Clock, EXTAL32: Clock>(EXTAL, EXTAL32);

impl<EXTAL: Clock, EXTAL32: Clock> DynamicClock<EXTAL, EXTAL32> {
    #[allow(dead_code)]
    fn core() -> Hz {
        let outdiv1: u32 = SIM.clkdiv1().outdiv1().into();;
        Self::mcgoutclk() / (outdiv1 + 1)
    }

    fn ircclk() -> Hz {
        if MCG.c2().ircs() != 0 {
            let fcrdiv: u32 = MCG.sc().fcrdiv().into();
            Self::irc4m() >> fcrdiv
        } else {
            Self::irc32k()
        }
    }

    fn oscselclk() -> Hz {
        match MCG.c7().oscsel() {
            U2::B00 => Self::oscclk(),
            U2::B01 => Self::rtc32k(),
            U2::B10 => Self::irc48mclk(),
            _ => panic!("Invalid Value"),
        }
    }

    fn mcgffclk() -> Hz {
        if MCG.s().irefst() != 0 {
            Self::irc32k()
        } else {
            Self::oscselclk() / (MCG.c1().frdiv() as u32)
        }
    }        

    fn mcgoutclk() -> Hz {
        match MCG.s().clkst() {
            U2::B00 => Self::mcgfllclk(),
            U2::B01 => Self::ircclk(),
            U2::B10 => Self::oscselclk(),
            U2::B11 => Self::mcgpllclk(),
        }
    }        

    fn mcgfllclk() -> Hz {
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
        (Self::mcgffclk() / div) * mul
    }        

    fn mcgpllclk() -> Hz {        
        if MCG.s().lock0() != 0 {
            let prdiv0: u32 = MCG.c5().prdiv0().into();
            let vdiv0: u32 = MCG.c6().vdiv0().into();
            let div: u32 = prdiv0 + 1u32;
            let mul: u32 = vdiv0 + 24u32;
            (Self::oscselclk() / div) * mul
        } else {
            Hz::from(0)
        }
        
    }        

    fn irc48mclk() -> Hz {
        if SIM.sopt2().pllfllsel() == 0b11 {
            Self::irc48m()
        } else if MCG.c7().oscsel() == 0b10 {
            Self::irc48m()
        } else {
            Hz::from(0)
        }
    }

    fn oscclk() -> Hz {
        if MCG.c2().erefs() != 0 && MCG.s().oscinit0() != 0 {
            // check if osc is active
            // S[OSCINIT0]
            Self::extal()
        } else {
            Self::extal()
        }
    }        

    #[allow(dead_code)]
    fn osc32kclk() -> Hz {
        // Only handling case when external clock is used
        unimplemented!()
    }

    fn rtc32k() -> Hz {
        // RTC_CR[OSCE]
        unimplemented!()
    }

    #[allow(dead_code)]
    fn rtc() -> Hz {
        unimplemented!()
    }

}

impl<EXTAL: Clock, EXTAL32: Clock> Clocks for DynamicClock<EXTAL, EXTAL32> {
    type Extal = EXTAL;
    type Extal32 = EXTAL32;

    fn system() -> Hz {
        let outdiv1: u32 = SIM.clkdiv1().outdiv1().into();
        Self::mcgoutclk() / (outdiv1 + 1)
    }

    fn bus() -> Hz {
        let outdiv2: u32 = SIM.clkdiv1().outdiv2().into();
        Self::mcgoutclk() / (outdiv2 + 1)
    }

    fn flexbus() -> Hz {
        let outdiv3: u32 = SIM.clkdiv1().outdiv3().into();
        Self::mcgoutclk() / (outdiv3 + 1)
    }    

    fn flash() -> Hz {
        let outdiv4: u32 = SIM.clkdiv1().outdiv4().into();
        Self::mcgoutclk() / (outdiv4 + 1)
    }     

    fn mcgirclk() -> Hz {
        if MCG.c1().irclken() != 0 {
            Self::ircclk()
        } else {
            Hz::from(0)
        }
    }        

    fn oscerclk() -> Hz {
        if OSC.cr().erclken() != 0 {
            Self::oscclk()
        } else {
            Hz::from(0)
        }
    }

    fn erclk32k() -> Hz {
        unimplemented!()
    }   

    fn systick() -> Hz {
        Self::system()
    }

}