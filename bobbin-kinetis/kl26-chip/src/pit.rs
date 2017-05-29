pub const PIT: Pit = Pit(0x40037000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pit(pub u32);

impl Pit {
  pub unsafe fn mcr(&self) -> Mcr { 
     Mcr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_mcr(&mut self, value: Mcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&mut self, f: F) {
     let tmp = self.mcr();
     self.set_mcr(f(tmp))
  }

  pub unsafe fn ldval(&self, index: usize) -> Ldval { 
     assert!(index < 4);
     Ldval(::core::ptr::read_volatile(((self.0 as usize) + 0x100 + (index << 4)) as *const u32))
  }
  pub unsafe fn set_ldval(&mut self, index: usize, value: Ldval) {
     assert!(index < 4);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index << 4)) as *mut u32, value.0);
  }
  pub unsafe fn with_ldval<F: FnOnce(Ldval) -> Ldval>(&mut self, index: usize, f: F) {
     let tmp = self.ldval(index);
     self.set_ldval(index, f(tmp))
  }

  pub unsafe fn cval(&self, index: usize) -> Cval { 
     assert!(index < 4);
     Cval(::core::ptr::read_volatile(((self.0 as usize) + 0x104 + (index << 4)) as *const u32))
  }

  pub unsafe fn tctrl(&self, index: usize) -> Tctrl { 
     assert!(index < 4);
     Tctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x108 + (index << 4)) as *const u32))
  }
  pub unsafe fn set_tctrl(&mut self, index: usize, value: Tctrl) {
     assert!(index < 4);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x108 + (index << 4)) as *mut u32, value.0);
  }
  pub unsafe fn with_tctrl<F: FnOnce(Tctrl) -> Tctrl>(&mut self, index: usize, f: F) {
     let tmp = self.tctrl(index);
     self.set_tctrl(index, f(tmp))
  }

  pub unsafe fn tflg(&self, index: usize) -> Tflg { 
     assert!(index < 4);
     Tflg(::core::ptr::read_volatile(((self.0 as usize) + 0x10c + (index << 4)) as *const u32))
  }
  pub unsafe fn set_tflg(&mut self, index: usize, value: Tflg) {
     assert!(index < 4);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10c + (index << 4)) as *mut u32, value.0);
  }
  pub unsafe fn with_tflg<F: FnOnce(Tflg) -> Tflg>(&mut self, index: usize, f: F) {
     let tmp = self.tflg(index);
     self.set_tflg(index, f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Mcr(pub u32);

impl Mcr {
  pub fn frz(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_frz(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mdis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_mdis(mut self, value: u32) -> Self {
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
  pub fn tsv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tsv(mut self, value: u32) -> Self {
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
  pub fn tvl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tvl(mut self, value: u32) -> Self {
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
  pub fn ten(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn chn(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_chn(mut self, value: u32) -> Self {
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
  pub fn tif(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tif(mut self, value: u32) -> Self {
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

