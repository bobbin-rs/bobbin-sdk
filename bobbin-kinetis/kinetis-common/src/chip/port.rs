#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "PORT", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("PORT"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "PCR", offset: 0, size: Some(32), access: Some(ReadWrite), reset_value: Some(1858), reset_mask: Some(4294967295), description: Some("Pin Control Register n"), fields: [Field { name: "PS", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Pull Select"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PE", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Pull Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Internal pullup or pulldown resistor is not enabled on the corresponding pin.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SRE", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Slew Rate Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Fast slew rate is configured on the corresponding pin, if the pin is configured as a digital output.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Slow slew rate is configured on the corresponding pin, if the pin is configured as a digital output.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "PFE", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Passive Filter Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Passive input filter is disabled on the corresponding pin.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Passive input filter is enabled on the corresponding pin, if the pin is configured as a digital input. Refer to the device data sheet for filter characteristics.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ODE", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Open Drain Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Open drain output is disabled on the corresponding pin.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Open drain output is enabled on the corresponding pin, if the pin is configured as a digital output.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DSE", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Drive Strength Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Low drive strength is configured on the corresponding pin, if pin is configured as a digital output.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("High drive strength is configured on the corresponding pin, if pin is configured as a digital output.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "MUX", bit_offset: 8, bit_width: 3, access: Some(ReadWrite), description: Some("Pin Mux Control"), enumerated_values: [EnumeratedValue { value: "#000", name: Some("000"), description: Some("Pin disabled (analog).") }, EnumeratedValue { value: "#001", name: Some("001"), description: Some("Alternative 1 (GPIO).") }, EnumeratedValue { value: "#010", name: Some("010"), description: Some("Alternative 2 (chip-specific).") }, EnumeratedValue { value: "#011", name: Some("011"), description: Some("Alternative 3 (chip-specific).") }, EnumeratedValue { value: "#100", name: Some("100"), description: Some("Alternative 4 (chip-specific).") }, EnumeratedValue { value: "#101", name: Some("101"), description: Some("Alternative 5 (chip-specific).") }, EnumeratedValue { value: "#110", name: Some("110"), description: Some("Alternative 6 (chip-specific).") }, EnumeratedValue { value: "#111", name: Some("111"), description: Some("Alternative 7 (chip-specific).") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LK", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("Lock Register"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Pin Control Register fields [15:0] are not locked.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Pin Control Register fields [15:0] are locked and cannot be updated until the next system reset.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IRQC", bit_offset: 16, bit_width: 4, access: Some(ReadWrite), description: Some("Interrupt Configuration"), enumerated_values: [EnumeratedValue { value: "#0000", name: Some("0000"), description: Some("Interrupt/DMA request disabled.") }, EnumeratedValue { value: "#0001", name: Some("0001"), description: Some("DMA request on rising edge.") }, EnumeratedValue { value: "#0010", name: Some("0010"), description: Some("DMA request on falling edge.") }, EnumeratedValue { value: "#0011", name: Some("0011"), description: Some("DMA request on either edge.") }, EnumeratedValue { value: "#1000", name: Some("1000"), description: Some("Interrupt when logic 0.") }, EnumeratedValue { value: "#1001", name: Some("1001"), description: Some("Interrupt on rising-edge.") }, EnumeratedValue { value: "#1010", name: Some("1010"), description: Some("Interrupt on falling-edge.") }, EnumeratedValue { value: "#1011", name: Some("1011"), description: Some("Interrupt on either edge.") }, EnumeratedValue { value: "#1100", name: Some("1100"), description: Some("Interrupt when logic 1.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ISF", bit_offset: 24, bit_width: 1, access: Some(ReadWrite), description: Some("Interrupt Status Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Configured interrupt is not detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(32), dim_increment: Some(4), dim_index: None }, Register { name: "GPCLR", offset: 128, size: Some(32), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Global Pin Control Low Register"), fields: [Field { name: "GPWD", bit_offset: 0, bit_width: 16, access: Some(WriteOnly), description: Some("Global Pin Write Data"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "GPWE", bit_offset: 16, bit_width: 16, access: Some(WriteOnly), description: Some("Global Pin Write Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Corresponding Pin Control Register is not updated with the value in GPWD.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Corresponding Pin Control Register is updated with the value in GPWD.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "GPCHR", offset: 132, size: Some(32), access: Some(WriteOnly), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Global Pin Control High Register"), fields: [Field { name: "GPWD", bit_offset: 0, bit_width: 16, access: Some(WriteOnly), description: Some("Global Pin Write Data"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "GPWE", bit_offset: 16, bit_width: 16, access: Some(WriteOnly), description: Some("Global Pin Write Enable"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Corresponding Pin Control Register is not updated with the value in GPWD.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Corresponding Pin Control Register is updated with the value in GPWD.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "ISFR", offset: 160, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Interrupt Status Flag Register"), fields: [Field { name: "ISF", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Interrupt Status Flag"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Configured interrupt is not detected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared.") }], links: [], dim: Some(32), dim_increment: Some(1), dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: true, has_channels: false, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PORT Peripheral"]
pub struct PortPeriph(pub usize); 


