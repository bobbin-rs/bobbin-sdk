::bobbin_mcu::periph!( TSC, Tsc, TSC_PERIPH, TscPeriph, TSC_OWNED, TSC_REF_COUNT, 0x40024000, 0x00, 0x1d);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TSC Peripheral"]
pub struct TscPeriph(pub usize); 

impl TscPeriph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the IER Register."]
    #[inline] pub fn ier_reg(&self) -> ::bobbin_mcu::register::Register<Ier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ier, 0x4)
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        self.ier_reg().read()
    }

    #[doc="Write the IER register."]
    #[inline] pub fn write_ier(&self, value: Ier) -> &Self { 
        self.ier_reg().write(value);
        self
    }

    #[doc="Set the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().set(f);
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().with(f);
        self
    }

    #[doc="Get the ICR Register."]
    #[inline] pub fn icr_reg(&self) -> ::bobbin_mcu::register::Register<Icr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Icr, 0x8)
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        self.icr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
        self.icr_reg().ptr()
    }

    #[doc="Read the ICR register."]
    #[inline] pub fn icr(&self) -> Icr { 
        self.icr_reg().read()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn write_icr(&self, value: Icr) -> &Self { 
        self.icr_reg().write(value);
        self
    }

    #[doc="Set the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        self.icr_reg().set(f);
        self
    }

    #[doc="Modify the ICR register."]
    #[inline] pub fn with_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        self.icr_reg().with(f);
        self
    }

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0xc)
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

    #[doc="Write the ISR register."]
    #[inline] pub fn write_isr(&self, value: Isr) -> &Self { 
        self.isr_reg().write(value);
        self
    }

    #[doc="Set the ISR register."]
    #[inline] pub fn set_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        self.isr_reg().set(f);
        self
    }

    #[doc="Modify the ISR register."]
    #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        self.isr_reg().with(f);
        self
    }

    #[doc="Get the IOHCR Register."]
    #[inline] pub fn iohcr_reg(&self) -> ::bobbin_mcu::register::Register<Iohcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iohcr, 0x10)
    }

    #[doc="Get the *mut pointer for the IOHCR register."]
    #[inline] pub fn iohcr_mut(&self) -> *mut Iohcr { 
        self.iohcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOHCR register."]
    #[inline] pub fn iohcr_ptr(&self) -> *const Iohcr { 
        self.iohcr_reg().ptr()
    }

    #[doc="Read the IOHCR register."]
    #[inline] pub fn iohcr(&self) -> Iohcr { 
        self.iohcr_reg().read()
    }

    #[doc="Write the IOHCR register."]
    #[inline] pub fn write_iohcr(&self, value: Iohcr) -> &Self { 
        self.iohcr_reg().write(value);
        self
    }

    #[doc="Set the IOHCR register."]
    #[inline] pub fn set_iohcr<F: FnOnce(Iohcr) -> Iohcr>(&self, f: F) -> &Self {
        self.iohcr_reg().set(f);
        self
    }

    #[doc="Modify the IOHCR register."]
    #[inline] pub fn with_iohcr<F: FnOnce(Iohcr) -> Iohcr>(&self, f: F) -> &Self {
        self.iohcr_reg().with(f);
        self
    }

    #[doc="Get the IOASCR Register."]
    #[inline] pub fn ioascr_reg(&self) -> ::bobbin_mcu::register::Register<Ioascr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ioascr, 0x18)
    }

    #[doc="Get the *mut pointer for the IOASCR register."]
    #[inline] pub fn ioascr_mut(&self) -> *mut Ioascr { 
        self.ioascr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOASCR register."]
    #[inline] pub fn ioascr_ptr(&self) -> *const Ioascr { 
        self.ioascr_reg().ptr()
    }

    #[doc="Read the IOASCR register."]
    #[inline] pub fn ioascr(&self) -> Ioascr { 
        self.ioascr_reg().read()
    }

    #[doc="Write the IOASCR register."]
    #[inline] pub fn write_ioascr(&self, value: Ioascr) -> &Self { 
        self.ioascr_reg().write(value);
        self
    }

    #[doc="Set the IOASCR register."]
    #[inline] pub fn set_ioascr<F: FnOnce(Ioascr) -> Ioascr>(&self, f: F) -> &Self {
        self.ioascr_reg().set(f);
        self
    }

    #[doc="Modify the IOASCR register."]
    #[inline] pub fn with_ioascr<F: FnOnce(Ioascr) -> Ioascr>(&self, f: F) -> &Self {
        self.ioascr_reg().with(f);
        self
    }

    #[doc="Get the IOSCR Register."]
    #[inline] pub fn ioscr_reg(&self) -> ::bobbin_mcu::register::Register<Ioscr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ioscr, 0x20)
    }

    #[doc="Get the *mut pointer for the IOSCR register."]
    #[inline] pub fn ioscr_mut(&self) -> *mut Ioscr { 
        self.ioscr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOSCR register."]
    #[inline] pub fn ioscr_ptr(&self) -> *const Ioscr { 
        self.ioscr_reg().ptr()
    }

    #[doc="Read the IOSCR register."]
    #[inline] pub fn ioscr(&self) -> Ioscr { 
        self.ioscr_reg().read()
    }

    #[doc="Write the IOSCR register."]
    #[inline] pub fn write_ioscr(&self, value: Ioscr) -> &Self { 
        self.ioscr_reg().write(value);
        self
    }

    #[doc="Set the IOSCR register."]
    #[inline] pub fn set_ioscr<F: FnOnce(Ioscr) -> Ioscr>(&self, f: F) -> &Self {
        self.ioscr_reg().set(f);
        self
    }

    #[doc="Modify the IOSCR register."]
    #[inline] pub fn with_ioscr<F: FnOnce(Ioscr) -> Ioscr>(&self, f: F) -> &Self {
        self.ioscr_reg().with(f);
        self
    }

    #[doc="Get the IOCCR Register."]
    #[inline] pub fn ioccr_reg(&self) -> ::bobbin_mcu::register::Register<Ioccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ioccr, 0x28)
    }

    #[doc="Get the *mut pointer for the IOCCR register."]
    #[inline] pub fn ioccr_mut(&self) -> *mut Ioccr { 
        self.ioccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOCCR register."]
    #[inline] pub fn ioccr_ptr(&self) -> *const Ioccr { 
        self.ioccr_reg().ptr()
    }

    #[doc="Read the IOCCR register."]
    #[inline] pub fn ioccr(&self) -> Ioccr { 
        self.ioccr_reg().read()
    }

    #[doc="Write the IOCCR register."]
    #[inline] pub fn write_ioccr(&self, value: Ioccr) -> &Self { 
        self.ioccr_reg().write(value);
        self
    }

    #[doc="Set the IOCCR register."]
    #[inline] pub fn set_ioccr<F: FnOnce(Ioccr) -> Ioccr>(&self, f: F) -> &Self {
        self.ioccr_reg().set(f);
        self
    }

    #[doc="Modify the IOCCR register."]
    #[inline] pub fn with_ioccr<F: FnOnce(Ioccr) -> Ioccr>(&self, f: F) -> &Self {
        self.ioccr_reg().with(f);
        self
    }

    #[doc="Get the IOGCSR Register."]
    #[inline] pub fn iogcsr_reg(&self) -> ::bobbin_mcu::register::Register<Iogcsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iogcsr, 0x30)
    }

    #[doc="Get the *mut pointer for the IOGCSR register."]
    #[inline] pub fn iogcsr_mut(&self) -> *mut Iogcsr { 
        self.iogcsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOGCSR register."]
    #[inline] pub fn iogcsr_ptr(&self) -> *const Iogcsr { 
        self.iogcsr_reg().ptr()
    }

    #[doc="Read the IOGCSR register."]
    #[inline] pub fn iogcsr(&self) -> Iogcsr { 
        self.iogcsr_reg().read()
    }

    #[doc="Write the IOGCSR register."]
    #[inline] pub fn write_iogcsr(&self, value: Iogcsr) -> &Self { 
        self.iogcsr_reg().write(value);
        self
    }

    #[doc="Set the IOGCSR register."]
    #[inline] pub fn set_iogcsr<F: FnOnce(Iogcsr) -> Iogcsr>(&self, f: F) -> &Self {
        self.iogcsr_reg().set(f);
        self
    }

    #[doc="Modify the IOGCSR register."]
    #[inline] pub fn with_iogcsr<F: FnOnce(Iogcsr) -> Iogcsr>(&self, f: F) -> &Self {
        self.iogcsr_reg().with(f);
        self
    }

    #[doc="Get the IOG1CR Register."]
    #[inline] pub fn iog1cr_reg(&self) -> ::bobbin_mcu::register::Register<Iog1cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iog1cr, 0x34)
    }

    #[doc="Get the *mut pointer for the IOG1CR register."]
    #[inline] pub fn iog1cr_mut(&self) -> *mut Iog1cr { 
        self.iog1cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOG1CR register."]
    #[inline] pub fn iog1cr_ptr(&self) -> *const Iog1cr { 
        self.iog1cr_reg().ptr()
    }

    #[doc="Read the IOG1CR register."]
    #[inline] pub fn iog1cr(&self) -> Iog1cr { 
        self.iog1cr_reg().read()
    }

    #[doc="Get the IOG2CR Register."]
    #[inline] pub fn iog2cr_reg(&self) -> ::bobbin_mcu::register::Register<Iog2cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iog2cr, 0x38)
    }

    #[doc="Get the *mut pointer for the IOG2CR register."]
    #[inline] pub fn iog2cr_mut(&self) -> *mut Iog2cr { 
        self.iog2cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOG2CR register."]
    #[inline] pub fn iog2cr_ptr(&self) -> *const Iog2cr { 
        self.iog2cr_reg().ptr()
    }

    #[doc="Read the IOG2CR register."]
    #[inline] pub fn iog2cr(&self) -> Iog2cr { 
        self.iog2cr_reg().read()
    }

    #[doc="Get the IOG3CR Register."]
    #[inline] pub fn iog3cr_reg(&self) -> ::bobbin_mcu::register::Register<Iog3cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iog3cr, 0x3c)
    }

    #[doc="Get the *mut pointer for the IOG3CR register."]
    #[inline] pub fn iog3cr_mut(&self) -> *mut Iog3cr { 
        self.iog3cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOG3CR register."]
    #[inline] pub fn iog3cr_ptr(&self) -> *const Iog3cr { 
        self.iog3cr_reg().ptr()
    }

    #[doc="Read the IOG3CR register."]
    #[inline] pub fn iog3cr(&self) -> Iog3cr { 
        self.iog3cr_reg().read()
    }

    #[doc="Get the IOG4CR Register."]
    #[inline] pub fn iog4cr_reg(&self) -> ::bobbin_mcu::register::Register<Iog4cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iog4cr, 0x40)
    }

    #[doc="Get the *mut pointer for the IOG4CR register."]
    #[inline] pub fn iog4cr_mut(&self) -> *mut Iog4cr { 
        self.iog4cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOG4CR register."]
    #[inline] pub fn iog4cr_ptr(&self) -> *const Iog4cr { 
        self.iog4cr_reg().ptr()
    }

    #[doc="Read the IOG4CR register."]
    #[inline] pub fn iog4cr(&self) -> Iog4cr { 
        self.iog4cr_reg().read()
    }

    #[doc="Get the IOG5CR Register."]
    #[inline] pub fn iog5cr_reg(&self) -> ::bobbin_mcu::register::Register<Iog5cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iog5cr, 0x44)
    }

    #[doc="Get the *mut pointer for the IOG5CR register."]
    #[inline] pub fn iog5cr_mut(&self) -> *mut Iog5cr { 
        self.iog5cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOG5CR register."]
    #[inline] pub fn iog5cr_ptr(&self) -> *const Iog5cr { 
        self.iog5cr_reg().ptr()
    }

    #[doc="Read the IOG5CR register."]
    #[inline] pub fn iog5cr(&self) -> Iog5cr { 
        self.iog5cr_reg().read()
    }

    #[doc="Get the IOG6CR Register."]
    #[inline] pub fn iog6cr_reg(&self) -> ::bobbin_mcu::register::Register<Iog6cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iog6cr, 0x48)
    }

    #[doc="Get the *mut pointer for the IOG6CR register."]
    #[inline] pub fn iog6cr_mut(&self) -> *mut Iog6cr { 
        self.iog6cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOG6CR register."]
    #[inline] pub fn iog6cr_ptr(&self) -> *const Iog6cr { 
        self.iog6cr_reg().ptr()
    }

    #[doc="Read the IOG6CR register."]
    #[inline] pub fn iog6cr(&self) -> Iog6cr { 
        self.iog6cr_reg().read()
    }

    #[doc="Get the IOG7CR Register."]
    #[inline] pub fn iog7cr_reg(&self) -> ::bobbin_mcu::register::Register<Iog7cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Iog7cr, 0x4c)
    }

    #[doc="Get the *mut pointer for the IOG7CR register."]
    #[inline] pub fn iog7cr_mut(&self) -> *mut Iog7cr { 
        self.iog7cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOG7CR register."]
    #[inline] pub fn iog7cr_ptr(&self) -> *const Iog7cr { 
        self.iog7cr_reg().ptr()
    }

    #[doc="Read the IOG7CR register."]
    #[inline] pub fn iog7cr(&self) -> Iog7cr { 
        self.iog7cr_reg().read()
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Charge transfer pulse high"]
    #[inline] pub fn ctph(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if CTPH != 0"]
    #[inline] pub fn test_ctph(&self) -> bool {
        self.ctph() != 0
    }

    #[doc="Sets the CTPH field."]
    #[inline] pub fn set_ctph<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Charge transfer pulse low"]
    #[inline] pub fn ctpl(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if CTPL != 0"]
    #[inline] pub fn test_ctpl(&self) -> bool {
        self.ctpl() != 0
    }

    #[doc="Sets the CTPL field."]
    #[inline] pub fn set_ctpl<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Spread spectrum deviation"]
    #[inline] pub fn ssd(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7f) as u8) } // [23:17]
    }

    #[doc="Returns true if SSD != 0"]
    #[inline] pub fn test_ssd(&self) -> bool {
        self.ssd() != 0
    }

    #[doc="Sets the SSD field."]
    #[inline] pub fn set_ssd<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Spread spectrum enable"]
    #[inline] pub fn sse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SSE != 0"]
    #[inline] pub fn test_sse(&self) -> bool {
        self.sse() != 0
    }

    #[doc="Sets the SSE field."]
    #[inline] pub fn set_sse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Spread spectrum prescaler"]
    #[inline] pub fn sspsc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SSPSC != 0"]
    #[inline] pub fn test_sspsc(&self) -> bool {
        self.sspsc() != 0
    }

    #[doc="Sets the SSPSC field."]
    #[inline] pub fn set_sspsc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="pulse generator prescaler"]
    #[inline] pub fn pgpsc(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if PGPSC != 0"]
    #[inline] pub fn test_pgpsc(&self) -> bool {
        self.pgpsc() != 0
    }

    #[doc="Sets the PGPSC field."]
    #[inline] pub fn set_pgpsc<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Max count value"]
    #[inline] pub fn mcv(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if MCV != 0"]
    #[inline] pub fn test_mcv(&self) -> bool {
        self.mcv() != 0
    }

    #[doc="Sets the MCV field."]
    #[inline] pub fn set_mcv<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I/O Default mode"]
    #[inline] pub fn iodef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IODEF != 0"]
    #[inline] pub fn test_iodef(&self) -> bool {
        self.iodef() != 0
    }

    #[doc="Sets the IODEF field."]
    #[inline] pub fn set_iodef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization pin polarity"]
    #[inline] pub fn syncpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCPOL != 0"]
    #[inline] pub fn test_syncpol(&self) -> bool {
        self.syncpol() != 0
    }

    #[doc="Sets the SYNCPOL field."]
    #[inline] pub fn set_syncpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Acquisition mode"]
    #[inline] pub fn am(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AM != 0"]
    #[inline] pub fn test_am(&self) -> bool {
        self.am() != 0
    }

    #[doc="Sets the AM field."]
    #[inline] pub fn set_am<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Start a new acquisition"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Touch sensing controller enable"]
    #[inline] pub fn tsce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TSCE != 0"]
    #[inline] pub fn test_tsce(&self) -> bool {
        self.tsce() != 0
    }

    #[doc="Sets the TSCE field."]
    #[inline] pub fn set_tsce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctph() != 0 { try!(write!(f, " ctph=0x{:x}", self.ctph()))}
        if self.ctpl() != 0 { try!(write!(f, " ctpl=0x{:x}", self.ctpl()))}
        if self.ssd() != 0 { try!(write!(f, " ssd=0x{:x}", self.ssd()))}
        if self.sse() != 0 { try!(write!(f, " sse"))}
        if self.sspsc() != 0 { try!(write!(f, " sspsc"))}
        if self.pgpsc() != 0 { try!(write!(f, " pgpsc=0x{:x}", self.pgpsc()))}
        if self.mcv() != 0 { try!(write!(f, " mcv=0x{:x}", self.mcv()))}
        if self.iodef() != 0 { try!(write!(f, " iodef"))}
        if self.syncpol() != 0 { try!(write!(f, " syncpol"))}
        if self.am() != 0 { try!(write!(f, " am"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.tsce() != 0 { try!(write!(f, " tsce"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Max count error interrupt enable"]
    #[inline] pub fn mceie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MCEIE != 0"]
    #[inline] pub fn test_mceie(&self) -> bool {
        self.mceie() != 0
    }

    #[doc="Sets the MCEIE field."]
    #[inline] pub fn set_mceie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of acquisition interrupt enable"]
    #[inline] pub fn eoaie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EOAIE != 0"]
    #[inline] pub fn test_eoaie(&self) -> bool {
        self.eoaie() != 0
    }

    #[doc="Sets the EOAIE field."]
    #[inline] pub fn set_eoaie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mceie() != 0 { try!(write!(f, " mceie"))}
        if self.eoaie() != 0 { try!(write!(f, " eoaie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Max count error interrupt clear"]
    #[inline] pub fn mceic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MCEIC != 0"]
    #[inline] pub fn test_mceic(&self) -> bool {
        self.mceic() != 0
    }

    #[doc="Sets the MCEIC field."]
    #[inline] pub fn set_mceic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of acquisition interrupt clear"]
    #[inline] pub fn eoaic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EOAIC != 0"]
    #[inline] pub fn test_eoaic(&self) -> bool {
        self.eoaic() != 0
    }

    #[doc="Sets the EOAIC field."]
    #[inline] pub fn set_eoaic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
    }
}

impl ::core::fmt::Display for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mceic() != 0 { try!(write!(f, " mceic"))}
        if self.eoaic() != 0 { try!(write!(f, " eoaic"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Max count error flag"]
    #[inline] pub fn mcef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MCEF != 0"]
    #[inline] pub fn test_mcef(&self) -> bool {
        self.mcef() != 0
    }

    #[doc="Sets the MCEF field."]
    #[inline] pub fn set_mcef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of acquisition flag"]
    #[inline] pub fn eoaf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EOAF != 0"]
    #[inline] pub fn test_eoaf(&self) -> bool {
        self.eoaf() != 0
    }

    #[doc="Sets the EOAF field."]
    #[inline] pub fn set_eoaf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.mcef() != 0 { try!(write!(f, " mcef"))}
        if self.eoaf() != 0 { try!(write!(f, " eoaf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O hysteresis control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iohcr(pub u32);
impl Iohcr {
    #[doc="G7_IO4"]
    #[inline] pub fn g7_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if G7_IO4 != 0"]
    #[inline] pub fn test_g7_io4(&self) -> bool {
        self.g7_io4() != 0
    }

    #[doc="Sets the G7_IO4 field."]
    #[inline] pub fn set_g7_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="G7_IO3"]
    #[inline] pub fn g7_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if G7_IO3 != 0"]
    #[inline] pub fn test_g7_io3(&self) -> bool {
        self.g7_io3() != 0
    }

    #[doc="Sets the G7_IO3 field."]
    #[inline] pub fn set_g7_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="G7_IO2"]
    #[inline] pub fn g7_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if G7_IO2 != 0"]
    #[inline] pub fn test_g7_io2(&self) -> bool {
        self.g7_io2() != 0
    }

    #[doc="Sets the G7_IO2 field."]
    #[inline] pub fn set_g7_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="G7_IO1"]
    #[inline] pub fn g7_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if G7_IO1 != 0"]
    #[inline] pub fn test_g7_io1(&self) -> bool {
        self.g7_io1() != 0
    }

    #[doc="Sets the G7_IO1 field."]
    #[inline] pub fn set_g7_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="G6_IO4"]
    #[inline] pub fn g6_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if G6_IO4 != 0"]
    #[inline] pub fn test_g6_io4(&self) -> bool {
        self.g6_io4() != 0
    }

    #[doc="Sets the G6_IO4 field."]
    #[inline] pub fn set_g6_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="G6_IO3"]
    #[inline] pub fn g6_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if G6_IO3 != 0"]
    #[inline] pub fn test_g6_io3(&self) -> bool {
        self.g6_io3() != 0
    }

    #[doc="Sets the G6_IO3 field."]
    #[inline] pub fn set_g6_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="G6_IO2"]
    #[inline] pub fn g6_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if G6_IO2 != 0"]
    #[inline] pub fn test_g6_io2(&self) -> bool {
        self.g6_io2() != 0
    }

    #[doc="Sets the G6_IO2 field."]
    #[inline] pub fn set_g6_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="G6_IO1"]
    #[inline] pub fn g6_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if G6_IO1 != 0"]
    #[inline] pub fn test_g6_io1(&self) -> bool {
        self.g6_io1() != 0
    }

    #[doc="Sets the G6_IO1 field."]
    #[inline] pub fn set_g6_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="G5_IO4"]
    #[inline] pub fn g5_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if G5_IO4 != 0"]
    #[inline] pub fn test_g5_io4(&self) -> bool {
        self.g5_io4() != 0
    }

    #[doc="Sets the G5_IO4 field."]
    #[inline] pub fn set_g5_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="G5_IO3"]
    #[inline] pub fn g5_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if G5_IO3 != 0"]
    #[inline] pub fn test_g5_io3(&self) -> bool {
        self.g5_io3() != 0
    }

    #[doc="Sets the G5_IO3 field."]
    #[inline] pub fn set_g5_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="G5_IO2"]
    #[inline] pub fn g5_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if G5_IO2 != 0"]
    #[inline] pub fn test_g5_io2(&self) -> bool {
        self.g5_io2() != 0
    }

    #[doc="Sets the G5_IO2 field."]
    #[inline] pub fn set_g5_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="G5_IO1"]
    #[inline] pub fn g5_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if G5_IO1 != 0"]
    #[inline] pub fn test_g5_io1(&self) -> bool {
        self.g5_io1() != 0
    }

    #[doc="Sets the G5_IO1 field."]
    #[inline] pub fn set_g5_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="G4_IO4"]
    #[inline] pub fn g4_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if G4_IO4 != 0"]
    #[inline] pub fn test_g4_io4(&self) -> bool {
        self.g4_io4() != 0
    }

    #[doc="Sets the G4_IO4 field."]
    #[inline] pub fn set_g4_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="G4_IO3"]
    #[inline] pub fn g4_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if G4_IO3 != 0"]
    #[inline] pub fn test_g4_io3(&self) -> bool {
        self.g4_io3() != 0
    }

    #[doc="Sets the G4_IO3 field."]
    #[inline] pub fn set_g4_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="G4_IO2"]
    #[inline] pub fn g4_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if G4_IO2 != 0"]
    #[inline] pub fn test_g4_io2(&self) -> bool {
        self.g4_io2() != 0
    }

    #[doc="Sets the G4_IO2 field."]
    #[inline] pub fn set_g4_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="G4_IO1"]
    #[inline] pub fn g4_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if G4_IO1 != 0"]
    #[inline] pub fn test_g4_io1(&self) -> bool {
        self.g4_io1() != 0
    }

    #[doc="Sets the G4_IO1 field."]
    #[inline] pub fn set_g4_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="G3_IO4"]
    #[inline] pub fn g3_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if G3_IO4 != 0"]
    #[inline] pub fn test_g3_io4(&self) -> bool {
        self.g3_io4() != 0
    }

    #[doc="Sets the G3_IO4 field."]
    #[inline] pub fn set_g3_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="G3_IO3"]
    #[inline] pub fn g3_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if G3_IO3 != 0"]
    #[inline] pub fn test_g3_io3(&self) -> bool {
        self.g3_io3() != 0
    }

    #[doc="Sets the G3_IO3 field."]
    #[inline] pub fn set_g3_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="G3_IO2"]
    #[inline] pub fn g3_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if G3_IO2 != 0"]
    #[inline] pub fn test_g3_io2(&self) -> bool {
        self.g3_io2() != 0
    }

    #[doc="Sets the G3_IO2 field."]
    #[inline] pub fn set_g3_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="G3_IO1"]
    #[inline] pub fn g3_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if G3_IO1 != 0"]
    #[inline] pub fn test_g3_io1(&self) -> bool {
        self.g3_io1() != 0
    }

    #[doc="Sets the G3_IO1 field."]
    #[inline] pub fn set_g3_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="G2_IO4"]
    #[inline] pub fn g2_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if G2_IO4 != 0"]
    #[inline] pub fn test_g2_io4(&self) -> bool {
        self.g2_io4() != 0
    }

    #[doc="Sets the G2_IO4 field."]
    #[inline] pub fn set_g2_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="G2_IO3"]
    #[inline] pub fn g2_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if G2_IO3 != 0"]
    #[inline] pub fn test_g2_io3(&self) -> bool {
        self.g2_io3() != 0
    }

    #[doc="Sets the G2_IO3 field."]
    #[inline] pub fn set_g2_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="G2_IO2"]
    #[inline] pub fn g2_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if G2_IO2 != 0"]
    #[inline] pub fn test_g2_io2(&self) -> bool {
        self.g2_io2() != 0
    }

    #[doc="Sets the G2_IO2 field."]
    #[inline] pub fn set_g2_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="G2_IO1"]
    #[inline] pub fn g2_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if G2_IO1 != 0"]
    #[inline] pub fn test_g2_io1(&self) -> bool {
        self.g2_io1() != 0
    }

    #[doc="Sets the G2_IO1 field."]
    #[inline] pub fn set_g2_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="G1_IO4"]
    #[inline] pub fn g1_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if G1_IO4 != 0"]
    #[inline] pub fn test_g1_io4(&self) -> bool {
        self.g1_io4() != 0
    }

    #[doc="Sets the G1_IO4 field."]
    #[inline] pub fn set_g1_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="G1_IO3"]
    #[inline] pub fn g1_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if G1_IO3 != 0"]
    #[inline] pub fn test_g1_io3(&self) -> bool {
        self.g1_io3() != 0
    }

    #[doc="Sets the G1_IO3 field."]
    #[inline] pub fn set_g1_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="G1_IO2"]
    #[inline] pub fn g1_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if G1_IO2 != 0"]
    #[inline] pub fn test_g1_io2(&self) -> bool {
        self.g1_io2() != 0
    }

    #[doc="Sets the G1_IO2 field."]
    #[inline] pub fn set_g1_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="G1_IO1"]
    #[inline] pub fn g1_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if G1_IO1 != 0"]
    #[inline] pub fn test_g1_io1(&self) -> bool {
        self.g1_io1() != 0
    }

    #[doc="Sets the G1_IO1 field."]
    #[inline] pub fn set_g1_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iohcr {
    #[inline]
    fn from(other: u32) -> Self {
         Iohcr(other)
    }
}

