
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMA_F3 Peripheral"]
pub struct DmaPeriph(pub usize); 

pub struct DmaCh { pub periph: DmaPeriph, pub index: usize }

impl DmaPeriph {
    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x0)
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        self.isr_reg().read()
    }

    #[doc="Get the IFCR Register."]
    #[inline] pub fn ifcr_reg(&self) -> ::bobbin_mcu::register::Register<Ifcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ifcr, 0x4)
    }

    #[doc="Get the *mut pointer for the IFCR register."]
    #[inline] pub fn ifcr_mut(&self) -> *mut Ifcr { 
        self.ifcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IFCR register."]
    #[inline] pub fn ifcr_ptr(&self) -> *const Ifcr { 
        self.ifcr_reg().ptr()
    }

    #[doc="Write the IFCR register."]
    #[inline] pub fn write_ifcr(&self, value: Ifcr) -> &Self { 
        self.ifcr_reg().write(value);
        self
    }

    #[doc="Set the IFCR register."]
    #[inline] pub fn set_ifcr<F: FnOnce(Ifcr) -> Ifcr>(&self, f: F) -> &Self {
        self.ifcr_reg().set(f);
        self
    }

    #[doc="Get the CCR Register."]
    #[inline] pub fn ccr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Ccr, ::bobbin_bits::R7> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Ccr, 0x8, 0x14)
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut<I: Into<::bobbin_bits::R7>>(&self, index: I) -> *mut Ccr { 
        self.ccr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr<I: Into<::bobbin_bits::R7>>(&self, index: I) -> *const Ccr { 
        self.ccr_reg().ptr(index.into())
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr<I: Into<::bobbin_bits::R7>>(&self, index: I) -> Ccr { 
        self.ccr_reg().read(index.into())
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn write_ccr<I: Into<::bobbin_bits::R7>>(&self, index: I, value: Ccr) -> &Self {
        self.ccr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CCR register."]
    #[inline] pub fn set_ccr<I: Into<::bobbin_bits::R7>, F: FnOnce(Ccr) -> Ccr>(&self, index: I, f: F) -> &Self {
        self.ccr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<I: Into<::bobbin_bits::R7> + Copy, F: FnOnce(Ccr) -> Ccr>(&self, index: I, f: F) -> &Self {
        self.ccr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CNDTR Register."]
    #[inline] pub fn cndtr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cndtr, ::bobbin_bits::R7> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cndtr, 0xc, 0x14)
    }

    #[doc="Get the *mut pointer for the CNDTR register."]
    #[inline] pub fn cndtr_mut<I: Into<::bobbin_bits::R7>>(&self, index: I) -> *mut Cndtr { 
        self.cndtr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CNDTR register."]
    #[inline] pub fn cndtr_ptr<I: Into<::bobbin_bits::R7>>(&self, index: I) -> *const Cndtr { 
        self.cndtr_reg().ptr(index.into())
    }

    #[doc="Read the CNDTR register."]
    #[inline] pub fn cndtr<I: Into<::bobbin_bits::R7>>(&self, index: I) -> Cndtr { 
        self.cndtr_reg().read(index.into())
    }

    #[doc="Write the CNDTR register."]
    #[inline] pub fn write_cndtr<I: Into<::bobbin_bits::R7>>(&self, index: I, value: Cndtr) -> &Self {
        self.cndtr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CNDTR register."]
    #[inline] pub fn set_cndtr<I: Into<::bobbin_bits::R7>, F: FnOnce(Cndtr) -> Cndtr>(&self, index: I, f: F) -> &Self {
        self.cndtr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CNDTR register."]
    #[inline] pub fn with_cndtr<I: Into<::bobbin_bits::R7> + Copy, F: FnOnce(Cndtr) -> Cndtr>(&self, index: I, f: F) -> &Self {
        self.cndtr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CPAR Register."]
    #[inline] pub fn cpar_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cpar, ::bobbin_bits::R7> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cpar, 0x10, 0x14)
    }

    #[doc="Get the *mut pointer for the CPAR register."]
    #[inline] pub fn cpar_mut<I: Into<::bobbin_bits::R7>>(&self, index: I) -> *mut Cpar { 
        self.cpar_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CPAR register."]
    #[inline] pub fn cpar_ptr<I: Into<::bobbin_bits::R7>>(&self, index: I) -> *const Cpar { 
        self.cpar_reg().ptr(index.into())
    }

    #[doc="Read the CPAR register."]
    #[inline] pub fn cpar<I: Into<::bobbin_bits::R7>>(&self, index: I) -> Cpar { 
        self.cpar_reg().read(index.into())
    }

    #[doc="Write the CPAR register."]
    #[inline] pub fn write_cpar<I: Into<::bobbin_bits::R7>>(&self, index: I, value: Cpar) -> &Self {
        self.cpar_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CPAR register."]
    #[inline] pub fn set_cpar<I: Into<::bobbin_bits::R7>, F: FnOnce(Cpar) -> Cpar>(&self, index: I, f: F) -> &Self {
        self.cpar_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CPAR register."]
    #[inline] pub fn with_cpar<I: Into<::bobbin_bits::R7> + Copy, F: FnOnce(Cpar) -> Cpar>(&self, index: I, f: F) -> &Self {
        self.cpar_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CMAR Register."]
    #[inline] pub fn cmar_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cmar, ::bobbin_bits::R7> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cmar, 0x14, 0x14)
    }

    #[doc="Get the *mut pointer for the CMAR register."]
    #[inline] pub fn cmar_mut<I: Into<::bobbin_bits::R7>>(&self, index: I) -> *mut Cmar { 
        self.cmar_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CMAR register."]
    #[inline] pub fn cmar_ptr<I: Into<::bobbin_bits::R7>>(&self, index: I) -> *const Cmar { 
        self.cmar_reg().ptr(index.into())
    }

    #[doc="Read the CMAR register."]
    #[inline] pub fn cmar<I: Into<::bobbin_bits::R7>>(&self, index: I) -> Cmar { 
        self.cmar_reg().read(index.into())
    }

    #[doc="Write the CMAR register."]
    #[inline] pub fn write_cmar<I: Into<::bobbin_bits::R7>>(&self, index: I, value: Cmar) -> &Self {
        self.cmar_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CMAR register."]
    #[inline] pub fn set_cmar<I: Into<::bobbin_bits::R7>, F: FnOnce(Cmar) -> Cmar>(&self, index: I, f: F) -> &Self {
        self.cmar_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CMAR register."]
    #[inline] pub fn with_cmar<I: Into<::bobbin_bits::R7> + Copy, F: FnOnce(Cmar) -> Cmar>(&self, index: I, f: F) -> &Self {
        self.cmar_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CSELR Register."]
    #[inline] pub fn cselr_reg(&self) -> ::bobbin_mcu::register::Register<Cselr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cselr, 0xa8)
    }

    #[doc="Get the *mut pointer for the CSELR register."]
    #[inline] pub fn cselr_mut(&self) -> *mut Cselr { 
        self.cselr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSELR register."]
    #[inline] pub fn cselr_ptr(&self) -> *const Cselr { 
        self.cselr_reg().ptr()
    }

    #[doc="Read the CSELR register."]
    #[inline] pub fn cselr(&self) -> Cselr { 
        self.cselr_reg().read()
    }

    #[doc="Write the CSELR register."]
    #[inline] pub fn write_cselr(&self, value: Cselr) -> &Self { 
        self.cselr_reg().write(value);
        self
    }

    #[doc="Set the CSELR register."]
    #[inline] pub fn set_cselr<F: FnOnce(Cselr) -> Cselr>(&self, f: F) -> &Self {
        self.cselr_reg().set(f);
        self
    }

    #[doc="Modify the CSELR register."]
    #[inline] pub fn with_cselr<F: FnOnce(Cselr) -> Cselr>(&self, f: F) -> &Self {
        self.cselr_reg().with(f);
        self
    }

}

