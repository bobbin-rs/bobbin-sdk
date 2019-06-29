::bobbin_mcu::periph!( PAC, Pac, PAC_PERIPH, PacPeriph, PAC_OWNED, PAC_REF_COUNT, 0x40000000, 0x00, 0x14);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PAC Peripheral"]
pub struct PacPeriph(pub usize); 

impl PacPeriph {
    #[doc="Get the WRCTRL Register."]
    #[inline] pub fn wrctrl_reg(&self) -> ::bobbin_mcu::register::Register<Wrctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wrctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the WRCTRL register."]
    #[inline] pub fn wrctrl_mut(&self) -> *mut Wrctrl { 
        self.wrctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRCTRL register."]
    #[inline] pub fn wrctrl_ptr(&self) -> *const Wrctrl { 
        self.wrctrl_reg().ptr()
    }

    #[doc="Read the WRCTRL register."]
    #[inline] pub fn wrctrl(&self) -> Wrctrl { 
        self.wrctrl_reg().read()
    }

    #[doc="Write the WRCTRL register."]
    #[inline] pub fn write_wrctrl(&self, value: Wrctrl) -> &Self { 
        self.wrctrl_reg().write(value);
        self
    }

    #[doc="Set the WRCTRL register."]
    #[inline] pub fn set_wrctrl<F: FnOnce(Wrctrl) -> Wrctrl>(&self, f: F) -> &Self {
        self.wrctrl_reg().set(f);
        self
    }

    #[doc="Modify the WRCTRL register."]
    #[inline] pub fn with_wrctrl<F: FnOnce(Wrctrl) -> Wrctrl>(&self, f: F) -> &Self {
        self.wrctrl_reg().with(f);
        self
    }

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x4)
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        self.evctrl_reg().read()
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn write_evctrl(&self, value: Evctrl) -> &Self { 
        self.evctrl_reg().write(value);
        self
    }

    #[doc="Set the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().set(f);
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x8)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x9)
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

    #[doc="Get the INTFLAGAHB Register."]
    #[inline] pub fn intflagahb_reg(&self) -> ::bobbin_mcu::register::Register<Intflagahb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflagahb, 0x10)
    }

    #[doc="Get the *mut pointer for the INTFLAGAHB register."]
    #[inline] pub fn intflagahb_mut(&self) -> *mut Intflagahb { 
        self.intflagahb_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAGAHB register."]
    #[inline] pub fn intflagahb_ptr(&self) -> *const Intflagahb { 
        self.intflagahb_reg().ptr()
    }

    #[doc="Read the INTFLAGAHB register."]
    #[inline] pub fn intflagahb(&self) -> Intflagahb { 
        self.intflagahb_reg().read()
    }

    #[doc="Write the INTFLAGAHB register."]
    #[inline] pub fn write_intflagahb(&self, value: Intflagahb) -> &Self { 
        self.intflagahb_reg().write(value);
        self
    }

    #[doc="Set the INTFLAGAHB register."]
    #[inline] pub fn set_intflagahb<F: FnOnce(Intflagahb) -> Intflagahb>(&self, f: F) -> &Self {
        self.intflagahb_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAGAHB register."]
    #[inline] pub fn with_intflagahb<F: FnOnce(Intflagahb) -> Intflagahb>(&self, f: F) -> &Self {
        self.intflagahb_reg().with(f);
        self
    }

    #[doc="Get the INTFLAGA Register."]
    #[inline] pub fn intflaga_reg(&self) -> ::bobbin_mcu::register::Register<Intflaga> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflaga, 0x14)
    }

    #[doc="Get the *mut pointer for the INTFLAGA register."]
    #[inline] pub fn intflaga_mut(&self) -> *mut Intflaga { 
        self.intflaga_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAGA register."]
    #[inline] pub fn intflaga_ptr(&self) -> *const Intflaga { 
        self.intflaga_reg().ptr()
    }

    #[doc="Read the INTFLAGA register."]
    #[inline] pub fn intflaga(&self) -> Intflaga { 
        self.intflaga_reg().read()
    }

    #[doc="Write the INTFLAGA register."]
    #[inline] pub fn write_intflaga(&self, value: Intflaga) -> &Self { 
        self.intflaga_reg().write(value);
        self
    }

    #[doc="Set the INTFLAGA register."]
    #[inline] pub fn set_intflaga<F: FnOnce(Intflaga) -> Intflaga>(&self, f: F) -> &Self {
        self.intflaga_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAGA register."]
    #[inline] pub fn with_intflaga<F: FnOnce(Intflaga) -> Intflaga>(&self, f: F) -> &Self {
        self.intflaga_reg().with(f);
        self
    }

    #[doc="Get the INTFLAGB Register."]
    #[inline] pub fn intflagb_reg(&self) -> ::bobbin_mcu::register::Register<Intflagb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflagb, 0x18)
    }

    #[doc="Get the *mut pointer for the INTFLAGB register."]
    #[inline] pub fn intflagb_mut(&self) -> *mut Intflagb { 
        self.intflagb_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAGB register."]
    #[inline] pub fn intflagb_ptr(&self) -> *const Intflagb { 
        self.intflagb_reg().ptr()
    }

    #[doc="Read the INTFLAGB register."]
    #[inline] pub fn intflagb(&self) -> Intflagb { 
        self.intflagb_reg().read()
    }

    #[doc="Write the INTFLAGB register."]
    #[inline] pub fn write_intflagb(&self, value: Intflagb) -> &Self { 
        self.intflagb_reg().write(value);
        self
    }

    #[doc="Set the INTFLAGB register."]
    #[inline] pub fn set_intflagb<F: FnOnce(Intflagb) -> Intflagb>(&self, f: F) -> &Self {
        self.intflagb_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAGB register."]
    #[inline] pub fn with_intflagb<F: FnOnce(Intflagb) -> Intflagb>(&self, f: F) -> &Self {
        self.intflagb_reg().with(f);
        self
    }

    #[doc="Get the INTFLAGC Register."]
    #[inline] pub fn intflagc_reg(&self) -> ::bobbin_mcu::register::Register<Intflagc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflagc, 0x1c)
    }

    #[doc="Get the *mut pointer for the INTFLAGC register."]
    #[inline] pub fn intflagc_mut(&self) -> *mut Intflagc { 
        self.intflagc_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAGC register."]
    #[inline] pub fn intflagc_ptr(&self) -> *const Intflagc { 
        self.intflagc_reg().ptr()
    }

    #[doc="Read the INTFLAGC register."]
    #[inline] pub fn intflagc(&self) -> Intflagc { 
        self.intflagc_reg().read()
    }

    #[doc="Write the INTFLAGC register."]
    #[inline] pub fn write_intflagc(&self, value: Intflagc) -> &Self { 
        self.intflagc_reg().write(value);
        self
    }

    #[doc="Set the INTFLAGC register."]
    #[inline] pub fn set_intflagc<F: FnOnce(Intflagc) -> Intflagc>(&self, f: F) -> &Self {
        self.intflagc_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAGC register."]
    #[inline] pub fn with_intflagc<F: FnOnce(Intflagc) -> Intflagc>(&self, f: F) -> &Self {
        self.intflagc_reg().with(f);
        self
    }

    #[doc="Get the INTFLAGD Register."]
    #[inline] pub fn intflagd_reg(&self) -> ::bobbin_mcu::register::Register<Intflagd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflagd, 0x20)
    }

    #[doc="Get the *mut pointer for the INTFLAGD register."]
    #[inline] pub fn intflagd_mut(&self) -> *mut Intflagd { 
        self.intflagd_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAGD register."]
    #[inline] pub fn intflagd_ptr(&self) -> *const Intflagd { 
        self.intflagd_reg().ptr()
    }

    #[doc="Read the INTFLAGD register."]
    #[inline] pub fn intflagd(&self) -> Intflagd { 
        self.intflagd_reg().read()
    }

    #[doc="Write the INTFLAGD register."]
    #[inline] pub fn write_intflagd(&self, value: Intflagd) -> &Self { 
        self.intflagd_reg().write(value);
        self
    }

    #[doc="Set the INTFLAGD register."]
    #[inline] pub fn set_intflagd<F: FnOnce(Intflagd) -> Intflagd>(&self, f: F) -> &Self {
        self.intflagd_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAGD register."]
    #[inline] pub fn with_intflagd<F: FnOnce(Intflagd) -> Intflagd>(&self, f: F) -> &Self {
        self.intflagd_reg().with(f);
        self
    }

    #[doc="Get the STATUSA Register."]
    #[inline] pub fn statusa_reg(&self) -> ::bobbin_mcu::register::Register<Statusa> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Statusa, 0x34)
    }

    #[doc="Get the *mut pointer for the STATUSA register."]
    #[inline] pub fn statusa_mut(&self) -> *mut Statusa { 
        self.statusa_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUSA register."]
    #[inline] pub fn statusa_ptr(&self) -> *const Statusa { 
        self.statusa_reg().ptr()
    }

    #[doc="Read the STATUSA register."]
    #[inline] pub fn statusa(&self) -> Statusa { 
        self.statusa_reg().read()
    }

    #[doc="Get the STATUSB Register."]
    #[inline] pub fn statusb_reg(&self) -> ::bobbin_mcu::register::Register<Statusb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Statusb, 0x38)
    }

    #[doc="Get the *mut pointer for the STATUSB register."]
    #[inline] pub fn statusb_mut(&self) -> *mut Statusb { 
        self.statusb_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUSB register."]
    #[inline] pub fn statusb_ptr(&self) -> *const Statusb { 
        self.statusb_reg().ptr()
    }

    #[doc="Read the STATUSB register."]
    #[inline] pub fn statusb(&self) -> Statusb { 
        self.statusb_reg().read()
    }

    #[doc="Get the STATUSC Register."]
    #[inline] pub fn statusc_reg(&self) -> ::bobbin_mcu::register::Register<Statusc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Statusc, 0x3c)
    }

    #[doc="Get the *mut pointer for the STATUSC register."]
    #[inline] pub fn statusc_mut(&self) -> *mut Statusc { 
        self.statusc_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUSC register."]
    #[inline] pub fn statusc_ptr(&self) -> *const Statusc { 
        self.statusc_reg().ptr()
    }

    #[doc="Read the STATUSC register."]
    #[inline] pub fn statusc(&self) -> Statusc { 
        self.statusc_reg().read()
    }

    #[doc="Get the STATUSD Register."]
    #[inline] pub fn statusd_reg(&self) -> ::bobbin_mcu::register::Register<Statusd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Statusd, 0x40)
    }

    #[doc="Get the *mut pointer for the STATUSD register."]
    #[inline] pub fn statusd_mut(&self) -> *mut Statusd { 
        self.statusd_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUSD register."]
    #[inline] pub fn statusd_ptr(&self) -> *const Statusd { 
        self.statusd_reg().ptr()
    }

    #[doc="Read the STATUSD register."]
    #[inline] pub fn statusd(&self) -> Statusd { 
        self.statusd_reg().read()
    }

}

