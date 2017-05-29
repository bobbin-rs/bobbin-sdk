pub const ETHERNET_PTP: EthernetPtp = EthernetPtp(0x40028700);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetPtp(pub u32);

impl EthernetPtp {
  pub unsafe fn ptptscr(&self) -> Ptptscr { 
     Ptptscr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_ptptscr(&mut self, value: Ptptscr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_ptptscr<F: FnOnce(Ptptscr) -> Ptptscr>(&mut self, f: F) {
     let tmp = self.ptptscr();
     self.set_ptptscr(f(tmp))
  }

  pub unsafe fn ptpssir(&self) -> Ptpssir { 
     Ptpssir(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_ptpssir(&mut self, value: Ptpssir) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_ptpssir<F: FnOnce(Ptpssir) -> Ptpssir>(&mut self, f: F) {
     let tmp = self.ptpssir();
     self.set_ptpssir(f(tmp))
  }

  pub unsafe fn ptptshr(&self) -> Ptptshr { 
     Ptptshr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }

  pub unsafe fn ptptslr(&self) -> Ptptslr { 
     Ptptslr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }

  pub unsafe fn ptptshur(&self) -> Ptptshur { 
     Ptptshur(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_ptptshur(&mut self, value: Ptptshur) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_ptptshur<F: FnOnce(Ptptshur) -> Ptptshur>(&mut self, f: F) {
     let tmp = self.ptptshur();
     self.set_ptptshur(f(tmp))
  }

  pub unsafe fn ptptslur(&self) -> Ptptslur { 
     Ptptslur(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_ptptslur(&mut self, value: Ptptslur) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_ptptslur<F: FnOnce(Ptptslur) -> Ptptslur>(&mut self, f: F) {
     let tmp = self.ptptslur();
     self.set_ptptslur(f(tmp))
  }

  pub unsafe fn ptptsar(&self) -> Ptptsar { 
     Ptptsar(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_ptptsar(&mut self, value: Ptptsar) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_ptptsar<F: FnOnce(Ptptsar) -> Ptptsar>(&mut self, f: F) {
     let tmp = self.ptptsar();
     self.set_ptptsar(f(tmp))
  }

  pub unsafe fn ptptthr(&self) -> Ptptthr { 
     Ptptthr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_ptptthr(&mut self, value: Ptptthr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_ptptthr<F: FnOnce(Ptptthr) -> Ptptthr>(&mut self, f: F) {
     let tmp = self.ptptthr();
     self.set_ptptthr(f(tmp))
  }

  pub unsafe fn ptpttlr(&self) -> Ptpttlr { 
     Ptpttlr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_ptpttlr(&mut self, value: Ptpttlr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_ptpttlr<F: FnOnce(Ptpttlr) -> Ptpttlr>(&mut self, f: F) {
     let tmp = self.ptpttlr();
     self.set_ptpttlr(f(tmp))
  }

  pub unsafe fn ptptssr(&self) -> Ptptssr { 
     Ptptssr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }

  pub unsafe fn ptpppscr(&self) -> Ptpppscr { 
     Ptpppscr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
  }

}

#[derive(PartialEq, Eq)]
pub struct Ptptscr(pub u32);

impl Ptptscr {
  pub fn tse(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tsfcu(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tsfcu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tsptppsv2e(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_tsptppsv2e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn tssptpoefe(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_tssptpoefe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn tssipv6fe(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_tssipv6fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn tssipv4fe(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_tssipv4fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn tsseme(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_tsseme(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn tssmrme(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_tssmrme(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn tscnt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_tscnt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tspffmae(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_tspffmae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn tssti(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tssti(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tsstu(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tsstu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn tsite(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_tsite(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ttsaru(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_ttsaru(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tssarfe(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_tssarfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn tsssr(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_tsssr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

}

impl ::core::fmt::Display for Ptptscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptptscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tse() != 0 { try!(write!(f, " tse"))}
      if self.tsfcu() != 0 { try!(write!(f, " tsfcu"))}
      if self.tsptppsv2e() != 0 { try!(write!(f, " tsptppsv2e"))}
      if self.tssptpoefe() != 0 { try!(write!(f, " tssptpoefe"))}
      if self.tssipv6fe() != 0 { try!(write!(f, " tssipv6fe"))}
      if self.tssipv4fe() != 0 { try!(write!(f, " tssipv4fe"))}
      if self.tsseme() != 0 { try!(write!(f, " tsseme"))}
      if self.tssmrme() != 0 { try!(write!(f, " tssmrme"))}
      if self.tscnt() != 0 { try!(write!(f, " tscnt=0x{:x}", self.tscnt()))}
      if self.tspffmae() != 0 { try!(write!(f, " tspffmae"))}
      if self.tssti() != 0 { try!(write!(f, " tssti"))}
      if self.tsstu() != 0 { try!(write!(f, " tsstu"))}
      if self.tsite() != 0 { try!(write!(f, " tsite"))}
      if self.ttsaru() != 0 { try!(write!(f, " ttsaru"))}
      if self.tssarfe() != 0 { try!(write!(f, " tssarfe"))}
      if self.tsssr() != 0 { try!(write!(f, " tsssr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptpssir(pub u32);

impl Ptpssir {
  pub fn stssi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_stssi(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ptpssir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptpssir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stssi() != 0 { try!(write!(f, " stssi=0x{:x}", self.stssi()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptptshr(pub u32);

impl Ptptshr {
  pub fn sts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_sts(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ptptshr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptptshr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptptslr(pub u32);

impl Ptptslr {
  pub fn stss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  pub fn set_stss(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn stpns(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_stpns(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Ptptslr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptptslr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stss() != 0 { try!(write!(f, " stss=0x{:x}", self.stss()))}
      if self.stpns() != 0 { try!(write!(f, " stpns"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptptshur(pub u32);

impl Ptptshur {
  pub fn tsus(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tsus(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ptptshur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptptshur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptptslur(pub u32);

impl Ptptslur {
  pub fn tsuss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  pub fn set_tsuss(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tsupns(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_tsupns(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Ptptslur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptptslur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tsuss() != 0 { try!(write!(f, " tsuss=0x{:x}", self.tsuss()))}
      if self.tsupns() != 0 { try!(write!(f, " tsupns"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptptsar(pub u32);

impl Ptptsar {
  pub fn tsa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tsa(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ptptsar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptptsar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptptthr(pub u32);

impl Ptptthr {
  pub fn ttsh(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_ttsh(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ptptthr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptptthr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptpttlr(pub u32);

impl Ptpttlr {
  pub fn ttsl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_ttsl(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ptpttlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptpttlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptptssr(pub u32);

impl Ptptssr {
  pub fn tsso(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tsso(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tsttr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tsttr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Ptptssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptptssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tsso() != 0 { try!(write!(f, " tsso"))}
      if self.tsttr() != 0 { try!(write!(f, " tsttr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ptpppscr(pub u32);

impl Ptpppscr {
  pub fn tsso(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tsso(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tsttr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tsttr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Ptpppscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ptpppscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tsso() != 0 { try!(write!(f, " tsso"))}
      if self.tsttr() != 0 { try!(write!(f, " tsttr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

