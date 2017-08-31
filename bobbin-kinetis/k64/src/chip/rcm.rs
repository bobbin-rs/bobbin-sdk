//! Reset Control Module
#[allow(unused_imports)] use bobbin_common::*;

periph!(RCM, Rcm, 0x4007f000);

#[doc="Reset Control Module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcm(pub usize);
impl Rcm {
#[doc="Get the *const pointer for the SRS0 register."]
   #[inline] pub fn srs0_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x0) as *const u8
   }
#[doc="Get the *mut pointer for the SRS0 register."]
   #[inline] pub fn srs0_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x0) as *mut u8
   }
#[doc="Read the SRS0 register."]
   #[inline] pub fn srs0(&self) -> Srs0 { 
      unsafe {
         Srs0(read_volatile((self.0 + 0x0) as *const u8))
      }
   }

#[doc="Get the *const pointer for the SRS1 register."]
   #[inline] pub fn srs1_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x1) as *const u8
   }
#[doc="Get the *mut pointer for the SRS1 register."]
   #[inline] pub fn srs1_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x1) as *mut u8
   }
#[doc="Read the SRS1 register."]
   #[inline] pub fn srs1(&self) -> Srs1 { 
      unsafe {
         Srs1(read_volatile((self.0 + 0x1) as *const u8))
      }
   }

#[doc="Get the *const pointer for the RPFC register."]
   #[inline] pub fn rpfc_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x4) as *const u8
   }
#[doc="Get the *mut pointer for the RPFC register."]
   #[inline] pub fn rpfc_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x4) as *mut u8
   }
#[doc="Read the RPFC register."]
   #[inline] pub fn rpfc(&self) -> Rpfc { 
      unsafe {
         Rpfc(read_volatile((self.0 + 0x4) as *const u8))
      }
   }
#[doc="Write the RPFC register."]
   #[inline] pub fn set_rpfc<F: FnOnce(Rpfc) -> Rpfc>(&self, f: F) -> &Self {
      let value = f(Rpfc(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the RPFC register."]
   #[inline] pub fn with_rpfc<F: FnOnce(Rpfc) -> Rpfc>(&self, f: F) -> &Self {
      let tmp = self.rpfc();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RPFW register."]
   #[inline] pub fn rpfw_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x5) as *const u8
   }
#[doc="Get the *mut pointer for the RPFW register."]
   #[inline] pub fn rpfw_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x5) as *mut u8
   }
#[doc="Read the RPFW register."]
   #[inline] pub fn rpfw(&self) -> Rpfw { 
      unsafe {
         Rpfw(read_volatile((self.0 + 0x5) as *const u8))
      }
   }
#[doc="Write the RPFW register."]
   #[inline] pub fn set_rpfw<F: FnOnce(Rpfw) -> Rpfw>(&self, f: F) -> &Self {
      let value = f(Rpfw(0));
      unsafe {
         write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the RPFW register."]
   #[inline] pub fn with_rpfw<F: FnOnce(Rpfw) -> Rpfw>(&self, f: F) -> &Self {
      let tmp = self.rpfw();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x5) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MR register."]
   #[inline] pub fn mr_ptr(&self) -> *const u8 { 
      ((self.0 as usize) + 0x7) as *const u8
   }
#[doc="Get the *mut pointer for the MR register."]
   #[inline] pub fn mr_mut(&self) -> *mut u8 { 
      ((self.0 as usize) + 0x7) as *mut u8
   }
#[doc="Read the MR register."]
   #[inline] pub fn mr(&self) -> Mr { 
      unsafe {
         Mr(read_volatile((self.0 + 0x7) as *const u8))
      }
   }

}

