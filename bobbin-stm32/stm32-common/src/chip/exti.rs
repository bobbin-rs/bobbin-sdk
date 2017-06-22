
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
  #[inline] pub fn imr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn imr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn imr(&self) -> Imr { 
     unsafe {
        Imr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_imr(&self, value: Imr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
     let tmp = self.imr();
     self.set_imr(f(tmp))
  }

  #[inline] pub fn emr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn emr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn emr(&self) -> Emr { 
     unsafe {
        Emr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_emr(&self, value: Emr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_emr<F: FnOnce(Emr) -> Emr>(&self, f: F) -> &Self {
     let tmp = self.emr();
     self.set_emr(f(tmp))
  }

  #[inline] pub fn rtsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn rtsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn rtsr(&self) -> Rtsr { 
     unsafe {
        Rtsr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_rtsr(&self, value: Rtsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_rtsr<F: FnOnce(Rtsr) -> Rtsr>(&self, f: F) -> &Self {
     let tmp = self.rtsr();
     self.set_rtsr(f(tmp))
  }

  #[inline] pub fn ftsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn ftsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn ftsr(&self) -> Ftsr { 
     unsafe {
        Ftsr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_ftsr(&self, value: Ftsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ftsr<F: FnOnce(Ftsr) -> Ftsr>(&self, f: F) -> &Self {
     let tmp = self.ftsr();
     self.set_ftsr(f(tmp))
  }

  #[inline] pub fn swier_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn swier_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn swier(&self) -> Swier { 
     unsafe {
        Swier(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_swier(&self, value: Swier) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_swier<F: FnOnce(Swier) -> Swier>(&self, f: F) -> &Self {
     let tmp = self.swier();
     self.set_swier(f(tmp))
  }

  #[inline] pub fn pr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline] pub fn pr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline] pub fn pr(&self) -> Pr { 
     unsafe {
        Pr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline] pub fn set_pr(&self, value: Pr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pr<F: FnOnce(Pr) -> Pr>(&self, f: F) -> &Self {
     let tmp = self.pr();
     self.set_pr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
  #[inline] pub fn mr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_mr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
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
      if self.mr(30) != 0 { try!(write!(f, " mr[30]"))}
      if self.mr(31) != 0 { try!(write!(f, " mr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Emr(pub u32);
impl Emr {
  #[inline] pub fn mr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_mr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
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
      if self.mr(30) != 0 { try!(write!(f, " mr[30]"))}
      if self.mr(31) != 0 { try!(write!(f, " mr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rtsr(pub u32);
impl Rtsr {
  #[inline] pub fn tr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_tr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
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
      if self.tr(23) != 0 { try!(write!(f, " tr[23]"))}
      if self.tr(24) != 0 { try!(write!(f, " tr[24]"))}
      if self.tr(25) != 0 { try!(write!(f, " tr[25]"))}
      if self.tr(26) != 0 { try!(write!(f, " tr[26]"))}
      if self.tr(27) != 0 { try!(write!(f, " tr[27]"))}
      if self.tr(28) != 0 { try!(write!(f, " tr[28]"))}
      if self.tr(29) != 0 { try!(write!(f, " tr[29]"))}
      if self.tr(30) != 0 { try!(write!(f, " tr[30]"))}
      if self.tr(31) != 0 { try!(write!(f, " tr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ftsr(pub u32);
impl Ftsr {
  #[inline] pub fn tr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_tr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
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
      if self.tr(23) != 0 { try!(write!(f, " tr[23]"))}
      if self.tr(24) != 0 { try!(write!(f, " tr[24]"))}
      if self.tr(25) != 0 { try!(write!(f, " tr[25]"))}
      if self.tr(26) != 0 { try!(write!(f, " tr[26]"))}
      if self.tr(27) != 0 { try!(write!(f, " tr[27]"))}
      if self.tr(28) != 0 { try!(write!(f, " tr[28]"))}
      if self.tr(29) != 0 { try!(write!(f, " tr[29]"))}
      if self.tr(30) != 0 { try!(write!(f, " tr[30]"))}
      if self.tr(31) != 0 { try!(write!(f, " tr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Swier(pub u32);
impl Swier {
  #[inline] pub fn swi(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_swi(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
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
      if self.swi(0) != 0 { try!(write!(f, " swi[0]"))}
      if self.swi(1) != 0 { try!(write!(f, " swi[1]"))}
      if self.swi(2) != 0 { try!(write!(f, " swi[2]"))}
      if self.swi(3) != 0 { try!(write!(f, " swi[3]"))}
      if self.swi(4) != 0 { try!(write!(f, " swi[4]"))}
      if self.swi(5) != 0 { try!(write!(f, " swi[5]"))}
      if self.swi(6) != 0 { try!(write!(f, " swi[6]"))}
      if self.swi(7) != 0 { try!(write!(f, " swi[7]"))}
      if self.swi(8) != 0 { try!(write!(f, " swi[8]"))}
      if self.swi(9) != 0 { try!(write!(f, " swi[9]"))}
      if self.swi(10) != 0 { try!(write!(f, " swi[10]"))}
      if self.swi(11) != 0 { try!(write!(f, " swi[11]"))}
      if self.swi(12) != 0 { try!(write!(f, " swi[12]"))}
      if self.swi(13) != 0 { try!(write!(f, " swi[13]"))}
      if self.swi(14) != 0 { try!(write!(f, " swi[14]"))}
      if self.swi(15) != 0 { try!(write!(f, " swi[15]"))}
      if self.swi(16) != 0 { try!(write!(f, " swi[16]"))}
      if self.swi(17) != 0 { try!(write!(f, " swi[17]"))}
      if self.swi(18) != 0 { try!(write!(f, " swi[18]"))}
      if self.swi(19) != 0 { try!(write!(f, " swi[19]"))}
      if self.swi(20) != 0 { try!(write!(f, " swi[20]"))}
      if self.swi(21) != 0 { try!(write!(f, " swi[21]"))}
      if self.swi(22) != 0 { try!(write!(f, " swi[22]"))}
      if self.swi(23) != 0 { try!(write!(f, " swi[23]"))}
      if self.swi(24) != 0 { try!(write!(f, " swi[24]"))}
      if self.swi(25) != 0 { try!(write!(f, " swi[25]"))}
      if self.swi(26) != 0 { try!(write!(f, " swi[26]"))}
      if self.swi(27) != 0 { try!(write!(f, " swi[27]"))}
      if self.swi(28) != 0 { try!(write!(f, " swi[28]"))}
      if self.swi(29) != 0 { try!(write!(f, " swi[29]"))}
      if self.swi(30) != 0 { try!(write!(f, " swi[30]"))}
      if self.swi(31) != 0 { try!(write!(f, " swi[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pr(pub u32);
impl Pr {
  #[inline] pub fn pr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_pr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
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
      if self.pr(23) != 0 { try!(write!(f, " pr[23]"))}
      if self.pr(24) != 0 { try!(write!(f, " pr[24]"))}
      if self.pr(25) != 0 { try!(write!(f, " pr[25]"))}
      if self.pr(26) != 0 { try!(write!(f, " pr[26]"))}
      if self.pr(27) != 0 { try!(write!(f, " pr[27]"))}
      if self.pr(28) != 0 { try!(write!(f, " pr[28]"))}
      if self.pr(29) != 0 { try!(write!(f, " pr[29]"))}
      if self.pr(30) != 0 { try!(write!(f, " pr[30]"))}
      if self.pr(31) != 0 { try!(write!(f, " pr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

