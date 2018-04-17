
::bobbin_mcu::periph!( MPU, Mpu, MPU_PERIPH, MpuPeriph, MPU_OWNED, MPU_REF_COUNT, 0xe000ed90, 0x00, 0x03);


#[doc="Memory Protection Unit"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuPeriph(pub usize);
impl MpuPeriph {
    #[doc="Get the MPU_TYPE Register."]
    #[inline] pub fn mpu_type_reg(&self) -> ::bobbin_mcu::register::Register<MpuType> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuType, 0x0)
    }

    #[doc="Get the *mut pointer for the MPU_TYPE register."]
    #[inline] pub fn mpu_type_mut(&self) -> *mut MpuType { 
        self.mpu_type_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_TYPE register."]
    #[inline] pub fn mpu_type_ptr(&self) -> *const MpuType { 
        self.mpu_type_reg().ptr()
    }

    #[doc="Read the MPU_TYPE register."]
    #[inline] pub fn mpu_type(&self) -> MpuType { 
        self.mpu_type_reg().read()
    }

    #[doc="Write the MPU_TYPE register."]
    #[inline] pub fn write_mpu_type(&self, value: MpuType) -> &Self { 
        self.mpu_type_reg().write(value);
        self
    }

    #[doc="Set the MPU_TYPE register."]
    #[inline] pub fn set_mpu_type<F: FnOnce(MpuType) -> MpuType>(&self, f: F) -> &Self {
        self.mpu_type_reg().set(f);
        self
    }

    #[doc="Modify the MPU_TYPE register."]
    #[inline] pub fn with_mpu_type<F: FnOnce(MpuType) -> MpuType>(&self, f: F) -> &Self {
        self.mpu_type_reg().with(f);
        self
    }

    #[doc="Get the MPU_CTRL Register."]
    #[inline] pub fn mpu_ctrl_reg(&self) -> ::bobbin_mcu::register::Register<MpuCtrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuCtrl, 0x4)
    }

    #[doc="Get the *mut pointer for the MPU_CTRL register."]
    #[inline] pub fn mpu_ctrl_mut(&self) -> *mut MpuCtrl { 
        self.mpu_ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_CTRL register."]
    #[inline] pub fn mpu_ctrl_ptr(&self) -> *const MpuCtrl { 
        self.mpu_ctrl_reg().ptr()
    }

    #[doc="Read the MPU_CTRL register."]
    #[inline] pub fn mpu_ctrl(&self) -> MpuCtrl { 
        self.mpu_ctrl_reg().read()
    }

    #[doc="Write the MPU_CTRL register."]
    #[inline] pub fn write_mpu_ctrl(&self, value: MpuCtrl) -> &Self { 
        self.mpu_ctrl_reg().write(value);
        self
    }

    #[doc="Set the MPU_CTRL register."]
    #[inline] pub fn set_mpu_ctrl<F: FnOnce(MpuCtrl) -> MpuCtrl>(&self, f: F) -> &Self {
        self.mpu_ctrl_reg().set(f);
        self
    }

    #[doc="Modify the MPU_CTRL register."]
    #[inline] pub fn with_mpu_ctrl<F: FnOnce(MpuCtrl) -> MpuCtrl>(&self, f: F) -> &Self {
        self.mpu_ctrl_reg().with(f);
        self
    }

    #[doc="Get the MPU_RNR Register."]
    #[inline] pub fn mpu_rnr_reg(&self) -> ::bobbin_mcu::register::Register<MpuRnr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRnr, 0x8)
    }

    #[doc="Get the *mut pointer for the MPU_RNR register."]
    #[inline] pub fn mpu_rnr_mut(&self) -> *mut MpuRnr { 
        self.mpu_rnr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RNR register."]
    #[inline] pub fn mpu_rnr_ptr(&self) -> *const MpuRnr { 
        self.mpu_rnr_reg().ptr()
    }

    #[doc="Read the MPU_RNR register."]
    #[inline] pub fn mpu_rnr(&self) -> MpuRnr { 
        self.mpu_rnr_reg().read()
    }

    #[doc="Write the MPU_RNR register."]
    #[inline] pub fn write_mpu_rnr(&self, value: MpuRnr) -> &Self { 
        self.mpu_rnr_reg().write(value);
        self
    }

    #[doc="Set the MPU_RNR register."]
    #[inline] pub fn set_mpu_rnr<F: FnOnce(MpuRnr) -> MpuRnr>(&self, f: F) -> &Self {
        self.mpu_rnr_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RNR register."]
    #[inline] pub fn with_mpu_rnr<F: FnOnce(MpuRnr) -> MpuRnr>(&self, f: F) -> &Self {
        self.mpu_rnr_reg().with(f);
        self
    }

    #[doc="Get the MPU_RBAR Register."]
    #[inline] pub fn mpu_rbar_reg(&self) -> ::bobbin_mcu::register::Register<MpuRbar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRbar, 0xc)
    }

    #[doc="Get the *mut pointer for the MPU_RBAR register."]
    #[inline] pub fn mpu_rbar_mut(&self) -> *mut MpuRbar { 
        self.mpu_rbar_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RBAR register."]
    #[inline] pub fn mpu_rbar_ptr(&self) -> *const MpuRbar { 
        self.mpu_rbar_reg().ptr()
    }

    #[doc="Read the MPU_RBAR register."]
    #[inline] pub fn mpu_rbar(&self) -> MpuRbar { 
        self.mpu_rbar_reg().read()
    }

    #[doc="Write the MPU_RBAR register."]
    #[inline] pub fn write_mpu_rbar(&self, value: MpuRbar) -> &Self { 
        self.mpu_rbar_reg().write(value);
        self
    }

    #[doc="Set the MPU_RBAR register."]
    #[inline] pub fn set_mpu_rbar<F: FnOnce(MpuRbar) -> MpuRbar>(&self, f: F) -> &Self {
        self.mpu_rbar_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RBAR register."]
    #[inline] pub fn with_mpu_rbar<F: FnOnce(MpuRbar) -> MpuRbar>(&self, f: F) -> &Self {
        self.mpu_rbar_reg().with(f);
        self
    }

    #[doc="Get the MPU_RASR Register."]
    #[inline] pub fn mpu_rasr_reg(&self) -> ::bobbin_mcu::register::Register<MpuRasr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRasr, 0x10)
    }

    #[doc="Get the *mut pointer for the MPU_RASR register."]
    #[inline] pub fn mpu_rasr_mut(&self) -> *mut MpuRasr { 
        self.mpu_rasr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RASR register."]
    #[inline] pub fn mpu_rasr_ptr(&self) -> *const MpuRasr { 
        self.mpu_rasr_reg().ptr()
    }

    #[doc="Read the MPU_RASR register."]
    #[inline] pub fn mpu_rasr(&self) -> MpuRasr { 
        self.mpu_rasr_reg().read()
    }

    #[doc="Write the MPU_RASR register."]
    #[inline] pub fn write_mpu_rasr(&self, value: MpuRasr) -> &Self { 
        self.mpu_rasr_reg().write(value);
        self
    }

    #[doc="Set the MPU_RASR register."]
    #[inline] pub fn set_mpu_rasr<F: FnOnce(MpuRasr) -> MpuRasr>(&self, f: F) -> &Self {
        self.mpu_rasr_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RASR register."]
    #[inline] pub fn with_mpu_rasr<F: FnOnce(MpuRasr) -> MpuRasr>(&self, f: F) -> &Self {
        self.mpu_rasr_reg().with(f);
        self
    }

    #[doc="Get the MPU_RBAR_A1 Register."]
    #[inline] pub fn mpu_rbar_a1_reg(&self) -> ::bobbin_mcu::register::Register<MpuRbarA1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRbarA1, 0x14)
    }

    #[doc="Get the *mut pointer for the MPU_RBAR_A1 register."]
    #[inline] pub fn mpu_rbar_a1_mut(&self) -> *mut MpuRbarA1 { 
        self.mpu_rbar_a1_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RBAR_A1 register."]
    #[inline] pub fn mpu_rbar_a1_ptr(&self) -> *const MpuRbarA1 { 
        self.mpu_rbar_a1_reg().ptr()
    }

    #[doc="Read the MPU_RBAR_A1 register."]
    #[inline] pub fn mpu_rbar_a1(&self) -> MpuRbarA1 { 
        self.mpu_rbar_a1_reg().read()
    }

    #[doc="Write the MPU_RBAR_A1 register."]
    #[inline] pub fn write_mpu_rbar_a1(&self, value: MpuRbarA1) -> &Self { 
        self.mpu_rbar_a1_reg().write(value);
        self
    }

    #[doc="Set the MPU_RBAR_A1 register."]
    #[inline] pub fn set_mpu_rbar_a1<F: FnOnce(MpuRbarA1) -> MpuRbarA1>(&self, f: F) -> &Self {
        self.mpu_rbar_a1_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RBAR_A1 register."]
    #[inline] pub fn with_mpu_rbar_a1<F: FnOnce(MpuRbarA1) -> MpuRbarA1>(&self, f: F) -> &Self {
        self.mpu_rbar_a1_reg().with(f);
        self
    }

    #[doc="Get the MPU_RASR_A1 Register."]
    #[inline] pub fn mpu_rasr_a1_reg(&self) -> ::bobbin_mcu::register::Register<MpuRasrA1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRasrA1, 0x18)
    }

    #[doc="Get the *mut pointer for the MPU_RASR_A1 register."]
    #[inline] pub fn mpu_rasr_a1_mut(&self) -> *mut MpuRasrA1 { 
        self.mpu_rasr_a1_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RASR_A1 register."]
    #[inline] pub fn mpu_rasr_a1_ptr(&self) -> *const MpuRasrA1 { 
        self.mpu_rasr_a1_reg().ptr()
    }

    #[doc="Read the MPU_RASR_A1 register."]
    #[inline] pub fn mpu_rasr_a1(&self) -> MpuRasrA1 { 
        self.mpu_rasr_a1_reg().read()
    }

    #[doc="Write the MPU_RASR_A1 register."]
    #[inline] pub fn write_mpu_rasr_a1(&self, value: MpuRasrA1) -> &Self { 
        self.mpu_rasr_a1_reg().write(value);
        self
    }

    #[doc="Set the MPU_RASR_A1 register."]
    #[inline] pub fn set_mpu_rasr_a1<F: FnOnce(MpuRasrA1) -> MpuRasrA1>(&self, f: F) -> &Self {
        self.mpu_rasr_a1_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RASR_A1 register."]
    #[inline] pub fn with_mpu_rasr_a1<F: FnOnce(MpuRasrA1) -> MpuRasrA1>(&self, f: F) -> &Self {
        self.mpu_rasr_a1_reg().with(f);
        self
    }

    #[doc="Get the MPU_RBAR_A2 Register."]
    #[inline] pub fn mpu_rbar_a2_reg(&self) -> ::bobbin_mcu::register::Register<MpuRbarA2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRbarA2, 0x1c)
    }

    #[doc="Get the *mut pointer for the MPU_RBAR_A2 register."]
    #[inline] pub fn mpu_rbar_a2_mut(&self) -> *mut MpuRbarA2 { 
        self.mpu_rbar_a2_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RBAR_A2 register."]
    #[inline] pub fn mpu_rbar_a2_ptr(&self) -> *const MpuRbarA2 { 
        self.mpu_rbar_a2_reg().ptr()
    }

    #[doc="Read the MPU_RBAR_A2 register."]
    #[inline] pub fn mpu_rbar_a2(&self) -> MpuRbarA2 { 
        self.mpu_rbar_a2_reg().read()
    }

    #[doc="Write the MPU_RBAR_A2 register."]
    #[inline] pub fn write_mpu_rbar_a2(&self, value: MpuRbarA2) -> &Self { 
        self.mpu_rbar_a2_reg().write(value);
        self
    }

    #[doc="Set the MPU_RBAR_A2 register."]
    #[inline] pub fn set_mpu_rbar_a2<F: FnOnce(MpuRbarA2) -> MpuRbarA2>(&self, f: F) -> &Self {
        self.mpu_rbar_a2_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RBAR_A2 register."]
    #[inline] pub fn with_mpu_rbar_a2<F: FnOnce(MpuRbarA2) -> MpuRbarA2>(&self, f: F) -> &Self {
        self.mpu_rbar_a2_reg().with(f);
        self
    }

    #[doc="Get the MPU_RASR_A2 Register."]
    #[inline] pub fn mpu_rasr_a2_reg(&self) -> ::bobbin_mcu::register::Register<MpuRasrA2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRasrA2, 0x20)
    }

    #[doc="Get the *mut pointer for the MPU_RASR_A2 register."]
    #[inline] pub fn mpu_rasr_a2_mut(&self) -> *mut MpuRasrA2 { 
        self.mpu_rasr_a2_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RASR_A2 register."]
    #[inline] pub fn mpu_rasr_a2_ptr(&self) -> *const MpuRasrA2 { 
        self.mpu_rasr_a2_reg().ptr()
    }

    #[doc="Read the MPU_RASR_A2 register."]
    #[inline] pub fn mpu_rasr_a2(&self) -> MpuRasrA2 { 
        self.mpu_rasr_a2_reg().read()
    }

    #[doc="Write the MPU_RASR_A2 register."]
    #[inline] pub fn write_mpu_rasr_a2(&self, value: MpuRasrA2) -> &Self { 
        self.mpu_rasr_a2_reg().write(value);
        self
    }

    #[doc="Set the MPU_RASR_A2 register."]
    #[inline] pub fn set_mpu_rasr_a2<F: FnOnce(MpuRasrA2) -> MpuRasrA2>(&self, f: F) -> &Self {
        self.mpu_rasr_a2_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RASR_A2 register."]
    #[inline] pub fn with_mpu_rasr_a2<F: FnOnce(MpuRasrA2) -> MpuRasrA2>(&self, f: F) -> &Self {
        self.mpu_rasr_a2_reg().with(f);
        self
    }

    #[doc="Get the MPU_RBAR_A3 Register."]
    #[inline] pub fn mpu_rbar_a3_reg(&self) -> ::bobbin_mcu::register::Register<MpuRbarA3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRbarA3, 0x24)
    }

    #[doc="Get the *mut pointer for the MPU_RBAR_A3 register."]
    #[inline] pub fn mpu_rbar_a3_mut(&self) -> *mut MpuRbarA3 { 
        self.mpu_rbar_a3_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RBAR_A3 register."]
    #[inline] pub fn mpu_rbar_a3_ptr(&self) -> *const MpuRbarA3 { 
        self.mpu_rbar_a3_reg().ptr()
    }

    #[doc="Read the MPU_RBAR_A3 register."]
    #[inline] pub fn mpu_rbar_a3(&self) -> MpuRbarA3 { 
        self.mpu_rbar_a3_reg().read()
    }

    #[doc="Write the MPU_RBAR_A3 register."]
    #[inline] pub fn write_mpu_rbar_a3(&self, value: MpuRbarA3) -> &Self { 
        self.mpu_rbar_a3_reg().write(value);
        self
    }

    #[doc="Set the MPU_RBAR_A3 register."]
    #[inline] pub fn set_mpu_rbar_a3<F: FnOnce(MpuRbarA3) -> MpuRbarA3>(&self, f: F) -> &Self {
        self.mpu_rbar_a3_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RBAR_A3 register."]
    #[inline] pub fn with_mpu_rbar_a3<F: FnOnce(MpuRbarA3) -> MpuRbarA3>(&self, f: F) -> &Self {
        self.mpu_rbar_a3_reg().with(f);
        self
    }

    #[doc="Get the MPU_RASR_A3 Register."]
    #[inline] pub fn mpu_rasr_a3_reg(&self) -> ::bobbin_mcu::register::Register<MpuRasrA3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut MpuRasrA3, 0x28)
    }

    #[doc="Get the *mut pointer for the MPU_RASR_A3 register."]
    #[inline] pub fn mpu_rasr_a3_mut(&self) -> *mut MpuRasrA3 { 
        self.mpu_rasr_a3_reg().ptr()
    }

    #[doc="Get the *const pointer for the MPU_RASR_A3 register."]
    #[inline] pub fn mpu_rasr_a3_ptr(&self) -> *const MpuRasrA3 { 
        self.mpu_rasr_a3_reg().ptr()
    }

    #[doc="Read the MPU_RASR_A3 register."]
    #[inline] pub fn mpu_rasr_a3(&self) -> MpuRasrA3 { 
        self.mpu_rasr_a3_reg().read()
    }

    #[doc="Write the MPU_RASR_A3 register."]
    #[inline] pub fn write_mpu_rasr_a3(&self, value: MpuRasrA3) -> &Self { 
        self.mpu_rasr_a3_reg().write(value);
        self
    }

    #[doc="Set the MPU_RASR_A3 register."]
    #[inline] pub fn set_mpu_rasr_a3<F: FnOnce(MpuRasrA3) -> MpuRasrA3>(&self, f: F) -> &Self {
        self.mpu_rasr_a3_reg().set(f);
        self
    }

    #[doc="Modify the MPU_RASR_A3 register."]
    #[inline] pub fn with_mpu_rasr_a3<F: FnOnce(MpuRasrA3) -> MpuRasrA3>(&self, f: F) -> &Self {
        self.mpu_rasr_a3_reg().with(f);
        self
    }

}

