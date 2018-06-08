
::bobbin_mcu::periph!( , Fsmc, _PERIPH, FsmcPeriph, _OWNED, _REF_COUNT, 0x00000000, 0x00, 0x08);


#[doc="Flexible memory controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FsmcPeriph(pub usize);
impl FsmcPeriph {
    #[doc="Get the BCR1 Register."]
    #[inline] pub fn bcr1_reg(&self) -> ::bobbin_mcu::register::Register<Bcr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bcr1, 0x0)
    }

    #[doc="Get the *mut pointer for the BCR1 register."]
    #[inline] pub fn bcr1_mut(&self) -> *mut Bcr1 { 
        self.bcr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the BCR1 register."]
    #[inline] pub fn bcr1_ptr(&self) -> *const Bcr1 { 
        self.bcr1_reg().ptr()
    }

    #[doc="Read the BCR1 register."]
    #[inline] pub fn bcr1(&self) -> Bcr1 { 
        self.bcr1_reg().read()
    }

    #[doc="Write the BCR1 register."]
    #[inline] pub fn write_bcr1(&self, value: Bcr1) -> &Self { 
        self.bcr1_reg().write(value);
        self
    }

    #[doc="Set the BCR1 register."]
    #[inline] pub fn set_bcr1<F: FnOnce(Bcr1) -> Bcr1>(&self, f: F) -> &Self {
        self.bcr1_reg().set(f);
        self
    }

    #[doc="Modify the BCR1 register."]
    #[inline] pub fn with_bcr1<F: FnOnce(Bcr1) -> Bcr1>(&self, f: F) -> &Self {
        self.bcr1_reg().with(f);
        self
    }

    #[doc="Get the BTR1 Register."]
    #[inline] pub fn btr1_reg(&self) -> ::bobbin_mcu::register::Register<Btr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Btr1, 0x4)
    }

    #[doc="Get the *mut pointer for the BTR1 register."]
    #[inline] pub fn btr1_mut(&self) -> *mut Btr1 { 
        self.btr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the BTR1 register."]
    #[inline] pub fn btr1_ptr(&self) -> *const Btr1 { 
        self.btr1_reg().ptr()
    }

    #[doc="Read the BTR1 register."]
    #[inline] pub fn btr1(&self) -> Btr1 { 
        self.btr1_reg().read()
    }

    #[doc="Write the BTR1 register."]
    #[inline] pub fn write_btr1(&self, value: Btr1) -> &Self { 
        self.btr1_reg().write(value);
        self
    }

    #[doc="Set the BTR1 register."]
    #[inline] pub fn set_btr1<F: FnOnce(Btr1) -> Btr1>(&self, f: F) -> &Self {
        self.btr1_reg().set(f);
        self
    }

    #[doc="Modify the BTR1 register."]
    #[inline] pub fn with_btr1<F: FnOnce(Btr1) -> Btr1>(&self, f: F) -> &Self {
        self.btr1_reg().with(f);
        self
    }

    #[doc="Get the BCR2 Register."]
    #[inline] pub fn bcr2_reg(&self) -> ::bobbin_mcu::register::Register<Bcr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bcr2, 0x8)
    }

    #[doc="Get the *mut pointer for the BCR2 register."]
    #[inline] pub fn bcr2_mut(&self) -> *mut Bcr2 { 
        self.bcr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the BCR2 register."]
    #[inline] pub fn bcr2_ptr(&self) -> *const Bcr2 { 
        self.bcr2_reg().ptr()
    }

    #[doc="Read the BCR2 register."]
    #[inline] pub fn bcr2(&self) -> Bcr2 { 
        self.bcr2_reg().read()
    }

    #[doc="Write the BCR2 register."]
    #[inline] pub fn write_bcr2(&self, value: Bcr2) -> &Self { 
        self.bcr2_reg().write(value);
        self
    }

    #[doc="Set the BCR2 register."]
    #[inline] pub fn set_bcr2<F: FnOnce(Bcr2) -> Bcr2>(&self, f: F) -> &Self {
        self.bcr2_reg().set(f);
        self
    }

    #[doc="Modify the BCR2 register."]
    #[inline] pub fn with_bcr2<F: FnOnce(Bcr2) -> Bcr2>(&self, f: F) -> &Self {
        self.bcr2_reg().with(f);
        self
    }

    #[doc="Get the BTR2 Register."]
    #[inline] pub fn btr2_reg(&self) -> ::bobbin_mcu::register::Register<Btr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Btr2, 0xc)
    }

    #[doc="Get the *mut pointer for the BTR2 register."]
    #[inline] pub fn btr2_mut(&self) -> *mut Btr2 { 
        self.btr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the BTR2 register."]
    #[inline] pub fn btr2_ptr(&self) -> *const Btr2 { 
        self.btr2_reg().ptr()
    }

    #[doc="Read the BTR2 register."]
    #[inline] pub fn btr2(&self) -> Btr2 { 
        self.btr2_reg().read()
    }

    #[doc="Write the BTR2 register."]
    #[inline] pub fn write_btr2(&self, value: Btr2) -> &Self { 
        self.btr2_reg().write(value);
        self
    }

    #[doc="Set the BTR2 register."]
    #[inline] pub fn set_btr2<F: FnOnce(Btr2) -> Btr2>(&self, f: F) -> &Self {
        self.btr2_reg().set(f);
        self
    }

    #[doc="Modify the BTR2 register."]
    #[inline] pub fn with_btr2<F: FnOnce(Btr2) -> Btr2>(&self, f: F) -> &Self {
        self.btr2_reg().with(f);
        self
    }

    #[doc="Get the BCR3 Register."]
    #[inline] pub fn bcr3_reg(&self) -> ::bobbin_mcu::register::Register<Bcr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bcr3, 0x10)
    }

    #[doc="Get the *mut pointer for the BCR3 register."]
    #[inline] pub fn bcr3_mut(&self) -> *mut Bcr3 { 
        self.bcr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the BCR3 register."]
    #[inline] pub fn bcr3_ptr(&self) -> *const Bcr3 { 
        self.bcr3_reg().ptr()
    }

    #[doc="Read the BCR3 register."]
    #[inline] pub fn bcr3(&self) -> Bcr3 { 
        self.bcr3_reg().read()
    }

    #[doc="Write the BCR3 register."]
    #[inline] pub fn write_bcr3(&self, value: Bcr3) -> &Self { 
        self.bcr3_reg().write(value);
        self
    }

    #[doc="Set the BCR3 register."]
    #[inline] pub fn set_bcr3<F: FnOnce(Bcr3) -> Bcr3>(&self, f: F) -> &Self {
        self.bcr3_reg().set(f);
        self
    }

    #[doc="Modify the BCR3 register."]
    #[inline] pub fn with_bcr3<F: FnOnce(Bcr3) -> Bcr3>(&self, f: F) -> &Self {
        self.bcr3_reg().with(f);
        self
    }

    #[doc="Get the BTR3 Register."]
    #[inline] pub fn btr3_reg(&self) -> ::bobbin_mcu::register::Register<Btr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Btr3, 0x14)
    }

    #[doc="Get the *mut pointer for the BTR3 register."]
    #[inline] pub fn btr3_mut(&self) -> *mut Btr3 { 
        self.btr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the BTR3 register."]
    #[inline] pub fn btr3_ptr(&self) -> *const Btr3 { 
        self.btr3_reg().ptr()
    }

    #[doc="Read the BTR3 register."]
    #[inline] pub fn btr3(&self) -> Btr3 { 
        self.btr3_reg().read()
    }

    #[doc="Write the BTR3 register."]
    #[inline] pub fn write_btr3(&self, value: Btr3) -> &Self { 
        self.btr3_reg().write(value);
        self
    }

    #[doc="Set the BTR3 register."]
    #[inline] pub fn set_btr3<F: FnOnce(Btr3) -> Btr3>(&self, f: F) -> &Self {
        self.btr3_reg().set(f);
        self
    }

    #[doc="Modify the BTR3 register."]
    #[inline] pub fn with_btr3<F: FnOnce(Btr3) -> Btr3>(&self, f: F) -> &Self {
        self.btr3_reg().with(f);
        self
    }

    #[doc="Get the BCR4 Register."]
    #[inline] pub fn bcr4_reg(&self) -> ::bobbin_mcu::register::Register<Bcr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bcr4, 0x18)
    }

    #[doc="Get the *mut pointer for the BCR4 register."]
    #[inline] pub fn bcr4_mut(&self) -> *mut Bcr4 { 
        self.bcr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the BCR4 register."]
    #[inline] pub fn bcr4_ptr(&self) -> *const Bcr4 { 
        self.bcr4_reg().ptr()
    }

    #[doc="Read the BCR4 register."]
    #[inline] pub fn bcr4(&self) -> Bcr4 { 
        self.bcr4_reg().read()
    }

    #[doc="Write the BCR4 register."]
    #[inline] pub fn write_bcr4(&self, value: Bcr4) -> &Self { 
        self.bcr4_reg().write(value);
        self
    }

    #[doc="Set the BCR4 register."]
    #[inline] pub fn set_bcr4<F: FnOnce(Bcr4) -> Bcr4>(&self, f: F) -> &Self {
        self.bcr4_reg().set(f);
        self
    }

    #[doc="Modify the BCR4 register."]
    #[inline] pub fn with_bcr4<F: FnOnce(Bcr4) -> Bcr4>(&self, f: F) -> &Self {
        self.bcr4_reg().with(f);
        self
    }

    #[doc="Get the BTR4 Register."]
    #[inline] pub fn btr4_reg(&self) -> ::bobbin_mcu::register::Register<Btr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Btr4, 0x1c)
    }

    #[doc="Get the *mut pointer for the BTR4 register."]
    #[inline] pub fn btr4_mut(&self) -> *mut Btr4 { 
        self.btr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the BTR4 register."]
    #[inline] pub fn btr4_ptr(&self) -> *const Btr4 { 
        self.btr4_reg().ptr()
    }

    #[doc="Read the BTR4 register."]
    #[inline] pub fn btr4(&self) -> Btr4 { 
        self.btr4_reg().read()
    }

    #[doc="Write the BTR4 register."]
    #[inline] pub fn write_btr4(&self, value: Btr4) -> &Self { 
        self.btr4_reg().write(value);
        self
    }

    #[doc="Set the BTR4 register."]
    #[inline] pub fn set_btr4<F: FnOnce(Btr4) -> Btr4>(&self, f: F) -> &Self {
        self.btr4_reg().set(f);
        self
    }

    #[doc="Modify the BTR4 register."]
    #[inline] pub fn with_btr4<F: FnOnce(Btr4) -> Btr4>(&self, f: F) -> &Self {
        self.btr4_reg().with(f);
        self
    }

    #[doc="Get the PCR2 Register."]
    #[inline] pub fn pcr2_reg(&self) -> ::bobbin_mcu::register::Register<Pcr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcr2, 0x60)
    }

    #[doc="Get the *mut pointer for the PCR2 register."]
    #[inline] pub fn pcr2_mut(&self) -> *mut Pcr2 { 
        self.pcr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCR2 register."]
    #[inline] pub fn pcr2_ptr(&self) -> *const Pcr2 { 
        self.pcr2_reg().ptr()
    }

    #[doc="Read the PCR2 register."]
    #[inline] pub fn pcr2(&self) -> Pcr2 { 
        self.pcr2_reg().read()
    }

    #[doc="Write the PCR2 register."]
    #[inline] pub fn write_pcr2(&self, value: Pcr2) -> &Self { 
        self.pcr2_reg().write(value);
        self
    }

    #[doc="Set the PCR2 register."]
    #[inline] pub fn set_pcr2<F: FnOnce(Pcr2) -> Pcr2>(&self, f: F) -> &Self {
        self.pcr2_reg().set(f);
        self
    }

    #[doc="Modify the PCR2 register."]
    #[inline] pub fn with_pcr2<F: FnOnce(Pcr2) -> Pcr2>(&self, f: F) -> &Self {
        self.pcr2_reg().with(f);
        self
    }

    #[doc="Get the SR2 Register."]
    #[inline] pub fn sr2_reg(&self) -> ::bobbin_mcu::register::Register<Sr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr2, 0x64)
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

    #[doc="Write the SR2 register."]
    #[inline] pub fn write_sr2(&self, value: Sr2) -> &Self { 
        self.sr2_reg().write(value);
        self
    }

    #[doc="Set the SR2 register."]
    #[inline] pub fn set_sr2<F: FnOnce(Sr2) -> Sr2>(&self, f: F) -> &Self {
        self.sr2_reg().set(f);
        self
    }

    #[doc="Modify the SR2 register."]
    #[inline] pub fn with_sr2<F: FnOnce(Sr2) -> Sr2>(&self, f: F) -> &Self {
        self.sr2_reg().with(f);
        self
    }

    #[doc="Get the PMEM2 Register."]
    #[inline] pub fn pmem2_reg(&self) -> ::bobbin_mcu::register::Register<Pmem2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pmem2, 0x68)
    }

    #[doc="Get the *mut pointer for the PMEM2 register."]
    #[inline] pub fn pmem2_mut(&self) -> *mut Pmem2 { 
        self.pmem2_reg().ptr()
    }

    #[doc="Get the *const pointer for the PMEM2 register."]
    #[inline] pub fn pmem2_ptr(&self) -> *const Pmem2 { 
        self.pmem2_reg().ptr()
    }

    #[doc="Read the PMEM2 register."]
    #[inline] pub fn pmem2(&self) -> Pmem2 { 
        self.pmem2_reg().read()
    }

    #[doc="Write the PMEM2 register."]
    #[inline] pub fn write_pmem2(&self, value: Pmem2) -> &Self { 
        self.pmem2_reg().write(value);
        self
    }

    #[doc="Set the PMEM2 register."]
    #[inline] pub fn set_pmem2<F: FnOnce(Pmem2) -> Pmem2>(&self, f: F) -> &Self {
        self.pmem2_reg().set(f);
        self
    }

    #[doc="Modify the PMEM2 register."]
    #[inline] pub fn with_pmem2<F: FnOnce(Pmem2) -> Pmem2>(&self, f: F) -> &Self {
        self.pmem2_reg().with(f);
        self
    }

    #[doc="Get the PATT2 Register."]
    #[inline] pub fn patt2_reg(&self) -> ::bobbin_mcu::register::Register<Patt2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Patt2, 0x6c)
    }

    #[doc="Get the *mut pointer for the PATT2 register."]
    #[inline] pub fn patt2_mut(&self) -> *mut Patt2 { 
        self.patt2_reg().ptr()
    }

    #[doc="Get the *const pointer for the PATT2 register."]
    #[inline] pub fn patt2_ptr(&self) -> *const Patt2 { 
        self.patt2_reg().ptr()
    }

    #[doc="Read the PATT2 register."]
    #[inline] pub fn patt2(&self) -> Patt2 { 
        self.patt2_reg().read()
    }

    #[doc="Write the PATT2 register."]
    #[inline] pub fn write_patt2(&self, value: Patt2) -> &Self { 
        self.patt2_reg().write(value);
        self
    }

    #[doc="Set the PATT2 register."]
    #[inline] pub fn set_patt2<F: FnOnce(Patt2) -> Patt2>(&self, f: F) -> &Self {
        self.patt2_reg().set(f);
        self
    }

    #[doc="Modify the PATT2 register."]
    #[inline] pub fn with_patt2<F: FnOnce(Patt2) -> Patt2>(&self, f: F) -> &Self {
        self.patt2_reg().with(f);
        self
    }

    #[doc="Get the ECCR2 Register."]
    #[inline] pub fn eccr2_reg(&self) -> ::bobbin_mcu::register::Register<Eccr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eccr2, 0x74)
    }

    #[doc="Get the *mut pointer for the ECCR2 register."]
    #[inline] pub fn eccr2_mut(&self) -> *mut Eccr2 { 
        self.eccr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the ECCR2 register."]
    #[inline] pub fn eccr2_ptr(&self) -> *const Eccr2 { 
        self.eccr2_reg().ptr()
    }

    #[doc="Read the ECCR2 register."]
    #[inline] pub fn eccr2(&self) -> Eccr2 { 
        self.eccr2_reg().read()
    }

    #[doc="Get the PCR3 Register."]
    #[inline] pub fn pcr3_reg(&self) -> ::bobbin_mcu::register::Register<Pcr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcr3, 0x80)
    }

    #[doc="Get the *mut pointer for the PCR3 register."]
    #[inline] pub fn pcr3_mut(&self) -> *mut Pcr3 { 
        self.pcr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCR3 register."]
    #[inline] pub fn pcr3_ptr(&self) -> *const Pcr3 { 
        self.pcr3_reg().ptr()
    }

    #[doc="Read the PCR3 register."]
    #[inline] pub fn pcr3(&self) -> Pcr3 { 
        self.pcr3_reg().read()
    }

    #[doc="Write the PCR3 register."]
    #[inline] pub fn write_pcr3(&self, value: Pcr3) -> &Self { 
        self.pcr3_reg().write(value);
        self
    }

    #[doc="Set the PCR3 register."]
    #[inline] pub fn set_pcr3<F: FnOnce(Pcr3) -> Pcr3>(&self, f: F) -> &Self {
        self.pcr3_reg().set(f);
        self
    }

    #[doc="Modify the PCR3 register."]
    #[inline] pub fn with_pcr3<F: FnOnce(Pcr3) -> Pcr3>(&self, f: F) -> &Self {
        self.pcr3_reg().with(f);
        self
    }

    #[doc="Get the SR3 Register."]
    #[inline] pub fn sr3_reg(&self) -> ::bobbin_mcu::register::Register<Sr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr3, 0x84)
    }

    #[doc="Get the *mut pointer for the SR3 register."]
    #[inline] pub fn sr3_mut(&self) -> *mut Sr3 { 
        self.sr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR3 register."]
    #[inline] pub fn sr3_ptr(&self) -> *const Sr3 { 
        self.sr3_reg().ptr()
    }

    #[doc="Read the SR3 register."]
    #[inline] pub fn sr3(&self) -> Sr3 { 
        self.sr3_reg().read()
    }

    #[doc="Write the SR3 register."]
    #[inline] pub fn write_sr3(&self, value: Sr3) -> &Self { 
        self.sr3_reg().write(value);
        self
    }

    #[doc="Set the SR3 register."]
    #[inline] pub fn set_sr3<F: FnOnce(Sr3) -> Sr3>(&self, f: F) -> &Self {
        self.sr3_reg().set(f);
        self
    }

    #[doc="Modify the SR3 register."]
    #[inline] pub fn with_sr3<F: FnOnce(Sr3) -> Sr3>(&self, f: F) -> &Self {
        self.sr3_reg().with(f);
        self
    }

    #[doc="Get the PMEM3 Register."]
    #[inline] pub fn pmem3_reg(&self) -> ::bobbin_mcu::register::Register<Pmem3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pmem3, 0x88)
    }

    #[doc="Get the *mut pointer for the PMEM3 register."]
    #[inline] pub fn pmem3_mut(&self) -> *mut Pmem3 { 
        self.pmem3_reg().ptr()
    }

    #[doc="Get the *const pointer for the PMEM3 register."]
    #[inline] pub fn pmem3_ptr(&self) -> *const Pmem3 { 
        self.pmem3_reg().ptr()
    }

    #[doc="Read the PMEM3 register."]
    #[inline] pub fn pmem3(&self) -> Pmem3 { 
        self.pmem3_reg().read()
    }

    #[doc="Write the PMEM3 register."]
    #[inline] pub fn write_pmem3(&self, value: Pmem3) -> &Self { 
        self.pmem3_reg().write(value);
        self
    }

    #[doc="Set the PMEM3 register."]
    #[inline] pub fn set_pmem3<F: FnOnce(Pmem3) -> Pmem3>(&self, f: F) -> &Self {
        self.pmem3_reg().set(f);
        self
    }

    #[doc="Modify the PMEM3 register."]
    #[inline] pub fn with_pmem3<F: FnOnce(Pmem3) -> Pmem3>(&self, f: F) -> &Self {
        self.pmem3_reg().with(f);
        self
    }

    #[doc="Get the PATT3 Register."]
    #[inline] pub fn patt3_reg(&self) -> ::bobbin_mcu::register::Register<Patt3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Patt3, 0x8c)
    }

    #[doc="Get the *mut pointer for the PATT3 register."]
    #[inline] pub fn patt3_mut(&self) -> *mut Patt3 { 
        self.patt3_reg().ptr()
    }

    #[doc="Get the *const pointer for the PATT3 register."]
    #[inline] pub fn patt3_ptr(&self) -> *const Patt3 { 
        self.patt3_reg().ptr()
    }

    #[doc="Read the PATT3 register."]
    #[inline] pub fn patt3(&self) -> Patt3 { 
        self.patt3_reg().read()
    }

    #[doc="Write the PATT3 register."]
    #[inline] pub fn write_patt3(&self, value: Patt3) -> &Self { 
        self.patt3_reg().write(value);
        self
    }

    #[doc="Set the PATT3 register."]
    #[inline] pub fn set_patt3<F: FnOnce(Patt3) -> Patt3>(&self, f: F) -> &Self {
        self.patt3_reg().set(f);
        self
    }

    #[doc="Modify the PATT3 register."]
    #[inline] pub fn with_patt3<F: FnOnce(Patt3) -> Patt3>(&self, f: F) -> &Self {
        self.patt3_reg().with(f);
        self
    }

    #[doc="Get the ECCR3 Register."]
    #[inline] pub fn eccr3_reg(&self) -> ::bobbin_mcu::register::Register<Eccr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eccr3, 0x94)
    }

    #[doc="Get the *mut pointer for the ECCR3 register."]
    #[inline] pub fn eccr3_mut(&self) -> *mut Eccr3 { 
        self.eccr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the ECCR3 register."]
    #[inline] pub fn eccr3_ptr(&self) -> *const Eccr3 { 
        self.eccr3_reg().ptr()
    }

    #[doc="Read the ECCR3 register."]
    #[inline] pub fn eccr3(&self) -> Eccr3 { 
        self.eccr3_reg().read()
    }

    #[doc="Get the PCR4 Register."]
    #[inline] pub fn pcr4_reg(&self) -> ::bobbin_mcu::register::Register<Pcr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcr4, 0xa0)
    }

    #[doc="Get the *mut pointer for the PCR4 register."]
    #[inline] pub fn pcr4_mut(&self) -> *mut Pcr4 { 
        self.pcr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCR4 register."]
    #[inline] pub fn pcr4_ptr(&self) -> *const Pcr4 { 
        self.pcr4_reg().ptr()
    }

    #[doc="Read the PCR4 register."]
    #[inline] pub fn pcr4(&self) -> Pcr4 { 
        self.pcr4_reg().read()
    }

    #[doc="Write the PCR4 register."]
    #[inline] pub fn write_pcr4(&self, value: Pcr4) -> &Self { 
        self.pcr4_reg().write(value);
        self
    }

    #[doc="Set the PCR4 register."]
    #[inline] pub fn set_pcr4<F: FnOnce(Pcr4) -> Pcr4>(&self, f: F) -> &Self {
        self.pcr4_reg().set(f);
        self
    }

    #[doc="Modify the PCR4 register."]
    #[inline] pub fn with_pcr4<F: FnOnce(Pcr4) -> Pcr4>(&self, f: F) -> &Self {
        self.pcr4_reg().with(f);
        self
    }

    #[doc="Get the SR4 Register."]
    #[inline] pub fn sr4_reg(&self) -> ::bobbin_mcu::register::Register<Sr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr4, 0xa4)
    }

    #[doc="Get the *mut pointer for the SR4 register."]
    #[inline] pub fn sr4_mut(&self) -> *mut Sr4 { 
        self.sr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR4 register."]
    #[inline] pub fn sr4_ptr(&self) -> *const Sr4 { 
        self.sr4_reg().ptr()
    }

    #[doc="Read the SR4 register."]
    #[inline] pub fn sr4(&self) -> Sr4 { 
        self.sr4_reg().read()
    }

    #[doc="Write the SR4 register."]
    #[inline] pub fn write_sr4(&self, value: Sr4) -> &Self { 
        self.sr4_reg().write(value);
        self
    }

    #[doc="Set the SR4 register."]
    #[inline] pub fn set_sr4<F: FnOnce(Sr4) -> Sr4>(&self, f: F) -> &Self {
        self.sr4_reg().set(f);
        self
    }

    #[doc="Modify the SR4 register."]
    #[inline] pub fn with_sr4<F: FnOnce(Sr4) -> Sr4>(&self, f: F) -> &Self {
        self.sr4_reg().with(f);
        self
    }

    #[doc="Get the PMEM4 Register."]
    #[inline] pub fn pmem4_reg(&self) -> ::bobbin_mcu::register::Register<Pmem4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pmem4, 0xa8)
    }

    #[doc="Get the *mut pointer for the PMEM4 register."]
    #[inline] pub fn pmem4_mut(&self) -> *mut Pmem4 { 
        self.pmem4_reg().ptr()
    }

    #[doc="Get the *const pointer for the PMEM4 register."]
    #[inline] pub fn pmem4_ptr(&self) -> *const Pmem4 { 
        self.pmem4_reg().ptr()
    }

    #[doc="Read the PMEM4 register."]
    #[inline] pub fn pmem4(&self) -> Pmem4 { 
        self.pmem4_reg().read()
    }

    #[doc="Write the PMEM4 register."]
    #[inline] pub fn write_pmem4(&self, value: Pmem4) -> &Self { 
        self.pmem4_reg().write(value);
        self
    }

    #[doc="Set the PMEM4 register."]
    #[inline] pub fn set_pmem4<F: FnOnce(Pmem4) -> Pmem4>(&self, f: F) -> &Self {
        self.pmem4_reg().set(f);
        self
    }

    #[doc="Modify the PMEM4 register."]
    #[inline] pub fn with_pmem4<F: FnOnce(Pmem4) -> Pmem4>(&self, f: F) -> &Self {
        self.pmem4_reg().with(f);
        self
    }

    #[doc="Get the PATT4 Register."]
    #[inline] pub fn patt4_reg(&self) -> ::bobbin_mcu::register::Register<Patt4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Patt4, 0xac)
    }

    #[doc="Get the *mut pointer for the PATT4 register."]
    #[inline] pub fn patt4_mut(&self) -> *mut Patt4 { 
        self.patt4_reg().ptr()
    }

    #[doc="Get the *const pointer for the PATT4 register."]
    #[inline] pub fn patt4_ptr(&self) -> *const Patt4 { 
        self.patt4_reg().ptr()
    }

    #[doc="Read the PATT4 register."]
    #[inline] pub fn patt4(&self) -> Patt4 { 
        self.patt4_reg().read()
    }

    #[doc="Write the PATT4 register."]
    #[inline] pub fn write_patt4(&self, value: Patt4) -> &Self { 
        self.patt4_reg().write(value);
        self
    }

    #[doc="Set the PATT4 register."]
    #[inline] pub fn set_patt4<F: FnOnce(Patt4) -> Patt4>(&self, f: F) -> &Self {
        self.patt4_reg().set(f);
        self
    }

    #[doc="Modify the PATT4 register."]
    #[inline] pub fn with_patt4<F: FnOnce(Patt4) -> Patt4>(&self, f: F) -> &Self {
        self.patt4_reg().with(f);
        self
    }

    #[doc="Get the PIO4 Register."]
    #[inline] pub fn pio4_reg(&self) -> ::bobbin_mcu::register::Register<Pio4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pio4, 0xb0)
    }

    #[doc="Get the *mut pointer for the PIO4 register."]
    #[inline] pub fn pio4_mut(&self) -> *mut Pio4 { 
        self.pio4_reg().ptr()
    }

    #[doc="Get the *const pointer for the PIO4 register."]
    #[inline] pub fn pio4_ptr(&self) -> *const Pio4 { 
        self.pio4_reg().ptr()
    }

    #[doc="Read the PIO4 register."]
    #[inline] pub fn pio4(&self) -> Pio4 { 
        self.pio4_reg().read()
    }

    #[doc="Write the PIO4 register."]
    #[inline] pub fn write_pio4(&self, value: Pio4) -> &Self { 
        self.pio4_reg().write(value);
        self
    }

    #[doc="Set the PIO4 register."]
    #[inline] pub fn set_pio4<F: FnOnce(Pio4) -> Pio4>(&self, f: F) -> &Self {
        self.pio4_reg().set(f);
        self
    }

    #[doc="Modify the PIO4 register."]
    #[inline] pub fn with_pio4<F: FnOnce(Pio4) -> Pio4>(&self, f: F) -> &Self {
        self.pio4_reg().with(f);
        self
    }

    #[doc="Get the BWTR1 Register."]
    #[inline] pub fn bwtr1_reg(&self) -> ::bobbin_mcu::register::Register<Bwtr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bwtr1, 0x104)
    }

    #[doc="Get the *mut pointer for the BWTR1 register."]
    #[inline] pub fn bwtr1_mut(&self) -> *mut Bwtr1 { 
        self.bwtr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the BWTR1 register."]
    #[inline] pub fn bwtr1_ptr(&self) -> *const Bwtr1 { 
        self.bwtr1_reg().ptr()
    }

    #[doc="Read the BWTR1 register."]
    #[inline] pub fn bwtr1(&self) -> Bwtr1 { 
        self.bwtr1_reg().read()
    }

    #[doc="Write the BWTR1 register."]
    #[inline] pub fn write_bwtr1(&self, value: Bwtr1) -> &Self { 
        self.bwtr1_reg().write(value);
        self
    }

    #[doc="Set the BWTR1 register."]
    #[inline] pub fn set_bwtr1<F: FnOnce(Bwtr1) -> Bwtr1>(&self, f: F) -> &Self {
        self.bwtr1_reg().set(f);
        self
    }

    #[doc="Modify the BWTR1 register."]
    #[inline] pub fn with_bwtr1<F: FnOnce(Bwtr1) -> Bwtr1>(&self, f: F) -> &Self {
        self.bwtr1_reg().with(f);
        self
    }

    #[doc="Get the BWTR2 Register."]
    #[inline] pub fn bwtr2_reg(&self) -> ::bobbin_mcu::register::Register<Bwtr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bwtr2, 0x10c)
    }

    #[doc="Get the *mut pointer for the BWTR2 register."]
    #[inline] pub fn bwtr2_mut(&self) -> *mut Bwtr2 { 
        self.bwtr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the BWTR2 register."]
    #[inline] pub fn bwtr2_ptr(&self) -> *const Bwtr2 { 
        self.bwtr2_reg().ptr()
    }

    #[doc="Read the BWTR2 register."]
    #[inline] pub fn bwtr2(&self) -> Bwtr2 { 
        self.bwtr2_reg().read()
    }

    #[doc="Write the BWTR2 register."]
    #[inline] pub fn write_bwtr2(&self, value: Bwtr2) -> &Self { 
        self.bwtr2_reg().write(value);
        self
    }

    #[doc="Set the BWTR2 register."]
    #[inline] pub fn set_bwtr2<F: FnOnce(Bwtr2) -> Bwtr2>(&self, f: F) -> &Self {
        self.bwtr2_reg().set(f);
        self
    }

    #[doc="Modify the BWTR2 register."]
    #[inline] pub fn with_bwtr2<F: FnOnce(Bwtr2) -> Bwtr2>(&self, f: F) -> &Self {
        self.bwtr2_reg().with(f);
        self
    }

    #[doc="Get the BWTR3 Register."]
    #[inline] pub fn bwtr3_reg(&self) -> ::bobbin_mcu::register::Register<Bwtr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bwtr3, 0x104)
    }

    #[doc="Get the *mut pointer for the BWTR3 register."]
    #[inline] pub fn bwtr3_mut(&self) -> *mut Bwtr3 { 
        self.bwtr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the BWTR3 register."]
    #[inline] pub fn bwtr3_ptr(&self) -> *const Bwtr3 { 
        self.bwtr3_reg().ptr()
    }

    #[doc="Read the BWTR3 register."]
    #[inline] pub fn bwtr3(&self) -> Bwtr3 { 
        self.bwtr3_reg().read()
    }

    #[doc="Write the BWTR3 register."]
    #[inline] pub fn write_bwtr3(&self, value: Bwtr3) -> &Self { 
        self.bwtr3_reg().write(value);
        self
    }

    #[doc="Set the BWTR3 register."]
    #[inline] pub fn set_bwtr3<F: FnOnce(Bwtr3) -> Bwtr3>(&self, f: F) -> &Self {
        self.bwtr3_reg().set(f);
        self
    }

    #[doc="Modify the BWTR3 register."]
    #[inline] pub fn with_bwtr3<F: FnOnce(Bwtr3) -> Bwtr3>(&self, f: F) -> &Self {
        self.bwtr3_reg().with(f);
        self
    }

    #[doc="Get the BWTR4 Register."]
    #[inline] pub fn bwtr4_reg(&self) -> ::bobbin_mcu::register::Register<Bwtr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bwtr4, 0x10c)
    }

    #[doc="Get the *mut pointer for the BWTR4 register."]
    #[inline] pub fn bwtr4_mut(&self) -> *mut Bwtr4 { 
        self.bwtr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the BWTR4 register."]
    #[inline] pub fn bwtr4_ptr(&self) -> *const Bwtr4 { 
        self.bwtr4_reg().ptr()
    }

    #[doc="Read the BWTR4 register."]
    #[inline] pub fn bwtr4(&self) -> Bwtr4 { 
        self.bwtr4_reg().read()
    }

    #[doc="Write the BWTR4 register."]
    #[inline] pub fn write_bwtr4(&self, value: Bwtr4) -> &Self { 
        self.bwtr4_reg().write(value);
        self
    }

    #[doc="Set the BWTR4 register."]
    #[inline] pub fn set_bwtr4<F: FnOnce(Bwtr4) -> Bwtr4>(&self, f: F) -> &Self {
        self.bwtr4_reg().set(f);
        self
    }

    #[doc="Modify the BWTR4 register."]
    #[inline] pub fn with_bwtr4<F: FnOnce(Bwtr4) -> Bwtr4>(&self, f: F) -> &Self {
        self.bwtr4_reg().with(f);
        self
    }

    #[doc="Get the SDCR1 Register."]
    #[inline] pub fn sdcr1_reg(&self) -> ::bobbin_mcu::register::Register<Sdcr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sdcr1, 0x140)
    }

    #[doc="Get the *mut pointer for the SDCR1 register."]
    #[inline] pub fn sdcr1_mut(&self) -> *mut Sdcr1 { 
        self.sdcr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SDCR1 register."]
    #[inline] pub fn sdcr1_ptr(&self) -> *const Sdcr1 { 
        self.sdcr1_reg().ptr()
    }

    #[doc="Read the SDCR1 register."]
    #[inline] pub fn sdcr1(&self) -> Sdcr1 { 
        self.sdcr1_reg().read()
    }

    #[doc="Write the SDCR1 register."]
    #[inline] pub fn write_sdcr1(&self, value: Sdcr1) -> &Self { 
        self.sdcr1_reg().write(value);
        self
    }

    #[doc="Set the SDCR1 register."]
    #[inline] pub fn set_sdcr1<F: FnOnce(Sdcr1) -> Sdcr1>(&self, f: F) -> &Self {
        self.sdcr1_reg().set(f);
        self
    }

    #[doc="Modify the SDCR1 register."]
    #[inline] pub fn with_sdcr1<F: FnOnce(Sdcr1) -> Sdcr1>(&self, f: F) -> &Self {
        self.sdcr1_reg().with(f);
        self
    }

    #[doc="Get the SDCR2 Register."]
    #[inline] pub fn sdcr2_reg(&self) -> ::bobbin_mcu::register::Register<Sdcr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sdcr2, 0x144)
    }

    #[doc="Get the *mut pointer for the SDCR2 register."]
    #[inline] pub fn sdcr2_mut(&self) -> *mut Sdcr2 { 
        self.sdcr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SDCR2 register."]
    #[inline] pub fn sdcr2_ptr(&self) -> *const Sdcr2 { 
        self.sdcr2_reg().ptr()
    }

    #[doc="Read the SDCR2 register."]
    #[inline] pub fn sdcr2(&self) -> Sdcr2 { 
        self.sdcr2_reg().read()
    }

    #[doc="Write the SDCR2 register."]
    #[inline] pub fn write_sdcr2(&self, value: Sdcr2) -> &Self { 
        self.sdcr2_reg().write(value);
        self
    }

    #[doc="Set the SDCR2 register."]
    #[inline] pub fn set_sdcr2<F: FnOnce(Sdcr2) -> Sdcr2>(&self, f: F) -> &Self {
        self.sdcr2_reg().set(f);
        self
    }

    #[doc="Modify the SDCR2 register."]
    #[inline] pub fn with_sdcr2<F: FnOnce(Sdcr2) -> Sdcr2>(&self, f: F) -> &Self {
        self.sdcr2_reg().with(f);
        self
    }

    #[doc="Get the SDTR1 Register."]
    #[inline] pub fn sdtr1_reg(&self) -> ::bobbin_mcu::register::Register<Sdtr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sdtr1, 0x148)
    }

    #[doc="Get the *mut pointer for the SDTR1 register."]
    #[inline] pub fn sdtr1_mut(&self) -> *mut Sdtr1 { 
        self.sdtr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SDTR1 register."]
    #[inline] pub fn sdtr1_ptr(&self) -> *const Sdtr1 { 
        self.sdtr1_reg().ptr()
    }

    #[doc="Read the SDTR1 register."]
    #[inline] pub fn sdtr1(&self) -> Sdtr1 { 
        self.sdtr1_reg().read()
    }

    #[doc="Write the SDTR1 register."]
    #[inline] pub fn write_sdtr1(&self, value: Sdtr1) -> &Self { 
        self.sdtr1_reg().write(value);
        self
    }

    #[doc="Set the SDTR1 register."]
    #[inline] pub fn set_sdtr1<F: FnOnce(Sdtr1) -> Sdtr1>(&self, f: F) -> &Self {
        self.sdtr1_reg().set(f);
        self
    }

    #[doc="Modify the SDTR1 register."]
    #[inline] pub fn with_sdtr1<F: FnOnce(Sdtr1) -> Sdtr1>(&self, f: F) -> &Self {
        self.sdtr1_reg().with(f);
        self
    }

    #[doc="Get the SDTR2 Register."]
    #[inline] pub fn sdtr2_reg(&self) -> ::bobbin_mcu::register::Register<Sdtr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sdtr2, 0x14c)
    }

    #[doc="Get the *mut pointer for the SDTR2 register."]
    #[inline] pub fn sdtr2_mut(&self) -> *mut Sdtr2 { 
        self.sdtr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SDTR2 register."]
    #[inline] pub fn sdtr2_ptr(&self) -> *const Sdtr2 { 
        self.sdtr2_reg().ptr()
    }

    #[doc="Read the SDTR2 register."]
    #[inline] pub fn sdtr2(&self) -> Sdtr2 { 
        self.sdtr2_reg().read()
    }

    #[doc="Write the SDTR2 register."]
    #[inline] pub fn write_sdtr2(&self, value: Sdtr2) -> &Self { 
        self.sdtr2_reg().write(value);
        self
    }

    #[doc="Set the SDTR2 register."]
    #[inline] pub fn set_sdtr2<F: FnOnce(Sdtr2) -> Sdtr2>(&self, f: F) -> &Self {
        self.sdtr2_reg().set(f);
        self
    }

    #[doc="Modify the SDTR2 register."]
    #[inline] pub fn with_sdtr2<F: FnOnce(Sdtr2) -> Sdtr2>(&self, f: F) -> &Self {
        self.sdtr2_reg().with(f);
        self
    }

    #[doc="Get the SDCMR Register."]
    #[inline] pub fn sdcmr_reg(&self) -> ::bobbin_mcu::register::Register<Sdcmr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sdcmr, 0x150)
    }

    #[doc="Get the *mut pointer for the SDCMR register."]
    #[inline] pub fn sdcmr_mut(&self) -> *mut Sdcmr { 
        self.sdcmr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SDCMR register."]
    #[inline] pub fn sdcmr_ptr(&self) -> *const Sdcmr { 
        self.sdcmr_reg().ptr()
    }

    #[doc="Read the SDCMR register."]
    #[inline] pub fn sdcmr(&self) -> Sdcmr { 
        self.sdcmr_reg().read()
    }

    #[doc="Write the SDCMR register."]
    #[inline] pub fn write_sdcmr(&self, value: Sdcmr) -> &Self { 
        self.sdcmr_reg().write(value);
        self
    }

    #[doc="Set the SDCMR register."]
    #[inline] pub fn set_sdcmr<F: FnOnce(Sdcmr) -> Sdcmr>(&self, f: F) -> &Self {
        self.sdcmr_reg().set(f);
        self
    }

    #[doc="Modify the SDCMR register."]
    #[inline] pub fn with_sdcmr<F: FnOnce(Sdcmr) -> Sdcmr>(&self, f: F) -> &Self {
        self.sdcmr_reg().with(f);
        self
    }

    #[doc="Get the SDRTR Register."]
    #[inline] pub fn sdrtr_reg(&self) -> ::bobbin_mcu::register::Register<Sdrtr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sdrtr, 0x154)
    }

    #[doc="Get the *mut pointer for the SDRTR register."]
    #[inline] pub fn sdrtr_mut(&self) -> *mut Sdrtr { 
        self.sdrtr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SDRTR register."]
    #[inline] pub fn sdrtr_ptr(&self) -> *const Sdrtr { 
        self.sdrtr_reg().ptr()
    }

    #[doc="Read the SDRTR register."]
    #[inline] pub fn sdrtr(&self) -> Sdrtr { 
        self.sdrtr_reg().read()
    }

    #[doc="Write the SDRTR register."]
    #[inline] pub fn write_sdrtr(&self, value: Sdrtr) -> &Self { 
        self.sdrtr_reg().write(value);
        self
    }

    #[doc="Set the SDRTR register."]
    #[inline] pub fn set_sdrtr<F: FnOnce(Sdrtr) -> Sdrtr>(&self, f: F) -> &Self {
        self.sdrtr_reg().set(f);
        self
    }

    #[doc="Modify the SDRTR register."]
    #[inline] pub fn with_sdrtr<F: FnOnce(Sdrtr) -> Sdrtr>(&self, f: F) -> &Self {
        self.sdrtr_reg().with(f);
        self
    }

    #[doc="Get the SDSR Register."]
    #[inline] pub fn sdsr_reg(&self) -> ::bobbin_mcu::register::Register<Sdsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sdsr, 0x158)
    }

    #[doc="Get the *mut pointer for the SDSR register."]
    #[inline] pub fn sdsr_mut(&self) -> *mut Sdsr { 
        self.sdsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SDSR register."]
    #[inline] pub fn sdsr_ptr(&self) -> *const Sdsr { 
        self.sdsr_reg().ptr()
    }

    #[doc="Read the SDSR register."]
    #[inline] pub fn sdsr(&self) -> Sdsr { 
        self.sdsr_reg().read()
    }

}

