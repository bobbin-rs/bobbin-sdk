//! System Integration Module
#[allow(unused_imports)] use bobbin_common::bits;
pub const SIM: Sim = Sim(0x40047000);

#[doc="System Integration Module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sim(pub u32);
impl Sim {
#[doc="Get the *const pointer for the SOPT1 register."]
  #[inline] pub fn sopt1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the SOPT1 register."]
  #[inline] pub fn sopt1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the SOPT1 register."]
  #[inline] pub fn sopt1(&self) -> Sopt1 { 
     unsafe {
        Sopt1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the SOPT1 register."]
  #[inline] pub fn set_sopt1(&self, value: Sopt1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SOPT1 register."]
  #[inline] pub fn with_sopt1<F: FnOnce(Sopt1) -> Sopt1>(&self, f: F) -> &Self {
     let tmp = self.sopt1();
     self.set_sopt1(f(tmp))
  }

#[doc="Get the *const pointer for the SOPT1CFG register."]
  #[inline] pub fn sopt1cfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the SOPT1CFG register."]
  #[inline] pub fn sopt1cfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the SOPT1CFG register."]
  #[inline] pub fn sopt1cfg(&self) -> Sopt1cfg { 
     unsafe {
        Sopt1cfg(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the SOPT1CFG register."]
  #[inline] pub fn set_sopt1cfg(&self, value: Sopt1cfg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SOPT1CFG register."]
  #[inline] pub fn with_sopt1cfg<F: FnOnce(Sopt1cfg) -> Sopt1cfg>(&self, f: F) -> &Self {
     let tmp = self.sopt1cfg();
     self.set_sopt1cfg(f(tmp))
  }

#[doc="Get the *const pointer for the SOPT2 register."]
  #[inline] pub fn sopt2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1004) as *const u32
  }
#[doc="Get the *mut pointer for the SOPT2 register."]
  #[inline] pub fn sopt2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1004) as *mut u32
  }
#[doc="Read the SOPT2 register."]
  #[inline] pub fn sopt2(&self) -> Sopt2 { 
     unsafe {
        Sopt2(::core::ptr::read_volatile(((self.0 as usize) + 0x1004) as *const u32))
     }
  }
#[doc="Write the SOPT2 register."]
  #[inline] pub fn set_sopt2(&self, value: Sopt2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1004) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SOPT2 register."]
  #[inline] pub fn with_sopt2<F: FnOnce(Sopt2) -> Sopt2>(&self, f: F) -> &Self {
     let tmp = self.sopt2();
     self.set_sopt2(f(tmp))
  }

#[doc="Get the *const pointer for the SOPT4 register."]
  #[inline] pub fn sopt4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x100c) as *const u32
  }
#[doc="Get the *mut pointer for the SOPT4 register."]
  #[inline] pub fn sopt4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x100c) as *mut u32
  }
#[doc="Read the SOPT4 register."]
  #[inline] pub fn sopt4(&self) -> Sopt4 { 
     unsafe {
        Sopt4(::core::ptr::read_volatile(((self.0 as usize) + 0x100c) as *const u32))
     }
  }
#[doc="Write the SOPT4 register."]
  #[inline] pub fn set_sopt4(&self, value: Sopt4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SOPT4 register."]
  #[inline] pub fn with_sopt4<F: FnOnce(Sopt4) -> Sopt4>(&self, f: F) -> &Self {
     let tmp = self.sopt4();
     self.set_sopt4(f(tmp))
  }

#[doc="Get the *const pointer for the SOPT5 register."]
  #[inline] pub fn sopt5_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1010) as *const u32
  }
#[doc="Get the *mut pointer for the SOPT5 register."]
  #[inline] pub fn sopt5_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1010) as *mut u32
  }
#[doc="Read the SOPT5 register."]
  #[inline] pub fn sopt5(&self) -> Sopt5 { 
     unsafe {
        Sopt5(::core::ptr::read_volatile(((self.0 as usize) + 0x1010) as *const u32))
     }
  }
#[doc="Write the SOPT5 register."]
  #[inline] pub fn set_sopt5(&self, value: Sopt5) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1010) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SOPT5 register."]
  #[inline] pub fn with_sopt5<F: FnOnce(Sopt5) -> Sopt5>(&self, f: F) -> &Self {
     let tmp = self.sopt5();
     self.set_sopt5(f(tmp))
  }

