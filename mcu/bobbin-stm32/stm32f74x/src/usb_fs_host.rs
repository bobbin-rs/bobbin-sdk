::bobbin_mcu::periph!( USB_FS_HOST, UsbFsHost, USB_FS_HOST_PERIPH, UsbFsHostPeriph, USB_FS_HOST_OWNED, USB_FS_HOST_REF_COUNT, 0x50000400, 0x00, 0x15);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("OTGFSRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for UsbFsHost {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2rstr().otgfsrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_otgfsrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("OTGFSEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for UsbFsHost {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2enr().otgfsen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_otgfsen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2LPENR"), field: Some("OTGFSLPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for UsbFsHost {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2lpenr().otgfslpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2lpenr(|r| r.set_otgfslpen(value));
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB_FS_HOST Peripheral"]
pub struct UsbFsHostPeriph(pub usize); 

impl UsbFsHostPeriph {
    #[doc="Get the HCFG Register."]
    #[inline] pub fn hcfg_reg(&self) -> ::bobbin_mcu::register::Register<Hcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hcfg, 0x0)
    }

    #[doc="Get the *mut pointer for the HCFG register."]
    #[inline] pub fn hcfg_mut(&self) -> *mut Hcfg { 
        self.hcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the HCFG register."]
    #[inline] pub fn hcfg_ptr(&self) -> *const Hcfg { 
        self.hcfg_reg().ptr()
    }

    #[doc="Read the HCFG register."]
    #[inline] pub fn hcfg(&self) -> Hcfg { 
        self.hcfg_reg().read()
    }

    #[doc="Write the HCFG register."]
    #[inline] pub fn write_hcfg(&self, value: Hcfg) -> &Self { 
        self.hcfg_reg().write(value);
        self
    }

    #[doc="Set the HCFG register."]
    #[inline] pub fn set_hcfg<F: FnOnce(Hcfg) -> Hcfg>(&self, f: F) -> &Self {
        self.hcfg_reg().set(f);
        self
    }

    #[doc="Modify the HCFG register."]
    #[inline] pub fn with_hcfg<F: FnOnce(Hcfg) -> Hcfg>(&self, f: F) -> &Self {
        self.hcfg_reg().with(f);
        self
    }

    #[doc="Get the HFIR Register."]
    #[inline] pub fn hfir_reg(&self) -> ::bobbin_mcu::register::Register<Hfir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hfir, 0x4)
    }

    #[doc="Get the *mut pointer for the HFIR register."]
    #[inline] pub fn hfir_mut(&self) -> *mut Hfir { 
        self.hfir_reg().ptr()
    }

    #[doc="Get the *const pointer for the HFIR register."]
    #[inline] pub fn hfir_ptr(&self) -> *const Hfir { 
        self.hfir_reg().ptr()
    }

    #[doc="Read the HFIR register."]
    #[inline] pub fn hfir(&self) -> Hfir { 
        self.hfir_reg().read()
    }

    #[doc="Write the HFIR register."]
    #[inline] pub fn write_hfir(&self, value: Hfir) -> &Self { 
        self.hfir_reg().write(value);
        self
    }

    #[doc="Set the HFIR register."]
    #[inline] pub fn set_hfir<F: FnOnce(Hfir) -> Hfir>(&self, f: F) -> &Self {
        self.hfir_reg().set(f);
        self
    }

    #[doc="Modify the HFIR register."]
    #[inline] pub fn with_hfir<F: FnOnce(Hfir) -> Hfir>(&self, f: F) -> &Self {
        self.hfir_reg().with(f);
        self
    }

    #[doc="Get the HFNUM Register."]
    #[inline] pub fn hfnum_reg(&self) -> ::bobbin_mcu::register::Register<Hfnum> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hfnum, 0x8)
    }

    #[doc="Get the *mut pointer for the HFNUM register."]
    #[inline] pub fn hfnum_mut(&self) -> *mut Hfnum { 
        self.hfnum_reg().ptr()
    }

    #[doc="Get the *const pointer for the HFNUM register."]
    #[inline] pub fn hfnum_ptr(&self) -> *const Hfnum { 
        self.hfnum_reg().ptr()
    }

    #[doc="Read the HFNUM register."]
    #[inline] pub fn hfnum(&self) -> Hfnum { 
        self.hfnum_reg().read()
    }

    #[doc="Get the HPTXSTS Register."]
    #[inline] pub fn hptxsts_reg(&self) -> ::bobbin_mcu::register::Register<Hptxsts> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hptxsts, 0x10)
    }

    #[doc="Get the *mut pointer for the HPTXSTS register."]
    #[inline] pub fn hptxsts_mut(&self) -> *mut Hptxsts { 
        self.hptxsts_reg().ptr()
    }

    #[doc="Get the *const pointer for the HPTXSTS register."]
    #[inline] pub fn hptxsts_ptr(&self) -> *const Hptxsts { 
        self.hptxsts_reg().ptr()
    }

    #[doc="Read the HPTXSTS register."]
    #[inline] pub fn hptxsts(&self) -> Hptxsts { 
        self.hptxsts_reg().read()
    }

    #[doc="Write the HPTXSTS register."]
    #[inline] pub fn write_hptxsts(&self, value: Hptxsts) -> &Self { 
        self.hptxsts_reg().write(value);
        self
    }

    #[doc="Set the HPTXSTS register."]
    #[inline] pub fn set_hptxsts<F: FnOnce(Hptxsts) -> Hptxsts>(&self, f: F) -> &Self {
        self.hptxsts_reg().set(f);
        self
    }

    #[doc="Modify the HPTXSTS register."]
    #[inline] pub fn with_hptxsts<F: FnOnce(Hptxsts) -> Hptxsts>(&self, f: F) -> &Self {
        self.hptxsts_reg().with(f);
        self
    }

    #[doc="Get the HAINT Register."]
    #[inline] pub fn haint_reg(&self) -> ::bobbin_mcu::register::Register<Haint> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Haint, 0x14)
    }

    #[doc="Get the *mut pointer for the HAINT register."]
    #[inline] pub fn haint_mut(&self) -> *mut Haint { 
        self.haint_reg().ptr()
    }

    #[doc="Get the *const pointer for the HAINT register."]
    #[inline] pub fn haint_ptr(&self) -> *const Haint { 
        self.haint_reg().ptr()
    }

    #[doc="Read the HAINT register."]
    #[inline] pub fn haint(&self) -> Haint { 
        self.haint_reg().read()
    }

    #[doc="Get the HAINTMSK Register."]
    #[inline] pub fn haintmsk_reg(&self) -> ::bobbin_mcu::register::Register<Haintmsk> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Haintmsk, 0x18)
    }

    #[doc="Get the *mut pointer for the HAINTMSK register."]
    #[inline] pub fn haintmsk_mut(&self) -> *mut Haintmsk { 
        self.haintmsk_reg().ptr()
    }

    #[doc="Get the *const pointer for the HAINTMSK register."]
    #[inline] pub fn haintmsk_ptr(&self) -> *const Haintmsk { 
        self.haintmsk_reg().ptr()
    }

    #[doc="Read the HAINTMSK register."]
    #[inline] pub fn haintmsk(&self) -> Haintmsk { 
        self.haintmsk_reg().read()
    }

    #[doc="Write the HAINTMSK register."]
    #[inline] pub fn write_haintmsk(&self, value: Haintmsk) -> &Self { 
        self.haintmsk_reg().write(value);
        self
    }

    #[doc="Set the HAINTMSK register."]
    #[inline] pub fn set_haintmsk<F: FnOnce(Haintmsk) -> Haintmsk>(&self, f: F) -> &Self {
        self.haintmsk_reg().set(f);
        self
    }

    #[doc="Modify the HAINTMSK register."]
    #[inline] pub fn with_haintmsk<F: FnOnce(Haintmsk) -> Haintmsk>(&self, f: F) -> &Self {
        self.haintmsk_reg().with(f);
        self
    }

    #[doc="Get the HPRT Register."]
    #[inline] pub fn hprt_reg(&self) -> ::bobbin_mcu::register::Register<Hprt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hprt, 0x40)
    }

    #[doc="Get the *mut pointer for the HPRT register."]
    #[inline] pub fn hprt_mut(&self) -> *mut Hprt { 
        self.hprt_reg().ptr()
    }

    #[doc="Get the *const pointer for the HPRT register."]
    #[inline] pub fn hprt_ptr(&self) -> *const Hprt { 
        self.hprt_reg().ptr()
    }

    #[doc="Read the HPRT register."]
    #[inline] pub fn hprt(&self) -> Hprt { 
        self.hprt_reg().read()
    }

    #[doc="Write the HPRT register."]
    #[inline] pub fn write_hprt(&self, value: Hprt) -> &Self { 
        self.hprt_reg().write(value);
        self
    }

    #[doc="Set the HPRT register."]
    #[inline] pub fn set_hprt<F: FnOnce(Hprt) -> Hprt>(&self, f: F) -> &Self {
        self.hprt_reg().set(f);
        self
    }

    #[doc="Modify the HPRT register."]
    #[inline] pub fn with_hprt<F: FnOnce(Hprt) -> Hprt>(&self, f: F) -> &Self {
        self.hprt_reg().with(f);
        self
    }

    #[doc="Get the HCCHAR Register."]
    #[inline] pub fn hcchar_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Hcchar, ::bobbin_bits::R12> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Hcchar, 0x100, 0x20)
    }

    #[doc="Get the *mut pointer for the HCCHAR register."]
    #[inline] pub fn hcchar_mut<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *mut Hcchar { 
        self.hcchar_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the HCCHAR register."]
    #[inline] pub fn hcchar_ptr<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *const Hcchar { 
        self.hcchar_reg().ptr(index.into())
    }

    #[doc="Read the HCCHAR register."]
    #[inline] pub fn hcchar<I: Into<::bobbin_bits::R12>>(&self, index: I) -> Hcchar { 
        self.hcchar_reg().read(index.into())
    }

    #[doc="Write the HCCHAR register."]
    #[inline] pub fn write_hcchar<I: Into<::bobbin_bits::R12>>(&self, index: I, value: Hcchar) -> &Self {
        self.hcchar_reg().write(index.into(), value);
        self
    }

    #[doc="Set the HCCHAR register."]
    #[inline] pub fn set_hcchar<I: Into<::bobbin_bits::R12>, F: FnOnce(Hcchar) -> Hcchar>(&self, index: I, f: F) -> &Self {
        self.hcchar_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the HCCHAR register."]
    #[inline] pub fn with_hcchar<I: Into<::bobbin_bits::R12> + Copy, F: FnOnce(Hcchar) -> Hcchar>(&self, index: I, f: F) -> &Self {
        self.hcchar_reg().with(index.into(), f);
        self
    }

    #[doc="Get the HCINT Register."]
    #[inline] pub fn hcint_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Hcint, ::bobbin_bits::R12> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Hcint, 0x108, 0x20)
    }

    #[doc="Get the *mut pointer for the HCINT register."]
    #[inline] pub fn hcint_mut<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *mut Hcint { 
        self.hcint_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the HCINT register."]
    #[inline] pub fn hcint_ptr<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *const Hcint { 
        self.hcint_reg().ptr(index.into())
    }

    #[doc="Read the HCINT register."]
    #[inline] pub fn hcint<I: Into<::bobbin_bits::R12>>(&self, index: I) -> Hcint { 
        self.hcint_reg().read(index.into())
    }

    #[doc="Write the HCINT register."]
    #[inline] pub fn write_hcint<I: Into<::bobbin_bits::R12>>(&self, index: I, value: Hcint) -> &Self {
        self.hcint_reg().write(index.into(), value);
        self
    }

    #[doc="Set the HCINT register."]
    #[inline] pub fn set_hcint<I: Into<::bobbin_bits::R12>, F: FnOnce(Hcint) -> Hcint>(&self, index: I, f: F) -> &Self {
        self.hcint_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the HCINT register."]
    #[inline] pub fn with_hcint<I: Into<::bobbin_bits::R12> + Copy, F: FnOnce(Hcint) -> Hcint>(&self, index: I, f: F) -> &Self {
        self.hcint_reg().with(index.into(), f);
        self
    }

    #[doc="Get the HCINTMSK Register."]
    #[inline] pub fn hcintmsk_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Hcintmsk, ::bobbin_bits::R12> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Hcintmsk, 0x10c, 0x20)
    }

    #[doc="Get the *mut pointer for the HCINTMSK register."]
    #[inline] pub fn hcintmsk_mut<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *mut Hcintmsk { 
        self.hcintmsk_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the HCINTMSK register."]
    #[inline] pub fn hcintmsk_ptr<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *const Hcintmsk { 
        self.hcintmsk_reg().ptr(index.into())
    }

    #[doc="Read the HCINTMSK register."]
    #[inline] pub fn hcintmsk<I: Into<::bobbin_bits::R12>>(&self, index: I) -> Hcintmsk { 
        self.hcintmsk_reg().read(index.into())
    }

    #[doc="Write the HCINTMSK register."]
    #[inline] pub fn write_hcintmsk<I: Into<::bobbin_bits::R12>>(&self, index: I, value: Hcintmsk) -> &Self {
        self.hcintmsk_reg().write(index.into(), value);
        self
    }

    #[doc="Set the HCINTMSK register."]
    #[inline] pub fn set_hcintmsk<I: Into<::bobbin_bits::R12>, F: FnOnce(Hcintmsk) -> Hcintmsk>(&self, index: I, f: F) -> &Self {
        self.hcintmsk_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the HCINTMSK register."]
    #[inline] pub fn with_hcintmsk<I: Into<::bobbin_bits::R12> + Copy, F: FnOnce(Hcintmsk) -> Hcintmsk>(&self, index: I, f: F) -> &Self {
        self.hcintmsk_reg().with(index.into(), f);
        self
    }

    #[doc="Get the HCTSIZ Register."]
    #[inline] pub fn hctsiz_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Hctsiz, ::bobbin_bits::R12> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Hctsiz, 0x110, 0x20)
    }

    #[doc="Get the *mut pointer for the HCTSIZ register."]
    #[inline] pub fn hctsiz_mut<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *mut Hctsiz { 
        self.hctsiz_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the HCTSIZ register."]
    #[inline] pub fn hctsiz_ptr<I: Into<::bobbin_bits::R12>>(&self, index: I) -> *const Hctsiz { 
        self.hctsiz_reg().ptr(index.into())
    }

    #[doc="Read the HCTSIZ register."]
    #[inline] pub fn hctsiz<I: Into<::bobbin_bits::R12>>(&self, index: I) -> Hctsiz { 
        self.hctsiz_reg().read(index.into())
    }

    #[doc="Write the HCTSIZ register."]
    #[inline] pub fn write_hctsiz<I: Into<::bobbin_bits::R12>>(&self, index: I, value: Hctsiz) -> &Self {
        self.hctsiz_reg().write(index.into(), value);
        self
    }

    #[doc="Set the HCTSIZ register."]
    #[inline] pub fn set_hctsiz<I: Into<::bobbin_bits::R12>, F: FnOnce(Hctsiz) -> Hctsiz>(&self, index: I, f: F) -> &Self {
        self.hctsiz_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the HCTSIZ register."]
    #[inline] pub fn with_hctsiz<I: Into<::bobbin_bits::R12> + Copy, F: FnOnce(Hctsiz) -> Hctsiz>(&self, index: I, f: F) -> &Self {
        self.hctsiz_reg().with(index.into(), f);
        self
    }

}

