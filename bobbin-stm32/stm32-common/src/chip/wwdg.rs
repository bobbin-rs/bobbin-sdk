
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WWDG Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the CFR register."]
  #[inline] pub fn cfr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CFR register."]
  #[inline] pub fn cfr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CFR register."]
  #[inline] pub fn cfr(&self) -> Cfr { 
     unsafe {
        Cfr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CFR register."]
  #[inline] pub fn set_cfr(&self, value: Cfr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFR register."]
  #[inline] pub fn with_cfr<F: FnOnce(Cfr) -> Cfr>(&self, f: F) -> &Self {
     let tmp = self.cfr();
     self.set_cfr(f(tmp))
  }

#[doc="Get the *const pointer for the SR register."]
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the SR register."]
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the SR register."]
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the SR register."]
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR register."]
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

}

#[doc="Control register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Activation bit"]
  #[inline] pub fn wdga(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Activation bit"]
  #[inline] pub fn set_wdga(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="7-bit counter (MSB to LSB)"]
  #[inline] pub fn t(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
#[doc="7-bit counter (MSB to LSB)"]
  #[inline] pub fn set_t(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdga() != 0 { try!(write!(f, " wdga"))}
      if self.t() != 0 { try!(write!(f, " t=0x{:x}", self.t()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configuration register"]
#[derive(PartialEq, Eq)]
pub struct Cfr(pub u32);
impl Cfr {
#[doc="Early wakeup interrupt"]
  #[inline] pub fn ewi(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Early wakeup interrupt"]
  #[inline] pub fn set_ewi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Timer base"]
  #[inline] pub fn wdgtb(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x3 // [8:7]
  }
#[doc="Timer base"]
  #[inline] pub fn set_wdgtb(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="7-bit window value"]
  #[inline] pub fn w(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
#[doc="7-bit window value"]
  #[inline] pub fn set_w(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ewi() != 0 { try!(write!(f, " ewi"))}
      if self.wdgtb() != 0 { try!(write!(f, " wdgtb=0x{:x}", self.wdgtb()))}
      if self.w() != 0 { try!(write!(f, " w=0x{:x}", self.w()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status register"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="Early wakeup interrupt flag"]
  #[inline] pub fn ewif(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Early wakeup interrupt flag"]
  #[inline] pub fn set_ewif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ewif() != 0 { try!(write!(f, " ewif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
