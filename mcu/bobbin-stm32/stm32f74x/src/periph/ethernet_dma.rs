#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Ethernet: DMA controller operation"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetDmaPeriph(pub usize);
impl EthernetDmaPeriph {
    #[doc="Get the DMABMR Register."]
    #[inline] pub fn dmabmr_reg(&self) -> Register<Dmabmr> { 
        Register::new(self.0 as *mut Dmabmr, 0x0)
    }

    #[doc="Get the *mut pointer for the DMABMR register."]
    #[inline] pub fn dmabmr_mut(&self) -> *mut Dmabmr { 
        self.dmabmr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMABMR register."]
    #[inline] pub fn dmabmr_ptr(&self) -> *const Dmabmr { 
        self.dmabmr_reg().ptr()
    }

    #[doc="Read the DMABMR register."]
    #[inline] pub fn dmabmr(&self) -> Dmabmr { 
        self.dmabmr_reg().read()
    }

    #[doc="Write the DMABMR register."]
    #[inline] pub fn write_dmabmr(&self, value: Dmabmr) -> &Self { 
        self.dmabmr_reg().write(value);
        self
    }

    #[doc="Set the DMABMR register."]
    #[inline] pub fn set_dmabmr<F: FnOnce(Dmabmr) -> Dmabmr>(&self, f: F) -> &Self {
        self.dmabmr_reg().set(f);
        self
    }

    #[doc="Modify the DMABMR register."]
    #[inline] pub fn with_dmabmr<F: FnOnce(Dmabmr) -> Dmabmr>(&self, f: F) -> &Self {
        self.dmabmr_reg().with(f);
        self
    }

    #[doc="Get the DMATPDR Register."]
    #[inline] pub fn dmatpdr_reg(&self) -> Register<Dmatpdr> { 
        Register::new(self.0 as *mut Dmatpdr, 0x4)
    }

    #[doc="Get the *mut pointer for the DMATPDR register."]
    #[inline] pub fn dmatpdr_mut(&self) -> *mut Dmatpdr { 
        self.dmatpdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMATPDR register."]
    #[inline] pub fn dmatpdr_ptr(&self) -> *const Dmatpdr { 
        self.dmatpdr_reg().ptr()
    }

    #[doc="Read the DMATPDR register."]
    #[inline] pub fn dmatpdr(&self) -> Dmatpdr { 
        self.dmatpdr_reg().read()
    }

    #[doc="Write the DMATPDR register."]
    #[inline] pub fn write_dmatpdr(&self, value: Dmatpdr) -> &Self { 
        self.dmatpdr_reg().write(value);
        self
    }

    #[doc="Set the DMATPDR register."]
    #[inline] pub fn set_dmatpdr<F: FnOnce(Dmatpdr) -> Dmatpdr>(&self, f: F) -> &Self {
        self.dmatpdr_reg().set(f);
        self
    }

    #[doc="Modify the DMATPDR register."]
    #[inline] pub fn with_dmatpdr<F: FnOnce(Dmatpdr) -> Dmatpdr>(&self, f: F) -> &Self {
        self.dmatpdr_reg().with(f);
        self
    }

    #[doc="Get the DMARPDR Register."]
    #[inline] pub fn dmarpdr_reg(&self) -> Register<Dmarpdr> { 
        Register::new(self.0 as *mut Dmarpdr, 0x8)
    }

    #[doc="Get the *mut pointer for the DMARPDR register."]
    #[inline] pub fn dmarpdr_mut(&self) -> *mut Dmarpdr { 
        self.dmarpdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMARPDR register."]
    #[inline] pub fn dmarpdr_ptr(&self) -> *const Dmarpdr { 
        self.dmarpdr_reg().ptr()
    }

    #[doc="Read the DMARPDR register."]
    #[inline] pub fn dmarpdr(&self) -> Dmarpdr { 
        self.dmarpdr_reg().read()
    }

    #[doc="Write the DMARPDR register."]
    #[inline] pub fn write_dmarpdr(&self, value: Dmarpdr) -> &Self { 
        self.dmarpdr_reg().write(value);
        self
    }

    #[doc="Set the DMARPDR register."]
    #[inline] pub fn set_dmarpdr<F: FnOnce(Dmarpdr) -> Dmarpdr>(&self, f: F) -> &Self {
        self.dmarpdr_reg().set(f);
        self
    }

    #[doc="Modify the DMARPDR register."]
    #[inline] pub fn with_dmarpdr<F: FnOnce(Dmarpdr) -> Dmarpdr>(&self, f: F) -> &Self {
        self.dmarpdr_reg().with(f);
        self
    }

    #[doc="Get the DMARDLAR Register."]
    #[inline] pub fn dmardlar_reg(&self) -> Register<Dmardlar> { 
        Register::new(self.0 as *mut Dmardlar, 0xc)
    }

    #[doc="Get the *mut pointer for the DMARDLAR register."]
    #[inline] pub fn dmardlar_mut(&self) -> *mut Dmardlar { 
        self.dmardlar_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMARDLAR register."]
    #[inline] pub fn dmardlar_ptr(&self) -> *const Dmardlar { 
        self.dmardlar_reg().ptr()
    }

    #[doc="Read the DMARDLAR register."]
    #[inline] pub fn dmardlar(&self) -> Dmardlar { 
        self.dmardlar_reg().read()
    }

    #[doc="Write the DMARDLAR register."]
    #[inline] pub fn write_dmardlar(&self, value: Dmardlar) -> &Self { 
        self.dmardlar_reg().write(value);
        self
    }

    #[doc="Set the DMARDLAR register."]
    #[inline] pub fn set_dmardlar<F: FnOnce(Dmardlar) -> Dmardlar>(&self, f: F) -> &Self {
        self.dmardlar_reg().set(f);
        self
    }

    #[doc="Modify the DMARDLAR register."]
    #[inline] pub fn with_dmardlar<F: FnOnce(Dmardlar) -> Dmardlar>(&self, f: F) -> &Self {
        self.dmardlar_reg().with(f);
        self
    }

    #[doc="Get the DMATDLAR Register."]
    #[inline] pub fn dmatdlar_reg(&self) -> Register<Dmatdlar> { 
        Register::new(self.0 as *mut Dmatdlar, 0x10)
    }

    #[doc="Get the *mut pointer for the DMATDLAR register."]
    #[inline] pub fn dmatdlar_mut(&self) -> *mut Dmatdlar { 
        self.dmatdlar_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMATDLAR register."]
    #[inline] pub fn dmatdlar_ptr(&self) -> *const Dmatdlar { 
        self.dmatdlar_reg().ptr()
    }

    #[doc="Read the DMATDLAR register."]
    #[inline] pub fn dmatdlar(&self) -> Dmatdlar { 
        self.dmatdlar_reg().read()
    }

    #[doc="Write the DMATDLAR register."]
    #[inline] pub fn write_dmatdlar(&self, value: Dmatdlar) -> &Self { 
        self.dmatdlar_reg().write(value);
        self
    }

    #[doc="Set the DMATDLAR register."]
    #[inline] pub fn set_dmatdlar<F: FnOnce(Dmatdlar) -> Dmatdlar>(&self, f: F) -> &Self {
        self.dmatdlar_reg().set(f);
        self
    }

    #[doc="Modify the DMATDLAR register."]
    #[inline] pub fn with_dmatdlar<F: FnOnce(Dmatdlar) -> Dmatdlar>(&self, f: F) -> &Self {
        self.dmatdlar_reg().with(f);
        self
    }

    #[doc="Get the DMASR Register."]
    #[inline] pub fn dmasr_reg(&self) -> Register<Dmasr> { 
        Register::new(self.0 as *mut Dmasr, 0x14)
    }

    #[doc="Get the *mut pointer for the DMASR register."]
    #[inline] pub fn dmasr_mut(&self) -> *mut Dmasr { 
        self.dmasr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMASR register."]
    #[inline] pub fn dmasr_ptr(&self) -> *const Dmasr { 
        self.dmasr_reg().ptr()
    }

    #[doc="Read the DMASR register."]
    #[inline] pub fn dmasr(&self) -> Dmasr { 
        self.dmasr_reg().read()
    }

    #[doc="Write the DMASR register."]
    #[inline] pub fn write_dmasr(&self, value: Dmasr) -> &Self { 
        self.dmasr_reg().write(value);
        self
    }

    #[doc="Set the DMASR register."]
    #[inline] pub fn set_dmasr<F: FnOnce(Dmasr) -> Dmasr>(&self, f: F) -> &Self {
        self.dmasr_reg().set(f);
        self
    }

    #[doc="Modify the DMASR register."]
    #[inline] pub fn with_dmasr<F: FnOnce(Dmasr) -> Dmasr>(&self, f: F) -> &Self {
        self.dmasr_reg().with(f);
        self
    }

    #[doc="Get the DMAOMR Register."]
    #[inline] pub fn dmaomr_reg(&self) -> Register<Dmaomr> { 
        Register::new(self.0 as *mut Dmaomr, 0x18)
    }

    #[doc="Get the *mut pointer for the DMAOMR register."]
    #[inline] pub fn dmaomr_mut(&self) -> *mut Dmaomr { 
        self.dmaomr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMAOMR register."]
    #[inline] pub fn dmaomr_ptr(&self) -> *const Dmaomr { 
        self.dmaomr_reg().ptr()
    }

    #[doc="Read the DMAOMR register."]
    #[inline] pub fn dmaomr(&self) -> Dmaomr { 
        self.dmaomr_reg().read()
    }

    #[doc="Write the DMAOMR register."]
    #[inline] pub fn write_dmaomr(&self, value: Dmaomr) -> &Self { 
        self.dmaomr_reg().write(value);
        self
    }

    #[doc="Set the DMAOMR register."]
    #[inline] pub fn set_dmaomr<F: FnOnce(Dmaomr) -> Dmaomr>(&self, f: F) -> &Self {
        self.dmaomr_reg().set(f);
        self
    }

    #[doc="Modify the DMAOMR register."]
    #[inline] pub fn with_dmaomr<F: FnOnce(Dmaomr) -> Dmaomr>(&self, f: F) -> &Self {
        self.dmaomr_reg().with(f);
        self
    }

    #[doc="Get the DMAIER Register."]
    #[inline] pub fn dmaier_reg(&self) -> Register<Dmaier> { 
        Register::new(self.0 as *mut Dmaier, 0x1c)
    }

    #[doc="Get the *mut pointer for the DMAIER register."]
    #[inline] pub fn dmaier_mut(&self) -> *mut Dmaier { 
        self.dmaier_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMAIER register."]
    #[inline] pub fn dmaier_ptr(&self) -> *const Dmaier { 
        self.dmaier_reg().ptr()
    }

    #[doc="Read the DMAIER register."]
    #[inline] pub fn dmaier(&self) -> Dmaier { 
        self.dmaier_reg().read()
    }

    #[doc="Write the DMAIER register."]
    #[inline] pub fn write_dmaier(&self, value: Dmaier) -> &Self { 
        self.dmaier_reg().write(value);
        self
    }

    #[doc="Set the DMAIER register."]
    #[inline] pub fn set_dmaier<F: FnOnce(Dmaier) -> Dmaier>(&self, f: F) -> &Self {
        self.dmaier_reg().set(f);
        self
    }

    #[doc="Modify the DMAIER register."]
    #[inline] pub fn with_dmaier<F: FnOnce(Dmaier) -> Dmaier>(&self, f: F) -> &Self {
        self.dmaier_reg().with(f);
        self
    }

    #[doc="Get the DMAMFBOCR Register."]
    #[inline] pub fn dmamfbocr_reg(&self) -> Register<Dmamfbocr> { 
        Register::new(self.0 as *mut Dmamfbocr, 0x20)
    }

    #[doc="Get the *mut pointer for the DMAMFBOCR register."]
    #[inline] pub fn dmamfbocr_mut(&self) -> *mut Dmamfbocr { 
        self.dmamfbocr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMAMFBOCR register."]
    #[inline] pub fn dmamfbocr_ptr(&self) -> *const Dmamfbocr { 
        self.dmamfbocr_reg().ptr()
    }

    #[doc="Read the DMAMFBOCR register."]
    #[inline] pub fn dmamfbocr(&self) -> Dmamfbocr { 
        self.dmamfbocr_reg().read()
    }

    #[doc="Write the DMAMFBOCR register."]
    #[inline] pub fn write_dmamfbocr(&self, value: Dmamfbocr) -> &Self { 
        self.dmamfbocr_reg().write(value);
        self
    }

    #[doc="Set the DMAMFBOCR register."]
    #[inline] pub fn set_dmamfbocr<F: FnOnce(Dmamfbocr) -> Dmamfbocr>(&self, f: F) -> &Self {
        self.dmamfbocr_reg().set(f);
        self
    }

    #[doc="Modify the DMAMFBOCR register."]
    #[inline] pub fn with_dmamfbocr<F: FnOnce(Dmamfbocr) -> Dmamfbocr>(&self, f: F) -> &Self {
        self.dmamfbocr_reg().with(f);
        self
    }

    #[doc="Get the DMARSWTR Register."]
    #[inline] pub fn dmarswtr_reg(&self) -> Register<Dmarswtr> { 
        Register::new(self.0 as *mut Dmarswtr, 0x24)
    }

    #[doc="Get the *mut pointer for the DMARSWTR register."]
    #[inline] pub fn dmarswtr_mut(&self) -> *mut Dmarswtr { 
        self.dmarswtr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMARSWTR register."]
    #[inline] pub fn dmarswtr_ptr(&self) -> *const Dmarswtr { 
        self.dmarswtr_reg().ptr()
    }

    #[doc="Read the DMARSWTR register."]
    #[inline] pub fn dmarswtr(&self) -> Dmarswtr { 
        self.dmarswtr_reg().read()
    }

    #[doc="Write the DMARSWTR register."]
    #[inline] pub fn write_dmarswtr(&self, value: Dmarswtr) -> &Self { 
        self.dmarswtr_reg().write(value);
        self
    }

    #[doc="Set the DMARSWTR register."]
    #[inline] pub fn set_dmarswtr<F: FnOnce(Dmarswtr) -> Dmarswtr>(&self, f: F) -> &Self {
        self.dmarswtr_reg().set(f);
        self
    }

    #[doc="Modify the DMARSWTR register."]
    #[inline] pub fn with_dmarswtr<F: FnOnce(Dmarswtr) -> Dmarswtr>(&self, f: F) -> &Self {
        self.dmarswtr_reg().with(f);
        self
    }

    #[doc="Get the DMACHTDR Register."]
    #[inline] pub fn dmachtdr_reg(&self) -> Register<Dmachtdr> { 
        Register::new(self.0 as *mut Dmachtdr, 0x48)
    }

    #[doc="Get the *mut pointer for the DMACHTDR register."]
    #[inline] pub fn dmachtdr_mut(&self) -> *mut Dmachtdr { 
        self.dmachtdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMACHTDR register."]
    #[inline] pub fn dmachtdr_ptr(&self) -> *const Dmachtdr { 
        self.dmachtdr_reg().ptr()
    }

    #[doc="Read the DMACHTDR register."]
    #[inline] pub fn dmachtdr(&self) -> Dmachtdr { 
        self.dmachtdr_reg().read()
    }

    #[doc="Get the DMACHRDR Register."]
    #[inline] pub fn dmachrdr_reg(&self) -> Register<Dmachrdr> { 
        Register::new(self.0 as *mut Dmachrdr, 0x4c)
    }

    #[doc="Get the *mut pointer for the DMACHRDR register."]
    #[inline] pub fn dmachrdr_mut(&self) -> *mut Dmachrdr { 
        self.dmachrdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMACHRDR register."]
    #[inline] pub fn dmachrdr_ptr(&self) -> *const Dmachrdr { 
        self.dmachrdr_reg().ptr()
    }

    #[doc="Read the DMACHRDR register."]
    #[inline] pub fn dmachrdr(&self) -> Dmachrdr { 
        self.dmachrdr_reg().read()
    }

    #[doc="Get the DMACHTBAR Register."]
    #[inline] pub fn dmachtbar_reg(&self) -> Register<Dmachtbar> { 
        Register::new(self.0 as *mut Dmachtbar, 0x50)
    }

    #[doc="Get the *mut pointer for the DMACHTBAR register."]
    #[inline] pub fn dmachtbar_mut(&self) -> *mut Dmachtbar { 
        self.dmachtbar_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMACHTBAR register."]
    #[inline] pub fn dmachtbar_ptr(&self) -> *const Dmachtbar { 
        self.dmachtbar_reg().ptr()
    }

    #[doc="Read the DMACHTBAR register."]
    #[inline] pub fn dmachtbar(&self) -> Dmachtbar { 
        self.dmachtbar_reg().read()
    }

    #[doc="Get the DMACHRBAR Register."]
    #[inline] pub fn dmachrbar_reg(&self) -> Register<Dmachrbar> { 
        Register::new(self.0 as *mut Dmachrbar, 0x54)
    }

    #[doc="Get the *mut pointer for the DMACHRBAR register."]
    #[inline] pub fn dmachrbar_mut(&self) -> *mut Dmachrbar { 
        self.dmachrbar_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMACHRBAR register."]
    #[inline] pub fn dmachrbar_ptr(&self) -> *const Dmachrbar { 
        self.dmachrbar_reg().ptr()
    }

    #[doc="Read the DMACHRBAR register."]
    #[inline] pub fn dmachrbar(&self) -> Dmachrbar { 
        self.dmachrbar_reg().read()
    }

}

