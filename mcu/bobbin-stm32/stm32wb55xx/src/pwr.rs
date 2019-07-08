
::bobbin_mcu::periph!( PWR, Pwr, PWR_PERIPH, PwrPeriph, PWR_OWNED, PWR_REF_COUNT, 0x58000400, 0x00, 0x01);


#[doc="Power control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PwrPeriph(pub usize);
impl PwrPeriph {
    #[doc="Get the CR1 Register."]
    #[inline] pub fn cr1_reg(&self) -> ::bobbin_mcu::register::Register<Cr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr1, 0x0)
    }

    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        self.cr1_reg().read()
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn write_cr1(&self, value: Cr1) -> &Self { 
        self.cr1_reg().write(value);
        self
    }

    #[doc="Set the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().set(f);
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().with(f);
        self
    }

    #[doc="Get the CR2 Register."]
    #[inline] pub fn cr2_reg(&self) -> ::bobbin_mcu::register::Register<Cr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr2, 0x4)
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        self.cr2_reg().read()
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn write_cr2(&self, value: Cr2) -> &Self { 
        self.cr2_reg().write(value);
        self
    }

    #[doc="Set the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().set(f);
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().with(f);
        self
    }

    #[doc="Get the CR3 Register."]
    #[inline] pub fn cr3_reg(&self) -> ::bobbin_mcu::register::Register<Cr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr3, 0x8)
    }

    #[doc="Get the *mut pointer for the CR3 register."]
    #[inline] pub fn cr3_mut(&self) -> *mut Cr3 { 
        self.cr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR3 register."]
    #[inline] pub fn cr3_ptr(&self) -> *const Cr3 { 
        self.cr3_reg().ptr()
    }

    #[doc="Read the CR3 register."]
    #[inline] pub fn cr3(&self) -> Cr3 { 
        self.cr3_reg().read()
    }

    #[doc="Write the CR3 register."]
    #[inline] pub fn write_cr3(&self, value: Cr3) -> &Self { 
        self.cr3_reg().write(value);
        self
    }

    #[doc="Set the CR3 register."]
    #[inline] pub fn set_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        self.cr3_reg().set(f);
        self
    }

    #[doc="Modify the CR3 register."]
    #[inline] pub fn with_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        self.cr3_reg().with(f);
        self
    }

    #[doc="Get the CR4 Register."]
    #[inline] pub fn cr4_reg(&self) -> ::bobbin_mcu::register::Register<Cr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr4, 0xc)
    }

    #[doc="Get the *mut pointer for the CR4 register."]
    #[inline] pub fn cr4_mut(&self) -> *mut Cr4 { 
        self.cr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR4 register."]
    #[inline] pub fn cr4_ptr(&self) -> *const Cr4 { 
        self.cr4_reg().ptr()
    }

    #[doc="Read the CR4 register."]
    #[inline] pub fn cr4(&self) -> Cr4 { 
        self.cr4_reg().read()
    }

    #[doc="Write the CR4 register."]
    #[inline] pub fn write_cr4(&self, value: Cr4) -> &Self { 
        self.cr4_reg().write(value);
        self
    }

    #[doc="Set the CR4 register."]
    #[inline] pub fn set_cr4<F: FnOnce(Cr4) -> Cr4>(&self, f: F) -> &Self {
        self.cr4_reg().set(f);
        self
    }

    #[doc="Modify the CR4 register."]
    #[inline] pub fn with_cr4<F: FnOnce(Cr4) -> Cr4>(&self, f: F) -> &Self {
        self.cr4_reg().with(f);
        self
    }

    #[doc="Get the SR1 Register."]
    #[inline] pub fn sr1_reg(&self) -> ::bobbin_mcu::register::Register<Sr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr1, 0x10)
    }

    #[doc="Get the *mut pointer for the SR1 register."]
    #[inline] pub fn sr1_mut(&self) -> *mut Sr1 { 
        self.sr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR1 register."]
    #[inline] pub fn sr1_ptr(&self) -> *const Sr1 { 
        self.sr1_reg().ptr()
    }

    #[doc="Read the SR1 register."]
    #[inline] pub fn sr1(&self) -> Sr1 { 
        self.sr1_reg().read()
    }

    #[doc="Get the SR2 Register."]
    #[inline] pub fn sr2_reg(&self) -> ::bobbin_mcu::register::Register<Sr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr2, 0x14)
    }

    #[doc="Get the *mut pointer for the SR2 register."]
    #[inline] pub fn sr2_mut(&self) -> *mut Sr2 { 
        self.sr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR2 register."]
    #[inline] pub fn sr2_ptr(&self) -> *const Sr2 { 
        self.sr2_reg().ptr()
    }

    #[doc="Read the SR2 register."]
    #[inline] pub fn sr2(&self) -> Sr2 { 
        self.sr2_reg().read()
    }

    #[doc="Get the SCR Register."]
    #[inline] pub fn scr_reg(&self) -> ::bobbin_mcu::register::Register<Scr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Scr, 0x18)
    }

    #[doc="Get the *mut pointer for the SCR register."]
    #[inline] pub fn scr_mut(&self) -> *mut Scr { 
        self.scr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SCR register."]
    #[inline] pub fn scr_ptr(&self) -> *const Scr { 
        self.scr_reg().ptr()
    }

    #[doc="Write the SCR register."]
    #[inline] pub fn write_scr(&self, value: Scr) -> &Self { 
        self.scr_reg().write(value);
        self
    }

    #[doc="Set the SCR register."]
    #[inline] pub fn set_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
        self.scr_reg().set(f);
        self
    }

    #[doc="Get the CR5 Register."]
    #[inline] pub fn cr5_reg(&self) -> ::bobbin_mcu::register::Register<Cr5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr5, 0x1c)
    }

    #[doc="Get the *mut pointer for the CR5 register."]
    #[inline] pub fn cr5_mut(&self) -> *mut Cr5 { 
        self.cr5_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR5 register."]
    #[inline] pub fn cr5_ptr(&self) -> *const Cr5 { 
        self.cr5_reg().ptr()
    }

    #[doc="Read the CR5 register."]
    #[inline] pub fn cr5(&self) -> Cr5 { 
        self.cr5_reg().read()
    }

    #[doc="Write the CR5 register."]
    #[inline] pub fn write_cr5(&self, value: Cr5) -> &Self { 
        self.cr5_reg().write(value);
        self
    }

    #[doc="Set the CR5 register."]
    #[inline] pub fn set_cr5<F: FnOnce(Cr5) -> Cr5>(&self, f: F) -> &Self {
        self.cr5_reg().set(f);
        self
    }

    #[doc="Modify the CR5 register."]
    #[inline] pub fn with_cr5<F: FnOnce(Cr5) -> Cr5>(&self, f: F) -> &Self {
        self.cr5_reg().with(f);
        self
    }

    #[doc="Get the PUCRA Register."]
    #[inline] pub fn pucra_reg(&self) -> ::bobbin_mcu::register::Register<Pucra> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pucra, 0x20)
    }

    #[doc="Get the *mut pointer for the PUCRA register."]
    #[inline] pub fn pucra_mut(&self) -> *mut Pucra { 
        self.pucra_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUCRA register."]
    #[inline] pub fn pucra_ptr(&self) -> *const Pucra { 
        self.pucra_reg().ptr()
    }

    #[doc="Read the PUCRA register."]
    #[inline] pub fn pucra(&self) -> Pucra { 
        self.pucra_reg().read()
    }

    #[doc="Write the PUCRA register."]
    #[inline] pub fn write_pucra(&self, value: Pucra) -> &Self { 
        self.pucra_reg().write(value);
        self
    }

    #[doc="Set the PUCRA register."]
    #[inline] pub fn set_pucra<F: FnOnce(Pucra) -> Pucra>(&self, f: F) -> &Self {
        self.pucra_reg().set(f);
        self
    }

    #[doc="Modify the PUCRA register."]
    #[inline] pub fn with_pucra<F: FnOnce(Pucra) -> Pucra>(&self, f: F) -> &Self {
        self.pucra_reg().with(f);
        self
    }

    #[doc="Get the PDCRA Register."]
    #[inline] pub fn pdcra_reg(&self) -> ::bobbin_mcu::register::Register<Pdcra> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdcra, 0x24)
    }

    #[doc="Get the *mut pointer for the PDCRA register."]
    #[inline] pub fn pdcra_mut(&self) -> *mut Pdcra { 
        self.pdcra_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDCRA register."]
    #[inline] pub fn pdcra_ptr(&self) -> *const Pdcra { 
        self.pdcra_reg().ptr()
    }

    #[doc="Read the PDCRA register."]
    #[inline] pub fn pdcra(&self) -> Pdcra { 
        self.pdcra_reg().read()
    }

    #[doc="Write the PDCRA register."]
    #[inline] pub fn write_pdcra(&self, value: Pdcra) -> &Self { 
        self.pdcra_reg().write(value);
        self
    }

    #[doc="Set the PDCRA register."]
    #[inline] pub fn set_pdcra<F: FnOnce(Pdcra) -> Pdcra>(&self, f: F) -> &Self {
        self.pdcra_reg().set(f);
        self
    }

    #[doc="Modify the PDCRA register."]
    #[inline] pub fn with_pdcra<F: FnOnce(Pdcra) -> Pdcra>(&self, f: F) -> &Self {
        self.pdcra_reg().with(f);
        self
    }

    #[doc="Get the PUCRB Register."]
    #[inline] pub fn pucrb_reg(&self) -> ::bobbin_mcu::register::Register<Pucrb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pucrb, 0x28)
    }

    #[doc="Get the *mut pointer for the PUCRB register."]
    #[inline] pub fn pucrb_mut(&self) -> *mut Pucrb { 
        self.pucrb_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUCRB register."]
    #[inline] pub fn pucrb_ptr(&self) -> *const Pucrb { 
        self.pucrb_reg().ptr()
    }

    #[doc="Read the PUCRB register."]
    #[inline] pub fn pucrb(&self) -> Pucrb { 
        self.pucrb_reg().read()
    }

    #[doc="Write the PUCRB register."]
    #[inline] pub fn write_pucrb(&self, value: Pucrb) -> &Self { 
        self.pucrb_reg().write(value);
        self
    }

    #[doc="Set the PUCRB register."]
    #[inline] pub fn set_pucrb<F: FnOnce(Pucrb) -> Pucrb>(&self, f: F) -> &Self {
        self.pucrb_reg().set(f);
        self
    }

    #[doc="Modify the PUCRB register."]
    #[inline] pub fn with_pucrb<F: FnOnce(Pucrb) -> Pucrb>(&self, f: F) -> &Self {
        self.pucrb_reg().with(f);
        self
    }

    #[doc="Get the PDCRB Register."]
    #[inline] pub fn pdcrb_reg(&self) -> ::bobbin_mcu::register::Register<Pdcrb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdcrb, 0x2c)
    }

    #[doc="Get the *mut pointer for the PDCRB register."]
    #[inline] pub fn pdcrb_mut(&self) -> *mut Pdcrb { 
        self.pdcrb_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDCRB register."]
    #[inline] pub fn pdcrb_ptr(&self) -> *const Pdcrb { 
        self.pdcrb_reg().ptr()
    }

    #[doc="Read the PDCRB register."]
    #[inline] pub fn pdcrb(&self) -> Pdcrb { 
        self.pdcrb_reg().read()
    }

    #[doc="Write the PDCRB register."]
    #[inline] pub fn write_pdcrb(&self, value: Pdcrb) -> &Self { 
        self.pdcrb_reg().write(value);
        self
    }

    #[doc="Set the PDCRB register."]
    #[inline] pub fn set_pdcrb<F: FnOnce(Pdcrb) -> Pdcrb>(&self, f: F) -> &Self {
        self.pdcrb_reg().set(f);
        self
    }

    #[doc="Modify the PDCRB register."]
    #[inline] pub fn with_pdcrb<F: FnOnce(Pdcrb) -> Pdcrb>(&self, f: F) -> &Self {
        self.pdcrb_reg().with(f);
        self
    }

    #[doc="Get the PUCRC Register."]
    #[inline] pub fn pucrc_reg(&self) -> ::bobbin_mcu::register::Register<Pucrc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pucrc, 0x30)
    }

    #[doc="Get the *mut pointer for the PUCRC register."]
    #[inline] pub fn pucrc_mut(&self) -> *mut Pucrc { 
        self.pucrc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUCRC register."]
    #[inline] pub fn pucrc_ptr(&self) -> *const Pucrc { 
        self.pucrc_reg().ptr()
    }

    #[doc="Read the PUCRC register."]
    #[inline] pub fn pucrc(&self) -> Pucrc { 
        self.pucrc_reg().read()
    }

    #[doc="Write the PUCRC register."]
    #[inline] pub fn write_pucrc(&self, value: Pucrc) -> &Self { 
        self.pucrc_reg().write(value);
        self
    }

    #[doc="Set the PUCRC register."]
    #[inline] pub fn set_pucrc<F: FnOnce(Pucrc) -> Pucrc>(&self, f: F) -> &Self {
        self.pucrc_reg().set(f);
        self
    }

    #[doc="Modify the PUCRC register."]
    #[inline] pub fn with_pucrc<F: FnOnce(Pucrc) -> Pucrc>(&self, f: F) -> &Self {
        self.pucrc_reg().with(f);
        self
    }

    #[doc="Get the PDCRC Register."]
    #[inline] pub fn pdcrc_reg(&self) -> ::bobbin_mcu::register::Register<Pdcrc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdcrc, 0x34)
    }

    #[doc="Get the *mut pointer for the PDCRC register."]
    #[inline] pub fn pdcrc_mut(&self) -> *mut Pdcrc { 
        self.pdcrc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDCRC register."]
    #[inline] pub fn pdcrc_ptr(&self) -> *const Pdcrc { 
        self.pdcrc_reg().ptr()
    }

    #[doc="Read the PDCRC register."]
    #[inline] pub fn pdcrc(&self) -> Pdcrc { 
        self.pdcrc_reg().read()
    }

    #[doc="Write the PDCRC register."]
    #[inline] pub fn write_pdcrc(&self, value: Pdcrc) -> &Self { 
        self.pdcrc_reg().write(value);
        self
    }

    #[doc="Set the PDCRC register."]
    #[inline] pub fn set_pdcrc<F: FnOnce(Pdcrc) -> Pdcrc>(&self, f: F) -> &Self {
        self.pdcrc_reg().set(f);
        self
    }

    #[doc="Modify the PDCRC register."]
    #[inline] pub fn with_pdcrc<F: FnOnce(Pdcrc) -> Pdcrc>(&self, f: F) -> &Self {
        self.pdcrc_reg().with(f);
        self
    }

    #[doc="Get the PUCRD Register."]
    #[inline] pub fn pucrd_reg(&self) -> ::bobbin_mcu::register::Register<Pucrd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pucrd, 0x38)
    }

    #[doc="Get the *mut pointer for the PUCRD register."]
    #[inline] pub fn pucrd_mut(&self) -> *mut Pucrd { 
        self.pucrd_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUCRD register."]
    #[inline] pub fn pucrd_ptr(&self) -> *const Pucrd { 
        self.pucrd_reg().ptr()
    }

    #[doc="Read the PUCRD register."]
    #[inline] pub fn pucrd(&self) -> Pucrd { 
        self.pucrd_reg().read()
    }

    #[doc="Write the PUCRD register."]
    #[inline] pub fn write_pucrd(&self, value: Pucrd) -> &Self { 
        self.pucrd_reg().write(value);
        self
    }

    #[doc="Set the PUCRD register."]
    #[inline] pub fn set_pucrd<F: FnOnce(Pucrd) -> Pucrd>(&self, f: F) -> &Self {
        self.pucrd_reg().set(f);
        self
    }

    #[doc="Modify the PUCRD register."]
    #[inline] pub fn with_pucrd<F: FnOnce(Pucrd) -> Pucrd>(&self, f: F) -> &Self {
        self.pucrd_reg().with(f);
        self
    }

    #[doc="Get the PDCRD Register."]
    #[inline] pub fn pdcrd_reg(&self) -> ::bobbin_mcu::register::Register<Pdcrd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdcrd, 0x3c)
    }

    #[doc="Get the *mut pointer for the PDCRD register."]
    #[inline] pub fn pdcrd_mut(&self) -> *mut Pdcrd { 
        self.pdcrd_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDCRD register."]
    #[inline] pub fn pdcrd_ptr(&self) -> *const Pdcrd { 
        self.pdcrd_reg().ptr()
    }

    #[doc="Read the PDCRD register."]
    #[inline] pub fn pdcrd(&self) -> Pdcrd { 
        self.pdcrd_reg().read()
    }

    #[doc="Write the PDCRD register."]
    #[inline] pub fn write_pdcrd(&self, value: Pdcrd) -> &Self { 
        self.pdcrd_reg().write(value);
        self
    }

    #[doc="Set the PDCRD register."]
    #[inline] pub fn set_pdcrd<F: FnOnce(Pdcrd) -> Pdcrd>(&self, f: F) -> &Self {
        self.pdcrd_reg().set(f);
        self
    }

    #[doc="Modify the PDCRD register."]
    #[inline] pub fn with_pdcrd<F: FnOnce(Pdcrd) -> Pdcrd>(&self, f: F) -> &Self {
        self.pdcrd_reg().with(f);
        self
    }

    #[doc="Get the PUCRE Register."]
    #[inline] pub fn pucre_reg(&self) -> ::bobbin_mcu::register::Register<Pucre> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pucre, 0x40)
    }

    #[doc="Get the *mut pointer for the PUCRE register."]
    #[inline] pub fn pucre_mut(&self) -> *mut Pucre { 
        self.pucre_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUCRE register."]
    #[inline] pub fn pucre_ptr(&self) -> *const Pucre { 
        self.pucre_reg().ptr()
    }

    #[doc="Read the PUCRE register."]
    #[inline] pub fn pucre(&self) -> Pucre { 
        self.pucre_reg().read()
    }

    #[doc="Write the PUCRE register."]
    #[inline] pub fn write_pucre(&self, value: Pucre) -> &Self { 
        self.pucre_reg().write(value);
        self
    }

    #[doc="Set the PUCRE register."]
    #[inline] pub fn set_pucre<F: FnOnce(Pucre) -> Pucre>(&self, f: F) -> &Self {
        self.pucre_reg().set(f);
        self
    }

    #[doc="Modify the PUCRE register."]
    #[inline] pub fn with_pucre<F: FnOnce(Pucre) -> Pucre>(&self, f: F) -> &Self {
        self.pucre_reg().with(f);
        self
    }

    #[doc="Get the PDCRE Register."]
    #[inline] pub fn pdcre_reg(&self) -> ::bobbin_mcu::register::Register<Pdcre> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdcre, 0x44)
    }

    #[doc="Get the *mut pointer for the PDCRE register."]
    #[inline] pub fn pdcre_mut(&self) -> *mut Pdcre { 
        self.pdcre_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDCRE register."]
    #[inline] pub fn pdcre_ptr(&self) -> *const Pdcre { 
        self.pdcre_reg().ptr()
    }

    #[doc="Read the PDCRE register."]
    #[inline] pub fn pdcre(&self) -> Pdcre { 
        self.pdcre_reg().read()
    }

    #[doc="Write the PDCRE register."]
    #[inline] pub fn write_pdcre(&self, value: Pdcre) -> &Self { 
        self.pdcre_reg().write(value);
        self
    }

    #[doc="Set the PDCRE register."]
    #[inline] pub fn set_pdcre<F: FnOnce(Pdcre) -> Pdcre>(&self, f: F) -> &Self {
        self.pdcre_reg().set(f);
        self
    }

    #[doc="Modify the PDCRE register."]
    #[inline] pub fn with_pdcre<F: FnOnce(Pdcre) -> Pdcre>(&self, f: F) -> &Self {
        self.pdcre_reg().with(f);
        self
    }

    #[doc="Get the PUCRH Register."]
    #[inline] pub fn pucrh_reg(&self) -> ::bobbin_mcu::register::Register<Pucrh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pucrh, 0x58)
    }

    #[doc="Get the *mut pointer for the PUCRH register."]
    #[inline] pub fn pucrh_mut(&self) -> *mut Pucrh { 
        self.pucrh_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUCRH register."]
    #[inline] pub fn pucrh_ptr(&self) -> *const Pucrh { 
        self.pucrh_reg().ptr()
    }

    #[doc="Read the PUCRH register."]
    #[inline] pub fn pucrh(&self) -> Pucrh { 
        self.pucrh_reg().read()
    }

    #[doc="Write the PUCRH register."]
    #[inline] pub fn write_pucrh(&self, value: Pucrh) -> &Self { 
        self.pucrh_reg().write(value);
        self
    }

    #[doc="Set the PUCRH register."]
    #[inline] pub fn set_pucrh<F: FnOnce(Pucrh) -> Pucrh>(&self, f: F) -> &Self {
        self.pucrh_reg().set(f);
        self
    }

    #[doc="Modify the PUCRH register."]
    #[inline] pub fn with_pucrh<F: FnOnce(Pucrh) -> Pucrh>(&self, f: F) -> &Self {
        self.pucrh_reg().with(f);
        self
    }

    #[doc="Get the PDCRH Register."]
    #[inline] pub fn pdcrh_reg(&self) -> ::bobbin_mcu::register::Register<Pdcrh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdcrh, 0x5c)
    }

    #[doc="Get the *mut pointer for the PDCRH register."]
    #[inline] pub fn pdcrh_mut(&self) -> *mut Pdcrh { 
        self.pdcrh_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDCRH register."]
    #[inline] pub fn pdcrh_ptr(&self) -> *const Pdcrh { 
        self.pdcrh_reg().ptr()
    }

    #[doc="Read the PDCRH register."]
    #[inline] pub fn pdcrh(&self) -> Pdcrh { 
        self.pdcrh_reg().read()
    }

    #[doc="Write the PDCRH register."]
    #[inline] pub fn write_pdcrh(&self, value: Pdcrh) -> &Self { 
        self.pdcrh_reg().write(value);
        self
    }

    #[doc="Set the PDCRH register."]
    #[inline] pub fn set_pdcrh<F: FnOnce(Pdcrh) -> Pdcrh>(&self, f: F) -> &Self {
        self.pdcrh_reg().set(f);
        self
    }

    #[doc="Modify the PDCRH register."]
    #[inline] pub fn with_pdcrh<F: FnOnce(Pdcrh) -> Pdcrh>(&self, f: F) -> &Self {
        self.pdcrh_reg().with(f);
        self
    }

    #[doc="Get the C2CR1 Register."]
    #[inline] pub fn c2cr1_reg(&self) -> ::bobbin_mcu::register::Register<C2cr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2cr1, 0x80)
    }

    #[doc="Get the *mut pointer for the C2CR1 register."]
    #[inline] pub fn c2cr1_mut(&self) -> *mut C2cr1 { 
        self.c2cr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2CR1 register."]
    #[inline] pub fn c2cr1_ptr(&self) -> *const C2cr1 { 
        self.c2cr1_reg().ptr()
    }

    #[doc="Read the C2CR1 register."]
    #[inline] pub fn c2cr1(&self) -> C2cr1 { 
        self.c2cr1_reg().read()
    }

    #[doc="Write the C2CR1 register."]
    #[inline] pub fn write_c2cr1(&self, value: C2cr1) -> &Self { 
        self.c2cr1_reg().write(value);
        self
    }

    #[doc="Set the C2CR1 register."]
    #[inline] pub fn set_c2cr1<F: FnOnce(C2cr1) -> C2cr1>(&self, f: F) -> &Self {
        self.c2cr1_reg().set(f);
        self
    }

    #[doc="Modify the C2CR1 register."]
    #[inline] pub fn with_c2cr1<F: FnOnce(C2cr1) -> C2cr1>(&self, f: F) -> &Self {
        self.c2cr1_reg().with(f);
        self
    }

    #[doc="Get the C2CR3 Register."]
    #[inline] pub fn c2cr3_reg(&self) -> ::bobbin_mcu::register::Register<C2cr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2cr3, 0x84)
    }

    #[doc="Get the *mut pointer for the C2CR3 register."]
    #[inline] pub fn c2cr3_mut(&self) -> *mut C2cr3 { 
        self.c2cr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2CR3 register."]
    #[inline] pub fn c2cr3_ptr(&self) -> *const C2cr3 { 
        self.c2cr3_reg().ptr()
    }

    #[doc="Read the C2CR3 register."]
    #[inline] pub fn c2cr3(&self) -> C2cr3 { 
        self.c2cr3_reg().read()
    }

    #[doc="Write the C2CR3 register."]
    #[inline] pub fn write_c2cr3(&self, value: C2cr3) -> &Self { 
        self.c2cr3_reg().write(value);
        self
    }

    #[doc="Set the C2CR3 register."]
    #[inline] pub fn set_c2cr3<F: FnOnce(C2cr3) -> C2cr3>(&self, f: F) -> &Self {
        self.c2cr3_reg().set(f);
        self
    }

    #[doc="Modify the C2CR3 register."]
    #[inline] pub fn with_c2cr3<F: FnOnce(C2cr3) -> C2cr3>(&self, f: F) -> &Self {
        self.c2cr3_reg().with(f);
        self
    }

    #[doc="Get the EXTSCR Register."]
    #[inline] pub fn extscr_reg(&self) -> ::bobbin_mcu::register::Register<Extscr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Extscr, 0x88)
    }

    #[doc="Get the *mut pointer for the EXTSCR register."]
    #[inline] pub fn extscr_mut(&self) -> *mut Extscr { 
        self.extscr_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTSCR register."]
    #[inline] pub fn extscr_ptr(&self) -> *const Extscr { 
        self.extscr_reg().ptr()
    }

    #[doc="Read the EXTSCR register."]
    #[inline] pub fn extscr(&self) -> Extscr { 
        self.extscr_reg().read()
    }

    #[doc="Write the EXTSCR register."]
    #[inline] pub fn write_extscr(&self, value: Extscr) -> &Self { 
        self.extscr_reg().write(value);
        self
    }

    #[doc="Set the EXTSCR register."]
    #[inline] pub fn set_extscr<F: FnOnce(Extscr) -> Extscr>(&self, f: F) -> &Self {
        self.extscr_reg().set(f);
        self
    }

    #[doc="Modify the EXTSCR register."]
    #[inline] pub fn with_extscr<F: FnOnce(Extscr) -> Extscr>(&self, f: F) -> &Self {
        self.extscr_reg().with(f);
        self
    }

}

