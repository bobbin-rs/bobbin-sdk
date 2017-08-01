//! Reset and clock control
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

#[doc="Get the *const pointer for the AHBRSTR register."]
  #[inline] pub fn ahbrstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the AHBRSTR register."]
  #[inline] pub fn ahbrstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the AHBRSTR register."]
  #[inline] pub fn ahbrstr(&self) -> Ahbrstr { 
     unsafe {
        Ahbrstr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the AHBRSTR register."]
  #[inline] pub fn set_ahbrstr(&self, value: Ahbrstr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHBRSTR register."]
  #[inline] pub fn with_ahbrstr<F: FnOnce(Ahbrstr) -> Ahbrstr>(&self, f: F) -> &Self {
     let tmp = self.ahbrstr();
     self.set_ahbrstr(f(tmp))
  }

#[doc="Get the *const pointer for the CFGR2 register."]
  #[inline] pub fn cfgr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR2 register."]
  #[inline] pub fn cfgr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Read the CFGR2 register."]
  #[inline] pub fn cfgr2(&self) -> Cfgr2 { 
     unsafe {
        Cfgr2(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
#[doc="Write the CFGR2 register."]
  #[inline] pub fn set_cfgr2(&self, value: Cfgr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR2 register."]
  #[inline] pub fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
     let tmp = self.cfgr2();
     self.set_cfgr2(f(tmp))
  }

#[doc="Get the *const pointer for the CFGR3 register."]
  #[inline] pub fn cfgr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR3 register."]
  #[inline] pub fn cfgr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the CFGR3 register."]
  #[inline] pub fn cfgr3(&self) -> Cfgr3 { 
     unsafe {
        Cfgr3(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
#[doc="Write the CFGR3 register."]
  #[inline] pub fn set_cfgr3(&self, value: Cfgr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR3 register."]
  #[inline] pub fn with_cfgr3<F: FnOnce(Cfgr3) -> Cfgr3>(&self, f: F) -> &Self {
     let tmp = self.cfgr3();
     self.set_cfgr3(f(tmp))
  }

}

#[doc="Clock control register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Internal High Speed clock enable"]
  #[inline] pub fn hsion(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Internal High Speed clock enable"]
  #[inline] pub fn set_hsion(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Internal High Speed clock ready flag"]
  #[inline] pub fn hsirdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Internal High Speed clock ready flag"]
  #[inline] pub fn set_hsirdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Internal High Speed clock trimming"]
  #[inline] pub fn hsitrim(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1f // [7:3]
  }
#[doc="Internal High Speed clock trimming"]
  #[inline] pub fn set_hsitrim(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Internal High Speed clock Calibration"]
  #[inline] pub fn hsical(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
#[doc="Internal High Speed clock Calibration"]
  #[inline] pub fn set_hsical(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="External High Speed clock enable"]
  #[inline] pub fn hseon(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="External High Speed clock enable"]
  #[inline] pub fn set_hseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="External High Speed clock ready flag"]
  #[inline] pub fn hserdy(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="External High Speed clock ready flag"]
  #[inline] pub fn set_hserdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="External High Speed clock Bypass"]
  #[inline] pub fn hsebyp(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="External High Speed clock Bypass"]
  #[inline] pub fn set_hsebyp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Clock Security System enable"]
  #[inline] pub fn csson(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Clock Security System enable"]
  #[inline] pub fn set_csson(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="PLL enable"]
  #[inline] pub fn pllon(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="PLL enable"]
  #[inline] pub fn set_pllon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="PLL clock ready flag"]
  #[inline] pub fn pllrdy(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="PLL clock ready flag"]
  #[inline] pub fn set_pllrdy(mut self, value: u32) -> Self {
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
  #[inline] pub fn sw(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="System clock Switch"]
  #[inline] pub fn set_sw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="System Clock Switch Status"]
  #[inline] pub fn sws(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
#[doc="System Clock Switch Status"]
  #[inline] pub fn set_sws(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="AHB prescaler"]
  #[inline] pub fn hpre(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
#[doc="AHB prescaler"]
  #[inline] pub fn set_hpre(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="APB Low speed prescaler (APB1)"]
  #[inline] pub fn ppre1(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="APB Low speed prescaler (APB1)"]
  #[inline] pub fn set_ppre1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="APB high speed prescaler (APB2)"]
  #[inline] pub fn ppre2(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x7 // [13:11]
  }
