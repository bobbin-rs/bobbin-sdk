pub const RCC: Rcc = Rcc(0x40023800);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcc(pub u32);

impl Rcc {
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

  pub unsafe fn pllcfgr(&self) -> Pllcfgr { 
     Pllcfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_pllcfgr(&mut self, value: Pllcfgr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_pllcfgr<F: FnOnce(Pllcfgr) -> Pllcfgr>(&mut self, f: F) {
     let tmp = self.pllcfgr();
     self.set_pllcfgr(f(tmp))
  }

  pub unsafe fn cfgr(&self) -> Cfgr { 
     Cfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_cfgr(&mut self, value: Cfgr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&mut self, f: F) {
     let tmp = self.cfgr();
     self.set_cfgr(f(tmp))
  }

  pub unsafe fn cir(&self) -> Cir { 
     Cir(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_cir(&mut self, value: Cir) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_cir<F: FnOnce(Cir) -> Cir>(&mut self, f: F) {
     let tmp = self.cir();
     self.set_cir(f(tmp))
  }

  pub unsafe fn ahb1rstr(&self) -> Ahb1rstr { 
     Ahb1rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_ahb1rstr(&mut self, value: Ahb1rstr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb1rstr<F: FnOnce(Ahb1rstr) -> Ahb1rstr>(&mut self, f: F) {
     let tmp = self.ahb1rstr();
     self.set_ahb1rstr(f(tmp))
  }

  pub unsafe fn ahb2rstr(&self) -> Ahb2rstr { 
     Ahb2rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_ahb2rstr(&mut self, value: Ahb2rstr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb2rstr<F: FnOnce(Ahb2rstr) -> Ahb2rstr>(&mut self, f: F) {
     let tmp = self.ahb2rstr();
     self.set_ahb2rstr(f(tmp))
  }

  pub unsafe fn ahb3rstr(&self) -> Ahb3rstr { 
     Ahb3rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_ahb3rstr(&mut self, value: Ahb3rstr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb3rstr<F: FnOnce(Ahb3rstr) -> Ahb3rstr>(&mut self, f: F) {
     let tmp = self.ahb3rstr();
     self.set_ahb3rstr(f(tmp))
  }

  pub unsafe fn apb1rstr(&self) -> Apb1rstr { 
     Apb1rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_apb1rstr(&mut self, value: Apb1rstr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&mut self, f: F) {
     let tmp = self.apb1rstr();
     self.set_apb1rstr(f(tmp))
  }

  pub unsafe fn apb2rstr(&self) -> Apb2rstr { 
     Apb2rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_apb2rstr(&mut self, value: Apb2rstr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&mut self, f: F) {
     let tmp = self.apb2rstr();
     self.set_apb2rstr(f(tmp))
  }

  pub unsafe fn ahb1enr(&self) -> Ahb1enr { 
     Ahb1enr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
  }
  pub unsafe fn set_ahb1enr(&mut self, value: Ahb1enr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb1enr<F: FnOnce(Ahb1enr) -> Ahb1enr>(&mut self, f: F) {
     let tmp = self.ahb1enr();
     self.set_ahb1enr(f(tmp))
  }

  pub unsafe fn ahb2enr(&self) -> Ahb2enr { 
     Ahb2enr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }
  pub unsafe fn set_ahb2enr(&mut self, value: Ahb2enr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb2enr<F: FnOnce(Ahb2enr) -> Ahb2enr>(&mut self, f: F) {
     let tmp = self.ahb2enr();
     self.set_ahb2enr(f(tmp))
  }

  pub unsafe fn ahb3enr(&self) -> Ahb3enr { 
     Ahb3enr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
  }
  pub unsafe fn set_ahb3enr(&mut self, value: Ahb3enr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb3enr<F: FnOnce(Ahb3enr) -> Ahb3enr>(&mut self, f: F) {
     let tmp = self.ahb3enr();
     self.set_ahb3enr(f(tmp))
  }

  pub unsafe fn apb1enr(&self) -> Apb1enr { 
     Apb1enr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
  }
  pub unsafe fn set_apb1enr(&mut self, value: Apb1enr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
  }
  pub unsafe fn with_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&mut self, f: F) {
     let tmp = self.apb1enr();
     self.set_apb1enr(f(tmp))
  }

  pub unsafe fn apb2enr(&self) -> Apb2enr { 
     Apb2enr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
  }
  pub unsafe fn set_apb2enr(&mut self, value: Apb2enr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
  }
  pub unsafe fn with_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&mut self, f: F) {
     let tmp = self.apb2enr();
     self.set_apb2enr(f(tmp))
  }

  pub unsafe fn ahb1lpenr(&self) -> Ahb1lpenr { 
     Ahb1lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
  }
  pub unsafe fn set_ahb1lpenr(&mut self, value: Ahb1lpenr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb1lpenr<F: FnOnce(Ahb1lpenr) -> Ahb1lpenr>(&mut self, f: F) {
     let tmp = self.ahb1lpenr();
     self.set_ahb1lpenr(f(tmp))
  }

  pub unsafe fn ahb2lpenr(&self) -> Ahb2lpenr { 
     Ahb2lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
  }
  pub unsafe fn set_ahb2lpenr(&mut self, value: Ahb2lpenr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb2lpenr<F: FnOnce(Ahb2lpenr) -> Ahb2lpenr>(&mut self, f: F) {
     let tmp = self.ahb2lpenr();
     self.set_ahb2lpenr(f(tmp))
  }

  pub unsafe fn ahb3lpenr(&self) -> Ahb3lpenr { 
     Ahb3lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
  }
  pub unsafe fn set_ahb3lpenr(&mut self, value: Ahb3lpenr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
  }
  pub unsafe fn with_ahb3lpenr<F: FnOnce(Ahb3lpenr) -> Ahb3lpenr>(&mut self, f: F) {
     let tmp = self.ahb3lpenr();
     self.set_ahb3lpenr(f(tmp))
  }

  pub unsafe fn apb1lpenr(&self) -> Apb1lpenr { 
     Apb1lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
  }
  pub unsafe fn set_apb1lpenr(&mut self, value: Apb1lpenr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
  }
  pub unsafe fn with_apb1lpenr<F: FnOnce(Apb1lpenr) -> Apb1lpenr>(&mut self, f: F) {
     let tmp = self.apb1lpenr();
     self.set_apb1lpenr(f(tmp))
  }

  pub unsafe fn apb2lpenr(&self) -> Apb2lpenr { 
     Apb2lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
  }
  pub unsafe fn set_apb2lpenr(&mut self, value: Apb2lpenr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
  }
  pub unsafe fn with_apb2lpenr<F: FnOnce(Apb2lpenr) -> Apb2lpenr>(&mut self, f: F) {
     let tmp = self.apb2lpenr();
     self.set_apb2lpenr(f(tmp))
  }

  pub unsafe fn bdcr(&self) -> Bdcr { 
     Bdcr(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
  }
  pub unsafe fn set_bdcr(&mut self, value: Bdcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x70) as *mut u32, value.0);
  }
  pub unsafe fn with_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&mut self, f: F) {
     let tmp = self.bdcr();
     self.set_bdcr(f(tmp))
  }

  pub unsafe fn csr(&self) -> Csr { 
     Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x74) as *const u32))
  }
  pub unsafe fn set_csr(&mut self, value: Csr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x74) as *mut u32, value.0);
  }
  pub unsafe fn with_csr<F: FnOnce(Csr) -> Csr>(&mut self, f: F) {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

  pub unsafe fn sscgr(&self) -> Sscgr { 
     Sscgr(::core::ptr::read_volatile(((self.0 as usize) + 0x80) as *const u32))
  }
  pub unsafe fn set_sscgr(&mut self, value: Sscgr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x80) as *mut u32, value.0);
  }
  pub unsafe fn with_sscgr<F: FnOnce(Sscgr) -> Sscgr>(&mut self, f: F) {
     let tmp = self.sscgr();
     self.set_sscgr(f(tmp))
  }

  pub unsafe fn plli2scfgr(&self) -> Plli2scfgr { 
     Plli2scfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
  }
  pub unsafe fn set_plli2scfgr(&mut self, value: Plli2scfgr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
  }
  pub unsafe fn with_plli2scfgr<F: FnOnce(Plli2scfgr) -> Plli2scfgr>(&mut self, f: F) {
     let tmp = self.plli2scfgr();
     self.set_plli2scfgr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);

impl Cr {
  pub fn plli2srdy(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_plli2srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn plli2son(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_plli2son(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

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

  pub fn csson(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_csson(mut self, value: u32) -> Self {
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

  pub fn hsical(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_hsical(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn hsitrim(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1f // [7:3]
  }
  pub fn set_hsitrim(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 3);
     self.0 |= value << 3;
     self
  }

  pub fn hsirdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_hsirdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn hsion(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_hsion(mut self, value: u32) -> Self {
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
      if self.plli2srdy() != 0 { try!(write!(f, " plli2srdy"))}
      if self.plli2son() != 0 { try!(write!(f, " plli2son"))}
      if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
      if self.pllon() != 0 { try!(write!(f, " pllon"))}
      if self.csson() != 0 { try!(write!(f, " csson"))}
      if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
      if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
      if self.hseon() != 0 { try!(write!(f, " hseon"))}
      if self.hsical() != 0 { try!(write!(f, " hsical=0x{:x}", self.hsical()))}
      if self.hsitrim() != 0 { try!(write!(f, " hsitrim=0x{:x}", self.hsitrim()))}
      if self.hsirdy() != 0 { try!(write!(f, " hsirdy"))}
      if self.hsion() != 0 { try!(write!(f, " hsion"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pllcfgr(pub u32);

impl Pllcfgr {
  pub fn pllq(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_pllq(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn pllsrc(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_pllsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn pllp(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_pllp(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn plln(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1ff // [14:6]
  }
  pub fn set_plln(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 6);
     self.0 |= value << 6;
     self
  }

  pub fn pllm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3f // [5:0]
  }
  pub fn set_pllm(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Pllcfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pllcfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pllq() != 0 { try!(write!(f, " pllq=0x{:x}", self.pllq()))}
      if self.pllsrc() != 0 { try!(write!(f, " pllsrc"))}
      if self.pllp() != 0 { try!(write!(f, " pllp=0x{:x}", self.pllp()))}
      if self.plln() != 0 { try!(write!(f, " plln=0x{:x}", self.plln()))}
      if self.pllm() != 0 { try!(write!(f, " pllm=0x{:x}", self.pllm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cfgr(pub u32);

impl Cfgr {
  pub fn mco2(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
  pub fn set_mco2(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn mco2pre(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x7 // [29:27]
  }
  pub fn set_mco2pre(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn mco1pre(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_mco1pre(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn i2ssrc(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_i2ssrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn mco1(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x3 // [22:21]
  }
  pub fn set_mco1(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn rtcpre(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1f // [20:16]
  }
  pub fn set_rtcpre(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ppre2(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  pub fn set_ppre2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn ppre1(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x7 // [12:10]
  }
  pub fn set_ppre1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 10);
     self.0 |= value << 10;
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

  pub fn sws1(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_sws1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn sws0(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_sws0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn sw1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_sw1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn sw0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_sw0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
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
      if self.mco2() != 0 { try!(write!(f, " mco2=0x{:x}", self.mco2()))}
      if self.mco2pre() != 0 { try!(write!(f, " mco2pre=0x{:x}", self.mco2pre()))}
      if self.mco1pre() != 0 { try!(write!(f, " mco1pre=0x{:x}", self.mco1pre()))}
      if self.i2ssrc() != 0 { try!(write!(f, " i2ssrc"))}
      if self.mco1() != 0 { try!(write!(f, " mco1=0x{:x}", self.mco1()))}
      if self.rtcpre() != 0 { try!(write!(f, " rtcpre=0x{:x}", self.rtcpre()))}
      if self.ppre2() != 0 { try!(write!(f, " ppre2=0x{:x}", self.ppre2()))}
      if self.ppre1() != 0 { try!(write!(f, " ppre1=0x{:x}", self.ppre1()))}
      if self.hpre() != 0 { try!(write!(f, " hpre=0x{:x}", self.hpre()))}
      if self.sws1() != 0 { try!(write!(f, " sws1"))}
      if self.sws0() != 0 { try!(write!(f, " sws0"))}
      if self.sw1() != 0 { try!(write!(f, " sw1"))}
      if self.sw0() != 0 { try!(write!(f, " sw0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cir(pub u32);

impl Cir {
  pub fn cssc(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_cssc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn pllsairdyc(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_pllsairdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn plli2srdyc(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_plli2srdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn pllrdyc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_pllrdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn hserdyc(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_hserdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn hsirdyc(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_hsirdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn lserdyc(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_lserdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn lsirdyc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_lsirdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn pllsairdyie(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_pllsairdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn plli2srdyie(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_plli2srdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn pllrdyie(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_pllrdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn hserdyie(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_hserdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn hsirdyie(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_hsirdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn lserdyie(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_lserdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn lsirdyie(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_lsirdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cssf(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_cssf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn pllsairdyf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_pllsairdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn plli2srdyf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_plli2srdyf(mut self, value: u32) -> Self {
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

  pub fn hsirdyf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_hsirdyf(mut self, value: u32) -> Self {
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

impl ::core::fmt::Display for Cir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cssc() != 0 { try!(write!(f, " cssc"))}
      if self.pllsairdyc() != 0 { try!(write!(f, " pllsairdyc"))}
      if self.plli2srdyc() != 0 { try!(write!(f, " plli2srdyc"))}
      if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
      if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
      if self.hsirdyc() != 0 { try!(write!(f, " hsirdyc"))}
      if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
      if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
      if self.pllsairdyie() != 0 { try!(write!(f, " pllsairdyie"))}
      if self.plli2srdyie() != 0 { try!(write!(f, " plli2srdyie"))}
      if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
      if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
      if self.hsirdyie() != 0 { try!(write!(f, " hsirdyie"))}
      if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
      if self.lsirdyie() != 0 { try!(write!(f, " lsirdyie"))}
      if self.cssf() != 0 { try!(write!(f, " cssf"))}
      if self.pllsairdyf() != 0 { try!(write!(f, " pllsairdyf"))}
      if self.plli2srdyf() != 0 { try!(write!(f, " plli2srdyf"))}
      if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
      if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
      if self.hsirdyf() != 0 { try!(write!(f, " hsirdyf"))}
      if self.lserdyf() != 0 { try!(write!(f, " lserdyf"))}
      if self.lsirdyf() != 0 { try!(write!(f, " lsirdyf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb1rstr(pub u32);

impl Ahb1rstr {
  pub fn otghsrst(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_otghsrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn ethmacrst(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_ethmacrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn dma2drst(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_dma2drst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn dma2rst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_dma2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn dma1rst(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_dma1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
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

  pub fn gpiokrst(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_gpiokrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn gpiojrst(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_gpiojrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn gpioirst(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_gpioirst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn gpiohrst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_gpiohrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn gpiogrst(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_gpiogrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn gpiofrst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_gpiofrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn gpioerst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_gpioerst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn gpiodrst(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_gpiodrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn gpiocrst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_gpiocrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn gpiobrst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_gpiobrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn gpioarst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_gpioarst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ahb1rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb1rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otghsrst() != 0 { try!(write!(f, " otghsrst"))}
      if self.ethmacrst() != 0 { try!(write!(f, " ethmacrst"))}
      if self.dma2drst() != 0 { try!(write!(f, " dma2drst"))}
      if self.dma2rst() != 0 { try!(write!(f, " dma2rst"))}
      if self.dma1rst() != 0 { try!(write!(f, " dma1rst"))}
      if self.crcrst() != 0 { try!(write!(f, " crcrst"))}
      if self.gpiokrst() != 0 { try!(write!(f, " gpiokrst"))}
      if self.gpiojrst() != 0 { try!(write!(f, " gpiojrst"))}
      if self.gpioirst() != 0 { try!(write!(f, " gpioirst"))}
      if self.gpiohrst() != 0 { try!(write!(f, " gpiohrst"))}
      if self.gpiogrst() != 0 { try!(write!(f, " gpiogrst"))}
      if self.gpiofrst() != 0 { try!(write!(f, " gpiofrst"))}
      if self.gpioerst() != 0 { try!(write!(f, " gpioerst"))}
      if self.gpiodrst() != 0 { try!(write!(f, " gpiodrst"))}
      if self.gpiocrst() != 0 { try!(write!(f, " gpiocrst"))}
      if self.gpiobrst() != 0 { try!(write!(f, " gpiobrst"))}
      if self.gpioarst() != 0 { try!(write!(f, " gpioarst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb2rstr(pub u32);

impl Ahb2rstr {
  pub fn otgfsrst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_otgfsrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rngrst(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_rngrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn dcmirst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_dcmirst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ahb2rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb2rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otgfsrst() != 0 { try!(write!(f, " otgfsrst"))}
      if self.rngrst() != 0 { try!(write!(f, " rngrst"))}
      if self.dcmirst() != 0 { try!(write!(f, " dcmirst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb3rstr(pub u32);

impl Ahb3rstr {
  pub fn fmcrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_fmcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ahb3rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb3rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fmcrst() != 0 { try!(write!(f, " fmcrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Apb1rstr(pub u32);

impl Apb1rstr {
  pub fn tim2rst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tim3rst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tim3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tim4rst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tim4rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tim5rst(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tim5rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
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

  pub fn tim7rst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tim7rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tim12rst(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tim12rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tim13rst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_tim13rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn tim14rst(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_tim14rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn wwdgrst(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_wwdgrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
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

  pub fn spi3rst(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_spi3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn uart2rst(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_uart2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn uart3rst(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_uart3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn uart4rst(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_uart4rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn uart5rst(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_uart5rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
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

  pub fn i2c2rst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_i2c2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn i2c3rst(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_i2c3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn can1rst(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_can1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn can2rst(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_can2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
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

  pub fn dacrst(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_dacrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn uart7rst(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_uart7rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn uart8rst(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_uart8rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
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
      if self.uart2rst() != 0 { try!(write!(f, " uart2rst"))}
      if self.uart3rst() != 0 { try!(write!(f, " uart3rst"))}
      if self.uart4rst() != 0 { try!(write!(f, " uart4rst"))}
      if self.uart5rst() != 0 { try!(write!(f, " uart5rst"))}
      if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
      if self.i2c2rst() != 0 { try!(write!(f, " i2c2rst"))}
      if self.i2c3rst() != 0 { try!(write!(f, " i2c3rst"))}
      if self.can1rst() != 0 { try!(write!(f, " can1rst"))}
      if self.can2rst() != 0 { try!(write!(f, " can2rst"))}
      if self.pwrrst() != 0 { try!(write!(f, " pwrrst"))}
      if self.dacrst() != 0 { try!(write!(f, " dacrst"))}
      if self.uart7rst() != 0 { try!(write!(f, " uart7rst"))}
      if self.uart8rst() != 0 { try!(write!(f, " uart8rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Apb2rstr(pub u32);

impl Apb2rstr {
  pub fn tim1rst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tim8rst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tim8rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn usart1rst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_usart1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn usart6rst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_usart6rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn adcrst(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_adcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn sdiorst(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_sdiorst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
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

  pub fn spi4rst(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_spi4rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn syscfgrst(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_syscfgrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn tim9rst(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_tim9rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tim10rst(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_tim10rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn tim11rst(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_tim11rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn spi5rst(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_spi5rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn spi6rst(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_spi6rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn sai1rst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_sai1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ltdcrst(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_ltdcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
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
      if self.tim1rst() != 0 { try!(write!(f, " tim1rst"))}
      if self.tim8rst() != 0 { try!(write!(f, " tim8rst"))}
      if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
      if self.usart6rst() != 0 { try!(write!(f, " usart6rst"))}
      if self.adcrst() != 0 { try!(write!(f, " adcrst"))}
      if self.sdiorst() != 0 { try!(write!(f, " sdiorst"))}
      if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
      if self.spi4rst() != 0 { try!(write!(f, " spi4rst"))}
      if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
      if self.tim9rst() != 0 { try!(write!(f, " tim9rst"))}
      if self.tim10rst() != 0 { try!(write!(f, " tim10rst"))}
      if self.tim11rst() != 0 { try!(write!(f, " tim11rst"))}
      if self.spi5rst() != 0 { try!(write!(f, " spi5rst"))}
      if self.spi6rst() != 0 { try!(write!(f, " spi6rst"))}
      if self.sai1rst() != 0 { try!(write!(f, " sai1rst"))}
      if self.ltdcrst() != 0 { try!(write!(f, " ltdcrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb1enr(pub u32);

impl Ahb1enr {
  pub fn otghsulpien(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_otghsulpien(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn otghsen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_otghsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn ethmacptpen(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_ethmacptpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn ethmacrxen(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_ethmacrxen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn ethmactxen(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_ethmactxen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn ethmacen(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_ethmacen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn dma2den(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_dma2den(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn dma2en(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_dma2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn dma1en(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_dma1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn ccmdataramen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_ccmdataramen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn bkpsramen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_bkpsramen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
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

  pub fn gpioen(&self, index: usize) -> u32 {
     assert!(index < 11);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_gpioen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 11);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Ahb1enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb1enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otghsulpien() != 0 { try!(write!(f, " otghsulpien"))}
      if self.otghsen() != 0 { try!(write!(f, " otghsen"))}
      if self.ethmacptpen() != 0 { try!(write!(f, " ethmacptpen"))}
      if self.ethmacrxen() != 0 { try!(write!(f, " ethmacrxen"))}
      if self.ethmactxen() != 0 { try!(write!(f, " ethmactxen"))}
      if self.ethmacen() != 0 { try!(write!(f, " ethmacen"))}
      if self.dma2den() != 0 { try!(write!(f, " dma2den"))}
      if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
      if self.dma1en() != 0 { try!(write!(f, " dma1en"))}
      if self.ccmdataramen() != 0 { try!(write!(f, " ccmdataramen"))}
      if self.bkpsramen() != 0 { try!(write!(f, " bkpsramen"))}
      if self.crcen() != 0 { try!(write!(f, " crcen"))}
      if self.gpioen(0) != 0 { try!(write!(f, " gpioen[0]"))}
      if self.gpioen(1) != 0 { try!(write!(f, " gpioen[1]"))}
      if self.gpioen(2) != 0 { try!(write!(f, " gpioen[2]"))}
      if self.gpioen(3) != 0 { try!(write!(f, " gpioen[3]"))}
      if self.gpioen(4) != 0 { try!(write!(f, " gpioen[4]"))}
      if self.gpioen(5) != 0 { try!(write!(f, " gpioen[5]"))}
      if self.gpioen(6) != 0 { try!(write!(f, " gpioen[6]"))}
      if self.gpioen(7) != 0 { try!(write!(f, " gpioen[7]"))}
      if self.gpioen(8) != 0 { try!(write!(f, " gpioen[8]"))}
      if self.gpioen(9) != 0 { try!(write!(f, " gpioen[9]"))}
      if self.gpioen(10) != 0 { try!(write!(f, " gpioen[10]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb2enr(pub u32);

impl Ahb2enr {
  pub fn otgfsen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_otgfsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rngen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_rngen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn dcmien(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_dcmien(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ahb2enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb2enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otgfsen() != 0 { try!(write!(f, " otgfsen"))}
      if self.rngen() != 0 { try!(write!(f, " rngen"))}
      if self.dcmien() != 0 { try!(write!(f, " dcmien"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb3enr(pub u32);

impl Ahb3enr {
  pub fn fmcen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_fmcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ahb3enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb3enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fmcen() != 0 { try!(write!(f, " fmcen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Apb1enr(pub u32);

impl Apb1enr {
  pub fn tim2en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tim3en(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tim3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tim4en(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tim4en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tim5en(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tim5en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
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

  pub fn tim7en(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tim7en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tim12en(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tim12en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tim13en(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_tim13en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn tim14en(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_tim14en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
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

  pub fn spi2en(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_spi2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn spi3en(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_spi3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
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

  pub fn usart3en(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_usart3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn uart4en(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_uart4en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn uart5en(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_uart5en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
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

  pub fn i2c2en(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_i2c2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn i2c3en(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_i2c3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn can1en(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_can1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn can2en(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_can2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
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

  pub fn dacen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_dacen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn uart7enr(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_uart7enr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn uart8enr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_uart8enr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
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
      if self.i2c3en() != 0 { try!(write!(f, " i2c3en"))}
      if self.can1en() != 0 { try!(write!(f, " can1en"))}
      if self.can2en() != 0 { try!(write!(f, " can2en"))}
      if self.pwren() != 0 { try!(write!(f, " pwren"))}
      if self.dacen() != 0 { try!(write!(f, " dacen"))}
      if self.uart7enr() != 0 { try!(write!(f, " uart7enr"))}
      if self.uart8enr() != 0 { try!(write!(f, " uart8enr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Apb2enr(pub u32);

impl Apb2enr {
  pub fn tim1en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tim8en(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tim8en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn usart1en(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_usart1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn usart6en(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_usart6en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn adc1en(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_adc1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn adc2en(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_adc2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn adc3en(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_adc3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn sdioen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_sdioen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
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

  pub fn spi4enr(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_spi4enr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn syscfgen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_syscfgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn tim9en(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_tim9en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tim10en(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_tim10en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn tim11en(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_tim11en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn spi5enr(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_spi5enr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn spi6enr(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_spi6enr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn sai1en(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_sai1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ltdcen(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_ltdcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
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
      if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
      if self.tim8en() != 0 { try!(write!(f, " tim8en"))}
      if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
      if self.usart6en() != 0 { try!(write!(f, " usart6en"))}
      if self.adc1en() != 0 { try!(write!(f, " adc1en"))}
      if self.adc2en() != 0 { try!(write!(f, " adc2en"))}
      if self.adc3en() != 0 { try!(write!(f, " adc3en"))}
      if self.sdioen() != 0 { try!(write!(f, " sdioen"))}
      if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
      if self.spi4enr() != 0 { try!(write!(f, " spi4enr"))}
      if self.syscfgen() != 0 { try!(write!(f, " syscfgen"))}
      if self.tim9en() != 0 { try!(write!(f, " tim9en"))}
      if self.tim10en() != 0 { try!(write!(f, " tim10en"))}
      if self.tim11en() != 0 { try!(write!(f, " tim11en"))}
      if self.spi5enr() != 0 { try!(write!(f, " spi5enr"))}
      if self.spi6enr() != 0 { try!(write!(f, " spi6enr"))}
      if self.sai1en() != 0 { try!(write!(f, " sai1en"))}
      if self.ltdcen() != 0 { try!(write!(f, " ltdcen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb1lpenr(pub u32);

impl Ahb1lpenr {
  pub fn gpioalpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_gpioalpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn gpioblpen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_gpioblpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn gpioclpen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_gpioclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn gpiodlpen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_gpiodlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn gpioelpen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_gpioelpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn gpioflpen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_gpioflpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn gpioglpen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_gpioglpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn gpiohlpen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_gpiohlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn gpioilpen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_gpioilpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn gpiojlpen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_gpiojlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn gpioklpen(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_gpioklpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn crclpen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_crclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn flitflpen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_flitflpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn sram1lpen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_sram1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn sram2lpen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_sram2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn bkpsramlpen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_bkpsramlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn sram3lpen(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_sram3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn dma1lpen(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_dma1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn dma2lpen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_dma2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn dma2dlpen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_dma2dlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn ethmaclpen(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_ethmaclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn ethmactxlpen(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_ethmactxlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn ethmacrxlpen(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_ethmacrxlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn ethmacptplpen(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_ethmacptplpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn otghslpen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_otghslpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn otghsulpilpen(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_otghsulpilpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}

impl ::core::fmt::Display for Ahb1lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb1lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gpioalpen() != 0 { try!(write!(f, " gpioalpen"))}
      if self.gpioblpen() != 0 { try!(write!(f, " gpioblpen"))}
      if self.gpioclpen() != 0 { try!(write!(f, " gpioclpen"))}
      if self.gpiodlpen() != 0 { try!(write!(f, " gpiodlpen"))}
      if self.gpioelpen() != 0 { try!(write!(f, " gpioelpen"))}
      if self.gpioflpen() != 0 { try!(write!(f, " gpioflpen"))}
      if self.gpioglpen() != 0 { try!(write!(f, " gpioglpen"))}
      if self.gpiohlpen() != 0 { try!(write!(f, " gpiohlpen"))}
      if self.gpioilpen() != 0 { try!(write!(f, " gpioilpen"))}
      if self.gpiojlpen() != 0 { try!(write!(f, " gpiojlpen"))}
      if self.gpioklpen() != 0 { try!(write!(f, " gpioklpen"))}
      if self.crclpen() != 0 { try!(write!(f, " crclpen"))}
      if self.flitflpen() != 0 { try!(write!(f, " flitflpen"))}
      if self.sram1lpen() != 0 { try!(write!(f, " sram1lpen"))}
      if self.sram2lpen() != 0 { try!(write!(f, " sram2lpen"))}
      if self.bkpsramlpen() != 0 { try!(write!(f, " bkpsramlpen"))}
      if self.sram3lpen() != 0 { try!(write!(f, " sram3lpen"))}
      if self.dma1lpen() != 0 { try!(write!(f, " dma1lpen"))}
      if self.dma2lpen() != 0 { try!(write!(f, " dma2lpen"))}
      if self.dma2dlpen() != 0 { try!(write!(f, " dma2dlpen"))}
      if self.ethmaclpen() != 0 { try!(write!(f, " ethmaclpen"))}
      if self.ethmactxlpen() != 0 { try!(write!(f, " ethmactxlpen"))}
      if self.ethmacrxlpen() != 0 { try!(write!(f, " ethmacrxlpen"))}
      if self.ethmacptplpen() != 0 { try!(write!(f, " ethmacptplpen"))}
      if self.otghslpen() != 0 { try!(write!(f, " otghslpen"))}
      if self.otghsulpilpen() != 0 { try!(write!(f, " otghsulpilpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb2lpenr(pub u32);

impl Ahb2lpenr {
  pub fn otgfslpen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_otgfslpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rnglpen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_rnglpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn dcmilpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_dcmilpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ahb2lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb2lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otgfslpen() != 0 { try!(write!(f, " otgfslpen"))}
      if self.rnglpen() != 0 { try!(write!(f, " rnglpen"))}
      if self.dcmilpen() != 0 { try!(write!(f, " dcmilpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ahb3lpenr(pub u32);

impl Ahb3lpenr {
  pub fn fmclpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_fmclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ahb3lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ahb3lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fmclpen() != 0 { try!(write!(f, " fmclpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Apb1lpenr(pub u32);

impl Apb1lpenr {
  pub fn tim2lpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tim3lpen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tim3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tim4lpen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tim4lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tim5lpen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tim5lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn tim6lpen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_tim6lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tim7lpen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tim7lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tim12lpen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tim12lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tim13lpen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_tim13lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn tim14lpen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_tim14lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn wwdglpen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_wwdglpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn spi2lpen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_spi2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn spi3lpen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_spi3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn usart2lpen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_usart2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn usart3lpen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_usart3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn uart4lpen(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_uart4lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn uart5lpen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_uart5lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn i2c1lpen(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_i2c1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn i2c2lpen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_i2c2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn i2c3lpen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_i2c3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn can1lpen(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_can1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn can2lpen(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_can2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn pwrlpen(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_pwrlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn daclpen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_daclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn uart7lpen(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_uart7lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn uart8lpen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_uart8lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Apb1lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Apb1lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim2lpen() != 0 { try!(write!(f, " tim2lpen"))}
      if self.tim3lpen() != 0 { try!(write!(f, " tim3lpen"))}
      if self.tim4lpen() != 0 { try!(write!(f, " tim4lpen"))}
      if self.tim5lpen() != 0 { try!(write!(f, " tim5lpen"))}
      if self.tim6lpen() != 0 { try!(write!(f, " tim6lpen"))}
      if self.tim7lpen() != 0 { try!(write!(f, " tim7lpen"))}
      if self.tim12lpen() != 0 { try!(write!(f, " tim12lpen"))}
      if self.tim13lpen() != 0 { try!(write!(f, " tim13lpen"))}
      if self.tim14lpen() != 0 { try!(write!(f, " tim14lpen"))}
      if self.wwdglpen() != 0 { try!(write!(f, " wwdglpen"))}
      if self.spi2lpen() != 0 { try!(write!(f, " spi2lpen"))}
      if self.spi3lpen() != 0 { try!(write!(f, " spi3lpen"))}
      if self.usart2lpen() != 0 { try!(write!(f, " usart2lpen"))}
      if self.usart3lpen() != 0 { try!(write!(f, " usart3lpen"))}
      if self.uart4lpen() != 0 { try!(write!(f, " uart4lpen"))}
      if self.uart5lpen() != 0 { try!(write!(f, " uart5lpen"))}
      if self.i2c1lpen() != 0 { try!(write!(f, " i2c1lpen"))}
      if self.i2c2lpen() != 0 { try!(write!(f, " i2c2lpen"))}
      if self.i2c3lpen() != 0 { try!(write!(f, " i2c3lpen"))}
      if self.can1lpen() != 0 { try!(write!(f, " can1lpen"))}
      if self.can2lpen() != 0 { try!(write!(f, " can2lpen"))}
      if self.pwrlpen() != 0 { try!(write!(f, " pwrlpen"))}
      if self.daclpen() != 0 { try!(write!(f, " daclpen"))}
      if self.uart7lpen() != 0 { try!(write!(f, " uart7lpen"))}
      if self.uart8lpen() != 0 { try!(write!(f, " uart8lpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Apb2lpenr(pub u32);

impl Apb2lpenr {
  pub fn tim1lpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tim1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tim8lpen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tim8lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn usart1lpen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_usart1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn usart6lpen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_usart6lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn adc1lpen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_adc1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn adc2lpen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_adc2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn adc3lpen(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_adc3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn sdiolpen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_sdiolpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn spi1lpen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_spi1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn spi4lpen(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_spi4lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn syscfglpen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_syscfglpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn tim9lpen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_tim9lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tim10lpen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_tim10lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn tim11lpen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_tim11lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn spi5lpen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_spi5lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn spi6lpen(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_spi6lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn sai1lpen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_sai1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ltdclpen(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_ltdclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Apb2lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Apb2lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim1lpen() != 0 { try!(write!(f, " tim1lpen"))}
      if self.tim8lpen() != 0 { try!(write!(f, " tim8lpen"))}
      if self.usart1lpen() != 0 { try!(write!(f, " usart1lpen"))}
      if self.usart6lpen() != 0 { try!(write!(f, " usart6lpen"))}
      if self.adc1lpen() != 0 { try!(write!(f, " adc1lpen"))}
      if self.adc2lpen() != 0 { try!(write!(f, " adc2lpen"))}
      if self.adc3lpen() != 0 { try!(write!(f, " adc3lpen"))}
      if self.sdiolpen() != 0 { try!(write!(f, " sdiolpen"))}
      if self.spi1lpen() != 0 { try!(write!(f, " spi1lpen"))}
      if self.spi4lpen() != 0 { try!(write!(f, " spi4lpen"))}
      if self.syscfglpen() != 0 { try!(write!(f, " syscfglpen"))}
      if self.tim9lpen() != 0 { try!(write!(f, " tim9lpen"))}
      if self.tim10lpen() != 0 { try!(write!(f, " tim10lpen"))}
      if self.tim11lpen() != 0 { try!(write!(f, " tim11lpen"))}
      if self.spi5lpen() != 0 { try!(write!(f, " spi5lpen"))}
      if self.spi6lpen() != 0 { try!(write!(f, " spi6lpen"))}
      if self.sai1lpen() != 0 { try!(write!(f, " sai1lpen"))}
      if self.ltdclpen() != 0 { try!(write!(f, " ltdclpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Bdcr(pub u32);

impl Bdcr {
  pub fn bdrst(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_bdrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rtcen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_rtcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn rtcsel1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_rtcsel1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn rtcsel0(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_rtcsel0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn lsebyp(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_lsebyp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn lserdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_lserdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn lseon(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_lseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.bdrst() != 0 { try!(write!(f, " bdrst"))}
      if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
      if self.rtcsel1() != 0 { try!(write!(f, " rtcsel1"))}
      if self.rtcsel0() != 0 { try!(write!(f, " rtcsel0"))}
      if self.lsebyp() != 0 { try!(write!(f, " lsebyp"))}
      if self.lserdy() != 0 { try!(write!(f, " lserdy"))}
      if self.lseon() != 0 { try!(write!(f, " lseon"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);

impl Csr {
  pub fn lpwrrstf(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_lpwrrstf(mut self, value: u32) -> Self {
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

  pub fn wdgrstf(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_wdgrstf(mut self, value: u32) -> Self {
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

  pub fn padrstf(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_padrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn borrstf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_borrstf(mut self, value: u32) -> Self {
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
      if self.lpwrrstf() != 0 { try!(write!(f, " lpwrrstf"))}
      if self.wwdgrstf() != 0 { try!(write!(f, " wwdgrstf"))}
      if self.wdgrstf() != 0 { try!(write!(f, " wdgrstf"))}
      if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
      if self.porrstf() != 0 { try!(write!(f, " porrstf"))}
      if self.padrstf() != 0 { try!(write!(f, " padrstf"))}
      if self.borrstf() != 0 { try!(write!(f, " borrstf"))}
      if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
      if self.lsirdy() != 0 { try!(write!(f, " lsirdy"))}
      if self.lsion() != 0 { try!(write!(f, " lsion"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sscgr(pub u32);

impl Sscgr {
  pub fn sscgen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_sscgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn spreadsel(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_spreadsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn incstep(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7fff // [27:13]
  }
  pub fn set_incstep(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 13);
     self.0 |= value << 13;
     self
  }

  pub fn modper(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fff // [12:0]
  }
  pub fn set_modper(mut self, value: u32) -> Self {
     assert!((value & !0x1fff) == 0);
     self.0 &= !(0x1fff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sscgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sscgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sscgen() != 0 { try!(write!(f, " sscgen"))}
      if self.spreadsel() != 0 { try!(write!(f, " spreadsel"))}
      if self.incstep() != 0 { try!(write!(f, " incstep=0x{:x}", self.incstep()))}
      if self.modper() != 0 { try!(write!(f, " modper=0x{:x}", self.modper()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Plli2scfgr(pub u32);

impl Plli2scfgr {
  pub fn plli2sr(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x7 // [30:28]
  }
  pub fn set_plli2sr(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn plli2sq(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_plli2sq(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn plli2sn(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1ff // [14:6]
  }
  pub fn set_plli2sn(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 6);
     self.0 |= value << 6;
     self
  }

}

impl ::core::fmt::Display for Plli2scfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Plli2scfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.plli2sr() != 0 { try!(write!(f, " plli2sr=0x{:x}", self.plli2sr()))}
      if self.plli2sq() != 0 { try!(write!(f, " plli2sq=0x{:x}", self.plli2sq()))}
      if self.plli2sn() != 0 { try!(write!(f, " plli2sn=0x{:x}", self.plli2sn()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

