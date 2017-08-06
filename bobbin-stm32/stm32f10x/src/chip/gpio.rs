#[allow(unused_imports)] use bobbin_common::bits;
pub const GPIOA: Gpioa = Periph(0x40010800, GpioaId {});
pub const GPIOB: Gpiob = Periph(0x40010c00, GpiobId {});
pub const GPIOC: Gpioc = Periph(0x40011000, GpiocId {});
pub const GPIOD: Gpiod = Periph(0x40011400, GpiodId {});
pub const GPIOE: Gpioe = Periph(0x40011800, GpioeId {});
pub const GPIOF: Gpiof = Periph(0x40011c00, GpiofId {});
pub const GPIOG: Gpiog = Periph(0x40012000, GpiogId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpioaId {}
pub type Gpioa = Periph<GpioaId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiobId {}
pub type Gpiob = Periph<GpiobId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiocId {}
pub type Gpioc = Periph<GpiocId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiodId {}
pub type Gpiod = Periph<GpiodId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpioeId {}
pub type Gpioe = Periph<GpioeId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiofId {}
pub type Gpiof = Periph<GpiofId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiogId {}
pub type Gpiog = Periph<GpiogId>;









impl<T> Periph<T> {
#[doc="Get the *const pointer for the CRL register."]
  #[inline] pub fn crl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CRL register."]
  #[inline] pub fn crl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CRL register."]
  #[inline] pub fn crl(&self) -> Crl { 
     unsafe {
        Crl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CRL register."]
  #[inline] pub fn set_crl(&self, value: Crl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CRL register."]
  #[inline] pub fn with_crl<F: FnOnce(Crl) -> Crl>(&self, f: F) -> &Self {
     let tmp = self.crl();
     self.set_crl(f(tmp))
  }

#[doc="Get the *const pointer for the CRH register."]
  #[inline] pub fn crh_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CRH register."]
  #[inline] pub fn crh_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CRH register."]
  #[inline] pub fn crh(&self) -> Crh { 
     unsafe {
        Crh(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CRH register."]
  #[inline] pub fn set_crh(&self, value: Crh) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CRH register."]
  #[inline] pub fn with_crh<F: FnOnce(Crh) -> Crh>(&self, f: F) -> &Self {
     let tmp = self.crh();
     self.set_crh(f(tmp))
  }

#[doc="Get the *const pointer for the IDR register."]
  #[inline] pub fn idr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the IDR register."]
  #[inline] pub fn idr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the IDR register."]
  #[inline] pub fn idr(&self) -> Idr { 
     unsafe {
        Idr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }

#[doc="Get the *const pointer for the ODR register."]
  #[inline] pub fn odr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the ODR register."]
  #[inline] pub fn odr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the ODR register."]
  #[inline] pub fn odr(&self) -> Odr { 
     unsafe {
        Odr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the ODR register."]
  #[inline] pub fn set_odr(&self, value: Odr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ODR register."]
  #[inline] pub fn with_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
     let tmp = self.odr();
     self.set_odr(f(tmp))
  }

#[doc="Get the *const pointer for the BSRR register."]
  #[inline] pub fn bsrr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the BSRR register."]
  #[inline] pub fn bsrr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Write the BSRR register."]
  #[inline] pub fn set_bsrr(&self, value: Bsrr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the BRR register."]
  #[inline] pub fn brr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the BRR register."]
  #[inline] pub fn brr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Write the BRR register."]
  #[inline] pub fn set_brr(&self, value: Brr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the LCKR register."]
  #[inline] pub fn lckr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the LCKR register."]
  #[inline] pub fn lckr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the LCKR register."]
  #[inline] pub fn lckr(&self) -> Lckr { 
     unsafe {
        Lckr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the LCKR register."]
  #[inline] pub fn set_lckr(&self, value: Lckr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LCKR register."]
  #[inline] pub fn with_lckr<F: FnOnce(Lckr) -> Lckr>(&self, f: F) -> &Self {
     let tmp = self.lckr();
     self.set_lckr(f(tmp))
  }

}

#[doc="Port configuration register low (GPIOn_CRL)"]
#[derive(PartialEq, Eq)]
pub struct Crl(pub u32);
impl Crl {
#[doc="Port n.m mode bits"]
  #[inline] pub fn mode(&self, index: usize) -> bits::B2 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     (((self.0 as u32) >> shift) & 0x3).into() // [1:0]
  }
#[doc="Port n.m mode bits"]
  #[inline] pub fn set_mode<V: Into<bits::B2>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!(index < 8);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Port n.m configuration bits"]
  #[inline] pub fn cnf(&self, index: usize) -> bits::B2 {
     assert!(index < 8);
     let shift: usize = 2 + (index << 2);
     (((self.0 as u32) >> shift) & 0x3).into() // [3:2]
  }
#[doc="Port n.m configuration bits"]
  #[inline] pub fn set_cnf<V: Into<bits::B2>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!(index < 8);
     assert!((value & !0x3) == 0);
     let shift: usize = 2 + (index << 2);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
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
#[derive(PartialEq, Eq)]
pub struct Crh(pub u32);
impl Crh {
#[doc="Port n.m mode bits"]
  #[inline] pub fn mode(&self, index: usize) -> bits::B2 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     (((self.0 as u32) >> shift) & 0x3).into() // [1:0]
  }
#[doc="Port n.m mode bits"]
  #[inline] pub fn set_mode<V: Into<bits::B2>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!(index < 8);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Port n.m configuration bits"]
  #[inline] pub fn cnf(&self, index: usize) -> bits::B2 {
     assert!(index < 8);
     let shift: usize = 2 + (index << 2);
     (((self.0 as u32) >> shift) & 0x3).into() // [3:2]
  }
