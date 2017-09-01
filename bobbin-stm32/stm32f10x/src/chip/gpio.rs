#[allow(unused_imports)] use bobbin_common::*;

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x40010800);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x40010c00);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x40011000);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x40011400);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x40011800);
periph!( GPIOF, Gpiof, _GPIOF, GpioPeriph, 0x40011c00);
periph!( GPIOG, Gpiog, _GPIOG, GpioPeriph, 0x40012000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 









impl GpioPeriph {
    #[doc="Get the *mut pointer for the CRL register."]
    #[inline] pub fn crl_mut(&self) -> *mut Crl { 
        (self.0 + 0x0) as *mut Crl
    }

    #[doc="Get the *const pointer for the CRL register."]
    #[inline] pub fn crl_ptr(&self) -> *const Crl { 
           self.crl_mut()
    }

    #[doc="Read the CRL register."]
    #[inline] pub fn crl(&self) -> Crl { 
        unsafe {
            read_volatile(self.crl_ptr())
        }
    }

    #[doc="Write the CRL register."]
    #[inline] pub fn set_crl<F: FnOnce(Crl) -> Crl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crl_mut(), f(Crl(0)));
        }
        self
    }

    #[doc="Modify the CRL register."]
    #[inline] pub fn with_crl<F: FnOnce(Crl) -> Crl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crl_mut(), f(self.crl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CRH register."]
    #[inline] pub fn crh_mut(&self) -> *mut Crh { 
        (self.0 + 0x4) as *mut Crh
    }

    #[doc="Get the *const pointer for the CRH register."]
    #[inline] pub fn crh_ptr(&self) -> *const Crh { 
           self.crh_mut()
    }

    #[doc="Read the CRH register."]
    #[inline] pub fn crh(&self) -> Crh { 
        unsafe {
            read_volatile(self.crh_ptr())
        }
    }

    #[doc="Write the CRH register."]
    #[inline] pub fn set_crh<F: FnOnce(Crh) -> Crh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crh_mut(), f(Crh(0)));
        }
        self
    }

    #[doc="Modify the CRH register."]
    #[inline] pub fn with_crh<F: FnOnce(Crh) -> Crh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crh_mut(), f(self.crh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IDR register."]
    #[inline] pub fn idr_mut(&self) -> *mut Idr { 
        (self.0 + 0x8) as *mut Idr
    }

    #[doc="Get the *const pointer for the IDR register."]
    #[inline] pub fn idr_ptr(&self) -> *const Idr { 
           self.idr_mut()
    }

    #[doc="Read the IDR register."]
    #[inline] pub fn idr(&self) -> Idr { 
        unsafe {
            read_volatile(self.idr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the ODR register."]
    #[inline] pub fn odr_mut(&self) -> *mut Odr { 
        (self.0 + 0xc) as *mut Odr
    }

    #[doc="Get the *const pointer for the ODR register."]
    #[inline] pub fn odr_ptr(&self) -> *const Odr { 
           self.odr_mut()
    }

    #[doc="Read the ODR register."]
    #[inline] pub fn odr(&self) -> Odr { 
        unsafe {
            read_volatile(self.odr_ptr())
        }
    }

    #[doc="Write the ODR register."]
    #[inline] pub fn set_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.odr_mut(), f(Odr(0)));
        }
        self
    }

    #[doc="Modify the ODR register."]
    #[inline] pub fn with_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.odr_mut(), f(self.odr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BSRR register."]
    #[inline] pub fn bsrr_mut(&self) -> *mut Bsrr { 
        (self.0 + 0x10) as *mut Bsrr
    }

    #[doc="Get the *const pointer for the BSRR register."]
    #[inline] pub fn bsrr_ptr(&self) -> *const Bsrr { 
           self.bsrr_mut()
    }

    #[doc="Write the BSRR register."]
    #[inline] pub fn set_bsrr<F: FnOnce(Bsrr) -> Bsrr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bsrr_mut(), f(Bsrr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the BRR register."]
    #[inline] pub fn brr_mut(&self) -> *mut Brr { 
        (self.0 + 0x14) as *mut Brr
    }

    #[doc="Get the *const pointer for the BRR register."]
    #[inline] pub fn brr_ptr(&self) -> *const Brr { 
           self.brr_mut()
    }

    #[doc="Write the BRR register."]
    #[inline] pub fn set_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.brr_mut(), f(Brr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the LCKR register."]
    #[inline] pub fn lckr_mut(&self) -> *mut Lckr { 
        (self.0 + 0x18) as *mut Lckr
    }

    #[doc="Get the *const pointer for the LCKR register."]
    #[inline] pub fn lckr_ptr(&self) -> *const Lckr { 
           self.lckr_mut()
    }

    #[doc="Read the LCKR register."]
    #[inline] pub fn lckr(&self) -> Lckr { 
        unsafe {
            read_volatile(self.lckr_ptr())
        }
    }

    #[doc="Write the LCKR register."]
    #[inline] pub fn set_lckr<F: FnOnce(Lckr) -> Lckr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lckr_mut(), f(Lckr(0)));
        }
        self
    }

    #[doc="Modify the LCKR register."]
    #[inline] pub fn with_lckr<F: FnOnce(Lckr) -> Lckr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lckr_mut(), f(self.lckr()));
        }
        self
    }

}