#[doc="DMA interrupt status register (DMA_ISR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Channel n Global interrupt flag"]
    #[inline] pub fn gif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GIF != 0"]
    #[inline] pub fn test_gif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.gif(index) != 0
    }

    #[doc="Sets the GIF field."]
    #[inline] pub fn set_gif<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Transfer Complete flag"]
    #[inline] pub fn tcif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TCIF != 0"]
    #[inline] pub fn test_tcif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.tcif(index) != 0
    }

    #[doc="Sets the TCIF field."]
    #[inline] pub fn set_tcif<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Half Transfer Complete flag"]
    #[inline] pub fn htif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HTIF != 0"]
    #[inline] pub fn test_htif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.htif(index) != 0
    }

    #[doc="Sets the HTIF field."]
    #[inline] pub fn set_htif<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Transfer Error flag"]
    #[inline] pub fn teif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 3 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TEIF != 0"]
    #[inline] pub fn test_teif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.teif(index) != 0
    }

    #[doc="Sets the TEIF field."]
    #[inline] pub fn set_teif<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 3 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Isr(other)
    }
}

impl ::core::fmt::Display for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gif(0) != 0 { try!(write!(f, " gif[0]"))}
        if self.gif(1) != 0 { try!(write!(f, " gif[1]"))}
        if self.gif(2) != 0 { try!(write!(f, " gif[2]"))}
        if self.gif(3) != 0 { try!(write!(f, " gif[3]"))}
        if self.gif(4) != 0 { try!(write!(f, " gif[4]"))}
        if self.gif(5) != 0 { try!(write!(f, " gif[5]"))}
        if self.gif(6) != 0 { try!(write!(f, " gif[6]"))}
        if self.tcif(0) != 0 { try!(write!(f, " tcif[0]"))}
        if self.tcif(1) != 0 { try!(write!(f, " tcif[1]"))}
        if self.tcif(2) != 0 { try!(write!(f, " tcif[2]"))}
        if self.tcif(3) != 0 { try!(write!(f, " tcif[3]"))}
        if self.tcif(4) != 0 { try!(write!(f, " tcif[4]"))}
        if self.tcif(5) != 0 { try!(write!(f, " tcif[5]"))}
        if self.tcif(6) != 0 { try!(write!(f, " tcif[6]"))}
        if self.htif(0) != 0 { try!(write!(f, " htif[0]"))}
        if self.htif(1) != 0 { try!(write!(f, " htif[1]"))}
        if self.htif(2) != 0 { try!(write!(f, " htif[2]"))}
        if self.htif(3) != 0 { try!(write!(f, " htif[3]"))}
        if self.htif(4) != 0 { try!(write!(f, " htif[4]"))}
        if self.htif(5) != 0 { try!(write!(f, " htif[5]"))}
        if self.htif(6) != 0 { try!(write!(f, " htif[6]"))}
        if self.teif(0) != 0 { try!(write!(f, " teif[0]"))}
        if self.teif(1) != 0 { try!(write!(f, " teif[1]"))}
        if self.teif(2) != 0 { try!(write!(f, " teif[2]"))}
        if self.teif(3) != 0 { try!(write!(f, " teif[3]"))}
        if self.teif(4) != 0 { try!(write!(f, " teif[4]"))}
        if self.teif(5) != 0 { try!(write!(f, " teif[5]"))}
        if self.teif(6) != 0 { try!(write!(f, " teif[6]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA interrupt flag clear register (DMA_IFCR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc="Channel n Global interrupt clear"]
    #[inline] pub fn cgif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CGIF != 0"]
    #[inline] pub fn test_cgif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.cgif(index) != 0
    }

    #[doc="Sets the CGIF field."]
    #[inline] pub fn set_cgif<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Transfer Complete clear"]
    #[inline] pub fn ctcif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTCIF != 0"]
    #[inline] pub fn test_ctcif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.ctcif(index) != 0
    }

    #[doc="Sets the CTCIF field."]
    #[inline] pub fn set_ctcif<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Half Transfer clear"]
    #[inline] pub fn chtif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CHTIF != 0"]
    #[inline] pub fn test_chtif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.chtif(index) != 0
    }

    #[doc="Sets the CHTIF field."]
    #[inline] pub fn set_chtif<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Transfer Error clear"]
    #[inline] pub fn cteif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 3 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTEIF != 0"]
    #[inline] pub fn test_cteif<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.cteif(index) != 0
    }

    #[doc="Sets the CTEIF field."]
    #[inline] pub fn set_cteif<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 3 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ifcr {
    #[inline]
    fn from(other: u32) -> Self {
         Ifcr(other)
    }
}

