::bobbin_mcu::periph!( CCL, Ccl, CCL_PERIPH, CclPeriph, CCL_OWNED, CCL_REF_COUNT, 0x42003800, 0x00, 0x04);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CCL Peripheral"]
pub struct CclPeriph(pub usize); 

impl CclPeriph {
    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        self.ctrl_reg().read()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().with(f);
        self
    }

    #[doc="Get the SEQCTRL Register."]
    #[inline] pub fn seqctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Seqctrl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Seqctrl, 0x4, 0x1)
    }

    #[doc="Get the *mut pointer for the SEQCTRL register."]
    #[inline] pub fn seqctrl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Seqctrl { 
        self.seqctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the SEQCTRL register."]
    #[inline] pub fn seqctrl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Seqctrl { 
        self.seqctrl_reg().ptr(index.into())
    }

    #[doc="Read the SEQCTRL register."]
    #[inline] pub fn seqctrl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Seqctrl { 
        self.seqctrl_reg().read(index.into())
    }

    #[doc="Write the SEQCTRL register."]
    #[inline] pub fn write_seqctrl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Seqctrl) -> &Self {
        self.seqctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the SEQCTRL register."]
    #[inline] pub fn set_seqctrl<I: Into<::bobbin_bits::R2>, F: FnOnce(Seqctrl) -> Seqctrl>(&self, index: I, f: F) -> &Self {
        self.seqctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the SEQCTRL register."]
    #[inline] pub fn with_seqctrl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Seqctrl) -> Seqctrl>(&self, index: I, f: F) -> &Self {
        self.seqctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the LUTCTRL Register."]
    #[inline] pub fn lutctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Lutctrl, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Lutctrl, 0x8, 0x4)
    }

    #[doc="Get the *mut pointer for the LUTCTRL register."]
    #[inline] pub fn lutctrl_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Lutctrl { 
        self.lutctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the LUTCTRL register."]
    #[inline] pub fn lutctrl_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Lutctrl { 
        self.lutctrl_reg().ptr(index.into())
    }

    #[doc="Read the LUTCTRL register."]
    #[inline] pub fn lutctrl<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Lutctrl { 
        self.lutctrl_reg().read(index.into())
    }

    #[doc="Write the LUTCTRL register."]
    #[inline] pub fn write_lutctrl<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Lutctrl) -> &Self {
        self.lutctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the LUTCTRL register."]
    #[inline] pub fn set_lutctrl<I: Into<::bobbin_bits::R4>, F: FnOnce(Lutctrl) -> Lutctrl>(&self, index: I, f: F) -> &Self {
        self.lutctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the LUTCTRL register."]
    #[inline] pub fn with_lutctrl<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Lutctrl) -> Lutctrl>(&self, index: I, f: F) -> &Self {
        self.lutctrl_reg().with(index.into(), f);
        self
    }

}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
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

    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrl(other)
    }
}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SEQ Control x"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Seqctrl(pub u8);
impl Seqctrl {
    #[doc="Sequential Selection"]
    #[inline] pub fn seqsel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SEQSEL != 0"]
    #[inline] pub fn test_seqsel(&self) -> bool {
        self.seqsel() != 0
    }

    #[doc="Sets the SEQSEL field."]
    #[inline] pub fn set_seqsel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Seqctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Seqctrl(other)
    }
}

impl ::core::fmt::Display for Seqctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Seqctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.seqsel() != 0 { try!(write!(f, " seqsel=0x{:x}", self.seqsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LUT Control x"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lutctrl(pub u32);
impl Lutctrl {
    #[doc="LUT Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Filter Selection"]
    #[inline] pub fn filtsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if FILTSEL != 0"]
    #[inline] pub fn test_filtsel(&self) -> bool {
        self.filtsel() != 0
    }

    #[doc="Sets the FILTSEL field."]
    #[inline] pub fn set_filtsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Edge Selection"]
    #[inline] pub fn edgesel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EDGESEL != 0"]
    #[inline] pub fn test_edgesel(&self) -> bool {
        self.edgesel() != 0
    }

    #[doc="Sets the EDGESEL field."]
    #[inline] pub fn set_edgesel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Input Selection 0"]
    #[inline] pub fn insel0(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if INSEL0 != 0"]
    #[inline] pub fn test_insel0(&self) -> bool {
        self.insel0() != 0
    }

    #[doc="Sets the INSEL0 field."]
    #[inline] pub fn set_insel0<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Input Selection 1"]
    #[inline] pub fn insel1(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if INSEL1 != 0"]
    #[inline] pub fn test_insel1(&self) -> bool {
        self.insel1() != 0
    }

    #[doc="Sets the INSEL1 field."]
    #[inline] pub fn set_insel1<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Input Selection 2"]
    #[inline] pub fn insel2(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if INSEL2 != 0"]
    #[inline] pub fn test_insel2(&self) -> bool {
        self.insel2() != 0
    }

    #[doc="Sets the INSEL2 field."]
    #[inline] pub fn set_insel2<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Inverted Event Input Enable"]
    #[inline] pub fn invei(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if INVEI != 0"]
    #[inline] pub fn test_invei(&self) -> bool {
        self.invei() != 0
    }

    #[doc="Sets the INVEI field."]
    #[inline] pub fn set_invei<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="LUT Event Input Enable"]
    #[inline] pub fn lutei(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if LUTEI != 0"]
    #[inline] pub fn test_lutei(&self) -> bool {
        self.lutei() != 0
    }

    #[doc="Sets the LUTEI field."]
    #[inline] pub fn set_lutei<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="LUT Event Output Enable"]
    #[inline] pub fn luteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if LUTEO != 0"]
    #[inline] pub fn test_luteo(&self) -> bool {
        self.luteo() != 0
    }

    #[doc="Sets the LUTEO field."]
    #[inline] pub fn set_luteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Truth Value"]
    #[inline] pub fn truth(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if TRUTH != 0"]
    #[inline] pub fn test_truth(&self) -> bool {
        self.truth() != 0
    }

    #[doc="Sets the TRUTH field."]
    #[inline] pub fn set_truth<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Lutctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Lutctrl(other)
    }
}

impl ::core::fmt::Display for Lutctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lutctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.filtsel() != 0 { try!(write!(f, " filtsel=0x{:x}", self.filtsel()))}
        if self.edgesel() != 0 { try!(write!(f, " edgesel"))}
        if self.insel0() != 0 { try!(write!(f, " insel0=0x{:x}", self.insel0()))}
        if self.insel1() != 0 { try!(write!(f, " insel1=0x{:x}", self.insel1()))}
        if self.insel2() != 0 { try!(write!(f, " insel2=0x{:x}", self.insel2()))}
        if self.invei() != 0 { try!(write!(f, " invei"))}
        if self.lutei() != 0 { try!(write!(f, " lutei"))}
        if self.luteo() != 0 { try!(write!(f, " luteo"))}
        if self.truth() != 0 { try!(write!(f, " truth=0x{:x}", self.truth()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

