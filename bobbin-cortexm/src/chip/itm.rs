//! ITM
#[allow(unused_imports)] use bobbin_common::bits;
pub const ITM: Itm = Itm(0xe0000000);

#[doc="ITM"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Itm(pub u32);
impl Itm {
#[doc="Get the *const pointer for the STIM register."]
  #[inline] pub fn stim_ptr<I: Into<bits::R32>>(&self, index: I) -> *const u32 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x0 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the STIM register."]
  #[inline] pub fn stim_mut<I: Into<bits::R32>>(&self, index: I) -> *mut u32 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x0 + (index << 2)) as *mut u32
  }
#[doc="Read the STIM register."]
  #[inline] pub fn stim<I: Into<bits::R32>>(&self, index: I) -> Stim { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     unsafe {
        Stim(::core::ptr::read_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the STIM register."]
  #[inline] pub fn set_stim<I: Into<bits::R32>, F: FnOnce(Stim) -> Stim>(&self, index: I, f: F) -> &Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value = f(Stim(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the STIM register."]
  #[inline] pub fn with_stim<I: Into<bits::R32> + Copy, F: FnOnce(Stim) -> Stim>(&self, index: I, f: F) -> &Self {
     let tmp = self.stim(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the STIM16 register."]
  #[inline] pub fn stim16_ptr<I: Into<bits::R32>>(&self, index: I) -> *const u16 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x0 + (index << 2)) as *const u16
  }
#[doc="Get the *mut pointer for the STIM16 register."]
  #[inline] pub fn stim16_mut<I: Into<bits::R32>>(&self, index: I) -> *mut u16 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x0 + (index << 2)) as *mut u16
  }
#[doc="Read the STIM16 register."]
  #[inline] pub fn stim16<I: Into<bits::R32>>(&self, index: I) -> Stim16 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     unsafe {
        Stim16(::core::ptr::read_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *const u16))
     }
  }
#[doc="Write the STIM16 register."]
  #[inline] pub fn set_stim16<I: Into<bits::R32>, F: FnOnce(Stim16) -> Stim16>(&self, index: I, f: F) -> &Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value = f(Stim16(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the STIM16 register."]
  #[inline] pub fn with_stim16<I: Into<bits::R32> + Copy, F: FnOnce(Stim16) -> Stim16>(&self, index: I, f: F) -> &Self {
     let tmp = self.stim16(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the STIM8 register."]
  #[inline] pub fn stim8_ptr<I: Into<bits::R32>>(&self, index: I) -> *const u8 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x0 + (index << 2)) as *const u8
  }
#[doc="Get the *mut pointer for the STIM8 register."]
  #[inline] pub fn stim8_mut<I: Into<bits::R32>>(&self, index: I) -> *mut u8 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x0 + (index << 2)) as *mut u8
  }
#[doc="Read the STIM8 register."]
  #[inline] pub fn stim8<I: Into<bits::R32>>(&self, index: I) -> Stim8 { 
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     unsafe {
        Stim8(::core::ptr::read_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *const u8))
     }
  }
#[doc="Write the STIM8 register."]
  #[inline] pub fn set_stim8<I: Into<bits::R32>, F: FnOnce(Stim8) -> Stim8>(&self, index: I, f: F) -> &Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value = f(Stim8(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the STIM8 register."]
  #[inline] pub fn with_stim8<I: Into<bits::R32> + Copy, F: FnOnce(Stim8) -> Stim8>(&self, index: I, f: F) -> &Self {
     let tmp = self.stim8(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the TER register."]
  #[inline] pub fn ter_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe00) as *const u32
  }
#[doc="Get the *mut pointer for the TER register."]
  #[inline] pub fn ter_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe00) as *mut u32
  }
#[doc="Read the TER register."]
  #[inline] pub fn ter(&self) -> Ter { 
     unsafe {
        Ter(::core::ptr::read_volatile(((self.0 as usize) + 0xe00) as *const u32))
     }
  }
#[doc="Write the TER register."]
  #[inline] pub fn set_ter<F: FnOnce(Ter) -> Ter>(&self, f: F) -> &Self {
     let value = f(Ter(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe00) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TER register."]
  #[inline] pub fn with_ter<F: FnOnce(Ter) -> Ter>(&self, f: F) -> &Self {
     let tmp = self.ter();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe00) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the TPR register."]
  #[inline] pub fn tpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe40) as *const u32
  }
#[doc="Get the *mut pointer for the TPR register."]
  #[inline] pub fn tpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe40) as *mut u32
  }
#[doc="Read the TPR register."]
  #[inline] pub fn tpr(&self) -> Tpr { 
     unsafe {
        Tpr(::core::ptr::read_volatile(((self.0 as usize) + 0xe40) as *const u32))
     }
  }