#[doc="Ethernet DMA bus mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmabmr(pub u32);
impl Dmabmr {
    #[doc="no description available"]
    #[inline] pub fn sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SR != 0"]
    #[inline] pub fn test_sr(&self) -> bool {
        self.sr() != 0
    }

    #[doc="Sets the SR field."]
    #[inline] pub fn set_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn da(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DA != 0"]
    #[inline] pub fn test_da(&self) -> bool {
        self.da() != 0
    }

    #[doc="Sets the DA field."]
    #[inline] pub fn set_da<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn dsl(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1f) as u8) } // [6:2]
    }

    #[doc="Returns true if DSL != 0"]
    #[inline] pub fn test_dsl(&self) -> bool {
        self.dsl() != 0
    }

    #[doc="Sets the DSL field."]
    #[inline] pub fn set_dsl<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn edfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EDFE != 0"]
    #[inline] pub fn test_edfe(&self) -> bool {
        self.edfe() != 0
    }

    #[doc="Sets the EDFE field."]
    #[inline] pub fn set_edfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn pbl(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if PBL != 0"]
    #[inline] pub fn test_pbl(&self) -> bool {
        self.pbl() != 0
    }

    #[doc="Sets the PBL field."]
    #[inline] pub fn set_pbl<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rtpr(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if RTPR != 0"]
    #[inline] pub fn test_rtpr(&self) -> bool {
        self.rtpr() != 0
    }

    #[doc="Sets the RTPR field."]
    #[inline] pub fn set_rtpr<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn fb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FB != 0"]
    #[inline] pub fn test_fb(&self) -> bool {
        self.fb() != 0
    }

    #[doc="Sets the FB field."]
    #[inline] pub fn set_fb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rdp(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3f) as u8) } // [22:17]
    }

    #[doc="Returns true if RDP != 0"]
    #[inline] pub fn test_rdp(&self) -> bool {
        self.rdp() != 0
    }

    #[doc="Sets the RDP field."]
    #[inline] pub fn set_rdp<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn usp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if USP != 0"]
    #[inline] pub fn test_usp(&self) -> bool {
        self.usp() != 0
    }

    #[doc="Sets the USP field."]
    #[inline] pub fn set_usp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn fpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FPM != 0"]
    #[inline] pub fn test_fpm(&self) -> bool {
        self.fpm() != 0
    }

    #[doc="Sets the FPM field."]
    #[inline] pub fn set_fpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn aab(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if AAB != 0"]
    #[inline] pub fn test_aab(&self) -> bool {
        self.aab() != 0
    }

    #[doc="Sets the AAB field."]
    #[inline] pub fn set_aab<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if MB != 0"]
    #[inline] pub fn test_mb(&self) -> bool {
        self.mb() != 0
    }

    #[doc="Sets the MB field."]
    #[inline] pub fn set_mb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Dmabmr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmabmr(other)
    }
}