impl ::core::fmt::Display for Iohcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iohcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.g7_io4() != 0 { try!(write!(f, " g7_io4"))}
        if self.g7_io3() != 0 { try!(write!(f, " g7_io3"))}
        if self.g7_io2() != 0 { try!(write!(f, " g7_io2"))}
        if self.g7_io1() != 0 { try!(write!(f, " g7_io1"))}
        if self.g6_io4() != 0 { try!(write!(f, " g6_io4"))}
        if self.g6_io3() != 0 { try!(write!(f, " g6_io3"))}
        if self.g6_io2() != 0 { try!(write!(f, " g6_io2"))}
        if self.g6_io1() != 0 { try!(write!(f, " g6_io1"))}
        if self.g5_io4() != 0 { try!(write!(f, " g5_io4"))}
        if self.g5_io3() != 0 { try!(write!(f, " g5_io3"))}
        if self.g5_io2() != 0 { try!(write!(f, " g5_io2"))}
        if self.g5_io1() != 0 { try!(write!(f, " g5_io1"))}
        if self.g4_io4() != 0 { try!(write!(f, " g4_io4"))}
        if self.g4_io3() != 0 { try!(write!(f, " g4_io3"))}
        if self.g4_io2() != 0 { try!(write!(f, " g4_io2"))}
        if self.g4_io1() != 0 { try!(write!(f, " g4_io1"))}
        if self.g3_io4() != 0 { try!(write!(f, " g3_io4"))}
        if self.g3_io3() != 0 { try!(write!(f, " g3_io3"))}
        if self.g3_io2() != 0 { try!(write!(f, " g3_io2"))}
        if self.g3_io1() != 0 { try!(write!(f, " g3_io1"))}
        if self.g2_io4() != 0 { try!(write!(f, " g2_io4"))}
        if self.g2_io3() != 0 { try!(write!(f, " g2_io3"))}
        if self.g2_io2() != 0 { try!(write!(f, " g2_io2"))}
        if self.g2_io1() != 0 { try!(write!(f, " g2_io1"))}
        if self.g1_io4() != 0 { try!(write!(f, " g1_io4"))}
        if self.g1_io3() != 0 { try!(write!(f, " g1_io3"))}
        if self.g1_io2() != 0 { try!(write!(f, " g1_io2"))}
        if self.g1_io1() != 0 { try!(write!(f, " g1_io1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O analog switch control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ioascr(pub u32);
impl Ioascr {
    #[doc="G7_IO4"]
    #[inline] pub fn g7_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if G7_IO4 != 0"]
    #[inline] pub fn test_g7_io4(&self) -> bool {
        self.g7_io4() != 0
    }

    #[doc="Sets the G7_IO4 field."]
    #[inline] pub fn set_g7_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="G7_IO3"]
    #[inline] pub fn g7_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if G7_IO3 != 0"]
    #[inline] pub fn test_g7_io3(&self) -> bool {
        self.g7_io3() != 0
    }

    #[doc="Sets the G7_IO3 field."]
    #[inline] pub fn set_g7_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="G7_IO2"]
    #[inline] pub fn g7_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if G7_IO2 != 0"]
    #[inline] pub fn test_g7_io2(&self) -> bool {
        self.g7_io2() != 0
    }

    #[doc="Sets the G7_IO2 field."]
    #[inline] pub fn set_g7_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="G7_IO1"]
    #[inline] pub fn g7_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if G7_IO1 != 0"]
    #[inline] pub fn test_g7_io1(&self) -> bool {
        self.g7_io1() != 0
    }

    #[doc="Sets the G7_IO1 field."]
    #[inline] pub fn set_g7_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="G6_IO4"]
    #[inline] pub fn g6_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if G6_IO4 != 0"]
    #[inline] pub fn test_g6_io4(&self) -> bool {
        self.g6_io4() != 0
    }

    #[doc="Sets the G6_IO4 field."]
    #[inline] pub fn set_g6_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="G6_IO3"]
    #[inline] pub fn g6_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if G6_IO3 != 0"]
    #[inline] pub fn test_g6_io3(&self) -> bool {
        self.g6_io3() != 0
    }

    #[doc="Sets the G6_IO3 field."]
    #[inline] pub fn set_g6_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="G6_IO2"]
    #[inline] pub fn g6_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if G6_IO2 != 0"]
    #[inline] pub fn test_g6_io2(&self) -> bool {
        self.g6_io2() != 0
    }

    #[doc="Sets the G6_IO2 field."]
    #[inline] pub fn set_g6_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="G6_IO1"]
    #[inline] pub fn g6_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if G6_IO1 != 0"]
    #[inline] pub fn test_g6_io1(&self) -> bool {
        self.g6_io1() != 0
    }

    #[doc="Sets the G6_IO1 field."]
    #[inline] pub fn set_g6_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="G5_IO4"]
    #[inline] pub fn g5_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if G5_IO4 != 0"]
    #[inline] pub fn test_g5_io4(&self) -> bool {
        self.g5_io4() != 0
    }

    #[doc="Sets the G5_IO4 field."]
    #[inline] pub fn set_g5_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="G5_IO3"]
    #[inline] pub fn g5_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if G5_IO3 != 0"]
    #[inline] pub fn test_g5_io3(&self) -> bool {
        self.g5_io3() != 0
    }

    #[doc="Sets the G5_IO3 field."]
    #[inline] pub fn set_g5_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="G5_IO2"]
    #[inline] pub fn g5_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if G5_IO2 != 0"]
    #[inline] pub fn test_g5_io2(&self) -> bool {
        self.g5_io2() != 0
    }

    #[doc="Sets the G5_IO2 field."]
    #[inline] pub fn set_g5_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="G5_IO1"]
    #[inline] pub fn g5_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if G5_IO1 != 0"]
    #[inline] pub fn test_g5_io1(&self) -> bool {
        self.g5_io1() != 0
    }

    #[doc="Sets the G5_IO1 field."]
    #[inline] pub fn set_g5_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="G4_IO4"]
    #[inline] pub fn g4_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if G4_IO4 != 0"]
    #[inline] pub fn test_g4_io4(&self) -> bool {
        self.g4_io4() != 0
    }

    #[doc="Sets the G4_IO4 field."]
    #[inline] pub fn set_g4_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="G4_IO3"]
    #[inline] pub fn g4_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if G4_IO3 != 0"]
    #[inline] pub fn test_g4_io3(&self) -> bool {
        self.g4_io3() != 0
    }

    #[doc="Sets the G4_IO3 field."]
    #[inline] pub fn set_g4_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="G4_IO2"]
    #[inline] pub fn g4_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if G4_IO2 != 0"]
    #[inline] pub fn test_g4_io2(&self) -> bool {
        self.g4_io2() != 0
    }

    #[doc="Sets the G4_IO2 field."]
    #[inline] pub fn set_g4_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="G4_IO1"]
    #[inline] pub fn g4_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if G4_IO1 != 0"]
    #[inline] pub fn test_g4_io1(&self) -> bool {
        self.g4_io1() != 0
    }

    #[doc="Sets the G4_IO1 field."]
    #[inline] pub fn set_g4_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="G3_IO4"]
    #[inline] pub fn g3_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if G3_IO4 != 0"]
    #[inline] pub fn test_g3_io4(&self) -> bool {
        self.g3_io4() != 0
    }

    #[doc="Sets the G3_IO4 field."]
    #[inline] pub fn set_g3_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="G3_IO3"]
    #[inline] pub fn g3_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if G3_IO3 != 0"]
    #[inline] pub fn test_g3_io3(&self) -> bool {
        self.g3_io3() != 0
    }

    #[doc="Sets the G3_IO3 field."]
    #[inline] pub fn set_g3_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="G3_IO2"]
    #[inline] pub fn g3_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if G3_IO2 != 0"]
    #[inline] pub fn test_g3_io2(&self) -> bool {
        self.g3_io2() != 0
    }

    #[doc="Sets the G3_IO2 field."]
    #[inline] pub fn set_g3_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="G3_IO1"]
    #[inline] pub fn g3_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if G3_IO1 != 0"]
    #[inline] pub fn test_g3_io1(&self) -> bool {
        self.g3_io1() != 0
    }

    #[doc="Sets the G3_IO1 field."]
    #[inline] pub fn set_g3_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="G2_IO4"]
    #[inline] pub fn g2_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if G2_IO4 != 0"]
    #[inline] pub fn test_g2_io4(&self) -> bool {
        self.g2_io4() != 0
    }

    #[doc="Sets the G2_IO4 field."]
    #[inline] pub fn set_g2_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="G2_IO3"]
    #[inline] pub fn g2_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if G2_IO3 != 0"]
    #[inline] pub fn test_g2_io3(&self) -> bool {
        self.g2_io3() != 0
    }

    #[doc="Sets the G2_IO3 field."]
    #[inline] pub fn set_g2_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="G2_IO2"]
    #[inline] pub fn g2_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if G2_IO2 != 0"]
    #[inline] pub fn test_g2_io2(&self) -> bool {
        self.g2_io2() != 0
    }

    #[doc="Sets the G2_IO2 field."]
    #[inline] pub fn set_g2_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="G2_IO1"]
    #[inline] pub fn g2_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if G2_IO1 != 0"]
    #[inline] pub fn test_g2_io1(&self) -> bool {
        self.g2_io1() != 0
    }

    #[doc="Sets the G2_IO1 field."]
    #[inline] pub fn set_g2_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="G1_IO4"]
    #[inline] pub fn g1_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if G1_IO4 != 0"]
    #[inline] pub fn test_g1_io4(&self) -> bool {
        self.g1_io4() != 0
    }

    #[doc="Sets the G1_IO4 field."]
    #[inline] pub fn set_g1_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="G1_IO3"]
    #[inline] pub fn g1_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if G1_IO3 != 0"]
    #[inline] pub fn test_g1_io3(&self) -> bool {
        self.g1_io3() != 0
    }

    #[doc="Sets the G1_IO3 field."]
    #[inline] pub fn set_g1_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="G1_IO2"]
    #[inline] pub fn g1_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if G1_IO2 != 0"]
    #[inline] pub fn test_g1_io2(&self) -> bool {
        self.g1_io2() != 0
    }

    #[doc="Sets the G1_IO2 field."]
    #[inline] pub fn set_g1_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="G1_IO1"]
    #[inline] pub fn g1_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if G1_IO1 != 0"]
    #[inline] pub fn test_g1_io1(&self) -> bool {
        self.g1_io1() != 0
    }

    #[doc="Sets the G1_IO1 field."]
    #[inline] pub fn set_g1_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ioascr {
    #[inline]
    fn from(other: u32) -> Self {
         Ioascr(other)
    }
}

