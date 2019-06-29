::bobbin_mcu::periph!( MCLK, Mclk, MCLK_PERIPH, MclkPeriph, MCLK_OWNED, MCLK_REF_COUNT, 0x40000800, 0x00, 0x10);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="MCLK Peripheral"]
pub struct MclkPeriph(pub usize); 

impl MclkPeriph {
    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x1)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x2)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x3)
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

    #[doc="Get the HSDIV Register."]
    #[inline] pub fn hsdiv_reg(&self) -> ::bobbin_mcu::register::Register<Hsdiv> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hsdiv, 0x4)
    }

    #[doc="Get the *mut pointer for the HSDIV register."]
    #[inline] pub fn hsdiv_mut(&self) -> *mut Hsdiv { 
        self.hsdiv_reg().ptr()
    }

    #[doc="Get the *const pointer for the HSDIV register."]
    #[inline] pub fn hsdiv_ptr(&self) -> *const Hsdiv { 
        self.hsdiv_reg().ptr()
    }

    #[doc="Read the HSDIV register."]
    #[inline] pub fn hsdiv(&self) -> Hsdiv { 
        self.hsdiv_reg().read()
    }

    #[doc="Get the CPUDIV Register."]
    #[inline] pub fn cpudiv_reg(&self) -> ::bobbin_mcu::register::Register<Cpudiv> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cpudiv, 0x5)
    }

    #[doc="Get the *mut pointer for the CPUDIV register."]
    #[inline] pub fn cpudiv_mut(&self) -> *mut Cpudiv { 
        self.cpudiv_reg().ptr()
    }

    #[doc="Get the *const pointer for the CPUDIV register."]
    #[inline] pub fn cpudiv_ptr(&self) -> *const Cpudiv { 
        self.cpudiv_reg().ptr()
    }

    #[doc="Read the CPUDIV register."]
    #[inline] pub fn cpudiv(&self) -> Cpudiv { 
        self.cpudiv_reg().read()
    }

    #[doc="Write the CPUDIV register."]
    #[inline] pub fn write_cpudiv(&self, value: Cpudiv) -> &Self { 
        self.cpudiv_reg().write(value);
        self
    }

    #[doc="Set the CPUDIV register."]
    #[inline] pub fn set_cpudiv<F: FnOnce(Cpudiv) -> Cpudiv>(&self, f: F) -> &Self {
        self.cpudiv_reg().set(f);
        self
    }

    #[doc="Modify the CPUDIV register."]
    #[inline] pub fn with_cpudiv<F: FnOnce(Cpudiv) -> Cpudiv>(&self, f: F) -> &Self {
        self.cpudiv_reg().with(f);
        self
    }

    #[doc="Get the AHBMASK Register."]
    #[inline] pub fn ahbmask_reg(&self) -> ::bobbin_mcu::register::Register<Ahbmask> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahbmask, 0x10)
    }

    #[doc="Get the *mut pointer for the AHBMASK register."]
    #[inline] pub fn ahbmask_mut(&self) -> *mut Ahbmask { 
        self.ahbmask_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHBMASK register."]
    #[inline] pub fn ahbmask_ptr(&self) -> *const Ahbmask { 
        self.ahbmask_reg().ptr()
    }

    #[doc="Read the AHBMASK register."]
    #[inline] pub fn ahbmask(&self) -> Ahbmask { 
        self.ahbmask_reg().read()
    }

    #[doc="Write the AHBMASK register."]
    #[inline] pub fn write_ahbmask(&self, value: Ahbmask) -> &Self { 
        self.ahbmask_reg().write(value);
        self
    }

    #[doc="Set the AHBMASK register."]
    #[inline] pub fn set_ahbmask<F: FnOnce(Ahbmask) -> Ahbmask>(&self, f: F) -> &Self {
        self.ahbmask_reg().set(f);
        self
    }

    #[doc="Modify the AHBMASK register."]
    #[inline] pub fn with_ahbmask<F: FnOnce(Ahbmask) -> Ahbmask>(&self, f: F) -> &Self {
        self.ahbmask_reg().with(f);
        self
    }

    #[doc="Get the APBAMASK Register."]
    #[inline] pub fn apbamask_reg(&self) -> ::bobbin_mcu::register::Register<Apbamask> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apbamask, 0x14)
    }

    #[doc="Get the *mut pointer for the APBAMASK register."]
    #[inline] pub fn apbamask_mut(&self) -> *mut Apbamask { 
        self.apbamask_reg().ptr()
    }

    #[doc="Get the *const pointer for the APBAMASK register."]
    #[inline] pub fn apbamask_ptr(&self) -> *const Apbamask { 
        self.apbamask_reg().ptr()
    }

    #[doc="Read the APBAMASK register."]
    #[inline] pub fn apbamask(&self) -> Apbamask { 
        self.apbamask_reg().read()
    }

    #[doc="Write the APBAMASK register."]
    #[inline] pub fn write_apbamask(&self, value: Apbamask) -> &Self { 
        self.apbamask_reg().write(value);
        self
    }

    #[doc="Set the APBAMASK register."]
    #[inline] pub fn set_apbamask<F: FnOnce(Apbamask) -> Apbamask>(&self, f: F) -> &Self {
        self.apbamask_reg().set(f);
        self
    }

    #[doc="Modify the APBAMASK register."]
    #[inline] pub fn with_apbamask<F: FnOnce(Apbamask) -> Apbamask>(&self, f: F) -> &Self {
        self.apbamask_reg().with(f);
        self
    }

    #[doc="Get the APBBMASK Register."]
    #[inline] pub fn apbbmask_reg(&self) -> ::bobbin_mcu::register::Register<Apbbmask> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apbbmask, 0x18)
    }

    #[doc="Get the *mut pointer for the APBBMASK register."]
    #[inline] pub fn apbbmask_mut(&self) -> *mut Apbbmask { 
        self.apbbmask_reg().ptr()
    }

    #[doc="Get the *const pointer for the APBBMASK register."]
    #[inline] pub fn apbbmask_ptr(&self) -> *const Apbbmask { 
        self.apbbmask_reg().ptr()
    }

    #[doc="Read the APBBMASK register."]
    #[inline] pub fn apbbmask(&self) -> Apbbmask { 
        self.apbbmask_reg().read()
    }

    #[doc="Write the APBBMASK register."]
    #[inline] pub fn write_apbbmask(&self, value: Apbbmask) -> &Self { 
        self.apbbmask_reg().write(value);
        self
    }

    #[doc="Set the APBBMASK register."]
    #[inline] pub fn set_apbbmask<F: FnOnce(Apbbmask) -> Apbbmask>(&self, f: F) -> &Self {
        self.apbbmask_reg().set(f);
        self
    }

    #[doc="Modify the APBBMASK register."]
    #[inline] pub fn with_apbbmask<F: FnOnce(Apbbmask) -> Apbbmask>(&self, f: F) -> &Self {
        self.apbbmask_reg().with(f);
        self
    }

    #[doc="Get the APBCMASK Register."]
    #[inline] pub fn apbcmask_reg(&self) -> ::bobbin_mcu::register::Register<Apbcmask> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apbcmask, 0x1c)
    }

    #[doc="Get the *mut pointer for the APBCMASK register."]
    #[inline] pub fn apbcmask_mut(&self) -> *mut Apbcmask { 
        self.apbcmask_reg().ptr()
    }

    #[doc="Get the *const pointer for the APBCMASK register."]
    #[inline] pub fn apbcmask_ptr(&self) -> *const Apbcmask { 
        self.apbcmask_reg().ptr()
    }

    #[doc="Read the APBCMASK register."]
    #[inline] pub fn apbcmask(&self) -> Apbcmask { 
        self.apbcmask_reg().read()
    }

    #[doc="Write the APBCMASK register."]
    #[inline] pub fn write_apbcmask(&self, value: Apbcmask) -> &Self { 
        self.apbcmask_reg().write(value);
        self
    }

    #[doc="Set the APBCMASK register."]
    #[inline] pub fn set_apbcmask<F: FnOnce(Apbcmask) -> Apbcmask>(&self, f: F) -> &Self {
        self.apbcmask_reg().set(f);
        self
    }

    #[doc="Modify the APBCMASK register."]
    #[inline] pub fn with_apbcmask<F: FnOnce(Apbcmask) -> Apbcmask>(&self, f: F) -> &Self {
        self.apbcmask_reg().with(f);
        self
    }

    #[doc="Get the APBDMASK Register."]
    #[inline] pub fn apbdmask_reg(&self) -> ::bobbin_mcu::register::Register<Apbdmask> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apbdmask, 0x20)
    }

    #[doc="Get the *mut pointer for the APBDMASK register."]
    #[inline] pub fn apbdmask_mut(&self) -> *mut Apbdmask { 
        self.apbdmask_reg().ptr()
    }

    #[doc="Get the *const pointer for the APBDMASK register."]
    #[inline] pub fn apbdmask_ptr(&self) -> *const Apbdmask { 
        self.apbdmask_reg().ptr()
    }

    #[doc="Read the APBDMASK register."]
    #[inline] pub fn apbdmask(&self) -> Apbdmask { 
        self.apbdmask_reg().read()
    }

    #[doc="Write the APBDMASK register."]
    #[inline] pub fn write_apbdmask(&self, value: Apbdmask) -> &Self { 
        self.apbdmask_reg().write(value);
        self
    }

    #[doc="Set the APBDMASK register."]
    #[inline] pub fn set_apbdmask<F: FnOnce(Apbdmask) -> Apbdmask>(&self, f: F) -> &Self {
        self.apbdmask_reg().set(f);
        self
    }

    #[doc="Modify the APBDMASK register."]
    #[inline] pub fn with_apbdmask<F: FnOnce(Apbdmask) -> Apbdmask>(&self, f: F) -> &Self {
        self.apbdmask_reg().with(f);
        self
    }

}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Clock Ready Interrupt Enable"]
    #[inline] pub fn ckrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CKRDY != 0"]
    #[inline] pub fn test_ckrdy(&self) -> bool {
        self.ckrdy() != 0
    }

    #[doc="Sets the CKRDY field."]
    #[inline] pub fn set_ckrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.ckrdy() != 0 { try!(write!(f, " ckrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Clock Ready Interrupt Enable"]
    #[inline] pub fn ckrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CKRDY != 0"]
    #[inline] pub fn test_ckrdy(&self) -> bool {
        self.ckrdy() != 0
    }

    #[doc="Sets the CKRDY field."]
    #[inline] pub fn set_ckrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.ckrdy() != 0 { try!(write!(f, " ckrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Clock Ready"]
    #[inline] pub fn ckrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CKRDY != 0"]
    #[inline] pub fn test_ckrdy(&self) -> bool {
        self.ckrdy() != 0
    }

    #[doc="Sets the CKRDY field."]
    #[inline] pub fn set_ckrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.ckrdy() != 0 { try!(write!(f, " ckrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HS Clock Division"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hsdiv(pub u8);
