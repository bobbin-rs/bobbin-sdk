::bobbin_mcu::periph!( SAI1, Sai1, SAI1_PERIPH, SaiPeriph, SAI1_OWNED, SAI1_REF_COUNT, 0x40015400, 0x00, 0x23);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SAI Peripheral"]
pub struct SaiPeriph(pub usize); 

impl SaiPeriph {
    #[doc="Get the GCR Register."]
    #[inline] pub fn gcr_reg(&self) -> ::bobbin_mcu::register::Register<Gcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gcr, 0x0)
    }

    #[doc="Get the *mut pointer for the GCR register."]
    #[inline] pub fn gcr_mut(&self) -> *mut Gcr { 
        self.gcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the GCR register."]
    #[inline] pub fn gcr_ptr(&self) -> *const Gcr { 
        self.gcr_reg().ptr()
    }

    #[doc="Read the GCR register."]
    #[inline] pub fn gcr(&self) -> Gcr { 
        self.gcr_reg().read()
    }

    #[doc="Write the GCR register."]
    #[inline] pub fn write_gcr(&self, value: Gcr) -> &Self { 
        self.gcr_reg().write(value);
        self
    }

    #[doc="Set the GCR register."]
    #[inline] pub fn set_gcr<F: FnOnce(Gcr) -> Gcr>(&self, f: F) -> &Self {
        self.gcr_reg().set(f);
        self
    }

    #[doc="Modify the GCR register."]
    #[inline] pub fn with_gcr<F: FnOnce(Gcr) -> Gcr>(&self, f: F) -> &Self {
        self.gcr_reg().with(f);
        self
    }

    #[doc="Get the BCR1 Register."]
    #[inline] pub fn bcr1_reg(&self) -> ::bobbin_mcu::register::Register<Bcr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bcr1, 0x24)
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

    #[doc="Get the BCR2 Register."]
    #[inline] pub fn bcr2_reg(&self) -> ::bobbin_mcu::register::Register<Bcr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bcr2, 0x28)
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

    #[doc="Get the BFRCR Register."]
    #[inline] pub fn bfrcr_reg(&self) -> ::bobbin_mcu::register::Register<Bfrcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bfrcr, 0x2c)
    }

    #[doc="Get the *mut pointer for the BFRCR register."]
    #[inline] pub fn bfrcr_mut(&self) -> *mut Bfrcr { 
        self.bfrcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BFRCR register."]
    #[inline] pub fn bfrcr_ptr(&self) -> *const Bfrcr { 
        self.bfrcr_reg().ptr()
    }

    #[doc="Read the BFRCR register."]
    #[inline] pub fn bfrcr(&self) -> Bfrcr { 
        self.bfrcr_reg().read()
    }

    #[doc="Write the BFRCR register."]
    #[inline] pub fn write_bfrcr(&self, value: Bfrcr) -> &Self { 
        self.bfrcr_reg().write(value);
        self
    }

    #[doc="Set the BFRCR register."]
    #[inline] pub fn set_bfrcr<F: FnOnce(Bfrcr) -> Bfrcr>(&self, f: F) -> &Self {
        self.bfrcr_reg().set(f);
        self
    }

    #[doc="Modify the BFRCR register."]
    #[inline] pub fn with_bfrcr<F: FnOnce(Bfrcr) -> Bfrcr>(&self, f: F) -> &Self {
        self.bfrcr_reg().with(f);
        self
    }

    #[doc="Get the BSLOTR Register."]
    #[inline] pub fn bslotr_reg(&self) -> ::bobbin_mcu::register::Register<Bslotr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bslotr, 0x30)
    }

    #[doc="Get the *mut pointer for the BSLOTR register."]
    #[inline] pub fn bslotr_mut(&self) -> *mut Bslotr { 
        self.bslotr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BSLOTR register."]
    #[inline] pub fn bslotr_ptr(&self) -> *const Bslotr { 
        self.bslotr_reg().ptr()
    }

    #[doc="Read the BSLOTR register."]
    #[inline] pub fn bslotr(&self) -> Bslotr { 
        self.bslotr_reg().read()
    }

    #[doc="Write the BSLOTR register."]
    #[inline] pub fn write_bslotr(&self, value: Bslotr) -> &Self { 
        self.bslotr_reg().write(value);
        self
    }

    #[doc="Set the BSLOTR register."]
    #[inline] pub fn set_bslotr<F: FnOnce(Bslotr) -> Bslotr>(&self, f: F) -> &Self {
        self.bslotr_reg().set(f);
        self
    }

    #[doc="Modify the BSLOTR register."]
    #[inline] pub fn with_bslotr<F: FnOnce(Bslotr) -> Bslotr>(&self, f: F) -> &Self {
        self.bslotr_reg().with(f);
        self
    }

    #[doc="Get the BIM Register."]
    #[inline] pub fn bim_reg(&self) -> ::bobbin_mcu::register::Register<Bim> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bim, 0x34)
    }

    #[doc="Get the *mut pointer for the BIM register."]
    #[inline] pub fn bim_mut(&self) -> *mut Bim { 
        self.bim_reg().ptr()
    }

    #[doc="Get the *const pointer for the BIM register."]
    #[inline] pub fn bim_ptr(&self) -> *const Bim { 
        self.bim_reg().ptr()
    }

    #[doc="Read the BIM register."]
    #[inline] pub fn bim(&self) -> Bim { 
        self.bim_reg().read()
    }

    #[doc="Write the BIM register."]
    #[inline] pub fn write_bim(&self, value: Bim) -> &Self { 
        self.bim_reg().write(value);
        self
    }

    #[doc="Set the BIM register."]
    #[inline] pub fn set_bim<F: FnOnce(Bim) -> Bim>(&self, f: F) -> &Self {
        self.bim_reg().set(f);
        self
    }

    #[doc="Modify the BIM register."]
    #[inline] pub fn with_bim<F: FnOnce(Bim) -> Bim>(&self, f: F) -> &Self {
        self.bim_reg().with(f);
        self
    }

    #[doc="Get the BSR Register."]
    #[inline] pub fn bsr_reg(&self) -> ::bobbin_mcu::register::Register<Bsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bsr, 0x38)
    }

    #[doc="Get the *mut pointer for the BSR register."]
    #[inline] pub fn bsr_mut(&self) -> *mut Bsr { 
        self.bsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BSR register."]
    #[inline] pub fn bsr_ptr(&self) -> *const Bsr { 
        self.bsr_reg().ptr()
    }

    #[doc="Read the BSR register."]
    #[inline] pub fn bsr(&self) -> Bsr { 
        self.bsr_reg().read()
    }

    #[doc="Get the BCLRFR Register."]
    #[inline] pub fn bclrfr_reg(&self) -> ::bobbin_mcu::register::Register<Bclrfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bclrfr, 0x3c)
    }

    #[doc="Get the *mut pointer for the BCLRFR register."]
    #[inline] pub fn bclrfr_mut(&self) -> *mut Bclrfr { 
        self.bclrfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BCLRFR register."]
    #[inline] pub fn bclrfr_ptr(&self) -> *const Bclrfr { 
        self.bclrfr_reg().ptr()
    }

    #[doc="Write the BCLRFR register."]
    #[inline] pub fn write_bclrfr(&self, value: Bclrfr) -> &Self { 
        self.bclrfr_reg().write(value);
        self
    }

    #[doc="Set the BCLRFR register."]
    #[inline] pub fn set_bclrfr<F: FnOnce(Bclrfr) -> Bclrfr>(&self, f: F) -> &Self {
        self.bclrfr_reg().set(f);
        self
    }

    #[doc="Get the BDR Register."]
    #[inline] pub fn bdr_reg(&self) -> ::bobbin_mcu::register::Register<Bdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdr, 0x40)
    }

    #[doc="Get the *mut pointer for the BDR register."]
    #[inline] pub fn bdr_mut(&self) -> *mut Bdr { 
        self.bdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDR register."]
    #[inline] pub fn bdr_ptr(&self) -> *const Bdr { 
        self.bdr_reg().ptr()
    }

    #[doc="Read the BDR register."]
    #[inline] pub fn bdr(&self) -> Bdr { 
        self.bdr_reg().read()
    }

    #[doc="Write the BDR register."]
    #[inline] pub fn write_bdr(&self, value: Bdr) -> &Self { 
        self.bdr_reg().write(value);
        self
    }

    #[doc="Set the BDR register."]
    #[inline] pub fn set_bdr<F: FnOnce(Bdr) -> Bdr>(&self, f: F) -> &Self {
        self.bdr_reg().set(f);
        self
    }

    #[doc="Modify the BDR register."]
    #[inline] pub fn with_bdr<F: FnOnce(Bdr) -> Bdr>(&self, f: F) -> &Self {
        self.bdr_reg().with(f);
        self
    }

    #[doc="Get the ACR1 Register."]
    #[inline] pub fn acr1_reg(&self) -> ::bobbin_mcu::register::Register<Acr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Acr1, 0x4)
    }

    #[doc="Get the *mut pointer for the ACR1 register."]
    #[inline] pub fn acr1_mut(&self) -> *mut Acr1 { 
        self.acr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACR1 register."]
    #[inline] pub fn acr1_ptr(&self) -> *const Acr1 { 
        self.acr1_reg().ptr()
    }

    #[doc="Read the ACR1 register."]
    #[inline] pub fn acr1(&self) -> Acr1 { 
        self.acr1_reg().read()
    }

    #[doc="Write the ACR1 register."]
    #[inline] pub fn write_acr1(&self, value: Acr1) -> &Self { 
        self.acr1_reg().write(value);
        self
    }

    #[doc="Set the ACR1 register."]
    #[inline] pub fn set_acr1<F: FnOnce(Acr1) -> Acr1>(&self, f: F) -> &Self {
        self.acr1_reg().set(f);
        self
    }

    #[doc="Modify the ACR1 register."]
    #[inline] pub fn with_acr1<F: FnOnce(Acr1) -> Acr1>(&self, f: F) -> &Self {
        self.acr1_reg().with(f);
        self
    }

    #[doc="Get the ACR2 Register."]
    #[inline] pub fn acr2_reg(&self) -> ::bobbin_mcu::register::Register<Acr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Acr2, 0x8)
    }

    #[doc="Get the *mut pointer for the ACR2 register."]
    #[inline] pub fn acr2_mut(&self) -> *mut Acr2 { 
        self.acr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACR2 register."]
    #[inline] pub fn acr2_ptr(&self) -> *const Acr2 { 
        self.acr2_reg().ptr()
    }

    #[doc="Read the ACR2 register."]
    #[inline] pub fn acr2(&self) -> Acr2 { 
        self.acr2_reg().read()
    }

    #[doc="Write the ACR2 register."]
    #[inline] pub fn write_acr2(&self, value: Acr2) -> &Self { 
        self.acr2_reg().write(value);
        self
    }

    #[doc="Set the ACR2 register."]
    #[inline] pub fn set_acr2<F: FnOnce(Acr2) -> Acr2>(&self, f: F) -> &Self {
        self.acr2_reg().set(f);
        self
    }

    #[doc="Modify the ACR2 register."]
    #[inline] pub fn with_acr2<F: FnOnce(Acr2) -> Acr2>(&self, f: F) -> &Self {
        self.acr2_reg().with(f);
        self
    }

    #[doc="Get the AFRCR Register."]
    #[inline] pub fn afrcr_reg(&self) -> ::bobbin_mcu::register::Register<Afrcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Afrcr, 0xc)
    }

    #[doc="Get the *mut pointer for the AFRCR register."]
    #[inline] pub fn afrcr_mut(&self) -> *mut Afrcr { 
        self.afrcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AFRCR register."]
    #[inline] pub fn afrcr_ptr(&self) -> *const Afrcr { 
        self.afrcr_reg().ptr()
    }

    #[doc="Read the AFRCR register."]
    #[inline] pub fn afrcr(&self) -> Afrcr { 
        self.afrcr_reg().read()
    }

    #[doc="Write the AFRCR register."]
    #[inline] pub fn write_afrcr(&self, value: Afrcr) -> &Self { 
        self.afrcr_reg().write(value);
        self
    }

    #[doc="Set the AFRCR register."]
    #[inline] pub fn set_afrcr<F: FnOnce(Afrcr) -> Afrcr>(&self, f: F) -> &Self {
        self.afrcr_reg().set(f);
        self
    }

    #[doc="Modify the AFRCR register."]
    #[inline] pub fn with_afrcr<F: FnOnce(Afrcr) -> Afrcr>(&self, f: F) -> &Self {
        self.afrcr_reg().with(f);
        self
    }

    #[doc="Get the ASLOTR Register."]
    #[inline] pub fn aslotr_reg(&self) -> ::bobbin_mcu::register::Register<Aslotr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Aslotr, 0x10)
    }

    #[doc="Get the *mut pointer for the ASLOTR register."]
    #[inline] pub fn aslotr_mut(&self) -> *mut Aslotr { 
        self.aslotr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ASLOTR register."]
    #[inline] pub fn aslotr_ptr(&self) -> *const Aslotr { 
        self.aslotr_reg().ptr()
    }

    #[doc="Read the ASLOTR register."]
    #[inline] pub fn aslotr(&self) -> Aslotr { 
        self.aslotr_reg().read()
    }

    #[doc="Write the ASLOTR register."]
    #[inline] pub fn write_aslotr(&self, value: Aslotr) -> &Self { 
        self.aslotr_reg().write(value);
        self
    }

    #[doc="Set the ASLOTR register."]
    #[inline] pub fn set_aslotr<F: FnOnce(Aslotr) -> Aslotr>(&self, f: F) -> &Self {
        self.aslotr_reg().set(f);
        self
    }

    #[doc="Modify the ASLOTR register."]
    #[inline] pub fn with_aslotr<F: FnOnce(Aslotr) -> Aslotr>(&self, f: F) -> &Self {
        self.aslotr_reg().with(f);
        self
    }

    #[doc="Get the AIM Register."]
    #[inline] pub fn aim_reg(&self) -> ::bobbin_mcu::register::Register<Aim> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Aim, 0x14)
    }

    #[doc="Get the *mut pointer for the AIM register."]
    #[inline] pub fn aim_mut(&self) -> *mut Aim { 
        self.aim_reg().ptr()
    }

    #[doc="Get the *const pointer for the AIM register."]
    #[inline] pub fn aim_ptr(&self) -> *const Aim { 
        self.aim_reg().ptr()
    }

    #[doc="Read the AIM register."]
    #[inline] pub fn aim(&self) -> Aim { 
        self.aim_reg().read()
    }

    #[doc="Write the AIM register."]
    #[inline] pub fn write_aim(&self, value: Aim) -> &Self { 
        self.aim_reg().write(value);
        self
    }

    #[doc="Set the AIM register."]
    #[inline] pub fn set_aim<F: FnOnce(Aim) -> Aim>(&self, f: F) -> &Self {
        self.aim_reg().set(f);
        self
    }

    #[doc="Modify the AIM register."]
    #[inline] pub fn with_aim<F: FnOnce(Aim) -> Aim>(&self, f: F) -> &Self {
        self.aim_reg().with(f);
        self
    }

    #[doc="Get the ASR Register."]
    #[inline] pub fn asr_reg(&self) -> ::bobbin_mcu::register::Register<Asr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Asr, 0x18)
    }

    #[doc="Get the *mut pointer for the ASR register."]
    #[inline] pub fn asr_mut(&self) -> *mut Asr { 
        self.asr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ASR register."]
    #[inline] pub fn asr_ptr(&self) -> *const Asr { 
        self.asr_reg().ptr()
    }

    #[doc="Read the ASR register."]
    #[inline] pub fn asr(&self) -> Asr { 
        self.asr_reg().read()
    }

    #[doc="Get the ACLRFR Register."]
    #[inline] pub fn aclrfr_reg(&self) -> ::bobbin_mcu::register::Register<Aclrfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Aclrfr, 0x1c)
    }

    #[doc="Get the *mut pointer for the ACLRFR register."]
    #[inline] pub fn aclrfr_mut(&self) -> *mut Aclrfr { 
        self.aclrfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACLRFR register."]
    #[inline] pub fn aclrfr_ptr(&self) -> *const Aclrfr { 
        self.aclrfr_reg().ptr()
    }

    #[doc="Write the ACLRFR register."]
    #[inline] pub fn write_aclrfr(&self, value: Aclrfr) -> &Self { 
        self.aclrfr_reg().write(value);
        self
    }

    #[doc="Set the ACLRFR register."]
    #[inline] pub fn set_aclrfr<F: FnOnce(Aclrfr) -> Aclrfr>(&self, f: F) -> &Self {
        self.aclrfr_reg().set(f);
        self
    }

    #[doc="Get the ADR Register."]
    #[inline] pub fn adr_reg(&self) -> ::bobbin_mcu::register::Register<Adr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Adr, 0x20)
    }

    #[doc="Get the *mut pointer for the ADR register."]
    #[inline] pub fn adr_mut(&self) -> *mut Adr { 
        self.adr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADR register."]
    #[inline] pub fn adr_ptr(&self) -> *const Adr { 
        self.adr_reg().ptr()
    }

    #[doc="Read the ADR register."]
    #[inline] pub fn adr(&self) -> Adr { 
        self.adr_reg().read()
    }

    #[doc="Write the ADR register."]
    #[inline] pub fn write_adr(&self, value: Adr) -> &Self { 
        self.adr_reg().write(value);
        self
    }

    #[doc="Set the ADR register."]
    #[inline] pub fn set_adr<F: FnOnce(Adr) -> Adr>(&self, f: F) -> &Self {
        self.adr_reg().set(f);
        self
    }

    #[doc="Modify the ADR register."]
    #[inline] pub fn with_adr<F: FnOnce(Adr) -> Adr>(&self, f: F) -> &Self {
        self.adr_reg().with(f);
        self
    }

    #[doc="Get the PDMCR Register."]
    #[inline] pub fn pdmcr_reg(&self) -> ::bobbin_mcu::register::Register<Pdmcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdmcr, 0x44)
    }

    #[doc="Get the *mut pointer for the PDMCR register."]
    #[inline] pub fn pdmcr_mut(&self) -> *mut Pdmcr { 
        self.pdmcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDMCR register."]
    #[inline] pub fn pdmcr_ptr(&self) -> *const Pdmcr { 
        self.pdmcr_reg().ptr()
    }

    #[doc="Read the PDMCR register."]
    #[inline] pub fn pdmcr(&self) -> Pdmcr { 
        self.pdmcr_reg().read()
    }

    #[doc="Write the PDMCR register."]
    #[inline] pub fn write_pdmcr(&self, value: Pdmcr) -> &Self { 
        self.pdmcr_reg().write(value);
        self
    }

    #[doc="Set the PDMCR register."]
    #[inline] pub fn set_pdmcr<F: FnOnce(Pdmcr) -> Pdmcr>(&self, f: F) -> &Self {
        self.pdmcr_reg().set(f);
        self
    }

    #[doc="Modify the PDMCR register."]
    #[inline] pub fn with_pdmcr<F: FnOnce(Pdmcr) -> Pdmcr>(&self, f: F) -> &Self {
        self.pdmcr_reg().with(f);
        self
    }

    #[doc="Get the PDMDLY Register."]
    #[inline] pub fn pdmdly_reg(&self) -> ::bobbin_mcu::register::Register<Pdmdly> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdmdly, 0x48)
    }

    #[doc="Get the *mut pointer for the PDMDLY register."]
    #[inline] pub fn pdmdly_mut(&self) -> *mut Pdmdly { 
        self.pdmdly_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDMDLY register."]
    #[inline] pub fn pdmdly_ptr(&self) -> *const Pdmdly { 
        self.pdmdly_reg().ptr()
    }

    #[doc="Read the PDMDLY register."]
    #[inline] pub fn pdmdly(&self) -> Pdmdly { 
        self.pdmdly_reg().read()
    }

    #[doc="Write the PDMDLY register."]
    #[inline] pub fn write_pdmdly(&self, value: Pdmdly) -> &Self { 
        self.pdmdly_reg().write(value);
        self
    }

    #[doc="Set the PDMDLY register."]
    #[inline] pub fn set_pdmdly<F: FnOnce(Pdmdly) -> Pdmdly>(&self, f: F) -> &Self {
        self.pdmdly_reg().set(f);
        self
    }

    #[doc="Modify the PDMDLY register."]
    #[inline] pub fn with_pdmdly<F: FnOnce(Pdmdly) -> Pdmdly>(&self, f: F) -> &Self {
        self.pdmdly_reg().with(f);
        self
    }

}