#[doc="Get the *const pointer for the SOPT7 register."]
  #[inline] pub fn sopt7_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1018) as *const u32
  }
#[doc="Get the *mut pointer for the SOPT7 register."]
  #[inline] pub fn sopt7_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1018) as *mut u32
  }
#[doc="Read the SOPT7 register."]
  #[inline] pub fn sopt7(&self) -> Sopt7 { 
     unsafe {
        Sopt7(::core::ptr::read_volatile(((self.0 as usize) + 0x1018) as *const u32))
     }
  }
#[doc="Write the SOPT7 register."]
  #[inline] pub fn set_sopt7(&self, value: Sopt7) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1018) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SOPT7 register."]
  #[inline] pub fn with_sopt7<F: FnOnce(Sopt7) -> Sopt7>(&self, f: F) -> &Self {
     let tmp = self.sopt7();
     self.set_sopt7(f(tmp))
  }

#[doc="Get the *const pointer for the SDID register."]
  #[inline] pub fn sdid_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1024) as *const u32
  }
#[doc="Get the *mut pointer for the SDID register."]
  #[inline] pub fn sdid_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1024) as *mut u32
  }
#[doc="Read the SDID register."]
  #[inline] pub fn sdid(&self) -> Sdid { 
     unsafe {
        Sdid(::core::ptr::read_volatile(((self.0 as usize) + 0x1024) as *const u32))
     }
  }

#[doc="Get the *const pointer for the SCGC4 register."]
  #[inline] pub fn scgc4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1034) as *const u32
  }
#[doc="Get the *mut pointer for the SCGC4 register."]
  #[inline] pub fn scgc4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1034) as *mut u32
  }
#[doc="Read the SCGC4 register."]
  #[inline] pub fn scgc4(&self) -> Scgc4 { 
     unsafe {
        Scgc4(::core::ptr::read_volatile(((self.0 as usize) + 0x1034) as *const u32))
     }
  }
#[doc="Write the SCGC4 register."]
  #[inline] pub fn set_scgc4(&self, value: Scgc4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1034) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SCGC4 register."]
  #[inline] pub fn with_scgc4<F: FnOnce(Scgc4) -> Scgc4>(&self, f: F) -> &Self {
     let tmp = self.scgc4();
     self.set_scgc4(f(tmp))
  }

#[doc="Get the *const pointer for the SCGC5 register."]
  #[inline] pub fn scgc5_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1038) as *const u32
  }
#[doc="Get the *mut pointer for the SCGC5 register."]
  #[inline] pub fn scgc5_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1038) as *mut u32
  }
#[doc="Read the SCGC5 register."]
  #[inline] pub fn scgc5(&self) -> Scgc5 { 
     unsafe {
        Scgc5(::core::ptr::read_volatile(((self.0 as usize) + 0x1038) as *const u32))
     }
  }
#[doc="Write the SCGC5 register."]
  #[inline] pub fn set_scgc5(&self, value: Scgc5) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1038) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SCGC5 register."]
  #[inline] pub fn with_scgc5<F: FnOnce(Scgc5) -> Scgc5>(&self, f: F) -> &Self {
     let tmp = self.scgc5();
     self.set_scgc5(f(tmp))
  }

#[doc="Get the *const pointer for the SCGC6 register."]
  #[inline] pub fn scgc6_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x103c) as *const u32
  }
#[doc="Get the *mut pointer for the SCGC6 register."]
  #[inline] pub fn scgc6_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x103c) as *mut u32
  }
#[doc="Read the SCGC6 register."]
  #[inline] pub fn scgc6(&self) -> Scgc6 { 
     unsafe {
        Scgc6(::core::ptr::read_volatile(((self.0 as usize) + 0x103c) as *const u32))
     }
  }
#[doc="Write the SCGC6 register."]
  #[inline] pub fn set_scgc6(&self, value: Scgc6) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x103c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SCGC6 register."]
  #[inline] pub fn with_scgc6<F: FnOnce(Scgc6) -> Scgc6>(&self, f: F) -> &Self {
     let tmp = self.scgc6();
     self.set_scgc6(f(tmp))
  }

#[doc="Get the *const pointer for the SCGC7 register."]
  #[inline] pub fn scgc7_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1040) as *const u32
  }
