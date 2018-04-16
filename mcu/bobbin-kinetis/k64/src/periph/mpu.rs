#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Memory protection unit"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MpuPeriph(pub usize);
impl MpuPeriph {
    #[doc="Get the CESR Register."]
    #[inline] pub fn cesr_reg(&self) -> Register<Cesr> { 
        Register::new(self.0 as *mut Cesr, 0x0)
    }

    #[doc="Get the *mut pointer for the CESR register."]
    #[inline] pub fn cesr_mut(&self) -> *mut Cesr { 
        self.cesr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CESR register."]
    #[inline] pub fn cesr_ptr(&self) -> *const Cesr { 
        self.cesr_reg().ptr()
    }

    #[doc="Read the CESR register."]
    #[inline] pub fn cesr(&self) -> Cesr { 
        self.cesr_reg().read()
    }

    #[doc="Write the CESR register."]
    #[inline] pub fn write_cesr(&self, value: Cesr) -> &Self { 
        self.cesr_reg().write(value);
        self
    }

    #[doc="Set the CESR register."]
    #[inline] pub fn set_cesr<F: FnOnce(Cesr) -> Cesr>(&self, f: F) -> &Self {
        self.cesr_reg().set(f);
        self
    }

    #[doc="Modify the CESR register."]
    #[inline] pub fn with_cesr<F: FnOnce(Cesr) -> Cesr>(&self, f: F) -> &Self {
        self.cesr_reg().with(f);
        self
    }

    #[doc="Get the EAR Register."]
    #[inline] pub fn ear_reg(&self) -> RegisterArray<Ear, bits::R5> { 
        RegisterArray::new(self.0 as *mut Ear, 0x10, 0x8)
    }

    #[doc="Get the *mut pointer for the EAR register."]
    #[inline] pub fn ear_mut<I: Into<bits::R5>>(&self, index: I) -> *mut Ear { 
        self.ear_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EAR register."]
    #[inline] pub fn ear_ptr<I: Into<bits::R5>>(&self, index: I) -> *const Ear { 
        self.ear_reg().ptr(index.into())
    }

    #[doc="Read the EAR register."]
    #[inline] pub fn ear<I: Into<bits::R5>>(&self, index: I) -> Ear { 
        self.ear_reg().read(index.into())
    }

    #[doc="Get the EDR Register."]
    #[inline] pub fn edr_reg(&self) -> RegisterArray<Edr, bits::R5> { 
        RegisterArray::new(self.0 as *mut Edr, 0x14, 0x8)
    }

    #[doc="Get the *mut pointer for the EDR register."]
    #[inline] pub fn edr_mut<I: Into<bits::R5>>(&self, index: I) -> *mut Edr { 
        self.edr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EDR register."]
    #[inline] pub fn edr_ptr<I: Into<bits::R5>>(&self, index: I) -> *const Edr { 
        self.edr_reg().ptr(index.into())
    }

    #[doc="Read the EDR register."]
    #[inline] pub fn edr<I: Into<bits::R5>>(&self, index: I) -> Edr { 
        self.edr_reg().read(index.into())
    }

    #[doc="Get the RGD_WORD0 Register."]
    #[inline] pub fn rgd_word0_reg(&self) -> RegisterArray<RgdWord0, bits::R12> { 
        RegisterArray::new(self.0 as *mut RgdWord0, 0x400, 0x10)
    }

    #[doc="Get the *mut pointer for the RGD_WORD0 register."]
    #[inline] pub fn rgd_word0_mut<I: Into<bits::R12>>(&self, index: I) -> *mut RgdWord0 { 
        self.rgd_word0_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the RGD_WORD0 register."]
    #[inline] pub fn rgd_word0_ptr<I: Into<bits::R12>>(&self, index: I) -> *const RgdWord0 { 
        self.rgd_word0_reg().ptr(index.into())
    }

    #[doc="Read the RGD_WORD0 register."]
    #[inline] pub fn rgd_word0<I: Into<bits::R12>>(&self, index: I) -> RgdWord0 { 
        self.rgd_word0_reg().read(index.into())
    }

    #[doc="Write the RGD_WORD0 register."]
    #[inline] pub fn write_rgd_word0<I: Into<bits::R12>>(&self, index: I, value: RgdWord0) -> &Self {
        self.rgd_word0_reg().write(index.into(), value);
        self
    }

    #[doc="Set the RGD_WORD0 register."]
    #[inline] pub fn set_rgd_word0<I: Into<bits::R12>, F: FnOnce(RgdWord0) -> RgdWord0>(&self, index: I, f: F) -> &Self {
        self.rgd_word0_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the RGD_WORD0 register."]
    #[inline] pub fn with_rgd_word0<I: Into<bits::R12> + Copy, F: FnOnce(RgdWord0) -> RgdWord0>(&self, index: I, f: F) -> &Self {
        self.rgd_word0_reg().with(index.into(), f);
        self
    }

    #[doc="Get the RGD_WORD1 Register."]
    #[inline] pub fn rgd_word1_reg(&self) -> RegisterArray<RgdWord1, bits::R12> { 
        RegisterArray::new(self.0 as *mut RgdWord1, 0x404, 0x10)
    }

    #[doc="Get the *mut pointer for the RGD_WORD1 register."]
    #[inline] pub fn rgd_word1_mut<I: Into<bits::R12>>(&self, index: I) -> *mut RgdWord1 { 
        self.rgd_word1_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the RGD_WORD1 register."]
    #[inline] pub fn rgd_word1_ptr<I: Into<bits::R12>>(&self, index: I) -> *const RgdWord1 { 
        self.rgd_word1_reg().ptr(index.into())
    }

    #[doc="Read the RGD_WORD1 register."]
    #[inline] pub fn rgd_word1<I: Into<bits::R12>>(&self, index: I) -> RgdWord1 { 
        self.rgd_word1_reg().read(index.into())
    }

    #[doc="Write the RGD_WORD1 register."]
    #[inline] pub fn write_rgd_word1<I: Into<bits::R12>>(&self, index: I, value: RgdWord1) -> &Self {
        self.rgd_word1_reg().write(index.into(), value);
        self
    }

    #[doc="Set the RGD_WORD1 register."]
    #[inline] pub fn set_rgd_word1<I: Into<bits::R12>, F: FnOnce(RgdWord1) -> RgdWord1>(&self, index: I, f: F) -> &Self {
        self.rgd_word1_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the RGD_WORD1 register."]
    #[inline] pub fn with_rgd_word1<I: Into<bits::R12> + Copy, F: FnOnce(RgdWord1) -> RgdWord1>(&self, index: I, f: F) -> &Self {
        self.rgd_word1_reg().with(index.into(), f);
        self
    }

    #[doc="Get the RGD_WORD2 Register."]
    #[inline] pub fn rgd_word2_reg(&self) -> RegisterArray<RgdWord2, bits::R12> { 
        RegisterArray::new(self.0 as *mut RgdWord2, 0x408, 0x10)
    }

    #[doc="Get the *mut pointer for the RGD_WORD2 register."]
    #[inline] pub fn rgd_word2_mut<I: Into<bits::R12>>(&self, index: I) -> *mut RgdWord2 { 
        self.rgd_word2_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the RGD_WORD2 register."]
    #[inline] pub fn rgd_word2_ptr<I: Into<bits::R12>>(&self, index: I) -> *const RgdWord2 { 
        self.rgd_word2_reg().ptr(index.into())
    }

    #[doc="Read the RGD_WORD2 register."]
    #[inline] pub fn rgd_word2<I: Into<bits::R12>>(&self, index: I) -> RgdWord2 { 
        self.rgd_word2_reg().read(index.into())
    }

    #[doc="Write the RGD_WORD2 register."]
    #[inline] pub fn write_rgd_word2<I: Into<bits::R12>>(&self, index: I, value: RgdWord2) -> &Self {
        self.rgd_word2_reg().write(index.into(), value);
        self
    }

    #[doc="Set the RGD_WORD2 register."]
    #[inline] pub fn set_rgd_word2<I: Into<bits::R12>, F: FnOnce(RgdWord2) -> RgdWord2>(&self, index: I, f: F) -> &Self {
        self.rgd_word2_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the RGD_WORD2 register."]
    #[inline] pub fn with_rgd_word2<I: Into<bits::R12> + Copy, F: FnOnce(RgdWord2) -> RgdWord2>(&self, index: I, f: F) -> &Self {
        self.rgd_word2_reg().with(index.into(), f);
        self
    }

    #[doc="Get the RGD_WORD3 Register."]
    #[inline] pub fn rgd_word3_reg(&self) -> RegisterArray<RgdWord3, bits::R12> { 
        RegisterArray::new(self.0 as *mut RgdWord3, 0x40c, 0x10)
    }

    #[doc="Get the *mut pointer for the RGD_WORD3 register."]
    #[inline] pub fn rgd_word3_mut<I: Into<bits::R12>>(&self, index: I) -> *mut RgdWord3 { 
        self.rgd_word3_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the RGD_WORD3 register."]
    #[inline] pub fn rgd_word3_ptr<I: Into<bits::R12>>(&self, index: I) -> *const RgdWord3 { 
        self.rgd_word3_reg().ptr(index.into())
    }

    #[doc="Read the RGD_WORD3 register."]
    #[inline] pub fn rgd_word3<I: Into<bits::R12>>(&self, index: I) -> RgdWord3 { 
        self.rgd_word3_reg().read(index.into())
    }

    #[doc="Write the RGD_WORD3 register."]
    #[inline] pub fn write_rgd_word3<I: Into<bits::R12>>(&self, index: I, value: RgdWord3) -> &Self {
        self.rgd_word3_reg().write(index.into(), value);
        self
    }

    #[doc="Set the RGD_WORD3 register."]
    #[inline] pub fn set_rgd_word3<I: Into<bits::R12>, F: FnOnce(RgdWord3) -> RgdWord3>(&self, index: I, f: F) -> &Self {
        self.rgd_word3_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the RGD_WORD3 register."]
    #[inline] pub fn with_rgd_word3<I: Into<bits::R12> + Copy, F: FnOnce(RgdWord3) -> RgdWord3>(&self, index: I, f: F) -> &Self {
        self.rgd_word3_reg().with(index.into(), f);
        self
    }

    #[doc="Get the RGDAAC Register."]
    #[inline] pub fn rgdaac_reg(&self) -> RegisterArray<Rgdaac, bits::R12> { 
        RegisterArray::new(self.0 as *mut Rgdaac, 0x800, 0x4)
    }

    #[doc="Get the *mut pointer for the RGDAAC register."]
    #[inline] pub fn rgdaac_mut<I: Into<bits::R12>>(&self, index: I) -> *mut Rgdaac { 
        self.rgdaac_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the RGDAAC register."]
    #[inline] pub fn rgdaac_ptr<I: Into<bits::R12>>(&self, index: I) -> *const Rgdaac { 
        self.rgdaac_reg().ptr(index.into())
    }

    #[doc="Read the RGDAAC register."]
    #[inline] pub fn rgdaac<I: Into<bits::R12>>(&self, index: I) -> Rgdaac { 
        self.rgdaac_reg().read(index.into())
    }

    #[doc="Write the RGDAAC register."]
    #[inline] pub fn write_rgdaac<I: Into<bits::R12>>(&self, index: I, value: Rgdaac) -> &Self {
        self.rgdaac_reg().write(index.into(), value);
        self
    }

    #[doc="Set the RGDAAC register."]
    #[inline] pub fn set_rgdaac<I: Into<bits::R12>, F: FnOnce(Rgdaac) -> Rgdaac>(&self, index: I, f: F) -> &Self {
        self.rgdaac_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the RGDAAC register."]
    #[inline] pub fn with_rgdaac<I: Into<bits::R12> + Copy, F: FnOnce(Rgdaac) -> Rgdaac>(&self, index: I, f: F) -> &Self {
        self.rgdaac_reg().with(index.into(), f);
        self
    }

}

