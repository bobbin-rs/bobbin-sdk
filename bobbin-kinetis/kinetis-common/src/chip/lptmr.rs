#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPTMR Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the CSR register."]
  #[inline] pub fn csr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CSR register."]
  #[inline] pub fn csr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CSR register."]
  #[inline] pub fn csr(&self) -> Csr { 
     unsafe {
        Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CSR register."]
  #[inline] pub fn set_csr(&self, value: Csr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CSR register."]
  #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

#[doc="Get the *const pointer for the PSR register."]
  #[inline] pub fn psr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PSR register."]
  #[inline] pub fn psr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the PSR register."]
  #[inline] pub fn psr(&self) -> Psr { 
     unsafe {
        Psr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the PSR register."]
  #[inline] pub fn set_psr(&self, value: Psr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PSR register."]
  #[inline] pub fn with_psr<F: FnOnce(Psr) -> Psr>(&self, f: F) -> &Self {
     let tmp = self.psr();
     self.set_psr(f(tmp))
  }

#[doc="Get the *const pointer for the CMR register."]
  #[inline] pub fn cmr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CMR register."]
  #[inline] pub fn cmr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CMR register."]
  #[inline] pub fn cmr(&self) -> Cmr { 
     unsafe {
        Cmr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CMR register."]
  #[inline] pub fn set_cmr(&self, value: Cmr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CMR register."]
  #[inline] pub fn with_cmr<F: FnOnce(Cmr) -> Cmr>(&self, f: F) -> &Self {
     let tmp = self.cmr();
     self.set_cmr(f(tmp))
  }

#[doc="Get the *const pointer for the CNR register."]
  #[inline] pub fn cnr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the CNR register."]
  #[inline] pub fn cnr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the CNR register."]
  #[inline] pub fn cnr(&self) -> Cnr { 
     unsafe {
        Cnr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the CNR register."]
  #[inline] pub fn set_cnr(&self, value: Cnr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CNR register."]
  #[inline] pub fn with_cnr<F: FnOnce(Cnr) -> Cnr>(&self, f: F) -> &Self {
     let tmp = self.cnr();
     self.set_cnr(f(tmp))
  }

}

#[doc="Low Power Timer Control Status Register"]
#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
#[doc="Timer Enable"]
  #[inline] pub fn ten(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Timer Enable"]
  #[inline] pub fn set_ten<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Timer Mode Select"]
  #[inline] pub fn tms(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Timer Mode Select"]
  #[inline] pub fn set_tms<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Timer Free-Running Counter"]
  #[inline] pub fn tfc(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Timer Free-Running Counter"]
  #[inline] pub fn set_tfc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Timer Pin Polarity"]
  #[inline] pub fn tpp(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Timer Pin Polarity"]
  #[inline] pub fn set_tpp<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Timer Pin Select"]
  #[inline] pub fn tps(&self) -> bits::B2 {
     (((self.0 as u32) >> 4) & 0x3).into() // [5:4]
  }
#[doc="Timer Pin Select"]
  #[inline] pub fn set_tps<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Timer Interrupt Enable"]
  #[inline] pub fn tie(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="Timer Interrupt Enable"]
  #[inline] pub fn set_tie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Timer Compare Flag"]
  #[inline] pub fn tcf(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Timer Compare Flag"]
  #[inline] pub fn set_tcf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ten() != 0 { try!(write!(f, " ten"))}
      if self.tms() != 0 { try!(write!(f, " tms"))}
      if self.tfc() != 0 { try!(write!(f, " tfc"))}
      if self.tpp() != 0 { try!(write!(f, " tpp"))}
      if self.tps() != 0 { try!(write!(f, " tps=0x{:x}", self.tps()))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.tcf() != 0 { try!(write!(f, " tcf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Low Power Timer Prescale Register"]
#[derive(PartialEq, Eq)]
pub struct Psr(pub u32);
impl Psr {
#[doc="Prescaler Clock Select"]
  #[inline] pub fn pcs(&self) -> bits::B2 {
     (((self.0 as u32) >> 0) & 0x3).into() // [1:0]
  }
#[doc="Prescaler Clock Select"]
  #[inline] pub fn set_pcs<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Prescaler Bypass"]
  #[inline] pub fn pbyp(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Prescaler Bypass"]
  #[inline] pub fn set_pbyp<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Prescale Value"]
  #[inline] pub fn prescale(&self) -> bits::B4 {
     (((self.0 as u32) >> 3) & 0xf).into() // [6:3]
  }
#[doc="Prescale Value"]
  #[inline] pub fn set_prescale<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for Psr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Psr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.pbyp() != 0 { try!(write!(f, " pbyp"))}
      if self.prescale() != 0 { try!(write!(f, " prescale=0x{:x}", self.prescale()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Low Power Timer Compare Register"]
#[derive(PartialEq, Eq)]
pub struct Cmr(pub u32);
impl Cmr {
#[doc="Compare Value"]
  #[inline] pub fn compare(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Compare Value"]
  #[inline] pub fn set_compare<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.compare() != 0 { try!(write!(f, " compare=0x{:x}", self.compare()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Low Power Timer Counter Register"]
#[derive(PartialEq, Eq)]
pub struct Cnr(pub u32);
impl Cnr {
#[doc="Counter Value"]
  #[inline] pub fn counter(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Counter Value"]
  #[inline] pub fn set_counter<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cnr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.counter() != 0 { try!(write!(f, " counter=0x{:x}", self.counter()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
