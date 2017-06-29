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

#[doc="Get the *const pointer for the PECR register."]
  #[inline] pub fn pecr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PECR register."]
  #[inline] pub fn pecr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the PECR register."]
  #[inline] pub fn pecr(&self) -> Pecr { 
     unsafe {
        Pecr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the PECR register."]
  #[inline] pub fn set_pecr(&self, value: Pecr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PECR register."]
  #[inline] pub fn with_pecr<F: FnOnce(Pecr) -> Pecr>(&self, f: F) -> &Self {
     let tmp = self.pecr();
     self.set_pecr(f(tmp))
  }

#[doc="Get the *const pointer for the PDKEYR register."]
  #[inline] pub fn pdkeyr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the PDKEYR register."]
  #[inline] pub fn pdkeyr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Write the PDKEYR register."]
  #[inline] pub fn set_pdkeyr(&self, value: Pdkeyr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PEKEYR register."]
  #[inline] pub fn pekeyr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the PEKEYR register."]
  #[inline] pub fn pekeyr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Write the PEKEYR register."]
  #[inline] pub fn set_pekeyr(&self, value: Pekeyr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PRGKEYR register."]
  #[inline] pub fn prgkeyr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the PRGKEYR register."]
  #[inline] pub fn prgkeyr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Write the PRGKEYR register."]
  #[inline] pub fn set_prgkeyr(&self, value: Prgkeyr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OPTKEYR register."]
  #[inline] pub fn optkeyr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the OPTKEYR register."]
  #[inline] pub fn optkeyr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Write the OPTKEYR register."]
  #[inline] pub fn set_optkeyr(&self, value: Optkeyr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SR register."]
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the SR register."]
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the SR register."]
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the SR register."]
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR register."]
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
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
#[doc="Write the WRPR register."]
  #[inline] pub fn set_wrpr(&self, value: Wrpr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the WRPR register."]
  #[inline] pub fn with_wrpr<F: FnOnce(Wrpr) -> Wrpr>(&self, f: F) -> &Self {
     let tmp = self.wrpr();
     self.set_wrpr(f(tmp))
  }

}