#[doc="Power control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Low-power run"]
    #[inline] pub fn lpr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LPR != 0"]
    #[inline] pub fn test_lpr(&self) -> bool {
        self.lpr() != 0
    }

    #[doc="Sets the LPR field."]
    #[inline] pub fn set_lpr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Voltage scaling range selection"]
    #[inline] pub fn vos(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if VOS != 0"]
    #[inline] pub fn test_vos(&self) -> bool {
        self.vos() != 0
    }

    #[doc="Sets the VOS field."]
    #[inline] pub fn set_vos<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Disable backup domain write protection"]
    #[inline] pub fn dbp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DBP != 0"]
    #[inline] pub fn test_dbp(&self) -> bool {
        self.dbp() != 0
    }

    #[doc="Sets the DBP field."]
    #[inline] pub fn set_dbp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Flash power down mode during LPsSleep for CPU1"]
    #[inline] pub fn fpds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FPDS != 0"]
    #[inline] pub fn test_fpds(&self) -> bool {
        self.fpds() != 0
    }

    #[doc="Sets the FPDS field."]
    #[inline] pub fn set_fpds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Flash power down mode during LPRun for CPU1"]
    #[inline] pub fn fpdr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FPDR != 0"]
    #[inline] pub fn test_fpdr(&self) -> bool {
        self.fpdr() != 0
    }

    #[doc="Sets the FPDR field."]
    #[inline] pub fn set_fpdr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Low-power mode selection for CPU1"]
    #[inline] pub fn lpms(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LPMS != 0"]
    #[inline] pub fn test_lpms(&self) -> bool {
        self.lpms() != 0
    }

    #[doc="Sets the LPMS field."]
    #[inline] pub fn set_lpms<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
    }
}

