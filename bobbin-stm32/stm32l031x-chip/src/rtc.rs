pub const RTC: Rtc = Rtc(0x40002800);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtc(pub u32);

impl Rtc {
  pub unsafe fn tr(&self) -> Tr { 
     Tr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_tr(&self, value: Tr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_tr<F: FnOnce(Tr) -> Tr>(&self, f: F) {
     let tmp = self.tr();
     self.set_tr(f(tmp))
  }

  pub unsafe fn dr(&self) -> Dr { 
     Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_dr(&self, value: Dr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

  pub unsafe fn cr(&self) -> Cr { 
     Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_cr(&self, value: Cr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  pub unsafe fn isr(&self) -> Isr { 
     Isr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_isr(&self, value: Isr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) {
     let tmp = self.isr();
     self.set_isr(f(tmp))
  }

  pub unsafe fn prer(&self) -> Prer { 
     Prer(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_prer(&self, value: Prer) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_prer<F: FnOnce(Prer) -> Prer>(&self, f: F) {
     let tmp = self.prer();
     self.set_prer(f(tmp))
  }

  pub unsafe fn wutr(&self) -> Wutr { 
     Wutr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_wutr(&self, value: Wutr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_wutr<F: FnOnce(Wutr) -> Wutr>(&self, f: F) {
     let tmp = self.wutr();
     self.set_wutr(f(tmp))
  }

  pub unsafe fn alrmar(&self) -> Alrmar { 
     Alrmar(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_alrmar(&self, value: Alrmar) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_alrmar<F: FnOnce(Alrmar) -> Alrmar>(&self, f: F) {
     let tmp = self.alrmar();
     self.set_alrmar(f(tmp))
  }

  pub unsafe fn alrmbr(&self) -> Alrmbr { 
     Alrmbr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_alrmbr(&self, value: Alrmbr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_alrmbr<F: FnOnce(Alrmbr) -> Alrmbr>(&self, f: F) {
     let tmp = self.alrmbr();
     self.set_alrmbr(f(tmp))
  }

  pub unsafe fn set_wpr(&self, value: Wpr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }

  pub unsafe fn ssr(&self) -> Ssr { 
     Ssr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }

  pub unsafe fn set_shiftr(&self, value: Shiftr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
  }

  pub unsafe fn tstr(&self) -> Tstr { 
     Tstr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
  }

  pub unsafe fn tsdr(&self) -> Tsdr { 
     Tsdr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }

  pub unsafe fn tsssr(&self) -> Tsssr { 
     Tsssr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
  }

  pub unsafe fn calr(&self) -> Calr { 
     Calr(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
  }
  pub unsafe fn set_calr(&self, value: Calr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
  }
  pub unsafe fn with_calr<F: FnOnce(Calr) -> Calr>(&self, f: F) {
     let tmp = self.calr();
     self.set_calr(f(tmp))
  }

  pub unsafe fn tampcr(&self) -> Tampcr { 
     Tampcr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
  }
  pub unsafe fn set_tampcr(&self, value: Tampcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
  }
  pub unsafe fn with_tampcr<F: FnOnce(Tampcr) -> Tampcr>(&self, f: F) {
     let tmp = self.tampcr();
     self.set_tampcr(f(tmp))
  }

  pub unsafe fn alrmassr(&self) -> Alrmassr { 
     Alrmassr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
  }
  pub unsafe fn set_alrmassr(&self, value: Alrmassr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
  }
  pub unsafe fn with_alrmassr<F: FnOnce(Alrmassr) -> Alrmassr>(&self, f: F) {
     let tmp = self.alrmassr();
     self.set_alrmassr(f(tmp))
  }

  pub unsafe fn alrmbssr(&self) -> Alrmbssr { 
     Alrmbssr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
  }
  pub unsafe fn set_alrmbssr(&self, value: Alrmbssr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
  }
  pub unsafe fn with_alrmbssr<F: FnOnce(Alrmbssr) -> Alrmbssr>(&self, f: F) {
     let tmp = self.alrmbssr();
     self.set_alrmbssr(f(tmp))
  }

  pub unsafe fn or(&self) -> Or { 
     Or(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
  }
  pub unsafe fn set_or(&self, value: Or) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
  }
  pub unsafe fn with_or<F: FnOnce(Or) -> Or>(&self, f: F) {
     let tmp = self.or();
     self.set_or(f(tmp))
  }

  pub unsafe fn bkpr(&self, index: usize) -> Bkpr { 
     assert!(index < 5);
     Bkpr(::core::ptr::read_volatile(((self.0 as usize) + 0x50 + (index << 2)) as *const u32))
  }
  pub unsafe fn set_bkpr(&self, index: usize, value: Bkpr) {
     assert!(index < 5);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x50 + (index << 2)) as *mut u32, value.0);
  }
  pub unsafe fn with_bkpr<F: FnOnce(Bkpr) -> Bkpr>(&self, index: usize, f: F) {
     let tmp = self.bkpr(index);
     self.set_bkpr(index, f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Tr(pub u32);

impl Tr {
  pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn mnt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_mnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mnu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_mnu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn su(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_su(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);

impl Dr {
  pub fn yt(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  pub fn set_yt(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  pub fn yu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_yu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn wdu(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  pub fn set_wdu(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn mt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_mt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_mu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn du(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_du(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
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
      if self.yt() != 0 { try!(write!(f, " yt=0x{:x}", self.yt()))}
      if self.yu() != 0 { try!(write!(f, " yu=0x{:x}", self.yu()))}
      if self.wdu() != 0 { try!(write!(f, " wdu=0x{:x}", self.wdu()))}
      if self.mt() != 0 { try!(write!(f, " mt"))}
      if self.mu() != 0 { try!(write!(f, " mu=0x{:x}", self.mu()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);

impl Cr {
  pub fn coe(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_coe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn osel(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x3 // [22:21]
  }
  pub fn set_osel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn pol(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_pol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn cosel(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_cosel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn bkp(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_bkp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn sub1h(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_sub1h(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn add1h(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_add1h(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tsie(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_tsie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn wutie(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_wutie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn alrbie(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_alrbie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn alraie(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_alraie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn tse(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_tse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn wute(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_wute(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn alrbe(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_alrbe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn alrae(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_alrae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn fmt(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_fmt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn bypshad(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_bypshad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn refckon(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_refckon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tsedge(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tsedge(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn wucksel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_wucksel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
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
      if self.coe() != 0 { try!(write!(f, " coe"))}
      if self.osel() != 0 { try!(write!(f, " osel=0x{:x}", self.osel()))}
      if self.pol() != 0 { try!(write!(f, " pol"))}
      if self.cosel() != 0 { try!(write!(f, " cosel"))}
      if self.bkp() != 0 { try!(write!(f, " bkp"))}
      if self.sub1h() != 0 { try!(write!(f, " sub1h"))}
      if self.add1h() != 0 { try!(write!(f, " add1h"))}
      if self.tsie() != 0 { try!(write!(f, " tsie"))}
      if self.wutie() != 0 { try!(write!(f, " wutie"))}
      if self.alrbie() != 0 { try!(write!(f, " alrbie"))}
      if self.alraie() != 0 { try!(write!(f, " alraie"))}
      if self.tse() != 0 { try!(write!(f, " tse"))}
      if self.wute() != 0 { try!(write!(f, " wute"))}
      if self.alrbe() != 0 { try!(write!(f, " alrbe"))}
      if self.alrae() != 0 { try!(write!(f, " alrae"))}
      if self.fmt() != 0 { try!(write!(f, " fmt"))}
      if self.bypshad() != 0 { try!(write!(f, " bypshad"))}
      if self.refckon() != 0 { try!(write!(f, " refckon"))}
      if self.tsedge() != 0 { try!(write!(f, " tsedge"))}
      if self.wucksel() != 0 { try!(write!(f, " wucksel=0x{:x}", self.wucksel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);

impl Isr {
  pub fn tamp2f(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_tamp2f(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn tamp1f(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_tamp1f(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn tsovf(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_tsovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn tsf(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_tsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn wutf(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_wutf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn alrbf(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_alrbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn alraf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_alraf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn init(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_init(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn initf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_initf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn rsf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_rsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn inits(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_inits(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn shpf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_shpf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn wutwf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_wutwf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn alrbwf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_alrbwf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn alrawf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_alrawf(mut self, value: u32) -> Self {
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
      if self.tamp2f() != 0 { try!(write!(f, " tamp2f"))}
      if self.tamp1f() != 0 { try!(write!(f, " tamp1f"))}
      if self.tsovf() != 0 { try!(write!(f, " tsovf"))}
      if self.tsf() != 0 { try!(write!(f, " tsf"))}
      if self.wutf() != 0 { try!(write!(f, " wutf"))}
      if self.alrbf() != 0 { try!(write!(f, " alrbf"))}
      if self.alraf() != 0 { try!(write!(f, " alraf"))}
      if self.init() != 0 { try!(write!(f, " init"))}
      if self.initf() != 0 { try!(write!(f, " initf"))}
      if self.rsf() != 0 { try!(write!(f, " rsf"))}
      if self.inits() != 0 { try!(write!(f, " inits"))}
      if self.shpf() != 0 { try!(write!(f, " shpf"))}
      if self.wutwf() != 0 { try!(write!(f, " wutwf"))}
      if self.alrbwf() != 0 { try!(write!(f, " alrbwf"))}
      if self.alrawf() != 0 { try!(write!(f, " alrawf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Prer(pub u32);

impl Prer {
  pub fn prediv_a(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  pub fn set_prediv_a(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

  pub fn prediv_s(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_prediv_s(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Prer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Prer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prediv_a() != 0 { try!(write!(f, " prediv_a=0x{:x}", self.prediv_a()))}
      if self.prediv_s() != 0 { try!(write!(f, " prediv_s=0x{:x}", self.prediv_s()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wutr(pub u32);

impl Wutr {
  pub fn wut(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_wut(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wutr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wutr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wut() != 0 { try!(write!(f, " wut=0x{:x}", self.wut()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Alrmar(pub u32);

impl Alrmar {
  pub fn msk4(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_msk4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn wdsel(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_wdsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
  pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn du(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_du(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn msk3(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_msk3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn msk2(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_msk2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn mnt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_mnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mnu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_mnu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn msk1(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_msk1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn su(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_su(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Alrmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Alrmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.msk4() != 0 { try!(write!(f, " msk4"))}
      if self.wdsel() != 0 { try!(write!(f, " wdsel"))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      if self.msk3() != 0 { try!(write!(f, " msk3"))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.msk2() != 0 { try!(write!(f, " msk2"))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.msk1() != 0 { try!(write!(f, " msk1"))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Alrmbr(pub u32);

impl Alrmbr {
  pub fn msk4(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_msk4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn wdsel(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_wdsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
  pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn du(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_du(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn msk3(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_msk3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn msk2(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_msk2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn mnt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_mnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mnu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_mnu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn msk1(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_msk1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn su(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_su(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Alrmbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Alrmbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.msk4() != 0 { try!(write!(f, " msk4"))}
      if self.wdsel() != 0 { try!(write!(f, " wdsel"))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      if self.msk3() != 0 { try!(write!(f, " msk3"))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.msk2() != 0 { try!(write!(f, " msk2"))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.msk1() != 0 { try!(write!(f, " msk1"))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wpr(pub u32);

impl Wpr {
  pub fn key(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_key(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ssr(pub u32);

impl Ssr {
  pub fn ss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ss(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Shiftr(pub u32);

impl Shiftr {
  pub fn add1s(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_add1s(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn subfs(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fff // [14:0]
  }
  pub fn set_subfs(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Shiftr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Shiftr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.add1s() != 0 { try!(write!(f, " add1s"))}
      if self.subfs() != 0 { try!(write!(f, " subfs=0x{:x}", self.subfs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tstr(pub u32);

impl Tstr {
  pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn mnt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
  pub fn set_mnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mnu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_mnu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn su(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_su(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tsdr(pub u32);

impl Tsdr {
  pub fn wdu(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
  pub fn set_wdu(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn mt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_mt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_mu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn du(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_du(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tsdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tsdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdu() != 0 { try!(write!(f, " wdu=0x{:x}", self.wdu()))}
      if self.mt() != 0 { try!(write!(f, " mt"))}
      if self.mu() != 0 { try!(write!(f, " mu=0x{:x}", self.mu()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tsssr(pub u32);

impl Tsssr {
  pub fn ss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ss(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tsssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tsssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Calr(pub u32);

impl Calr {
  pub fn calp(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_calp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn calw8(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_calw8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn calw16(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_calw16(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn calm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1ff // [8:0]
  }
  pub fn set_calm(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Calr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Calr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.calp() != 0 { try!(write!(f, " calp"))}
      if self.calw8() != 0 { try!(write!(f, " calw8"))}
      if self.calw16() != 0 { try!(write!(f, " calw16"))}
      if self.calm() != 0 { try!(write!(f, " calm=0x{:x}", self.calm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tampcr(pub u32);

impl Tampcr {
  pub fn tamp2mf(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_tamp2mf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn tamp2noerase(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_tamp2noerase(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn tamp2ie(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_tamp2ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn tamp1mf(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_tamp1mf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn tamp1noerase(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_tamp1noerase(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn tamp1ie(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_tamp1ie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tamppudis(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_tamppudis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn tampprch(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x3 // [14:13]
  }
  pub fn set_tampprch(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn tampflt(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x3 // [12:11]
  }
  pub fn set_tampflt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn tampfreq(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_tampfreq(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn tampts(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_tampts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn tamp2_trg(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_tamp2_trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn tamp2e(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tamp2e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn tampie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tampie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tamp1trg(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tamp1trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tamp1e(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tamp1e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tampcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tampcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tamp2mf() != 0 { try!(write!(f, " tamp2mf"))}
      if self.tamp2noerase() != 0 { try!(write!(f, " tamp2noerase"))}
      if self.tamp2ie() != 0 { try!(write!(f, " tamp2ie"))}
      if self.tamp1mf() != 0 { try!(write!(f, " tamp1mf"))}
      if self.tamp1noerase() != 0 { try!(write!(f, " tamp1noerase"))}
      if self.tamp1ie() != 0 { try!(write!(f, " tamp1ie"))}
      if self.tamppudis() != 0 { try!(write!(f, " tamppudis"))}
      if self.tampprch() != 0 { try!(write!(f, " tampprch=0x{:x}", self.tampprch()))}
      if self.tampflt() != 0 { try!(write!(f, " tampflt=0x{:x}", self.tampflt()))}
      if self.tampfreq() != 0 { try!(write!(f, " tampfreq=0x{:x}", self.tampfreq()))}
      if self.tampts() != 0 { try!(write!(f, " tampts"))}
      if self.tamp2_trg() != 0 { try!(write!(f, " tamp2_trg"))}
      if self.tamp2e() != 0 { try!(write!(f, " tamp2e"))}
      if self.tampie() != 0 { try!(write!(f, " tampie"))}
      if self.tamp1trg() != 0 { try!(write!(f, " tamp1trg"))}
      if self.tamp1e() != 0 { try!(write!(f, " tamp1e"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Alrmassr(pub u32);

impl Alrmassr {
  pub fn maskss(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_maskss(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn ss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fff // [14:0]
  }
  pub fn set_ss(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Alrmassr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Alrmassr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maskss() != 0 { try!(write!(f, " maskss=0x{:x}", self.maskss()))}
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Alrmbssr(pub u32);

impl Alrmbssr {
  pub fn maskss(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_maskss(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  pub fn ss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fff // [14:0]
  }
  pub fn set_ss(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Alrmbssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Alrmbssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maskss() != 0 { try!(write!(f, " maskss=0x{:x}", self.maskss()))}
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Or(pub u32);

impl Or {
  pub fn rtc_out_rmp(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_rtc_out_rmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rtc_alarm_type(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_rtc_alarm_type(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.rtc_out_rmp() != 0 { try!(write!(f, " rtc_out_rmp"))}
      if self.rtc_alarm_type() != 0 { try!(write!(f, " rtc_alarm_type"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Bkpr(pub u32);

impl Bkpr {
  pub fn bkp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_bkp(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Bkpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Bkpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