impl ::core::fmt::Display for Ioascr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ioascr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.g7_io4() != 0 { try!(write!(f, " g7_io4"))}
        if self.g7_io3() != 0 { try!(write!(f, " g7_io3"))}
        if self.g7_io2() != 0 { try!(write!(f, " g7_io2"))}
        if self.g7_io1() != 0 { try!(write!(f, " g7_io1"))}
        if self.g6_io4() != 0 { try!(write!(f, " g6_io4"))}
        if self.g6_io3() != 0 { try!(write!(f, " g6_io3"))}
        if self.g6_io2() != 0 { try!(write!(f, " g6_io2"))}
        if self.g6_io1() != 0 { try!(write!(f, " g6_io1"))}
        if self.g5_io4() != 0 { try!(write!(f, " g5_io4"))}
        if self.g5_io3() != 0 { try!(write!(f, " g5_io3"))}
        if self.g5_io2() != 0 { try!(write!(f, " g5_io2"))}
        if self.g5_io1() != 0 { try!(write!(f, " g5_io1"))}
        if self.g4_io4() != 0 { try!(write!(f, " g4_io4"))}
        if self.g4_io3() != 0 { try!(write!(f, " g4_io3"))}
        if self.g4_io2() != 0 { try!(write!(f, " g4_io2"))}
        if self.g4_io1() != 0 { try!(write!(f, " g4_io1"))}
        if self.g3_io4() != 0 { try!(write!(f, " g3_io4"))}
        if self.g3_io3() != 0 { try!(write!(f, " g3_io3"))}
        if self.g3_io2() != 0 { try!(write!(f, " g3_io2"))}
        if self.g3_io1() != 0 { try!(write!(f, " g3_io1"))}
        if self.g2_io4() != 0 { try!(write!(f, " g2_io4"))}
        if self.g2_io3() != 0 { try!(write!(f, " g2_io3"))}
        if self.g2_io2() != 0 { try!(write!(f, " g2_io2"))}
        if self.g2_io1() != 0 { try!(write!(f, " g2_io1"))}
        if self.g1_io4() != 0 { try!(write!(f, " g1_io4"))}
        if self.g1_io3() != 0 { try!(write!(f, " g1_io3"))}
        if self.g1_io2() != 0 { try!(write!(f, " g1_io2"))}
        if self.g1_io1() != 0 { try!(write!(f, " g1_io1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O sampling control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ioscr(pub u32);
impl Ioscr {
    #[doc="G7_IO4"]
    #[inline] pub fn g7_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if G7_IO4 != 0"]
    #[inline] pub fn test_g7_io4(&self) -> bool {
        self.g7_io4() != 0
    }

    #[doc="Sets the G7_IO4 field."]
    #[inline] pub fn set_g7_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="G7_IO3"]
    #[inline] pub fn g7_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if G7_IO3 != 0"]
    #[inline] pub fn test_g7_io3(&self) -> bool {
        self.g7_io3() != 0
    }

    #[doc="Sets the G7_IO3 field."]
    #[inline] pub fn set_g7_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="G7_IO2"]
    #[inline] pub fn g7_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if G7_IO2 != 0"]
    #[inline] pub fn test_g7_io2(&self) -> bool {
        self.g7_io2() != 0
    }

    #[doc="Sets the G7_IO2 field."]
    #[inline] pub fn set_g7_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="G7_IO1"]
    #[inline] pub fn g7_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if G7_IO1 != 0"]
    #[inline] pub fn test_g7_io1(&self) -> bool {
        self.g7_io1() != 0
    }

    #[doc="Sets the G7_IO1 field."]
    #[inline] pub fn set_g7_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="G6_IO4"]
    #[inline] pub fn g6_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if G6_IO4 != 0"]
    #[inline] pub fn test_g6_io4(&self) -> bool {
        self.g6_io4() != 0
    }

    #[doc="Sets the G6_IO4 field."]
    #[inline] pub fn set_g6_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="G6_IO3"]
    #[inline] pub fn g6_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if G6_IO3 != 0"]
    #[inline] pub fn test_g6_io3(&self) -> bool {
        self.g6_io3() != 0
    }

    #[doc="Sets the G6_IO3 field."]
    #[inline] pub fn set_g6_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="G6_IO2"]
    #[inline] pub fn g6_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if G6_IO2 != 0"]
    #[inline] pub fn test_g6_io2(&self) -> bool {
        self.g6_io2() != 0
    }

    #[doc="Sets the G6_IO2 field."]
    #[inline] pub fn set_g6_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="G6_IO1"]
    #[inline] pub fn g6_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if G6_IO1 != 0"]
    #[inline] pub fn test_g6_io1(&self) -> bool {
        self.g6_io1() != 0
    }

    #[doc="Sets the G6_IO1 field."]
    #[inline] pub fn set_g6_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="G5_IO4"]
    #[inline] pub fn g5_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if G5_IO4 != 0"]
    #[inline] pub fn test_g5_io4(&self) -> bool {
        self.g5_io4() != 0
    }

    #[doc="Sets the G5_IO4 field."]
    #[inline] pub fn set_g5_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="G5_IO3"]
    #[inline] pub fn g5_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if G5_IO3 != 0"]
    #[inline] pub fn test_g5_io3(&self) -> bool {
        self.g5_io3() != 0
    }

    #[doc="Sets the G5_IO3 field."]
    #[inline] pub fn set_g5_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="G5_IO2"]
    #[inline] pub fn g5_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if G5_IO2 != 0"]
    #[inline] pub fn test_g5_io2(&self) -> bool {
        self.g5_io2() != 0
    }

    #[doc="Sets the G5_IO2 field."]
    #[inline] pub fn set_g5_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="G5_IO1"]
    #[inline] pub fn g5_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if G5_IO1 != 0"]
    #[inline] pub fn test_g5_io1(&self) -> bool {
        self.g5_io1() != 0
    }

    #[doc="Sets the G5_IO1 field."]
    #[inline] pub fn set_g5_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="G4_IO4"]
    #[inline] pub fn g4_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if G4_IO4 != 0"]
    #[inline] pub fn test_g4_io4(&self) -> bool {
        self.g4_io4() != 0
    }

    #[doc="Sets the G4_IO4 field."]
    #[inline] pub fn set_g4_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="G4_IO3"]
    #[inline] pub fn g4_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if G4_IO3 != 0"]
    #[inline] pub fn test_g4_io3(&self) -> bool {
        self.g4_io3() != 0
    }

    #[doc="Sets the G4_IO3 field."]
    #[inline] pub fn set_g4_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="G4_IO2"]
    #[inline] pub fn g4_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if G4_IO2 != 0"]
    #[inline] pub fn test_g4_io2(&self) -> bool {
        self.g4_io2() != 0
    }

    #[doc="Sets the G4_IO2 field."]
    #[inline] pub fn set_g4_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="G4_IO1"]
    #[inline] pub fn g4_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if G4_IO1 != 0"]
    #[inline] pub fn test_g4_io1(&self) -> bool {
        self.g4_io1() != 0
    }

    #[doc="Sets the G4_IO1 field."]
    #[inline] pub fn set_g4_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="G3_IO4"]
    #[inline] pub fn g3_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if G3_IO4 != 0"]
    #[inline] pub fn test_g3_io4(&self) -> bool {
        self.g3_io4() != 0
    }

    #[doc="Sets the G3_IO4 field."]
    #[inline] pub fn set_g3_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="G3_IO3"]
    #[inline] pub fn g3_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if G3_IO3 != 0"]
    #[inline] pub fn test_g3_io3(&self) -> bool {
        self.g3_io3() != 0
    }

    #[doc="Sets the G3_IO3 field."]
    #[inline] pub fn set_g3_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="G3_IO2"]
    #[inline] pub fn g3_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if G3_IO2 != 0"]
    #[inline] pub fn test_g3_io2(&self) -> bool {
        self.g3_io2() != 0
    }

    #[doc="Sets the G3_IO2 field."]
    #[inline] pub fn set_g3_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="G3_IO1"]
    #[inline] pub fn g3_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if G3_IO1 != 0"]
    #[inline] pub fn test_g3_io1(&self) -> bool {
        self.g3_io1() != 0
    }

    #[doc="Sets the G3_IO1 field."]
    #[inline] pub fn set_g3_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="G2_IO4"]
    #[inline] pub fn g2_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if G2_IO4 != 0"]
    #[inline] pub fn test_g2_io4(&self) -> bool {
        self.g2_io4() != 0
    }

    #[doc="Sets the G2_IO4 field."]
    #[inline] pub fn set_g2_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="G2_IO3"]
    #[inline] pub fn g2_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if G2_IO3 != 0"]
    #[inline] pub fn test_g2_io3(&self) -> bool {
        self.g2_io3() != 0
    }

    #[doc="Sets the G2_IO3 field."]
    #[inline] pub fn set_g2_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="G2_IO2"]
    #[inline] pub fn g2_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if G2_IO2 != 0"]
    #[inline] pub fn test_g2_io2(&self) -> bool {
        self.g2_io2() != 0
    }

    #[doc="Sets the G2_IO2 field."]
    #[inline] pub fn set_g2_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="G2_IO1"]
    #[inline] pub fn g2_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if G2_IO1 != 0"]
    #[inline] pub fn test_g2_io1(&self) -> bool {
        self.g2_io1() != 0
    }

    #[doc="Sets the G2_IO1 field."]
    #[inline] pub fn set_g2_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="G1_IO4"]
    #[inline] pub fn g1_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if G1_IO4 != 0"]
    #[inline] pub fn test_g1_io4(&self) -> bool {
        self.g1_io4() != 0
    }

    #[doc="Sets the G1_IO4 field."]
    #[inline] pub fn set_g1_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="G1_IO3"]
    #[inline] pub fn g1_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if G1_IO3 != 0"]
    #[inline] pub fn test_g1_io3(&self) -> bool {
        self.g1_io3() != 0
    }

    #[doc="Sets the G1_IO3 field."]
    #[inline] pub fn set_g1_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="G1_IO2"]
    #[inline] pub fn g1_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if G1_IO2 != 0"]
    #[inline] pub fn test_g1_io2(&self) -> bool {
        self.g1_io2() != 0
    }

    #[doc="Sets the G1_IO2 field."]
    #[inline] pub fn set_g1_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="G1_IO1"]
    #[inline] pub fn g1_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if G1_IO1 != 0"]
    #[inline] pub fn test_g1_io1(&self) -> bool {
        self.g1_io1() != 0
    }

    #[doc="Sets the G1_IO1 field."]
    #[inline] pub fn set_g1_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ioscr {
    #[inline]
    fn from(other: u32) -> Self {
         Ioscr(other)
    }
}