#[doc="Write the TPR register."]
  #[inline] pub fn set_tpr<F: FnOnce(Tpr) -> Tpr>(&self, f: F) -> &Self {
     let value = f(Tpr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TPR register."]
  #[inline] pub fn with_tpr<F: FnOnce(Tpr) -> Tpr>(&self, f: F) -> &Self {
     let tmp = self.tpr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe40) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the TCR register."]
  #[inline] pub fn tcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe80) as *const u32
  }
#[doc="Get the *mut pointer for the TCR register."]
  #[inline] pub fn tcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe80) as *mut u32
  }
#[doc="Read the TCR register."]
  #[inline] pub fn tcr(&self) -> Tcr { 
     unsafe {
        Tcr(::core::ptr::read_volatile(((self.0 as usize) + 0xe80) as *const u32))
     }
  }
#[doc="Write the TCR register."]
  #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
     let value = f(Tcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe80) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TCR register."]
  #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
     let tmp = self.tcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe80) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the LAR register."]
  #[inline] pub fn lar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfb0) as *const u32
  }
#[doc="Get the *mut pointer for the LAR register."]
  #[inline] pub fn lar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfb0) as *mut u32
  }
#[doc="Write the LAR register."]
  #[inline] pub fn set_lar<F: FnOnce(Lar) -> Lar>(&self, f: F) -> &Self {
     let value = f(Lar(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xfb0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the LSR register."]
  #[inline] pub fn lsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfb4) as *const u32
  }
#[doc="Get the *mut pointer for the LSR register."]
  #[inline] pub fn lsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfb4) as *mut u32
  }
#[doc="Read the LSR register."]
  #[inline] pub fn lsr(&self) -> Lsr { 
     unsafe {
        Lsr(::core::ptr::read_volatile(((self.0 as usize) + 0xfb4) as *const u32))
     }
  }

}