impl ::core::fmt::Display for Dmabmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmabmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sr() != 0 { try!(write!(f, " sr"))}
        if self.da() != 0 { try!(write!(f, " da"))}
        if self.dsl() != 0 { try!(write!(f, " dsl=0x{:x}", self.dsl()))}
        if self.edfe() != 0 { try!(write!(f, " edfe"))}
        if self.pbl() != 0 { try!(write!(f, " pbl=0x{:x}", self.pbl()))}
        if self.rtpr() != 0 { try!(write!(f, " rtpr=0x{:x}", self.rtpr()))}
        if self.fb() != 0 { try!(write!(f, " fb"))}
        if self.rdp() != 0 { try!(write!(f, " rdp=0x{:x}", self.rdp()))}
        if self.usp() != 0 { try!(write!(f, " usp"))}
        if self.fpm() != 0 { try!(write!(f, " fpm"))}
        if self.aab() != 0 { try!(write!(f, " aab"))}
        if self.mb() != 0 { try!(write!(f, " mb"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA transmit poll demand register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmatpdr(pub u32);
impl Dmatpdr {
    #[doc="no description available"]
    #[inline] pub fn tpd(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TPD != 0"]
    #[inline] pub fn test_tpd(&self) -> bool {
        self.tpd() != 0
    }

    #[doc="Sets the TPD field."]
    #[inline] pub fn set_tpd<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmatpdr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmatpdr(other)
    }
}

impl ::core::fmt::Display for Dmatpdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmatpdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EHERNET DMA receive poll demand register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmarpdr(pub u32);
impl Dmarpdr {
    #[doc="RPD"]
    #[inline] pub fn rpd(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RPD != 0"]
    #[inline] pub fn test_rpd(&self) -> bool {
        self.rpd() != 0
    }

    #[doc="Sets the RPD field."]
    #[inline] pub fn set_rpd<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmarpdr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmarpdr(other)
    }
}