#[doc="Global configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc="Synchronization outputs"]
    #[inline] pub fn syncout(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if SYNCOUT != 0"]
    #[inline] pub fn test_syncout(&self) -> bool {
        self.syncout() != 0
    }

    #[doc="Sets the SYNCOUT field."]
    #[inline] pub fn set_syncout<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization inputs"]
    #[inline] pub fn syncin(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SYNCIN != 0"]
    #[inline] pub fn test_syncin(&self) -> bool {
        self.syncin() != 0
    }

    #[doc="Sets the SYNCIN field."]
    #[inline] pub fn set_syncin<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gcr {
    #[inline]
    fn from(other: u32) -> Self {
         Gcr(other)
    }
}

impl ::core::fmt::Display for Gcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syncout() != 0 { try!(write!(f, " syncout=0x{:x}", self.syncout()))}
        if self.syncin() != 0 { try!(write!(f, " syncin=0x{:x}", self.syncin()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BConfiguration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr1(pub u32);
impl Bcr1 {
    #[doc="Master clock generation enable"]
    #[inline] pub fn mcken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if MCKEN != 0"]
    #[inline] pub fn test_mcken(&self) -> bool {
        self.mcken() != 0
    }

    #[doc="Sets the MCKEN field."]
    #[inline] pub fn set_mcken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Oversampling ratio for master clock"]
    #[inline] pub fn osr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if OSR != 0"]
    #[inline] pub fn test_osr(&self) -> bool {
        self.osr() != 0
    }

    #[doc="Sets the OSR field."]
    #[inline] pub fn set_osr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Master clock divider"]
    #[inline] pub fn mcjdiv(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3f) as u8) } // [25:20]
    }

    #[doc="Returns true if MCJDIV != 0"]
    #[inline] pub fn test_mcjdiv(&self) -> bool {
        self.mcjdiv() != 0
    }

    #[doc="Sets the MCJDIV field."]
    #[inline] pub fn set_mcjdiv<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="No divider"]
    #[inline] pub fn nodiv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if NODIV != 0"]
    #[inline] pub fn test_nodiv(&self) -> bool {
        self.nodiv() != 0
    }

    #[doc="Sets the NODIV field."]
    #[inline] pub fn set_nodiv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DMA enable"]
    #[inline] pub fn dmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Audio block B enable"]
    #[inline] pub fn saiben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SAIBEN != 0"]
    #[inline] pub fn test_saiben(&self) -> bool {
        self.saiben() != 0
    }

    #[doc="Sets the SAIBEN field."]
    #[inline] pub fn set_saiben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output drive"]
    #[inline] pub fn outdri(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if OUTDRI != 0"]
    #[inline] pub fn test_outdri(&self) -> bool {
        self.outdri() != 0
    }

    #[doc="Sets the OUTDRI field."]
    #[inline] pub fn set_outdri<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Mono mode"]
    #[inline] pub fn mono(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MONO != 0"]
    #[inline] pub fn test_mono(&self) -> bool {
        self.mono() != 0
    }

    #[doc="Sets the MONO field."]
    #[inline] pub fn set_mono<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn syncen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if SYNCEN != 0"]
    #[inline] pub fn test_syncen(&self) -> bool {
        self.syncen() != 0
    }

    #[doc="Sets the SYNCEN field."]
    #[inline] pub fn set_syncen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clock strobing edge"]
    #[inline] pub fn ckstr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CKSTR != 0"]
    #[inline] pub fn test_ckstr(&self) -> bool {
        self.ckstr() != 0
    }

    #[doc="Sets the CKSTR field."]
    #[inline] pub fn set_ckstr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Least significant bit first"]
    #[inline] pub fn lsbfirst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LSBFIRST != 0"]
    #[inline] pub fn test_lsbfirst(&self) -> bool {
        self.lsbfirst() != 0
    }

    #[doc="Sets the LSBFIRST field."]
    #[inline] pub fn set_lsbfirst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data size"]
    #[inline] pub fn ds(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if DS != 0"]
    #[inline] pub fn test_ds(&self) -> bool {
        self.ds() != 0
    }

    #[doc="Sets the DS field."]
    #[inline] pub fn set_ds<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Protocol configuration"]
    #[inline] pub fn prtcfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if PRTCFG != 0"]
    #[inline] pub fn test_prtcfg(&self) -> bool {
        self.prtcfg() != 0
    }

    #[doc="Sets the PRTCFG field."]
    #[inline] pub fn set_prtcfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Audio block mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
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
        if self.mcken() != 0 { try!(write!(f, " mcken"))}
        if self.osr() != 0 { try!(write!(f, " osr"))}
        if self.mcjdiv() != 0 { try!(write!(f, " mcjdiv=0x{:x}", self.mcjdiv()))}
        if self.nodiv() != 0 { try!(write!(f, " nodiv"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.saiben() != 0 { try!(write!(f, " saiben"))}
        if self.outdri() != 0 { try!(write!(f, " outdri"))}
        if self.mono() != 0 { try!(write!(f, " mono"))}
        if self.syncen() != 0 { try!(write!(f, " syncen=0x{:x}", self.syncen()))}
        if self.ckstr() != 0 { try!(write!(f, " ckstr"))}
        if self.lsbfirst() != 0 { try!(write!(f, " lsbfirst"))}
        if self.ds() != 0 { try!(write!(f, " ds=0x{:x}", self.ds()))}
        if self.prtcfg() != 0 { try!(write!(f, " prtcfg=0x{:x}", self.prtcfg()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BConfiguration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr2(pub u32);
impl Bcr2 {
    #[doc="Companding mode"]
    #[inline] pub fn comp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp(&self) -> bool {
        self.comp() != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Complement bit"]
    #[inline] pub fn cpl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CPL != 0"]
    #[inline] pub fn test_cpl(&self) -> bool {
        self.cpl() != 0
    }

    #[doc="Sets the CPL field."]
    #[inline] pub fn set_cpl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Mute counter"]
    #[inline] pub fn mutecn(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3f) as u8) } // [12:7]
    }

    #[doc="Returns true if MUTECN != 0"]
    #[inline] pub fn test_mutecn(&self) -> bool {
        self.mutecn() != 0
    }

    #[doc="Sets the MUTECN field."]
    #[inline] pub fn set_mutecn<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Mute value"]
    #[inline] pub fn muteval(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if MUTEVAL != 0"]
    #[inline] pub fn test_muteval(&self) -> bool {
        self.muteval() != 0
    }

    #[doc="Sets the MUTEVAL field."]
    #[inline] pub fn set_muteval<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Mute"]
    #[inline] pub fn mute(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MUTE != 0"]
    #[inline] pub fn test_mute(&self) -> bool {
        self.mute() != 0
    }

    #[doc="Sets the MUTE field."]
    #[inline] pub fn set_mute<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Tristate management on data line"]
    #[inline] pub fn tris(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TRIS != 0"]
    #[inline] pub fn test_tris(&self) -> bool {
        self.tris() != 0
    }

    #[doc="Sets the TRIS field."]
    #[inline] pub fn set_tris<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO flush"]
    #[inline] pub fn fflus(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FFLUS != 0"]
    #[inline] pub fn test_fflus(&self) -> bool {
        self.fflus() != 0
    }

    #[doc="Sets the FFLUS field."]
    #[inline] pub fn set_fflus<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO threshold"]
    #[inline] pub fn fth(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if FTH != 0"]
    #[inline] pub fn test_fth(&self) -> bool {
        self.fth() != 0
    }

    #[doc="Sets the FTH field."]
    #[inline] pub fn set_fth<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
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
        if self.comp() != 0 { try!(write!(f, " comp=0x{:x}", self.comp()))}
        if self.cpl() != 0 { try!(write!(f, " cpl"))}
        if self.mutecn() != 0 { try!(write!(f, " mutecn=0x{:x}", self.mutecn()))}
        if self.muteval() != 0 { try!(write!(f, " muteval"))}
        if self.mute() != 0 { try!(write!(f, " mute"))}
        if self.tris() != 0 { try!(write!(f, " tris"))}
        if self.fflus() != 0 { try!(write!(f, " fflus"))}
        if self.fth() != 0 { try!(write!(f, " fth=0x{:x}", self.fth()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BFRCR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bfrcr(pub u32);
impl Bfrcr {
    #[doc="Frame synchronization offset"]
    #[inline] pub fn fsoff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FSOFF != 0"]
    #[inline] pub fn test_fsoff(&self) -> bool {
        self.fsoff() != 0
    }

    #[doc="Sets the FSOFF field."]
    #[inline] pub fn set_fsoff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Frame synchronization polarity"]
    #[inline] pub fn fspol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FSPOL != 0"]
    #[inline] pub fn test_fspol(&self) -> bool {
        self.fspol() != 0
    }

    #[doc="Sets the FSPOL field."]
    #[inline] pub fn set_fspol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Frame synchronization definition"]
    #[inline] pub fn fsdef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FSDEF != 0"]
    #[inline] pub fn test_fsdef(&self) -> bool {
        self.fsdef() != 0
    }

    #[doc="Sets the FSDEF field."]
    #[inline] pub fn set_fsdef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Frame synchronization active level length"]
    #[inline] pub fn fsall(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if FSALL != 0"]
    #[inline] pub fn test_fsall(&self) -> bool {
        self.fsall() != 0
    }

    #[doc="Sets the FSALL field."]
    #[inline] pub fn set_fsall<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame length"]
    #[inline] pub fn frl(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FRL != 0"]
    #[inline] pub fn test_frl(&self) -> bool {
        self.frl() != 0
    }

    #[doc="Sets the FRL field."]
    #[inline] pub fn set_frl<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bfrcr {
    #[inline]
    fn from(other: u32) -> Self {
         Bfrcr(other)
    }
}

