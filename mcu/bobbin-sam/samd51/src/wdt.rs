::bobbin_mcu::periph!( WDT, Wdt, WDT_PERIPH, WdtPeriph, WDT_OWNED, WDT_REF_COUNT, 0x40002000, 0x00, 0x32);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WDT Peripheral"]
pub struct WdtPeriph(pub usize); 

impl WdtPeriph {
    #[doc="Get the CTRLA Register."]
    #[inline] pub fn ctrla_reg(&self) -> ::bobbin_mcu::register::Register<Ctrla> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrla, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        self.ctrla_reg().read()
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn write_ctrla(&self, value: Ctrla) -> &Self { 
        self.ctrla_reg().write(value);
        self
    }

    #[doc="Set the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().set(f);
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().with(f);
        self
    }

    #[doc="Get the CONFIG Register."]
    #[inline] pub fn config_reg(&self) -> ::bobbin_mcu::register::Register<Config> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Config, 0x1)
    }

    #[doc="Get the *mut pointer for the CONFIG register."]
    #[inline] pub fn config_mut(&self) -> *mut Config { 
        self.config_reg().ptr()
    }

    #[doc="Get the *const pointer for the CONFIG register."]
    #[inline] pub fn config_ptr(&self) -> *const Config { 
        self.config_reg().ptr()
    }

    #[doc="Read the CONFIG register."]
    #[inline] pub fn config(&self) -> Config { 
        self.config_reg().read()
    }

    #[doc="Write the CONFIG register."]
    #[inline] pub fn write_config(&self, value: Config) -> &Self { 
        self.config_reg().write(value);
        self
    }

    #[doc="Set the CONFIG register."]
    #[inline] pub fn set_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
        self.config_reg().set(f);
        self
    }

    #[doc="Modify the CONFIG register."]
    #[inline] pub fn with_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
        self.config_reg().with(f);
        self
    }

    #[doc="Get the EWCTRL Register."]
    #[inline] pub fn ewctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ewctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ewctrl, 0x2)
    }

    #[doc="Get the *mut pointer for the EWCTRL register."]
    #[inline] pub fn ewctrl_mut(&self) -> *mut Ewctrl { 
        self.ewctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the EWCTRL register."]
    #[inline] pub fn ewctrl_ptr(&self) -> *const Ewctrl { 
        self.ewctrl_reg().ptr()
    }

    #[doc="Read the EWCTRL register."]
    #[inline] pub fn ewctrl(&self) -> Ewctrl { 
        self.ewctrl_reg().read()
    }

    #[doc="Write the EWCTRL register."]
    #[inline] pub fn write_ewctrl(&self, value: Ewctrl) -> &Self { 
        self.ewctrl_reg().write(value);
        self
    }

    #[doc="Set the EWCTRL register."]
    #[inline] pub fn set_ewctrl<F: FnOnce(Ewctrl) -> Ewctrl>(&self, f: F) -> &Self {
        self.ewctrl_reg().set(f);
        self
    }

    #[doc="Modify the EWCTRL register."]
    #[inline] pub fn with_ewctrl<F: FnOnce(Ewctrl) -> Ewctrl>(&self, f: F) -> &Self {
        self.ewctrl_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x4)
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        self.intenclr_reg().read()
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn write_intenclr(&self, value: Intenclr) -> &Self { 
        self.intenclr_reg().write(value);
        self
    }

    #[doc="Set the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().set(f);
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().with(f);
        self
    }

    #[doc="Get the INTENSET Register."]
    #[inline] pub fn intenset_reg(&self) -> ::bobbin_mcu::register::Register<Intenset> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x5)
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        self.intenset_reg().read()
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn write_intenset(&self, value: Intenset) -> &Self { 
        self.intenset_reg().write(value);
        self
    }

    #[doc="Set the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().set(f);
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().with(f);
        self
    }

    #[doc="Get the INTFLAG Register."]
    #[inline] pub fn intflag_reg(&self) -> ::bobbin_mcu::register::Register<Intflag> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x6)
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        self.intflag_reg().read()
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn write_intflag(&self, value: Intflag) -> &Self { 
        self.intflag_reg().write(value);
        self
    }

    #[doc="Set the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().with(f);
        self
    }

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x8)
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        self.syncbusy_reg().read()
    }

    #[doc="Get the CLEAR Register."]
    #[inline] pub fn clear_reg(&self) -> ::bobbin_mcu::register::Register<Clear> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clear, 0xc)
    }

    #[doc="Get the *mut pointer for the CLEAR register."]
    #[inline] pub fn clear_mut(&self) -> *mut Clear { 
        self.clear_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLEAR register."]
    #[inline] pub fn clear_ptr(&self) -> *const Clear { 
        self.clear_reg().ptr()
    }

    #[doc="Write the CLEAR register."]
    #[inline] pub fn write_clear(&self, value: Clear) -> &Self { 
        self.clear_reg().write(value);
        self
    }

    #[doc="Set the CLEAR register."]
    #[inline] pub fn set_clear<F: FnOnce(Clear) -> Clear>(&self, f: F) -> &Self {
        self.clear_reg().set(f);
        self
    }

}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u8);
impl Ctrla {
    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Watchdog Timer Window Mode Enable"]
    #[inline] pub fn wen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WEN != 0"]
    #[inline] pub fn test_wen(&self) -> bool {
        self.wen() != 0
    }

    #[doc="Sets the WEN field."]
    #[inline] pub fn set_wen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Always-On"]
    #[inline] pub fn alwayson(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ALWAYSON != 0"]
    #[inline] pub fn test_alwayson(&self) -> bool {
        self.alwayson() != 0
    }

    #[doc="Sets the ALWAYSON field."]
    #[inline] pub fn set_alwayson<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ctrla {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.wen() != 0 { try!(write!(f, " wen"))}
        if self.alwayson() != 0 { try!(write!(f, " alwayson"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Config(pub u8);
impl Config {
    #[doc="Time-Out Period"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Window Mode Time-Out Period"]
    #[inline] pub fn window(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if WINDOW != 0"]
    #[inline] pub fn test_window(&self) -> bool {
        self.window() != 0
    }

    #[doc="Sets the WINDOW field."]
    #[inline] pub fn set_window<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
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

#[doc="Early Warning Interrupt Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ewctrl(pub u8);
impl Ewctrl {
    #[doc="Early Warning Interrupt Time Offset"]
    #[inline] pub fn ewoffset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EWOFFSET != 0"]
    #[inline] pub fn test_ewoffset(&self) -> bool {
        self.ewoffset() != 0
    }

    #[doc="Sets the EWOFFSET field."]
    #[inline] pub fn set_ewoffset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
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
    #[inline] pub fn ew(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EW != 0"]
    #[inline] pub fn test_ew(&self) -> bool {
        self.ew() != 0
    }

    #[doc="Sets the EW field."]
    #[inline] pub fn set_ew<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn ew(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EW != 0"]
    #[inline] pub fn test_ew(&self) -> bool {
        self.ew() != 0
    }

    #[doc="Sets the EW field."]
    #[inline] pub fn set_ew<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn ew(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EW != 0"]
    #[inline] pub fn test_ew(&self) -> bool {
        self.ew() != 0
    }

    #[doc="Sets the EW field."]
    #[inline] pub fn set_ew<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Enable Synchronization Busy"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window Enable Synchronization Busy"]
    #[inline] pub fn wen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WEN != 0"]
    #[inline] pub fn test_wen(&self) -> bool {
        self.wen() != 0
    }

    #[doc="Sets the WEN field."]
    #[inline] pub fn set_wen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Always-On Synchronization Busy"]
    #[inline] pub fn alwayson(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ALWAYSON != 0"]
    #[inline] pub fn test_alwayson(&self) -> bool {
        self.alwayson() != 0
    }

    #[doc="Sets the ALWAYSON field."]
    #[inline] pub fn set_alwayson<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear Synchronization Busy"]
    #[inline] pub fn clear(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CLEAR != 0"]
    #[inline] pub fn test_clear(&self) -> bool {
        self.clear() != 0
    }

    #[doc="Sets the CLEAR field."]
    #[inline] pub fn set_clear<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.wen() != 0 { try!(write!(f, " wen"))}
        if self.alwayson() != 0 { try!(write!(f, " alwayson"))}
        if self.clear() != 0 { try!(write!(f, " clear"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clear(pub u8);
impl Clear {
    #[doc="Watchdog Clear"]
    #[inline] pub fn clear(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLEAR != 0"]
    #[inline] pub fn test_clear(&self) -> bool {
        self.clear() != 0
    }

    #[doc="Sets the CLEAR field."]
    #[inline] pub fn set_clear<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
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

