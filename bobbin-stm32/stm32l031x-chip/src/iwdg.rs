pub const IWDG: Iwdg = Iwdg(0x40003000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Iwdg(pub u32);

impl Iwdg {
  pub fn set_kr(&self, value: Kr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }  }

  pub fn pr(&self) -> Pr { 
     unsafe {       Pr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }  }
  pub fn set_pr(&self, value: Pr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }  }
  pub fn with_pr<F: FnOnce(Pr) -> Pr>(&self, f: F) {
     let tmp = self.pr();
     self.set_pr(f(tmp))
  }

  pub fn rlr(&self) -> Rlr { 
     unsafe {       Rlr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }  }
  pub fn set_rlr(&self, value: Rlr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }  }
  pub fn with_rlr<F: FnOnce(Rlr) -> Rlr>(&self, f: F) {
     let tmp = self.rlr();
     self.set_rlr(f(tmp))
  }

  pub fn sr(&self) -> Sr { 
     unsafe {       Sr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }  }

  pub fn winr(&self) -> Winr { 
     unsafe {       Winr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }  }
  pub fn set_winr(&self, value: Winr) {
     unsafe {       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }  }
  pub fn with_winr<F: FnOnce(Winr) -> Winr>(&self, f: F) {
     let tmp = self.winr();
     self.set_winr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Kr(pub u32);

impl Kr {
  pub fn key(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_key(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Kr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Kr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pr(pub u32);

impl Pr {
  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Pr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pr() != 0 { try!(write!(f, " pr=0x{:x}", self.pr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rlr(pub u32);

impl Rlr {
  pub fn rl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_rl(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rl() != 0 { try!(write!(f, " rl=0x{:x}", self.rl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);

impl Sr {
  pub fn wvu(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_wvu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn rvu(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_rvu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn pvu(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_pvu(mut self, value: u32) -> Self {
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
      if self.wvu() != 0 { try!(write!(f, " wvu"))}
      if self.rvu() != 0 { try!(write!(f, " rvu"))}
      if self.pvu() != 0 { try!(write!(f, " pvu"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Winr(pub u32);

impl Winr {
  pub fn win(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_win(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Winr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Winr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.win() != 0 { try!(write!(f, " win=0x{:x}", self.win()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

