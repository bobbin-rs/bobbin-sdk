//! Reset and clock control
#[allow(unused_imports)] use bobbin_common::bits;
pub const RCC: Rcc = Rcc(0x40021000);

#[doc="Reset and clock control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcc(pub u32);
impl Rcc {
#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the CFGR register."]
  #[inline] pub fn cfgr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR register."]
  #[inline] pub fn cfgr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CFGR register."]
  #[inline] pub fn cfgr(&self) -> Cfgr { 
     unsafe {
        Cfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CFGR register."]
  #[inline] pub fn set_cfgr(&self, value: Cfgr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR register."]
  #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
     let tmp = self.cfgr();
     self.set_cfgr(f(tmp))
  }

#[doc="Get the *const pointer for the CIR register."]
  #[inline] pub fn cir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CIR register."]
  #[inline] pub fn cir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CIR register."]
  #[inline] pub fn cir(&self) -> Cir { 
     unsafe {
        Cir(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CIR register."]
  #[inline] pub fn set_cir(&self, value: Cir) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CIR register."]
  #[inline] pub fn with_cir<F: FnOnce(Cir) -> Cir>(&self, f: F) -> &Self {
     let tmp = self.cir();
     self.set_cir(f(tmp))
  }

#[doc="Get the *const pointer for the APB2RSTR register."]
  #[inline] pub fn apb2rstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the APB2RSTR register."]
  #[inline] pub fn apb2rstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the APB2RSTR register."]
  #[inline] pub fn apb2rstr(&self) -> Apb2rstr { 
     unsafe {
        Apb2rstr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the APB2RSTR register."]
  #[inline] pub fn set_apb2rstr(&self, value: Apb2rstr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB2RSTR register."]
  #[inline] pub fn with_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Self {
     let tmp = self.apb2rstr();
     self.set_apb2rstr(f(tmp))
  }

#[doc="Get the *const pointer for the APB1RSTR register."]
  #[inline] pub fn apb1rstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the APB1RSTR register."]
  #[inline] pub fn apb1rstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the APB1RSTR register."]
  #[inline] pub fn apb1rstr(&self) -> Apb1rstr { 
     unsafe {
        Apb1rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the APB1RSTR register."]
  #[inline] pub fn set_apb1rstr(&self, value: Apb1rstr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB1RSTR register."]
  #[inline] pub fn with_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Self {
     let tmp = self.apb1rstr();
     self.set_apb1rstr(f(tmp))
  }

#[doc="Get the *const pointer for the AHBENR register."]
  #[inline] pub fn ahbenr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the AHBENR register."]
  #[inline] pub fn ahbenr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the AHBENR register."]
  #[inline] pub fn ahbenr(&self) -> Ahbenr { 
     unsafe {
        Ahbenr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the AHBENR register."]
  #[inline] pub fn set_ahbenr(&self, value: Ahbenr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHBENR register."]
  #[inline] pub fn with_ahbenr<F: FnOnce(Ahbenr) -> Ahbenr>(&self, f: F) -> &Self {
     let tmp = self.ahbenr();
     self.set_ahbenr(f(tmp))
  }

#[doc="Get the *const pointer for the APB2ENR register."]
  #[inline] pub fn apb2enr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the APB2ENR register."]
  #[inline] pub fn apb2enr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the APB2ENR register."]
  #[inline] pub fn apb2enr(&self) -> Apb2enr { 
     unsafe {
        Apb2enr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the APB2ENR register."]
  #[inline] pub fn set_apb2enr(&self, value: Apb2enr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB2ENR register."]
  #[inline] pub fn with_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Self {
     let tmp = self.apb2enr();
     self.set_apb2enr(f(tmp))
  }

#[doc="Get the *const pointer for the APB1ENR register."]
  #[inline] pub fn apb1enr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the APB1ENR register."]
  #[inline] pub fn apb1enr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the APB1ENR register."]
  #[inline] pub fn apb1enr(&self) -> Apb1enr { 
     unsafe {
        Apb1enr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the APB1ENR register."]
  #[inline] pub fn set_apb1enr(&self, value: Apb1enr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB1ENR register."]
  #[inline] pub fn with_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Self {
     let tmp = self.apb1enr();
     self.set_apb1enr(f(tmp))
  }

#[doc="Get the *const pointer for the BDCR register."]
  #[inline] pub fn bdcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the BDCR register."]
  #[inline] pub fn bdcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the BDCR register."]
  #[inline] pub fn bdcr(&self) -> Bdcr { 
     unsafe {
        Bdcr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the BDCR register."]
  #[inline] pub fn set_bdcr(&self, value: Bdcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BDCR register."]
  #[inline] pub fn with_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&self, f: F) -> &Self {
     let tmp = self.bdcr();
     self.set_bdcr(f(tmp))
  }

#[doc="Get the *const pointer for the CSR register."]
  #[inline] pub fn csr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the CSR register."]
  #[inline] pub fn csr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the CSR register."]
  #[inline] pub fn csr(&self) -> Csr { 
     unsafe {
        Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the CSR register."]
  #[inline] pub fn set_csr(&self, value: Csr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CSR register."]
  #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

}

#[doc="Clock control register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Internal High Speed clock enable"]
  #[inline] pub fn hsion(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Internal High Speed clock enable"]
  #[inline] pub fn set_hsion<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Internal High Speed clock ready flag"]
  #[inline] pub fn hsirdy(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Internal High Speed clock ready flag"]
  #[inline] pub fn set_hsirdy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Internal High Speed clock trimming"]
  #[inline] pub fn hsitrim(&self) -> bits::B5 {
     (((self.0 as u32) >> 3) & 0x1f).into() // [7:3]
  }
#[doc="Internal High Speed clock trimming"]
  #[inline] pub fn set_hsitrim<V: Into<bits::B5>>(mut self, value: V) -> Self {
     let value: bits::B5 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Internal High Speed clock Calibration"]
  #[inline] pub fn hsical(&self) -> bits::B8 {
     (((self.0 as u32) >> 8) & 0xff).into() // [15:8]
  }
