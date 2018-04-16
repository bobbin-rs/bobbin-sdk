#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DAC Peripheral"]
pub struct DacPeriph(pub usize); 

impl DacPeriph {
    #[doc="Get the DATSL Register."]
    #[inline] pub fn datsl_reg(&self) -> RegisterArray<Datsl, bits::R16> { 
        RegisterArray::new(self.0 as *mut Datsl, 0x0, 0x2)
    }

    #[doc="Get the *mut pointer for the DATSL register."]
    #[inline] pub fn datsl_mut<I: Into<bits::R16>>(&self, index: I) -> *mut Datsl { 
        self.datsl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DATSL register."]
    #[inline] pub fn datsl_ptr<I: Into<bits::R16>>(&self, index: I) -> *const Datsl { 
        self.datsl_reg().ptr(index.into())
    }

    #[doc="Read the DATSL register."]
    #[inline] pub fn datsl<I: Into<bits::R16>>(&self, index: I) -> Datsl { 
        self.datsl_reg().read(index.into())
    }

    #[doc="Write the DATSL register."]
    #[inline] pub fn write_datsl<I: Into<bits::R16>>(&self, index: I, value: Datsl) -> &Self {
        self.datsl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DATSL register."]
    #[inline] pub fn set_datsl<I: Into<bits::R16>, F: FnOnce(Datsl) -> Datsl>(&self, index: I, f: F) -> &Self {
        self.datsl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DATSL register."]
    #[inline] pub fn with_datsl<I: Into<bits::R16> + Copy, F: FnOnce(Datsl) -> Datsl>(&self, index: I, f: F) -> &Self {
        self.datsl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DATSH Register."]
    #[inline] pub fn datsh_reg(&self) -> RegisterArray<Datsh, bits::R16> { 
        RegisterArray::new(self.0 as *mut Datsh, 0x1, 0x2)
    }

    #[doc="Get the *mut pointer for the DATSH register."]
    #[inline] pub fn datsh_mut<I: Into<bits::R16>>(&self, index: I) -> *mut Datsh { 
        self.datsh_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DATSH register."]
    #[inline] pub fn datsh_ptr<I: Into<bits::R16>>(&self, index: I) -> *const Datsh { 
        self.datsh_reg().ptr(index.into())
    }

    #[doc="Read the DATSH register."]
    #[inline] pub fn datsh<I: Into<bits::R16>>(&self, index: I) -> Datsh { 
        self.datsh_reg().read(index.into())
    }

    #[doc="Write the DATSH register."]
    #[inline] pub fn write_datsh<I: Into<bits::R16>>(&self, index: I, value: Datsh) -> &Self {
        self.datsh_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DATSH register."]
    #[inline] pub fn set_datsh<I: Into<bits::R16>, F: FnOnce(Datsh) -> Datsh>(&self, index: I, f: F) -> &Self {
        self.datsh_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DATSH register."]
    #[inline] pub fn with_datsh<I: Into<bits::R16> + Copy, F: FnOnce(Datsh) -> Datsh>(&self, index: I, f: F) -> &Self {
        self.datsh_reg().with(index.into(), f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> Register<Sr> { 
        Register::new(self.0 as *mut Sr, 0x20)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

    #[doc="Get the C0 Register."]
    #[inline] pub fn c0_reg(&self) -> Register<C0> { 
        Register::new(self.0 as *mut C0, 0x21)
    }

    #[doc="Get the *mut pointer for the C0 register."]
    #[inline] pub fn c0_mut(&self) -> *mut C0 { 
        self.c0_reg().ptr()
    }

    #[doc="Get the *const pointer for the C0 register."]
    #[inline] pub fn c0_ptr(&self) -> *const C0 { 
        self.c0_reg().ptr()
    }

    #[doc="Read the C0 register."]
    #[inline] pub fn c0(&self) -> C0 { 
        self.c0_reg().read()
    }

    #[doc="Write the C0 register."]
    #[inline] pub fn write_c0(&self, value: C0) -> &Self { 
        self.c0_reg().write(value);
        self
    }

    #[doc="Set the C0 register."]
    #[inline] pub fn set_c0<F: FnOnce(C0) -> C0>(&self, f: F) -> &Self {
        self.c0_reg().set(f);
        self
    }

    #[doc="Modify the C0 register."]
    #[inline] pub fn with_c0<F: FnOnce(C0) -> C0>(&self, f: F) -> &Self {
        self.c0_reg().with(f);
        self
    }

    #[doc="Get the C1 Register."]
    #[inline] pub fn c1_reg(&self) -> Register<C1> { 
        Register::new(self.0 as *mut C1, 0x22)
    }

    #[doc="Get the *mut pointer for the C1 register."]
    #[inline] pub fn c1_mut(&self) -> *mut C1 { 
        self.c1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1 register."]
    #[inline] pub fn c1_ptr(&self) -> *const C1 { 
        self.c1_reg().ptr()
    }

    #[doc="Read the C1 register."]
    #[inline] pub fn c1(&self) -> C1 { 
        self.c1_reg().read()
    }

    #[doc="Write the C1 register."]
    #[inline] pub fn write_c1(&self, value: C1) -> &Self { 
        self.c1_reg().write(value);
        self
    }

    #[doc="Set the C1 register."]
    #[inline] pub fn set_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        self.c1_reg().set(f);
        self
    }

    #[doc="Modify the C1 register."]
    #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        self.c1_reg().with(f);
        self
    }

    #[doc="Get the C2 Register."]
    #[inline] pub fn c2_reg(&self) -> Register<C2> { 
        Register::new(self.0 as *mut C2, 0x23)
    }

    #[doc="Get the *mut pointer for the C2 register."]
    #[inline] pub fn c2_mut(&self) -> *mut C2 { 
        self.c2_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2 register."]
    #[inline] pub fn c2_ptr(&self) -> *const C2 { 
        self.c2_reg().ptr()
    }

    #[doc="Read the C2 register."]
    #[inline] pub fn c2(&self) -> C2 { 
        self.c2_reg().read()
    }

    #[doc="Write the C2 register."]
    #[inline] pub fn write_c2(&self, value: C2) -> &Self { 
        self.c2_reg().write(value);
        self
    }

    #[doc="Set the C2 register."]
    #[inline] pub fn set_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        self.c2_reg().set(f);
        self
    }

    #[doc="Modify the C2 register."]
    #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        self.c2_reg().with(f);
        self
    }

}

