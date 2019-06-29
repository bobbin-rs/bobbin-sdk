::bobbin_mcu::periph!( EIC, Eic, EIC_PERIPH, EicPeriph, EIC_OWNED, EIC_REF_COUNT, 0x40002800, 0x00, 0x09);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="EIC Peripheral"]
pub struct EicPeriph(pub usize); 

impl EicPeriph {
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

    #[doc="Get the NMICTRL Register."]
    #[inline] pub fn nmictrl_reg(&self) -> ::bobbin_mcu::register::Register<Nmictrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Nmictrl, 0x1)
    }

    #[doc="Get the *mut pointer for the NMICTRL register."]
    #[inline] pub fn nmictrl_mut(&self) -> *mut Nmictrl { 
        self.nmictrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the NMICTRL register."]
    #[inline] pub fn nmictrl_ptr(&self) -> *const Nmictrl { 
        self.nmictrl_reg().ptr()
    }

    #[doc="Read the NMICTRL register."]
    #[inline] pub fn nmictrl(&self) -> Nmictrl { 
        self.nmictrl_reg().read()
    }

    #[doc="Write the NMICTRL register."]
    #[inline] pub fn write_nmictrl(&self, value: Nmictrl) -> &Self { 
        self.nmictrl_reg().write(value);
        self
    }

    #[doc="Set the NMICTRL register."]
    #[inline] pub fn set_nmictrl<F: FnOnce(Nmictrl) -> Nmictrl>(&self, f: F) -> &Self {
        self.nmictrl_reg().set(f);
        self
    }

    #[doc="Modify the NMICTRL register."]
    #[inline] pub fn with_nmictrl<F: FnOnce(Nmictrl) -> Nmictrl>(&self, f: F) -> &Self {
        self.nmictrl_reg().with(f);
        self
    }

    #[doc="Get the NMIFLAG Register."]
    #[inline] pub fn nmiflag_reg(&self) -> ::bobbin_mcu::register::Register<Nmiflag> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Nmiflag, 0x2)
    }

    #[doc="Get the *mut pointer for the NMIFLAG register."]
    #[inline] pub fn nmiflag_mut(&self) -> *mut Nmiflag { 
        self.nmiflag_reg().ptr()
    }

    #[doc="Get the *const pointer for the NMIFLAG register."]
    #[inline] pub fn nmiflag_ptr(&self) -> *const Nmiflag { 
        self.nmiflag_reg().ptr()
    }

    #[doc="Read the NMIFLAG register."]
    #[inline] pub fn nmiflag(&self) -> Nmiflag { 
        self.nmiflag_reg().read()
    }

    #[doc="Write the NMIFLAG register."]
    #[inline] pub fn write_nmiflag(&self, value: Nmiflag) -> &Self { 
        self.nmiflag_reg().write(value);
        self
    }

    #[doc="Set the NMIFLAG register."]
    #[inline] pub fn set_nmiflag<F: FnOnce(Nmiflag) -> Nmiflag>(&self, f: F) -> &Self {
        self.nmiflag_reg().set(f);
        self
    }

    #[doc="Modify the NMIFLAG register."]
    #[inline] pub fn with_nmiflag<F: FnOnce(Nmiflag) -> Nmiflag>(&self, f: F) -> &Self {
        self.nmiflag_reg().with(f);
        self
    }

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x4)
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

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x8)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0xc)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x10)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x14)
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

    #[doc="Get the ASYNCH Register."]
    #[inline] pub fn asynch_reg(&self) -> ::bobbin_mcu::register::Register<Asynch> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Asynch, 0x18)
    }

    #[doc="Get the *mut pointer for the ASYNCH register."]
    #[inline] pub fn asynch_mut(&self) -> *mut Asynch { 
        self.asynch_reg().ptr()
    }

    #[doc="Get the *const pointer for the ASYNCH register."]
    #[inline] pub fn asynch_ptr(&self) -> *const Asynch { 
        self.asynch_reg().ptr()
    }

    #[doc="Read the ASYNCH register."]
    #[inline] pub fn asynch(&self) -> Asynch { 
        self.asynch_reg().read()
    }

    #[doc="Write the ASYNCH register."]
    #[inline] pub fn write_asynch(&self, value: Asynch) -> &Self { 
        self.asynch_reg().write(value);
        self
    }

    #[doc="Set the ASYNCH register."]
    #[inline] pub fn set_asynch<F: FnOnce(Asynch) -> Asynch>(&self, f: F) -> &Self {
        self.asynch_reg().set(f);
        self
    }

    #[doc="Modify the ASYNCH register."]
    #[inline] pub fn with_asynch<F: FnOnce(Asynch) -> Asynch>(&self, f: F) -> &Self {
        self.asynch_reg().with(f);
        self
    }

    #[doc="Get the CONFIG Register."]
    #[inline] pub fn config_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Config, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Config, 0x1c, 0x4)
    }

    #[doc="Get the *mut pointer for the CONFIG register."]
    #[inline] pub fn config_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Config { 
        self.config_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CONFIG register."]
    #[inline] pub fn config_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Config { 
        self.config_reg().ptr(index.into())
    }

    #[doc="Read the CONFIG register."]
    #[inline] pub fn config<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Config { 
        self.config_reg().read(index.into())
    }

    #[doc="Write the CONFIG register."]
    #[inline] pub fn write_config<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Config) -> &Self {
        self.config_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CONFIG register."]
    #[inline] pub fn set_config<I: Into<::bobbin_bits::R2>, F: FnOnce(Config) -> Config>(&self, index: I, f: F) -> &Self {
        self.config_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CONFIG register."]
    #[inline] pub fn with_config<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Config) -> Config>(&self, index: I, f: F) -> &Self {
        self.config_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DEBOUNCEN Register."]
    #[inline] pub fn debouncen_reg(&self) -> ::bobbin_mcu::register::Register<Debouncen> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Debouncen, 0x30)
    }

    #[doc="Get the *mut pointer for the DEBOUNCEN register."]
    #[inline] pub fn debouncen_mut(&self) -> *mut Debouncen { 
        self.debouncen_reg().ptr()
    }

    #[doc="Get the *const pointer for the DEBOUNCEN register."]
    #[inline] pub fn debouncen_ptr(&self) -> *const Debouncen { 
        self.debouncen_reg().ptr()
    }

    #[doc="Read the DEBOUNCEN register."]
    #[inline] pub fn debouncen(&self) -> Debouncen { 
        self.debouncen_reg().read()
    }

    #[doc="Write the DEBOUNCEN register."]
    #[inline] pub fn write_debouncen(&self, value: Debouncen) -> &Self { 
        self.debouncen_reg().write(value);
        self
    }

    #[doc="Set the DEBOUNCEN register."]
    #[inline] pub fn set_debouncen<F: FnOnce(Debouncen) -> Debouncen>(&self, f: F) -> &Self {
        self.debouncen_reg().set(f);
        self
    }

    #[doc="Modify the DEBOUNCEN register."]
    #[inline] pub fn with_debouncen<F: FnOnce(Debouncen) -> Debouncen>(&self, f: F) -> &Self {
        self.debouncen_reg().with(f);
        self
    }

    #[doc="Get the DPRESCALER Register."]
    #[inline] pub fn dprescaler_reg(&self) -> ::bobbin_mcu::register::Register<Dprescaler> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dprescaler, 0x34)
    }

    #[doc="Get the *mut pointer for the DPRESCALER register."]
    #[inline] pub fn dprescaler_mut(&self) -> *mut Dprescaler { 
        self.dprescaler_reg().ptr()
    }

    #[doc="Get the *const pointer for the DPRESCALER register."]
    #[inline] pub fn dprescaler_ptr(&self) -> *const Dprescaler { 
        self.dprescaler_reg().ptr()
    }

    #[doc="Read the DPRESCALER register."]
    #[inline] pub fn dprescaler(&self) -> Dprescaler { 
        self.dprescaler_reg().read()
    }

    #[doc="Write the DPRESCALER register."]
    #[inline] pub fn write_dprescaler(&self, value: Dprescaler) -> &Self { 
        self.dprescaler_reg().write(value);
        self
    }

    #[doc="Set the DPRESCALER register."]
    #[inline] pub fn set_dprescaler<F: FnOnce(Dprescaler) -> Dprescaler>(&self, f: F) -> &Self {
        self.dprescaler_reg().set(f);
        self
    }

    #[doc="Modify the DPRESCALER register."]
    #[inline] pub fn with_dprescaler<F: FnOnce(Dprescaler) -> Dprescaler>(&self, f: F) -> &Self {
        self.dprescaler_reg().with(f);
        self
    }

    #[doc="Get the PINSTATE Register."]
    #[inline] pub fn pinstate_reg(&self) -> ::bobbin_mcu::register::Register<Pinstate> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pinstate, 0x38)
    }

    #[doc="Get the *mut pointer for the PINSTATE register."]
    #[inline] pub fn pinstate_mut(&self) -> *mut Pinstate { 
        self.pinstate_reg().ptr()
    }

    #[doc="Get the *const pointer for the PINSTATE register."]
    #[inline] pub fn pinstate_ptr(&self) -> *const Pinstate { 
        self.pinstate_reg().ptr()
    }

    #[doc="Read the PINSTATE register."]
    #[inline] pub fn pinstate(&self) -> Pinstate { 
        self.pinstate_reg().read()
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u8);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

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

    #[doc="Clock Selection"]
    #[inline] pub fn cksel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CKSEL != 0"]
    #[inline] pub fn test_cksel(&self) -> bool {
        self.cksel() != 0
    }

    #[doc="Sets the CKSEL field."]
    #[inline] pub fn set_cksel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
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
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.cksel() != 0 { try!(write!(f, " cksel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Non-Maskable Interrupt Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nmictrl(pub u8);
impl Nmictrl {
    #[doc="Non-Maskable Interrupt Sense Configuration"]
    #[inline] pub fn nmisense(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if NMISENSE != 0"]
    #[inline] pub fn test_nmisense(&self) -> bool {
        self.nmisense() != 0
    }

    #[doc="Sets the NMISENSE field."]
    #[inline] pub fn set_nmisense<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Non-Maskable Interrupt Filter Enable"]
    #[inline] pub fn nmifilten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if NMIFILTEN != 0"]
    #[inline] pub fn test_nmifilten(&self) -> bool {
        self.nmifilten() != 0
    }

    #[doc="Sets the NMIFILTEN field."]
    #[inline] pub fn set_nmifilten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Asynchronous Edge Detection Mode"]
    #[inline] pub fn nmiasynch(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NMIASYNCH != 0"]
    #[inline] pub fn test_nmiasynch(&self) -> bool {
        self.nmiasynch() != 0
    }

    #[doc="Sets the NMIASYNCH field."]
    #[inline] pub fn set_nmiasynch<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Nmictrl {
    #[inline]
    fn from(other: u8) -> Self {
         Nmictrl(other)
    }
}