#[doc="Control/Error Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cesr(pub u32);
impl Cesr {
    #[doc="Valid"]
    #[inline] pub fn vld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VLD != 0"]
    #[inline] pub fn test_vld(&self) -> bool {
        self.vld() != 0
    }

    #[doc="Sets the VLD field."]
    #[inline] pub fn set_vld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number Of Region Descriptors"]
    #[inline] pub fn nrgd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if NRGD != 0"]
    #[inline] pub fn test_nrgd(&self) -> bool {
        self.nrgd() != 0
    }

    #[doc="Sets the NRGD field."]
    #[inline] pub fn set_nrgd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Number Of Slave Ports"]
    #[inline] pub fn nsp(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if NSP != 0"]
    #[inline] pub fn test_nsp(&self) -> bool {
        self.nsp() != 0
    }

    #[doc="Sets the NSP field."]
    #[inline] pub fn set_nsp<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Hardware Revision Level"]
    #[inline] pub fn hrl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if HRL != 0"]
    #[inline] pub fn test_hrl(&self) -> bool {
        self.hrl() != 0
    }

    #[doc="Sets the HRL field."]
    #[inline] pub fn set_hrl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Slave Port n Error"]
    #[inline] pub fn sperr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1f) as u8) } // [31:27]
    }

    #[doc="Returns true if SPERR != 0"]
    #[inline] pub fn test_sperr(&self) -> bool {
        self.sperr() != 0
    }

    #[doc="Sets the SPERR field."]
    #[inline] pub fn set_sperr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Cesr {
    #[inline]
    fn from(other: u32) -> Self {
         Cesr(other)
    }
}

