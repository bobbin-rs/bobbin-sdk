pub const FTFA: Ftfa = Ftfa(0x00000400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftfa(pub u32);

impl Ftfa {
  pub unsafe fn backkey3(&self) -> Backkey3 { 
     Backkey3(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
  }

  pub unsafe fn backkey2(&self) -> Backkey2 { 
     Backkey2(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
  }

  pub unsafe fn backkey1(&self) -> Backkey1 { 
     Backkey1(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
  }

  pub unsafe fn backkey0(&self) -> Backkey0 { 
     Backkey0(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
  }

  pub unsafe fn backkey7(&self) -> Backkey7 { 
     Backkey7(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
  }

  pub unsafe fn backkey6(&self) -> Backkey6 { 
     Backkey6(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
  }

  pub unsafe fn backkey5(&self) -> Backkey5 { 
     Backkey5(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
  }

  pub unsafe fn backkey4(&self) -> Backkey4 { 
     Backkey4(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
  }

  pub unsafe fn fprot3(&self) -> Fprot3 { 
     Fprot3(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
  }

  pub unsafe fn fprot2(&self) -> Fprot2 { 
     Fprot2(::core::ptr::read_volatile(((self.0 as usize) + 0x9) as *const u8))
  }

  pub unsafe fn fprot1(&self) -> Fprot1 { 
     Fprot1(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
  }

  pub unsafe fn fprot0(&self) -> Fprot0 { 
     Fprot0(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
  }

  pub unsafe fn fsec(&self) -> Fsec { 
     Fsec(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
  }

  pub unsafe fn fopt(&self) -> Fopt { 
     Fopt(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
  }

}

#[derive(PartialEq, Eq)]
pub struct Backkey3(pub u8);

impl Backkey3 {
  pub fn key(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Backkey3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Backkey3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Backkey2(pub u8);

impl Backkey2 {
  pub fn key(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Backkey2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Backkey2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Backkey1(pub u8);

impl Backkey1 {
  pub fn key(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Backkey1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Backkey1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Backkey0(pub u8);

impl Backkey0 {
  pub fn key(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Backkey0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Backkey0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Backkey7(pub u8);

impl Backkey7 {
  pub fn key(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Backkey7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Backkey7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Backkey6(pub u8);

impl Backkey6 {
  pub fn key(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Backkey6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Backkey6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Backkey5(pub u8);

impl Backkey5 {
  pub fn key(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Backkey5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Backkey5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Backkey4(pub u8);

impl Backkey4 {
  pub fn key(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Backkey4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Backkey4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fprot3(pub u8);

impl Fprot3 {
  pub fn prot(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_prot(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Fprot3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fprot3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fprot2(pub u8);

impl Fprot2 {
  pub fn prot(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_prot(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Fprot2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fprot2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fprot1(pub u8);

impl Fprot1 {
  pub fn prot(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_prot(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Fprot1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fprot1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fprot0(pub u8);

impl Fprot0 {
  pub fn prot(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_prot(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Fprot0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fprot0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fsec(pub u8);

impl Fsec {
  pub fn sec(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }
  pub fn set_sec(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fslacc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x3 // [3:2]
  }
  pub fn set_fslacc(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn meen(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x3 // [5:4]
  }
  pub fn set_meen(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn keyen(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x3 // [7:6]
  }
  pub fn set_keyen(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

}

impl ::core::fmt::Display for Fsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sec() != 0 { try!(write!(f, " sec=0x{:x}", self.sec()))}
      if self.fslacc() != 0 { try!(write!(f, " fslacc=0x{:x}", self.fslacc()))}
      if self.meen() != 0 { try!(write!(f, " meen=0x{:x}", self.meen()))}
      if self.keyen() != 0 { try!(write!(f, " keyen=0x{:x}", self.keyen()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Fopt(pub u8);

impl Fopt {
  pub fn lpboot0(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_lpboot0(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn nmi_dis(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_nmi_dis(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn reset_pin_cfg(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_reset_pin_cfg(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn lpboot1(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_lpboot1(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn fast_init(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_fast_init(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}

impl ::core::fmt::Display for Fopt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Fopt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lpboot0() != 0 { try!(write!(f, " lpboot0"))}
      if self.nmi_dis() != 0 { try!(write!(f, " nmi_dis"))}
      if self.reset_pin_cfg() != 0 { try!(write!(f, " reset_pin_cfg"))}
      if self.lpboot1() != 0 { try!(write!(f, " lpboot1"))}
      if self.fast_init() != 0 { try!(write!(f, " fast_init"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