#[doc="DAC Data Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datsl(pub u8);
impl Datsl {
    #[doc="When the DAC buffer is not enabled, DATA[11:0] controls the output voltage based on the following formula: V out = V in * (1 + DACDAT0[11:0])/4096 When the DAC buffer is enabled, DATA is mapped to the 16-word buffer"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Datsl {
    #[inline]
    fn from(other: u8) -> Self {
         Datsl(other)
    }
}

impl ::core::fmt::Display for Datsl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datsl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DAC Data High Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datsh(pub u8);
impl Datsh {
    #[doc="When the DAC Buffer is not enabled, DATA[11:0] controls the output voltage based on the following formula"]
    #[inline] pub fn data1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Datsh {
    #[inline]
    fn from(other: u8) -> Self {
         Datsh(other)
    }
}

impl ::core::fmt::Display for Datsh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datsh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DAC Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u8);
impl Sr {
    #[doc="DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline] pub fn dacbfrpbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DACBFRPBF != 0"]
    #[inline] pub fn test_dacbfrpbf(&self) -> bool {
        self.dacbfrpbf() != 0
    }

    #[doc="Sets the DACBFRPBF field."]
    #[inline] pub fn set_dacbfrpbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DAC Buffer Read Pointer Top Position Flag"]
    #[inline] pub fn dacbfrptf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DACBFRPTF != 0"]
    #[inline] pub fn test_dacbfrptf(&self) -> bool {
        self.dacbfrptf() != 0
    }

    #[doc="Sets the DACBFRPTF field."]
    #[inline] pub fn set_dacbfrptf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DAC Buffer Watermark Flag"]
    #[inline] pub fn dacbfwmf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DACBFWMF != 0"]
    #[inline] pub fn test_dacbfwmf(&self) -> bool {
        self.dacbfwmf() != 0
    }

    #[doc="Sets the DACBFWMF field."]
    #[inline] pub fn set_dacbfwmf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Sr {
    #[inline]
    fn from(other: u8) -> Self {
         Sr(other)
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
        if self.dacbfrpbf() != 0 { try!(write!(f, " dacbfrpbf"))}
        if self.dacbfrptf() != 0 { try!(write!(f, " dacbfrptf"))}
        if self.dacbfwmf() != 0 { try!(write!(f, " dacbfwmf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DAC Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C0(pub u8);
impl C0 {
    #[doc="DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline] pub fn dacbbien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DACBBIEN != 0"]
    #[inline] pub fn test_dacbbien(&self) -> bool {
        self.dacbbien() != 0
    }

    #[doc="Sets the DACBBIEN field."]
    #[inline] pub fn set_dacbbien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline] pub fn dacbtien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DACBTIEN != 0"]
    #[inline] pub fn test_dacbtien(&self) -> bool {
        self.dacbtien() != 0
    }

    #[doc="Sets the DACBTIEN field."]
    #[inline] pub fn set_dacbtien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DAC Buffer Watermark Interrupt Enable"]
    #[inline] pub fn dacbwien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DACBWIEN != 0"]
    #[inline] pub fn test_dacbwien(&self) -> bool {
        self.dacbwien() != 0
    }

    #[doc="Sets the DACBWIEN field."]
    #[inline] pub fn set_dacbwien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DAC Low Power Control"]
    #[inline] pub fn lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LPEN != 0"]
    #[inline] pub fn test_lpen(&self) -> bool {
        self.lpen() != 0
    }

    #[doc="Sets the LPEN field."]
    #[inline] pub fn set_lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DAC Software Trigger"]
    #[inline] pub fn dacswtrg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DACSWTRG != 0"]
    #[inline] pub fn test_dacswtrg(&self) -> bool {
        self.dacswtrg() != 0
    }

    #[doc="Sets the DACSWTRG field."]
    #[inline] pub fn set_dacswtrg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DAC Trigger Select"]
    #[inline] pub fn dactrgsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DACTRGSEL != 0"]
    #[inline] pub fn test_dactrgsel(&self) -> bool {
        self.dactrgsel() != 0
    }

    #[doc="Sets the DACTRGSEL field."]
    #[inline] pub fn set_dactrgsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DAC Reference Select"]
    #[inline] pub fn dacrfs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DACRFS != 0"]
    #[inline] pub fn test_dacrfs(&self) -> bool {
        self.dacrfs() != 0
    }

    #[doc="Sets the DACRFS field."]
    #[inline] pub fn set_dacrfs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DAC Enable"]
    #[inline] pub fn dacen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DACEN != 0"]
    #[inline] pub fn test_dacen(&self) -> bool {
        self.dacen() != 0
    }

    #[doc="Sets the DACEN field."]
    #[inline] pub fn set_dacen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C0 {
    #[inline]
    fn from(other: u8) -> Self {
         C0(other)
    }
}