impl ::core::fmt::Display for Cesr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cesr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vld() != 0 { try!(write!(f, " vld"))}
        if self.nrgd() != 0 { try!(write!(f, " nrgd=0x{:x}", self.nrgd()))}
        if self.nsp() != 0 { try!(write!(f, " nsp=0x{:x}", self.nsp()))}
        if self.hrl() != 0 { try!(write!(f, " hrl=0x{:x}", self.hrl()))}
        if self.sperr() != 0 { try!(write!(f, " sperr=0x{:x}", self.sperr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Address Register, slave port n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ear(pub u32);
impl Ear {
    #[doc="Error Address"]
    #[inline] pub fn eaddr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if EADDR != 0"]
    #[inline] pub fn test_eaddr(&self) -> bool {
        self.eaddr() != 0
    }

    #[doc="Sets the EADDR field."]
    #[inline] pub fn set_eaddr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ear {
    #[inline]
    fn from(other: u32) -> Self {
         Ear(other)
    }
}

impl ::core::fmt::Display for Ear {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ear {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Detail Register, slave port n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Edr(pub u32);
impl Edr {
    #[doc="Error Read/Write"]
    #[inline] pub fn erw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ERW != 0"]
    #[inline] pub fn test_erw(&self) -> bool {
        self.erw() != 0
    }

    #[doc="Sets the ERW field."]
    #[inline] pub fn set_erw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error Attributes"]
    #[inline] pub fn eattr(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if EATTR != 0"]
    #[inline] pub fn test_eattr(&self) -> bool {
        self.eattr() != 0
    }

    #[doc="Sets the EATTR field."]
    #[inline] pub fn set_eattr<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error Master Number"]
    #[inline] pub fn emn(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EMN != 0"]
    #[inline] pub fn test_emn(&self) -> bool {
        self.emn() != 0
    }

    #[doc="Sets the EMN field."]
    #[inline] pub fn set_emn<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Error Process Identification"]
    #[inline] pub fn epid(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if EPID != 0"]
    #[inline] pub fn test_epid(&self) -> bool {
        self.epid() != 0
    }

    #[doc="Sets the EPID field."]
    #[inline] pub fn set_epid<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Error Access Control Detail"]
    #[inline] pub fn eacd(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if EACD != 0"]
    #[inline] pub fn test_eacd(&self) -> bool {
        self.eacd() != 0
    }

    #[doc="Sets the EACD field."]
    #[inline] pub fn set_eacd<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Edr {
    #[inline]
    fn from(other: u32) -> Self {
         Edr(other)
    }
}