impl ::core::fmt::Display for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lpr() != 0 { try!(write!(f, " lpr"))}
        if self.vos() != 0 { try!(write!(f, " vos=0x{:x}", self.vos()))}
        if self.dbp() != 0 { try!(write!(f, " dbp"))}
        if self.fpds() != 0 { try!(write!(f, " fpds"))}
        if self.fpdr() != 0 { try!(write!(f, " fpdr"))}
        if self.lpms() != 0 { try!(write!(f, " lpms=0x{:x}", self.lpms()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="VDDUSB USB supply valid"]
    #[inline] pub fn usv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if USV != 0"]
    #[inline] pub fn test_usv(&self) -> bool {
        self.usv() != 0
    }

    #[doc="Sets the USV field."]
    #[inline] pub fn set_usv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline] pub fn pvme3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PVME3 != 0"]
    #[inline] pub fn test_pvme3(&self) -> bool {
        self.pvme3() != 0
    }

    #[doc="Sets the PVME3 field."]
    #[inline] pub fn set_pvme3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline] pub fn pvme1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PVME1 != 0"]
    #[inline] pub fn test_pvme1(&self) -> bool {
        self.pvme1() != 0
    }

    #[doc="Sets the PVME1 field."]
    #[inline] pub fn set_pvme1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Power voltage detector level selection"]
    #[inline] pub fn pls(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if PLS != 0"]
    #[inline] pub fn test_pls(&self) -> bool {
        self.pls() != 0
    }

    #[doc="Sets the PLS field."]
    #[inline] pub fn set_pls<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Power voltage detector enable"]
    #[inline] pub fn pvde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PVDE != 0"]
    #[inline] pub fn test_pvde(&self) -> bool {
        self.pvde() != 0
    }

    #[doc="Sets the PVDE field."]
    #[inline] pub fn set_pvde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr2(other)
    }
}