#[doc="MPU Type Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuType(pub u32);
impl MpuType {
    #[doc="Indicates the number of supported MPU instruction regions. Always contains 0x00. The MPU memory map is unified and is described by the DREGION field."]
    #[inline] pub fn iregion(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IREGION != 0"]
    #[inline] pub fn test_iregion(&self) -> bool {
        self.iregion() != 0
    }

    #[doc="Sets the IREGION field."]
    #[inline] pub fn set_iregion<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Indicates the number of supported MPU data regions: 0x08 = eight MPU regions."]
    #[inline] pub fn dregion(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DREGION != 0"]
    #[inline] pub fn test_dregion(&self) -> bool {
        self.dregion() != 0
    }

    #[doc="Sets the DREGION field."]
    #[inline] pub fn set_dregion<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Indicates support for unified or separate instruction and date memory maps: 0 = unified."]
    #[inline] pub fn separate(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SEPARATE != 0"]
    #[inline] pub fn test_separate(&self) -> bool {
        self.separate() != 0
    }

    #[doc="Sets the SEPARATE field."]
    #[inline] pub fn set_separate<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for MpuType {
    #[inline]
    fn from(other: u32) -> Self {
         MpuType(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuCtrl(pub u32);
impl MpuCtrl {
    #[doc="Enables privileged software access to the default memory map: 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault. 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses. When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map. If the MPU is disabled, the processor ignores this bit."]
    #[inline] pub fn privdefena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PRIVDEFENA != 0"]
    #[inline] pub fn test_privdefena(&self) -> bool {
        self.privdefena() != 0
    }

    #[doc="Sets the PRIVDEFENA field."]
    #[inline] pub fn set_privdefena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enables the operation of MPU during hard fault, NMI, and FAULTMASK handlers. When the MPU is enabled: 0 = MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit1 = the MPU is enabled during hard fault, NMI, and FAULTMASK handlers. When the MPU is disabled, if this bit is set to 1 the behavior is Unpredictable."]
    #[inline] pub fn hfnmiena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HFNMIENA != 0"]
    #[inline] pub fn test_hfnmiena(&self) -> bool {
        self.hfnmiena() != 0
    }

    #[doc="Sets the HFNMIENA field."]
    #[inline] pub fn set_hfnmiena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enables the MPU: 0 = MPU disabled, 1 = MPU enabled."]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for MpuCtrl {
    #[inline]
    fn from(other: u32) -> Self {
         MpuCtrl(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRnr(pub u32);
impl MpuRnr {
    #[doc="Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
    #[inline] pub fn region(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if REGION != 0"]
    #[inline] pub fn test_region(&self) -> bool {
        self.region() != 0
    }

    #[doc="Sets the REGION field."]
    #[inline] pub fn set_region<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for MpuRnr {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRnr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRbar(pub u32);
impl MpuRbar {
    #[doc="Region base address field. The value of N depends on the region size."]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U27 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffffff) as u32) } // [31:5]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U27>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U27 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffffff << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="MPU Region Number valid bit: 0 = MPU_RNR not changed, and the processor: updates the base address for the region specified in the MPU_RNR ignores the value of the REGION field
1 = the processor: updates the value of the MPU_RNR to the value of the REGION field updates the base address for the region specified in the REGION field. Always reads as zero."]
    #[inline] pub fn valid(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if VALID != 0"]
    #[inline] pub fn test_valid(&self) -> bool {
        self.valid() != 0
    }

    #[doc="Sets the VALID field."]
    #[inline] pub fn set_valid<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MPU region field: For the behavior on writes, see the description of the VALID field. On reads, returns the current region number, as specified by the RNR."]
    #[inline] pub fn region(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if REGION != 0"]
    #[inline] pub fn test_region(&self) -> bool {
        self.region() != 0
    }

    #[doc="Sets the REGION field."]
    #[inline] pub fn set_region<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for MpuRbar {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRbar(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRasr(pub u32);
impl MpuRasr {
    #[doc="Instruction access disable bit: 0 = instruction fetches enabled, 1 = instruction fetches disabled."]
    #[inline] pub fn xn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if XN != 0"]
    #[inline] pub fn test_xn(&self) -> bool {
        self.xn() != 0
    }

    #[doc="Sets the XN field."]
    #[inline] pub fn set_xn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Access permission field."]
    #[inline] pub fn ap(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if AP != 0"]
    #[inline] pub fn test_ap(&self) -> bool {
        self.ap() != 0
    }

    #[doc="Sets the AP field."]
    #[inline] pub fn set_ap<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Memory access attribute TEX"]
    #[inline] pub fn tex(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if TEX != 0"]
    #[inline] pub fn test_tex(&self) -> bool {
        self.tex() != 0
    }

    #[doc="Sets the TEX field."]
    #[inline] pub fn set_tex<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="C Bit"]
    #[inline] pub fn c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if C != 0"]
    #[inline] pub fn test_c(&self) -> bool {
        self.c() != 0
    }

    #[doc="Sets the C field."]
    #[inline] pub fn set_c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="B Bit"]
    #[inline] pub fn b(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if B != 0"]
    #[inline] pub fn test_b(&self) -> bool {
        self.b() != 0
    }

    #[doc="Sets the B field."]
    #[inline] pub fn set_b<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Sharable Bit"]
    #[inline] pub fn s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S != 0"]
    #[inline] pub fn test_s(&self) -> bool {
        self.s() != 0
    }

    #[doc="Sets the S field."]
    #[inline] pub fn set_s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Subregion disable bits. For each bit in this field: 0 = corresponding sub-region is enabled1 = corresponding sub-region is disabled. See Subregions for more information. Region sizes of 128 bytes and less do not support subregions. When writing the attributes for such a region, write the SRD field as 0x00."]
    #[inline] pub fn srd(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if SRD != 0"]
    #[inline] pub fn test_srd(&self) -> bool {
        self.srd() != 0
    }

    #[doc="Sets the SRD field."]
    #[inline] pub fn set_srd<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Specifies the size of the MPU protection region. The minimum permitted value is 3 (0b00010), see See SIZE field values for more information."]
    #[inline] pub fn size(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1f) as u8) } // [5:1]
    }

    #[doc="Returns true if SIZE != 0"]
    #[inline] pub fn test_size(&self) -> bool {
        self.size() != 0
    }

    #[doc="Sets the SIZE field."]
    #[inline] pub fn set_size<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Region enable bit."]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for MpuRasr {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRasr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRbarA1(pub u32);
impl MpuRbarA1 {
}

impl From<u32> for MpuRbarA1 {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRbarA1(other)
    }
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRasrA1(pub u32);
impl MpuRasrA1 {
}

impl From<u32> for MpuRasrA1 {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRasrA1(other)
    }
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRbarA2(pub u32);
impl MpuRbarA2 {
}

impl From<u32> for MpuRbarA2 {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRbarA2(other)
    }
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRasrA2(pub u32);
impl MpuRasrA2 {
}

impl From<u32> for MpuRasrA2 {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRasrA2(other)
    }
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRbarA3(pub u32);
impl MpuRbarA3 {
}

impl From<u32> for MpuRbarA3 {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRbarA3(other)
    }
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MpuRasrA3(pub u32);
impl MpuRasrA3 {
}

impl From<u32> for MpuRasrA3 {
    #[inline]
    fn from(other: u32) -> Self {
         MpuRasrA3(other)
    }
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