impl ::core::fmt::Display for Nmictrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nmictrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nmisense() != 0 { try!(write!(f, " nmisense=0x{:x}", self.nmisense()))}
        if self.nmifilten() != 0 { try!(write!(f, " nmifilten"))}
        if self.nmiasynch() != 0 { try!(write!(f, " nmiasynch"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Non-Maskable Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nmiflag(pub u16);
impl Nmiflag {
    #[doc="Non-Maskable Interrupt"]
    #[inline] pub fn nmi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if NMI != 0"]
    #[inline] pub fn test_nmi(&self) -> bool {
        self.nmi() != 0
    }

    #[doc="Sets the NMI field."]
    #[inline] pub fn set_nmi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Nmiflag {
    #[inline]
    fn from(other: u16) -> Self {
         Nmiflag(other)
    }
}

impl ::core::fmt::Display for Nmiflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nmiflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nmi() != 0 { try!(write!(f, " nmi"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Synchronization Busy Status"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable Synchronization Busy Status"]
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
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u32);
impl Evctrl {
    #[doc="External Interrupt Event Output Enable"]
    #[inline] pub fn extinteo(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXTINTEO != 0"]
    #[inline] pub fn test_extinteo(&self) -> bool {
        self.extinteo() != 0
    }

    #[doc="Sets the EXTINTEO field."]
    #[inline] pub fn set_extinteo<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Evctrl {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.extinteo() != 0 { try!(write!(f, " extinteo=0x{:x}", self.extinteo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc="External Interrupt Enable"]
    #[inline] pub fn extint(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXTINT != 0"]
    #[inline] pub fn test_extint(&self) -> bool {
        self.extint() != 0
    }

    #[doc="Sets the EXTINT field."]
    #[inline] pub fn set_extint<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intenclr {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.extint() != 0 { try!(write!(f, " extint=0x{:x}", self.extint()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc="External Interrupt Enable"]
    #[inline] pub fn extint(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXTINT != 0"]
    #[inline] pub fn test_extint(&self) -> bool {
        self.extint() != 0
    }

    #[doc="Sets the EXTINT field."]
    #[inline] pub fn set_extint<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intenset {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.extint() != 0 { try!(write!(f, " extint=0x{:x}", self.extint()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
    #[doc="External Interrupt"]
    #[inline] pub fn extint(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXTINT != 0"]
    #[inline] pub fn test_extint(&self) -> bool {
        self.extint() != 0
    }

    #[doc="Sets the EXTINT field."]
    #[inline] pub fn set_extint<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intflag {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.extint() != 0 { try!(write!(f, " extint=0x{:x}", self.extint()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Interrupt Asynchronous Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Asynch(pub u32);
