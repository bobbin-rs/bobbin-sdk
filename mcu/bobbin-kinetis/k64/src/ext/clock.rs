pub use ::clock::*;
pub use systick_ext::SystickHz;

use bobbin_common::bits::*;
use mcu::sim::SIM;
use mcu::osc::OSC;
use mcu::mcg::MCG;

#[derive(Default)]
pub struct DynamicClock<EXTAL: Clock, EXTAL32: Clock>(EXTAL, EXTAL32);

impl<EXTAL: Clock, EXTAL32: Clock> DynamicClock<EXTAL, EXTAL32> {
    #[allow(dead_code)]
    fn core(&self) -> Hz {
        let outdiv1: u32 = SIM.clkdiv1().outdiv1().into();;
        self.mcgoutclk() / (outdiv1 + 1)
    }

    fn ircclk(&self) -> Hz {
        if MCG.c2().ircs() != 0 {
            let fcrdiv: u32 = MCG.sc().fcrdiv().into();
            self.irc4m() >> fcrdiv
        } else {
            self.irc32k()
        }
    }

    fn oscselclk(&self) -> Hz {
        match MCG.c7().oscsel() {
            U2::B00 => self.oscclk(),
            U2::B01 => self.rtc32k(),
            U2::B10 => self.irc48mclk(),
            _ => panic!("Invalid Value"),
        }
    }

    fn mcgffclk(&self) -> Hz {
        if MCG.s().irefst() != 0 {
            self.irc32k()
        } else {
            self.oscselclk() / (MCG.c1().frdiv() as u32)
        }
    }        

    fn mcgoutclk(&self) -> Hz {
        match MCG.s().clkst() {
            U2::B00 => self.mcgfllclk(),
            U2::B01 => self.ircclk(),
            U2::B10 => self.oscselclk(),
            U2::B11 => self.mcgpllclk(),
        }
    }        

    fn mcgfllclk(&self) -> Hz {
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
        (self.mcgffclk() / div) * mul
    }        

    fn mcgpllclk(&self) -> Hz {        
        if MCG.s().lock0() != 0 {
            let prdiv0: u32 = MCG.c5().prdiv0().into();
            let vdiv0: u32 = MCG.c6().vdiv0().into();
            let div: u32 = prdiv0 + 1u32;
            let mul: u32 = vdiv0 + 24u32;
            (self.oscselclk() / div) * mul
        } else {
            Hz::from(0)
        }
        
    }        

    fn irc48mclk(&self) -> Hz {
        if SIM.sopt2().pllfllsel() == 0b11 {
            self.irc48m()
        } else if MCG.c7().oscsel() == 0b10 {
            self.irc48m()
        } else {
            Hz::from(0)
        }
    }

    fn oscclk(&self) -> Hz {
        if MCG.c2().erefs() != 0 && MCG.s().oscinit0() != 0 {
            // check if osc is active
            // S[OSCINIT0]
            self.extal()
        } else {
            self.extal()
        }
    }        

    #[allow(dead_code)]
    fn osc32kclk(&self) -> Hz {
        // Only handling case when external clock is used
        unimplemented!()
    }

    fn rtc32k(&self) -> Hz {
        // RTC_CR[OSCE]
        unimplemented!()
    }

    #[allow(dead_code)]
    fn rtc(&self) -> Hz {
        unimplemented!()
    }

}

impl<EXTAL: Clock, EXTAL32: Clock> ClockProvider for DynamicClock<EXTAL, EXTAL32> {
    type Extal = EXTAL;
    type Extal32 = EXTAL32;

    fn system(&self) -> Hz {
        let outdiv1: u32 = SIM.clkdiv1().outdiv1().into();
        self.mcgoutclk() / (outdiv1 + 1)
    }

    fn bus(&self) -> Hz {
        let outdiv2: u32 = SIM.clkdiv1().outdiv2().into();
        self.mcgoutclk() / (outdiv2 + 1)
    }

    fn flexbus(&self) -> Hz {
        let outdiv3: u32 = SIM.clkdiv1().outdiv3().into();
        self.mcgoutclk() / (outdiv3 + 1)
    }    

    fn flash(&self) -> Hz {
        let outdiv4: u32 = SIM.clkdiv1().outdiv4().into();
        self.mcgoutclk() / (outdiv4 + 1)
    }     

    fn mcgirclk(&self) -> Hz {
        if MCG.c1().irclken() != 0 {
            self.ircclk()
        } else {
            Hz::from(0)
        }
    }        

    fn oscerclk(&self) -> Hz {
        if OSC.cr().erclken() != 0 {
            self.oscclk()
        } else {
            Hz::from(0)
        }
    }

    fn erclk32k(&self) -> Hz {
        unimplemented!()
    }   

    fn systick(&self) -> Hz {
        self.system()
    }

}

impl<EXTAL: Clock, EXTAL32: Clock> SystickHz for DynamicClock<EXTAL, EXTAL32> {
    fn systick_hz(&self) -> Hz {
        self.systick()
    }
}