#[doc="SRAM/NOR-Flash chip-select control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr1(pub u32);
impl Bcr1 {
    #[doc="CCLKEN"]
    #[inline] pub fn cclken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CCLKEN != 0"]
    #[inline] pub fn test_cclken(&self) -> bool {
        self.cclken() != 0
    }

    #[doc="Sets the CCLKEN field."]
    #[inline] pub fn set_cclken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="CBURSTRW"]
    #[inline] pub fn cburstrw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CBURSTRW != 0"]
    #[inline] pub fn test_cburstrw(&self) -> bool {
        self.cburstrw() != 0
    }

    #[doc="Sets the CBURSTRW field."]
    #[inline] pub fn set_cburstrw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ASYNCWAIT"]
    #[inline] pub fn asyncwait(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ASYNCWAIT != 0"]
    #[inline] pub fn test_asyncwait(&self) -> bool {
        self.asyncwait() != 0
    }

    #[doc="Sets the ASYNCWAIT field."]
    #[inline] pub fn set_asyncwait<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="EXTMOD"]
    #[inline] pub fn extmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if EXTMOD != 0"]
    #[inline] pub fn test_extmod(&self) -> bool {
        self.extmod() != 0
    }

    #[doc="Sets the EXTMOD field."]
    #[inline] pub fn set_extmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="WAITEN"]
    #[inline] pub fn waiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if WAITEN != 0"]
    #[inline] pub fn test_waiten(&self) -> bool {
        self.waiten() != 0
    }

    #[doc="Sets the WAITEN field."]
    #[inline] pub fn set_waiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="WREN"]
    #[inline] pub fn wren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WREN != 0"]
    #[inline] pub fn test_wren(&self) -> bool {
        self.wren() != 0
    }

    #[doc="Sets the WREN field."]
    #[inline] pub fn set_wren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="WAITCFG"]
    #[inline] pub fn waitcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WAITCFG != 0"]
    #[inline] pub fn test_waitcfg(&self) -> bool {
        self.waitcfg() != 0
    }

    #[doc="Sets the WAITCFG field."]
    #[inline] pub fn set_waitcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="WAITPOL"]
    #[inline] pub fn waitpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if WAITPOL != 0"]
    #[inline] pub fn test_waitpol(&self) -> bool {
        self.waitpol() != 0
    }

    #[doc="Sets the WAITPOL field."]
    #[inline] pub fn set_waitpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BURSTEN"]
    #[inline] pub fn bursten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BURSTEN != 0"]
    #[inline] pub fn test_bursten(&self) -> bool {
        self.bursten() != 0
    }

    #[doc="Sets the BURSTEN field."]
    #[inline] pub fn set_bursten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FACCEN"]
    #[inline] pub fn faccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FACCEN != 0"]
    #[inline] pub fn test_faccen(&self) -> bool {
        self.faccen() != 0
    }

    #[doc="Sets the FACCEN field."]
    #[inline] pub fn set_faccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MWID"]
    #[inline] pub fn mwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if MWID != 0"]
    #[inline] pub fn test_mwid(&self) -> bool {
        self.mwid() != 0
    }

    #[doc="Sets the MWID field."]
    #[inline] pub fn set_mwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MTYP"]
    #[inline] pub fn mtyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MTYP != 0"]
    #[inline] pub fn test_mtyp(&self) -> bool {
        self.mtyp() != 0
    }

    #[doc="Sets the MTYP field."]
    #[inline] pub fn set_mtyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MUXEN"]
    #[inline] pub fn muxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUXEN != 0"]
    #[inline] pub fn test_muxen(&self) -> bool {
        self.muxen() != 0
    }

    #[doc="Sets the MUXEN field."]
    #[inline] pub fn set_muxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MBKEN"]
    #[inline] pub fn mbken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MBKEN != 0"]
    #[inline] pub fn test_mbken(&self) -> bool {
        self.mbken() != 0
    }

    #[doc="Sets the MBKEN field."]
    #[inline] pub fn set_mbken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bcr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Bcr1(other)
    }
}