impl ::core::fmt::Display for Ioscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ioscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.g7_io4() != 0 { try!(write!(f, " g7_io4"))}
        if self.g7_io3() != 0 { try!(write!(f, " g7_io3"))}
        if self.g7_io2() != 0 { try!(write!(f, " g7_io2"))}
        if self.g7_io1() != 0 { try!(write!(f, " g7_io1"))}
        if self.g6_io4() != 0 { try!(write!(f, " g6_io4"))}
        if self.g6_io3() != 0 { try!(write!(f, " g6_io3"))}
        if self.g6_io2() != 0 { try!(write!(f, " g6_io2"))}
        if self.g6_io1() != 0 { try!(write!(f, " g6_io1"))}
        if self.g5_io4() != 0 { try!(write!(f, " g5_io4"))}
        if self.g5_io3() != 0 { try!(write!(f, " g5_io3"))}
        if self.g5_io2() != 0 { try!(write!(f, " g5_io2"))}
        if self.g5_io1() != 0 { try!(write!(f, " g5_io1"))}
        if self.g4_io4() != 0 { try!(write!(f, " g4_io4"))}
        if self.g4_io3() != 0 { try!(write!(f, " g4_io3"))}
        if self.g4_io2() != 0 { try!(write!(f, " g4_io2"))}
        if self.g4_io1() != 0 { try!(write!(f, " g4_io1"))}
        if self.g3_io4() != 0 { try!(write!(f, " g3_io4"))}
        if self.g3_io3() != 0 { try!(write!(f, " g3_io3"))}
        if self.g3_io2() != 0 { try!(write!(f, " g3_io2"))}
        if self.g3_io1() != 0 { try!(write!(f, " g3_io1"))}
        if self.g2_io4() != 0 { try!(write!(f, " g2_io4"))}
        if self.g2_io3() != 0 { try!(write!(f, " g2_io3"))}
        if self.g2_io2() != 0 { try!(write!(f, " g2_io2"))}
        if self.g2_io1() != 0 { try!(write!(f, " g2_io1"))}
        if self.g1_io4() != 0 { try!(write!(f, " g1_io4"))}
        if self.g1_io3() != 0 { try!(write!(f, " g1_io3"))}
        if self.g1_io2() != 0 { try!(write!(f, " g1_io2"))}
        if self.g1_io1() != 0 { try!(write!(f, " g1_io1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O channel control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ioccr(pub u32);
impl Ioccr {
    #[doc="G7_IO4"]
    #[inline] pub fn g7_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if G7_IO4 != 0"]
    #[inline] pub fn test_g7_io4(&self) -> bool {
        self.g7_io4() != 0
    }

    #[doc="Sets the G7_IO4 field."]
    #[inline] pub fn set_g7_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="G7_IO3"]
    #[inline] pub fn g7_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if G7_IO3 != 0"]
    #[inline] pub fn test_g7_io3(&self) -> bool {
        self.g7_io3() != 0
    }

    #[doc="Sets the G7_IO3 field."]
    #[inline] pub fn set_g7_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="G7_IO2"]
    #[inline] pub fn g7_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if G7_IO2 != 0"]
    #[inline] pub fn test_g7_io2(&self) -> bool {
        self.g7_io2() != 0
    }

    #[doc="Sets the G7_IO2 field."]
    #[inline] pub fn set_g7_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="G7_IO1"]
    #[inline] pub fn g7_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if G7_IO1 != 0"]
    #[inline] pub fn test_g7_io1(&self) -> bool {
        self.g7_io1() != 0
    }

    #[doc="Sets the G7_IO1 field."]
    #[inline] pub fn set_g7_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="G6_IO4"]
    #[inline] pub fn g6_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if G6_IO4 != 0"]
    #[inline] pub fn test_g6_io4(&self) -> bool {
        self.g6_io4() != 0
    }

    #[doc="Sets the G6_IO4 field."]
    #[inline] pub fn set_g6_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="G6_IO3"]
    #[inline] pub fn g6_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if G6_IO3 != 0"]
    #[inline] pub fn test_g6_io3(&self) -> bool {
        self.g6_io3() != 0
    }

    #[doc="Sets the G6_IO3 field."]
    #[inline] pub fn set_g6_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="G6_IO2"]
    #[inline] pub fn g6_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if G6_IO2 != 0"]
    #[inline] pub fn test_g6_io2(&self) -> bool {
        self.g6_io2() != 0
    }

    #[doc="Sets the G6_IO2 field."]
    #[inline] pub fn set_g6_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="G6_IO1"]
    #[inline] pub fn g6_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if G6_IO1 != 0"]
    #[inline] pub fn test_g6_io1(&self) -> bool {
        self.g6_io1() != 0
    }

    #[doc="Sets the G6_IO1 field."]
    #[inline] pub fn set_g6_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="G5_IO4"]
    #[inline] pub fn g5_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if G5_IO4 != 0"]
    #[inline] pub fn test_g5_io4(&self) -> bool {
        self.g5_io4() != 0
    }

    #[doc="Sets the G5_IO4 field."]
    #[inline] pub fn set_g5_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="G5_IO3"]
    #[inline] pub fn g5_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if G5_IO3 != 0"]
    #[inline] pub fn test_g5_io3(&self) -> bool {
        self.g5_io3() != 0
    }

    #[doc="Sets the G5_IO3 field."]
    #[inline] pub fn set_g5_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="G5_IO2"]
    #[inline] pub fn g5_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if G5_IO2 != 0"]
    #[inline] pub fn test_g5_io2(&self) -> bool {
        self.g5_io2() != 0
    }

    #[doc="Sets the G5_IO2 field."]
    #[inline] pub fn set_g5_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="G5_IO1"]
    #[inline] pub fn g5_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if G5_IO1 != 0"]
    #[inline] pub fn test_g5_io1(&self) -> bool {
        self.g5_io1() != 0
    }

    #[doc="Sets the G5_IO1 field."]
    #[inline] pub fn set_g5_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="G4_IO4"]
    #[inline] pub fn g4_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if G4_IO4 != 0"]
    #[inline] pub fn test_g4_io4(&self) -> bool {
        self.g4_io4() != 0
    }

    #[doc="Sets the G4_IO4 field."]
    #[inline] pub fn set_g4_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="G4_IO3"]
    #[inline] pub fn g4_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if G4_IO3 != 0"]
    #[inline] pub fn test_g4_io3(&self) -> bool {
        self.g4_io3() != 0
    }

    #[doc="Sets the G4_IO3 field."]
    #[inline] pub fn set_g4_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="G4_IO2"]
    #[inline] pub fn g4_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if G4_IO2 != 0"]
    #[inline] pub fn test_g4_io2(&self) -> bool {
        self.g4_io2() != 0
    }

    #[doc="Sets the G4_IO2 field."]
    #[inline] pub fn set_g4_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="G4_IO1"]
    #[inline] pub fn g4_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if G4_IO1 != 0"]
    #[inline] pub fn test_g4_io1(&self) -> bool {
        self.g4_io1() != 0
    }

    #[doc="Sets the G4_IO1 field."]
    #[inline] pub fn set_g4_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="G3_IO4"]
    #[inline] pub fn g3_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if G3_IO4 != 0"]
    #[inline] pub fn test_g3_io4(&self) -> bool {
        self.g3_io4() != 0
    }

    #[doc="Sets the G3_IO4 field."]
    #[inline] pub fn set_g3_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="G3_IO3"]
    #[inline] pub fn g3_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if G3_IO3 != 0"]
    #[inline] pub fn test_g3_io3(&self) -> bool {
        self.g3_io3() != 0
    }

    #[doc="Sets the G3_IO3 field."]
    #[inline] pub fn set_g3_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="G3_IO2"]
    #[inline] pub fn g3_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if G3_IO2 != 0"]
    #[inline] pub fn test_g3_io2(&self) -> bool {
        self.g3_io2() != 0
    }

    #[doc="Sets the G3_IO2 field."]
    #[inline] pub fn set_g3_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="G3_IO1"]
    #[inline] pub fn g3_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if G3_IO1 != 0"]
    #[inline] pub fn test_g3_io1(&self) -> bool {
        self.g3_io1() != 0
    }

    #[doc="Sets the G3_IO1 field."]
    #[inline] pub fn set_g3_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="G2_IO4"]
    #[inline] pub fn g2_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if G2_IO4 != 0"]
    #[inline] pub fn test_g2_io4(&self) -> bool {
        self.g2_io4() != 0
    }

    #[doc="Sets the G2_IO4 field."]
    #[inline] pub fn set_g2_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="G2_IO3"]
    #[inline] pub fn g2_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if G2_IO3 != 0"]
    #[inline] pub fn test_g2_io3(&self) -> bool {
        self.g2_io3() != 0
    }

    #[doc="Sets the G2_IO3 field."]
    #[inline] pub fn set_g2_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="G2_IO2"]
    #[inline] pub fn g2_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if G2_IO2 != 0"]
    #[inline] pub fn test_g2_io2(&self) -> bool {
        self.g2_io2() != 0
    }

    #[doc="Sets the G2_IO2 field."]
    #[inline] pub fn set_g2_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="G2_IO1"]
    #[inline] pub fn g2_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if G2_IO1 != 0"]
    #[inline] pub fn test_g2_io1(&self) -> bool {
        self.g2_io1() != 0
    }

    #[doc="Sets the G2_IO1 field."]
    #[inline] pub fn set_g2_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="G1_IO4"]
    #[inline] pub fn g1_io4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if G1_IO4 != 0"]
    #[inline] pub fn test_g1_io4(&self) -> bool {
        self.g1_io4() != 0
    }

    #[doc="Sets the G1_IO4 field."]
    #[inline] pub fn set_g1_io4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="G1_IO3"]
    #[inline] pub fn g1_io3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if G1_IO3 != 0"]
    #[inline] pub fn test_g1_io3(&self) -> bool {
        self.g1_io3() != 0
    }

    #[doc="Sets the G1_IO3 field."]
    #[inline] pub fn set_g1_io3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="G1_IO2"]
    #[inline] pub fn g1_io2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if G1_IO2 != 0"]
    #[inline] pub fn test_g1_io2(&self) -> bool {
        self.g1_io2() != 0
    }

    #[doc="Sets the G1_IO2 field."]
    #[inline] pub fn set_g1_io2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="G1_IO1"]
    #[inline] pub fn g1_io1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if G1_IO1 != 0"]
    #[inline] pub fn test_g1_io1(&self) -> bool {
        self.g1_io1() != 0
    }

    #[doc="Sets the G1_IO1 field."]
    #[inline] pub fn set_g1_io1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ioccr {
    #[inline]
    fn from(other: u32) -> Self {
         Ioccr(other)
    }
}