#[doc="Write control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrctrl(pub u32);
impl Wrctrl {
    #[doc="Peripheral identifier"]
    #[inline] pub fn perid(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PERID != 0"]
    #[inline] pub fn test_perid(&self) -> bool {
        self.perid() != 0
    }

    #[doc="Sets the PERID field."]
    #[inline] pub fn set_perid<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Peripheral access control key"]
    #[inline] pub fn key(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Wrctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Wrctrl(other)
    }
}

impl ::core::fmt::Display for Wrctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.perid() != 0 { try!(write!(f, " perid=0x{:x}", self.perid()))}
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u8);
impl Evctrl {
    #[doc="Peripheral acess error event output"]
    #[inline] pub fn erreo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ERREO != 0"]
    #[inline] pub fn test_erreo(&self) -> bool {
        self.erreo() != 0
    }

    #[doc="Sets the ERREO field."]
    #[inline] pub fn set_erreo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Evctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.erreo() != 0 { try!(write!(f, " erreo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt enable clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Peripheral access error interrupt disable"]
    #[inline] pub fn err(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.err() != 0 { try!(write!(f, " err"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt enable set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Peripheral access error interrupt enable"]
    #[inline] pub fn err(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.err() != 0 { try!(write!(f, " err"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Bridge interrupt flag status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflagahb(pub u32);