impl ::core::fmt::Display for Bfrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bfrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsoff() != 0 { try!(write!(f, " fsoff"))}
        if self.fspol() != 0 { try!(write!(f, " fspol"))}
        if self.fsdef() != 0 { try!(write!(f, " fsdef"))}
        if self.fsall() != 0 { try!(write!(f, " fsall=0x{:x}", self.fsall()))}
        if self.frl() != 0 { try!(write!(f, " frl=0x{:x}", self.frl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BSlot register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bslotr(pub u32);
impl Bslotr {
    #[doc="Slot enable"]
    #[inline] pub fn sloten(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if SLOTEN != 0"]
    #[inline] pub fn test_sloten(&self) -> bool {
        self.sloten() != 0
    }

    #[doc="Sets the SLOTEN field."]
    #[inline] pub fn set_sloten<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Number of slots in an audio frame"]
    #[inline] pub fn nbslot(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if NBSLOT != 0"]
    #[inline] pub fn test_nbslot(&self) -> bool {
        self.nbslot() != 0
    }

    #[doc="Sets the NBSLOT field."]
    #[inline] pub fn set_nbslot<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Slot size"]
    #[inline] pub fn slotsz(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if SLOTSZ != 0"]
    #[inline] pub fn test_slotsz(&self) -> bool {
        self.slotsz() != 0
    }

    #[doc="Sets the SLOTSZ field."]
    #[inline] pub fn set_slotsz<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="First bit offset"]
    #[inline] pub fn fboff(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if FBOFF != 0"]
    #[inline] pub fn test_fboff(&self) -> bool {
        self.fboff() != 0
    }

    #[doc="Sets the FBOFF field."]
    #[inline] pub fn set_fboff<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bslotr {
    #[inline]
    fn from(other: u32) -> Self {
         Bslotr(other)
    }
}