#[doc="Internal High Speed clock Calibration"]
  #[inline] pub fn set_hsical<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="External High Speed clock enable"]
  #[inline] pub fn hseon(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="External High Speed clock enable"]
  #[inline] pub fn set_hseon<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="External High Speed clock ready flag"]
  #[inline] pub fn hserdy(&self) -> bits::B1 {
     (((self.0 as u32) >> 17) & 0x1).into() // [17]
  }
#[doc="External High Speed clock ready flag"]
  #[inline] pub fn set_hserdy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="External High Speed clock Bypass"]
  #[inline] pub fn hsebyp(&self) -> bits::B1 {
     (((self.0 as u32) >> 18) & 0x1).into() // [18]
  }
#[doc="External High Speed clock Bypass"]
  #[inline] pub fn set_hsebyp<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Clock Security System enable"]
  #[inline] pub fn csson(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="Clock Security System enable"]
  #[inline] pub fn set_csson<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="PLL enable"]
  #[inline] pub fn pllon(&self) -> bits::B1 {
     (((self.0 as u32) >> 24) & 0x1).into() // [24]
  }
#[doc="PLL enable"]
  #[inline] pub fn set_pllon<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="PLL clock ready flag"]
  #[inline] pub fn pllrdy(&self) -> bits::B1 {
     (((self.0 as u32) >> 25) & 0x1).into() // [25]
  }
#[doc="PLL clock ready flag"]
  #[inline] pub fn set_pllrdy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
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
      if self.hsion() != 0 { try!(write!(f, " hsion"))}
      if self.hsirdy() != 0 { try!(write!(f, " hsirdy"))}
      if self.hsitrim() != 0 { try!(write!(f, " hsitrim=0x{:x}", self.hsitrim()))}
      if self.hsical() != 0 { try!(write!(f, " hsical=0x{:x}", self.hsical()))}
      if self.hseon() != 0 { try!(write!(f, " hseon"))}
      if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
      if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
      if self.csson() != 0 { try!(write!(f, " csson"))}
      if self.pllon() != 0 { try!(write!(f, " pllon"))}
      if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock configuration register (RCC_CFGR)"]
#[derive(PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
#[doc="System clock Switch"]
  #[inline] pub fn sw(&self) -> bits::B2 {
     (((self.0 as u32) >> 0) & 0x3).into() // [1:0]
  }
#[doc="System clock Switch"]
  #[inline] pub fn set_sw<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="System Clock Switch Status"]
  #[inline] pub fn sws(&self) -> bits::B2 {
     (((self.0 as u32) >> 2) & 0x3).into() // [3:2]
  }
#[doc="System Clock Switch Status"]
  #[inline] pub fn set_sws<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="AHB prescaler"]
  #[inline] pub fn hpre(&self) -> bits::B4 {
     (((self.0 as u32) >> 4) & 0xf).into() // [7:4]
  }
#[doc="AHB prescaler"]
  #[inline] pub fn set_hpre<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="APB Low speed prescaler (APB1)"]
  #[inline] pub fn ppre1(&self) -> bits::B3 {
     (((self.0 as u32) >> 8) & 0x7).into() // [10:8]
  }
#[doc="APB Low speed prescaler (APB1)"]
  #[inline] pub fn set_ppre1<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="APB High speed prescaler (APB2)"]
  #[inline] pub fn ppre2(&self) -> bits::B3 {
     (((self.0 as u32) >> 11) & 0x7).into() // [13:11]
  }
#[doc="APB High speed prescaler (APB2)"]
  #[inline] pub fn set_ppre2<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="ADC prescaler"]
  #[inline] pub fn adcpre(&self) -> bits::B2 {
     (((self.0 as u32) >> 14) & 0x3).into() // [15:14]
  }
#[doc="ADC prescaler"]
  #[inline] pub fn set_adcpre<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="PLL entry clock source"]
  #[inline] pub fn pllsrc(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="PLL entry clock source"]
  #[inline] pub fn set_pllsrc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="HSE divider for PLL entry"]
  #[inline] pub fn pllxtpre(&self) -> bits::B1 {
     (((self.0 as u32) >> 17) & 0x1).into() // [17]
  }
#[doc="HSE divider for PLL entry"]
  #[inline] pub fn set_pllxtpre<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="PLL Multiplication Factor"]
  #[inline] pub fn pllmul(&self) -> bits::B4 {
     (((self.0 as u32) >> 18) & 0xf).into() // [21:18]
  }
#[doc="PLL Multiplication Factor"]
  #[inline] pub fn set_pllmul<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 18);
     self.0 |= value << 18;
     self
  }

#[doc="USB OTG FS prescaler"]
  #[inline] pub fn otgfspre(&self) -> bits::B1 {
     (((self.0 as u32) >> 22) & 0x1).into() // [22]
  }
#[doc="USB OTG FS prescaler"]
  #[inline] pub fn set_otgfspre<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Microcontroller clock output"]
  #[inline] pub fn mco(&self) -> bits::B3 {
     (((self.0 as u32) >> 24) & 0x7).into() // [26:24]
  }
