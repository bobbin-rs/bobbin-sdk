//! System Mode Controller
#[allow(unused_imports)] use bobbin_common::*;

periph!(SmcPeriph, SMC, Smc, 0x4007e000);

#[doc="System Mode Controller"]
pub trait SmcPeriph : Base {
#[doc="Get the *const pointer for the VERID register."]
   #[inline] fn verid_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the VERID register."]
   #[inline] fn verid_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the VERID register."]
   #[inline] fn verid(&self) -> Verid { 
      unsafe {
         Verid(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }

#[doc="Get the *const pointer for the PARAM register."]
   #[inline] fn param_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the PARAM register."]
   #[inline] fn param_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the PARAM register."]
   #[inline] fn param(&self) -> Param { 
      unsafe {
         Param(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }

#[doc="Get the *const pointer for the PMPROT register."]
   #[inline] fn pmprot_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the PMPROT register."]
   #[inline] fn pmprot_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the PMPROT register."]
   #[inline] fn pmprot(&self) -> Pmprot { 
      unsafe {
         Pmprot(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }
#[doc="Write the PMPROT register."]
   #[inline] fn set_pmprot<F: FnOnce(Pmprot) -> Pmprot>(&self, f: F) -> &Self {
      let value = f(Pmprot(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PMPROT register."]
   #[inline] fn with_pmprot<F: FnOnce(Pmprot) -> Pmprot>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pmprot(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PMCTRL register."]
   #[inline] fn pmctrl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the PMCTRL register."]
   #[inline] fn pmctrl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the PMCTRL register."]
   #[inline] fn pmctrl(&self) -> Pmctrl { 
      unsafe {
         Pmctrl(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      }
   }
#[doc="Write the PMCTRL register."]
   #[inline] fn set_pmctrl<F: FnOnce(Pmctrl) -> Pmctrl>(&self, f: F) -> &Self {
      let value = f(Pmctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PMCTRL register."]
   #[inline] fn with_pmctrl<F: FnOnce(Pmctrl) -> Pmctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pmctrl(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STOPCTRL register."]
   #[inline] fn stopctrl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the STOPCTRL register."]
   #[inline] fn stopctrl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the STOPCTRL register."]
   #[inline] fn stopctrl(&self) -> Stopctrl { 
      unsafe {
         Stopctrl(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      }
   }
#[doc="Write the STOPCTRL register."]
   #[inline] fn set_stopctrl<F: FnOnce(Stopctrl) -> Stopctrl>(&self, f: F) -> &Self {
      let value = f(Stopctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the STOPCTRL register."]
   #[inline] fn with_stopctrl<F: FnOnce(Stopctrl) -> Stopctrl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Stopctrl(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PMSTAT register."]
   #[inline] fn pmstat_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Get the *mut pointer for the PMSTAT register."]
   #[inline] fn pmstat_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Read the PMSTAT register."]
   #[inline] fn pmstat(&self) -> Pmstat { 
      unsafe {
         Pmstat(::core::ptr::read_volatile((self.base() + 0x14) as *const u32))
      }
   }

}

#[doc="SMC Version ID Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
#[doc="Feature Specification Number"]
   #[inline] pub fn feature(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Feature Specification Number"]
   #[inline] pub fn set_feature<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Minor Version Number"]
   #[inline] pub fn minor(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Minor Version Number"]
   #[inline] pub fn set_minor<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Major Version Number"]
   #[inline] pub fn major(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Major Version Number"]
   #[inline] pub fn set_major<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
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
#[doc="SMC Parameter Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
#[doc="Existence of HSRUN feature"]
   #[inline] pub fn ehsrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Existence of HSRUN feature"]
   #[inline] pub fn set_ehsrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Existence of LLS feature"]
   #[inline] pub fn ells(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Existence of LLS feature"]
   #[inline] pub fn set_ells<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Existence of LLS2 feature"]
   #[inline] pub fn ells2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Existence of LLS2 feature"]
   #[inline] pub fn set_ells2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Existence of VLLS0 feature"]
   #[inline] pub fn evlls0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Existence of VLLS0 feature"]
   #[inline] pub fn set_evlls0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
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
      if self.ehsrun() != 0 { try!(write!(f, " ehsrun"))}
      if self.ells() != 0 { try!(write!(f, " ells"))}
      if self.ells2() != 0 { try!(write!(f, " ells2"))}
      if self.evlls0() != 0 { try!(write!(f, " evlls0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Power Mode Protection register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pmprot(pub u32);
impl Pmprot {
#[doc="Allow Very-Low-Power Modes"]
   #[inline] pub fn avlp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Allow Very-Low-Power Modes"]
   #[inline] pub fn set_avlp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Allow High Speed Run mode"]
   #[inline] pub fn ahsrun(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Allow High Speed Run mode"]
   #[inline] pub fn set_ahsrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Pmprot {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pmprot {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.avlp() != 0 { try!(write!(f, " avlp"))}
      if self.ahsrun() != 0 { try!(write!(f, " ahsrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Power Mode Control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pmctrl(pub u32);
impl Pmctrl {
#[doc="Stop Mode Control"]
   #[inline] pub fn stopm(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Stop Mode Control"]
   #[inline] pub fn set_stopm<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Very Low Power Stop Aborted"]
   #[inline] pub fn vlpsa(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Very Low Power Stop Aborted"]
   #[inline] pub fn set_vlpsa<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Run Mode Control"]
   #[inline] pub fn runm(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="Run Mode Control"]
   #[inline] pub fn set_runm<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Pmctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pmctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stopm() != 0 { try!(write!(f, " stopm=0x{:x}", self.stopm()))}
      if self.vlpsa() != 0 { try!(write!(f, " vlpsa"))}
      if self.runm() != 0 { try!(write!(f, " runm=0x{:x}", self.runm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Stop Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stopctrl(pub u32);
impl Stopctrl {
#[doc="Stop Option"]
   #[inline] pub fn stopo(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Stop Option"]
   #[inline] pub fn set_stopo<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Stopctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stopctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stopo() != 0 { try!(write!(f, " stopo=0x{:x}", self.stopo()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Power Mode Status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pmstat(pub u32);
impl Pmstat {
#[doc="Power Mode Status"]
   #[inline] pub fn pmstat(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Power Mode Status"]
   #[inline] pub fn set_pmstat<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pmstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pmstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pmstat() != 0 { try!(write!(f, " pmstat=0x{:x}", self.pmstat()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