impl ::core::fmt::Display for Bslotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bslotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sloten() != 0 { try!(write!(f, " sloten=0x{:x}", self.sloten()))}
        if self.nbslot() != 0 { try!(write!(f, " nbslot=0x{:x}", self.nbslot()))}
        if self.slotsz() != 0 { try!(write!(f, " slotsz=0x{:x}", self.slotsz()))}
        if self.fboff() != 0 { try!(write!(f, " fboff=0x{:x}", self.fboff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BInterrupt mask register2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bim(pub u32);
impl Bim {
    #[doc="Late frame synchronization detection interrupt enable"]
    #[inline] pub fn lfsdetie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDETIE != 0"]
    #[inline] pub fn test_lfsdetie(&self) -> bool {
        self.lfsdetie() != 0
    }

    #[doc="Sets the LFSDETIE field."]
    #[inline] pub fn set_lfsdetie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Anticipated frame synchronization detection interrupt enable"]
    #[inline] pub fn afsdetie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFSDETIE != 0"]
    #[inline] pub fn test_afsdetie(&self) -> bool {
        self.afsdetie() != 0
    }

    #[doc="Sets the AFSDETIE field."]
    #[inline] pub fn set_afsdetie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Codec not ready interrupt enable"]
    #[inline] pub fn cnrdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDYIE != 0"]
    #[inline] pub fn test_cnrdyie(&self) -> bool {
        self.cnrdyie() != 0
    }

    #[doc="Sets the CNRDYIE field."]
    #[inline] pub fn set_cnrdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO request interrupt enable"]
    #[inline] pub fn freqie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FREQIE != 0"]
    #[inline] pub fn test_freqie(&self) -> bool {
        self.freqie() != 0
    }

    #[doc="Sets the FREQIE field."]
    #[inline] pub fn set_freqie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wrong clock configuration interrupt enable"]
    #[inline] pub fn wckcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection interrupt enable"]
    #[inline] pub fn mutedet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Overrun/underrun interrupt enable"]
    #[inline] pub fn ovrudrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDRIE != 0"]
    #[inline] pub fn test_ovrudrie(&self) -> bool {
        self.ovrudrie() != 0
    }

    #[doc="Sets the OVRUDRIE field."]
    #[inline] pub fn set_ovrudrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bim {
    #[inline]
    fn from(other: u32) -> Self {
         Bim(other)
    }
}

