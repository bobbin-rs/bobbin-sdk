
::bobbin_mcu::periph!( ITM, Itm, ITM_PERIPH, ItmPeriph, ITM_OWNED, ITM_REF_COUNT, 0xe0000000, 0x00, 0x06);


#[doc="ITM"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ItmPeriph(pub usize);
impl ItmPeriph {
    #[doc="Get the STIM Register."]
    #[inline] pub fn stim_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Stim, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Stim, 0x0, 0x4)
    }

    #[doc="Get the *mut pointer for the STIM register."]
    #[inline] pub fn stim_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Stim { 
        self.stim_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the STIM register."]
    #[inline] pub fn stim_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Stim { 
        self.stim_reg().ptr(index.into())
    }

    #[doc="Read the STIM register."]
    #[inline] pub fn stim<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Stim { 
        self.stim_reg().read(index.into())
    }

    #[doc="Write the STIM register."]
    #[inline] pub fn write_stim<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Stim) -> &Self {
        self.stim_reg().write(index.into(), value);
        self
    }

    #[doc="Set the STIM register."]
    #[inline] pub fn set_stim<I: Into<::bobbin_bits::R32>, F: FnOnce(Stim) -> Stim>(&self, index: I, f: F) -> &Self {
        self.stim_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the STIM register."]
    #[inline] pub fn with_stim<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Stim) -> Stim>(&self, index: I, f: F) -> &Self {
        self.stim_reg().with(index.into(), f);
        self
    }

    #[doc="Get the STIM16 Register."]
    #[inline] pub fn stim16_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Stim16, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Stim16, 0x0, 0x4)
    }

    #[doc="Get the *mut pointer for the STIM16 register."]
    #[inline] pub fn stim16_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Stim16 { 
        self.stim16_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the STIM16 register."]
    #[inline] pub fn stim16_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Stim16 { 
        self.stim16_reg().ptr(index.into())
    }

    #[doc="Read the STIM16 register."]
    #[inline] pub fn stim16<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Stim16 { 
        self.stim16_reg().read(index.into())
    }

    #[doc="Write the STIM16 register."]
    #[inline] pub fn write_stim16<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Stim16) -> &Self {
        self.stim16_reg().write(index.into(), value);
        self
    }

    #[doc="Set the STIM16 register."]
    #[inline] pub fn set_stim16<I: Into<::bobbin_bits::R32>, F: FnOnce(Stim16) -> Stim16>(&self, index: I, f: F) -> &Self {
        self.stim16_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the STIM16 register."]
    #[inline] pub fn with_stim16<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Stim16) -> Stim16>(&self, index: I, f: F) -> &Self {
        self.stim16_reg().with(index.into(), f);
        self
    }

    #[doc="Get the STIM8 Register."]
    #[inline] pub fn stim8_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Stim8, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Stim8, 0x0, 0x4)
    }

    #[doc="Get the *mut pointer for the STIM8 register."]
    #[inline] pub fn stim8_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Stim8 { 
        self.stim8_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the STIM8 register."]
    #[inline] pub fn stim8_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Stim8 { 
        self.stim8_reg().ptr(index.into())
    }

    #[doc="Read the STIM8 register."]
    #[inline] pub fn stim8<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Stim8 { 
        self.stim8_reg().read(index.into())
    }

    #[doc="Write the STIM8 register."]
    #[inline] pub fn write_stim8<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Stim8) -> &Self {
        self.stim8_reg().write(index.into(), value);
        self
    }

    #[doc="Set the STIM8 register."]
    #[inline] pub fn set_stim8<I: Into<::bobbin_bits::R32>, F: FnOnce(Stim8) -> Stim8>(&self, index: I, f: F) -> &Self {
        self.stim8_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the STIM8 register."]
    #[inline] pub fn with_stim8<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Stim8) -> Stim8>(&self, index: I, f: F) -> &Self {
        self.stim8_reg().with(index.into(), f);
        self
    }

    #[doc="Get the TER Register."]
    #[inline] pub fn ter_reg(&self) -> ::bobbin_mcu::register::Register<Ter> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ter, 0xe00)
    }

    #[doc="Get the *mut pointer for the TER register."]
    #[inline] pub fn ter_mut(&self) -> *mut Ter { 
        self.ter_reg().ptr()
    }

    #[doc="Get the *const pointer for the TER register."]
    #[inline] pub fn ter_ptr(&self) -> *const Ter { 
        self.ter_reg().ptr()
    }

    #[doc="Read the TER register."]
    #[inline] pub fn ter(&self) -> Ter { 
        self.ter_reg().read()
    }

    #[doc="Write the TER register."]
    #[inline] pub fn write_ter(&self, value: Ter) -> &Self { 
        self.ter_reg().write(value);
        self
    }

    #[doc="Set the TER register."]
    #[inline] pub fn set_ter<F: FnOnce(Ter) -> Ter>(&self, f: F) -> &Self {
        self.ter_reg().set(f);
        self
    }

    #[doc="Modify the TER register."]
    #[inline] pub fn with_ter<F: FnOnce(Ter) -> Ter>(&self, f: F) -> &Self {
        self.ter_reg().with(f);
        self
    }

    #[doc="Get the TPR Register."]
    #[inline] pub fn tpr_reg(&self) -> ::bobbin_mcu::register::Register<Tpr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tpr, 0xe40)
    }

    #[doc="Get the *mut pointer for the TPR register."]
    #[inline] pub fn tpr_mut(&self) -> *mut Tpr { 
        self.tpr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TPR register."]
    #[inline] pub fn tpr_ptr(&self) -> *const Tpr { 
        self.tpr_reg().ptr()
    }

    #[doc="Read the TPR register."]
    #[inline] pub fn tpr(&self) -> Tpr { 
        self.tpr_reg().read()
    }

    #[doc="Write the TPR register."]
    #[inline] pub fn write_tpr(&self, value: Tpr) -> &Self { 
        self.tpr_reg().write(value);
        self
    }

    #[doc="Set the TPR register."]
    #[inline] pub fn set_tpr<F: FnOnce(Tpr) -> Tpr>(&self, f: F) -> &Self {
        self.tpr_reg().set(f);
        self
    }

    #[doc="Modify the TPR register."]
    #[inline] pub fn with_tpr<F: FnOnce(Tpr) -> Tpr>(&self, f: F) -> &Self {
        self.tpr_reg().with(f);
        self
    }

    #[doc="Get the TCR Register."]
    #[inline] pub fn tcr_reg(&self) -> ::bobbin_mcu::register::Register<Tcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tcr, 0xe80)
    }

    #[doc="Get the *mut pointer for the TCR register."]
    #[inline] pub fn tcr_mut(&self) -> *mut Tcr { 
        self.tcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TCR register."]
    #[inline] pub fn tcr_ptr(&self) -> *const Tcr { 
        self.tcr_reg().ptr()
    }

    #[doc="Read the TCR register."]
    #[inline] pub fn tcr(&self) -> Tcr { 
        self.tcr_reg().read()
    }

    #[doc="Write the TCR register."]
    #[inline] pub fn write_tcr(&self, value: Tcr) -> &Self { 
        self.tcr_reg().write(value);
        self
    }

    #[doc="Set the TCR register."]
    #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        self.tcr_reg().set(f);
        self
    }

    #[doc="Modify the TCR register."]
    #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        self.tcr_reg().with(f);
        self
    }

    #[doc="Get the LAR Register."]
    #[inline] pub fn lar_reg(&self) -> ::bobbin_mcu::register::Register<Lar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Lar, 0xfb0)
    }

    #[doc="Get the *mut pointer for the LAR register."]
    #[inline] pub fn lar_mut(&self) -> *mut Lar { 
        self.lar_reg().ptr()
    }

    #[doc="Get the *const pointer for the LAR register."]
    #[inline] pub fn lar_ptr(&self) -> *const Lar { 
        self.lar_reg().ptr()
    }

    #[doc="Write the LAR register."]
    #[inline] pub fn write_lar(&self, value: Lar) -> &Self { 
        self.lar_reg().write(value);
        self
    }

    #[doc="Set the LAR register."]
    #[inline] pub fn set_lar<F: FnOnce(Lar) -> Lar>(&self, f: F) -> &Self {
        self.lar_reg().set(f);
        self
    }

    #[doc="Get the LSR Register."]
    #[inline] pub fn lsr_reg(&self) -> ::bobbin_mcu::register::Register<Lsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Lsr, 0xfb4)
    }

    #[doc="Get the *mut pointer for the LSR register."]
    #[inline] pub fn lsr_mut(&self) -> *mut Lsr { 
        self.lsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the LSR register."]
    #[inline] pub fn lsr_ptr(&self) -> *const Lsr { 
        self.lsr_reg().ptr()
    }

    #[doc="Read the LSR register."]
    #[inline] pub fn lsr(&self) -> Lsr { 
        self.lsr_reg().read()
    }

}

