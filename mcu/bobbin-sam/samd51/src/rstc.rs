::bobbin_mcu::periph!( RSTC, Rstc, RSTC_PERIPH, RstcPeriph, RSTC_OWNED, RSTC_REF_COUNT, 0x40000c00, 0x00, 0x1c);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="RSTC Peripheral"]
pub struct RstcPeriph(pub usize); 

impl RstcPeriph {
    #[doc="Get the RCAUSE Register."]
    #[inline] pub fn rcause_reg(&self) -> ::bobbin_mcu::register::Register<Rcause> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rcause, 0x0)
    }

    #[doc="Get the *mut pointer for the RCAUSE register."]
    #[inline] pub fn rcause_mut(&self) -> *mut Rcause { 
        self.rcause_reg().ptr()
    }

    #[doc="Get the *const pointer for the RCAUSE register."]
    #[inline] pub fn rcause_ptr(&self) -> *const Rcause { 
        self.rcause_reg().ptr()
    }

    #[doc="Read the RCAUSE register."]
    #[inline] pub fn rcause(&self) -> Rcause { 
        self.rcause_reg().read()
    }

    #[doc="Get the BKUPEXIT Register."]
    #[inline] pub fn bkupexit_reg(&self) -> ::bobbin_mcu::register::Register<Bkupexit> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bkupexit, 0x2)
    }

    #[doc="Get the *mut pointer for the BKUPEXIT register."]
    #[inline] pub fn bkupexit_mut(&self) -> *mut Bkupexit { 
        self.bkupexit_reg().ptr()
    }

    #[doc="Get the *const pointer for the BKUPEXIT register."]
    #[inline] pub fn bkupexit_ptr(&self) -> *const Bkupexit { 
        self.bkupexit_reg().ptr()
    }

    #[doc="Read the BKUPEXIT register."]
    #[inline] pub fn bkupexit(&self) -> Bkupexit { 
        self.bkupexit_reg().read()
    }

}

#[doc="Reset Cause"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcause(pub u8);
impl Rcause {
    #[doc="Power On Reset"]
    #[inline] pub fn por(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if POR != 0"]
    #[inline] pub fn test_por(&self) -> bool {
        self.por() != 0
    }

    #[doc="Sets the POR field."]
    #[inline] pub fn set_por<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Brown Out CORE Detector Reset"]
    #[inline] pub fn bodcore(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BODCORE != 0"]
    #[inline] pub fn test_bodcore(&self) -> bool {
        self.bodcore() != 0
    }

    #[doc="Sets the BODCORE field."]
    #[inline] pub fn set_bodcore<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Brown Out VDD Detector Reset"]
    #[inline] pub fn bodvdd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BODVDD != 0"]
    #[inline] pub fn test_bodvdd(&self) -> bool {
        self.bodvdd() != 0
    }

    #[doc="Sets the BODVDD field."]
    #[inline] pub fn set_bodvdd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="NVM Reset"]
    #[inline] pub fn nvm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if NVM != 0"]
    #[inline] pub fn test_nvm(&self) -> bool {
        self.nvm() != 0
    }

    #[doc="Sets the NVM field."]
    #[inline] pub fn set_nvm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="External Reset"]
    #[inline] pub fn ext(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EXT != 0"]
    #[inline] pub fn test_ext(&self) -> bool {
        self.ext() != 0
    }

    #[doc="Sets the EXT field."]
    #[inline] pub fn set_ext<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Watchdog Reset"]
    #[inline] pub fn wdt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
    #[inline] pub fn set_wdt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="System Reset Request"]
    #[inline] pub fn syst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SYST != 0"]
    #[inline] pub fn test_syst(&self) -> bool {
        self.syst() != 0
    }

    #[doc="Sets the SYST field."]
    #[inline] pub fn set_syst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Backup Reset"]
    #[inline] pub fn backup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BACKUP != 0"]
    #[inline] pub fn test_backup(&self) -> bool {
        self.backup() != 0
    }

    #[doc="Sets the BACKUP field."]
    #[inline] pub fn set_backup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Rcause {
    #[inline]
    fn from(other: u8) -> Self {
         Rcause(other)
    }
}

impl ::core::fmt::Display for Rcause {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcause {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.por() != 0 { try!(write!(f, " por"))}
        if self.bodcore() != 0 { try!(write!(f, " bodcore"))}
        if self.bodvdd() != 0 { try!(write!(f, " bodvdd"))}
        if self.nvm() != 0 { try!(write!(f, " nvm"))}
        if self.ext() != 0 { try!(write!(f, " ext"))}
        if self.wdt() != 0 { try!(write!(f, " wdt"))}
        if self.syst() != 0 { try!(write!(f, " syst"))}
        if self.backup() != 0 { try!(write!(f, " backup"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup Exit Source"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bkupexit(pub u8);
impl Bkupexit {
    #[doc="Real Timer Counter Interrupt"]
    #[inline] pub fn rtc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTC != 0"]
    #[inline] pub fn test_rtc(&self) -> bool {
        self.rtc() != 0
    }

    #[doc="Sets the RTC field."]
    #[inline] pub fn set_rtc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Battery Backup Power Switch"]
    #[inline] pub fn bbps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BBPS != 0"]
    #[inline] pub fn test_bbps(&self) -> bool {
        self.bbps() != 0
    }

    #[doc="Sets the BBPS field."]
    #[inline] pub fn set_bbps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Hibernate"]
    #[inline] pub fn hib(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if HIB != 0"]
    #[inline] pub fn test_hib(&self) -> bool {
        self.hib() != 0
    }

    #[doc="Sets the HIB field."]
    #[inline] pub fn set_hib<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Bkupexit {
    #[inline]
    fn from(other: u8) -> Self {
         Bkupexit(other)
    }
}

impl ::core::fmt::Display for Bkupexit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bkupexit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtc() != 0 { try!(write!(f, " rtc"))}
        if self.bbps() != 0 { try!(write!(f, " bbps"))}
        if self.hib() != 0 { try!(write!(f, " hib"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