impl ::core::fmt::Display for C0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacbbien() != 0 { try!(write!(f, " dacbbien"))}
        if self.dacbtien() != 0 { try!(write!(f, " dacbtien"))}
        if self.dacbwien() != 0 { try!(write!(f, " dacbwien"))}
        if self.lpen() != 0 { try!(write!(f, " lpen"))}
        if self.dacswtrg() != 0 { try!(write!(f, " dacswtrg"))}
        if self.dactrgsel() != 0 { try!(write!(f, " dactrgsel"))}
        if self.dacrfs() != 0 { try!(write!(f, " dacrfs"))}
        if self.dacen() != 0 { try!(write!(f, " dacen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DAC Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
    #[doc="DAC Buffer Enable"]
    #[inline] pub fn dacbfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DACBFEN != 0"]
    #[inline] pub fn test_dacbfen(&self) -> bool {
        self.dacbfen() != 0
    }

    #[doc="Sets the DACBFEN field."]
    #[inline] pub fn set_dacbfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DAC Buffer Work Mode Select"]
    #[inline] pub fn dacbfmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if DACBFMD != 0"]
    #[inline] pub fn test_dacbfmd(&self) -> bool {
        self.dacbfmd() != 0
    }

    #[doc="Sets the DACBFMD field."]
    #[inline] pub fn set_dacbfmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DAC Buffer Watermark Select"]
    #[inline] pub fn dacbfwm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if DACBFWM != 0"]
    #[inline] pub fn test_dacbfwm(&self) -> bool {
        self.dacbfwm() != 0
    }

    #[doc="Sets the DACBFWM field."]
    #[inline] pub fn set_dacbfwm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DMA Enable Select"]
    #[inline] pub fn dmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C1 {
    #[inline]
    fn from(other: u8) -> Self {
         C1(other)
    }
}

impl ::core::fmt::Display for C1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacbfen() != 0 { try!(write!(f, " dacbfen"))}
        if self.dacbfmd() != 0 { try!(write!(f, " dacbfmd=0x{:x}", self.dacbfmd()))}
        if self.dacbfwm() != 0 { try!(write!(f, " dacbfwm=0x{:x}", self.dacbfwm()))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DAC Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
    #[doc="DAC Buffer Upper Limit"]
    #[inline] pub fn dacbfup(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DACBFUP != 0"]
    #[inline] pub fn test_dacbfup(&self) -> bool {
        self.dacbfup() != 0
    }

    #[doc="Sets the DACBFUP field."]
    #[inline] pub fn set_dacbfup<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DAC Buffer Read Pointer"]
    #[inline] pub fn dacbfrp(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if DACBFRP != 0"]
    #[inline] pub fn test_dacbfrp(&self) -> bool {
        self.dacbfrp() != 0
    }

    #[doc="Sets the DACBFRP field."]
    #[inline] pub fn set_dacbfrp<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for C2 {
    #[inline]
    fn from(other: u8) -> Self {
         C2(other)
    }
}

impl ::core::fmt::Display for C2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacbfup() != 0 { try!(write!(f, " dacbfup=0x{:x}", self.dacbfup()))}
        if self.dacbfrp() != 0 { try!(write!(f, " dacbfrp=0x{:x}", self.dacbfrp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