#[doc="Port configuration register low (GPIOn_CRL)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crl(pub u32);
impl Crl {
    #[doc="Port n.m mode bits"]
    #[inline] pub fn mode<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
    }

    #[doc="Port n.m mode bits"]
    #[inline] pub fn test_mode<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mode(index) != 0
    }

    #[doc="Port n.m mode bits"]
    #[inline] pub fn set_mode<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Port n.m configuration bits"]
    #[inline] pub fn cnf<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [3:2]
    }

    #[doc="Port n.m configuration bits"]
    #[inline] pub fn test_cnf<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.cnf(index) != 0
    }

    #[doc="Port n.m configuration bits"]
    #[inline] pub fn set_cnf<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 2);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Crl {
    #[inline]
    fn from(other: u32) -> Self {
         Crl(other)
    }
}

impl ::core::fmt::Display for Crl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode(0) != 0 { try!(write!(f, " mode[0]=0x{:x}", self.mode(0)))}
        if self.mode(1) != 0 { try!(write!(f, " mode[1]=0x{:x}", self.mode(1)))}
        if self.mode(2) != 0 { try!(write!(f, " mode[2]=0x{:x}", self.mode(2)))}
        if self.mode(3) != 0 { try!(write!(f, " mode[3]=0x{:x}", self.mode(3)))}
        if self.mode(4) != 0 { try!(write!(f, " mode[4]=0x{:x}", self.mode(4)))}
        if self.mode(5) != 0 { try!(write!(f, " mode[5]=0x{:x}", self.mode(5)))}
        if self.mode(6) != 0 { try!(write!(f, " mode[6]=0x{:x}", self.mode(6)))}
        if self.mode(7) != 0 { try!(write!(f, " mode[7]=0x{:x}", self.mode(7)))}
        if self.cnf(0) != 0 { try!(write!(f, " cnf[0]=0x{:x}", self.cnf(0)))}
        if self.cnf(1) != 0 { try!(write!(f, " cnf[1]=0x{:x}", self.cnf(1)))}
        if self.cnf(2) != 0 { try!(write!(f, " cnf[2]=0x{:x}", self.cnf(2)))}
        if self.cnf(3) != 0 { try!(write!(f, " cnf[3]=0x{:x}", self.cnf(3)))}
        if self.cnf(4) != 0 { try!(write!(f, " cnf[4]=0x{:x}", self.cnf(4)))}
        if self.cnf(5) != 0 { try!(write!(f, " cnf[5]=0x{:x}", self.cnf(5)))}
        if self.cnf(6) != 0 { try!(write!(f, " cnf[6]=0x{:x}", self.cnf(6)))}
        if self.cnf(7) != 0 { try!(write!(f, " cnf[7]=0x{:x}", self.cnf(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port configuration register high (GPIOn_CRL)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crh(pub u32);
impl Crh {
    #[doc="Port n.m mode bits"]
    #[inline] pub fn mode<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
    }

    #[doc="Port n.m mode bits"]
    #[inline] pub fn test_mode<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mode(index) != 0
    }

    #[doc="Port n.m mode bits"]
    #[inline] pub fn set_mode<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Port n.m configuration bits"]
    #[inline] pub fn cnf<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [3:2]
    }

    #[doc="Port n.m configuration bits"]
    #[inline] pub fn test_cnf<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.cnf(index) != 0
    }

    #[doc="Port n.m configuration bits"]
    #[inline] pub fn set_cnf<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 2);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Crh {
    #[inline]
    fn from(other: u32) -> Self {
         Crh(other)
    }
}