impl ::core::fmt::Display for Edr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Edr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.erw() != 0 { try!(write!(f, " erw"))}
        if self.eattr() != 0 { try!(write!(f, " eattr=0x{:x}", self.eattr()))}
        if self.emn() != 0 { try!(write!(f, " emn=0x{:x}", self.emn()))}
        if self.epid() != 0 { try!(write!(f, " epid=0x{:x}", self.epid()))}
        if self.eacd() != 0 { try!(write!(f, " eacd=0x{:x}", self.eacd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Region Descriptor n, Word 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RgdWord0(pub u32);
impl RgdWord0 {
    #[doc="Start Address"]
    #[inline] pub fn srtaddr(&self) -> bits::U27 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffffff) as u32) } // [31:5]
    }

    #[doc="Returns true if SRTADDR != 0"]
    #[inline] pub fn test_srtaddr(&self) -> bool {
        self.srtaddr() != 0
    }

    #[doc="Sets the SRTADDR field."]
    #[inline] pub fn set_srtaddr<V: Into<bits::U27>>(mut self, value: V) -> Self {
        let value: bits::U27 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffffff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for RgdWord0 {
    #[inline]
    fn from(other: u32) -> Self {
         RgdWord0(other)
    }
}

impl ::core::fmt::Display for RgdWord0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RgdWord0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.srtaddr() != 0 { try!(write!(f, " srtaddr=0x{:x}", self.srtaddr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Region Descriptor n, Word 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RgdWord1(pub u32);
impl RgdWord1 {
    #[doc="End Address"]
    #[inline] pub fn endaddr(&self) -> bits::U27 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffffff) as u32) } // [31:5]
    }

    #[doc="Returns true if ENDADDR != 0"]
    #[inline] pub fn test_endaddr(&self) -> bool {
        self.endaddr() != 0
    }

    #[doc="Sets the ENDADDR field."]
    #[inline] pub fn set_endaddr<V: Into<bits::U27>>(mut self, value: V) -> Self {
        let value: bits::U27 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffffff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for RgdWord1 {
    #[inline]
    fn from(other: u32) -> Self {
         RgdWord1(other)
    }
}

impl ::core::fmt::Display for RgdWord1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RgdWord1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.endaddr() != 0 { try!(write!(f, " endaddr=0x{:x}", self.endaddr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Region Descriptor n, Word 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RgdWord2(pub u32);
impl RgdWord2 {
    #[doc="Bus Master 0 User Mode Access Control"]
    #[inline] pub fn m0um(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if M0UM != 0"]
    #[inline] pub fn test_m0um(&self) -> bool {
        self.m0um() != 0
    }

    #[doc="Sets the M0UM field."]
    #[inline] pub fn set_m0um<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bus Master 0 Supervisor Mode Access Control"]
    #[inline] pub fn m0sm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if M0SM != 0"]
    #[inline] pub fn test_m0sm(&self) -> bool {
        self.m0sm() != 0
    }

    #[doc="Sets the M0SM field."]
    #[inline] pub fn set_m0sm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Bus Master 0 Process Identifier enable"]
    #[inline] pub fn m0pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if M0PE != 0"]
    #[inline] pub fn test_m0pe(&self) -> bool {
        self.m0pe() != 0
    }

    #[doc="Sets the M0PE field."]
    #[inline] pub fn set_m0pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Bus Master 1 User Mode Access Control"]
    #[inline] pub fn m1um(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if M1UM != 0"]
    #[inline] pub fn test_m1um(&self) -> bool {
        self.m1um() != 0
    }

    #[doc="Sets the M1UM field."]
    #[inline] pub fn set_m1um<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Bus Master 1 Supervisor Mode Access Control"]
    #[inline] pub fn m1sm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if M1SM != 0"]
    #[inline] pub fn test_m1sm(&self) -> bool {
        self.m1sm() != 0
    }

    #[doc="Sets the M1SM field."]
    #[inline] pub fn set_m1sm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Bus Master 1 Process Identifier enable"]
    #[inline] pub fn m1pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if M1PE != 0"]
    #[inline] pub fn test_m1pe(&self) -> bool {
        self.m1pe() != 0
    }

    #[doc="Sets the M1PE field."]
    #[inline] pub fn set_m1pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Bus Master 2 User Mode Access control"]
    #[inline] pub fn m2um(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if M2UM != 0"]
    #[inline] pub fn test_m2um(&self) -> bool {
        self.m2um() != 0
    }

    #[doc="Sets the M2UM field."]
    #[inline] pub fn set_m2um<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Bus Master 2 Supervisor Mode Access Control"]
    #[inline] pub fn m2sm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x3) as u8) } // [16:15]
    }

    #[doc="Returns true if M2SM != 0"]
    #[inline] pub fn test_m2sm(&self) -> bool {
        self.m2sm() != 0
    }

    #[doc="Sets the M2SM field."]
    #[inline] pub fn set_m2sm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Bus Master 2 Process Identifier Enable"]
    #[inline] pub fn m2pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if M2PE != 0"]
    #[inline] pub fn test_m2pe(&self) -> bool {
        self.m2pe() != 0
    }

    #[doc="Sets the M2PE field."]
    #[inline] pub fn set_m2pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Bus Master 3 User Mode Access Control"]
    #[inline] pub fn m3um(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7) as u8) } // [20:18]
    }

    #[doc="Returns true if M3UM != 0"]
    #[inline] pub fn test_m3um(&self) -> bool {
        self.m3um() != 0
    }

    #[doc="Sets the M3UM field."]
    #[inline] pub fn set_m3um<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Bus Master 3 Supervisor Mode Access Control"]
    #[inline] pub fn m3sm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if M3SM != 0"]
    #[inline] pub fn test_m3sm(&self) -> bool {
        self.m3sm() != 0
    }

    #[doc="Sets the M3SM field."]
    #[inline] pub fn set_m3sm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Bus Master 3 Process Identifier Enable"]
    #[inline] pub fn m3pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if M3PE != 0"]
    #[inline] pub fn test_m3pe(&self) -> bool {
        self.m3pe() != 0
    }

    #[doc="Sets the M3PE field."]
    #[inline] pub fn set_m3pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Bus Master 4 Write Enable"]
    #[inline] pub fn m4we(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if M4WE != 0"]
    #[inline] pub fn test_m4we(&self) -> bool {
        self.m4we() != 0
    }

    #[doc="Sets the M4WE field."]
    #[inline] pub fn set_m4we<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Bus Master 4 Read Enable"]
    #[inline] pub fn m4re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if M4RE != 0"]
    #[inline] pub fn test_m4re(&self) -> bool {
        self.m4re() != 0
    }

    #[doc="Sets the M4RE field."]
    #[inline] pub fn set_m4re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Bus Master 5 Write Enable"]
    #[inline] pub fn m5we(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if M5WE != 0"]
    #[inline] pub fn test_m5we(&self) -> bool {
        self.m5we() != 0
    }

    #[doc="Sets the M5WE field."]
    #[inline] pub fn set_m5we<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Bus Master 5 Read Enable"]
    #[inline] pub fn m5re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if M5RE != 0"]
    #[inline] pub fn test_m5re(&self) -> bool {
        self.m5re() != 0
    }

    #[doc="Sets the M5RE field."]
    #[inline] pub fn set_m5re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Bus Master 6 Write Enable"]
    #[inline] pub fn m6we(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if M6WE != 0"]
    #[inline] pub fn test_m6we(&self) -> bool {
        self.m6we() != 0
    }

    #[doc="Sets the M6WE field."]
    #[inline] pub fn set_m6we<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Bus Master 6 Read Enable"]
    #[inline] pub fn m6re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if M6RE != 0"]
    #[inline] pub fn test_m6re(&self) -> bool {
        self.m6re() != 0
    }

    #[doc="Sets the M6RE field."]
    #[inline] pub fn set_m6re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Bus Master 7 Write Enable"]
    #[inline] pub fn m7we(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if M7WE != 0"]
    #[inline] pub fn test_m7we(&self) -> bool {
        self.m7we() != 0
    }

    #[doc="Sets the M7WE field."]
    #[inline] pub fn set_m7we<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Bus Master 7 Read Enable"]
    #[inline] pub fn m7re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if M7RE != 0"]
    #[inline] pub fn test_m7re(&self) -> bool {
        self.m7re() != 0
    }

    #[doc="Sets the M7RE field."]
    #[inline] pub fn set_m7re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for RgdWord2 {
    #[inline]
    fn from(other: u32) -> Self {
         RgdWord2(other)
    }
}

