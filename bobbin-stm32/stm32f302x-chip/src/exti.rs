pub const EXTI: Exti = Exti(0x40010400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exti(pub u32);

impl Exti {
  pub unsafe fn imr1(&self) -> Imr1 { 
     Imr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_imr1(&mut self, value: Imr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_imr1<F: FnOnce(Imr1) -> Imr1>(&mut self, f: F) {
     let tmp = self.imr1();
     self.set_imr1(f(tmp))
  }

  pub unsafe fn emr1(&self) -> Emr1 { 
     Emr1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_emr1(&mut self, value: Emr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_emr1<F: FnOnce(Emr1) -> Emr1>(&mut self, f: F) {
     let tmp = self.emr1();
     self.set_emr1(f(tmp))
  }

  pub unsafe fn rtsr1(&self) -> Rtsr1 { 
     Rtsr1(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_rtsr1(&mut self, value: Rtsr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_rtsr1<F: FnOnce(Rtsr1) -> Rtsr1>(&mut self, f: F) {
     let tmp = self.rtsr1();
     self.set_rtsr1(f(tmp))
  }

  pub unsafe fn ftsr1(&self) -> Ftsr1 { 
     Ftsr1(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_ftsr1(&mut self, value: Ftsr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_ftsr1<F: FnOnce(Ftsr1) -> Ftsr1>(&mut self, f: F) {
     let tmp = self.ftsr1();
     self.set_ftsr1(f(tmp))
  }

  pub unsafe fn swier1(&self) -> Swier1 { 
     Swier1(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_swier1(&mut self, value: Swier1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_swier1<F: FnOnce(Swier1) -> Swier1>(&mut self, f: F) {
     let tmp = self.swier1();
     self.set_swier1(f(tmp))
  }

  pub unsafe fn pr1(&self) -> Pr1 { 
     Pr1(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_pr1(&mut self, value: Pr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_pr1<F: FnOnce(Pr1) -> Pr1>(&mut self, f: F) {
     let tmp = self.pr1();
     self.set_pr1(f(tmp))
  }

  pub unsafe fn imr2(&self) -> Imr2 { 
     Imr2(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_imr2(&mut self, value: Imr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_imr2<F: FnOnce(Imr2) -> Imr2>(&mut self, f: F) {
     let tmp = self.imr2();
     self.set_imr2(f(tmp))
  }

  pub unsafe fn emr2(&self) -> Emr2 { 
     Emr2(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_emr2(&mut self, value: Emr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_emr2<F: FnOnce(Emr2) -> Emr2>(&mut self, f: F) {
     let tmp = self.emr2();
     self.set_emr2(f(tmp))
  }

  pub unsafe fn rtsr2(&self) -> Rtsr2 { 
     Rtsr2(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_rtsr2(&mut self, value: Rtsr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_rtsr2<F: FnOnce(Rtsr2) -> Rtsr2>(&mut self, f: F) {
     let tmp = self.rtsr2();
     self.set_rtsr2(f(tmp))
  }

  pub unsafe fn ftsr2(&self) -> Ftsr2 { 
     Ftsr2(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_ftsr2(&mut self, value: Ftsr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_ftsr2<F: FnOnce(Ftsr2) -> Ftsr2>(&mut self, f: F) {
     let tmp = self.ftsr2();
     self.set_ftsr2(f(tmp))
  }

  pub unsafe fn swier2(&self) -> Swier2 { 
     Swier2(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }
  pub unsafe fn set_swier2(&mut self, value: Swier2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }
  pub unsafe fn with_swier2<F: FnOnce(Swier2) -> Swier2>(&mut self, f: F) {
     let tmp = self.swier2();
     self.set_swier2(f(tmp))
  }

  pub unsafe fn pr2(&self) -> Pr2 { 
     Pr2(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
  }
  pub unsafe fn set_pr2(&mut self, value: Pr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
  }
  pub unsafe fn with_pr2<F: FnOnce(Pr2) -> Pr2>(&mut self, f: F) {
     let tmp = self.pr2();
     self.set_pr2(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Imr1(pub u32);

impl Imr1 {
  pub fn mr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_mr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Imr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Imr1 {
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
pub struct Emr1(pub u32);

impl Emr1 {
  pub fn mr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_mr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Emr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Emr1 {
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
pub struct Rtsr1(pub u32);

impl Rtsr1 {
  pub fn tr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_tr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Rtsr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rtsr1 {
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
pub struct Ftsr1(pub u32);

impl Ftsr1 {
  pub fn tr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_tr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Ftsr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ftsr1 {
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
pub struct Swier1(pub u32);

impl Swier1 {
  pub fn swier(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_swier(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Swier1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Swier1 {
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
      if self.swier(23) != 0 { try!(write!(f, " swier[23]"))}
      if self.swier(24) != 0 { try!(write!(f, " swier[24]"))}
      if self.swier(25) != 0 { try!(write!(f, " swier[25]"))}
      if self.swier(26) != 0 { try!(write!(f, " swier[26]"))}
      if self.swier(27) != 0 { try!(write!(f, " swier[27]"))}
      if self.swier(28) != 0 { try!(write!(f, " swier[28]"))}
      if self.swier(29) != 0 { try!(write!(f, " swier[29]"))}
      if self.swier(30) != 0 { try!(write!(f, " swier[30]"))}
      if self.swier(31) != 0 { try!(write!(f, " swier[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pr1(pub u32);

impl Pr1 {
  pub fn pr(&self, index: usize) -> u32 {
     assert!(index < 32);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_pr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 32);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Pr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pr1 {
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

#[derive(PartialEq, Eq)]
pub struct Imr2(pub u32);

impl Imr2 {
  pub fn mr(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_mr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Imr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Imr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mr(0) != 0 { try!(write!(f, " mr[0]"))}
      if self.mr(1) != 0 { try!(write!(f, " mr[1]"))}
      if self.mr(2) != 0 { try!(write!(f, " mr[2]"))}
      if self.mr(3) != 0 { try!(write!(f, " mr[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Emr2(pub u32);

impl Emr2 {
  pub fn mr(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_mr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Emr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Emr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mr(0) != 0 { try!(write!(f, " mr[0]"))}
      if self.mr(1) != 0 { try!(write!(f, " mr[1]"))}
      if self.mr(2) != 0 { try!(write!(f, " mr[2]"))}
      if self.mr(3) != 0 { try!(write!(f, " mr[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rtsr2(pub u32);

impl Rtsr2 {
  pub fn tr(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_tr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Rtsr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rtsr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tr(0) != 0 { try!(write!(f, " tr[0]"))}
      if self.tr(1) != 0 { try!(write!(f, " tr[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ftsr2(pub u32);

impl Ftsr2 {
  pub fn tr(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_tr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Ftsr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ftsr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tr(0) != 0 { try!(write!(f, " tr[0]"))}
      if self.tr(1) != 0 { try!(write!(f, " tr[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Swier2(pub u32);

impl Swier2 {
  pub fn swier(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_swier(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Swier2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Swier2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swier(0) != 0 { try!(write!(f, " swier[0]"))}
      if self.swier(1) != 0 { try!(write!(f, " swier[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pr2(pub u32);

impl Pr2 {
  pub fn pr(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_pr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Pr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pr(0) != 0 { try!(write!(f, " pr[0]"))}
      if self.pr(1) != 0 { try!(write!(f, " pr[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