impl ::core::fmt::Display for Ioccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ioccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.g7_io4() != 0 { try!(write!(f, " g7_io4"))}
        if self.g7_io3() != 0 { try!(write!(f, " g7_io3"))}
        if self.g7_io2() != 0 { try!(write!(f, " g7_io2"))}
        if self.g7_io1() != 0 { try!(write!(f, " g7_io1"))}
        if self.g6_io4() != 0 { try!(write!(f, " g6_io4"))}
        if self.g6_io3() != 0 { try!(write!(f, " g6_io3"))}
        if self.g6_io2() != 0 { try!(write!(f, " g6_io2"))}
        if self.g6_io1() != 0 { try!(write!(f, " g6_io1"))}
        if self.g5_io4() != 0 { try!(write!(f, " g5_io4"))}
        if self.g5_io3() != 0 { try!(write!(f, " g5_io3"))}
        if self.g5_io2() != 0 { try!(write!(f, " g5_io2"))}
        if self.g5_io1() != 0 { try!(write!(f, " g5_io1"))}
        if self.g4_io4() != 0 { try!(write!(f, " g4_io4"))}
        if self.g4_io3() != 0 { try!(write!(f, " g4_io3"))}
        if self.g4_io2() != 0 { try!(write!(f, " g4_io2"))}
        if self.g4_io1() != 0 { try!(write!(f, " g4_io1"))}
        if self.g3_io4() != 0 { try!(write!(f, " g3_io4"))}
        if self.g3_io3() != 0 { try!(write!(f, " g3_io3"))}
        if self.g3_io2() != 0 { try!(write!(f, " g3_io2"))}
        if self.g3_io1() != 0 { try!(write!(f, " g3_io1"))}
        if self.g2_io4() != 0 { try!(write!(f, " g2_io4"))}
        if self.g2_io3() != 0 { try!(write!(f, " g2_io3"))}
        if self.g2_io2() != 0 { try!(write!(f, " g2_io2"))}
        if self.g2_io1() != 0 { try!(write!(f, " g2_io1"))}
        if self.g1_io4() != 0 { try!(write!(f, " g1_io4"))}
        if self.g1_io3() != 0 { try!(write!(f, " g1_io3"))}
        if self.g1_io2() != 0 { try!(write!(f, " g1_io2"))}
        if self.g1_io1() != 0 { try!(write!(f, " g1_io1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O group control status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iogcsr(pub u32);
impl Iogcsr {
    #[doc="Analog I/O group x status"]
    #[inline] pub fn g7s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if G7S != 0"]
    #[inline] pub fn test_g7s(&self) -> bool {
        self.g7s() != 0
    }

    #[doc="Sets the G7S field."]
    #[inline] pub fn set_g7s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Analog I/O group x status"]
    #[inline] pub fn g6s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if G6S != 0"]
    #[inline] pub fn test_g6s(&self) -> bool {
        self.g6s() != 0
    }

    #[doc="Sets the G6S field."]
    #[inline] pub fn set_g6s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Analog I/O group x status"]
    #[inline] pub fn g5s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if G5S != 0"]
    #[inline] pub fn test_g5s(&self) -> bool {
        self.g5s() != 0
    }

    #[doc="Sets the G5S field."]
    #[inline] pub fn set_g5s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Analog I/O group x status"]
    #[inline] pub fn g4s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if G4S != 0"]
    #[inline] pub fn test_g4s(&self) -> bool {
        self.g4s() != 0
    }

    #[doc="Sets the G4S field."]
    #[inline] pub fn set_g4s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Analog I/O group x status"]
    #[inline] pub fn g3s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if G3S != 0"]
    #[inline] pub fn test_g3s(&self) -> bool {
        self.g3s() != 0
    }

    #[doc="Sets the G3S field."]
    #[inline] pub fn set_g3s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Analog I/O group x status"]
    #[inline] pub fn g2s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if G2S != 0"]
    #[inline] pub fn test_g2s(&self) -> bool {
        self.g2s() != 0
    }

    #[doc="Sets the G2S field."]
    #[inline] pub fn set_g2s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Analog I/O group x status"]
    #[inline] pub fn g1s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if G1S != 0"]
    #[inline] pub fn test_g1s(&self) -> bool {
        self.g1s() != 0
    }

    #[doc="Sets the G1S field."]
    #[inline] pub fn set_g1s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Analog I/O group x enable"]
    #[inline] pub fn g7e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if G7E != 0"]
    #[inline] pub fn test_g7e(&self) -> bool {
        self.g7e() != 0
    }

    #[doc="Sets the G7E field."]
    #[inline] pub fn set_g7e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Analog I/O group x enable"]
    #[inline] pub fn g6e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if G6E != 0"]
    #[inline] pub fn test_g6e(&self) -> bool {
        self.g6e() != 0
    }

    #[doc="Sets the G6E field."]
    #[inline] pub fn set_g6e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Analog I/O group x enable"]
    #[inline] pub fn g5e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if G5E != 0"]
    #[inline] pub fn test_g5e(&self) -> bool {
        self.g5e() != 0
    }

    #[doc="Sets the G5E field."]
    #[inline] pub fn set_g5e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Analog I/O group x enable"]
    #[inline] pub fn g4e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if G4E != 0"]
    #[inline] pub fn test_g4e(&self) -> bool {
        self.g4e() != 0
    }

    #[doc="Sets the G4E field."]
    #[inline] pub fn set_g4e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Analog I/O group x enable"]
    #[inline] pub fn g3e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if G3E != 0"]
    #[inline] pub fn test_g3e(&self) -> bool {
        self.g3e() != 0
    }

    #[doc="Sets the G3E field."]
    #[inline] pub fn set_g3e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Analog I/O group x enable"]
    #[inline] pub fn g2e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if G2E != 0"]
    #[inline] pub fn test_g2e(&self) -> bool {
        self.g2e() != 0
    }

    #[doc="Sets the G2E field."]
    #[inline] pub fn set_g2e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Analog I/O group x enable"]
    #[inline] pub fn g1e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if G1E != 0"]
    #[inline] pub fn test_g1e(&self) -> bool {
        self.g1e() != 0
    }

    #[doc="Sets the G1E field."]
    #[inline] pub fn set_g1e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iogcsr {
    #[inline]
    fn from(other: u32) -> Self {
         Iogcsr(other)
    }
}