impl ::core::fmt::Display for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usv() != 0 { try!(write!(f, " usv"))}
        if self.pvme3() != 0 { try!(write!(f, " pvme3"))}
        if self.pvme1() != 0 { try!(write!(f, " pvme1"))}
        if self.pls() != 0 { try!(write!(f, " pls=0x{:x}", self.pls()))}
        if self.pvde() != 0 { try!(write!(f, " pvde"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power control register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc="Enable internal wakeup line for CPU1"]
    #[inline] pub fn eiwul(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EIWUL != 0"]
    #[inline] pub fn test_eiwul(&self) -> bool {
        self.eiwul() != 0
    }

    #[doc="Sets the EIWUL field."]
    #[inline] pub fn set_eiwul<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Enable CPU2 Hold interrupt for CPU1"]
    #[inline] pub fn ec2h(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if EC2H != 0"]
    #[inline] pub fn test_ec2h(&self) -> bool {
        self.ec2h() != 0
    }

    #[doc="Sets the EC2H field."]
    #[inline] pub fn set_ec2h<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Enable end of activity interrupt for CPU1"]
    #[inline] pub fn e802a(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if E802A != 0"]
    #[inline] pub fn test_e802a(&self) -> bool {
        self.e802a() != 0
    }

    #[doc="Sets the E802A field."]
    #[inline] pub fn set_e802a<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Enable BLE end of activity interrupt for CPU1"]
    #[inline] pub fn eblea(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if EBLEA != 0"]
    #[inline] pub fn test_eblea(&self) -> bool {
        self.eblea() != 0
    }

    #[doc="Sets the EBLEA field."]
    #[inline] pub fn set_eblea<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Enable critical radio phase end of activity interrupt for CPU1"]
    #[inline] pub fn ecrpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if ECRPE != 0"]
    #[inline] pub fn test_ecrpe(&self) -> bool {
        self.ecrpe() != 0
    }

    #[doc="Sets the ECRPE field."]
    #[inline] pub fn set_ecrpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Apply pull-up and pull-down configuration"]
    #[inline] pub fn apc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if APC != 0"]
    #[inline] pub fn test_apc(&self) -> bool {
        self.apc() != 0
    }

    #[doc="Sets the APC field."]
    #[inline] pub fn set_apc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="SRAM2a retention in Standby mode"]
    #[inline] pub fn rrs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RRS != 0"]
    #[inline] pub fn test_rrs(&self) -> bool {
        self.rrs() != 0
    }

    #[doc="Sets the RRS field."]
    #[inline] pub fn set_rrs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable BORH and Step Down counverter forced in Bypass interrups for CPU1"]
    #[inline] pub fn eborhsdfb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EBORHSDFB != 0"]
    #[inline] pub fn test_eborhsdfb(&self) -> bool {
        self.eborhsdfb() != 0
    }

    #[doc="Sets the EBORHSDFB field."]
    #[inline] pub fn set_eborhsdfb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enable Wakeup pin WKUP5"]
    #[inline] pub fn ewup5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EWUP5 != 0"]
    #[inline] pub fn test_ewup5(&self) -> bool {
        self.ewup5() != 0
    }

    #[doc="Sets the EWUP5 field."]
    #[inline] pub fn set_ewup5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enable Wakeup pin WKUP4"]
    #[inline] pub fn ewup4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EWUP4 != 0"]
    #[inline] pub fn test_ewup4(&self) -> bool {
        self.ewup4() != 0
    }

    #[doc="Sets the EWUP4 field."]
    #[inline] pub fn set_ewup4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enable Wakeup pin WKUP3"]
    #[inline] pub fn ewup3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EWUP3 != 0"]
    #[inline] pub fn test_ewup3(&self) -> bool {
        self.ewup3() != 0
    }

    #[doc="Sets the EWUP3 field."]
    #[inline] pub fn set_ewup3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enable Wakeup pin WKUP2"]
    #[inline] pub fn ewup2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EWUP2 != 0"]
    #[inline] pub fn test_ewup2(&self) -> bool {
        self.ewup2() != 0
    }

    #[doc="Sets the EWUP2 field."]
    #[inline] pub fn set_ewup2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable Wakeup pin WKUP1"]
    #[inline] pub fn ewup1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EWUP1 != 0"]
    #[inline] pub fn test_ewup1(&self) -> bool {
        self.ewup1() != 0
    }

    #[doc="Sets the EWUP1 field."]
    #[inline] pub fn set_ewup1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr3(other)
    }
}

impl ::core::fmt::Display for Cr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eiwul() != 0 { try!(write!(f, " eiwul"))}
        if self.ec2h() != 0 { try!(write!(f, " ec2h"))}
        if self.e802a() != 0 { try!(write!(f, " e802a"))}
        if self.eblea() != 0 { try!(write!(f, " eblea"))}
        if self.ecrpe() != 0 { try!(write!(f, " ecrpe"))}
        if self.apc() != 0 { try!(write!(f, " apc"))}
        if self.rrs() != 0 { try!(write!(f, " rrs"))}
        if self.eborhsdfb() != 0 { try!(write!(f, " eborhsdfb"))}
        if self.ewup5() != 0 { try!(write!(f, " ewup5"))}
        if self.ewup4() != 0 { try!(write!(f, " ewup4"))}
        if self.ewup3() != 0 { try!(write!(f, " ewup3"))}
        if self.ewup2() != 0 { try!(write!(f, " ewup2"))}
        if self.ewup1() != 0 { try!(write!(f, " ewup1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power control register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr4(pub u32);
impl Cr4 {
    #[doc="BOOT CPU2 after reset or wakeup from Stop or Standby modes"]
    #[inline] pub fn c2boot(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if C2BOOT != 0"]
    #[inline] pub fn test_c2boot(&self) -> bool {
        self.c2boot() != 0
    }

    #[doc="Sets the C2BOOT field."]
    #[inline] pub fn set_c2boot<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="VBAT battery charging resistor selection"]
    #[inline] pub fn vbrs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if VBRS != 0"]
    #[inline] pub fn test_vbrs(&self) -> bool {
        self.vbrs() != 0
    }

    #[doc="Sets the VBRS field."]
    #[inline] pub fn set_vbrs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="VBAT battery charging enable"]
    #[inline] pub fn vbe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if VBE != 0"]
    #[inline] pub fn test_vbe(&self) -> bool {
        self.vbe() != 0
    }

    #[doc="Sets the VBE field."]
    #[inline] pub fn set_vbe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Wakeup pin WKUP5 polarity"]
    #[inline] pub fn wp5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WP5 != 0"]
    #[inline] pub fn test_wp5(&self) -> bool {
        self.wp5() != 0
    }

    #[doc="Sets the WP5 field."]
    #[inline] pub fn set_wp5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Wakeup pin WKUP4 polarity"]
    #[inline] pub fn wp4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WP4 != 0"]
    #[inline] pub fn test_wp4(&self) -> bool {
        self.wp4() != 0
    }

    #[doc="Sets the WP4 field."]
    #[inline] pub fn set_wp4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wakeup pin WKUP3 polarity"]
    #[inline] pub fn wp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WP3 != 0"]
    #[inline] pub fn test_wp3(&self) -> bool {
        self.wp3() != 0
    }

    #[doc="Sets the WP3 field."]
    #[inline] pub fn set_wp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Wakeup pin WKUP2 polarity"]
    #[inline] pub fn wp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WP2 != 0"]
    #[inline] pub fn test_wp2(&self) -> bool {
        self.wp2() != 0
    }

    #[doc="Sets the WP2 field."]
    #[inline] pub fn set_wp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wakeup pin WKUP1 polarity"]
    #[inline] pub fn wp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WP1 != 0"]
    #[inline] pub fn test_wp1(&self) -> bool {
        self.wp1() != 0
    }

    #[doc="Sets the WP1 field."]
    #[inline] pub fn set_wp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr4(other)
    }
}

