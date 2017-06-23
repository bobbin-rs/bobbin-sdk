pub use chip::wdog::*;

pub enum ClockSource {
    Lpo = 0,
    Alt = 1,
}

pub enum Prescaler {
    Div1 = 0b000,
    Div2 = 0b001,
    Div3 = 0b010,
    Div4 = 0b011,
    Div5 = 0b100,
    Div6 = 0b101,
    Div7 = 0b110,
    Div8 = 0b111,
}

pub struct Config {
    pub source: ClockSource,
    pub prescaler: Prescaler,
    pub timeout: u32,
    pub window: u32,
}

pub trait WdogExt {
    fn configure(&self, cfg: Config) -> &Self;
    fn unlock(&self);
    fn refresh(&self);
    fn set_clock_source(&self, ClockSource) -> &Self;
    fn is_enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;
    fn timeout(&self) -> u32;
    fn set_timeout(&self, u32) -> &Self;
    fn window(&self) -> u32;
    fn set_window(&self, u32) -> &Self;
    fn timer(&self) -> u32;
    fn reset_count(&self) -> u16;
    fn prescaler(&self) -> Prescaler;
    fn set_prescaler(&self, Prescaler) -> &Self;
}

impl<T> WdogExt for Periph<T> {
    fn configure(&self, cfg: Config) -> &Self {
        use core::ptr;
        let stctrlh = Stctrlh(0xd0).set_clksrc(cfg.source as u16).set_wdogen(1);

        unsafe {
            ptr::write_volatile(0x4005_200e as *mut u16, 0xc520);
            ptr::write_volatile(0x4005_200e as *mut u16, 0xd928);
        }
        self.set_prescaler(cfg.prescaler);
        self.set_timeout(cfg.timeout);
        self.set_window(cfg.window);
        self.set_stctrlh(stctrlh);        
        self.refresh();
        self
    }    
    #[inline]
    fn unlock(&self) {
        use core::ptr;
        unsafe {
            ptr::write_volatile(0x4005_200e as *mut u16, 0xc520);
            ptr::write_volatile(0x4005_200e as *mut u16, 0xd928);
        }
    }
    #[inline]
    fn refresh(&self) {
        use core::ptr;
        unsafe {
            ptr::write_volatile(0x4005_200c as *mut u16, 0xA602);
            ptr::write_volatile(0x4005_200c as *mut u16, 0xB480);
        }
    }
    fn set_clock_source(&self, value: ClockSource) -> &Self {
        self.with_stctrlh(|r| r.set_clksrc(value as u16))
    }
    fn is_enabled(&self) -> bool {
        self.stctrlh().wdogen() != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_stctrlh(|r| r.set_wdogen(value))
    }
    fn timeout(&self) -> u32 {
        ((self.tovalh().tovalhigh() as u32) << 16) | (self.tovall().tovallow() as u32)
    }
    fn set_timeout(&self, value: u32) -> &Self {
        self.set_tovalh(Tovalh((value >> 16) as u16)).set_tovall(Tovall(value as u16))
    }
    fn window(&self) -> u32 {
        ((self.winh().winhigh() as u32) << 16) | (self.winl().winlow() as u32)
    }
    fn set_window(&self, value: u32) -> &Self {
        self.set_winh(Winh((value >> 16) as u16)).set_winl(Winl(value as u16))
    }
    fn timer(&self) -> u32 {
        ((self.tmrouth().timerouthigh() as u32) << 16) | (self.tmroutl().timeroutlow() as u32)
    }
    fn reset_count(&self) -> u16 {
        self.rstcnt().rstcnt()
    }
    fn prescaler(&self) -> Prescaler {
        match self.presc().prescval() {
            0b000 => Prescaler::Div1,
            0b001 => Prescaler::Div2,
            0b010 => Prescaler::Div3,
            0b011 => Prescaler::Div4,
            0b100 => Prescaler::Div5,
            0b101 => Prescaler::Div6,
            0b110 => Prescaler::Div7,
            0b111 => Prescaler::Div8,
            _ => unimplemented!()
        }
    }
    fn set_prescaler(&self, value: Prescaler) -> &Self {
        self.set_presc(Presc(0).set_prescval(value as u16))
    }
}