pub const DAC: Dac = Dac(0x40007400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dac(pub u32);

impl Dac {
  pub unsafe fn cr(&self) -> Cr { 
     Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_cr(&mut self, value: Cr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_cr<F: FnOnce(Cr) -> Cr>(&mut self, f: F) {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  pub unsafe fn set_swtrigr(&mut self, value: Swtrigr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }

  pub unsafe fn dhr12r1(&self) -> Dhr12r1 { 
     Dhr12r1(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_dhr12r1(&mut self, value: Dhr12r1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr12r1<F: FnOnce(Dhr12r1) -> Dhr12r1>(&mut self, f: F) {
     let tmp = self.dhr12r1();
     self.set_dhr12r1(f(tmp))
  }

  pub unsafe fn dhr12l1(&self) -> Dhr12l1 { 
     Dhr12l1(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_dhr12l1(&mut self, value: Dhr12l1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr12l1<F: FnOnce(Dhr12l1) -> Dhr12l1>(&mut self, f: F) {
     let tmp = self.dhr12l1();
     self.set_dhr12l1(f(tmp))
  }

  pub unsafe fn dhr8r1(&self) -> Dhr8r1 { 
     Dhr8r1(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_dhr8r1(&mut self, value: Dhr8r1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr8r1<F: FnOnce(Dhr8r1) -> Dhr8r1>(&mut self, f: F) {
     let tmp = self.dhr8r1();
     self.set_dhr8r1(f(tmp))
  }

  pub unsafe fn dhr12r2(&self) -> Dhr12r2 { 
     Dhr12r2(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_dhr12r2(&mut self, value: Dhr12r2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr12r2<F: FnOnce(Dhr12r2) -> Dhr12r2>(&mut self, f: F) {
     let tmp = self.dhr12r2();
     self.set_dhr12r2(f(tmp))
  }

  pub unsafe fn dhr12l2(&self) -> Dhr12l2 { 
     Dhr12l2(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_dhr12l2(&mut self, value: Dhr12l2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr12l2<F: FnOnce(Dhr12l2) -> Dhr12l2>(&mut self, f: F) {
     let tmp = self.dhr12l2();
     self.set_dhr12l2(f(tmp))
  }

  pub unsafe fn dhr8r2(&self) -> Dhr8r2 { 
     Dhr8r2(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_dhr8r2(&mut self, value: Dhr8r2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr8r2<F: FnOnce(Dhr8r2) -> Dhr8r2>(&mut self, f: F) {
     let tmp = self.dhr8r2();
     self.set_dhr8r2(f(tmp))
  }

  pub unsafe fn dhr12rd(&self) -> Dhr12rd { 
     Dhr12rd(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_dhr12rd(&mut self, value: Dhr12rd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr12rd<F: FnOnce(Dhr12rd) -> Dhr12rd>(&mut self, f: F) {
     let tmp = self.dhr12rd();
     self.set_dhr12rd(f(tmp))
  }

  pub unsafe fn dhr12ld(&self) -> Dhr12ld { 
     Dhr12ld(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_dhr12ld(&mut self, value: Dhr12ld) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr12ld<F: FnOnce(Dhr12ld) -> Dhr12ld>(&mut self, f: F) {
     let tmp = self.dhr12ld();
     self.set_dhr12ld(f(tmp))
  }

  pub unsafe fn dhr8rd(&self) -> Dhr8rd { 
     Dhr8rd(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }
  pub unsafe fn set_dhr8rd(&mut self, value: Dhr8rd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }
  pub unsafe fn with_dhr8rd<F: FnOnce(Dhr8rd) -> Dhr8rd>(&mut self, f: F) {
     let tmp = self.dhr8rd();
     self.set_dhr8rd(f(tmp))
  }

  pub unsafe fn dor1(&self) -> Dor1 { 
     Dor1(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
  }

  pub unsafe fn dor2(&self) -> Dor2 { 
     Dor2(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
  }

  pub unsafe fn sr(&self) -> Sr { 
     Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }
  pub unsafe fn set_sr(&mut self, value: Sr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
  }
  pub unsafe fn with_sr<F: FnOnce(Sr) -> Sr>(&mut self, f: F) {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);

impl Cr {
  pub fn dmaudrie2(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_dmaudrie2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn dmaen2(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_dmaen2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn mamp2(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_mamp2(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn wave2(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
  pub fn set_wave2(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn tsel2(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x7 // [21:19]
  }
  pub fn set_tsel2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn ten2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_ten2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn boff2(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_boff2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn en2(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_en2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn dmaudrie1(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_dmaudrie1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn dmaen1(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_dmaen1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mamp1(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_mamp1(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn wave1(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  pub fn set_wave1(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tsel1(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x7 // [5:3]
  }
  pub fn set_tsel1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ten1(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_ten1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn boff1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_boff1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn en1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_en1(mut self, value: u32) -> Self {
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
      if self.dmaudrie2() != 0 { try!(write!(f, " dmaudrie2"))}
      if self.dmaen2() != 0 { try!(write!(f, " dmaen2"))}
      if self.mamp2() != 0 { try!(write!(f, " mamp2=0x{:x}", self.mamp2()))}
      if self.wave2() != 0 { try!(write!(f, " wave2=0x{:x}", self.wave2()))}
      if self.tsel2() != 0 { try!(write!(f, " tsel2=0x{:x}", self.tsel2()))}
      if self.ten2() != 0 { try!(write!(f, " ten2"))}
      if self.boff2() != 0 { try!(write!(f, " boff2"))}
      if self.en2() != 0 { try!(write!(f, " en2"))}
      if self.dmaudrie1() != 0 { try!(write!(f, " dmaudrie1"))}
      if self.dmaen1() != 0 { try!(write!(f, " dmaen1"))}
      if self.mamp1() != 0 { try!(write!(f, " mamp1=0x{:x}", self.mamp1()))}
      if self.wave1() != 0 { try!(write!(f, " wave1=0x{:x}", self.wave1()))}
      if self.tsel1() != 0 { try!(write!(f, " tsel1=0x{:x}", self.tsel1()))}
      if self.ten1() != 0 { try!(write!(f, " ten1"))}
      if self.boff1() != 0 { try!(write!(f, " boff1"))}
      if self.en1() != 0 { try!(write!(f, " en1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Swtrigr(pub u32);

impl Swtrigr {
  pub fn swtrig2(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_swtrig2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn swtrig1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_swtrig1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Swtrigr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Swtrigr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swtrig2() != 0 { try!(write!(f, " swtrig2"))}
      if self.swtrig1() != 0 { try!(write!(f, " swtrig1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr12r1(pub u32);

impl Dhr12r1 {
  pub fn dacc1dhr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_dacc1dhr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dhr12r1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr12r1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr12l1(pub u32);

impl Dhr12l1 {
  pub fn dacc1dhr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xfff // [15:4]
  }
  pub fn set_dacc1dhr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Dhr12l1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr12l1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr8r1(pub u32);

impl Dhr8r1 {
  pub fn dacc1dhr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_dacc1dhr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dhr8r1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr8r1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr12r2(pub u32);

impl Dhr12r2 {
  pub fn dacc2dhr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_dacc2dhr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dhr12r2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr12r2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr12l2(pub u32);

impl Dhr12l2 {
  pub fn dacc2dhr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xfff // [15:4]
  }
  pub fn set_dacc2dhr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Dhr12l2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr12l2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr8r2(pub u32);

impl Dhr8r2 {
  pub fn dacc2dhr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_dacc2dhr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dhr8r2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr8r2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr12rd(pub u32);

impl Dhr12rd {
  pub fn dacc2dhr(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xfff // [27:16]
  }
  pub fn set_dacc2dhr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn dacc1dhr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_dacc1dhr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dhr12rd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr12rd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr12ld(pub u32);

impl Dhr12ld {
  pub fn dacc2dhr(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xfff // [31:20]
  }
  pub fn set_dacc2dhr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 20);
     self.0 |= value << 20;
     self
  }

  pub fn dacc1dhr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xfff // [15:4]
  }
  pub fn set_dacc1dhr(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Dhr12ld {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr12ld {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dhr8rd(pub u32);

impl Dhr8rd {
  pub fn dacc2dhr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_dacc2dhr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dacc1dhr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_dacc1dhr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dhr8rd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dhr8rd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dor1(pub u32);

impl Dor1 {
  pub fn dacc1dor(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_dacc1dor(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dor1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dor1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc1dor() != 0 { try!(write!(f, " dacc1dor=0x{:x}", self.dacc1dor()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dor2(pub u32);

impl Dor2 {
  pub fn dacc2dor(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_dacc2dor(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dor2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dor2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dor() != 0 { try!(write!(f, " dacc2dor=0x{:x}", self.dacc2dor()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);

impl Sr {
  pub fn dmaudr2(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_dmaudr2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn dmaudr1(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_dmaudr1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
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
      if self.dmaudr2() != 0 { try!(write!(f, " dmaudr2"))}
      if self.dmaudr1() != 0 { try!(write!(f, " dmaudr1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