#[doc="System Reset Status Register 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srs0(pub u8);
impl Srs0 {
#[doc="Low Leakage Wakeup Reset"]
   #[inline] pub fn wakeup(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Low Leakage Wakeup Reset"]
   #[inline] pub fn set_wakeup<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Low-Voltage Detect Reset"]
   #[inline] pub fn lvd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Low-Voltage Detect Reset"]
   #[inline] pub fn set_lvd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Loss-of-Clock Reset"]
   #[inline] pub fn loc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Loss-of-Clock Reset"]
   #[inline] pub fn set_loc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Loss-of-Lock Reset"]
   #[inline] pub fn lol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Loss-of-Lock Reset"]
   #[inline] pub fn set_lol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Watchdog"]
   #[inline] pub fn wdog(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Watchdog"]
   #[inline] pub fn set_wdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="External Reset Pin"]
   #[inline] pub fn pin(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="External Reset Pin"]
   #[inline] pub fn set_pin<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Power-On Reset"]
   #[inline] pub fn por(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Power-On Reset"]
   #[inline] pub fn set_por<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Srs0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srs0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
      if self.lvd() != 0 { try!(write!(f, " lvd"))}
      if self.loc() != 0 { try!(write!(f, " loc"))}
      if self.lol() != 0 { try!(write!(f, " lol"))}
      if self.wdog() != 0 { try!(write!(f, " wdog"))}
      if self.pin() != 0 { try!(write!(f, " pin"))}
      if self.por() != 0 { try!(write!(f, " por"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Reset Status Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srs1(pub u8);
impl Srs1 {
#[doc="JTAG Generated Reset"]
   #[inline] pub fn jtag(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="JTAG Generated Reset"]
   #[inline] pub fn set_jtag<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Core Lockup"]
   #[inline] pub fn lockup(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Core Lockup"]
   #[inline] pub fn set_lockup<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Software"]
   #[inline] pub fn sw(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Software"]
   #[inline] pub fn set_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="MDM-AP System Reset Request"]
   #[inline] pub fn mdm_ap(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="MDM-AP System Reset Request"]
   #[inline] pub fn set_mdm_ap<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="EzPort Reset"]
   #[inline] pub fn ezpt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="EzPort Reset"]
   #[inline] pub fn set_ezpt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Stop Mode Acknowledge Error Reset"]
   #[inline] pub fn sackerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Stop Mode Acknowledge Error Reset"]
   #[inline] pub fn set_sackerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Srs1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srs1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jtag() != 0 { try!(write!(f, " jtag"))}
      if self.lockup() != 0 { try!(write!(f, " lockup"))}
      if self.sw() != 0 { try!(write!(f, " sw"))}
      if self.mdm_ap() != 0 { try!(write!(f, " mdm_ap"))}
      if self.ezpt() != 0 { try!(write!(f, " ezpt"))}
      if self.sackerr() != 0 { try!(write!(f, " sackerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Reset Pin Filter Control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rpfc(pub u8);
impl Rpfc {
#[doc="Reset Pin Filter Select in Run and Wait Modes"]
   #[inline] pub fn rstfltsrw(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Reset Pin Filter Select in Run and Wait Modes"]
   #[inline] pub fn set_rstfltsrw<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Reset Pin Filter Select in Stop Mode"]
   #[inline] pub fn rstfltss(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Reset Pin Filter Select in Stop Mode"]
   #[inline] pub fn set_rstfltss<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

}
impl ::core::fmt::Display for Rpfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rpfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rstfltsrw() != 0 { try!(write!(f, " rstfltsrw=0x{:x}", self.rstfltsrw()))}
      if self.rstfltss() != 0 { try!(write!(f, " rstfltss"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Reset Pin Filter Width register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rpfw(pub u8);
impl Rpfw {
#[doc="Reset Pin Filter Bus Clock Select"]
   #[inline] pub fn rstfltsel(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Reset Pin Filter Bus Clock Select"]
   #[inline] pub fn set_rstfltsel<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rpfw {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rpfw {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rstfltsel() != 0 { try!(write!(f, " rstfltsel=0x{:x}", self.rstfltsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Mode Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mr(pub u8);
impl Mr {
#[doc="EZP_MS_B pin state"]
   #[inline] pub fn ezp_ms(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="EZP_MS_B pin state"]
   #[inline] pub fn set_ezp_ms<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Mr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ezp_ms() != 0 { try!(write!(f, " ezp_ms"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