impl Hsdiv {
    #[doc="CPU Clock Division Factor"]
    #[inline] pub fn div(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Hsdiv {
    #[inline]
    fn from(other: u8) -> Self {
         Hsdiv(other)
    }
}

impl ::core::fmt::Display for Hsdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hsdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU Clock Division"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpudiv(pub u8);
impl Cpudiv {
    #[doc="Low-Power Clock Division Factor"]
    #[inline] pub fn div(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Cpudiv {
    #[inline]
    fn from(other: u8) -> Self {
         Cpudiv(other)
    }
}

impl ::core::fmt::Display for Cpudiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cpudiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbmask(pub u32);
impl Ahbmask {
    #[doc="HPB0 AHB Clock Mask"]
    #[inline] pub fn hpb0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HPB0_ != 0"]
    #[inline] pub fn test_hpb0_(&self) -> bool {
        self.hpb0_() != 0
    }

    #[doc="Sets the HPB0_ field."]
    #[inline] pub fn set_hpb0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HPB1 AHB Clock Mask"]
    #[inline] pub fn hpb1_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HPB1_ != 0"]
    #[inline] pub fn test_hpb1_(&self) -> bool {
        self.hpb1_() != 0
    }

    #[doc="Sets the HPB1_ field."]
    #[inline] pub fn set_hpb1_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HPB2 AHB Clock Mask"]
    #[inline] pub fn hpb2_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HPB2_ != 0"]
    #[inline] pub fn test_hpb2_(&self) -> bool {
        self.hpb2_() != 0
    }

    #[doc="Sets the HPB2_ field."]
    #[inline] pub fn set_hpb2_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="HPB3 AHB Clock Mask"]
    #[inline] pub fn hpb3_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HPB3_ != 0"]
    #[inline] pub fn test_hpb3_(&self) -> bool {
        self.hpb3_() != 0
    }

    #[doc="Sets the HPB3_ field."]
    #[inline] pub fn set_hpb3_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DSU AHB Clock Mask"]
    #[inline] pub fn dsu_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DSU_ != 0"]
    #[inline] pub fn test_dsu_(&self) -> bool {
        self.dsu_() != 0
    }

    #[doc="Sets the DSU_ field."]
    #[inline] pub fn set_dsu_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HMATRIX AHB Clock Mask"]
    #[inline] pub fn hmatrix_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if HMATRIX_ != 0"]
    #[inline] pub fn test_hmatrix_(&self) -> bool {
        self.hmatrix_() != 0
    }

    #[doc="Sets the HMATRIX_ field."]
    #[inline] pub fn set_hmatrix_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="NVMCTRL AHB Clock Mask"]
    #[inline] pub fn nvmctrl_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NVMCTRL_ != 0"]
    #[inline] pub fn test_nvmctrl_(&self) -> bool {
        self.nvmctrl_() != 0
    }

    #[doc="Sets the NVMCTRL_ field."]
    #[inline] pub fn set_nvmctrl_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="HSRAM AHB Clock Mask"]
    #[inline] pub fn hsram_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if HSRAM_ != 0"]
    #[inline] pub fn test_hsram_(&self) -> bool {
        self.hsram_() != 0
    }

    #[doc="Sets the HSRAM_ field."]
    #[inline] pub fn set_hsram_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CMCC AHB Clock Mask"]
    #[inline] pub fn cmcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMCC_ != 0"]
    #[inline] pub fn test_cmcc_(&self) -> bool {
        self.cmcc_() != 0
    }

    #[doc="Sets the CMCC_ field."]
    #[inline] pub fn set_cmcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMAC AHB Clock Mask"]
    #[inline] pub fn dmac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DMAC_ != 0"]
    #[inline] pub fn test_dmac_(&self) -> bool {
        self.dmac_() != 0
    }

    #[doc="Sets the DMAC_ field."]
    #[inline] pub fn set_dmac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="USB AHB Clock Mask"]
    #[inline] pub fn usb_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if USB_ != 0"]
    #[inline] pub fn test_usb_(&self) -> bool {
        self.usb_() != 0
    }

    #[doc="Sets the USB_ field."]
    #[inline] pub fn set_usb_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BKUPRAM AHB Clock Mask"]
    #[inline] pub fn bkupram_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BKUPRAM_ != 0"]
    #[inline] pub fn test_bkupram_(&self) -> bool {
        self.bkupram_() != 0
    }

    #[doc="Sets the BKUPRAM_ field."]
    #[inline] pub fn set_bkupram_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PAC AHB Clock Mask"]
    #[inline] pub fn pac_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAC_ != 0"]
    #[inline] pub fn test_pac_(&self) -> bool {
        self.pac_() != 0
    }

    #[doc="Sets the PAC_ field."]
    #[inline] pub fn set_pac_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="QSPI AHB Clock Mask"]
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

    #[doc="SDHC0 AHB Clock Mask"]
    #[inline] pub fn sdhc0_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SDHC0_ != 0"]
    #[inline] pub fn test_sdhc0_(&self) -> bool {
        self.sdhc0_() != 0
    }

    #[doc="Sets the SDHC0_ field."]
    #[inline] pub fn set_sdhc0_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="ICM AHB Clock Mask"]
    #[inline] pub fn icm_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if ICM_ != 0"]
    #[inline] pub fn test_icm_(&self) -> bool {
        self.icm_() != 0
    }

    #[doc="Sets the ICM_ field."]
    #[inline] pub fn set_icm_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="PUKCC AHB Clock Mask"]
    #[inline] pub fn pukcc_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PUKCC_ != 0"]
    #[inline] pub fn test_pukcc_(&self) -> bool {
        self.pukcc_() != 0
    }

    #[doc="Sets the PUKCC_ field."]
    #[inline] pub fn set_pukcc_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="QSPI_2X AHB Clock Mask"]
    #[inline] pub fn qspi_2x_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if QSPI_2X_ != 0"]
    #[inline] pub fn test_qspi_2x_(&self) -> bool {
        self.qspi_2x_() != 0
    }

    #[doc="Sets the QSPI_2X_ field."]
    #[inline] pub fn set_qspi_2x_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="NVMCTRL_SMEEPROM AHB Clock Mask"]
    #[inline] pub fn nvmctrl_smeeprom_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if NVMCTRL_SMEEPROM_ != 0"]
    #[inline] pub fn test_nvmctrl_smeeprom_(&self) -> bool {
        self.nvmctrl_smeeprom_() != 0
    }

    #[doc="Sets the NVMCTRL_SMEEPROM_ field."]
    #[inline] pub fn set_nvmctrl_smeeprom_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="NVMCTRL_CACHE AHB Clock Mask"]
    #[inline] pub fn nvmctrl_cache_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if NVMCTRL_CACHE_ != 0"]
    #[inline] pub fn test_nvmctrl_cache_(&self) -> bool {
        self.nvmctrl_cache_() != 0
    }

    #[doc="Sets the NVMCTRL_CACHE_ field."]
    #[inline] pub fn set_nvmctrl_cache_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Ahbmask {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbmask(other)
    }
}