#[doc="OTG_FS host configuration register (OTG_FS_HCFG)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcfg(pub u32);
impl Hcfg {
    #[doc="FS/LS PHY clock select"]
    #[inline] pub fn fslspcs(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if FSLSPCS != 0"]
    #[inline] pub fn test_fslspcs(&self) -> bool {
        self.fslspcs() != 0
    }

    #[doc="Sets the FSLSPCS field."]
    #[inline] pub fn set_fslspcs<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FS- and LS-only support"]
    #[inline] pub fn fslss(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FSLSS != 0"]
    #[inline] pub fn test_fslss(&self) -> bool {
        self.fslss() != 0
    }

    #[doc="Sets the FSLSS field."]
    #[inline] pub fn set_fslss<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Hcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Hcfg(other)
    }
}

impl ::core::fmt::Display for Hcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fslspcs() != 0 { try!(write!(f, " fslspcs=0x{:x}", self.fslspcs()))}
        if self.fslss() != 0 { try!(write!(f, " fslss"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Host frame interval register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfir(pub u32);
impl Hfir {
    #[doc="Frame interval"]
    #[inline] pub fn frivl(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FRIVL != 0"]
    #[inline] pub fn test_frivl(&self) -> bool {
        self.frivl() != 0
    }

    #[doc="Sets the FRIVL field."]
    #[inline] pub fn set_frivl<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hfir {
    #[inline]
    fn from(other: u32) -> Self {
         Hfir(other)
    }
}

impl ::core::fmt::Display for Hfir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frivl() != 0 { try!(write!(f, " frivl=0x{:x}", self.frivl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfnum(pub u32);
impl Hfnum {
    #[doc="Frame number"]
    #[inline] pub fn frnum(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FRNUM != 0"]
    #[inline] pub fn test_frnum(&self) -> bool {
        self.frnum() != 0
    }

    #[doc="Sets the FRNUM field."]
    #[inline] pub fn set_frnum<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Frame time remaining"]
    #[inline] pub fn ftrem(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if FTREM != 0"]
    #[inline] pub fn test_ftrem(&self) -> bool {
        self.ftrem() != 0
    }

    #[doc="Sets the FTREM field."]
    #[inline] pub fn set_ftrem<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Hfnum {
    #[inline]
    fn from(other: u32) -> Self {
         Hfnum(other)
    }
}

impl ::core::fmt::Display for Hfnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frnum() != 0 { try!(write!(f, " frnum=0x{:x}", self.frnum()))}
        if self.ftrem() != 0 { try!(write!(f, " ftrem=0x{:x}", self.ftrem()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hptxsts(pub u32);
impl Hptxsts {
    #[doc="Periodic transmit data FIFO space available"]
    #[inline] pub fn ptxfsavl(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PTXFSAVL != 0"]
    #[inline] pub fn test_ptxfsavl(&self) -> bool {
        self.ptxfsavl() != 0
    }

    #[doc="Sets the PTXFSAVL field."]
    #[inline] pub fn set_ptxfsavl<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic transmit request queue space available"]
    #[inline] pub fn ptxqsav(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if PTXQSAV != 0"]
    #[inline] pub fn test_ptxqsav(&self) -> bool {
        self.ptxqsav() != 0
    }

    #[doc="Sets the PTXQSAV field."]
    #[inline] pub fn set_ptxqsav<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Top of the periodic transmit request queue"]
    #[inline] pub fn ptxqtop(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if PTXQTOP != 0"]
    #[inline] pub fn test_ptxqtop(&self) -> bool {
        self.ptxqtop() != 0
    }

    #[doc="Sets the PTXQTOP field."]
    #[inline] pub fn set_ptxqtop<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Hptxsts {
    #[inline]
    fn from(other: u32) -> Self {
         Hptxsts(other)
    }
}

impl ::core::fmt::Display for Hptxsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hptxsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptxfsavl() != 0 { try!(write!(f, " ptxfsavl=0x{:x}", self.ptxfsavl()))}
        if self.ptxqsav() != 0 { try!(write!(f, " ptxqsav=0x{:x}", self.ptxqsav()))}
        if self.ptxqtop() != 0 { try!(write!(f, " ptxqtop=0x{:x}", self.ptxqtop()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Host all channels interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Haint(pub u32);
impl Haint {
    #[doc="Channel interrupts"]
    #[inline] pub fn haint<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HAINT != 0"]
    #[inline] pub fn test_haint<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.haint(index) != 0
    }

    #[doc="Sets the HAINT field."]
    #[inline] pub fn set_haint<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Haint {
    #[inline]
    fn from(other: u32) -> Self {
         Haint(other)
    }
}

impl ::core::fmt::Display for Haint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Haint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.haint(0) != 0 { try!(write!(f, " haint[0]"))}
        if self.haint(1) != 0 { try!(write!(f, " haint[1]"))}
        if self.haint(2) != 0 { try!(write!(f, " haint[2]"))}
        if self.haint(3) != 0 { try!(write!(f, " haint[3]"))}
        if self.haint(4) != 0 { try!(write!(f, " haint[4]"))}
        if self.haint(5) != 0 { try!(write!(f, " haint[5]"))}
        if self.haint(6) != 0 { try!(write!(f, " haint[6]"))}
        if self.haint(7) != 0 { try!(write!(f, " haint[7]"))}
        if self.haint(8) != 0 { try!(write!(f, " haint[8]"))}
        if self.haint(9) != 0 { try!(write!(f, " haint[9]"))}
        if self.haint(10) != 0 { try!(write!(f, " haint[10]"))}
        if self.haint(11) != 0 { try!(write!(f, " haint[11]"))}
        if self.haint(12) != 0 { try!(write!(f, " haint[12]"))}
        if self.haint(13) != 0 { try!(write!(f, " haint[13]"))}
        if self.haint(14) != 0 { try!(write!(f, " haint[14]"))}
        if self.haint(15) != 0 { try!(write!(f, " haint[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host all channels interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Haintmsk(pub u32);
impl Haintmsk {
    #[doc="Channel interrupt mask"]
    #[inline] pub fn haintm<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HAINTM != 0"]
    #[inline] pub fn test_haintm<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.haintm(index) != 0
    }

    #[doc="Sets the HAINTM field."]
    #[inline] pub fn set_haintm<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Haintmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Haintmsk(other)
    }
}

impl ::core::fmt::Display for Haintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Haintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.haintm(0) != 0 { try!(write!(f, " haintm[0]"))}
        if self.haintm(1) != 0 { try!(write!(f, " haintm[1]"))}
        if self.haintm(2) != 0 { try!(write!(f, " haintm[2]"))}
        if self.haintm(3) != 0 { try!(write!(f, " haintm[3]"))}
        if self.haintm(4) != 0 { try!(write!(f, " haintm[4]"))}
        if self.haintm(5) != 0 { try!(write!(f, " haintm[5]"))}
        if self.haintm(6) != 0 { try!(write!(f, " haintm[6]"))}
        if self.haintm(7) != 0 { try!(write!(f, " haintm[7]"))}
        if self.haintm(8) != 0 { try!(write!(f, " haintm[8]"))}
        if self.haintm(9) != 0 { try!(write!(f, " haintm[9]"))}
        if self.haintm(10) != 0 { try!(write!(f, " haintm[10]"))}
        if self.haintm(11) != 0 { try!(write!(f, " haintm[11]"))}
        if self.haintm(12) != 0 { try!(write!(f, " haintm[12]"))}
        if self.haintm(13) != 0 { try!(write!(f, " haintm[13]"))}
        if self.haintm(14) != 0 { try!(write!(f, " haintm[14]"))}
        if self.haintm(15) != 0 { try!(write!(f, " haintm[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host port control and status register (OTG_FS_HPRT)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hprt(pub u32);
impl Hprt {
    #[doc="Port connect status"]
    #[inline] pub fn pcsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PCSTS != 0"]
    #[inline] pub fn test_pcsts(&self) -> bool {
        self.pcsts() != 0
    }

    #[doc="Sets the PCSTS field."]
    #[inline] pub fn set_pcsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port connect detected"]
    #[inline] pub fn pcdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PCDET != 0"]
    #[inline] pub fn test_pcdet(&self) -> bool {
        self.pcdet() != 0
    }

    #[doc="Sets the PCDET field."]
    #[inline] pub fn set_pcdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port enable"]
    #[inline] pub fn pena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PENA != 0"]
    #[inline] pub fn test_pena(&self) -> bool {
        self.pena() != 0
    }

    #[doc="Sets the PENA field."]
    #[inline] pub fn set_pena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port enable/disable change"]
    #[inline] pub fn penchng(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PENCHNG != 0"]
    #[inline] pub fn test_penchng(&self) -> bool {
        self.penchng() != 0
    }

    #[doc="Sets the PENCHNG field."]
    #[inline] pub fn set_penchng<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port overcurrent active"]
    #[inline] pub fn poca(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if POCA != 0"]
    #[inline] pub fn test_poca(&self) -> bool {
        self.poca() != 0
    }

    #[doc="Sets the POCA field."]
    #[inline] pub fn set_poca<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port overcurrent change"]
    #[inline] pub fn pocchng(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if POCCHNG != 0"]
    #[inline] pub fn test_pocchng(&self) -> bool {
        self.pocchng() != 0
    }

    #[doc="Sets the POCCHNG field."]
    #[inline] pub fn set_pocchng<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port resume"]
    #[inline] pub fn pres(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PRES != 0"]
    #[inline] pub fn test_pres(&self) -> bool {
        self.pres() != 0
    }

    #[doc="Sets the PRES field."]
    #[inline] pub fn set_pres<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port suspend"]
    #[inline] pub fn psusp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PSUSP != 0"]
    #[inline] pub fn test_psusp(&self) -> bool {
        self.psusp() != 0
    }

    #[doc="Sets the PSUSP field."]
    #[inline] pub fn set_psusp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port reset"]
    #[inline] pub fn prst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PRST != 0"]
    #[inline] pub fn test_prst(&self) -> bool {
        self.prst() != 0
    }

    #[doc="Sets the PRST field."]
    #[inline] pub fn set_prst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port line status"]
    #[inline] pub fn plsts(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if PLSTS != 0"]
    #[inline] pub fn test_plsts(&self) -> bool {
        self.plsts() != 0
    }

    #[doc="Sets the PLSTS field."]
    #[inline] pub fn set_plsts<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port power"]
    #[inline] pub fn ppwr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PPWR != 0"]
    #[inline] pub fn test_ppwr(&self) -> bool {
        self.ppwr() != 0
    }

    #[doc="Sets the PPWR field."]
    #[inline] pub fn set_ppwr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port test control"]
    #[inline] pub fn ptctl(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
    }

    #[doc="Returns true if PTCTL != 0"]
    #[inline] pub fn test_ptctl(&self) -> bool {
        self.ptctl() != 0
    }

    #[doc="Sets the PTCTL field."]
    #[inline] pub fn set_ptctl<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port speed"]
    #[inline] pub fn pspd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if PSPD != 0"]
    #[inline] pub fn test_pspd(&self) -> bool {
        self.pspd() != 0
    }

    #[doc="Sets the PSPD field."]
    #[inline] pub fn set_pspd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Hprt {
    #[inline]
    fn from(other: u32) -> Self {
         Hprt(other)
    }
}

