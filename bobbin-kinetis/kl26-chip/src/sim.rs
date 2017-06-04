pub const SIM: Sim = Sim(0x40047000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sim(pub u32);
impl Sim {
  #[inline]
  pub fn sopt1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline]
  pub fn sopt1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline]
  pub fn sopt1(&self) -> Sopt1 { 
     unsafe {
       Sopt1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline]
  pub fn set_sopt1(&self, value: Sopt1) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sopt1<F: FnOnce(Sopt1) -> Sopt1>(&self, f: F) -> &Self {
     let tmp = self.sopt1();
     self.set_sopt1(f(tmp))
  }

  #[inline]
  pub fn sopt1cfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline]
  pub fn sopt1cfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline]
  pub fn sopt1cfg(&self) -> Sopt1cfg { 
     unsafe {
       Sopt1cfg(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline]
  pub fn set_sopt1cfg(&self, value: Sopt1cfg) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sopt1cfg<F: FnOnce(Sopt1cfg) -> Sopt1cfg>(&self, f: F) -> &Self {
     let tmp = self.sopt1cfg();
     self.set_sopt1cfg(f(tmp))
  }

  #[inline]
  pub fn sopt2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1004) as *const u32
  }
  #[inline]
  pub fn sopt2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1004) as *mut u32
  }
  #[inline]
  pub fn sopt2(&self) -> Sopt2 { 
     unsafe {
       Sopt2(::core::ptr::read_volatile(((self.0 as usize) + 0x1004) as *const u32))
     }
  }
  #[inline]
  pub fn set_sopt2(&self, value: Sopt2) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1004) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sopt2<F: FnOnce(Sopt2) -> Sopt2>(&self, f: F) -> &Self {
     let tmp = self.sopt2();
     self.set_sopt2(f(tmp))
  }

  #[inline]
  pub fn sopt4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x100c) as *const u32
  }
  #[inline]
  pub fn sopt4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x100c) as *mut u32
  }
  #[inline]
  pub fn sopt4(&self) -> Sopt4 { 
     unsafe {
       Sopt4(::core::ptr::read_volatile(((self.0 as usize) + 0x100c) as *const u32))
     }
  }
  #[inline]
  pub fn set_sopt4(&self, value: Sopt4) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x100c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sopt4<F: FnOnce(Sopt4) -> Sopt4>(&self, f: F) -> &Self {
     let tmp = self.sopt4();
     self.set_sopt4(f(tmp))
  }

  #[inline]
  pub fn sopt5_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1010) as *const u32
  }
  #[inline]
  pub fn sopt5_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1010) as *mut u32
  }
  #[inline]
  pub fn sopt5(&self) -> Sopt5 { 
     unsafe {
       Sopt5(::core::ptr::read_volatile(((self.0 as usize) + 0x1010) as *const u32))
     }
  }
  #[inline]
  pub fn set_sopt5(&self, value: Sopt5) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1010) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sopt5<F: FnOnce(Sopt5) -> Sopt5>(&self, f: F) -> &Self {
     let tmp = self.sopt5();
     self.set_sopt5(f(tmp))
  }

  #[inline]
  pub fn sopt7_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1018) as *const u32
  }
  #[inline]
  pub fn sopt7_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1018) as *mut u32
  }
  #[inline]
  pub fn sopt7(&self) -> Sopt7 { 
     unsafe {
       Sopt7(::core::ptr::read_volatile(((self.0 as usize) + 0x1018) as *const u32))
     }
  }
  #[inline]
  pub fn set_sopt7(&self, value: Sopt7) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1018) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_sopt7<F: FnOnce(Sopt7) -> Sopt7>(&self, f: F) -> &Self {
     let tmp = self.sopt7();
     self.set_sopt7(f(tmp))
  }

  #[inline]
  pub fn sdid_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1024) as *const u32
  }
  #[inline]
  pub fn sdid_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1024) as *mut u32
  }
  #[inline]
  pub fn sdid(&self) -> Sdid { 
     unsafe {
       Sdid(::core::ptr::read_volatile(((self.0 as usize) + 0x1024) as *const u32))
     }
  }

  #[inline]
  pub fn scgc4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1034) as *const u32
  }
  #[inline]
  pub fn scgc4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1034) as *mut u32
  }
  #[inline]
  pub fn scgc4(&self) -> Scgc4 { 
     unsafe {
       Scgc4(::core::ptr::read_volatile(((self.0 as usize) + 0x1034) as *const u32))
     }
  }
  #[inline]
  pub fn set_scgc4(&self, value: Scgc4) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1034) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_scgc4<F: FnOnce(Scgc4) -> Scgc4>(&self, f: F) -> &Self {
     let tmp = self.scgc4();
     self.set_scgc4(f(tmp))
  }

  #[inline]
  pub fn scgc5_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1038) as *const u32
  }
  #[inline]
  pub fn scgc5_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1038) as *mut u32
  }
  #[inline]
  pub fn scgc5(&self) -> Scgc5 { 
     unsafe {
       Scgc5(::core::ptr::read_volatile(((self.0 as usize) + 0x1038) as *const u32))
     }
  }
  #[inline]
  pub fn set_scgc5(&self, value: Scgc5) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1038) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_scgc5<F: FnOnce(Scgc5) -> Scgc5>(&self, f: F) -> &Self {
     let tmp = self.scgc5();
     self.set_scgc5(f(tmp))
  }

  #[inline]
  pub fn scgc6_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x103c) as *const u32
  }
  #[inline]
  pub fn scgc6_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x103c) as *mut u32
  }
  #[inline]
  pub fn scgc6(&self) -> Scgc6 { 
     unsafe {
       Scgc6(::core::ptr::read_volatile(((self.0 as usize) + 0x103c) as *const u32))
     }
  }
  #[inline]
  pub fn set_scgc6(&self, value: Scgc6) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x103c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_scgc6<F: FnOnce(Scgc6) -> Scgc6>(&self, f: F) -> &Self {
     let tmp = self.scgc6();
     self.set_scgc6(f(tmp))
  }

  #[inline]
  pub fn scgc7_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1040) as *const u32
  }
  #[inline]
  pub fn scgc7_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1040) as *mut u32
  }
  #[inline]
  pub fn scgc7(&self) -> Scgc7 { 
     unsafe {
       Scgc7(::core::ptr::read_volatile(((self.0 as usize) + 0x1040) as *const u32))
     }
  }
  #[inline]
  pub fn set_scgc7(&self, value: Scgc7) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1040) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_scgc7<F: FnOnce(Scgc7) -> Scgc7>(&self, f: F) -> &Self {
     let tmp = self.scgc7();
     self.set_scgc7(f(tmp))
  }

  #[inline]
  pub fn clkdiv1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1044) as *const u32
  }
  #[inline]
  pub fn clkdiv1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1044) as *mut u32
  }
  #[inline]
  pub fn clkdiv1(&self) -> Clkdiv1 { 
     unsafe {
       Clkdiv1(::core::ptr::read_volatile(((self.0 as usize) + 0x1044) as *const u32))
     }
  }
  #[inline]
  pub fn set_clkdiv1(&self, value: Clkdiv1) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1044) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_clkdiv1<F: FnOnce(Clkdiv1) -> Clkdiv1>(&self, f: F) -> &Self {
     let tmp = self.clkdiv1();
     self.set_clkdiv1(f(tmp))
  }

  #[inline]
  pub fn fcfg1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x104c) as *const u32
  }
  #[inline]
  pub fn fcfg1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x104c) as *mut u32
  }
  #[inline]
  pub fn fcfg1(&self) -> Fcfg1 { 
     unsafe {
       Fcfg1(::core::ptr::read_volatile(((self.0 as usize) + 0x104c) as *const u32))
     }
  }
  #[inline]
  pub fn set_fcfg1(&self, value: Fcfg1) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x104c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_fcfg1<F: FnOnce(Fcfg1) -> Fcfg1>(&self, f: F) -> &Self {
     let tmp = self.fcfg1();
     self.set_fcfg1(f(tmp))
  }

  #[inline]
  pub fn fcfg2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1050) as *const u32
  }
  #[inline]
  pub fn fcfg2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1050) as *mut u32
  }
  #[inline]
  pub fn fcfg2(&self) -> Fcfg2 { 
     unsafe {
       Fcfg2(::core::ptr::read_volatile(((self.0 as usize) + 0x1050) as *const u32))
     }
  }

  #[inline]
  pub fn uidmh_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1058) as *const u32
  }
  #[inline]
  pub fn uidmh_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1058) as *mut u32
  }
  #[inline]
  pub fn uidmh(&self) -> Uidmh { 
     unsafe {
       Uidmh(::core::ptr::read_volatile(((self.0 as usize) + 0x1058) as *const u32))
     }
  }

  #[inline]
  pub fn uidml_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x105c) as *const u32
  }
  #[inline]
  pub fn uidml_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x105c) as *mut u32
  }
  #[inline]
  pub fn uidml(&self) -> Uidml { 
     unsafe {
       Uidml(::core::ptr::read_volatile(((self.0 as usize) + 0x105c) as *const u32))
     }
  }

  #[inline]
  pub fn uidl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1060) as *const u32
  }
  #[inline]
  pub fn uidl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1060) as *mut u32
  }
  #[inline]
  pub fn uidl(&self) -> Uidl { 
     unsafe {
       Uidl(::core::ptr::read_volatile(((self.0 as usize) + 0x1060) as *const u32))
     }
  }

  #[inline]
  pub fn copc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1100) as *const u32
  }
  #[inline]
  pub fn copc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1100) as *mut u32
  }
  #[inline]
  pub fn copc(&self) -> Copc { 
     unsafe {
       Copc(::core::ptr::read_volatile(((self.0 as usize) + 0x1100) as *const u32))
     }
  }
  #[inline]
  pub fn set_copc(&self, value: Copc) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1100) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_copc<F: FnOnce(Copc) -> Copc>(&self, f: F) -> &Self {
     let tmp = self.copc();
     self.set_copc(f(tmp))
  }

  #[inline]
  pub fn srvcop_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1104) as *const u32
  }
  #[inline]
  pub fn srvcop_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1104) as *mut u32
  }
  #[inline]
  pub fn set_srvcop(&self, value: Srvcop) -> &Self {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1104) as *mut u32, value.0);
     }
     self
  }

}