#[doc="Get the *mut pointer for the SCGC7 register."]
  #[inline] pub fn scgc7_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1040) as *mut u32
  }
#[doc="Read the SCGC7 register."]
  #[inline] pub fn scgc7(&self) -> Scgc7 { 
     unsafe {
        Scgc7(::core::ptr::read_volatile(((self.0 as usize) + 0x1040) as *const u32))
     }
  }
#[doc="Write the SCGC7 register."]
  #[inline] pub fn set_scgc7(&self, value: Scgc7) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1040) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SCGC7 register."]
  #[inline] pub fn with_scgc7<F: FnOnce(Scgc7) -> Scgc7>(&self, f: F) -> &Self {
     let tmp = self.scgc7();
     self.set_scgc7(f(tmp))
  }

#[doc="Get the *const pointer for the CLKDIV1 register."]
  #[inline] pub fn clkdiv1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1044) as *const u32
  }
#[doc="Get the *mut pointer for the CLKDIV1 register."]
  #[inline] pub fn clkdiv1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1044) as *mut u32
  }
#[doc="Read the CLKDIV1 register."]
  #[inline] pub fn clkdiv1(&self) -> Clkdiv1 { 
     unsafe {
        Clkdiv1(::core::ptr::read_volatile(((self.0 as usize) + 0x1044) as *const u32))
     }
  }
#[doc="Write the CLKDIV1 register."]
  #[inline] pub fn set_clkdiv1(&self, value: Clkdiv1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1044) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLKDIV1 register."]
  #[inline] pub fn with_clkdiv1<F: FnOnce(Clkdiv1) -> Clkdiv1>(&self, f: F) -> &Self {
     let tmp = self.clkdiv1();
     self.set_clkdiv1(f(tmp))
  }

#[doc="Get the *const pointer for the FCFG1 register."]
  #[inline] pub fn fcfg1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x104c) as *const u32
  }
#[doc="Get the *mut pointer for the FCFG1 register."]
  #[inline] pub fn fcfg1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x104c) as *mut u32
  }
#[doc="Read the FCFG1 register."]
  #[inline] pub fn fcfg1(&self) -> Fcfg1 { 
     unsafe {
        Fcfg1(::core::ptr::read_volatile(((self.0 as usize) + 0x104c) as *const u32))
     }
  }
#[doc="Write the FCFG1 register."]
  #[inline] pub fn set_fcfg1(&self, value: Fcfg1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x104c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FCFG1 register."]
  #[inline] pub fn with_fcfg1<F: FnOnce(Fcfg1) -> Fcfg1>(&self, f: F) -> &Self {
     let tmp = self.fcfg1();
     self.set_fcfg1(f(tmp))
  }

#[doc="Get the *const pointer for the FCFG2 register."]
  #[inline] pub fn fcfg2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1050) as *const u32
  }
#[doc="Get the *mut pointer for the FCFG2 register."]
  #[inline] pub fn fcfg2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1050) as *mut u32
  }
#[doc="Read the FCFG2 register."]
  #[inline] pub fn fcfg2(&self) -> Fcfg2 { 
     unsafe {
        Fcfg2(::core::ptr::read_volatile(((self.0 as usize) + 0x1050) as *const u32))
     }
  }

#[doc="Get the *const pointer for the UIDMH register."]
  #[inline] pub fn uidmh_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1058) as *const u32
  }
#[doc="Get the *mut pointer for the UIDMH register."]
  #[inline] pub fn uidmh_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1058) as *mut u32
  }
#[doc="Read the UIDMH register."]
  #[inline] pub fn uidmh(&self) -> Uidmh { 
     unsafe {
        Uidmh(::core::ptr::read_volatile(((self.0 as usize) + 0x1058) as *const u32))
     }
  }

#[doc="Get the *const pointer for the UIDML register."]
  #[inline] pub fn uidml_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x105c) as *const u32
  }
#[doc="Get the *mut pointer for the UIDML register."]
  #[inline] pub fn uidml_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x105c) as *mut u32
  }