#[doc="APB high speed prescaler (APB2)"]
  #[inline] pub fn set_ppre2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="PLL entry clock source"]
  #[inline] pub fn pllsrc(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x3 // [16:15]
  }
#[doc="PLL entry clock source"]
  #[inline] pub fn set_pllsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="HSE divider for PLL entry"]
  #[inline] pub fn pllxtpre(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="HSE divider for PLL entry"]
  #[inline] pub fn set_pllxtpre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="PLL Multiplication Factor"]
  #[inline] pub fn pllmul(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0xf // [21:18]
  }
#[doc="PLL Multiplication Factor"]
  #[inline] pub fn set_pllmul(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 18);
     self.0 |= value << 18;
     self
  }

#[doc="USB prescaler"]
  #[inline] pub fn usbpres(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="USB prescaler"]
  #[inline] pub fn set_usbpres(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Microcontroller clock output"]
  #[inline] pub fn mco(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Microcontroller clock output"]
  #[inline] pub fn set_mco(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Microcontroller Clock Output Flag"]
  #[inline] pub fn mcof(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Microcontroller Clock Output Flag"]
  #[inline] pub fn set_mcof(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Microcontroller clock output prescaler"]
  #[inline] pub fn mcopre(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x7 // [31:29]
  }
#[doc="Microcontroller clock output prescaler"]
  #[inline] pub fn set_mcopre(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="I2S external clock source selection"]
  #[inline] pub fn i2ssrc(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="I2S external clock source selection"]
  #[inline] pub fn set_i2ssrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
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
      if self.pllsrc() != 0 { try!(write!(f, " pllsrc=0x{:x}", self.pllsrc()))}
      if self.pllxtpre() != 0 { try!(write!(f, " pllxtpre"))}
      if self.pllmul() != 0 { try!(write!(f, " pllmul=0x{:x}", self.pllmul()))}
      if self.usbpres() != 0 { try!(write!(f, " usbpres"))}
      if self.mco() != 0 { try!(write!(f, " mco=0x{:x}", self.mco()))}
      if self.mcof() != 0 { try!(write!(f, " mcof"))}
      if self.mcopre() != 0 { try!(write!(f, " mcopre=0x{:x}", self.mcopre()))}
      if self.i2ssrc() != 0 { try!(write!(f, " i2ssrc"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock interrupt register (RCC_CIR)"]
#[derive(PartialEq, Eq)]
pub struct Cir(pub u32);
impl Cir {
#[doc="LSI Ready Interrupt flag"]
  #[inline] pub fn lsirdyf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="LSI Ready Interrupt flag"]
  #[inline] pub fn set_lsirdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="LSE Ready Interrupt flag"]
  #[inline] pub fn lserdyf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="LSE Ready Interrupt flag"]
  #[inline] pub fn set_lserdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="HSI Ready Interrupt flag"]
  #[inline] pub fn hsirdyf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="HSI Ready Interrupt flag"]
  #[inline] pub fn set_hsirdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="HSE Ready Interrupt flag"]
  #[inline] pub fn hserdyf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="HSE Ready Interrupt flag"]
  #[inline] pub fn set_hserdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="PLL Ready Interrupt flag"]
  #[inline] pub fn pllrdyf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="PLL Ready Interrupt flag"]
  #[inline] pub fn set_pllrdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Clock Security System Interrupt flag"]
  #[inline] pub fn cssf(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Clock Security System Interrupt flag"]
  #[inline] pub fn set_cssf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="LSI Ready Interrupt Enable"]
  #[inline] pub fn lsirdyie(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="LSI Ready Interrupt Enable"]
  #[inline] pub fn set_lsirdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="LSE Ready Interrupt Enable"]
  #[inline] pub fn lserdyie(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="LSE Ready Interrupt Enable"]
  #[inline] pub fn set_lserdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="HSI Ready Interrupt Enable"]
  #[inline] pub fn hsirdyie(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="HSI Ready Interrupt Enable"]
  #[inline] pub fn set_hsirdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="HSE Ready Interrupt Enable"]
  #[inline] pub fn hserdyie(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="HSE Ready Interrupt Enable"]
  #[inline] pub fn set_hserdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="PLL Ready Interrupt Enable"]
  #[inline] pub fn pllrdyie(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="PLL Ready Interrupt Enable"]
  #[inline] pub fn set_pllrdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="LSI Ready Interrupt Clear"]
  #[inline] pub fn lsirdyc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="LSI Ready Interrupt Clear"]
  #[inline] pub fn set_lsirdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="LSE Ready Interrupt Clear"]
  #[inline] pub fn lserdyc(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="LSE Ready Interrupt Clear"]
  #[inline] pub fn set_lserdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="HSI Ready Interrupt Clear"]
  #[inline] pub fn hsirdyc(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="HSI Ready Interrupt Clear"]
  #[inline] pub fn set_hsirdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="HSE Ready Interrupt Clear"]
  #[inline] pub fn hserdyc(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="HSE Ready Interrupt Clear"]
  #[inline] pub fn set_hserdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="PLL Ready Interrupt Clear"]
  #[inline] pub fn pllrdyc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="PLL Ready Interrupt Clear"]
  #[inline] pub fn set_pllrdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Clock security system interrupt clear"]
  #[inline] pub fn cssc(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Clock security system interrupt clear"]
  #[inline] pub fn set_cssc(mut self, value: u32) -> Self {
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
#[doc="SYSCFG and COMP reset"]
  #[inline] pub fn syscfgrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="SYSCFG and COMP reset"]
  #[inline] pub fn set_syscfgrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TIM1 timer reset"]
  #[inline] pub fn tim1rst(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="TIM1 timer reset"]
  #[inline] pub fn set_tim1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="SPI 1 reset"]
  #[inline] pub fn spi1rst(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="SPI 1 reset"]
  #[inline] pub fn set_spi1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="TIM8 timer reset"]
  #[inline] pub fn tim8rst(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="TIM8 timer reset"]
  #[inline] pub fn set_tim8rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="USART1 reset"]
  #[inline] pub fn usart1rst(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="USART1 reset"]
  #[inline] pub fn set_usart1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="TIM15 timer reset"]
  #[inline] pub fn tim15rst(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="TIM15 timer reset"]
  #[inline] pub fn set_tim15rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="TIM16 timer reset"]
  #[inline] pub fn tim16rst(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="TIM16 timer reset"]
  #[inline] pub fn set_tim16rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="TIM17 timer reset"]
  #[inline] pub fn tim17rst(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="TIM17 timer reset"]
  #[inline] pub fn set_tim17rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
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
      if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
      if self.tim1rst() != 0 { try!(write!(f, " tim1rst"))}
      if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
      if self.tim8rst() != 0 { try!(write!(f, " tim8rst"))}
      if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
      if self.tim15rst() != 0 { try!(write!(f, " tim15rst"))}
      if self.tim16rst() != 0 { try!(write!(f, " tim16rst"))}
      if self.tim17rst() != 0 { try!(write!(f, " tim17rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB1 peripheral reset register (RCC_APB1RSTR)"]
#[derive(PartialEq, Eq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
#[doc="Timer 2 reset"]
  #[inline] pub fn tim2rst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Timer 2 reset"]
  #[inline] pub fn set_tim2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Timer 3 reset"]
  #[inline] pub fn tim3rst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Timer 3 reset"]
  #[inline] pub fn set_tim3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Timer 14 reset"]
  #[inline] pub fn tim4rst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Timer 14 reset"]
  #[inline] pub fn set_tim4rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Timer 6 reset"]
  #[inline] pub fn tim6rst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Timer 6 reset"]
  #[inline] pub fn set_tim6rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Timer 7 reset"]
  #[inline] pub fn tim7rst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Timer 7 reset"]
  #[inline] pub fn set_tim7rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Window watchdog reset"]
  #[inline] pub fn wwdgrst(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Window watchdog reset"]
  #[inline] pub fn set_wwdgrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="SPI2 reset"]
  #[inline] pub fn spi2rst(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="SPI2 reset"]
  #[inline] pub fn set_spi2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="SPI3 reset"]
  #[inline] pub fn spi3rst(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="SPI3 reset"]
  #[inline] pub fn set_spi3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="USART 2 reset"]
  #[inline] pub fn usart2rst(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="USART 2 reset"]
  #[inline] pub fn set_usart2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="USART3 reset"]
  #[inline] pub fn usart3rst(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="USART3 reset"]
  #[inline] pub fn set_usart3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="UART 4 reset"]
  #[inline] pub fn uart4rst(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="UART 4 reset"]
  #[inline] pub fn set_uart4rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="UART 5 reset"]
  #[inline] pub fn uart5rst(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="UART 5 reset"]
  #[inline] pub fn set_uart5rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="I2C1 reset"]
  #[inline] pub fn i2c1rst(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="I2C1 reset"]
  #[inline] pub fn set_i2c1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="I2C2 reset"]
  #[inline] pub fn i2c2rst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="I2C2 reset"]
  #[inline] pub fn set_i2c2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="USB reset"]
  #[inline] pub fn usbrst(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="USB reset"]
  #[inline] pub fn set_usbrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="CAN reset"]
  #[inline] pub fn canrst(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="CAN reset"]
  #[inline] pub fn set_canrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Power interface reset"]
  #[inline] pub fn pwrrst(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Power interface reset"]
  #[inline] pub fn set_pwrrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DAC interface reset"]
  #[inline] pub fn dacrst(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="DAC interface reset"]
  #[inline] pub fn set_dacrst(mut self, value: u32) -> Self {
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
      if self.tim6rst() != 0 { try!(write!(f, " tim6rst"))}
      if self.tim7rst() != 0 { try!(write!(f, " tim7rst"))}
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
  #[inline] pub fn dmaen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="DMA1 clock enable"]
  #[inline] pub fn set_dmaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="DMA2 clock enable"]
  #[inline] pub fn dma2en(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="DMA2 clock enable"]
  #[inline] pub fn set_dma2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SRAM interface clock enable"]
  #[inline] pub fn sramen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="SRAM interface clock enable"]
  #[inline] pub fn set_sramen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="FLITF clock enable"]
  #[inline] pub fn flitfen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="FLITF clock enable"]
  #[inline] pub fn set_flitfen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="CRC clock enable"]
  #[inline] pub fn crcen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="CRC clock enable"]
  #[inline] pub fn set_crcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="I/O port H clock enable"]
  #[inline] pub fn iophen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="I/O port H clock enable"]
  #[inline] pub fn set_iophen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="I/O port A clock enable"]
  #[inline] pub fn iopaen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="I/O port A clock enable"]
  #[inline] pub fn set_iopaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="I/O port B clock enable"]
  #[inline] pub fn iopben(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="I/O port B clock enable"]
  #[inline] pub fn set_iopben(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="I/O port C clock enable"]
  #[inline] pub fn iopcen(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="I/O port C clock enable"]
  #[inline] pub fn set_iopcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="I/O port D clock enable"]
  #[inline] pub fn iopden(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="I/O port D clock enable"]
  #[inline] pub fn set_iopden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="I/O port E clock enable"]
  #[inline] pub fn iopeen(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="I/O port E clock enable"]
  #[inline] pub fn set_iopeen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="I/O port F clock enable"]
  #[inline] pub fn iopfen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="I/O port F clock enable"]
  #[inline] pub fn set_iopfen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="I/O port G clock enable"]
  #[inline] pub fn iopgen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="I/O port G clock enable"]
  #[inline] pub fn set_iopgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Touch sensing controller clock enable"]
  #[inline] pub fn tscen(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Touch sensing controller clock enable"]
  #[inline] pub fn set_tscen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="ADC1 and ADC2 clock enable"]
  #[inline] pub fn adc12en(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="ADC1 and ADC2 clock enable"]
  #[inline] pub fn set_adc12en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="ADC3 and ADC4 clock enable"]
  #[inline] pub fn adc34en(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="ADC3 and ADC4 clock enable"]
  #[inline] pub fn set_adc34en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
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
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
      if self.sramen() != 0 { try!(write!(f, " sramen"))}
      if self.flitfen() != 0 { try!(write!(f, " flitfen"))}
      if self.crcen() != 0 { try!(write!(f, " crcen"))}
      if self.iophen() != 0 { try!(write!(f, " iophen"))}
      if self.iopaen() != 0 { try!(write!(f, " iopaen"))}
      if self.iopben() != 0 { try!(write!(f, " iopben"))}
      if self.iopcen() != 0 { try!(write!(f, " iopcen"))}
      if self.iopden() != 0 { try!(write!(f, " iopden"))}
      if self.iopeen() != 0 { try!(write!(f, " iopeen"))}
      if self.iopfen() != 0 { try!(write!(f, " iopfen"))}
      if self.iopgen() != 0 { try!(write!(f, " iopgen"))}
      if self.tscen() != 0 { try!(write!(f, " tscen"))}
      if self.adc12en() != 0 { try!(write!(f, " adc12en"))}
      if self.adc34en() != 0 { try!(write!(f, " adc34en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB2 peripheral clock enable register (RCC_APB2ENR)"]
#[derive(PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
#[doc="SYSCFG clock enable"]
  #[inline] pub fn syscfgen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="SYSCFG clock enable"]
  #[inline] pub fn set_syscfgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TIM1 Timer clock enable"]
  #[inline] pub fn tim1en(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="TIM1 Timer clock enable"]
  #[inline] pub fn set_tim1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="SPI 1 clock enable"]
  #[inline] pub fn spi1en(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="SPI 1 clock enable"]
  #[inline] pub fn set_spi1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="TIM8 Timer clock enable"]
  #[inline] pub fn tim8en(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="TIM8 Timer clock enable"]
  #[inline] pub fn set_tim8en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="USART1 clock enable"]
  #[inline] pub fn usart1en(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="USART1 clock enable"]
  #[inline] pub fn set_usart1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="TIM15 timer clock enable"]
  #[inline] pub fn tim15en(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="TIM15 timer clock enable"]
  #[inline] pub fn set_tim15en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="TIM16 timer clock enable"]
  #[inline] pub fn tim16en(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="TIM16 timer clock enable"]
  #[inline] pub fn set_tim16en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="TIM17 timer clock enable"]
  #[inline] pub fn tim17en(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="TIM17 timer clock enable"]
  #[inline] pub fn set_tim17en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="TIM20 timer clock enable"]
  #[inline] pub fn tim20en(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="TIM20 timer clock enable"]
  #[inline] pub fn set_tim20en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
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
      if self.syscfgen() != 0 { try!(write!(f, " syscfgen"))}
      if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
      if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
      if self.tim8en() != 0 { try!(write!(f, " tim8en"))}
      if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
      if self.tim15en() != 0 { try!(write!(f, " tim15en"))}
      if self.tim16en() != 0 { try!(write!(f, " tim16en"))}
      if self.tim17en() != 0 { try!(write!(f, " tim17en"))}
      if self.tim20en() != 0 { try!(write!(f, " tim20en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB1 peripheral clock enable register (RCC_APB1ENR)"]
#[derive(PartialEq, Eq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
#[doc="Timer 2 clock enable"]
  #[inline] pub fn tim2en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Timer 2 clock enable"]
  #[inline] pub fn set_tim2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Timer 3 clock enable"]
  #[inline] pub fn tim3en(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Timer 3 clock enable"]
  #[inline] pub fn set_tim3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Timer 4 clock enable"]
  #[inline] pub fn tim4en(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Timer 4 clock enable"]
  #[inline] pub fn set_tim4en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Timer 6 clock enable"]
  #[inline] pub fn tim6en(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Timer 6 clock enable"]
  #[inline] pub fn set_tim6en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Timer 7 clock enable"]
  #[inline] pub fn tim7en(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Timer 7 clock enable"]
  #[inline] pub fn set_tim7en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Window watchdog clock enable"]
  #[inline] pub fn wwdgen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Window watchdog clock enable"]
  #[inline] pub fn set_wwdgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="SPI 2 clock enable"]
  #[inline] pub fn spi2en(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="SPI 2 clock enable"]
  #[inline] pub fn set_spi2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="SPI 3 clock enable"]
  #[inline] pub fn spi3en(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="SPI 3 clock enable"]
  #[inline] pub fn set_spi3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="USART 2 clock enable"]
  #[inline] pub fn usart2en(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="USART 2 clock enable"]
  #[inline] pub fn set_usart2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="USART 3 clock enable"]
  #[inline] pub fn usart3en(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="USART 3 clock enable"]
  #[inline] pub fn set_usart3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="UART 4 clock enable"]
  #[inline] pub fn uart4en(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="UART 4 clock enable"]
  #[inline] pub fn set_uart4en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="UART 5 clock enable"]
  #[inline] pub fn uart5en(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="UART 5 clock enable"]
  #[inline] pub fn set_uart5en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="I2C 1 clock enable"]
  #[inline] pub fn i2c1en(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="I2C 1 clock enable"]
  #[inline] pub fn set_i2c1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="I2C 2 clock enable"]
  #[inline] pub fn i2c2en(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="I2C 2 clock enable"]
  #[inline] pub fn set_i2c2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="USB clock enable"]
  #[inline] pub fn usben(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="USB clock enable"]
  #[inline] pub fn set_usben(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="CAN clock enable"]
  #[inline] pub fn canen(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="CAN clock enable"]
  #[inline] pub fn set_canen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Power interface clock enable"]
  #[inline] pub fn pwren(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Power interface clock enable"]
  #[inline] pub fn set_pwren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DAC interface clock enable"]
  #[inline] pub fn dacen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="DAC interface clock enable"]
  #[inline] pub fn set_dacen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="I2C 3 clock enable"]
  #[inline] pub fn i2c3en(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="I2C 3 clock enable"]
  #[inline] pub fn set_i2c3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
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
      if self.tim6en() != 0 { try!(write!(f, " tim6en"))}
      if self.tim7en() != 0 { try!(write!(f, " tim7en"))}
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
      if self.pwren() != 0 { try!(write!(f, " pwren"))}
      if self.dacen() != 0 { try!(write!(f, " dacen"))}
      if self.i2c3en() != 0 { try!(write!(f, " i2c3en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backup domain control register (RCC_BDCR)"]
#[derive(PartialEq, Eq)]
pub struct Bdcr(pub u32);
impl Bdcr {
#[doc="External Low Speed oscillator enable"]
  #[inline] pub fn lseon(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="External Low Speed oscillator enable"]
  #[inline] pub fn set_lseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="External Low Speed oscillator ready"]
  #[inline] pub fn lserdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="External Low Speed oscillator ready"]
  #[inline] pub fn set_lserdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="External Low Speed oscillator bypass"]
  #[inline] pub fn lsebyp(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="External Low Speed oscillator bypass"]
  #[inline] pub fn set_lsebyp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="LSE oscillator drive capability"]
  #[inline] pub fn lsedrv(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
#[doc="LSE oscillator drive capability"]
  #[inline] pub fn set_lsedrv(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="RTC clock source selection"]
  #[inline] pub fn rtcsel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
#[doc="RTC clock source selection"]
  #[inline] pub fn set_rtcsel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="RTC clock enable"]
  #[inline] pub fn rtcen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="RTC clock enable"]
  #[inline] pub fn set_rtcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Backup domain software reset"]
  #[inline] pub fn bdrst(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Backup domain software reset"]
  #[inline] pub fn set_bdrst(mut self, value: u32) -> Self {
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
      if self.lsedrv() != 0 { try!(write!(f, " lsedrv=0x{:x}", self.lsedrv()))}
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
  #[inline] pub fn lsion(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Internal low speed oscillator enable"]
  #[inline] pub fn set_lsion(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Internal low speed oscillator ready"]
  #[inline] pub fn lsirdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Internal low speed oscillator ready"]
  #[inline] pub fn set_lsirdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Remove reset flag"]
  #[inline] pub fn rmvf(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Remove reset flag"]
  #[inline] pub fn set_rmvf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Option byte loader reset flag"]
  #[inline] pub fn oblrstf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Option byte loader reset flag"]
  #[inline] pub fn set_oblrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="PIN reset flag"]
  #[inline] pub fn pinrstf(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="PIN reset flag"]
  #[inline] pub fn set_pinrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="POR/PDR reset flag"]
  #[inline] pub fn porrstf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="POR/PDR reset flag"]
  #[inline] pub fn set_porrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Software reset flag"]
  #[inline] pub fn sftrstf(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Software reset flag"]
  #[inline] pub fn set_sftrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Independent watchdog reset flag"]
  #[inline] pub fn iwdgrstf(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="Independent watchdog reset flag"]
  #[inline] pub fn set_iwdgrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Window watchdog reset flag"]
  #[inline] pub fn wwdgrstf(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Window watchdog reset flag"]
  #[inline] pub fn set_wwdgrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Low-power reset flag"]
  #[inline] pub fn lpwrrstf(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Low-power reset flag"]
  #[inline] pub fn set_lpwrrstf(mut self, value: u32) -> Self {
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
      if self.oblrstf() != 0 { try!(write!(f, " oblrstf"))}
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
#[doc="AHB peripheral reset register"]
#[derive(PartialEq, Eq)]
pub struct Ahbrstr(pub u32);
impl Ahbrstr {
#[doc="I/O port A reset"]
  #[inline] pub fn ioparst(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="I/O port A reset"]
  #[inline] pub fn set_ioparst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="I/O port B reset"]
  #[inline] pub fn iopbrst(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="I/O port B reset"]
  #[inline] pub fn set_iopbrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="I/O port C reset"]
  #[inline] pub fn iopcrst(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="I/O port C reset"]
  #[inline] pub fn set_iopcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="I/O port D reset"]
  #[inline] pub fn iopdrst(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="I/O port D reset"]
  #[inline] pub fn set_iopdrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="I/O port E reset"]
  #[inline] pub fn ioperst(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="I/O port E reset"]
  #[inline] pub fn set_ioperst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="I/O port F reset"]
  #[inline] pub fn iopfrst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="I/O port F reset"]
  #[inline] pub fn set_iopfrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Touch sensing controller reset"]
  #[inline] pub fn tscrst(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Touch sensing controller reset"]
  #[inline] pub fn set_tscrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="ADC1 and ADC2 reset"]
  #[inline] pub fn adc12rst(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="ADC1 and ADC2 reset"]
  #[inline] pub fn set_adc12rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="ADC3 and ADC4 reset"]
  #[inline] pub fn adc34rst(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="ADC3 and ADC4 reset"]
  #[inline] pub fn set_adc34rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
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
      if self.ioparst() != 0 { try!(write!(f, " ioparst"))}
      if self.iopbrst() != 0 { try!(write!(f, " iopbrst"))}
      if self.iopcrst() != 0 { try!(write!(f, " iopcrst"))}
      if self.iopdrst() != 0 { try!(write!(f, " iopdrst"))}
      if self.ioperst() != 0 { try!(write!(f, " ioperst"))}
      if self.iopfrst() != 0 { try!(write!(f, " iopfrst"))}
      if self.tscrst() != 0 { try!(write!(f, " tscrst"))}
      if self.adc12rst() != 0 { try!(write!(f, " adc12rst"))}
      if self.adc34rst() != 0 { try!(write!(f, " adc34rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock configuration register 2"]
#[derive(PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
#[doc="PREDIV division factor"]
  #[inline] pub fn prediv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="PREDIV division factor"]
  #[inline] pub fn set_prediv(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="ADC1 and ADC2 prescaler"]
  #[inline] pub fn adc12pres(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1f // [8:4]
  }
#[doc="ADC1 and ADC2 prescaler"]
  #[inline] pub fn set_adc12pres(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADC3 and ADC4 prescaler"]
  #[inline] pub fn adc34pres(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1f // [13:9]
  }
#[doc="ADC3 and ADC4 prescaler"]
  #[inline] pub fn set_adc34pres(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 9);
     self.0 |= value << 9;
     self
  }

}
impl ::core::fmt::Display for Cfgr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prediv() != 0 { try!(write!(f, " prediv=0x{:x}", self.prediv()))}
      if self.adc12pres() != 0 { try!(write!(f, " adc12pres=0x{:x}", self.adc12pres()))}
      if self.adc34pres() != 0 { try!(write!(f, " adc34pres=0x{:x}", self.adc34pres()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock configuration register 3"]
#[derive(PartialEq, Eq)]
pub struct Cfgr3(pub u32);
impl Cfgr3 {
#[doc="USART1 clock source selection"]
  #[inline] pub fn usart1sw(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="USART1 clock source selection"]
  #[inline] pub fn set_usart1sw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="I2C1 clock source selection"]
  #[inline] pub fn i2c1sw(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="I2C1 clock source selection"]
  #[inline] pub fn set_i2c1sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="I2C2 clock source selection"]
  #[inline] pub fn i2c2sw(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="I2C2 clock source selection"]
  #[inline] pub fn set_i2c2sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="I2C3 clock source selection"]
  #[inline] pub fn i2c3sw(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="I2C3 clock source selection"]
  #[inline] pub fn set_i2c3sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="USART2 clock source selection"]
  #[inline] pub fn usart2sw(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
#[doc="USART2 clock source selection"]
  #[inline] pub fn set_usart2sw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="USART3 clock source selection"]
  #[inline] pub fn usart3sw(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x3 // [19:18]
  }
#[doc="USART3 clock source selection"]
  #[inline] pub fn set_usart3sw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Timer1 clock source selection"]
  #[inline] pub fn tim1sw(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Timer1 clock source selection"]
  #[inline] pub fn set_tim1sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Timer8 clock source selection"]
  #[inline] pub fn tim8sw(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Timer8 clock source selection"]
  #[inline] pub fn set_tim8sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="UART4 clock source selection"]
  #[inline] pub fn uart4sw(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="UART4 clock source selection"]
  #[inline] pub fn set_uart4sw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="UART5 clock source selection"]
  #[inline] pub fn uart5sw(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
#[doc="UART5 clock source selection"]
  #[inline] pub fn set_uart5sw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

}
impl ::core::fmt::Display for Cfgr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.usart1sw() != 0 { try!(write!(f, " usart1sw=0x{:x}", self.usart1sw()))}
      if self.i2c1sw() != 0 { try!(write!(f, " i2c1sw"))}
      if self.i2c2sw() != 0 { try!(write!(f, " i2c2sw"))}
      if self.i2c3sw() != 0 { try!(write!(f, " i2c3sw"))}
      if self.usart2sw() != 0 { try!(write!(f, " usart2sw=0x{:x}", self.usart2sw()))}
      if self.usart3sw() != 0 { try!(write!(f, " usart3sw=0x{:x}", self.usart3sw()))}
      if self.tim1sw() != 0 { try!(write!(f, " tim1sw"))}
      if self.tim8sw() != 0 { try!(write!(f, " tim8sw"))}
      if self.uart4sw() != 0 { try!(write!(f, " uart4sw=0x{:x}", self.uart4sw()))}
      if self.uart5sw() != 0 { try!(write!(f, " uart5sw=0x{:x}", self.uart5sw()))}
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
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().dmaen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_dmaen(value)); }
}

impl En for super::dma::Dma2 {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().dma2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_dma2en(value)); }
}

impl En for super::crc::Crc {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().crcen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_crcen(value)); }
}

impl En for super::gpio::Gpioa {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().iopaen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_iopaen(value)); }
}

impl En for super::gpio::Gpiob {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().iopben() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_iopben(value)); }
}

impl En for super::gpio::Gpioc {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().iopcen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_iopcen(value)); }
}

impl En for super::gpio::Gpiod {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().iopden() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_iopden(value)); }
}

