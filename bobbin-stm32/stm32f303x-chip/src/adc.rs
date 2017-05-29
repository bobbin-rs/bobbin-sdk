pub const ADC1: Adc = Adc(0x50000000);
pub const ADC2: Adc = Adc(0x50000100);
pub const ADC3: Adc = Adc(0x50000400);
pub const ADC4: Adc = Adc(0x50000500);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Adc(pub u32);

impl Adc {
  pub unsafe fn isr(&self) -> Isr { 
     Isr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_isr(&mut self, value: Isr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_isr<F: FnOnce(Isr) -> Isr>(&mut self, f: F) {
     let tmp = self.isr();
     self.set_isr(f(tmp))
  }

  pub unsafe fn ier(&self) -> Ier { 
     Ier(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_ier(&mut self, value: Ier) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_ier<F: FnOnce(Ier) -> Ier>(&mut self, f: F) {
     let tmp = self.ier();
     self.set_ier(f(tmp))
  }

  pub unsafe fn cr(&self) -> Cr { 
     Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_cr(&mut self, value: Cr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_cr<F: FnOnce(Cr) -> Cr>(&mut self, f: F) {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  pub unsafe fn cfgr(&self) -> Cfgr { 
     Cfgr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_cfgr(&mut self, value: Cfgr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&mut self, f: F) {
     let tmp = self.cfgr();
     self.set_cfgr(f(tmp))
  }

  pub unsafe fn smpr1(&self) -> Smpr1 { 
     Smpr1(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_smpr1(&mut self, value: Smpr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_smpr1<F: FnOnce(Smpr1) -> Smpr1>(&mut self, f: F) {
     let tmp = self.smpr1();
     self.set_smpr1(f(tmp))
  }

  pub unsafe fn smpr2(&self) -> Smpr2 { 
     Smpr2(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_smpr2(&mut self, value: Smpr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_smpr2<F: FnOnce(Smpr2) -> Smpr2>(&mut self, f: F) {
     let tmp = self.smpr2();
     self.set_smpr2(f(tmp))
  }

  pub unsafe fn tr1(&self) -> Tr1 { 
     Tr1(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_tr1(&mut self, value: Tr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_tr1<F: FnOnce(Tr1) -> Tr1>(&mut self, f: F) {
     let tmp = self.tr1();
     self.set_tr1(f(tmp))
  }

  pub unsafe fn tr2(&self) -> Tr2 { 
     Tr2(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_tr2(&mut self, value: Tr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_tr2<F: FnOnce(Tr2) -> Tr2>(&mut self, f: F) {
     let tmp = self.tr2();
     self.set_tr2(f(tmp))
  }

  pub unsafe fn tr3(&self) -> Tr3 { 
     Tr3(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }
  pub unsafe fn set_tr3(&mut self, value: Tr3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }
  pub unsafe fn with_tr3<F: FnOnce(Tr3) -> Tr3>(&mut self, f: F) {
     let tmp = self.tr3();
     self.set_tr3(f(tmp))
  }

  pub unsafe fn sqr1(&self) -> Sqr1 { 
     Sqr1(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
  }
  pub unsafe fn set_sqr1(&mut self, value: Sqr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
  }
  pub unsafe fn with_sqr1<F: FnOnce(Sqr1) -> Sqr1>(&mut self, f: F) {
     let tmp = self.sqr1();
     self.set_sqr1(f(tmp))
  }

  pub unsafe fn sqr2(&self) -> Sqr2 { 
     Sqr2(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }
  pub unsafe fn set_sqr2(&mut self, value: Sqr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
  }
  pub unsafe fn with_sqr2<F: FnOnce(Sqr2) -> Sqr2>(&mut self, f: F) {
     let tmp = self.sqr2();
     self.set_sqr2(f(tmp))
  }

  pub unsafe fn sqr3(&self) -> Sqr3 { 
     Sqr3(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
  }
  pub unsafe fn set_sqr3(&mut self, value: Sqr3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
  }
  pub unsafe fn with_sqr3<F: FnOnce(Sqr3) -> Sqr3>(&mut self, f: F) {
     let tmp = self.sqr3();
     self.set_sqr3(f(tmp))
  }

  pub unsafe fn sqr4(&self) -> Sqr4 { 
     Sqr4(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
  }
  pub unsafe fn set_sqr4(&mut self, value: Sqr4) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
  }
  pub unsafe fn with_sqr4<F: FnOnce(Sqr4) -> Sqr4>(&mut self, f: F) {
     let tmp = self.sqr4();
     self.set_sqr4(f(tmp))
  }

  pub unsafe fn dr(&self) -> Dr { 
     Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
  }

  pub unsafe fn jsqr(&self) -> Jsqr { 
     Jsqr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
  }
  pub unsafe fn set_jsqr(&mut self, value: Jsqr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
  }
  pub unsafe fn with_jsqr<F: FnOnce(Jsqr) -> Jsqr>(&mut self, f: F) {
     let tmp = self.jsqr();
     self.set_jsqr(f(tmp))
  }

  pub unsafe fn ofr1(&self) -> Ofr1 { 
     Ofr1(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
  }
  pub unsafe fn set_ofr1(&mut self, value: Ofr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
  }
  pub unsafe fn with_ofr1<F: FnOnce(Ofr1) -> Ofr1>(&mut self, f: F) {
     let tmp = self.ofr1();
     self.set_ofr1(f(tmp))
  }

  pub unsafe fn ofr2(&self) -> Ofr2 { 
     Ofr2(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
  }
  pub unsafe fn set_ofr2(&mut self, value: Ofr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
  }
  pub unsafe fn with_ofr2<F: FnOnce(Ofr2) -> Ofr2>(&mut self, f: F) {
     let tmp = self.ofr2();
     self.set_ofr2(f(tmp))
  }

  pub unsafe fn ofr3(&self) -> Ofr3 { 
     Ofr3(::core::ptr::read_volatile(((self.0 as usize) + 0x68) as *const u32))
  }
  pub unsafe fn set_ofr3(&mut self, value: Ofr3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x68) as *mut u32, value.0);
  }
  pub unsafe fn with_ofr3<F: FnOnce(Ofr3) -> Ofr3>(&mut self, f: F) {
     let tmp = self.ofr3();
     self.set_ofr3(f(tmp))
  }

  pub unsafe fn ofr4(&self) -> Ofr4 { 
     Ofr4(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
  }
  pub unsafe fn set_ofr4(&mut self, value: Ofr4) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
  }
  pub unsafe fn with_ofr4<F: FnOnce(Ofr4) -> Ofr4>(&mut self, f: F) {
     let tmp = self.ofr4();
     self.set_ofr4(f(tmp))
  }

  pub unsafe fn jdr1(&self) -> Jdr1 { 
     Jdr1(::core::ptr::read_volatile(((self.0 as usize) + 0x80) as *const u32))
  }

  pub unsafe fn jdr2(&self) -> Jdr2 { 
     Jdr2(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
  }

  pub unsafe fn jdr3(&self) -> Jdr3 { 
     Jdr3(::core::ptr::read_volatile(((self.0 as usize) + 0x88) as *const u32))
  }

  pub unsafe fn jdr4(&self) -> Jdr4 { 
     Jdr4(::core::ptr::read_volatile(((self.0 as usize) + 0x8c) as *const u32))
  }

  pub unsafe fn awd2cr(&self) -> Awd2cr { 
     Awd2cr(::core::ptr::read_volatile(((self.0 as usize) + 0xa0) as *const u32))
  }
  pub unsafe fn set_awd2cr(&mut self, value: Awd2cr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa0) as *mut u32, value.0);
  }
  pub unsafe fn with_awd2cr<F: FnOnce(Awd2cr) -> Awd2cr>(&mut self, f: F) {
     let tmp = self.awd2cr();
     self.set_awd2cr(f(tmp))
  }

  pub unsafe fn awd3cr(&self) -> Awd3cr { 
     Awd3cr(::core::ptr::read_volatile(((self.0 as usize) + 0xa4) as *const u32))
  }
  pub unsafe fn set_awd3cr(&mut self, value: Awd3cr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa4) as *mut u32, value.0);
  }
  pub unsafe fn with_awd3cr<F: FnOnce(Awd3cr) -> Awd3cr>(&mut self, f: F) {
     let tmp = self.awd3cr();
     self.set_awd3cr(f(tmp))
  }

  pub unsafe fn difsel(&self) -> Difsel { 
     Difsel(::core::ptr::read_volatile(((self.0 as usize) + 0xb0) as *const u32))
  }
  pub unsafe fn set_difsel(&mut self, value: Difsel) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
  }
  pub unsafe fn with_difsel<F: FnOnce(Difsel) -> Difsel>(&mut self, f: F) {
     let tmp = self.difsel();
     self.set_difsel(f(tmp))
  }

  pub unsafe fn calfact(&self) -> Calfact { 
     Calfact(::core::ptr::read_volatile(((self.0 as usize) + 0xb4) as *const u32))
  }
  pub unsafe fn set_calfact(&mut self, value: Calfact) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb4) as *mut u32, value.0);
  }
  pub unsafe fn with_calfact<F: FnOnce(Calfact) -> Calfact>(&mut self, f: F) {
     let tmp = self.calfact();
     self.set_calfact(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);

impl Isr {
  pub fn jqovf(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_jqovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn awd3(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_awd3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn awd2(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_awd2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn awd1(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_awd1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn jeos(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_jeos(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn jeoc(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_jeoc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn ovr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_ovr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn eos(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_eos(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn eoc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_eoc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn eosmp(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_eosmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn adrdy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_adrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jqovf() != 0 { try!(write!(f, " jqovf"))}
      if self.awd3() != 0 { try!(write!(f, " awd3"))}
      if self.awd2() != 0 { try!(write!(f, " awd2"))}
      if self.awd1() != 0 { try!(write!(f, " awd1"))}
      if self.jeos() != 0 { try!(write!(f, " jeos"))}
      if self.jeoc() != 0 { try!(write!(f, " jeoc"))}
      if self.ovr() != 0 { try!(write!(f, " ovr"))}
      if self.eos() != 0 { try!(write!(f, " eos"))}
      if self.eoc() != 0 { try!(write!(f, " eoc"))}
      if self.eosmp() != 0 { try!(write!(f, " eosmp"))}
      if self.adrdy() != 0 { try!(write!(f, " adrdy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ier(pub u32);

impl Ier {
  pub fn jqovfie(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_jqovfie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn awd3ie(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_awd3ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn awd2ie(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_awd2ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn awd1ie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_awd1ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn jeosie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_jeosie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn jeocie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_jeocie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn ovrie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_ovrie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn eosie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_eosie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn eocie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_eocie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn eosmpie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_eosmpie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn adrdyie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_adrdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jqovfie() != 0 { try!(write!(f, " jqovfie"))}
      if self.awd3ie() != 0 { try!(write!(f, " awd3ie"))}
      if self.awd2ie() != 0 { try!(write!(f, " awd2ie"))}
      if self.awd1ie() != 0 { try!(write!(f, " awd1ie"))}
      if self.jeosie() != 0 { try!(write!(f, " jeosie"))}
      if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
      if self.ovrie() != 0 { try!(write!(f, " ovrie"))}
      if self.eosie() != 0 { try!(write!(f, " eosie"))}
      if self.eocie() != 0 { try!(write!(f, " eocie"))}
      if self.eosmpie() != 0 { try!(write!(f, " eosmpie"))}
      if self.adrdyie() != 0 { try!(write!(f, " adrdyie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);

impl Cr {
  pub fn adcal(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_adcal(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn adcaldif(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_adcaldif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn advregen(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
  pub fn set_advregen(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn jadstp(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_jadstp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn adstp(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_adstp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn jadstart(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_jadstart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn adstart(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_adstart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn addis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_addis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn aden(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_aden(mut self, value: u32) -> Self {
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
      if self.adcal() != 0 { try!(write!(f, " adcal"))}
      if self.adcaldif() != 0 { try!(write!(f, " adcaldif"))}
      if self.advregen() != 0 { try!(write!(f, " advregen=0x{:x}", self.advregen()))}
      if self.jadstp() != 0 { try!(write!(f, " jadstp"))}
      if self.adstp() != 0 { try!(write!(f, " adstp"))}
      if self.jadstart() != 0 { try!(write!(f, " jadstart"))}
      if self.adstart() != 0 { try!(write!(f, " adstart"))}
      if self.addis() != 0 { try!(write!(f, " addis"))}
      if self.aden() != 0 { try!(write!(f, " aden"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cfgr(pub u32);

impl Cfgr {
  pub fn awdch1ch(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1f // [30:26]
  }
  pub fn set_awdch1ch(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 26);
     self.0 |= value << 26;
     self
  }

  pub fn jauto(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_jauto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn jawd1en(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_jawd1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn awd1en(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_awd1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn awd1sgl(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_awd1sgl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn jqm(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_jqm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn jdiscen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_jdiscen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn discnum(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7 // [19:17]
  }
  pub fn set_discnum(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn discen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_discen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn autoff(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_autoff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn autdly(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_autdly(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn cont(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_cont(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn ovrmod(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_ovrmod(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn exten(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  pub fn set_exten(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn extsel(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0xf // [9:6]
  }
  pub fn set_extsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 6);
     self.0 |= value << 6;
     self
  }

  pub fn align(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_align(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn res(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  pub fn set_res(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dmacfg(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_dmacfg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
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

impl ::core::fmt::Display for Cfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.awdch1ch() != 0 { try!(write!(f, " awdch1ch=0x{:x}", self.awdch1ch()))}
      if self.jauto() != 0 { try!(write!(f, " jauto"))}
      if self.jawd1en() != 0 { try!(write!(f, " jawd1en"))}
      if self.awd1en() != 0 { try!(write!(f, " awd1en"))}
      if self.awd1sgl() != 0 { try!(write!(f, " awd1sgl"))}
      if self.jqm() != 0 { try!(write!(f, " jqm"))}
      if self.jdiscen() != 0 { try!(write!(f, " jdiscen"))}
      if self.discnum() != 0 { try!(write!(f, " discnum=0x{:x}", self.discnum()))}
      if self.discen() != 0 { try!(write!(f, " discen"))}
      if self.autoff() != 0 { try!(write!(f, " autoff"))}
      if self.autdly() != 0 { try!(write!(f, " autdly"))}
      if self.cont() != 0 { try!(write!(f, " cont"))}
      if self.ovrmod() != 0 { try!(write!(f, " ovrmod"))}
      if self.exten() != 0 { try!(write!(f, " exten=0x{:x}", self.exten()))}
      if self.extsel() != 0 { try!(write!(f, " extsel=0x{:x}", self.extsel()))}
      if self.align() != 0 { try!(write!(f, " align"))}
      if self.res() != 0 { try!(write!(f, " res=0x{:x}", self.res()))}
      if self.dmacfg() != 0 { try!(write!(f, " dmacfg"))}
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Smpr1(pub u32);

impl Smpr1 {
  pub fn smp9(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x7 // [29:27]
  }
  pub fn set_smp9(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn smp8(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_smp8(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn smp7(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x7 // [23:21]
  }
  pub fn set_smp7(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn smp6(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x7 // [20:18]
  }
  pub fn set_smp6(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn smp5(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x7 // [17:15]
  }
  pub fn set_smp5(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn smp4(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_smp4(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn smp3(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x7 // [11:9]
  }
  pub fn set_smp3(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn smp2(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x7 // [8:6]
  }
  pub fn set_smp2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn smp1(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x7 // [5:3]
  }
  pub fn set_smp1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Smpr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Smpr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smp9() != 0 { try!(write!(f, " smp9=0x{:x}", self.smp9()))}
      if self.smp8() != 0 { try!(write!(f, " smp8=0x{:x}", self.smp8()))}
      if self.smp7() != 0 { try!(write!(f, " smp7=0x{:x}", self.smp7()))}
      if self.smp6() != 0 { try!(write!(f, " smp6=0x{:x}", self.smp6()))}
      if self.smp5() != 0 { try!(write!(f, " smp5=0x{:x}", self.smp5()))}
      if self.smp4() != 0 { try!(write!(f, " smp4=0x{:x}", self.smp4()))}
      if self.smp3() != 0 { try!(write!(f, " smp3=0x{:x}", self.smp3()))}
      if self.smp2() != 0 { try!(write!(f, " smp2=0x{:x}", self.smp2()))}
      if self.smp1() != 0 { try!(write!(f, " smp1=0x{:x}", self.smp1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Smpr2(pub u32);

impl Smpr2 {
  pub fn smp18(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_smp18(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn smp17(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x7 // [23:21]
  }
  pub fn set_smp17(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn smp16(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x7 // [20:18]
  }
  pub fn set_smp16(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn smp15(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x7 // [17:15]
  }
  pub fn set_smp15(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn smp14(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_smp14(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn smp13(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x7 // [11:9]
  }
  pub fn set_smp13(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn smp12(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x7 // [8:6]
  }
  pub fn set_smp12(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn smp11(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x7 // [5:3]
  }
  pub fn set_smp11(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn smp10(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_smp10(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Smpr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Smpr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smp18() != 0 { try!(write!(f, " smp18=0x{:x}", self.smp18()))}
      if self.smp17() != 0 { try!(write!(f, " smp17=0x{:x}", self.smp17()))}
      if self.smp16() != 0 { try!(write!(f, " smp16=0x{:x}", self.smp16()))}
      if self.smp15() != 0 { try!(write!(f, " smp15=0x{:x}", self.smp15()))}
      if self.smp14() != 0 { try!(write!(f, " smp14=0x{:x}", self.smp14()))}
      if self.smp13() != 0 { try!(write!(f, " smp13=0x{:x}", self.smp13()))}
      if self.smp12() != 0 { try!(write!(f, " smp12=0x{:x}", self.smp12()))}
      if self.smp11() != 0 { try!(write!(f, " smp11=0x{:x}", self.smp11()))}
      if self.smp10() != 0 { try!(write!(f, " smp10=0x{:x}", self.smp10()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tr1(pub u32);

impl Tr1 {
  pub fn ht1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xfff // [27:16]
  }
  pub fn set_ht1(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn lt1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_lt1(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ht1() != 0 { try!(write!(f, " ht1=0x{:x}", self.ht1()))}
      if self.lt1() != 0 { try!(write!(f, " lt1=0x{:x}", self.lt1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tr2(pub u32);

impl Tr2 {
  pub fn ht2(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_ht2(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn lt2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_lt2(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ht2() != 0 { try!(write!(f, " ht2=0x{:x}", self.ht2()))}
      if self.lt2() != 0 { try!(write!(f, " lt2=0x{:x}", self.lt2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tr3(pub u32);

impl Tr3 {
  pub fn ht3(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_ht3(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn lt3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_lt3(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ht3() != 0 { try!(write!(f, " ht3=0x{:x}", self.ht3()))}
      if self.lt3() != 0 { try!(write!(f, " lt3=0x{:x}", self.lt3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sqr1(pub u32);

impl Sqr1 {
  pub fn sq4(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1f // [28:24]
  }
  pub fn set_sq4(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 24);
     self.0 |= value << 24;
     self
  }

  pub fn sq3(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1f // [22:18]
  }
  pub fn set_sq3(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 18);
     self.0 |= value << 18;
     self
  }

  pub fn sq2(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1f // [16:12]
  }
  pub fn set_sq2(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 12);
     self.0 |= value << 12;
     self
  }

  pub fn sq1(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1f // [10:6]
  }
  pub fn set_sq1(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 6);
     self.0 |= value << 6;
     self
  }

  pub fn l(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_l(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sqr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sqr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sq4() != 0 { try!(write!(f, " sq4=0x{:x}", self.sq4()))}
      if self.sq3() != 0 { try!(write!(f, " sq3=0x{:x}", self.sq3()))}
      if self.sq2() != 0 { try!(write!(f, " sq2=0x{:x}", self.sq2()))}
      if self.sq1() != 0 { try!(write!(f, " sq1=0x{:x}", self.sq1()))}
      if self.l() != 0 { try!(write!(f, " l=0x{:x}", self.l()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sqr2(pub u32);

impl Sqr2 {
  pub fn sq9(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1f // [28:24]
  }
  pub fn set_sq9(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 24);
     self.0 |= value << 24;
     self
  }

  pub fn sq8(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1f // [22:18]
  }
  pub fn set_sq8(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 18);
     self.0 |= value << 18;
     self
  }

  pub fn sq7(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1f // [16:12]
  }
  pub fn set_sq7(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 12);
     self.0 |= value << 12;
     self
  }

  pub fn sq6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1f // [10:6]
  }
  pub fn set_sq6(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 6);
     self.0 |= value << 6;
     self
  }

  pub fn sq5(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_sq5(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sqr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sqr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sq9() != 0 { try!(write!(f, " sq9=0x{:x}", self.sq9()))}
      if self.sq8() != 0 { try!(write!(f, " sq8=0x{:x}", self.sq8()))}
      if self.sq7() != 0 { try!(write!(f, " sq7=0x{:x}", self.sq7()))}
      if self.sq6() != 0 { try!(write!(f, " sq6=0x{:x}", self.sq6()))}
      if self.sq5() != 0 { try!(write!(f, " sq5=0x{:x}", self.sq5()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sqr3(pub u32);

impl Sqr3 {
  pub fn sq14(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1f // [28:24]
  }
  pub fn set_sq14(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 24);
     self.0 |= value << 24;
     self
  }

  pub fn sq13(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1f // [22:18]
  }
  pub fn set_sq13(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 18);
     self.0 |= value << 18;
     self
  }

  pub fn sq12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1f // [16:12]
  }
  pub fn set_sq12(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 12);
     self.0 |= value << 12;
     self
  }

  pub fn sq11(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1f // [10:6]
  }
  pub fn set_sq11(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 6);
     self.0 |= value << 6;
     self
  }

  pub fn sq10(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_sq10(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sqr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sqr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sq14() != 0 { try!(write!(f, " sq14=0x{:x}", self.sq14()))}
      if self.sq13() != 0 { try!(write!(f, " sq13=0x{:x}", self.sq13()))}
      if self.sq12() != 0 { try!(write!(f, " sq12=0x{:x}", self.sq12()))}
      if self.sq11() != 0 { try!(write!(f, " sq11=0x{:x}", self.sq11()))}
      if self.sq10() != 0 { try!(write!(f, " sq10=0x{:x}", self.sq10()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sqr4(pub u32);

impl Sqr4 {
  pub fn sq16(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1f // [10:6]
  }
  pub fn set_sq16(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 6);
     self.0 |= value << 6;
     self
  }

  pub fn sq15(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_sq15(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sqr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sqr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sq16() != 0 { try!(write!(f, " sq16=0x{:x}", self.sq16()))}
      if self.sq15() != 0 { try!(write!(f, " sq15=0x{:x}", self.sq15()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);

impl Dr {
  pub fn regulardata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_regulardata(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
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
      if self.regulardata() != 0 { try!(write!(f, " regulardata=0x{:x}", self.regulardata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Jsqr(pub u32);

impl Jsqr {
  pub fn jsq4(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1f // [30:26]
  }
  pub fn set_jsq4(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 26);
     self.0 |= value << 26;
     self
  }

  pub fn jsq3(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1f // [24:20]
  }
  pub fn set_jsq3(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 20);
     self.0 |= value << 20;
     self
  }

  pub fn jsq2(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1f // [18:14]
  }
  pub fn set_jsq2(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 14);
     self.0 |= value << 14;
     self
  }

  pub fn jsq1(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  pub fn set_jsq1(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

  pub fn jexten(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  pub fn set_jexten(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn jextsel(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0xf // [5:2]
  }
  pub fn set_jextsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 2);
     self.0 |= value << 2;
     self
  }

  pub fn jl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_jl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Jsqr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jsqr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jsq4() != 0 { try!(write!(f, " jsq4=0x{:x}", self.jsq4()))}
      if self.jsq3() != 0 { try!(write!(f, " jsq3=0x{:x}", self.jsq3()))}
      if self.jsq2() != 0 { try!(write!(f, " jsq2=0x{:x}", self.jsq2()))}
      if self.jsq1() != 0 { try!(write!(f, " jsq1=0x{:x}", self.jsq1()))}
      if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
      if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
      if self.jl() != 0 { try!(write!(f, " jl=0x{:x}", self.jl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ofr1(pub u32);

impl Ofr1 {
  pub fn offset1_en(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_offset1_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn offset1_ch(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1f // [30:26]
  }
  pub fn set_offset1_ch(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 26);
     self.0 |= value << 26;
     self
  }

  pub fn offset1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_offset1(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ofr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ofr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offset1_en() != 0 { try!(write!(f, " offset1_en"))}
      if self.offset1_ch() != 0 { try!(write!(f, " offset1_ch=0x{:x}", self.offset1_ch()))}
      if self.offset1() != 0 { try!(write!(f, " offset1=0x{:x}", self.offset1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ofr2(pub u32);

impl Ofr2 {
  pub fn offset2_en(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_offset2_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn offset2_ch(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1f // [30:26]
  }
  pub fn set_offset2_ch(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 26);
     self.0 |= value << 26;
     self
  }

  pub fn offset2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_offset2(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ofr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ofr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offset2_en() != 0 { try!(write!(f, " offset2_en"))}
      if self.offset2_ch() != 0 { try!(write!(f, " offset2_ch=0x{:x}", self.offset2_ch()))}
      if self.offset2() != 0 { try!(write!(f, " offset2=0x{:x}", self.offset2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ofr3(pub u32);

impl Ofr3 {
  pub fn offset3_en(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_offset3_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn offset3_ch(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1f // [30:26]
  }
  pub fn set_offset3_ch(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 26);
     self.0 |= value << 26;
     self
  }

  pub fn offset3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_offset3(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ofr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ofr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offset3_en() != 0 { try!(write!(f, " offset3_en"))}
      if self.offset3_ch() != 0 { try!(write!(f, " offset3_ch=0x{:x}", self.offset3_ch()))}
      if self.offset3() != 0 { try!(write!(f, " offset3=0x{:x}", self.offset3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ofr4(pub u32);

impl Ofr4 {
  pub fn offset4_en(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_offset4_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn offset4_ch(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1f // [30:26]
  }
  pub fn set_offset4_ch(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 26);
     self.0 |= value << 26;
     self
  }

  pub fn offset4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  pub fn set_offset4(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ofr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ofr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offset4_en() != 0 { try!(write!(f, " offset4_en"))}
      if self.offset4_ch() != 0 { try!(write!(f, " offset4_ch=0x{:x}", self.offset4_ch()))}
      if self.offset4() != 0 { try!(write!(f, " offset4=0x{:x}", self.offset4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Jdr1(pub u32);

impl Jdr1 {
  pub fn jdata1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_jdata1(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Jdr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jdr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jdata1() != 0 { try!(write!(f, " jdata1=0x{:x}", self.jdata1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Jdr2(pub u32);

impl Jdr2 {
  pub fn jdata2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_jdata2(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Jdr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jdr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jdata2() != 0 { try!(write!(f, " jdata2=0x{:x}", self.jdata2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Jdr3(pub u32);

impl Jdr3 {
  pub fn jdata3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_jdata3(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Jdr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jdr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jdata3() != 0 { try!(write!(f, " jdata3=0x{:x}", self.jdata3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Jdr4(pub u32);

impl Jdr4 {
  pub fn jdata4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_jdata4(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Jdr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jdr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jdata4() != 0 { try!(write!(f, " jdata4=0x{:x}", self.jdata4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Awd2cr(pub u32);

impl Awd2cr {
  pub fn awd2ch(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3ffff // [18:1]
  }
  pub fn set_awd2ch(mut self, value: u32) -> Self {
     assert!((value & !0x3ffff) == 0);
     self.0 &= !(0x3ffff << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Awd2cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Awd2cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.awd2ch() != 0 { try!(write!(f, " awd2ch=0x{:x}", self.awd2ch()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Awd3cr(pub u32);

impl Awd3cr {
  pub fn awd3ch(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3ffff // [18:1]
  }
  pub fn set_awd3ch(mut self, value: u32) -> Self {
     assert!((value & !0x3ffff) == 0);
     self.0 &= !(0x3ffff << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Awd3cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Awd3cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.awd3ch() != 0 { try!(write!(f, " awd3ch=0x{:x}", self.awd3ch()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Difsel(pub u32);

impl Difsel {
  pub fn difsel_1_15(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x7fff // [15:1]
  }
  pub fn set_difsel_1_15(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 1);
     self.0 |= value << 1;
     self
  }

  pub fn difsel_16_18(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7 // [18:16]
  }
  pub fn set_difsel_16_18(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Difsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Difsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.difsel_1_15() != 0 { try!(write!(f, " difsel_1_15=0x{:x}", self.difsel_1_15()))}
      if self.difsel_16_18() != 0 { try!(write!(f, " difsel_16_18=0x{:x}", self.difsel_16_18()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Calfact(pub u32);

impl Calfact {
  pub fn calfact_d(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  pub fn set_calfact_d(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

  pub fn calfact_s(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  pub fn set_calfact_s(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Calfact {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Calfact {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.calfact_d() != 0 { try!(write!(f, " calfact_d=0x{:x}", self.calfact_d()))}
      if self.calfact_s() != 0 { try!(write!(f, " calfact_s=0x{:x}", self.calfact_s()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

