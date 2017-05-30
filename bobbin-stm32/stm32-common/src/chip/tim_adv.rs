
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TimAdvImpl(pub u32);
impl TimAdvImpl {
  pub fn cr1(&self) -> Cr1 { 
     unsafe {
       Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  pub fn set_cr1(&self, value: Cr1) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

  pub fn cr2(&self) -> Cr2 { 
     unsafe {
       Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  pub fn set_cr2(&self, value: Cr2) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.cr2();
     self.set_cr2(f(tmp))
  }

  pub fn smcr(&self) -> Smcr { 
     unsafe {
       Smcr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  pub fn set_smcr(&self, value: Smcr) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  pub fn with_smcr<F: FnOnce(Smcr) -> Smcr>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.smcr();
     self.set_smcr(f(tmp))
  }

  pub fn dier(&self) -> Dier { 
     unsafe {
       Dier(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  pub fn set_dier(&self, value: Dier) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  pub fn with_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.dier();
     self.set_dier(f(tmp))
  }

  pub fn sr(&self) -> Sr { 
     unsafe {
       Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  pub fn set_sr(&self, value: Sr) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

  pub fn set_egr(&self, value: Egr) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

  pub fn ccmr1_output(&self) -> Ccmr1Output { 
     unsafe {
       Ccmr1Output(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  pub fn set_ccmr1_output(&self, value: Ccmr1Output) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccmr1_output<F: FnOnce(Ccmr1Output) -> Ccmr1Output>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccmr1_output();
     self.set_ccmr1_output(f(tmp))
  }

  pub fn ccmr1_input(&self) -> Ccmr1Input { 
     unsafe {
       Ccmr1Input(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  pub fn set_ccmr1_input(&self, value: Ccmr1Input) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccmr1_input<F: FnOnce(Ccmr1Input) -> Ccmr1Input>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccmr1_input();
     self.set_ccmr1_input(f(tmp))
  }

  pub fn ccmr2_output(&self) -> Ccmr2Output { 
     unsafe {
       Ccmr2Output(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  pub fn set_ccmr2_output(&self, value: Ccmr2Output) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccmr2_output<F: FnOnce(Ccmr2Output) -> Ccmr2Output>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccmr2_output();
     self.set_ccmr2_output(f(tmp))
  }

  pub fn ccmr2_input(&self) -> Ccmr2Input { 
     unsafe {
       Ccmr2Input(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  pub fn set_ccmr2_input(&self, value: Ccmr2Input) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccmr2_input<F: FnOnce(Ccmr2Input) -> Ccmr2Input>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccmr2_input();
     self.set_ccmr2_input(f(tmp))
  }

  pub fn ccer(&self) -> Ccer { 
     unsafe {
       Ccer(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  pub fn set_ccer(&self, value: Ccer) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccer<F: FnOnce(Ccer) -> Ccer>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccer();
     self.set_ccer(f(tmp))
  }

  pub fn cnt(&self) -> Cnt { 
     unsafe {
       Cnt(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  pub fn set_cnt(&self, value: Cnt) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.cnt();
     self.set_cnt(f(tmp))
  }

  pub fn psc(&self) -> Psc { 
     unsafe {
       Psc(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  pub fn set_psc(&self, value: Psc) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  pub fn with_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.psc();
     self.set_psc(f(tmp))
  }

  pub fn arr(&self) -> Arr { 
     unsafe {
       Arr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
  pub fn set_arr(&self, value: Arr) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.arr();
     self.set_arr(f(tmp))
  }

  pub fn rcr(&self) -> Rcr { 
     unsafe {
       Rcr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
  pub fn set_rcr(&self, value: Rcr) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
  pub fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.rcr();
     self.set_rcr(f(tmp))
  }

  pub fn ccr1(&self) -> Ccr1 { 
     unsafe {
       Ccr1(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
  pub fn set_ccr1(&self, value: Ccr1) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccr1<F: FnOnce(Ccr1) -> Ccr1>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccr1();
     self.set_ccr1(f(tmp))
  }

  pub fn ccr2(&self) -> Ccr2 { 
     unsafe {
       Ccr2(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
  pub fn set_ccr2(&self, value: Ccr2) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccr2<F: FnOnce(Ccr2) -> Ccr2>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccr2();
     self.set_ccr2(f(tmp))
  }

  pub fn ccr3(&self) -> Ccr3 { 
     unsafe {
       Ccr3(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
  pub fn set_ccr3(&self, value: Ccr3) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccr3<F: FnOnce(Ccr3) -> Ccr3>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccr3();
     self.set_ccr3(f(tmp))
  }

  pub fn ccr4(&self) -> Ccr4 { 
     unsafe {
       Ccr4(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
  pub fn set_ccr4(&self, value: Ccr4) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccr4<F: FnOnce(Ccr4) -> Ccr4>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccr4();
     self.set_ccr4(f(tmp))
  }

  pub fn bdtr(&self) -> Bdtr { 
     unsafe {
       Bdtr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
  pub fn set_bdtr(&self, value: Bdtr) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
  pub fn with_bdtr<F: FnOnce(Bdtr) -> Bdtr>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.bdtr();
     self.set_bdtr(f(tmp))
  }

  pub fn dcr(&self) -> Dcr { 
     unsafe {
       Dcr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
  pub fn set_dcr(&self, value: Dcr) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
  pub fn with_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.dcr();
     self.set_dcr(f(tmp))
  }

  pub fn dmar(&self) -> Dmar { 
     unsafe {
       Dmar(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
  pub fn set_dmar(&self, value: Dmar) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_dmar<F: FnOnce(Dmar) -> Dmar>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.dmar();
     self.set_dmar(f(tmp))
  }

  pub fn ccmr3_output(&self) -> Ccmr3Output { 
     unsafe {
       Ccmr3Output(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
  pub fn set_ccmr3_output(&self, value: Ccmr3Output) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccmr3_output<F: FnOnce(Ccmr3Output) -> Ccmr3Output>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccmr3_output();
     self.set_ccmr3_output(f(tmp))
  }

  pub fn ccr5(&self) -> Ccr5 { 
     unsafe {
       Ccr5(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
  pub fn set_ccr5(&self, value: Ccr5) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccr5<F: FnOnce(Ccr5) -> Ccr5>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccr5();
     self.set_ccr5(f(tmp))
  }

  pub fn ccr6(&self) -> Ccr6 { 
     unsafe {
       Ccr6(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }
  pub fn set_ccr6(&self, value: Ccr6) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccr6<F: FnOnce(Ccr6) -> Ccr6>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.ccr6();
     self.set_ccr6(f(tmp))
  }

  pub fn or(&self) -> Or { 
     unsafe {
       Or(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
  pub fn set_or(&self, value: Or) -> &TimAdvImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
  pub fn with_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &TimAdvImpl {
     let tmp = self.or();
     self.set_or(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
  pub fn cen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_cen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn udis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_udis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn urs(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_urs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn opm(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_opm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dir(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_dir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cms(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  pub fn set_cms(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn arpe(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_arpe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn ckd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_ckd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn uifremap(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_uifremap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
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
      if self.cen() != 0 { try!(write!(f, " cen"))}
      if self.udis() != 0 { try!(write!(f, " udis"))}
      if self.urs() != 0 { try!(write!(f, " urs"))}
      if self.opm() != 0 { try!(write!(f, " opm"))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.cms() != 0 { try!(write!(f, " cms=0x{:x}", self.cms()))}
      if self.arpe() != 0 { try!(write!(f, " arpe"))}
      if self.ckd() != 0 { try!(write!(f, " ckd=0x{:x}", self.ckd()))}
      if self.uifremap() != 0 { try!(write!(f, " uifremap"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
  pub fn ccpc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ccpc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ccus(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_ccus(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn ccds(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_ccds(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn mms(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_mms(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ti1s(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_ti1s(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn ois1(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_ois1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn ois1n(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_ois1n(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn ois2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_ois2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn ois2n(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_ois2n(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn ois3(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_ois3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn ois3n(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_ois3n(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn ois4(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_ois4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn ois5(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_ois5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ois6(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_ois6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn mms2(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  pub fn set_mms2(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
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
      if self.ccpc() != 0 { try!(write!(f, " ccpc"))}
      if self.ccus() != 0 { try!(write!(f, " ccus"))}
      if self.ccds() != 0 { try!(write!(f, " ccds"))}
      if self.mms() != 0 { try!(write!(f, " mms=0x{:x}", self.mms()))}
      if self.ti1s() != 0 { try!(write!(f, " ti1s"))}
      if self.ois1() != 0 { try!(write!(f, " ois1"))}
      if self.ois1n() != 0 { try!(write!(f, " ois1n"))}
      if self.ois2() != 0 { try!(write!(f, " ois2"))}
      if self.ois2n() != 0 { try!(write!(f, " ois2n"))}
      if self.ois3() != 0 { try!(write!(f, " ois3"))}
      if self.ois3n() != 0 { try!(write!(f, " ois3n"))}
      if self.ois4() != 0 { try!(write!(f, " ois4"))}
      if self.ois5() != 0 { try!(write!(f, " ois5"))}
      if self.ois6() != 0 { try!(write!(f, " ois6"))}
      if self.mms2() != 0 { try!(write!(f, " mms2=0x{:x}", self.mms2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Smcr(pub u32);
impl Smcr {
  pub fn sms(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_sms(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn occs(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_occs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ts(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_ts(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn msm(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_msm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn etf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_etf(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn etps(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  pub fn set_etps(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn ece(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_ece(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn etp(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_etp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn sms3(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_sms3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Smcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Smcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sms() != 0 { try!(write!(f, " sms=0x{:x}", self.sms()))}
      if self.occs() != 0 { try!(write!(f, " occs"))}
      if self.ts() != 0 { try!(write!(f, " ts=0x{:x}", self.ts()))}
      if self.msm() != 0 { try!(write!(f, " msm"))}
      if self.etf() != 0 { try!(write!(f, " etf=0x{:x}", self.etf()))}
      if self.etps() != 0 { try!(write!(f, " etps=0x{:x}", self.etps()))}
      if self.ece() != 0 { try!(write!(f, " ece"))}
      if self.etp() != 0 { try!(write!(f, " etp"))}
      if self.sms3() != 0 { try!(write!(f, " sms3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dier(pub u32);
impl Dier {
  pub fn tde(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_tde(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn comde(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_comde(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn cc4de(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_cc4de(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn cc3de(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_cc3de(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn cc2de(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_cc2de(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cc1de(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_cc1de(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn ude(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_ude(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn bie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_bie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn comie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_comie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn cc4ie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_cc4ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cc3ie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_cc3ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn cc2ie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_cc2ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cc1ie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_cc1ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn uie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_uie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tde() != 0 { try!(write!(f, " tde"))}
      if self.comde() != 0 { try!(write!(f, " comde"))}
      if self.cc4de() != 0 { try!(write!(f, " cc4de"))}
      if self.cc3de() != 0 { try!(write!(f, " cc3de"))}
      if self.cc2de() != 0 { try!(write!(f, " cc2de"))}
      if self.cc1de() != 0 { try!(write!(f, " cc1de"))}
      if self.ude() != 0 { try!(write!(f, " ude"))}
      if self.bie() != 0 { try!(write!(f, " bie"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.comie() != 0 { try!(write!(f, " comie"))}
      if self.cc4ie() != 0 { try!(write!(f, " cc4ie"))}
      if self.cc3ie() != 0 { try!(write!(f, " cc3ie"))}
      if self.cc2ie() != 0 { try!(write!(f, " cc2ie"))}
      if self.cc1ie() != 0 { try!(write!(f, " cc1ie"))}
      if self.uie() != 0 { try!(write!(f, " uie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
  pub fn uif(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_uif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cc1if(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_cc1if(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cc2if(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_cc2if(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cc3if(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_cc3if(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn cc4if(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_cc4if(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn comif(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_comif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tif(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn bif(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_bif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn b2if(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_b2if(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cc1of(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_cc1of(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn cc2of(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_cc2of(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cc3of(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_cc3of(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn cc4of(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_cc4of(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn cc5if(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_cc5if(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn cc6if(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_cc6if(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.uif() != 0 { try!(write!(f, " uif"))}
      if self.cc1if() != 0 { try!(write!(f, " cc1if"))}
      if self.cc2if() != 0 { try!(write!(f, " cc2if"))}
      if self.cc3if() != 0 { try!(write!(f, " cc3if"))}
      if self.cc4if() != 0 { try!(write!(f, " cc4if"))}
      if self.comif() != 0 { try!(write!(f, " comif"))}
      if self.tif() != 0 { try!(write!(f, " tif"))}
      if self.bif() != 0 { try!(write!(f, " bif"))}
      if self.b2if() != 0 { try!(write!(f, " b2if"))}
      if self.cc1of() != 0 { try!(write!(f, " cc1of"))}
      if self.cc2of() != 0 { try!(write!(f, " cc2of"))}
      if self.cc3of() != 0 { try!(write!(f, " cc3of"))}
      if self.cc4of() != 0 { try!(write!(f, " cc4of"))}
      if self.cc5if() != 0 { try!(write!(f, " cc5if"))}
      if self.cc6if() != 0 { try!(write!(f, " cc6if"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Egr(pub u32);
impl Egr {
  pub fn ug(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ug(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cc1g(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_cc1g(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cc2g(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_cc2g(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cc3g(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_cc3g(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn cc4g(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_cc4g(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn comg(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_comg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tg(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn bg(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_bg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn b2g(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_b2g(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Egr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Egr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ug() != 0 { try!(write!(f, " ug"))}
      if self.cc1g() != 0 { try!(write!(f, " cc1g"))}
      if self.cc2g() != 0 { try!(write!(f, " cc2g"))}
      if self.cc3g() != 0 { try!(write!(f, " cc3g"))}
      if self.cc4g() != 0 { try!(write!(f, " cc4g"))}
      if self.comg() != 0 { try!(write!(f, " comg"))}
      if self.tg() != 0 { try!(write!(f, " tg"))}
      if self.bg() != 0 { try!(write!(f, " bg"))}
      if self.b2g() != 0 { try!(write!(f, " b2g"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccmr1Output(pub u32);
impl Ccmr1Output {
  pub fn oc2ce(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_oc2ce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn oc2m(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_oc2m(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn oc2pe(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_oc2pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn oc2fe(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_oc2fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cc2s(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_cc2s(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn oc1ce(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_oc1ce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn oc1m(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_oc1m(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn oc1pe(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_oc1pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn oc1fe(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_oc1fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cc1s(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_cc1s(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn oc1m_3(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_oc1m_3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn oc2m_3(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_oc2m_3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Ccmr1Output {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccmr1Output {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.oc2ce() != 0 { try!(write!(f, " oc2ce"))}
      if self.oc2m() != 0 { try!(write!(f, " oc2m=0x{:x}", self.oc2m()))}
      if self.oc2pe() != 0 { try!(write!(f, " oc2pe"))}
      if self.oc2fe() != 0 { try!(write!(f, " oc2fe"))}
      if self.cc2s() != 0 { try!(write!(f, " cc2s=0x{:x}", self.cc2s()))}
      if self.oc1ce() != 0 { try!(write!(f, " oc1ce"))}
      if self.oc1m() != 0 { try!(write!(f, " oc1m=0x{:x}", self.oc1m()))}
      if self.oc1pe() != 0 { try!(write!(f, " oc1pe"))}
      if self.oc1fe() != 0 { try!(write!(f, " oc1fe"))}
      if self.cc1s() != 0 { try!(write!(f, " cc1s=0x{:x}", self.cc1s()))}
      if self.oc1m_3() != 0 { try!(write!(f, " oc1m_3"))}
      if self.oc2m_3() != 0 { try!(write!(f, " oc2m_3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccmr1Input(pub u32);
impl Ccmr1Input {
  pub fn ic2f(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  pub fn set_ic2f(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  pub fn ic2pcs(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  pub fn set_ic2pcs(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cc2s(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_cc2s(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn ic1f(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_ic1f(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ic1pcs(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_ic1pcs(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cc1s(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_cc1s(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccmr1Input {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccmr1Input {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ic2f() != 0 { try!(write!(f, " ic2f=0x{:x}", self.ic2f()))}
      if self.ic2pcs() != 0 { try!(write!(f, " ic2pcs=0x{:x}", self.ic2pcs()))}
      if self.cc2s() != 0 { try!(write!(f, " cc2s=0x{:x}", self.cc2s()))}
      if self.ic1f() != 0 { try!(write!(f, " ic1f=0x{:x}", self.ic1f()))}
      if self.ic1pcs() != 0 { try!(write!(f, " ic1pcs=0x{:x}", self.ic1pcs()))}
      if self.cc1s() != 0 { try!(write!(f, " cc1s=0x{:x}", self.cc1s()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccmr2Output(pub u32);
impl Ccmr2Output {
  pub fn oc4ce(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_oc4ce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn oc4m(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_oc4m(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn oc4pe(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_oc4pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn oc4fe(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_oc4fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cc4s(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_cc4s(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn oc3ce(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_oc3ce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn oc3m(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_oc3m(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn oc3pe(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_oc3pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn oc3fe(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_oc3fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cc3s(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_cc3s(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn oc3m_3(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_oc3m_3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn oc4m_3(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_oc4m_3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Ccmr2Output {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccmr2Output {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.oc4ce() != 0 { try!(write!(f, " oc4ce"))}
      if self.oc4m() != 0 { try!(write!(f, " oc4m=0x{:x}", self.oc4m()))}
      if self.oc4pe() != 0 { try!(write!(f, " oc4pe"))}
      if self.oc4fe() != 0 { try!(write!(f, " oc4fe"))}
      if self.cc4s() != 0 { try!(write!(f, " cc4s=0x{:x}", self.cc4s()))}
      if self.oc3ce() != 0 { try!(write!(f, " oc3ce"))}
      if self.oc3m() != 0 { try!(write!(f, " oc3m=0x{:x}", self.oc3m()))}
      if self.oc3pe() != 0 { try!(write!(f, " oc3pe"))}
      if self.oc3fe() != 0 { try!(write!(f, " oc3fe"))}
      if self.cc3s() != 0 { try!(write!(f, " cc3s=0x{:x}", self.cc3s()))}
      if self.oc3m_3() != 0 { try!(write!(f, " oc3m_3"))}
      if self.oc4m_3() != 0 { try!(write!(f, " oc4m_3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccmr2Input(pub u32);
impl Ccmr2Input {
  pub fn ic4f(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  pub fn set_ic4f(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  pub fn ic4psc(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  pub fn set_ic4psc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cc4s(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_cc4s(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn ic3f(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_ic3f(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ic3psc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_ic3psc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cc3s(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_cc3s(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccmr2Input {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccmr2Input {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ic4f() != 0 { try!(write!(f, " ic4f=0x{:x}", self.ic4f()))}
      if self.ic4psc() != 0 { try!(write!(f, " ic4psc=0x{:x}", self.ic4psc()))}
      if self.cc4s() != 0 { try!(write!(f, " cc4s=0x{:x}", self.cc4s()))}
      if self.ic3f() != 0 { try!(write!(f, " ic3f=0x{:x}", self.ic3f()))}
      if self.ic3psc() != 0 { try!(write!(f, " ic3psc=0x{:x}", self.ic3psc()))}
      if self.cc3s() != 0 { try!(write!(f, " cc3s=0x{:x}", self.cc3s()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccer(pub u32);
impl Ccer {
  pub fn cc1e(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_cc1e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cc1p(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_cc1p(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cc1ne(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_cc1ne(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cc1np(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_cc1np(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn cc2e(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_cc2e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cc2p(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_cc2p(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn cc2ne(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_cc2ne(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn cc2np(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_cc2np(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn cc3e(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_cc3e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cc3p(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_cc3p(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn cc3ne(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_cc3ne(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cc3np(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_cc3np(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn cc4e(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_cc4e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn cc4p(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_cc4p(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn cc4np(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_cc4np(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn cc5e(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_cc5e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn cc5p(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_cc5p(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn cc6e(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_cc6e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn cc6p(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_cc6p(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

}
impl ::core::fmt::Display for Ccer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cc1e() != 0 { try!(write!(f, " cc1e"))}
      if self.cc1p() != 0 { try!(write!(f, " cc1p"))}
      if self.cc1ne() != 0 { try!(write!(f, " cc1ne"))}
      if self.cc1np() != 0 { try!(write!(f, " cc1np"))}
      if self.cc2e() != 0 { try!(write!(f, " cc2e"))}
      if self.cc2p() != 0 { try!(write!(f, " cc2p"))}
      if self.cc2ne() != 0 { try!(write!(f, " cc2ne"))}
      if self.cc2np() != 0 { try!(write!(f, " cc2np"))}
      if self.cc3e() != 0 { try!(write!(f, " cc3e"))}
      if self.cc3p() != 0 { try!(write!(f, " cc3p"))}
      if self.cc3ne() != 0 { try!(write!(f, " cc3ne"))}
      if self.cc3np() != 0 { try!(write!(f, " cc3np"))}
      if self.cc4e() != 0 { try!(write!(f, " cc4e"))}
      if self.cc4p() != 0 { try!(write!(f, " cc4p"))}
      if self.cc4np() != 0 { try!(write!(f, " cc4np"))}
      if self.cc5e() != 0 { try!(write!(f, " cc5e"))}
      if self.cc5p() != 0 { try!(write!(f, " cc5p"))}
      if self.cc6e() != 0 { try!(write!(f, " cc6e"))}
      if self.cc6p() != 0 { try!(write!(f, " cc6p"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
  pub fn cnt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_cnt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn uifcpy(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_uifcpy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
      if self.uifcpy() != 0 { try!(write!(f, " uifcpy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Psc(pub u32);
impl Psc {
  pub fn psc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_psc(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Psc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Psc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
  pub fn arr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_arr(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Arr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Arr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.arr() != 0 { try!(write!(f, " arr=0x{:x}", self.arr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
  pub fn rep(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_rep(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rep() != 0 { try!(write!(f, " rep=0x{:x}", self.rep()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
  pub fn ccr1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ccr1(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccr1() != 0 { try!(write!(f, " ccr1=0x{:x}", self.ccr1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
  pub fn ccr2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ccr2(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccr2() != 0 { try!(write!(f, " ccr2=0x{:x}", self.ccr2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccr3(pub u32);
impl Ccr3 {
  pub fn ccr3(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ccr3(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccr3() != 0 { try!(write!(f, " ccr3=0x{:x}", self.ccr3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccr4(pub u32);
impl Ccr4 {
  pub fn ccr4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ccr4(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccr4() != 0 { try!(write!(f, " ccr4=0x{:x}", self.ccr4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Bdtr(pub u32);
impl Bdtr {
  pub fn dtg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_dtg(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn lock(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_lock(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn ossi(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_ossi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn ossr(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_ossr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn bke(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_bke(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn bkp(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_bkp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn aoe(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_aoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn moe(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_moe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn bkf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_bkf(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn bk2f(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  pub fn set_bk2f(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  pub fn bk2e(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_bk2e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn bk2p(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_bk2p(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

}
impl ::core::fmt::Display for Bdtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bdtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dtg() != 0 { try!(write!(f, " dtg=0x{:x}", self.dtg()))}
      if self.lock() != 0 { try!(write!(f, " lock=0x{:x}", self.lock()))}
      if self.ossi() != 0 { try!(write!(f, " ossi"))}
      if self.ossr() != 0 { try!(write!(f, " ossr"))}
      if self.bke() != 0 { try!(write!(f, " bke"))}
      if self.bkp() != 0 { try!(write!(f, " bkp"))}
      if self.aoe() != 0 { try!(write!(f, " aoe"))}
      if self.moe() != 0 { try!(write!(f, " moe"))}
      if self.bkf() != 0 { try!(write!(f, " bkf=0x{:x}", self.bkf()))}
      if self.bk2f() != 0 { try!(write!(f, " bk2f=0x{:x}", self.bk2f()))}
      if self.bk2e() != 0 { try!(write!(f, " bk2e"))}
      if self.bk2p() != 0 { try!(write!(f, " bk2p"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dcr(pub u32);
impl Dcr {
  pub fn dbl(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  pub fn set_dbl(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dba(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_dba(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbl() != 0 { try!(write!(f, " dbl=0x{:x}", self.dbl()))}
      if self.dba() != 0 { try!(write!(f, " dba=0x{:x}", self.dba()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmar(pub u32);
impl Dmar {
  pub fn dmab(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_dmab(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dmab() != 0 { try!(write!(f, " dmab=0x{:x}", self.dmab()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccmr3Output(pub u32);
impl Ccmr3Output {
  pub fn oc5fe(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_oc5fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn oc5pe(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_oc5pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn oc5m(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_oc5m(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn oc5ce(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_oc5ce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn oc6fe(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_oc6fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn oc6pe(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_oc6pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn oc6m(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_oc6m(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn oc6ce(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_oc6ce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn oc5m_3(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_oc5m_3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn oc6m_3(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_oc6m_3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Ccmr3Output {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccmr3Output {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.oc5fe() != 0 { try!(write!(f, " oc5fe"))}
      if self.oc5pe() != 0 { try!(write!(f, " oc5pe"))}
      if self.oc5m() != 0 { try!(write!(f, " oc5m=0x{:x}", self.oc5m()))}
      if self.oc5ce() != 0 { try!(write!(f, " oc5ce"))}
      if self.oc6fe() != 0 { try!(write!(f, " oc6fe"))}
      if self.oc6pe() != 0 { try!(write!(f, " oc6pe"))}
      if self.oc6m() != 0 { try!(write!(f, " oc6m=0x{:x}", self.oc6m()))}
      if self.oc6ce() != 0 { try!(write!(f, " oc6ce"))}
      if self.oc5m_3() != 0 { try!(write!(f, " oc5m_3"))}
      if self.oc6m_3() != 0 { try!(write!(f, " oc6m_3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccr5(pub u32);
impl Ccr5 {
  pub fn ccr5(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ccr5(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn gc5c1(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_gc5c1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  pub fn gc5c2(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_gc5c2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn gc5c3(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_gc5c3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Ccr5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccr5() != 0 { try!(write!(f, " ccr5=0x{:x}", self.ccr5()))}
      if self.gc5c1() != 0 { try!(write!(f, " gc5c1"))}
      if self.gc5c2() != 0 { try!(write!(f, " gc5c2"))}
      if self.gc5c3() != 0 { try!(write!(f, " gc5c3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccr6(pub u32);
impl Ccr6 {
  pub fn ccr6(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ccr6(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ccr6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccr6() != 0 { try!(write!(f, " ccr6=0x{:x}", self.ccr6()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Or(pub u32);
impl Or {
  pub fn tim1_etr_adc1_rmp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_tim1_etr_adc1_rmp(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tim1_etr_adc4_rmp(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  pub fn set_tim1_etr_adc4_rmp(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Or {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Or {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim1_etr_adc1_rmp() != 0 { try!(write!(f, " tim1_etr_adc1_rmp=0x{:x}", self.tim1_etr_adc1_rmp()))}
      if self.tim1_etr_adc4_rmp() != 0 { try!(write!(f, " tim1_etr_adc4_rmp=0x{:x}", self.tim1_etr_adc4_rmp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