impl En for super::gpio::Gpiof {
   #[inline] fn en(&self) -> u32 { RCC.ahbenr().iopfen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_iopfen(value)); }
}

impl En for super::syscfg::Syscfg {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().syscfgen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

impl En for super::tim_adv::Tim1 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim1en(value)); }
}

impl En for super::spi::Spi1 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().spi1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::usart::Usart1 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().usart1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_usart1en(value)); }
}

impl En for super::tim_gen::Tim15 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim15en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim15en(value)); }
}

impl En for super::tim_gen::Tim16 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim16en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim16en(value)); }
}

impl En for super::tim_gen::Tim17 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim17en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim17en(value)); }
}

impl En for super::tim_gen::Tim2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim2en(value)); }
}

impl En for super::tim_bas::Tim6 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim6en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim6en(value)); }
}

impl En for super::wwdg::Wwdg {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().wwdgen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

impl En for super::spi::Spi2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().spi2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_spi2en(value)); }
}

impl En for super::spi::Spi3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().spi3en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_spi3en(value)); }
}

impl En for super::usart::Usart2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().usart2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_usart2en(value)); }
}

impl En for super::usart::Usart3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().usart3en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_usart3en(value)); }
}

impl En for super::usart::Uart4 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().uart4en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_uart4en(value)); }
}

impl En for super::usart::Uart5 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().uart5en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_uart5en(value)); }
}

impl En for super::i2c::I2c1 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().i2c1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c1en(value)); }
}

impl En for super::i2c::I2c2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().i2c2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c2en(value)); }
}

impl En for super::pwr::Pwr {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().pwren() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_pwren(value)); }
}

impl En for super::i2c::I2c3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().i2c3en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c3en(value)); }
}