impl Intflagahb {
    #[doc="FLASH"]
    #[inline] pub fn flash_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FLASH_ != 0"]
    #[inline] pub fn test_flash_(&self) -> bool {
        self.flash_() != 0
    }

    #[doc="Sets the FLASH_ field."]
    #[inline] pub fn set_flash_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FLASH_ALT"]
    #[inline] pub fn flash_alt_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FLASH_ALT_ != 0"]
    #[inline] pub fn test_flash_alt_(&self) -> bool {
        self.flash_alt_() != 0
    }

    #[doc="Sets the FLASH_ALT_ field."]
    #[inline] pub fn set_flash_alt_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SEEPROM"]
    #[inline] pub fn seeprom_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SEEPROM_ != 0"]
    #[inline] pub fn test_seeprom_(&self) -> bool {
        self.seeprom_() != 0
    }

    #[doc="Sets the SEEPROM_ field."]
    #[inline] pub fn set_seeprom_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RAMCM4S"]
    #[inline] pub fn ramcm4s_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RAMCM4S_ != 0"]
    #[inline] pub fn test_ramcm4s_(&self) -> bool {
        self.ramcm4s_() != 0
    }

    #[doc="Sets the RAMCM4S_ field."]
    #[inline] pub fn set_ramcm4s_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RAMPPPDSU"]
    #[inline] pub fn rampppdsu_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RAMPPPDSU_ != 0"]
    #[inline] pub fn test_rampppdsu_(&self) -> bool {
        self.rampppdsu_() != 0
    }

    #[doc="Sets the RAMPPPDSU_ field."]
    #[inline] pub fn set_rampppdsu_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RAMDMAWR"]
    #[inline] pub fn ramdmawr_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RAMDMAWR_ != 0"]
    #[inline] pub fn test_ramdmawr_(&self) -> bool {
        self.ramdmawr_() != 0
    }

    #[doc="Sets the RAMDMAWR_ field."]
    #[inline] pub fn set_ramdmawr_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RAMDMACICM"]
    #[inline] pub fn ramdmacicm_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RAMDMACICM_ != 0"]
    #[inline] pub fn test_ramdmacicm_(&self) -> bool {
        self.ramdmacicm_() != 0
    }

    #[doc="Sets the RAMDMACICM_ field."]
    #[inline] pub fn set_ramdmacicm_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="HPB0"]
    #[inline] pub fn hpb0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if HPB0_ != 0"]
    #[inline] pub fn test_hpb0_(&self) -> bool {
        self.hpb0_() != 0
    }

    #[doc="Sets the HPB0_ field."]
    #[inline] pub fn set_hpb0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="HPB1"]
    #[inline] pub fn hpb1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HPB1_ != 0"]
    #[inline] pub fn test_hpb1_(&self) -> bool {
        self.hpb1_() != 0
    }

    #[doc="Sets the HPB1_ field."]
    #[inline] pub fn set_hpb1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HPB2"]
    #[inline] pub fn hpb2_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HPB2_ != 0"]
    #[inline] pub fn test_hpb2_(&self) -> bool {
        self.hpb2_() != 0
    }

    #[doc="Sets the HPB2_ field."]
    #[inline] pub fn set_hpb2_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HPB3"]
    #[inline] pub fn hpb3_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HPB3_ != 0"]
    #[inline] pub fn test_hpb3_(&self) -> bool {
        self.hpb3_() != 0
    }

    #[doc="Sets the HPB3_ field."]
    #[inline] pub fn set_hpb3_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="PUKCC"]
    #[inline] pub fn pukcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PUKCC_ != 0"]
    #[inline] pub fn test_pukcc_(&self) -> bool {
        self.pukcc_() != 0
    }

    #[doc="Sets the PUKCC_ field."]
    #[inline] pub fn set_pukcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SDHC0"]
    #[inline] pub fn sdhc0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SDHC0_ != 0"]
    #[inline] pub fn test_sdhc0_(&self) -> bool {
        self.sdhc0_() != 0
    }

    #[doc="Sets the SDHC0_ field."]
    #[inline] pub fn set_sdhc0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="QSPI"]
    #[inline] pub fn qspi_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if QSPI_ != 0"]
    #[inline] pub fn test_qspi_(&self) -> bool {
        self.qspi_() != 0
    }

    #[doc="Sets the QSPI_ field."]
    #[inline] pub fn set_qspi_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="BKUPRAM"]
    #[inline] pub fn bkupram_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BKUPRAM_ != 0"]
    #[inline] pub fn test_bkupram_(&self) -> bool {
        self.bkupram_() != 0
    }

    #[doc="Sets the BKUPRAM_ field."]
    #[inline] pub fn set_bkupram_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Intflagahb {
    #[inline]
    fn from(other: u32) -> Self {
         Intflagahb(other)
    }
}

impl ::core::fmt::Display for Intflagahb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflagahb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flash_() != 0 { try!(write!(f, " flash_"))}
        if self.flash_alt_() != 0 { try!(write!(f, " flash_alt_"))}
        if self.seeprom_() != 0 { try!(write!(f, " seeprom_"))}
        if self.ramcm4s_() != 0 { try!(write!(f, " ramcm4s_"))}
        if self.rampppdsu_() != 0 { try!(write!(f, " rampppdsu_"))}
        if self.ramdmawr_() != 0 { try!(write!(f, " ramdmawr_"))}
        if self.ramdmacicm_() != 0 { try!(write!(f, " ramdmacicm_"))}
        if self.hpb0_() != 0 { try!(write!(f, " hpb0_"))}
        if self.hpb1_() != 0 { try!(write!(f, " hpb1_"))}
        if self.hpb2_() != 0 { try!(write!(f, " hpb2_"))}
        if self.hpb3_() != 0 { try!(write!(f, " hpb3_"))}
        if self.pukcc_() != 0 { try!(write!(f, " pukcc_"))}
        if self.sdhc0_() != 0 { try!(write!(f, " sdhc0_"))}
        if self.qspi_() != 0 { try!(write!(f, " qspi_"))}
        if self.bkupram_() != 0 { try!(write!(f, " bkupram_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral interrupt flag status - Bridge A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflaga(pub u32);
impl Intflaga {
    #[doc="PAC"]
    #[inline] pub fn pac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAC_ != 0"]
    #[inline] pub fn test_pac_(&self) -> bool {
        self.pac_() != 0
    }

    #[doc="Sets the PAC_ field."]
    #[inline] pub fn set_pac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PM"]
    #[inline] pub fn pm_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PM_ != 0"]
    #[inline] pub fn test_pm_(&self) -> bool {
        self.pm_() != 0
    }

    #[doc="Sets the PM_ field."]
    #[inline] pub fn set_pm_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MCLK"]
    #[inline] pub fn mclk_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MCLK_ != 0"]
    #[inline] pub fn test_mclk_(&self) -> bool {
        self.mclk_() != 0
    }

    #[doc="Sets the MCLK_ field."]
    #[inline] pub fn set_mclk_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RSTC"]
    #[inline] pub fn rstc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RSTC_ != 0"]
    #[inline] pub fn test_rstc_(&self) -> bool {
        self.rstc_() != 0
    }

    #[doc="Sets the RSTC_ field."]
    #[inline] pub fn set_rstc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="OSCCTRL"]
    #[inline] pub fn oscctrl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OSCCTRL_ != 0"]
    #[inline] pub fn test_oscctrl_(&self) -> bool {
        self.oscctrl_() != 0
    }

    #[doc="Sets the OSCCTRL_ field."]
    #[inline] pub fn set_oscctrl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="OSC32KCTRL"]
    #[inline] pub fn osc32kctrl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if OSC32KCTRL_ != 0"]
    #[inline] pub fn test_osc32kctrl_(&self) -> bool {
        self.osc32kctrl_() != 0
    }

    #[doc="Sets the OSC32KCTRL_ field."]
    #[inline] pub fn set_osc32kctrl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SUPC"]
    #[inline] pub fn supc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SUPC_ != 0"]
    #[inline] pub fn test_supc_(&self) -> bool {
        self.supc_() != 0
    }

    #[doc="Sets the SUPC_ field."]
    #[inline] pub fn set_supc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GCLK"]
    #[inline] pub fn gclk_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GCLK_ != 0"]
    #[inline] pub fn test_gclk_(&self) -> bool {
        self.gclk_() != 0
    }

    #[doc="Sets the GCLK_ field."]
    #[inline] pub fn set_gclk_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="WDT"]
    #[inline] pub fn wdt_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WDT_ != 0"]
    #[inline] pub fn test_wdt_(&self) -> bool {
        self.wdt_() != 0
    }

    #[doc="Sets the WDT_ field."]
    #[inline] pub fn set_wdt_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="RTC"]
    #[inline] pub fn rtc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RTC_ != 0"]
    #[inline] pub fn test_rtc_(&self) -> bool {
        self.rtc_() != 0
    }

    #[doc="Sets the RTC_ field."]
    #[inline] pub fn set_rtc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="EIC"]
    #[inline] pub fn eic_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if EIC_ != 0"]
    #[inline] pub fn test_eic_(&self) -> bool {
        self.eic_() != 0
    }

    #[doc="Sets the EIC_ field."]
    #[inline] pub fn set_eic_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FREQM"]
    #[inline] pub fn freqm_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FREQM_ != 0"]
    #[inline] pub fn test_freqm_(&self) -> bool {
        self.freqm_() != 0
    }

    #[doc="Sets the FREQM_ field."]
    #[inline] pub fn set_freqm_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SERCOM0"]
    #[inline] pub fn sercom0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SERCOM0_ != 0"]
    #[inline] pub fn test_sercom0_(&self) -> bool {
        self.sercom0_() != 0
    }

    #[doc="Sets the SERCOM0_ field."]
    #[inline] pub fn set_sercom0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SERCOM1"]
    #[inline] pub fn sercom1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SERCOM1_ != 0"]
    #[inline] pub fn test_sercom1_(&self) -> bool {
        self.sercom1_() != 0
    }

    #[doc="Sets the SERCOM1_ field."]
    #[inline] pub fn set_sercom1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TC0"]
    #[inline] pub fn tc0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TC0_ != 0"]
    #[inline] pub fn test_tc0_(&self) -> bool {
        self.tc0_() != 0
    }

    #[doc="Sets the TC0_ field."]
    #[inline] pub fn set_tc0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TC1"]
    #[inline] pub fn tc1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TC1_ != 0"]
    #[inline] pub fn test_tc1_(&self) -> bool {
        self.tc1_() != 0
    }

    #[doc="Sets the TC1_ field."]
    #[inline] pub fn set_tc1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Intflaga {
    #[inline]
    fn from(other: u32) -> Self {
         Intflaga(other)
    }
}

