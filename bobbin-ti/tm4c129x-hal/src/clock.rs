use ::chip::sysctl::*;
use ::chip::uart::*;
use ::chip::timer::*;

use core::fmt;

pub fn set_clock(psysdiv: u16, mint: u16, mfrac: u16, n: u8, q: u8) {
    let s = SYSCTL;

    // MOSC Init
    s.with_moscctl(|r| r.set_noxtal(0).set_oscrng(1).set_pwrdn(0));

    // PLL Init
    s.with_rsclkcfg(|r| r.set_usepll(0));
    s.with_rsclkcfg(|r| r.set_pllsrc(0x3).set_psysdiv(psysdiv as u32));
    s.with_pllfreq0(|r| r.set_pllpwr(1).set_mint(mint as u32).set_mfrac(mfrac as u32));
    s.with_pllfreq1(|r| r.set_n(n as u32).set_q(q as u32));
    s.with_rsclkcfg(|r| r.set_newfreq(1));
    while s.pllstat().lock() == 0 {}
    
    // Memory Init
    s.with_memtim0(|r| 
        r.set_fbcht(0x6)
            .set_ebcht(0x6)
            .set_fws(0x5)
            .set_ews(0x5)                    
    );
    s.with_rsclkcfg(|r| r.set_memtimu(1));

    // Use PLL
    s.with_rsclkcfg(|r| r.set_usepll(1));
}

pub type Hz = Option<u32>;

pub const PIOSC: Hz = Some(16_000_000);
pub const LFIOSC: Hz = Some(33_000);

pub trait ClockTree {
    fn sysclk(&self) -> Hz;
}

pub struct DynamicClock {
    pub osc: Hz,
    pub xosc: Hz,
}

impl DynamicClock {
    // Fundamental Clock Sources
    pub fn mosc(&self) -> Hz {
        self.osc
    }
    pub fn rtcosc(&self) -> Hz {
        self.xosc
    }
    pub fn piosc(&self) -> Hz { 
        PIOSC 
    }
    pub fn lfiosc(&self) -> Hz {
        LFIOSC
    }
    // Internal Clocks
    pub fn oscclk(&self) -> Hz {
        match SYSCTL.rsclkcfg().oscsrc() {
            0 => self.piosc(),
            2 => self.lfiosc(),
            3 => self.mosc(),
            4 => self.rtcosc(),
            _ => unimplemented!(),
        }
    }

    pub fn vco(&self) -> Hz {
        let pllstat = SYSCTL.pllstat();
        if pllstat.lock() == 0 {
            return None
        }
        let rsclkcfg = SYSCTL.rsclkcfg();
        let pllfreq0 = SYSCTL.pllfreq0();
        let pllfreq1 = SYSCTL.pllfreq1();
        let (q, n) = (pllfreq1.q(), pllfreq1.n());
        let f_in = match rsclkcfg.pllsrc() {
            0 => self.piosc(),
            3 => self.mosc(),
            _ => unimplemented!(),
        }.map(|v| v / ((q + 1)  * (n + 1)));
        let mdiv = pllfreq0.mint() + pllfreq0.mfrac() / 1024;
        f_in.map(|v| v * mdiv)
    }
}

impl ClockTree for DynamicClock {
    fn sysclk(&self) -> Hz {
        let rsclkcfg = SYSCTL.rsclkcfg();
        if rsclkcfg.usepll() == 0 {
            self.oscclk().map(|v| v / (SYSCTL.rsclkcfg().osysdiv() + 1))    
        } else {
            self.vco().map(|v| v / (SYSCTL.rsclkcfg().psysdiv() + 1))
        }
    }
}

impl fmt::Debug for DynamicClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DynamicCLock")?;
        write!(f, " MOSC={:?}", self.mosc())?;
        write!(f, " RTCOSC={:?}", self.rtcosc())?;
        write!(f, " PIOSC={:?}", self.piosc())?;
        write!(f, " LFIOSC={:?}", self.lfiosc())?;
        write!(f, " OSCCLK={:?}", self.oscclk())?;
        write!(f, " VCO={:?}", self.vco())?;
        write!(f, " SYSCLK={:?}", self.sysclk())?;
        write!(f, "]")?;
        Ok(())
    }
}
pub trait Clock<T: ClockTree + ?Sized> {
    fn clock(&self, t: &T) -> Hz;
}

macro_rules! impl_clock {
    ($t:ty, $m:ident) => (
        impl<T> Clock<T> for $t where T: ClockTree + ?Sized {
            fn clock(&self, t: &T) -> Hz {
                if self.rcgc() != 0 {
                    t.$m()
                } else {
                    None
                }
            }
        }        
    )
}

impl_clock!(Uart0, sysclk);
impl_clock!(Uart1, sysclk);
impl_clock!(Uart2, sysclk);
impl_clock!(Uart3, sysclk);
impl_clock!(Uart4, sysclk);
impl_clock!(Uart5, sysclk);
impl_clock!(Uart6, sysclk);
impl_clock!(Uart7, sysclk);

impl_clock!(Timer0, sysclk);
impl_clock!(Timer1, sysclk);
impl_clock!(Timer2, sysclk);
impl_clock!(Timer3, sysclk);
impl_clock!(Timer4, sysclk);
impl_clock!(Timer5, sysclk);
impl_clock!(Timer6, sysclk);
impl_clock!(Timer7, sysclk);

