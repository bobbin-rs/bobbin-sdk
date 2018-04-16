
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PORT Peripheral"]
pub struct PortPeriph(pub usize); 

pub struct PortPin { pub port: PortPeriph, pub index: usize }

impl PortPeriph {
    #[doc="Get the PCR Register."]
    #[inline] pub fn pcr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pcr, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pcr, 0x0, 0x4)
    }

    #[doc="Get the *mut pointer for the PCR register."]
    #[inline] pub fn pcr_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Pcr { 
        self.pcr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PCR register."]
    #[inline] pub fn pcr_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Pcr { 
        self.pcr_reg().ptr(index.into())
    }

    #[doc="Read the PCR register."]
    #[inline] pub fn pcr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Pcr { 
        self.pcr_reg().read(index.into())
    }

    #[doc="Write the PCR register."]
    #[inline] pub fn write_pcr<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Pcr) -> &Self {
        self.pcr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PCR register."]
    #[inline] pub fn set_pcr<I: Into<::bobbin_bits::R32>, F: FnOnce(Pcr) -> Pcr>(&self, index: I, f: F) -> &Self {
        self.pcr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PCR register."]
    #[inline] pub fn with_pcr<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Pcr) -> Pcr>(&self, index: I, f: F) -> &Self {
        self.pcr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the GPCLR Register."]
    #[inline] pub fn gpclr_reg(&self) -> ::bobbin_mcu::register::Register<Gpclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpclr, 0x80)
    }

    #[doc="Get the *mut pointer for the GPCLR register."]
    #[inline] pub fn gpclr_mut(&self) -> *mut Gpclr { 
        self.gpclr_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPCLR register."]
    #[inline] pub fn gpclr_ptr(&self) -> *const Gpclr { 
        self.gpclr_reg().ptr()
    }

    #[doc="Write the GPCLR register."]
    #[inline] pub fn write_gpclr(&self, value: Gpclr) -> &Self { 
        self.gpclr_reg().write(value);
        self
    }

    #[doc="Set the GPCLR register."]
    #[inline] pub fn set_gpclr<F: FnOnce(Gpclr) -> Gpclr>(&self, f: F) -> &Self {
        self.gpclr_reg().set(f);
        self
    }

    #[doc="Get the GPCHR Register."]
    #[inline] pub fn gpchr_reg(&self) -> ::bobbin_mcu::register::Register<Gpchr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpchr, 0x84)
    }

    #[doc="Get the *mut pointer for the GPCHR register."]
    #[inline] pub fn gpchr_mut(&self) -> *mut Gpchr { 
        self.gpchr_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPCHR register."]
    #[inline] pub fn gpchr_ptr(&self) -> *const Gpchr { 
        self.gpchr_reg().ptr()
    }

    #[doc="Write the GPCHR register."]
    #[inline] pub fn write_gpchr(&self, value: Gpchr) -> &Self { 
        self.gpchr_reg().write(value);
        self
    }

    #[doc="Set the GPCHR register."]
    #[inline] pub fn set_gpchr<F: FnOnce(Gpchr) -> Gpchr>(&self, f: F) -> &Self {
        self.gpchr_reg().set(f);
        self
    }

    #[doc="Get the ISFR Register."]
    #[inline] pub fn isfr_reg(&self) -> ::bobbin_mcu::register::Register<Isfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isfr, 0xa0)
    }

    #[doc="Get the *mut pointer for the ISFR register."]
    #[inline] pub fn isfr_mut(&self) -> *mut Isfr { 
        self.isfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISFR register."]
    #[inline] pub fn isfr_ptr(&self) -> *const Isfr { 
        self.isfr_reg().ptr()
    }

    #[doc="Read the ISFR register."]
    #[inline] pub fn isfr(&self) -> Isfr { 
        self.isfr_reg().read()
    }

    #[doc="Write the ISFR register."]
    #[inline] pub fn write_isfr(&self, value: Isfr) -> &Self { 
        self.isfr_reg().write(value);
        self
    }

    #[doc="Set the ISFR register."]
    #[inline] pub fn set_isfr<F: FnOnce(Isfr) -> Isfr>(&self, f: F) -> &Self {
        self.isfr_reg().set(f);
        self
    }

    #[doc="Modify the ISFR register."]
    #[inline] pub fn with_isfr<F: FnOnce(Isfr) -> Isfr>(&self, f: F) -> &Self {
        self.isfr_reg().with(f);
        self
    }

}

