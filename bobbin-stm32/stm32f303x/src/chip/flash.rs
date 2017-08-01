//! Flash
pub const FLASH: Flash = Flash(0x40022000);

#[doc="Flash"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flash(pub u32);
impl Flash {
#[doc="Get the *const pointer for the ACR register."]
  #[inline] pub fn acr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the ACR register."]
  #[inline] pub fn acr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the ACR register."]
  #[inline] pub fn acr(&self) -> Acr { 
     unsafe {
        Acr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the ACR register."]
  #[inline] pub fn set_acr(&self, value: Acr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ACR register."]
  #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
     let tmp = self.acr();
     self.set_acr(f(tmp))
  }

#[doc="Get the *const pointer for the KEYR register."]
  #[inline] pub fn keyr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the KEYR register."]
  #[inline] pub fn keyr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Write the KEYR register."]
  #[inline] pub fn set_keyr(&self, value: Keyr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OPTKEYR register."]
  #[inline] pub fn optkeyr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the OPTKEYR register."]
  #[inline] pub fn optkeyr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Write the OPTKEYR register."]
  #[inline] pub fn set_optkeyr(&self, value: Optkeyr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SR register."]
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the SR register."]
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the SR register."]
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the SR register."]
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR register."]
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the AR register."]
  #[inline] pub fn ar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the AR register."]
  #[inline] pub fn ar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Write the AR register."]
  #[inline] pub fn set_ar(&self, value: Ar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OBR register."]
  #[inline] pub fn obr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the OBR register."]
  #[inline] pub fn obr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the OBR register."]
  #[inline] pub fn obr(&self) -> Obr { 
     unsafe {
        Obr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the WRPR register."]
  #[inline] pub fn wrpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the WRPR register."]
  #[inline] pub fn wrpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the WRPR register."]
  #[inline] pub fn wrpr(&self) -> Wrpr { 
     unsafe {
        Wrpr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }

}

#[doc="Flash access control register"]
#[derive(PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
#[doc="LATENCY"]
  #[inline] pub fn latency(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
#[doc="LATENCY"]
  #[inline] pub fn set_latency(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="PRFTBE"]
  #[inline] pub fn prftbe(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="PRFTBE"]
  #[inline] pub fn set_prftbe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="PRFTBS"]
  #[inline] pub fn prftbs(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="PRFTBS"]
  #[inline] pub fn set_prftbs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Acr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Acr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.latency() != 0 { try!(write!(f, " latency=0x{:x}", self.latency()))}
      if self.prftbe() != 0 { try!(write!(f, " prftbe"))}
      if self.prftbs() != 0 { try!(write!(f, " prftbs"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash key register"]
#[derive(PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
#[doc="Flash Key"]
  #[inline] pub fn fkeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Flash Key"]
  #[inline] pub fn set_fkeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Keyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Keyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash option key register"]
#[derive(PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
#[doc="Option byte key"]
  #[inline] pub fn optkeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Option byte key"]
  #[inline] pub fn set_optkeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Optkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Optkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash status register"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="End of operation"]
  #[inline] pub fn eop(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="End of operation"]
  #[inline] pub fn set_eop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Write protection error"]
  #[inline] pub fn wrprt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Write protection error"]
  #[inline] pub fn set_wrprt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Programming error"]
  #[inline] pub fn pgerr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Programming error"]
  #[inline] pub fn set_pgerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Busy"]
  #[inline] pub fn bsy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Busy"]
  #[inline] pub fn set_bsy(mut self, value: u32) -> Self {
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
      if self.eop() != 0 { try!(write!(f, " eop"))}
      if self.wrprt() != 0 { try!(write!(f, " wrprt"))}
      if self.pgerr() != 0 { try!(write!(f, " pgerr"))}
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash control register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Force option byte loading"]
  #[inline] pub fn force_optload(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Force option byte loading"]
  #[inline] pub fn set_force_optload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="End of operation interrupt enable"]
  #[inline] pub fn eopie(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="End of operation interrupt enable"]
  #[inline] pub fn set_eopie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Error interrupt enable"]
  #[inline] pub fn errie(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Error interrupt enable"]
  #[inline] pub fn set_errie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Option bytes write enable"]
  #[inline] pub fn optwre(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Option bytes write enable"]
  #[inline] pub fn set_optwre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Lock"]
  #[inline] pub fn lock(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Lock"]
  #[inline] pub fn set_lock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Start"]
  #[inline] pub fn strt(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Start"]
  #[inline] pub fn set_strt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Option byte erase"]
  #[inline] pub fn opter(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Option byte erase"]
  #[inline] pub fn set_opter(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Option byte programming"]
  #[inline] pub fn optpg(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Option byte programming"]
  #[inline] pub fn set_optpg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Mass erase"]
  #[inline] pub fn mer(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Mass erase"]
  #[inline] pub fn set_mer(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Page erase"]
  #[inline] pub fn per(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Page erase"]
  #[inline] pub fn set_per(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Programming"]
  #[inline] pub fn pg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Programming"]
  #[inline] pub fn set_pg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
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
      if self.force_optload() != 0 { try!(write!(f, " force_optload"))}
      if self.eopie() != 0 { try!(write!(f, " eopie"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.optwre() != 0 { try!(write!(f, " optwre"))}
      if self.lock() != 0 { try!(write!(f, " lock"))}
      if self.strt() != 0 { try!(write!(f, " strt"))}
      if self.opter() != 0 { try!(write!(f, " opter"))}
      if self.optpg() != 0 { try!(write!(f, " optpg"))}
      if self.mer() != 0 { try!(write!(f, " mer"))}
      if self.per() != 0 { try!(write!(f, " per"))}
      if self.pg() != 0 { try!(write!(f, " pg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash address register"]
#[derive(PartialEq, Eq)]
pub struct Ar(pub u32);
impl Ar {
#[doc="Flash address"]
  #[inline] pub fn far(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Flash address"]
  #[inline] pub fn set_far(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Option byte register"]
#[derive(PartialEq, Eq)]
pub struct Obr(pub u32);
impl Obr {
#[doc="Option byte error"]
  #[inline] pub fn opterr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Option byte error"]
  #[inline] pub fn set_opterr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Level 1 protection status"]
  #[inline] pub fn level1_prot(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Level 1 protection status"]
  #[inline] pub fn set_level1_prot(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Level 2 protection status"]
  #[inline] pub fn level2_prot(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Level 2 protection status"]
  #[inline] pub fn set_level2_prot(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="WDG_SW"]
  #[inline] pub fn wdg_sw(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="WDG_SW"]
  #[inline] pub fn set_wdg_sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="nRST_STOP"]
  #[inline] pub fn nrst_stop(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="nRST_STOP"]
  #[inline] pub fn set_nrst_stop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="nRST_STDBY"]
  #[inline] pub fn nrst_stdby(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="nRST_STDBY"]
  #[inline] pub fn set_nrst_stdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="BOOT1"]
  #[inline] pub fn boot1(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="BOOT1"]
  #[inline] pub fn set_boot1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="VDDA_MONITOR"]
  #[inline] pub fn vdda_monitor(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="VDDA_MONITOR"]
  #[inline] pub fn set_vdda_monitor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="SRAM_PARITY_CHECK"]
  #[inline] pub fn sram_parity_check(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="SRAM_PARITY_CHECK"]
  #[inline] pub fn set_sram_parity_check(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Data0"]
  #[inline] pub fn data0(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="Data0"]
  #[inline] pub fn set_data0(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Data1"]
  #[inline] pub fn data1(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
#[doc="Data1"]
  #[inline] pub fn set_data1(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Obr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Obr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.opterr() != 0 { try!(write!(f, " opterr"))}
      if self.level1_prot() != 0 { try!(write!(f, " level1_prot"))}
      if self.level2_prot() != 0 { try!(write!(f, " level2_prot"))}
      if self.wdg_sw() != 0 { try!(write!(f, " wdg_sw"))}
      if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
      if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
      if self.boot1() != 0 { try!(write!(f, " boot1"))}
      if self.vdda_monitor() != 0 { try!(write!(f, " vdda_monitor"))}
      if self.sram_parity_check() != 0 { try!(write!(f, " sram_parity_check"))}
      if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
      if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Write protection register"]
#[derive(PartialEq, Eq)]
pub struct Wrpr(pub u32);
impl Wrpr {
#[doc="Write protect"]
  #[inline] pub fn wrp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Write protect"]
  #[inline] pub fn set_wrp(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Wrpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wrpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