impl ::core::fmt::Display for Dmarpdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmarpdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA receive descriptor list address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmardlar(pub u32);
impl Dmardlar {
    #[doc="no description available"]
    #[inline] pub fn srl(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SRL != 0"]
    #[inline] pub fn test_srl(&self) -> bool {
        self.srl() != 0
    }

    #[doc="Sets the SRL field."]
    #[inline] pub fn set_srl<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmardlar {
    #[inline]
    fn from(other: u32) -> Self {
         Dmardlar(other)
    }
}

impl ::core::fmt::Display for Dmardlar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmardlar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA transmit descriptor list address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmatdlar(pub u32);
impl Dmatdlar {
    #[doc="no description available"]
    #[inline] pub fn stl(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if STL != 0"]
    #[inline] pub fn test_stl(&self) -> bool {
        self.stl() != 0
    }

    #[doc="Sets the STL field."]
    #[inline] pub fn set_stl<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmatdlar {
    #[inline]
    fn from(other: u32) -> Self {
         Dmatdlar(other)
    }
}

impl ::core::fmt::Display for Dmatdlar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmatdlar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmasr(pub u32);
impl Dmasr {
    #[doc="no description available"]
    #[inline] pub fn ts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TS != 0"]
    #[inline] pub fn test_ts(&self) -> bool {
        self.ts() != 0
    }

    #[doc="Sets the TS field."]
    #[inline] pub fn set_ts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tpss(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TPSS != 0"]
    #[inline] pub fn test_tpss(&self) -> bool {
        self.tpss() != 0
    }

    #[doc="Sets the TPSS field."]
    #[inline] pub fn set_tpss<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tbus(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TBUS != 0"]
    #[inline] pub fn test_tbus(&self) -> bool {
        self.tbus() != 0
    }

    #[doc="Sets the TBUS field."]
    #[inline] pub fn set_tbus<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tjts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TJTS != 0"]
    #[inline] pub fn test_tjts(&self) -> bool {
        self.tjts() != 0
    }

    #[doc="Sets the TJTS field."]
    #[inline] pub fn set_tjts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ros(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ROS != 0"]
    #[inline] pub fn test_ros(&self) -> bool {
        self.ros() != 0
    }

    #[doc="Sets the ROS field."]
    #[inline] pub fn set_ros<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tus(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TUS != 0"]
    #[inline] pub fn test_tus(&self) -> bool {
        self.tus() != 0
    }

    #[doc="Sets the TUS field."]
    #[inline] pub fn set_tus<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RS != 0"]
    #[inline] pub fn test_rs(&self) -> bool {
        self.rs() != 0
    }

    #[doc="Sets the RS field."]
    #[inline] pub fn set_rs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rbus(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RBUS != 0"]
    #[inline] pub fn test_rbus(&self) -> bool {
        self.rbus() != 0
    }

    #[doc="Sets the RBUS field."]
    #[inline] pub fn set_rbus<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rpss(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RPSS != 0"]
    #[inline] pub fn test_rpss(&self) -> bool {
        self.rpss() != 0
    }

    #[doc="Sets the RPSS field."]
    #[inline] pub fn set_rpss<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn pwts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PWTS != 0"]
    #[inline] pub fn test_pwts(&self) -> bool {
        self.pwts() != 0
    }

    #[doc="Sets the PWTS field."]
    #[inline] pub fn set_pwts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ets(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ETS != 0"]
    #[inline] pub fn test_ets(&self) -> bool {
        self.ets() != 0
    }

    #[doc="Sets the ETS field."]
    #[inline] pub fn set_ets<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn fbes(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FBES != 0"]
    #[inline] pub fn test_fbes(&self) -> bool {
        self.fbes() != 0
    }

    #[doc="Sets the FBES field."]
    #[inline] pub fn set_fbes<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ers(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ERS != 0"]
    #[inline] pub fn test_ers(&self) -> bool {
        self.ers() != 0
    }

    #[doc="Sets the ERS field."]
    #[inline] pub fn set_ers<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ais(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if AIS != 0"]
    #[inline] pub fn test_ais(&self) -> bool {
        self.ais() != 0
    }

    #[doc="Sets the AIS field."]
    #[inline] pub fn set_ais<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn nis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if NIS != 0"]
    #[inline] pub fn test_nis(&self) -> bool {
        self.nis() != 0
    }

    #[doc="Sets the NIS field."]
    #[inline] pub fn set_nis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rps(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if RPS != 0"]
    #[inline] pub fn test_rps(&self) -> bool {
        self.rps() != 0
    }

    #[doc="Sets the RPS field."]
    #[inline] pub fn set_rps<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tps(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if TPS != 0"]
    #[inline] pub fn test_tps(&self) -> bool {
        self.tps() != 0
    }

    #[doc="Sets the TPS field."]
    #[inline] pub fn set_tps<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ebs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x7) as u8) } // [25:23]
    }

    #[doc="Returns true if EBS != 0"]
    #[inline] pub fn test_ebs(&self) -> bool {
        self.ebs() != 0
    }

    #[doc="Sets the EBS field."]
    #[inline] pub fn set_ebs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mmcs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if MMCS != 0"]
    #[inline] pub fn test_mmcs(&self) -> bool {
        self.mmcs() != 0
    }

    #[doc="Sets the MMCS field."]
    #[inline] pub fn set_mmcs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn pmts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PMTS != 0"]
    #[inline] pub fn test_pmts(&self) -> bool {
        self.pmts() != 0
    }

    #[doc="Sets the PMTS field."]
    #[inline] pub fn set_pmts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if TSTS != 0"]
    #[inline] pub fn test_tsts(&self) -> bool {
        self.tsts() != 0
    }

    #[doc="Sets the TSTS field."]
    #[inline] pub fn set_tsts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Dmasr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmasr(other)
    }
}