#[doc="Read the UIDML register."]
  #[inline] pub fn uidml(&self) -> Uidml { 
     unsafe {
        Uidml(::core::ptr::read_volatile(((self.0 as usize) + 0x105c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the UIDL register."]
  #[inline] pub fn uidl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1060) as *const u32
  }
#[doc="Get the *mut pointer for the UIDL register."]
  #[inline] pub fn uidl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1060) as *mut u32
  }
#[doc="Read the UIDL register."]
  #[inline] pub fn uidl(&self) -> Uidl { 
     unsafe {
        Uidl(::core::ptr::read_volatile(((self.0 as usize) + 0x1060) as *const u32))
     }
  }

#[doc="Get the *const pointer for the COPC register."]
  #[inline] pub fn copc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1100) as *const u32
  }
#[doc="Get the *mut pointer for the COPC register."]
  #[inline] pub fn copc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1100) as *mut u32
  }
#[doc="Read the COPC register."]
  #[inline] pub fn copc(&self) -> Copc { 
     unsafe {
        Copc(::core::ptr::read_volatile(((self.0 as usize) + 0x1100) as *const u32))
     }
  }
#[doc="Write the COPC register."]
  #[inline] pub fn set_copc(&self, value: Copc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1100) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the COPC register."]
  #[inline] pub fn with_copc<F: FnOnce(Copc) -> Copc>(&self, f: F) -> &Self {
     let tmp = self.copc();
     self.set_copc(f(tmp))
  }

#[doc="Get the *const pointer for the SRVCOP register."]
  #[inline] pub fn srvcop_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1104) as *const u32
  }
#[doc="Get the *mut pointer for the SRVCOP register."]
  #[inline] pub fn srvcop_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1104) as *mut u32
  }
#[doc="Write the SRVCOP register."]
  #[inline] pub fn set_srvcop(&self, value: Srvcop) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1104) as *mut u32, value.0);
     }
     self
  }

}

#[doc="System Options Register 1"]
#[derive(PartialEq, Eq)]
pub struct Sopt1(pub u32);
impl Sopt1 {
#[doc="32K Oscillator Clock Select"]
  #[inline] pub fn osc32ksel(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
  }
#[doc="32K Oscillator Clock Select"]
  #[inline] pub fn set_osc32ksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="USB voltage regulator in standby mode during VLPR and VLPW modes"]
  #[inline] pub fn usbvstby(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
  }
#[doc="USB voltage regulator in standby mode during VLPR and VLPW modes"]
  #[inline] pub fn set_usbvstby<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
  #[inline] pub fn usbsstby(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
  }
#[doc="USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
  #[inline] pub fn set_usbsstby<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="USB voltage regulator enable"]
  #[inline] pub fn usbregen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
  }
#[doc="USB voltage regulator enable"]
  #[inline] pub fn set_usbregen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="SOPT1 Configuration Register"]
#[derive(PartialEq, Eq)]
pub struct Sopt1cfg(pub u32);
impl Sopt1cfg {
#[doc="USB voltage regulator enable write enable"]
  #[inline] pub fn urwe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
  }
#[doc="USB voltage regulator enable write enable"]
  #[inline] pub fn set_urwe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="USB voltage regulator VLP standby write enable"]
  #[inline] pub fn uvswe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
  }
#[doc="USB voltage regulator VLP standby write enable"]
  #[inline] pub fn set_uvswe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="USB voltage regulator stop standby write enable"]
  #[inline] pub fn usswe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
  }
#[doc="USB voltage regulator stop standby write enable"]
  #[inline] pub fn set_usswe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="System Options Register 2"]
#[derive(PartialEq, Eq)]
pub struct Sopt2(pub u32);
impl Sopt2 {
#[doc="RTC Clock Out Select"]
  #[inline] pub fn rtcclkoutsel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="RTC Clock Out Select"]
  #[inline] pub fn set_rtcclkoutsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="CLKOUT select"]
  #[inline] pub fn clkoutsel(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
  }
#[doc="CLKOUT select"]
  #[inline] pub fn set_clkoutsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="PLL/FLL clock select"]
  #[inline] pub fn pllfllsel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="PLL/FLL clock select"]
  #[inline] pub fn set_pllfllsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="USB clock source select"]
  #[inline] pub fn usbsrc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="USB clock source select"]
  #[inline] pub fn set_usbsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="TPM Clock Source Select"]
  #[inline] pub fn tpmsrc(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
  }
#[doc="TPM Clock Source Select"]
  #[inline] pub fn set_tpmsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="UART0 Clock Source Select"]
  #[inline] pub fn uart0src(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3) as u8) } // [27:26]
  }