impl ::core::fmt::Display for RgdWord2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RgdWord2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.m0um() != 0 { try!(write!(f, " m0um=0x{:x}", self.m0um()))}
        if self.m0sm() != 0 { try!(write!(f, " m0sm=0x{:x}", self.m0sm()))}
        if self.m0pe() != 0 { try!(write!(f, " m0pe"))}
        if self.m1um() != 0 { try!(write!(f, " m1um=0x{:x}", self.m1um()))}
        if self.m1sm() != 0 { try!(write!(f, " m1sm=0x{:x}", self.m1sm()))}
        if self.m1pe() != 0 { try!(write!(f, " m1pe"))}
        if self.m2um() != 0 { try!(write!(f, " m2um=0x{:x}", self.m2um()))}
        if self.m2sm() != 0 { try!(write!(f, " m2sm=0x{:x}", self.m2sm()))}
        if self.m2pe() != 0 { try!(write!(f, " m2pe"))}
        if self.m3um() != 0 { try!(write!(f, " m3um=0x{:x}", self.m3um()))}
        if self.m3sm() != 0 { try!(write!(f, " m3sm=0x{:x}", self.m3sm()))}
        if self.m3pe() != 0 { try!(write!(f, " m3pe"))}
        if self.m4we() != 0 { try!(write!(f, " m4we"))}
        if self.m4re() != 0 { try!(write!(f, " m4re"))}
        if self.m5we() != 0 { try!(write!(f, " m5we"))}
        if self.m5re() != 0 { try!(write!(f, " m5re"))}
        if self.m6we() != 0 { try!(write!(f, " m6we"))}
        if self.m6re() != 0 { try!(write!(f, " m6re"))}
        if self.m7we() != 0 { try!(write!(f, " m7we"))}
        if self.m7re() != 0 { try!(write!(f, " m7re"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Region Descriptor n, Word 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RgdWord3(pub u32);
impl RgdWord3 {
    #[doc="Valid"]
    #[inline] pub fn vld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VLD != 0"]
    #[inline] pub fn test_vld(&self) -> bool {
        self.vld() != 0
    }

    #[doc="Sets the VLD field."]
    #[inline] pub fn set_vld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Process Identifier Mask"]
    #[inline] pub fn pidmask(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if PIDMASK != 0"]
    #[inline] pub fn test_pidmask(&self) -> bool {
        self.pidmask() != 0
    }

    #[doc="Sets the PIDMASK field."]
    #[inline] pub fn set_pidmask<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Process Identifier"]
    #[inline] pub fn pid(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if PID != 0"]
    #[inline] pub fn test_pid(&self) -> bool {
        self.pid() != 0
    }

    #[doc="Sets the PID field."]
    #[inline] pub fn set_pid<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for RgdWord3 {
    #[inline]
    fn from(other: u32) -> Self {
         RgdWord3(other)
    }
}