impl ::core::fmt::Display for Intflaga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflaga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pac_() != 0 { try!(write!(f, " pac_"))}
        if self.pm_() != 0 { try!(write!(f, " pm_"))}
        if self.mclk_() != 0 { try!(write!(f, " mclk_"))}
        if self.rstc_() != 0 { try!(write!(f, " rstc_"))}
        if self.oscctrl_() != 0 { try!(write!(f, " oscctrl_"))}
        if self.osc32kctrl_() != 0 { try!(write!(f, " osc32kctrl_"))}
        if self.supc_() != 0 { try!(write!(f, " supc_"))}
        if self.gclk_() != 0 { try!(write!(f, " gclk_"))}
        if self.wdt_() != 0 { try!(write!(f, " wdt_"))}
        if self.rtc_() != 0 { try!(write!(f, " rtc_"))}
        if self.eic_() != 0 { try!(write!(f, " eic_"))}
        if self.freqm_() != 0 { try!(write!(f, " freqm_"))}
        if self.sercom0_() != 0 { try!(write!(f, " sercom0_"))}
        if self.sercom1_() != 0 { try!(write!(f, " sercom1_"))}
        if self.tc0_() != 0 { try!(write!(f, " tc0_"))}
        if self.tc1_() != 0 { try!(write!(f, " tc1_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral interrupt flag status - Bridge B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflagb(pub u32);
impl Intflagb {
    #[doc="USB"]
    #[inline] pub fn usb_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USB_ != 0"]
    #[inline] pub fn test_usb_(&self) -> bool {
        self.usb_() != 0
    }

    #[doc="Sets the USB_ field."]
    #[inline] pub fn set_usb_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DSU"]
    #[inline] pub fn dsu_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DSU_ != 0"]
    #[inline] pub fn test_dsu_(&self) -> bool {
        self.dsu_() != 0
    }

    #[doc="Sets the DSU_ field."]
    #[inline] pub fn set_dsu_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="NVMCTRL"]
    #[inline] pub fn nvmctrl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NVMCTRL_ != 0"]
    #[inline] pub fn test_nvmctrl_(&self) -> bool {
        self.nvmctrl_() != 0
    }

    #[doc="Sets the NVMCTRL_ field."]
    #[inline] pub fn set_nvmctrl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CMCC"]
    #[inline] pub fn cmcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMCC_ != 0"]
    #[inline] pub fn test_cmcc_(&self) -> bool {
        self.cmcc_() != 0
    }

    #[doc="Sets the CMCC_ field."]
    #[inline] pub fn set_cmcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PORT"]
    #[inline] pub fn port_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PORT_ != 0"]
    #[inline] pub fn test_port_(&self) -> bool {
        self.port_() != 0
    }

    #[doc="Sets the PORT_ field."]
    #[inline] pub fn set_port_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DMAC"]
    #[inline] pub fn dmac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAC_ != 0"]
    #[inline] pub fn test_dmac_(&self) -> bool {
        self.dmac_() != 0
    }

    #[doc="Sets the DMAC_ field."]
    #[inline] pub fn set_dmac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HMATRIX"]
    #[inline] pub fn hmatrix_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HMATRIX_ != 0"]
    #[inline] pub fn test_hmatrix_(&self) -> bool {
        self.hmatrix_() != 0
    }

    #[doc="Sets the HMATRIX_ field."]
    #[inline] pub fn set_hmatrix_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="EVSYS"]
    #[inline] pub fn evsys_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EVSYS_ != 0"]
    #[inline] pub fn test_evsys_(&self) -> bool {
        self.evsys_() != 0
    }

    #[doc="Sets the EVSYS_ field."]
    #[inline] pub fn set_evsys_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SERCOM2"]
    #[inline] pub fn sercom2_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SERCOM2_ != 0"]
    #[inline] pub fn test_sercom2_(&self) -> bool {
        self.sercom2_() != 0
    }

    #[doc="Sets the SERCOM2_ field."]
    #[inline] pub fn set_sercom2_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SERCOM3"]
    #[inline] pub fn sercom3_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SERCOM3_ != 0"]
    #[inline] pub fn test_sercom3_(&self) -> bool {
        self.sercom3_() != 0
    }

    #[doc="Sets the SERCOM3_ field."]
    #[inline] pub fn set_sercom3_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="TCC0"]
    #[inline] pub fn tcc0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TCC0_ != 0"]
    #[inline] pub fn test_tcc0_(&self) -> bool {
        self.tcc0_() != 0
    }

    #[doc="Sets the TCC0_ field."]
    #[inline] pub fn set_tcc0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TCC1"]
    #[inline] pub fn tcc1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TCC1_ != 0"]
    #[inline] pub fn test_tcc1_(&self) -> bool {
        self.tcc1_() != 0
    }

    #[doc="Sets the TCC1_ field."]
    #[inline] pub fn set_tcc1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TC2"]
    #[inline] pub fn tc2_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TC2_ != 0"]
    #[inline] pub fn test_tc2_(&self) -> bool {
        self.tc2_() != 0
    }

    #[doc="Sets the TC2_ field."]
    #[inline] pub fn set_tc2_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TC3"]
    #[inline] pub fn tc3_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TC3_ != 0"]
    #[inline] pub fn test_tc3_(&self) -> bool {
        self.tc3_() != 0
    }

    #[doc="Sets the TC3_ field."]
    #[inline] pub fn set_tc3_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="RAMECC"]
    #[inline] pub fn ramecc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RAMECC_ != 0"]
    #[inline] pub fn test_ramecc_(&self) -> bool {
        self.ramecc_() != 0
    }

    #[doc="Sets the RAMECC_ field."]
    #[inline] pub fn set_ramecc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Intflagb {
    #[inline]
    fn from(other: u32) -> Self {
         Intflagb(other)
    }
}

