pub const CRC: Crc = Crc(0x40023000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Crc(pub u32);
impl Crc {
  pub fn dr(&self) -> Dr { 
     unsafe {
       Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  pub fn set_dr(&self, value: Dr) -> &Crc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Crc {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

  pub fn idr(&self) -> Idr { 
     unsafe {
       Idr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  pub fn set_idr(&self, value: Idr) -> &Crc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  pub fn with_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Crc {
     let tmp = self.idr();
     self.set_idr(f(tmp))
  }

  pub fn cr(&self) -> Cr { 
     unsafe {
       Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  pub fn set_cr(&self, value: Cr) -> &Crc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Crc {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  pub fn init(&self) -> Init { 
     unsafe {
       Init(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  pub fn set_init(&self, value: Init) -> &Crc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  pub fn with_init<F: FnOnce(Init) -> Init>(&self, f: F) -> &Crc {
     let tmp = self.init();
     self.set_init(f(tmp))
  }

  pub fn pol(&self) -> Pol { 
     unsafe {
       Pol(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  pub fn set_pol(&self, value: Pol) -> &Crc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  pub fn with_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Crc {
     let tmp = self.pol();
     self.set_pol(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
  pub fn dr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_dr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
  pub fn idr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_idr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
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
      if self.idr() != 0 { try!(write!(f, " idr=0x{:x}", self.idr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
  pub fn rev_out(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_rev_out(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rev_in(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  pub fn set_rev_in(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn polysize(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  pub fn set_polysize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn _reset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_reset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
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
      if self.rev_out() != 0 { try!(write!(f, " rev_out"))}
      if self.rev_in() != 0 { try!(write!(f, " rev_in=0x{:x}", self.rev_in()))}
      if self.polysize() != 0 { try!(write!(f, " polysize=0x{:x}", self.polysize()))}
      if self._reset() != 0 { try!(write!(f, " _reset"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Init(pub u32);
impl Init {
  pub fn init(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_init(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Init {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Init {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pol(pub u32);
impl Pol {
  pub fn pol(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_pol(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
