//! Reset Control Module
pub const RCM: Rcm = Rcm(0x4007f000);

#[doc="Reset Control Module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcm(pub u32);
impl Rcm {
#[doc="Get the *const pointer for the VERID register."]
  #[inline] pub fn verid_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the VERID register."]
  #[inline] pub fn verid_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the VERID register."]
  #[inline] pub fn verid(&self) -> Verid { 
     unsafe {
        Verid(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PARAM register."]
  #[inline] pub fn param_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PARAM register."]
  #[inline] pub fn param_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the PARAM register."]
  #[inline] pub fn param(&self) -> Param { 
     unsafe {
        Param(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }

#[doc="Get the *const pointer for the SRS register."]
  #[inline] pub fn srs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the SRS register."]
  #[inline] pub fn srs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the SRS register."]
  #[inline] pub fn srs(&self) -> Srs { 
     unsafe {
        Srs(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RPC register."]
  #[inline] pub fn rpc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the RPC register."]
  #[inline] pub fn rpc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the RPC register."]
  #[inline] pub fn rpc(&self) -> Rpc { 
     unsafe {
        Rpc(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the RPC register."]
  #[inline] pub fn set_rpc(&self, value: Rpc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RPC register."]
  #[inline] pub fn with_rpc<F: FnOnce(Rpc) -> Rpc>(&self, f: F) -> &Self {
     let tmp = self.rpc();
     self.set_rpc(f(tmp))
  }

#[doc="Get the *const pointer for the SSRS register."]
  #[inline] pub fn ssrs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the SSRS register."]
  #[inline] pub fn ssrs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the SSRS register."]
  #[inline] pub fn ssrs(&self) -> Ssrs { 
     unsafe {
        Ssrs(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the SSRS register."]
  #[inline] pub fn set_ssrs(&self, value: Ssrs) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SSRS register."]
  #[inline] pub fn with_ssrs<F: FnOnce(Ssrs) -> Ssrs>(&self, f: F) -> &Self {
     let tmp = self.ssrs();
     self.set_ssrs(f(tmp))
  }

#[doc="Get the *const pointer for the SRIE register."]
  #[inline] pub fn srie_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the SRIE register."]
  #[inline] pub fn srie_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the SRIE register."]
  #[inline] pub fn srie(&self) -> Srie { 
     unsafe {
        Srie(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the SRIE register."]
  #[inline] pub fn set_srie(&self, value: Srie) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SRIE register."]
  #[inline] pub fn with_srie<F: FnOnce(Srie) -> Srie>(&self, f: F) -> &Self {
     let tmp = self.srie();
     self.set_srie(f(tmp))
  }

}

#[doc="Version ID Register"]
#[derive(PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
#[doc="Feature Specification Number"]
  #[inline] pub fn feature(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Feature Specification Number"]
  #[inline] pub fn set_feature(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Minor Version Number"]
  #[inline] pub fn minor(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="Minor Version Number"]
  #[inline] pub fn set_minor(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Major Version Number"]
  #[inline] pub fn major(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
#[doc="Major Version Number"]
  #[inline] pub fn set_major(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
      if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
      if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Parameter Register"]
#[derive(PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
#[doc="Existence of SRS[WAKEUP] status indication feature"]
  #[inline] pub fn ewakeup(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Existence of SRS[WAKEUP] status indication feature"]
  #[inline] pub fn set_ewakeup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Existence of SRS[LVD] status indication feature"]
  #[inline] pub fn elvd(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Existence of SRS[LVD] status indication feature"]
  #[inline] pub fn set_elvd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Existence of SRS[LOC] status indication feature"]
  #[inline] pub fn eloc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Existence of SRS[LOC] status indication feature"]
  #[inline] pub fn set_eloc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Existence of SRS[LOL] status indication feature"]
  #[inline] pub fn elol(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Existence of SRS[LOL] status indication feature"]
  #[inline] pub fn set_elol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Existence of SRS[WDOG] status indication feature"]
  #[inline] pub fn ewdog(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Existence of SRS[WDOG] status indication feature"]
  #[inline] pub fn set_ewdog(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Existence of SRS[PIN] status indication feature"]
  #[inline] pub fn epin(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Existence of SRS[PIN] status indication feature"]
  #[inline] pub fn set_epin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Existence of SRS[POR] status indication feature"]
  #[inline] pub fn epor(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Existence of SRS[POR] status indication feature"]
  #[inline] pub fn set_epor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Existence of SRS[JTAG] status indication feature"]
  #[inline] pub fn ejtag(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Existence of SRS[JTAG] status indication feature"]
  #[inline] pub fn set_ejtag(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Existence of SRS[LOCKUP] status indication feature"]
  #[inline] pub fn elockup(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Existence of SRS[LOCKUP] status indication feature"]
  #[inline] pub fn set_elockup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Existence of SRS[SW] status indication feature"]
  #[inline] pub fn esw(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Existence of SRS[SW] status indication feature"]
  #[inline] pub fn set_esw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Existence of SRS[MDM_AP] status indication feature"]
  #[inline] pub fn emdm_ap(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Existence of SRS[MDM_AP] status indication feature"]
  #[inline] pub fn set_emdm_ap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Existence of SRS[SACKERR] status indication feature"]
  #[inline] pub fn esackerr(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Existence of SRS[SACKERR] status indication feature"]
  #[inline] pub fn set_esackerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Existence of SRS[TAMPER] status indication feature"]
  #[inline] pub fn etamper(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Existence of SRS[TAMPER] status indication feature"]
  #[inline] pub fn set_etamper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Existence of SRS[CORE1] status indication feature"]
  #[inline] pub fn ecore1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Existence of SRS[CORE1] status indication feature"]
  #[inline] pub fn set_ecore1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ewakeup() != 0 { try!(write!(f, " ewakeup"))}
      if self.elvd() != 0 { try!(write!(f, " elvd"))}
      if self.eloc() != 0 { try!(write!(f, " eloc"))}
      if self.elol() != 0 { try!(write!(f, " elol"))}
      if self.ewdog() != 0 { try!(write!(f, " ewdog"))}
      if self.epin() != 0 { try!(write!(f, " epin"))}
      if self.epor() != 0 { try!(write!(f, " epor"))}
      if self.ejtag() != 0 { try!(write!(f, " ejtag"))}
      if self.elockup() != 0 { try!(write!(f, " elockup"))}
      if self.esw() != 0 { try!(write!(f, " esw"))}
      if self.emdm_ap() != 0 { try!(write!(f, " emdm_ap"))}
      if self.esackerr() != 0 { try!(write!(f, " esackerr"))}
      if self.etamper() != 0 { try!(write!(f, " etamper"))}
      if self.ecore1() != 0 { try!(write!(f, " ecore1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Reset Status Register"]
#[derive(PartialEq, Eq)]
pub struct Srs(pub u32);
impl Srs {
#[doc="Low-Voltage Detect Reset or High-Voltage Detect Reset"]
  #[inline] pub fn lvd(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Low-Voltage Detect Reset or High-Voltage Detect Reset"]
  #[inline] pub fn set_lvd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Loss-of-Clock Reset"]
  #[inline] pub fn loc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Loss-of-Clock Reset"]
  #[inline] pub fn set_loc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Loss-of-Lock Reset"]
  #[inline] pub fn lol(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Loss-of-Lock Reset"]
  #[inline] pub fn set_lol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Watchdog"]
  #[inline] pub fn wdog(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Watchdog"]
  #[inline] pub fn set_wdog(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="External Reset Pin"]
  #[inline] pub fn pin(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="External Reset Pin"]
  #[inline] pub fn set_pin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Power-On Reset"]
  #[inline] pub fn por(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Power-On Reset"]
  #[inline] pub fn set_por(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="JTAG generated reset"]
  #[inline] pub fn jtag(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="JTAG generated reset"]
  #[inline] pub fn set_jtag(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Core Lockup"]
  #[inline] pub fn lockup(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Core Lockup"]
  #[inline] pub fn set_lockup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Software"]
  #[inline] pub fn sw(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Software"]
  #[inline] pub fn set_sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="MDM-AP System Reset Request"]
  #[inline] pub fn mdm_ap(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="MDM-AP System Reset Request"]
  #[inline] pub fn set_mdm_ap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Stop Acknowledge Error"]
  #[inline] pub fn sackerr(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Stop Acknowledge Error"]
  #[inline] pub fn set_sackerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Srs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lvd() != 0 { try!(write!(f, " lvd"))}
      if self.loc() != 0 { try!(write!(f, " loc"))}
      if self.lol() != 0 { try!(write!(f, " lol"))}
      if self.wdog() != 0 { try!(write!(f, " wdog"))}
      if self.pin() != 0 { try!(write!(f, " pin"))}
      if self.por() != 0 { try!(write!(f, " por"))}
      if self.jtag() != 0 { try!(write!(f, " jtag"))}
      if self.lockup() != 0 { try!(write!(f, " lockup"))}
      if self.sw() != 0 { try!(write!(f, " sw"))}
      if self.mdm_ap() != 0 { try!(write!(f, " mdm_ap"))}
      if self.sackerr() != 0 { try!(write!(f, " sackerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Reset Pin Control register"]
#[derive(PartialEq, Eq)]
pub struct Rpc(pub u32);
impl Rpc {
#[doc="Reset Pin Filter Select in Run and Wait Modes"]
  #[inline] pub fn rstfltsrw(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="Reset Pin Filter Select in Run and Wait Modes"]
  #[inline] pub fn set_rstfltsrw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Reset Pin Filter Select in Stop Mode"]
  #[inline] pub fn rstfltss(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Reset Pin Filter Select in Stop Mode"]
  #[inline] pub fn set_rstfltss(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Reset Pin Filter Bus Clock Select"]
  #[inline] pub fn rstfltsel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
#[doc="Reset Pin Filter Bus Clock Select"]
  #[inline] pub fn set_rstfltsel(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Rpc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rpc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rstfltsrw() != 0 { try!(write!(f, " rstfltsrw=0x{:x}", self.rstfltsrw()))}
      if self.rstfltss() != 0 { try!(write!(f, " rstfltss"))}
      if self.rstfltsel() != 0 { try!(write!(f, " rstfltsel=0x{:x}", self.rstfltsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Sticky System Reset Status Register"]
#[derive(PartialEq, Eq)]
pub struct Ssrs(pub u32);
impl Ssrs {
#[doc="Sticky Low-Voltage Detect Reset"]
  #[inline] pub fn slvd(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Sticky Low-Voltage Detect Reset"]
  #[inline] pub fn set_slvd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Sticky Loss-of-Clock Reset"]
  #[inline] pub fn sloc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Sticky Loss-of-Clock Reset"]
  #[inline] pub fn set_sloc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Sticky Loss-of-Lock Reset"]
  #[inline] pub fn slol(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Sticky Loss-of-Lock Reset"]
  #[inline] pub fn set_slol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Sticky Watchdog"]
  #[inline] pub fn swdog(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Sticky Watchdog"]
  #[inline] pub fn set_swdog(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Sticky External Reset Pin"]
  #[inline] pub fn spin(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Sticky External Reset Pin"]
  #[inline] pub fn set_spin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Sticky Power-On Reset"]
  #[inline] pub fn spor(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Sticky Power-On Reset"]
  #[inline] pub fn set_spor(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Sticky JTAG generated reset"]
  #[inline] pub fn sjtag(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Sticky JTAG generated reset"]
  #[inline] pub fn set_sjtag(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Sticky Core Lockup"]
  #[inline] pub fn slockup(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Sticky Core Lockup"]
  #[inline] pub fn set_slockup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Sticky Software"]
  #[inline] pub fn ssw(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Sticky Software"]
  #[inline] pub fn set_ssw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Sticky MDM-AP System Reset Request"]
  #[inline] pub fn smdm_ap(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Sticky MDM-AP System Reset Request"]
  #[inline] pub fn set_smdm_ap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Sticky Stop Acknowledge Error"]
  #[inline] pub fn ssackerr(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Sticky Stop Acknowledge Error"]
  #[inline] pub fn set_ssackerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Ssrs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ssrs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.slvd() != 0 { try!(write!(f, " slvd"))}
      if self.sloc() != 0 { try!(write!(f, " sloc"))}
      if self.slol() != 0 { try!(write!(f, " slol"))}
      if self.swdog() != 0 { try!(write!(f, " swdog"))}
      if self.spin() != 0 { try!(write!(f, " spin"))}
      if self.spor() != 0 { try!(write!(f, " spor"))}
      if self.sjtag() != 0 { try!(write!(f, " sjtag"))}
      if self.slockup() != 0 { try!(write!(f, " slockup"))}
      if self.ssw() != 0 { try!(write!(f, " ssw"))}
      if self.smdm_ap() != 0 { try!(write!(f, " smdm_ap"))}
      if self.ssackerr() != 0 { try!(write!(f, " ssackerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Reset Interrupt Enable Register"]
#[derive(PartialEq, Eq)]
pub struct Srie(pub u32);
impl Srie {
#[doc="Reset Delay Time"]
  #[inline] pub fn delay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="Reset Delay Time"]
  #[inline] pub fn set_delay(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Loss-of-Clock Interrupt"]
  #[inline] pub fn loc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Loss-of-Clock Interrupt"]
  #[inline] pub fn set_loc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Loss-of-Lock Interrupt"]
  #[inline] pub fn lol(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Loss-of-Lock Interrupt"]
  #[inline] pub fn set_lol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Watchdog Interrupt"]
  #[inline] pub fn wdog(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Watchdog Interrupt"]
  #[inline] pub fn set_wdog(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="External Reset Pin Interrupt"]
  #[inline] pub fn pin(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="External Reset Pin Interrupt"]
  #[inline] pub fn set_pin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Global Interrupt Enable"]
  #[inline] pub fn gie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Global Interrupt Enable"]
  #[inline] pub fn set_gie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="JTAG generated reset"]
  #[inline] pub fn jtag(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="JTAG generated reset"]
  #[inline] pub fn set_jtag(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Core Lockup Interrupt"]
  #[inline] pub fn lockup(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Core Lockup Interrupt"]
  #[inline] pub fn set_lockup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Software Interrupt"]
  #[inline] pub fn sw(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Software Interrupt"]
  #[inline] pub fn set_sw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="MDM-AP System Reset Request"]
  #[inline] pub fn mdm_ap(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="MDM-AP System Reset Request"]
  #[inline] pub fn set_mdm_ap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Stop Acknowledge Error Interrupt"]
  #[inline] pub fn sackerr(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Stop Acknowledge Error Interrupt"]
  #[inline] pub fn set_sackerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Srie {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srie {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
      if self.loc() != 0 { try!(write!(f, " loc"))}
      if self.lol() != 0 { try!(write!(f, " lol"))}
      if self.wdog() != 0 { try!(write!(f, " wdog"))}
      if self.pin() != 0 { try!(write!(f, " pin"))}
      if self.gie() != 0 { try!(write!(f, " gie"))}
      if self.jtag() != 0 { try!(write!(f, " jtag"))}
      if self.lockup() != 0 { try!(write!(f, " lockup"))}
      if self.sw() != 0 { try!(write!(f, " sw"))}
      if self.mdm_ap() != 0 { try!(write!(f, " mdm_ap"))}
      if self.sackerr() != 0 { try!(write!(f, " sackerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

