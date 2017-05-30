pub const RCC: Rcc = Rcc(0x40021000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcc(pub u32);
impl Rcc {
  pub fn cr(&self) -> Cr { 
     unsafe {
       Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  pub fn set_cr(&self, value: Cr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Rcc {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  pub fn icscr(&self) -> Icscr { 
     unsafe {
       Icscr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  pub fn set_icscr(&self, value: Icscr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  pub fn with_icscr<F: FnOnce(Icscr) -> Icscr>(&self, f: F) -> &Rcc {
     let tmp = self.icscr();
     self.set_icscr(f(tmp))
  }

  pub fn crrcr(&self) -> Crrcr { 
     unsafe {
       Crrcr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  pub fn set_crrcr(&self, value: Crrcr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  pub fn with_crrcr<F: FnOnce(Crrcr) -> Crrcr>(&self, f: F) -> &Rcc {
     let tmp = self.crrcr();
     self.set_crrcr(f(tmp))
  }

  pub fn cfgr(&self) -> Cfgr { 
     unsafe {
       Cfgr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  pub fn set_cfgr(&self, value: Cfgr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Rcc {
     let tmp = self.cfgr();
     self.set_cfgr(f(tmp))
  }

  pub fn cier(&self) -> Cier { 
     unsafe {
       Cier(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }

  pub fn cifr(&self) -> Cifr { 
     unsafe {
       Cifr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }

  pub fn cicr(&self) -> Cicr { 
     unsafe {
       Cicr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }

  pub fn ioprstr(&self) -> Ioprstr { 
     unsafe {
       Ioprstr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  pub fn set_ioprstr(&self, value: Ioprstr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ioprstr<F: FnOnce(Ioprstr) -> Ioprstr>(&self, f: F) -> &Rcc {
     let tmp = self.ioprstr();
     self.set_ioprstr(f(tmp))
  }

  pub fn ahbrstr(&self) -> Ahbrstr { 
     unsafe {
       Ahbrstr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  pub fn set_ahbrstr(&self, value: Ahbrstr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ahbrstr<F: FnOnce(Ahbrstr) -> Ahbrstr>(&self, f: F) -> &Rcc {
     let tmp = self.ahbrstr();
     self.set_ahbrstr(f(tmp))
  }

  pub fn apb2rstr(&self) -> Apb2rstr { 
     unsafe {
       Apb2rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  pub fn set_apb2rstr(&self, value: Apb2rstr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  pub fn with_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Rcc {
     let tmp = self.apb2rstr();
     self.set_apb2rstr(f(tmp))
  }

  pub fn apb1rstr(&self) -> Apb1rstr { 
     unsafe {
       Apb1rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  pub fn set_apb1rstr(&self, value: Apb1rstr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  pub fn with_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Rcc {
     let tmp = self.apb1rstr();
     self.set_apb1rstr(f(tmp))
  }

  pub fn iopenr(&self) -> Iopenr { 
     unsafe {
       Iopenr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
  pub fn set_iopenr(&self, value: Iopenr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_iopenr<F: FnOnce(Iopenr) -> Iopenr>(&self, f: F) -> &Rcc {
     let tmp = self.iopenr();
     self.set_iopenr(f(tmp))
  }

  pub fn ahbenr(&self) -> Ahbenr { 
     unsafe {
       Ahbenr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
  pub fn set_ahbenr(&self, value: Ahbenr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ahbenr<F: FnOnce(Ahbenr) -> Ahbenr>(&self, f: F) -> &Rcc {
     let tmp = self.ahbenr();
     self.set_ahbenr(f(tmp))
  }

  pub fn apb2enr(&self) -> Apb2enr { 
     unsafe {
       Apb2enr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
  pub fn set_apb2enr(&self, value: Apb2enr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
  pub fn with_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Rcc {
     let tmp = self.apb2enr();
     self.set_apb2enr(f(tmp))
  }

  pub fn apb1enr(&self) -> Apb1enr { 
     unsafe {
       Apb1enr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
  pub fn set_apb1enr(&self, value: Apb1enr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
  pub fn with_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Rcc {
     let tmp = self.apb1enr();
     self.set_apb1enr(f(tmp))
  }

  pub fn iopsmen(&self) -> Iopsmen { 
     unsafe {
       Iopsmen(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
  pub fn set_iopsmen(&self, value: Iopsmen) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_iopsmen<F: FnOnce(Iopsmen) -> Iopsmen>(&self, f: F) -> &Rcc {
     let tmp = self.iopsmen();
     self.set_iopsmen(f(tmp))
  }

  pub fn ahbsmenr(&self) -> Ahbsmenr { 
     unsafe {
       Ahbsmenr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
  pub fn set_ahbsmenr(&self, value: Ahbsmenr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ahbsmenr<F: FnOnce(Ahbsmenr) -> Ahbsmenr>(&self, f: F) -> &Rcc {
     let tmp = self.ahbsmenr();
     self.set_ahbsmenr(f(tmp))
  }

  pub fn apb2smenr(&self) -> Apb2smenr { 
     unsafe {
       Apb2smenr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
  pub fn set_apb2smenr(&self, value: Apb2smenr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
  pub fn with_apb2smenr<F: FnOnce(Apb2smenr) -> Apb2smenr>(&self, f: F) -> &Rcc {
     let tmp = self.apb2smenr();
     self.set_apb2smenr(f(tmp))
  }

  pub fn apb1smenr(&self) -> Apb1smenr { 
     unsafe {
       Apb1smenr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
  pub fn set_apb1smenr(&self, value: Apb1smenr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
  pub fn with_apb1smenr<F: FnOnce(Apb1smenr) -> Apb1smenr>(&self, f: F) -> &Rcc {
     let tmp = self.apb1smenr();
     self.set_apb1smenr(f(tmp))
  }

  pub fn ccipr(&self) -> Ccipr { 
     unsafe {
       Ccipr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
  pub fn set_ccipr(&self, value: Ccipr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccipr<F: FnOnce(Ccipr) -> Ccipr>(&self, f: F) -> &Rcc {
     let tmp = self.ccipr();
     self.set_ccipr(f(tmp))
  }

  pub fn csr(&self) -> Csr { 
     unsafe {
       Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
  pub fn set_csr(&self, value: Csr) -> &Rcc {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
  pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Rcc {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
  pub fn pllrdy(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_pllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn pllon(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_pllon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn rtcpre(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  pub fn set_rtcpre(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn csslseon(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_csslseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn hsebyp(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_hsebyp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn hserdy(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_hserdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn hseon(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_hseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn msirdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_msirdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn msion(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_msion(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn hsi16divf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_hsi16divf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn hsi16diven(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_hsi16diven(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn hsi16rdyf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_hsi16rdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn hsi16keron(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_hsi16keron(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn hsi16on(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_hsi16on(mut self, value: u32) -> Self {
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
      if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
      if self.pllon() != 0 { try!(write!(f, " pllon"))}
      if self.rtcpre() != 0 { try!(write!(f, " rtcpre=0x{:x}", self.rtcpre()))}
      if self.csslseon() != 0 { try!(write!(f, " csslseon"))}
      if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
      if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
      if self.hseon() != 0 { try!(write!(f, " hseon"))}
      if self.msirdy() != 0 { try!(write!(f, " msirdy"))}
      if self.msion() != 0 { try!(write!(f, " msion"))}
      if self.hsi16divf() != 0 { try!(write!(f, " hsi16divf"))}
      if self.hsi16diven() != 0 { try!(write!(f, " hsi16diven"))}
      if self.hsi16rdyf() != 0 { try!(write!(f, " hsi16rdyf"))}
      if self.hsi16keron() != 0 { try!(write!(f, " hsi16keron"))}
      if self.hsi16on() != 0 { try!(write!(f, " hsi16on"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Icscr(pub u32);
impl Icscr {
  pub fn msitrim(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_msitrim(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

  pub fn msical(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_msical(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn msirange(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  pub fn set_msirange(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn hsi16trim(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  pub fn set_hsi16trim(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

  pub fn hsi16cal(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_hsi16cal(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Icscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icscr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.msitrim() != 0 { try!(write!(f, " msitrim=0x{:x}", self.msitrim()))}
      if self.msical() != 0 { try!(write!(f, " msical=0x{:x}", self.msical()))}
      if self.msirange() != 0 { try!(write!(f, " msirange=0x{:x}", self.msirange()))}
      if self.hsi16trim() != 0 { try!(write!(f, " hsi16trim=0x{:x}", self.hsi16trim()))}
      if self.hsi16cal() != 0 { try!(write!(f, " hsi16cal=0x{:x}", self.hsi16cal()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Crrcr(pub u32);
impl Crrcr {
  pub fn hsi48cal(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_hsi48cal(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn hsi48rdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_hsi48rdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn hsi48on(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_hsi48on(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Crrcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crrcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.hsi48cal() != 0 { try!(write!(f, " hsi48cal=0x{:x}", self.hsi48cal()))}
      if self.hsi48rdy() != 0 { try!(write!(f, " hsi48rdy"))}
      if self.hsi48on() != 0 { try!(write!(f, " hsi48on"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
  pub fn mcopre(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x7 // [30:28]
  }
  pub fn set_mcopre(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn mcosel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_mcosel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn plldiv(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
  pub fn set_plldiv(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn pllmul(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0xf // [21:18]
  }
  pub fn set_pllmul(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 18);
     self.0 |= value << 18;
     self
  }

  pub fn pllsrc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_pllsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn stopwuck(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_stopwuck(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn ppre2(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x7 // [13:11]
  }
  pub fn set_ppre2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn ppre1(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_ppre1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn hpre(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_hpre(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn sws(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_sws(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn sw(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_sw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mcopre() != 0 { try!(write!(f, " mcopre=0x{:x}", self.mcopre()))}
      if self.mcosel() != 0 { try!(write!(f, " mcosel=0x{:x}", self.mcosel()))}
      if self.plldiv() != 0 { try!(write!(f, " plldiv=0x{:x}", self.plldiv()))}
      if self.pllmul() != 0 { try!(write!(f, " pllmul=0x{:x}", self.pllmul()))}
      if self.pllsrc() != 0 { try!(write!(f, " pllsrc"))}
      if self.stopwuck() != 0 { try!(write!(f, " stopwuck"))}
      if self.ppre2() != 0 { try!(write!(f, " ppre2=0x{:x}", self.ppre2()))}
      if self.ppre1() != 0 { try!(write!(f, " ppre1=0x{:x}", self.ppre1()))}
      if self.hpre() != 0 { try!(write!(f, " hpre=0x{:x}", self.hpre()))}
      if self.sws() != 0 { try!(write!(f, " sws=0x{:x}", self.sws()))}
      if self.sw() != 0 { try!(write!(f, " sw=0x{:x}", self.sw()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cier(pub u32);
impl Cier {
  pub fn csslse(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_csslse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn hsi48rdyie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_hsi48rdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn msirdyie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_msirdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pllrdyie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_pllrdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn hserdyie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_hserdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn hsi16rdyie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_hsi16rdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn lserdyie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_lserdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn lsirdyie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_lsirdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.csslse() != 0 { try!(write!(f, " csslse"))}
      if self.hsi48rdyie() != 0 { try!(write!(f, " hsi48rdyie"))}
      if self.msirdyie() != 0 { try!(write!(f, " msirdyie"))}
      if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
      if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
      if self.hsi16rdyie() != 0 { try!(write!(f, " hsi16rdyie"))}
      if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
      if self.lsirdyie() != 0 { try!(write!(f, " lsirdyie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cifr(pub u32);
impl Cifr {
  pub fn csshsef(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_csshsef(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn csslsef(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_csslsef(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn hsi48rdyf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_hsi48rdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn msirdyf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_msirdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pllrdyf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_pllrdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn hserdyf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_hserdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn hsi16rdyf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_hsi16rdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn lserdyf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_lserdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn lsirdyf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_lsirdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cifr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cifr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.csshsef() != 0 { try!(write!(f, " csshsef"))}
      if self.csslsef() != 0 { try!(write!(f, " csslsef"))}
      if self.hsi48rdyf() != 0 { try!(write!(f, " hsi48rdyf"))}
      if self.msirdyf() != 0 { try!(write!(f, " msirdyf"))}
      if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
      if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
      if self.hsi16rdyf() != 0 { try!(write!(f, " hsi16rdyf"))}
      if self.lserdyf() != 0 { try!(write!(f, " lserdyf"))}
      if self.lsirdyf() != 0 { try!(write!(f, " lsirdyf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cicr(pub u32);
impl Cicr {
  pub fn csshsec(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_csshsec(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn csslsec(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_csslsec(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn hsi48rdyc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_hsi48rdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn msirdyc(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_msirdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pllrdyc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_pllrdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn hserdyc(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_hserdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn hsi16rdyc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_hsi16rdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn lserdyc(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_lserdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn lsirdyc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_lsirdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cicr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cicr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.csshsec() != 0 { try!(write!(f, " csshsec"))}
      if self.csslsec() != 0 { try!(write!(f, " csslsec"))}
      if self.hsi48rdyc() != 0 { try!(write!(f, " hsi48rdyc"))}
      if self.msirdyc() != 0 { try!(write!(f, " msirdyc"))}
      if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
      if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
      if self.hsi16rdyc() != 0 { try!(write!(f, " hsi16rdyc"))}
      if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
      if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ioprstr(pub u32);
impl Ioprstr {
  pub fn iophrst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_iophrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn iopcrst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_iopcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn iopbrst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_iopbrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn ioparst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ioparst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ioprstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ioprstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iophrst() != 0 { try!(write!(f, " iophrst"))}
      if self.iopcrst() != 0 { try!(write!(f, " iopcrst"))}
      if self.iopbrst() != 0 { try!(write!(f, " iopbrst"))}
      if self.ioparst() != 0 { try!(write!(f, " ioparst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ahbrstr(pub u32);
impl Ahbrstr {
  pub fn cryprst(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_cryprst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn rngrst(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rngrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn touchrst(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_touchrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn crcrst(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_crcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mifrst(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_mifrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dmarst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_dmarst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahbrstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahbrstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cryprst() != 0 { try!(write!(f, " cryprst"))}
      if self.rngrst() != 0 { try!(write!(f, " rngrst"))}
      if self.touchrst() != 0 { try!(write!(f, " touchrst"))}
      if self.crcrst() != 0 { try!(write!(f, " crcrst"))}
      if self.mifrst() != 0 { try!(write!(f, " mifrst"))}
      if self.dmarst() != 0 { try!(write!(f, " dmarst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
  pub fn dbgrst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_dbgrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn usart1rst(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_usart1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn spi1rst(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_spi1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn adcrst(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_adcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn tm12rst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tm12rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tim21rst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tim21rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn syscfgrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_syscfgrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb2rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb2rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrst() != 0 { try!(write!(f, " dbgrst"))}
      if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
      if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
      if self.adcrst() != 0 { try!(write!(f, " adcrst"))}
      if self.tm12rst() != 0 { try!(write!(f, " tm12rst"))}
      if self.tim21rst() != 0 { try!(write!(f, " tim21rst"))}
      if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
  pub fn lptim1rst(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_lptim1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn dacrst(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_dacrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn pwrrst(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_pwrrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn crsrst(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_crsrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn usbrst(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_usbrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn i2c2rst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_i2c2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn i2c1rst(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_i2c1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn lpuart1rst(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_lpuart1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn usart2rst(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_usart2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn spi2rst(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_spi2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn wwdrst(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_wwdrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn tim6rst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_tim6rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tim2rst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb1rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb1rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lptim1rst() != 0 { try!(write!(f, " lptim1rst"))}
      if self.dacrst() != 0 { try!(write!(f, " dacrst"))}
      if self.pwrrst() != 0 { try!(write!(f, " pwrrst"))}
      if self.crsrst() != 0 { try!(write!(f, " crsrst"))}
      if self.usbrst() != 0 { try!(write!(f, " usbrst"))}
      if self.i2c2rst() != 0 { try!(write!(f, " i2c2rst"))}
      if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
      if self.lpuart1rst() != 0 { try!(write!(f, " lpuart1rst"))}
      if self.usart2rst() != 0 { try!(write!(f, " usart2rst"))}
      if self.spi2rst() != 0 { try!(write!(f, " spi2rst"))}
      if self.wwdrst() != 0 { try!(write!(f, " wwdrst"))}
      if self.tim6rst() != 0 { try!(write!(f, " tim6rst"))}
      if self.tim2rst() != 0 { try!(write!(f, " tim2rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Iopenr(pub u32);
impl Iopenr {
  pub fn iopaen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_iopaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn iopben(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_iopben(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn iopcen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_iopcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn iophen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_iophen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Iopenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Iopenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iopaen() != 0 { try!(write!(f, " iopaen"))}
      if self.iopben() != 0 { try!(write!(f, " iopben"))}
      if self.iopcen() != 0 { try!(write!(f, " iopcen"))}
      if self.iophen() != 0 { try!(write!(f, " iophen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ahbenr(pub u32);
impl Ahbenr {
  pub fn crypen(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_crypen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn rngen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rngen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn touchen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_touchen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn crcen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_crcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mifen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_mifen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dmaen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_dmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahbenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahbenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.crypen() != 0 { try!(write!(f, " crypen"))}
      if self.rngen() != 0 { try!(write!(f, " rngen"))}
      if self.touchen() != 0 { try!(write!(f, " touchen"))}
      if self.crcen() != 0 { try!(write!(f, " crcen"))}
      if self.mifen() != 0 { try!(write!(f, " mifen"))}
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
  pub fn dbgen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_dbgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn usart1en(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_usart1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn spi1en(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_spi1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn adcen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_adcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn mifien(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_mifien(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn tim22en(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tim22en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tim21en(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tim21en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn syscfgen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_syscfgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb2enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb2enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
      if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
      if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
      if self.adcen() != 0 { try!(write!(f, " adcen"))}
      if self.mifien() != 0 { try!(write!(f, " mifien"))}
      if self.tim22en() != 0 { try!(write!(f, " tim22en"))}
      if self.tim21en() != 0 { try!(write!(f, " tim21en"))}
      if self.syscfgen() != 0 { try!(write!(f, " syscfgen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
  pub fn lptim1en(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_lptim1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn dacen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_dacen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn pwren(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_pwren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn crsen(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_crsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn usben(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_usben(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn i2c2en(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_i2c2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn i2c1en(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_i2c1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn lpuart1en(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_lpuart1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn usart2en(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_usart2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn spi2en(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_spi2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn wwdgen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_wwdgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn tim6en(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_tim6en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tim2en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb1enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb1enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lptim1en() != 0 { try!(write!(f, " lptim1en"))}
      if self.dacen() != 0 { try!(write!(f, " dacen"))}
      if self.pwren() != 0 { try!(write!(f, " pwren"))}
      if self.crsen() != 0 { try!(write!(f, " crsen"))}
      if self.usben() != 0 { try!(write!(f, " usben"))}
      if self.i2c2en() != 0 { try!(write!(f, " i2c2en"))}
      if self.i2c1en() != 0 { try!(write!(f, " i2c1en"))}
      if self.lpuart1en() != 0 { try!(write!(f, " lpuart1en"))}
      if self.usart2en() != 0 { try!(write!(f, " usart2en"))}
      if self.spi2en() != 0 { try!(write!(f, " spi2en"))}
      if self.wwdgen() != 0 { try!(write!(f, " wwdgen"))}
      if self.tim6en() != 0 { try!(write!(f, " tim6en"))}
      if self.tim2en() != 0 { try!(write!(f, " tim2en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Iopsmen(pub u32);
impl Iopsmen {
  pub fn iophsmen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_iophsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn iopdsmen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_iopdsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn iopcsmen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_iopcsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn iopbsmen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_iopbsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn iopasmen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_iopasmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Iopsmen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Iopsmen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iophsmen() != 0 { try!(write!(f, " iophsmen"))}
      if self.iopdsmen() != 0 { try!(write!(f, " iopdsmen"))}
      if self.iopcsmen() != 0 { try!(write!(f, " iopcsmen"))}
      if self.iopbsmen() != 0 { try!(write!(f, " iopbsmen"))}
      if self.iopasmen() != 0 { try!(write!(f, " iopasmen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ahbsmenr(pub u32);
impl Ahbsmenr {
  pub fn crypsmen(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_crypsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn rngsmen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_rngsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn touchsmen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_touchsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn crcsmen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_crcsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn sramsmen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_sramsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn mifsmen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_mifsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dmasmen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_dmasmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahbsmenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahbsmenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.crypsmen() != 0 { try!(write!(f, " crypsmen"))}
      if self.rngsmen() != 0 { try!(write!(f, " rngsmen"))}
      if self.touchsmen() != 0 { try!(write!(f, " touchsmen"))}
      if self.crcsmen() != 0 { try!(write!(f, " crcsmen"))}
      if self.sramsmen() != 0 { try!(write!(f, " sramsmen"))}
      if self.mifsmen() != 0 { try!(write!(f, " mifsmen"))}
      if self.dmasmen() != 0 { try!(write!(f, " dmasmen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apb2smenr(pub u32);
impl Apb2smenr {
  pub fn dbgsmen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_dbgsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn usart1smen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_usart1smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn spi1smen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_spi1smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn adcsmen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_adcsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn tim22smen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tim22smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tim21smen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tim21smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn syscfgsmen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_syscfgsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb2smenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb2smenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgsmen() != 0 { try!(write!(f, " dbgsmen"))}
      if self.usart1smen() != 0 { try!(write!(f, " usart1smen"))}
      if self.spi1smen() != 0 { try!(write!(f, " spi1smen"))}
      if self.adcsmen() != 0 { try!(write!(f, " adcsmen"))}
      if self.tim22smen() != 0 { try!(write!(f, " tim22smen"))}
      if self.tim21smen() != 0 { try!(write!(f, " tim21smen"))}
      if self.syscfgsmen() != 0 { try!(write!(f, " syscfgsmen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Apb1smenr(pub u32);
impl Apb1smenr {
  pub fn lptim1smen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_lptim1smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn dacsmen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_dacsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn pwrsmen(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_pwrsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn crssmen(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_crssmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn usbsmen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_usbsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn i2c2smen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_i2c2smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn i2c1smen(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_i2c1smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn lpuart1smen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_lpuart1smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn usart2smen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_usart2smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn spi2smen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_spi2smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn wwdgsmen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_wwdgsmen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn tim6smen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_tim6smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tim2smen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim2smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb1smenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb1smenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lptim1smen() != 0 { try!(write!(f, " lptim1smen"))}
      if self.dacsmen() != 0 { try!(write!(f, " dacsmen"))}
      if self.pwrsmen() != 0 { try!(write!(f, " pwrsmen"))}
      if self.crssmen() != 0 { try!(write!(f, " crssmen"))}
      if self.usbsmen() != 0 { try!(write!(f, " usbsmen"))}
      if self.i2c2smen() != 0 { try!(write!(f, " i2c2smen"))}
      if self.i2c1smen() != 0 { try!(write!(f, " i2c1smen"))}
      if self.lpuart1smen() != 0 { try!(write!(f, " lpuart1smen"))}
      if self.usart2smen() != 0 { try!(write!(f, " usart2smen"))}
      if self.spi2smen() != 0 { try!(write!(f, " spi2smen"))}
      if self.wwdgsmen() != 0 { try!(write!(f, " wwdgsmen"))}
      if self.tim6smen() != 0 { try!(write!(f, " tim6smen"))}
      if self.tim2smen() != 0 { try!(write!(f, " tim2smen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccipr(pub u32);
impl Ccipr {
  pub fn hsi48msel(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_hsi48msel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn lptim1sel(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x3 // [19:18]
  }
  pub fn set_lptim1sel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn i2c1sel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  pub fn set_i2c1sel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn lpuart1sel(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  pub fn set_lpuart1sel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn usart2sel(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_usart2sel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn usart1sel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_usart1sel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccipr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccipr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.hsi48msel() != 0 { try!(write!(f, " hsi48msel"))}
      if self.lptim1sel() != 0 { try!(write!(f, " lptim1sel=0x{:x}", self.lptim1sel()))}
      if self.i2c1sel() != 0 { try!(write!(f, " i2c1sel=0x{:x}", self.i2c1sel()))}
      if self.lpuart1sel() != 0 { try!(write!(f, " lpuart1sel=0x{:x}", self.lpuart1sel()))}
      if self.usart2sel() != 0 { try!(write!(f, " usart2sel=0x{:x}", self.usart2sel()))}
      if self.usart1sel() != 0 { try!(write!(f, " usart1sel=0x{:x}", self.usart1sel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
  pub fn lpwrstf(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_lpwrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn wwdgrstf(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_wwdgrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn iwdgrstf(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_iwdgrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn sftrstf(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_sftrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn porrstf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_porrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn pinrstf(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_pinrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn oblrstf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_oblrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn rmvf(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_rmvf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn rtcrst(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_rtcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn rtcen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_rtcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn rtcsel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_rtcsel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn csslsed(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_csslsed(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn csslseon(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_csslseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn lsedrv(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x3 // [12:11]
  }
  pub fn set_lsedrv(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn lsebyp(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_lsebyp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn lserdy(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_lserdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn lseon(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_lseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn lsirdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_lsirdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn lsion(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_lsion(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lpwrstf() != 0 { try!(write!(f, " lpwrstf"))}
      if self.wwdgrstf() != 0 { try!(write!(f, " wwdgrstf"))}
      if self.iwdgrstf() != 0 { try!(write!(f, " iwdgrstf"))}
      if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
      if self.porrstf() != 0 { try!(write!(f, " porrstf"))}
      if self.pinrstf() != 0 { try!(write!(f, " pinrstf"))}
      if self.oblrstf() != 0 { try!(write!(f, " oblrstf"))}
      if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
      if self.rtcrst() != 0 { try!(write!(f, " rtcrst"))}
      if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
      if self.rtcsel() != 0 { try!(write!(f, " rtcsel=0x{:x}", self.rtcsel()))}
      if self.csslsed() != 0 { try!(write!(f, " csslsed"))}
      if self.csslseon() != 0 { try!(write!(f, " csslseon"))}
      if self.lsedrv() != 0 { try!(write!(f, " lsedrv=0x{:x}", self.lsedrv()))}
      if self.lsebyp() != 0 { try!(write!(f, " lsebyp"))}
      if self.lserdy() != 0 { try!(write!(f, " lserdy"))}
      if self.lseon() != 0 { try!(write!(f, " lseon"))}
      if self.lsirdy() != 0 { try!(write!(f, " lsirdy"))}
      if self.lsion() != 0 { try!(write!(f, " lsion"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait En {
   fn en(&self) -> u32;
   fn set_en(&self, value: u32);
}

impl Rcc {
   pub fn en<P: En>(&self, p: &P) -> u32 {
      p.en()
   }
   pub fn set_en<P: En>(&self, p: &P, value: u32) {
      p.set_en(value)
   }
}

pub trait Rst {
   fn rst(&self) -> u32;
   fn set_rst(&self, value: u32);
}

impl Rcc {
   pub fn rst<P: Rst>(&self, p: &P) -> u32 {
      p.rst()
   }
   pub fn set_rst<P: Rst>(&self, p: &P, value: u32) {
      p.set_rst(value)
   }
}

impl Rst for super::gpio::Gpioh {
   fn rst(&self) -> u32 { RCC.ioprstr().iophrst() }
   fn set_rst(&self, value: u32) { RCC.with_ioprstr(|r| r.set_iophrst(value)); }
}

impl Rst for super::gpio::Gpioc {
   fn rst(&self) -> u32 { RCC.ioprstr().iopcrst() }
   fn set_rst(&self, value: u32) { RCC.with_ioprstr(|r| r.set_iopcrst(value)); }
}

impl Rst for super::gpio::Gpiob {
   fn rst(&self) -> u32 { RCC.ioprstr().iopbrst() }
   fn set_rst(&self, value: u32) { RCC.with_ioprstr(|r| r.set_iopbrst(value)); }
}

impl Rst for super::gpio::Gpioa {
   fn rst(&self) -> u32 { RCC.ioprstr().ioparst() }
   fn set_rst(&self, value: u32) { RCC.with_ioprstr(|r| r.set_ioparst(value)); }
}

impl Rst for super::rng::Rng {
   fn rst(&self) -> u32 { RCC.ahbrstr().rngrst() }
   fn set_rst(&self, value: u32) { RCC.with_ahbrstr(|r| r.set_rngrst(value)); }
}

impl Rst for super::crc::Crc {
   fn rst(&self) -> u32 { RCC.ahbrstr().crcrst() }
   fn set_rst(&self, value: u32) { RCC.with_ahbrstr(|r| r.set_crcrst(value)); }
}

impl Rst for super::dma::Dma {
   fn rst(&self) -> u32 { RCC.ahbrstr().dmarst() }
   fn set_rst(&self, value: u32) { RCC.with_ahbrstr(|r| r.set_dmarst(value)); }
}

impl Rst for super::usart::Usart1 {
   fn rst(&self) -> u32 { RCC.apb2rstr().usart1rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_usart1rst(value)); }
}

impl Rst for super::spi::Spi1 {
   fn rst(&self) -> u32 { RCC.apb2rstr().spi1rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_spi1rst(value)); }
}

impl Rst for super::tim_gen::Tim22 {
   fn rst(&self) -> u32 { RCC.apb2rstr().tm12rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_tm12rst(value)); }
}

impl Rst for super::tim_gen::Tim21 {
   fn rst(&self) -> u32 { RCC.apb2rstr().tim21rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_tim21rst(value)); }
}

impl Rst for super::syscfg::Syscfg {
   fn rst(&self) -> u32 { RCC.apb2rstr().syscfgrst() }
   fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_syscfgrst(value)); }
}

impl Rst for super::lptim::Lptim {
   fn rst(&self) -> u32 { RCC.apb1rstr().lptim1rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_lptim1rst(value)); }
}

impl Rst for super::pwr::Pwr {
   fn rst(&self) -> u32 { RCC.apb1rstr().pwrrst() }
   fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_pwrrst(value)); }
}

impl Rst for super::i2c::I2c2 {
   fn rst(&self) -> u32 { RCC.apb1rstr().i2c2rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_i2c2rst(value)); }
}

impl Rst for super::i2c::I2c1 {
   fn rst(&self) -> u32 { RCC.apb1rstr().i2c1rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_i2c1rst(value)); }
}

impl Rst for super::usart::Usart2 {
   fn rst(&self) -> u32 { RCC.apb1rstr().usart2rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_usart2rst(value)); }
}

impl Rst for super::spi::Spi2 {
   fn rst(&self) -> u32 { RCC.apb1rstr().spi2rst() }
   fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_spi2rst(value)); }
}

impl Rst for super::wwdg::Wwdg {
   fn rst(&self) -> u32 { RCC.apb1rstr().wwdrst() }
   fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_wwdrst(value)); }
}

impl En for super::gpio::Gpioa {
   fn en(&self) -> u32 { RCC.iopenr().iopaen() }
   fn set_en(&self, value: u32) { RCC.with_iopenr(|r| r.set_iopaen(value)); }
}

impl En for super::gpio::Gpiob {
   fn en(&self) -> u32 { RCC.iopenr().iopben() }
   fn set_en(&self, value: u32) { RCC.with_iopenr(|r| r.set_iopben(value)); }
}

impl En for super::gpio::Gpioc {
   fn en(&self) -> u32 { RCC.iopenr().iopcen() }
   fn set_en(&self, value: u32) { RCC.with_iopenr(|r| r.set_iopcen(value)); }
}

impl En for super::gpio::Gpioh {
   fn en(&self) -> u32 { RCC.iopenr().iophen() }
   fn set_en(&self, value: u32) { RCC.with_iopenr(|r| r.set_iophen(value)); }
}

impl En for super::rng::Rng {
   fn en(&self) -> u32 { RCC.ahbenr().rngen() }
   fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_rngen(value)); }
}

impl En for super::crc::Crc {
   fn en(&self) -> u32 { RCC.ahbenr().crcen() }
   fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_crcen(value)); }
}

impl En for super::dma::Dma {
   fn en(&self) -> u32 { RCC.ahbenr().dmaen() }
   fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_dmaen(value)); }
}

impl En for super::usart::Usart1 {
   fn en(&self) -> u32 { RCC.apb2enr().usart1en() }
   fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_usart1en(value)); }
}

impl En for super::spi::Spi1 {
   fn en(&self) -> u32 { RCC.apb2enr().spi1en() }
   fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::tim_gen::Tim22 {
   fn en(&self) -> u32 { RCC.apb2enr().tim22en() }
   fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim22en(value)); }
}

impl En for super::tim_gen::Tim21 {
   fn en(&self) -> u32 { RCC.apb2enr().tim21en() }
   fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim21en(value)); }
}

impl En for super::syscfg::Syscfg {
   fn en(&self) -> u32 { RCC.apb2enr().syscfgen() }
   fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

impl En for super::lptim::Lptim {
   fn en(&self) -> u32 { RCC.apb1enr().lptim1en() }
   fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_lptim1en(value)); }
}

impl En for super::pwr::Pwr {
   fn en(&self) -> u32 { RCC.apb1enr().pwren() }
   fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_pwren(value)); }
}

impl En for super::i2c::I2c2 {
   fn en(&self) -> u32 { RCC.apb1enr().i2c2en() }
   fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c2en(value)); }
}

impl En for super::i2c::I2c1 {
   fn en(&self) -> u32 { RCC.apb1enr().i2c1en() }
   fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c1en(value)); }
}

impl En for super::usart::Usart2 {
   fn en(&self) -> u32 { RCC.apb1enr().usart2en() }
   fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_usart2en(value)); }
}

impl En for super::spi::Spi2 {
   fn en(&self) -> u32 { RCC.apb1enr().spi2en() }
   fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_spi2en(value)); }
}

impl En for super::wwdg::Wwdg {
   fn en(&self) -> u32 { RCC.apb1enr().wwdgen() }
   fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