impl ::core::fmt::Display for Intflagb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflagb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usb_() != 0 { try!(write!(f, " usb_"))}
        if self.dsu_() != 0 { try!(write!(f, " dsu_"))}
        if self.nvmctrl_() != 0 { try!(write!(f, " nvmctrl_"))}
        if self.cmcc_() != 0 { try!(write!(f, " cmcc_"))}
        if self.port_() != 0 { try!(write!(f, " port_"))}
        if self.dmac_() != 0 { try!(write!(f, " dmac_"))}
        if self.hmatrix_() != 0 { try!(write!(f, " hmatrix_"))}
        if self.evsys_() != 0 { try!(write!(f, " evsys_"))}
        if self.sercom2_() != 0 { try!(write!(f, " sercom2_"))}
        if self.sercom3_() != 0 { try!(write!(f, " sercom3_"))}
        if self.tcc0_() != 0 { try!(write!(f, " tcc0_"))}
        if self.tcc1_() != 0 { try!(write!(f, " tcc1_"))}
        if self.tc2_() != 0 { try!(write!(f, " tc2_"))}
        if self.tc3_() != 0 { try!(write!(f, " tc3_"))}
        if self.ramecc_() != 0 { try!(write!(f, " ramecc_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral interrupt flag status - Bridge C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflagc(pub u32);
impl Intflagc {
    #[doc="TCC2"]
    #[inline] pub fn tcc2_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TCC2_ != 0"]
    #[inline] pub fn test_tcc2_(&self) -> bool {
        self.tcc2_() != 0
    }

    #[doc="Sets the TCC2_ field."]
    #[inline] pub fn set_tcc2_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TCC3"]
    #[inline] pub fn tcc3_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCC3_ != 0"]
    #[inline] pub fn test_tcc3_(&self) -> bool {
        self.tcc3_() != 0
    }

    #[doc="Sets the TCC3_ field."]
    #[inline] pub fn set_tcc3_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TC4"]
    #[inline] pub fn tc4_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TC4_ != 0"]
    #[inline] pub fn test_tc4_(&self) -> bool {
        self.tc4_() != 0
    }

    #[doc="Sets the TC4_ field."]
    #[inline] pub fn set_tc4_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TC5"]
    #[inline] pub fn tc5_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC5_ != 0"]
    #[inline] pub fn test_tc5_(&self) -> bool {
        self.tc5_() != 0
    }

    #[doc="Sets the TC5_ field."]
    #[inline] pub fn set_tc5_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PDEC"]
    #[inline] pub fn pdec_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PDEC_ != 0"]
    #[inline] pub fn test_pdec_(&self) -> bool {
        self.pdec_() != 0
    }

    #[doc="Sets the PDEC_ field."]
    #[inline] pub fn set_pdec_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="AC"]
    #[inline] pub fn ac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if AC_ != 0"]
    #[inline] pub fn test_ac_(&self) -> bool {
        self.ac_() != 0
    }

    #[doc="Sets the AC_ field."]
    #[inline] pub fn set_ac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AES"]
    #[inline] pub fn aes_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if AES_ != 0"]
    #[inline] pub fn test_aes_(&self) -> bool {
        self.aes_() != 0
    }

    #[doc="Sets the AES_ field."]
    #[inline] pub fn set_aes_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TRNG"]
    #[inline] pub fn trng_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TRNG_ != 0"]
    #[inline] pub fn test_trng_(&self) -> bool {
        self.trng_() != 0
    }

    #[doc="Sets the TRNG_ field."]
    #[inline] pub fn set_trng_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ICM"]
    #[inline] pub fn icm_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICM_ != 0"]
    #[inline] pub fn test_icm_(&self) -> bool {
        self.icm_() != 0
    }

    #[doc="Sets the ICM_ field."]
    #[inline] pub fn set_icm_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PUKCC"]
    #[inline] pub fn pukcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PUKCC_ != 0"]
    #[inline] pub fn test_pukcc_(&self) -> bool {
        self.pukcc_() != 0
    }

    #[doc="Sets the PUKCC_ field."]
    #[inline] pub fn set_pukcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="QSPI"]
    #[inline] pub fn qspi_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if QSPI_ != 0"]
    #[inline] pub fn test_qspi_(&self) -> bool {
        self.qspi_() != 0
    }

    #[doc="Sets the QSPI_ field."]
    #[inline] pub fn set_qspi_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CCL"]
    #[inline] pub fn ccl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CCL_ != 0"]
    #[inline] pub fn test_ccl_(&self) -> bool {
        self.ccl_() != 0
    }

    #[doc="Sets the CCL_ field."]
    #[inline] pub fn set_ccl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Intflagc {
    #[inline]
    fn from(other: u32) -> Self {
         Intflagc(other)
    }
}

impl ::core::fmt::Display for Intflagc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflagc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcc2_() != 0 { try!(write!(f, " tcc2_"))}
        if self.tcc3_() != 0 { try!(write!(f, " tcc3_"))}
        if self.tc4_() != 0 { try!(write!(f, " tc4_"))}
        if self.tc5_() != 0 { try!(write!(f, " tc5_"))}
        if self.pdec_() != 0 { try!(write!(f, " pdec_"))}
        if self.ac_() != 0 { try!(write!(f, " ac_"))}
        if self.aes_() != 0 { try!(write!(f, " aes_"))}
        if self.trng_() != 0 { try!(write!(f, " trng_"))}
        if self.icm_() != 0 { try!(write!(f, " icm_"))}
        if self.pukcc_() != 0 { try!(write!(f, " pukcc_"))}
        if self.qspi_() != 0 { try!(write!(f, " qspi_"))}
        if self.ccl_() != 0 { try!(write!(f, " ccl_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral interrupt flag status - Bridge D"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflagd(pub u32);
impl Intflagd {
    #[doc="SERCOM4"]
    #[inline] pub fn sercom4_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SERCOM4_ != 0"]
    #[inline] pub fn test_sercom4_(&self) -> bool {
        self.sercom4_() != 0
    }

    #[doc="Sets the SERCOM4_ field."]
    #[inline] pub fn set_sercom4_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SERCOM5"]
    #[inline] pub fn sercom5_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SERCOM5_ != 0"]
    #[inline] pub fn test_sercom5_(&self) -> bool {
        self.sercom5_() != 0
    }

    #[doc="Sets the SERCOM5_ field."]
    #[inline] pub fn set_sercom5_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TCC4"]
    #[inline] pub fn tcc4_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCC4_ != 0"]
    #[inline] pub fn test_tcc4_(&self) -> bool {
        self.tcc4_() != 0
    }

    #[doc="Sets the TCC4_ field."]
    #[inline] pub fn set_tcc4_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADC0"]
    #[inline] pub fn adc0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADC0_ != 0"]
    #[inline] pub fn test_adc0_(&self) -> bool {
        self.adc0_() != 0
    }

    #[doc="Sets the ADC0_ field."]
    #[inline] pub fn set_adc0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ADC1"]
    #[inline] pub fn adc1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1_ != 0"]
    #[inline] pub fn test_adc1_(&self) -> bool {
        self.adc1_() != 0
    }

    #[doc="Sets the ADC1_ field."]
    #[inline] pub fn set_adc1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DAC"]
    #[inline] pub fn dac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DAC_ != 0"]
    #[inline] pub fn test_dac_(&self) -> bool {
        self.dac_() != 0
    }

    #[doc="Sets the DAC_ field."]
    #[inline] pub fn set_dac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="I2S"]
    #[inline] pub fn i2s_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if I2S_ != 0"]
    #[inline] pub fn test_i2s_(&self) -> bool {
        self.i2s_() != 0
    }

    #[doc="Sets the I2S_ field."]
    #[inline] pub fn set_i2s_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="PCC"]
    #[inline] pub fn pcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PCC_ != 0"]
    #[inline] pub fn test_pcc_(&self) -> bool {
        self.pcc_() != 0
    }

    #[doc="Sets the PCC_ field."]
    #[inline] pub fn set_pcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Intflagd {
    #[inline]
    fn from(other: u32) -> Self {
         Intflagd(other)
    }
}