impl ::core::fmt::Display for Dmasr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmasr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ts() != 0 { try!(write!(f, " ts"))}
        if self.tpss() != 0 { try!(write!(f, " tpss"))}
        if self.tbus() != 0 { try!(write!(f, " tbus"))}
        if self.tjts() != 0 { try!(write!(f, " tjts"))}
        if self.ros() != 0 { try!(write!(f, " ros"))}
        if self.tus() != 0 { try!(write!(f, " tus"))}
        if self.rs() != 0 { try!(write!(f, " rs"))}
        if self.rbus() != 0 { try!(write!(f, " rbus"))}
        if self.rpss() != 0 { try!(write!(f, " rpss"))}
        if self.pwts() != 0 { try!(write!(f, " pwts"))}
        if self.ets() != 0 { try!(write!(f, " ets"))}
        if self.fbes() != 0 { try!(write!(f, " fbes"))}
        if self.ers() != 0 { try!(write!(f, " ers"))}
        if self.ais() != 0 { try!(write!(f, " ais"))}
        if self.nis() != 0 { try!(write!(f, " nis"))}
        if self.rps() != 0 { try!(write!(f, " rps=0x{:x}", self.rps()))}
        if self.tps() != 0 { try!(write!(f, " tps=0x{:x}", self.tps()))}
        if self.ebs() != 0 { try!(write!(f, " ebs=0x{:x}", self.ebs()))}
        if self.mmcs() != 0 { try!(write!(f, " mmcs"))}
        if self.pmts() != 0 { try!(write!(f, " pmts"))}
        if self.tsts() != 0 { try!(write!(f, " tsts"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA operation mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmaomr(pub u32);
impl Dmaomr {
    #[doc="SR"]
    #[inline] pub fn sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SR != 0"]
    #[inline] pub fn test_sr(&self) -> bool {
        self.sr() != 0
    }

    #[doc="Sets the SR field."]
    #[inline] pub fn set_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="OSF"]
    #[inline] pub fn osf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OSF != 0"]
    #[inline] pub fn test_osf(&self) -> bool {
        self.osf() != 0
    }

    #[doc="Sets the OSF field."]
    #[inline] pub fn set_osf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTC"]
    #[inline] pub fn rtc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if RTC != 0"]
    #[inline] pub fn test_rtc(&self) -> bool {
        self.rtc() != 0
    }

    #[doc="Sets the RTC field."]
    #[inline] pub fn set_rtc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FUGF"]
    #[inline] pub fn fugf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FUGF != 0"]
    #[inline] pub fn test_fugf(&self) -> bool {
        self.fugf() != 0
    }

    #[doc="Sets the FUGF field."]
    #[inline] pub fn set_fugf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="FEF"]
    #[inline] pub fn fef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FEF != 0"]
    #[inline] pub fn test_fef(&self) -> bool {
        self.fef() != 0
    }

    #[doc="Sets the FEF field."]
    #[inline] pub fn set_fef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ST"]
    #[inline] pub fn st(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ST != 0"]
    #[inline] pub fn test_st(&self) -> bool {
        self.st() != 0
    }

    #[doc="Sets the ST field."]
    #[inline] pub fn set_st<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TTC"]
    #[inline] pub fn ttc(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x7) as u8) } // [16:14]
    }

    #[doc="Returns true if TTC != 0"]
    #[inline] pub fn test_ttc(&self) -> bool {
        self.ttc() != 0
    }

    #[doc="Sets the TTC field."]
    #[inline] pub fn set_ttc<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="FTF"]
    #[inline] pub fn ftf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if FTF != 0"]
    #[inline] pub fn test_ftf(&self) -> bool {
        self.ftf() != 0
    }

    #[doc="Sets the FTF field."]
    #[inline] pub fn set_ftf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="TSF"]
    #[inline] pub fn tsf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if TSF != 0"]
    #[inline] pub fn test_tsf(&self) -> bool {
        self.tsf() != 0
    }

    #[doc="Sets the TSF field."]
    #[inline] pub fn set_tsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="DFRF"]
    #[inline] pub fn dfrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DFRF != 0"]
    #[inline] pub fn test_dfrf(&self) -> bool {
        self.dfrf() != 0
    }

    #[doc="Sets the DFRF field."]
    #[inline] pub fn set_dfrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="RSF"]
    #[inline] pub fn rsf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if RSF != 0"]
    #[inline] pub fn test_rsf(&self) -> bool {
        self.rsf() != 0
    }

    #[doc="Sets the RSF field."]
    #[inline] pub fn set_rsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DTCEFD"]
    #[inline] pub fn dtcefd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DTCEFD != 0"]
    #[inline] pub fn test_dtcefd(&self) -> bool {
        self.dtcefd() != 0
    }

    #[doc="Sets the DTCEFD field."]
    #[inline] pub fn set_dtcefd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Dmaomr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmaomr(other)
    }
}