#[doc="Microcontroller clock output"]
  #[inline] pub fn set_mco<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
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
      if self.sw() != 0 { try!(write!(f, " sw=0x{:x}", self.sw()))}
      if self.sws() != 0 { try!(write!(f, " sws=0x{:x}", self.sws()))}
      if self.hpre() != 0 { try!(write!(f, " hpre=0x{:x}", self.hpre()))}
      if self.ppre1() != 0 { try!(write!(f, " ppre1=0x{:x}", self.ppre1()))}
      if self.ppre2() != 0 { try!(write!(f, " ppre2=0x{:x}", self.ppre2()))}
      if self.adcpre() != 0 { try!(write!(f, " adcpre=0x{:x}", self.adcpre()))}
      if self.pllsrc() != 0 { try!(write!(f, " pllsrc"))}
      if self.pllxtpre() != 0 { try!(write!(f, " pllxtpre"))}
      if self.pllmul() != 0 { try!(write!(f, " pllmul=0x{:x}", self.pllmul()))}
      if self.otgfspre() != 0 { try!(write!(f, " otgfspre"))}
      if self.mco() != 0 { try!(write!(f, " mco=0x{:x}", self.mco()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock interrupt register (RCC_CIR)"]
#[derive(PartialEq, Eq)]
pub struct Cir(pub u32);
impl Cir {
#[doc="LSI Ready Interrupt flag"]
  #[inline] pub fn lsirdyf(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="LSI Ready Interrupt flag"]
  #[inline] pub fn set_lsirdyf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="LSE Ready Interrupt flag"]
  #[inline] pub fn lserdyf(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="LSE Ready Interrupt flag"]
  #[inline] pub fn set_lserdyf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="HSI Ready Interrupt flag"]
  #[inline] pub fn hsirdyf(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="HSI Ready Interrupt flag"]
  #[inline] pub fn set_hsirdyf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="HSE Ready Interrupt flag"]
  #[inline] pub fn hserdyf(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="HSE Ready Interrupt flag"]
  #[inline] pub fn set_hserdyf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="PLL Ready Interrupt flag"]
  #[inline] pub fn pllrdyf(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="PLL Ready Interrupt flag"]
  #[inline] pub fn set_pllrdyf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Clock Security System Interrupt flag"]
  #[inline] pub fn cssf(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Clock Security System Interrupt flag"]
  #[inline] pub fn set_cssf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="LSI Ready Interrupt Enable"]
  #[inline] pub fn lsirdyie(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="LSI Ready Interrupt Enable"]
  #[inline] pub fn set_lsirdyie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="LSE Ready Interrupt Enable"]
  #[inline] pub fn lserdyie(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="LSE Ready Interrupt Enable"]
  #[inline] pub fn set_lserdyie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="HSI Ready Interrupt Enable"]
  #[inline] pub fn hsirdyie(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="HSI Ready Interrupt Enable"]
  #[inline] pub fn set_hsirdyie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="HSE Ready Interrupt Enable"]
  #[inline] pub fn hserdyie(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="HSE Ready Interrupt Enable"]
  #[inline] pub fn set_hserdyie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="PLL Ready Interrupt Enable"]
  #[inline] pub fn pllrdyie(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="PLL Ready Interrupt Enable"]
  #[inline] pub fn set_pllrdyie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="LSI Ready Interrupt Clear"]
  #[inline] pub fn lsirdyc(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="LSI Ready Interrupt Clear"]
  #[inline] pub fn set_lsirdyc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="LSE Ready Interrupt Clear"]
  #[inline] pub fn lserdyc(&self) -> bits::B1 {
     (((self.0 as u32) >> 17) & 0x1).into() // [17]
  }
#[doc="LSE Ready Interrupt Clear"]
  #[inline] pub fn set_lserdyc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="HSI Ready Interrupt Clear"]
  #[inline] pub fn hsirdyc(&self) -> bits::B1 {
     (((self.0 as u32) >> 18) & 0x1).into() // [18]
  }
#[doc="HSI Ready Interrupt Clear"]
  #[inline] pub fn set_hsirdyc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="HSE Ready Interrupt Clear"]
  #[inline] pub fn hserdyc(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="HSE Ready Interrupt Clear"]
  #[inline] pub fn set_hserdyc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="PLL Ready Interrupt Clear"]
  #[inline] pub fn pllrdyc(&self) -> bits::B1 {
     (((self.0 as u32) >> 20) & 0x1).into() // [20]
  }
#[doc="PLL Ready Interrupt Clear"]
  #[inline] pub fn set_pllrdyc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Clock security system interrupt clear"]
  #[inline] pub fn cssc(&self) -> bits::B1 {
     (((self.0 as u32) >> 23) & 0x1).into() // [23]
  }
#[doc="Clock security system interrupt clear"]
  #[inline] pub fn set_cssc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

}
impl ::core::fmt::Display for Cir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lsirdyf() != 0 { try!(write!(f, " lsirdyf"))}
      if self.lserdyf() != 0 { try!(write!(f, " lserdyf"))}
      if self.hsirdyf() != 0 { try!(write!(f, " hsirdyf"))}
      if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
      if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
      if self.cssf() != 0 { try!(write!(f, " cssf"))}
      if self.lsirdyie() != 0 { try!(write!(f, " lsirdyie"))}
      if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
      if self.hsirdyie() != 0 { try!(write!(f, " hsirdyie"))}
      if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
      if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
      if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
      if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
      if self.hsirdyc() != 0 { try!(write!(f, " hsirdyc"))}
      if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
      if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
      if self.cssc() != 0 { try!(write!(f, " cssc"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB2 peripheral reset register (RCC_APB2RSTR)"]
#[derive(PartialEq, Eq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
#[doc="Alternate function I/O reset"]
  #[inline] pub fn afiorst(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Alternate function I/O reset"]
  #[inline] pub fn set_afiorst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="IO port A reset"]
  #[inline] pub fn ioparst(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="IO port A reset"]
  #[inline] pub fn set_ioparst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="IO port B reset"]
  #[inline] pub fn iopbrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="IO port B reset"]
  #[inline] pub fn set_iopbrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="IO port C reset"]
  #[inline] pub fn iopcrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="IO port C reset"]
  #[inline] pub fn set_iopcrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="IO port D reset"]
  #[inline] pub fn iopdrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="IO port D reset"]
  #[inline] pub fn set_iopdrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="IO port E reset"]
  #[inline] pub fn ioperst(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="IO port E reset"]
  #[inline] pub fn set_ioperst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="IO port F reset"]
  #[inline] pub fn iopfrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="IO port F reset"]
  #[inline] pub fn set_iopfrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="IO port G reset"]
  #[inline] pub fn iopgrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="IO port G reset"]
  #[inline] pub fn set_iopgrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADC 1 interface reset"]
  #[inline] pub fn adc1rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="ADC 1 interface reset"]
  #[inline] pub fn set_adc1rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="ADC 2 interface reset"]
  #[inline] pub fn adc2rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="ADC 2 interface reset"]
  #[inline] pub fn set_adc2rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="TIM1 timer reset"]
  #[inline] pub fn tim1rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="TIM1 timer reset"]
  #[inline] pub fn set_tim1rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="SPI 1 reset"]
  #[inline] pub fn spi1rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="SPI 1 reset"]
  #[inline] pub fn set_spi1rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="TIM8 timer reset"]
  #[inline] pub fn tim8rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 13) & 0x1).into() // [13]
  }
#[doc="TIM8 timer reset"]
  #[inline] pub fn set_tim8rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="USART1 reset"]
  #[inline] pub fn usart1rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 14) & 0x1).into() // [14]
  }
#[doc="USART1 reset"]
  #[inline] pub fn set_usart1rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="ADC 3 interface reset"]
  #[inline] pub fn adc3rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="ADC 3 interface reset"]
  #[inline] pub fn set_adc3rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="TIM9 timer reset"]
  #[inline] pub fn tim9rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="TIM9 timer reset"]
  #[inline] pub fn set_tim9rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="TIM10 timer reset"]
  #[inline] pub fn tim10rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 20) & 0x1).into() // [20]
  }