impl ::core::fmt::Display for Bcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cclken() != 0 { try!(write!(f, " cclken"))}
        if self.cburstrw() != 0 { try!(write!(f, " cburstrw"))}
        if self.asyncwait() != 0 { try!(write!(f, " asyncwait"))}
        if self.extmod() != 0 { try!(write!(f, " extmod"))}
        if self.waiten() != 0 { try!(write!(f, " waiten"))}
        if self.wren() != 0 { try!(write!(f, " wren"))}
        if self.waitcfg() != 0 { try!(write!(f, " waitcfg"))}
        if self.waitpol() != 0 { try!(write!(f, " waitpol"))}
        if self.bursten() != 0 { try!(write!(f, " bursten"))}
        if self.faccen() != 0 { try!(write!(f, " faccen"))}
        if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
        if self.mtyp() != 0 { try!(write!(f, " mtyp=0x{:x}", self.mtyp()))}
        if self.muxen() != 0 { try!(write!(f, " muxen"))}
        if self.mbken() != 0 { try!(write!(f, " mbken"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash chip-select timing register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btr1(pub u32);
impl Btr1 {
    #[doc="ACCMOD"]
    #[inline] pub fn accmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ACCMOD != 0"]
    #[inline] pub fn test_accmod(&self) -> bool {
        self.accmod() != 0
    }

    #[doc="Sets the ACCMOD field."]
    #[inline] pub fn set_accmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DATLAT"]
    #[inline] pub fn datlat(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DATLAT != 0"]
    #[inline] pub fn test_datlat(&self) -> bool {
        self.datlat() != 0
    }

    #[doc="Sets the DATLAT field."]
    #[inline] pub fn set_datlat<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CLKDIV"]
    #[inline] pub fn clkdiv(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="BUSTURN"]
    #[inline] pub fn busturn(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if BUSTURN != 0"]
    #[inline] pub fn test_busturn(&self) -> bool {
        self.busturn() != 0
    }

    #[doc="Sets the BUSTURN field."]
    #[inline] pub fn set_busturn<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATAST"]
    #[inline] pub fn datast(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATAST != 0"]
    #[inline] pub fn test_datast(&self) -> bool {
        self.datast() != 0
    }

    #[doc="Sets the DATAST field."]
    #[inline] pub fn set_datast<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADDHLD"]
    #[inline] pub fn addhld(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ADDHLD != 0"]
    #[inline] pub fn test_addhld(&self) -> bool {
        self.addhld() != 0
    }

    #[doc="Sets the ADDHLD field."]
    #[inline] pub fn set_addhld<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADDSET"]
    #[inline] pub fn addset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADDSET != 0"]
    #[inline] pub fn test_addset(&self) -> bool {
        self.addset() != 0
    }

    #[doc="Sets the ADDSET field."]
    #[inline] pub fn set_addset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Btr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Btr1(other)
    }
}

