::bobbin_mcu::periph!( GCLK, Gclk, GCLK_PERIPH, GclkPeriph, GCLK_OWNED, GCLK_REF_COUNT, 0x40001c00, 0x00, 0x0d);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GCLK Peripheral"]
pub struct GclkPeriph(pub usize); 

impl GclkPeriph {
    #[doc="Get the CTRLA Register."]
    #[inline] pub fn ctrla_reg(&self) -> ::bobbin_mcu::register::Register<Ctrla> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrla, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        self.ctrla_reg().read()
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn write_ctrla(&self, value: Ctrla) -> &Self { 
        self.ctrla_reg().write(value);
        self
    }

    #[doc="Set the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().set(f);
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().with(f);
        self
    }

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x4)
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        self.syncbusy_reg().read()
    }

    #[doc="Get the GENCTRL Register."]
    #[inline] pub fn genctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Genctrl, ::bobbin_bits::R12> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Genctrl, 0x20, 0x4)
    }

    #[doc="Get the *mut pointer for the GENCTRL register."]
    #[inline] pub fn genctrl_mut<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *mut Genctrl { 
        self.genctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the GENCTRL register."]
    #[inline] pub fn genctrl_ptr<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *const Genctrl { 
        self.genctrl_reg().ptr(index.into())
    }

    #[doc="Read the GENCTRL register."]
    #[inline] pub fn genctrl<I: Into<::bobbin_bits::R12>>(&self, index: I) -> Genctrl { 
        self.genctrl_reg().read(index.into())
    }

    #[doc="Write the GENCTRL register."]
    #[inline] pub fn write_genctrl<I: Into<::bobbin_bits::R12>>(&self, index: I, value: Genctrl) -> &Self {
        self.genctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the GENCTRL register."]
    #[inline] pub fn set_genctrl<I: Into<::bobbin_bits::R12>, F: FnOnce(Genctrl) -> Genctrl>(&self, index: I, f: F) -> &Self {
        self.genctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the GENCTRL register."]
    #[inline] pub fn with_genctrl<I: Into<::bobbin_bits::R12> + Copy, F: FnOnce(Genctrl) -> Genctrl>(&self, index: I, f: F) -> &Self {
        self.genctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PCHCTRL Register."]
    #[inline] pub fn pchctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pchctrl, ::bobbin_bits::U6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pchctrl, 0x80, 0x4)
    }

    #[doc="Get the *mut pointer for the PCHCTRL register."]
    #[inline] pub fn pchctrl_mut<I: Into<::bobbin_bits::U6>>(&self, index: I) -> *mut Pchctrl { 
        self.pchctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PCHCTRL register."]
    #[inline] pub fn pchctrl_ptr<I: Into<::bobbin_bits::U6>>(&self, index: I) -> *const Pchctrl { 
        self.pchctrl_reg().ptr(index.into())
    }

    #[doc="Read the PCHCTRL register."]
    #[inline] pub fn pchctrl<I: Into<::bobbin_bits::U6>>(&self, index: I) -> Pchctrl { 
        self.pchctrl_reg().read(index.into())
    }

    #[doc="Write the PCHCTRL register."]
    #[inline] pub fn write_pchctrl<I: Into<::bobbin_bits::U6>>(&self, index: I, value: Pchctrl) -> &Self {
        self.pchctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PCHCTRL register."]
    #[inline] pub fn set_pchctrl<I: Into<::bobbin_bits::U6>, F: FnOnce(Pchctrl) -> Pchctrl>(&self, index: I, f: F) -> &Self {
        self.pchctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PCHCTRL register."]
    #[inline] pub fn with_pchctrl<I: Into<::bobbin_bits::U6> + Copy, F: FnOnce(Pchctrl) -> Pchctrl>(&self, index: I, f: F) -> &Self {
        self.pchctrl_reg().with(index.into(), f);
        self
    }

}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u8);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Ctrla {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Synchroniation Busy bit"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline] pub fn genctrl0(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0xfff) as u16) } // [13:2]
    }

    #[doc="Returns true if GENCTRL0 != 0"]
    #[inline] pub fn test_genctrl0(&self) -> bool {
        self.genctrl0() != 0
    }

    #[doc="Sets the GENCTRL0 field."]
    #[inline] pub fn set_genctrl0<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline] pub fn genctrl1(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0xfff) as u16) } // [14:3]
    }

    #[doc="Returns true if GENCTRL1 != 0"]
    #[inline] pub fn test_genctrl1(&self) -> bool {
        self.genctrl1() != 0
    }

    #[doc="Sets the GENCTRL1 field."]
    #[inline] pub fn set_genctrl1<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline] pub fn genctrl2(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if GENCTRL2 != 0"]
    #[inline] pub fn test_genctrl2(&self) -> bool {
        self.genctrl2() != 0
    }

    #[doc="Sets the GENCTRL2 field."]
    #[inline] pub fn set_genctrl2<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline] pub fn genctrl3(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0xfff) as u16) } // [16:5]
    }

    #[doc="Returns true if GENCTRL3 != 0"]
    #[inline] pub fn test_genctrl3(&self) -> bool {
        self.genctrl3() != 0
    }

    #[doc="Sets the GENCTRL3 field."]
    #[inline] pub fn set_genctrl3<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline] pub fn genctrl4(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0xfff) as u16) } // [17:6]
    }

    #[doc="Returns true if GENCTRL4 != 0"]
    #[inline] pub fn test_genctrl4(&self) -> bool {
        self.genctrl4() != 0
    }

    #[doc="Sets the GENCTRL4 field."]
    #[inline] pub fn set_genctrl4<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Generic Clock Generator Control 5 Synchronization Busy bits"]
    #[inline] pub fn genctrl5(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0xfff) as u16) } // [18:7]
    }

    #[doc="Returns true if GENCTRL5 != 0"]
    #[inline] pub fn test_genctrl5(&self) -> bool {
        self.genctrl5() != 0
    }

    #[doc="Sets the GENCTRL5 field."]
    #[inline] pub fn set_genctrl5<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Generic Clock Generator Control 6 Synchronization Busy bits"]
    #[inline] pub fn genctrl6(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xfff) as u16) } // [19:8]
    }

    #[doc="Returns true if GENCTRL6 != 0"]
    #[inline] pub fn test_genctrl6(&self) -> bool {
        self.genctrl6() != 0
    }

    #[doc="Sets the GENCTRL6 field."]
    #[inline] pub fn set_genctrl6<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Generic Clock Generator Control 7 Synchronization Busy bits"]
    #[inline] pub fn genctrl7(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xfff) as u16) } // [20:9]
    }

    #[doc="Returns true if GENCTRL7 != 0"]
    #[inline] pub fn test_genctrl7(&self) -> bool {
        self.genctrl7() != 0
    }

    #[doc="Sets the GENCTRL7 field."]
    #[inline] pub fn set_genctrl7<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Generic Clock Generator Control 8 Synchronization Busy bits"]
    #[inline] pub fn genctrl8(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0xfff) as u16) } // [21:10]
    }

    #[doc="Returns true if GENCTRL8 != 0"]
    #[inline] pub fn test_genctrl8(&self) -> bool {
        self.genctrl8() != 0
    }

    #[doc="Sets the GENCTRL8 field."]
    #[inline] pub fn set_genctrl8<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Generic Clock Generator Control 9 Synchronization Busy bits"]
    #[inline] pub fn genctrl9(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xfff) as u16) } // [22:11]
    }

    #[doc="Returns true if GENCTRL9 != 0"]
    #[inline] pub fn test_genctrl9(&self) -> bool {
        self.genctrl9() != 0
    }

    #[doc="Sets the GENCTRL9 field."]
    #[inline] pub fn set_genctrl9<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Generic Clock Generator Control 10 Synchronization Busy bits"]
    #[inline] pub fn genctrl10(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xfff) as u16) } // [23:12]
    }

    #[doc="Returns true if GENCTRL10 != 0"]
    #[inline] pub fn test_genctrl10(&self) -> bool {
        self.genctrl10() != 0
    }

    #[doc="Sets the GENCTRL10 field."]
    #[inline] pub fn set_genctrl10<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Generic Clock Generator Control 11 Synchronization Busy bits"]
    #[inline] pub fn genctrl11(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xfff) as u16) } // [24:13]
    }

    #[doc="Returns true if GENCTRL11 != 0"]
    #[inline] pub fn test_genctrl11(&self) -> bool {
        self.genctrl11() != 0
    }

    #[doc="Sets the GENCTRL11 field."]
    #[inline] pub fn set_genctrl11<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.genctrl0() != 0 { try!(write!(f, " genctrl0=0x{:x}", self.genctrl0()))}
        if self.genctrl1() != 0 { try!(write!(f, " genctrl1=0x{:x}", self.genctrl1()))}
        if self.genctrl2() != 0 { try!(write!(f, " genctrl2=0x{:x}", self.genctrl2()))}
        if self.genctrl3() != 0 { try!(write!(f, " genctrl3=0x{:x}", self.genctrl3()))}
        if self.genctrl4() != 0 { try!(write!(f, " genctrl4=0x{:x}", self.genctrl4()))}
        if self.genctrl5() != 0 { try!(write!(f, " genctrl5=0x{:x}", self.genctrl5()))}
        if self.genctrl6() != 0 { try!(write!(f, " genctrl6=0x{:x}", self.genctrl6()))}
        if self.genctrl7() != 0 { try!(write!(f, " genctrl7=0x{:x}", self.genctrl7()))}
        if self.genctrl8() != 0 { try!(write!(f, " genctrl8=0x{:x}", self.genctrl8()))}
        if self.genctrl9() != 0 { try!(write!(f, " genctrl9=0x{:x}", self.genctrl9()))}
        if self.genctrl10() != 0 { try!(write!(f, " genctrl10=0x{:x}", self.genctrl10()))}
        if self.genctrl11() != 0 { try!(write!(f, " genctrl11=0x{:x}", self.genctrl11()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Generic Clock Generator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Genctrl(pub u32);
impl Genctrl {
    #[doc="Source Select"]
    #[inline] pub fn src(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SRC != 0"]
    #[inline] pub fn test_src(&self) -> bool {
        self.src() != 0
    }

    #[doc="Sets the SRC field."]
    #[inline] pub fn set_src<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Generic Clock Generator Enable"]
    #[inline] pub fn genen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GENEN != 0"]
    #[inline] pub fn test_genen(&self) -> bool {
        self.genen() != 0
    }

    #[doc="Sets the GENEN field."]
    #[inline] pub fn set_genen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Improve Duty Cycle"]
    #[inline] pub fn idc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if IDC != 0"]
    #[inline] pub fn test_idc(&self) -> bool {
        self.idc() != 0
    }

    #[doc="Sets the IDC field."]
    #[inline] pub fn set_idc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Output Off Value"]
    #[inline] pub fn oov(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OOV != 0"]
    #[inline] pub fn test_oov(&self) -> bool {
        self.oov() != 0
    }

    #[doc="Sets the OOV field."]
    #[inline] pub fn set_oov<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Output Enable"]
    #[inline] pub fn oe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OE != 0"]
    #[inline] pub fn test_oe(&self) -> bool {
        self.oe() != 0
    }

    #[doc="Sets the OE field."]
    #[inline] pub fn set_oe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Divide Selection"]
    #[inline] pub fn divsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DIVSEL != 0"]
    #[inline] pub fn test_divsel(&self) -> bool {
        self.divsel() != 0
    }

    #[doc="Sets the DIVSEL field."]
    #[inline] pub fn set_divsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Division Factor"]
    #[inline] pub fn div(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Genctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Genctrl(other)
    }
}

impl ::core::fmt::Display for Genctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Genctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
        if self.genen() != 0 { try!(write!(f, " genen"))}
        if self.idc() != 0 { try!(write!(f, " idc"))}
        if self.oov() != 0 { try!(write!(f, " oov"))}
        if self.oe() != 0 { try!(write!(f, " oe"))}
        if self.divsel() != 0 { try!(write!(f, " divsel"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Clock Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pchctrl(pub u32);
impl Pchctrl {
    #[doc="Generic Clock Generator"]
    #[inline] pub fn gen(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if GEN != 0"]
    #[inline] pub fn test_gen(&self) -> bool {
        self.gen() != 0
    }

    #[doc="Sets the GEN field."]
    #[inline] pub fn set_gen<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Enable"]
    #[inline] pub fn chen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Write Lock"]
    #[inline] pub fn wrtlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WRTLOCK != 0"]
    #[inline] pub fn test_wrtlock(&self) -> bool {
        self.wrtlock() != 0
    }

    #[doc="Sets the WRTLOCK field."]
    #[inline] pub fn set_wrtlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Pchctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Pchctrl(other)
    }
}

impl ::core::fmt::Display for Pchctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pchctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gen() != 0 { try!(write!(f, " gen=0x{:x}", self.gen()))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

