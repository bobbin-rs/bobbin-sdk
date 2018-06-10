
::bobbin_mcu::periph!( , Dma2d, _PERIPH, Dma2dPeriph, _OWNED, _REF_COUNT, 0x00000000, 0x00, 0x0c);


#[doc="DMA2D controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dma2dPeriph(pub usize);
impl Dma2dPeriph {
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

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x4)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ifcr, 0x8)
    }

    #[doc="Get the *mut pointer for the IFCR register."]
    #[inline] pub fn ifcr_mut(&self) -> *mut Ifcr { 
        self.ifcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IFCR register."]
    #[inline] pub fn ifcr_ptr(&self) -> *const Ifcr { 
        self.ifcr_reg().ptr()
    }

    #[doc="Read the IFCR register."]
    #[inline] pub fn ifcr(&self) -> Ifcr { 
        self.ifcr_reg().read()
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

    #[doc="Modify the IFCR register."]
    #[inline] pub fn with_ifcr<F: FnOnce(Ifcr) -> Ifcr>(&self, f: F) -> &Self {
        self.ifcr_reg().with(f);
        self
    }

    #[doc="Get the FGMAR Register."]
    #[inline] pub fn fgmar_reg(&self) -> ::bobbin_mcu::register::Register<Fgmar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fgmar, 0xc)
    }

    #[doc="Get the *mut pointer for the FGMAR register."]
    #[inline] pub fn fgmar_mut(&self) -> *mut Fgmar { 
        self.fgmar_reg().ptr()
    }

    #[doc="Get the *const pointer for the FGMAR register."]
    #[inline] pub fn fgmar_ptr(&self) -> *const Fgmar { 
        self.fgmar_reg().ptr()
    }

    #[doc="Read the FGMAR register."]
    #[inline] pub fn fgmar(&self) -> Fgmar { 
        self.fgmar_reg().read()
    }

    #[doc="Write the FGMAR register."]
    #[inline] pub fn write_fgmar(&self, value: Fgmar) -> &Self { 
        self.fgmar_reg().write(value);
        self
    }

    #[doc="Set the FGMAR register."]
    #[inline] pub fn set_fgmar<F: FnOnce(Fgmar) -> Fgmar>(&self, f: F) -> &Self {
        self.fgmar_reg().set(f);
        self
    }

    #[doc="Modify the FGMAR register."]
    #[inline] pub fn with_fgmar<F: FnOnce(Fgmar) -> Fgmar>(&self, f: F) -> &Self {
        self.fgmar_reg().with(f);
        self
    }

    #[doc="Get the FGOR Register."]
    #[inline] pub fn fgor_reg(&self) -> ::bobbin_mcu::register::Register<Fgor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fgor, 0x10)
    }

    #[doc="Get the *mut pointer for the FGOR register."]
    #[inline] pub fn fgor_mut(&self) -> *mut Fgor { 
        self.fgor_reg().ptr()
    }

    #[doc="Get the *const pointer for the FGOR register."]
    #[inline] pub fn fgor_ptr(&self) -> *const Fgor { 
        self.fgor_reg().ptr()
    }

    #[doc="Read the FGOR register."]
    #[inline] pub fn fgor(&self) -> Fgor { 
        self.fgor_reg().read()
    }

    #[doc="Write the FGOR register."]
    #[inline] pub fn write_fgor(&self, value: Fgor) -> &Self { 
        self.fgor_reg().write(value);
        self
    }

    #[doc="Set the FGOR register."]
    #[inline] pub fn set_fgor<F: FnOnce(Fgor) -> Fgor>(&self, f: F) -> &Self {
        self.fgor_reg().set(f);
        self
    }

    #[doc="Modify the FGOR register."]
    #[inline] pub fn with_fgor<F: FnOnce(Fgor) -> Fgor>(&self, f: F) -> &Self {
        self.fgor_reg().with(f);
        self
    }

    #[doc="Get the BGMAR Register."]
    #[inline] pub fn bgmar_reg(&self) -> ::bobbin_mcu::register::Register<Bgmar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bgmar, 0x14)
    }

    #[doc="Get the *mut pointer for the BGMAR register."]
    #[inline] pub fn bgmar_mut(&self) -> *mut Bgmar { 
        self.bgmar_reg().ptr()
    }

    #[doc="Get the *const pointer for the BGMAR register."]
    #[inline] pub fn bgmar_ptr(&self) -> *const Bgmar { 
        self.bgmar_reg().ptr()
    }

    #[doc="Read the BGMAR register."]
    #[inline] pub fn bgmar(&self) -> Bgmar { 
        self.bgmar_reg().read()
    }

    #[doc="Write the BGMAR register."]
    #[inline] pub fn write_bgmar(&self, value: Bgmar) -> &Self { 
        self.bgmar_reg().write(value);
        self
    }

    #[doc="Set the BGMAR register."]
    #[inline] pub fn set_bgmar<F: FnOnce(Bgmar) -> Bgmar>(&self, f: F) -> &Self {
        self.bgmar_reg().set(f);
        self
    }

    #[doc="Modify the BGMAR register."]
    #[inline] pub fn with_bgmar<F: FnOnce(Bgmar) -> Bgmar>(&self, f: F) -> &Self {
        self.bgmar_reg().with(f);
        self
    }

    #[doc="Get the BGOR Register."]
    #[inline] pub fn bgor_reg(&self) -> ::bobbin_mcu::register::Register<Bgor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bgor, 0x18)
    }

    #[doc="Get the *mut pointer for the BGOR register."]
    #[inline] pub fn bgor_mut(&self) -> *mut Bgor { 
        self.bgor_reg().ptr()
    }

    #[doc="Get the *const pointer for the BGOR register."]
    #[inline] pub fn bgor_ptr(&self) -> *const Bgor { 
        self.bgor_reg().ptr()
    }

    #[doc="Read the BGOR register."]
    #[inline] pub fn bgor(&self) -> Bgor { 
        self.bgor_reg().read()
    }

    #[doc="Write the BGOR register."]
    #[inline] pub fn write_bgor(&self, value: Bgor) -> &Self { 
        self.bgor_reg().write(value);
        self
    }

    #[doc="Set the BGOR register."]
    #[inline] pub fn set_bgor<F: FnOnce(Bgor) -> Bgor>(&self, f: F) -> &Self {
        self.bgor_reg().set(f);
        self
    }

    #[doc="Modify the BGOR register."]
    #[inline] pub fn with_bgor<F: FnOnce(Bgor) -> Bgor>(&self, f: F) -> &Self {
        self.bgor_reg().with(f);
        self
    }

    #[doc="Get the FGPFCCR Register."]
    #[inline] pub fn fgpfccr_reg(&self) -> ::bobbin_mcu::register::Register<Fgpfccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fgpfccr, 0x1c)
    }

    #[doc="Get the *mut pointer for the FGPFCCR register."]
    #[inline] pub fn fgpfccr_mut(&self) -> *mut Fgpfccr { 
        self.fgpfccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FGPFCCR register."]
    #[inline] pub fn fgpfccr_ptr(&self) -> *const Fgpfccr { 
        self.fgpfccr_reg().ptr()
    }

    #[doc="Read the FGPFCCR register."]
    #[inline] pub fn fgpfccr(&self) -> Fgpfccr { 
        self.fgpfccr_reg().read()
    }

    #[doc="Write the FGPFCCR register."]
    #[inline] pub fn write_fgpfccr(&self, value: Fgpfccr) -> &Self { 
        self.fgpfccr_reg().write(value);
        self
    }

    #[doc="Set the FGPFCCR register."]
    #[inline] pub fn set_fgpfccr<F: FnOnce(Fgpfccr) -> Fgpfccr>(&self, f: F) -> &Self {
        self.fgpfccr_reg().set(f);
        self
    }

    #[doc="Modify the FGPFCCR register."]
    #[inline] pub fn with_fgpfccr<F: FnOnce(Fgpfccr) -> Fgpfccr>(&self, f: F) -> &Self {
        self.fgpfccr_reg().with(f);
        self
    }

    #[doc="Get the FGCOLR Register."]
    #[inline] pub fn fgcolr_reg(&self) -> ::bobbin_mcu::register::Register<Fgcolr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fgcolr, 0x20)
    }

    #[doc="Get the *mut pointer for the FGCOLR register."]
    #[inline] pub fn fgcolr_mut(&self) -> *mut Fgcolr { 
        self.fgcolr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FGCOLR register."]
    #[inline] pub fn fgcolr_ptr(&self) -> *const Fgcolr { 
        self.fgcolr_reg().ptr()
    }

    #[doc="Read the FGCOLR register."]
    #[inline] pub fn fgcolr(&self) -> Fgcolr { 
        self.fgcolr_reg().read()
    }

    #[doc="Write the FGCOLR register."]
    #[inline] pub fn write_fgcolr(&self, value: Fgcolr) -> &Self { 
        self.fgcolr_reg().write(value);
        self
    }

    #[doc="Set the FGCOLR register."]
    #[inline] pub fn set_fgcolr<F: FnOnce(Fgcolr) -> Fgcolr>(&self, f: F) -> &Self {
        self.fgcolr_reg().set(f);
        self
    }

    #[doc="Modify the FGCOLR register."]
    #[inline] pub fn with_fgcolr<F: FnOnce(Fgcolr) -> Fgcolr>(&self, f: F) -> &Self {
        self.fgcolr_reg().with(f);
        self
    }

    #[doc="Get the BGPFCCR Register."]
    #[inline] pub fn bgpfccr_reg(&self) -> ::bobbin_mcu::register::Register<Bgpfccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bgpfccr, 0x24)
    }

    #[doc="Get the *mut pointer for the BGPFCCR register."]
    #[inline] pub fn bgpfccr_mut(&self) -> *mut Bgpfccr { 
        self.bgpfccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BGPFCCR register."]
    #[inline] pub fn bgpfccr_ptr(&self) -> *const Bgpfccr { 
        self.bgpfccr_reg().ptr()
    }

    #[doc="Read the BGPFCCR register."]
    #[inline] pub fn bgpfccr(&self) -> Bgpfccr { 
        self.bgpfccr_reg().read()
    }

    #[doc="Write the BGPFCCR register."]
    #[inline] pub fn write_bgpfccr(&self, value: Bgpfccr) -> &Self { 
        self.bgpfccr_reg().write(value);
        self
    }

    #[doc="Set the BGPFCCR register."]
    #[inline] pub fn set_bgpfccr<F: FnOnce(Bgpfccr) -> Bgpfccr>(&self, f: F) -> &Self {
        self.bgpfccr_reg().set(f);
        self
    }

    #[doc="Modify the BGPFCCR register."]
    #[inline] pub fn with_bgpfccr<F: FnOnce(Bgpfccr) -> Bgpfccr>(&self, f: F) -> &Self {
        self.bgpfccr_reg().with(f);
        self
    }

    #[doc="Get the BGCOLR Register."]
    #[inline] pub fn bgcolr_reg(&self) -> ::bobbin_mcu::register::Register<Bgcolr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bgcolr, 0x28)
    }

    #[doc="Get the *mut pointer for the BGCOLR register."]
    #[inline] pub fn bgcolr_mut(&self) -> *mut Bgcolr { 
        self.bgcolr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BGCOLR register."]
    #[inline] pub fn bgcolr_ptr(&self) -> *const Bgcolr { 
        self.bgcolr_reg().ptr()
    }

    #[doc="Read the BGCOLR register."]
    #[inline] pub fn bgcolr(&self) -> Bgcolr { 
        self.bgcolr_reg().read()
    }

    #[doc="Write the BGCOLR register."]
    #[inline] pub fn write_bgcolr(&self, value: Bgcolr) -> &Self { 
        self.bgcolr_reg().write(value);
        self
    }

    #[doc="Set the BGCOLR register."]
    #[inline] pub fn set_bgcolr<F: FnOnce(Bgcolr) -> Bgcolr>(&self, f: F) -> &Self {
        self.bgcolr_reg().set(f);
        self
    }

    #[doc="Modify the BGCOLR register."]
    #[inline] pub fn with_bgcolr<F: FnOnce(Bgcolr) -> Bgcolr>(&self, f: F) -> &Self {
        self.bgcolr_reg().with(f);
        self
    }

    #[doc="Get the FGCMAR Register."]
    #[inline] pub fn fgcmar_reg(&self) -> ::bobbin_mcu::register::Register<Fgcmar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fgcmar, 0x2c)
    }

    #[doc="Get the *mut pointer for the FGCMAR register."]
    #[inline] pub fn fgcmar_mut(&self) -> *mut Fgcmar { 
        self.fgcmar_reg().ptr()
    }

    #[doc="Get the *const pointer for the FGCMAR register."]
    #[inline] pub fn fgcmar_ptr(&self) -> *const Fgcmar { 
        self.fgcmar_reg().ptr()
    }

    #[doc="Read the FGCMAR register."]
    #[inline] pub fn fgcmar(&self) -> Fgcmar { 
        self.fgcmar_reg().read()
    }

    #[doc="Write the FGCMAR register."]
    #[inline] pub fn write_fgcmar(&self, value: Fgcmar) -> &Self { 
        self.fgcmar_reg().write(value);
        self
    }

    #[doc="Set the FGCMAR register."]
    #[inline] pub fn set_fgcmar<F: FnOnce(Fgcmar) -> Fgcmar>(&self, f: F) -> &Self {
        self.fgcmar_reg().set(f);
        self
    }

    #[doc="Modify the FGCMAR register."]
    #[inline] pub fn with_fgcmar<F: FnOnce(Fgcmar) -> Fgcmar>(&self, f: F) -> &Self {
        self.fgcmar_reg().with(f);
        self
    }

    #[doc="Get the BGCMAR Register."]
    #[inline] pub fn bgcmar_reg(&self) -> ::bobbin_mcu::register::Register<Bgcmar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bgcmar, 0x30)
    }

    #[doc="Get the *mut pointer for the BGCMAR register."]
    #[inline] pub fn bgcmar_mut(&self) -> *mut Bgcmar { 
        self.bgcmar_reg().ptr()
    }

    #[doc="Get the *const pointer for the BGCMAR register."]
    #[inline] pub fn bgcmar_ptr(&self) -> *const Bgcmar { 
        self.bgcmar_reg().ptr()
    }

    #[doc="Read the BGCMAR register."]
    #[inline] pub fn bgcmar(&self) -> Bgcmar { 
        self.bgcmar_reg().read()
    }

    #[doc="Write the BGCMAR register."]
    #[inline] pub fn write_bgcmar(&self, value: Bgcmar) -> &Self { 
        self.bgcmar_reg().write(value);
        self
    }

    #[doc="Set the BGCMAR register."]
    #[inline] pub fn set_bgcmar<F: FnOnce(Bgcmar) -> Bgcmar>(&self, f: F) -> &Self {
        self.bgcmar_reg().set(f);
        self
    }

    #[doc="Modify the BGCMAR register."]
    #[inline] pub fn with_bgcmar<F: FnOnce(Bgcmar) -> Bgcmar>(&self, f: F) -> &Self {
        self.bgcmar_reg().with(f);
        self
    }

    #[doc="Get the OPFCCR Register."]
    #[inline] pub fn opfccr_reg(&self) -> ::bobbin_mcu::register::Register<Opfccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Opfccr, 0x34)
    }

    #[doc="Get the *mut pointer for the OPFCCR register."]
    #[inline] pub fn opfccr_mut(&self) -> *mut Opfccr { 
        self.opfccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OPFCCR register."]
    #[inline] pub fn opfccr_ptr(&self) -> *const Opfccr { 
        self.opfccr_reg().ptr()
    }

    #[doc="Read the OPFCCR register."]
    #[inline] pub fn opfccr(&self) -> Opfccr { 
        self.opfccr_reg().read()
    }

    #[doc="Write the OPFCCR register."]
    #[inline] pub fn write_opfccr(&self, value: Opfccr) -> &Self { 
        self.opfccr_reg().write(value);
        self
    }

    #[doc="Set the OPFCCR register."]
    #[inline] pub fn set_opfccr<F: FnOnce(Opfccr) -> Opfccr>(&self, f: F) -> &Self {
        self.opfccr_reg().set(f);
        self
    }

    #[doc="Modify the OPFCCR register."]
    #[inline] pub fn with_opfccr<F: FnOnce(Opfccr) -> Opfccr>(&self, f: F) -> &Self {
        self.opfccr_reg().with(f);
        self
    }

    #[doc="Get the OCOLR Register."]
    #[inline] pub fn ocolr_reg(&self) -> ::bobbin_mcu::register::Register<Ocolr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ocolr, 0x38)
    }

    #[doc="Get the *mut pointer for the OCOLR register."]
    #[inline] pub fn ocolr_mut(&self) -> *mut Ocolr { 
        self.ocolr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OCOLR register."]
    #[inline] pub fn ocolr_ptr(&self) -> *const Ocolr { 
        self.ocolr_reg().ptr()
    }

    #[doc="Read the OCOLR register."]
    #[inline] pub fn ocolr(&self) -> Ocolr { 
        self.ocolr_reg().read()
    }

    #[doc="Write the OCOLR register."]
    #[inline] pub fn write_ocolr(&self, value: Ocolr) -> &Self { 
        self.ocolr_reg().write(value);
        self
    }

    #[doc="Set the OCOLR register."]
    #[inline] pub fn set_ocolr<F: FnOnce(Ocolr) -> Ocolr>(&self, f: F) -> &Self {
        self.ocolr_reg().set(f);
        self
    }

    #[doc="Modify the OCOLR register."]
    #[inline] pub fn with_ocolr<F: FnOnce(Ocolr) -> Ocolr>(&self, f: F) -> &Self {
        self.ocolr_reg().with(f);
        self
    }

    #[doc="Get the OMAR Register."]
    #[inline] pub fn omar_reg(&self) -> ::bobbin_mcu::register::Register<Omar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Omar, 0x3c)
    }

    #[doc="Get the *mut pointer for the OMAR register."]
    #[inline] pub fn omar_mut(&self) -> *mut Omar { 
        self.omar_reg().ptr()
    }

    #[doc="Get the *const pointer for the OMAR register."]
    #[inline] pub fn omar_ptr(&self) -> *const Omar { 
        self.omar_reg().ptr()
    }

    #[doc="Read the OMAR register."]
    #[inline] pub fn omar(&self) -> Omar { 
        self.omar_reg().read()
    }

    #[doc="Write the OMAR register."]
    #[inline] pub fn write_omar(&self, value: Omar) -> &Self { 
        self.omar_reg().write(value);
        self
    }

    #[doc="Set the OMAR register."]
    #[inline] pub fn set_omar<F: FnOnce(Omar) -> Omar>(&self, f: F) -> &Self {
        self.omar_reg().set(f);
        self
    }

    #[doc="Modify the OMAR register."]
    #[inline] pub fn with_omar<F: FnOnce(Omar) -> Omar>(&self, f: F) -> &Self {
        self.omar_reg().with(f);
        self
    }

    #[doc="Get the OOR Register."]
    #[inline] pub fn oor_reg(&self) -> ::bobbin_mcu::register::Register<Oor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Oor, 0x40)
    }

    #[doc="Get the *mut pointer for the OOR register."]
    #[inline] pub fn oor_mut(&self) -> *mut Oor { 
        self.oor_reg().ptr()
    }

    #[doc="Get the *const pointer for the OOR register."]
    #[inline] pub fn oor_ptr(&self) -> *const Oor { 
        self.oor_reg().ptr()
    }

    #[doc="Read the OOR register."]
    #[inline] pub fn oor(&self) -> Oor { 
        self.oor_reg().read()
    }

    #[doc="Write the OOR register."]
    #[inline] pub fn write_oor(&self, value: Oor) -> &Self { 
        self.oor_reg().write(value);
        self
    }

    #[doc="Set the OOR register."]
    #[inline] pub fn set_oor<F: FnOnce(Oor) -> Oor>(&self, f: F) -> &Self {
        self.oor_reg().set(f);
        self
    }

    #[doc="Modify the OOR register."]
    #[inline] pub fn with_oor<F: FnOnce(Oor) -> Oor>(&self, f: F) -> &Self {
        self.oor_reg().with(f);
        self
    }

    #[doc="Get the NLR Register."]
    #[inline] pub fn nlr_reg(&self) -> ::bobbin_mcu::register::Register<Nlr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Nlr, 0x44)
    }

    #[doc="Get the *mut pointer for the NLR register."]
    #[inline] pub fn nlr_mut(&self) -> *mut Nlr { 
        self.nlr_reg().ptr()
    }

    #[doc="Get the *const pointer for the NLR register."]
    #[inline] pub fn nlr_ptr(&self) -> *const Nlr { 
        self.nlr_reg().ptr()
    }

    #[doc="Read the NLR register."]
    #[inline] pub fn nlr(&self) -> Nlr { 
        self.nlr_reg().read()
    }

    #[doc="Write the NLR register."]
    #[inline] pub fn write_nlr(&self, value: Nlr) -> &Self { 
        self.nlr_reg().write(value);
        self
    }

    #[doc="Set the NLR register."]
    #[inline] pub fn set_nlr<F: FnOnce(Nlr) -> Nlr>(&self, f: F) -> &Self {
        self.nlr_reg().set(f);
        self
    }

    #[doc="Modify the NLR register."]
    #[inline] pub fn with_nlr<F: FnOnce(Nlr) -> Nlr>(&self, f: F) -> &Self {
        self.nlr_reg().with(f);
        self
    }

    #[doc="Get the LWR Register."]
    #[inline] pub fn lwr_reg(&self) -> ::bobbin_mcu::register::Register<Lwr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Lwr, 0x48)
    }

    #[doc="Get the *mut pointer for the LWR register."]
    #[inline] pub fn lwr_mut(&self) -> *mut Lwr { 
        self.lwr_reg().ptr()
    }

    #[doc="Get the *const pointer for the LWR register."]
    #[inline] pub fn lwr_ptr(&self) -> *const Lwr { 
        self.lwr_reg().ptr()
    }

    #[doc="Read the LWR register."]
    #[inline] pub fn lwr(&self) -> Lwr { 
        self.lwr_reg().read()
    }

    #[doc="Write the LWR register."]
    #[inline] pub fn write_lwr(&self, value: Lwr) -> &Self { 
        self.lwr_reg().write(value);
        self
    }

    #[doc="Set the LWR register."]
    #[inline] pub fn set_lwr<F: FnOnce(Lwr) -> Lwr>(&self, f: F) -> &Self {
        self.lwr_reg().set(f);
        self
    }

    #[doc="Modify the LWR register."]
    #[inline] pub fn with_lwr<F: FnOnce(Lwr) -> Lwr>(&self, f: F) -> &Self {
        self.lwr_reg().with(f);
        self
    }

    #[doc="Get the AMTCR Register."]
    #[inline] pub fn amtcr_reg(&self) -> ::bobbin_mcu::register::Register<Amtcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Amtcr, 0x4c)
    }

    #[doc="Get the *mut pointer for the AMTCR register."]
    #[inline] pub fn amtcr_mut(&self) -> *mut Amtcr { 
        self.amtcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AMTCR register."]
    #[inline] pub fn amtcr_ptr(&self) -> *const Amtcr { 
        self.amtcr_reg().ptr()
    }

    #[doc="Read the AMTCR register."]
    #[inline] pub fn amtcr(&self) -> Amtcr { 
        self.amtcr_reg().read()
    }

    #[doc="Write the AMTCR register."]
    #[inline] pub fn write_amtcr(&self, value: Amtcr) -> &Self { 
        self.amtcr_reg().write(value);
        self
    }

    #[doc="Set the AMTCR register."]
    #[inline] pub fn set_amtcr<F: FnOnce(Amtcr) -> Amtcr>(&self, f: F) -> &Self {
        self.amtcr_reg().set(f);
        self
    }

    #[doc="Modify the AMTCR register."]
    #[inline] pub fn with_amtcr<F: FnOnce(Amtcr) -> Amtcr>(&self, f: F) -> &Self {
        self.amtcr_reg().with(f);
        self
    }

    #[doc="Get the FGCLUT Register."]
    #[inline] pub fn fgclut_reg(&self) -> ::bobbin_mcu::register::Register<Fgclut> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fgclut, 0x400)
    }

    #[doc="Get the *mut pointer for the FGCLUT register."]
    #[inline] pub fn fgclut_mut(&self) -> *mut Fgclut { 
        self.fgclut_reg().ptr()
    }

    #[doc="Get the *const pointer for the FGCLUT register."]
    #[inline] pub fn fgclut_ptr(&self) -> *const Fgclut { 
        self.fgclut_reg().ptr()
    }

    #[doc="Read the FGCLUT register."]
    #[inline] pub fn fgclut(&self) -> Fgclut { 
        self.fgclut_reg().read()
    }

    #[doc="Write the FGCLUT register."]
    #[inline] pub fn write_fgclut(&self, value: Fgclut) -> &Self { 
        self.fgclut_reg().write(value);
        self
    }

    #[doc="Set the FGCLUT register."]
    #[inline] pub fn set_fgclut<F: FnOnce(Fgclut) -> Fgclut>(&self, f: F) -> &Self {
        self.fgclut_reg().set(f);
        self
    }

    #[doc="Modify the FGCLUT register."]
    #[inline] pub fn with_fgclut<F: FnOnce(Fgclut) -> Fgclut>(&self, f: F) -> &Self {
        self.fgclut_reg().with(f);
        self
    }

    #[doc="Get the BGCLUT Register."]
    #[inline] pub fn bgclut_reg(&self) -> ::bobbin_mcu::register::Register<Bgclut> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bgclut, 0x800)
    }

    #[doc="Get the *mut pointer for the BGCLUT register."]
    #[inline] pub fn bgclut_mut(&self) -> *mut Bgclut { 
        self.bgclut_reg().ptr()
    }

    #[doc="Get the *const pointer for the BGCLUT register."]
    #[inline] pub fn bgclut_ptr(&self) -> *const Bgclut { 
        self.bgclut_reg().ptr()
    }

    #[doc="Read the BGCLUT register."]
    #[inline] pub fn bgclut(&self) -> Bgclut { 
        self.bgclut_reg().read()
    }

    #[doc="Write the BGCLUT register."]
    #[inline] pub fn write_bgclut(&self, value: Bgclut) -> &Self { 
        self.bgclut_reg().write(value);
        self
    }

    #[doc="Set the BGCLUT register."]
    #[inline] pub fn set_bgclut<F: FnOnce(Bgclut) -> Bgclut>(&self, f: F) -> &Self {
        self.bgclut_reg().set(f);
        self
    }

    #[doc="Modify the BGCLUT register."]
    #[inline] pub fn with_bgclut<F: FnOnce(Bgclut) -> Bgclut>(&self, f: F) -> &Self {
        self.bgclut_reg().with(f);
        self
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="DMA2D mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Configuration Error Interrupt Enable"]
    #[inline] pub fn ceie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CEIE != 0"]
    #[inline] pub fn test_ceie(&self) -> bool {
        self.ceie() != 0
    }

    #[doc="Sets the CEIE field."]
    #[inline] pub fn set_ceie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CLUT transfer complete interrupt enable"]
    #[inline] pub fn ctcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTCIE != 0"]
    #[inline] pub fn test_ctcie(&self) -> bool {
        self.ctcie() != 0
    }

    #[doc="Sets the CTCIE field."]
    #[inline] pub fn set_ctcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CLUT access error interrupt enable"]
    #[inline] pub fn caeie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAEIE != 0"]
    #[inline] pub fn test_caeie(&self) -> bool {
        self.caeie() != 0
    }

    #[doc="Sets the CAEIE field."]
    #[inline] pub fn set_caeie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Transfer watermark interrupt enable"]
    #[inline] pub fn twie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TWIE != 0"]
    #[inline] pub fn test_twie(&self) -> bool {
        self.twie() != 0
    }

    #[doc="Sets the TWIE field."]
    #[inline] pub fn set_twie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Transfer complete interrupt enable"]
    #[inline] pub fn tcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer error interrupt enable"]
    #[inline] pub fn teie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TEIE != 0"]
    #[inline] pub fn test_teie(&self) -> bool {
        self.teie() != 0
    }

    #[doc="Sets the TEIE field."]
    #[inline] pub fn set_teie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Abort"]
    #[inline] pub fn abort(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ABORT != 0"]
    #[inline] pub fn test_abort(&self) -> bool {
        self.abort() != 0
    }

    #[doc="Sets the ABORT field."]
    #[inline] pub fn set_abort<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Suspend"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Start"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.ceie() != 0 { try!(write!(f, " ceie"))}
        if self.ctcie() != 0 { try!(write!(f, " ctcie"))}
        if self.caeie() != 0 { try!(write!(f, " caeie"))}
        if self.twie() != 0 { try!(write!(f, " twie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.teie() != 0 { try!(write!(f, " teie"))}
        if self.abort() != 0 { try!(write!(f, " abort"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Configuration error interrupt flag"]
    #[inline] pub fn ceif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CEIF != 0"]
    #[inline] pub fn test_ceif(&self) -> bool {
        self.ceif() != 0
    }

    #[doc="Sets the CEIF field."]
    #[inline] pub fn set_ceif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="CLUT transfer complete interrupt flag"]
    #[inline] pub fn ctcif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTCIF != 0"]
    #[inline] pub fn test_ctcif(&self) -> bool {
        self.ctcif() != 0
    }

    #[doc="Sets the CTCIF field."]
    #[inline] pub fn set_ctcif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CLUT access error interrupt flag"]
    #[inline] pub fn caeif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CAEIF != 0"]
    #[inline] pub fn test_caeif(&self) -> bool {
        self.caeif() != 0
    }

    #[doc="Sets the CAEIF field."]
    #[inline] pub fn set_caeif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transfer watermark interrupt flag"]
    #[inline] pub fn twif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TWIF != 0"]
    #[inline] pub fn test_twif(&self) -> bool {
        self.twif() != 0
    }

    #[doc="Sets the TWIF field."]
    #[inline] pub fn set_twif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transfer complete interrupt flag"]
    #[inline] pub fn tcif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TCIF != 0"]
    #[inline] pub fn test_tcif(&self) -> bool {
        self.tcif() != 0
    }

    #[doc="Sets the TCIF field."]
    #[inline] pub fn set_tcif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transfer error interrupt flag"]
    #[inline] pub fn teif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TEIF != 0"]
    #[inline] pub fn test_teif(&self) -> bool {
        self.teif() != 0
    }

    #[doc="Sets the TEIF field."]
    #[inline] pub fn set_teif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.ceif() != 0 { try!(write!(f, " ceif"))}
        if self.ctcif() != 0 { try!(write!(f, " ctcif"))}
        if self.caeif() != 0 { try!(write!(f, " caeif"))}
        if self.twif() != 0 { try!(write!(f, " twif"))}
        if self.tcif() != 0 { try!(write!(f, " tcif"))}
        if self.teif() != 0 { try!(write!(f, " teif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc="Clear configuration error interrupt flag"]
    #[inline] pub fn cceif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CCEIF != 0"]
    #[inline] pub fn test_cceif(&self) -> bool {
        self.cceif() != 0
    }

    #[doc="Sets the CCEIF field."]
    #[inline] pub fn set_cceif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear CLUT transfer complete interrupt flag"]
    #[inline] pub fn cctcif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CCTCIF != 0"]
    #[inline] pub fn test_cctcif(&self) -> bool {
        self.cctcif() != 0
    }

    #[doc="Sets the CCTCIF field."]
    #[inline] pub fn set_cctcif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear CLUT access error interrupt flag"]
    #[inline] pub fn caecif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CAECIF != 0"]
    #[inline] pub fn test_caecif(&self) -> bool {
        self.caecif() != 0
    }

    #[doc="Sets the CAECIF field."]
    #[inline] pub fn set_caecif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear transfer watermark interrupt flag"]
    #[inline] pub fn ctwif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTWIF != 0"]
    #[inline] pub fn test_ctwif(&self) -> bool {
        self.ctwif() != 0
    }

    #[doc="Sets the CTWIF field."]
    #[inline] pub fn set_ctwif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear transfer complete interrupt flag"]
    #[inline] pub fn ctcif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTCIF != 0"]
    #[inline] pub fn test_ctcif(&self) -> bool {
        self.ctcif() != 0
    }

    #[doc="Sets the CTCIF field."]
    #[inline] pub fn set_ctcif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear Transfer error interrupt flag"]
    #[inline] pub fn cteif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTEIF != 0"]
    #[inline] pub fn test_cteif(&self) -> bool {
        self.cteif() != 0
    }

    #[doc="Sets the CTEIF field."]
    #[inline] pub fn set_cteif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.cceif() != 0 { try!(write!(f, " cceif"))}
        if self.cctcif() != 0 { try!(write!(f, " cctcif"))}
        if self.caecif() != 0 { try!(write!(f, " caecif"))}
        if self.ctwif() != 0 { try!(write!(f, " ctwif"))}
        if self.ctcif() != 0 { try!(write!(f, " ctcif"))}
        if self.cteif() != 0 { try!(write!(f, " cteif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="foreground memory address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fgmar(pub u32);
impl Fgmar {
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

impl From<u32> for Fgmar {
    #[inline]
    fn from(other: u32) -> Self {
         Fgmar(other)
    }
}

impl ::core::fmt::Display for Fgmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fgmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="foreground offset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fgor(pub u32);
impl Fgor {
    #[doc="Line offset"]
    #[inline] pub fn lo(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if LO != 0"]
    #[inline] pub fn test_lo(&self) -> bool {
        self.lo() != 0
    }

    #[doc="Sets the LO field."]
    #[inline] pub fn set_lo<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fgor {
    #[inline]
    fn from(other: u32) -> Self {
         Fgor(other)
    }
}

impl ::core::fmt::Display for Fgor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fgor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lo() != 0 { try!(write!(f, " lo=0x{:x}", self.lo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="background memory address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bgmar(pub u32);
impl Bgmar {
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

impl From<u32> for Bgmar {
    #[inline]
    fn from(other: u32) -> Self {
         Bgmar(other)
    }
}

impl ::core::fmt::Display for Bgmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bgmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="background offset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bgor(pub u32);
impl Bgor {
    #[doc="Line offset"]
    #[inline] pub fn lo(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if LO != 0"]
    #[inline] pub fn test_lo(&self) -> bool {
        self.lo() != 0
    }

    #[doc="Sets the LO field."]
    #[inline] pub fn set_lo<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bgor {
    #[inline]
    fn from(other: u32) -> Self {
         Bgor(other)
    }
}

impl ::core::fmt::Display for Bgor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bgor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lo() != 0 { try!(write!(f, " lo=0x{:x}", self.lo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="foreground PFC control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fgpfccr(pub u32);
impl Fgpfccr {
    #[doc="Alpha value"]
    #[inline] pub fn alpha(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if ALPHA != 0"]
    #[inline] pub fn test_alpha(&self) -> bool {
        self.alpha() != 0
    }

    #[doc="Sets the ALPHA field."]
    #[inline] pub fn set_alpha<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Alpha mode"]
    #[inline] pub fn am(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if AM != 0"]
    #[inline] pub fn test_am(&self) -> bool {
        self.am() != 0
    }

    #[doc="Sets the AM field."]
    #[inline] pub fn set_am<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CLUT size"]
    #[inline] pub fn cs(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CS != 0"]
    #[inline] pub fn test_cs(&self) -> bool {
        self.cs() != 0
    }

    #[doc="Sets the CS field."]
    #[inline] pub fn set_cs<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Start"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="CLUT color mode"]
    #[inline] pub fn ccm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CCM != 0"]
    #[inline] pub fn test_ccm(&self) -> bool {
        self.ccm() != 0
    }

    #[doc="Sets the CCM field."]
    #[inline] pub fn set_ccm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Color mode"]
    #[inline] pub fn cm(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CM != 0"]
    #[inline] pub fn test_cm(&self) -> bool {
        self.cm() != 0
    }

    #[doc="Sets the CM field."]
    #[inline] pub fn set_cm<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fgpfccr {
    #[inline]
    fn from(other: u32) -> Self {
         Fgpfccr(other)
    }
}

impl ::core::fmt::Display for Fgpfccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fgpfccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.alpha() != 0 { try!(write!(f, " alpha=0x{:x}", self.alpha()))}
        if self.am() != 0 { try!(write!(f, " am=0x{:x}", self.am()))}
        if self.cs() != 0 { try!(write!(f, " cs=0x{:x}", self.cs()))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.ccm() != 0 { try!(write!(f, " ccm"))}
        if self.cm() != 0 { try!(write!(f, " cm=0x{:x}", self.cm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="foreground color register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fgcolr(pub u32);
impl Fgcolr {
    #[doc="Red Value"]
    #[inline] pub fn red(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if RED != 0"]
    #[inline] pub fn test_red(&self) -> bool {
        self.red() != 0
    }

    #[doc="Sets the RED field."]
    #[inline] pub fn set_red<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Green Value"]
    #[inline] pub fn green(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GREEN != 0"]
    #[inline] pub fn test_green(&self) -> bool {
        self.green() != 0
    }

    #[doc="Sets the GREEN field."]
    #[inline] pub fn set_green<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Blue Value"]
    #[inline] pub fn blue(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BLUE != 0"]
    #[inline] pub fn test_blue(&self) -> bool {
        self.blue() != 0
    }

    #[doc="Sets the BLUE field."]
    #[inline] pub fn set_blue<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fgcolr {
    #[inline]
    fn from(other: u32) -> Self {
         Fgcolr(other)
    }
}

impl ::core::fmt::Display for Fgcolr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fgcolr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.red() != 0 { try!(write!(f, " red=0x{:x}", self.red()))}
        if self.green() != 0 { try!(write!(f, " green=0x{:x}", self.green()))}
        if self.blue() != 0 { try!(write!(f, " blue=0x{:x}", self.blue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="background PFC control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bgpfccr(pub u32);
impl Bgpfccr {
    #[doc="Alpha value"]
    #[inline] pub fn alpha(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if ALPHA != 0"]
    #[inline] pub fn test_alpha(&self) -> bool {
        self.alpha() != 0
    }

    #[doc="Sets the ALPHA field."]
    #[inline] pub fn set_alpha<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Alpha mode"]
    #[inline] pub fn am(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if AM != 0"]
    #[inline] pub fn test_am(&self) -> bool {
        self.am() != 0
    }

    #[doc="Sets the AM field."]
    #[inline] pub fn set_am<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CLUT size"]
    #[inline] pub fn cs(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CS != 0"]
    #[inline] pub fn test_cs(&self) -> bool {
        self.cs() != 0
    }

    #[doc="Sets the CS field."]
    #[inline] pub fn set_cs<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Start"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="CLUT Color mode"]
    #[inline] pub fn ccm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CCM != 0"]
    #[inline] pub fn test_ccm(&self) -> bool {
        self.ccm() != 0
    }

    #[doc="Sets the CCM field."]
    #[inline] pub fn set_ccm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Color mode"]
    #[inline] pub fn cm(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CM != 0"]
    #[inline] pub fn test_cm(&self) -> bool {
        self.cm() != 0
    }

    #[doc="Sets the CM field."]
    #[inline] pub fn set_cm<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bgpfccr {
    #[inline]
    fn from(other: u32) -> Self {
         Bgpfccr(other)
    }
}

impl ::core::fmt::Display for Bgpfccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bgpfccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.alpha() != 0 { try!(write!(f, " alpha=0x{:x}", self.alpha()))}
        if self.am() != 0 { try!(write!(f, " am=0x{:x}", self.am()))}
        if self.cs() != 0 { try!(write!(f, " cs=0x{:x}", self.cs()))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.ccm() != 0 { try!(write!(f, " ccm"))}
        if self.cm() != 0 { try!(write!(f, " cm=0x{:x}", self.cm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="background color register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bgcolr(pub u32);
impl Bgcolr {
    #[doc="Red Value"]
    #[inline] pub fn red(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if RED != 0"]
    #[inline] pub fn test_red(&self) -> bool {
        self.red() != 0
    }

    #[doc="Sets the RED field."]
    #[inline] pub fn set_red<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Green Value"]
    #[inline] pub fn green(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GREEN != 0"]
    #[inline] pub fn test_green(&self) -> bool {
        self.green() != 0
    }

    #[doc="Sets the GREEN field."]
    #[inline] pub fn set_green<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Blue Value"]
    #[inline] pub fn blue(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BLUE != 0"]
    #[inline] pub fn test_blue(&self) -> bool {
        self.blue() != 0
    }

    #[doc="Sets the BLUE field."]
    #[inline] pub fn set_blue<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bgcolr {
    #[inline]
    fn from(other: u32) -> Self {
         Bgcolr(other)
    }
}

impl ::core::fmt::Display for Bgcolr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bgcolr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.red() != 0 { try!(write!(f, " red=0x{:x}", self.red()))}
        if self.green() != 0 { try!(write!(f, " green=0x{:x}", self.green()))}
        if self.blue() != 0 { try!(write!(f, " blue=0x{:x}", self.blue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="foreground CLUT memory address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fgcmar(pub u32);
impl Fgcmar {
    #[doc="Memory Address"]
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

impl From<u32> for Fgcmar {
    #[inline]
    fn from(other: u32) -> Self {
         Fgcmar(other)
    }
}

impl ::core::fmt::Display for Fgcmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fgcmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="background CLUT memory address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bgcmar(pub u32);
impl Bgcmar {
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

impl From<u32> for Bgcmar {
    #[inline]
    fn from(other: u32) -> Self {
         Bgcmar(other)
    }
}

impl ::core::fmt::Display for Bgcmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bgcmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="output PFC control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opfccr(pub u32);
impl Opfccr {
    #[doc="Color mode"]
    #[inline] pub fn cm(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if CM != 0"]
    #[inline] pub fn test_cm(&self) -> bool {
        self.cm() != 0
    }

    #[doc="Sets the CM field."]
    #[inline] pub fn set_cm<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Opfccr {
    #[inline]
    fn from(other: u32) -> Self {
         Opfccr(other)
    }
}

impl ::core::fmt::Display for Opfccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opfccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cm() != 0 { try!(write!(f, " cm=0x{:x}", self.cm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="output color register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ocolr(pub u32);
impl Ocolr {
    #[doc="Alpha Channel Value"]
    #[inline] pub fn aplha(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if APLHA != 0"]
    #[inline] pub fn test_aplha(&self) -> bool {
        self.aplha() != 0
    }

    #[doc="Sets the APLHA field."]
    #[inline] pub fn set_aplha<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Red Value"]
    #[inline] pub fn red(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if RED != 0"]
    #[inline] pub fn test_red(&self) -> bool {
        self.red() != 0
    }

    #[doc="Sets the RED field."]
    #[inline] pub fn set_red<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Green Value"]
    #[inline] pub fn green(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GREEN != 0"]
    #[inline] pub fn test_green(&self) -> bool {
        self.green() != 0
    }

    #[doc="Sets the GREEN field."]
    #[inline] pub fn set_green<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Blue Value"]
    #[inline] pub fn blue(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BLUE != 0"]
    #[inline] pub fn test_blue(&self) -> bool {
        self.blue() != 0
    }

    #[doc="Sets the BLUE field."]
    #[inline] pub fn set_blue<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ocolr {
    #[inline]
    fn from(other: u32) -> Self {
         Ocolr(other)
    }
}

impl ::core::fmt::Display for Ocolr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ocolr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aplha() != 0 { try!(write!(f, " aplha=0x{:x}", self.aplha()))}
        if self.red() != 0 { try!(write!(f, " red=0x{:x}", self.red()))}
        if self.green() != 0 { try!(write!(f, " green=0x{:x}", self.green()))}
        if self.blue() != 0 { try!(write!(f, " blue=0x{:x}", self.blue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="output memory address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Omar(pub u32);
impl Omar {
    #[doc="Memory Address"]
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

impl From<u32> for Omar {
    #[inline]
    fn from(other: u32) -> Self {
         Omar(other)
    }
}

impl ::core::fmt::Display for Omar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Omar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="output offset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Oor(pub u32);
impl Oor {
    #[doc="Line Offset"]
    #[inline] pub fn lo(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if LO != 0"]
    #[inline] pub fn test_lo(&self) -> bool {
        self.lo() != 0
    }

    #[doc="Sets the LO field."]
    #[inline] pub fn set_lo<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Oor {
    #[inline]
    fn from(other: u32) -> Self {
         Oor(other)
    }
}

impl ::core::fmt::Display for Oor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Oor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lo() != 0 { try!(write!(f, " lo=0x{:x}", self.lo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="number of line register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nlr(pub u32);
impl Nlr {
    #[doc="Pixel per lines"]
    #[inline] pub fn pl(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3fff) as u16) } // [29:16]
    }

    #[doc="Returns true if PL != 0"]
    #[inline] pub fn test_pl(&self) -> bool {
        self.pl() != 0
    }

    #[doc="Sets the PL field."]
    #[inline] pub fn set_pl<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Number of lines"]
    #[inline] pub fn nl(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if NL != 0"]
    #[inline] pub fn test_nl(&self) -> bool {
        self.nl() != 0
    }

    #[doc="Sets the NL field."]
    #[inline] pub fn set_nl<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Nlr {
    #[inline]
    fn from(other: u32) -> Self {
         Nlr(other)
    }
}

impl ::core::fmt::Display for Nlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pl() != 0 { try!(write!(f, " pl=0x{:x}", self.pl()))}
        if self.nl() != 0 { try!(write!(f, " nl=0x{:x}", self.nl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="line watermark register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lwr(pub u32);
impl Lwr {
    #[doc="Line watermark"]
    #[inline] pub fn lw(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if LW != 0"]
    #[inline] pub fn test_lw(&self) -> bool {
        self.lw() != 0
    }

    #[doc="Sets the LW field."]
    #[inline] pub fn set_lw<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lwr {
    #[inline]
    fn from(other: u32) -> Self {
         Lwr(other)
    }
}

impl ::core::fmt::Display for Lwr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lwr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lw() != 0 { try!(write!(f, " lw=0x{:x}", self.lw()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB master timer configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Amtcr(pub u32);
impl Amtcr {
    #[doc="Dead Time"]
    #[inline] pub fn dt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DT != 0"]
    #[inline] pub fn test_dt(&self) -> bool {
        self.dt() != 0
    }

    #[doc="Sets the DT field."]
    #[inline] pub fn set_dt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enable"]
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

}

impl From<u32> for Amtcr {
    #[inline]
    fn from(other: u32) -> Self {
         Amtcr(other)
    }
}

impl ::core::fmt::Display for Amtcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Amtcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
        if self.en() != 0 { try!(write!(f, " en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FGCLUT"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fgclut(pub u32);
impl Fgclut {
    #[doc="APLHA"]
    #[inline] pub fn aplha(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if APLHA != 0"]
    #[inline] pub fn test_aplha(&self) -> bool {
        self.aplha() != 0
    }

    #[doc="Sets the APLHA field."]
    #[inline] pub fn set_aplha<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="RED"]
    #[inline] pub fn red(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if RED != 0"]
    #[inline] pub fn test_red(&self) -> bool {
        self.red() != 0
    }

    #[doc="Sets the RED field."]
    #[inline] pub fn set_red<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GREEN"]
    #[inline] pub fn green(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GREEN != 0"]
    #[inline] pub fn test_green(&self) -> bool {
        self.green() != 0
    }

    #[doc="Sets the GREEN field."]
    #[inline] pub fn set_green<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BLUE"]
    #[inline] pub fn blue(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BLUE != 0"]
    #[inline] pub fn test_blue(&self) -> bool {
        self.blue() != 0
    }

    #[doc="Sets the BLUE field."]
    #[inline] pub fn set_blue<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fgclut {
    #[inline]
    fn from(other: u32) -> Self {
         Fgclut(other)
    }
}

impl ::core::fmt::Display for Fgclut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fgclut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aplha() != 0 { try!(write!(f, " aplha=0x{:x}", self.aplha()))}
        if self.red() != 0 { try!(write!(f, " red=0x{:x}", self.red()))}
        if self.green() != 0 { try!(write!(f, " green=0x{:x}", self.green()))}
        if self.blue() != 0 { try!(write!(f, " blue=0x{:x}", self.blue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BGCLUT"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bgclut(pub u32);
impl Bgclut {
    #[doc="APLHA"]
    #[inline] pub fn aplha(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if APLHA != 0"]
    #[inline] pub fn test_aplha(&self) -> bool {
        self.aplha() != 0
    }

    #[doc="Sets the APLHA field."]
    #[inline] pub fn set_aplha<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="RED"]
    #[inline] pub fn red(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if RED != 0"]
    #[inline] pub fn test_red(&self) -> bool {
        self.red() != 0
    }

    #[doc="Sets the RED field."]
    #[inline] pub fn set_red<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GREEN"]
    #[inline] pub fn green(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GREEN != 0"]
    #[inline] pub fn test_green(&self) -> bool {
        self.green() != 0
    }

    #[doc="Sets the GREEN field."]
    #[inline] pub fn set_green<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BLUE"]
    #[inline] pub fn blue(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BLUE != 0"]
    #[inline] pub fn test_blue(&self) -> bool {
        self.blue() != 0
    }

    #[doc="Sets the BLUE field."]
    #[inline] pub fn set_blue<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bgclut {
    #[inline]
    fn from(other: u32) -> Self {
         Bgclut(other)
    }
}

impl ::core::fmt::Display for Bgclut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bgclut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aplha() != 0 { try!(write!(f, " aplha=0x{:x}", self.aplha()))}
        if self.red() != 0 { try!(write!(f, " red=0x{:x}", self.red()))}
        if self.green() != 0 { try!(write!(f, " green=0x{:x}", self.green()))}
        if self.blue() != 0 { try!(write!(f, " blue=0x{:x}", self.blue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