impl ::core::fmt::Display for Btr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
        if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.busturn() != 0 { try!(write!(f, " busturn=0x{:x}", self.busturn()))}
        if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
        if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
        if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash chip-select control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr2(pub u32);
impl Bcr2 {
    #[doc="CBURSTRW"]
    #[inline] pub fn cburstrw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CBURSTRW != 0"]
    #[inline] pub fn test_cburstrw(&self) -> bool {
        self.cburstrw() != 0
    }

    #[doc="Sets the CBURSTRW field."]
    #[inline] pub fn set_cburstrw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ASYNCWAIT"]
    #[inline] pub fn asyncwait(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ASYNCWAIT != 0"]
    #[inline] pub fn test_asyncwait(&self) -> bool {
        self.asyncwait() != 0
    }

    #[doc="Sets the ASYNCWAIT field."]
    #[inline] pub fn set_asyncwait<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="EXTMOD"]
    #[inline] pub fn extmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if EXTMOD != 0"]
    #[inline] pub fn test_extmod(&self) -> bool {
        self.extmod() != 0
    }

    #[doc="Sets the EXTMOD field."]
    #[inline] pub fn set_extmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="WAITEN"]
    #[inline] pub fn waiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if WAITEN != 0"]
    #[inline] pub fn test_waiten(&self) -> bool {
        self.waiten() != 0
    }

    #[doc="Sets the WAITEN field."]
    #[inline] pub fn set_waiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="WREN"]
    #[inline] pub fn wren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WREN != 0"]
    #[inline] pub fn test_wren(&self) -> bool {
        self.wren() != 0
    }

    #[doc="Sets the WREN field."]
    #[inline] pub fn set_wren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="WAITCFG"]
    #[inline] pub fn waitcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WAITCFG != 0"]
    #[inline] pub fn test_waitcfg(&self) -> bool {
        self.waitcfg() != 0
    }

    #[doc="Sets the WAITCFG field."]
    #[inline] pub fn set_waitcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="WRAPMOD"]
    #[inline] pub fn wrapmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if WRAPMOD != 0"]
    #[inline] pub fn test_wrapmod(&self) -> bool {
        self.wrapmod() != 0
    }

    #[doc="Sets the WRAPMOD field."]
    #[inline] pub fn set_wrapmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="WAITPOL"]
    #[inline] pub fn waitpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if WAITPOL != 0"]
    #[inline] pub fn test_waitpol(&self) -> bool {
        self.waitpol() != 0
    }

    #[doc="Sets the WAITPOL field."]
    #[inline] pub fn set_waitpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BURSTEN"]
    #[inline] pub fn bursten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BURSTEN != 0"]
    #[inline] pub fn test_bursten(&self) -> bool {
        self.bursten() != 0
    }

    #[doc="Sets the BURSTEN field."]
    #[inline] pub fn set_bursten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FACCEN"]
    #[inline] pub fn faccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FACCEN != 0"]
    #[inline] pub fn test_faccen(&self) -> bool {
        self.faccen() != 0
    }

    #[doc="Sets the FACCEN field."]
    #[inline] pub fn set_faccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MWID"]
    #[inline] pub fn mwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if MWID != 0"]
    #[inline] pub fn test_mwid(&self) -> bool {
        self.mwid() != 0
    }

    #[doc="Sets the MWID field."]
    #[inline] pub fn set_mwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MTYP"]
    #[inline] pub fn mtyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MTYP != 0"]
    #[inline] pub fn test_mtyp(&self) -> bool {
        self.mtyp() != 0
    }

    #[doc="Sets the MTYP field."]
    #[inline] pub fn set_mtyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MUXEN"]
    #[inline] pub fn muxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUXEN != 0"]
    #[inline] pub fn test_muxen(&self) -> bool {
        self.muxen() != 0
    }

    #[doc="Sets the MUXEN field."]
    #[inline] pub fn set_muxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MBKEN"]
    #[inline] pub fn mbken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MBKEN != 0"]
    #[inline] pub fn test_mbken(&self) -> bool {
        self.mbken() != 0
    }

    #[doc="Sets the MBKEN field."]
    #[inline] pub fn set_mbken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bcr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Bcr2(other)
    }
}

impl ::core::fmt::Display for Bcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cburstrw() != 0 { try!(write!(f, " cburstrw"))}
        if self.asyncwait() != 0 { try!(write!(f, " asyncwait"))}
        if self.extmod() != 0 { try!(write!(f, " extmod"))}
        if self.waiten() != 0 { try!(write!(f, " waiten"))}
        if self.wren() != 0 { try!(write!(f, " wren"))}
        if self.waitcfg() != 0 { try!(write!(f, " waitcfg"))}
        if self.wrapmod() != 0 { try!(write!(f, " wrapmod"))}
        if self.waitpol() != 0 { try!(write!(f, " waitpol"))}
        if self.bursten() != 0 { try!(write!(f, " bursten"))}
        if self.faccen() != 0 { try!(write!(f, " faccen"))}
        if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
        if self.mtyp() != 0 { try!(write!(f, " mtyp=0x{:x}", self.mtyp()))}
        if self.muxen() != 0 { try!(write!(f, " muxen"))}
        if self.mbken() != 0 { try!(write!(f, " mbken"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash chip-select timing register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btr2(pub u32);
impl Btr2 {
    #[doc="ACCMOD"]
    #[inline] pub fn accmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ACCMOD != 0"]
    #[inline] pub fn test_accmod(&self) -> bool {
        self.accmod() != 0
    }

    #[doc="Sets the ACCMOD field."]
    #[inline] pub fn set_accmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DATLAT"]
    #[inline] pub fn datlat(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DATLAT != 0"]
    #[inline] pub fn test_datlat(&self) -> bool {
        self.datlat() != 0
    }

    #[doc="Sets the DATLAT field."]
    #[inline] pub fn set_datlat<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CLKDIV"]
    #[inline] pub fn clkdiv(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="BUSTURN"]
    #[inline] pub fn busturn(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if BUSTURN != 0"]
    #[inline] pub fn test_busturn(&self) -> bool {
        self.busturn() != 0
    }

    #[doc="Sets the BUSTURN field."]
    #[inline] pub fn set_busturn<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATAST"]
    #[inline] pub fn datast(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATAST != 0"]
    #[inline] pub fn test_datast(&self) -> bool {
        self.datast() != 0
    }

    #[doc="Sets the DATAST field."]
    #[inline] pub fn set_datast<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADDHLD"]
    #[inline] pub fn addhld(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ADDHLD != 0"]
    #[inline] pub fn test_addhld(&self) -> bool {
        self.addhld() != 0
    }

    #[doc="Sets the ADDHLD field."]
    #[inline] pub fn set_addhld<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADDSET"]
    #[inline] pub fn addset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADDSET != 0"]
    #[inline] pub fn test_addset(&self) -> bool {
        self.addset() != 0
    }

    #[doc="Sets the ADDSET field."]
    #[inline] pub fn set_addset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Btr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Btr2(other)
    }
}

impl ::core::fmt::Display for Btr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
        if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.busturn() != 0 { try!(write!(f, " busturn=0x{:x}", self.busturn()))}
        if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
        if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
        if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash chip-select control register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr3(pub u32);
impl Bcr3 {
    #[doc="CBURSTRW"]
    #[inline] pub fn cburstrw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CBURSTRW != 0"]
    #[inline] pub fn test_cburstrw(&self) -> bool {
        self.cburstrw() != 0
    }

    #[doc="Sets the CBURSTRW field."]
    #[inline] pub fn set_cburstrw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ASYNCWAIT"]
    #[inline] pub fn asyncwait(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ASYNCWAIT != 0"]
    #[inline] pub fn test_asyncwait(&self) -> bool {
        self.asyncwait() != 0
    }

    #[doc="Sets the ASYNCWAIT field."]
    #[inline] pub fn set_asyncwait<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="EXTMOD"]
    #[inline] pub fn extmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if EXTMOD != 0"]
    #[inline] pub fn test_extmod(&self) -> bool {
        self.extmod() != 0
    }

    #[doc="Sets the EXTMOD field."]
    #[inline] pub fn set_extmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="WAITEN"]
    #[inline] pub fn waiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if WAITEN != 0"]
    #[inline] pub fn test_waiten(&self) -> bool {
        self.waiten() != 0
    }

    #[doc="Sets the WAITEN field."]
    #[inline] pub fn set_waiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="WREN"]
    #[inline] pub fn wren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WREN != 0"]
    #[inline] pub fn test_wren(&self) -> bool {
        self.wren() != 0
    }

    #[doc="Sets the WREN field."]
    #[inline] pub fn set_wren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="WAITCFG"]
    #[inline] pub fn waitcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WAITCFG != 0"]
    #[inline] pub fn test_waitcfg(&self) -> bool {
        self.waitcfg() != 0
    }

    #[doc="Sets the WAITCFG field."]
    #[inline] pub fn set_waitcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="WRAPMOD"]
    #[inline] pub fn wrapmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if WRAPMOD != 0"]
    #[inline] pub fn test_wrapmod(&self) -> bool {
        self.wrapmod() != 0
    }

    #[doc="Sets the WRAPMOD field."]
    #[inline] pub fn set_wrapmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="WAITPOL"]
    #[inline] pub fn waitpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if WAITPOL != 0"]
    #[inline] pub fn test_waitpol(&self) -> bool {
        self.waitpol() != 0
    }

    #[doc="Sets the WAITPOL field."]
    #[inline] pub fn set_waitpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BURSTEN"]
    #[inline] pub fn bursten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BURSTEN != 0"]
    #[inline] pub fn test_bursten(&self) -> bool {
        self.bursten() != 0
    }

    #[doc="Sets the BURSTEN field."]
    #[inline] pub fn set_bursten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FACCEN"]
    #[inline] pub fn faccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FACCEN != 0"]
    #[inline] pub fn test_faccen(&self) -> bool {
        self.faccen() != 0
    }

    #[doc="Sets the FACCEN field."]
    #[inline] pub fn set_faccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MWID"]
    #[inline] pub fn mwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if MWID != 0"]
    #[inline] pub fn test_mwid(&self) -> bool {
        self.mwid() != 0
    }

    #[doc="Sets the MWID field."]
    #[inline] pub fn set_mwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MTYP"]
    #[inline] pub fn mtyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MTYP != 0"]
    #[inline] pub fn test_mtyp(&self) -> bool {
        self.mtyp() != 0
    }

    #[doc="Sets the MTYP field."]
    #[inline] pub fn set_mtyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MUXEN"]
    #[inline] pub fn muxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUXEN != 0"]
    #[inline] pub fn test_muxen(&self) -> bool {
        self.muxen() != 0
    }

    #[doc="Sets the MUXEN field."]
    #[inline] pub fn set_muxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MBKEN"]
    #[inline] pub fn mbken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MBKEN != 0"]
    #[inline] pub fn test_mbken(&self) -> bool {
        self.mbken() != 0
    }

    #[doc="Sets the MBKEN field."]
    #[inline] pub fn set_mbken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bcr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Bcr3(other)
    }
}