#[derive(PartialEq, Eq)]
pub struct Sopt1(pub u32);
impl Sopt1 {
  #[inline]
  pub fn osc32ksel(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x3 // [19:18]
  }
  #[inline]
  pub fn set_osc32ksel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn usbvstby(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  #[inline]
  pub fn set_usbvstby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  #[inline]
  pub fn usbsstby(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline]
  pub fn set_usbsstby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline]
  pub fn usbregen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
  pub fn set_usbregen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Sopt1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sopt1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.osc32ksel() != 0 { try!(write!(f, " osc32ksel=0x{:x}", self.osc32ksel()))}
      if self.usbvstby() != 0 { try!(write!(f, " usbvstby"))}
      if self.usbsstby() != 0 { try!(write!(f, " usbsstby"))}
      if self.usbregen() != 0 { try!(write!(f, " usbregen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sopt1cfg(pub u32);
impl Sopt1cfg {
  #[inline]
  pub fn urwe(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_urwe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn uvswe(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_uvswe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn usswe(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_usswe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}
impl ::core::fmt::Display for Sopt1cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sopt1cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.urwe() != 0 { try!(write!(f, " urwe"))}
      if self.uvswe() != 0 { try!(write!(f, " uvswe"))}
      if self.usswe() != 0 { try!(write!(f, " usswe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sopt2(pub u32);
impl Sopt2 {
  #[inline]
  pub fn rtcclkoutsel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_rtcclkoutsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn clkoutsel(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7 // [7:5]
  }
  #[inline]
  pub fn set_clkoutsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn pllfllsel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_pllfllsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn usbsrc(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_usbsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn tpmsrc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
  #[inline]
  pub fn set_tpmsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn uart0src(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x3 // [27:26]
  }
  #[inline]
  pub fn set_uart0src(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 26);
     self.0 |= value << 26;
     self
  }

}
impl ::core::fmt::Display for Sopt2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sopt2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rtcclkoutsel() != 0 { try!(write!(f, " rtcclkoutsel"))}
      if self.clkoutsel() != 0 { try!(write!(f, " clkoutsel=0x{:x}", self.clkoutsel()))}
      if self.pllfllsel() != 0 { try!(write!(f, " pllfllsel"))}
      if self.usbsrc() != 0 { try!(write!(f, " usbsrc"))}
      if self.tpmsrc() != 0 { try!(write!(f, " tpmsrc=0x{:x}", self.tpmsrc()))}
      if self.uart0src() != 0 { try!(write!(f, " uart0src=0x{:x}", self.uart0src()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sopt4(pub u32);
impl Sopt4 {
  #[inline]
  pub fn tpm1ch0src(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x3 // [19:18]
  }
  #[inline]
  pub fn set_tpm1ch0src(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn tpm2ch0src(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
  pub fn set_tpm2ch0src(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn tpm0clksel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_tpm0clksel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn tpm1clksel(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_tpm1clksel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn tpm2clksel(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_tpm2clksel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}
impl ::core::fmt::Display for Sopt4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sopt4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tpm1ch0src() != 0 { try!(write!(f, " tpm1ch0src=0x{:x}", self.tpm1ch0src()))}
      if self.tpm2ch0src() != 0 { try!(write!(f, " tpm2ch0src"))}
      if self.tpm0clksel() != 0 { try!(write!(f, " tpm0clksel"))}
      if self.tpm1clksel() != 0 { try!(write!(f, " tpm1clksel"))}
      if self.tpm2clksel() != 0 { try!(write!(f, " tpm2clksel"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sopt5(pub u32);
impl Sopt5 {
  #[inline]
  pub fn uart0txsrc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_uart0txsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn uart0rxsrc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_uart0rxsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn uart1txsrc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline]
  pub fn set_uart1txsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn uart1rxsrc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_uart1rxsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn uart0ode(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_uart0ode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn uart1ode(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_uart1ode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn uart2ode(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_uart2ode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for Sopt5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sopt5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.uart0txsrc() != 0 { try!(write!(f, " uart0txsrc=0x{:x}", self.uart0txsrc()))}
      if self.uart0rxsrc() != 0 { try!(write!(f, " uart0rxsrc"))}
      if self.uart1txsrc() != 0 { try!(write!(f, " uart1txsrc=0x{:x}", self.uart1txsrc()))}
      if self.uart1rxsrc() != 0 { try!(write!(f, " uart1rxsrc"))}
      if self.uart0ode() != 0 { try!(write!(f, " uart0ode"))}
      if self.uart1ode() != 0 { try!(write!(f, " uart1ode"))}
      if self.uart2ode() != 0 { try!(write!(f, " uart2ode"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sopt7(pub u32);
impl Sopt7 {
  #[inline]
  pub fn adc0trgsel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_adc0trgsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn adc0pretrgsel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_adc0pretrgsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn adc0alttrgen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_adc0alttrgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Sopt7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sopt7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adc0trgsel() != 0 { try!(write!(f, " adc0trgsel=0x{:x}", self.adc0trgsel()))}
      if self.adc0pretrgsel() != 0 { try!(write!(f, " adc0pretrgsel"))}
      if self.adc0alttrgen() != 0 { try!(write!(f, " adc0alttrgen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sdid(pub u32);
impl Sdid {
  #[inline]
  pub fn pinid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_pinid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn dieid(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1f // [11:7]
  }
  #[inline]
  pub fn set_dieid(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn revid(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  #[inline]
  pub fn set_revid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn sramsize(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline]
  pub fn set_sramsize(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn seriesid(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  #[inline]
  pub fn set_seriesid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn subfamid(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline]
  pub fn set_subfamid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn famid(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  #[inline]
  pub fn set_famid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Sdid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sdid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pinid() != 0 { try!(write!(f, " pinid=0x{:x}", self.pinid()))}
      if self.dieid() != 0 { try!(write!(f, " dieid=0x{:x}", self.dieid()))}
      if self.revid() != 0 { try!(write!(f, " revid=0x{:x}", self.revid()))}
      if self.sramsize() != 0 { try!(write!(f, " sramsize=0x{:x}", self.sramsize()))}
      if self.seriesid() != 0 { try!(write!(f, " seriesid=0x{:x}", self.seriesid()))}
      if self.subfamid() != 0 { try!(write!(f, " subfamid=0x{:x}", self.subfamid()))}
      if self.famid() != 0 { try!(write!(f, " famid=0x{:x}", self.famid()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Scgc4(pub u32);
impl Scgc4 {
  #[inline]
  pub fn i2c0(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_i2c0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn i2c1(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_i2c1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn uart0(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_uart0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn uart1(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_uart1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn uart2(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_uart2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn usbotg(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_usbotg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn cmp(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_cmp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline]
  pub fn spi0(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline]
  pub fn set_spi0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline]
  pub fn spi1(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_spi1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

}
impl ::core::fmt::Display for Scgc4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgc4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.i2c0() != 0 { try!(write!(f, " i2c0"))}
      if self.i2c1() != 0 { try!(write!(f, " i2c1"))}
      if self.uart0() != 0 { try!(write!(f, " uart0"))}
      if self.uart1() != 0 { try!(write!(f, " uart1"))}
      if self.uart2() != 0 { try!(write!(f, " uart2"))}
      if self.usbotg() != 0 { try!(write!(f, " usbotg"))}
      if self.cmp() != 0 { try!(write!(f, " cmp"))}
      if self.spi0() != 0 { try!(write!(f, " spi0"))}
      if self.spi1() != 0 { try!(write!(f, " spi1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Scgc5(pub u32);
impl Scgc5 {
  #[inline]
  pub fn lptmr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_lptmr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tsi(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_tsi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn porta(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_porta(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn portb(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_portb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn portc(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_portc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn portd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_portd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn porte(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_porte(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Scgc5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgc5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lptmr() != 0 { try!(write!(f, " lptmr"))}
      if self.tsi() != 0 { try!(write!(f, " tsi"))}
      if self.porta() != 0 { try!(write!(f, " porta"))}
      if self.portb() != 0 { try!(write!(f, " portb"))}
      if self.portc() != 0 { try!(write!(f, " portc"))}
      if self.portd() != 0 { try!(write!(f, " portd"))}
      if self.porte() != 0 { try!(write!(f, " porte"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Scgc6(pub u32);
impl Scgc6 {
  #[inline]
  pub fn ftf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ftf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn dmamux(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_dmamux(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn i2s(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_i2s(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn pit(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_pit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline]
  pub fn tpm0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_tpm0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn tpm1(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_tpm1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn tpm2(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_tpm2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline]
  pub fn adc0(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_adc0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  #[inline]
  pub fn rtc(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  #[inline]
  pub fn set_rtc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

  #[inline]
  pub fn dac0(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
  pub fn set_dac0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Scgc6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgc6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ftf() != 0 { try!(write!(f, " ftf"))}
      if self.dmamux() != 0 { try!(write!(f, " dmamux"))}
      if self.i2s() != 0 { try!(write!(f, " i2s"))}
      if self.pit() != 0 { try!(write!(f, " pit"))}
      if self.tpm0() != 0 { try!(write!(f, " tpm0"))}
      if self.tpm1() != 0 { try!(write!(f, " tpm1"))}
      if self.tpm2() != 0 { try!(write!(f, " tpm2"))}
      if self.adc0() != 0 { try!(write!(f, " adc0"))}
      if self.rtc() != 0 { try!(write!(f, " rtc"))}
      if self.dac0() != 0 { try!(write!(f, " dac0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Scgc7(pub u32);
impl Scgc7 {
  #[inline]
  pub fn dma(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_dma(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Scgc7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgc7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dma() != 0 { try!(write!(f, " dma"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Clkdiv1(pub u32);
impl Clkdiv1 {
  #[inline]
  pub fn outdiv4(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7 // [18:16]
  }
  #[inline]
  pub fn set_outdiv4(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn outdiv1(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
  #[inline]
  pub fn set_outdiv1(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Clkdiv1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clkdiv1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.outdiv4() != 0 { try!(write!(f, " outdiv4=0x{:x}", self.outdiv4()))}
      if self.outdiv1() != 0 { try!(write!(f, " outdiv1=0x{:x}", self.outdiv1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Fcfg1(pub u32);
impl Fcfg1 {
  #[inline]
  pub fn flashdis(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_flashdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn flashdoze(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_flashdoze(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn pfsize(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline]
  pub fn set_pfsize(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Fcfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fcfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flashdis() != 0 { try!(write!(f, " flashdis"))}
      if self.flashdoze() != 0 { try!(write!(f, " flashdoze"))}
      if self.pfsize() != 0 { try!(write!(f, " pfsize=0x{:x}", self.pfsize()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Fcfg2(pub u32);
impl Fcfg2 {
  #[inline]
  pub fn maxaddr1(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
  #[inline]
  pub fn set_maxaddr1(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn maxaddr0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7f // [30:24]
  }
  #[inline]
  pub fn set_maxaddr0(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Fcfg2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fcfg2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maxaddr1() != 0 { try!(write!(f, " maxaddr1=0x{:x}", self.maxaddr1()))}
      if self.maxaddr0() != 0 { try!(write!(f, " maxaddr0=0x{:x}", self.maxaddr0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Uidmh(pub u32);
impl Uidmh {
  #[inline]
  pub fn uid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_uid(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Uidmh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Uidmh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.uid() != 0 { try!(write!(f, " uid=0x{:x}", self.uid()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Uidml(pub u32);
impl Uidml {
  #[inline]
  pub fn uid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_uid(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Uidml {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Uidml {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Uidl(pub u32);
impl Uidl {
  #[inline]
  pub fn uid(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_uid(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Uidl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Uidl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Copc(pub u32);
impl Copc {
  #[inline]
  pub fn copw(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_copw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn copclks(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_copclks(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn copt(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
  #[inline]
  pub fn set_copt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Copc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Copc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.copw() != 0 { try!(write!(f, " copw"))}
      if self.copclks() != 0 { try!(write!(f, " copclks"))}
      if self.copt() != 0 { try!(write!(f, " copt=0x{:x}", self.copt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Srvcop(pub u32);
impl Srvcop {
  #[inline]
  pub fn srvcop(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline]
  pub fn set_srvcop(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Srvcop {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srvcop {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.srvcop() != 0 { try!(write!(f, " srvcop=0x{:x}", self.srvcop()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait En {
   fn en(&self) -> u32;
   fn set_en(&self, value: u32);
}

impl Sim {
   #[inline]
   pub fn en<P: En>(&self, p: &P) -> u32 {
      p.en()
   }
   #[inline]
   pub fn set_en<P: En>(&self, p: &P, value: u32) {
      p.set_en(value)
   }
}

pub trait Src {
   fn src(&self) -> u32;
   fn set_src(&self, value: u32);
}

impl Sim {
   #[inline]
   pub fn src<P: Src>(&self, p: &P) -> u32 {
      p.src()
   }
   #[inline]
   pub fn set_src<P: Src>(&self, p: &P, value: u32) {
      p.set_src(value)
   }
}

pub trait Rst {
   fn rst(&self) -> u32;
   fn set_rst(&self, value: u32);
}

impl Sim {
   #[inline]
   pub fn rst<P: Rst>(&self, p: &P) -> u32 {
      p.rst()
   }
   #[inline]
   pub fn set_rst<P: Rst>(&self, p: &P, value: u32) {
      p.set_rst(value)
   }
}

impl Src for super::uart0::Uart0 {
   #[inline]
   fn src(&self) -> u32 { SIM.sopt2().uart0src() }
   #[inline]
   fn set_src(&self, value: u32) { SIM.with_sopt2(|r| r.set_uart0src(value)); }
}

impl En for super::uart0::Uart0 {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc4().uart0() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart0(value)); }
}

impl En for super::uart::Uart1 {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc4().uart1() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart1(value)); }
}

impl En for super::uart::Uart2 {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc4().uart2() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart2(value)); }
}

impl En for super::port::Porta {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc5().porta() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_porta(value)); }
}

impl En for super::port::Portb {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc5().portb() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portb(value)); }
}

impl En for super::port::Portc {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc5().portc() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portc(value)); }
}

impl En for super::port::Portd {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc5().portd() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portd(value)); }
}

impl En for super::port::Porte {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc5().porte() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_porte(value)); }
}

impl En for super::pit::Pit {
   #[inline]
   fn en(&self) -> u32 { SIM.scgc6().pit() }
   #[inline]
   fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_pit(value)); }
}