impl ::core::fmt::Display for Ifcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgif(0) != 0 { try!(write!(f, " cgif[0]"))}
        if self.cgif(1) != 0 { try!(write!(f, " cgif[1]"))}
        if self.cgif(2) != 0 { try!(write!(f, " cgif[2]"))}
        if self.cgif(3) != 0 { try!(write!(f, " cgif[3]"))}
        if self.cgif(4) != 0 { try!(write!(f, " cgif[4]"))}
        if self.cgif(5) != 0 { try!(write!(f, " cgif[5]"))}
        if self.cgif(6) != 0 { try!(write!(f, " cgif[6]"))}
        if self.ctcif(0) != 0 { try!(write!(f, " ctcif[0]"))}
        if self.ctcif(1) != 0 { try!(write!(f, " ctcif[1]"))}
        if self.ctcif(2) != 0 { try!(write!(f, " ctcif[2]"))}
        if self.ctcif(3) != 0 { try!(write!(f, " ctcif[3]"))}
        if self.ctcif(4) != 0 { try!(write!(f, " ctcif[4]"))}
        if self.ctcif(5) != 0 { try!(write!(f, " ctcif[5]"))}
        if self.ctcif(6) != 0 { try!(write!(f, " ctcif[6]"))}
        if self.chtif(0) != 0 { try!(write!(f, " chtif[0]"))}
        if self.chtif(1) != 0 { try!(write!(f, " chtif[1]"))}
        if self.chtif(2) != 0 { try!(write!(f, " chtif[2]"))}
        if self.chtif(3) != 0 { try!(write!(f, " chtif[3]"))}
        if self.chtif(4) != 0 { try!(write!(f, " chtif[4]"))}
        if self.chtif(5) != 0 { try!(write!(f, " chtif[5]"))}
        if self.chtif(6) != 0 { try!(write!(f, " chtif[6]"))}
        if self.cteif(0) != 0 { try!(write!(f, " cteif[0]"))}
        if self.cteif(1) != 0 { try!(write!(f, " cteif[1]"))}
        if self.cteif(2) != 0 { try!(write!(f, " cteif[2]"))}
        if self.cteif(3) != 0 { try!(write!(f, " cteif[3]"))}
        if self.cteif(4) != 0 { try!(write!(f, " cteif[4]"))}
        if self.cteif(5) != 0 { try!(write!(f, " cteif[5]"))}
        if self.cteif(6) != 0 { try!(write!(f, " cteif[6]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA channel configuration register (DMA_CCR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc="Channel enable"]
    #[inline] pub fn en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer complete interrupt enable"]
    #[inline] pub fn tcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Half Transfer interrupt enable"]
    #[inline] pub fn htie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HTIE != 0"]
    #[inline] pub fn test_htie(&self) -> bool {
        self.htie() != 0
    }

    #[doc="Sets the HTIE field."]
    #[inline] pub fn set_htie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transfer error interrupt enable"]
    #[inline] pub fn teie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TEIE != 0"]
    #[inline] pub fn test_teie(&self) -> bool {
        self.teie() != 0
    }

    #[doc="Sets the TEIE field."]
    #[inline] pub fn set_teie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data transfer direction"]
    #[inline] pub fn dir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Circular mode"]
    #[inline] pub fn circ(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CIRC != 0"]
    #[inline] pub fn test_circ(&self) -> bool {
        self.circ() != 0
    }

    #[doc="Sets the CIRC field."]
    #[inline] pub fn set_circ<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Peripheral increment mode"]
    #[inline] pub fn pinc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PINC != 0"]
    #[inline] pub fn test_pinc(&self) -> bool {
        self.pinc() != 0
    }

    #[doc="Sets the PINC field."]
    #[inline] pub fn set_pinc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Memory increment mode"]
    #[inline] pub fn minc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MINC != 0"]
    #[inline] pub fn test_minc(&self) -> bool {
        self.minc() != 0
    }

    #[doc="Sets the MINC field."]
    #[inline] pub fn set_minc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Peripheral size"]
    #[inline] pub fn psize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if PSIZE != 0"]
    #[inline] pub fn test_psize(&self) -> bool {
        self.psize() != 0
    }

    #[doc="Sets the PSIZE field."]
    #[inline] pub fn set_psize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Memory size"]
    #[inline] pub fn msize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if MSIZE != 0"]
    #[inline] pub fn test_msize(&self) -> bool {
        self.msize() != 0
    }

    #[doc="Sets the MSIZE field."]
    #[inline] pub fn set_msize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Channel Priority level"]
    #[inline] pub fn pl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if PL != 0"]
    #[inline] pub fn test_pl(&self) -> bool {
        self.pl() != 0
    }

    #[doc="Sets the PL field."]
    #[inline] pub fn set_pl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Memory to memory mode"]
    #[inline] pub fn mem2mem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if MEM2MEM != 0"]
    #[inline] pub fn test_mem2mem(&self) -> bool {
        self.mem2mem() != 0
    }

    #[doc="Sets the MEM2MEM field."]
    #[inline] pub fn set_mem2mem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Ccr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr(other)
    }
}