#[doc="TIM10 timer reset"]
  #[inline] pub fn set_tim10rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="TIM11 timer reset"]
  #[inline] pub fn tim11rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 21) & 0x1).into() // [21]
  }
#[doc="TIM11 timer reset"]
  #[inline] pub fn set_tim11rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
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
      if self.afiorst() != 0 { try!(write!(f, " afiorst"))}
      if self.ioparst() != 0 { try!(write!(f, " ioparst"))}
      if self.iopbrst() != 0 { try!(write!(f, " iopbrst"))}
      if self.iopcrst() != 0 { try!(write!(f, " iopcrst"))}
      if self.iopdrst() != 0 { try!(write!(f, " iopdrst"))}
      if self.ioperst() != 0 { try!(write!(f, " ioperst"))}
      if self.iopfrst() != 0 { try!(write!(f, " iopfrst"))}
      if self.iopgrst() != 0 { try!(write!(f, " iopgrst"))}
      if self.adc1rst() != 0 { try!(write!(f, " adc1rst"))}
      if self.adc2rst() != 0 { try!(write!(f, " adc2rst"))}
      if self.tim1rst() != 0 { try!(write!(f, " tim1rst"))}
      if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
      if self.tim8rst() != 0 { try!(write!(f, " tim8rst"))}
      if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
      if self.adc3rst() != 0 { try!(write!(f, " adc3rst"))}
      if self.tim9rst() != 0 { try!(write!(f, " tim9rst"))}
      if self.tim10rst() != 0 { try!(write!(f, " tim10rst"))}
      if self.tim11rst() != 0 { try!(write!(f, " tim11rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB1 peripheral reset register (RCC_APB1RSTR)"]
#[derive(PartialEq, Eq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
#[doc="Timer 2 reset"]
  #[inline] pub fn tim2rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Timer 2 reset"]
  #[inline] pub fn set_tim2rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Timer 3 reset"]
  #[inline] pub fn tim3rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Timer 3 reset"]
  #[inline] pub fn set_tim3rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Timer 4 reset"]
  #[inline] pub fn tim4rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Timer 4 reset"]
  #[inline] pub fn set_tim4rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Timer 5 reset"]
  #[inline] pub fn tim5rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Timer 5 reset"]
  #[inline] pub fn set_tim5rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Timer 6 reset"]
  #[inline] pub fn tim6rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Timer 6 reset"]
  #[inline] pub fn set_tim6rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Timer 7 reset"]
  #[inline] pub fn tim7rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Timer 7 reset"]
  #[inline] pub fn set_tim7rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Timer 12 reset"]
  #[inline] pub fn tim12rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="Timer 12 reset"]
  #[inline] pub fn set_tim12rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Timer 13 reset"]
  #[inline] pub fn tim13rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Timer 13 reset"]
  #[inline] pub fn set_tim13rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Timer 14 reset"]
  #[inline] pub fn tim14rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Timer 14 reset"]
  #[inline] pub fn set_tim14rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Window watchdog reset"]
  #[inline] pub fn wwdgrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="Window watchdog reset"]
  #[inline] pub fn set_wwdgrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="SPI2 reset"]
  #[inline] pub fn spi2rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 14) & 0x1).into() // [14]
  }
#[doc="SPI2 reset"]
  #[inline] pub fn set_spi2rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="SPI3 reset"]
  #[inline] pub fn spi3rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="SPI3 reset"]
  #[inline] pub fn set_spi3rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="USART 2 reset"]
  #[inline] pub fn usart2rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 17) & 0x1).into() // [17]
  }
#[doc="USART 2 reset"]
  #[inline] pub fn set_usart2rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="USART 3 reset"]
  #[inline] pub fn usart3rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 18) & 0x1).into() // [18]
  }
#[doc="USART 3 reset"]
  #[inline] pub fn set_usart3rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="UART 4 reset"]
  #[inline] pub fn uart4rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="UART 4 reset"]
  #[inline] pub fn set_uart4rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="UART 5 reset"]
  #[inline] pub fn uart5rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 20) & 0x1).into() // [20]
  }
#[doc="UART 5 reset"]
  #[inline] pub fn set_uart5rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="I2C1 reset"]
  #[inline] pub fn i2c1rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 21) & 0x1).into() // [21]
  }