impl ::core::fmt::Display for Crh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode(0) != 0 { try!(write!(f, " mode[0]=0x{:x}", self.mode(0)))}
        if self.mode(1) != 0 { try!(write!(f, " mode[1]=0x{:x}", self.mode(1)))}
        if self.mode(2) != 0 { try!(write!(f, " mode[2]=0x{:x}", self.mode(2)))}
        if self.mode(3) != 0 { try!(write!(f, " mode[3]=0x{:x}", self.mode(3)))}
        if self.mode(4) != 0 { try!(write!(f, " mode[4]=0x{:x}", self.mode(4)))}
        if self.mode(5) != 0 { try!(write!(f, " mode[5]=0x{:x}", self.mode(5)))}
        if self.mode(6) != 0 { try!(write!(f, " mode[6]=0x{:x}", self.mode(6)))}
        if self.mode(7) != 0 { try!(write!(f, " mode[7]=0x{:x}", self.mode(7)))}
        if self.cnf(0) != 0 { try!(write!(f, " cnf[0]=0x{:x}", self.cnf(0)))}
        if self.cnf(1) != 0 { try!(write!(f, " cnf[1]=0x{:x}", self.cnf(1)))}
        if self.cnf(2) != 0 { try!(write!(f, " cnf[2]=0x{:x}", self.cnf(2)))}
        if self.cnf(3) != 0 { try!(write!(f, " cnf[3]=0x{:x}", self.cnf(3)))}
        if self.cnf(4) != 0 { try!(write!(f, " cnf[4]=0x{:x}", self.cnf(4)))}
        if self.cnf(5) != 0 { try!(write!(f, " cnf[5]=0x{:x}", self.cnf(5)))}
        if self.cnf(6) != 0 { try!(write!(f, " cnf[6]=0x{:x}", self.cnf(6)))}
        if self.cnf(7) != 0 { try!(write!(f, " cnf[7]=0x{:x}", self.cnf(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port input data register (GPIOn_IDR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc="Port input data"]
    #[inline] pub fn idr<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Port input data"]
    #[inline] pub fn test_idr<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.idr(index) != 0
    }

    #[doc="Port input data"]
    #[inline] pub fn set_idr<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Idr {
    #[inline]
    fn from(other: u32) -> Self {
         Idr(other)
    }
}

impl ::core::fmt::Display for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.idr(0) != 0 { try!(write!(f, " idr[0]"))}
        if self.idr(1) != 0 { try!(write!(f, " idr[1]"))}
        if self.idr(2) != 0 { try!(write!(f, " idr[2]"))}
        if self.idr(3) != 0 { try!(write!(f, " idr[3]"))}
        if self.idr(4) != 0 { try!(write!(f, " idr[4]"))}
        if self.idr(5) != 0 { try!(write!(f, " idr[5]"))}
        if self.idr(6) != 0 { try!(write!(f, " idr[6]"))}
        if self.idr(7) != 0 { try!(write!(f, " idr[7]"))}
        if self.idr(8) != 0 { try!(write!(f, " idr[8]"))}
        if self.idr(9) != 0 { try!(write!(f, " idr[9]"))}
        if self.idr(10) != 0 { try!(write!(f, " idr[10]"))}
        if self.idr(11) != 0 { try!(write!(f, " idr[11]"))}
        if self.idr(12) != 0 { try!(write!(f, " idr[12]"))}
        if self.idr(13) != 0 { try!(write!(f, " idr[13]"))}
        if self.idr(14) != 0 { try!(write!(f, " idr[14]"))}
        if self.idr(15) != 0 { try!(write!(f, " idr[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port output data register (GPIOn_ODR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Odr(pub u32);
impl Odr {
    #[doc="Port output data"]
    #[inline] pub fn odr<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Port output data"]
    #[inline] pub fn test_odr<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.odr(index) != 0
    }

    #[doc="Port output data"]
    #[inline] pub fn set_odr<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Odr {
    #[inline]
    fn from(other: u32) -> Self {
         Odr(other)
    }
}

impl ::core::fmt::Display for Odr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Odr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.odr(0) != 0 { try!(write!(f, " odr[0]"))}
        if self.odr(1) != 0 { try!(write!(f, " odr[1]"))}
        if self.odr(2) != 0 { try!(write!(f, " odr[2]"))}
        if self.odr(3) != 0 { try!(write!(f, " odr[3]"))}
        if self.odr(4) != 0 { try!(write!(f, " odr[4]"))}
        if self.odr(5) != 0 { try!(write!(f, " odr[5]"))}
        if self.odr(6) != 0 { try!(write!(f, " odr[6]"))}
        if self.odr(7) != 0 { try!(write!(f, " odr[7]"))}
        if self.odr(8) != 0 { try!(write!(f, " odr[8]"))}
        if self.odr(9) != 0 { try!(write!(f, " odr[9]"))}
        if self.odr(10) != 0 { try!(write!(f, " odr[10]"))}
        if self.odr(11) != 0 { try!(write!(f, " odr[11]"))}
        if self.odr(12) != 0 { try!(write!(f, " odr[12]"))}
        if self.odr(13) != 0 { try!(write!(f, " odr[13]"))}
        if self.odr(14) != 0 { try!(write!(f, " odr[14]"))}
        if self.odr(15) != 0 { try!(write!(f, " odr[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port bit set/reset register (GPIOn_BSRR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bsrr(pub u32);
impl Bsrr {
    #[doc="Set bit n"]
    #[inline] pub fn bs<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Set bit n"]
    #[inline] pub fn test_bs<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.bs(index) != 0
    }

    #[doc="Set bit n"]
    #[inline] pub fn set_bs<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Reset bit n"]
    #[inline] pub fn br<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Reset bit n"]
    #[inline] pub fn test_br<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.br(index) != 0
    }

    #[doc="Reset bit n"]
    #[inline] pub fn set_br<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Bsrr {
    #[inline]
    fn from(other: u32) -> Self {
         Bsrr(other)
    }
}

impl ::core::fmt::Display for Bsrr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bsrr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bs(0) != 0 { try!(write!(f, " bs[0]"))}
        if self.bs(1) != 0 { try!(write!(f, " bs[1]"))}
        if self.bs(2) != 0 { try!(write!(f, " bs[2]"))}
        if self.bs(3) != 0 { try!(write!(f, " bs[3]"))}
        if self.bs(4) != 0 { try!(write!(f, " bs[4]"))}
        if self.bs(5) != 0 { try!(write!(f, " bs[5]"))}
        if self.bs(6) != 0 { try!(write!(f, " bs[6]"))}
        if self.bs(7) != 0 { try!(write!(f, " bs[7]"))}
        if self.bs(8) != 0 { try!(write!(f, " bs[8]"))}
        if self.bs(9) != 0 { try!(write!(f, " bs[9]"))}
        if self.bs(10) != 0 { try!(write!(f, " bs[10]"))}
        if self.bs(11) != 0 { try!(write!(f, " bs[11]"))}
        if self.bs(12) != 0 { try!(write!(f, " bs[12]"))}
        if self.bs(13) != 0 { try!(write!(f, " bs[13]"))}
        if self.bs(14) != 0 { try!(write!(f, " bs[14]"))}
        if self.bs(15) != 0 { try!(write!(f, " bs[15]"))}
        if self.br(0) != 0 { try!(write!(f, " br[0]"))}
        if self.br(1) != 0 { try!(write!(f, " br[1]"))}
        if self.br(2) != 0 { try!(write!(f, " br[2]"))}
        if self.br(3) != 0 { try!(write!(f, " br[3]"))}
        if self.br(4) != 0 { try!(write!(f, " br[4]"))}
        if self.br(5) != 0 { try!(write!(f, " br[5]"))}
        if self.br(6) != 0 { try!(write!(f, " br[6]"))}
        if self.br(7) != 0 { try!(write!(f, " br[7]"))}
        if self.br(8) != 0 { try!(write!(f, " br[8]"))}
        if self.br(9) != 0 { try!(write!(f, " br[9]"))}
        if self.br(10) != 0 { try!(write!(f, " br[10]"))}
        if self.br(11) != 0 { try!(write!(f, " br[11]"))}
        if self.br(12) != 0 { try!(write!(f, " br[12]"))}
        if self.br(13) != 0 { try!(write!(f, " br[13]"))}
        if self.br(14) != 0 { try!(write!(f, " br[14]"))}
        if self.br(15) != 0 { try!(write!(f, " br[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port bit reset register (GPIOn_BRR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc="Reset bit n"]
    #[inline] pub fn br<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Reset bit n"]
    #[inline] pub fn test_br<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.br(index) != 0
    }

    #[doc="Reset bit n"]
    #[inline] pub fn set_br<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Brr {
    #[inline]
    fn from(other: u32) -> Self {
         Brr(other)
    }
}

impl ::core::fmt::Display for Brr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Brr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.br(0) != 0 { try!(write!(f, " br[0]"))}
        if self.br(1) != 0 { try!(write!(f, " br[1]"))}
        if self.br(2) != 0 { try!(write!(f, " br[2]"))}
        if self.br(3) != 0 { try!(write!(f, " br[3]"))}
        if self.br(4) != 0 { try!(write!(f, " br[4]"))}
        if self.br(5) != 0 { try!(write!(f, " br[5]"))}
        if self.br(6) != 0 { try!(write!(f, " br[6]"))}
        if self.br(7) != 0 { try!(write!(f, " br[7]"))}
        if self.br(8) != 0 { try!(write!(f, " br[8]"))}
        if self.br(9) != 0 { try!(write!(f, " br[9]"))}
        if self.br(10) != 0 { try!(write!(f, " br[10]"))}
        if self.br(11) != 0 { try!(write!(f, " br[11]"))}
        if self.br(12) != 0 { try!(write!(f, " br[12]"))}
        if self.br(13) != 0 { try!(write!(f, " br[13]"))}
        if self.br(14) != 0 { try!(write!(f, " br[14]"))}
        if self.br(15) != 0 { try!(write!(f, " br[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port configuration lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lckr(pub u32);
impl Lckr {
    #[doc="Port Lock bit n"]
    #[inline] pub fn lck0<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Port Lock bit n"]
    #[inline] pub fn test_lck0<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.lck0(index) != 0
    }

    #[doc="Port Lock bit n"]
    #[inline] pub fn set_lck0<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Lock key"]
    #[inline] pub fn lckk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Lock key"]
    #[inline] pub fn test_lckk(&self) -> bool {
        self.lckk() != 0
    }

    #[doc="Lock key"]
    #[inline] pub fn set_lckk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Lckr {
    #[inline]
    fn from(other: u32) -> Self {
         Lckr(other)
    }
}

impl ::core::fmt::Display for Lckr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lckr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lck0(0) != 0 { try!(write!(f, " lck0[0]"))}
        if self.lck0(1) != 0 { try!(write!(f, " lck0[1]"))}
        if self.lck0(2) != 0 { try!(write!(f, " lck0[2]"))}
        if self.lck0(3) != 0 { try!(write!(f, " lck0[3]"))}
        if self.lck0(4) != 0 { try!(write!(f, " lck0[4]"))}
        if self.lck0(5) != 0 { try!(write!(f, " lck0[5]"))}
        if self.lck0(6) != 0 { try!(write!(f, " lck0[6]"))}
        if self.lck0(7) != 0 { try!(write!(f, " lck0[7]"))}
        if self.lck0(8) != 0 { try!(write!(f, " lck0[8]"))}
        if self.lck0(9) != 0 { try!(write!(f, " lck0[9]"))}
        if self.lck0(10) != 0 { try!(write!(f, " lck0[10]"))}
        if self.lck0(11) != 0 { try!(write!(f, " lck0[11]"))}
        if self.lck0(12) != 0 { try!(write!(f, " lck0[12]"))}
        if self.lck0(13) != 0 { try!(write!(f, " lck0[13]"))}
        if self.lck0(14) != 0 { try!(write!(f, " lck0[14]"))}
        if self.lck0(15) != 0 { try!(write!(f, " lck0[15]"))}
        if self.lckk() != 0 { try!(write!(f, " lckk"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct GpioPin { pub port: GpioPeriph, pub index: usize }
pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);

pin!(PA8, Pa8, GPIOA, Gpioa, _PA8, GpioPin, _GPIOA, 8);

pin!(PA9, Pa9, GPIOA, Gpioa, _PA9, GpioPin, _GPIOA, 9);

pin!(PA10, Pa10, GPIOA, Gpioa, _PA10, GpioPin, _GPIOA, 10);

pin!(PA11, Pa11, GPIOA, Gpioa, _PA11, GpioPin, _GPIOA, 11);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);

pin!(PA14, Pa14, GPIOA, Gpioa, _PA14, GpioPin, _GPIOA, 14);

pin!(PA15, Pa15, GPIOA, Gpioa, _PA15, GpioPin, _GPIOA, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);

pin!(PB6, Pb6, GPIOB, Gpiob, _PB6, GpioPin, _GPIOB, 6);

pin!(PB7, Pb7, GPIOB, Gpiob, _PB7, GpioPin, _GPIOB, 7);

pin!(PB8, Pb8, GPIOB, Gpiob, _PB8, GpioPin, _GPIOB, 8);

pin!(PB9, Pb9, GPIOB, Gpiob, _PB9, GpioPin, _GPIOB, 9);

pin!(PB10, Pb10, GPIOB, Gpiob, _PB10, GpioPin, _GPIOB, 10);

pin!(PB11, Pb11, GPIOB, Gpiob, _PB11, GpioPin, _GPIOB, 11);

pin!(PB12, Pb12, GPIOB, Gpiob, _PB12, GpioPin, _GPIOB, 12);

pin!(PB13, Pb13, GPIOB, Gpiob, _PB13, GpioPin, _GPIOB, 13);

pin!(PB14, Pb14, GPIOB, Gpiob, _PB14, GpioPin, _GPIOB, 14);

pin!(PB15, Pb15, GPIOB, Gpiob, _PB15, GpioPin, _GPIOB, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);

pin!(PC1, Pc1, GPIOC, Gpioc, _PC1, GpioPin, _GPIOC, 1);

pin!(PC2, Pc2, GPIOC, Gpioc, _PC2, GpioPin, _GPIOC, 2);

pin!(PC3, Pc3, GPIOC, Gpioc, _PC3, GpioPin, _GPIOC, 3);

pin!(PC4, Pc4, GPIOC, Gpioc, _PC4, GpioPin, _GPIOC, 4);

pin!(PC5, Pc5, GPIOC, Gpioc, _PC5, GpioPin, _GPIOC, 5);

pin!(PC6, Pc6, GPIOC, Gpioc, _PC6, GpioPin, _GPIOC, 6);

pin!(PC7, Pc7, GPIOC, Gpioc, _PC7, GpioPin, _GPIOC, 7);

pin!(PC8, Pc8, GPIOC, Gpioc, _PC8, GpioPin, _GPIOC, 8);

pin!(PC9, Pc9, GPIOC, Gpioc, _PC9, GpioPin, _GPIOC, 9);

pin!(PC10, Pc10, GPIOC, Gpioc, _PC10, GpioPin, _GPIOC, 10);

pin!(PC11, Pc11, GPIOC, Gpioc, _PC11, GpioPin, _GPIOC, 11);

pin!(PC12, Pc12, GPIOC, Gpioc, _PC12, GpioPin, _GPIOC, 12);

pin!(PC13, Pc13, GPIOC, Gpioc, _PC13, GpioPin, _GPIOC, 13);

pin!(PC14, Pc14, GPIOC, Gpioc, _PC14, GpioPin, _GPIOC, 14);

pin!(PC15, Pc15, GPIOC, Gpioc, _PC15, GpioPin, _GPIOC, 15);

pin!(PD0, Pd0, GPIOD, Gpiod, _PD0, GpioPin, _GPIOD, 0);

pin!(PD1, Pd1, GPIOD, Gpiod, _PD1, GpioPin, _GPIOD, 1);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);

pin!(PD3, Pd3, GPIOD, Gpiod, _PD3, GpioPin, _GPIOD, 3);

pin!(PD4, Pd4, GPIOD, Gpiod, _PD4, GpioPin, _GPIOD, 4);

pin!(PD5, Pd5, GPIOD, Gpiod, _PD5, GpioPin, _GPIOD, 5);

pin!(PD6, Pd6, GPIOD, Gpiod, _PD6, GpioPin, _GPIOD, 6);

pin!(PD7, Pd7, GPIOD, Gpiod, _PD7, GpioPin, _GPIOD, 7);

pin!(PD8, Pd8, GPIOD, Gpiod, _PD8, GpioPin, _GPIOD, 8);

pin!(PD9, Pd9, GPIOD, Gpiod, _PD9, GpioPin, _GPIOD, 9);

pin!(PD10, Pd10, GPIOD, Gpiod, _PD10, GpioPin, _GPIOD, 10);

pin!(PD11, Pd11, GPIOD, Gpiod, _PD11, GpioPin, _GPIOD, 11);

pin!(PD12, Pd12, GPIOD, Gpiod, _PD12, GpioPin, _GPIOD, 12);

pin!(PD13, Pd13, GPIOD, Gpiod, _PD13, GpioPin, _GPIOD, 13);

pin!(PD14, Pd14, GPIOD, Gpiod, _PD14, GpioPin, _GPIOD, 14);

pin!(PD15, Pd15, GPIOD, Gpiod, _PD15, GpioPin, _GPIOD, 15);

pin!(PE0, Pe0, GPIOE, Gpioe, _PE0, GpioPin, _GPIOE, 0);

pin!(PE1, Pe1, GPIOE, Gpioe, _PE1, GpioPin, _GPIOE, 1);

pin!(PE2, Pe2, GPIOE, Gpioe, _PE2, GpioPin, _GPIOE, 2);

pin!(PE3, Pe3, GPIOE, Gpioe, _PE3, GpioPin, _GPIOE, 3);

pin!(PE4, Pe4, GPIOE, Gpioe, _PE4, GpioPin, _GPIOE, 4);

pin!(PE5, Pe5, GPIOE, Gpioe, _PE5, GpioPin, _GPIOE, 5);

pin!(PE6, Pe6, GPIOE, Gpioe, _PE6, GpioPin, _GPIOE, 6);

pin!(PE7, Pe7, GPIOE, Gpioe, _PE7, GpioPin, _GPIOE, 7);

pin!(PE8, Pe8, GPIOE, Gpioe, _PE8, GpioPin, _GPIOE, 8);

pin!(PE9, Pe9, GPIOE, Gpioe, _PE9, GpioPin, _GPIOE, 9);

pin!(PE10, Pe10, GPIOE, Gpioe, _PE10, GpioPin, _GPIOE, 10);

pin!(PE11, Pe11, GPIOE, Gpioe, _PE11, GpioPin, _GPIOE, 11);

pin!(PE12, Pe12, GPIOE, Gpioe, _PE12, GpioPin, _GPIOE, 12);

pin!(PE13, Pe13, GPIOE, Gpioe, _PE13, GpioPin, _GPIOE, 13);

pin!(PE14, Pe14, GPIOE, Gpioe, _PE14, GpioPin, _GPIOE, 14);

pin!(PE15, Pe15, GPIOE, Gpioe, _PE15, GpioPin, _GPIOE, 15);

pin!(PF0, Pf0, GPIOF, Gpiof, _PF0, GpioPin, _GPIOF, 0);

pin!(PF1, Pf1, GPIOF, Gpiof, _PF1, GpioPin, _GPIOF, 1);

pin!(PF2, Pf2, GPIOF, Gpiof, _PF2, GpioPin, _GPIOF, 2);

pin!(PF3, Pf3, GPIOF, Gpiof, _PF3, GpioPin, _GPIOF, 3);

pin!(PF4, Pf4, GPIOF, Gpiof, _PF4, GpioPin, _GPIOF, 4);

pin!(PF5, Pf5, GPIOF, Gpiof, _PF5, GpioPin, _GPIOF, 5);

pin!(PF6, Pf6, GPIOF, Gpiof, _PF6, GpioPin, _GPIOF, 6);

pin!(PF7, Pf7, GPIOF, Gpiof, _PF7, GpioPin, _GPIOF, 7);

pin!(PF8, Pf8, GPIOF, Gpiof, _PF8, GpioPin, _GPIOF, 8);

pin!(PF9, Pf9, GPIOF, Gpiof, _PF9, GpioPin, _GPIOF, 9);

pin!(PF10, Pf10, GPIOF, Gpiof, _PF10, GpioPin, _GPIOF, 10);

pin!(PF11, Pf11, GPIOF, Gpiof, _PF11, GpioPin, _GPIOF, 11);

pin!(PF12, Pf12, GPIOF, Gpiof, _PF12, GpioPin, _GPIOF, 12);

pin!(PF13, Pf13, GPIOF, Gpiof, _PF13, GpioPin, _GPIOF, 13);

pin!(PF14, Pf14, GPIOF, Gpiof, _PF14, GpioPin, _GPIOF, 14);

pin!(PF15, Pf15, GPIOF, Gpiof, _PF15, GpioPin, _GPIOF, 15);

pin!(PG0, Pg0, GPIOG, Gpiog, _PG0, GpioPin, _GPIOG, 0);

pin!(PG1, Pg1, GPIOG, Gpiog, _PG1, GpioPin, _GPIOG, 1);

pin!(PG2, Pg2, GPIOG, Gpiog, _PG2, GpioPin, _GPIOG, 2);

pin!(PG3, Pg3, GPIOG, Gpiog, _PG3, GpioPin, _GPIOG, 3);

pin!(PG4, Pg4, GPIOG, Gpiog, _PG4, GpioPin, _GPIOG, 4);

pin!(PG5, Pg5, GPIOG, Gpiog, _PG5, GpioPin, _GPIOG, 5);

pin!(PG6, Pg6, GPIOG, Gpiog, _PG6, GpioPin, _GPIOG, 6);

pin!(PG7, Pg7, GPIOG, Gpiog, _PG7, GpioPin, _GPIOG, 7);

pin!(PG8, Pg8, GPIOG, Gpiog, _PG8, GpioPin, _GPIOG, 8);

pin!(PG9, Pg9, GPIOG, Gpiog, _PG9, GpioPin, _GPIOG, 9);

pin!(PG10, Pg10, GPIOG, Gpiog, _PG10, GpioPin, _GPIOG, 10);

pin!(PG11, Pg11, GPIOG, Gpiog, _PG11, GpioPin, _GPIOG, 11);

pin!(PG12, Pg12, GPIOG, Gpiog, _PG12, GpioPin, _GPIOG, 12);

pin!(PG13, Pg13, GPIOG, Gpiog, _PG13, GpioPin, _GPIOG, 13);

pin!(PG14, Pg14, GPIOG, Gpiog, _PG14, GpioPin, _GPIOG, 14);

pin!(PG15, Pg15, GPIOG, Gpiog, _PG15, GpioPin, _GPIOG, 15);