impl ::core::fmt::Display for Bim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfsdetie() != 0 { try!(write!(f, " lfsdetie"))}
        if self.afsdetie() != 0 { try!(write!(f, " afsdetie"))}
        if self.cnrdyie() != 0 { try!(write!(f, " cnrdyie"))}
        if self.freqie() != 0 { try!(write!(f, " freqie"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudrie() != 0 { try!(write!(f, " ovrudrie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BStatus register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bsr(pub u32);
impl Bsr {
    #[doc="FIFO level threshold"]
    #[inline] pub fn flvl(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if FLVL != 0"]
    #[inline] pub fn test_flvl(&self) -> bool {
        self.flvl() != 0
    }

    #[doc="Sets the FLVL field."]
    #[inline] pub fn set_flvl<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Late frame synchronization detection"]
    #[inline] pub fn lfsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Anticipated frame synchronization detection"]
    #[inline] pub fn afsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFSDET != 0"]
    #[inline] pub fn test_afsdet(&self) -> bool {
        self.afsdet() != 0
    }

    #[doc="Sets the AFSDET field."]
    #[inline] pub fn set_afsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Codec not ready"]
    #[inline] pub fn cnrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDY != 0"]
    #[inline] pub fn test_cnrdy(&self) -> bool {
        self.cnrdy() != 0
    }

    #[doc="Sets the CNRDY field."]
    #[inline] pub fn set_cnrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO request"]
    #[inline] pub fn freq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FREQ != 0"]
    #[inline] pub fn test_freq(&self) -> bool {
        self.freq() != 0
    }

    #[doc="Sets the FREQ field."]
    #[inline] pub fn set_freq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wrong clock configuration flag"]
    #[inline] pub fn wckcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection"]
    #[inline] pub fn mutedet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Overrun / underrun"]
    #[inline] pub fn ovrudr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDR != 0"]
    #[inline] pub fn test_ovrudr(&self) -> bool {
        self.ovrudr() != 0
    }

    #[doc="Sets the OVRUDR field."]
    #[inline] pub fn set_ovrudr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bsr {
    #[inline]
    fn from(other: u32) -> Self {
         Bsr(other)
    }
}

impl ::core::fmt::Display for Bsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flvl() != 0 { try!(write!(f, " flvl=0x{:x}", self.flvl()))}
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.afsdet() != 0 { try!(write!(f, " afsdet"))}
        if self.cnrdy() != 0 { try!(write!(f, " cnrdy"))}
        if self.freq() != 0 { try!(write!(f, " freq"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudr() != 0 { try!(write!(f, " ovrudr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BClear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bclrfr(pub u32);
impl Bclrfr {
    #[doc="Clear late frame synchronization detection flag"]
    #[inline] pub fn lfsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear anticipated frame synchronization detection flag"]
    #[inline] pub fn cafsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CAFSDET != 0"]
    #[inline] pub fn test_cafsdet(&self) -> bool {
        self.cafsdet() != 0
    }

    #[doc="Sets the CAFSDET field."]
    #[inline] pub fn set_cafsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear codec not ready flag"]
    #[inline] pub fn cnrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDY != 0"]
    #[inline] pub fn test_cnrdy(&self) -> bool {
        self.cnrdy() != 0
    }

    #[doc="Sets the CNRDY field."]
    #[inline] pub fn set_cnrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear wrong clock configuration flag"]
    #[inline] pub fn wckcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection flag"]
    #[inline] pub fn mutedet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear overrun / underrun"]
    #[inline] pub fn ovrudr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDR != 0"]
    #[inline] pub fn test_ovrudr(&self) -> bool {
        self.ovrudr() != 0
    }

    #[doc="Sets the OVRUDR field."]
    #[inline] pub fn set_ovrudr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bclrfr {
    #[inline]
    fn from(other: u32) -> Self {
         Bclrfr(other)
    }
}

impl ::core::fmt::Display for Bclrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bclrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.cafsdet() != 0 { try!(write!(f, " cafsdet"))}
        if self.cnrdy() != 0 { try!(write!(f, " cnrdy"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudr() != 0 { try!(write!(f, " ovrudr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BData register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdr(pub u32);
impl Bdr {
    #[doc="Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bdr {
    #[inline]
    fn from(other: u32) -> Self {
         Bdr(other)
    }
}

impl ::core::fmt::Display for Bdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AConfiguration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr1(pub u32);
impl Acr1 {
    #[doc="Master clock generation enable"]
    #[inline] pub fn mcken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if MCKEN != 0"]
    #[inline] pub fn test_mcken(&self) -> bool {
        self.mcken() != 0
    }

    #[doc="Sets the MCKEN field."]
    #[inline] pub fn set_mcken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Oversampling ratio for master clock"]
    #[inline] pub fn osr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if OSR != 0"]
    #[inline] pub fn test_osr(&self) -> bool {
        self.osr() != 0
    }

    #[doc="Sets the OSR field."]
    #[inline] pub fn set_osr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Master clock divider"]
    #[inline] pub fn mcjdiv(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3f) as u8) } // [25:20]
    }

    #[doc="Returns true if MCJDIV != 0"]
    #[inline] pub fn test_mcjdiv(&self) -> bool {
        self.mcjdiv() != 0
    }

    #[doc="Sets the MCJDIV field."]
    #[inline] pub fn set_mcjdiv<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="No divider"]
    #[inline] pub fn nodiv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if NODIV != 0"]
    #[inline] pub fn test_nodiv(&self) -> bool {
        self.nodiv() != 0
    }

    #[doc="Sets the NODIV field."]
    #[inline] pub fn set_nodiv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DMA enable"]
    #[inline] pub fn dmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Audio block B enable"]
    #[inline] pub fn saiben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SAIBEN != 0"]
    #[inline] pub fn test_saiben(&self) -> bool {
        self.saiben() != 0
    }

    #[doc="Sets the SAIBEN field."]
    #[inline] pub fn set_saiben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output drive"]
    #[inline] pub fn outdri(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if OUTDRI != 0"]
    #[inline] pub fn test_outdri(&self) -> bool {
        self.outdri() != 0
    }

    #[doc="Sets the OUTDRI field."]
    #[inline] pub fn set_outdri<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Mono mode"]
    #[inline] pub fn mono(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MONO != 0"]
    #[inline] pub fn test_mono(&self) -> bool {
        self.mono() != 0
    }

    #[doc="Sets the MONO field."]
    #[inline] pub fn set_mono<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn syncen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if SYNCEN != 0"]
    #[inline] pub fn test_syncen(&self) -> bool {
        self.syncen() != 0
    }

    #[doc="Sets the SYNCEN field."]
    #[inline] pub fn set_syncen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clock strobing edge"]
    #[inline] pub fn ckstr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CKSTR != 0"]
    #[inline] pub fn test_ckstr(&self) -> bool {
        self.ckstr() != 0
    }

    #[doc="Sets the CKSTR field."]
    #[inline] pub fn set_ckstr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Least significant bit first"]
    #[inline] pub fn lsbfirst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LSBFIRST != 0"]
    #[inline] pub fn test_lsbfirst(&self) -> bool {
        self.lsbfirst() != 0
    }

    #[doc="Sets the LSBFIRST field."]
    #[inline] pub fn set_lsbfirst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data size"]
    #[inline] pub fn ds(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if DS != 0"]
    #[inline] pub fn test_ds(&self) -> bool {
        self.ds() != 0
    }

    #[doc="Sets the DS field."]
    #[inline] pub fn set_ds<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Protocol configuration"]
    #[inline] pub fn prtcfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if PRTCFG != 0"]
    #[inline] pub fn test_prtcfg(&self) -> bool {
        self.prtcfg() != 0
    }

    #[doc="Sets the PRTCFG field."]
    #[inline] pub fn set_prtcfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Audio block mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Acr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Acr1(other)
    }
}