impl ::core::fmt::Display for Dmaomr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmaomr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sr() != 0 { try!(write!(f, " sr"))}
        if self.osf() != 0 { try!(write!(f, " osf"))}
        if self.rtc() != 0 { try!(write!(f, " rtc=0x{:x}", self.rtc()))}
        if self.fugf() != 0 { try!(write!(f, " fugf"))}
        if self.fef() != 0 { try!(write!(f, " fef"))}
        if self.st() != 0 { try!(write!(f, " st"))}
        if self.ttc() != 0 { try!(write!(f, " ttc=0x{:x}", self.ttc()))}
        if self.ftf() != 0 { try!(write!(f, " ftf"))}
        if self.tsf() != 0 { try!(write!(f, " tsf"))}
        if self.dfrf() != 0 { try!(write!(f, " dfrf"))}
        if self.rsf() != 0 { try!(write!(f, " rsf"))}
        if self.dtcefd() != 0 { try!(write!(f, " dtcefd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmaier(pub u32);
impl Dmaier {
    #[doc="no description available"]
    #[inline] pub fn tie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tpsie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TPSIE != 0"]
    #[inline] pub fn test_tpsie(&self) -> bool {
        self.tpsie() != 0
    }

    #[doc="Sets the TPSIE field."]
    #[inline] pub fn set_tpsie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tbuie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TBUIE != 0"]
    #[inline] pub fn test_tbuie(&self) -> bool {
        self.tbuie() != 0
    }

    #[doc="Sets the TBUIE field."]
    #[inline] pub fn set_tbuie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tjtie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TJTIE != 0"]
    #[inline] pub fn test_tjtie(&self) -> bool {
        self.tjtie() != 0
    }

    #[doc="Sets the TJTIE field."]
    #[inline] pub fn set_tjtie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn roie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ROIE != 0"]
    #[inline] pub fn test_roie(&self) -> bool {
        self.roie() != 0
    }

    #[doc="Sets the ROIE field."]
    #[inline] pub fn set_roie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tuie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TUIE != 0"]
    #[inline] pub fn test_tuie(&self) -> bool {
        self.tuie() != 0
    }

    #[doc="Sets the TUIE field."]
    #[inline] pub fn set_tuie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RIE != 0"]
    #[inline] pub fn test_rie(&self) -> bool {
        self.rie() != 0
    }

    #[doc="Sets the RIE field."]
    #[inline] pub fn set_rie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rbuie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RBUIE != 0"]
    #[inline] pub fn test_rbuie(&self) -> bool {
        self.rbuie() != 0
    }

    #[doc="Sets the RBUIE field."]
    #[inline] pub fn set_rbuie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rpsie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RPSIE != 0"]
    #[inline] pub fn test_rpsie(&self) -> bool {
        self.rpsie() != 0
    }

    #[doc="Sets the RPSIE field."]
    #[inline] pub fn set_rpsie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rwtie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RWTIE != 0"]
    #[inline] pub fn test_rwtie(&self) -> bool {
        self.rwtie() != 0
    }

    #[doc="Sets the RWTIE field."]
    #[inline] pub fn set_rwtie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn etie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ETIE != 0"]
    #[inline] pub fn test_etie(&self) -> bool {
        self.etie() != 0
    }

    #[doc="Sets the ETIE field."]
    #[inline] pub fn set_etie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn fbeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FBEIE != 0"]
    #[inline] pub fn test_fbeie(&self) -> bool {
        self.fbeie() != 0
    }

    #[doc="Sets the FBEIE field."]
    #[inline] pub fn set_fbeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn erie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ERIE != 0"]
    #[inline] pub fn test_erie(&self) -> bool {
        self.erie() != 0
    }

    #[doc="Sets the ERIE field."]
    #[inline] pub fn set_erie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn aise(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if AISE != 0"]
    #[inline] pub fn test_aise(&self) -> bool {
        self.aise() != 0
    }

    #[doc="Sets the AISE field."]
    #[inline] pub fn set_aise<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn nise(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if NISE != 0"]
    #[inline] pub fn test_nise(&self) -> bool {
        self.nise() != 0
    }

    #[doc="Sets the NISE field."]
    #[inline] pub fn set_nise<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dmaier {
    #[inline]
    fn from(other: u32) -> Self {
         Dmaier(other)
    }
}