impl ::core::fmt::Display for Bcr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cburstrw() != 0 { try!(write!(f, " cburstrw"))}
        if self.asyncwait() != 0 { try!(write!(f, " asyncwait"))}
        if self.extmod() != 0 { try!(write!(f, " extmod"))}
        if self.waiten() != 0 { try!(write!(f, " waiten"))}
        if self.wren() != 0 { try!(write!(f, " wren"))}
        if self.waitcfg() != 0 { try!(write!(f, " waitcfg"))}
        if self.wrapmod() != 0 { try!(write!(f, " wrapmod"))}
        if self.waitpol() != 0 { try!(write!(f, " waitpol"))}
        if self.bursten() != 0 { try!(write!(f, " bursten"))}
        if self.faccen() != 0 { try!(write!(f, " faccen"))}
        if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
        if self.mtyp() != 0 { try!(write!(f, " mtyp=0x{:x}", self.mtyp()))}
        if self.muxen() != 0 { try!(write!(f, " muxen"))}
        if self.mbken() != 0 { try!(write!(f, " mbken"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash chip-select timing register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btr3(pub u32);
impl Btr3 {
    #[doc="ACCMOD"]
    #[inline] pub fn accmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ACCMOD != 0"]
    #[inline] pub fn test_accmod(&self) -> bool {
        self.accmod() != 0
    }

    #[doc="Sets the ACCMOD field."]
    #[inline] pub fn set_accmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DATLAT"]
    #[inline] pub fn datlat(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DATLAT != 0"]
    #[inline] pub fn test_datlat(&self) -> bool {
        self.datlat() != 0
    }

    #[doc="Sets the DATLAT field."]
    #[inline] pub fn set_datlat<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CLKDIV"]
    #[inline] pub fn clkdiv(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="BUSTURN"]
    #[inline] pub fn busturn(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if BUSTURN != 0"]
    #[inline] pub fn test_busturn(&self) -> bool {
        self.busturn() != 0
    }

    #[doc="Sets the BUSTURN field."]
    #[inline] pub fn set_busturn<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATAST"]
    #[inline] pub fn datast(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATAST != 0"]
    #[inline] pub fn test_datast(&self) -> bool {
        self.datast() != 0
    }

    #[doc="Sets the DATAST field."]
    #[inline] pub fn set_datast<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADDHLD"]
    #[inline] pub fn addhld(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ADDHLD != 0"]
    #[inline] pub fn test_addhld(&self) -> bool {
        self.addhld() != 0
    }

    #[doc="Sets the ADDHLD field."]
    #[inline] pub fn set_addhld<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADDSET"]
    #[inline] pub fn addset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADDSET != 0"]
    #[inline] pub fn test_addset(&self) -> bool {
        self.addset() != 0
    }

    #[doc="Sets the ADDSET field."]
    #[inline] pub fn set_addset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Btr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Btr3(other)
    }
}

impl ::core::fmt::Display for Btr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
        if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.busturn() != 0 { try!(write!(f, " busturn=0x{:x}", self.busturn()))}
        if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
        if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
        if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash chip-select control register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr4(pub u32);
impl Bcr4 {
    #[doc="CBURSTRW"]
    #[inline] pub fn cburstrw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CBURSTRW != 0"]
    #[inline] pub fn test_cburstrw(&self) -> bool {
        self.cburstrw() != 0
    }

    #[doc="Sets the CBURSTRW field."]
    #[inline] pub fn set_cburstrw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ASYNCWAIT"]
    #[inline] pub fn asyncwait(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ASYNCWAIT != 0"]
    #[inline] pub fn test_asyncwait(&self) -> bool {
        self.asyncwait() != 0
    }

    #[doc="Sets the ASYNCWAIT field."]
    #[inline] pub fn set_asyncwait<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="EXTMOD"]
    #[inline] pub fn extmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if EXTMOD != 0"]
    #[inline] pub fn test_extmod(&self) -> bool {
        self.extmod() != 0
    }

    #[doc="Sets the EXTMOD field."]
    #[inline] pub fn set_extmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="WAITEN"]
    #[inline] pub fn waiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if WAITEN != 0"]
    #[inline] pub fn test_waiten(&self) -> bool {
        self.waiten() != 0
    }

    #[doc="Sets the WAITEN field."]
    #[inline] pub fn set_waiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="WREN"]
    #[inline] pub fn wren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WREN != 0"]
    #[inline] pub fn test_wren(&self) -> bool {
        self.wren() != 0
    }

    #[doc="Sets the WREN field."]
    #[inline] pub fn set_wren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="WAITCFG"]
    #[inline] pub fn waitcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WAITCFG != 0"]
    #[inline] pub fn test_waitcfg(&self) -> bool {
        self.waitcfg() != 0
    }

    #[doc="Sets the WAITCFG field."]
    #[inline] pub fn set_waitcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="WRAPMOD"]
    #[inline] pub fn wrapmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if WRAPMOD != 0"]
    #[inline] pub fn test_wrapmod(&self) -> bool {
        self.wrapmod() != 0
    }

    #[doc="Sets the WRAPMOD field."]
    #[inline] pub fn set_wrapmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="WAITPOL"]
    #[inline] pub fn waitpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if WAITPOL != 0"]
    #[inline] pub fn test_waitpol(&self) -> bool {
        self.waitpol() != 0
    }

    #[doc="Sets the WAITPOL field."]
    #[inline] pub fn set_waitpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BURSTEN"]
    #[inline] pub fn bursten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BURSTEN != 0"]
    #[inline] pub fn test_bursten(&self) -> bool {
        self.bursten() != 0
    }

    #[doc="Sets the BURSTEN field."]
    #[inline] pub fn set_bursten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FACCEN"]
    #[inline] pub fn faccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FACCEN != 0"]
    #[inline] pub fn test_faccen(&self) -> bool {
        self.faccen() != 0
    }

    #[doc="Sets the FACCEN field."]
    #[inline] pub fn set_faccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MWID"]
    #[inline] pub fn mwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if MWID != 0"]
    #[inline] pub fn test_mwid(&self) -> bool {
        self.mwid() != 0
    }

    #[doc="Sets the MWID field."]
    #[inline] pub fn set_mwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MTYP"]
    #[inline] pub fn mtyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MTYP != 0"]
    #[inline] pub fn test_mtyp(&self) -> bool {
        self.mtyp() != 0
    }

    #[doc="Sets the MTYP field."]
    #[inline] pub fn set_mtyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MUXEN"]
    #[inline] pub fn muxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUXEN != 0"]
    #[inline] pub fn test_muxen(&self) -> bool {
        self.muxen() != 0
    }

    #[doc="Sets the MUXEN field."]
    #[inline] pub fn set_muxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MBKEN"]
    #[inline] pub fn mbken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MBKEN != 0"]
    #[inline] pub fn test_mbken(&self) -> bool {
        self.mbken() != 0
    }

    #[doc="Sets the MBKEN field."]
    #[inline] pub fn set_mbken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bcr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Bcr4(other)
    }
}

impl ::core::fmt::Display for Bcr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cburstrw() != 0 { try!(write!(f, " cburstrw"))}
        if self.asyncwait() != 0 { try!(write!(f, " asyncwait"))}
        if self.extmod() != 0 { try!(write!(f, " extmod"))}
        if self.waiten() != 0 { try!(write!(f, " waiten"))}
        if self.wren() != 0 { try!(write!(f, " wren"))}
        if self.waitcfg() != 0 { try!(write!(f, " waitcfg"))}
        if self.wrapmod() != 0 { try!(write!(f, " wrapmod"))}
        if self.waitpol() != 0 { try!(write!(f, " waitpol"))}
        if self.bursten() != 0 { try!(write!(f, " bursten"))}
        if self.faccen() != 0 { try!(write!(f, " faccen"))}
        if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
        if self.mtyp() != 0 { try!(write!(f, " mtyp=0x{:x}", self.mtyp()))}
        if self.muxen() != 0 { try!(write!(f, " muxen"))}
        if self.mbken() != 0 { try!(write!(f, " mbken"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash chip-select timing register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btr4(pub u32);
impl Btr4 {
    #[doc="ACCMOD"]
    #[inline] pub fn accmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ACCMOD != 0"]
    #[inline] pub fn test_accmod(&self) -> bool {
        self.accmod() != 0
    }

    #[doc="Sets the ACCMOD field."]
    #[inline] pub fn set_accmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DATLAT"]
    #[inline] pub fn datlat(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DATLAT != 0"]
    #[inline] pub fn test_datlat(&self) -> bool {
        self.datlat() != 0
    }

    #[doc="Sets the DATLAT field."]
    #[inline] pub fn set_datlat<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CLKDIV"]
    #[inline] pub fn clkdiv(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="BUSTURN"]
    #[inline] pub fn busturn(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if BUSTURN != 0"]
    #[inline] pub fn test_busturn(&self) -> bool {
        self.busturn() != 0
    }

    #[doc="Sets the BUSTURN field."]
    #[inline] pub fn set_busturn<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATAST"]
    #[inline] pub fn datast(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATAST != 0"]
    #[inline] pub fn test_datast(&self) -> bool {
        self.datast() != 0
    }

    #[doc="Sets the DATAST field."]
    #[inline] pub fn set_datast<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADDHLD"]
    #[inline] pub fn addhld(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ADDHLD != 0"]
    #[inline] pub fn test_addhld(&self) -> bool {
        self.addhld() != 0
    }

    #[doc="Sets the ADDHLD field."]
    #[inline] pub fn set_addhld<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADDSET"]
    #[inline] pub fn addset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADDSET != 0"]
    #[inline] pub fn test_addset(&self) -> bool {
        self.addset() != 0
    }

    #[doc="Sets the ADDSET field."]
    #[inline] pub fn set_addset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Btr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Btr4(other)
    }
}

impl ::core::fmt::Display for Btr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
        if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.busturn() != 0 { try!(write!(f, " busturn=0x{:x}", self.busturn()))}
        if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
        if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
        if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PC Card/NAND Flash control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcr2(pub u32);
impl Pcr2 {
    #[doc="ECCPS"]
    #[inline] pub fn eccps(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if ECCPS != 0"]
    #[inline] pub fn test_eccps(&self) -> bool {
        self.eccps() != 0
    }

    #[doc="Sets the ECCPS field."]
    #[inline] pub fn set_eccps<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TAR"]
    #[inline] pub fn tar(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
    }

    #[doc="Returns true if TAR != 0"]
    #[inline] pub fn test_tar(&self) -> bool {
        self.tar() != 0
    }

    #[doc="Sets the TAR field."]
    #[inline] pub fn set_tar<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TCLR"]
    #[inline] pub fn tclr(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
    }

    #[doc="Returns true if TCLR != 0"]
    #[inline] pub fn test_tclr(&self) -> bool {
        self.tclr() != 0
    }

    #[doc="Sets the TCLR field."]
    #[inline] pub fn set_tclr<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="ECCEN"]
    #[inline] pub fn eccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ECCEN != 0"]
    #[inline] pub fn test_eccen(&self) -> bool {
        self.eccen() != 0
    }

    #[doc="Sets the ECCEN field."]
    #[inline] pub fn set_eccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PWID"]
    #[inline] pub fn pwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PWID != 0"]
    #[inline] pub fn test_pwid(&self) -> bool {
        self.pwid() != 0
    }

    #[doc="Sets the PWID field."]
    #[inline] pub fn set_pwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PTYP"]
    #[inline] pub fn ptyp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTYP != 0"]
    #[inline] pub fn test_ptyp(&self) -> bool {
        self.ptyp() != 0
    }

    #[doc="Sets the PTYP field."]
    #[inline] pub fn set_ptyp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PBKEN"]
    #[inline] pub fn pbken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PBKEN != 0"]
    #[inline] pub fn test_pbken(&self) -> bool {
        self.pbken() != 0
    }

    #[doc="Sets the PBKEN field."]
    #[inline] pub fn set_pbken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="PWAITEN"]
    #[inline] pub fn pwaiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PWAITEN != 0"]
    #[inline] pub fn test_pwaiten(&self) -> bool {
        self.pwaiten() != 0
    }

    #[doc="Sets the PWAITEN field."]
    #[inline] pub fn set_pwaiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Pcr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Pcr2(other)
    }
}

impl ::core::fmt::Display for Pcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eccps() != 0 { try!(write!(f, " eccps=0x{:x}", self.eccps()))}
        if self.tar() != 0 { try!(write!(f, " tar=0x{:x}", self.tar()))}
        if self.tclr() != 0 { try!(write!(f, " tclr=0x{:x}", self.tclr()))}
        if self.eccen() != 0 { try!(write!(f, " eccen"))}
        if self.pwid() != 0 { try!(write!(f, " pwid=0x{:x}", self.pwid()))}
        if self.ptyp() != 0 { try!(write!(f, " ptyp"))}
        if self.pbken() != 0 { try!(write!(f, " pbken"))}
        if self.pwaiten() != 0 { try!(write!(f, " pwaiten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO status and interrupt register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr2(pub u32);
impl Sr2 {
    #[doc="FEMPT"]
    #[inline] pub fn fempt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FEMPT != 0"]
    #[inline] pub fn test_fempt(&self) -> bool {
        self.fempt() != 0
    }

    #[doc="Sets the FEMPT field."]
    #[inline] pub fn set_fempt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IFEN"]
    #[inline] pub fn ifen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IFEN != 0"]
    #[inline] pub fn test_ifen(&self) -> bool {
        self.ifen() != 0
    }

    #[doc="Sets the IFEN field."]
    #[inline] pub fn set_ifen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="ILEN"]
    #[inline] pub fn ilen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ILEN != 0"]
    #[inline] pub fn test_ilen(&self) -> bool {
        self.ilen() != 0
    }

    #[doc="Sets the ILEN field."]
    #[inline] pub fn set_ilen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IREN"]
    #[inline] pub fn iren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IFS"]
    #[inline] pub fn ifs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IFS != 0"]
    #[inline] pub fn test_ifs(&self) -> bool {
        self.ifs() != 0
    }

    #[doc="Sets the IFS field."]
    #[inline] pub fn set_ifs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ILS"]
    #[inline] pub fn ils(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ILS != 0"]
    #[inline] pub fn test_ils(&self) -> bool {
        self.ils() != 0
    }

    #[doc="Sets the ILS field."]
    #[inline] pub fn set_ils<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IRS"]
    #[inline] pub fn irs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IRS != 0"]
    #[inline] pub fn test_irs(&self) -> bool {
        self.irs() != 0
    }

    #[doc="Sets the IRS field."]
    #[inline] pub fn set_irs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.fempt() != 0 { try!(write!(f, " fempt"))}
        if self.ifen() != 0 { try!(write!(f, " ifen"))}
        if self.ilen() != 0 { try!(write!(f, " ilen"))}
        if self.iren() != 0 { try!(write!(f, " iren"))}
        if self.ifs() != 0 { try!(write!(f, " ifs"))}
        if self.ils() != 0 { try!(write!(f, " ils"))}
        if self.irs() != 0 { try!(write!(f, " irs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Common memory space timing register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmem2(pub u32);
impl Pmem2 {
    #[doc="MEMHIZx"]
    #[inline] pub fn memhizx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MEMHIZX != 0"]
    #[inline] pub fn test_memhizx(&self) -> bool {
        self.memhizx() != 0
    }

    #[doc="Sets the MEMHIZX field."]
    #[inline] pub fn set_memhizx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="MEMHOLDx"]
    #[inline] pub fn memholdx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MEMHOLDX != 0"]
    #[inline] pub fn test_memholdx(&self) -> bool {
        self.memholdx() != 0
    }

    #[doc="Sets the MEMHOLDX field."]
    #[inline] pub fn set_memholdx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MEMWAITx"]
    #[inline] pub fn memwaitx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if MEMWAITX != 0"]
    #[inline] pub fn test_memwaitx(&self) -> bool {
        self.memwaitx() != 0
    }

    #[doc="Sets the MEMWAITX field."]
    #[inline] pub fn set_memwaitx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MEMSETx"]
    #[inline] pub fn memsetx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MEMSETX != 0"]
    #[inline] pub fn test_memsetx(&self) -> bool {
        self.memsetx() != 0
    }

    #[doc="Sets the MEMSETX field."]
    #[inline] pub fn set_memsetx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pmem2 {
    #[inline]
    fn from(other: u32) -> Self {
         Pmem2(other)
    }
}

