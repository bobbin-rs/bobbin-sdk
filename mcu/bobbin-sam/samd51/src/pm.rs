::bobbin_mcu::periph!( PM, Pm, PM_PERIPH, PmPeriph, PM_OWNED, PM_REF_COUNT, 0x40000400, 0x00, 0x17);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PM Peripheral"]
pub struct PmPeriph(pub usize); 

impl PmPeriph {
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

    #[doc="Get the SLEEPCFG Register."]
    #[inline] pub fn sleepcfg_reg(&self) -> ::bobbin_mcu::register::Register<Sleepcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sleepcfg, 0x1)
    }

    #[doc="Get the *mut pointer for the SLEEPCFG register."]
    #[inline] pub fn sleepcfg_mut(&self) -> *mut Sleepcfg { 
        self.sleepcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the SLEEPCFG register."]
    #[inline] pub fn sleepcfg_ptr(&self) -> *const Sleepcfg { 
        self.sleepcfg_reg().ptr()
    }

    #[doc="Read the SLEEPCFG register."]
    #[inline] pub fn sleepcfg(&self) -> Sleepcfg { 
        self.sleepcfg_reg().read()
    }

    #[doc="Write the SLEEPCFG register."]
    #[inline] pub fn write_sleepcfg(&self, value: Sleepcfg) -> &Self { 
        self.sleepcfg_reg().write(value);
        self
    }

    #[doc="Set the SLEEPCFG register."]
    #[inline] pub fn set_sleepcfg<F: FnOnce(Sleepcfg) -> Sleepcfg>(&self, f: F) -> &Self {
        self.sleepcfg_reg().set(f);
        self
    }

    #[doc="Modify the SLEEPCFG register."]
    #[inline] pub fn with_sleepcfg<F: FnOnce(Sleepcfg) -> Sleepcfg>(&self, f: F) -> &Self {
        self.sleepcfg_reg().with(f);
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

    #[doc="Get the STDBYCFG Register."]
    #[inline] pub fn stdbycfg_reg(&self) -> ::bobbin_mcu::register::Register<Stdbycfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Stdbycfg, 0x8)
    }

    #[doc="Get the *mut pointer for the STDBYCFG register."]
    #[inline] pub fn stdbycfg_mut(&self) -> *mut Stdbycfg { 
        self.stdbycfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the STDBYCFG register."]
    #[inline] pub fn stdbycfg_ptr(&self) -> *const Stdbycfg { 
        self.stdbycfg_reg().ptr()
    }

    #[doc="Read the STDBYCFG register."]
    #[inline] pub fn stdbycfg(&self) -> Stdbycfg { 
        self.stdbycfg_reg().read()
    }

    #[doc="Write the STDBYCFG register."]
    #[inline] pub fn write_stdbycfg(&self, value: Stdbycfg) -> &Self { 
        self.stdbycfg_reg().write(value);
        self
    }

    #[doc="Set the STDBYCFG register."]
    #[inline] pub fn set_stdbycfg<F: FnOnce(Stdbycfg) -> Stdbycfg>(&self, f: F) -> &Self {
        self.stdbycfg_reg().set(f);
        self
    }

    #[doc="Modify the STDBYCFG register."]
    #[inline] pub fn with_stdbycfg<F: FnOnce(Stdbycfg) -> Stdbycfg>(&self, f: F) -> &Self {
        self.stdbycfg_reg().with(f);
        self
    }

    #[doc="Get the HIBCFG Register."]
    #[inline] pub fn hibcfg_reg(&self) -> ::bobbin_mcu::register::Register<Hibcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hibcfg, 0x9)
    }

    #[doc="Get the *mut pointer for the HIBCFG register."]
    #[inline] pub fn hibcfg_mut(&self) -> *mut Hibcfg { 
        self.hibcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the HIBCFG register."]
    #[inline] pub fn hibcfg_ptr(&self) -> *const Hibcfg { 
        self.hibcfg_reg().ptr()
    }

    #[doc="Read the HIBCFG register."]
    #[inline] pub fn hibcfg(&self) -> Hibcfg { 
        self.hibcfg_reg().read()
    }

    #[doc="Write the HIBCFG register."]
    #[inline] pub fn write_hibcfg(&self, value: Hibcfg) -> &Self { 
        self.hibcfg_reg().write(value);
        self
    }

    #[doc="Set the HIBCFG register."]
    #[inline] pub fn set_hibcfg<F: FnOnce(Hibcfg) -> Hibcfg>(&self, f: F) -> &Self {
        self.hibcfg_reg().set(f);
        self
    }

    #[doc="Modify the HIBCFG register."]
    #[inline] pub fn with_hibcfg<F: FnOnce(Hibcfg) -> Hibcfg>(&self, f: F) -> &Self {
        self.hibcfg_reg().with(f);
        self
    }

    #[doc="Get the BKUPCFG Register."]
    #[inline] pub fn bkupcfg_reg(&self) -> ::bobbin_mcu::register::Register<Bkupcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bkupcfg, 0xa)
    }

    #[doc="Get the *mut pointer for the BKUPCFG register."]
    #[inline] pub fn bkupcfg_mut(&self) -> *mut Bkupcfg { 
        self.bkupcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the BKUPCFG register."]
    #[inline] pub fn bkupcfg_ptr(&self) -> *const Bkupcfg { 
        self.bkupcfg_reg().ptr()
    }

    #[doc="Read the BKUPCFG register."]
    #[inline] pub fn bkupcfg(&self) -> Bkupcfg { 
        self.bkupcfg_reg().read()
    }

    #[doc="Write the BKUPCFG register."]
    #[inline] pub fn write_bkupcfg(&self, value: Bkupcfg) -> &Self { 
        self.bkupcfg_reg().write(value);
        self
    }

    #[doc="Set the BKUPCFG register."]
    #[inline] pub fn set_bkupcfg<F: FnOnce(Bkupcfg) -> Bkupcfg>(&self, f: F) -> &Self {
        self.bkupcfg_reg().set(f);
        self
    }

    #[doc="Modify the BKUPCFG register."]
    #[inline] pub fn with_bkupcfg<F: FnOnce(Bkupcfg) -> Bkupcfg>(&self, f: F) -> &Self {
        self.bkupcfg_reg().with(f);
        self
    }

    #[doc="Get the PWSAKDLY Register."]
    #[inline] pub fn pwsakdly_reg(&self) -> ::bobbin_mcu::register::Register<Pwsakdly> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pwsakdly, 0x12)
    }

    #[doc="Get the *mut pointer for the PWSAKDLY register."]
    #[inline] pub fn pwsakdly_mut(&self) -> *mut Pwsakdly { 
        self.pwsakdly_reg().ptr()
    }

    #[doc="Get the *const pointer for the PWSAKDLY register."]
    #[inline] pub fn pwsakdly_ptr(&self) -> *const Pwsakdly { 
        self.pwsakdly_reg().ptr()
    }

    #[doc="Read the PWSAKDLY register."]
    #[inline] pub fn pwsakdly(&self) -> Pwsakdly { 
        self.pwsakdly_reg().read()
    }

    #[doc="Write the PWSAKDLY register."]
    #[inline] pub fn write_pwsakdly(&self, value: Pwsakdly) -> &Self { 
        self.pwsakdly_reg().write(value);
        self
    }

    #[doc="Set the PWSAKDLY register."]
    #[inline] pub fn set_pwsakdly<F: FnOnce(Pwsakdly) -> Pwsakdly>(&self, f: F) -> &Self {
        self.pwsakdly_reg().set(f);
        self
    }

    #[doc="Modify the PWSAKDLY register."]
    #[inline] pub fn with_pwsakdly<F: FnOnce(Pwsakdly) -> Pwsakdly>(&self, f: F) -> &Self {
        self.pwsakdly_reg().with(f);
        self
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u8);
impl Ctrla {
    #[doc="I/O Retention"]
    #[inline] pub fn ioret(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IORET != 0"]
    #[inline] pub fn test_ioret(&self) -> bool {
        self.ioret() != 0
    }

    #[doc="Sets the IORET field."]
    #[inline] pub fn set_ioret<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
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
        if self.ioret() != 0 { try!(write!(f, " ioret"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sleep Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sleepcfg(pub u8);
impl Sleepcfg {
    #[doc="Sleep Mode"]
    #[inline] pub fn sleepmode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SLEEPMODE != 0"]
    #[inline] pub fn test_sleepmode(&self) -> bool {
        self.sleepmode() != 0
    }

    #[doc="Sets the SLEEPMODE field."]
    #[inline] pub fn set_sleepmode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Sleepcfg {
    #[inline]
    fn from(other: u8) -> Self {
         Sleepcfg(other)
    }
}

impl ::core::fmt::Display for Sleepcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sleepcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sleepmode() != 0 { try!(write!(f, " sleepmode=0x{:x}", self.sleepmode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Sleep Mode Entry Ready Enable"]
    #[inline] pub fn sleeprdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEEPRDY != 0"]
    #[inline] pub fn test_sleeprdy(&self) -> bool {
        self.sleeprdy() != 0
    }

    #[doc="Sets the SLEEPRDY field."]
    #[inline] pub fn set_sleeprdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.sleeprdy() != 0 { try!(write!(f, " sleeprdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Sleep Mode Entry Ready Enable"]
    #[inline] pub fn sleeprdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEEPRDY != 0"]
    #[inline] pub fn test_sleeprdy(&self) -> bool {
        self.sleeprdy() != 0
    }

    #[doc="Sets the SLEEPRDY field."]
    #[inline] pub fn set_sleeprdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.sleeprdy() != 0 { try!(write!(f, " sleeprdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Sleep Mode Entry Ready"]
    #[inline] pub fn sleeprdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEEPRDY != 0"]
    #[inline] pub fn test_sleeprdy(&self) -> bool {
        self.sleeprdy() != 0
    }

    #[doc="Sets the SLEEPRDY field."]
    #[inline] pub fn set_sleeprdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.sleeprdy() != 0 { try!(write!(f, " sleeprdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Standby Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stdbycfg(pub u8);
impl Stdbycfg {
    #[doc="Ram Configuration"]
    #[inline] pub fn ramcfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if RAMCFG != 0"]
    #[inline] pub fn test_ramcfg(&self) -> bool {
        self.ramcfg() != 0
    }

    #[doc="Sets the RAMCFG field."]
    #[inline] pub fn set_ramcfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fast Wakeup"]
    #[inline] pub fn fastwkup(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if FASTWKUP != 0"]
    #[inline] pub fn test_fastwkup(&self) -> bool {
        self.fastwkup() != 0
    }

    #[doc="Sets the FASTWKUP field."]
    #[inline] pub fn set_fastwkup<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Stdbycfg {
    #[inline]
    fn from(other: u8) -> Self {
         Stdbycfg(other)
    }
}