impl ::core::fmt::Display for Intflagd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflagd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sercom4_() != 0 { try!(write!(f, " sercom4_"))}
        if self.sercom5_() != 0 { try!(write!(f, " sercom5_"))}
        if self.tcc4_() != 0 { try!(write!(f, " tcc4_"))}
        if self.adc0_() != 0 { try!(write!(f, " adc0_"))}
        if self.adc1_() != 0 { try!(write!(f, " adc1_"))}
        if self.dac_() != 0 { try!(write!(f, " dac_"))}
        if self.i2s_() != 0 { try!(write!(f, " i2s_"))}
        if self.pcc_() != 0 { try!(write!(f, " pcc_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral write protection status - Bridge A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusa(pub u32);
impl Statusa {
    #[doc="PAC APB Protect Enable"]
    #[inline] pub fn pac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAC_ != 0"]
    #[inline] pub fn test_pac_(&self) -> bool {
        self.pac_() != 0
    }

    #[doc="Sets the PAC_ field."]
    #[inline] pub fn set_pac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PM APB Protect Enable"]
    #[inline] pub fn pm_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PM_ != 0"]
    #[inline] pub fn test_pm_(&self) -> bool {
        self.pm_() != 0
    }

    #[doc="Sets the PM_ field."]
    #[inline] pub fn set_pm_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MCLK APB Protect Enable"]
    #[inline] pub fn mclk_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MCLK_ != 0"]
    #[inline] pub fn test_mclk_(&self) -> bool {
        self.mclk_() != 0
    }

    #[doc="Sets the MCLK_ field."]
    #[inline] pub fn set_mclk_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RSTC APB Protect Enable"]
    #[inline] pub fn rstc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RSTC_ != 0"]
    #[inline] pub fn test_rstc_(&self) -> bool {
        self.rstc_() != 0
    }

    #[doc="Sets the RSTC_ field."]
    #[inline] pub fn set_rstc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="OSCCTRL APB Protect Enable"]
    #[inline] pub fn oscctrl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OSCCTRL_ != 0"]
    #[inline] pub fn test_oscctrl_(&self) -> bool {
        self.oscctrl_() != 0
    }

    #[doc="Sets the OSCCTRL_ field."]
    #[inline] pub fn set_oscctrl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="OSC32KCTRL APB Protect Enable"]
    #[inline] pub fn osc32kctrl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if OSC32KCTRL_ != 0"]
    #[inline] pub fn test_osc32kctrl_(&self) -> bool {
        self.osc32kctrl_() != 0
    }

    #[doc="Sets the OSC32KCTRL_ field."]
    #[inline] pub fn set_osc32kctrl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SUPC APB Protect Enable"]
    #[inline] pub fn supc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SUPC_ != 0"]
    #[inline] pub fn test_supc_(&self) -> bool {
        self.supc_() != 0
    }

    #[doc="Sets the SUPC_ field."]
    #[inline] pub fn set_supc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GCLK APB Protect Enable"]
    #[inline] pub fn gclk_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GCLK_ != 0"]
    #[inline] pub fn test_gclk_(&self) -> bool {
        self.gclk_() != 0
    }

    #[doc="Sets the GCLK_ field."]
    #[inline] pub fn set_gclk_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="WDT APB Protect Enable"]
    #[inline] pub fn wdt_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WDT_ != 0"]
    #[inline] pub fn test_wdt_(&self) -> bool {
        self.wdt_() != 0
    }

    #[doc="Sets the WDT_ field."]
    #[inline] pub fn set_wdt_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="RTC APB Protect Enable"]
    #[inline] pub fn rtc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RTC_ != 0"]
    #[inline] pub fn test_rtc_(&self) -> bool {
        self.rtc_() != 0
    }

    #[doc="Sets the RTC_ field."]
    #[inline] pub fn set_rtc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="EIC APB Protect Enable"]
    #[inline] pub fn eic_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if EIC_ != 0"]
    #[inline] pub fn test_eic_(&self) -> bool {
        self.eic_() != 0
    }

    #[doc="Sets the EIC_ field."]
    #[inline] pub fn set_eic_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FREQM APB Protect Enable"]
    #[inline] pub fn freqm_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FREQM_ != 0"]
    #[inline] pub fn test_freqm_(&self) -> bool {
        self.freqm_() != 0
    }

    #[doc="Sets the FREQM_ field."]
    #[inline] pub fn set_freqm_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SERCOM0 APB Protect Enable"]
    #[inline] pub fn sercom0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SERCOM0_ != 0"]
    #[inline] pub fn test_sercom0_(&self) -> bool {
        self.sercom0_() != 0
    }

    #[doc="Sets the SERCOM0_ field."]
    #[inline] pub fn set_sercom0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SERCOM1 APB Protect Enable"]
    #[inline] pub fn sercom1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SERCOM1_ != 0"]
    #[inline] pub fn test_sercom1_(&self) -> bool {
        self.sercom1_() != 0
    }

    #[doc="Sets the SERCOM1_ field."]
    #[inline] pub fn set_sercom1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TC0 APB Protect Enable"]
    #[inline] pub fn tc0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TC0_ != 0"]
    #[inline] pub fn test_tc0_(&self) -> bool {
        self.tc0_() != 0
    }

    #[doc="Sets the TC0_ field."]
    #[inline] pub fn set_tc0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TC1 APB Protect Enable"]
    #[inline] pub fn tc1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TC1_ != 0"]
    #[inline] pub fn test_tc1_(&self) -> bool {
        self.tc1_() != 0
    }

    #[doc="Sets the TC1_ field."]
    #[inline] pub fn set_tc1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Statusa {
    #[inline]
    fn from(other: u32) -> Self {
         Statusa(other)
    }
}

