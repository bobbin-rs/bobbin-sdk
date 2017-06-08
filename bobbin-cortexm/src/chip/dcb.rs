pub const DCB: Dcb = Dcb(0xe000edf0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcb(pub u32);
impl Dcb {
  #[inline] pub fn dhcsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn dhcsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn dhcsr(&self) -> Dhcsr { 
     unsafe {
        Dhcsr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_dhcsr(&self, value: Dhcsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dhcsr<F: FnOnce(Dhcsr) -> Dhcsr>(&self, f: F) -> &Self {
     let tmp = self.dhcsr();
     self.set_dhcsr(f(tmp))
  }

  #[inline] pub fn dcrsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn dcrsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn dcrsr(&self) -> Dcrsr { 
     unsafe {
        Dcrsr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_dcrsr(&self, value: Dcrsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dcrsr<F: FnOnce(Dcrsr) -> Dcrsr>(&self, f: F) -> &Self {
     let tmp = self.dcrsr();
     self.set_dcrsr(f(tmp))
  }

  #[inline] pub fn derdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn derdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn derdr(&self) -> Derdr { 
     unsafe {
        Derdr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_derdr(&self, value: Derdr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_derdr<F: FnOnce(Derdr) -> Derdr>(&self, f: F) -> &Self {
     let tmp = self.derdr();
     self.set_derdr(f(tmp))
  }

  #[inline] pub fn demcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn demcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn demcr(&self) -> Demcr { 
     unsafe {
        Demcr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_demcr(&self, value: Demcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_demcr<F: FnOnce(Demcr) -> Demcr>(&self, f: F) -> &Self {
     let tmp = self.demcr();
     self.set_demcr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Dhcsr(pub u32);
impl Dhcsr {
  #[inline] pub fn dbgkey(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  #[inline] pub fn set_dbgkey(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn s_reset_st(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline] pub fn set_s_reset_st(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline] pub fn s_retire_st(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline] pub fn set_s_retire_st(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn s_lockup(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline] pub fn set_s_lockup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline] pub fn s_sleep(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_s_sleep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline] pub fn s_halt(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_s_halt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn s_regrdy(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_s_regrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn c_snapstall(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_c_snapstall(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn c_maskints(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_c_maskints(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn c_step(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_c_step(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn c_halt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_c_halt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn c_debugen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_c_debugen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dhcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgkey() != 0 { try!(write!(f, " dbgkey=0x{:x}", self.dbgkey()))}
      if self.s_reset_st() != 0 { try!(write!(f, " s_reset_st"))}
      if self.s_retire_st() != 0 { try!(write!(f, " s_retire_st"))}
      if self.s_lockup() != 0 { try!(write!(f, " s_lockup"))}
      if self.s_sleep() != 0 { try!(write!(f, " s_sleep"))}
      if self.s_halt() != 0 { try!(write!(f, " s_halt"))}
      if self.s_regrdy() != 0 { try!(write!(f, " s_regrdy"))}
      if self.c_snapstall() != 0 { try!(write!(f, " c_snapstall"))}
      if self.c_maskints() != 0 { try!(write!(f, " c_maskints"))}
      if self.c_step() != 0 { try!(write!(f, " c_step"))}
      if self.c_halt() != 0 { try!(write!(f, " c_halt"))}
      if self.c_debugen() != 0 { try!(write!(f, " c_debugen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dcrsr(pub u32);
impl Dcrsr {
  #[inline] pub fn regwnr(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_regwnr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn regsel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_regsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dcrsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcrsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.regwnr() != 0 { try!(write!(f, " regwnr"))}
      if self.regsel() != 0 { try!(write!(f, " regsel=0x{:x}", self.regsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Derdr(pub u32);
impl Derdr {
  #[inline] pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Derdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Derdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Demcr(pub u32);
impl Demcr {
  #[inline] pub fn trcena(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline] pub fn set_trcena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn mon_req(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline] pub fn set_mon_req(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline] pub fn mon_step(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline] pub fn set_mon_step(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline] pub fn mon_pend(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_mon_pend(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline] pub fn mon_en(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_mon_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn vc_harderr(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_vc_harderr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn vc_interr(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_vc_interr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn vc_buserr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_vc_buserr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn vc_staterr(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_vc_staterr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn vc_chkerr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_vc_chkerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn vc_nocperr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_vc_nocperr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn vc_mmerr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_vc_mmerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn vc_corereset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_vc_corereset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Demcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Demcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.trcena() != 0 { try!(write!(f, " trcena"))}
      if self.mon_req() != 0 { try!(write!(f, " mon_req"))}
      if self.mon_step() != 0 { try!(write!(f, " mon_step"))}
      if self.mon_pend() != 0 { try!(write!(f, " mon_pend"))}
      if self.mon_en() != 0 { try!(write!(f, " mon_en"))}
      if self.vc_harderr() != 0 { try!(write!(f, " vc_harderr"))}
      if self.vc_interr() != 0 { try!(write!(f, " vc_interr"))}
      if self.vc_buserr() != 0 { try!(write!(f, " vc_buserr"))}
      if self.vc_staterr() != 0 { try!(write!(f, " vc_staterr"))}
      if self.vc_chkerr() != 0 { try!(write!(f, " vc_chkerr"))}
      if self.vc_nocperr() != 0 { try!(write!(f, " vc_nocperr"))}
      if self.vc_mmerr() != 0 { try!(write!(f, " vc_mmerr"))}
      if self.vc_corereset() != 0 { try!(write!(f, " vc_corereset"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