impl ::core::fmt::Display for Cr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.c2boot() != 0 { try!(write!(f, " c2boot"))}
        if self.vbrs() != 0 { try!(write!(f, " vbrs"))}
        if self.vbe() != 0 { try!(write!(f, " vbe"))}
        if self.wp5() != 0 { try!(write!(f, " wp5"))}
        if self.wp4() != 0 { try!(write!(f, " wp4"))}
        if self.wp3() != 0 { try!(write!(f, " wp3"))}
        if self.wp2() != 0 { try!(write!(f, " wp2"))}
        if self.wp1() != 0 { try!(write!(f, " wp1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power status register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr1(pub u32);
impl Sr1 {
    #[doc="Internal Wakeup interrupt flag"]
    #[inline] pub fn wufi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if WUFI != 0"]
    #[inline] pub fn test_wufi(&self) -> bool {
        self.wufi() != 0
    }

    #[doc="Sets the WUFI field."]
    #[inline] pub fn set_wufi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="CPU2 Hold interrupt flag"]
    #[inline] pub fn c2hf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if C2HF != 0"]
    #[inline] pub fn test_c2hf(&self) -> bool {
        self.c2hf() != 0
    }

    #[doc="Sets the C2HF field."]
    #[inline] pub fn set_c2hf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="802.15.4 end of activity interrupt flag"]
    #[inline] pub fn af802(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if AF802 != 0"]
    #[inline] pub fn test_af802(&self) -> bool {
        self.af802() != 0
    }

    #[doc="Sets the AF802 field."]
    #[inline] pub fn set_af802<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="BLE end of activity interrupt flag"]
    #[inline] pub fn bleaf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if BLEAF != 0"]
    #[inline] pub fn test_bleaf(&self) -> bool {
        self.bleaf() != 0
    }

    #[doc="Sets the BLEAF field."]
    #[inline] pub fn set_bleaf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable critical radio phase end of activity interrupt flag"]
    #[inline] pub fn crpef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CRPEF != 0"]
    #[inline] pub fn test_crpef(&self) -> bool {
        self.crpef() != 0
    }

    #[doc="Sets the CRPEF field."]
    #[inline] pub fn set_crpef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="802.15.4 wakeup interrupt flag"]
    #[inline] pub fn ieee802wuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if IEEE802WUF != 0"]
    #[inline] pub fn test_ieee802wuf(&self) -> bool {
        self.ieee802wuf() != 0
    }

    #[doc="Sets the IEEE802WUF field."]
    #[inline] pub fn set_ieee802wuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BLE wakeup interrupt flag"]
    #[inline] pub fn blewuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BLEWUF != 0"]
    #[inline] pub fn test_blewuf(&self) -> bool {
        self.blewuf() != 0
    }

    #[doc="Sets the BLEWUF field."]
    #[inline] pub fn set_blewuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BORH interrupt flag"]
    #[inline] pub fn borhf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BORHF != 0"]
    #[inline] pub fn test_borhf(&self) -> bool {
        self.borhf() != 0
    }

    #[doc="Sets the BORHF field."]
    #[inline] pub fn set_borhf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Step Down converter forced in Bypass interrupt flag"]
    #[inline] pub fn sdfbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SDFBF != 0"]
    #[inline] pub fn test_sdfbf(&self) -> bool {
        self.sdfbf() != 0
    }

    #[doc="Sets the SDFBF field."]
    #[inline] pub fn set_sdfbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Wakeup flag 5"]
    #[inline] pub fn cwuf5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CWUF5 != 0"]
    #[inline] pub fn test_cwuf5(&self) -> bool {
        self.cwuf5() != 0
    }

    #[doc="Sets the CWUF5 field."]
    #[inline] pub fn set_cwuf5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Wakeup flag 4"]
    #[inline] pub fn cwuf4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CWUF4 != 0"]
    #[inline] pub fn test_cwuf4(&self) -> bool {
        self.cwuf4() != 0
    }

    #[doc="Sets the CWUF4 field."]
    #[inline] pub fn set_cwuf4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wakeup flag 3"]
    #[inline] pub fn cwuf3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CWUF3 != 0"]
    #[inline] pub fn test_cwuf3(&self) -> bool {
        self.cwuf3() != 0
    }

    #[doc="Sets the CWUF3 field."]
    #[inline] pub fn set_cwuf3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Wakeup flag 2"]
    #[inline] pub fn cwuf2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CWUF2 != 0"]
    #[inline] pub fn test_cwuf2(&self) -> bool {
        self.cwuf2() != 0
    }

    #[doc="Sets the CWUF2 field."]
    #[inline] pub fn set_cwuf2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wakeup flag 1"]
    #[inline] pub fn cwuf1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CWUF1 != 0"]
    #[inline] pub fn test_cwuf1(&self) -> bool {
        self.cwuf1() != 0
    }

    #[doc="Sets the CWUF1 field."]
    #[inline] pub fn set_cwuf1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sr1(other)
    }
}

impl ::core::fmt::Display for Sr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wufi() != 0 { try!(write!(f, " wufi"))}
        if self.c2hf() != 0 { try!(write!(f, " c2hf"))}
        if self.af802() != 0 { try!(write!(f, " af802"))}
        if self.bleaf() != 0 { try!(write!(f, " bleaf"))}
        if self.crpef() != 0 { try!(write!(f, " crpef"))}
        if self.ieee802wuf() != 0 { try!(write!(f, " ieee802wuf"))}
        if self.blewuf() != 0 { try!(write!(f, " blewuf"))}
        if self.borhf() != 0 { try!(write!(f, " borhf"))}
        if self.sdfbf() != 0 { try!(write!(f, " sdfbf"))}
        if self.cwuf5() != 0 { try!(write!(f, " cwuf5"))}
        if self.cwuf4() != 0 { try!(write!(f, " cwuf4"))}
        if self.cwuf3() != 0 { try!(write!(f, " cwuf3"))}
        if self.cwuf2() != 0 { try!(write!(f, " cwuf2"))}
        if self.cwuf1() != 0 { try!(write!(f, " cwuf1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power status register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr2(pub u32);
impl Sr2 {
    #[doc="Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline] pub fn pvmo3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PVMO3 != 0"]
    #[inline] pub fn test_pvmo3(&self) -> bool {
        self.pvmo3() != 0
    }

    #[doc="Sets the PVMO3 field."]
    #[inline] pub fn set_pvmo3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
    #[inline] pub fn pvmo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PVMO1 != 0"]
    #[inline] pub fn test_pvmo1(&self) -> bool {
        self.pvmo1() != 0
    }

    #[doc="Sets the PVMO1 field."]
    #[inline] pub fn set_pvmo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Power voltage detector output"]
    #[inline] pub fn pvdo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PVDO != 0"]
    #[inline] pub fn test_pvdo(&self) -> bool {
        self.pvdo() != 0
    }

    #[doc="Sets the PVDO field."]
    #[inline] pub fn set_pvdo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Voltage scaling flag"]
    #[inline] pub fn vosf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VOSF != 0"]
    #[inline] pub fn test_vosf(&self) -> bool {
        self.vosf() != 0
    }

    #[doc="Sets the VOSF field."]
    #[inline] pub fn set_vosf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Low-power regulator flag"]
    #[inline] pub fn reglpf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if REGLPF != 0"]
    #[inline] pub fn test_reglpf(&self) -> bool {
        self.reglpf() != 0
    }

    #[doc="Sets the REGLPF field."]
    #[inline] pub fn set_reglpf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Low-power regulator started"]
    #[inline] pub fn reglps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if REGLPS != 0"]
    #[inline] pub fn test_reglps(&self) -> bool {
        self.reglps() != 0
    }

    #[doc="Sets the REGLPS field."]
    #[inline] pub fn set_reglps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Step Down converter SMPS mode flag"]
    #[inline] pub fn sdsmpsf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SDSMPSF != 0"]
    #[inline] pub fn test_sdsmpsf(&self) -> bool {
        self.sdsmpsf() != 0
    }

    #[doc="Sets the SDSMPSF field."]
    #[inline] pub fn set_sdsmpsf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Step Down converter Bypass mode flag"]
    #[inline] pub fn sdbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SDBF != 0"]
    #[inline] pub fn test_sdbf(&self) -> bool {
        self.sdbf() != 0
    }

    #[doc="Sets the SDBF field."]
    #[inline] pub fn set_sdbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sr2(other)
    }
}

