//! Memory Protection Unit
#[allow(unused_imports)] use bobbin_common::*;

periph!(MPU, Mpu, 0xe000ed90);

#[doc="Memory Protection Unit"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mpu(pub usize);
impl Mpu {
#[doc="Get the *const pointer for the MPU_TYPE register."]
   #[inline] pub fn mpu_type_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_TYPE register."]
   #[inline] pub fn mpu_type_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the MPU_TYPE register."]
   #[inline] pub fn mpu_type(&self) -> MpuType { 
      unsafe {
         MpuType(read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the MPU_TYPE register."]
   #[inline] pub fn set_mpu_type<F: FnOnce(MpuType) -> MpuType>(&self, f: F) -> &Self {
      let value = f(MpuType(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_TYPE register."]
   #[inline] pub fn with_mpu_type<F: FnOnce(MpuType) -> MpuType>(&self, f: F) -> &Self {
      let tmp = self.mpu_type();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_CTRL register."]
   #[inline] pub fn mpu_ctrl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_CTRL register."]
   #[inline] pub fn mpu_ctrl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the MPU_CTRL register."]
   #[inline] pub fn mpu_ctrl(&self) -> MpuCtrl { 
      unsafe {
         MpuCtrl(read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the MPU_CTRL register."]
   #[inline] pub fn set_mpu_ctrl<F: FnOnce(MpuCtrl) -> MpuCtrl>(&self, f: F) -> &Self {
      let value = f(MpuCtrl(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_CTRL register."]
   #[inline] pub fn with_mpu_ctrl<F: FnOnce(MpuCtrl) -> MpuCtrl>(&self, f: F) -> &Self {
      let tmp = self.mpu_ctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RNR register."]
   #[inline] pub fn mpu_rnr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RNR register."]
   #[inline] pub fn mpu_rnr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the MPU_RNR register."]
   #[inline] pub fn mpu_rnr(&self) -> MpuRnr { 
      unsafe {
         MpuRnr(read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the MPU_RNR register."]
   #[inline] pub fn set_mpu_rnr<F: FnOnce(MpuRnr) -> MpuRnr>(&self, f: F) -> &Self {
      let value = f(MpuRnr(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RNR register."]
   #[inline] pub fn with_mpu_rnr<F: FnOnce(MpuRnr) -> MpuRnr>(&self, f: F) -> &Self {
      let tmp = self.mpu_rnr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RBAR register."]
   #[inline] pub fn mpu_rbar_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RBAR register."]
   #[inline] pub fn mpu_rbar_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }
#[doc="Read the MPU_RBAR register."]
   #[inline] pub fn mpu_rbar(&self) -> MpuRbar { 
      unsafe {
         MpuRbar(read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the MPU_RBAR register."]
   #[inline] pub fn set_mpu_rbar<F: FnOnce(MpuRbar) -> MpuRbar>(&self, f: F) -> &Self {
      let value = f(MpuRbar(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RBAR register."]
   #[inline] pub fn with_mpu_rbar<F: FnOnce(MpuRbar) -> MpuRbar>(&self, f: F) -> &Self {
      let tmp = self.mpu_rbar();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RASR register."]
   #[inline] pub fn mpu_rasr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x10) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RASR register."]
   #[inline] pub fn mpu_rasr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x10) as *mut u32
   }
#[doc="Read the MPU_RASR register."]
   #[inline] pub fn mpu_rasr(&self) -> MpuRasr { 
      unsafe {
         MpuRasr(read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the MPU_RASR register."]
   #[inline] pub fn set_mpu_rasr<F: FnOnce(MpuRasr) -> MpuRasr>(&self, f: F) -> &Self {
      let value = f(MpuRasr(0));
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RASR register."]
   #[inline] pub fn with_mpu_rasr<F: FnOnce(MpuRasr) -> MpuRasr>(&self, f: F) -> &Self {
      let tmp = self.mpu_rasr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RBAR_A1 register."]
   #[inline] pub fn mpu_rbar_a1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x14) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RBAR_A1 register."]
   #[inline] pub fn mpu_rbar_a1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x14) as *mut u32
   }
#[doc="Read the MPU_RBAR_A1 register."]
   #[inline] pub fn mpu_rbar_a1(&self) -> MpuRbarA1 { 
      unsafe {
         MpuRbarA1(read_volatile((self.0 + 0x14) as *const u32))
      }
   }
#[doc="Write the MPU_RBAR_A1 register."]
   #[inline] pub fn set_mpu_rbar_a1<F: FnOnce(MpuRbarA1) -> MpuRbarA1>(&self, f: F) -> &Self {
      let value = f(MpuRbarA1(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RBAR_A1 register."]
   #[inline] pub fn with_mpu_rbar_a1<F: FnOnce(MpuRbarA1) -> MpuRbarA1>(&self, f: F) -> &Self {
      let tmp = self.mpu_rbar_a1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RASR_A1 register."]
   #[inline] pub fn mpu_rasr_a1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x18) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RASR_A1 register."]
   #[inline] pub fn mpu_rasr_a1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x18) as *mut u32
   }
#[doc="Read the MPU_RASR_A1 register."]
   #[inline] pub fn mpu_rasr_a1(&self) -> MpuRasrA1 { 
      unsafe {
         MpuRasrA1(read_volatile((self.0 + 0x18) as *const u32))
      }
   }
#[doc="Write the MPU_RASR_A1 register."]
   #[inline] pub fn set_mpu_rasr_a1<F: FnOnce(MpuRasrA1) -> MpuRasrA1>(&self, f: F) -> &Self {
      let value = f(MpuRasrA1(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RASR_A1 register."]
   #[inline] pub fn with_mpu_rasr_a1<F: FnOnce(MpuRasrA1) -> MpuRasrA1>(&self, f: F) -> &Self {
      let tmp = self.mpu_rasr_a1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RBAR_A2 register."]
   #[inline] pub fn mpu_rbar_a2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x1c) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RBAR_A2 register."]
   #[inline] pub fn mpu_rbar_a2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x1c) as *mut u32
   }
#[doc="Read the MPU_RBAR_A2 register."]
   #[inline] pub fn mpu_rbar_a2(&self) -> MpuRbarA2 { 
      unsafe {
         MpuRbarA2(read_volatile((self.0 + 0x1c) as *const u32))
      }
   }
#[doc="Write the MPU_RBAR_A2 register."]
   #[inline] pub fn set_mpu_rbar_a2<F: FnOnce(MpuRbarA2) -> MpuRbarA2>(&self, f: F) -> &Self {
      let value = f(MpuRbarA2(0));
      unsafe {
         write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RBAR_A2 register."]
   #[inline] pub fn with_mpu_rbar_a2<F: FnOnce(MpuRbarA2) -> MpuRbarA2>(&self, f: F) -> &Self {
      let tmp = self.mpu_rbar_a2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RASR_A2 register."]
   #[inline] pub fn mpu_rasr_a2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x20) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RASR_A2 register."]
   #[inline] pub fn mpu_rasr_a2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x20) as *mut u32
   }
#[doc="Read the MPU_RASR_A2 register."]
   #[inline] pub fn mpu_rasr_a2(&self) -> MpuRasrA2 { 
      unsafe {
         MpuRasrA2(read_volatile((self.0 + 0x20) as *const u32))
      }
   }
#[doc="Write the MPU_RASR_A2 register."]
   #[inline] pub fn set_mpu_rasr_a2<F: FnOnce(MpuRasrA2) -> MpuRasrA2>(&self, f: F) -> &Self {
      let value = f(MpuRasrA2(0));
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RASR_A2 register."]
   #[inline] pub fn with_mpu_rasr_a2<F: FnOnce(MpuRasrA2) -> MpuRasrA2>(&self, f: F) -> &Self {
      let tmp = self.mpu_rasr_a2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RBAR_A3 register."]
   #[inline] pub fn mpu_rbar_a3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x24) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RBAR_A3 register."]
   #[inline] pub fn mpu_rbar_a3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x24) as *mut u32
   }
#[doc="Read the MPU_RBAR_A3 register."]
   #[inline] pub fn mpu_rbar_a3(&self) -> MpuRbarA3 { 
      unsafe {
         MpuRbarA3(read_volatile((self.0 + 0x24) as *const u32))
      }
   }
#[doc="Write the MPU_RBAR_A3 register."]
   #[inline] pub fn set_mpu_rbar_a3<F: FnOnce(MpuRbarA3) -> MpuRbarA3>(&self, f: F) -> &Self {
      let value = f(MpuRbarA3(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RBAR_A3 register."]
   #[inline] pub fn with_mpu_rbar_a3<F: FnOnce(MpuRbarA3) -> MpuRbarA3>(&self, f: F) -> &Self {
      let tmp = self.mpu_rbar_a3();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MPU_RASR_A3 register."]
   #[inline] pub fn mpu_rasr_a3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x28) as *const u32
   }
#[doc="Get the *mut pointer for the MPU_RASR_A3 register."]
   #[inline] pub fn mpu_rasr_a3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x28) as *mut u32
   }
#[doc="Read the MPU_RASR_A3 register."]
   #[inline] pub fn mpu_rasr_a3(&self) -> MpuRasrA3 { 
      unsafe {
         MpuRasrA3(read_volatile((self.0 + 0x28) as *const u32))
      }
   }
#[doc="Write the MPU_RASR_A3 register."]
   #[inline] pub fn set_mpu_rasr_a3<F: FnOnce(MpuRasrA3) -> MpuRasrA3>(&self, f: F) -> &Self {
      let value = f(MpuRasrA3(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MPU_RASR_A3 register."]
   #[inline] pub fn with_mpu_rasr_a3<F: FnOnce(MpuRasrA3) -> MpuRasrA3>(&self, f: F) -> &Self {
      let tmp = self.mpu_rasr_a3();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

}

#[doc="MPU Type Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuType(pub u32);
impl MpuType {
#[doc="Indicates the number of supported MPU instruction regions. Always contains 0x00. The MPU memory map is unified and is described by the DREGION field."]
   #[inline] pub fn iregion(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Indicates the number of supported MPU instruction regions. Always contains 0x00. The MPU memory map is unified and is described by the DREGION field."]
   #[inline] pub fn set_iregion<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Indicates the number of supported MPU data regions: 0x08 = eight MPU regions."]
   #[inline] pub fn dregion(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Indicates the number of supported MPU data regions: 0x08 = eight MPU regions."]
   #[inline] pub fn set_dregion<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Indicates support for unified or separate instruction and date memory maps: 0 = unified."]
   #[inline] pub fn separate(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Indicates support for unified or separate instruction and date memory maps: 0 = unified."]
   #[inline] pub fn set_separate<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for MpuType {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuType {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iregion() != 0 { try!(write!(f, " iregion=0x{:x}", self.iregion()))}
      if self.dregion() != 0 { try!(write!(f, " dregion=0x{:x}", self.dregion()))}
      if self.separate() != 0 { try!(write!(f, " separate"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="MPU Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuCtrl(pub u32);
impl MpuCtrl {
#[doc="Enables privileged software access to the default memory map: 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault. 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses. When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map. If the MPU is disabled, the processor ignores this bit."]
   #[inline] pub fn privdefena(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Enables privileged software access to the default memory map: 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault. 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses. When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map. If the MPU is disabled, the processor ignores this bit."]
   #[inline] pub fn set_privdefena<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Enables the operation of MPU during hard fault, NMI, and FAULTMASK handlers. When the MPU is enabled: 0 = MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit1 = the MPU is enabled during hard fault, NMI, and FAULTMASK handlers. When the MPU is disabled, if this bit is set to 1 the behavior is Unpredictable."]
   #[inline] pub fn hfnmiena(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enables the operation of MPU during hard fault, NMI, and FAULTMASK handlers. When the MPU is enabled: 0 = MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit1 = the MPU is enabled during hard fault, NMI, and FAULTMASK handlers. When the MPU is disabled, if this bit is set to 1 the behavior is Unpredictable."]
   #[inline] pub fn set_hfnmiena<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Enables the MPU: 0 = MPU disabled, 1 = MPU enabled."]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Enables the MPU: 0 = MPU disabled, 1 = MPU enabled."]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for MpuCtrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuCtrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.privdefena() != 0 { try!(write!(f, " privdefena"))}
      if self.hfnmiena() != 0 { try!(write!(f, " hfnmiena"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="MPU Region Number Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRnr(pub u32);
impl MpuRnr {
#[doc="Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
   #[inline] pub fn region(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
   #[inline] pub fn set_region<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for MpuRnr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRnr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.region() != 0 { try!(write!(f, " region=0x{:x}", self.region()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="MPU Region Base Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRbar(pub u32);
impl MpuRbar {
#[doc="Region base address field. The value of N depends on the region size."]
   #[inline] pub fn addr(&self) -> bits::U27 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffffff) as u32) } // [31:5]
   }
#[doc="Region base address field. The value of N depends on the region size."]
   #[inline] pub fn set_addr<V: Into<bits::U27>>(mut self, value: V) -> Self {
      let value: bits::U27 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7ffffff << 5);
      self.0 |= value << 5;
      self
   }

#[doc="MPU Region Number valid bit: 0 = MPU_RNR not changed, and the processor: updates the base address for the region specified in the MPU_RNR ignores the value of the REGION field
1 = the processor: updates the value of the MPU_RNR to the value of the REGION field updates the base address for the region specified in the REGION field. Always reads as zero."]
   #[inline] pub fn valid(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="MPU Region Number valid bit: 0 = MPU_RNR not changed, and the processor: updates the base address for the region specified in the MPU_RNR ignores the value of the REGION field
1 = the processor: updates the value of the MPU_RNR to the value of the REGION field updates the base address for the region specified in the REGION field. Always reads as zero."]
   #[inline] pub fn set_valid<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="MPU region field: For the behavior on writes, see the description of the VALID field. On reads, returns the current region number, as specified by the RNR."]
   #[inline] pub fn region(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="MPU region field: For the behavior on writes, see the description of the VALID field. On reads, returns the current region number, as specified by the RNR."]
   #[inline] pub fn set_region<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for MpuRbar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRbar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.valid() != 0 { try!(write!(f, " valid"))}
      if self.region() != 0 { try!(write!(f, " region=0x{:x}", self.region()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="MPU Region Attribute and Size Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRasr(pub u32);
impl MpuRasr {
#[doc="Instruction access disable bit: 0 = instruction fetches enabled, 1 = instruction fetches disabled."]
   #[inline] pub fn xn(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Instruction access disable bit: 0 = instruction fetches enabled, 1 = instruction fetches disabled."]
   #[inline] pub fn set_xn<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Access permission field."]
   #[inline] pub fn ap(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Access permission field."]
   #[inline] pub fn set_ap<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Memory access attribute TEX"]
   #[inline] pub fn tex(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
   }
#[doc="Memory access attribute TEX"]
   #[inline] pub fn set_tex<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="C Bit"]
   #[inline] pub fn c(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="C Bit"]
   #[inline] pub fn set_c<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="B Bit"]
   #[inline] pub fn b(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="B Bit"]
   #[inline] pub fn set_b<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Sharable Bit"]
   #[inline] pub fn s(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Sharable Bit"]
   #[inline] pub fn set_s<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="Subregion disable bits. For each bit in this field: 0 = corresponding sub-region is enabled1 = corresponding sub-region is disabled. See Subregions for more information. Region sizes of 128 bytes and less do not support subregions. When writing the attributes for such a region, write the SRD field as 0x00."]
   #[inline] pub fn srd(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Subregion disable bits. For each bit in this field: 0 = corresponding sub-region is enabled1 = corresponding sub-region is disabled. See Subregions for more information. Region sizes of 128 bytes and less do not support subregions. When writing the attributes for such a region, write the SRD field as 0x00."]
   #[inline] pub fn set_srd<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Specifies the size of the MPU protection region. The minimum permitted value is 3 (0b00010), see See SIZE field values for more information."]
   #[inline] pub fn size(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1f) as u8) } // [5:1]
   }
#[doc="Specifies the size of the MPU protection region. The minimum permitted value is 3 (0b00010), see See SIZE field values for more information."]
   #[inline] pub fn set_size<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Region enable bit."]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Region enable bit."]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for MpuRasr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRasr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.xn() != 0 { try!(write!(f, " xn"))}
      if self.ap() != 0 { try!(write!(f, " ap=0x{:x}", self.ap()))}
      if self.tex() != 0 { try!(write!(f, " tex=0x{:x}", self.tex()))}
      if self.c() != 0 { try!(write!(f, " c"))}
      if self.b() != 0 { try!(write!(f, " b"))}
      if self.s() != 0 { try!(write!(f, " s"))}
      if self.srd() != 0 { try!(write!(f, " srd=0x{:x}", self.srd()))}
      if self.size() != 0 { try!(write!(f, " size=0x{:x}", self.size()))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Alias of RBAR, MPU Region Base Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRbarA1(pub u32);
impl MpuRbarA1 {
}
impl ::core::fmt::Display for MpuRbarA1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRbarA1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Alias of RASR, MPU Region Attribute and Size Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRasrA1(pub u32);
impl MpuRasrA1 {
}
impl ::core::fmt::Display for MpuRasrA1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRasrA1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Alias of RBAR, MPU Region Base Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRbarA2(pub u32);
impl MpuRbarA2 {
}
impl ::core::fmt::Display for MpuRbarA2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRbarA2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Alias of RASR, MPU Region Attribute and Size Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRasrA2(pub u32);
impl MpuRasrA2 {
}
impl ::core::fmt::Display for MpuRasrA2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRasrA2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Alias of RBAR, MPU Region Base Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRbarA3(pub u32);
impl MpuRbarA3 {
}
impl ::core::fmt::Display for MpuRbarA3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRbarA3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Alias of RASR, MPU Region Attribute and Size Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuRasrA3(pub u32);
impl MpuRasrA3 {
}
impl ::core::fmt::Display for MpuRasrA3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for MpuRasrA3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