#[doc="UART0 Clock Source Select"]
  #[inline] pub fn set_uart0src<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
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
#[doc="System Options Register 4"]
#[derive(PartialEq, Eq)]
pub struct Sopt4(pub u32);
impl Sopt4 {
#[doc="TPM1 channel 0 input capture source select"]
  #[inline] pub fn tpm1ch0src(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
  }
#[doc="TPM1 channel 0 input capture source select"]
  #[inline] pub fn set_tpm1ch0src<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="TPM2 Channel 0 Input Capture Source Select"]
  #[inline] pub fn tpm2ch0src(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="TPM2 Channel 0 Input Capture Source Select"]
  #[inline] pub fn set_tpm2ch0src<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="TPM0 External Clock Pin Select"]
  #[inline] pub fn tpm0clksel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
  }
#[doc="TPM0 External Clock Pin Select"]
  #[inline] pub fn set_tpm0clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="TPM1 External Clock Pin Select"]
  #[inline] pub fn tpm1clksel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
  }
#[doc="TPM1 External Clock Pin Select"]
  #[inline] pub fn set_tpm1clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="TPM2 External Clock Pin Select"]
  #[inline] pub fn tpm2clksel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
  }
#[doc="TPM2 External Clock Pin Select"]
  #[inline] pub fn set_tpm2clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="System Options Register 5"]
#[derive(PartialEq, Eq)]
pub struct Sopt5(pub u32);
impl Sopt5 {
#[doc="UART0 Transmit Data Source Select"]
  #[inline] pub fn uart0txsrc(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
  }
#[doc="UART0 Transmit Data Source Select"]
  #[inline] pub fn set_uart0txsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="UART0 Receive Data Source Select"]
  #[inline] pub fn uart0rxsrc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="UART0 Receive Data Source Select"]
  #[inline] pub fn set_uart0rxsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="UART1 Transmit Data Source Select"]
  #[inline] pub fn uart1txsrc(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="UART1 Transmit Data Source Select"]
  #[inline] pub fn set_uart1txsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="UART1 Receive Data Source Select"]
  #[inline] pub fn uart1rxsrc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="UART1 Receive Data Source Select"]
  #[inline] pub fn set_uart1rxsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="UART0 Open Drain Enable"]
  #[inline] pub fn uart0ode(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="UART0 Open Drain Enable"]
  #[inline] pub fn set_uart0ode<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="UART1 Open Drain Enable"]
  #[inline] pub fn uart1ode(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
  }
#[doc="UART1 Open Drain Enable"]
  #[inline] pub fn set_uart1ode<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="UART2 Open Drain Enable"]
  #[inline] pub fn uart2ode(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="UART2 Open Drain Enable"]
  #[inline] pub fn set_uart2ode<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="System Options Register 7"]
#[derive(PartialEq, Eq)]
pub struct Sopt7(pub u32);
impl Sopt7 {
#[doc="ADC0 Trigger Select"]
  #[inline] pub fn adc0trgsel(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADC0 Trigger Select"]
  #[inline] pub fn set_adc0trgsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="ADC0 Pretrigger Select"]
  #[inline] pub fn adc0pretrgsel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="ADC0 Pretrigger Select"]
  #[inline] pub fn set_adc0pretrgsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADC0 Alternate Trigger Enable"]
  #[inline] pub fn adc0alttrgen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="ADC0 Alternate Trigger Enable"]
  #[inline] pub fn set_adc0alttrgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="System Device Identification Register"]
#[derive(PartialEq, Eq)]
pub struct Sdid(pub u32);
impl Sdid {
#[doc="Pincount Identification"]
  #[inline] pub fn pinid(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="Pincount Identification"]
  #[inline] pub fn set_pinid<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Device Die Number"]
  #[inline] pub fn dieid(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1f) as u8) } // [11:7]
  }
#[doc="Device Die Number"]
  #[inline] pub fn set_dieid<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1f << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Device Revision Number"]
  #[inline] pub fn revid(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
  }
#[doc="Device Revision Number"]
  #[inline] pub fn set_revid<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="System SRAM Size"]
  #[inline] pub fn sramsize(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="System SRAM Size"]
  #[inline] pub fn set_sramsize<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Kinetis Series ID"]
  #[inline] pub fn seriesid(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="Kinetis Series ID"]
  #[inline] pub fn set_seriesid<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Kinetis Sub-Family ID"]
  #[inline] pub fn subfamid(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Kinetis Sub-Family ID"]
  #[inline] pub fn set_subfamid<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Kinetis family ID"]
  #[inline] pub fn famid(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
  }