#[doc="I2C1 reset"]
  #[inline] pub fn set_i2c1rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="I2C2 reset"]
  #[inline] pub fn i2c2rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 22) & 0x1).into() // [22]
  }
#[doc="I2C2 reset"]
  #[inline] pub fn set_i2c2rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="USB reset"]
  #[inline] pub fn usbrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 23) & 0x1).into() // [23]
  }
#[doc="USB reset"]
  #[inline] pub fn set_usbrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="CAN reset"]
  #[inline] pub fn canrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 25) & 0x1).into() // [25]
  }
#[doc="CAN reset"]
  #[inline] pub fn set_canrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Backup interface reset"]
  #[inline] pub fn bkprst(&self) -> bits::B1 {
     (((self.0 as u32) >> 27) & 0x1).into() // [27]
  }
#[doc="Backup interface reset"]
  #[inline] pub fn set_bkprst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Power interface reset"]
  #[inline] pub fn pwrrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 28) & 0x1).into() // [28]
  }
#[doc="Power interface reset"]
  #[inline] pub fn set_pwrrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DAC interface reset"]
  #[inline] pub fn dacrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 29) & 0x1).into() // [29]
  }
#[doc="DAC interface reset"]
  #[inline] pub fn set_dacrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
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
      if self.tim2rst() != 0 { try!(write!(f, " tim2rst"))}
      if self.tim3rst() != 0 { try!(write!(f, " tim3rst"))}
      if self.tim4rst() != 0 { try!(write!(f, " tim4rst"))}
      if self.tim5rst() != 0 { try!(write!(f, " tim5rst"))}
      if self.tim6rst() != 0 { try!(write!(f, " tim6rst"))}
      if self.tim7rst() != 0 { try!(write!(f, " tim7rst"))}
      if self.tim12rst() != 0 { try!(write!(f, " tim12rst"))}
      if self.tim13rst() != 0 { try!(write!(f, " tim13rst"))}
      if self.tim14rst() != 0 { try!(write!(f, " tim14rst"))}
      if self.wwdgrst() != 0 { try!(write!(f, " wwdgrst"))}
      if self.spi2rst() != 0 { try!(write!(f, " spi2rst"))}
      if self.spi3rst() != 0 { try!(write!(f, " spi3rst"))}
      if self.usart2rst() != 0 { try!(write!(f, " usart2rst"))}
      if self.usart3rst() != 0 { try!(write!(f, " usart3rst"))}
      if self.uart4rst() != 0 { try!(write!(f, " uart4rst"))}
      if self.uart5rst() != 0 { try!(write!(f, " uart5rst"))}
      if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
      if self.i2c2rst() != 0 { try!(write!(f, " i2c2rst"))}
      if self.usbrst() != 0 { try!(write!(f, " usbrst"))}
      if self.canrst() != 0 { try!(write!(f, " canrst"))}
      if self.bkprst() != 0 { try!(write!(f, " bkprst"))}
      if self.pwrrst() != 0 { try!(write!(f, " pwrrst"))}
      if self.dacrst() != 0 { try!(write!(f, " dacrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB Peripheral Clock enable register (RCC_AHBENR)"]
#[derive(PartialEq, Eq)]
pub struct Ahbenr(pub u32);
impl Ahbenr {
#[doc="DMA1 clock enable"]
  #[inline] pub fn dma1en(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="DMA1 clock enable"]
  #[inline] pub fn set_dma1en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="DMA2 clock enable"]
  #[inline] pub fn dma2en(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="DMA2 clock enable"]
  #[inline] pub fn set_dma2en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SRAM interface clock enable"]
  #[inline] pub fn sramen(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="SRAM interface clock enable"]
  #[inline] pub fn set_sramen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="FLITF clock enable"]
  #[inline] pub fn flitfen(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="FLITF clock enable"]
  #[inline] pub fn set_flitfen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="CRC clock enable"]
  #[inline] pub fn crcen(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="CRC clock enable"]
  #[inline] pub fn set_crcen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="FSMC clock enable"]
  #[inline] pub fn fsmcen(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="FSMC clock enable"]
  #[inline] pub fn set_fsmcen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="SDIO clock enable"]
  #[inline] pub fn sdioen(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="SDIO clock enable"]
  #[inline] pub fn set_sdioen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
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
      if self.dma1en() != 0 { try!(write!(f, " dma1en"))}
      if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
      if self.sramen() != 0 { try!(write!(f, " sramen"))}
      if self.flitfen() != 0 { try!(write!(f, " flitfen"))}
      if self.crcen() != 0 { try!(write!(f, " crcen"))}
      if self.fsmcen() != 0 { try!(write!(f, " fsmcen"))}
      if self.sdioen() != 0 { try!(write!(f, " sdioen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB2 peripheral clock enable register (RCC_APB2ENR)"]
#[derive(PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
#[doc="Alternate function I/O clock enable"]
  #[inline] pub fn afioen(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Alternate function I/O clock enable"]
  #[inline] pub fn set_afioen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="I/O port A clock enable"]
  #[inline] pub fn iopaen(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="I/O port A clock enable"]
  #[inline] pub fn set_iopaen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="I/O port B clock enable"]
  #[inline] pub fn iopben(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="I/O port B clock enable"]
  #[inline] pub fn set_iopben<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="I/O port C clock enable"]
  #[inline] pub fn iopcen(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="I/O port C clock enable"]
  #[inline] pub fn set_iopcen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="I/O port D clock enable"]
  #[inline] pub fn iopden(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="I/O port D clock enable"]
  #[inline] pub fn set_iopden<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="I/O port E clock enable"]
  #[inline] pub fn iopeen(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="I/O port E clock enable"]
  #[inline] pub fn set_iopeen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="I/O port F clock enable"]
  #[inline] pub fn iopfen(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="I/O port F clock enable"]
  #[inline] pub fn set_iopfen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="I/O port G clock enable"]
  #[inline] pub fn iopgen(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="I/O port G clock enable"]
  #[inline] pub fn set_iopgen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADC 1 interface clock enable"]
  #[inline] pub fn adc1en(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="ADC 1 interface clock enable"]
  #[inline] pub fn set_adc1en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="ADC 2 interface clock enable"]
  #[inline] pub fn adc2en(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="ADC 2 interface clock enable"]
  #[inline] pub fn set_adc2en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="TIM1 Timer clock enable"]
  #[inline] pub fn tim1en(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="TIM1 Timer clock enable"]
  #[inline] pub fn set_tim1en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="SPI 1 clock enable"]
  #[inline] pub fn spi1en(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="SPI 1 clock enable"]
  #[inline] pub fn set_spi1en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="TIM8 Timer clock enable"]
  #[inline] pub fn tim8en(&self) -> bits::B1 {
     (((self.0 as u32) >> 13) & 0x1).into() // [13]
  }
#[doc="TIM8 Timer clock enable"]
  #[inline] pub fn set_tim8en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="USART1 clock enable"]
  #[inline] pub fn usart1en(&self) -> bits::B1 {
     (((self.0 as u32) >> 14) & 0x1).into() // [14]
  }
#[doc="USART1 clock enable"]
  #[inline] pub fn set_usart1en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="ADC3 interface clock enable"]
  #[inline] pub fn adc3en(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="ADC3 interface clock enable"]
  #[inline] pub fn set_adc3en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="TIM9 Timer clock enable"]
  #[inline] pub fn tim9en(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="TIM9 Timer clock enable"]
  #[inline] pub fn set_tim9en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="TIM10 Timer clock enable"]
  #[inline] pub fn tim10en(&self) -> bits::B1 {
     (((self.0 as u32) >> 20) & 0x1).into() // [20]
  }
#[doc="TIM10 Timer clock enable"]
  #[inline] pub fn set_tim10en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="TIM11 Timer clock enable"]
  #[inline] pub fn tim11en(&self) -> bits::B1 {
     (((self.0 as u32) >> 21) & 0x1).into() // [21]
  }
#[doc="TIM11 Timer clock enable"]
  #[inline] pub fn set_tim11en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
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
      if self.afioen() != 0 { try!(write!(f, " afioen"))}
      if self.iopaen() != 0 { try!(write!(f, " iopaen"))}
      if self.iopben() != 0 { try!(write!(f, " iopben"))}
      if self.iopcen() != 0 { try!(write!(f, " iopcen"))}
      if self.iopden() != 0 { try!(write!(f, " iopden"))}
      if self.iopeen() != 0 { try!(write!(f, " iopeen"))}
      if self.iopfen() != 0 { try!(write!(f, " iopfen"))}
      if self.iopgen() != 0 { try!(write!(f, " iopgen"))}
      if self.adc1en() != 0 { try!(write!(f, " adc1en"))}
      if self.adc2en() != 0 { try!(write!(f, " adc2en"))}
      if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
      if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
      if self.tim8en() != 0 { try!(write!(f, " tim8en"))}
      if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
      if self.adc3en() != 0 { try!(write!(f, " adc3en"))}
      if self.tim9en() != 0 { try!(write!(f, " tim9en"))}
      if self.tim10en() != 0 { try!(write!(f, " tim10en"))}
      if self.tim11en() != 0 { try!(write!(f, " tim11en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB1 peripheral clock enable register (RCC_APB1ENR)"]
#[derive(PartialEq, Eq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
#[doc="Timer 2 clock enable"]
  #[inline] pub fn tim2en(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Timer 2 clock enable"]
  #[inline] pub fn set_tim2en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Timer 3 clock enable"]
  #[inline] pub fn tim3en(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Timer 3 clock enable"]
  #[inline] pub fn set_tim3en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Timer 4 clock enable"]
  #[inline] pub fn tim4en(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Timer 4 clock enable"]
  #[inline] pub fn set_tim4en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Timer 5 clock enable"]
  #[inline] pub fn tim5en(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Timer 5 clock enable"]
  #[inline] pub fn set_tim5en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Timer 6 clock enable"]
  #[inline] pub fn tim6en(&self) -> bits::B1 {
     (((self.0 as u32) >> 4) & 0x1).into() // [4]
  }
#[doc="Timer 6 clock enable"]
  #[inline] pub fn set_tim6en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Timer 7 clock enable"]
  #[inline] pub fn tim7en(&self) -> bits::B1 {
     (((self.0 as u32) >> 5) & 0x1).into() // [5]
  }
#[doc="Timer 7 clock enable"]
  #[inline] pub fn set_tim7en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Timer 12 clock enable"]
  #[inline] pub fn tim12en(&self) -> bits::B1 {
     (((self.0 as u32) >> 6) & 0x1).into() // [6]
  }