impl ::core::fmt::Display for Pmem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.memhizx() != 0 { try!(write!(f, " memhizx=0x{:x}", self.memhizx()))}
        if self.memholdx() != 0 { try!(write!(f, " memholdx=0x{:x}", self.memholdx()))}
        if self.memwaitx() != 0 { try!(write!(f, " memwaitx=0x{:x}", self.memwaitx()))}
        if self.memsetx() != 0 { try!(write!(f, " memsetx=0x{:x}", self.memsetx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Attribute memory space timing register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Patt2(pub u32);
impl Patt2 {
    #[doc="ATTHIZx"]
    #[inline] pub fn atthizx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if ATTHIZX != 0"]
    #[inline] pub fn test_atthizx(&self) -> bool {
        self.atthizx() != 0
    }

    #[doc="Sets the ATTHIZX field."]
    #[inline] pub fn set_atthizx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ATTHOLDx"]
    #[inline] pub fn attholdx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if ATTHOLDX != 0"]
    #[inline] pub fn test_attholdx(&self) -> bool {
        self.attholdx() != 0
    }

    #[doc="Sets the ATTHOLDX field."]
    #[inline] pub fn set_attholdx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ATTWAITx"]
    #[inline] pub fn attwaitx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if ATTWAITX != 0"]
    #[inline] pub fn test_attwaitx(&self) -> bool {
        self.attwaitx() != 0
    }

    #[doc="Sets the ATTWAITX field."]
    #[inline] pub fn set_attwaitx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ATTSETx"]
    #[inline] pub fn attsetx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ATTSETX != 0"]
    #[inline] pub fn test_attsetx(&self) -> bool {
        self.attsetx() != 0
    }

    #[doc="Sets the ATTSETX field."]
    #[inline] pub fn set_attsetx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Patt2 {
    #[inline]
    fn from(other: u32) -> Self {
         Patt2(other)
    }
}

impl ::core::fmt::Display for Patt2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Patt2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.atthizx() != 0 { try!(write!(f, " atthizx=0x{:x}", self.atthizx()))}
        if self.attholdx() != 0 { try!(write!(f, " attholdx=0x{:x}", self.attholdx()))}
        if self.attwaitx() != 0 { try!(write!(f, " attwaitx=0x{:x}", self.attwaitx()))}
        if self.attsetx() != 0 { try!(write!(f, " attsetx=0x{:x}", self.attsetx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ECC result register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eccr2(pub u32);
impl Eccr2 {
    #[doc="ECCx"]
    #[inline] pub fn eccx(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ECCX != 0"]
    #[inline] pub fn test_eccx(&self) -> bool {
        self.eccx() != 0
    }

    #[doc="Sets the ECCX field."]
    #[inline] pub fn set_eccx<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Eccr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Eccr2(other)
    }
}

impl ::core::fmt::Display for Eccr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eccr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PC Card/NAND Flash control register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcr3(pub u32);
impl Pcr3 {
    #[doc="ECCPS"]
    #[inline] pub fn eccps(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if ECCPS != 0"]
    #[inline] pub fn test_eccps(&self) -> bool {
        self.eccps() != 0
    }

    #[doc="Sets the ECCPS field."]
    #[inline] pub fn set_eccps<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TAR"]
    #[inline] pub fn tar(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
    }

    #[doc="Returns true if TAR != 0"]
    #[inline] pub fn test_tar(&self) -> bool {
        self.tar() != 0
    }

    #[doc="Sets the TAR field."]
    #[inline] pub fn set_tar<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TCLR"]
    #[inline] pub fn tclr(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
    }

    #[doc="Returns true if TCLR != 0"]
    #[inline] pub fn test_tclr(&self) -> bool {
        self.tclr() != 0
    }

    #[doc="Sets the TCLR field."]
    #[inline] pub fn set_tclr<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="ECCEN"]
    #[inline] pub fn eccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ECCEN != 0"]
    #[inline] pub fn test_eccen(&self) -> bool {
        self.eccen() != 0
    }

    #[doc="Sets the ECCEN field."]
    #[inline] pub fn set_eccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PWID"]
    #[inline] pub fn pwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PWID != 0"]
    #[inline] pub fn test_pwid(&self) -> bool {
        self.pwid() != 0
    }

    #[doc="Sets the PWID field."]
    #[inline] pub fn set_pwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PTYP"]
    #[inline] pub fn ptyp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTYP != 0"]
    #[inline] pub fn test_ptyp(&self) -> bool {
        self.ptyp() != 0
    }

    #[doc="Sets the PTYP field."]
    #[inline] pub fn set_ptyp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PBKEN"]
    #[inline] pub fn pbken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PBKEN != 0"]
    #[inline] pub fn test_pbken(&self) -> bool {
        self.pbken() != 0
    }

    #[doc="Sets the PBKEN field."]
    #[inline] pub fn set_pbken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="PWAITEN"]
    #[inline] pub fn pwaiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PWAITEN != 0"]
    #[inline] pub fn test_pwaiten(&self) -> bool {
        self.pwaiten() != 0
    }

    #[doc="Sets the PWAITEN field."]
    #[inline] pub fn set_pwaiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Pcr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Pcr3(other)
    }
}

impl ::core::fmt::Display for Pcr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eccps() != 0 { try!(write!(f, " eccps=0x{:x}", self.eccps()))}
        if self.tar() != 0 { try!(write!(f, " tar=0x{:x}", self.tar()))}
        if self.tclr() != 0 { try!(write!(f, " tclr=0x{:x}", self.tclr()))}
        if self.eccen() != 0 { try!(write!(f, " eccen"))}
        if self.pwid() != 0 { try!(write!(f, " pwid=0x{:x}", self.pwid()))}
        if self.ptyp() != 0 { try!(write!(f, " ptyp"))}
        if self.pbken() != 0 { try!(write!(f, " pbken"))}
        if self.pwaiten() != 0 { try!(write!(f, " pwaiten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO status and interrupt register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr3(pub u32);
impl Sr3 {
    #[doc="FEMPT"]
    #[inline] pub fn fempt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FEMPT != 0"]
    #[inline] pub fn test_fempt(&self) -> bool {
        self.fempt() != 0
    }

    #[doc="Sets the FEMPT field."]
    #[inline] pub fn set_fempt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IFEN"]
    #[inline] pub fn ifen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IFEN != 0"]
    #[inline] pub fn test_ifen(&self) -> bool {
        self.ifen() != 0
    }

    #[doc="Sets the IFEN field."]
    #[inline] pub fn set_ifen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="ILEN"]
    #[inline] pub fn ilen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ILEN != 0"]
    #[inline] pub fn test_ilen(&self) -> bool {
        self.ilen() != 0
    }

    #[doc="Sets the ILEN field."]
    #[inline] pub fn set_ilen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IREN"]
    #[inline] pub fn iren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IFS"]
    #[inline] pub fn ifs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IFS != 0"]
    #[inline] pub fn test_ifs(&self) -> bool {
        self.ifs() != 0
    }

    #[doc="Sets the IFS field."]
    #[inline] pub fn set_ifs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ILS"]
    #[inline] pub fn ils(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ILS != 0"]
    #[inline] pub fn test_ils(&self) -> bool {
        self.ils() != 0
    }

    #[doc="Sets the ILS field."]
    #[inline] pub fn set_ils<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IRS"]
    #[inline] pub fn irs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IRS != 0"]
    #[inline] pub fn test_irs(&self) -> bool {
        self.irs() != 0
    }

    #[doc="Sets the IRS field."]
    #[inline] pub fn set_irs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Sr3(other)
    }
}

impl ::core::fmt::Display for Sr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fempt() != 0 { try!(write!(f, " fempt"))}
        if self.ifen() != 0 { try!(write!(f, " ifen"))}
        if self.ilen() != 0 { try!(write!(f, " ilen"))}
        if self.iren() != 0 { try!(write!(f, " iren"))}
        if self.ifs() != 0 { try!(write!(f, " ifs"))}
        if self.ils() != 0 { try!(write!(f, " ils"))}
        if self.irs() != 0 { try!(write!(f, " irs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Common memory space timing register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmem3(pub u32);
impl Pmem3 {
    #[doc="MEMHIZx"]
    #[inline] pub fn memhizx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MEMHIZX != 0"]
    #[inline] pub fn test_memhizx(&self) -> bool {
        self.memhizx() != 0
    }

    #[doc="Sets the MEMHIZX field."]
    #[inline] pub fn set_memhizx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="MEMHOLDx"]
    #[inline] pub fn memholdx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MEMHOLDX != 0"]
    #[inline] pub fn test_memholdx(&self) -> bool {
        self.memholdx() != 0
    }

    #[doc="Sets the MEMHOLDX field."]
    #[inline] pub fn set_memholdx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MEMWAITx"]
    #[inline] pub fn memwaitx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if MEMWAITX != 0"]
    #[inline] pub fn test_memwaitx(&self) -> bool {
        self.memwaitx() != 0
    }

    #[doc="Sets the MEMWAITX field."]
    #[inline] pub fn set_memwaitx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MEMSETx"]
    #[inline] pub fn memsetx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MEMSETX != 0"]
    #[inline] pub fn test_memsetx(&self) -> bool {
        self.memsetx() != 0
    }

    #[doc="Sets the MEMSETX field."]
    #[inline] pub fn set_memsetx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pmem3 {
    #[inline]
    fn from(other: u32) -> Self {
         Pmem3(other)
    }
}

impl ::core::fmt::Display for Pmem3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmem3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.memhizx() != 0 { try!(write!(f, " memhizx=0x{:x}", self.memhizx()))}
        if self.memholdx() != 0 { try!(write!(f, " memholdx=0x{:x}", self.memholdx()))}
        if self.memwaitx() != 0 { try!(write!(f, " memwaitx=0x{:x}", self.memwaitx()))}
        if self.memsetx() != 0 { try!(write!(f, " memsetx=0x{:x}", self.memsetx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Attribute memory space timing register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Patt3(pub u32);
impl Patt3 {
    #[doc="ATTHIZx"]
    #[inline] pub fn atthizx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if ATTHIZX != 0"]
    #[inline] pub fn test_atthizx(&self) -> bool {
        self.atthizx() != 0
    }

    #[doc="Sets the ATTHIZX field."]
    #[inline] pub fn set_atthizx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ATTHOLDx"]
    #[inline] pub fn attholdx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if ATTHOLDX != 0"]
    #[inline] pub fn test_attholdx(&self) -> bool {
        self.attholdx() != 0
    }

    #[doc="Sets the ATTHOLDX field."]
    #[inline] pub fn set_attholdx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ATTWAITx"]
    #[inline] pub fn attwaitx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if ATTWAITX != 0"]
    #[inline] pub fn test_attwaitx(&self) -> bool {
        self.attwaitx() != 0
    }

    #[doc="Sets the ATTWAITX field."]
    #[inline] pub fn set_attwaitx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ATTSETx"]
    #[inline] pub fn attsetx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ATTSETX != 0"]
    #[inline] pub fn test_attsetx(&self) -> bool {
        self.attsetx() != 0
    }

    #[doc="Sets the ATTSETX field."]
    #[inline] pub fn set_attsetx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Patt3 {
    #[inline]
    fn from(other: u32) -> Self {
         Patt3(other)
    }
}

impl ::core::fmt::Display for Patt3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Patt3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.atthizx() != 0 { try!(write!(f, " atthizx=0x{:x}", self.atthizx()))}
        if self.attholdx() != 0 { try!(write!(f, " attholdx=0x{:x}", self.attholdx()))}
        if self.attwaitx() != 0 { try!(write!(f, " attwaitx=0x{:x}", self.attwaitx()))}
        if self.attsetx() != 0 { try!(write!(f, " attsetx=0x{:x}", self.attsetx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ECC result register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eccr3(pub u32);
impl Eccr3 {
    #[doc="ECCx"]
    #[inline] pub fn eccx(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ECCX != 0"]
    #[inline] pub fn test_eccx(&self) -> bool {
        self.eccx() != 0
    }

    #[doc="Sets the ECCX field."]
    #[inline] pub fn set_eccx<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Eccr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Eccr3(other)
    }
}

impl ::core::fmt::Display for Eccr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eccr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PC Card/NAND Flash control register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcr4(pub u32);
impl Pcr4 {
    #[doc="ECCPS"]
    #[inline] pub fn eccps(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if ECCPS != 0"]
    #[inline] pub fn test_eccps(&self) -> bool {
        self.eccps() != 0
    }

    #[doc="Sets the ECCPS field."]
    #[inline] pub fn set_eccps<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TAR"]
    #[inline] pub fn tar(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
    }

    #[doc="Returns true if TAR != 0"]
    #[inline] pub fn test_tar(&self) -> bool {
        self.tar() != 0
    }

    #[doc="Sets the TAR field."]
    #[inline] pub fn set_tar<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TCLR"]
    #[inline] pub fn tclr(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
    }

    #[doc="Returns true if TCLR != 0"]
    #[inline] pub fn test_tclr(&self) -> bool {
        self.tclr() != 0
    }

    #[doc="Sets the TCLR field."]
    #[inline] pub fn set_tclr<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="ECCEN"]
    #[inline] pub fn eccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ECCEN != 0"]
    #[inline] pub fn test_eccen(&self) -> bool {
        self.eccen() != 0
    }

    #[doc="Sets the ECCEN field."]
    #[inline] pub fn set_eccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PWID"]
    #[inline] pub fn pwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PWID != 0"]
    #[inline] pub fn test_pwid(&self) -> bool {
        self.pwid() != 0
    }

    #[doc="Sets the PWID field."]
    #[inline] pub fn set_pwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PTYP"]
    #[inline] pub fn ptyp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTYP != 0"]
    #[inline] pub fn test_ptyp(&self) -> bool {
        self.ptyp() != 0
    }

    #[doc="Sets the PTYP field."]
    #[inline] pub fn set_ptyp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PBKEN"]
    #[inline] pub fn pbken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PBKEN != 0"]
    #[inline] pub fn test_pbken(&self) -> bool {
        self.pbken() != 0
    }

    #[doc="Sets the PBKEN field."]
    #[inline] pub fn set_pbken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="PWAITEN"]
    #[inline] pub fn pwaiten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PWAITEN != 0"]
    #[inline] pub fn test_pwaiten(&self) -> bool {
        self.pwaiten() != 0
    }

    #[doc="Sets the PWAITEN field."]
    #[inline] pub fn set_pwaiten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Pcr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Pcr4(other)
    }
}

