#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "PIT", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("PIT"), name: "PIT", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "MCR", offset: 0, size: Some(32), access: Some(ReadWrite), reset_value: Some(6), reset_mask: Some(4294967295), description: Some("PIT Module Control Register"), fields: [Field { name: "FRZ", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Freeze"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Timers continue to run in Debug mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Timers are stopped in Debug mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MDIS", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Module Disable - (PIT section)"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Clock for standard PIT timers is enabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Clock for standard PIT timers is disabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "LDVAL", offset: 256, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Timer Load Value Register"), fields: [Field { name: "TSV", bit_offset: 0, bit_width: 32, access: Some(ReadWrite), description: Some("Timer Start Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(4), dim_increment: Some(16), dim_index: Some("0,1,2,3") }, Register { name: "CVAL", offset: 260, size: Some(32), access: Some(ReadOnly), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Current Timer Value Register"), fields: [Field { name: "TVL", bit_offset: 0, bit_width: 32, access: Some(ReadOnly), description: Some("Current Timer Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(4), dim_increment: Some(16), dim_index: Some("0,1,2,3") }, Register { name: "TCTRL", offset: 264, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Timer Control Register"), fields: [Field { name: "TEN", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Timer n is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Timer n is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TIE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Interrupt requests from Timer n are disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Interrupt will be requested whenever TIF is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CHN", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Chain Mode"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Timer is not chained.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(4), dim_increment: Some(16), dim_index: Some("0,1,2,3") }, Register { name: "TFLG", offset: 268, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Timer Flag Register"), fields: [Field { name: "TIF", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Interrupt Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Timeout has not yet occurred.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Timeout has occurred.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(4), dim_increment: Some(16), dim_index: Some("0,1,2,3") }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: true, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PIT Peripheral"]
pub struct PitPeriph(pub usize); 


impl PitPeriph {
#[doc="Get the *const pointer for the MCR register."]
   #[inline] pub fn mcr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the MCR register."]
   #[inline] pub fn mcr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the MCR register."]
   #[inline] pub fn mcr(&self) -> Mcr { 
      unsafe {
         Mcr(::core::ptr::read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the MCR register."]
   #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
      let value = f(Mcr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MCR register."]
   #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
      let tmp = self.mcr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the LDVAL register."]
   #[inline] pub fn ldval_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x100 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the LDVAL register."]
   #[inline] pub fn ldval_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x100 + (index << 4)) as *mut u32
   }
#[doc="Read the LDVAL register."]
   #[inline] pub fn ldval<I: Into<bits::R4>>(&self, index: I) -> Ldval { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Ldval(::core::ptr::read_volatile((self.0 + 0x100 + (index << 4)) as *const u32))
      }
   }
#[doc="Write the LDVAL register."]
   #[inline] pub fn set_ldval<I: Into<bits::R4>, F: FnOnce(Ldval) -> Ldval>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Ldval(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x100 + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LDVAL register."]
   #[inline] pub fn with_ldval<I: Into<bits::R4> + Copy, F: FnOnce(Ldval) -> Ldval>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.ldval(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x100 + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CVAL register."]
   #[inline] pub fn cval_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x104 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the CVAL register."]
   #[inline] pub fn cval_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x104 + (index << 4)) as *mut u32
   }
#[doc="Read the CVAL register."]
   #[inline] pub fn cval<I: Into<bits::R4>>(&self, index: I) -> Cval { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Cval(::core::ptr::read_volatile((self.0 + 0x104 + (index << 4)) as *const u32))
      }
   }

#[doc="Get the *const pointer for the TCTRL register."]
   #[inline] pub fn tctrl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x108 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the TCTRL register."]
   #[inline] pub fn tctrl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x108 + (index << 4)) as *mut u32
   }
#[doc="Read the TCTRL register."]
   #[inline] pub fn tctrl<I: Into<bits::R4>>(&self, index: I) -> Tctrl { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Tctrl(::core::ptr::read_volatile((self.0 + 0x108 + (index << 4)) as *const u32))
      }
   }
#[doc="Write the TCTRL register."]
   #[inline] pub fn set_tctrl<I: Into<bits::R4>, F: FnOnce(Tctrl) -> Tctrl>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Tctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x108 + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCTRL register."]
   #[inline] pub fn with_tctrl<I: Into<bits::R4> + Copy, F: FnOnce(Tctrl) -> Tctrl>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tctrl(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x108 + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TFLG register."]
   #[inline] pub fn tflg_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10c + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the TFLG register."]
   #[inline] pub fn tflg_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10c + (index << 4)) as *mut u32
   }
#[doc="Read the TFLG register."]
   #[inline] pub fn tflg<I: Into<bits::R4>>(&self, index: I) -> Tflg { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Tflg(::core::ptr::read_volatile((self.0 + 0x10c + (index << 4)) as *const u32))
      }
   }
#[doc="Write the TFLG register."]
   #[inline] pub fn set_tflg<I: Into<bits::R4>, F: FnOnce(Tflg) -> Tflg>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Tflg(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10c + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TFLG register."]
   #[inline] pub fn with_tflg<I: Into<bits::R4> + Copy, F: FnOnce(Tflg) -> Tflg>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.tflg(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10c + (index << 4)) as *mut u32, value.0);
      }
      self
   }

}

#[doc="PIT Module Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
#[doc="Freeze"]
   #[inline] pub fn frz(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Freeze"]
   #[inline] pub fn set_frz<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Module Disable - (PIT section)"]
   #[inline] pub fn mdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Module Disable - (PIT section)"]
   #[inline] pub fn set_mdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.frz() != 0 { try!(write!(f, " frz"))}
      if self.mdis() != 0 { try!(write!(f, " mdis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Timer Load Value Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ldval(pub u32);
impl Ldval {
#[doc="Timer Start Value"]
   #[inline] pub fn tsv(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Timer Start Value"]
   #[inline] pub fn set_tsv<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ldval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ldval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Current Timer Value Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cval(pub u32);
impl Cval {
#[doc="Current Timer Value"]
   #[inline] pub fn tvl(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Current Timer Value"]
   #[inline] pub fn set_tvl<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Timer Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tctrl(pub u32);
impl Tctrl {
#[doc="Timer Enable"]
   #[inline] pub fn ten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Timer Enable"]
   #[inline] pub fn set_ten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Timer Interrupt Enable"]
   #[inline] pub fn tie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Timer Interrupt Enable"]
   #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Chain Mode"]
   #[inline] pub fn chn(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Chain Mode"]
   #[inline] pub fn set_chn<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

}
impl ::core::fmt::Display for Tctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ten() != 0 { try!(write!(f, " ten"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.chn() != 0 { try!(write!(f, " chn"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Timer Flag Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tflg(pub u32);
impl Tflg {
#[doc="Timer Interrupt Flag"]
   #[inline] pub fn tif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Timer Interrupt Flag"]
   #[inline] pub fn set_tif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tflg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tflg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tif() != 0 { try!(write!(f, " tif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct PitCh { pub periph: PitPeriph, pub index: usize }