#[doc="Timer 12 clock enable"]
  #[inline] pub fn set_tim12en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Timer 13 clock enable"]
  #[inline] pub fn tim13en(&self) -> bits::B1 {
     (((self.0 as u32) >> 7) & 0x1).into() // [7]
  }
#[doc="Timer 13 clock enable"]
  #[inline] pub fn set_tim13en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Timer 14 clock enable"]
  #[inline] pub fn tim14en(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Timer 14 clock enable"]
  #[inline] pub fn set_tim14en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Window watchdog clock enable"]
  #[inline] pub fn wwdgen(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="Window watchdog clock enable"]
  #[inline] pub fn set_wwdgen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="SPI 2 clock enable"]
  #[inline] pub fn spi2en(&self) -> bits::B1 {
     (((self.0 as u32) >> 14) & 0x1).into() // [14]
  }
#[doc="SPI 2 clock enable"]
  #[inline] pub fn set_spi2en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="SPI 3 clock enable"]
  #[inline] pub fn spi3en(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="SPI 3 clock enable"]
  #[inline] pub fn set_spi3en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="USART 2 clock enable"]
  #[inline] pub fn usart2en(&self) -> bits::B1 {
     (((self.0 as u32) >> 17) & 0x1).into() // [17]
  }
#[doc="USART 2 clock enable"]
  #[inline] pub fn set_usart2en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="USART 3 clock enable"]
  #[inline] pub fn usart3en(&self) -> bits::B1 {
     (((self.0 as u32) >> 18) & 0x1).into() // [18]
  }