impl PortPeriph {
#[doc="Get the *const pointer for the PCR register."]
   #[inline] pub fn pcr_ptr<I: Into<bits::R32>>(&self, index: I) -> *const u32 { 
      let index: bits::R32 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x0 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the PCR register."]
   #[inline] pub fn pcr_mut<I: Into<bits::R32>>(&self, index: I) -> *mut u32 { 
      let index: bits::R32 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x0 + (index << 2)) as *mut u32
   }
#[doc="Read the PCR register."]
   #[inline] pub fn pcr<I: Into<bits::R32>>(&self, index: I) -> Pcr { 
      let index: bits::R32 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Pcr(::core::ptr::read_volatile((self.0 + 0x0 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the PCR register."]
   #[inline] pub fn set_pcr<I: Into<bits::R32>, F: FnOnce(Pcr) -> Pcr>(&self, index: I, f: F) -> &Self {
      let index: bits::R32 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Pcr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCR register."]
   #[inline] pub fn with_pcr<I: Into<bits::R32> + Copy, F: FnOnce(Pcr) -> Pcr>(&self, index: I, f: F) -> &Self {
      let index: bits::R32 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.pcr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the GPCLR register."]
   #[inline] pub fn gpclr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x80) as *const u32
   }
#[doc="Get the *mut pointer for the GPCLR register."]
   #[inline] pub fn gpclr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x80) as *mut u32
   }
#[doc="Write the GPCLR register."]
   #[inline] pub fn set_gpclr<F: FnOnce(Gpclr) -> Gpclr>(&self, f: F) -> &Self {
      let value = f(Gpclr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x80) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the GPCHR register."]
   #[inline] pub fn gpchr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x84) as *const u32
   }
#[doc="Get the *mut pointer for the GPCHR register."]
   #[inline] pub fn gpchr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x84) as *mut u32
   }
#[doc="Write the GPCHR register."]
   #[inline] pub fn set_gpchr<F: FnOnce(Gpchr) -> Gpchr>(&self, f: F) -> &Self {
      let value = f(Gpchr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x84) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ISFR register."]
   #[inline] pub fn isfr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa0) as *const u32
   }
#[doc="Get the *mut pointer for the ISFR register."]
   #[inline] pub fn isfr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa0) as *mut u32
   }
#[doc="Read the ISFR register."]
   #[inline] pub fn isfr(&self) -> Isfr { 
      unsafe {
         Isfr(::core::ptr::read_volatile((self.0 + 0xa0) as *const u32))
      }
   }
