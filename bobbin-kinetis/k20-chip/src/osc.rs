pub const OSC: Osc = Osc(0x40065000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Osc(pub u32);
impl Osc {
  #[inline] pub fn cr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
  #[inline] pub fn cr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u8);
impl Cr {
  #[inline] pub fn sc16p(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_sc16p(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn sc8p(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_sc8p(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn sc4p(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_sc4p(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn sc2p(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_sc2p(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn erefsten(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_erefsten(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn erclken(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_erclken(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.sc16p() != 0 { try!(write!(f, " sc16p"))}
      if self.sc8p() != 0 { try!(write!(f, " sc8p"))}
      if self.sc4p() != 0 { try!(write!(f, " sc4p"))}
      if self.sc2p() != 0 { try!(write!(f, " sc2p"))}
      if self.erefsten() != 0 { try!(write!(f, " erefsten"))}
      if self.erclken() != 0 { try!(write!(f, " erclken"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