impl Asynch {
    #[doc="Asynchronous Edge Detection Mode"]
    #[inline] pub fn asynch(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if ASYNCH != 0"]
    #[inline] pub fn test_asynch(&self) -> bool {
        self.asynch() != 0
    }

    #[doc="Sets the ASYNCH field."]
    #[inline] pub fn set_asynch<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Asynch {
    #[inline]
    fn from(other: u32) -> Self {
         Asynch(other)
    }
}

impl ::core::fmt::Display for Asynch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Asynch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.asynch() != 0 { try!(write!(f, " asynch=0x{:x}", self.asynch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Interrupt Sense Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Config(pub u32);
impl Config {
    #[doc="Input Sense Configuration 0"]
    #[inline] pub fn sense0(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SENSE0 != 0"]
    #[inline] pub fn test_sense0(&self) -> bool {
        self.sense0() != 0
    }

    #[doc="Sets the SENSE0 field."]
    #[inline] pub fn set_sense0<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Filter Enable 0"]
    #[inline] pub fn filten0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FILTEN0 != 0"]
    #[inline] pub fn test_filten0(&self) -> bool {
        self.filten0() != 0
    }

    #[doc="Sets the FILTEN0 field."]
    #[inline] pub fn set_filten0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Input Sense Configuration 1"]
    #[inline] pub fn sense1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if SENSE1 != 0"]
    #[inline] pub fn test_sense1(&self) -> bool {
        self.sense1() != 0
    }

