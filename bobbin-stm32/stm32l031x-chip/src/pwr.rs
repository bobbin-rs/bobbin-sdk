pub const PWR: Pwr = Pwr(0x40007000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pwr(pub u32);
impl Pwr {
  pub fn cr(&self) -> Cr { 
     unsafe {
       Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  pub fn set_cr(&self, value: Cr) -> &Pwr {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Pwr {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  pub fn csr(&self) -> Csr { 
     unsafe {
       Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  pub fn set_csr(&self, value: Csr) -> &Pwr {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Pwr {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
  pub fn lpds(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_lpds(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn pdds(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_pdds(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cwuf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_cwuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn csbf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_csbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn pvde(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_pvde(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn pls(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7 // [7:5]
  }
  pub fn set_pls(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn dbp(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dbp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn fpds(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_fpds(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn fwu(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_fwu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn vos(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x3 // [12:11]
  }
  pub fn set_vos(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn ds_ee_koff(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_ds_ee_koff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn lprun(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_lprun(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
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
      if self.lpds() != 0 { try!(write!(f, " lpds"))}
      if self.pdds() != 0 { try!(write!(f, " pdds"))}
      if self.cwuf() != 0 { try!(write!(f, " cwuf"))}
      if self.csbf() != 0 { try!(write!(f, " csbf"))}
      if self.pvde() != 0 { try!(write!(f, " pvde"))}
      if self.pls() != 0 { try!(write!(f, " pls=0x{:x}", self.pls()))}
      if self.dbp() != 0 { try!(write!(f, " dbp"))}
      if self.fpds() != 0 { try!(write!(f, " fpds"))}
      if self.fwu() != 0 { try!(write!(f, " fwu"))}
      if self.vos() != 0 { try!(write!(f, " vos=0x{:x}", self.vos()))}
      if self.ds_ee_koff() != 0 { try!(write!(f, " ds_ee_koff"))}
      if self.lprun() != 0 { try!(write!(f, " lprun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
  pub fn bre(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_bre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn ewup(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_ewup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn brr(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_brr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn pvdo(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_pvdo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn sbf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_sbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn wuf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_wuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn vosf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_vosf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn reglpf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_reglpf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
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
      if self.bre() != 0 { try!(write!(f, " bre"))}
      if self.ewup() != 0 { try!(write!(f, " ewup"))}
      if self.brr() != 0 { try!(write!(f, " brr"))}
      if self.pvdo() != 0 { try!(write!(f, " pvdo"))}
      if self.sbf() != 0 { try!(write!(f, " sbf"))}
      if self.wuf() != 0 { try!(write!(f, " wuf"))}
      if self.vosf() != 0 { try!(write!(f, " vosf"))}
      if self.reglpf() != 0 { try!(write!(f, " reglpf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