impl ::core::fmt::Display for RgdWord3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RgdWord3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vld() != 0 { try!(write!(f, " vld"))}
        if self.pidmask() != 0 { try!(write!(f, " pidmask=0x{:x}", self.pidmask()))}
        if self.pid() != 0 { try!(write!(f, " pid=0x{:x}", self.pid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Region Descriptor Alternate Access Control n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgdaac(pub u32);
impl Rgdaac {
    #[doc="Bus Master 0 User Mode Access Control"]
    #[inline] pub fn m0um(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if M0UM != 0"]
    #[inline] pub fn test_m0um(&self) -> bool {
        self.m0um() != 0
    }

    #[doc="Sets the M0UM field."]
    #[inline] pub fn set_m0um<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bus Master 0 Supervisor Mode Access Control"]
    #[inline] pub fn m0sm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if M0SM != 0"]
    #[inline] pub fn test_m0sm(&self) -> bool {
        self.m0sm() != 0
    }

    #[doc="Sets the M0SM field."]
    #[inline] pub fn set_m0sm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Bus Master 0 Process Identifier Enable"]
    #[inline] pub fn m0pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if M0PE != 0"]
    #[inline] pub fn test_m0pe(&self) -> bool {
        self.m0pe() != 0
    }

    #[doc="Sets the M0PE field."]
    #[inline] pub fn set_m0pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Bus Master 1 User Mode Access Control"]
    #[inline] pub fn m1um(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if M1UM != 0"]
    #[inline] pub fn test_m1um(&self) -> bool {
        self.m1um() != 0
    }

    #[doc="Sets the M1UM field."]
    #[inline] pub fn set_m1um<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Bus Master 1 Supervisor Mode Access Control"]
    #[inline] pub fn m1sm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if M1SM != 0"]
    #[inline] pub fn test_m1sm(&self) -> bool {
        self.m1sm() != 0
    }

    #[doc="Sets the M1SM field."]
    #[inline] pub fn set_m1sm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Bus Master 1 Process Identifier Enable"]
    #[inline] pub fn m1pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if M1PE != 0"]
    #[inline] pub fn test_m1pe(&self) -> bool {
        self.m1pe() != 0
    }

    #[doc="Sets the M1PE field."]
    #[inline] pub fn set_m1pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Bus Master 2 User Mode Access Control"]
    #[inline] pub fn m2um(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if M2UM != 0"]
    #[inline] pub fn test_m2um(&self) -> bool {
        self.m2um() != 0
    }

    #[doc="Sets the M2UM field."]
    #[inline] pub fn set_m2um<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Bus Master 2 Supervisor Mode Access Control"]
    #[inline] pub fn m2sm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x3) as u8) } // [16:15]
    }

    #[doc="Returns true if M2SM != 0"]
    #[inline] pub fn test_m2sm(&self) -> bool {
        self.m2sm() != 0
    }

    #[doc="Sets the M2SM field."]
    #[inline] pub fn set_m2sm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Bus Master 2 Process Identifier Enable"]
    #[inline] pub fn m2pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if M2PE != 0"]
    #[inline] pub fn test_m2pe(&self) -> bool {
        self.m2pe() != 0
    }

    #[doc="Sets the M2PE field."]
    #[inline] pub fn set_m2pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Bus Master 3 User Mode Access Control"]
    #[inline] pub fn m3um(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7) as u8) } // [20:18]
    }

    #[doc="Returns true if M3UM != 0"]
    #[inline] pub fn test_m3um(&self) -> bool {
        self.m3um() != 0
    }

    #[doc="Sets the M3UM field."]
    #[inline] pub fn set_m3um<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Bus Master 3 Supervisor Mode Access Control"]
    #[inline] pub fn m3sm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if M3SM != 0"]
    #[inline] pub fn test_m3sm(&self) -> bool {
        self.m3sm() != 0
    }

    #[doc="Sets the M3SM field."]
    #[inline] pub fn set_m3sm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Bus Master 3 Process Identifier Enable"]
    #[inline] pub fn m3pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if M3PE != 0"]
    #[inline] pub fn test_m3pe(&self) -> bool {
        self.m3pe() != 0
    }

    #[doc="Sets the M3PE field."]
    #[inline] pub fn set_m3pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Bus Master 4 Write Enable"]
    #[inline] pub fn m4we(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if M4WE != 0"]
    #[inline] pub fn test_m4we(&self) -> bool {
        self.m4we() != 0
    }

    #[doc="Sets the M4WE field."]
    #[inline] pub fn set_m4we<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Bus Master 4 Read Enable"]
    #[inline] pub fn m4re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if M4RE != 0"]
    #[inline] pub fn test_m4re(&self) -> bool {
        self.m4re() != 0
    }

    #[doc="Sets the M4RE field."]
    #[inline] pub fn set_m4re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Bus Master 5 Write Enable"]
    #[inline] pub fn m5we(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if M5WE != 0"]
    #[inline] pub fn test_m5we(&self) -> bool {
        self.m5we() != 0
    }

    #[doc="Sets the M5WE field."]
    #[inline] pub fn set_m5we<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Bus Master 5 Read Enable"]
    #[inline] pub fn m5re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if M5RE != 0"]
    #[inline] pub fn test_m5re(&self) -> bool {
        self.m5re() != 0
    }

    #[doc="Sets the M5RE field."]
    #[inline] pub fn set_m5re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Bus Master 6 Write Enable"]
    #[inline] pub fn m6we(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if M6WE != 0"]
    #[inline] pub fn test_m6we(&self) -> bool {
        self.m6we() != 0
    }

    #[doc="Sets the M6WE field."]
    #[inline] pub fn set_m6we<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Bus Master 6 Read Enable"]
    #[inline] pub fn m6re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if M6RE != 0"]
    #[inline] pub fn test_m6re(&self) -> bool {
        self.m6re() != 0
    }

    #[doc="Sets the M6RE field."]
    #[inline] pub fn set_m6re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Bus Master 7 Write Enable"]
    #[inline] pub fn m7we(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if M7WE != 0"]
    #[inline] pub fn test_m7we(&self) -> bool {
        self.m7we() != 0
    }

    #[doc="Sets the M7WE field."]
    #[inline] pub fn set_m7we<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Bus Master 7 Read Enable"]
    #[inline] pub fn m7re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if M7RE != 0"]
    #[inline] pub fn test_m7re(&self) -> bool {
        self.m7re() != 0
    }

    #[doc="Sets the M7RE field."]
    #[inline] pub fn set_m7re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Rgdaac {
    #[inline]
    fn from(other: u32) -> Self {
         Rgdaac(other)
    }
}