#[doc="Port n.m configuration bits"]
  #[inline] pub fn set_cnf<V: Into<bits::B2>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!(index < 8);
     assert!((value & !0x3) == 0);
     let shift: usize = 2 + (index << 2);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
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
#[derive(PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
#[doc="Port input data"]
  #[inline] pub fn idr(&self, index: usize) -> bits::B1 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     (((self.0 as u32) >> shift) & 0x1).into() // [0]
  }
#[doc="Port input data"]
  #[inline] pub fn set_idr<V: Into<bits::B1>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
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
#[derive(PartialEq, Eq)]
pub struct Odr(pub u32);
impl Odr {
#[doc="Port output data"]
  #[inline] pub fn odr(&self, index: usize) -> bits::B1 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     (((self.0 as u32) >> shift) & 0x1).into() // [0]
  }
#[doc="Port output data"]
  #[inline] pub fn set_odr<V: Into<bits::B1>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
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
#[derive(PartialEq, Eq)]
pub struct Bsrr(pub u32);
impl Bsrr {
#[doc="Set bit n"]
  #[inline] pub fn bs(&self, index: usize) -> bits::B1 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     (((self.0 as u32) >> shift) & 0x1).into() // [0]
  }
#[doc="Set bit n"]
  #[inline] pub fn set_bs<V: Into<bits::B1>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Reset bit n"]
  #[inline] pub fn br(&self, index: usize) -> bits::B1 {
     assert!(index < 16);
     let shift: usize = 16 + index;
     (((self.0 as u32) >> shift) & 0x1).into() // [16]
  }
#[doc="Reset bit n"]
  #[inline] pub fn set_br<V: Into<bits::B1>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 16 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
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
#[derive(PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
#[doc="Reset bit n"]
  #[inline] pub fn br(&self, index: usize) -> bits::B1 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     (((self.0 as u32) >> shift) & 0x1).into() // [0]
  }
#[doc="Reset bit n"]
  #[inline] pub fn set_br<V: Into<bits::B1>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
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
#[derive(PartialEq, Eq)]
pub struct Lckr(pub u32);
impl Lckr {
#[doc="Port Lock bit n"]
  #[inline] pub fn lck0(&self, index: usize) -> bits::B1 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     (((self.0 as u32) >> shift) & 0x1).into() // [0]
  }