impl ::core::fmt::Display for Acr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcken() != 0 { try!(write!(f, " mcken"))}
        if self.osr() != 0 { try!(write!(f, " osr"))}
        if self.mcjdiv() != 0 { try!(write!(f, " mcjdiv=0x{:x}", self.mcjdiv()))}
        if self.nodiv() != 0 { try!(write!(f, " nodiv"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.saiben() != 0 { try!(write!(f, " saiben"))}
        if self.outdri() != 0 { try!(write!(f, " outdri"))}
        if self.mono() != 0 { try!(write!(f, " mono"))}
        if self.syncen() != 0 { try!(write!(f, " syncen=0x{:x}", self.syncen()))}
        if self.ckstr() != 0 { try!(write!(f, " ckstr"))}
        if self.lsbfirst() != 0 { try!(write!(f, " lsbfirst"))}
        if self.ds() != 0 { try!(write!(f, " ds=0x{:x}", self.ds()))}
        if self.prtcfg() != 0 { try!(write!(f, " prtcfg=0x{:x}", self.prtcfg()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AConfiguration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr2(pub u32);
impl Acr2 {
    #[doc="Companding mode"]
    #[inline] pub fn comp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp(&self) -> bool {
        self.comp() != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Complement bit"]
    #[inline] pub fn cpl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CPL != 0"]
    #[inline] pub fn test_cpl(&self) -> bool {
        self.cpl() != 0
    }

    #[doc="Sets the CPL field."]
    #[inline] pub fn set_cpl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Mute counter"]
    #[inline] pub fn mutecn(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3f) as u8) } // [12:7]
    }

    #[doc="Returns true if MUTECN != 0"]
    #[inline] pub fn test_mutecn(&self) -> bool {
        self.mutecn() != 0
    }

    #[doc="Sets the MUTECN field."]
    #[inline] pub fn set_mutecn<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Mute value"]
    #[inline] pub fn muteval(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if MUTEVAL != 0"]
    #[inline] pub fn test_muteval(&self) -> bool {
        self.muteval() != 0
    }

    #[doc="Sets the MUTEVAL field."]
    #[inline] pub fn set_muteval<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Mute"]
    #[inline] pub fn mute(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MUTE != 0"]
    #[inline] pub fn test_mute(&self) -> bool {
        self.mute() != 0
    }

    #[doc="Sets the MUTE field."]
    #[inline] pub fn set_mute<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Tristate management on data line"]
    #[inline] pub fn tris(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TRIS != 0"]
    #[inline] pub fn test_tris(&self) -> bool {
        self.tris() != 0
    }

    #[doc="Sets the TRIS field."]
    #[inline] pub fn set_tris<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO flush"]
    #[inline] pub fn fflus(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FFLUS != 0"]
    #[inline] pub fn test_fflus(&self) -> bool {
        self.fflus() != 0
    }

    #[doc="Sets the FFLUS field."]
    #[inline] pub fn set_fflus<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO threshold"]
    #[inline] pub fn fth(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if FTH != 0"]
    #[inline] pub fn test_fth(&self) -> bool {
        self.fth() != 0
    }

    #[doc="Sets the FTH field."]
    #[inline] pub fn set_fth<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Acr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Acr2(other)
    }
}