impl ::core::fmt::Display for Dmaier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmaier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.tpsie() != 0 { try!(write!(f, " tpsie"))}
        if self.tbuie() != 0 { try!(write!(f, " tbuie"))}
        if self.tjtie() != 0 { try!(write!(f, " tjtie"))}
        if self.roie() != 0 { try!(write!(f, " roie"))}
        if self.tuie() != 0 { try!(write!(f, " tuie"))}
        if self.rie() != 0 { try!(write!(f, " rie"))}
        if self.rbuie() != 0 { try!(write!(f, " rbuie"))}
        if self.rpsie() != 0 { try!(write!(f, " rpsie"))}
        if self.rwtie() != 0 { try!(write!(f, " rwtie"))}
        if self.etie() != 0 { try!(write!(f, " etie"))}
        if self.fbeie() != 0 { try!(write!(f, " fbeie"))}
        if self.erie() != 0 { try!(write!(f, " erie"))}
        if self.aise() != 0 { try!(write!(f, " aise"))}
        if self.nise() != 0 { try!(write!(f, " nise"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA missed frame and buffer overflow counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmamfbocr(pub u32);
impl Dmamfbocr {
    #[doc="no description available"]
    #[inline] pub fn mfc(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MFC != 0"]
    #[inline] pub fn test_mfc(&self) -> bool {
        self.mfc() != 0
    }

    #[doc="Sets the MFC field."]
    #[inline] pub fn set_mfc<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn omfc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if OMFC != 0"]
    #[inline] pub fn test_omfc(&self) -> bool {
        self.omfc() != 0
    }

    #[doc="Sets the OMFC field."]
    #[inline] pub fn set_omfc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mfa(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7ff) as u16) } // [27:17]
    }

    #[doc="Returns true if MFA != 0"]
    #[inline] pub fn test_mfa(&self) -> bool {
        self.mfa() != 0
    }

    #[doc="Sets the MFA field."]
    #[inline] pub fn set_mfa<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ofoc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if OFOC != 0"]
    #[inline] pub fn test_ofoc(&self) -> bool {
        self.ofoc() != 0
    }

    #[doc="Sets the OFOC field."]
    #[inline] pub fn set_ofoc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Dmamfbocr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmamfbocr(other)
    }
}

