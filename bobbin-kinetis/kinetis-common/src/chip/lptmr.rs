#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "LPTMR", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: None, name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "CSR", offset: 0, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Low Power Timer Control Status Register"), fields: [Field { name: "TEN", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("LPTMR is disabled and internal logic is reset.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("LPTMR is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TMS", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Mode Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Time Counter mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Pulse Counter mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TFC", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Free-Running Counter"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("CNR is reset whenever TCF is set.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("CNR is reset on overflow.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TPP", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Pin Polarity"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Pulse Counter input source is active-high, and the CNR will increment on the rising-edge.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Pulse Counter input source is active-low, and the CNR will increment on the falling-edge.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TPS", bit_offset: 4, bit_width: 2, access: Some(ReadWrite), description: Some("Timer Pin Select"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("Pulse counter input 0 is selected.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Pulse counter input 1 is selected.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Pulse counter input 2 is selected.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Pulse counter input 3 is selected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TIE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Interrupt Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Timer interrupt disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Timer interrupt enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TCF", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Timer Compare Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("The value of CNR is not equal to CMR and increments.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The value of CNR is equal to CMR and increments.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PSR", offset: 4, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Low Power Timer Prescale Register"), fields: [Field { name: "PCS", bit_offset: 0, bit_width: 2, access: Some(ReadWrite), description: Some("Prescaler Clock Select"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("Prescaler/glitch filter clock 0 selected.") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Prescaler/glitch filter clock 1 selected.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Prescaler/glitch filter clock 2 selected.") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Prescaler/glitch filter clock 3 selected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PBYP", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Prescaler Bypass"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Prescaler/glitch filter is enabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Prescaler/glitch filter is bypassed.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PRESCALE", bit_offset: 3, bit_width: 4, access: Some(ReadWrite), description: Some("Prescale Value"), enumerated_values: [EnumeratedValue { value: "#0000", name: Some("0000"), description: Some("Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration.") }, EnumeratedValue { value: "#0001", name: Some("0001"), description: Some("Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges.") }, EnumeratedValue { value: "#0010", name: Some("0010"), description: Some("Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges.") }, EnumeratedValue { value: "#0011", name: Some("0011"), description: Some("Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges.") }, EnumeratedValue { value: "#0100", name: Some("0100"), description: Some("Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges.") }, EnumeratedValue { value: "#0101", name: Some("0101"), description: Some("Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges.") }, EnumeratedValue { value: "#0110", name: Some("0110"), description: Some("Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges.") }, EnumeratedValue { value: "#0111", name: Some("0111"), description: Some("Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges.") }, EnumeratedValue { value: "#1000", name: Some("1000"), description: Some("Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges.") }, EnumeratedValue { value: "#1001", name: Some("1001"), description: Some("Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges.") }, EnumeratedValue { value: "#1010", name: Some("1010"), description: Some("Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges.") }, EnumeratedValue { value: "#1011", name: Some("1011"), description: Some("Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges.") }, EnumeratedValue { value: "#1100", name: Some("1100"), description: Some("Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges.") }, EnumeratedValue { value: "#1101", name: Some("1101"), description: Some("Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges.") }, EnumeratedValue { value: "#1110", name: Some("1110"), description: Some("Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges.") }, EnumeratedValue { value: "#1111", name: Some("1111"), description: Some("Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CMR", offset: 8, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Low Power Timer Compare Register"), fields: [Field { name: "COMPARE", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Compare Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "CNR", offset: 12, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Low Power Timer Counter Register"), fields: [Field { name: "COUNTER", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Counter Value"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: false, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPTMR Peripheral"]
pub struct LptmrPeriph(pub usize); 