#[doc="USART 3 clock enable"]
  #[inline] pub fn set_usart3en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="UART 4 clock enable"]
  #[inline] pub fn uart4en(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="UART 4 clock enable"]
  #[inline] pub fn set_uart4en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="UART 5 clock enable"]
  #[inline] pub fn uart5en(&self) -> bits::B1 {
     (((self.0 as u32) >> 20) & 0x1).into() // [20]
  }
#[doc="UART 5 clock enable"]
  #[inline] pub fn set_uart5en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="I2C 1 clock enable"]
  #[inline] pub fn i2c1en(&self) -> bits::B1 {
     (((self.0 as u32) >> 21) & 0x1).into() // [21]
  }
#[doc="I2C 1 clock enable"]
  #[inline] pub fn set_i2c1en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="I2C 2 clock enable"]
  #[inline] pub fn i2c2en(&self) -> bits::B1 {
     (((self.0 as u32) >> 22) & 0x1).into() // [22]
  }
#[doc="I2C 2 clock enable"]
  #[inline] pub fn set_i2c2en<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="USB clock enable"]
  #[inline] pub fn usben(&self) -> bits::B1 {
     (((self.0 as u32) >> 23) & 0x1).into() // [23]
  }
#[doc="USB clock enable"]
  #[inline] pub fn set_usben<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="CAN clock enable"]
  #[inline] pub fn canen(&self) -> bits::B1 {
     (((self.0 as u32) >> 25) & 0x1).into() // [25]
  }
#[doc="CAN clock enable"]
  #[inline] pub fn set_canen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Backup interface clock enable"]
  #[inline] pub fn bkpen(&self) -> bits::B1 {
     (((self.0 as u32) >> 27) & 0x1).into() // [27]
  }
#[doc="Backup interface clock enable"]
  #[inline] pub fn set_bkpen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Power interface clock enable"]
  #[inline] pub fn pwren(&self) -> bits::B1 {
     (((self.0 as u32) >> 28) & 0x1).into() // [28]
  }
#[doc="Power interface clock enable"]
  #[inline] pub fn set_pwren<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DAC interface clock enable"]
  #[inline] pub fn dacen(&self) -> bits::B1 {
     (((self.0 as u32) >> 29) & 0x1).into() // [29]
  }
#[doc="DAC interface clock enable"]
  #[inline] pub fn set_dacen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
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
      if self.tim2en() != 0 { try!(write!(f, " tim2en"))}
      if self.tim3en() != 0 { try!(write!(f, " tim3en"))}
      if self.tim4en() != 0 { try!(write!(f, " tim4en"))}
      if self.tim5en() != 0 { try!(write!(f, " tim5en"))}
      if self.tim6en() != 0 { try!(write!(f, " tim6en"))}
      if self.tim7en() != 0 { try!(write!(f, " tim7en"))}
      if self.tim12en() != 0 { try!(write!(f, " tim12en"))}
      if self.tim13en() != 0 { try!(write!(f, " tim13en"))}
      if self.tim14en() != 0 { try!(write!(f, " tim14en"))}
      if self.wwdgen() != 0 { try!(write!(f, " wwdgen"))}
      if self.spi2en() != 0 { try!(write!(f, " spi2en"))}
      if self.spi3en() != 0 { try!(write!(f, " spi3en"))}
      if self.usart2en() != 0 { try!(write!(f, " usart2en"))}
      if self.usart3en() != 0 { try!(write!(f, " usart3en"))}
      if self.uart4en() != 0 { try!(write!(f, " uart4en"))}
      if self.uart5en() != 0 { try!(write!(f, " uart5en"))}
      if self.i2c1en() != 0 { try!(write!(f, " i2c1en"))}
      if self.i2c2en() != 0 { try!(write!(f, " i2c2en"))}
      if self.usben() != 0 { try!(write!(f, " usben"))}
      if self.canen() != 0 { try!(write!(f, " canen"))}
      if self.bkpen() != 0 { try!(write!(f, " bkpen"))}
      if self.pwren() != 0 { try!(write!(f, " pwren"))}
      if self.dacen() != 0 { try!(write!(f, " dacen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backup domain control register (RCC_BDCR)"]
#[derive(PartialEq, Eq)]
pub struct Bdcr(pub u32);
impl Bdcr {
#[doc="External Low Speed oscillator enable"]
  #[inline] pub fn lseon(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="External Low Speed oscillator enable"]
  #[inline] pub fn set_lseon<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="External Low Speed oscillator ready"]
  #[inline] pub fn lserdy(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="External Low Speed oscillator ready"]
  #[inline] pub fn set_lserdy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="External Low Speed oscillator bypass"]
  #[inline] pub fn lsebyp(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="External Low Speed oscillator bypass"]
  #[inline] pub fn set_lsebyp<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="RTC clock source selection"]
  #[inline] pub fn rtcsel(&self) -> bits::B2 {
     (((self.0 as u32) >> 8) & 0x3).into() // [9:8]
  }
#[doc="RTC clock source selection"]
  #[inline] pub fn set_rtcsel<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="RTC clock enable"]
  #[inline] pub fn rtcen(&self) -> bits::B1 {
     (((self.0 as u32) >> 15) & 0x1).into() // [15]
  }
#[doc="RTC clock enable"]
  #[inline] pub fn set_rtcen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Backup domain software reset"]
  #[inline] pub fn bdrst(&self) -> bits::B1 {
     (((self.0 as u32) >> 16) & 0x1).into() // [16]
  }