#[doc="Kinetis family ID"]
  #[inline] pub fn set_famid<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
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
#[doc="System Clock Gating Control Register 4"]
#[derive(PartialEq, Eq)]
pub struct Scgc4(pub u32);
impl Scgc4 {
#[doc="I2C0 Clock Gate Control"]
  #[inline] pub fn i2c0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="I2C0 Clock Gate Control"]
  #[inline] pub fn set_i2c0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="I2C1 Clock Gate Control"]
  #[inline] pub fn i2c1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="I2C1 Clock Gate Control"]
  #[inline] pub fn set_i2c1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="UART0 Clock Gate Control"]
  #[inline] pub fn uart0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="UART0 Clock Gate Control"]
  #[inline] pub fn set_uart0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="UART1 Clock Gate Control"]
  #[inline] pub fn uart1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="UART1 Clock Gate Control"]
  #[inline] pub fn set_uart1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="UART2 Clock Gate Control"]
  #[inline] pub fn uart2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="UART2 Clock Gate Control"]
  #[inline] pub fn set_uart2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="USB Clock Gate Control"]
  #[inline] pub fn usbotg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="USB Clock Gate Control"]
  #[inline] pub fn set_usbotg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Comparator Clock Gate Control"]
  #[inline] pub fn cmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Comparator Clock Gate Control"]
  #[inline] pub fn set_cmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="SPI0 Clock Gate Control"]
  #[inline] pub fn spi0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="SPI0 Clock Gate Control"]
  #[inline] pub fn set_spi0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="SPI1 Clock Gate Control"]
  #[inline] pub fn spi1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="SPI1 Clock Gate Control"]
  #[inline] pub fn set_spi1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="System Clock Gating Control Register 5"]
#[derive(PartialEq, Eq)]
pub struct Scgc5(pub u32);
impl Scgc5 {
#[doc="Low Power Timer Access Control"]
  #[inline] pub fn lptmr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Low Power Timer Access Control"]
  #[inline] pub fn set_lptmr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="TSI Access Control"]
  #[inline] pub fn tsi(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="TSI Access Control"]
  #[inline] pub fn set_tsi<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Port A Clock Gate Control"]
  #[inline] pub fn porta(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Port A Clock Gate Control"]
  #[inline] pub fn set_porta<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Port B Clock Gate Control"]
  #[inline] pub fn portb(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Port B Clock Gate Control"]
  #[inline] pub fn set_portb<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Port C Clock Gate Control"]
  #[inline] pub fn portc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Port C Clock Gate Control"]
  #[inline] pub fn set_portc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Port D Clock Gate Control"]
  #[inline] pub fn portd(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Port D Clock Gate Control"]
  #[inline] pub fn set_portd<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Port E Clock Gate Control"]
  #[inline] pub fn porte(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="Port E Clock Gate Control"]
  #[inline] pub fn set_porte<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="System Clock Gating Control Register 6"]
#[derive(PartialEq, Eq)]
pub struct Scgc6(pub u32);
impl Scgc6 {
#[doc="Flash Memory Clock Gate Control"]
  #[inline] pub fn ftf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Flash Memory Clock Gate Control"]
  #[inline] pub fn set_ftf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="DMA Mux Clock Gate Control"]
  #[inline] pub fn dmamux(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="DMA Mux Clock Gate Control"]
  #[inline] pub fn set_dmamux<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="I2S Clock Gate Control"]
  #[inline] pub fn i2s(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="I2S Clock Gate Control"]
  #[inline] pub fn set_i2s<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="PIT Clock Gate Control"]
  #[inline] pub fn pit(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="PIT Clock Gate Control"]
  #[inline] pub fn set_pit<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="TPM0 Clock Gate Control"]
  #[inline] pub fn tpm0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
  }
#[doc="TPM0 Clock Gate Control"]
  #[inline] pub fn set_tpm0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="TPM1 Clock Gate Control"]
  #[inline] pub fn tpm1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
  }
#[doc="TPM1 Clock Gate Control"]
  #[inline] pub fn set_tpm1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="TPM2 Clock Gate Control"]
  #[inline] pub fn tpm2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
  }