#[doc="Write the ISFR register."]
   #[inline] pub fn set_isfr<F: FnOnce(Isfr) -> Isfr>(&self, f: F) -> &Self {
      let value = f(Isfr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ISFR register."]
   #[inline] pub fn with_isfr<F: FnOnce(Isfr) -> Isfr>(&self, f: F) -> &Self {
      let tmp = self.isfr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Pin Control Register n"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcr(pub u32);
impl Pcr {
#[doc="Pull Select"]
   #[inline] pub fn ps(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Pull Select"]
   #[inline] pub fn set_ps<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Pull Enable"]
   #[inline] pub fn pe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Pull Enable"]
   #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Slew Rate Enable"]
   #[inline] pub fn sre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Slew Rate Enable"]
   #[inline] pub fn set_sre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Passive Filter Enable"]
   #[inline] pub fn pfe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Passive Filter Enable"]
   #[inline] pub fn set_pfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Open Drain Enable"]
   #[inline] pub fn ode(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Open Drain Enable"]
   #[inline] pub fn set_ode<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Drive Strength Enable"]
   #[inline] pub fn dse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Drive Strength Enable"]
   #[inline] pub fn set_dse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Pin Mux Control"]
   #[inline] pub fn mux(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Pin Mux Control"]
   #[inline] pub fn set_mux<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Lock Register"]
   #[inline] pub fn lk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Lock Register"]
   #[inline] pub fn set_lk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Interrupt Configuration"]
   #[inline] pub fn irqc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="Interrupt Configuration"]
   #[inline] pub fn set_irqc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Interrupt Status Flag"]
   #[inline] pub fn isf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Interrupt Status Flag"]
   #[inline] pub fn set_isf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Pcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ps() != 0 { try!(write!(f, " ps"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      if self.sre() != 0 { try!(write!(f, " sre"))}
      if self.pfe() != 0 { try!(write!(f, " pfe"))}
      if self.ode() != 0 { try!(write!(f, " ode"))}
      if self.dse() != 0 { try!(write!(f, " dse"))}
      if self.mux() != 0 { try!(write!(f, " mux=0x{:x}", self.mux()))}
      if self.lk() != 0 { try!(write!(f, " lk"))}
      if self.irqc() != 0 { try!(write!(f, " irqc=0x{:x}", self.irqc()))}
      if self.isf() != 0 { try!(write!(f, " isf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Global Pin Control Low Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpclr(pub u32);
impl Gpclr {
#[doc="Global Pin Write Data"]
   #[inline] pub fn gpwd(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Global Pin Write Data"]
   #[inline] pub fn set_gpwd<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Global Pin Write Enable"]
   #[inline] pub fn gpwe(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
   }
#[doc="Global Pin Write Enable"]
   #[inline] pub fn set_gpwe<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Gpclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gpwd() != 0 { try!(write!(f, " gpwd=0x{:x}", self.gpwd()))}
      if self.gpwe() != 0 { try!(write!(f, " gpwe=0x{:x}", self.gpwe()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Global Pin Control High Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpchr(pub u32);
impl Gpchr {
#[doc="Global Pin Write Data"]
   #[inline] pub fn gpwd(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Global Pin Write Data"]
   #[inline] pub fn set_gpwd<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Global Pin Write Enable"]
   #[inline] pub fn gpwe(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
   }
#[doc="Global Pin Write Enable"]
   #[inline] pub fn set_gpwe<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Gpchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpchr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gpwd() != 0 { try!(write!(f, " gpwd=0x{:x}", self.gpwd()))}
      if self.gpwe() != 0 { try!(write!(f, " gpwe=0x{:x}", self.gpwe()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Status Flag Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Isfr(pub u32);
impl Isfr {
#[doc="Interrupt Status Flag"]
   #[inline] pub fn isf<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
      let index: bits::R32 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Interrupt Status Flag"]
   #[inline] pub fn set_isf<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R32 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Isfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.isf(0) != 0 { try!(write!(f, " isf[0]"))}
      if self.isf(1) != 0 { try!(write!(f, " isf[1]"))}
      if self.isf(2) != 0 { try!(write!(f, " isf[2]"))}
      if self.isf(3) != 0 { try!(write!(f, " isf[3]"))}
      if self.isf(4) != 0 { try!(write!(f, " isf[4]"))}
      if self.isf(5) != 0 { try!(write!(f, " isf[5]"))}
      if self.isf(6) != 0 { try!(write!(f, " isf[6]"))}
      if self.isf(7) != 0 { try!(write!(f, " isf[7]"))}
      if self.isf(8) != 0 { try!(write!(f, " isf[8]"))}
      if self.isf(9) != 0 { try!(write!(f, " isf[9]"))}
      if self.isf(10) != 0 { try!(write!(f, " isf[10]"))}
      if self.isf(11) != 0 { try!(write!(f, " isf[11]"))}
      if self.isf(12) != 0 { try!(write!(f, " isf[12]"))}
      if self.isf(13) != 0 { try!(write!(f, " isf[13]"))}
      if self.isf(14) != 0 { try!(write!(f, " isf[14]"))}
      if self.isf(15) != 0 { try!(write!(f, " isf[15]"))}
      if self.isf(16) != 0 { try!(write!(f, " isf[16]"))}
      if self.isf(17) != 0 { try!(write!(f, " isf[17]"))}
      if self.isf(18) != 0 { try!(write!(f, " isf[18]"))}
      if self.isf(19) != 0 { try!(write!(f, " isf[19]"))}
      if self.isf(20) != 0 { try!(write!(f, " isf[20]"))}
      if self.isf(21) != 0 { try!(write!(f, " isf[21]"))}
      if self.isf(22) != 0 { try!(write!(f, " isf[22]"))}
      if self.isf(23) != 0 { try!(write!(f, " isf[23]"))}
      if self.isf(24) != 0 { try!(write!(f, " isf[24]"))}
      if self.isf(25) != 0 { try!(write!(f, " isf[25]"))}
      if self.isf(26) != 0 { try!(write!(f, " isf[26]"))}
      if self.isf(27) != 0 { try!(write!(f, " isf[27]"))}
      if self.isf(28) != 0 { try!(write!(f, " isf[28]"))}
      if self.isf(29) != 0 { try!(write!(f, " isf[29]"))}
      if self.isf(30) != 0 { try!(write!(f, " isf[30]"))}
      if self.isf(31) != 0 { try!(write!(f, " isf[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct PortPin { pub port: PortPeriph, pub index: usize }