#[doc="Backup domain software reset"]
  #[inline] pub fn set_bdrst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Bdcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bdcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lseon() != 0 { try!(write!(f, " lseon"))}
      if self.lserdy() != 0 { try!(write!(f, " lserdy"))}
      if self.lsebyp() != 0 { try!(write!(f, " lsebyp"))}
      if self.rtcsel() != 0 { try!(write!(f, " rtcsel=0x{:x}", self.rtcsel()))}
      if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
      if self.bdrst() != 0 { try!(write!(f, " bdrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control/status register (RCC_CSR)"]
#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
#[doc="Internal low speed oscillator enable"]
  #[inline] pub fn lsion(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Internal low speed oscillator enable"]
  #[inline] pub fn set_lsion<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Internal low speed oscillator ready"]
  #[inline] pub fn lsirdy(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Internal low speed oscillator ready"]
  #[inline] pub fn set_lsirdy<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Remove reset flag"]
  #[inline] pub fn rmvf(&self) -> bits::B1 {
     (((self.0 as u32) >> 24) & 0x1).into() // [24]
  }
#[doc="Remove reset flag"]
  #[inline] pub fn set_rmvf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="PIN reset flag"]
  #[inline] pub fn pinrstf(&self) -> bits::B1 {
     (((self.0 as u32) >> 26) & 0x1).into() // [26]
  }
#[doc="PIN reset flag"]
  #[inline] pub fn set_pinrstf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="POR/PDR reset flag"]
  #[inline] pub fn porrstf(&self) -> bits::B1 {
     (((self.0 as u32) >> 27) & 0x1).into() // [27]
  }
#[doc="POR/PDR reset flag"]
  #[inline] pub fn set_porrstf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Software reset flag"]
  #[inline] pub fn sftrstf(&self) -> bits::B1 {
     (((self.0 as u32) >> 28) & 0x1).into() // [28]
  }
#[doc="Software reset flag"]
  #[inline] pub fn set_sftrstf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Independent watchdog reset flag"]
  #[inline] pub fn iwdgrstf(&self) -> bits::B1 {
     (((self.0 as u32) >> 29) & 0x1).into() // [29]
  }
#[doc="Independent watchdog reset flag"]
  #[inline] pub fn set_iwdgrstf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Window watchdog reset flag"]
  #[inline] pub fn wwdgrstf(&self) -> bits::B1 {
     (((self.0 as u32) >> 30) & 0x1).into() // [30]
  }
#[doc="Window watchdog reset flag"]
  #[inline] pub fn set_wwdgrstf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Low-power reset flag"]
  #[inline] pub fn lpwrrstf(&self) -> bits::B1 {
     (((self.0 as u32) >> 31) & 0x1).into() // [31]
  }
#[doc="Low-power reset flag"]
  #[inline] pub fn set_lpwrrstf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
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
      if self.lsion() != 0 { try!(write!(f, " lsion"))}
      if self.lsirdy() != 0 { try!(write!(f, " lsirdy"))}
      if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
      if self.pinrstf() != 0 { try!(write!(f, " pinrstf"))}
      if self.porrstf() != 0 { try!(write!(f, " porrstf"))}
      if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
      if self.iwdgrstf() != 0 { try!(write!(f, " iwdgrstf"))}
      if self.wwdgrstf() != 0 { try!(write!(f, " wwdgrstf"))}
      if self.lpwrrstf() != 0 { try!(write!(f, " lpwrrstf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait En {
   fn en(&self) -> u32;
   fn set_en(&self, value: u32);
}

impl Rcc {
   #[inline] pub fn en<P: En>(&self, p: &P) -> u32 {
      p.en()
   }
   #[inline] pub fn set_en<P: En>(&self, p: &P, value: u32) {
      p.set_en(value)
   }
}

pub trait Rst {
   fn rst(&self) -> u32;
   fn set_rst(&self, value: u32);
}

impl Rcc {
   #[inline] pub fn rst<P: Rst>(&self, p: &P) -> u32 {
      p.rst()
   }
   #[inline] pub fn set_rst<P: Rst>(&self, p: &P, value: u32) {
      p.set_rst(value)
   }
}

impl En for super::dma::Dma1 {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().dma1en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_dma1en(value)); }
}

impl En for super::dma::Dma2 {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().dma2en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_dma2en(value)); }
}

impl En for super::crc::Crc {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().crcen().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_crcen(value)); }
}

impl En for super::afio::Afio {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().afioen().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_afioen(value)); }
}

impl En for super::gpio::Gpioa {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().iopaen().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_iopaen(value)); }
}

impl En for super::gpio::Gpiob {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().iopben().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_iopben(value)); }
}

impl En for super::gpio::Gpioc {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().iopcen().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_iopcen(value)); }
}

impl En for super::gpio::Gpiod {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().iopden().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_iopden(value)); }
}

impl En for super::gpio::Gpioe {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().iopeen().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_iopeen(value)); }
}

impl En for super::gpio::Gpiof {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().iopfen().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_iopfen(value)); }
}

impl En for super::gpio::Gpiog {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().iopgen().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_iopgen(value)); }
}

impl En for super::tim_adv::Tim1 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim1en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim1en(value)); }
}

impl En for super::usart::Usart1 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().usart1en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_usart1en(value)); }
}

impl En for super::tim_gen::Tim2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim2en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim2en(value)); }
}

impl En for super::tim_gen::Tim3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim3en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim3en(value)); }
}

impl En for super::tim_gen::Tim4 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim4en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim4en(value)); }
}

impl En for super::wwdg::Wwdg {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().wwdgen().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

impl En for super::usart::Usart2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().usart2en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_usart2en(value)); }
}

impl En for super::usart::Usart3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().usart3en().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_usart3en(value)); }
}

impl En for super::pwr::Pwr {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().pwren().into() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_pwren(value)); }
}