impl ::core::fmt::Display for Acr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.comp() != 0 { try!(write!(f, " comp=0x{:x}", self.comp()))}
        if self.cpl() != 0 { try!(write!(f, " cpl"))}
        if self.mutecn() != 0 { try!(write!(f, " mutecn=0x{:x}", self.mutecn()))}
        if self.muteval() != 0 { try!(write!(f, " muteval"))}
        if self.mute() != 0 { try!(write!(f, " mute"))}
        if self.tris() != 0 { try!(write!(f, " tris"))}
        if self.fflus() != 0 { try!(write!(f, " fflus"))}
        if self.fth() != 0 { try!(write!(f, " fth=0x{:x}", self.fth()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AFRCR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afrcr(pub u32);
impl Afrcr {
    #[doc="Frame synchronization offset"]
    #[inline] pub fn fsoff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FSOFF != 0"]
    #[inline] pub fn test_fsoff(&self) -> bool {
        self.fsoff() != 0
    }

    #[doc="Sets the FSOFF field."]
    #[inline] pub fn set_fsoff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Frame synchronization polarity"]
    #[inline] pub fn fspol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FSPOL != 0"]
    #[inline] pub fn test_fspol(&self) -> bool {
        self.fspol() != 0
    }

    #[doc="Sets the FSPOL field."]
    #[inline] pub fn set_fspol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Frame synchronization definition"]
    #[inline] pub fn fsdef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FSDEF != 0"]
    #[inline] pub fn test_fsdef(&self) -> bool {
        self.fsdef() != 0
    }

    #[doc="Sets the FSDEF field."]
    #[inline] pub fn set_fsdef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Frame synchronization active level length"]
    #[inline] pub fn fsall(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if FSALL != 0"]
    #[inline] pub fn test_fsall(&self) -> bool {
        self.fsall() != 0
    }

    #[doc="Sets the FSALL field."]
    #[inline] pub fn set_fsall<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame length"]
    #[inline] pub fn frl(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FRL != 0"]
    #[inline] pub fn test_frl(&self) -> bool {
        self.frl() != 0
    }

    #[doc="Sets the FRL field."]
    #[inline] pub fn set_frl<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Afrcr {
    #[inline]
    fn from(other: u32) -> Self {
         Afrcr(other)
    }
}

impl ::core::fmt::Display for Afrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsoff() != 0 { try!(write!(f, " fsoff"))}
        if self.fspol() != 0 { try!(write!(f, " fspol"))}
        if self.fsdef() != 0 { try!(write!(f, " fsdef"))}
        if self.fsall() != 0 { try!(write!(f, " fsall=0x{:x}", self.fsall()))}
        if self.frl() != 0 { try!(write!(f, " frl=0x{:x}", self.frl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ASlot register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aslotr(pub u32);
impl Aslotr {
    #[doc="Slot enable"]
    #[inline] pub fn sloten(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if SLOTEN != 0"]
    #[inline] pub fn test_sloten(&self) -> bool {
        self.sloten() != 0
    }

    #[doc="Sets the SLOTEN field."]
    #[inline] pub fn set_sloten<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Number of slots in an audio frame"]
    #[inline] pub fn nbslot(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if NBSLOT != 0"]
    #[inline] pub fn test_nbslot(&self) -> bool {
        self.nbslot() != 0
    }

    #[doc="Sets the NBSLOT field."]
    #[inline] pub fn set_nbslot<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Slot size"]
    #[inline] pub fn slotsz(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if SLOTSZ != 0"]
    #[inline] pub fn test_slotsz(&self) -> bool {
        self.slotsz() != 0
    }

    #[doc="Sets the SLOTSZ field."]
    #[inline] pub fn set_slotsz<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="First bit offset"]
    #[inline] pub fn fboff(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if FBOFF != 0"]
    #[inline] pub fn test_fboff(&self) -> bool {
        self.fboff() != 0
    }

    #[doc="Sets the FBOFF field."]
    #[inline] pub fn set_fboff<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Aslotr {
    #[inline]
    fn from(other: u32) -> Self {
         Aslotr(other)
    }
}

impl ::core::fmt::Display for Aslotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aslotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sloten() != 0 { try!(write!(f, " sloten=0x{:x}", self.sloten()))}
        if self.nbslot() != 0 { try!(write!(f, " nbslot=0x{:x}", self.nbslot()))}
        if self.slotsz() != 0 { try!(write!(f, " slotsz=0x{:x}", self.slotsz()))}
        if self.fboff() != 0 { try!(write!(f, " fboff=0x{:x}", self.fboff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AInterrupt mask register2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aim(pub u32);
impl Aim {
    #[doc="Late frame synchronization detection interrupt enable"]
    #[inline] pub fn lfsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Anticipated frame synchronization detection interrupt enable"]
    #[inline] pub fn afsdetie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFSDETIE != 0"]
    #[inline] pub fn test_afsdetie(&self) -> bool {
        self.afsdetie() != 0
    }

    #[doc="Sets the AFSDETIE field."]
    #[inline] pub fn set_afsdetie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Codec not ready interrupt enable"]
    #[inline] pub fn cnrdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDYIE != 0"]
    #[inline] pub fn test_cnrdyie(&self) -> bool {
        self.cnrdyie() != 0
    }

    #[doc="Sets the CNRDYIE field."]
    #[inline] pub fn set_cnrdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO request interrupt enable"]
    #[inline] pub fn freqie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FREQIE != 0"]
    #[inline] pub fn test_freqie(&self) -> bool {
        self.freqie() != 0
    }

    #[doc="Sets the FREQIE field."]
    #[inline] pub fn set_freqie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wrong clock configuration interrupt enable"]
    #[inline] pub fn wckcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection interrupt enable"]
    #[inline] pub fn mutedet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Overrun/underrun interrupt enable"]
    #[inline] pub fn ovrudrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDRIE != 0"]
    #[inline] pub fn test_ovrudrie(&self) -> bool {
        self.ovrudrie() != 0
    }

    #[doc="Sets the OVRUDRIE field."]
    #[inline] pub fn set_ovrudrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Aim {
    #[inline]
    fn from(other: u32) -> Self {
         Aim(other)
    }
}