    #[doc="Sets the SENSE1 field."]
    #[inline] pub fn set_sense1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Filter Enable 1"]
    #[inline] pub fn filten1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FILTEN1 != 0"]
    #[inline] pub fn test_filten1(&self) -> bool {
        self.filten1() != 0
    }

    #[doc="Sets the FILTEN1 field."]
    #[inline] pub fn set_filten1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Input Sense Configuration 2"]
    #[inline] pub fn sense2(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if SENSE2 != 0"]
    #[inline] pub fn test_sense2(&self) -> bool {
        self.sense2() != 0
    }

    #[doc="Sets the SENSE2 field."]
    #[inline] pub fn set_sense2<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Filter Enable 2"]
    #[inline] pub fn filten2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FILTEN2 != 0"]
    #[inline] pub fn test_filten2(&self) -> bool {
        self.filten2() != 0
    }

    #[doc="Sets the FILTEN2 field."]
    #[inline] pub fn set_filten2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Input Sense Configuration 3"]
    #[inline] pub fn sense3(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if SENSE3 != 0"]
    #[inline] pub fn test_sense3(&self) -> bool {
        self.sense3() != 0
    }

    #[doc="Sets the SENSE3 field."]
    #[inline] pub fn set_sense3<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Filter Enable 3"]
    #[inline] pub fn filten3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FILTEN3 != 0"]
    #[inline] pub fn test_filten3(&self) -> bool {
        self.filten3() != 0
    }

    #[doc="Sets the FILTEN3 field."]
    #[inline] pub fn set_filten3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Input Sense Configuration 4"]
    #[inline] pub fn sense4(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if SENSE4 != 0"]
    #[inline] pub fn test_sense4(&self) -> bool {
        self.sense4() != 0
    }

    #[doc="Sets the SENSE4 field."]
    #[inline] pub fn set_sense4<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Filter Enable 4"]
    #[inline] pub fn filten4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if FILTEN4 != 0"]
    #[inline] pub fn test_filten4(&self) -> bool {
        self.filten4() != 0
    }

    #[doc="Sets the FILTEN4 field."]
    #[inline] pub fn set_filten4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Input Sense Configuration 5"]
    #[inline] pub fn sense5(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if SENSE5 != 0"]
    #[inline] pub fn test_sense5(&self) -> bool {
        self.sense5() != 0
    }

    #[doc="Sets the SENSE5 field."]
    #[inline] pub fn set_sense5<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Filter Enable 5"]
    #[inline] pub fn filten5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if FILTEN5 != 0"]
    #[inline] pub fn test_filten5(&self) -> bool {
        self.filten5() != 0
    }

    #[doc="Sets the FILTEN5 field."]
    #[inline] pub fn set_filten5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Input Sense Configuration 6"]
    #[inline] pub fn sense6(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if SENSE6 != 0"]
    #[inline] pub fn test_sense6(&self) -> bool {
        self.sense6() != 0
    }

    #[doc="Sets the SENSE6 field."]
    #[inline] pub fn set_sense6<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Filter Enable 6"]
    #[inline] pub fn filten6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FILTEN6 != 0"]
    #[inline] pub fn test_filten6(&self) -> bool {
        self.filten6() != 0
    }

    #[doc="Sets the FILTEN6 field."]
    #[inline] pub fn set_filten6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Input Sense Configuration 7"]
    #[inline] pub fn sense7(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if SENSE7 != 0"]
    #[inline] pub fn test_sense7(&self) -> bool {
        self.sense7() != 0
    }

    #[doc="Sets the SENSE7 field."]
    #[inline] pub fn set_sense7<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Filter Enable 7"]
    #[inline] pub fn filten7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if FILTEN7 != 0"]
    #[inline] pub fn test_filten7(&self) -> bool {
        self.filten7() != 0
    }

    #[doc="Sets the FILTEN7 field."]
    #[inline] pub fn set_filten7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Config {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.sense0() != 0 { try!(write!(f, " sense0=0x{:x}", self.sense0()))}
        if self.filten0() != 0 { try!(write!(f, " filten0"))}
        if self.sense1() != 0 { try!(write!(f, " sense1=0x{:x}", self.sense1()))}
        if self.filten1() != 0 { try!(write!(f, " filten1"))}
        if self.sense2() != 0 { try!(write!(f, " sense2=0x{:x}", self.sense2()))}
        if self.filten2() != 0 { try!(write!(f, " filten2"))}
        if self.sense3() != 0 { try!(write!(f, " sense3=0x{:x}", self.sense3()))}
        if self.filten3() != 0 { try!(write!(f, " filten3"))}
        if self.sense4() != 0 { try!(write!(f, " sense4=0x{:x}", self.sense4()))}
        if self.filten4() != 0 { try!(write!(f, " filten4"))}
        if self.sense5() != 0 { try!(write!(f, " sense5=0x{:x}", self.sense5()))}
        if self.filten5() != 0 { try!(write!(f, " filten5"))}
        if self.sense6() != 0 { try!(write!(f, " sense6=0x{:x}", self.sense6()))}
        if self.filten6() != 0 { try!(write!(f, " filten6"))}
        if self.sense7() != 0 { try!(write!(f, " sense7=0x{:x}", self.sense7()))}
        if self.filten7() != 0 { try!(write!(f, " filten7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debouncer Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Debouncen(pub u32);
impl Debouncen {
    #[doc="Debouncer Enable"]
    #[inline] pub fn debouncen(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DEBOUNCEN != 0"]
    #[inline] pub fn test_debouncen(&self) -> bool {
        self.debouncen() != 0
    }

    #[doc="Sets the DEBOUNCEN field."]
    #[inline] pub fn set_debouncen<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Debouncen {
    #[inline]
    fn from(other: u32) -> Self {
         Debouncen(other)
    }
}

impl ::core::fmt::Display for Debouncen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Debouncen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.debouncen() != 0 { try!(write!(f, " debouncen=0x{:x}", self.debouncen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debouncer Prescaler"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dprescaler(pub u32);
impl Dprescaler {
    #[doc="Debouncer Prescaler"]
    #[inline] pub fn prescaler0(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PRESCALER0 != 0"]
    #[inline] pub fn test_prescaler0(&self) -> bool {
        self.prescaler0() != 0
    }

    #[doc="Sets the PRESCALER0 field."]
    #[inline] pub fn set_prescaler0<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Debouncer Prescaler"]
    #[inline] pub fn prescaler1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if PRESCALER1 != 0"]
    #[inline] pub fn test_prescaler1(&self) -> bool {
        self.prescaler1() != 0
    }

    #[doc="Sets the PRESCALER1 field."]
    #[inline] pub fn set_prescaler1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Debouncer number of states"]
    #[inline] pub fn states0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STATES0 != 0"]
    #[inline] pub fn test_states0(&self) -> bool {
        self.states0() != 0
    }

    #[doc="Sets the STATES0 field."]
    #[inline] pub fn set_states0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Debouncer number of states"]
    #[inline] pub fn states1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if STATES1 != 0"]
    #[inline] pub fn test_states1(&self) -> bool {
        self.states1() != 0
    }

    #[doc="Sets the STATES1 field."]
    #[inline] pub fn set_states1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pin Sampler frequency selection"]
    #[inline] pub fn tickon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TICKON != 0"]
    #[inline] pub fn test_tickon(&self) -> bool {
        self.tickon() != 0
    }

    #[doc="Sets the TICKON field."]
    #[inline] pub fn set_tickon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dprescaler {
    #[inline]
    fn from(other: u32) -> Self {
         Dprescaler(other)
    }
}

impl ::core::fmt::Display for Dprescaler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dprescaler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prescaler0() != 0 { try!(write!(f, " prescaler0=0x{:x}", self.prescaler0()))}
        if self.prescaler1() != 0 { try!(write!(f, " prescaler1=0x{:x}", self.prescaler1()))}
        if self.states0() != 0 { try!(write!(f, " states0"))}
        if self.states1() != 0 { try!(write!(f, " states1"))}
        if self.tickon() != 0 { try!(write!(f, " tickon"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pin State"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pinstate(pub u32);
impl Pinstate {
    #[doc="Pin State"]
    #[inline] pub fn pinstate(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PINSTATE != 0"]
    #[inline] pub fn test_pinstate(&self) -> bool {
        self.pinstate() != 0
    }

    #[doc="Sets the PINSTATE field."]
    #[inline] pub fn set_pinstate<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pinstate {
    #[inline]
    fn from(other: u32) -> Self {
         Pinstate(other)
    }
}

impl ::core::fmt::Display for Pinstate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pinstate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pinstate() != 0 { try!(write!(f, " pinstate=0x{:x}", self.pinstate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

