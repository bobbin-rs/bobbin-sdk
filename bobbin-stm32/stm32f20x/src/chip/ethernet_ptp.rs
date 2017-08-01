//! Ethernet: Precision time protocol
pub const ETHERNET_PTP: EthernetPtp = EthernetPtp(0x40028700);

#[doc="Ethernet: Precision time protocol"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetPtp(pub u32);
impl EthernetPtp {
#[doc="Get the *const pointer for the PTPTSCR register."]
  #[inline] pub fn ptptscr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTSCR register."]
  #[inline] pub fn ptptscr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the PTPTSCR register."]
  #[inline] pub fn ptptscr(&self) -> Ptptscr { 
     unsafe {
        Ptptscr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the PTPTSCR register."]
  #[inline] pub fn set_ptptscr(&self, value: Ptptscr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PTPTSCR register."]
  #[inline] pub fn with_ptptscr<F: FnOnce(Ptptscr) -> Ptptscr>(&self, f: F) -> &Self {
     let tmp = self.ptptscr();
     self.set_ptptscr(f(tmp))
  }

#[doc="Get the *const pointer for the PTPSSIR register."]
  #[inline] pub fn ptpssir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PTPSSIR register."]
  #[inline] pub fn ptpssir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the PTPSSIR register."]
  #[inline] pub fn ptpssir(&self) -> Ptpssir { 
     unsafe {
        Ptpssir(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the PTPSSIR register."]
  #[inline] pub fn set_ptpssir(&self, value: Ptpssir) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PTPSSIR register."]
  #[inline] pub fn with_ptpssir<F: FnOnce(Ptpssir) -> Ptpssir>(&self, f: F) -> &Self {
     let tmp = self.ptpssir();
     self.set_ptpssir(f(tmp))
  }

#[doc="Get the *const pointer for the PTPTSHR register."]
  #[inline] pub fn ptptshr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTSHR register."]
  #[inline] pub fn ptptshr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the PTPTSHR register."]
  #[inline] pub fn ptptshr(&self) -> Ptptshr { 
     unsafe {
        Ptptshr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PTPTSLR register."]
  #[inline] pub fn ptptslr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTSLR register."]
  #[inline] pub fn ptptslr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the PTPTSLR register."]
  #[inline] pub fn ptptslr(&self) -> Ptptslr { 
     unsafe {
        Ptptslr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PTPTSHUR register."]
  #[inline] pub fn ptptshur_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTSHUR register."]
  #[inline] pub fn ptptshur_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the PTPTSHUR register."]
  #[inline] pub fn ptptshur(&self) -> Ptptshur { 
     unsafe {
        Ptptshur(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the PTPTSHUR register."]
  #[inline] pub fn set_ptptshur(&self, value: Ptptshur) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PTPTSHUR register."]
  #[inline] pub fn with_ptptshur<F: FnOnce(Ptptshur) -> Ptptshur>(&self, f: F) -> &Self {
     let tmp = self.ptptshur();
     self.set_ptptshur(f(tmp))
  }

#[doc="Get the *const pointer for the PTPTSLUR register."]
  #[inline] pub fn ptptslur_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTSLUR register."]
  #[inline] pub fn ptptslur_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the PTPTSLUR register."]
  #[inline] pub fn ptptslur(&self) -> Ptptslur { 
     unsafe {
        Ptptslur(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the PTPTSLUR register."]
  #[inline] pub fn set_ptptslur(&self, value: Ptptslur) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PTPTSLUR register."]
  #[inline] pub fn with_ptptslur<F: FnOnce(Ptptslur) -> Ptptslur>(&self, f: F) -> &Self {
     let tmp = self.ptptslur();
     self.set_ptptslur(f(tmp))
  }

#[doc="Get the *const pointer for the PTPTSAR register."]
  #[inline] pub fn ptptsar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTSAR register."]
  #[inline] pub fn ptptsar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the PTPTSAR register."]
  #[inline] pub fn ptptsar(&self) -> Ptptsar { 
     unsafe {
        Ptptsar(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the PTPTSAR register."]
  #[inline] pub fn set_ptptsar(&self, value: Ptptsar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PTPTSAR register."]
  #[inline] pub fn with_ptptsar<F: FnOnce(Ptptsar) -> Ptptsar>(&self, f: F) -> &Self {
     let tmp = self.ptptsar();
     self.set_ptptsar(f(tmp))
  }

#[doc="Get the *const pointer for the PTPTTHR register."]
  #[inline] pub fn ptptthr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTTHR register."]
  #[inline] pub fn ptptthr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the PTPTTHR register."]
  #[inline] pub fn ptptthr(&self) -> Ptptthr { 
     unsafe {
        Ptptthr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the PTPTTHR register."]
  #[inline] pub fn set_ptptthr(&self, value: Ptptthr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PTPTTHR register."]
  #[inline] pub fn with_ptptthr<F: FnOnce(Ptptthr) -> Ptptthr>(&self, f: F) -> &Self {
     let tmp = self.ptptthr();
     self.set_ptptthr(f(tmp))
  }

#[doc="Get the *const pointer for the PTPTTLR register."]
  #[inline] pub fn ptpttlr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTTLR register."]
  #[inline] pub fn ptpttlr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the PTPTTLR register."]
  #[inline] pub fn ptpttlr(&self) -> Ptpttlr { 
     unsafe {
        Ptpttlr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the PTPTTLR register."]
  #[inline] pub fn set_ptpttlr(&self, value: Ptpttlr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PTPTTLR register."]
  #[inline] pub fn with_ptpttlr<F: FnOnce(Ptpttlr) -> Ptpttlr>(&self, f: F) -> &Self {
     let tmp = self.ptpttlr();
     self.set_ptpttlr(f(tmp))
  }

#[doc="Get the *const pointer for the PTPTSSR register."]
  #[inline] pub fn ptptssr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the PTPTSSR register."]
  #[inline] pub fn ptptssr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the PTPTSSR register."]
  #[inline] pub fn ptptssr(&self) -> Ptptssr { 
     unsafe {
        Ptptssr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PTPPPSCR register."]
  #[inline] pub fn ptpppscr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the PTPPPSCR register."]
  #[inline] pub fn ptpppscr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Read the PTPPPSCR register."]
  #[inline] pub fn ptpppscr(&self) -> Ptpppscr { 
     unsafe {
        Ptpppscr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }

}

#[doc="Ethernet PTP time stamp control register"]
#[derive(PartialEq, Eq)]
pub struct Ptptscr(pub u32);
impl Ptptscr {
#[doc="no description available"]
  #[inline] pub fn tse(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="no description available"]
  #[inline] pub fn set_tse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsfcu(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsfcu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsptppsv2e(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsptppsv2e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tssptpoefe(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="no description available"]
  #[inline] pub fn set_tssptpoefe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tssipv6fe(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="no description available"]
  #[inline] pub fn set_tssipv6fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tssipv4fe(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="no description available"]
  #[inline] pub fn set_tssipv4fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsseme(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsseme(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tssmrme(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="no description available"]
  #[inline] pub fn set_tssmrme(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tscnt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
#[doc="no description available"]
  #[inline] pub fn set_tscnt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tspffmae(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="no description available"]
  #[inline] pub fn set_tspffmae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tssti(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="no description available"]
  #[inline] pub fn set_tssti(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsstu(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsstu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsite(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsite(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="no description available"]
  #[inline] pub fn ttsaru(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="no description available"]
  #[inline] pub fn set_ttsaru(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tssarfe(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="no description available"]
  #[inline] pub fn set_tssarfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsssr(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsssr(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP subsecond increment register"]
#[derive(PartialEq, Eq)]
pub struct Ptpssir(pub u32);
impl Ptpssir {
#[doc="no description available"]
  #[inline] pub fn stssi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_stssi(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP time stamp high register"]
#[derive(PartialEq, Eq)]
pub struct Ptptshr(pub u32);
impl Ptptshr {
#[doc="no description available"]
  #[inline] pub fn sts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_sts(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP time stamp low register"]
#[derive(PartialEq, Eq)]
pub struct Ptptslr(pub u32);
impl Ptptslr {
#[doc="no description available"]
  #[inline] pub fn stss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_stss(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn stpns(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="no description available"]
  #[inline] pub fn set_stpns(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP time stamp high update register"]
#[derive(PartialEq, Eq)]
pub struct Ptptshur(pub u32);
impl Ptptshur {
#[doc="no description available"]
  #[inline] pub fn tsus(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsus(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP time stamp low update register"]
#[derive(PartialEq, Eq)]
pub struct Ptptslur(pub u32);
impl Ptptslur {
#[doc="no description available"]
  #[inline] pub fn tsuss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsuss(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsupns(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsupns(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP time stamp addend register"]
#[derive(PartialEq, Eq)]
pub struct Ptptsar(pub u32);
impl Ptptsar {
#[doc="no description available"]
  #[inline] pub fn tsa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsa(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP target time high register"]
#[derive(PartialEq, Eq)]
pub struct Ptptthr(pub u32);
impl Ptptthr {
#[doc="0"]
  #[inline] pub fn ttsh(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="0"]
  #[inline] pub fn set_ttsh(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP target time low register"]
#[derive(PartialEq, Eq)]
pub struct Ptpttlr(pub u32);
impl Ptpttlr {
#[doc="no description available"]
  #[inline] pub fn ttsl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_ttsl(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP time stamp status register"]
#[derive(PartialEq, Eq)]
pub struct Ptptssr(pub u32);
impl Ptptssr {
#[doc="no description available"]
  #[inline] pub fn tsso(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsso(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsttr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsttr(mut self, value: u32) -> Self {
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
#[doc="Ethernet PTP PPS control register"]
#[derive(PartialEq, Eq)]
pub struct Ptpppscr(pub u32);
impl Ptpppscr {
#[doc="TSSO"]
  #[inline] pub fn tsso(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="TSSO"]
  #[inline] pub fn set_tsso(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TSTTR"]
  #[inline] pub fn tsttr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="TSTTR"]
  #[inline] pub fn set_tsttr(mut self, value: u32) -> Self {
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