impl ::core::fmt::Display for Sr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pvmo3() != 0 { try!(write!(f, " pvmo3"))}
        if self.pvmo1() != 0 { try!(write!(f, " pvmo1"))}
        if self.pvdo() != 0 { try!(write!(f, " pvdo"))}
        if self.vosf() != 0 { try!(write!(f, " vosf"))}
        if self.reglpf() != 0 { try!(write!(f, " reglpf"))}
        if self.reglps() != 0 { try!(write!(f, " reglps"))}
        if self.sdsmpsf() != 0 { try!(write!(f, " sdsmpsf"))}
        if self.sdbf() != 0 { try!(write!(f, " sdbf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power status clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc="Clear CPU2 Hold interrupt flag"]
    #[inline] pub fn cc2hf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CC2HF != 0"]
    #[inline] pub fn test_cc2hf(&self) -> bool {
        self.cc2hf() != 0
    }

    #[doc="Sets the CC2HF field."]
    #[inline] pub fn set_cc2hf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Clear 802.15.4 end of activity interrupt flag"]
    #[inline] pub fn c802af(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if C802AF != 0"]
    #[inline] pub fn test_c802af(&self) -> bool {
        self.c802af() != 0
    }

    #[doc="Sets the C802AF field."]
    #[inline] pub fn set_c802af<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Clear BLE end of activity interrupt flag"]
    #[inline] pub fn cbleaf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CBLEAF != 0"]
    #[inline] pub fn test_cbleaf(&self) -> bool {
        self.cbleaf() != 0
    }

    #[doc="Sets the CBLEAF field."]
    #[inline] pub fn set_cbleaf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clear critical radio phase end of activity interrupt flag"]
    #[inline] pub fn ccrpef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CCRPEF != 0"]
    #[inline] pub fn test_ccrpef(&self) -> bool {
        self.ccrpef() != 0
    }

    #[doc="Sets the CCRPEF field."]
    #[inline] pub fn set_ccrpef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clear 802.15.4 wakeup interrupt flag"]
    #[inline] pub fn c802wuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if C802WUF != 0"]
    #[inline] pub fn test_c802wuf(&self) -> bool {
        self.c802wuf() != 0
    }

    #[doc="Sets the C802WUF field."]
    #[inline] pub fn set_c802wuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clear BLE wakeup interrupt flag"]
    #[inline] pub fn cblewuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CBLEWUF != 0"]
    #[inline] pub fn test_cblewuf(&self) -> bool {
        self.cblewuf() != 0
    }

    #[doc="Sets the CBLEWUF field."]
    #[inline] pub fn set_cblewuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clear BORH interrupt flag"]
    #[inline] pub fn cborhf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CBORHF != 0"]
    #[inline] pub fn test_cborhf(&self) -> bool {
        self.cborhf() != 0
    }

    #[doc="Sets the CBORHF field."]
    #[inline] pub fn set_cborhf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear SMPS Step Down converter forced in Bypass interrupt flag"]
    #[inline] pub fn csmpsfbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CSMPSFBF != 0"]
    #[inline] pub fn test_csmpsfbf(&self) -> bool {
        self.csmpsfbf() != 0
    }

    #[doc="Sets the CSMPSFBF field."]
    #[inline] pub fn set_csmpsfbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Clear wakeup flag 5"]
    #[inline] pub fn cwuf5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CWUF5 != 0"]
    #[inline] pub fn test_cwuf5(&self) -> bool {
        self.cwuf5() != 0
    }

    #[doc="Sets the CWUF5 field."]
    #[inline] pub fn set_cwuf5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear wakeup flag 4"]
    #[inline] pub fn cwuf4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CWUF4 != 0"]
    #[inline] pub fn test_cwuf4(&self) -> bool {
        self.cwuf4() != 0
    }

    #[doc="Sets the CWUF4 field."]
    #[inline] pub fn set_cwuf4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear wakeup flag 3"]
    #[inline] pub fn cwuf3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CWUF3 != 0"]
    #[inline] pub fn test_cwuf3(&self) -> bool {
        self.cwuf3() != 0
    }

    #[doc="Sets the CWUF3 field."]
    #[inline] pub fn set_cwuf3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear wakeup flag 2"]
    #[inline] pub fn cwuf2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CWUF2 != 0"]
    #[inline] pub fn test_cwuf2(&self) -> bool {
        self.cwuf2() != 0
    }

    #[doc="Sets the CWUF2 field."]
    #[inline] pub fn set_cwuf2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear wakeup flag 1"]
    #[inline] pub fn cwuf1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CWUF1 != 0"]
    #[inline] pub fn test_cwuf1(&self) -> bool {
        self.cwuf1() != 0
    }

    #[doc="Sets the CWUF1 field."]
    #[inline] pub fn set_cwuf1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scr {
    #[inline]
    fn from(other: u32) -> Self {
         Scr(other)
    }
}

impl ::core::fmt::Display for Scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cc2hf() != 0 { try!(write!(f, " cc2hf"))}
        if self.c802af() != 0 { try!(write!(f, " c802af"))}
        if self.cbleaf() != 0 { try!(write!(f, " cbleaf"))}
        if self.ccrpef() != 0 { try!(write!(f, " ccrpef"))}
        if self.c802wuf() != 0 { try!(write!(f, " c802wuf"))}
        if self.cblewuf() != 0 { try!(write!(f, " cblewuf"))}
        if self.cborhf() != 0 { try!(write!(f, " cborhf"))}
        if self.csmpsfbf() != 0 { try!(write!(f, " csmpsfbf"))}
        if self.cwuf5() != 0 { try!(write!(f, " cwuf5"))}
        if self.cwuf4() != 0 { try!(write!(f, " cwuf4"))}
        if self.cwuf3() != 0 { try!(write!(f, " cwuf3"))}
        if self.cwuf2() != 0 { try!(write!(f, " cwuf2"))}
        if self.cwuf1() != 0 { try!(write!(f, " cwuf1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power control register 5"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr5(pub u32);
impl Cr5 {
    #[doc="Enable Step Down converter SMPS mode enabled"]
    #[inline] pub fn sdeb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SDEB != 0"]
    #[inline] pub fn test_sdeb(&self) -> bool {
        self.sdeb() != 0
    }

    #[doc="Sets the SDEB field."]
    #[inline] pub fn set_sdeb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Enable Step Down converter Bypass mode enabled"]
    #[inline] pub fn sdben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SDBEN != 0"]
    #[inline] pub fn test_sdben(&self) -> bool {
        self.sdben() != 0
    }

    #[doc="Sets the SDBEN field."]
    #[inline] pub fn set_sdben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="VOS configuration selection (non user)"]
    #[inline] pub fn smpscfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SMPSCFG != 0"]
    #[inline] pub fn test_smpscfg(&self) -> bool {
        self.smpscfg() != 0
    }

    #[doc="Sets the SMPSCFG field."]
    #[inline] pub fn set_smpscfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BORH configuration selection"]
    #[inline] pub fn borhc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BORHC != 0"]
    #[inline] pub fn test_borhc(&self) -> bool {
        self.borhc() != 0
    }

    #[doc="Sets the BORHC field."]
    #[inline] pub fn set_borhc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Step Down converter supplt startup current selection"]
    #[inline] pub fn sdsc(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if SDSC != 0"]
    #[inline] pub fn test_sdsc(&self) -> bool {
        self.sdsc() != 0
    }

    #[doc="Sets the SDSC field."]
    #[inline] pub fn set_sdsc<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Step Down converter voltage output scaling"]
    #[inline] pub fn sdvos(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SDVOS != 0"]
    #[inline] pub fn test_sdvos(&self) -> bool {
        self.sdvos() != 0
    }

    #[doc="Sets the SDVOS field."]
    #[inline] pub fn set_sdvos<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr5 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr5(other)
    }
}

