
::bobbin_mcu::periph!( MCLK, Mclk, MCLK_PERIPH, MclkPeriph, MCLK_OWNED, MCLK_REF_COUNT, 0x40000800, 0x00, 0x00);


#[doc="Main Clock"]
#[derive(Clone, Copy, PartialEq, Eq)]
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
    #[inline] pub fn hpb0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HPB0 != 0"]
    #[inline] pub fn test_hpb0(&self) -> bool {
        self.hpb0() != 0
    }

    #[doc="Sets the HPB0 field."]
    #[inline] pub fn set_hpb0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HPB1 AHB Clock Mask"]
    #[inline] pub fn hpb1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HPB1 != 0"]
    #[inline] pub fn test_hpb1(&self) -> bool {
        self.hpb1() != 0
    }

    #[doc="Sets the HPB1 field."]
    #[inline] pub fn set_hpb1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HPB2 AHB Clock Mask"]
    #[inline] pub fn hpb2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HPB2 != 0"]
    #[inline] pub fn test_hpb2(&self) -> bool {
        self.hpb2() != 0
    }

    #[doc="Sets the HPB2 field."]
    #[inline] pub fn set_hpb2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="HPB3 AHB Clock Mask"]
    #[inline] pub fn hpb3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HPB3 != 0"]
    #[inline] pub fn test_hpb3(&self) -> bool {
        self.hpb3() != 0
    }

    #[doc="Sets the HPB3 field."]
    #[inline] pub fn set_hpb3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DSU AHB Clock Mask"]
    #[inline] pub fn dsu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DSU != 0"]
    #[inline] pub fn test_dsu(&self) -> bool {
        self.dsu() != 0
    }

    #[doc="Sets the DSU field."]
    #[inline] pub fn set_dsu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HMATRIX AHB Clock Mask"]
    #[inline] pub fn hmatrix(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if HMATRIX != 0"]
    #[inline] pub fn test_hmatrix(&self) -> bool {
        self.hmatrix() != 0
    }

    #[doc="Sets the HMATRIX field."]
    #[inline] pub fn set_hmatrix<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="NVMCTRL AHB Clock Mask"]
    #[inline] pub fn nvmctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NVMCTRL != 0"]
    #[inline] pub fn test_nvmctrl(&self) -> bool {
        self.nvmctrl() != 0
    }

    #[doc="Sets the NVMCTRL field."]
    #[inline] pub fn set_nvmctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="HSRAM AHB Clock Mask"]
    #[inline] pub fn hsram(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if HSRAM != 0"]
    #[inline] pub fn test_hsram(&self) -> bool {
        self.hsram() != 0
    }

    #[doc="Sets the HSRAM field."]
    #[inline] pub fn set_hsram<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CMCC AHB Clock Mask"]
    #[inline] pub fn cmcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CMCC != 0"]
    #[inline] pub fn test_cmcc(&self) -> bool {
        self.cmcc() != 0
    }

    #[doc="Sets the CMCC field."]
    #[inline] pub fn set_cmcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMAC AHB Clock Mask"]
    #[inline] pub fn dmac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DMAC != 0"]
    #[inline] pub fn test_dmac(&self) -> bool {
        self.dmac() != 0
    }

    #[doc="Sets the DMAC field."]
    #[inline] pub fn set_dmac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="USB AHB Clock Mask"]
    #[inline] pub fn usb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if USB != 0"]
    #[inline] pub fn test_usb(&self) -> bool {
        self.usb() != 0
    }

    #[doc="Sets the USB field."]
    #[inline] pub fn set_usb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BKUPRAM AHB Clock Mask"]
    #[inline] pub fn bkupram(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BKUPRAM != 0"]
    #[inline] pub fn test_bkupram(&self) -> bool {
        self.bkupram() != 0
    }

    #[doc="Sets the BKUPRAM field."]
    #[inline] pub fn set_bkupram<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PAC AHB Clock Mask"]
    #[inline] pub fn pac(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAC != 0"]
    #[inline] pub fn test_pac(&self) -> bool {
        self.pac() != 0
    }

    #[doc="Sets the PAC field."]
    #[inline] pub fn set_pac<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="QSPI AHB Clock Mask"]
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

    #[doc="SDHC0 AHB Clock Mask"]
    #[inline] pub fn sdhc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SDHC0 != 0"]
    #[inline] pub fn test_sdhc0(&self) -> bool {
        self.sdhc0() != 0
    }

    #[doc="Sets the SDHC0 field."]
    #[inline] pub fn set_sdhc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="ICM AHB Clock Mask"]
    #[inline] pub fn icm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if ICM != 0"]
    #[inline] pub fn test_icm(&self) -> bool {
        self.icm() != 0
    }

    #[doc="Sets the ICM field."]
    #[inline] pub fn set_icm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="PUKCC AHB Clock Mask"]
    #[inline] pub fn pukcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PUKCC != 0"]
    #[inline] pub fn test_pukcc(&self) -> bool {
        self.pukcc() != 0
    }

    #[doc="Sets the PUKCC field."]
    #[inline] pub fn set_pukcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="QSPI_2X AHB Clock Mask"]
    #[inline] pub fn qspi_2x(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if QSPI_2X != 0"]
    #[inline] pub fn test_qspi_2x(&self) -> bool {
        self.qspi_2x() != 0
    }

    #[doc="Sets the QSPI_2X field."]
    #[inline] pub fn set_qspi_2x<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="NVMCTRL_SMEEPROM AHB Clock Mask"]
    #[inline] pub fn nvmctrl_smeeprom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if NVMCTRL_SMEEPROM != 0"]
    #[inline] pub fn test_nvmctrl_smeeprom(&self) -> bool {
        self.nvmctrl_smeeprom() != 0
    }

    #[doc="Sets the NVMCTRL_SMEEPROM field."]
    #[inline] pub fn set_nvmctrl_smeeprom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="NVMCTRL_CACHE AHB Clock Mask"]
    #[inline] pub fn nvmctrl_cache(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if NVMCTRL_CACHE != 0"]
    #[inline] pub fn test_nvmctrl_cache(&self) -> bool {
        self.nvmctrl_cache() != 0
    }

    #[doc="Sets the NVMCTRL_CACHE field."]
    #[inline] pub fn set_nvmctrl_cache<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.hpb0() != 0 { try!(write!(f, " hpb0"))}
        if self.hpb1() != 0 { try!(write!(f, " hpb1"))}
        if self.hpb2() != 0 { try!(write!(f, " hpb2"))}
        if self.hpb3() != 0 { try!(write!(f, " hpb3"))}
        if self.dsu() != 0 { try!(write!(f, " dsu"))}
        if self.hmatrix() != 0 { try!(write!(f, " hmatrix"))}
        if self.nvmctrl() != 0 { try!(write!(f, " nvmctrl"))}
        if self.hsram() != 0 { try!(write!(f, " hsram"))}
        if self.cmcc() != 0 { try!(write!(f, " cmcc"))}
        if self.dmac() != 0 { try!(write!(f, " dmac"))}
        if self.usb() != 0 { try!(write!(f, " usb"))}
        if self.bkupram() != 0 { try!(write!(f, " bkupram"))}
        if self.pac() != 0 { try!(write!(f, " pac"))}
        if self.qspi() != 0 { try!(write!(f, " qspi"))}
        if self.sdhc0() != 0 { try!(write!(f, " sdhc0"))}
        if self.icm() != 0 { try!(write!(f, " icm"))}
        if self.pukcc() != 0 { try!(write!(f, " pukcc"))}
        if self.qspi_2x() != 0 { try!(write!(f, " qspi_2x"))}
        if self.nvmctrl_smeeprom() != 0 { try!(write!(f, " nvmctrl_smeeprom"))}
        if self.nvmctrl_cache() != 0 { try!(write!(f, " nvmctrl_cache"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APBA Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbamask(pub u32);
impl Apbamask {
    #[doc="PAC APB Clock Enable"]
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

    #[doc="PM APB Clock Enable"]
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

    #[doc="MCLK APB Clock Enable"]
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

    #[doc="RSTC APB Clock Enable"]
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

    #[doc="OSCCTRL APB Clock Enable"]
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

    #[doc="OSC32KCTRL APB Clock Enable"]
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

    #[doc="SUPC APB Clock Enable"]
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

    #[doc="GCLK APB Clock Enable"]
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

    #[doc="WDT APB Clock Enable"]
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

    #[doc="RTC APB Clock Enable"]
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

    #[doc="EIC APB Clock Enable"]
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

    #[doc="FREQM APB Clock Enable"]
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

    #[doc="SERCOM0 APB Clock Enable"]
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

    #[doc="SERCOM1 APB Clock Enable"]
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

    #[doc="TC0 APB Clock Enable"]
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

    #[doc="TC1 APB Clock Enable"]
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

#[doc="APBB Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbbmask(pub u32);
impl Apbbmask {
    #[doc="USB APB Clock Enable"]
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

    #[doc="DSU APB Clock Enable"]
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

    #[doc="NVMCTRL APB Clock Enable"]
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

    #[doc="PORT APB Clock Enable"]
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

    #[doc="HMATRIX APB Clock Enable"]
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

    #[doc="EVSYS APB Clock Enable"]
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

    #[doc="SERCOM2 APB Clock Enable"]
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

    #[doc="SERCOM3 APB Clock Enable"]
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

    #[doc="TCC0 APB Clock Enable"]
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

    #[doc="TCC1 APB Clock Enable"]
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

    #[doc="TC2 APB Clock Enable"]
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

    #[doc="TC3 APB Clock Enable"]
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

    #[doc="RAMECC APB Clock Enable"]
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
        if self.usb() != 0 { try!(write!(f, " usb"))}
        if self.dsu() != 0 { try!(write!(f, " dsu"))}
        if self.nvmctrl() != 0 { try!(write!(f, " nvmctrl"))}
        if self.port() != 0 { try!(write!(f, " port"))}
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

#[doc="APBC Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbcmask(pub u32);
impl Apbcmask {
    #[doc="TCC2 APB Clock Enable"]
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

    #[doc="TCC3 APB Clock Enable"]
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

    #[doc="TC4 APB Clock Enable"]
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

    #[doc="TC5 APB Clock Enable"]
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

    #[doc="PDEC APB Clock Enable"]
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

    #[doc="AC APB Clock Enable"]
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

    #[doc="AES APB Clock Enable"]
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

    #[doc="TRNG APB Clock Enable"]
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

    #[doc="ICM APB Clock Enable"]
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

    #[doc="QSPI APB Clock Enable"]
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

    #[doc="CCL APB Clock Enable"]
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
        if self.tcc2() != 0 { try!(write!(f, " tcc2"))}
        if self.tcc3() != 0 { try!(write!(f, " tcc3"))}
        if self.tc4() != 0 { try!(write!(f, " tc4"))}
        if self.tc5() != 0 { try!(write!(f, " tc5"))}
        if self.pdec() != 0 { try!(write!(f, " pdec"))}
        if self.ac() != 0 { try!(write!(f, " ac"))}
        if self.aes() != 0 { try!(write!(f, " aes"))}
        if self.trng() != 0 { try!(write!(f, " trng"))}
        if self.icm() != 0 { try!(write!(f, " icm"))}
        if self.qspi() != 0 { try!(write!(f, " qspi"))}
        if self.ccl() != 0 { try!(write!(f, " ccl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APBD Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apbdmask(pub u32);
impl Apbdmask {
    #[doc="SERCOM4 APB Clock Enable"]
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

    #[doc="SERCOM5 APB Clock Enable"]
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

    #[doc="TCC4 APB Clock Enable"]
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

    #[doc="ADC0 APB Clock Enable"]
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

    #[doc="ADC1 APB Clock Enable"]
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

    #[doc="DAC APB Clock Enable"]
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

    #[doc="I2S APB Clock Enable"]
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

    #[doc="PCC APB Clock Enable"]
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

