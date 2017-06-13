pub const ADC1: Adc1 = Periph(0x40012000, Adc1Id {});
pub const ADC2: Adc2 = Periph(0x40012100, Adc2Id {});
pub const ADC3: Adc3 = Periph(0x40012200, Adc3Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Adc1Id {}
pub type Adc1 = Periph<Adc1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Adc2Id {}
pub type Adc2 = Periph<Adc2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Adc3Id {}
pub type Adc3 = Periph<Adc3Id>;





impl<T> Periph<T> {
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

  #[inline] pub fn cr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn cr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn cr1(&self) -> Cr1 { 
     unsafe {
        Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_cr1(&self, value: Cr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

  #[inline] pub fn cr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn cr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn cr2(&self) -> Cr2 { 
     unsafe {
        Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_cr2(&self, value: Cr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
     let tmp = self.cr2();
     self.set_cr2(f(tmp))
  }

  #[inline] pub fn smpr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn smpr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn smpr1(&self) -> Smpr1 { 
     unsafe {
        Smpr1(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_smpr1(&self, value: Smpr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_smpr1<F: FnOnce(Smpr1) -> Smpr1>(&self, f: F) -> &Self {
     let tmp = self.smpr1();
     self.set_smpr1(f(tmp))
  }

  #[inline] pub fn smpr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn smpr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn smpr2(&self) -> Smpr2 { 
     unsafe {
        Smpr2(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_smpr2(&self, value: Smpr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_smpr2<F: FnOnce(Smpr2) -> Smpr2>(&self, f: F) -> &Self {
     let tmp = self.smpr2();
     self.set_smpr2(f(tmp))
  }

  #[inline] pub fn jofr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline] pub fn jofr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline] pub fn jofr1(&self) -> Jofr1 { 
     unsafe {
        Jofr1(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline] pub fn set_jofr1(&self, value: Jofr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_jofr1<F: FnOnce(Jofr1) -> Jofr1>(&self, f: F) -> &Self {
     let tmp = self.jofr1();
     self.set_jofr1(f(tmp))
  }

  #[inline] pub fn jofr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline] pub fn jofr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline] pub fn jofr2(&self) -> Jofr2 { 
     unsafe {
        Jofr2(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline] pub fn set_jofr2(&self, value: Jofr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_jofr2<F: FnOnce(Jofr2) -> Jofr2>(&self, f: F) -> &Self {
     let tmp = self.jofr2();
     self.set_jofr2(f(tmp))
  }

  #[inline] pub fn jofr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline] pub fn jofr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline] pub fn jofr3(&self) -> Jofr3 { 
     unsafe {
        Jofr3(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline] pub fn set_jofr3(&self, value: Jofr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_jofr3<F: FnOnce(Jofr3) -> Jofr3>(&self, f: F) -> &Self {
     let tmp = self.jofr3();
     self.set_jofr3(f(tmp))
  }

  #[inline] pub fn jofr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
  #[inline] pub fn jofr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
  #[inline] pub fn jofr4(&self) -> Jofr4 { 
     unsafe {
        Jofr4(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  #[inline] pub fn set_jofr4(&self, value: Jofr4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_jofr4<F: FnOnce(Jofr4) -> Jofr4>(&self, f: F) -> &Self {
     let tmp = self.jofr4();
     self.set_jofr4(f(tmp))
  }

  #[inline] pub fn htr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline] pub fn htr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline] pub fn htr(&self) -> Htr { 
     unsafe {
        Htr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  #[inline] pub fn set_htr(&self, value: Htr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_htr<F: FnOnce(Htr) -> Htr>(&self, f: F) -> &Self {
     let tmp = self.htr();
     self.set_htr(f(tmp))
  }

  #[inline] pub fn ltr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
  #[inline] pub fn ltr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
  #[inline] pub fn ltr(&self) -> Ltr { 
     unsafe {
        Ltr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  #[inline] pub fn set_ltr(&self, value: Ltr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ltr<F: FnOnce(Ltr) -> Ltr>(&self, f: F) -> &Self {
     let tmp = self.ltr();
     self.set_ltr(f(tmp))
  }

  #[inline] pub fn sqr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
  #[inline] pub fn sqr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
  #[inline] pub fn sqr1(&self) -> Sqr1 { 
     unsafe {
        Sqr1(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
  #[inline] pub fn set_sqr1(&self, value: Sqr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sqr1<F: FnOnce(Sqr1) -> Sqr1>(&self, f: F) -> &Self {
     let tmp = self.sqr1();
     self.set_sqr1(f(tmp))
  }

  #[inline] pub fn sqr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
  #[inline] pub fn sqr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
  #[inline] pub fn sqr2(&self) -> Sqr2 { 
     unsafe {
        Sqr2(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
  #[inline] pub fn set_sqr2(&self, value: Sqr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sqr2<F: FnOnce(Sqr2) -> Sqr2>(&self, f: F) -> &Self {
     let tmp = self.sqr2();
     self.set_sqr2(f(tmp))
  }

  #[inline] pub fn sqr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
  #[inline] pub fn sqr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
  #[inline] pub fn sqr3(&self) -> Sqr3 { 
     unsafe {
        Sqr3(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
  #[inline] pub fn set_sqr3(&self, value: Sqr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_sqr3<F: FnOnce(Sqr3) -> Sqr3>(&self, f: F) -> &Self {
     let tmp = self.sqr3();
     self.set_sqr3(f(tmp))
  }

  #[inline] pub fn jsqr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
  #[inline] pub fn jsqr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
  #[inline] pub fn jsqr(&self) -> Jsqr { 
     unsafe {
        Jsqr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
  #[inline] pub fn set_jsqr(&self, value: Jsqr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_jsqr<F: FnOnce(Jsqr) -> Jsqr>(&self, f: F) -> &Self {
     let tmp = self.jsqr();
     self.set_jsqr(f(tmp))
  }

  #[inline] pub fn jdr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
  #[inline] pub fn jdr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
  #[inline] pub fn jdr1(&self) -> Jdr1 { 
     unsafe {
        Jdr1(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }

  #[inline] pub fn jdr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
  #[inline] pub fn jdr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
  #[inline] pub fn jdr2(&self) -> Jdr2 { 
     unsafe {
        Jdr2(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }

  #[inline] pub fn jdr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
  #[inline] pub fn jdr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
  #[inline] pub fn jdr3(&self) -> Jdr3 { 
     unsafe {
        Jdr3(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }

  #[inline] pub fn jdr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
  #[inline] pub fn jdr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
  #[inline] pub fn jdr4(&self) -> Jdr4 { 
     unsafe {
        Jdr4(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }

  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
  #[inline] pub fn ovr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_ovr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn strt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_strt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn jstrt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_jstrt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn jeoc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_jeoc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn eoc(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_eoc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn awd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_awd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.ovr() != 0 { try!(write!(f, " ovr"))}
      if self.strt() != 0 { try!(write!(f, " strt"))}
      if self.jstrt() != 0 { try!(write!(f, " jstrt"))}
      if self.jeoc() != 0 { try!(write!(f, " jeoc"))}
      if self.eoc() != 0 { try!(write!(f, " eoc"))}
      if self.awd() != 0 { try!(write!(f, " awd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
  #[inline] pub fn ovrie(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline] pub fn set_ovrie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline] pub fn res(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
  #[inline] pub fn set_res(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn awden(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline] pub fn set_awden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline] pub fn jawden(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline] pub fn set_jawden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline] pub fn discnum(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  #[inline] pub fn set_discnum(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn jdiscen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_jdiscen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn discen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_discen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn jauto(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_jauto(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn awdsgl(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_awdsgl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn scan(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_scan(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn jeocie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_jeocie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn awdie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_awdie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn eocie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_eocie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn awdch(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_awdch(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovrie() != 0 { try!(write!(f, " ovrie"))}
      if self.res() != 0 { try!(write!(f, " res=0x{:x}", self.res()))}
      if self.awden() != 0 { try!(write!(f, " awden"))}
      if self.jawden() != 0 { try!(write!(f, " jawden"))}
      if self.discnum() != 0 { try!(write!(f, " discnum=0x{:x}", self.discnum()))}
      if self.jdiscen() != 0 { try!(write!(f, " jdiscen"))}
      if self.discen() != 0 { try!(write!(f, " discen"))}
      if self.jauto() != 0 { try!(write!(f, " jauto"))}
      if self.awdsgl() != 0 { try!(write!(f, " awdsgl"))}
      if self.scan() != 0 { try!(write!(f, " scan"))}
      if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
      if self.awdie() != 0 { try!(write!(f, " awdie"))}
      if self.eocie() != 0 { try!(write!(f, " eocie"))}
      if self.awdch() != 0 { try!(write!(f, " awdch=0x{:x}", self.awdch()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
  #[inline] pub fn swstart(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline] pub fn set_swstart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline] pub fn exten(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
  #[inline] pub fn set_exten(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

  #[inline] pub fn extsel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline] pub fn set_extsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn jswstart(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline] pub fn set_jswstart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline] pub fn jexten(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  #[inline] pub fn set_jexten(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn jextsel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline] pub fn set_jextsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn align(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_align(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn eocs(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_eocs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn dds(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_dds(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn dma(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_dma(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn cont(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_cont(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn adon(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_adon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swstart() != 0 { try!(write!(f, " swstart"))}
      if self.exten() != 0 { try!(write!(f, " exten=0x{:x}", self.exten()))}
      if self.extsel() != 0 { try!(write!(f, " extsel=0x{:x}", self.extsel()))}
      if self.jswstart() != 0 { try!(write!(f, " jswstart"))}
      if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
      if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
      if self.align() != 0 { try!(write!(f, " align"))}
      if self.eocs() != 0 { try!(write!(f, " eocs"))}
      if self.dds() != 0 { try!(write!(f, " dds"))}
      if self.dma() != 0 { try!(write!(f, " dma"))}
      if self.cont() != 0 { try!(write!(f, " cont"))}
      if self.adon() != 0 { try!(write!(f, " adon"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Smpr1(pub u32);
impl Smpr1 {
  #[inline] pub fn smpx_x(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_smpx_x(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
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
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Smpr2(pub u32);
impl Smpr2 {
  #[inline] pub fn smpx_x(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_smpx_x(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
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
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jofr1(pub u32);
impl Jofr1 {
  #[inline] pub fn joffset1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_joffset1(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Jofr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Jofr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.joffset1() != 0 { try!(write!(f, " joffset1=0x{:x}", self.joffset1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jofr2(pub u32);
impl Jofr2 {
  #[inline] pub fn joffset2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_joffset2(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Jofr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Jofr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.joffset2() != 0 { try!(write!(f, " joffset2=0x{:x}", self.joffset2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jofr3(pub u32);
impl Jofr3 {
  #[inline] pub fn joffset3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_joffset3(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Jofr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Jofr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.joffset3() != 0 { try!(write!(f, " joffset3=0x{:x}", self.joffset3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jofr4(pub u32);
impl Jofr4 {
  #[inline] pub fn joffset4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_joffset4(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Jofr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Jofr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.joffset4() != 0 { try!(write!(f, " joffset4=0x{:x}", self.joffset4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Htr(pub u32);
impl Htr {
  #[inline] pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Htr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Htr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ltr(pub u32);
impl Ltr {
  #[inline] pub fn lt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
  #[inline] pub fn set_lt(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ltr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ltr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lt() != 0 { try!(write!(f, " lt=0x{:x}", self.lt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sqr1(pub u32);
impl Sqr1 {
  #[inline] pub fn l(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  #[inline] pub fn set_l(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn sq16(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1f // [19:15]
  }
  #[inline] pub fn set_sq16(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 15);
     self.0 |= value << 15;
     self
  }

  #[inline] pub fn sq15(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1f // [14:10]
  }
  #[inline] pub fn set_sq15(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn sq14(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1f // [9:5]
  }
  #[inline] pub fn set_sq14(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn sq13(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_sq13(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
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
      if self.l() != 0 { try!(write!(f, " l=0x{:x}", self.l()))}
      if self.sq16() != 0 { try!(write!(f, " sq16=0x{:x}", self.sq16()))}
      if self.sq15() != 0 { try!(write!(f, " sq15=0x{:x}", self.sq15()))}
      if self.sq14() != 0 { try!(write!(f, " sq14=0x{:x}", self.sq14()))}
      if self.sq13() != 0 { try!(write!(f, " sq13=0x{:x}", self.sq13()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sqr2(pub u32);
impl Sqr2 {
  #[inline] pub fn sq12(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1f // [29:25]
  }
  #[inline] pub fn set_sq12(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 25);
     self.0 |= value << 25;
     self
  }

  #[inline] pub fn sq11(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1f // [24:20]
  }
  #[inline] pub fn set_sq11(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn sq10(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1f // [19:15]
  }
  #[inline] pub fn set_sq10(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 15);
     self.0 |= value << 15;
     self
  }

  #[inline] pub fn sq9(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1f // [14:10]
  }
  #[inline] pub fn set_sq9(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn sq8(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1f // [9:5]
  }
  #[inline] pub fn set_sq8(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn sq7(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_sq7(mut self, value: u32) -> Self {
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
      if self.sq12() != 0 { try!(write!(f, " sq12=0x{:x}", self.sq12()))}
      if self.sq11() != 0 { try!(write!(f, " sq11=0x{:x}", self.sq11()))}
      if self.sq10() != 0 { try!(write!(f, " sq10=0x{:x}", self.sq10()))}
      if self.sq9() != 0 { try!(write!(f, " sq9=0x{:x}", self.sq9()))}
      if self.sq8() != 0 { try!(write!(f, " sq8=0x{:x}", self.sq8()))}
      if self.sq7() != 0 { try!(write!(f, " sq7=0x{:x}", self.sq7()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sqr3(pub u32);
impl Sqr3 {
  #[inline] pub fn sq6(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1f // [29:25]
  }
  #[inline] pub fn set_sq6(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 25);
     self.0 |= value << 25;
     self
  }

  #[inline] pub fn sq5(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1f // [24:20]
  }
  #[inline] pub fn set_sq5(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn sq4(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1f // [19:15]
  }
  #[inline] pub fn set_sq4(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 15);
     self.0 |= value << 15;
     self
  }

  #[inline] pub fn sq3(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1f // [14:10]
  }
  #[inline] pub fn set_sq3(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn sq2(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1f // [9:5]
  }
  #[inline] pub fn set_sq2(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn sq1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_sq1(mut self, value: u32) -> Self {
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
      if self.sq6() != 0 { try!(write!(f, " sq6=0x{:x}", self.sq6()))}
      if self.sq5() != 0 { try!(write!(f, " sq5=0x{:x}", self.sq5()))}
      if self.sq4() != 0 { try!(write!(f, " sq4=0x{:x}", self.sq4()))}
      if self.sq3() != 0 { try!(write!(f, " sq3=0x{:x}", self.sq3()))}
      if self.sq2() != 0 { try!(write!(f, " sq2=0x{:x}", self.sq2()))}
      if self.sq1() != 0 { try!(write!(f, " sq1=0x{:x}", self.sq1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jsqr(pub u32);
impl Jsqr {
  #[inline] pub fn jl(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  #[inline] pub fn set_jl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline] pub fn jsq4(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1f // [19:15]
  }
  #[inline] pub fn set_jsq4(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 15);
     self.0 |= value << 15;
     self
  }

  #[inline] pub fn jsq3(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1f // [14:10]
  }
  #[inline] pub fn set_jsq3(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn jsq2(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1f // [9:5]
  }
  #[inline] pub fn set_jsq2(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn jsq1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  #[inline] pub fn set_jsq1(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
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
      if self.jl() != 0 { try!(write!(f, " jl=0x{:x}", self.jl()))}
      if self.jsq4() != 0 { try!(write!(f, " jsq4=0x{:x}", self.jsq4()))}
      if self.jsq3() != 0 { try!(write!(f, " jsq3=0x{:x}", self.jsq3()))}
      if self.jsq2() != 0 { try!(write!(f, " jsq2=0x{:x}", self.jsq2()))}
      if self.jsq1() != 0 { try!(write!(f, " jsq1=0x{:x}", self.jsq1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jdr1(pub u32);
impl Jdr1 {
  #[inline] pub fn jdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_jdata(mut self, value: u32) -> Self {
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
      if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jdr2(pub u32);
impl Jdr2 {
  #[inline] pub fn jdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_jdata(mut self, value: u32) -> Self {
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
      if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jdr3(pub u32);
impl Jdr3 {
  #[inline] pub fn jdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_jdata(mut self, value: u32) -> Self {
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
      if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Jdr4(pub u32);
impl Jdr4 {
  #[inline] pub fn jdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_jdata(mut self, value: u32) -> Self {
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
      if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
  #[inline] pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_data(mut self, value: u32) -> Self {
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
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait IrqAdc<T> {
   fn irq_adc(&self) -> super::irq::Irq<T>;
}

pub trait RegisterAdcHandler {
   fn register_adc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleAdc>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleAdc {
   fn handle_adc(&self);
}

impl IrqAdc<super::irq::Adc1Id> for Adc1 {
   fn irq_adc(&self) -> super::irq::IrqAdc1 { super::irq::IRQ_ADC1 }
}

impl RegisterAdcHandler for Adc1 {
   fn register_adc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleAdc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleAdc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_adc() }
       }
       super::irq::set_handler(18, Some(wrapper::<F>));
       super::irq::IrqGuard::new(18)
   }
}

impl IrqAdc<super::irq::Adc2Id> for Adc2 {
   fn irq_adc(&self) -> super::irq::IrqAdc2 { super::irq::IRQ_ADC2 }
}

impl RegisterAdcHandler for Adc2 {
   fn register_adc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleAdc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleAdc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_adc() }
       }
       super::irq::set_handler(18, Some(wrapper::<F>));
       super::irq::IrqGuard::new(18)
   }
}

impl IrqAdc<super::irq::Adc3Id> for Adc3 {
   fn irq_adc(&self) -> super::irq::IrqAdc3 { super::irq::IRQ_ADC3 }
}

impl RegisterAdcHandler for Adc3 {
   fn register_adc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleAdc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleAdc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_adc() }
       }
       super::irq::set_handler(18, Some(wrapper::<F>));
       super::irq::IrqGuard::new(18)
   }
}