impl ::core::fmt::Display for Pcr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eccps() != 0 { try!(write!(f, " eccps=0x{:x}", self.eccps()))}
        if self.tar() != 0 { try!(write!(f, " tar=0x{:x}", self.tar()))}
        if self.tclr() != 0 { try!(write!(f, " tclr=0x{:x}", self.tclr()))}
        if self.eccen() != 0 { try!(write!(f, " eccen"))}
        if self.pwid() != 0 { try!(write!(f, " pwid=0x{:x}", self.pwid()))}
        if self.ptyp() != 0 { try!(write!(f, " ptyp"))}
        if self.pbken() != 0 { try!(write!(f, " pbken"))}
        if self.pwaiten() != 0 { try!(write!(f, " pwaiten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO status and interrupt register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr4(pub u32);
impl Sr4 {
    #[doc="FEMPT"]
    #[inline] pub fn fempt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FEMPT != 0"]
    #[inline] pub fn test_fempt(&self) -> bool {
        self.fempt() != 0
    }

    #[doc="Sets the FEMPT field."]
    #[inline] pub fn set_fempt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IFEN"]
    #[inline] pub fn ifen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IFEN != 0"]
    #[inline] pub fn test_ifen(&self) -> bool {
        self.ifen() != 0
    }

    #[doc="Sets the IFEN field."]
    #[inline] pub fn set_ifen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="ILEN"]
    #[inline] pub fn ilen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ILEN != 0"]
    #[inline] pub fn test_ilen(&self) -> bool {
        self.ilen() != 0
    }

    #[doc="Sets the ILEN field."]
    #[inline] pub fn set_ilen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IREN"]
    #[inline] pub fn iren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IFS"]
    #[inline] pub fn ifs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IFS != 0"]
    #[inline] pub fn test_ifs(&self) -> bool {
        self.ifs() != 0
    }

    #[doc="Sets the IFS field."]
    #[inline] pub fn set_ifs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ILS"]
    #[inline] pub fn ils(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ILS != 0"]
    #[inline] pub fn test_ils(&self) -> bool {
        self.ils() != 0
    }

    #[doc="Sets the ILS field."]
    #[inline] pub fn set_ils<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IRS"]
    #[inline] pub fn irs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IRS != 0"]
    #[inline] pub fn test_irs(&self) -> bool {
        self.irs() != 0
    }

    #[doc="Sets the IRS field."]
    #[inline] pub fn set_irs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Sr4(other)
    }
}

impl ::core::fmt::Display for Sr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fempt() != 0 { try!(write!(f, " fempt"))}
        if self.ifen() != 0 { try!(write!(f, " ifen"))}
        if self.ilen() != 0 { try!(write!(f, " ilen"))}
        if self.iren() != 0 { try!(write!(f, " iren"))}
        if self.ifs() != 0 { try!(write!(f, " ifs"))}
        if self.ils() != 0 { try!(write!(f, " ils"))}
        if self.irs() != 0 { try!(write!(f, " irs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Common memory space timing register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmem4(pub u32);
impl Pmem4 {
    #[doc="MEMHIZx"]
    #[inline] pub fn memhizx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MEMHIZX != 0"]
    #[inline] pub fn test_memhizx(&self) -> bool {
        self.memhizx() != 0
    }

    #[doc="Sets the MEMHIZX field."]
    #[inline] pub fn set_memhizx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="MEMHOLDx"]
    #[inline] pub fn memholdx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MEMHOLDX != 0"]
    #[inline] pub fn test_memholdx(&self) -> bool {
        self.memholdx() != 0
    }

    #[doc="Sets the MEMHOLDX field."]
    #[inline] pub fn set_memholdx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MEMWAITx"]
    #[inline] pub fn memwaitx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if MEMWAITX != 0"]
    #[inline] pub fn test_memwaitx(&self) -> bool {
        self.memwaitx() != 0
    }

    #[doc="Sets the MEMWAITX field."]
    #[inline] pub fn set_memwaitx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MEMSETx"]
    #[inline] pub fn memsetx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MEMSETX != 0"]
    #[inline] pub fn test_memsetx(&self) -> bool {
        self.memsetx() != 0
    }

    #[doc="Sets the MEMSETX field."]
    #[inline] pub fn set_memsetx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pmem4 {
    #[inline]
    fn from(other: u32) -> Self {
         Pmem4(other)
    }
}

impl ::core::fmt::Display for Pmem4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmem4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.memhizx() != 0 { try!(write!(f, " memhizx=0x{:x}", self.memhizx()))}
        if self.memholdx() != 0 { try!(write!(f, " memholdx=0x{:x}", self.memholdx()))}
        if self.memwaitx() != 0 { try!(write!(f, " memwaitx=0x{:x}", self.memwaitx()))}
        if self.memsetx() != 0 { try!(write!(f, " memsetx=0x{:x}", self.memsetx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Attribute memory space timing register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Patt4(pub u32);
impl Patt4 {
    #[doc="ATTHIZx"]
    #[inline] pub fn atthizx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if ATTHIZX != 0"]
    #[inline] pub fn test_atthizx(&self) -> bool {
        self.atthizx() != 0
    }

    #[doc="Sets the ATTHIZX field."]
    #[inline] pub fn set_atthizx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ATTHOLDx"]
    #[inline] pub fn attholdx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if ATTHOLDX != 0"]
    #[inline] pub fn test_attholdx(&self) -> bool {
        self.attholdx() != 0
    }

    #[doc="Sets the ATTHOLDX field."]
    #[inline] pub fn set_attholdx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ATTWAITx"]
    #[inline] pub fn attwaitx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if ATTWAITX != 0"]
    #[inline] pub fn test_attwaitx(&self) -> bool {
        self.attwaitx() != 0
    }

    #[doc="Sets the ATTWAITX field."]
    #[inline] pub fn set_attwaitx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ATTSETx"]
    #[inline] pub fn attsetx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ATTSETX != 0"]
    #[inline] pub fn test_attsetx(&self) -> bool {
        self.attsetx() != 0
    }

    #[doc="Sets the ATTSETX field."]
    #[inline] pub fn set_attsetx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Patt4 {
    #[inline]
    fn from(other: u32) -> Self {
         Patt4(other)
    }
}

impl ::core::fmt::Display for Patt4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Patt4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.atthizx() != 0 { try!(write!(f, " atthizx=0x{:x}", self.atthizx()))}
        if self.attholdx() != 0 { try!(write!(f, " attholdx=0x{:x}", self.attholdx()))}
        if self.attwaitx() != 0 { try!(write!(f, " attwaitx=0x{:x}", self.attwaitx()))}
        if self.attsetx() != 0 { try!(write!(f, " attsetx=0x{:x}", self.attsetx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O space timing register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pio4(pub u32);
impl Pio4 {
    #[doc="IOHIZx"]
    #[inline] pub fn iohizx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IOHIZX != 0"]
    #[inline] pub fn test_iohizx(&self) -> bool {
        self.iohizx() != 0
    }

    #[doc="Sets the IOHIZX field."]
    #[inline] pub fn set_iohizx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="IOHOLDx"]
    #[inline] pub fn ioholdx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IOHOLDX != 0"]
    #[inline] pub fn test_ioholdx(&self) -> bool {
        self.ioholdx() != 0
    }

    #[doc="Sets the IOHOLDX field."]
    #[inline] pub fn set_ioholdx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IOWAITx"]
    #[inline] pub fn iowaitx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IOWAITX != 0"]
    #[inline] pub fn test_iowaitx(&self) -> bool {
        self.iowaitx() != 0
    }

    #[doc="Sets the IOWAITX field."]
    #[inline] pub fn set_iowaitx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IOSETx"]
    #[inline] pub fn iosetx(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IOSETX != 0"]
    #[inline] pub fn test_iosetx(&self) -> bool {
        self.iosetx() != 0
    }

    #[doc="Sets the IOSETX field."]
    #[inline] pub fn set_iosetx<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pio4 {
    #[inline]
    fn from(other: u32) -> Self {
         Pio4(other)
    }
}

impl ::core::fmt::Display for Pio4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pio4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iohizx() != 0 { try!(write!(f, " iohizx=0x{:x}", self.iohizx()))}
        if self.ioholdx() != 0 { try!(write!(f, " ioholdx=0x{:x}", self.ioholdx()))}
        if self.iowaitx() != 0 { try!(write!(f, " iowaitx=0x{:x}", self.iowaitx()))}
        if self.iosetx() != 0 { try!(write!(f, " iosetx=0x{:x}", self.iosetx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash write timing registers 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bwtr1(pub u32);
impl Bwtr1 {
    #[doc="ACCMOD"]
    #[inline] pub fn accmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ACCMOD != 0"]
    #[inline] pub fn test_accmod(&self) -> bool {
        self.accmod() != 0
    }

    #[doc="Sets the ACCMOD field."]
    #[inline] pub fn set_accmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DATLAT"]
    #[inline] pub fn datlat(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DATLAT != 0"]
    #[inline] pub fn test_datlat(&self) -> bool {
        self.datlat() != 0
    }

    #[doc="Sets the DATLAT field."]
    #[inline] pub fn set_datlat<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CLKDIV"]
    #[inline] pub fn clkdiv(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="DATAST"]
    #[inline] pub fn datast(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATAST != 0"]
    #[inline] pub fn test_datast(&self) -> bool {
        self.datast() != 0
    }

    #[doc="Sets the DATAST field."]
    #[inline] pub fn set_datast<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADDHLD"]
    #[inline] pub fn addhld(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ADDHLD != 0"]
    #[inline] pub fn test_addhld(&self) -> bool {
        self.addhld() != 0
    }

    #[doc="Sets the ADDHLD field."]
    #[inline] pub fn set_addhld<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADDSET"]
    #[inline] pub fn addset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADDSET != 0"]
    #[inline] pub fn test_addset(&self) -> bool {
        self.addset() != 0
    }

    #[doc="Sets the ADDSET field."]
    #[inline] pub fn set_addset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bwtr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Bwtr1(other)
    }
}

impl ::core::fmt::Display for Bwtr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bwtr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
        if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
        if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
        if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash write timing registers 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bwtr2(pub u32);
impl Bwtr2 {
    #[doc="ACCMOD"]
    #[inline] pub fn accmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ACCMOD != 0"]
    #[inline] pub fn test_accmod(&self) -> bool {
        self.accmod() != 0
    }

    #[doc="Sets the ACCMOD field."]
    #[inline] pub fn set_accmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DATLAT"]
    #[inline] pub fn datlat(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DATLAT != 0"]
    #[inline] pub fn test_datlat(&self) -> bool {
        self.datlat() != 0
    }

    #[doc="Sets the DATLAT field."]
    #[inline] pub fn set_datlat<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CLKDIV"]
    #[inline] pub fn clkdiv(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="DATAST"]
    #[inline] pub fn datast(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATAST != 0"]
    #[inline] pub fn test_datast(&self) -> bool {
        self.datast() != 0
    }

    #[doc="Sets the DATAST field."]
    #[inline] pub fn set_datast<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADDHLD"]
    #[inline] pub fn addhld(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ADDHLD != 0"]
    #[inline] pub fn test_addhld(&self) -> bool {
        self.addhld() != 0
    }

    #[doc="Sets the ADDHLD field."]
    #[inline] pub fn set_addhld<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADDSET"]
    #[inline] pub fn addset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADDSET != 0"]
    #[inline] pub fn test_addset(&self) -> bool {
        self.addset() != 0
    }

    #[doc="Sets the ADDSET field."]
    #[inline] pub fn set_addset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bwtr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Bwtr2(other)
    }
}

impl ::core::fmt::Display for Bwtr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bwtr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
        if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
        if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
        if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash write timing registers 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bwtr3(pub u32);
impl Bwtr3 {
    #[doc="ACCMOD"]
    #[inline] pub fn accmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ACCMOD != 0"]
    #[inline] pub fn test_accmod(&self) -> bool {
        self.accmod() != 0
    }

    #[doc="Sets the ACCMOD field."]
    #[inline] pub fn set_accmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DATLAT"]
    #[inline] pub fn datlat(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DATLAT != 0"]
    #[inline] pub fn test_datlat(&self) -> bool {
        self.datlat() != 0
    }

    #[doc="Sets the DATLAT field."]
    #[inline] pub fn set_datlat<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CLKDIV"]
    #[inline] pub fn clkdiv(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="DATAST"]
    #[inline] pub fn datast(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATAST != 0"]
    #[inline] pub fn test_datast(&self) -> bool {
        self.datast() != 0
    }

    #[doc="Sets the DATAST field."]
    #[inline] pub fn set_datast<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADDHLD"]
    #[inline] pub fn addhld(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ADDHLD != 0"]
    #[inline] pub fn test_addhld(&self) -> bool {
        self.addhld() != 0
    }

    #[doc="Sets the ADDHLD field."]
    #[inline] pub fn set_addhld<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADDSET"]
    #[inline] pub fn addset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADDSET != 0"]
    #[inline] pub fn test_addset(&self) -> bool {
        self.addset() != 0
    }

    #[doc="Sets the ADDSET field."]
    #[inline] pub fn set_addset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bwtr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Bwtr3(other)
    }
}

impl ::core::fmt::Display for Bwtr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bwtr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
        if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
        if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
        if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM/NOR-Flash write timing registers 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bwtr4(pub u32);
impl Bwtr4 {
    #[doc="ACCMOD"]
    #[inline] pub fn accmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ACCMOD != 0"]
    #[inline] pub fn test_accmod(&self) -> bool {
        self.accmod() != 0
    }

    #[doc="Sets the ACCMOD field."]
    #[inline] pub fn set_accmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DATLAT"]
    #[inline] pub fn datlat(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if DATLAT != 0"]
    #[inline] pub fn test_datlat(&self) -> bool {
        self.datlat() != 0
    }

    #[doc="Sets the DATLAT field."]
    #[inline] pub fn set_datlat<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CLKDIV"]
    #[inline] pub fn clkdiv(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="DATAST"]
    #[inline] pub fn datast(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATAST != 0"]
    #[inline] pub fn test_datast(&self) -> bool {
        self.datast() != 0
    }

    #[doc="Sets the DATAST field."]
    #[inline] pub fn set_datast<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADDHLD"]
    #[inline] pub fn addhld(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ADDHLD != 0"]
    #[inline] pub fn test_addhld(&self) -> bool {
        self.addhld() != 0
    }

    #[doc="Sets the ADDHLD field."]
    #[inline] pub fn set_addhld<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADDSET"]
    #[inline] pub fn addset(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADDSET != 0"]
    #[inline] pub fn test_addset(&self) -> bool {
        self.addset() != 0
    }

    #[doc="Sets the ADDSET field."]
    #[inline] pub fn set_addset<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bwtr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Bwtr4(other)
    }
}

impl ::core::fmt::Display for Bwtr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bwtr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
        if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
        if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
        if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDRAM Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdcr1(pub u32);
