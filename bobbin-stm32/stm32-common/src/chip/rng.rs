#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "RNG", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("RNG"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "CR", offset: 0, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: None, description: Some("control register"), fields: [Field { name: "IE", bit_offset: 3, bit_width: 1, access: None, description: Some("Interrupt enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "RNGEN", bit_offset: 2, bit_width: 1, access: None, description: Some("Random number generator enable"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "SR", offset: 4, size: Some(32), access: None, reset_value: Some(0), reset_mask: None, description: Some("status register"), fields: [Field { name: "SEIS", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Seed error interrupt status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CEIS", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Clock error interrupt status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SECS", bit_offset: 2, bit_width: 1, access: Some(ReadOnly), description: Some("Seed error current status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CECS", bit_offset: 1, bit_width: 1, access: Some(ReadOnly), description: Some("Clock error current status"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DRDY", bit_offset: 0, bit_width: 1, access: Some(ReadOnly), description: Some("Data ready"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "DR", offset: 8, size: Some(32), access: Some(ReadOnly), reset_value: Some(0), reset_mask: None, description: Some("data register"), fields: [Field { name: "RNDATA", bit_offset: 0, bit_width: 32, access: None, description: Some("Random data"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: false, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="RNG Peripheral"]
pub struct RngPeriph(pub usize); 


impl RngPeriph {
#[doc="Get the *const pointer for the CR register."]
   #[inline] pub fn cr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the CR register."]
   #[inline] pub fn cr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the CR register."]
   #[inline] pub fn cr(&self) -> Cr { 
      unsafe {
         Cr(::core::ptr::read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the CR register."]
   #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR register."]
   #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = self.cr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SR register."]
   #[inline] pub fn sr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the SR register."]
   #[inline] pub fn sr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the SR register."]
   #[inline] pub fn sr(&self) -> Sr { 
      unsafe {
         Sr(::core::ptr::read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the SR register."]
   #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let value = f(Sr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SR register."]
   #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let tmp = self.sr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DR register."]
   #[inline] pub fn dr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the DR register."]
   #[inline] pub fn dr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the DR register."]
   #[inline] pub fn dr(&self) -> Dr { 
      unsafe {
         Dr(::core::ptr::read_volatile((self.0 + 0x8) as *const u32))
      }
   }

}

#[doc="control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Interrupt enable"]
   #[inline] pub fn ie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Interrupt enable"]
   #[inline] pub fn set_ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Random number generator enable"]
   #[inline] pub fn rngen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Random number generator enable"]
   #[inline] pub fn set_rngen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
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
      if self.ie() != 0 { try!(write!(f, " ie"))}
      if self.rngen() != 0 { try!(write!(f, " rngen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="Seed error interrupt status"]
   #[inline] pub fn seis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Seed error interrupt status"]
   #[inline] pub fn set_seis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Clock error interrupt status"]
   #[inline] pub fn ceis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Clock error interrupt status"]
   #[inline] pub fn set_ceis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Seed error current status"]
   #[inline] pub fn secs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Seed error current status"]
   #[inline] pub fn set_secs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Clock error current status"]
   #[inline] pub fn cecs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Clock error current status"]
   #[inline] pub fn set_cecs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Data ready"]
   #[inline] pub fn drdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Data ready"]
   #[inline] pub fn set_drdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
      if self.seis() != 0 { try!(write!(f, " seis"))}
      if self.ceis() != 0 { try!(write!(f, " ceis"))}
      if self.secs() != 0 { try!(write!(f, " secs"))}
      if self.cecs() != 0 { try!(write!(f, " cecs"))}
      if self.drdy() != 0 { try!(write!(f, " drdy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="data register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="Random data"]
   #[inline] pub fn rndata(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Random data"]
   #[inline] pub fn set_rndata<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
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
      try!(write!(f, "]"));
      Ok(())
   }
}
