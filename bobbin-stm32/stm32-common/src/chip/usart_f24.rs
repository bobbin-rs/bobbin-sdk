
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 



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

  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

  #[inline] pub fn brr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn brr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn brr(&self) -> Brr { 
     unsafe {
        Brr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_brr(&self, value: Brr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
     let tmp = self.brr();
     self.set_brr(f(tmp))
  }

  #[inline] pub fn cr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn cr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn cr1(&self) -> Cr1 { 
     unsafe {
        Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_cr1(&self, value: Cr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

  #[inline] pub fn cr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn cr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn cr2(&self) -> Cr2 { 
     unsafe {
        Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_cr2(&self, value: Cr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
     let tmp = self.cr2();
     self.set_cr2(f(tmp))
  }

  #[inline] pub fn cr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline] pub fn cr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline] pub fn cr3(&self) -> Cr3 { 
     unsafe {
        Cr3(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline] pub fn set_cr3(&self, value: Cr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
     let tmp = self.cr3();
     self.set_cr3(f(tmp))
  }

  #[inline] pub fn gtpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline] pub fn gtpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline] pub fn gtpr(&self) -> Gtpr { 
     unsafe {
        Gtpr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline] pub fn set_gtpr(&self, value: Gtpr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_gtpr<F: FnOnce(Gtpr) -> Gtpr>(&self, f: F) -> &Self {
     let tmp = self.gtpr();
     self.set_gtpr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
  #[inline] pub fn cts(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_cts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn lbd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_lbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn txe(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_txe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn tc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_tc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn rxne(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_rxne(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn idle(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_idle(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn ore(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_ore(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn nf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_nf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn fe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pe(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pe(mut self, value: u32) -> Self {
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
      if self.cts() != 0 { try!(write!(f, " cts"))}
      if self.lbd() != 0 { try!(write!(f, " lbd"))}
      if self.txe() != 0 { try!(write!(f, " txe"))}
      if self.tc() != 0 { try!(write!(f, " tc"))}
      if self.rxne() != 0 { try!(write!(f, " rxne"))}
      if self.idle() != 0 { try!(write!(f, " idle"))}
      if self.ore() != 0 { try!(write!(f, " ore"))}
      if self.nf() != 0 { try!(write!(f, " nf"))}
      if self.fe() != 0 { try!(write!(f, " fe"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
  #[inline] pub fn dr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1ff // [8:0]
  }
  #[inline] pub fn set_dr(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 0);
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
      if self.dr() != 0 { try!(write!(f, " dr=0x{:x}", self.dr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
  #[inline] pub fn div_mantissa(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xfff // [15:4]
  }
  #[inline] pub fn set_div_mantissa(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn div_fraction(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_div_fraction(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Brr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Brr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.div_mantissa() != 0 { try!(write!(f, " div_mantissa=0x{:x}", self.div_mantissa()))}
      if self.div_fraction() != 0 { try!(write!(f, " div_fraction=0x{:x}", self.div_fraction()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
  #[inline] pub fn over8(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_over8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline] pub fn ue(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_ue(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn m(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_m(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn wake(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_wake(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn pce(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_pce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn ps(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_ps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn peie(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_peie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn txeie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_txeie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn tcie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_tcie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn rxneie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_rxneie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn idleie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_idleie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn te(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_te(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn re(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn rwu(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_rwu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn sbk(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_sbk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
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
      if self.over8() != 0 { try!(write!(f, " over8"))}
      if self.ue() != 0 { try!(write!(f, " ue"))}
      if self.m() != 0 { try!(write!(f, " m"))}
      if self.wake() != 0 { try!(write!(f, " wake"))}
      if self.pce() != 0 { try!(write!(f, " pce"))}
      if self.ps() != 0 { try!(write!(f, " ps"))}
      if self.peie() != 0 { try!(write!(f, " peie"))}
      if self.txeie() != 0 { try!(write!(f, " txeie"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
      if self.idleie() != 0 { try!(write!(f, " idleie"))}
      if self.te() != 0 { try!(write!(f, " te"))}
      if self.re() != 0 { try!(write!(f, " re"))}
      if self.rwu() != 0 { try!(write!(f, " rwu"))}
      if self.sbk() != 0 { try!(write!(f, " sbk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
  #[inline] pub fn linen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_linen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn stop(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  #[inline] pub fn set_stop(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn clken(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_clken(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn cpol(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_cpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn cpha(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_cpha(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn lbcl(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_lbcl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn lbdie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_lbdie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn lbdl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_lbdl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn add(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_add(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
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
      if self.linen() != 0 { try!(write!(f, " linen"))}
      if self.stop() != 0 { try!(write!(f, " stop=0x{:x}", self.stop()))}
      if self.clken() != 0 { try!(write!(f, " clken"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      if self.lbcl() != 0 { try!(write!(f, " lbcl"))}
      if self.lbdie() != 0 { try!(write!(f, " lbdie"))}
      if self.lbdl() != 0 { try!(write!(f, " lbdl"))}
      if self.add() != 0 { try!(write!(f, " add=0x{:x}", self.add()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr3(pub u32);
impl Cr3 {
  #[inline] pub fn onebit(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_onebit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn ctsie(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_ctsie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn ctse(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_ctse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn rtse(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_rtse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn dmat(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_dmat(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn dmar(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_dmar(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn scen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_scen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn nack(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_nack(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn hdsel(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_hdsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn irlp(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_irlp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn iren(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_iren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn eie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_eie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.onebit() != 0 { try!(write!(f, " onebit"))}
      if self.ctsie() != 0 { try!(write!(f, " ctsie"))}
      if self.ctse() != 0 { try!(write!(f, " ctse"))}
      if self.rtse() != 0 { try!(write!(f, " rtse"))}
      if self.dmat() != 0 { try!(write!(f, " dmat"))}
      if self.dmar() != 0 { try!(write!(f, " dmar"))}
      if self.scen() != 0 { try!(write!(f, " scen"))}
      if self.nack() != 0 { try!(write!(f, " nack"))}
      if self.hdsel() != 0 { try!(write!(f, " hdsel"))}
      if self.irlp() != 0 { try!(write!(f, " irlp"))}
      if self.iren() != 0 { try!(write!(f, " iren"))}
      if self.eie() != 0 { try!(write!(f, " eie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Gtpr(pub u32);
impl Gtpr {
  #[inline] pub fn gt(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  #[inline] pub fn set_gt(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn psc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_psc(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Gtpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gtpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gt() != 0 { try!(write!(f, " gt=0x{:x}", self.gt()))}
      if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