impl ::core::fmt::Display for Rgdaac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rgdaac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.m0um() != 0 { try!(write!(f, " m0um=0x{:x}", self.m0um()))}
        if self.m0sm() != 0 { try!(write!(f, " m0sm=0x{:x}", self.m0sm()))}
        if self.m0pe() != 0 { try!(write!(f, " m0pe"))}
        if self.m1um() != 0 { try!(write!(f, " m1um=0x{:x}", self.m1um()))}
        if self.m1sm() != 0 { try!(write!(f, " m1sm=0x{:x}", self.m1sm()))}
        if self.m1pe() != 0 { try!(write!(f, " m1pe"))}
        if self.m2um() != 0 { try!(write!(f, " m2um=0x{:x}", self.m2um()))}
        if self.m2sm() != 0 { try!(write!(f, " m2sm=0x{:x}", self.m2sm()))}
        if self.m2pe() != 0 { try!(write!(f, " m2pe"))}
        if self.m3um() != 0 { try!(write!(f, " m3um=0x{:x}", self.m3um()))}
        if self.m3sm() != 0 { try!(write!(f, " m3sm=0x{:x}", self.m3sm()))}
        if self.m3pe() != 0 { try!(write!(f, " m3pe"))}
        if self.m4we() != 0 { try!(write!(f, " m4we"))}
        if self.m4re() != 0 { try!(write!(f, " m4re"))}
        if self.m5we() != 0 { try!(write!(f, " m5we"))}
        if self.m5re() != 0 { try!(write!(f, " m5re"))}
        if self.m6we() != 0 { try!(write!(f, " m6we"))}
        if self.m6re() != 0 { try!(write!(f, " m6re"))}
        if self.m7we() != 0 { try!(write!(f, " m7we"))}
        if self.m7re() != 0 { try!(write!(f, " m7re"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

