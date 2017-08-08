use bobbin_common::bits::*;
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
        self.set_stctrlh(|_| stctrlh);        
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
        self.with_stctrlh(|r| r.set_wdogen(value))
    }

    fn timeout(&self) -> u32 {
        join_u32(self.tovalh().tovalhigh(), self.tovall().tovallow())
        // ((self.tovalh().tovalhigh() as u32) << 16) | (self.tovall().tovallow() as u32)
    }

    fn set_timeout(&self, value: u32) -> &Self {
        let (hi, lo) = split_u32(value);
        self.set_tovalh(|r| r.set_tovalhigh(hi)).set_tovall(|r| r.set_tovallow(lo))
        // self.set_tovalh(Tovalh((value >> 16) as u16)).set_tovall(Tovall(value as u16))
    }

    fn window(&self) -> u32 {
        join_u32(self.winh().winhigh(), self.winl().winlow())
        // ((self.winh().winhigh() as u32) << 16) | (self.winl().winlow() as u32)
    }

    fn set_window(&self, value: u32) -> &Self {
        let (hi, lo) = split_u32(value);
        self.set_winh(|r| r.set_winhigh(hi)).set_winl(|r| r.set_winlow(lo))
        // self.set_winh(Winh((value >> 16) as u16)).set_winl(Winl(value as u16))
    }

    fn timer(&self) -> u32 {
        join_u32(self.tmrouth().timerouthigh(), self.tmroutl().timeroutlow())
        // (( as u32) << 16) | (self.tmroutl().timeroutlow() as u32)
    }

    fn reset_count(&self) -> u16 {
        self.rstcnt().rstcnt().into()        
    }

    fn prescaler(&self) -> Prescaler {
        match self.presc().prescval() {
            U3::B000 => Prescaler::Div1,
            U3::B001 => Prescaler::Div2,
            U3::B010 => Prescaler::Div3,
            U3::B011 => Prescaler::Div4,
            U3::B100 => Prescaler::Div5,
            U3::B101 => Prescaler::Div6,
            U3::B110 => Prescaler::Div7,
            U3::B111 => Prescaler::Div8,
        }
    }
    fn set_prescaler(&self, value: Prescaler) -> &Self {
        self.set_presc(|r| r.set_prescval(value as u16))
    }
}

fn split_u32(value: u32) -> (u16, u16) {
    (
        (value >> 16) as u16, 
        value as u16
    )
}

fn join_u32<I: Into<u16>>(hi: I, lo: I) -> u32 {
    ((hi.into() as u32) << 16) | (lo.into() as u32)
}