impl ::core::fmt::Display for Statusa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Statusa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pac_() != 0 { try!(write!(f, " pac_"))}
        if self.pm_() != 0 { try!(write!(f, " pm_"))}
        if self.mclk_() != 0 { try!(write!(f, " mclk_"))}
        if self.rstc_() != 0 { try!(write!(f, " rstc_"))}
        if self.oscctrl_() != 0 { try!(write!(f, " oscctrl_"))}
        if self.osc32kctrl_() != 0 { try!(write!(f, " osc32kctrl_"))}
        if self.supc_() != 0 { try!(write!(f, " supc_"))}
        if self.gclk_() != 0 { try!(write!(f, " gclk_"))}
        if self.wdt_() != 0 { try!(write!(f, " wdt_"))}
        if self.rtc_() != 0 { try!(write!(f, " rtc_"))}
        if self.eic_() != 0 { try!(write!(f, " eic_"))}
        if self.freqm_() != 0 { try!(write!(f, " freqm_"))}
        if self.sercom0_() != 0 { try!(write!(f, " sercom0_"))}
        if self.sercom1_() != 0 { try!(write!(f, " sercom1_"))}
        if self.tc0_() != 0 { try!(write!(f, " tc0_"))}
        if self.tc1_() != 0 { try!(write!(f, " tc1_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral write protection status - Bridge B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusb(pub u32);
impl Statusb {
    #[doc="USB APB Protect Enable"]
    #[inline] pub fn usb_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USB_ != 0"]
    #[inline] pub fn test_usb_(&self) -> bool {
        self.usb_() != 0
    }

    #[doc="Sets the USB_ field."]
    #[inline] pub fn set_usb_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DSU APB Protect Enable"]
    #[inline] pub fn dsu_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DSU_ != 0"]
    #[inline] pub fn test_dsu_(&self) -> bool {
        self.dsu_() != 0
    }

    #[doc="Sets the DSU_ field."]
    #[inline] pub fn set_dsu_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="NVMCTRL APB Protect Enable"]
    #[inline] pub fn nvmctrl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NVMCTRL_ != 0"]
    #[inline] pub fn test_nvmctrl_(&self) -> bool {
        self.nvmctrl_() != 0
    }

    #[doc="Sets the NVMCTRL_ field."]
    #[inline] pub fn set_nvmctrl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CMCC APB Protect Enable"]
    #[inline] pub fn cmcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMCC_ != 0"]
    #[inline] pub fn test_cmcc_(&self) -> bool {
        self.cmcc_() != 0
    }

    #[doc="Sets the CMCC_ field."]
    #[inline] pub fn set_cmcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PORT APB Protect Enable"]
    #[inline] pub fn port_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PORT_ != 0"]
    #[inline] pub fn test_port_(&self) -> bool {
        self.port_() != 0
    }

    #[doc="Sets the PORT_ field."]
    #[inline] pub fn set_port_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DMAC APB Protect Enable"]
    #[inline] pub fn dmac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAC_ != 0"]
    #[inline] pub fn test_dmac_(&self) -> bool {
        self.dmac_() != 0
    }

    #[doc="Sets the DMAC_ field."]
    #[inline] pub fn set_dmac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HMATRIX APB Protect Enable"]
    #[inline] pub fn hmatrix_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HMATRIX_ != 0"]
    #[inline] pub fn test_hmatrix_(&self) -> bool {
        self.hmatrix_() != 0
    }

    #[doc="Sets the HMATRIX_ field."]
    #[inline] pub fn set_hmatrix_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="EVSYS APB Protect Enable"]
    #[inline] pub fn evsys_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EVSYS_ != 0"]
    #[inline] pub fn test_evsys_(&self) -> bool {
        self.evsys_() != 0
    }

    #[doc="Sets the EVSYS_ field."]
    #[inline] pub fn set_evsys_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SERCOM2 APB Protect Enable"]
    #[inline] pub fn sercom2_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SERCOM2_ != 0"]
    #[inline] pub fn test_sercom2_(&self) -> bool {
        self.sercom2_() != 0
    }

    #[doc="Sets the SERCOM2_ field."]
    #[inline] pub fn set_sercom2_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SERCOM3 APB Protect Enable"]
    #[inline] pub fn sercom3_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SERCOM3_ != 0"]
    #[inline] pub fn test_sercom3_(&self) -> bool {
        self.sercom3_() != 0
    }

    #[doc="Sets the SERCOM3_ field."]
    #[inline] pub fn set_sercom3_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="TCC0 APB Protect Enable"]
    #[inline] pub fn tcc0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TCC0_ != 0"]
    #[inline] pub fn test_tcc0_(&self) -> bool {
        self.tcc0_() != 0
    }

    #[doc="Sets the TCC0_ field."]
    #[inline] pub fn set_tcc0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TCC1 APB Protect Enable"]
    #[inline] pub fn tcc1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TCC1_ != 0"]
    #[inline] pub fn test_tcc1_(&self) -> bool {
        self.tcc1_() != 0
    }

    #[doc="Sets the TCC1_ field."]
    #[inline] pub fn set_tcc1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TC2 APB Protect Enable"]
    #[inline] pub fn tc2_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TC2_ != 0"]
    #[inline] pub fn test_tc2_(&self) -> bool {
        self.tc2_() != 0
    }

    #[doc="Sets the TC2_ field."]
    #[inline] pub fn set_tc2_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TC3 APB Protect Enable"]
    #[inline] pub fn tc3_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TC3_ != 0"]
    #[inline] pub fn test_tc3_(&self) -> bool {
        self.tc3_() != 0
    }

    #[doc="Sets the TC3_ field."]
    #[inline] pub fn set_tc3_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="RAMECC APB Protect Enable"]
    #[inline] pub fn ramecc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RAMECC_ != 0"]
    #[inline] pub fn test_ramecc_(&self) -> bool {
        self.ramecc_() != 0
    }

    #[doc="Sets the RAMECC_ field."]
    #[inline] pub fn set_ramecc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Statusb {
    #[inline]
    fn from(other: u32) -> Self {
         Statusb(other)
    }
}