impl ::core::fmt::Display for Hprt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hprt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcsts() != 0 { try!(write!(f, " pcsts"))}
        if self.pcdet() != 0 { try!(write!(f, " pcdet"))}
        if self.pena() != 0 { try!(write!(f, " pena"))}
        if self.penchng() != 0 { try!(write!(f, " penchng"))}
        if self.poca() != 0 { try!(write!(f, " poca"))}
        if self.pocchng() != 0 { try!(write!(f, " pocchng"))}
        if self.pres() != 0 { try!(write!(f, " pres"))}
        if self.psusp() != 0 { try!(write!(f, " psusp"))}
        if self.prst() != 0 { try!(write!(f, " prst"))}
        if self.plsts() != 0 { try!(write!(f, " plsts=0x{:x}", self.plsts()))}
        if self.ppwr() != 0 { try!(write!(f, " ppwr"))}
        if self.ptctl() != 0 { try!(write!(f, " ptctl=0x{:x}", self.ptctl()))}
        if self.pspd() != 0 { try!(write!(f, " pspd=0x{:x}", self.pspd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-n characteristics register (OTG_FS_HCCHARn)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar(pub u32);
impl Hcchar {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar(other)
    }
}

impl ::core::fmt::Display for Hcchar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint(pub u32);
impl Hcint {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint(other)
    }
}

impl ::core::fmt::Display for Hcint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-n mask register (OTG_FS_HCINTMSK)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk(pub u32);
impl Hcintmsk {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk(other)
    }
}

impl ::core::fmt::Display for Hcintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-0 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz(pub u32);
impl Hctsiz {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz(other)
    }
}

impl ::core::fmt::Display for Hctsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