impl LptmrPeriph {
#[doc="Get the *const pointer for the CSR register."]
   #[inline] pub fn csr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the CSR register."]
   #[inline] pub fn csr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the CSR register."]
   #[inline] pub fn csr(&self) -> Csr { 
      unsafe {
         Csr(::core::ptr::read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the CSR register."]
   #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
      let value = f(Csr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CSR register."]
   #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
      let tmp = self.csr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PSR register."]
   #[inline] pub fn psr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the PSR register."]
   #[inline] pub fn psr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the PSR register."]
   #[inline] pub fn psr(&self) -> Psr { 
      unsafe {
         Psr(::core::ptr::read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the PSR register."]
   #[inline] pub fn set_psr<F: FnOnce(Psr) -> Psr>(&self, f: F) -> &Self {
      let value = f(Psr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PSR register."]
   #[inline] pub fn with_psr<F: FnOnce(Psr) -> Psr>(&self, f: F) -> &Self {
      let tmp = self.psr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CMR register."]
   #[inline] pub fn cmr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the CMR register."]
   #[inline] pub fn cmr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the CMR register."]
   #[inline] pub fn cmr(&self) -> Cmr { 
      unsafe {
         Cmr(::core::ptr::read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the CMR register."]
   #[inline] pub fn set_cmr<F: FnOnce(Cmr) -> Cmr>(&self, f: F) -> &Self {
      let value = f(Cmr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CMR register."]
   #[inline] pub fn with_cmr<F: FnOnce(Cmr) -> Cmr>(&self, f: F) -> &Self {
      let tmp = self.cmr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CNR register."]
   #[inline] pub fn cnr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }
#[doc="Get the *mut pointer for the CNR register."]
   #[inline] pub fn cnr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }
#[doc="Read the CNR register."]
   #[inline] pub fn cnr(&self) -> Cnr { 
      unsafe {
         Cnr(::core::ptr::read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the CNR register."]
   #[inline] pub fn set_cnr<F: FnOnce(Cnr) -> Cnr>(&self, f: F) -> &Self {
      let value = f(Cnr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CNR register."]
   #[inline] pub fn with_cnr<F: FnOnce(Cnr) -> Cnr>(&self, f: F) -> &Self {
      let tmp = self.cnr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Low Power Timer Control Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
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

#[doc="Timer Mode Select"]
   #[inline] pub fn tms(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Timer Mode Select"]
   #[inline] pub fn set_tms<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Timer Free-Running Counter"]
   #[inline] pub fn tfc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Timer Free-Running Counter"]
   #[inline] pub fn set_tfc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Timer Pin Polarity"]
   #[inline] pub fn tpp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Timer Pin Polarity"]
   #[inline] pub fn set_tpp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Timer Pin Select"]
   #[inline] pub fn tps(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Timer Pin Select"]
   #[inline] pub fn set_tps<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Timer Interrupt Enable"]
   #[inline] pub fn tie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Timer Interrupt Enable"]
   #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Timer Compare Flag"]
   #[inline] pub fn tcf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Timer Compare Flag"]
   #[inline] pub fn set_tcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ten() != 0 { try!(write!(f, " ten"))}
      if self.tms() != 0 { try!(write!(f, " tms"))}
      if self.tfc() != 0 { try!(write!(f, " tfc"))}
      if self.tpp() != 0 { try!(write!(f, " tpp"))}
      if self.tps() != 0 { try!(write!(f, " tps=0x{:x}", self.tps()))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.tcf() != 0 { try!(write!(f, " tcf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Low Power Timer Prescale Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Psr(pub u32);
impl Psr {
#[doc="Prescaler Clock Select"]
   #[inline] pub fn pcs(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Prescaler Clock Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Prescaler Bypass"]
   #[inline] pub fn pbyp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Prescaler Bypass"]
   #[inline] pub fn set_pbyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Prescale Value"]
   #[inline] pub fn prescale(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0xf) as u8) } // [6:3]
   }
#[doc="Prescale Value"]
   #[inline] pub fn set_prescale<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Psr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Psr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.pbyp() != 0 { try!(write!(f, " pbyp"))}
      if self.prescale() != 0 { try!(write!(f, " prescale=0x{:x}", self.prescale()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Low Power Timer Compare Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cmr(pub u32);
impl Cmr {
#[doc="Compare Value"]
   #[inline] pub fn compare(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Compare Value"]
   #[inline] pub fn set_compare<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cmr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.compare() != 0 { try!(write!(f, " compare=0x{:x}", self.compare()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Low Power Timer Counter Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cnr(pub u32);
impl Cnr {
#[doc="Counter Value"]
   #[inline] pub fn counter(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Counter Value"]
   #[inline] pub fn set_counter<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cnr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.counter() != 0 { try!(write!(f, " counter=0x{:x}", self.counter()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
