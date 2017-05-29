pub const ADC: Adc = Adc(0x42004000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Adc(pub u32);

impl Adc {
  pub unsafe fn avgctrl(&self) -> Avgctrl { 
     Avgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
  }
  pub unsafe fn set_avgctrl(&mut self, value: Avgctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
  }
  pub unsafe fn with_avgctrl<F: FnOnce(Avgctrl) -> Avgctrl>(&mut self, f: F) {
     let tmp = self.avgctrl();
     self.set_avgctrl(f(tmp))
  }

  pub unsafe fn calib(&self) -> Calib { 
     Calib(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u16))
  }
  pub unsafe fn set_calib(&mut self, value: Calib) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u16, value.0);
  }
  pub unsafe fn with_calib<F: FnOnce(Calib) -> Calib>(&mut self, f: F) {
     let tmp = self.calib();
     self.set_calib(f(tmp))
  }

  pub unsafe fn ctrla(&self) -> Ctrla { 
     Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
  }
  pub unsafe fn set_ctrla(&mut self, value: Ctrla) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
  }
  pub unsafe fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&mut self, f: F) {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

  pub unsafe fn ctrlb(&self) -> Ctrlb { 
     Ctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u16))
  }
  pub unsafe fn set_ctrlb(&mut self, value: Ctrlb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u16, value.0);
  }
  pub unsafe fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&mut self, f: F) {
     let tmp = self.ctrlb();
     self.set_ctrlb(f(tmp))
  }

  pub unsafe fn dbgctrl(&self) -> Dbgctrl { 
     Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x2a) as *const u8))
  }
  pub unsafe fn set_dbgctrl(&mut self, value: Dbgctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2a) as *mut u8, value.0);
  }
  pub unsafe fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&mut self, f: F) {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  pub unsafe fn evctrl(&self) -> Evctrl { 
     Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
  }
  pub unsafe fn set_evctrl(&mut self, value: Evctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
  }
  pub unsafe fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&mut self, f: F) {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
  }

  pub unsafe fn gaincorr(&self) -> Gaincorr { 
     Gaincorr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u16))
  }
  pub unsafe fn set_gaincorr(&mut self, value: Gaincorr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u16, value.0);
  }
  pub unsafe fn with_gaincorr<F: FnOnce(Gaincorr) -> Gaincorr>(&mut self, f: F) {
     let tmp = self.gaincorr();
     self.set_gaincorr(f(tmp))
  }

  pub unsafe fn inputctrl(&self) -> Inputctrl { 
     Inputctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_inputctrl(&mut self, value: Inputctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_inputctrl<F: FnOnce(Inputctrl) -> Inputctrl>(&mut self, f: F) {
     let tmp = self.inputctrl();
     self.set_inputctrl(f(tmp))
  }

  pub unsafe fn intenclr(&self) -> Intenclr { 
     Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x16) as *const u8))
  }
  pub unsafe fn set_intenclr(&mut self, value: Intenclr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x16) as *mut u8, value.0);
  }
  pub unsafe fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&mut self, f: F) {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  pub unsafe fn intenset(&self) -> Intenset { 
     Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x17) as *const u8))
  }
  pub unsafe fn set_intenset(&mut self, value: Intenset) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x17) as *mut u8, value.0);
  }
  pub unsafe fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&mut self, f: F) {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  pub unsafe fn intflag(&self) -> Intflag { 
     Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u8))
  }
  pub unsafe fn set_intflag(&mut self, value: Intflag) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u8, value.0);
  }
  pub unsafe fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&mut self, f: F) {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  pub unsafe fn offsetcorr(&self) -> Offsetcorr { 
     Offsetcorr(::core::ptr::read_volatile(((self.0 as usize) + 0x26) as *const u16))
  }
  pub unsafe fn set_offsetcorr(&mut self, value: Offsetcorr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x26) as *mut u16, value.0);
  }
  pub unsafe fn with_offsetcorr<F: FnOnce(Offsetcorr) -> Offsetcorr>(&mut self, f: F) {
     let tmp = self.offsetcorr();
     self.set_offsetcorr(f(tmp))
  }

  pub unsafe fn refctrl(&self) -> Refctrl { 
     Refctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
  }
  pub unsafe fn set_refctrl(&mut self, value: Refctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
  }
  pub unsafe fn with_refctrl<F: FnOnce(Refctrl) -> Refctrl>(&mut self, f: F) {
     let tmp = self.refctrl();
     self.set_refctrl(f(tmp))
  }

  pub unsafe fn result(&self) -> Result { 
     Result(::core::ptr::read_volatile(((self.0 as usize) + 0x1a) as *const u16))
  }

  pub unsafe fn sampctrl(&self) -> Sampctrl { 
     Sampctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
  }
  pub unsafe fn set_sampctrl(&mut self, value: Sampctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
  }
  pub unsafe fn with_sampctrl<F: FnOnce(Sampctrl) -> Sampctrl>(&mut self, f: F) {
     let tmp = self.sampctrl();
     self.set_sampctrl(f(tmp))
  }

  pub unsafe fn status(&self) -> Status { 
     Status(::core::ptr::read_volatile(((self.0 as usize) + 0x19) as *const u8))
  }

  pub unsafe fn swtrig(&self) -> Swtrig { 
     Swtrig(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
  }
  pub unsafe fn set_swtrig(&mut self, value: Swtrig) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
  }
  pub unsafe fn with_swtrig<F: FnOnce(Swtrig) -> Swtrig>(&mut self, f: F) {
     let tmp = self.swtrig();
     self.set_swtrig(f(tmp))
  }

  pub unsafe fn winctrl(&self) -> Winctrl { 
     Winctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
  }
  pub unsafe fn set_winctrl(&mut self, value: Winctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
  }
  pub unsafe fn with_winctrl<F: FnOnce(Winctrl) -> Winctrl>(&mut self, f: F) {
     let tmp = self.winctrl();
     self.set_winctrl(f(tmp))
  }

  pub unsafe fn winlt(&self) -> Winlt { 
     Winlt(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u16))
  }
  pub unsafe fn set_winlt(&mut self, value: Winlt) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u16, value.0);
  }
  pub unsafe fn with_winlt<F: FnOnce(Winlt) -> Winlt>(&mut self, f: F) {
     let tmp = self.winlt();
     self.set_winlt(f(tmp))
  }

  pub unsafe fn winut(&self) -> Winut { 
     Winut(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u16))
  }
  pub unsafe fn set_winut(&mut self, value: Winut) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u16, value.0);
  }
  pub unsafe fn with_winut<F: FnOnce(Winut) -> Winut>(&mut self, f: F) {
     let tmp = self.winut();
     self.set_winut(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Avgctrl(pub u8);

impl Avgctrl {
  pub fn samplenum(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  pub fn set_samplenum(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn adjres(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x7 // [6:4]
  }
  pub fn set_adjres(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Avgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Avgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.samplenum() != 0 { try!(write!(f, " samplenum=0x{:x}", self.samplenum()))}
      if self.adjres() != 0 { try!(write!(f, " adjres=0x{:x}", self.adjres()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Calib(pub u16);

impl Calib {
  pub fn linearity_cal(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xff // [7:0]
  }
  pub fn set_linearity_cal(mut self, value: u16) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn bias_cal(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
  pub fn set_bias_cal(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Calib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Calib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.linearity_cal() != 0 { try!(write!(f, " linearity_cal=0x{:x}", self.linearity_cal()))}
      if self.bias_cal() != 0 { try!(write!(f, " bias_cal=0x{:x}", self.bias_cal()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u8);

impl Ctrla {
  pub fn swrst(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_swrst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn runstdby(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_runstdby(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}

impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctrlb(pub u16);

impl Ctrlb {
  pub fn diffmode(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  pub fn set_diffmode(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn leftadj(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_leftadj(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn freerun(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  pub fn set_freerun(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn corren(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  pub fn set_corren(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ressel(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x3 // [5:4]
  }
  pub fn set_ressel(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
  pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.diffmode() != 0 { try!(write!(f, " diffmode"))}
      if self.leftadj() != 0 { try!(write!(f, " leftadj"))}
      if self.freerun() != 0 { try!(write!(f, " freerun"))}
      if self.corren() != 0 { try!(write!(f, " corren"))}
      if self.ressel() != 0 { try!(write!(f, " ressel=0x{:x}", self.ressel()))}
      if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);

impl Dbgctrl {
  pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_dbgrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u8);

impl Evctrl {
  pub fn startei(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_startei(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn syncei(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_syncei(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn resrdyeo(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_resrdyeo(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn winmoneo(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_winmoneo(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}

impl ::core::fmt::Display for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Evctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.startei() != 0 { try!(write!(f, " startei"))}
      if self.syncei() != 0 { try!(write!(f, " syncei"))}
      if self.resrdyeo() != 0 { try!(write!(f, " resrdyeo"))}
      if self.winmoneo() != 0 { try!(write!(f, " winmoneo"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Gaincorr(pub u16);

impl Gaincorr {
  pub fn gaincorr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xfff // [11:0]
  }
  pub fn set_gaincorr(mut self, value: u16) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Gaincorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Gaincorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gaincorr() != 0 { try!(write!(f, " gaincorr=0x{:x}", self.gaincorr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Inputctrl(pub u32);

impl Inputctrl {
  pub fn muxpos(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_muxpos(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn muxneg(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  pub fn set_muxneg(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

  pub fn inputscan(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_inputscan(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn inputoffset(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  pub fn set_inputoffset(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  pub fn gain(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  pub fn set_gain(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Inputctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Inputctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.muxpos() != 0 { try!(write!(f, " muxpos=0x{:x}", self.muxpos()))}
      if self.muxneg() != 0 { try!(write!(f, " muxneg=0x{:x}", self.muxneg()))}
      if self.inputscan() != 0 { try!(write!(f, " inputscan=0x{:x}", self.inputscan()))}
      if self.inputoffset() != 0 { try!(write!(f, " inputoffset=0x{:x}", self.inputoffset()))}
      if self.gain() != 0 { try!(write!(f, " gain=0x{:x}", self.gain()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);

impl Intenclr {
  pub fn resrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_resrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn overrun(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_overrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn winmon(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_winmon(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
      if self.overrun() != 0 { try!(write!(f, " overrun"))}
      if self.winmon() != 0 { try!(write!(f, " winmon"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);

impl Intenset {
  pub fn resrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_resrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn overrun(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_overrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn winmon(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_winmon(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
      if self.overrun() != 0 { try!(write!(f, " overrun"))}
      if self.winmon() != 0 { try!(write!(f, " winmon"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);

impl Intflag {
  pub fn resrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_resrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn overrun(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_overrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn winmon(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_winmon(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
      if self.overrun() != 0 { try!(write!(f, " overrun"))}
      if self.winmon() != 0 { try!(write!(f, " winmon"))}
      if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Offsetcorr(pub u16);

impl Offsetcorr {
  pub fn offsetcorr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xfff // [11:0]
  }
  pub fn set_offsetcorr(mut self, value: u16) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Offsetcorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Offsetcorr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offsetcorr() != 0 { try!(write!(f, " offsetcorr=0x{:x}", self.offsetcorr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Refctrl(pub u8);

impl Refctrl {
  pub fn refsel(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  pub fn set_refsel(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn refcomp(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_refcomp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Refctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Refctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
      if self.refcomp() != 0 { try!(write!(f, " refcomp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Result(pub u16);

impl Result {
  pub fn result(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  pub fn set_result(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Result {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Result {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.result() != 0 { try!(write!(f, " result=0x{:x}", self.result()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sampctrl(pub u8);

impl Sampctrl {
  pub fn samplen(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3f // [5:0]
  }
  pub fn set_samplen(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sampctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sampctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.samplen() != 0 { try!(write!(f, " samplen=0x{:x}", self.samplen()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Status(pub u8);

impl Status {
  pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_syncbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Swtrig(pub u8);

impl Swtrig {
  pub fn flush(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_flush(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn start(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_start(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Swtrig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Swtrig {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flush() != 0 { try!(write!(f, " flush"))}
      if self.start() != 0 { try!(write!(f, " start"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Winctrl(pub u8);

impl Winctrl {
  pub fn winmode(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  pub fn set_winmode(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Winctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Winctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.winmode() != 0 { try!(write!(f, " winmode=0x{:x}", self.winmode()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Winlt(pub u16);

impl Winlt {
  pub fn winlt(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  pub fn set_winlt(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Winlt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Winlt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.winlt() != 0 { try!(write!(f, " winlt=0x{:x}", self.winlt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Winut(pub u16);

impl Winut {
  pub fn winut(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  pub fn set_winut(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Winut {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Winut {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.winut() != 0 { try!(write!(f, " winut=0x{:x}", self.winut()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