impl ::core::fmt::Display for Iogcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iogcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.g7s() != 0 { try!(write!(f, " g7s"))}
        if self.g6s() != 0 { try!(write!(f, " g6s"))}
        if self.g5s() != 0 { try!(write!(f, " g5s"))}
        if self.g4s() != 0 { try!(write!(f, " g4s"))}
        if self.g3s() != 0 { try!(write!(f, " g3s"))}
        if self.g2s() != 0 { try!(write!(f, " g2s"))}
        if self.g1s() != 0 { try!(write!(f, " g1s"))}
        if self.g7e() != 0 { try!(write!(f, " g7e"))}
        if self.g6e() != 0 { try!(write!(f, " g6e"))}
        if self.g5e() != 0 { try!(write!(f, " g5e"))}
        if self.g4e() != 0 { try!(write!(f, " g4e"))}
        if self.g3e() != 0 { try!(write!(f, " g3e"))}
        if self.g2e() != 0 { try!(write!(f, " g2e"))}
        if self.g1e() != 0 { try!(write!(f, " g1e"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O group x counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iog1cr(pub u32);
impl Iog1cr {
    #[doc="Counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iog1cr {
    #[inline]
    fn from(other: u32) -> Self {
         Iog1cr(other)
    }
}

impl ::core::fmt::Display for Iog1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iog1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O group x counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iog2cr(pub u32);
impl Iog2cr {
    #[doc="Counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iog2cr {
    #[inline]
    fn from(other: u32) -> Self {
         Iog2cr(other)
    }
}

impl ::core::fmt::Display for Iog2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iog2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O group x counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iog3cr(pub u32);
impl Iog3cr {
    #[doc="Counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iog3cr {
    #[inline]
    fn from(other: u32) -> Self {
         Iog3cr(other)
    }
}

impl ::core::fmt::Display for Iog3cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iog3cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O group x counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iog4cr(pub u32);
impl Iog4cr {
    #[doc="Counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iog4cr {
    #[inline]
    fn from(other: u32) -> Self {
         Iog4cr(other)
    }
}

impl ::core::fmt::Display for Iog4cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iog4cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O group x counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iog5cr(pub u32);
impl Iog5cr {
    #[doc="Counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iog5cr {
    #[inline]
    fn from(other: u32) -> Self {
         Iog5cr(other)
    }
}

impl ::core::fmt::Display for Iog5cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iog5cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O group x counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iog6cr(pub u32);
impl Iog6cr {
    #[doc="Counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iog6cr {
    #[inline]
    fn from(other: u32) -> Self {
         Iog6cr(other)
    }
}

impl ::core::fmt::Display for Iog6cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iog6cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O group x counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iog7cr(pub u32);
impl Iog7cr {
    #[doc="Counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iog7cr {
    #[inline]
    fn from(other: u32) -> Self {
         Iog7cr(other)
    }
}

impl ::core::fmt::Display for Iog7cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iog7cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