impl Sdcr1 {
    #[doc="Number of column address bits"]
    #[inline] pub fn nc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if NC != 0"]
    #[inline] pub fn test_nc(&self) -> bool {
        self.nc() != 0
    }

    #[doc="Sets the NC field."]
    #[inline] pub fn set_nc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of row address bits"]
    #[inline] pub fn nr(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if NR != 0"]
    #[inline] pub fn test_nr(&self) -> bool {
        self.nr() != 0
    }

    #[doc="Sets the NR field."]
    #[inline] pub fn set_nr<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Memory data bus width"]
    #[inline] pub fn mwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if MWID != 0"]
    #[inline] pub fn test_mwid(&self) -> bool {
        self.mwid() != 0
    }

    #[doc="Sets the MWID field."]
    #[inline] pub fn set_mwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Number of internal banks"]
    #[inline] pub fn nb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NB != 0"]
    #[inline] pub fn test_nb(&self) -> bool {
        self.nb() != 0
    }

    #[doc="Sets the NB field."]
    #[inline] pub fn set_nb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="CAS latency"]
    #[inline] pub fn cas(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3) as u8) } // [8:7]
    }

    #[doc="Returns true if CAS != 0"]
    #[inline] pub fn test_cas(&self) -> bool {
        self.cas() != 0
    }

    #[doc="Sets the CAS field."]
    #[inline] pub fn set_cas<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Write protection"]
    #[inline] pub fn wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if WP != 0"]
    #[inline] pub fn test_wp(&self) -> bool {
        self.wp() != 0
    }

    #[doc="Sets the WP field."]
    #[inline] pub fn set_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SDRAM clock configuration"]
    #[inline] pub fn sdclk(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if SDCLK != 0"]
    #[inline] pub fn test_sdclk(&self) -> bool {
        self.sdclk() != 0
    }

    #[doc="Sets the SDCLK field."]
    #[inline] pub fn set_sdclk<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Burst read"]
    #[inline] pub fn rburst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if RBURST != 0"]
    #[inline] pub fn test_rburst(&self) -> bool {
        self.rburst() != 0
    }

    #[doc="Sets the RBURST field."]
    #[inline] pub fn set_rburst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Read pipe"]
    #[inline] pub fn rpipe(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if RPIPE != 0"]
    #[inline] pub fn test_rpipe(&self) -> bool {
        self.rpipe() != 0
    }

    #[doc="Sets the RPIPE field."]
    #[inline] pub fn set_rpipe<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Sdcr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sdcr1(other)
    }
}

impl ::core::fmt::Display for Sdcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nc() != 0 { try!(write!(f, " nc=0x{:x}", self.nc()))}
        if self.nr() != 0 { try!(write!(f, " nr=0x{:x}", self.nr()))}
        if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
        if self.nb() != 0 { try!(write!(f, " nb"))}
        if self.cas() != 0 { try!(write!(f, " cas=0x{:x}", self.cas()))}
        if self.wp() != 0 { try!(write!(f, " wp"))}
        if self.sdclk() != 0 { try!(write!(f, " sdclk=0x{:x}", self.sdclk()))}
        if self.rburst() != 0 { try!(write!(f, " rburst"))}
        if self.rpipe() != 0 { try!(write!(f, " rpipe=0x{:x}", self.rpipe()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDRAM Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdcr2(pub u32);
impl Sdcr2 {
    #[doc="Number of column address bits"]
    #[inline] pub fn nc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if NC != 0"]
    #[inline] pub fn test_nc(&self) -> bool {
        self.nc() != 0
    }

    #[doc="Sets the NC field."]
    #[inline] pub fn set_nc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of row address bits"]
    #[inline] pub fn nr(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if NR != 0"]
    #[inline] pub fn test_nr(&self) -> bool {
        self.nr() != 0
    }

    #[doc="Sets the NR field."]
    #[inline] pub fn set_nr<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Memory data bus width"]
    #[inline] pub fn mwid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if MWID != 0"]
    #[inline] pub fn test_mwid(&self) -> bool {
        self.mwid() != 0
    }

    #[doc="Sets the MWID field."]
    #[inline] pub fn set_mwid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Number of internal banks"]
    #[inline] pub fn nb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NB != 0"]
    #[inline] pub fn test_nb(&self) -> bool {
        self.nb() != 0
    }

    #[doc="Sets the NB field."]
    #[inline] pub fn set_nb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="CAS latency"]
    #[inline] pub fn cas(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3) as u8) } // [8:7]
    }

    #[doc="Returns true if CAS != 0"]
    #[inline] pub fn test_cas(&self) -> bool {
        self.cas() != 0
    }

    #[doc="Sets the CAS field."]
    #[inline] pub fn set_cas<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Write protection"]
    #[inline] pub fn wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if WP != 0"]
    #[inline] pub fn test_wp(&self) -> bool {
        self.wp() != 0
    }

    #[doc="Sets the WP field."]
    #[inline] pub fn set_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SDRAM clock configuration"]
    #[inline] pub fn sdclk(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if SDCLK != 0"]
    #[inline] pub fn test_sdclk(&self) -> bool {
        self.sdclk() != 0
    }

    #[doc="Sets the SDCLK field."]
    #[inline] pub fn set_sdclk<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Burst read"]
    #[inline] pub fn rburst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if RBURST != 0"]
    #[inline] pub fn test_rburst(&self) -> bool {
        self.rburst() != 0
    }

    #[doc="Sets the RBURST field."]
    #[inline] pub fn set_rburst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Read pipe"]
    #[inline] pub fn rpipe(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if RPIPE != 0"]
    #[inline] pub fn test_rpipe(&self) -> bool {
        self.rpipe() != 0
    }

    #[doc="Sets the RPIPE field."]
    #[inline] pub fn set_rpipe<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Sdcr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sdcr2(other)
    }
}

impl ::core::fmt::Display for Sdcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nc() != 0 { try!(write!(f, " nc=0x{:x}", self.nc()))}
        if self.nr() != 0 { try!(write!(f, " nr=0x{:x}", self.nr()))}
        if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
        if self.nb() != 0 { try!(write!(f, " nb"))}
        if self.cas() != 0 { try!(write!(f, " cas=0x{:x}", self.cas()))}
        if self.wp() != 0 { try!(write!(f, " wp"))}
        if self.sdclk() != 0 { try!(write!(f, " sdclk=0x{:x}", self.sdclk()))}
        if self.rburst() != 0 { try!(write!(f, " rburst"))}
        if self.rpipe() != 0 { try!(write!(f, " rpipe=0x{:x}", self.rpipe()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDRAM Timing register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdtr1(pub u32);
impl Sdtr1 {
    #[doc="Load Mode Register to Active"]
    #[inline] pub fn tmrd(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if TMRD != 0"]
    #[inline] pub fn test_tmrd(&self) -> bool {
        self.tmrd() != 0
    }

    #[doc="Sets the TMRD field."]
    #[inline] pub fn set_tmrd<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Exit self-refresh delay"]
    #[inline] pub fn txsr(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if TXSR != 0"]
    #[inline] pub fn test_txsr(&self) -> bool {
        self.txsr() != 0
    }

    #[doc="Sets the TXSR field."]
    #[inline] pub fn set_txsr<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Self refresh time"]
    #[inline] pub fn tras(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if TRAS != 0"]
    #[inline] pub fn test_tras(&self) -> bool {
        self.tras() != 0
    }

    #[doc="Sets the TRAS field."]
    #[inline] pub fn set_tras<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Row cycle delay"]
    #[inline] pub fn trc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if TRC != 0"]
    #[inline] pub fn test_trc(&self) -> bool {
        self.trc() != 0
    }

    #[doc="Sets the TRC field."]
    #[inline] pub fn set_trc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recovery delay"]
    #[inline] pub fn twr(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if TWR != 0"]
    #[inline] pub fn test_twr(&self) -> bool {
        self.twr() != 0
    }

    #[doc="Sets the TWR field."]
    #[inline] pub fn set_twr<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Row precharge delay"]
    #[inline] pub fn trp(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if TRP != 0"]
    #[inline] pub fn test_trp(&self) -> bool {
        self.trp() != 0
    }

    #[doc="Sets the TRP field."]
    #[inline] pub fn set_trp<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Row to column delay"]
    #[inline] pub fn trcd(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if TRCD != 0"]
    #[inline] pub fn test_trcd(&self) -> bool {
        self.trcd() != 0
    }

    #[doc="Sets the TRCD field."]
    #[inline] pub fn set_trcd<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Sdtr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sdtr1(other)
    }
}

impl ::core::fmt::Display for Sdtr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdtr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tmrd() != 0 { try!(write!(f, " tmrd=0x{:x}", self.tmrd()))}
        if self.txsr() != 0 { try!(write!(f, " txsr=0x{:x}", self.txsr()))}
        if self.tras() != 0 { try!(write!(f, " tras=0x{:x}", self.tras()))}
        if self.trc() != 0 { try!(write!(f, " trc=0x{:x}", self.trc()))}
        if self.twr() != 0 { try!(write!(f, " twr=0x{:x}", self.twr()))}
        if self.trp() != 0 { try!(write!(f, " trp=0x{:x}", self.trp()))}
        if self.trcd() != 0 { try!(write!(f, " trcd=0x{:x}", self.trcd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDRAM Timing register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdtr2(pub u32);
impl Sdtr2 {
    #[doc="Load Mode Register to Active"]
    #[inline] pub fn tmrd(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if TMRD != 0"]
    #[inline] pub fn test_tmrd(&self) -> bool {
        self.tmrd() != 0
    }

    #[doc="Sets the TMRD field."]
    #[inline] pub fn set_tmrd<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Exit self-refresh delay"]
    #[inline] pub fn txsr(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if TXSR != 0"]
    #[inline] pub fn test_txsr(&self) -> bool {
        self.txsr() != 0
    }

    #[doc="Sets the TXSR field."]
    #[inline] pub fn set_txsr<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Self refresh time"]
    #[inline] pub fn tras(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if TRAS != 0"]
    #[inline] pub fn test_tras(&self) -> bool {
        self.tras() != 0
    }

    #[doc="Sets the TRAS field."]
    #[inline] pub fn set_tras<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Row cycle delay"]
    #[inline] pub fn trc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if TRC != 0"]
    #[inline] pub fn test_trc(&self) -> bool {
        self.trc() != 0
    }

    #[doc="Sets the TRC field."]
    #[inline] pub fn set_trc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recovery delay"]
    #[inline] pub fn twr(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if TWR != 0"]
    #[inline] pub fn test_twr(&self) -> bool {
        self.twr() != 0
    }

    #[doc="Sets the TWR field."]
    #[inline] pub fn set_twr<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Row precharge delay"]
    #[inline] pub fn trp(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if TRP != 0"]
    #[inline] pub fn test_trp(&self) -> bool {
        self.trp() != 0
    }

    #[doc="Sets the TRP field."]
    #[inline] pub fn set_trp<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Row to column delay"]
    #[inline] pub fn trcd(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if TRCD != 0"]
    #[inline] pub fn test_trcd(&self) -> bool {
        self.trcd() != 0
    }

    #[doc="Sets the TRCD field."]
    #[inline] pub fn set_trcd<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Sdtr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sdtr2(other)
    }
}

impl ::core::fmt::Display for Sdtr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdtr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tmrd() != 0 { try!(write!(f, " tmrd=0x{:x}", self.tmrd()))}
        if self.txsr() != 0 { try!(write!(f, " txsr=0x{:x}", self.txsr()))}
        if self.tras() != 0 { try!(write!(f, " tras=0x{:x}", self.tras()))}
        if self.trc() != 0 { try!(write!(f, " trc=0x{:x}", self.trc()))}
        if self.twr() != 0 { try!(write!(f, " twr=0x{:x}", self.twr()))}
        if self.trp() != 0 { try!(write!(f, " trp=0x{:x}", self.trp()))}
        if self.trcd() != 0 { try!(write!(f, " trcd=0x{:x}", self.trcd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDRAM Command Mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdcmr(pub u32);
impl Sdcmr {
    #[doc="Command mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command target bank 2"]
    #[inline] pub fn ctb2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTB2 != 0"]
    #[inline] pub fn test_ctb2(&self) -> bool {
        self.ctb2() != 0
    }

    #[doc="Sets the CTB2 field."]
    #[inline] pub fn set_ctb2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Command target bank 1"]
    #[inline] pub fn ctb1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTB1 != 0"]
    #[inline] pub fn test_ctb1(&self) -> bool {
        self.ctb1() != 0
    }

    #[doc="Sets the CTB1 field."]
    #[inline] pub fn set_ctb1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Number of Auto-refresh"]
    #[inline] pub fn nrfs(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0xf) as u8) } // [8:5]
    }

    #[doc="Returns true if NRFS != 0"]
    #[inline] pub fn test_nrfs(&self) -> bool {
        self.nrfs() != 0
    }

    #[doc="Sets the NRFS field."]
    #[inline] pub fn set_nrfs<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Mode Register definition"]
    #[inline] pub fn mrd(&self) -> ::bobbin_bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1fff) as u16) } // [21:9]
    }

    #[doc="Returns true if MRD != 0"]
    #[inline] pub fn test_mrd(&self) -> bool {
        self.mrd() != 0
    }

    #[doc="Sets the MRD field."]
    #[inline] pub fn set_mrd<V: Into<::bobbin_bits::U13>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Sdcmr {
    #[inline]
    fn from(other: u32) -> Self {
         Sdcmr(other)
    }
}

impl ::core::fmt::Display for Sdcmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdcmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.ctb2() != 0 { try!(write!(f, " ctb2"))}
        if self.ctb1() != 0 { try!(write!(f, " ctb1"))}
        if self.nrfs() != 0 { try!(write!(f, " nrfs=0x{:x}", self.nrfs()))}
        if self.mrd() != 0 { try!(write!(f, " mrd=0x{:x}", self.mrd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDRAM Refresh Timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdrtr(pub u32);
impl Sdrtr {
    #[doc="Clear Refresh error flag"]
    #[inline] pub fn cre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CRE != 0"]
    #[inline] pub fn test_cre(&self) -> bool {
        self.cre() != 0
    }

    #[doc="Sets the CRE field."]
    #[inline] pub fn set_cre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Refresh Timer Count"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1fff) as u16) } // [13:1]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U13>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RES Interrupt Enable"]
    #[inline] pub fn reie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if REIE != 0"]
    #[inline] pub fn test_reie(&self) -> bool {
        self.reie() != 0
    }

    #[doc="Sets the REIE field."]
    #[inline] pub fn set_reie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Sdrtr {
    #[inline]
    fn from(other: u32) -> Self {
         Sdrtr(other)
    }
}

impl ::core::fmt::Display for Sdrtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdrtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cre() != 0 { try!(write!(f, " cre"))}
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        if self.reie() != 0 { try!(write!(f, " reie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDRAM Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdsr(pub u32);
impl Sdsr {
    #[doc="Refresh error flag"]
    #[inline] pub fn re(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RE != 0"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Sets the RE field."]
    #[inline] pub fn set_re<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status Mode for Bank 1"]
    #[inline] pub fn modes1(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if MODES1 != 0"]
    #[inline] pub fn test_modes1(&self) -> bool {
        self.modes1() != 0
    }

    #[doc="Sets the MODES1 field."]
    #[inline] pub fn set_modes1<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Status Mode for Bank 2"]
    #[inline] pub fn modes2(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if MODES2 != 0"]
    #[inline] pub fn test_modes2(&self) -> bool {
        self.modes2() != 0
    }

    #[doc="Sets the MODES2 field."]
    #[inline] pub fn set_modes2<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Busy status"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Sdsr {
    #[inline]
    fn from(other: u32) -> Self {
         Sdsr(other)
    }
}

impl ::core::fmt::Display for Sdsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.modes1() != 0 { try!(write!(f, " modes1=0x{:x}", self.modes1()))}
        if self.modes2() != 0 { try!(write!(f, " modes2=0x{:x}", self.modes2()))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

