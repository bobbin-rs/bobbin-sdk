pub const EXTI: Exti = Exti(0x40010400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exti(pub u32);
impl Exti {
  pub fn imr(&self) -> Imr { 
     unsafe {
       Imr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  pub fn set_imr(&self, value: Imr) -> &Exti {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Exti {
     let tmp = self.imr();
     self.set_imr(f(tmp))
  }

  pub fn emr(&self) -> Emr { 
     unsafe {
       Emr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  pub fn set_emr(&self, value: Emr) -> &Exti {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  pub fn with_emr<F: FnOnce(Emr) -> Emr>(&self, f: F) -> &Exti {
     let tmp = self.emr();
     self.set_emr(f(tmp))
  }

  pub fn rtsr(&self) -> Rtsr { 
     unsafe {
       Rtsr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  pub fn set_rtsr(&self, value: Rtsr) -> &Exti {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  pub fn with_rtsr<F: FnOnce(Rtsr) -> Rtsr>(&self, f: F) -> &Exti {
     let tmp = self.rtsr();
     self.set_rtsr(f(tmp))
  }

  pub fn ftsr(&self) -> Ftsr { 
     unsafe {
       Ftsr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  pub fn set_ftsr(&self, value: Ftsr) -> &Exti {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ftsr<F: FnOnce(Ftsr) -> Ftsr>(&self, f: F) -> &Exti {
     let tmp = self.ftsr();
     self.set_ftsr(f(tmp))
  }

  pub fn swier(&self) -> Swier { 
     unsafe {
       Swier(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  pub fn set_swier(&self, value: Swier) -> &Exti {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  pub fn with_swier<F: FnOnce(Swier) -> Swier>(&self, f: F) -> &Exti {
     let tmp = self.swier();
     self.set_swier(f(tmp))
  }

  pub fn pr(&self) -> Pr { 
     unsafe {
       Pr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  pub fn set_pr(&self, value: Pr) -> &Exti {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  pub fn with_pr<F: FnOnce(Pr) -> Pr>(&self, f: F) -> &Exti {
     let tmp = self.pr();
     self.set_pr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
  pub fn mr(&self, index: usize) -> u32 {
     assert!(index < 30);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_mr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 30);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Imr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Imr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mr(0) != 0 { try!(write!(f, " mr[0]"))}
      if self.mr(1) != 0 { try!(write!(f, " mr[1]"))}
      if self.mr(2) != 0 { try!(write!(f, " mr[2]"))}
      if self.mr(3) != 0 { try!(write!(f, " mr[3]"))}
      if self.mr(4) != 0 { try!(write!(f, " mr[4]"))}
      if self.mr(5) != 0 { try!(write!(f, " mr[5]"))}
      if self.mr(6) != 0 { try!(write!(f, " mr[6]"))}
      if self.mr(7) != 0 { try!(write!(f, " mr[7]"))}
      if self.mr(8) != 0 { try!(write!(f, " mr[8]"))}
      if self.mr(9) != 0 { try!(write!(f, " mr[9]"))}
      if self.mr(10) != 0 { try!(write!(f, " mr[10]"))}
      if self.mr(11) != 0 { try!(write!(f, " mr[11]"))}
      if self.mr(12) != 0 { try!(write!(f, " mr[12]"))}
      if self.mr(13) != 0 { try!(write!(f, " mr[13]"))}
      if self.mr(14) != 0 { try!(write!(f, " mr[14]"))}
      if self.mr(15) != 0 { try!(write!(f, " mr[15]"))}
      if self.mr(16) != 0 { try!(write!(f, " mr[16]"))}
      if self.mr(17) != 0 { try!(write!(f, " mr[17]"))}
      if self.mr(18) != 0 { try!(write!(f, " mr[18]"))}
      if self.mr(19) != 0 { try!(write!(f, " mr[19]"))}
      if self.mr(20) != 0 { try!(write!(f, " mr[20]"))}
      if self.mr(21) != 0 { try!(write!(f, " mr[21]"))}
      if self.mr(22) != 0 { try!(write!(f, " mr[22]"))}
      if self.mr(23) != 0 { try!(write!(f, " mr[23]"))}
      if self.mr(24) != 0 { try!(write!(f, " mr[24]"))}
      if self.mr(25) != 0 { try!(write!(f, " mr[25]"))}
      if self.mr(26) != 0 { try!(write!(f, " mr[26]"))}
      if self.mr(27) != 0 { try!(write!(f, " mr[27]"))}
      if self.mr(28) != 0 { try!(write!(f, " mr[28]"))}
      if self.mr(29) != 0 { try!(write!(f, " mr[29]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Emr(pub u32);
impl Emr {
  pub fn mr(&self, index: usize) -> u32 {
     assert!(index < 30);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_mr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 30);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Emr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Emr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mr(0) != 0 { try!(write!(f, " mr[0]"))}
      if self.mr(1) != 0 { try!(write!(f, " mr[1]"))}
      if self.mr(2) != 0 { try!(write!(f, " mr[2]"))}
      if self.mr(3) != 0 { try!(write!(f, " mr[3]"))}
      if self.mr(4) != 0 { try!(write!(f, " mr[4]"))}
      if self.mr(5) != 0 { try!(write!(f, " mr[5]"))}
      if self.mr(6) != 0 { try!(write!(f, " mr[6]"))}
      if self.mr(7) != 0 { try!(write!(f, " mr[7]"))}
      if self.mr(8) != 0 { try!(write!(f, " mr[8]"))}
      if self.mr(9) != 0 { try!(write!(f, " mr[9]"))}
      if self.mr(10) != 0 { try!(write!(f, " mr[10]"))}
      if self.mr(11) != 0 { try!(write!(f, " mr[11]"))}
      if self.mr(12) != 0 { try!(write!(f, " mr[12]"))}
      if self.mr(13) != 0 { try!(write!(f, " mr[13]"))}
      if self.mr(14) != 0 { try!(write!(f, " mr[14]"))}
      if self.mr(15) != 0 { try!(write!(f, " mr[15]"))}
      if self.mr(16) != 0 { try!(write!(f, " mr[16]"))}
      if self.mr(17) != 0 { try!(write!(f, " mr[17]"))}
      if self.mr(18) != 0 { try!(write!(f, " mr[18]"))}
      if self.mr(19) != 0 { try!(write!(f, " mr[19]"))}
      if self.mr(20) != 0 { try!(write!(f, " mr[20]"))}
      if self.mr(21) != 0 { try!(write!(f, " mr[21]"))}
      if self.mr(22) != 0 { try!(write!(f, " mr[22]"))}
      if self.mr(23) != 0 { try!(write!(f, " mr[23]"))}
      if self.mr(24) != 0 { try!(write!(f, " mr[24]"))}
      if self.mr(25) != 0 { try!(write!(f, " mr[25]"))}
      if self.mr(26) != 0 { try!(write!(f, " mr[26]"))}
      if self.mr(27) != 0 { try!(write!(f, " mr[27]"))}
      if self.mr(28) != 0 { try!(write!(f, " mr[28]"))}
      if self.mr(29) != 0 { try!(write!(f, " mr[29]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rtsr(pub u32);
impl Rtsr {
  pub fn tr(&self, index: usize) -> u32 {
     assert!(index < 23);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_tr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 23);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Rtsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rtsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tr(0) != 0 { try!(write!(f, " tr[0]"))}
      if self.tr(1) != 0 { try!(write!(f, " tr[1]"))}
      if self.tr(2) != 0 { try!(write!(f, " tr[2]"))}
      if self.tr(3) != 0 { try!(write!(f, " tr[3]"))}
      if self.tr(4) != 0 { try!(write!(f, " tr[4]"))}
      if self.tr(5) != 0 { try!(write!(f, " tr[5]"))}
      if self.tr(6) != 0 { try!(write!(f, " tr[6]"))}
      if self.tr(7) != 0 { try!(write!(f, " tr[7]"))}
      if self.tr(8) != 0 { try!(write!(f, " tr[8]"))}
      if self.tr(9) != 0 { try!(write!(f, " tr[9]"))}
      if self.tr(10) != 0 { try!(write!(f, " tr[10]"))}
      if self.tr(11) != 0 { try!(write!(f, " tr[11]"))}
      if self.tr(12) != 0 { try!(write!(f, " tr[12]"))}
      if self.tr(13) != 0 { try!(write!(f, " tr[13]"))}
      if self.tr(14) != 0 { try!(write!(f, " tr[14]"))}
      if self.tr(15) != 0 { try!(write!(f, " tr[15]"))}
      if self.tr(16) != 0 { try!(write!(f, " tr[16]"))}
      if self.tr(17) != 0 { try!(write!(f, " tr[17]"))}
      if self.tr(18) != 0 { try!(write!(f, " tr[18]"))}
      if self.tr(19) != 0 { try!(write!(f, " tr[19]"))}
      if self.tr(20) != 0 { try!(write!(f, " tr[20]"))}
      if self.tr(21) != 0 { try!(write!(f, " tr[21]"))}
      if self.tr(22) != 0 { try!(write!(f, " tr[22]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ftsr(pub u32);
impl Ftsr {
  pub fn tr(&self, index: usize) -> u32 {
     assert!(index < 23);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_tr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 23);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ftsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ftsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tr(0) != 0 { try!(write!(f, " tr[0]"))}
      if self.tr(1) != 0 { try!(write!(f, " tr[1]"))}
      if self.tr(2) != 0 { try!(write!(f, " tr[2]"))}
      if self.tr(3) != 0 { try!(write!(f, " tr[3]"))}
      if self.tr(4) != 0 { try!(write!(f, " tr[4]"))}
      if self.tr(5) != 0 { try!(write!(f, " tr[5]"))}
      if self.tr(6) != 0 { try!(write!(f, " tr[6]"))}
      if self.tr(7) != 0 { try!(write!(f, " tr[7]"))}
      if self.tr(8) != 0 { try!(write!(f, " tr[8]"))}
      if self.tr(9) != 0 { try!(write!(f, " tr[9]"))}
      if self.tr(10) != 0 { try!(write!(f, " tr[10]"))}
      if self.tr(11) != 0 { try!(write!(f, " tr[11]"))}
      if self.tr(12) != 0 { try!(write!(f, " tr[12]"))}
      if self.tr(13) != 0 { try!(write!(f, " tr[13]"))}
      if self.tr(14) != 0 { try!(write!(f, " tr[14]"))}
      if self.tr(15) != 0 { try!(write!(f, " tr[15]"))}
      if self.tr(16) != 0 { try!(write!(f, " tr[16]"))}
      if self.tr(17) != 0 { try!(write!(f, " tr[17]"))}
      if self.tr(18) != 0 { try!(write!(f, " tr[18]"))}
      if self.tr(19) != 0 { try!(write!(f, " tr[19]"))}
      if self.tr(20) != 0 { try!(write!(f, " tr[20]"))}
      if self.tr(21) != 0 { try!(write!(f, " tr[21]"))}
      if self.tr(22) != 0 { try!(write!(f, " tr[22]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Swier(pub u32);
impl Swier {
  pub fn swier(&self, index: usize) -> u32 {
     assert!(index < 23);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_swier(mut self, index: usize, value: u32) -> Self {
     assert!(index < 23);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Swier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Swier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swier(0) != 0 { try!(write!(f, " swier[0]"))}
      if self.swier(1) != 0 { try!(write!(f, " swier[1]"))}
      if self.swier(2) != 0 { try!(write!(f, " swier[2]"))}
      if self.swier(3) != 0 { try!(write!(f, " swier[3]"))}
      if self.swier(4) != 0 { try!(write!(f, " swier[4]"))}
      if self.swier(5) != 0 { try!(write!(f, " swier[5]"))}
      if self.swier(6) != 0 { try!(write!(f, " swier[6]"))}
      if self.swier(7) != 0 { try!(write!(f, " swier[7]"))}
      if self.swier(8) != 0 { try!(write!(f, " swier[8]"))}
      if self.swier(9) != 0 { try!(write!(f, " swier[9]"))}
      if self.swier(10) != 0 { try!(write!(f, " swier[10]"))}
      if self.swier(11) != 0 { try!(write!(f, " swier[11]"))}
      if self.swier(12) != 0 { try!(write!(f, " swier[12]"))}
      if self.swier(13) != 0 { try!(write!(f, " swier[13]"))}
      if self.swier(14) != 0 { try!(write!(f, " swier[14]"))}
      if self.swier(15) != 0 { try!(write!(f, " swier[15]"))}
      if self.swier(16) != 0 { try!(write!(f, " swier[16]"))}
      if self.swier(17) != 0 { try!(write!(f, " swier[17]"))}
      if self.swier(18) != 0 { try!(write!(f, " swier[18]"))}
      if self.swier(19) != 0 { try!(write!(f, " swier[19]"))}
      if self.swier(20) != 0 { try!(write!(f, " swier[20]"))}
      if self.swier(21) != 0 { try!(write!(f, " swier[21]"))}
      if self.swier(22) != 0 { try!(write!(f, " swier[22]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pr(pub u32);
impl Pr {
  pub fn pr(&self, index: usize) -> u32 {
     assert!(index < 23);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_pr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 23);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pr(0) != 0 { try!(write!(f, " pr[0]"))}
      if self.pr(1) != 0 { try!(write!(f, " pr[1]"))}
      if self.pr(2) != 0 { try!(write!(f, " pr[2]"))}
      if self.pr(3) != 0 { try!(write!(f, " pr[3]"))}
      if self.pr(4) != 0 { try!(write!(f, " pr[4]"))}
      if self.pr(5) != 0 { try!(write!(f, " pr[5]"))}
      if self.pr(6) != 0 { try!(write!(f, " pr[6]"))}
      if self.pr(7) != 0 { try!(write!(f, " pr[7]"))}
      if self.pr(8) != 0 { try!(write!(f, " pr[8]"))}
      if self.pr(9) != 0 { try!(write!(f, " pr[9]"))}
      if self.pr(10) != 0 { try!(write!(f, " pr[10]"))}
      if self.pr(11) != 0 { try!(write!(f, " pr[11]"))}
      if self.pr(12) != 0 { try!(write!(f, " pr[12]"))}
      if self.pr(13) != 0 { try!(write!(f, " pr[13]"))}
      if self.pr(14) != 0 { try!(write!(f, " pr[14]"))}
      if self.pr(15) != 0 { try!(write!(f, " pr[15]"))}
      if self.pr(16) != 0 { try!(write!(f, " pr[16]"))}
      if self.pr(17) != 0 { try!(write!(f, " pr[17]"))}
      if self.pr(18) != 0 { try!(write!(f, " pr[18]"))}
      if self.pr(19) != 0 { try!(write!(f, " pr[19]"))}
      if self.pr(20) != 0 { try!(write!(f, " pr[20]"))}
      if self.pr(21) != 0 { try!(write!(f, " pr[21]"))}
      if self.pr(22) != 0 { try!(write!(f, " pr[22]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
