pub const RCM: Rcm = Rcm(0x4007f000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcm(pub u32);
impl Rcm {
  #[inline] pub fn srs0_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
  #[inline] pub fn srs0_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
  #[inline] pub fn srs0(&self) -> Srs0 { 
     unsafe {
        Srs0(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }

  #[inline] pub fn srs1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
  #[inline] pub fn srs1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
  #[inline] pub fn srs1(&self) -> Srs1 { 
     unsafe {
        Srs1(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }

  #[inline] pub fn rpfc_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
  #[inline] pub fn rpfc_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
  #[inline] pub fn rpfc(&self) -> Rpfc { 
     unsafe {
        Rpfc(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
  #[inline] pub fn set_rpfc(&self, value: Rpfc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_rpfc<F: FnOnce(Rpfc) -> Rpfc>(&self, f: F) -> &Self {
     let tmp = self.rpfc();
     self.set_rpfc(f(tmp))
  }

  #[inline] pub fn rpfw_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
  #[inline] pub fn rpfw_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
  #[inline] pub fn rpfw(&self) -> Rpfw { 
     unsafe {
        Rpfw(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
  #[inline] pub fn set_rpfw(&self, value: Rpfw) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_rpfw<F: FnOnce(Rpfw) -> Rpfw>(&self, f: F) -> &Self {
     let tmp = self.rpfw();
     self.set_rpfw(f(tmp))
  }

  #[inline] pub fn mr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x7) as *const u8
  }
  #[inline] pub fn mr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x7) as *mut u8
  }
  #[inline] pub fn mr(&self) -> Mr { 
     unsafe {
        Mr(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct Srs0(pub u8);
impl Srs0 {
  #[inline] pub fn wakeup(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_wakeup(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn lvd(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_lvd(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn loc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_loc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn lol(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_lol(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn wdog(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_wdog(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pin(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pin(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn por(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_por(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Srs0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srs0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
      if self.lvd() != 0 { try!(write!(f, " lvd"))}
      if self.loc() != 0 { try!(write!(f, " loc"))}
      if self.lol() != 0 { try!(write!(f, " lol"))}
      if self.wdog() != 0 { try!(write!(f, " wdog"))}
      if self.pin() != 0 { try!(write!(f, " pin"))}
      if self.por() != 0 { try!(write!(f, " por"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Srs1(pub u8);
impl Srs1 {
  #[inline] pub fn jtag(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_jtag(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn lockup(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_lockup(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn sw(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_sw(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn mdm_ap(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_mdm_ap(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn ezpt(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_ezpt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn sackerr(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_sackerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Srs1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srs1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jtag() != 0 { try!(write!(f, " jtag"))}
      if self.lockup() != 0 { try!(write!(f, " lockup"))}
      if self.sw() != 0 { try!(write!(f, " sw"))}
      if self.mdm_ap() != 0 { try!(write!(f, " mdm_ap"))}
      if self.ezpt() != 0 { try!(write!(f, " ezpt"))}
      if self.sackerr() != 0 { try!(write!(f, " sackerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rpfc(pub u8);
impl Rpfc {
  #[inline] pub fn rstfltsrw(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_rstfltsrw(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn rstfltss(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_rstfltss(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Rpfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rpfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rstfltsrw() != 0 { try!(write!(f, " rstfltsrw=0x{:x}", self.rstfltsrw()))}
      if self.rstfltss() != 0 { try!(write!(f, " rstfltss"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rpfw(pub u8);
impl Rpfw {
  #[inline] pub fn rstfltsel(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_rstfltsel(mut self, value: u8) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rpfw {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rpfw {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rstfltsel() != 0 { try!(write!(f, " rstfltsel=0x{:x}", self.rstfltsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Mr(pub u8);
impl Mr {
  #[inline] pub fn ezp_ms(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_ezp_ms(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Mr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ezp_ms() != 0 { try!(write!(f, " ezp_ms"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

