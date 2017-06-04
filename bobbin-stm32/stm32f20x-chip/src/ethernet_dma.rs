pub const ETHERNET_DMA: EthernetDma = EthernetDma(0x40029000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetDma(pub u32);
impl EthernetDma {
  #[inline]
  pub fn dmabmr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline]
  pub fn dmabmr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline]
  pub fn dmabmr(&self) -> Dmabmr { 
     unsafe {
       Dmabmr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmabmr(&self, value: Dmabmr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmabmr<F: FnOnce(Dmabmr) -> Dmabmr>(&self, f: F) -> &Self {
     let tmp = self.dmabmr();
     self.set_dmabmr(f(tmp))
  }

  #[inline]
  pub fn dmatpdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline]
  pub fn dmatpdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline]
  pub fn dmatpdr(&self) -> Dmatpdr { 
     unsafe {
       Dmatpdr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmatpdr(&self, value: Dmatpdr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmatpdr<F: FnOnce(Dmatpdr) -> Dmatpdr>(&self, f: F) -> &Self {
     let tmp = self.dmatpdr();
     self.set_dmatpdr(f(tmp))
  }

  #[inline]
  pub fn dmarpdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline]
  pub fn dmarpdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline]
  pub fn dmarpdr(&self) -> Dmarpdr { 
     unsafe {
       Dmarpdr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmarpdr(&self, value: Dmarpdr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmarpdr<F: FnOnce(Dmarpdr) -> Dmarpdr>(&self, f: F) -> &Self {
     let tmp = self.dmarpdr();
     self.set_dmarpdr(f(tmp))
  }

  #[inline]
  pub fn dmardlar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline]
  pub fn dmardlar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline]
  pub fn dmardlar(&self) -> Dmardlar { 
     unsafe {
       Dmardlar(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmardlar(&self, value: Dmardlar) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmardlar<F: FnOnce(Dmardlar) -> Dmardlar>(&self, f: F) -> &Self {
     let tmp = self.dmardlar();
     self.set_dmardlar(f(tmp))
  }

  #[inline]
  pub fn dmatdlar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline]
  pub fn dmatdlar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline]
  pub fn dmatdlar(&self) -> Dmatdlar { 
     unsafe {
       Dmatdlar(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmatdlar(&self, value: Dmatdlar) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmatdlar<F: FnOnce(Dmatdlar) -> Dmatdlar>(&self, f: F) -> &Self {
     let tmp = self.dmatdlar();
     self.set_dmatdlar(f(tmp))
  }

  #[inline]
  pub fn dmasr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline]
  pub fn dmasr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline]
  pub fn dmasr(&self) -> Dmasr { 
     unsafe {
       Dmasr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmasr(&self, value: Dmasr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmasr<F: FnOnce(Dmasr) -> Dmasr>(&self, f: F) -> &Self {
     let tmp = self.dmasr();
     self.set_dmasr(f(tmp))
  }

  #[inline]
  pub fn dmaomr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline]
  pub fn dmaomr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline]
  pub fn dmaomr(&self) -> Dmaomr { 
     unsafe {
       Dmaomr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmaomr(&self, value: Dmaomr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmaomr<F: FnOnce(Dmaomr) -> Dmaomr>(&self, f: F) -> &Self {
     let tmp = self.dmaomr();
     self.set_dmaomr(f(tmp))
  }

  #[inline]
  pub fn dmaier_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline]
  pub fn dmaier_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline]
  pub fn dmaier(&self) -> Dmaier { 
     unsafe {
       Dmaier(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmaier(&self, value: Dmaier) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmaier<F: FnOnce(Dmaier) -> Dmaier>(&self, f: F) -> &Self {
     let tmp = self.dmaier();
     self.set_dmaier(f(tmp))
  }

  #[inline]
  pub fn dmamfbocr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline]
  pub fn dmamfbocr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline]
  pub fn dmamfbocr(&self) -> Dmamfbocr { 
     unsafe {
       Dmamfbocr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmamfbocr(&self, value: Dmamfbocr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmamfbocr<F: FnOnce(Dmamfbocr) -> Dmamfbocr>(&self, f: F) -> &Self {
     let tmp = self.dmamfbocr();
     self.set_dmamfbocr(f(tmp))
  }

  #[inline]
  pub fn dmarswtr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline]
  pub fn dmarswtr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline]
  pub fn dmarswtr(&self) -> Dmarswtr { 
     unsafe {
       Dmarswtr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmarswtr(&self, value: Dmarswtr) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmarswtr<F: FnOnce(Dmarswtr) -> Dmarswtr>(&self, f: F) -> &Self {
     let tmp = self.dmarswtr();
     self.set_dmarswtr(f(tmp))
  }

  #[inline]
  pub fn dmachtdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
  #[inline]
  pub fn dmachtdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
  #[inline]
  pub fn dmachtdr(&self) -> Dmachtdr { 
     unsafe {
       Dmachtdr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }

  #[inline]
  pub fn dmachrdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
  #[inline]
  pub fn dmachrdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
  #[inline]
  pub fn dmachrdr(&self) -> Dmachrdr { 
     unsafe {
       Dmachrdr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }

  #[inline]
  pub fn dmachtbar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
  #[inline]
  pub fn dmachtbar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
  #[inline]
  pub fn dmachtbar(&self) -> Dmachtbar { 
     unsafe {
       Dmachtbar(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }

  #[inline]
  pub fn dmachrbar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
  #[inline]
  pub fn dmachrbar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
  #[inline]
  pub fn dmachrbar(&self) -> Dmachrbar { 
     unsafe {
       Dmachrbar(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct Dmabmr(pub u32);
impl Dmabmr {
  #[inline]
  pub fn sr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_sr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn da(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_da(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn dsl(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1f // [6:2]
  }
  #[inline]
  pub fn set_dsl(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn edfe(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_edfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn pbl(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3f // [13:8]
  }
  #[inline]
  pub fn set_pbl(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn rtpr(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  #[inline]
  pub fn set_rtpr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn fb(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_fb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn rdp(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x3f // [22:17]
  }
  #[inline]
  pub fn set_rdp(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn usp(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_usp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline]
  pub fn fpm(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_fpm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn aab(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_aab(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn mb(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_mb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}
impl ::core::fmt::Display for Dmabmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmabmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sr() != 0 { try!(write!(f, " sr"))}
      if self.da() != 0 { try!(write!(f, " da"))}
      if self.dsl() != 0 { try!(write!(f, " dsl=0x{:x}", self.dsl()))}
      if self.edfe() != 0 { try!(write!(f, " edfe"))}
      if self.pbl() != 0 { try!(write!(f, " pbl=0x{:x}", self.pbl()))}
      if self.rtpr() != 0 { try!(write!(f, " rtpr=0x{:x}", self.rtpr()))}
      if self.fb() != 0 { try!(write!(f, " fb"))}
      if self.rdp() != 0 { try!(write!(f, " rdp=0x{:x}", self.rdp()))}
      if self.usp() != 0 { try!(write!(f, " usp"))}
      if self.fpm() != 0 { try!(write!(f, " fpm"))}
      if self.aab() != 0 { try!(write!(f, " aab"))}
      if self.mb() != 0 { try!(write!(f, " mb"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmatpdr(pub u32);
impl Dmatpdr {
  #[inline]
  pub fn tpd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_tpd(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmatpdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmatpdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmarpdr(pub u32);
impl Dmarpdr {
  #[inline]
  pub fn rpd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_rpd(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmarpdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmarpdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmardlar(pub u32);
impl Dmardlar {
  #[inline]
  pub fn srl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_srl(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmardlar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmardlar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmatdlar(pub u32);
impl Dmatdlar {
  #[inline]
  pub fn stl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_stl(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmatdlar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmatdlar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmasr(pub u32);
impl Dmasr {
  #[inline]
  pub fn ts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tpss(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tpss(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn tbus(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_tbus(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn tjts(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_tjts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn ros(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_ros(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tus(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_tus(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn rs(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_rs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn rbus(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_rbus(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn rpss(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_rpss(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn pwts(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_pwts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn ets(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_ets(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn fbes(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_fbes(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn ers(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_ers(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn ais(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_ais(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn nis(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_nis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn rps(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7 // [19:17]
  }
  #[inline]
  pub fn set_rps(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn tps(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x7 // [22:20]
  }
  #[inline]
  pub fn set_tps(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn ebs(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x7 // [25:23]
  }
  #[inline]
  pub fn set_ebs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline]
  pub fn mmcs(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_mmcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  #[inline]
  pub fn pmts(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  #[inline]
  pub fn set_pmts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  #[inline]
  pub fn tsts(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  #[inline]
  pub fn set_tsts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

}
impl ::core::fmt::Display for Dmasr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmasr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ts() != 0 { try!(write!(f, " ts"))}
      if self.tpss() != 0 { try!(write!(f, " tpss"))}
      if self.tbus() != 0 { try!(write!(f, " tbus"))}
      if self.tjts() != 0 { try!(write!(f, " tjts"))}
      if self.ros() != 0 { try!(write!(f, " ros"))}
      if self.tus() != 0 { try!(write!(f, " tus"))}
      if self.rs() != 0 { try!(write!(f, " rs"))}
      if self.rbus() != 0 { try!(write!(f, " rbus"))}
      if self.rpss() != 0 { try!(write!(f, " rpss"))}
      if self.pwts() != 0 { try!(write!(f, " pwts"))}
      if self.ets() != 0 { try!(write!(f, " ets"))}
      if self.fbes() != 0 { try!(write!(f, " fbes"))}
      if self.ers() != 0 { try!(write!(f, " ers"))}
      if self.ais() != 0 { try!(write!(f, " ais"))}
      if self.nis() != 0 { try!(write!(f, " nis"))}
      if self.rps() != 0 { try!(write!(f, " rps=0x{:x}", self.rps()))}
      if self.tps() != 0 { try!(write!(f, " tps=0x{:x}", self.tps()))}
      if self.ebs() != 0 { try!(write!(f, " ebs=0x{:x}", self.ebs()))}
      if self.mmcs() != 0 { try!(write!(f, " mmcs"))}
      if self.pmts() != 0 { try!(write!(f, " pmts"))}
      if self.tsts() != 0 { try!(write!(f, " tsts"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmaomr(pub u32);
impl Dmaomr {
  #[inline]
  pub fn sr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_sr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn osf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_osf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtc(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  #[inline]
  pub fn set_rtc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn fugf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_fugf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn fef(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_fef(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn ttc(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x7 // [16:14]
  }
  #[inline]
  pub fn set_ttc(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn ftf(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
  pub fn set_ftf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn tsf(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  #[inline]
  pub fn set_tsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  #[inline]
  pub fn dfrf(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_dfrf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn rsf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_rsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn dtcefd(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_dtcefd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}
impl ::core::fmt::Display for Dmaomr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmaomr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sr() != 0 { try!(write!(f, " sr"))}
      if self.osf() != 0 { try!(write!(f, " osf"))}
      if self.rtc() != 0 { try!(write!(f, " rtc=0x{:x}", self.rtc()))}
      if self.fugf() != 0 { try!(write!(f, " fugf"))}
      if self.fef() != 0 { try!(write!(f, " fef"))}
      if self.st() != 0 { try!(write!(f, " st"))}
      if self.ttc() != 0 { try!(write!(f, " ttc=0x{:x}", self.ttc()))}
      if self.ftf() != 0 { try!(write!(f, " ftf"))}
      if self.tsf() != 0 { try!(write!(f, " tsf"))}
      if self.dfrf() != 0 { try!(write!(f, " dfrf"))}
      if self.rsf() != 0 { try!(write!(f, " rsf"))}
      if self.dtcefd() != 0 { try!(write!(f, " dtcefd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmaier(pub u32);
impl Dmaier {
  #[inline]
  pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tpsie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tpsie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn tbuie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_tbuie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn tjtie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_tjtie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn roie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_roie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn tuie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_tuie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn rie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_rie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn rbuie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_rbuie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn rpsie(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_rpsie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn rwtie(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_rwtie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn etie(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_etie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn fbeie(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_fbeie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn erie(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_erie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn aise(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_aise(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn nise(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_nise(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Dmaier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmaier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.tpsie() != 0 { try!(write!(f, " tpsie"))}
      if self.tbuie() != 0 { try!(write!(f, " tbuie"))}
      if self.tjtie() != 0 { try!(write!(f, " tjtie"))}
      if self.roie() != 0 { try!(write!(f, " roie"))}
      if self.tuie() != 0 { try!(write!(f, " tuie"))}
      if self.rie() != 0 { try!(write!(f, " rie"))}
      if self.rbuie() != 0 { try!(write!(f, " rbuie"))}
      if self.rpsie() != 0 { try!(write!(f, " rpsie"))}
      if self.rwtie() != 0 { try!(write!(f, " rwtie"))}
      if self.etie() != 0 { try!(write!(f, " etie"))}
      if self.fbeie() != 0 { try!(write!(f, " fbeie"))}
      if self.erie() != 0 { try!(write!(f, " erie"))}
      if self.aise() != 0 { try!(write!(f, " aise"))}
      if self.nise() != 0 { try!(write!(f, " nise"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmamfbocr(pub u32);
impl Dmamfbocr {
  #[inline]
  pub fn mfc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_mfc(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn omfc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_omfc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn mfa(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7ff // [27:17]
  }
  #[inline]
  pub fn set_mfa(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn ofoc(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  #[inline]
  pub fn set_ofoc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Dmamfbocr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmamfbocr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mfc() != 0 { try!(write!(f, " mfc=0x{:x}", self.mfc()))}
      if self.omfc() != 0 { try!(write!(f, " omfc"))}
      if self.mfa() != 0 { try!(write!(f, " mfa=0x{:x}", self.mfa()))}
      if self.ofoc() != 0 { try!(write!(f, " ofoc"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmarswtr(pub u32);
impl Dmarswtr {
  #[inline]
  pub fn rswtc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_rswtc(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmarswtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmarswtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rswtc() != 0 { try!(write!(f, " rswtc=0x{:x}", self.rswtc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmachtdr(pub u32);
impl Dmachtdr {
  #[inline]
  pub fn htdap(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_htdap(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmachtdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmachtdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmachrdr(pub u32);
impl Dmachrdr {
  #[inline]
  pub fn hrdap(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_hrdap(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmachrdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmachrdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmachtbar(pub u32);
impl Dmachtbar {
  #[inline]
  pub fn htbap(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_htbap(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmachtbar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmachtbar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmachrbar(pub u32);
impl Dmachrbar {
  #[inline]
  pub fn hrbap(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_hrbap(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmachrbar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmachrbar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