impl ::core::fmt::Display for Statusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Statusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usb_() != 0 { try!(write!(f, " usb_"))}
        if self.dsu_() != 0 { try!(write!(f, " dsu_"))}
        if self.nvmctrl_() != 0 { try!(write!(f, " nvmctrl_"))}
        if self.cmcc_() != 0 { try!(write!(f, " cmcc_"))}
        if self.port_() != 0 { try!(write!(f, " port_"))}
        if self.dmac_() != 0 { try!(write!(f, " dmac_"))}
        if self.hmatrix_() != 0 { try!(write!(f, " hmatrix_"))}
        if self.evsys_() != 0 { try!(write!(f, " evsys_"))}
        if self.sercom2_() != 0 { try!(write!(f, " sercom2_"))}
        if self.sercom3_() != 0 { try!(write!(f, " sercom3_"))}
        if self.tcc0_() != 0 { try!(write!(f, " tcc0_"))}
        if self.tcc1_() != 0 { try!(write!(f, " tcc1_"))}
        if self.tc2_() != 0 { try!(write!(f, " tc2_"))}
        if self.tc3_() != 0 { try!(write!(f, " tc3_"))}
        if self.ramecc_() != 0 { try!(write!(f, " ramecc_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral write protection status - Bridge C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusc(pub u32);
impl Statusc {
    #[doc="TCC2 APB Protect Enable"]
    #[inline] pub fn tcc2_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TCC2_ != 0"]
    #[inline] pub fn test_tcc2_(&self) -> bool {
        self.tcc2_() != 0
    }

    #[doc="Sets the TCC2_ field."]
    #[inline] pub fn set_tcc2_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TCC3 APB Protect Enable"]
    #[inline] pub fn tcc3_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCC3_ != 0"]
    #[inline] pub fn test_tcc3_(&self) -> bool {
        self.tcc3_() != 0
    }

    #[doc="Sets the TCC3_ field."]
    #[inline] pub fn set_tcc3_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TC4 APB Protect Enable"]
    #[inline] pub fn tc4_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TC4_ != 0"]
    #[inline] pub fn test_tc4_(&self) -> bool {
        self.tc4_() != 0
    }

    #[doc="Sets the TC4_ field."]
    #[inline] pub fn set_tc4_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TC5 APB Protect Enable"]
    #[inline] pub fn tc5_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC5_ != 0"]
    #[inline] pub fn test_tc5_(&self) -> bool {
        self.tc5_() != 0
    }

    #[doc="Sets the TC5_ field."]
    #[inline] pub fn set_tc5_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PDEC APB Protect Enable"]
    #[inline] pub fn pdec_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PDEC_ != 0"]
    #[inline] pub fn test_pdec_(&self) -> bool {
        self.pdec_() != 0
    }

    #[doc="Sets the PDEC_ field."]
    #[inline] pub fn set_pdec_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="AC APB Protect Enable"]
    #[inline] pub fn ac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if AC_ != 0"]
    #[inline] pub fn test_ac_(&self) -> bool {
        self.ac_() != 0
    }

    #[doc="Sets the AC_ field."]
    #[inline] pub fn set_ac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AES APB Protect Enable"]
    #[inline] pub fn aes_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if AES_ != 0"]
    #[inline] pub fn test_aes_(&self) -> bool {
        self.aes_() != 0
    }

    #[doc="Sets the AES_ field."]
    #[inline] pub fn set_aes_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TRNG APB Protect Enable"]
    #[inline] pub fn trng_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TRNG_ != 0"]
    #[inline] pub fn test_trng_(&self) -> bool {
        self.trng_() != 0
    }

    #[doc="Sets the TRNG_ field."]
    #[inline] pub fn set_trng_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ICM APB Protect Enable"]
    #[inline] pub fn icm_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICM_ != 0"]
    #[inline] pub fn test_icm_(&self) -> bool {
        self.icm_() != 0
    }

    #[doc="Sets the ICM_ field."]
    #[inline] pub fn set_icm_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PUKCC APB Protect Enable"]
    #[inline] pub fn pukcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PUKCC_ != 0"]
    #[inline] pub fn test_pukcc_(&self) -> bool {
        self.pukcc_() != 0
    }

    #[doc="Sets the PUKCC_ field."]
    #[inline] pub fn set_pukcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="QSPI APB Protect Enable"]
    #[inline] pub fn qspi_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if QSPI_ != 0"]
    #[inline] pub fn test_qspi_(&self) -> bool {
        self.qspi_() != 0
    }

    #[doc="Sets the QSPI_ field."]
    #[inline] pub fn set_qspi_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CCL APB Protect Enable"]
    #[inline] pub fn ccl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CCL_ != 0"]
    #[inline] pub fn test_ccl_(&self) -> bool {
        self.ccl_() != 0
    }

    #[doc="Sets the CCL_ field."]
    #[inline] pub fn set_ccl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Statusc {
    #[inline]
    fn from(other: u32) -> Self {
         Statusc(other)
    }
}

impl ::core::fmt::Display for Statusc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Statusc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcc2_() != 0 { try!(write!(f, " tcc2_"))}
        if self.tcc3_() != 0 { try!(write!(f, " tcc3_"))}
        if self.tc4_() != 0 { try!(write!(f, " tc4_"))}
        if self.tc5_() != 0 { try!(write!(f, " tc5_"))}
        if self.pdec_() != 0 { try!(write!(f, " pdec_"))}
        if self.ac_() != 0 { try!(write!(f, " ac_"))}
        if self.aes_() != 0 { try!(write!(f, " aes_"))}
        if self.trng_() != 0 { try!(write!(f, " trng_"))}
        if self.icm_() != 0 { try!(write!(f, " icm_"))}
        if self.pukcc_() != 0 { try!(write!(f, " pukcc_"))}
        if self.qspi_() != 0 { try!(write!(f, " qspi_"))}
        if self.ccl_() != 0 { try!(write!(f, " ccl_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral write protection status - Bridge D"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusd(pub u32);
impl Statusd {
    #[doc="SERCOM4 APB Protect Enable"]
    #[inline] pub fn sercom4_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SERCOM4_ != 0"]
    #[inline] pub fn test_sercom4_(&self) -> bool {
        self.sercom4_() != 0
    }

    #[doc="Sets the SERCOM4_ field."]
    #[inline] pub fn set_sercom4_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SERCOM5 APB Protect Enable"]
    #[inline] pub fn sercom5_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SERCOM5_ != 0"]
    #[inline] pub fn test_sercom5_(&self) -> bool {
        self.sercom5_() != 0
    }

    #[doc="Sets the SERCOM5_ field."]
    #[inline] pub fn set_sercom5_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TCC4 APB Protect Enable"]
    #[inline] pub fn tcc4_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCC4_ != 0"]
    #[inline] pub fn test_tcc4_(&self) -> bool {
        self.tcc4_() != 0
    }

    #[doc="Sets the TCC4_ field."]
    #[inline] pub fn set_tcc4_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADC0 APB Protect Enable"]
    #[inline] pub fn adc0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADC0_ != 0"]
    #[inline] pub fn test_adc0_(&self) -> bool {
        self.adc0_() != 0
    }

    #[doc="Sets the ADC0_ field."]
    #[inline] pub fn set_adc0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ADC1 APB Protect Enable"]
    #[inline] pub fn adc1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1_ != 0"]
    #[inline] pub fn test_adc1_(&self) -> bool {
        self.adc1_() != 0
    }

    #[doc="Sets the ADC1_ field."]
    #[inline] pub fn set_adc1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DAC APB Protect Enable"]
    #[inline] pub fn dac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DAC_ != 0"]
    #[inline] pub fn test_dac_(&self) -> bool {
        self.dac_() != 0
    }

    #[doc="Sets the DAC_ field."]
    #[inline] pub fn set_dac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="I2S APB Protect Enable"]
    #[inline] pub fn i2s_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if I2S_ != 0"]
    #[inline] pub fn test_i2s_(&self) -> bool {
        self.i2s_() != 0
    }

    #[doc="Sets the I2S_ field."]
    #[inline] pub fn set_i2s_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="PCC APB Protect Enable"]
    #[inline] pub fn pcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PCC_ != 0"]
    #[inline] pub fn test_pcc_(&self) -> bool {
        self.pcc_() != 0
    }

    #[doc="Sets the PCC_ field."]
    #[inline] pub fn set_pcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Statusd {
    #[inline]
    fn from(other: u32) -> Self {
         Statusd(other)
    }
}

impl ::core::fmt::Display for Statusd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Statusd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sercom4_() != 0 { try!(write!(f, " sercom4_"))}
        if self.sercom5_() != 0 { try!(write!(f, " sercom5_"))}
        if self.tcc4_() != 0 { try!(write!(f, " tcc4_"))}
        if self.adc0_() != 0 { try!(write!(f, " adc0_"))}
        if self.adc1_() != 0 { try!(write!(f, " adc1_"))}
        if self.dac_() != 0 { try!(write!(f, " dac_"))}
        if self.i2s_() != 0 { try!(write!(f, " i2s_"))}
        if self.pcc_() != 0 { try!(write!(f, " pcc_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