impl ::core::fmt::Display for Stdbycfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stdbycfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ramcfg() != 0 { try!(write!(f, " ramcfg=0x{:x}", self.ramcfg()))}
        if self.fastwkup() != 0 { try!(write!(f, " fastwkup=0x{:x}", self.fastwkup()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hibernate Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hibcfg(pub u8);
impl Hibcfg {
    #[doc="Ram Configuration"]
    #[inline] pub fn ramcfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if RAMCFG != 0"]
    #[inline] pub fn test_ramcfg(&self) -> bool {
        self.ramcfg() != 0
    }

    #[doc="Sets the RAMCFG field."]
    #[inline] pub fn set_ramcfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Backup Ram Configuration"]
    #[inline] pub fn bramcfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if BRAMCFG != 0"]
    #[inline] pub fn test_bramcfg(&self) -> bool {
        self.bramcfg() != 0
    }

    #[doc="Sets the BRAMCFG field."]
    #[inline] pub fn set_bramcfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Hibcfg {
    #[inline]
    fn from(other: u8) -> Self {
         Hibcfg(other)
    }
}

impl ::core::fmt::Display for Hibcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hibcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ramcfg() != 0 { try!(write!(f, " ramcfg=0x{:x}", self.ramcfg()))}
        if self.bramcfg() != 0 { try!(write!(f, " bramcfg=0x{:x}", self.bramcfg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bkupcfg(pub u8);
impl Bkupcfg {
    #[doc="Ram Configuration"]
    #[inline] pub fn bramcfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if BRAMCFG != 0"]
    #[inline] pub fn test_bramcfg(&self) -> bool {
        self.bramcfg() != 0
    }

    #[doc="Sets the BRAMCFG field."]
    #[inline] pub fn set_bramcfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Bkupcfg {
    #[inline]
    fn from(other: u8) -> Self {
         Bkupcfg(other)
    }
}

impl ::core::fmt::Display for Bkupcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bkupcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bramcfg() != 0 { try!(write!(f, " bramcfg=0x{:x}", self.bramcfg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Switch Acknowledge Delay"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pwsakdly(pub u8);
impl Pwsakdly {
    #[doc="Delay Value"]
    #[inline] pub fn dlyval(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if DLYVAL != 0"]
    #[inline] pub fn test_dlyval(&self) -> bool {
        self.dlyval() != 0
    }

    #[doc="Sets the DLYVAL field."]
    #[inline] pub fn set_dlyval<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ignore Acknowledge"]
    #[inline] pub fn ignack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IGNACK != 0"]
    #[inline] pub fn test_ignack(&self) -> bool {
        self.ignack() != 0
    }

    #[doc="Sets the IGNACK field."]
    #[inline] pub fn set_ignack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Pwsakdly {
    #[inline]
    fn from(other: u8) -> Self {
         Pwsakdly(other)
    }
}

impl ::core::fmt::Display for Pwsakdly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pwsakdly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dlyval() != 0 { try!(write!(f, " dlyval=0x{:x}", self.dlyval()))}
        if self.ignack() != 0 { try!(write!(f, " ignack"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

