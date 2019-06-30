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
    #[inline] pub fn flash(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FLASH != 0"]
    #[inline] pub fn test_flash(&self) -> bool {
        self.flash() != 0
    }

    #[doc="Sets the FLASH field."]
    #[inline] pub fn set_flash<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FLASH_ALT"]
    #[inline] pub fn flash_alt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FLASH_ALT != 0"]
    #[inline] pub fn test_flash_alt(&self) -> bool {
        self.flash_alt() != 0
    }

    #[doc="Sets the FLASH_ALT field."]
    #[inline] pub fn set_flash_alt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SEEPROM"]
    #[inline] pub fn seeprom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SEEPROM != 0"]
    #[inline] pub fn test_seeprom(&self) -> bool {
        self.seeprom() != 0
    }

    #[doc="Sets the SEEPROM field."]
    #[inline] pub fn set_seeprom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RAMCM4S"]
    #[inline] pub fn ramcm4s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RAMCM4S != 0"]
    #[inline] pub fn test_ramcm4s(&self) -> bool {
        self.ramcm4s() != 0
    }

    #[doc="Sets the RAMCM4S field."]
    #[inline] pub fn set_ramcm4s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RAMPPPDSU"]
    #[inline] pub fn rampppdsu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RAMPPPDSU != 0"]
    #[inline] pub fn test_rampppdsu(&self) -> bool {
        self.rampppdsu() != 0
    }

    #[doc="Sets the RAMPPPDSU field."]
    #[inline] pub fn set_rampppdsu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RAMDMAWR"]
    #[inline] pub fn ramdmawr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RAMDMAWR != 0"]
    #[inline] pub fn test_ramdmawr(&self) -> bool {
        self.ramdmawr() != 0
    }

    #[doc="Sets the RAMDMAWR field."]
    #[inline] pub fn set_ramdmawr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RAMDMACICM"]
    #[inline] pub fn ramdmacicm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RAMDMACICM != 0"]
    #[inline] pub fn test_ramdmacicm(&self) -> bool {
        self.ramdmacicm() != 0
    }

    #[doc="Sets the RAMDMACICM field."]
    #[inline] pub fn set_ramdmacicm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="HPB0"]
    #[inline] pub fn hpb0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if HPB0 != 0"]
    #[inline] pub fn test_hpb0(&self) -> bool {
        self.hpb0() != 0
    }

    #[doc="Sets the HPB0 field."]
    #[inline] pub fn set_hpb0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="HPB1"]
    #[inline] pub fn hpb1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HPB1 != 0"]
    #[inline] pub fn test_hpb1(&self) -> bool {
        self.hpb1() != 0
    }

    #[doc="Sets the HPB1 field."]
    #[inline] pub fn set_hpb1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HPB2"]
    #[inline] pub fn hpb2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HPB2 != 0"]
    #[inline] pub fn test_hpb2(&self) -> bool {
        self.hpb2() != 0
    }

    #[doc="Sets the HPB2 field."]
    #[inline] pub fn set_hpb2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HPB3"]
    #[inline] pub fn hpb3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HPB3 != 0"]
    #[inline] pub fn test_hpb3(&self) -> bool {
        self.hpb3() != 0
    }

    #[doc="Sets the HPB3 field."]
    #[inline] pub fn set_hpb3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="PUKCC"]
    #[inline] pub fn pukcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PUKCC != 0"]
    #[inline] pub fn test_pukcc(&self) -> bool {
        self.pukcc() != 0
    }

    #[doc="Sets the PUKCC field."]
    #[inline] pub fn set_pukcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SDHC0"]
    #[inline] pub fn sdhc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SDHC0 != 0"]
    #[inline] pub fn test_sdhc0(&self) -> bool {
        self.sdhc0() != 0
    }

    #[doc="Sets the SDHC0 field."]
    #[inline] pub fn set_sdhc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="QSPI"]
    #[inline] pub fn qspi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if QSPI != 0"]
    #[inline] pub fn test_qspi(&self) -> bool {
        self.qspi() != 0
    }

    #[doc="Sets the QSPI field."]
    #[inline] pub fn set_qspi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="BKUPRAM"]
    #[inline] pub fn bkupram(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BKUPRAM != 0"]
    #[inline] pub fn test_bkupram(&self) -> bool {
        self.bkupram() != 0
    }

    #[doc="Sets the BKUPRAM field."]
    #[inline] pub fn set_bkupram<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.flash() != 0 { try!(write!(f, " flash"))}
        if self.flash_alt() != 0 { try!(write!(f, " flash_alt"))}
        if self.seeprom() != 0 { try!(write!(f, " seeprom"))}
        if self.ramcm4s() != 0 { try!(write!(f, " ramcm4s"))}
        if self.rampppdsu() != 0 { try!(write!(f, " rampppdsu"))}
        if self.ramdmawr() != 0 { try!(write!(f, " ramdmawr"))}
        if self.ramdmacicm() != 0 { try!(write!(f, " ramdmacicm"))}
        if self.hpb0() != 0 { try!(write!(f, " hpb0"))}
        if self.hpb1() != 0 { try!(write!(f, " hpb1"))}
        if self.hpb2() != 0 { try!(write!(f, " hpb2"))}
        if self.hpb3() != 0 { try!(write!(f, " hpb3"))}
        if self.pukcc() != 0 { try!(write!(f, " pukcc"))}
        if self.sdhc0() != 0 { try!(write!(f, " sdhc0"))}
        if self.qspi() != 0 { try!(write!(f, " qspi"))}
        if self.bkupram() != 0 { try!(write!(f, " bkupram"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral interrupt flag status - Bridge A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflaga(pub u32);
impl Intflaga {
    #[doc="PAC"]
    #[inline] pub fn pac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAC != 0"]
    #[inline] pub fn test_pac(&self) -> bool {
        self.pac() != 0
    }

    #[doc="Sets the PAC field."]
    #[inline] pub fn set_pac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PM"]
    #[inline] pub fn pm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PM != 0"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm() != 0
    }

    #[doc="Sets the PM field."]
    #[inline] pub fn set_pm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MCLK"]
    #[inline] pub fn mclk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MCLK != 0"]
    #[inline] pub fn test_mclk(&self) -> bool {
        self.mclk() != 0
    }

    #[doc="Sets the MCLK field."]
    #[inline] pub fn set_mclk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RSTC"]
    #[inline] pub fn rstc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RSTC != 0"]
    #[inline] pub fn test_rstc(&self) -> bool {
        self.rstc() != 0
    }

    #[doc="Sets the RSTC field."]
    #[inline] pub fn set_rstc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="OSCCTRL"]
    #[inline] pub fn oscctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OSCCTRL != 0"]
    #[inline] pub fn test_oscctrl(&self) -> bool {
        self.oscctrl() != 0
    }

    #[doc="Sets the OSCCTRL field."]
    #[inline] pub fn set_oscctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="OSC32KCTRL"]
    #[inline] pub fn osc32kctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if OSC32KCTRL != 0"]
    #[inline] pub fn test_osc32kctrl(&self) -> bool {
        self.osc32kctrl() != 0
    }

    #[doc="Sets the OSC32KCTRL field."]
    #[inline] pub fn set_osc32kctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SUPC"]
    #[inline] pub fn supc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SUPC != 0"]
    #[inline] pub fn test_supc(&self) -> bool {
        self.supc() != 0
    }

    #[doc="Sets the SUPC field."]
    #[inline] pub fn set_supc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GCLK"]
    #[inline] pub fn gclk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GCLK != 0"]
    #[inline] pub fn test_gclk(&self) -> bool {
        self.gclk() != 0
    }

    #[doc="Sets the GCLK field."]
    #[inline] pub fn set_gclk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="WDT"]
    #[inline] pub fn wdt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
    #[inline] pub fn set_wdt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="RTC"]
    #[inline] pub fn rtc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RTC != 0"]
    #[inline] pub fn test_rtc(&self) -> bool {
        self.rtc() != 0
    }

    #[doc="Sets the RTC field."]
    #[inline] pub fn set_rtc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="EIC"]
    #[inline] pub fn eic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if EIC != 0"]
    #[inline] pub fn test_eic(&self) -> bool {
        self.eic() != 0
    }

    #[doc="Sets the EIC field."]
    #[inline] pub fn set_eic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FREQM"]
    #[inline] pub fn freqm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FREQM != 0"]
    #[inline] pub fn test_freqm(&self) -> bool {
        self.freqm() != 0
    }

    #[doc="Sets the FREQM field."]
    #[inline] pub fn set_freqm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SERCOM0"]
    #[inline] pub fn sercom0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SERCOM0 != 0"]
    #[inline] pub fn test_sercom0(&self) -> bool {
        self.sercom0() != 0
    }

    #[doc="Sets the SERCOM0 field."]
    #[inline] pub fn set_sercom0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SERCOM1"]
    #[inline] pub fn sercom1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SERCOM1 != 0"]
    #[inline] pub fn test_sercom1(&self) -> bool {
        self.sercom1() != 0
    }

    #[doc="Sets the SERCOM1 field."]
    #[inline] pub fn set_sercom1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TC0"]
    #[inline] pub fn tc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TC0 != 0"]
    #[inline] pub fn test_tc0(&self) -> bool {
        self.tc0() != 0
    }

    #[doc="Sets the TC0 field."]
    #[inline] pub fn set_tc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TC1"]
    #[inline] pub fn tc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TC1 != 0"]
    #[inline] pub fn test_tc1(&self) -> bool {
        self.tc1() != 0
    }

    #[doc="Sets the TC1 field."]
    #[inline] pub fn set_tc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.pac() != 0 { try!(write!(f, " pac"))}
        if self.pm() != 0 { try!(write!(f, " pm"))}
        if self.mclk() != 0 { try!(write!(f, " mclk"))}
        if self.rstc() != 0 { try!(write!(f, " rstc"))}
        if self.oscctrl() != 0 { try!(write!(f, " oscctrl"))}
        if self.osc32kctrl() != 0 { try!(write!(f, " osc32kctrl"))}
        if self.supc() != 0 { try!(write!(f, " supc"))}
        if self.gclk() != 0 { try!(write!(f, " gclk"))}
        if self.wdt() != 0 { try!(write!(f, " wdt"))}
        if self.rtc() != 0 { try!(write!(f, " rtc"))}
        if self.eic() != 0 { try!(write!(f, " eic"))}
        if self.freqm() != 0 { try!(write!(f, " freqm"))}
        if self.sercom0() != 0 { try!(write!(f, " sercom0"))}
        if self.sercom1() != 0 { try!(write!(f, " sercom1"))}
        if self.tc0() != 0 { try!(write!(f, " tc0"))}
        if self.tc1() != 0 { try!(write!(f, " tc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral interrupt flag status - Bridge B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflagb(pub u32);
impl Intflagb {
    #[doc="USB"]
    #[inline] pub fn usb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USB != 0"]
    #[inline] pub fn test_usb(&self) -> bool {
        self.usb() != 0
    }

    #[doc="Sets the USB field."]
    #[inline] pub fn set_usb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DSU"]
    #[inline] pub fn dsu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DSU != 0"]
    #[inline] pub fn test_dsu(&self) -> bool {
        self.dsu() != 0
    }

    #[doc="Sets the DSU field."]
    #[inline] pub fn set_dsu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="NVMCTRL"]
    #[inline] pub fn nvmctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NVMCTRL != 0"]
    #[inline] pub fn test_nvmctrl(&self) -> bool {
        self.nvmctrl() != 0
    }

    #[doc="Sets the NVMCTRL field."]
    #[inline] pub fn set_nvmctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CMCC"]
    #[inline] pub fn cmcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMCC != 0"]
    #[inline] pub fn test_cmcc(&self) -> bool {
        self.cmcc() != 0
    }

    #[doc="Sets the CMCC field."]
    #[inline] pub fn set_cmcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PORT"]
    #[inline] pub fn port(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PORT != 0"]
    #[inline] pub fn test_port(&self) -> bool {
        self.port() != 0
    }

    #[doc="Sets the PORT field."]
    #[inline] pub fn set_port<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DMAC"]
    #[inline] pub fn dmac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAC != 0"]
    #[inline] pub fn test_dmac(&self) -> bool {
        self.dmac() != 0
    }

    #[doc="Sets the DMAC field."]
    #[inline] pub fn set_dmac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HMATRIX"]
    #[inline] pub fn hmatrix(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HMATRIX != 0"]
    #[inline] pub fn test_hmatrix(&self) -> bool {
        self.hmatrix() != 0
    }

    #[doc="Sets the HMATRIX field."]
    #[inline] pub fn set_hmatrix<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="EVSYS"]
    #[inline] pub fn evsys(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EVSYS != 0"]
    #[inline] pub fn test_evsys(&self) -> bool {
        self.evsys() != 0
    }

    #[doc="Sets the EVSYS field."]
    #[inline] pub fn set_evsys<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SERCOM2"]
    #[inline] pub fn sercom2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SERCOM2 != 0"]
    #[inline] pub fn test_sercom2(&self) -> bool {
        self.sercom2() != 0
    }

    #[doc="Sets the SERCOM2 field."]
    #[inline] pub fn set_sercom2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SERCOM3"]
    #[inline] pub fn sercom3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SERCOM3 != 0"]
    #[inline] pub fn test_sercom3(&self) -> bool {
        self.sercom3() != 0
    }

    #[doc="Sets the SERCOM3 field."]
    #[inline] pub fn set_sercom3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="TCC0"]
    #[inline] pub fn tcc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TCC0 != 0"]
    #[inline] pub fn test_tcc0(&self) -> bool {
        self.tcc0() != 0
    }

    #[doc="Sets the TCC0 field."]
    #[inline] pub fn set_tcc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TCC1"]
    #[inline] pub fn tcc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TCC1 != 0"]
    #[inline] pub fn test_tcc1(&self) -> bool {
        self.tcc1() != 0
    }

    #[doc="Sets the TCC1 field."]
    #[inline] pub fn set_tcc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TC2"]
    #[inline] pub fn tc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TC2 != 0"]
    #[inline] pub fn test_tc2(&self) -> bool {
        self.tc2() != 0
    }

    #[doc="Sets the TC2 field."]
    #[inline] pub fn set_tc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TC3"]
    #[inline] pub fn tc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TC3 != 0"]
    #[inline] pub fn test_tc3(&self) -> bool {
        self.tc3() != 0
    }

    #[doc="Sets the TC3 field."]
    #[inline] pub fn set_tc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="RAMECC"]
    #[inline] pub fn ramecc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RAMECC != 0"]
    #[inline] pub fn test_ramecc(&self) -> bool {
        self.ramecc() != 0
    }

    #[doc="Sets the RAMECC field."]
    #[inline] pub fn set_ramecc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.usb() != 0 { try!(write!(f, " usb"))}
        if self.dsu() != 0 { try!(write!(f, " dsu"))}
        if self.nvmctrl() != 0 { try!(write!(f, " nvmctrl"))}
        if self.cmcc() != 0 { try!(write!(f, " cmcc"))}
        if self.port() != 0 { try!(write!(f, " port"))}
        if self.dmac() != 0 { try!(write!(f, " dmac"))}
        if self.hmatrix() != 0 { try!(write!(f, " hmatrix"))}
        if self.evsys() != 0 { try!(write!(f, " evsys"))}
        if self.sercom2() != 0 { try!(write!(f, " sercom2"))}
        if self.sercom3() != 0 { try!(write!(f, " sercom3"))}
        if self.tcc0() != 0 { try!(write!(f, " tcc0"))}
        if self.tcc1() != 0 { try!(write!(f, " tcc1"))}
        if self.tc2() != 0 { try!(write!(f, " tc2"))}
        if self.tc3() != 0 { try!(write!(f, " tc3"))}
        if self.ramecc() != 0 { try!(write!(f, " ramecc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral interrupt flag status - Bridge C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflagc(pub u32);
impl Intflagc {
    #[doc="TCC2"]
    #[inline] pub fn tcc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TCC2 != 0"]
    #[inline] pub fn test_tcc2(&self) -> bool {
        self.tcc2() != 0
    }

    #[doc="Sets the TCC2 field."]
    #[inline] pub fn set_tcc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TCC3"]
    #[inline] pub fn tcc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCC3 != 0"]
    #[inline] pub fn test_tcc3(&self) -> bool {
        self.tcc3() != 0
    }

    #[doc="Sets the TCC3 field."]
    #[inline] pub fn set_tcc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TC4"]
    #[inline] pub fn tc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TC4 != 0"]
    #[inline] pub fn test_tc4(&self) -> bool {
        self.tc4() != 0
    }

    #[doc="Sets the TC4 field."]
    #[inline] pub fn set_tc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TC5"]
    #[inline] pub fn tc5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC5 != 0"]
    #[inline] pub fn test_tc5(&self) -> bool {
        self.tc5() != 0
    }

    #[doc="Sets the TC5 field."]
    #[inline] pub fn set_tc5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PDEC"]
    #[inline] pub fn pdec(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PDEC != 0"]
    #[inline] pub fn test_pdec(&self) -> bool {
        self.pdec() != 0
    }

    #[doc="Sets the PDEC field."]
    #[inline] pub fn set_pdec<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="AC"]
    #[inline] pub fn ac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if AC != 0"]
    #[inline] pub fn test_ac(&self) -> bool {
        self.ac() != 0
    }

    #[doc="Sets the AC field."]
    #[inline] pub fn set_ac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AES"]
    #[inline] pub fn aes(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if AES != 0"]
    #[inline] pub fn test_aes(&self) -> bool {
        self.aes() != 0
    }

    #[doc="Sets the AES field."]
    #[inline] pub fn set_aes<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TRNG"]
    #[inline] pub fn trng(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TRNG != 0"]
    #[inline] pub fn test_trng(&self) -> bool {
        self.trng() != 0
    }

    #[doc="Sets the TRNG field."]
    #[inline] pub fn set_trng<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ICM"]
    #[inline] pub fn icm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICM != 0"]
    #[inline] pub fn test_icm(&self) -> bool {
        self.icm() != 0
    }

    #[doc="Sets the ICM field."]
    #[inline] pub fn set_icm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PUKCC"]
    #[inline] pub fn pukcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PUKCC != 0"]
    #[inline] pub fn test_pukcc(&self) -> bool {
        self.pukcc() != 0
    }

    #[doc="Sets the PUKCC field."]
    #[inline] pub fn set_pukcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="QSPI"]
    #[inline] pub fn qspi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if QSPI != 0"]
    #[inline] pub fn test_qspi(&self) -> bool {
        self.qspi() != 0
    }

    #[doc="Sets the QSPI field."]
    #[inline] pub fn set_qspi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CCL"]
    #[inline] pub fn ccl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CCL != 0"]
    #[inline] pub fn test_ccl(&self) -> bool {
        self.ccl() != 0
    }

    #[doc="Sets the CCL field."]
    #[inline] pub fn set_ccl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.tcc2() != 0 { try!(write!(f, " tcc2"))}
        if self.tcc3() != 0 { try!(write!(f, " tcc3"))}
        if self.tc4() != 0 { try!(write!(f, " tc4"))}
        if self.tc5() != 0 { try!(write!(f, " tc5"))}
        if self.pdec() != 0 { try!(write!(f, " pdec"))}
        if self.ac() != 0 { try!(write!(f, " ac"))}
        if self.aes() != 0 { try!(write!(f, " aes"))}
        if self.trng() != 0 { try!(write!(f, " trng"))}
        if self.icm() != 0 { try!(write!(f, " icm"))}
        if self.pukcc() != 0 { try!(write!(f, " pukcc"))}
        if self.qspi() != 0 { try!(write!(f, " qspi"))}
        if self.ccl() != 0 { try!(write!(f, " ccl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral interrupt flag status - Bridge D"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflagd(pub u32);
impl Intflagd {
    #[doc="SERCOM4"]
    #[inline] pub fn sercom4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SERCOM4 != 0"]
    #[inline] pub fn test_sercom4(&self) -> bool {
        self.sercom4() != 0
    }

    #[doc="Sets the SERCOM4 field."]
    #[inline] pub fn set_sercom4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SERCOM5"]
    #[inline] pub fn sercom5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SERCOM5 != 0"]
    #[inline] pub fn test_sercom5(&self) -> bool {
        self.sercom5() != 0
    }

    #[doc="Sets the SERCOM5 field."]
    #[inline] pub fn set_sercom5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TCC4"]
    #[inline] pub fn tcc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCC4 != 0"]
    #[inline] pub fn test_tcc4(&self) -> bool {
        self.tcc4() != 0
    }

    #[doc="Sets the TCC4 field."]
    #[inline] pub fn set_tcc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADC0"]
    #[inline] pub fn adc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADC0 != 0"]
    #[inline] pub fn test_adc0(&self) -> bool {
        self.adc0() != 0
    }

    #[doc="Sets the ADC0 field."]
    #[inline] pub fn set_adc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ADC1"]
    #[inline] pub fn adc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1 != 0"]
    #[inline] pub fn test_adc1(&self) -> bool {
        self.adc1() != 0
    }

    #[doc="Sets the ADC1 field."]
    #[inline] pub fn set_adc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DAC"]
    #[inline] pub fn dac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DAC != 0"]
    #[inline] pub fn test_dac(&self) -> bool {
        self.dac() != 0
    }

    #[doc="Sets the DAC field."]
    #[inline] pub fn set_dac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="I2S"]
    #[inline] pub fn i2s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if I2S != 0"]
    #[inline] pub fn test_i2s(&self) -> bool {
        self.i2s() != 0
    }

    #[doc="Sets the I2S field."]
    #[inline] pub fn set_i2s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="PCC"]
    #[inline] pub fn pcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PCC != 0"]
    #[inline] pub fn test_pcc(&self) -> bool {
        self.pcc() != 0
    }

    #[doc="Sets the PCC field."]
    #[inline] pub fn set_pcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.sercom4() != 0 { try!(write!(f, " sercom4"))}
        if self.sercom5() != 0 { try!(write!(f, " sercom5"))}
        if self.tcc4() != 0 { try!(write!(f, " tcc4"))}
        if self.adc0() != 0 { try!(write!(f, " adc0"))}
        if self.adc1() != 0 { try!(write!(f, " adc1"))}
        if self.dac() != 0 { try!(write!(f, " dac"))}
        if self.i2s() != 0 { try!(write!(f, " i2s"))}
        if self.pcc() != 0 { try!(write!(f, " pcc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral write protection status - Bridge A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusa(pub u32);
impl Statusa {
    #[doc="PAC APB Protect Enable"]
    #[inline] pub fn pac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAC != 0"]
    #[inline] pub fn test_pac(&self) -> bool {
        self.pac() != 0
    }

    #[doc="Sets the PAC field."]
    #[inline] pub fn set_pac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PM APB Protect Enable"]
    #[inline] pub fn pm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PM != 0"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm() != 0
    }

    #[doc="Sets the PM field."]
    #[inline] pub fn set_pm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MCLK APB Protect Enable"]
    #[inline] pub fn mclk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MCLK != 0"]
    #[inline] pub fn test_mclk(&self) -> bool {
        self.mclk() != 0
    }

    #[doc="Sets the MCLK field."]
    #[inline] pub fn set_mclk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RSTC APB Protect Enable"]
    #[inline] pub fn rstc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RSTC != 0"]
    #[inline] pub fn test_rstc(&self) -> bool {
        self.rstc() != 0
    }

    #[doc="Sets the RSTC field."]
    #[inline] pub fn set_rstc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="OSCCTRL APB Protect Enable"]
    #[inline] pub fn oscctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OSCCTRL != 0"]
    #[inline] pub fn test_oscctrl(&self) -> bool {
        self.oscctrl() != 0
    }

    #[doc="Sets the OSCCTRL field."]
    #[inline] pub fn set_oscctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="OSC32KCTRL APB Protect Enable"]
    #[inline] pub fn osc32kctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if OSC32KCTRL != 0"]
    #[inline] pub fn test_osc32kctrl(&self) -> bool {
        self.osc32kctrl() != 0
    }

    #[doc="Sets the OSC32KCTRL field."]
    #[inline] pub fn set_osc32kctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SUPC APB Protect Enable"]
    #[inline] pub fn supc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SUPC != 0"]
    #[inline] pub fn test_supc(&self) -> bool {
        self.supc() != 0
    }

    #[doc="Sets the SUPC field."]
    #[inline] pub fn set_supc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GCLK APB Protect Enable"]
    #[inline] pub fn gclk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GCLK != 0"]
    #[inline] pub fn test_gclk(&self) -> bool {
        self.gclk() != 0
    }

    #[doc="Sets the GCLK field."]
    #[inline] pub fn set_gclk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="WDT APB Protect Enable"]
    #[inline] pub fn wdt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WDT != 0"]
    #[inline] pub fn test_wdt(&self) -> bool {
        self.wdt() != 0
    }

    #[doc="Sets the WDT field."]
    #[inline] pub fn set_wdt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="RTC APB Protect Enable"]
    #[inline] pub fn rtc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RTC != 0"]
    #[inline] pub fn test_rtc(&self) -> bool {
        self.rtc() != 0
    }

    #[doc="Sets the RTC field."]
    #[inline] pub fn set_rtc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="EIC APB Protect Enable"]
    #[inline] pub fn eic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if EIC != 0"]
    #[inline] pub fn test_eic(&self) -> bool {
        self.eic() != 0
    }

    #[doc="Sets the EIC field."]
    #[inline] pub fn set_eic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FREQM APB Protect Enable"]
    #[inline] pub fn freqm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FREQM != 0"]
    #[inline] pub fn test_freqm(&self) -> bool {
        self.freqm() != 0
    }

    #[doc="Sets the FREQM field."]
    #[inline] pub fn set_freqm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SERCOM0 APB Protect Enable"]
    #[inline] pub fn sercom0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SERCOM0 != 0"]
    #[inline] pub fn test_sercom0(&self) -> bool {
        self.sercom0() != 0
    }

    #[doc="Sets the SERCOM0 field."]
    #[inline] pub fn set_sercom0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SERCOM1 APB Protect Enable"]
    #[inline] pub fn sercom1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SERCOM1 != 0"]
    #[inline] pub fn test_sercom1(&self) -> bool {
        self.sercom1() != 0
    }

    #[doc="Sets the SERCOM1 field."]
    #[inline] pub fn set_sercom1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TC0 APB Protect Enable"]
    #[inline] pub fn tc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TC0 != 0"]
    #[inline] pub fn test_tc0(&self) -> bool {
        self.tc0() != 0
    }

    #[doc="Sets the TC0 field."]
    #[inline] pub fn set_tc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TC1 APB Protect Enable"]
    #[inline] pub fn tc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TC1 != 0"]
    #[inline] pub fn test_tc1(&self) -> bool {
        self.tc1() != 0
    }

    #[doc="Sets the TC1 field."]
    #[inline] pub fn set_tc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.pac() != 0 { try!(write!(f, " pac"))}
        if self.pm() != 0 { try!(write!(f, " pm"))}
        if self.mclk() != 0 { try!(write!(f, " mclk"))}
        if self.rstc() != 0 { try!(write!(f, " rstc"))}
        if self.oscctrl() != 0 { try!(write!(f, " oscctrl"))}
        if self.osc32kctrl() != 0 { try!(write!(f, " osc32kctrl"))}
        if self.supc() != 0 { try!(write!(f, " supc"))}
        if self.gclk() != 0 { try!(write!(f, " gclk"))}
        if self.wdt() != 0 { try!(write!(f, " wdt"))}
        if self.rtc() != 0 { try!(write!(f, " rtc"))}
        if self.eic() != 0 { try!(write!(f, " eic"))}
        if self.freqm() != 0 { try!(write!(f, " freqm"))}
        if self.sercom0() != 0 { try!(write!(f, " sercom0"))}
        if self.sercom1() != 0 { try!(write!(f, " sercom1"))}
        if self.tc0() != 0 { try!(write!(f, " tc0"))}
        if self.tc1() != 0 { try!(write!(f, " tc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral write protection status - Bridge B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusb(pub u32);
impl Statusb {
    #[doc="USB APB Protect Enable"]
    #[inline] pub fn usb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USB != 0"]
    #[inline] pub fn test_usb(&self) -> bool {
        self.usb() != 0
    }

    #[doc="Sets the USB field."]
    #[inline] pub fn set_usb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DSU APB Protect Enable"]
    #[inline] pub fn dsu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DSU != 0"]
    #[inline] pub fn test_dsu(&self) -> bool {
        self.dsu() != 0
    }

    #[doc="Sets the DSU field."]
    #[inline] pub fn set_dsu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="NVMCTRL APB Protect Enable"]
    #[inline] pub fn nvmctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NVMCTRL != 0"]
    #[inline] pub fn test_nvmctrl(&self) -> bool {
        self.nvmctrl() != 0
    }

    #[doc="Sets the NVMCTRL field."]
    #[inline] pub fn set_nvmctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CMCC APB Protect Enable"]
    #[inline] pub fn cmcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMCC != 0"]
    #[inline] pub fn test_cmcc(&self) -> bool {
        self.cmcc() != 0
    }

    #[doc="Sets the CMCC field."]
    #[inline] pub fn set_cmcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PORT APB Protect Enable"]
    #[inline] pub fn port(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PORT != 0"]
    #[inline] pub fn test_port(&self) -> bool {
        self.port() != 0
    }

    #[doc="Sets the PORT field."]
    #[inline] pub fn set_port<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DMAC APB Protect Enable"]
    #[inline] pub fn dmac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAC != 0"]
    #[inline] pub fn test_dmac(&self) -> bool {
        self.dmac() != 0
    }

    #[doc="Sets the DMAC field."]
    #[inline] pub fn set_dmac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HMATRIX APB Protect Enable"]
    #[inline] pub fn hmatrix(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HMATRIX != 0"]
    #[inline] pub fn test_hmatrix(&self) -> bool {
        self.hmatrix() != 0
    }

    #[doc="Sets the HMATRIX field."]
    #[inline] pub fn set_hmatrix<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="EVSYS APB Protect Enable"]
    #[inline] pub fn evsys(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EVSYS != 0"]
    #[inline] pub fn test_evsys(&self) -> bool {
        self.evsys() != 0
    }

    #[doc="Sets the EVSYS field."]
    #[inline] pub fn set_evsys<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SERCOM2 APB Protect Enable"]
    #[inline] pub fn sercom2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SERCOM2 != 0"]
    #[inline] pub fn test_sercom2(&self) -> bool {
        self.sercom2() != 0
    }

    #[doc="Sets the SERCOM2 field."]
    #[inline] pub fn set_sercom2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SERCOM3 APB Protect Enable"]
    #[inline] pub fn sercom3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SERCOM3 != 0"]
    #[inline] pub fn test_sercom3(&self) -> bool {
        self.sercom3() != 0
    }

    #[doc="Sets the SERCOM3 field."]
    #[inline] pub fn set_sercom3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="TCC0 APB Protect Enable"]
    #[inline] pub fn tcc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TCC0 != 0"]
    #[inline] pub fn test_tcc0(&self) -> bool {
        self.tcc0() != 0
    }

    #[doc="Sets the TCC0 field."]
    #[inline] pub fn set_tcc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TCC1 APB Protect Enable"]
    #[inline] pub fn tcc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TCC1 != 0"]
    #[inline] pub fn test_tcc1(&self) -> bool {
        self.tcc1() != 0
    }

    #[doc="Sets the TCC1 field."]
    #[inline] pub fn set_tcc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TC2 APB Protect Enable"]
    #[inline] pub fn tc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TC2 != 0"]
    #[inline] pub fn test_tc2(&self) -> bool {
        self.tc2() != 0
    }

    #[doc="Sets the TC2 field."]
    #[inline] pub fn set_tc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TC3 APB Protect Enable"]
    #[inline] pub fn tc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TC3 != 0"]
    #[inline] pub fn test_tc3(&self) -> bool {
        self.tc3() != 0
    }

    #[doc="Sets the TC3 field."]
    #[inline] pub fn set_tc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="RAMECC APB Protect Enable"]
    #[inline] pub fn ramecc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RAMECC != 0"]
    #[inline] pub fn test_ramecc(&self) -> bool {
        self.ramecc() != 0
    }

    #[doc="Sets the RAMECC field."]
    #[inline] pub fn set_ramecc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.usb() != 0 { try!(write!(f, " usb"))}
        if self.dsu() != 0 { try!(write!(f, " dsu"))}
        if self.nvmctrl() != 0 { try!(write!(f, " nvmctrl"))}
        if self.cmcc() != 0 { try!(write!(f, " cmcc"))}
        if self.port() != 0 { try!(write!(f, " port"))}
        if self.dmac() != 0 { try!(write!(f, " dmac"))}
        if self.hmatrix() != 0 { try!(write!(f, " hmatrix"))}
        if self.evsys() != 0 { try!(write!(f, " evsys"))}
        if self.sercom2() != 0 { try!(write!(f, " sercom2"))}
        if self.sercom3() != 0 { try!(write!(f, " sercom3"))}
        if self.tcc0() != 0 { try!(write!(f, " tcc0"))}
        if self.tcc1() != 0 { try!(write!(f, " tcc1"))}
        if self.tc2() != 0 { try!(write!(f, " tc2"))}
        if self.tc3() != 0 { try!(write!(f, " tc3"))}
        if self.ramecc() != 0 { try!(write!(f, " ramecc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral write protection status - Bridge C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusc(pub u32);
impl Statusc {
    #[doc="TCC2 APB Protect Enable"]
    #[inline] pub fn tcc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TCC2 != 0"]
    #[inline] pub fn test_tcc2(&self) -> bool {
        self.tcc2() != 0
    }

    #[doc="Sets the TCC2 field."]
    #[inline] pub fn set_tcc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TCC3 APB Protect Enable"]
    #[inline] pub fn tcc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCC3 != 0"]
    #[inline] pub fn test_tcc3(&self) -> bool {
        self.tcc3() != 0
    }

    #[doc="Sets the TCC3 field."]
    #[inline] pub fn set_tcc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TC4 APB Protect Enable"]
    #[inline] pub fn tc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TC4 != 0"]
    #[inline] pub fn test_tc4(&self) -> bool {
        self.tc4() != 0
    }

    #[doc="Sets the TC4 field."]
    #[inline] pub fn set_tc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TC5 APB Protect Enable"]
    #[inline] pub fn tc5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC5 != 0"]
    #[inline] pub fn test_tc5(&self) -> bool {
        self.tc5() != 0
    }

    #[doc="Sets the TC5 field."]
    #[inline] pub fn set_tc5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PDEC APB Protect Enable"]
    #[inline] pub fn pdec(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PDEC != 0"]
    #[inline] pub fn test_pdec(&self) -> bool {
        self.pdec() != 0
    }

    #[doc="Sets the PDEC field."]
    #[inline] pub fn set_pdec<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="AC APB Protect Enable"]
    #[inline] pub fn ac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if AC != 0"]
    #[inline] pub fn test_ac(&self) -> bool {
        self.ac() != 0
    }

    #[doc="Sets the AC field."]
    #[inline] pub fn set_ac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AES APB Protect Enable"]
    #[inline] pub fn aes(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if AES != 0"]
    #[inline] pub fn test_aes(&self) -> bool {
        self.aes() != 0
    }

    #[doc="Sets the AES field."]
    #[inline] pub fn set_aes<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TRNG APB Protect Enable"]
    #[inline] pub fn trng(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TRNG != 0"]
    #[inline] pub fn test_trng(&self) -> bool {
        self.trng() != 0
    }

    #[doc="Sets the TRNG field."]
    #[inline] pub fn set_trng<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ICM APB Protect Enable"]
    #[inline] pub fn icm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICM != 0"]
    #[inline] pub fn test_icm(&self) -> bool {
        self.icm() != 0
    }

    #[doc="Sets the ICM field."]
    #[inline] pub fn set_icm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PUKCC APB Protect Enable"]
    #[inline] pub fn pukcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PUKCC != 0"]
    #[inline] pub fn test_pukcc(&self) -> bool {
        self.pukcc() != 0
    }

    #[doc="Sets the PUKCC field."]
    #[inline] pub fn set_pukcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="QSPI APB Protect Enable"]
    #[inline] pub fn qspi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if QSPI != 0"]
    #[inline] pub fn test_qspi(&self) -> bool {
        self.qspi() != 0
    }

    #[doc="Sets the QSPI field."]
    #[inline] pub fn set_qspi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CCL APB Protect Enable"]
    #[inline] pub fn ccl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CCL != 0"]
    #[inline] pub fn test_ccl(&self) -> bool {
        self.ccl() != 0
    }

    #[doc="Sets the CCL field."]
    #[inline] pub fn set_ccl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.tcc2() != 0 { try!(write!(f, " tcc2"))}
        if self.tcc3() != 0 { try!(write!(f, " tcc3"))}
        if self.tc4() != 0 { try!(write!(f, " tc4"))}
        if self.tc5() != 0 { try!(write!(f, " tc5"))}
        if self.pdec() != 0 { try!(write!(f, " pdec"))}
        if self.ac() != 0 { try!(write!(f, " ac"))}
        if self.aes() != 0 { try!(write!(f, " aes"))}
        if self.trng() != 0 { try!(write!(f, " trng"))}
        if self.icm() != 0 { try!(write!(f, " icm"))}
        if self.pukcc() != 0 { try!(write!(f, " pukcc"))}
        if self.qspi() != 0 { try!(write!(f, " qspi"))}
        if self.ccl() != 0 { try!(write!(f, " ccl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral write protection status - Bridge D"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusd(pub u32);
impl Statusd {
    #[doc="SERCOM4 APB Protect Enable"]
    #[inline] pub fn sercom4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SERCOM4 != 0"]
    #[inline] pub fn test_sercom4(&self) -> bool {
        self.sercom4() != 0
    }

    #[doc="Sets the SERCOM4 field."]
    #[inline] pub fn set_sercom4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SERCOM5 APB Protect Enable"]
    #[inline] pub fn sercom5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SERCOM5 != 0"]
    #[inline] pub fn test_sercom5(&self) -> bool {
        self.sercom5() != 0
    }

    #[doc="Sets the SERCOM5 field."]
    #[inline] pub fn set_sercom5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TCC4 APB Protect Enable"]
    #[inline] pub fn tcc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCC4 != 0"]
    #[inline] pub fn test_tcc4(&self) -> bool {
        self.tcc4() != 0
    }

    #[doc="Sets the TCC4 field."]
    #[inline] pub fn set_tcc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADC0 APB Protect Enable"]
    #[inline] pub fn adc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADC0 != 0"]
    #[inline] pub fn test_adc0(&self) -> bool {
        self.adc0() != 0
    }

    #[doc="Sets the ADC0 field."]
    #[inline] pub fn set_adc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ADC1 APB Protect Enable"]
    #[inline] pub fn adc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1 != 0"]
    #[inline] pub fn test_adc1(&self) -> bool {
        self.adc1() != 0
    }

    #[doc="Sets the ADC1 field."]
    #[inline] pub fn set_adc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DAC APB Protect Enable"]
    #[inline] pub fn dac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DAC != 0"]
    #[inline] pub fn test_dac(&self) -> bool {
        self.dac() != 0
    }

    #[doc="Sets the DAC field."]
    #[inline] pub fn set_dac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="I2S APB Protect Enable"]
    #[inline] pub fn i2s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if I2S != 0"]
    #[inline] pub fn test_i2s(&self) -> bool {
        self.i2s() != 0
    }

    #[doc="Sets the I2S field."]
    #[inline] pub fn set_i2s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="PCC APB Protect Enable"]
    #[inline] pub fn pcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PCC != 0"]
    #[inline] pub fn test_pcc(&self) -> bool {
        self.pcc() != 0
    }

    #[doc="Sets the PCC field."]
    #[inline] pub fn set_pcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.sercom4() != 0 { try!(write!(f, " sercom4"))}
        if self.sercom5() != 0 { try!(write!(f, " sercom5"))}
        if self.tcc4() != 0 { try!(write!(f, " tcc4"))}
        if self.adc0() != 0 { try!(write!(f, " adc0"))}
        if self.adc1() != 0 { try!(write!(f, " adc1"))}
        if self.dac() != 0 { try!(write!(f, " dac"))}
        if self.i2s() != 0 { try!(write!(f, " i2s"))}
        if self.pcc() != 0 { try!(write!(f, " pcc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

