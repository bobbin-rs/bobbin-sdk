#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 

pub struct GpioPin { pub port: GpioPeriph, pub index: usize }
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

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mode(index) != 0
    }

    #[doc="Sets the MODE field."]
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

    #[doc="Returns true if CNF != 0"]
    #[inline] pub fn test_cnf<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.cnf(index) != 0
    }

    #[doc="Sets the CNF field."]
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

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mode(index) != 0
    }

    #[doc="Sets the MODE field."]
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

    #[doc="Returns true if CNF != 0"]
    #[inline] pub fn test_cnf<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.cnf(index) != 0
    }

    #[doc="Sets the CNF field."]
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

    #[doc="Returns true if IDR != 0"]
    #[inline] pub fn test_idr<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.idr(index) != 0
    }

    #[doc="Sets the IDR field."]
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

    #[doc="Returns true if ODR != 0"]
    #[inline] pub fn test_odr<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.odr(index) != 0
    }

    #[doc="Sets the ODR field."]
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

    #[doc="Returns true if BS != 0"]
    #[inline] pub fn test_bs<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.bs(index) != 0
    }

    #[doc="Sets the BS field."]
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

    #[doc="Returns true if BR != 0"]
    #[inline] pub fn test_br<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.br(index) != 0
    }

    #[doc="Sets the BR field."]
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

    #[doc="Returns true if BR != 0"]
    #[inline] pub fn test_br<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.br(index) != 0
    }

    #[doc="Sets the BR field."]
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

    #[doc="Returns true if LCK0 != 0"]
    #[inline] pub fn test_lck0<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.lck0(index) != 0
    }

    #[doc="Sets the LCK0 field."]
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

    #[doc="Returns true if LCKK != 0"]
    #[inline] pub fn test_lckk(&self) -> bool {
        self.lckk() != 0
    }

    #[doc="Sets the LCKK field."]
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