impl ::core::fmt::Display for Ahbmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahbmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hpb0_() != 0 { try!(write!(f, " hpb0_"))}
        if self.hpb1_() != 0 { try!(write!(f, " hpb1_"))}
        if self.hpb2_() != 0 { try!(write!(f, " hpb2_"))}
        if self.hpb3_() != 0 { try!(write!(f, " hpb3_"))}
        if self.dsu_() != 0 { try!(write!(f, " dsu_"))}
        if self.hmatrix_() != 0 { try!(write!(f, " hmatrix_"))}
        if self.nvmctrl_() != 0 { try!(write!(f, " nvmctrl_"))}
        if self.hsram_() != 0 { try!(write!(f, " hsram_"))}
        if self.cmcc_() != 0 { try!(write!(f, " cmcc_"))}
        if self.dmac_() != 0 { try!(write!(f, " dmac_"))}
        if self.usb_() != 0 { try!(write!(f, " usb_"))}
        if self.bkupram_() != 0 { try!(write!(f, " bkupram_"))}
        if self.pac_() != 0 { try!(write!(f, " pac_"))}
        if self.qspi_() != 0 { try!(write!(f, " qspi_"))}
        if self.sdhc0_() != 0 { try!(write!(f, " sdhc0_"))}
        if self.icm_() != 0 { try!(write!(f, " icm_"))}
        if self.pukcc_() != 0 { try!(write!(f, " pukcc_"))}
        if self.qspi_2x_() != 0 { try!(write!(f, " qspi_2x_"))}
        if self.nvmctrl_smeeprom_() != 0 { try!(write!(f, " nvmctrl_smeeprom_"))}
        if self.nvmctrl_cache_() != 0 { try!(write!(f, " nvmctrl_cache_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APBA Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbamask(pub u32);
impl Apbamask {
    #[doc="PAC APB Clock Enable"]
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

    #[doc="PM APB Clock Enable"]
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

    #[doc="MCLK APB Clock Enable"]
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

    #[doc="RSTC APB Clock Enable"]
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

    #[doc="OSCCTRL APB Clock Enable"]
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

    #[doc="OSC32KCTRL APB Clock Enable"]
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

    #[doc="SUPC APB Clock Enable"]
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

    #[doc="GCLK APB Clock Enable"]
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

    #[doc="WDT APB Clock Enable"]
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

    #[doc="RTC APB Clock Enable"]
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

    #[doc="EIC APB Clock Enable"]
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

    #[doc="FREQM APB Clock Enable"]
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

    #[doc="SERCOM0 APB Clock Enable"]
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

    #[doc="SERCOM1 APB Clock Enable"]
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

    #[doc="TC0 APB Clock Enable"]
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

    #[doc="TC1 APB Clock Enable"]
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

impl From<u32> for Apbamask {
    #[inline]
    fn from(other: u32) -> Self {
         Apbamask(other)
    }
}

impl ::core::fmt::Display for Apbamask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apbamask {
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

#[doc="APBB Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbbmask(pub u32);
impl Apbbmask {
    #[doc="USB APB Clock Enable"]
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

    #[doc="DSU APB Clock Enable"]
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

    #[doc="NVMCTRL APB Clock Enable"]
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

    #[doc="PORT APB Clock Enable"]
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

    #[doc="HMATRIX APB Clock Enable"]
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

    #[doc="EVSYS APB Clock Enable"]
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

    #[doc="SERCOM2 APB Clock Enable"]
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

    #[doc="SERCOM3 APB Clock Enable"]
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

    #[doc="TCC0 APB Clock Enable"]
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

    #[doc="TCC1 APB Clock Enable"]
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

    #[doc="TC2 APB Clock Enable"]
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

    #[doc="TC3 APB Clock Enable"]
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

    #[doc="RAMECC APB Clock Enable"]
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

impl From<u32> for Apbbmask {
    #[inline]
    fn from(other: u32) -> Self {
         Apbbmask(other)
    }
}

impl ::core::fmt::Display for Apbbmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apbbmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usb_() != 0 { try!(write!(f, " usb_"))}
        if self.dsu_() != 0 { try!(write!(f, " dsu_"))}
        if self.nvmctrl_() != 0 { try!(write!(f, " nvmctrl_"))}
        if self.port_() != 0 { try!(write!(f, " port_"))}
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

#[doc="APBC Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbcmask(pub u32);
impl Apbcmask {
    #[doc="TCC2 APB Clock Enable"]
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

    #[doc="TCC3 APB Clock Enable"]
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

    #[doc="TC4 APB Clock Enable"]
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

    #[doc="TC5 APB Clock Enable"]
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

    #[doc="PDEC APB Clock Enable"]
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

    #[doc="AC APB Clock Enable"]
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

    #[doc="AES APB Clock Enable"]
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

    #[doc="TRNG APB Clock Enable"]
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

    #[doc="ICM APB Clock Enable"]
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

    #[doc="QSPI APB Clock Enable"]
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

    #[doc="CCL APB Clock Enable"]
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

impl From<u32> for Apbcmask {
    #[inline]
    fn from(other: u32) -> Self {
         Apbcmask(other)
    }
}

impl ::core::fmt::Display for Apbcmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apbcmask {
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
        if self.qspi_() != 0 { try!(write!(f, " qspi_"))}
        if self.ccl_() != 0 { try!(write!(f, " ccl_"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APBD Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbdmask(pub u32);
impl Apbdmask {
    #[doc="SERCOM4 APB Clock Enable"]
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

    #[doc="SERCOM5 APB Clock Enable"]
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

    #[doc="TCC4 APB Clock Enable"]
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

    #[doc="ADC0 APB Clock Enable"]
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

    #[doc="ADC1 APB Clock Enable"]
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

    #[doc="DAC APB Clock Enable"]
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

    #[doc="I2S APB Clock Enable"]
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

    #[doc="PCC APB Clock Enable"]
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

impl From<u32> for Apbdmask {
    #[inline]
    fn from(other: u32) -> Self {
         Apbdmask(other)
    }
}

impl ::core::fmt::Display for Apbdmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apbdmask {
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