#[doc="TPM2 Clock Gate Control"]
  #[inline] pub fn set_tpm2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="ADC0 Clock Gate Control"]
  #[inline] pub fn adc0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
  }
#[doc="ADC0 Clock Gate Control"]
  #[inline] pub fn set_adc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="RTC Access Control"]
  #[inline] pub fn rtc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
  }
#[doc="RTC Access Control"]
  #[inline] pub fn set_rtc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="DAC0 Clock Gate Control"]
  #[inline] pub fn dac0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
  }
#[doc="DAC0 Clock Gate Control"]
  #[inline] pub fn set_dac0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="System Clock Gating Control Register 7"]
#[derive(PartialEq, Eq)]
pub struct Scgc7(pub u32);
impl Scgc7 {
#[doc="DMA Clock Gate Control"]
  #[inline] pub fn dma(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="DMA Clock Gate Control"]
  #[inline] pub fn set_dma<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
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
#[doc="System Clock Divider Register 1"]
#[derive(PartialEq, Eq)]
pub struct Clkdiv1(pub u32);
impl Clkdiv1 {
#[doc="Clock 4 Output Divider value"]
  #[inline] pub fn outdiv4(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
  }
#[doc="Clock 4 Output Divider value"]
  #[inline] pub fn set_outdiv4<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Clock 1 Output Divider value"]
  #[inline] pub fn outdiv1(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
  }
#[doc="Clock 1 Output Divider value"]
  #[inline] pub fn set_outdiv1<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
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
#[doc="Flash Configuration Register 1"]
#[derive(PartialEq, Eq)]
pub struct Fcfg1(pub u32);
impl Fcfg1 {
#[doc="Flash Disable"]
  #[inline] pub fn flashdis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Flash Disable"]
  #[inline] pub fn set_flashdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Flash Doze"]
  #[inline] pub fn flashdoze(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Flash Doze"]
  #[inline] pub fn set_flashdoze<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Program Flash Size"]
  #[inline] pub fn pfsize(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Program Flash Size"]
  #[inline] pub fn set_pfsize<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
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
#[doc="Flash Configuration Register 2"]
#[derive(PartialEq, Eq)]
pub struct Fcfg2(pub u32);
impl Fcfg2 {
#[doc="This field concatenated with leading zeros plus the value of the MAXADDR1 field indicates the first invalid address of the second program flash block (flash block 1)"]
  #[inline] pub fn maxaddr1(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
  }
#[doc="This field concatenated with leading zeros plus the value of the MAXADDR1 field indicates the first invalid address of the second program flash block (flash block 1)"]
  #[inline] pub fn set_maxaddr1<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Max Address lock"]
  #[inline] pub fn maxaddr0(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7f) as u8) } // [30:24]
  }
#[doc="Max Address lock"]
  #[inline] pub fn set_maxaddr0<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
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
#[doc="Unique Identification Register Mid-High"]
#[derive(PartialEq, Eq)]
pub struct Uidmh(pub u32);
impl Uidmh {
#[doc="Unique Identification"]
  #[inline] pub fn uid(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Unique Identification"]
  #[inline] pub fn set_uid<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
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
#[doc="Unique Identification Register Mid Low"]
#[derive(PartialEq, Eq)]
pub struct Uidml(pub u32);
impl Uidml {
#[doc="Unique Identification"]
  #[inline] pub fn uid(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Unique Identification"]
  #[inline] pub fn set_uid<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
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
#[doc="Unique Identification Register Low"]
#[derive(PartialEq, Eq)]
pub struct Uidl(pub u32);
impl Uidl {
#[doc="Unique Identification"]
  #[inline] pub fn uid(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Unique Identification"]
  #[inline] pub fn set_uid<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
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
#[doc="COP Control Register"]
#[derive(PartialEq, Eq)]
pub struct Copc(pub u32);
impl Copc {
#[doc="COP Windowed Mode"]
  #[inline] pub fn copw(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="COP Windowed Mode"]
  #[inline] pub fn set_copw<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="COP Clock Select"]
  #[inline] pub fn copclks(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="COP Clock Select"]
  #[inline] pub fn set_copclks<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="COP Watchdog Timeout"]
  #[inline] pub fn copt(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
  }
#[doc="COP Watchdog Timeout"]
  #[inline] pub fn set_copt<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
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
#[doc="Service COP"]
#[derive(PartialEq, Eq)]
pub struct Srvcop(pub u32);
impl Srvcop {
#[doc="Service COP Register"]
  #[inline] pub fn srvcop(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Service COP Register"]
  #[inline] pub fn set_srvcop<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
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
   #[inline] pub fn en<P: En>(&self, p: &P) -> u32 {
      p.en()
   }
   #[inline] pub fn set_en<P: En>(&self, p: &P, value: u32) {
      p.set_en(value)
   }
}

pub trait Src {
   fn src(&self) -> u32;
   fn set_src(&self, value: u32);
}

impl Sim {
   #[inline] pub fn src<P: Src>(&self, p: &P) -> u32 {
      p.src()
   }
   #[inline] pub fn set_src<P: Src>(&self, p: &P, value: u32) {
      p.set_src(value)
   }
}

