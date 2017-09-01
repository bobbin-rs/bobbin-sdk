//! Watchdog Timer
#[allow(unused_imports)] use bobbin_common::*;

periph!(WDT, Wdt, 0x40001000);

#[doc="Watchdog Timer"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wdt(pub usize);
impl Wdt {
    #[doc="Get the *const pointer for the CLEAR register."]
    #[inline] pub fn clear_ptr(&self) -> *const Clear { 
        (self.0 + 0x8) as *const Clear
    }

    #[doc="Get the *mut pointer for the CLEAR register."]
    #[inline] pub fn clear_mut(&self) -> *mut Clear { 
        (self.0 + 0x8) as *mut Clear
    }

    #[doc="Write the CLEAR register."]
    #[inline] pub fn set_clear<F: FnOnce(Clear) -> Clear>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x8) as *mut Clear, f(Clear(0)));
        }
        self
    }

    #[doc="Get the *const pointer for the CONFIG register."]
    #[inline] pub fn config_ptr(&self) -> *const Config { 
        (self.0 + 0x1) as *const Config
    }

    #[doc="Get the *mut pointer for the CONFIG register."]
    #[inline] pub fn config_mut(&self) -> *mut Config { 
        (self.0 + 0x1) as *mut Config
    }

    #[doc="Read the CONFIG register."]
    #[inline] pub fn config(&self) -> Config { 
        unsafe {
            read_volatile((self.0 + 0x1) as *const Config)
        }
    }

    #[doc="Write the CONFIG register."]
    #[inline] pub fn set_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1) as *mut Config, f(Config(0)));
        }
        self
    }

    #[doc="Modify the CONFIG register."]
    #[inline] pub fn with_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1) as *mut Config, f(self.config()));
        }
        self
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        (self.0 + 0x0) as *const Ctrl
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        (self.0 + 0x0) as *mut Ctrl
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        unsafe {
            read_volatile((self.0 + 0x0) as *const Ctrl)
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x0) as *mut Ctrl, f(Ctrl(0)));
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x0) as *mut Ctrl, f(self.ctrl()));
        }
        self
    }

    #[doc="Get the *const pointer for the EWCTRL register."]
    #[inline] pub fn ewctrl_ptr(&self) -> *const Ewctrl { 
        (self.0 + 0x2) as *const Ewctrl
    }

    #[doc="Get the *mut pointer for the EWCTRL register."]
    #[inline] pub fn ewctrl_mut(&self) -> *mut Ewctrl { 
        (self.0 + 0x2) as *mut Ewctrl
    }

    #[doc="Read the EWCTRL register."]
    #[inline] pub fn ewctrl(&self) -> Ewctrl { 
        unsafe {
            read_volatile((self.0 + 0x2) as *const Ewctrl)
        }
    }

    #[doc="Write the EWCTRL register."]
    #[inline] pub fn set_ewctrl<F: FnOnce(Ewctrl) -> Ewctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x2) as *mut Ewctrl, f(Ewctrl(0)));
        }
        self
    }

    #[doc="Modify the EWCTRL register."]
    #[inline] pub fn with_ewctrl<F: FnOnce(Ewctrl) -> Ewctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x2) as *mut Ewctrl, f(self.ewctrl()));
        }
        self
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
        (self.0 + 0x4) as *const Intenclr
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        (self.0 + 0x4) as *mut Intenclr
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        unsafe {
            read_volatile((self.0 + 0x4) as *const Intenclr)
        }
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x4) as *mut Intenclr, f(Intenclr(0)));
        }
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x4) as *mut Intenclr, f(self.intenclr()));
        }
        self
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
        (self.0 + 0x5) as *const Intenset
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        (self.0 + 0x5) as *mut Intenset
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        unsafe {
            read_volatile((self.0 + 0x5) as *const Intenset)
        }
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x5) as *mut Intenset, f(Intenset(0)));
        }
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x5) as *mut Intenset, f(self.intenset()));
        }
        self
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
        (self.0 + 0x6) as *const Intflag
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        (self.0 + 0x6) as *mut Intflag
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        unsafe {
            read_volatile((self.0 + 0x6) as *const Intflag)
        }
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x6) as *mut Intflag, f(Intflag(0)));
        }
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x6) as *mut Intflag, f(self.intflag()));
        }
        self
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        (self.0 + 0x7) as *const Status
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0x7) as *mut Status
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile((self.0 + 0x7) as *const Status)
        }
    }

}

