pub use bobbin_common::hal::watchdog::*;
pub use chip::wdog::*;

impl Unlock for Wdog {
    fn unlock(&self) -> &Self {
        self.set_cnt(|_| Cnt(0xD928C520));
        while self.cs().ulk() == 0 {}
        self
    }
}

impl Enable for Wdog {
    fn enable(&self) -> &Self {
        self.with_cs(|r| r.set_en(1))
    }
}

impl Disable for Wdog {
    fn disable(&self) -> &Self {
        self.with_cs(|r| r.set_en(0))
    }
}

impl Timeout<u16> for Wdog {
    fn timeout(&self) -> u16 {
        let toval = self.toval();
        toval.tovallow().into_u16() | (toval.tovalhigh().into_u16() << 8)
    }
}

impl SetTimeout<u16> for Wdog {
    fn set_timeout(&self, value: u16) -> &Self {
        let (lo, hi) = (value as u8, (value >> 8) as u8);
        self.set_toval(|r| r.set_tovallow(lo).set_tovalhigh(hi));
        while self.cs().rcs() != 0 {}
        self
    }
}

impl Counter<u16> for Wdog {
    fn counter(&self) -> u16 {
        let cnt = self.cnt();
        cnt.cntlow().into_u16() | (cnt.cnthigh().into_u16() << 8)
    }
}

impl Refresh for Wdog {
    fn refresh(&self) -> &Self {
        self.set_cnt(|_| Cnt(0xB480A602))
    }
}