#[doc="Access control register"]
#[derive(PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
#[doc="Latency"]
  #[inline] pub fn latency(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Latency"]
  #[inline] pub fn set_latency(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Prefetch enable"]
  #[inline] pub fn prften(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Prefetch enable"]
  #[inline] pub fn set_prften(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Flash mode during Sleep"]
  #[inline] pub fn sleep_pd(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Flash mode during Sleep"]
  #[inline] pub fn set_sleep_pd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Flash mode during Run"]
  #[inline] pub fn run_pd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Flash mode during Run"]
  #[inline] pub fn set_run_pd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Disable Buffer"]
  #[inline] pub fn desab_buf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Disable Buffer"]
  #[inline] pub fn set_desab_buf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Pre-read data address"]
  #[inline] pub fn pre_read(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Pre-read data address"]
  #[inline] pub fn set_pre_read(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
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
      if self.latency() != 0 { try!(write!(f, " latency"))}
      if self.prften() != 0 { try!(write!(f, " prften"))}
      if self.sleep_pd() != 0 { try!(write!(f, " sleep_pd"))}
      if self.run_pd() != 0 { try!(write!(f, " run_pd"))}
      if self.desab_buf() != 0 { try!(write!(f, " desab_buf"))}
      if self.pre_read() != 0 { try!(write!(f, " pre_read"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Program/erase control register"]
#[derive(PartialEq, Eq)]
pub struct Pecr(pub u32);
impl Pecr {
#[doc="FLASH_PECR and data EEPROM lock"]
  #[inline] pub fn pelock(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="FLASH_PECR and data EEPROM lock"]
  #[inline] pub fn set_pelock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Program memory lock"]
  #[inline] pub fn prglock(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Program memory lock"]
  #[inline] pub fn set_prglock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Option bytes block lock"]
  #[inline] pub fn optlock(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Option bytes block lock"]
  #[inline] pub fn set_optlock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Program memory selection"]
  #[inline] pub fn prog(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Program memory selection"]
  #[inline] pub fn set_prog(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Data EEPROM selection"]
  #[inline] pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Data EEPROM selection"]
  #[inline] pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Fixed time data write for Byte, Half Word and Word programming"]
  #[inline] pub fn ftdw(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Fixed time data write for Byte, Half Word and Word programming"]
  #[inline] pub fn set_ftdw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Page or Double Word erase mode"]
  #[inline] pub fn erase(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Page or Double Word erase mode"]
  #[inline] pub fn set_erase(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Half Page/Double Word programming mode"]
  #[inline] pub fn fprg(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Half Page/Double Word programming mode"]
  #[inline] pub fn set_fprg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Parallel bank mode"]
  #[inline] pub fn parallelbank(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Parallel bank mode"]
  #[inline] pub fn set_parallelbank(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="End of programming interrupt enable"]
  #[inline] pub fn eopie(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="End of programming interrupt enable"]
  #[inline] pub fn set_eopie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Error interrupt enable"]
  #[inline] pub fn errie(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Error interrupt enable"]
  #[inline] pub fn set_errie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Launch the option byte loading"]
  #[inline] pub fn obl_launch(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Launch the option byte loading"]
  #[inline] pub fn set_obl_launch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for Pecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pelock() != 0 { try!(write!(f, " pelock"))}
      if self.prglock() != 0 { try!(write!(f, " prglock"))}
      if self.optlock() != 0 { try!(write!(f, " optlock"))}
      if self.prog() != 0 { try!(write!(f, " prog"))}
      if self.data() != 0 { try!(write!(f, " data"))}
      if self.ftdw() != 0 { try!(write!(f, " ftdw"))}
      if self.erase() != 0 { try!(write!(f, " erase"))}
      if self.fprg() != 0 { try!(write!(f, " fprg"))}
      if self.parallelbank() != 0 { try!(write!(f, " parallelbank"))}
      if self.eopie() != 0 { try!(write!(f, " eopie"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.obl_launch() != 0 { try!(write!(f, " obl_launch"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Power down key register"]
#[derive(PartialEq, Eq)]
pub struct Pdkeyr(pub u32);
impl Pdkeyr {
#[doc="RUN_PD in FLASH_ACR key"]
  #[inline] pub fn pdkeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="RUN_PD in FLASH_ACR key"]
  #[inline] pub fn set_pdkeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pdkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pdkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Program/erase key register"]
#[derive(PartialEq, Eq)]
pub struct Pekeyr(pub u32);
impl Pekeyr {
#[doc="FLASH_PEC and data EEPROM key"]
  #[inline] pub fn pekeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="FLASH_PEC and data EEPROM key"]
  #[inline] pub fn set_pekeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pekeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pekeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Program memory key register"]
#[derive(PartialEq, Eq)]
pub struct Prgkeyr(pub u32);
impl Prgkeyr {
#[doc="Program memory key"]
  #[inline] pub fn prgkeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Program memory key"]
  #[inline] pub fn set_prgkeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Prgkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prgkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Option byte key register"]
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
#[doc="Status register"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="Write/erase operations in progress"]
  #[inline] pub fn bsy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Write/erase operations in progress"]
  #[inline] pub fn set_bsy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="End of operation"]
  #[inline] pub fn eop(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="End of operation"]
  #[inline] pub fn set_eop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="End of high voltage"]
  #[inline] pub fn endhv(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="End of high voltage"]
  #[inline] pub fn set_endhv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Flash memory module ready after low power mode"]
  #[inline] pub fn ready(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Flash memory module ready after low power mode"]
  #[inline] pub fn set_ready(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Write protected error"]
  #[inline] pub fn wrperr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Write protected error"]
  #[inline] pub fn set_wrperr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Programming alignment error"]
  #[inline] pub fn pgaerr(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Programming alignment error"]
  #[inline] pub fn set_pgaerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Size error"]
  #[inline] pub fn sizerr(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Size error"]
  #[inline] pub fn set_sizerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Option validity error"]
  #[inline] pub fn optverr(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Option validity error"]
  #[inline] pub fn set_optverr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="RDERR"]
  #[inline] pub fn rderr(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="RDERR"]
  #[inline] pub fn set_rderr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="NOTZEROERR"]
  #[inline] pub fn notzeroerr(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="NOTZEROERR"]
  #[inline] pub fn set_notzeroerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="FWWERR"]
  #[inline] pub fn fwwerr(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="FWWERR"]
  #[inline] pub fn set_fwwerr(mut self, value: u32) -> Self {
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
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      if self.eop() != 0 { try!(write!(f, " eop"))}
      if self.endhv() != 0 { try!(write!(f, " endhv"))}
      if self.ready() != 0 { try!(write!(f, " ready"))}
      if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
      if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
      if self.sizerr() != 0 { try!(write!(f, " sizerr"))}
      if self.optverr() != 0 { try!(write!(f, " optverr"))}
      if self.rderr() != 0 { try!(write!(f, " rderr"))}
      if self.notzeroerr() != 0 { try!(write!(f, " notzeroerr"))}
      if self.fwwerr() != 0 { try!(write!(f, " fwwerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Option byte register"]
#[derive(PartialEq, Eq)]
pub struct Obr(pub u32);
impl Obr {
#[doc="Read protection"]
  #[inline] pub fn rdprt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Read protection"]
  #[inline] pub fn set_rdprt(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="BOR_LEV"]
  #[inline] pub fn bor_lev(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="BOR_LEV"]
  #[inline] pub fn set_bor_lev(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Selection of protection mode of WPR bits"]
  #[inline] pub fn sprmod(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Selection of protection mode of WPR bits"]
  #[inline] pub fn set_sprmod(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
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
      if self.rdprt() != 0 { try!(write!(f, " rdprt=0x{:x}", self.rdprt()))}
      if self.bor_lev() != 0 { try!(write!(f, " bor_lev=0x{:x}", self.bor_lev()))}
      if self.sprmod() != 0 { try!(write!(f, " sprmod"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Write protection register"]
#[derive(PartialEq, Eq)]
pub struct Wrpr(pub u32);
impl Wrpr {
#[doc="Write protection"]
  #[inline] pub fn wrp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Write protection"]
  #[inline] pub fn set_wrp(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
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
      if self.wrp() != 0 { try!(write!(f, " wrp=0x{:x}", self.wrp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