impl ::core::fmt::Display for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.en() != 0 { try!(write!(f, " en"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.htie() != 0 { try!(write!(f, " htie"))}
        if self.teie() != 0 { try!(write!(f, " teie"))}
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.circ() != 0 { try!(write!(f, " circ"))}
        if self.pinc() != 0 { try!(write!(f, " pinc"))}
        if self.minc() != 0 { try!(write!(f, " minc"))}
        if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
        if self.msize() != 0 { try!(write!(f, " msize=0x{:x}", self.msize()))}
        if self.pl() != 0 { try!(write!(f, " pl=0x{:x}", self.pl()))}
        if self.mem2mem() != 0 { try!(write!(f, " mem2mem"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA channel n number of data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cndtr(pub u32);
impl Cndtr {
    #[doc="Number of data to transfer"]
    #[inline] pub fn ndt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if NDT != 0"]
    #[inline] pub fn test_ndt(&self) -> bool {
        self.ndt() != 0
    }

    #[doc="Sets the NDT field."]
    #[inline] pub fn set_ndt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cndtr {
    #[inline]
    fn from(other: u32) -> Self {
         Cndtr(other)
    }
}

impl ::core::fmt::Display for Cndtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cndtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ndt() != 0 { try!(write!(f, " ndt=0x{:x}", self.ndt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA channel n peripheral address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpar(pub u32);
impl Cpar {
    #[doc="Peripheral address"]
    #[inline] pub fn pa(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PA != 0"]
    #[inline] pub fn test_pa(&self) -> bool {
        self.pa() != 0
    }

    #[doc="Sets the PA field."]
    #[inline] pub fn set_pa<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cpar {
    #[inline]
    fn from(other: u32) -> Self {
         Cpar(other)
    }
}

impl ::core::fmt::Display for Cpar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cpar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA channel n memory address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmar(pub u32);
impl Cmar {
    #[doc="Memory address"]
    #[inline] pub fn ma(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MA != 0"]
    #[inline] pub fn test_ma(&self) -> bool {
        self.ma() != 0
    }

    #[doc="Sets the MA field."]
    #[inline] pub fn set_ma<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmar {
    #[inline]
    fn from(other: u32) -> Self {
         Cmar(other)
    }
}

impl ::core::fmt::Display for Cmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel selection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cselr(pub u32);
impl Cselr {
    #[doc="DMA channel selection"]
    #[inline] pub fn cs<I: Into<::bobbin_bits::R7>>(&self, index: I) -> ::bobbin_bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CS != 0"]
    #[inline] pub fn test_cs<I: Into<::bobbin_bits::R7>>(&self, index: I) -> bool{
        self.cs(index) != 0
    }

    #[doc="Sets the CS field."]
    #[inline] pub fn set_cs<I: Into<::bobbin_bits::R7>, V: Into<::bobbin_bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Cselr {
    #[inline]
    fn from(other: u32) -> Self {
         Cselr(other)
    }
}

impl ::core::fmt::Display for Cselr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cselr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cs(0) != 0 { try!(write!(f, " cs[0]=0x{:x}", self.cs(0)))}
        if self.cs(1) != 0 { try!(write!(f, " cs[1]=0x{:x}", self.cs(1)))}
        if self.cs(2) != 0 { try!(write!(f, " cs[2]=0x{:x}", self.cs(2)))}
        if self.cs(3) != 0 { try!(write!(f, " cs[3]=0x{:x}", self.cs(3)))}
        if self.cs(4) != 0 { try!(write!(f, " cs[4]=0x{:x}", self.cs(4)))}
        if self.cs(5) != 0 { try!(write!(f, " cs[5]=0x{:x}", self.cs(5)))}
        if self.cs(6) != 0 { try!(write!(f, " cs[6]=0x{:x}", self.cs(6)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