impl ::core::fmt::Display for Cr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sdeb() != 0 { try!(write!(f, " sdeb"))}
        if self.sdben() != 0 { try!(write!(f, " sdben"))}
        if self.smpscfg() != 0 { try!(write!(f, " smpscfg"))}
        if self.borhc() != 0 { try!(write!(f, " borhc"))}
        if self.sdsc() != 0 { try!(write!(f, " sdsc=0x{:x}", self.sdsc()))}
        if self.sdvos() != 0 { try!(write!(f, " sdvos=0x{:x}", self.sdvos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port A pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucra(pub u32);
impl Pucra {
    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pucra {
    #[inline]
    fn from(other: u32) -> Self {
         Pucra(other)
    }
}

impl ::core::fmt::Display for Pucra {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pucra {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pu15() != 0 { try!(write!(f, " pu15"))}
        if self.pu13() != 0 { try!(write!(f, " pu13"))}
        if self.pu12() != 0 { try!(write!(f, " pu12"))}
        if self.pu11() != 0 { try!(write!(f, " pu11"))}
        if self.pu10() != 0 { try!(write!(f, " pu10"))}
        if self.pu9() != 0 { try!(write!(f, " pu9"))}
        if self.pu8() != 0 { try!(write!(f, " pu8"))}
        if self.pu7() != 0 { try!(write!(f, " pu7"))}
        if self.pu6() != 0 { try!(write!(f, " pu6"))}
        if self.pu5() != 0 { try!(write!(f, " pu5"))}
        if self.pu4() != 0 { try!(write!(f, " pu4"))}
        if self.pu3() != 0 { try!(write!(f, " pu3"))}
        if self.pu2() != 0 { try!(write!(f, " pu2"))}
        if self.pu1() != 0 { try!(write!(f, " pu1"))}
        if self.pu0() != 0 { try!(write!(f, " pu0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port A pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcra(pub u32);
impl Pdcra {
    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdcra {
    #[inline]
    fn from(other: u32) -> Self {
         Pdcra(other)
    }
}

impl ::core::fmt::Display for Pdcra {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdcra {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pd14() != 0 { try!(write!(f, " pd14"))}
        if self.pd12() != 0 { try!(write!(f, " pd12"))}
        if self.pd11() != 0 { try!(write!(f, " pd11"))}
        if self.pd10() != 0 { try!(write!(f, " pd10"))}
        if self.pd9() != 0 { try!(write!(f, " pd9"))}
        if self.pd8() != 0 { try!(write!(f, " pd8"))}
        if self.pd7() != 0 { try!(write!(f, " pd7"))}
        if self.pd6() != 0 { try!(write!(f, " pd6"))}
        if self.pd5() != 0 { try!(write!(f, " pd5"))}
        if self.pd4() != 0 { try!(write!(f, " pd4"))}
        if self.pd3() != 0 { try!(write!(f, " pd3"))}
        if self.pd2() != 0 { try!(write!(f, " pd2"))}
        if self.pd1() != 0 { try!(write!(f, " pd1"))}
        if self.pd0() != 0 { try!(write!(f, " pd0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port B pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucrb(pub u32);
impl Pucrb {
    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pucrb {
    #[inline]
    fn from(other: u32) -> Self {
         Pucrb(other)
    }
}

impl ::core::fmt::Display for Pucrb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pucrb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pu15() != 0 { try!(write!(f, " pu15"))}
        if self.pu14() != 0 { try!(write!(f, " pu14"))}
        if self.pu13() != 0 { try!(write!(f, " pu13"))}
        if self.pu12() != 0 { try!(write!(f, " pu12"))}
        if self.pu11() != 0 { try!(write!(f, " pu11"))}
        if self.pu10() != 0 { try!(write!(f, " pu10"))}
        if self.pu9() != 0 { try!(write!(f, " pu9"))}
        if self.pu8() != 0 { try!(write!(f, " pu8"))}
        if self.pu7() != 0 { try!(write!(f, " pu7"))}
        if self.pu6() != 0 { try!(write!(f, " pu6"))}
        if self.pu5() != 0 { try!(write!(f, " pu5"))}
        if self.pu4() != 0 { try!(write!(f, " pu4"))}
        if self.pu3() != 0 { try!(write!(f, " pu3"))}
        if self.pu2() != 0 { try!(write!(f, " pu2"))}
        if self.pu1() != 0 { try!(write!(f, " pu1"))}
        if self.pu0() != 0 { try!(write!(f, " pu0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port B pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcrb(pub u32);
impl Pdcrb {
    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdcrb {
    #[inline]
    fn from(other: u32) -> Self {
         Pdcrb(other)
    }
}

impl ::core::fmt::Display for Pdcrb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdcrb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pd15() != 0 { try!(write!(f, " pd15"))}
        if self.pd14() != 0 { try!(write!(f, " pd14"))}
        if self.pd13() != 0 { try!(write!(f, " pd13"))}
        if self.pd12() != 0 { try!(write!(f, " pd12"))}
        if self.pd11() != 0 { try!(write!(f, " pd11"))}
        if self.pd10() != 0 { try!(write!(f, " pd10"))}
        if self.pd9() != 0 { try!(write!(f, " pd9"))}
        if self.pd8() != 0 { try!(write!(f, " pd8"))}
        if self.pd7() != 0 { try!(write!(f, " pd7"))}
        if self.pd6() != 0 { try!(write!(f, " pd6"))}
        if self.pd5() != 0 { try!(write!(f, " pd5"))}
        if self.pd3() != 0 { try!(write!(f, " pd3"))}
        if self.pd2() != 0 { try!(write!(f, " pd2"))}
        if self.pd1() != 0 { try!(write!(f, " pd1"))}
        if self.pd0() != 0 { try!(write!(f, " pd0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port C pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucrc(pub u32);
impl Pucrc {
    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pucrc {
    #[inline]
    fn from(other: u32) -> Self {
         Pucrc(other)
    }
}

impl ::core::fmt::Display for Pucrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pucrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pu15() != 0 { try!(write!(f, " pu15"))}
        if self.pu14() != 0 { try!(write!(f, " pu14"))}
        if self.pu13() != 0 { try!(write!(f, " pu13"))}
        if self.pu12() != 0 { try!(write!(f, " pu12"))}
        if self.pu11() != 0 { try!(write!(f, " pu11"))}
        if self.pu10() != 0 { try!(write!(f, " pu10"))}
        if self.pu9() != 0 { try!(write!(f, " pu9"))}
        if self.pu8() != 0 { try!(write!(f, " pu8"))}
        if self.pu7() != 0 { try!(write!(f, " pu7"))}
        if self.pu6() != 0 { try!(write!(f, " pu6"))}
        if self.pu5() != 0 { try!(write!(f, " pu5"))}
        if self.pu4() != 0 { try!(write!(f, " pu4"))}
        if self.pu3() != 0 { try!(write!(f, " pu3"))}
        if self.pu2() != 0 { try!(write!(f, " pu2"))}
        if self.pu1() != 0 { try!(write!(f, " pu1"))}
        if self.pu0() != 0 { try!(write!(f, " pu0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port C pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcrc(pub u32);
impl Pdcrc {
    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdcrc {
    #[inline]
    fn from(other: u32) -> Self {
         Pdcrc(other)
    }
}

impl ::core::fmt::Display for Pdcrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdcrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pd15() != 0 { try!(write!(f, " pd15"))}
        if self.pd14() != 0 { try!(write!(f, " pd14"))}
        if self.pd13() != 0 { try!(write!(f, " pd13"))}
        if self.pd12() != 0 { try!(write!(f, " pd12"))}
        if self.pd11() != 0 { try!(write!(f, " pd11"))}
        if self.pd10() != 0 { try!(write!(f, " pd10"))}
        if self.pd9() != 0 { try!(write!(f, " pd9"))}
        if self.pd8() != 0 { try!(write!(f, " pd8"))}
        if self.pd7() != 0 { try!(write!(f, " pd7"))}
        if self.pd6() != 0 { try!(write!(f, " pd6"))}
        if self.pd5() != 0 { try!(write!(f, " pd5"))}
        if self.pd4() != 0 { try!(write!(f, " pd4"))}
        if self.pd3() != 0 { try!(write!(f, " pd3"))}
        if self.pd2() != 0 { try!(write!(f, " pd2"))}
        if self.pd1() != 0 { try!(write!(f, " pd1"))}
        if self.pd0() != 0 { try!(write!(f, " pd0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port D pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucrd(pub u32);
impl Pucrd {
    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pucrd {
    #[inline]
    fn from(other: u32) -> Self {
         Pucrd(other)
    }
}

impl ::core::fmt::Display for Pucrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pucrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pu15() != 0 { try!(write!(f, " pu15"))}
        if self.pu14() != 0 { try!(write!(f, " pu14"))}
        if self.pu13() != 0 { try!(write!(f, " pu13"))}
        if self.pu12() != 0 { try!(write!(f, " pu12"))}
        if self.pu11() != 0 { try!(write!(f, " pu11"))}
        if self.pu10() != 0 { try!(write!(f, " pu10"))}
        if self.pu9() != 0 { try!(write!(f, " pu9"))}
        if self.pu8() != 0 { try!(write!(f, " pu8"))}
        if self.pu7() != 0 { try!(write!(f, " pu7"))}
        if self.pu6() != 0 { try!(write!(f, " pu6"))}
        if self.pu5() != 0 { try!(write!(f, " pu5"))}
        if self.pu4() != 0 { try!(write!(f, " pu4"))}
        if self.pu3() != 0 { try!(write!(f, " pu3"))}
        if self.pu2() != 0 { try!(write!(f, " pu2"))}
        if self.pu1() != 0 { try!(write!(f, " pu1"))}
        if self.pu0() != 0 { try!(write!(f, " pu0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port D pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcrd(pub u32);
impl Pdcrd {
    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdcrd {
    #[inline]
    fn from(other: u32) -> Self {
         Pdcrd(other)
    }
}

impl ::core::fmt::Display for Pdcrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdcrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pd15() != 0 { try!(write!(f, " pd15"))}
        if self.pd14() != 0 { try!(write!(f, " pd14"))}
        if self.pd13() != 0 { try!(write!(f, " pd13"))}
        if self.pd12() != 0 { try!(write!(f, " pd12"))}
        if self.pd11() != 0 { try!(write!(f, " pd11"))}
        if self.pd10() != 0 { try!(write!(f, " pd10"))}
        if self.pd9() != 0 { try!(write!(f, " pd9"))}
        if self.pd8() != 0 { try!(write!(f, " pd8"))}
        if self.pd7() != 0 { try!(write!(f, " pd7"))}
        if self.pd6() != 0 { try!(write!(f, " pd6"))}
        if self.pd5() != 0 { try!(write!(f, " pd5"))}
        if self.pd4() != 0 { try!(write!(f, " pd4"))}
        if self.pd3() != 0 { try!(write!(f, " pd3"))}
        if self.pd2() != 0 { try!(write!(f, " pd2"))}
        if self.pd1() != 0 { try!(write!(f, " pd1"))}
        if self.pd0() != 0 { try!(write!(f, " pd0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port E pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucre(pub u32);
impl Pucre {
    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pucre {
    #[inline]
    fn from(other: u32) -> Self {
         Pucre(other)
    }
}

impl ::core::fmt::Display for Pucre {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pucre {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pu4() != 0 { try!(write!(f, " pu4"))}
        if self.pu3() != 0 { try!(write!(f, " pu3"))}
        if self.pu2() != 0 { try!(write!(f, " pu2"))}
        if self.pu1() != 0 { try!(write!(f, " pu1"))}
        if self.pu0() != 0 { try!(write!(f, " pu0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port E pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcre(pub u32);
impl Pdcre {
    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdcre {
    #[inline]
    fn from(other: u32) -> Self {
         Pdcre(other)
    }
}

impl ::core::fmt::Display for Pdcre {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdcre {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pd4() != 0 { try!(write!(f, " pd4"))}
        if self.pd3() != 0 { try!(write!(f, " pd3"))}
        if self.pd2() != 0 { try!(write!(f, " pd2"))}
        if self.pd1() != 0 { try!(write!(f, " pd1"))}
        if self.pd0() != 0 { try!(write!(f, " pd0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port H pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucrh(pub u32);
impl Pucrh {
    #[doc="Port H pull-up bit y (y=0..1)"]
    #[inline] pub fn pu3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port H pull-up bit y (y=0..1)"]
    #[inline] pub fn pu1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port H pull-up bit y (y=0..1)"]
    #[inline] pub fn pu0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pucrh {
    #[inline]
    fn from(other: u32) -> Self {
         Pucrh(other)
    }
}

impl ::core::fmt::Display for Pucrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pucrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pu3() != 0 { try!(write!(f, " pu3"))}
        if self.pu1() != 0 { try!(write!(f, " pu1"))}
        if self.pu0() != 0 { try!(write!(f, " pu0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port H pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcrh(pub u32);
impl Pdcrh {
    #[doc="Port H pull-down bit y (y=0..1)"]
    #[inline] pub fn pd3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port H pull-down bit y (y=0..1)"]
    #[inline] pub fn pd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port H pull-down bit y (y=0..1)"]
    #[inline] pub fn pd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdcrh {
    #[inline]
    fn from(other: u32) -> Self {
         Pdcrh(other)
    }
}

impl ::core::fmt::Display for Pdcrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdcrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pd3() != 0 { try!(write!(f, " pd3"))}
        if self.pd1() != 0 { try!(write!(f, " pd1"))}
        if self.pd0() != 0 { try!(write!(f, " pd0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 Power control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2cr1(pub u32);
impl C2cr1 {
    #[doc="802.15.4 external wakeup signal"]
    #[inline] pub fn ieee802ewkup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if IEEE802EWKUP != 0"]
    #[inline] pub fn test_ieee802ewkup(&self) -> bool {
        self.ieee802ewkup() != 0
    }

    #[doc="Sets the IEEE802EWKUP field."]
    #[inline] pub fn set_ieee802ewkup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="BLE external wakeup signal"]
    #[inline] pub fn bleewkup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if BLEEWKUP != 0"]
    #[inline] pub fn test_bleewkup(&self) -> bool {
        self.bleewkup() != 0
    }

    #[doc="Sets the BLEEWKUP field."]
    #[inline] pub fn set_bleewkup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Flash power down mode during LPSleep for CPU2"]
    #[inline] pub fn fpds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FPDS != 0"]
    #[inline] pub fn test_fpds(&self) -> bool {
        self.fpds() != 0
    }

    #[doc="Sets the FPDS field."]
    #[inline] pub fn set_fpds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Flash power down mode during LPRun for CPU2"]
    #[inline] pub fn fpdr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FPDR != 0"]
    #[inline] pub fn test_fpdr(&self) -> bool {
        self.fpdr() != 0
    }

    #[doc="Sets the FPDR field."]
    #[inline] pub fn set_fpdr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Low-power mode selection for CPU2"]
    #[inline] pub fn lpms(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LPMS != 0"]
    #[inline] pub fn test_lpms(&self) -> bool {
        self.lpms() != 0
    }

    #[doc="Sets the LPMS field."]
    #[inline] pub fn set_lpms<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         C2cr1(other)
    }
}

impl ::core::fmt::Display for C2cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ieee802ewkup() != 0 { try!(write!(f, " ieee802ewkup"))}
        if self.bleewkup() != 0 { try!(write!(f, " bleewkup"))}
        if self.fpds() != 0 { try!(write!(f, " fpds"))}
        if self.fpdr() != 0 { try!(write!(f, " fpdr"))}
        if self.lpms() != 0 { try!(write!(f, " lpms=0x{:x}", self.lpms()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 Power control register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2cr3(pub u32);
impl C2cr3 {
    #[doc="Enable internal wakeup line for CPU2"]
    #[inline] pub fn eiwul(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EIWUL != 0"]
    #[inline] pub fn test_eiwul(&self) -> bool {
        self.eiwul() != 0
    }

    #[doc="Sets the EIWUL field."]
    #[inline] pub fn set_eiwul<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Apply pull-up and pull-down configuration for CPU2"]
    #[inline] pub fn apc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if APC != 0"]
    #[inline] pub fn test_apc(&self) -> bool {
        self.apc() != 0
    }

    #[doc="Sets the APC field."]
    #[inline] pub fn set_apc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable 802.15.4 host wakeup interrupt for CPU2"]
    #[inline] pub fn e802wup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if E802WUP != 0"]
    #[inline] pub fn test_e802wup(&self) -> bool {
        self.e802wup() != 0
    }

    #[doc="Sets the E802WUP field."]
    #[inline] pub fn set_e802wup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Enable BLE host wakeup interrupt for CPU2"]
    #[inline] pub fn eblewup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EBLEWUP != 0"]
    #[inline] pub fn test_eblewup(&self) -> bool {
        self.eblewup() != 0
    }

    #[doc="Sets the EBLEWUP field."]
    #[inline] pub fn set_eblewup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable Wakeup pin WKUP5 for CPU2"]
    #[inline] pub fn ewup5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EWUP5 != 0"]
    #[inline] pub fn test_ewup5(&self) -> bool {
        self.ewup5() != 0
    }

    #[doc="Sets the EWUP5 field."]
    #[inline] pub fn set_ewup5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enable Wakeup pin WKUP4 for CPU2"]
    #[inline] pub fn ewup4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EWUP4 != 0"]
    #[inline] pub fn test_ewup4(&self) -> bool {
        self.ewup4() != 0
    }

    #[doc="Sets the EWUP4 field."]
    #[inline] pub fn set_ewup4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enable Wakeup pin WKUP3 for CPU2"]
    #[inline] pub fn ewup3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EWUP3 != 0"]
    #[inline] pub fn test_ewup3(&self) -> bool {
        self.ewup3() != 0
    }

    #[doc="Sets the EWUP3 field."]
    #[inline] pub fn set_ewup3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enable Wakeup pin WKUP2 for CPU2"]
    #[inline] pub fn ewup2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EWUP2 != 0"]
    #[inline] pub fn test_ewup2(&self) -> bool {
        self.ewup2() != 0
    }

    #[doc="Sets the EWUP2 field."]
    #[inline] pub fn set_ewup2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable Wakeup pin WKUP1 for CPU2"]
    #[inline] pub fn ewup1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EWUP1 != 0"]
    #[inline] pub fn test_ewup1(&self) -> bool {
        self.ewup1() != 0
    }

    #[doc="Sets the EWUP1 field."]
    #[inline] pub fn set_ewup1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2cr3 {
    #[inline]
    fn from(other: u32) -> Self {
         C2cr3(other)
    }
}

impl ::core::fmt::Display for C2cr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2cr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eiwul() != 0 { try!(write!(f, " eiwul"))}
        if self.apc() != 0 { try!(write!(f, " apc"))}
        if self.e802wup() != 0 { try!(write!(f, " e802wup"))}
        if self.eblewup() != 0 { try!(write!(f, " eblewup"))}
        if self.ewup5() != 0 { try!(write!(f, " ewup5"))}
        if self.ewup4() != 0 { try!(write!(f, " ewup4"))}
        if self.ewup3() != 0 { try!(write!(f, " ewup3"))}
        if self.ewup2() != 0 { try!(write!(f, " ewup2"))}
        if self.ewup1() != 0 { try!(write!(f, " ewup1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power status clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extscr(pub u32);
impl Extscr {
    #[doc="CPU2 deepsleep mode"]
    #[inline] pub fn c2ds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if C2DS != 0"]
    #[inline] pub fn test_c2ds(&self) -> bool {
        self.c2ds() != 0
    }

    #[doc="Sets the C2DS field."]
    #[inline] pub fn set_c2ds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="CPU1 deepsleep mode"]
    #[inline] pub fn c1ds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if C1DS != 0"]
    #[inline] pub fn test_c1ds(&self) -> bool {
        self.c1ds() != 0
    }

    #[doc="Sets the C1DS field."]
    #[inline] pub fn set_c1ds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Critical Radio system phase"]
    #[inline] pub fn crpf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CRPF != 0"]
    #[inline] pub fn test_crpf(&self) -> bool {
        self.crpf() != 0
    }

    #[doc="Sets the CRPF field."]
    #[inline] pub fn set_crpf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="System Stop flag for CPU2"]
    #[inline] pub fn c2stopf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if C2STOPF != 0"]
    #[inline] pub fn test_c2stopf(&self) -> bool {
        self.c2stopf() != 0
    }

    #[doc="Sets the C2STOPF field."]
    #[inline] pub fn set_c2stopf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="System Standby flag for CPU2"]
    #[inline] pub fn c2sbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if C2SBF != 0"]
    #[inline] pub fn test_c2sbf(&self) -> bool {
        self.c2sbf() != 0
    }

    #[doc="Sets the C2SBF field."]
    #[inline] pub fn set_c2sbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="System Stop flag for CPU1"]
    #[inline] pub fn c1stopf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if C1STOPF != 0"]
    #[inline] pub fn test_c1stopf(&self) -> bool {
        self.c1stopf() != 0
    }

    #[doc="Sets the C1STOPF field."]
    #[inline] pub fn set_c1stopf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="System Standby flag for CPU1"]
    #[inline] pub fn c1sbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if C1SBF != 0"]
    #[inline] pub fn test_c1sbf(&self) -> bool {
        self.c1sbf() != 0
    }

    #[doc="Sets the C1SBF field."]
    #[inline] pub fn set_c1sbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear Critical Radio system phase"]
    #[inline] pub fn ccrpf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CCRPF != 0"]
    #[inline] pub fn test_ccrpf(&self) -> bool {
        self.ccrpf() != 0
    }

    #[doc="Sets the CCRPF field."]
    #[inline] pub fn set_ccrpf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear CPU2 Stop Standby flags"]
    #[inline] pub fn c2cssf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if C2CSSF != 0"]
    #[inline] pub fn test_c2cssf(&self) -> bool {
        self.c2cssf() != 0
    }

    #[doc="Sets the C2CSSF field."]
    #[inline] pub fn set_c2cssf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear CPU1 Stop Standby flags"]
    #[inline] pub fn c1cssf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if C1CSSF != 0"]
    #[inline] pub fn test_c1cssf(&self) -> bool {
        self.c1cssf() != 0
    }

    #[doc="Sets the C1CSSF field."]
    #[inline] pub fn set_c1cssf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Extscr {
    #[inline]
    fn from(other: u32) -> Self {
         Extscr(other)
    }
}

impl ::core::fmt::Display for Extscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.c2ds() != 0 { try!(write!(f, " c2ds"))}
        if self.c1ds() != 0 { try!(write!(f, " c1ds"))}
        if self.crpf() != 0 { try!(write!(f, " crpf"))}
        if self.c2stopf() != 0 { try!(write!(f, " c2stopf"))}
        if self.c2sbf() != 0 { try!(write!(f, " c2sbf"))}
        if self.c1stopf() != 0 { try!(write!(f, " c1stopf"))}
        if self.c1sbf() != 0 { try!(write!(f, " c1sbf"))}
        if self.ccrpf() != 0 { try!(write!(f, " ccrpf"))}
        if self.c2cssf() != 0 { try!(write!(f, " c2cssf"))}
        if self.c1cssf() != 0 { try!(write!(f, " c1cssf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