impl ::core::fmt::Display for Dmamfbocr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmamfbocr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mfc() != 0 { try!(write!(f, " mfc=0x{:x}", self.mfc()))}
        if self.omfc() != 0 { try!(write!(f, " omfc"))}
        if self.mfa() != 0 { try!(write!(f, " mfa=0x{:x}", self.mfa()))}
        if self.ofoc() != 0 { try!(write!(f, " ofoc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA receive status watchdog timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmarswtr(pub u32);
impl Dmarswtr {
    #[doc="RSWTC"]
    #[inline] pub fn rswtc(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RSWTC != 0"]
    #[inline] pub fn test_rswtc(&self) -> bool {
        self.rswtc() != 0
    }

    #[doc="Sets the RSWTC field."]
    #[inline] pub fn set_rswtc<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmarswtr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmarswtr(other)
    }
}

impl ::core::fmt::Display for Dmarswtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmarswtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rswtc() != 0 { try!(write!(f, " rswtc=0x{:x}", self.rswtc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA current host transmit descriptor register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmachtdr(pub u32);
impl Dmachtdr {
    #[doc="HTDAP"]
    #[inline] pub fn htdap(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if HTDAP != 0"]
    #[inline] pub fn test_htdap(&self) -> bool {
        self.htdap() != 0
    }

    #[doc="Sets the HTDAP field."]
    #[inline] pub fn set_htdap<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmachtdr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmachtdr(other)
    }
}

impl ::core::fmt::Display for Dmachtdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmachtdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA current host receive descriptor register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmachrdr(pub u32);
impl Dmachrdr {
    #[doc="HRDAP"]
    #[inline] pub fn hrdap(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if HRDAP != 0"]
    #[inline] pub fn test_hrdap(&self) -> bool {
        self.hrdap() != 0
    }

    #[doc="Sets the HRDAP field."]
    #[inline] pub fn set_hrdap<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmachrdr {
    #[inline]
    fn from(other: u32) -> Self {
         Dmachrdr(other)
    }
}

impl ::core::fmt::Display for Dmachrdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmachrdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA current host transmit buffer address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmachtbar(pub u32);
impl Dmachtbar {
    #[doc="no description available"]
    #[inline] pub fn htbap(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if HTBAP != 0"]
    #[inline] pub fn test_htbap(&self) -> bool {
        self.htbap() != 0
    }

    #[doc="Sets the HTBAP field."]
    #[inline] pub fn set_htbap<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmachtbar {
    #[inline]
    fn from(other: u32) -> Self {
         Dmachtbar(other)
    }
}

impl ::core::fmt::Display for Dmachtbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmachtbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet DMA current host receive buffer address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmachrbar(pub u32);
impl Dmachrbar {
    #[doc="no description available"]
    #[inline] pub fn hrbap(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if HRBAP != 0"]
    #[inline] pub fn test_hrbap(&self) -> bool {
        self.hrbap() != 0
    }

    #[doc="Sets the HRBAP field."]
    #[inline] pub fn set_hrbap<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmachrbar {
    #[inline]
    fn from(other: u32) -> Self {
         Dmachrbar(other)
    }
}

impl ::core::fmt::Display for Dmachrbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmachrbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