#[doc="ITM Stimulus Port"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stim(pub u32);
impl Stim {
  #[inline] pub fn data(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
  #[inline] pub fn set_data<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Stim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ITM Stimulus Port - 16 Bit Access"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stim16(pub u16);
impl Stim16 {
  #[inline] pub fn data(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
  #[inline] pub fn set_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u16 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Stim16 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stim16 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ITM Stimulus Port - 8 Bit Access"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stim8(pub u8);
impl Stim8 {
  #[inline] pub fn data(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
  #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Stim8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stim8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Trace Enable Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ter(pub u32);
impl Ter {
#[doc="Bit mask to enable tracing on ITM stimulus ports. One bit per stimulus port."]
  #[inline] pub fn ena<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Bit mask to enable tracing on ITM stimulus ports. One bit per stimulus port."]
  #[inline] pub fn set_ena<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
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
impl ::core::fmt::Display for Ter {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ter {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ena(0) != 0 { try!(write!(f, " ena[0]"))}
      if self.ena(1) != 0 { try!(write!(f, " ena[1]"))}
      if self.ena(2) != 0 { try!(write!(f, " ena[2]"))}
      if self.ena(3) != 0 { try!(write!(f, " ena[3]"))}
      if self.ena(4) != 0 { try!(write!(f, " ena[4]"))}
      if self.ena(5) != 0 { try!(write!(f, " ena[5]"))}
      if self.ena(6) != 0 { try!(write!(f, " ena[6]"))}
      if self.ena(7) != 0 { try!(write!(f, " ena[7]"))}
      if self.ena(8) != 0 { try!(write!(f, " ena[8]"))}
      if self.ena(9) != 0 { try!(write!(f, " ena[9]"))}
      if self.ena(10) != 0 { try!(write!(f, " ena[10]"))}
      if self.ena(11) != 0 { try!(write!(f, " ena[11]"))}
      if self.ena(12) != 0 { try!(write!(f, " ena[12]"))}
      if self.ena(13) != 0 { try!(write!(f, " ena[13]"))}
      if self.ena(14) != 0 { try!(write!(f, " ena[14]"))}
      if self.ena(15) != 0 { try!(write!(f, " ena[15]"))}
      if self.ena(16) != 0 { try!(write!(f, " ena[16]"))}
      if self.ena(17) != 0 { try!(write!(f, " ena[17]"))}
      if self.ena(18) != 0 { try!(write!(f, " ena[18]"))}
      if self.ena(19) != 0 { try!(write!(f, " ena[19]"))}
      if self.ena(20) != 0 { try!(write!(f, " ena[20]"))}
      if self.ena(21) != 0 { try!(write!(f, " ena[21]"))}
      if self.ena(22) != 0 { try!(write!(f, " ena[22]"))}
      if self.ena(23) != 0 { try!(write!(f, " ena[23]"))}
      if self.ena(24) != 0 { try!(write!(f, " ena[24]"))}
      if self.ena(25) != 0 { try!(write!(f, " ena[25]"))}
      if self.ena(26) != 0 { try!(write!(f, " ena[26]"))}
      if self.ena(27) != 0 { try!(write!(f, " ena[27]"))}
      if self.ena(28) != 0 { try!(write!(f, " ena[28]"))}
      if self.ena(29) != 0 { try!(write!(f, " ena[29]"))}
      if self.ena(30) != 0 { try!(write!(f, " ena[30]"))}
      if self.ena(31) != 0 { try!(write!(f, " ena[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Trace Privilege Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tpr(pub u32);
impl Tpr {
#[doc="Bit mask to enable tracing on ITM stimulus ports: bit [0] = stimulus ports [7:0], bit [1] = stimulus ports [15:8], bit [2] = stimulus ports [23:16], bit [3] = stimulus ports [31:24]"]
  #[inline] pub fn tpr<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Bit mask to enable tracing on ITM stimulus ports: bit [0] = stimulus ports [7:0], bit [1] = stimulus ports [15:8], bit [2] = stimulus ports [23:16], bit [3] = stimulus ports [31:24]"]
  #[inline] pub fn set_tpr<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Tpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tpr(0) != 0 { try!(write!(f, " tpr[0]"))}
      if self.tpr(1) != 0 { try!(write!(f, " tpr[1]"))}
      if self.tpr(2) != 0 { try!(write!(f, " tpr[2]"))}
      if self.tpr(3) != 0 { try!(write!(f, " tpr[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Trace Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
#[doc="Set when ITM events present and being drained"]
  #[inline] pub fn busy(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="Set when ITM events present and being drained"]
  #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="ATB ID for CoreSight System"]
  #[inline] pub fn atbid(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
  }
#[doc="ATB ID for CoreSight System"]
  #[inline] pub fn set_atbid<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Timestamp prescaler: 0b00 = no prescaling, 0b01 = divide by 4, 0b10 = divide by 16, 0b11 = divide by 64."]
  #[inline] pub fn tsprescale(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
  }
#[doc="Timestamp prescaler: 0b00 = no prescaling, 0b01 = divide by 4, 0b10 = divide by 16, 0b11 = divide by 64."]
  #[inline] pub fn set_tsprescale<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Enable SWV behavior – count on TPIUEMIT and TPIUBAUD."]
  #[inline] pub fn swoena(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Enable SWV behavior – count on TPIUEMIT and TPIUBAUD."]
  #[inline] pub fn set_swoena<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Enables the DWT stimulus."]
  #[inline] pub fn dwtena(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Enables the DWT stimulus."]
  #[inline] pub fn set_dwtena<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Enables sync packets for TPIU."]
  #[inline] pub fn syncena(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Enables sync packets for TPIU."]
  #[inline] pub fn set_syncena<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of cycles. This provides a time reference for packets and inter-packet gaps."]
  #[inline] pub fn tsena(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of cycles. This provides a time reference for packets and inter-packet gaps."]
  #[inline] pub fn set_tsena<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Enable ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
  #[inline] pub fn itmena(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Enable ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
  #[inline] pub fn set_itmena<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.atbid() != 0 { try!(write!(f, " atbid=0x{:x}", self.atbid()))}
      if self.tsprescale() != 0 { try!(write!(f, " tsprescale=0x{:x}", self.tsprescale()))}
      if self.swoena() != 0 { try!(write!(f, " swoena"))}
      if self.dwtena() != 0 { try!(write!(f, " dwtena"))}
      if self.syncena() != 0 { try!(write!(f, " syncena"))}
      if self.tsena() != 0 { try!(write!(f, " tsena"))}
      if self.itmena() != 0 { try!(write!(f, " itmena"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Lock Access Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lar(pub u32);
impl Lar {
#[doc="A privileged write of 0xC5ACCE55 enables more write access to Control Register 0xE00::0xFFC. An invalid write removes write access."]
  #[inline] pub fn access(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="A privileged write of 0xC5ACCE55 enables more write access to Control Register 0xE00::0xFFC. An invalid write removes write access."]
  #[inline] pub fn set_access<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Lar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Lock Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lsr(pub u32);
impl Lsr {
#[doc="You cannot implement 8-bit lock accesses."]
  #[inline] pub fn byteacc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="You cannot implement 8-bit lock accesses."]
  #[inline] pub fn set_byteacc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Write access to component is blocked. All writes are ignored, reads are permitted."]
  #[inline] pub fn access(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Write access to component is blocked. All writes are ignored, reads are permitted."]
  #[inline] pub fn set_access<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Indicates that a lock mechanism exists for this component."]
  #[inline] pub fn present(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Indicates that a lock mechanism exists for this component."]
  #[inline] pub fn set_present<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Lsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.byteacc() != 0 { try!(write!(f, " byteacc"))}
      if self.access() != 0 { try!(write!(f, " access"))}
      if self.present() != 0 { try!(write!(f, " present"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