#[doc="Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clear(pub u8);
impl Clear {
    #[doc="Watchdog Clear"]
    #[inline] pub fn clear(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Watchdog Clear"]
    #[inline] pub fn test_clear(&self) -> bool {
        self.clear() != 0
    }

    #[doc="Watchdog Clear"]
    #[inline] pub fn set_clear<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Clear {
    #[inline]
    fn from(other: u8) -> Self {
         Clear(other)
    }
}

impl ::core::fmt::Display for Clear {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clear {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clear() != 0 { try!(write!(f, " clear=0x{:x}", self.clear()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Config(pub u8);
impl Config {
    #[doc="Time-Out Period"]
    #[inline] pub fn per(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Time-Out Period"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Time-Out Period"]
    #[inline] pub fn set_per<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Window Mode Time-Out Period"]
    #[inline] pub fn window(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Window Mode Time-Out Period"]
    #[inline] pub fn test_window(&self) -> bool {
        self.window() != 0
    }

    #[doc="Window Mode Time-Out Period"]
    #[inline] pub fn set_window<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Config {
    #[inline]
    fn from(other: u8) -> Self {
         Config(other)
    }
}

impl ::core::fmt::Display for Config {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Config {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
        if self.window() != 0 { try!(write!(f, " window=0x{:x}", self.window()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Enable"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Enable"]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Watchdog Timer Window Mode Enable"]
    #[inline] pub fn wen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Watchdog Timer Window Mode Enable"]
    #[inline] pub fn test_wen(&self) -> bool {
        self.wen() != 0
    }

    #[doc="Watchdog Timer Window Mode Enable"]
    #[inline] pub fn set_wen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Always-On"]
    #[inline] pub fn alwayson(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Always-On"]
    #[inline] pub fn test_alwayson(&self) -> bool {
        self.alwayson() != 0
    }

    #[doc="Always-On"]
    #[inline] pub fn set_alwayson<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrl(other)
    }
}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.wen() != 0 { try!(write!(f, " wen"))}
        if self.alwayson() != 0 { try!(write!(f, " alwayson"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Early Warning Interrupt Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ewctrl(pub u8);
impl Ewctrl {
    #[doc="Early Warning Interrupt Time Offset"]
    #[inline] pub fn ewoffset(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Early Warning Interrupt Time Offset"]
    #[inline] pub fn test_ewoffset(&self) -> bool {
        self.ewoffset() != 0
    }

    #[doc="Early Warning Interrupt Time Offset"]
    #[inline] pub fn set_ewoffset<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Ewctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Ewctrl(other)
    }
}

impl ::core::fmt::Display for Ewctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ewctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ewoffset() != 0 { try!(write!(f, " ewoffset=0x{:x}", self.ewoffset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Early Warning Interrupt Enable"]
    #[inline] pub fn ew(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Early Warning Interrupt Enable"]
    #[inline] pub fn test_ew(&self) -> bool {
        self.ew() != 0
    }

    #[doc="Early Warning Interrupt Enable"]
    #[inline] pub fn set_ew<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Intenclr(other)
    }
}

impl ::core::fmt::Display for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ew() != 0 { try!(write!(f, " ew"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Early Warning Interrupt Enable"]
    #[inline] pub fn ew(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Early Warning Interrupt Enable"]
    #[inline] pub fn test_ew(&self) -> bool {
        self.ew() != 0
    }

    #[doc="Early Warning Interrupt Enable"]
    #[inline] pub fn set_ew<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
         Intenset(other)
    }
}

impl ::core::fmt::Display for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ew() != 0 { try!(write!(f, " ew"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Early Warning"]
    #[inline] pub fn ew(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Early Warning"]
    #[inline] pub fn test_ew(&self) -> bool {
        self.ew() != 0
    }

    #[doc="Early Warning"]
    #[inline] pub fn set_ew<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
         Intflag(other)
    }
}

impl ::core::fmt::Display for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ew() != 0 { try!(write!(f, " ew"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Synchronization Busy"]
    #[inline] pub fn syncbusy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Synchronization Busy"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Synchronization Busy"]
    #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