#[doc="ITM Stimulus Port"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stim(pub u32);
impl Stim {
    #[doc="Gets the DATA field."]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stim {
    #[inline]
    fn from(other: u32) -> Self {
         Stim(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stim16(pub u16);
impl Stim16 {
    #[doc="Gets the DATA field."]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Stim16 {
    #[inline]
    fn from(other: u16) -> Self {
         Stim16(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stim8(pub u8);
impl Stim8 {
    #[doc="Gets the DATA field."]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Stim8 {
    #[inline]
    fn from(other: u8) -> Self {
         Stim8(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ter(pub u32);
impl Ter {
    #[doc="Bit mask to enable tracing on ITM stimulus ports. One bit per stimulus port."]
    #[inline] pub fn ena<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENA != 0"]
    #[inline] pub fn test_ena<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.ena(index) != 0
    }

    #[doc="Sets the ENA field."]
    #[inline] pub fn set_ena<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ter {
    #[inline]
    fn from(other: u32) -> Self {
         Ter(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tpr(pub u32);
impl Tpr {
    #[doc="Bit mask to enable tracing on ITM stimulus ports: bit [0] = stimulus ports [7:0], bit [1] = stimulus ports [15:8], bit [2] = stimulus ports [23:16], bit [3] = stimulus ports [31:24]"]
    #[inline] pub fn tpr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TPR != 0"]
    #[inline] pub fn test_tpr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.tpr(index) != 0
    }

    #[doc="Sets the TPR field."]
    #[inline] pub fn set_tpr<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Tpr {
    #[inline]
    fn from(other: u32) -> Self {
         Tpr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc="Set when ITM events present and being drained"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="ATB ID for CoreSight System"]
    #[inline] pub fn atbid(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Returns true if ATBID != 0"]
    #[inline] pub fn test_atbid(&self) -> bool {
        self.atbid() != 0
    }

    #[doc="Sets the ATBID field."]
    #[inline] pub fn set_atbid<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Timestamp prescaler: 0b00 = no prescaling, 0b01 = divide by 4, 0b10 = divide by 16, 0b11 = divide by 64."]
    #[inline] pub fn tsprescale(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if TSPRESCALE != 0"]
    #[inline] pub fn test_tsprescale(&self) -> bool {
        self.tsprescale() != 0
    }

    #[doc="Sets the TSPRESCALE field."]
    #[inline] pub fn set_tsprescale<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enable SWV behavior – count on TPIUEMIT and TPIUBAUD."]
    #[inline] pub fn swoena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SWOENA != 0"]
    #[inline] pub fn test_swoena(&self) -> bool {
        self.swoena() != 0
    }

    #[doc="Sets the SWOENA field."]
    #[inline] pub fn set_swoena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enables the DWT stimulus."]
    #[inline] pub fn dwtena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DWTENA != 0"]
    #[inline] pub fn test_dwtena(&self) -> bool {
        self.dwtena() != 0
    }

    #[doc="Sets the DWTENA field."]
    #[inline] pub fn set_dwtena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enables sync packets for TPIU."]
    #[inline] pub fn syncena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SYNCENA != 0"]
    #[inline] pub fn test_syncena(&self) -> bool {
        self.syncena() != 0
    }

    #[doc="Sets the SYNCENA field."]
    #[inline] pub fn set_syncena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of cycles. This provides a time reference for packets and inter-packet gaps."]
    #[inline] pub fn tsena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TSENA != 0"]
    #[inline] pub fn test_tsena(&self) -> bool {
        self.tsena() != 0
    }

    #[doc="Sets the TSENA field."]
    #[inline] pub fn set_tsena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline] pub fn itmena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ITMENA != 0"]
    #[inline] pub fn test_itmena(&self) -> bool {
        self.itmena() != 0
    }

    #[doc="Sets the ITMENA field."]
    #[inline] pub fn set_itmena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tcr {
    #[inline]
    fn from(other: u32) -> Self {
         Tcr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lar(pub u32);
impl Lar {
    #[doc="A privileged write of 0xC5ACCE55 enables more write access to Control Register 0xE00::0xFFC. An invalid write removes write access."]
    #[inline] pub fn access(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ACCESS != 0"]
    #[inline] pub fn test_access(&self) -> bool {
        self.access() != 0
    }

    #[doc="Sets the ACCESS field."]
    #[inline] pub fn set_access<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lar {
    #[inline]
    fn from(other: u32) -> Self {
         Lar(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lsr(pub u32);
impl Lsr {
    #[doc="You cannot implement 8-bit lock accesses."]
    #[inline] pub fn byteacc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BYTEACC != 0"]
    #[inline] pub fn test_byteacc(&self) -> bool {
        self.byteacc() != 0
    }

    #[doc="Sets the BYTEACC field."]
    #[inline] pub fn set_byteacc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Write access to component is blocked. All writes are ignored, reads are permitted."]
    #[inline] pub fn access(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ACCESS != 0"]
    #[inline] pub fn test_access(&self) -> bool {
        self.access() != 0
    }

    #[doc="Sets the ACCESS field."]
    #[inline] pub fn set_access<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Indicates that a lock mechanism exists for this component."]
    #[inline] pub fn present(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PRESENT != 0"]
    #[inline] pub fn test_present(&self) -> bool {
        self.present() != 0
    }

    #[doc="Sets the PRESENT field."]
    #[inline] pub fn set_present<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lsr {
    #[inline]
    fn from(other: u32) -> Self {
         Lsr(other)
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