#[doc="Pin Control Register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc="Pull Select"]
    #[inline] pub fn ps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pull Enable"]
    #[inline] pub fn pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Slew Rate Enable"]
    #[inline] pub fn sre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SRE != 0"]
    #[inline] pub fn test_sre(&self) -> bool {
        self.sre() != 0
    }

    #[doc="Sets the SRE field."]
    #[inline] pub fn set_sre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Passive Filter Enable"]
    #[inline] pub fn pfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PFE != 0"]
    #[inline] pub fn test_pfe(&self) -> bool {
        self.pfe() != 0
    }

    #[doc="Sets the PFE field."]
    #[inline] pub fn set_pfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Open Drain Enable"]
    #[inline] pub fn ode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ODE != 0"]
    #[inline] pub fn test_ode(&self) -> bool {
        self.ode() != 0
    }

    #[doc="Sets the ODE field."]
    #[inline] pub fn set_ode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Drive Strength Enable"]
    #[inline] pub fn dse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DSE != 0"]
    #[inline] pub fn test_dse(&self) -> bool {
        self.dse() != 0
    }

    #[doc="Sets the DSE field."]
    #[inline] pub fn set_dse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pin Mux Control"]
    #[inline] pub fn mux(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if MUX != 0"]
    #[inline] pub fn test_mux(&self) -> bool {
        self.mux() != 0
    }

    #[doc="Sets the MUX field."]
    #[inline] pub fn set_mux<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Lock Register"]
    #[inline] pub fn lk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if LK != 0"]
    #[inline] pub fn test_lk(&self) -> bool {
        self.lk() != 0
    }

    #[doc="Sets the LK field."]
    #[inline] pub fn set_lk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Interrupt Configuration"]
    #[inline] pub fn irqc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if IRQC != 0"]
    #[inline] pub fn test_irqc(&self) -> bool {
        self.irqc() != 0
    }

    #[doc="Sets the IRQC field."]
    #[inline] pub fn set_irqc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Interrupt Status Flag"]
    #[inline] pub fn isf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ISF != 0"]
    #[inline] pub fn test_isf(&self) -> bool {
        self.isf() != 0
    }

    #[doc="Sets the ISF field."]
    #[inline] pub fn set_isf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Pcr {
    #[inline]
    fn from(other: u32) -> Self {
         Pcr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpclr(pub u32);
impl Gpclr {
    #[doc="Global Pin Write Data"]
    #[inline] pub fn gpwd(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if GPWD != 0"]
    #[inline] pub fn test_gpwd(&self) -> bool {
        self.gpwd() != 0
    }

    #[doc="Sets the GPWD field."]
    #[inline] pub fn set_gpwd<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Global Pin Write Enable"]
    #[inline] pub fn gpwe(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if GPWE != 0"]
    #[inline] pub fn test_gpwe(&self) -> bool {
        self.gpwe() != 0
    }

    #[doc="Sets the GPWE field."]
    #[inline] pub fn set_gpwe<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Gpclr {
    #[inline]
    fn from(other: u32) -> Self {
         Gpclr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpchr(pub u32);
impl Gpchr {
    #[doc="Global Pin Write Data"]
    #[inline] pub fn gpwd(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if GPWD != 0"]
    #[inline] pub fn test_gpwd(&self) -> bool {
        self.gpwd() != 0
    }

    #[doc="Sets the GPWD field."]
    #[inline] pub fn set_gpwd<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Global Pin Write Enable"]
    #[inline] pub fn gpwe(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if GPWE != 0"]
    #[inline] pub fn test_gpwe(&self) -> bool {
        self.gpwe() != 0
    }

    #[doc="Sets the GPWE field."]
    #[inline] pub fn set_gpwe<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Gpchr {
    #[inline]
    fn from(other: u32) -> Self {
         Gpchr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isfr(pub u32);
impl Isfr {
    #[doc="Interrupt Status Flag"]
    #[inline] pub fn isf<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ISF != 0"]
    #[inline] pub fn test_isf<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.isf(index) != 0
    }

    #[doc="Sets the ISF field."]
    #[inline] pub fn set_isf<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Isfr {
    #[inline]
    fn from(other: u32) -> Self {
         Isfr(other)
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