impl ::core::fmt::Display for Aim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.afsdetie() != 0 { try!(write!(f, " afsdetie"))}
        if self.cnrdyie() != 0 { try!(write!(f, " cnrdyie"))}
        if self.freqie() != 0 { try!(write!(f, " freqie"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudrie() != 0 { try!(write!(f, " ovrudrie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AStatus register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Asr(pub u32);
impl Asr {
    #[doc="FIFO level threshold"]
    #[inline] pub fn flvl(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if FLVL != 0"]
    #[inline] pub fn test_flvl(&self) -> bool {
        self.flvl() != 0
    }

    #[doc="Sets the FLVL field."]
    #[inline] pub fn set_flvl<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Late frame synchronization detection"]
    #[inline] pub fn lfsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Anticipated frame synchronization detection"]
    #[inline] pub fn afsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFSDET != 0"]
    #[inline] pub fn test_afsdet(&self) -> bool {
        self.afsdet() != 0
    }

    #[doc="Sets the AFSDET field."]
    #[inline] pub fn set_afsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Codec not ready"]
    #[inline] pub fn cnrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDY != 0"]
    #[inline] pub fn test_cnrdy(&self) -> bool {
        self.cnrdy() != 0
    }

    #[doc="Sets the CNRDY field."]
    #[inline] pub fn set_cnrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO request"]
    #[inline] pub fn freq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FREQ != 0"]
    #[inline] pub fn test_freq(&self) -> bool {
        self.freq() != 0
    }

    #[doc="Sets the FREQ field."]
    #[inline] pub fn set_freq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wrong clock configuration flag. This bit is read only"]
    #[inline] pub fn wckcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection"]
    #[inline] pub fn mutedet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Overrun / underrun"]
    #[inline] pub fn ovrudr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDR != 0"]
    #[inline] pub fn test_ovrudr(&self) -> bool {
        self.ovrudr() != 0
    }

    #[doc="Sets the OVRUDR field."]
    #[inline] pub fn set_ovrudr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Asr {
    #[inline]
    fn from(other: u32) -> Self {
         Asr(other)
    }
}

impl ::core::fmt::Display for Asr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Asr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flvl() != 0 { try!(write!(f, " flvl=0x{:x}", self.flvl()))}
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.afsdet() != 0 { try!(write!(f, " afsdet"))}
        if self.cnrdy() != 0 { try!(write!(f, " cnrdy"))}
        if self.freq() != 0 { try!(write!(f, " freq"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudr() != 0 { try!(write!(f, " ovrudr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AClear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aclrfr(pub u32);
impl Aclrfr {
    #[doc="Clear late frame synchronization detection flag"]
    #[inline] pub fn lfsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear anticipated frame synchronization detection flag"]
    #[inline] pub fn cafsdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CAFSDET != 0"]
    #[inline] pub fn test_cafsdet(&self) -> bool {
        self.cafsdet() != 0
    }

    #[doc="Sets the CAFSDET field."]
    #[inline] pub fn set_cafsdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear codec not ready flag"]
    #[inline] pub fn cnrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDY != 0"]
    #[inline] pub fn test_cnrdy(&self) -> bool {
        self.cnrdy() != 0
    }

    #[doc="Sets the CNRDY field."]
    #[inline] pub fn set_cnrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear wrong clock configuration flag"]
    #[inline] pub fn wckcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection flag"]
    #[inline] pub fn mutedet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear overrun / underrun"]
    #[inline] pub fn ovrudr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDR != 0"]
    #[inline] pub fn test_ovrudr(&self) -> bool {
        self.ovrudr() != 0
    }

    #[doc="Sets the OVRUDR field."]
    #[inline] pub fn set_ovrudr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Aclrfr {
    #[inline]
    fn from(other: u32) -> Self {
         Aclrfr(other)
    }
}

impl ::core::fmt::Display for Aclrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aclrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.cafsdet() != 0 { try!(write!(f, " cafsdet"))}
        if self.cnrdy() != 0 { try!(write!(f, " cnrdy"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudr() != 0 { try!(write!(f, " ovrudr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AData register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adr(pub u32);
impl Adr {
    #[doc="Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Adr {
    #[inline]
    fn from(other: u32) -> Self {
         Adr(other)
    }
}

impl ::core::fmt::Display for Adr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PDM control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdmcr(pub u32);
impl Pdmcr {
    #[doc="Clock enable of bitstream clock number 4"]
    #[inline] pub fn cken4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CKEN4 != 0"]
    #[inline] pub fn test_cken4(&self) -> bool {
        self.cken4() != 0
    }

    #[doc="Sets the CKEN4 field."]
    #[inline] pub fn set_cken4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clock enable of bitstream clock number 3"]
    #[inline] pub fn cken3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CKEN3 != 0"]
    #[inline] pub fn test_cken3(&self) -> bool {
        self.cken3() != 0
    }

    #[doc="Sets the CKEN3 field."]
    #[inline] pub fn set_cken3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clock enable of bitstream clock number 2"]
    #[inline] pub fn cken2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CKEN2 != 0"]
    #[inline] pub fn test_cken2(&self) -> bool {
        self.cken2() != 0
    }

    #[doc="Sets the CKEN2 field."]
    #[inline] pub fn set_cken2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clock enable of bitstream clock number 1"]
    #[inline] pub fn cken1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CKEN1 != 0"]
    #[inline] pub fn test_cken1(&self) -> bool {
        self.cken1() != 0
    }

    #[doc="Sets the CKEN1 field."]
    #[inline] pub fn set_cken1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Number of microphones"]
    #[inline] pub fn micnbr(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if MICNBR != 0"]
    #[inline] pub fn test_micnbr(&self) -> bool {
        self.micnbr() != 0
    }

    #[doc="Sets the MICNBR field."]
    #[inline] pub fn set_micnbr<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PDM enable"]
    #[inline] pub fn pdmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PDMEN != 0"]
    #[inline] pub fn test_pdmen(&self) -> bool {
        self.pdmen() != 0
    }

    #[doc="Sets the PDMEN field."]
    #[inline] pub fn set_pdmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdmcr {
    #[inline]
    fn from(other: u32) -> Self {
         Pdmcr(other)
    }
}

impl ::core::fmt::Display for Pdmcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdmcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cken4() != 0 { try!(write!(f, " cken4"))}
        if self.cken3() != 0 { try!(write!(f, " cken3"))}
        if self.cken2() != 0 { try!(write!(f, " cken2"))}
        if self.cken1() != 0 { try!(write!(f, " cken1"))}
        if self.micnbr() != 0 { try!(write!(f, " micnbr=0x{:x}", self.micnbr()))}
        if self.pdmen() != 0 { try!(write!(f, " pdmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PDM delay register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdmdly(pub u32);
impl Pdmdly {
    #[doc="Delay line for second microphone of pair 4"]
    #[inline] pub fn dlym4r(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if DLYM4R != 0"]
    #[inline] pub fn test_dlym4r(&self) -> bool {
        self.dlym4r() != 0
    }

    #[doc="Sets the DLYM4R field."]
    #[inline] pub fn set_dlym4r<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Delay line for first microphone of pair 4"]
    #[inline] pub fn dlym4l(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if DLYM4L != 0"]
    #[inline] pub fn test_dlym4l(&self) -> bool {
        self.dlym4l() != 0
    }

    #[doc="Sets the DLYM4L field."]
    #[inline] pub fn set_dlym4l<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Delay line for second microphone of pair 3"]
    #[inline] pub fn dlym3r(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if DLYM3R != 0"]
    #[inline] pub fn test_dlym3r(&self) -> bool {
        self.dlym3r() != 0
    }

    #[doc="Sets the DLYM3R field."]
    #[inline] pub fn set_dlym3r<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Delay line for first microphone of pair 3"]
    #[inline] pub fn dlym3l(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if DLYM3L != 0"]
    #[inline] pub fn test_dlym3l(&self) -> bool {
        self.dlym3l() != 0
    }

    #[doc="Sets the DLYM3L field."]
    #[inline] pub fn set_dlym3l<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Delay line for second microphone of pair 2"]
    #[inline] pub fn dlym2r(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if DLYM2R != 0"]
    #[inline] pub fn test_dlym2r(&self) -> bool {
        self.dlym2r() != 0
    }

    #[doc="Sets the DLYM2R field."]
    #[inline] pub fn set_dlym2r<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Delay line for first microphone of pair 2"]
    #[inline] pub fn dlym2l(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if DLYM2L != 0"]
    #[inline] pub fn test_dlym2l(&self) -> bool {
        self.dlym2l() != 0
    }

    #[doc="Sets the DLYM2L field."]
    #[inline] pub fn set_dlym2l<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Delay line for second microphone of pair 1"]
    #[inline] pub fn dlym1r(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if DLYM1R != 0"]
    #[inline] pub fn test_dlym1r(&self) -> bool {
        self.dlym1r() != 0
    }

    #[doc="Sets the DLYM1R field."]
    #[inline] pub fn set_dlym1r<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Delay line for first microphone of pair 1"]
    #[inline] pub fn dlym1l(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if DLYM1L != 0"]
    #[inline] pub fn test_dlym1l(&self) -> bool {
        self.dlym1l() != 0
    }

    #[doc="Sets the DLYM1L field."]
    #[inline] pub fn set_dlym1l<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdmdly {
    #[inline]
    fn from(other: u32) -> Self {
         Pdmdly(other)
    }
}

impl ::core::fmt::Display for Pdmdly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdmdly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dlym4r() != 0 { try!(write!(f, " dlym4r=0x{:x}", self.dlym4r()))}
        if self.dlym4l() != 0 { try!(write!(f, " dlym4l=0x{:x}", self.dlym4l()))}
        if self.dlym3r() != 0 { try!(write!(f, " dlym3r=0x{:x}", self.dlym3r()))}
        if self.dlym3l() != 0 { try!(write!(f, " dlym3l=0x{:x}", self.dlym3l()))}
        if self.dlym2r() != 0 { try!(write!(f, " dlym2r=0x{:x}", self.dlym2r()))}
        if self.dlym2l() != 0 { try!(write!(f, " dlym2l=0x{:x}", self.dlym2l()))}
        if self.dlym1r() != 0 { try!(write!(f, " dlym1r=0x{:x}", self.dlym1r()))}
        if self.dlym1l() != 0 { try!(write!(f, " dlym1l=0x{:x}", self.dlym1l()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