pub trait Rst {
   fn rst(&self) -> u32;
   fn set_rst(&self, value: u32);
}

impl Sim {
   #[inline] pub fn rst<P: Rst>(&self, p: &P) -> u32 {
      p.rst()
   }
   #[inline] pub fn set_rst<P: Rst>(&self, p: &P, value: u32) {
      p.set_rst(value)
   }
}

impl Src for super::tpm::Tpm0 {
   #[inline] fn src(&self) -> u32 { SIM.sopt2().tpmsrc().into() }
   #[inline] fn set_src(&self, value: u32) { SIM.with_sopt2(|r| r.set_tpmsrc(value)); }
}

impl Src for super::tpm::Tpm1 {
   #[inline] fn src(&self) -> u32 { SIM.sopt2().tpmsrc().into() }
   #[inline] fn set_src(&self, value: u32) { SIM.with_sopt2(|r| r.set_tpmsrc(value)); }
}

impl Src for super::tpm::Tpm2 {
   #[inline] fn src(&self) -> u32 { SIM.sopt2().tpmsrc().into() }
   #[inline] fn set_src(&self, value: u32) { SIM.with_sopt2(|r| r.set_tpmsrc(value)); }
}

impl Src for super::uart0::Uart0 {
   #[inline] fn src(&self) -> u32 { SIM.sopt2().uart0src().into() }
   #[inline] fn set_src(&self, value: u32) { SIM.with_sopt2(|r| r.set_uart0src(value)); }
}

impl En for super::uart0::Uart0 {
   #[inline] fn en(&self) -> u32 { SIM.scgc4().uart0().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart0(value)); }
}

impl En for super::uart::Uart1 {
   #[inline] fn en(&self) -> u32 { SIM.scgc4().uart1().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart1(value)); }
}

impl En for super::uart::Uart2 {
   #[inline] fn en(&self) -> u32 { SIM.scgc4().uart2().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart2(value)); }
}

impl En for super::port::Porta {
   #[inline] fn en(&self) -> u32 { SIM.scgc5().porta().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_porta(value)); }
}

impl En for super::port::Portb {
   #[inline] fn en(&self) -> u32 { SIM.scgc5().portb().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portb(value)); }
}

impl En for super::port::Portc {
   #[inline] fn en(&self) -> u32 { SIM.scgc5().portc().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portc(value)); }
}

impl En for super::port::Portd {
   #[inline] fn en(&self) -> u32 { SIM.scgc5().portd().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portd(value)); }
}

impl En for super::port::Porte {
   #[inline] fn en(&self) -> u32 { SIM.scgc5().porte().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_porte(value)); }
}

impl En for super::pit::Pit {
   #[inline] fn en(&self) -> u32 { SIM.scgc6().pit().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_pit(value)); }
}

impl En for super::tpm::Tpm0 {
   #[inline] fn en(&self) -> u32 { SIM.scgc6().tpm0().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_tpm0(value)); }
}

impl En for super::tpm::Tpm1 {
   #[inline] fn en(&self) -> u32 { SIM.scgc6().tpm1().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_tpm1(value)); }
}

impl En for super::tpm::Tpm2 {
   #[inline] fn en(&self) -> u32 { SIM.scgc6().tpm2().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_tpm2(value)); }
}

impl En for super::dma::Dma {
   #[inline] fn en(&self) -> u32 { SIM.scgc7().dma().into() }
   #[inline] fn set_en(&self, value: u32) { SIM.with_scgc7(|r| r.set_dma(value)); }
}


