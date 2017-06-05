
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
  #[inline] pub fn mcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn mcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn mcr(&self) -> Mcr { 
     unsafe {
        Mcr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_mcr(&self, value: Mcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
     let tmp = self.mcr();
     self.set_mcr(f(tmp))
  }

  #[inline] pub fn ldval_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x100 + (index << 4)) as *const u32
  }
  #[inline] pub fn ldval_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x100 + (index << 4)) as *mut u32
  }
  #[inline] pub fn ldval(&self, index: usize) -> Ldval { 
     assert!(index < 4);
     unsafe {
        Ldval(::core::ptr::read_volatile(((self.0 as usize) + 0x100 + (index << 4)) as *const u32))
     }
  }
  #[inline] pub fn set_ldval(&self, index: usize, value: Ldval) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index << 4)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ldval<F: FnOnce(Ldval) -> Ldval>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ldval(index);
     self.set_ldval(index, f(tmp))
  }

  #[inline] pub fn cval_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x104 + (index << 4)) as *const u32
  }
  #[inline] pub fn cval_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x104 + (index << 4)) as *mut u32
  }
  #[inline] pub fn cval(&self, index: usize) -> Cval { 
     assert!(index < 4);
     unsafe {
        Cval(::core::ptr::read_volatile(((self.0 as usize) + 0x104 + (index << 4)) as *const u32))
     }
  }

  #[inline] pub fn tctrl_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x108 + (index << 4)) as *const u32
  }
  #[inline] pub fn tctrl_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x108 + (index << 4)) as *mut u32
  }
  #[inline] pub fn tctrl(&self, index: usize) -> Tctrl { 
     assert!(index < 4);
     unsafe {
        Tctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x108 + (index << 4)) as *const u32))
     }
  }
  #[inline] pub fn set_tctrl(&self, index: usize, value: Tctrl) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x108 + (index << 4)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tctrl<F: FnOnce(Tctrl) -> Tctrl>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tctrl(index);
     self.set_tctrl(index, f(tmp))
  }

  #[inline] pub fn tflg_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x10c + (index << 4)) as *const u32
  }
  #[inline] pub fn tflg_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x10c + (index << 4)) as *mut u32
  }
  #[inline] pub fn tflg(&self, index: usize) -> Tflg { 
     assert!(index < 4);
     unsafe {
        Tflg(::core::ptr::read_volatile(((self.0 as usize) + 0x10c + (index << 4)) as *const u32))
     }
  }
  #[inline] pub fn set_tflg(&self, index: usize, value: Tflg) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10c + (index << 4)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tflg<F: FnOnce(Tflg) -> Tflg>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tflg(index);
     self.set_tflg(index, f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
  #[inline] pub fn frz(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_frz(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn mdis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_mdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.frz() != 0 { try!(write!(f, " frz"))}
      if self.mdis() != 0 { try!(write!(f, " mdis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ldval(pub u32);
impl Ldval {
  #[inline] pub fn tsv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_tsv(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ldval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ldval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cval(pub u32);
impl Cval {
  #[inline] pub fn tvl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_tvl(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tctrl(pub u32);
impl Tctrl {
  #[inline] pub fn ten(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn chn(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_chn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Tctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ten() != 0 { try!(write!(f, " ten"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.chn() != 0 { try!(write!(f, " chn"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Tflg(pub u32);
impl Tflg {
  #[inline] pub fn tif(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_tif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tflg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tflg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tif() != 0 { try!(write!(f, " tif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