#[doc="Port Lock bit n"]
  #[inline] pub fn set_lck0<V: Into<bits::B1>>(mut self, index: usize, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Lock key"]
  #[inline] pub fn lckk(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="Lock key"]
  #[inline] pub fn set_lckk<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
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
#[doc="GPIO Pin"]
pub struct Pin<P, T> { pub port: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Pin<P,T> {
   #[inline] pub fn port(&self) -> &Periph<T> { &self.port }
   #[inline] pub fn index(&self) -> usize { self.index }
}
pub trait AltFn<T> {
   fn alt_fn(&self) -> usize;
}

pub const PA0: Pin<Pa0Id, GpioaId> = Pin { port: GPIOA, index: 0, id: Pa0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa0Id {}
pub type Pa0 = Pin<Pa0Id, GpioaId>;
pub const PA1: Pin<Pa1Id, GpioaId> = Pin { port: GPIOA, index: 1, id: Pa1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa1Id {}
pub type Pa1 = Pin<Pa1Id, GpioaId>;
pub const PA2: Pin<Pa2Id, GpioaId> = Pin { port: GPIOA, index: 2, id: Pa2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa2Id {}
pub type Pa2 = Pin<Pa2Id, GpioaId>;
pub const PA3: Pin<Pa3Id, GpioaId> = Pin { port: GPIOA, index: 3, id: Pa3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa3Id {}
pub type Pa3 = Pin<Pa3Id, GpioaId>;
pub const PA4: Pin<Pa4Id, GpioaId> = Pin { port: GPIOA, index: 4, id: Pa4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa4Id {}
pub type Pa4 = Pin<Pa4Id, GpioaId>;
pub const PA5: Pin<Pa5Id, GpioaId> = Pin { port: GPIOA, index: 5, id: Pa5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa5Id {}
pub type Pa5 = Pin<Pa5Id, GpioaId>;
pub const PA6: Pin<Pa6Id, GpioaId> = Pin { port: GPIOA, index: 6, id: Pa6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa6Id {}
pub type Pa6 = Pin<Pa6Id, GpioaId>;
pub const PA7: Pin<Pa7Id, GpioaId> = Pin { port: GPIOA, index: 7, id: Pa7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa7Id {}
pub type Pa7 = Pin<Pa7Id, GpioaId>;
pub const PA8: Pin<Pa8Id, GpioaId> = Pin { port: GPIOA, index: 8, id: Pa8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa8Id {}
pub type Pa8 = Pin<Pa8Id, GpioaId>;
pub const PA9: Pin<Pa9Id, GpioaId> = Pin { port: GPIOA, index: 9, id: Pa9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa9Id {}
pub type Pa9 = Pin<Pa9Id, GpioaId>;
pub const PA10: Pin<Pa10Id, GpioaId> = Pin { port: GPIOA, index: 10, id: Pa10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa10Id {}
pub type Pa10 = Pin<Pa10Id, GpioaId>;
pub const PA11: Pin<Pa11Id, GpioaId> = Pin { port: GPIOA, index: 11, id: Pa11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa11Id {}
pub type Pa11 = Pin<Pa11Id, GpioaId>;
pub const PA12: Pin<Pa12Id, GpioaId> = Pin { port: GPIOA, index: 12, id: Pa12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa12Id {}
pub type Pa12 = Pin<Pa12Id, GpioaId>;
pub const PA13: Pin<Pa13Id, GpioaId> = Pin { port: GPIOA, index: 13, id: Pa13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa13Id {}
pub type Pa13 = Pin<Pa13Id, GpioaId>;
pub const PA14: Pin<Pa14Id, GpioaId> = Pin { port: GPIOA, index: 14, id: Pa14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa14Id {}
pub type Pa14 = Pin<Pa14Id, GpioaId>;
pub const PA15: Pin<Pa15Id, GpioaId> = Pin { port: GPIOA, index: 15, id: Pa15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pa15Id {}
pub type Pa15 = Pin<Pa15Id, GpioaId>;
pub const PB0: Pin<Pb0Id, GpiobId> = Pin { port: GPIOB, index: 0, id: Pb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb0Id {}
pub type Pb0 = Pin<Pb0Id, GpiobId>;
pub const PB1: Pin<Pb1Id, GpiobId> = Pin { port: GPIOB, index: 1, id: Pb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb1Id {}
pub type Pb1 = Pin<Pb1Id, GpiobId>;
pub const PB2: Pin<Pb2Id, GpiobId> = Pin { port: GPIOB, index: 2, id: Pb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb2Id {}
pub type Pb2 = Pin<Pb2Id, GpiobId>;
pub const PB3: Pin<Pb3Id, GpiobId> = Pin { port: GPIOB, index: 3, id: Pb3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb3Id {}
pub type Pb3 = Pin<Pb3Id, GpiobId>;
pub const PB4: Pin<Pb4Id, GpiobId> = Pin { port: GPIOB, index: 4, id: Pb4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb4Id {}
pub type Pb4 = Pin<Pb4Id, GpiobId>;
pub const PB5: Pin<Pb5Id, GpiobId> = Pin { port: GPIOB, index: 5, id: Pb5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb5Id {}
pub type Pb5 = Pin<Pb5Id, GpiobId>;
pub const PB6: Pin<Pb6Id, GpiobId> = Pin { port: GPIOB, index: 6, id: Pb6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb6Id {}
pub type Pb6 = Pin<Pb6Id, GpiobId>;
pub const PB7: Pin<Pb7Id, GpiobId> = Pin { port: GPIOB, index: 7, id: Pb7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb7Id {}
pub type Pb7 = Pin<Pb7Id, GpiobId>;
pub const PB8: Pin<Pb8Id, GpiobId> = Pin { port: GPIOB, index: 8, id: Pb8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb8Id {}
pub type Pb8 = Pin<Pb8Id, GpiobId>;
pub const PB9: Pin<Pb9Id, GpiobId> = Pin { port: GPIOB, index: 9, id: Pb9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb9Id {}
pub type Pb9 = Pin<Pb9Id, GpiobId>;
pub const PB10: Pin<Pb10Id, GpiobId> = Pin { port: GPIOB, index: 10, id: Pb10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb10Id {}
pub type Pb10 = Pin<Pb10Id, GpiobId>;
pub const PB11: Pin<Pb11Id, GpiobId> = Pin { port: GPIOB, index: 11, id: Pb11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb11Id {}
pub type Pb11 = Pin<Pb11Id, GpiobId>;
pub const PB12: Pin<Pb12Id, GpiobId> = Pin { port: GPIOB, index: 12, id: Pb12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb12Id {}
pub type Pb12 = Pin<Pb12Id, GpiobId>;
pub const PB13: Pin<Pb13Id, GpiobId> = Pin { port: GPIOB, index: 13, id: Pb13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb13Id {}
pub type Pb13 = Pin<Pb13Id, GpiobId>;
pub const PB14: Pin<Pb14Id, GpiobId> = Pin { port: GPIOB, index: 14, id: Pb14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb14Id {}
pub type Pb14 = Pin<Pb14Id, GpiobId>;
pub const PB15: Pin<Pb15Id, GpiobId> = Pin { port: GPIOB, index: 15, id: Pb15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pb15Id {}
pub type Pb15 = Pin<Pb15Id, GpiobId>;
pub const PC0: Pin<Pc0Id, GpiocId> = Pin { port: GPIOC, index: 0, id: Pc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc0Id {}
pub type Pc0 = Pin<Pc0Id, GpiocId>;
pub const PC1: Pin<Pc1Id, GpiocId> = Pin { port: GPIOC, index: 1, id: Pc1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc1Id {}
pub type Pc1 = Pin<Pc1Id, GpiocId>;
pub const PC2: Pin<Pc2Id, GpiocId> = Pin { port: GPIOC, index: 2, id: Pc2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc2Id {}
pub type Pc2 = Pin<Pc2Id, GpiocId>;
pub const PC3: Pin<Pc3Id, GpiocId> = Pin { port: GPIOC, index: 3, id: Pc3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc3Id {}
pub type Pc3 = Pin<Pc3Id, GpiocId>;
pub const PC4: Pin<Pc4Id, GpiocId> = Pin { port: GPIOC, index: 4, id: Pc4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc4Id {}
pub type Pc4 = Pin<Pc4Id, GpiocId>;
pub const PC5: Pin<Pc5Id, GpiocId> = Pin { port: GPIOC, index: 5, id: Pc5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc5Id {}
pub type Pc5 = Pin<Pc5Id, GpiocId>;
pub const PC6: Pin<Pc6Id, GpiocId> = Pin { port: GPIOC, index: 6, id: Pc6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc6Id {}
pub type Pc6 = Pin<Pc6Id, GpiocId>;
pub const PC7: Pin<Pc7Id, GpiocId> = Pin { port: GPIOC, index: 7, id: Pc7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc7Id {}
pub type Pc7 = Pin<Pc7Id, GpiocId>;
pub const PC8: Pin<Pc8Id, GpiocId> = Pin { port: GPIOC, index: 8, id: Pc8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc8Id {}
pub type Pc8 = Pin<Pc8Id, GpiocId>;
pub const PC9: Pin<Pc9Id, GpiocId> = Pin { port: GPIOC, index: 9, id: Pc9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc9Id {}
pub type Pc9 = Pin<Pc9Id, GpiocId>;
pub const PC10: Pin<Pc10Id, GpiocId> = Pin { port: GPIOC, index: 10, id: Pc10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc10Id {}
pub type Pc10 = Pin<Pc10Id, GpiocId>;
pub const PC11: Pin<Pc11Id, GpiocId> = Pin { port: GPIOC, index: 11, id: Pc11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc11Id {}
pub type Pc11 = Pin<Pc11Id, GpiocId>;
pub const PC12: Pin<Pc12Id, GpiocId> = Pin { port: GPIOC, index: 12, id: Pc12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc12Id {}
pub type Pc12 = Pin<Pc12Id, GpiocId>;
pub const PC13: Pin<Pc13Id, GpiocId> = Pin { port: GPIOC, index: 13, id: Pc13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc13Id {}
pub type Pc13 = Pin<Pc13Id, GpiocId>;
pub const PC14: Pin<Pc14Id, GpiocId> = Pin { port: GPIOC, index: 14, id: Pc14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc14Id {}
pub type Pc14 = Pin<Pc14Id, GpiocId>;
pub const PC15: Pin<Pc15Id, GpiocId> = Pin { port: GPIOC, index: 15, id: Pc15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pc15Id {}
pub type Pc15 = Pin<Pc15Id, GpiocId>;
pub const PD0: Pin<Pd0Id, GpiodId> = Pin { port: GPIOD, index: 0, id: Pd0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd0Id {}
pub type Pd0 = Pin<Pd0Id, GpiodId>;
pub const PD1: Pin<Pd1Id, GpiodId> = Pin { port: GPIOD, index: 1, id: Pd1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd1Id {}
pub type Pd1 = Pin<Pd1Id, GpiodId>;
pub const PD2: Pin<Pd2Id, GpiodId> = Pin { port: GPIOD, index: 2, id: Pd2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd2Id {}
pub type Pd2 = Pin<Pd2Id, GpiodId>;
pub const PD3: Pin<Pd3Id, GpiodId> = Pin { port: GPIOD, index: 3, id: Pd3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd3Id {}
pub type Pd3 = Pin<Pd3Id, GpiodId>;
pub const PD4: Pin<Pd4Id, GpiodId> = Pin { port: GPIOD, index: 4, id: Pd4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd4Id {}
pub type Pd4 = Pin<Pd4Id, GpiodId>;
pub const PD5: Pin<Pd5Id, GpiodId> = Pin { port: GPIOD, index: 5, id: Pd5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd5Id {}
pub type Pd5 = Pin<Pd5Id, GpiodId>;
pub const PD6: Pin<Pd6Id, GpiodId> = Pin { port: GPIOD, index: 6, id: Pd6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd6Id {}
pub type Pd6 = Pin<Pd6Id, GpiodId>;
pub const PD7: Pin<Pd7Id, GpiodId> = Pin { port: GPIOD, index: 7, id: Pd7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd7Id {}
pub type Pd7 = Pin<Pd7Id, GpiodId>;
pub const PD8: Pin<Pd8Id, GpiodId> = Pin { port: GPIOD, index: 8, id: Pd8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd8Id {}
pub type Pd8 = Pin<Pd8Id, GpiodId>;
pub const PD9: Pin<Pd9Id, GpiodId> = Pin { port: GPIOD, index: 9, id: Pd9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd9Id {}
pub type Pd9 = Pin<Pd9Id, GpiodId>;
pub const PD10: Pin<Pd10Id, GpiodId> = Pin { port: GPIOD, index: 10, id: Pd10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd10Id {}
pub type Pd10 = Pin<Pd10Id, GpiodId>;
pub const PD11: Pin<Pd11Id, GpiodId> = Pin { port: GPIOD, index: 11, id: Pd11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd11Id {}
pub type Pd11 = Pin<Pd11Id, GpiodId>;
pub const PD12: Pin<Pd12Id, GpiodId> = Pin { port: GPIOD, index: 12, id: Pd12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd12Id {}
pub type Pd12 = Pin<Pd12Id, GpiodId>;
pub const PD13: Pin<Pd13Id, GpiodId> = Pin { port: GPIOD, index: 13, id: Pd13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd13Id {}
pub type Pd13 = Pin<Pd13Id, GpiodId>;
pub const PD14: Pin<Pd14Id, GpiodId> = Pin { port: GPIOD, index: 14, id: Pd14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd14Id {}
pub type Pd14 = Pin<Pd14Id, GpiodId>;
pub const PD15: Pin<Pd15Id, GpiodId> = Pin { port: GPIOD, index: 15, id: Pd15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pd15Id {}
pub type Pd15 = Pin<Pd15Id, GpiodId>;
pub const PE0: Pin<Pe0Id, GpioeId> = Pin { port: GPIOE, index: 0, id: Pe0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe0Id {}
pub type Pe0 = Pin<Pe0Id, GpioeId>;
pub const PE1: Pin<Pe1Id, GpioeId> = Pin { port: GPIOE, index: 1, id: Pe1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe1Id {}
pub type Pe1 = Pin<Pe1Id, GpioeId>;
pub const PE2: Pin<Pe2Id, GpioeId> = Pin { port: GPIOE, index: 2, id: Pe2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe2Id {}
pub type Pe2 = Pin<Pe2Id, GpioeId>;
pub const PE3: Pin<Pe3Id, GpioeId> = Pin { port: GPIOE, index: 3, id: Pe3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe3Id {}
pub type Pe3 = Pin<Pe3Id, GpioeId>;
pub const PE4: Pin<Pe4Id, GpioeId> = Pin { port: GPIOE, index: 4, id: Pe4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe4Id {}
pub type Pe4 = Pin<Pe4Id, GpioeId>;
pub const PE5: Pin<Pe5Id, GpioeId> = Pin { port: GPIOE, index: 5, id: Pe5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe5Id {}
pub type Pe5 = Pin<Pe5Id, GpioeId>;
pub const PE6: Pin<Pe6Id, GpioeId> = Pin { port: GPIOE, index: 6, id: Pe6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe6Id {}
pub type Pe6 = Pin<Pe6Id, GpioeId>;
pub const PE7: Pin<Pe7Id, GpioeId> = Pin { port: GPIOE, index: 7, id: Pe7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe7Id {}
pub type Pe7 = Pin<Pe7Id, GpioeId>;
pub const PE8: Pin<Pe8Id, GpioeId> = Pin { port: GPIOE, index: 8, id: Pe8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe8Id {}
pub type Pe8 = Pin<Pe8Id, GpioeId>;
pub const PE9: Pin<Pe9Id, GpioeId> = Pin { port: GPIOE, index: 9, id: Pe9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe9Id {}
pub type Pe9 = Pin<Pe9Id, GpioeId>;
pub const PE10: Pin<Pe10Id, GpioeId> = Pin { port: GPIOE, index: 10, id: Pe10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe10Id {}
pub type Pe10 = Pin<Pe10Id, GpioeId>;
pub const PE11: Pin<Pe11Id, GpioeId> = Pin { port: GPIOE, index: 11, id: Pe11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe11Id {}
pub type Pe11 = Pin<Pe11Id, GpioeId>;
pub const PE12: Pin<Pe12Id, GpioeId> = Pin { port: GPIOE, index: 12, id: Pe12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe12Id {}
pub type Pe12 = Pin<Pe12Id, GpioeId>;
pub const PE13: Pin<Pe13Id, GpioeId> = Pin { port: GPIOE, index: 13, id: Pe13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe13Id {}
pub type Pe13 = Pin<Pe13Id, GpioeId>;
pub const PE14: Pin<Pe14Id, GpioeId> = Pin { port: GPIOE, index: 14, id: Pe14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe14Id {}
pub type Pe14 = Pin<Pe14Id, GpioeId>;
pub const PE15: Pin<Pe15Id, GpioeId> = Pin { port: GPIOE, index: 15, id: Pe15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pe15Id {}
pub type Pe15 = Pin<Pe15Id, GpioeId>;
pub const PF0: Pin<Pf0Id, GpiofId> = Pin { port: GPIOF, index: 0, id: Pf0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf0Id {}
pub type Pf0 = Pin<Pf0Id, GpiofId>;
pub const PF1: Pin<Pf1Id, GpiofId> = Pin { port: GPIOF, index: 1, id: Pf1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf1Id {}
pub type Pf1 = Pin<Pf1Id, GpiofId>;
pub const PF2: Pin<Pf2Id, GpiofId> = Pin { port: GPIOF, index: 2, id: Pf2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf2Id {}
pub type Pf2 = Pin<Pf2Id, GpiofId>;
pub const PF3: Pin<Pf3Id, GpiofId> = Pin { port: GPIOF, index: 3, id: Pf3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf3Id {}
pub type Pf3 = Pin<Pf3Id, GpiofId>;
pub const PF4: Pin<Pf4Id, GpiofId> = Pin { port: GPIOF, index: 4, id: Pf4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf4Id {}
pub type Pf4 = Pin<Pf4Id, GpiofId>;
pub const PF5: Pin<Pf5Id, GpiofId> = Pin { port: GPIOF, index: 5, id: Pf5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf5Id {}
pub type Pf5 = Pin<Pf5Id, GpiofId>;
pub const PF6: Pin<Pf6Id, GpiofId> = Pin { port: GPIOF, index: 6, id: Pf6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf6Id {}
pub type Pf6 = Pin<Pf6Id, GpiofId>;
pub const PF7: Pin<Pf7Id, GpiofId> = Pin { port: GPIOF, index: 7, id: Pf7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf7Id {}
pub type Pf7 = Pin<Pf7Id, GpiofId>;
pub const PF8: Pin<Pf8Id, GpiofId> = Pin { port: GPIOF, index: 8, id: Pf8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf8Id {}
pub type Pf8 = Pin<Pf8Id, GpiofId>;
pub const PF9: Pin<Pf9Id, GpiofId> = Pin { port: GPIOF, index: 9, id: Pf9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf9Id {}
pub type Pf9 = Pin<Pf9Id, GpiofId>;
pub const PF10: Pin<Pf10Id, GpiofId> = Pin { port: GPIOF, index: 10, id: Pf10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf10Id {}
pub type Pf10 = Pin<Pf10Id, GpiofId>;
pub const PF11: Pin<Pf11Id, GpiofId> = Pin { port: GPIOF, index: 11, id: Pf11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf11Id {}
pub type Pf11 = Pin<Pf11Id, GpiofId>;
pub const PF12: Pin<Pf12Id, GpiofId> = Pin { port: GPIOF, index: 12, id: Pf12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf12Id {}
pub type Pf12 = Pin<Pf12Id, GpiofId>;
pub const PF13: Pin<Pf13Id, GpiofId> = Pin { port: GPIOF, index: 13, id: Pf13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf13Id {}
pub type Pf13 = Pin<Pf13Id, GpiofId>;
pub const PF14: Pin<Pf14Id, GpiofId> = Pin { port: GPIOF, index: 14, id: Pf14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf14Id {}
pub type Pf14 = Pin<Pf14Id, GpiofId>;
pub const PF15: Pin<Pf15Id, GpiofId> = Pin { port: GPIOF, index: 15, id: Pf15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pf15Id {}
pub type Pf15 = Pin<Pf15Id, GpiofId>;
pub const PG0: Pin<Pg0Id, GpiogId> = Pin { port: GPIOG, index: 0, id: Pg0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg0Id {}
pub type Pg0 = Pin<Pg0Id, GpiogId>;
pub const PG1: Pin<Pg1Id, GpiogId> = Pin { port: GPIOG, index: 1, id: Pg1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg1Id {}
pub type Pg1 = Pin<Pg1Id, GpiogId>;
pub const PG2: Pin<Pg2Id, GpiogId> = Pin { port: GPIOG, index: 2, id: Pg2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg2Id {}
pub type Pg2 = Pin<Pg2Id, GpiogId>;
pub const PG3: Pin<Pg3Id, GpiogId> = Pin { port: GPIOG, index: 3, id: Pg3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg3Id {}
pub type Pg3 = Pin<Pg3Id, GpiogId>;
pub const PG4: Pin<Pg4Id, GpiogId> = Pin { port: GPIOG, index: 4, id: Pg4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg4Id {}
pub type Pg4 = Pin<Pg4Id, GpiogId>;
pub const PG5: Pin<Pg5Id, GpiogId> = Pin { port: GPIOG, index: 5, id: Pg5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg5Id {}
pub type Pg5 = Pin<Pg5Id, GpiogId>;
pub const PG6: Pin<Pg6Id, GpiogId> = Pin { port: GPIOG, index: 6, id: Pg6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg6Id {}
pub type Pg6 = Pin<Pg6Id, GpiogId>;
pub const PG7: Pin<Pg7Id, GpiogId> = Pin { port: GPIOG, index: 7, id: Pg7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg7Id {}
pub type Pg7 = Pin<Pg7Id, GpiogId>;
pub const PG8: Pin<Pg8Id, GpiogId> = Pin { port: GPIOG, index: 8, id: Pg8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg8Id {}
pub type Pg8 = Pin<Pg8Id, GpiogId>;
pub const PG9: Pin<Pg9Id, GpiogId> = Pin { port: GPIOG, index: 9, id: Pg9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg9Id {}
pub type Pg9 = Pin<Pg9Id, GpiogId>;
pub const PG10: Pin<Pg10Id, GpiogId> = Pin { port: GPIOG, index: 10, id: Pg10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg10Id {}
pub type Pg10 = Pin<Pg10Id, GpiogId>;
pub const PG11: Pin<Pg11Id, GpiogId> = Pin { port: GPIOG, index: 11, id: Pg11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg11Id {}
pub type Pg11 = Pin<Pg11Id, GpiogId>;
pub const PG12: Pin<Pg12Id, GpiogId> = Pin { port: GPIOG, index: 12, id: Pg12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg12Id {}
pub type Pg12 = Pin<Pg12Id, GpiogId>;
pub const PG13: Pin<Pg13Id, GpiogId> = Pin { port: GPIOG, index: 13, id: Pg13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg13Id {}
pub type Pg13 = Pin<Pg13Id, GpiogId>;
pub const PG14: Pin<Pg14Id, GpiogId> = Pin { port: GPIOG, index: 14, id: Pg14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg14Id {}
pub type Pg14 = Pin<Pg14Id, GpiogId>;
pub const PG15: Pin<Pg15Id, GpiogId> = Pin { port: GPIOG, index: 15, id: Pg15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pg15Id {}
pub type Pg15 = Pin<Pg15Id, GpiogId>;
