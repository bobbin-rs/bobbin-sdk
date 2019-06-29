::bobbin_mcu::periph!( SDHC0, Sdhc0, SDHC0_PERIPH, SdhcPeriph, SDHC0_OWNED, SDHC0_REF_COUNT, 0x45000000, 0x00, 0x1e);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SDHC Peripheral"]
pub struct SdhcPeriph(pub usize); 

impl SdhcPeriph {
    #[doc="Get the SSAR Register."]
    #[inline] pub fn ssar_reg(&self) -> ::bobbin_mcu::register::Register<Ssar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ssar, 0x0)
    }

    #[doc="Get the *mut pointer for the SSAR register."]
    #[inline] pub fn ssar_mut(&self) -> *mut Ssar { 
        self.ssar_reg().ptr()
    }

    #[doc="Get the *const pointer for the SSAR register."]
    #[inline] pub fn ssar_ptr(&self) -> *const Ssar { 
        self.ssar_reg().ptr()
    }

    #[doc="Read the SSAR register."]
    #[inline] pub fn ssar(&self) -> Ssar { 
        self.ssar_reg().read()
    }

    #[doc="Write the SSAR register."]
    #[inline] pub fn write_ssar(&self, value: Ssar) -> &Self { 
        self.ssar_reg().write(value);
        self
    }

    #[doc="Set the SSAR register."]
    #[inline] pub fn set_ssar<F: FnOnce(Ssar) -> Ssar>(&self, f: F) -> &Self {
        self.ssar_reg().set(f);
        self
    }

    #[doc="Modify the SSAR register."]
    #[inline] pub fn with_ssar<F: FnOnce(Ssar) -> Ssar>(&self, f: F) -> &Self {
        self.ssar_reg().with(f);
        self
    }

    #[doc="Get the SSAR_CMD23 Register."]
    #[inline] pub fn ssar_cmd23_reg(&self) -> ::bobbin_mcu::register::Register<SsarCmd23> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut SsarCmd23, 0x0)
    }

    #[doc="Get the *mut pointer for the SSAR_CMD23 register."]
    #[inline] pub fn ssar_cmd23_mut(&self) -> *mut SsarCmd23 { 
        self.ssar_cmd23_reg().ptr()
    }

    #[doc="Get the *const pointer for the SSAR_CMD23 register."]
    #[inline] pub fn ssar_cmd23_ptr(&self) -> *const SsarCmd23 { 
        self.ssar_cmd23_reg().ptr()
    }

    #[doc="Read the SSAR_CMD23 register."]
    #[inline] pub fn ssar_cmd23(&self) -> SsarCmd23 { 
        self.ssar_cmd23_reg().read()
    }

    #[doc="Write the SSAR_CMD23 register."]
    #[inline] pub fn write_ssar_cmd23(&self, value: SsarCmd23) -> &Self { 
        self.ssar_cmd23_reg().write(value);
        self
    }

    #[doc="Set the SSAR_CMD23 register."]
    #[inline] pub fn set_ssar_cmd23<F: FnOnce(SsarCmd23) -> SsarCmd23>(&self, f: F) -> &Self {
        self.ssar_cmd23_reg().set(f);
        self
    }

    #[doc="Modify the SSAR_CMD23 register."]
    #[inline] pub fn with_ssar_cmd23<F: FnOnce(SsarCmd23) -> SsarCmd23>(&self, f: F) -> &Self {
        self.ssar_cmd23_reg().with(f);
        self
    }

    #[doc="Get the BSR Register."]
    #[inline] pub fn bsr_reg(&self) -> ::bobbin_mcu::register::Register<Bsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bsr, 0x4)
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

    #[doc="Write the BSR register."]
    #[inline] pub fn write_bsr(&self, value: Bsr) -> &Self { 
        self.bsr_reg().write(value);
        self
    }

    #[doc="Set the BSR register."]
    #[inline] pub fn set_bsr<F: FnOnce(Bsr) -> Bsr>(&self, f: F) -> &Self {
        self.bsr_reg().set(f);
        self
    }

    #[doc="Modify the BSR register."]
    #[inline] pub fn with_bsr<F: FnOnce(Bsr) -> Bsr>(&self, f: F) -> &Self {
        self.bsr_reg().with(f);
        self
    }

    #[doc="Get the BCR Register."]
    #[inline] pub fn bcr_reg(&self) -> ::bobbin_mcu::register::Register<Bcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bcr, 0x6)
    }

    #[doc="Get the *mut pointer for the BCR register."]
    #[inline] pub fn bcr_mut(&self) -> *mut Bcr { 
        self.bcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BCR register."]
    #[inline] pub fn bcr_ptr(&self) -> *const Bcr { 
        self.bcr_reg().ptr()
    }

    #[doc="Read the BCR register."]
    #[inline] pub fn bcr(&self) -> Bcr { 
        self.bcr_reg().read()
    }

    #[doc="Write the BCR register."]
    #[inline] pub fn write_bcr(&self, value: Bcr) -> &Self { 
        self.bcr_reg().write(value);
        self
    }

    #[doc="Set the BCR register."]
    #[inline] pub fn set_bcr<F: FnOnce(Bcr) -> Bcr>(&self, f: F) -> &Self {
        self.bcr_reg().set(f);
        self
    }

    #[doc="Modify the BCR register."]
    #[inline] pub fn with_bcr<F: FnOnce(Bcr) -> Bcr>(&self, f: F) -> &Self {
        self.bcr_reg().with(f);
        self
    }

    #[doc="Get the ARG1R Register."]
    #[inline] pub fn arg1r_reg(&self) -> ::bobbin_mcu::register::Register<Arg1r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Arg1r, 0x8)
    }

    #[doc="Get the *mut pointer for the ARG1R register."]
    #[inline] pub fn arg1r_mut(&self) -> *mut Arg1r { 
        self.arg1r_reg().ptr()
    }

    #[doc="Get the *const pointer for the ARG1R register."]
    #[inline] pub fn arg1r_ptr(&self) -> *const Arg1r { 
        self.arg1r_reg().ptr()
    }

    #[doc="Read the ARG1R register."]
    #[inline] pub fn arg1r(&self) -> Arg1r { 
        self.arg1r_reg().read()
    }

    #[doc="Write the ARG1R register."]
    #[inline] pub fn write_arg1r(&self, value: Arg1r) -> &Self { 
        self.arg1r_reg().write(value);
        self
    }

    #[doc="Set the ARG1R register."]
    #[inline] pub fn set_arg1r<F: FnOnce(Arg1r) -> Arg1r>(&self, f: F) -> &Self {
        self.arg1r_reg().set(f);
        self
    }

    #[doc="Modify the ARG1R register."]
    #[inline] pub fn with_arg1r<F: FnOnce(Arg1r) -> Arg1r>(&self, f: F) -> &Self {
        self.arg1r_reg().with(f);
        self
    }

    #[doc="Get the TMR Register."]
    #[inline] pub fn tmr_reg(&self) -> ::bobbin_mcu::register::Register<Tmr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tmr, 0xc)
    }

    #[doc="Get the *mut pointer for the TMR register."]
    #[inline] pub fn tmr_mut(&self) -> *mut Tmr { 
        self.tmr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TMR register."]
    #[inline] pub fn tmr_ptr(&self) -> *const Tmr { 
        self.tmr_reg().ptr()
    }

    #[doc="Read the TMR register."]
    #[inline] pub fn tmr(&self) -> Tmr { 
        self.tmr_reg().read()
    }

    #[doc="Write the TMR register."]
    #[inline] pub fn write_tmr(&self, value: Tmr) -> &Self { 
        self.tmr_reg().write(value);
        self
    }

    #[doc="Set the TMR register."]
    #[inline] pub fn set_tmr<F: FnOnce(Tmr) -> Tmr>(&self, f: F) -> &Self {
        self.tmr_reg().set(f);
        self
    }

    #[doc="Modify the TMR register."]
    #[inline] pub fn with_tmr<F: FnOnce(Tmr) -> Tmr>(&self, f: F) -> &Self {
        self.tmr_reg().with(f);
        self
    }

    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0xe)
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

    #[doc="Get the RR Register."]
    #[inline] pub fn rr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Rr, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Rr, 0x10, 0x4)
    }

    #[doc="Get the *mut pointer for the RR register."]
    #[inline] pub fn rr_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Rr { 
        self.rr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the RR register."]
    #[inline] pub fn rr_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Rr { 
        self.rr_reg().ptr(index.into())
    }

    #[doc="Read the RR register."]
    #[inline] pub fn rr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Rr { 
        self.rr_reg().read(index.into())
    }

    #[doc="Get the BDPR Register."]
    #[inline] pub fn bdpr_reg(&self) -> ::bobbin_mcu::register::Register<Bdpr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdpr, 0x20)
    }

    #[doc="Get the *mut pointer for the BDPR register."]
    #[inline] pub fn bdpr_mut(&self) -> *mut Bdpr { 
        self.bdpr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDPR register."]
    #[inline] pub fn bdpr_ptr(&self) -> *const Bdpr { 
        self.bdpr_reg().ptr()
    }

    #[doc="Read the BDPR register."]
    #[inline] pub fn bdpr(&self) -> Bdpr { 
        self.bdpr_reg().read()
    }

    #[doc="Write the BDPR register."]
    #[inline] pub fn write_bdpr(&self, value: Bdpr) -> &Self { 
        self.bdpr_reg().write(value);
        self
    }

    #[doc="Set the BDPR register."]
    #[inline] pub fn set_bdpr<F: FnOnce(Bdpr) -> Bdpr>(&self, f: F) -> &Self {
        self.bdpr_reg().set(f);
        self
    }

    #[doc="Modify the BDPR register."]
    #[inline] pub fn with_bdpr<F: FnOnce(Bdpr) -> Bdpr>(&self, f: F) -> &Self {
        self.bdpr_reg().with(f);
        self
    }

    #[doc="Get the PSR Register."]
    #[inline] pub fn psr_reg(&self) -> ::bobbin_mcu::register::Register<Psr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Psr, 0x24)
    }

    #[doc="Get the *mut pointer for the PSR register."]
    #[inline] pub fn psr_mut(&self) -> *mut Psr { 
        self.psr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PSR register."]
    #[inline] pub fn psr_ptr(&self) -> *const Psr { 
        self.psr_reg().ptr()
    }

    #[doc="Read the PSR register."]
    #[inline] pub fn psr(&self) -> Psr { 
        self.psr_reg().read()
    }

    #[doc="Get the HC1R Register."]
    #[inline] pub fn hc1r_reg(&self) -> ::bobbin_mcu::register::Register<Hc1r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hc1r, 0x28)
    }

    #[doc="Get the *mut pointer for the HC1R register."]
    #[inline] pub fn hc1r_mut(&self) -> *mut Hc1r { 
        self.hc1r_reg().ptr()
    }

    #[doc="Get the *const pointer for the HC1R register."]
    #[inline] pub fn hc1r_ptr(&self) -> *const Hc1r { 
        self.hc1r_reg().ptr()
    }

    #[doc="Read the HC1R register."]
    #[inline] pub fn hc1r(&self) -> Hc1r { 
        self.hc1r_reg().read()
    }

    #[doc="Write the HC1R register."]
    #[inline] pub fn write_hc1r(&self, value: Hc1r) -> &Self { 
        self.hc1r_reg().write(value);
        self
    }

    #[doc="Set the HC1R register."]
    #[inline] pub fn set_hc1r<F: FnOnce(Hc1r) -> Hc1r>(&self, f: F) -> &Self {
        self.hc1r_reg().set(f);
        self
    }

    #[doc="Modify the HC1R register."]
    #[inline] pub fn with_hc1r<F: FnOnce(Hc1r) -> Hc1r>(&self, f: F) -> &Self {
        self.hc1r_reg().with(f);
        self
    }

    #[doc="Get the HC1R_EMMC Register."]
    #[inline] pub fn hc1r_emmc_reg(&self) -> ::bobbin_mcu::register::Register<Hc1rEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hc1rEmmc, 0x28)
    }

    #[doc="Get the *mut pointer for the HC1R_EMMC register."]
    #[inline] pub fn hc1r_emmc_mut(&self) -> *mut Hc1rEmmc { 
        self.hc1r_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the HC1R_EMMC register."]
    #[inline] pub fn hc1r_emmc_ptr(&self) -> *const Hc1rEmmc { 
        self.hc1r_emmc_reg().ptr()
    }

    #[doc="Read the HC1R_EMMC register."]
    #[inline] pub fn hc1r_emmc(&self) -> Hc1rEmmc { 
        self.hc1r_emmc_reg().read()
    }

    #[doc="Write the HC1R_EMMC register."]
    #[inline] pub fn write_hc1r_emmc(&self, value: Hc1rEmmc) -> &Self { 
        self.hc1r_emmc_reg().write(value);
        self
    }

    #[doc="Set the HC1R_EMMC register."]
    #[inline] pub fn set_hc1r_emmc<F: FnOnce(Hc1rEmmc) -> Hc1rEmmc>(&self, f: F) -> &Self {
        self.hc1r_emmc_reg().set(f);
        self
    }

    #[doc="Modify the HC1R_EMMC register."]
    #[inline] pub fn with_hc1r_emmc<F: FnOnce(Hc1rEmmc) -> Hc1rEmmc>(&self, f: F) -> &Self {
        self.hc1r_emmc_reg().with(f);
        self
    }

    #[doc="Get the PCR Register."]
    #[inline] pub fn pcr_reg(&self) -> ::bobbin_mcu::register::Register<Pcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcr, 0x29)
    }

    #[doc="Get the *mut pointer for the PCR register."]
    #[inline] pub fn pcr_mut(&self) -> *mut Pcr { 
        self.pcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCR register."]
    #[inline] pub fn pcr_ptr(&self) -> *const Pcr { 
        self.pcr_reg().ptr()
    }

    #[doc="Read the PCR register."]
    #[inline] pub fn pcr(&self) -> Pcr { 
        self.pcr_reg().read()
    }

    #[doc="Write the PCR register."]
    #[inline] pub fn write_pcr(&self, value: Pcr) -> &Self { 
        self.pcr_reg().write(value);
        self
    }

    #[doc="Set the PCR register."]
    #[inline] pub fn set_pcr<F: FnOnce(Pcr) -> Pcr>(&self, f: F) -> &Self {
        self.pcr_reg().set(f);
        self
    }

    #[doc="Modify the PCR register."]
    #[inline] pub fn with_pcr<F: FnOnce(Pcr) -> Pcr>(&self, f: F) -> &Self {
        self.pcr_reg().with(f);
        self
    }

    #[doc="Get the BGCR Register."]
    #[inline] pub fn bgcr_reg(&self) -> ::bobbin_mcu::register::Register<Bgcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bgcr, 0x2a)
    }

    #[doc="Get the *mut pointer for the BGCR register."]
    #[inline] pub fn bgcr_mut(&self) -> *mut Bgcr { 
        self.bgcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BGCR register."]
    #[inline] pub fn bgcr_ptr(&self) -> *const Bgcr { 
        self.bgcr_reg().ptr()
    }

    #[doc="Read the BGCR register."]
    #[inline] pub fn bgcr(&self) -> Bgcr { 
        self.bgcr_reg().read()
    }

    #[doc="Write the BGCR register."]
    #[inline] pub fn write_bgcr(&self, value: Bgcr) -> &Self { 
        self.bgcr_reg().write(value);
        self
    }

    #[doc="Set the BGCR register."]
    #[inline] pub fn set_bgcr<F: FnOnce(Bgcr) -> Bgcr>(&self, f: F) -> &Self {
        self.bgcr_reg().set(f);
        self
    }

    #[doc="Modify the BGCR register."]
    #[inline] pub fn with_bgcr<F: FnOnce(Bgcr) -> Bgcr>(&self, f: F) -> &Self {
        self.bgcr_reg().with(f);
        self
    }

    #[doc="Get the BGCR_EMMC Register."]
    #[inline] pub fn bgcr_emmc_reg(&self) -> ::bobbin_mcu::register::Register<BgcrEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut BgcrEmmc, 0x2a)
    }

    #[doc="Get the *mut pointer for the BGCR_EMMC register."]
    #[inline] pub fn bgcr_emmc_mut(&self) -> *mut BgcrEmmc { 
        self.bgcr_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the BGCR_EMMC register."]
    #[inline] pub fn bgcr_emmc_ptr(&self) -> *const BgcrEmmc { 
        self.bgcr_emmc_reg().ptr()
    }

    #[doc="Read the BGCR_EMMC register."]
    #[inline] pub fn bgcr_emmc(&self) -> BgcrEmmc { 
        self.bgcr_emmc_reg().read()
    }

    #[doc="Write the BGCR_EMMC register."]
    #[inline] pub fn write_bgcr_emmc(&self, value: BgcrEmmc) -> &Self { 
        self.bgcr_emmc_reg().write(value);
        self
    }

    #[doc="Set the BGCR_EMMC register."]
    #[inline] pub fn set_bgcr_emmc<F: FnOnce(BgcrEmmc) -> BgcrEmmc>(&self, f: F) -> &Self {
        self.bgcr_emmc_reg().set(f);
        self
    }

    #[doc="Modify the BGCR_EMMC register."]
    #[inline] pub fn with_bgcr_emmc<F: FnOnce(BgcrEmmc) -> BgcrEmmc>(&self, f: F) -> &Self {
        self.bgcr_emmc_reg().with(f);
        self
    }

    #[doc="Get the WCR Register."]
    #[inline] pub fn wcr_reg(&self) -> ::bobbin_mcu::register::Register<Wcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wcr, 0x2b)
    }

    #[doc="Get the *mut pointer for the WCR register."]
    #[inline] pub fn wcr_mut(&self) -> *mut Wcr { 
        self.wcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the WCR register."]
    #[inline] pub fn wcr_ptr(&self) -> *const Wcr { 
        self.wcr_reg().ptr()
    }

    #[doc="Read the WCR register."]
    #[inline] pub fn wcr(&self) -> Wcr { 
        self.wcr_reg().read()
    }

    #[doc="Write the WCR register."]
    #[inline] pub fn write_wcr(&self, value: Wcr) -> &Self { 
        self.wcr_reg().write(value);
        self
    }

    #[doc="Set the WCR register."]
    #[inline] pub fn set_wcr<F: FnOnce(Wcr) -> Wcr>(&self, f: F) -> &Self {
        self.wcr_reg().set(f);
        self
    }

    #[doc="Modify the WCR register."]
    #[inline] pub fn with_wcr<F: FnOnce(Wcr) -> Wcr>(&self, f: F) -> &Self {
        self.wcr_reg().with(f);
        self
    }

    #[doc="Get the CCR Register."]
    #[inline] pub fn ccr_reg(&self) -> ::bobbin_mcu::register::Register<Ccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr, 0x2c)
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut(&self) -> *mut Ccr { 
        self.ccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr(&self) -> *const Ccr { 
        self.ccr_reg().ptr()
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr(&self) -> Ccr { 
        self.ccr_reg().read()
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn write_ccr(&self, value: Ccr) -> &Self { 
        self.ccr_reg().write(value);
        self
    }

    #[doc="Set the CCR register."]
    #[inline] pub fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.ccr_reg().set(f);
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.ccr_reg().with(f);
        self
    }

    #[doc="Get the TCR Register."]
    #[inline] pub fn tcr_reg(&self) -> ::bobbin_mcu::register::Register<Tcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tcr, 0x2e)
    }

    #[doc="Get the *mut pointer for the TCR register."]
    #[inline] pub fn tcr_mut(&self) -> *mut Tcr { 
        self.tcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TCR register."]
    #[inline] pub fn tcr_ptr(&self) -> *const Tcr { 
        self.tcr_reg().ptr()
    }

    #[doc="Read the TCR register."]
    #[inline] pub fn tcr(&self) -> Tcr { 
        self.tcr_reg().read()
    }

    #[doc="Write the TCR register."]
    #[inline] pub fn write_tcr(&self, value: Tcr) -> &Self { 
        self.tcr_reg().write(value);
        self
    }

    #[doc="Set the TCR register."]
    #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        self.tcr_reg().set(f);
        self
    }

    #[doc="Modify the TCR register."]
    #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        self.tcr_reg().with(f);
        self
    }

    #[doc="Get the SRR Register."]
    #[inline] pub fn srr_reg(&self) -> ::bobbin_mcu::register::Register<Srr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Srr, 0x2f)
    }

    #[doc="Get the *mut pointer for the SRR register."]
    #[inline] pub fn srr_mut(&self) -> *mut Srr { 
        self.srr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SRR register."]
    #[inline] pub fn srr_ptr(&self) -> *const Srr { 
        self.srr_reg().ptr()
    }

    #[doc="Read the SRR register."]
    #[inline] pub fn srr(&self) -> Srr { 
        self.srr_reg().read()
    }

    #[doc="Write the SRR register."]
    #[inline] pub fn write_srr(&self, value: Srr) -> &Self { 
        self.srr_reg().write(value);
        self
    }

    #[doc="Set the SRR register."]
    #[inline] pub fn set_srr<F: FnOnce(Srr) -> Srr>(&self, f: F) -> &Self {
        self.srr_reg().set(f);
        self
    }

    #[doc="Modify the SRR register."]
    #[inline] pub fn with_srr<F: FnOnce(Srr) -> Srr>(&self, f: F) -> &Self {
        self.srr_reg().with(f);
        self
    }

    #[doc="Get the NISTR Register."]
    #[inline] pub fn nistr_reg(&self) -> ::bobbin_mcu::register::Register<Nistr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Nistr, 0x30)
    }

    #[doc="Get the *mut pointer for the NISTR register."]
    #[inline] pub fn nistr_mut(&self) -> *mut Nistr { 
        self.nistr_reg().ptr()
    }

    #[doc="Get the *const pointer for the NISTR register."]
    #[inline] pub fn nistr_ptr(&self) -> *const Nistr { 
        self.nistr_reg().ptr()
    }

    #[doc="Read the NISTR register."]
    #[inline] pub fn nistr(&self) -> Nistr { 
        self.nistr_reg().read()
    }

    #[doc="Write the NISTR register."]
    #[inline] pub fn write_nistr(&self, value: Nistr) -> &Self { 
        self.nistr_reg().write(value);
        self
    }

    #[doc="Set the NISTR register."]
    #[inline] pub fn set_nistr<F: FnOnce(Nistr) -> Nistr>(&self, f: F) -> &Self {
        self.nistr_reg().set(f);
        self
    }

    #[doc="Modify the NISTR register."]
    #[inline] pub fn with_nistr<F: FnOnce(Nistr) -> Nistr>(&self, f: F) -> &Self {
        self.nistr_reg().with(f);
        self
    }

    #[doc="Get the NISTR_EMMC Register."]
    #[inline] pub fn nistr_emmc_reg(&self) -> ::bobbin_mcu::register::Register<NistrEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut NistrEmmc, 0x30)
    }

    #[doc="Get the *mut pointer for the NISTR_EMMC register."]
    #[inline] pub fn nistr_emmc_mut(&self) -> *mut NistrEmmc { 
        self.nistr_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the NISTR_EMMC register."]
    #[inline] pub fn nistr_emmc_ptr(&self) -> *const NistrEmmc { 
        self.nistr_emmc_reg().ptr()
    }

    #[doc="Read the NISTR_EMMC register."]
    #[inline] pub fn nistr_emmc(&self) -> NistrEmmc { 
        self.nistr_emmc_reg().read()
    }

    #[doc="Write the NISTR_EMMC register."]
    #[inline] pub fn write_nistr_emmc(&self, value: NistrEmmc) -> &Self { 
        self.nistr_emmc_reg().write(value);
        self
    }

    #[doc="Set the NISTR_EMMC register."]
    #[inline] pub fn set_nistr_emmc<F: FnOnce(NistrEmmc) -> NistrEmmc>(&self, f: F) -> &Self {
        self.nistr_emmc_reg().set(f);
        self
    }

    #[doc="Modify the NISTR_EMMC register."]
    #[inline] pub fn with_nistr_emmc<F: FnOnce(NistrEmmc) -> NistrEmmc>(&self, f: F) -> &Self {
        self.nistr_emmc_reg().with(f);
        self
    }

    #[doc="Get the EISTR Register."]
    #[inline] pub fn eistr_reg(&self) -> ::bobbin_mcu::register::Register<Eistr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eistr, 0x32)
    }

    #[doc="Get the *mut pointer for the EISTR register."]
    #[inline] pub fn eistr_mut(&self) -> *mut Eistr { 
        self.eistr_reg().ptr()
    }

    #[doc="Get the *const pointer for the EISTR register."]
    #[inline] pub fn eistr_ptr(&self) -> *const Eistr { 
        self.eistr_reg().ptr()
    }

    #[doc="Read the EISTR register."]
    #[inline] pub fn eistr(&self) -> Eistr { 
        self.eistr_reg().read()
    }

    #[doc="Write the EISTR register."]
    #[inline] pub fn write_eistr(&self, value: Eistr) -> &Self { 
        self.eistr_reg().write(value);
        self
    }

    #[doc="Set the EISTR register."]
    #[inline] pub fn set_eistr<F: FnOnce(Eistr) -> Eistr>(&self, f: F) -> &Self {
        self.eistr_reg().set(f);
        self
    }

    #[doc="Modify the EISTR register."]
    #[inline] pub fn with_eistr<F: FnOnce(Eistr) -> Eistr>(&self, f: F) -> &Self {
        self.eistr_reg().with(f);
        self
    }

    #[doc="Get the EISTR_EMMC Register."]
    #[inline] pub fn eistr_emmc_reg(&self) -> ::bobbin_mcu::register::Register<EistrEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut EistrEmmc, 0x32)
    }

    #[doc="Get the *mut pointer for the EISTR_EMMC register."]
    #[inline] pub fn eistr_emmc_mut(&self) -> *mut EistrEmmc { 
        self.eistr_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the EISTR_EMMC register."]
    #[inline] pub fn eistr_emmc_ptr(&self) -> *const EistrEmmc { 
        self.eistr_emmc_reg().ptr()
    }

    #[doc="Read the EISTR_EMMC register."]
    #[inline] pub fn eistr_emmc(&self) -> EistrEmmc { 
        self.eistr_emmc_reg().read()
    }

    #[doc="Write the EISTR_EMMC register."]
    #[inline] pub fn write_eistr_emmc(&self, value: EistrEmmc) -> &Self { 
        self.eistr_emmc_reg().write(value);
        self
    }

    #[doc="Set the EISTR_EMMC register."]
    #[inline] pub fn set_eistr_emmc<F: FnOnce(EistrEmmc) -> EistrEmmc>(&self, f: F) -> &Self {
        self.eistr_emmc_reg().set(f);
        self
    }

    #[doc="Modify the EISTR_EMMC register."]
    #[inline] pub fn with_eistr_emmc<F: FnOnce(EistrEmmc) -> EistrEmmc>(&self, f: F) -> &Self {
        self.eistr_emmc_reg().with(f);
        self
    }

    #[doc="Get the NISTER Register."]
    #[inline] pub fn nister_reg(&self) -> ::bobbin_mcu::register::Register<Nister> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Nister, 0x34)
    }

    #[doc="Get the *mut pointer for the NISTER register."]
    #[inline] pub fn nister_mut(&self) -> *mut Nister { 
        self.nister_reg().ptr()
    }

    #[doc="Get the *const pointer for the NISTER register."]
    #[inline] pub fn nister_ptr(&self) -> *const Nister { 
        self.nister_reg().ptr()
    }

    #[doc="Read the NISTER register."]
    #[inline] pub fn nister(&self) -> Nister { 
        self.nister_reg().read()
    }

    #[doc="Write the NISTER register."]
    #[inline] pub fn write_nister(&self, value: Nister) -> &Self { 
        self.nister_reg().write(value);
        self
    }

    #[doc="Set the NISTER register."]
    #[inline] pub fn set_nister<F: FnOnce(Nister) -> Nister>(&self, f: F) -> &Self {
        self.nister_reg().set(f);
        self
    }

    #[doc="Modify the NISTER register."]
    #[inline] pub fn with_nister<F: FnOnce(Nister) -> Nister>(&self, f: F) -> &Self {
        self.nister_reg().with(f);
        self
    }

    #[doc="Get the NISTER_EMMC Register."]
    #[inline] pub fn nister_emmc_reg(&self) -> ::bobbin_mcu::register::Register<NisterEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut NisterEmmc, 0x34)
    }

    #[doc="Get the *mut pointer for the NISTER_EMMC register."]
    #[inline] pub fn nister_emmc_mut(&self) -> *mut NisterEmmc { 
        self.nister_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the NISTER_EMMC register."]
    #[inline] pub fn nister_emmc_ptr(&self) -> *const NisterEmmc { 
        self.nister_emmc_reg().ptr()
    }

    #[doc="Read the NISTER_EMMC register."]
    #[inline] pub fn nister_emmc(&self) -> NisterEmmc { 
        self.nister_emmc_reg().read()
    }

    #[doc="Write the NISTER_EMMC register."]
    #[inline] pub fn write_nister_emmc(&self, value: NisterEmmc) -> &Self { 
        self.nister_emmc_reg().write(value);
        self
    }

    #[doc="Set the NISTER_EMMC register."]
    #[inline] pub fn set_nister_emmc<F: FnOnce(NisterEmmc) -> NisterEmmc>(&self, f: F) -> &Self {
        self.nister_emmc_reg().set(f);
        self
    }

    #[doc="Modify the NISTER_EMMC register."]
    #[inline] pub fn with_nister_emmc<F: FnOnce(NisterEmmc) -> NisterEmmc>(&self, f: F) -> &Self {
        self.nister_emmc_reg().with(f);
        self
    }

    #[doc="Get the EISTER Register."]
    #[inline] pub fn eister_reg(&self) -> ::bobbin_mcu::register::Register<Eister> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eister, 0x36)
    }

    #[doc="Get the *mut pointer for the EISTER register."]
    #[inline] pub fn eister_mut(&self) -> *mut Eister { 
        self.eister_reg().ptr()
    }

    #[doc="Get the *const pointer for the EISTER register."]
    #[inline] pub fn eister_ptr(&self) -> *const Eister { 
        self.eister_reg().ptr()
    }

    #[doc="Read the EISTER register."]
    #[inline] pub fn eister(&self) -> Eister { 
        self.eister_reg().read()
    }

    #[doc="Write the EISTER register."]
    #[inline] pub fn write_eister(&self, value: Eister) -> &Self { 
        self.eister_reg().write(value);
        self
    }

    #[doc="Set the EISTER register."]
    #[inline] pub fn set_eister<F: FnOnce(Eister) -> Eister>(&self, f: F) -> &Self {
        self.eister_reg().set(f);
        self
    }

    #[doc="Modify the EISTER register."]
    #[inline] pub fn with_eister<F: FnOnce(Eister) -> Eister>(&self, f: F) -> &Self {
        self.eister_reg().with(f);
        self
    }

    #[doc="Get the EISTER_EMMC Register."]
    #[inline] pub fn eister_emmc_reg(&self) -> ::bobbin_mcu::register::Register<EisterEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut EisterEmmc, 0x36)
    }

    #[doc="Get the *mut pointer for the EISTER_EMMC register."]
    #[inline] pub fn eister_emmc_mut(&self) -> *mut EisterEmmc { 
        self.eister_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the EISTER_EMMC register."]
    #[inline] pub fn eister_emmc_ptr(&self) -> *const EisterEmmc { 
        self.eister_emmc_reg().ptr()
    }

    #[doc="Read the EISTER_EMMC register."]
    #[inline] pub fn eister_emmc(&self) -> EisterEmmc { 
        self.eister_emmc_reg().read()
    }

    #[doc="Write the EISTER_EMMC register."]
    #[inline] pub fn write_eister_emmc(&self, value: EisterEmmc) -> &Self { 
        self.eister_emmc_reg().write(value);
        self
    }

    #[doc="Set the EISTER_EMMC register."]
    #[inline] pub fn set_eister_emmc<F: FnOnce(EisterEmmc) -> EisterEmmc>(&self, f: F) -> &Self {
        self.eister_emmc_reg().set(f);
        self
    }

    #[doc="Modify the EISTER_EMMC register."]
    #[inline] pub fn with_eister_emmc<F: FnOnce(EisterEmmc) -> EisterEmmc>(&self, f: F) -> &Self {
        self.eister_emmc_reg().with(f);
        self
    }

    #[doc="Get the NISIER Register."]
    #[inline] pub fn nisier_reg(&self) -> ::bobbin_mcu::register::Register<Nisier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Nisier, 0x38)
    }

    #[doc="Get the *mut pointer for the NISIER register."]
    #[inline] pub fn nisier_mut(&self) -> *mut Nisier { 
        self.nisier_reg().ptr()
    }

    #[doc="Get the *const pointer for the NISIER register."]
    #[inline] pub fn nisier_ptr(&self) -> *const Nisier { 
        self.nisier_reg().ptr()
    }

    #[doc="Read the NISIER register."]
    #[inline] pub fn nisier(&self) -> Nisier { 
        self.nisier_reg().read()
    }

    #[doc="Write the NISIER register."]
    #[inline] pub fn write_nisier(&self, value: Nisier) -> &Self { 
        self.nisier_reg().write(value);
        self
    }

    #[doc="Set the NISIER register."]
    #[inline] pub fn set_nisier<F: FnOnce(Nisier) -> Nisier>(&self, f: F) -> &Self {
        self.nisier_reg().set(f);
        self
    }

    #[doc="Modify the NISIER register."]
    #[inline] pub fn with_nisier<F: FnOnce(Nisier) -> Nisier>(&self, f: F) -> &Self {
        self.nisier_reg().with(f);
        self
    }

    #[doc="Get the NISIER_EMMC Register."]
    #[inline] pub fn nisier_emmc_reg(&self) -> ::bobbin_mcu::register::Register<NisierEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut NisierEmmc, 0x38)
    }

    #[doc="Get the *mut pointer for the NISIER_EMMC register."]
    #[inline] pub fn nisier_emmc_mut(&self) -> *mut NisierEmmc { 
        self.nisier_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the NISIER_EMMC register."]
    #[inline] pub fn nisier_emmc_ptr(&self) -> *const NisierEmmc { 
        self.nisier_emmc_reg().ptr()
    }

    #[doc="Read the NISIER_EMMC register."]
    #[inline] pub fn nisier_emmc(&self) -> NisierEmmc { 
        self.nisier_emmc_reg().read()
    }

    #[doc="Write the NISIER_EMMC register."]
    #[inline] pub fn write_nisier_emmc(&self, value: NisierEmmc) -> &Self { 
        self.nisier_emmc_reg().write(value);
        self
    }

    #[doc="Set the NISIER_EMMC register."]
    #[inline] pub fn set_nisier_emmc<F: FnOnce(NisierEmmc) -> NisierEmmc>(&self, f: F) -> &Self {
        self.nisier_emmc_reg().set(f);
        self
    }

    #[doc="Modify the NISIER_EMMC register."]
    #[inline] pub fn with_nisier_emmc<F: FnOnce(NisierEmmc) -> NisierEmmc>(&self, f: F) -> &Self {
        self.nisier_emmc_reg().with(f);
        self
    }

    #[doc="Get the EISIER Register."]
    #[inline] pub fn eisier_reg(&self) -> ::bobbin_mcu::register::Register<Eisier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eisier, 0x3a)
    }

    #[doc="Get the *mut pointer for the EISIER register."]
    #[inline] pub fn eisier_mut(&self) -> *mut Eisier { 
        self.eisier_reg().ptr()
    }

    #[doc="Get the *const pointer for the EISIER register."]
    #[inline] pub fn eisier_ptr(&self) -> *const Eisier { 
        self.eisier_reg().ptr()
    }

    #[doc="Read the EISIER register."]
    #[inline] pub fn eisier(&self) -> Eisier { 
        self.eisier_reg().read()
    }

    #[doc="Write the EISIER register."]
    #[inline] pub fn write_eisier(&self, value: Eisier) -> &Self { 
        self.eisier_reg().write(value);
        self
    }

    #[doc="Set the EISIER register."]
    #[inline] pub fn set_eisier<F: FnOnce(Eisier) -> Eisier>(&self, f: F) -> &Self {
        self.eisier_reg().set(f);
        self
    }

    #[doc="Modify the EISIER register."]
    #[inline] pub fn with_eisier<F: FnOnce(Eisier) -> Eisier>(&self, f: F) -> &Self {
        self.eisier_reg().with(f);
        self
    }

    #[doc="Get the EISIER_EMMC Register."]
    #[inline] pub fn eisier_emmc_reg(&self) -> ::bobbin_mcu::register::Register<EisierEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut EisierEmmc, 0x3a)
    }

    #[doc="Get the *mut pointer for the EISIER_EMMC register."]
    #[inline] pub fn eisier_emmc_mut(&self) -> *mut EisierEmmc { 
        self.eisier_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the EISIER_EMMC register."]
    #[inline] pub fn eisier_emmc_ptr(&self) -> *const EisierEmmc { 
        self.eisier_emmc_reg().ptr()
    }

    #[doc="Read the EISIER_EMMC register."]
    #[inline] pub fn eisier_emmc(&self) -> EisierEmmc { 
        self.eisier_emmc_reg().read()
    }

    #[doc="Write the EISIER_EMMC register."]
    #[inline] pub fn write_eisier_emmc(&self, value: EisierEmmc) -> &Self { 
        self.eisier_emmc_reg().write(value);
        self
    }

    #[doc="Set the EISIER_EMMC register."]
    #[inline] pub fn set_eisier_emmc<F: FnOnce(EisierEmmc) -> EisierEmmc>(&self, f: F) -> &Self {
        self.eisier_emmc_reg().set(f);
        self
    }

    #[doc="Modify the EISIER_EMMC register."]
    #[inline] pub fn with_eisier_emmc<F: FnOnce(EisierEmmc) -> EisierEmmc>(&self, f: F) -> &Self {
        self.eisier_emmc_reg().with(f);
        self
    }

    #[doc="Get the ACESR Register."]
    #[inline] pub fn acesr_reg(&self) -> ::bobbin_mcu::register::Register<Acesr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Acesr, 0x3c)
    }

    #[doc="Get the *mut pointer for the ACESR register."]
    #[inline] pub fn acesr_mut(&self) -> *mut Acesr { 
        self.acesr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACESR register."]
    #[inline] pub fn acesr_ptr(&self) -> *const Acesr { 
        self.acesr_reg().ptr()
    }

    #[doc="Read the ACESR register."]
    #[inline] pub fn acesr(&self) -> Acesr { 
        self.acesr_reg().read()
    }

    #[doc="Get the HC2R Register."]
    #[inline] pub fn hc2r_reg(&self) -> ::bobbin_mcu::register::Register<Hc2r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hc2r, 0x3e)
    }

    #[doc="Get the *mut pointer for the HC2R register."]
    #[inline] pub fn hc2r_mut(&self) -> *mut Hc2r { 
        self.hc2r_reg().ptr()
    }

    #[doc="Get the *const pointer for the HC2R register."]
    #[inline] pub fn hc2r_ptr(&self) -> *const Hc2r { 
        self.hc2r_reg().ptr()
    }

    #[doc="Read the HC2R register."]
    #[inline] pub fn hc2r(&self) -> Hc2r { 
        self.hc2r_reg().read()
    }

    #[doc="Write the HC2R register."]
    #[inline] pub fn write_hc2r(&self, value: Hc2r) -> &Self { 
        self.hc2r_reg().write(value);
        self
    }

    #[doc="Set the HC2R register."]
    #[inline] pub fn set_hc2r<F: FnOnce(Hc2r) -> Hc2r>(&self, f: F) -> &Self {
        self.hc2r_reg().set(f);
        self
    }

    #[doc="Modify the HC2R register."]
    #[inline] pub fn with_hc2r<F: FnOnce(Hc2r) -> Hc2r>(&self, f: F) -> &Self {
        self.hc2r_reg().with(f);
        self
    }

    #[doc="Get the HC2R_EMMC Register."]
    #[inline] pub fn hc2r_emmc_reg(&self) -> ::bobbin_mcu::register::Register<Hc2rEmmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hc2rEmmc, 0x3e)
    }

    #[doc="Get the *mut pointer for the HC2R_EMMC register."]
    #[inline] pub fn hc2r_emmc_mut(&self) -> *mut Hc2rEmmc { 
        self.hc2r_emmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the HC2R_EMMC register."]
    #[inline] pub fn hc2r_emmc_ptr(&self) -> *const Hc2rEmmc { 
        self.hc2r_emmc_reg().ptr()
    }

    #[doc="Read the HC2R_EMMC register."]
    #[inline] pub fn hc2r_emmc(&self) -> Hc2rEmmc { 
        self.hc2r_emmc_reg().read()
    }

    #[doc="Write the HC2R_EMMC register."]
    #[inline] pub fn write_hc2r_emmc(&self, value: Hc2rEmmc) -> &Self { 
        self.hc2r_emmc_reg().write(value);
        self
    }

    #[doc="Set the HC2R_EMMC register."]
    #[inline] pub fn set_hc2r_emmc<F: FnOnce(Hc2rEmmc) -> Hc2rEmmc>(&self, f: F) -> &Self {
        self.hc2r_emmc_reg().set(f);
        self
    }

    #[doc="Modify the HC2R_EMMC register."]
    #[inline] pub fn with_hc2r_emmc<F: FnOnce(Hc2rEmmc) -> Hc2rEmmc>(&self, f: F) -> &Self {
        self.hc2r_emmc_reg().with(f);
        self
    }

    #[doc="Get the CA0R Register."]
    #[inline] pub fn ca0r_reg(&self) -> ::bobbin_mcu::register::Register<Ca0r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ca0r, 0x40)
    }

    #[doc="Get the *mut pointer for the CA0R register."]
    #[inline] pub fn ca0r_mut(&self) -> *mut Ca0r { 
        self.ca0r_reg().ptr()
    }

    #[doc="Get the *const pointer for the CA0R register."]
    #[inline] pub fn ca0r_ptr(&self) -> *const Ca0r { 
        self.ca0r_reg().ptr()
    }

    #[doc="Read the CA0R register."]
    #[inline] pub fn ca0r(&self) -> Ca0r { 
        self.ca0r_reg().read()
    }

    #[doc="Get the CA1R Register."]
    #[inline] pub fn ca1r_reg(&self) -> ::bobbin_mcu::register::Register<Ca1r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ca1r, 0x44)
    }

    #[doc="Get the *mut pointer for the CA1R register."]
    #[inline] pub fn ca1r_mut(&self) -> *mut Ca1r { 
        self.ca1r_reg().ptr()
    }

    #[doc="Get the *const pointer for the CA1R register."]
    #[inline] pub fn ca1r_ptr(&self) -> *const Ca1r { 
        self.ca1r_reg().ptr()
    }

    #[doc="Read the CA1R register."]
    #[inline] pub fn ca1r(&self) -> Ca1r { 
        self.ca1r_reg().read()
    }

    #[doc="Get the MCCAR Register."]
    #[inline] pub fn mccar_reg(&self) -> ::bobbin_mcu::register::Register<Mccar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mccar, 0x48)
    }

    #[doc="Get the *mut pointer for the MCCAR register."]
    #[inline] pub fn mccar_mut(&self) -> *mut Mccar { 
        self.mccar_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCCAR register."]
    #[inline] pub fn mccar_ptr(&self) -> *const Mccar { 
        self.mccar_reg().ptr()
    }

    #[doc="Read the MCCAR register."]
    #[inline] pub fn mccar(&self) -> Mccar { 
        self.mccar_reg().read()
    }

    #[doc="Get the FERACES Register."]
    #[inline] pub fn feraces_reg(&self) -> ::bobbin_mcu::register::Register<Feraces> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Feraces, 0x50)
    }

    #[doc="Get the *mut pointer for the FERACES register."]
    #[inline] pub fn feraces_mut(&self) -> *mut Feraces { 
        self.feraces_reg().ptr()
    }

    #[doc="Get the *const pointer for the FERACES register."]
    #[inline] pub fn feraces_ptr(&self) -> *const Feraces { 
        self.feraces_reg().ptr()
    }

    #[doc="Write the FERACES register."]
    #[inline] pub fn write_feraces(&self, value: Feraces) -> &Self { 
        self.feraces_reg().write(value);
        self
    }

    #[doc="Set the FERACES register."]
    #[inline] pub fn set_feraces<F: FnOnce(Feraces) -> Feraces>(&self, f: F) -> &Self {
        self.feraces_reg().set(f);
        self
    }

    #[doc="Get the FEREIS Register."]
    #[inline] pub fn fereis_reg(&self) -> ::bobbin_mcu::register::Register<Fereis> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fereis, 0x52)
    }

    #[doc="Get the *mut pointer for the FEREIS register."]
    #[inline] pub fn fereis_mut(&self) -> *mut Fereis { 
        self.fereis_reg().ptr()
    }

    #[doc="Get the *const pointer for the FEREIS register."]
    #[inline] pub fn fereis_ptr(&self) -> *const Fereis { 
        self.fereis_reg().ptr()
    }

    #[doc="Write the FEREIS register."]
    #[inline] pub fn write_fereis(&self, value: Fereis) -> &Self { 
        self.fereis_reg().write(value);
        self
    }

    #[doc="Set the FEREIS register."]
    #[inline] pub fn set_fereis<F: FnOnce(Fereis) -> Fereis>(&self, f: F) -> &Self {
        self.fereis_reg().set(f);
        self
    }

    #[doc="Get the AESR Register."]
    #[inline] pub fn aesr_reg(&self) -> ::bobbin_mcu::register::Register<Aesr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Aesr, 0x54)
    }

    #[doc="Get the *mut pointer for the AESR register."]
    #[inline] pub fn aesr_mut(&self) -> *mut Aesr { 
        self.aesr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AESR register."]
    #[inline] pub fn aesr_ptr(&self) -> *const Aesr { 
        self.aesr_reg().ptr()
    }

    #[doc="Read the AESR register."]
    #[inline] pub fn aesr(&self) -> Aesr { 
        self.aesr_reg().read()
    }

    #[doc="Get the ASAR Register."]
    #[inline] pub fn asar_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Asar, ::bobbin_bits::R1> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Asar, 0x58, 0x4)
    }

    #[doc="Get the *mut pointer for the ASAR register."]
    #[inline] pub fn asar_mut<I: Into<::bobbin_bits::R1>>(&self, index: I) -> *mut Asar { 
        self.asar_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the ASAR register."]
    #[inline] pub fn asar_ptr<I: Into<::bobbin_bits::R1>>(&self, index: I) -> *const Asar { 
        self.asar_reg().ptr(index.into())
    }

    #[doc="Read the ASAR register."]
    #[inline] pub fn asar<I: Into<::bobbin_bits::R1>>(&self, index: I) -> Asar { 
        self.asar_reg().read(index.into())
    }

    #[doc="Write the ASAR register."]
    #[inline] pub fn write_asar<I: Into<::bobbin_bits::R1>>(&self, index: I, value: Asar) -> &Self {
        self.asar_reg().write(index.into(), value);
        self
    }

    #[doc="Set the ASAR register."]
    #[inline] pub fn set_asar<I: Into<::bobbin_bits::R1>, F: FnOnce(Asar) -> Asar>(&self, index: I, f: F) -> &Self {
        self.asar_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the ASAR register."]
    #[inline] pub fn with_asar<I: Into<::bobbin_bits::R1> + Copy, F: FnOnce(Asar) -> Asar>(&self, index: I, f: F) -> &Self {
        self.asar_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PVR Register."]
    #[inline] pub fn pvr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pvr, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pvr, 0x60, 0x2)
    }

    #[doc="Get the *mut pointer for the PVR register."]
    #[inline] pub fn pvr_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Pvr { 
        self.pvr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PVR register."]
    #[inline] pub fn pvr_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Pvr { 
        self.pvr_reg().ptr(index.into())
    }

    #[doc="Read the PVR register."]
    #[inline] pub fn pvr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Pvr { 
        self.pvr_reg().read(index.into())
    }

    #[doc="Write the PVR register."]
    #[inline] pub fn write_pvr<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Pvr) -> &Self {
        self.pvr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PVR register."]
    #[inline] pub fn set_pvr<I: Into<::bobbin_bits::R8>, F: FnOnce(Pvr) -> Pvr>(&self, index: I, f: F) -> &Self {
        self.pvr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PVR register."]
    #[inline] pub fn with_pvr<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Pvr) -> Pvr>(&self, index: I, f: F) -> &Self {
        self.pvr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the SISR Register."]
    #[inline] pub fn sisr_reg(&self) -> ::bobbin_mcu::register::Register<Sisr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sisr, 0xfc)
    }

    #[doc="Get the *mut pointer for the SISR register."]
    #[inline] pub fn sisr_mut(&self) -> *mut Sisr { 
        self.sisr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SISR register."]
    #[inline] pub fn sisr_ptr(&self) -> *const Sisr { 
        self.sisr_reg().ptr()
    }

    #[doc="Read the SISR register."]
    #[inline] pub fn sisr(&self) -> Sisr { 
        self.sisr_reg().read()
    }

    #[doc="Get the HCVR Register."]
    #[inline] pub fn hcvr_reg(&self) -> ::bobbin_mcu::register::Register<Hcvr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hcvr, 0xfe)
    }

    #[doc="Get the *mut pointer for the HCVR register."]
    #[inline] pub fn hcvr_mut(&self) -> *mut Hcvr { 
        self.hcvr_reg().ptr()
    }

    #[doc="Get the *const pointer for the HCVR register."]
    #[inline] pub fn hcvr_ptr(&self) -> *const Hcvr { 
        self.hcvr_reg().ptr()
    }

    #[doc="Read the HCVR register."]
    #[inline] pub fn hcvr(&self) -> Hcvr { 
        self.hcvr_reg().read()
    }

    #[doc="Get the MC1R Register."]
    #[inline] pub fn mc1r_reg(&self) -> ::bobbin_mcu::register::Register<Mc1r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mc1r, 0x204)
    }

    #[doc="Get the *mut pointer for the MC1R register."]
    #[inline] pub fn mc1r_mut(&self) -> *mut Mc1r { 
        self.mc1r_reg().ptr()
    }

    #[doc="Get the *const pointer for the MC1R register."]
    #[inline] pub fn mc1r_ptr(&self) -> *const Mc1r { 
        self.mc1r_reg().ptr()
    }

    #[doc="Read the MC1R register."]
    #[inline] pub fn mc1r(&self) -> Mc1r { 
        self.mc1r_reg().read()
    }

    #[doc="Write the MC1R register."]
    #[inline] pub fn write_mc1r(&self, value: Mc1r) -> &Self { 
        self.mc1r_reg().write(value);
        self
    }

    #[doc="Set the MC1R register."]
    #[inline] pub fn set_mc1r<F: FnOnce(Mc1r) -> Mc1r>(&self, f: F) -> &Self {
        self.mc1r_reg().set(f);
        self
    }

    #[doc="Modify the MC1R register."]
    #[inline] pub fn with_mc1r<F: FnOnce(Mc1r) -> Mc1r>(&self, f: F) -> &Self {
        self.mc1r_reg().with(f);
        self
    }

    #[doc="Get the MC2R Register."]
    #[inline] pub fn mc2r_reg(&self) -> ::bobbin_mcu::register::Register<Mc2r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mc2r, 0x205)
    }

    #[doc="Get the *mut pointer for the MC2R register."]
    #[inline] pub fn mc2r_mut(&self) -> *mut Mc2r { 
        self.mc2r_reg().ptr()
    }

    #[doc="Get the *const pointer for the MC2R register."]
    #[inline] pub fn mc2r_ptr(&self) -> *const Mc2r { 
        self.mc2r_reg().ptr()
    }

    #[doc="Write the MC2R register."]
    #[inline] pub fn write_mc2r(&self, value: Mc2r) -> &Self { 
        self.mc2r_reg().write(value);
        self
    }

    #[doc="Set the MC2R register."]
    #[inline] pub fn set_mc2r<F: FnOnce(Mc2r) -> Mc2r>(&self, f: F) -> &Self {
        self.mc2r_reg().set(f);
        self
    }

    #[doc="Get the ACR Register."]
    #[inline] pub fn acr_reg(&self) -> ::bobbin_mcu::register::Register<Acr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Acr, 0x208)
    }

    #[doc="Get the *mut pointer for the ACR register."]
    #[inline] pub fn acr_mut(&self) -> *mut Acr { 
        self.acr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACR register."]
    #[inline] pub fn acr_ptr(&self) -> *const Acr { 
        self.acr_reg().ptr()
    }

    #[doc="Read the ACR register."]
    #[inline] pub fn acr(&self) -> Acr { 
        self.acr_reg().read()
    }

    #[doc="Write the ACR register."]
    #[inline] pub fn write_acr(&self, value: Acr) -> &Self { 
        self.acr_reg().write(value);
        self
    }

    #[doc="Set the ACR register."]
    #[inline] pub fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        self.acr_reg().set(f);
        self
    }

    #[doc="Modify the ACR register."]
    #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        self.acr_reg().with(f);
        self
    }

    #[doc="Get the CC2R Register."]
    #[inline] pub fn cc2r_reg(&self) -> ::bobbin_mcu::register::Register<Cc2r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cc2r, 0x20c)
    }

    #[doc="Get the *mut pointer for the CC2R register."]
    #[inline] pub fn cc2r_mut(&self) -> *mut Cc2r { 
        self.cc2r_reg().ptr()
    }

    #[doc="Get the *const pointer for the CC2R register."]
    #[inline] pub fn cc2r_ptr(&self) -> *const Cc2r { 
        self.cc2r_reg().ptr()
    }

    #[doc="Read the CC2R register."]
    #[inline] pub fn cc2r(&self) -> Cc2r { 
        self.cc2r_reg().read()
    }

    #[doc="Write the CC2R register."]
    #[inline] pub fn write_cc2r(&self, value: Cc2r) -> &Self { 
        self.cc2r_reg().write(value);
        self
    }

    #[doc="Set the CC2R register."]
    #[inline] pub fn set_cc2r<F: FnOnce(Cc2r) -> Cc2r>(&self, f: F) -> &Self {
        self.cc2r_reg().set(f);
        self
    }

    #[doc="Modify the CC2R register."]
    #[inline] pub fn with_cc2r<F: FnOnce(Cc2r) -> Cc2r>(&self, f: F) -> &Self {
        self.cc2r_reg().with(f);
        self
    }

    #[doc="Get the CACR Register."]
    #[inline] pub fn cacr_reg(&self) -> ::bobbin_mcu::register::Register<Cacr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cacr, 0x230)
    }

    #[doc="Get the *mut pointer for the CACR register."]
    #[inline] pub fn cacr_mut(&self) -> *mut Cacr { 
        self.cacr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CACR register."]
    #[inline] pub fn cacr_ptr(&self) -> *const Cacr { 
        self.cacr_reg().ptr()
    }

    #[doc="Read the CACR register."]
    #[inline] pub fn cacr(&self) -> Cacr { 
        self.cacr_reg().read()
    }

    #[doc="Write the CACR register."]
    #[inline] pub fn write_cacr(&self, value: Cacr) -> &Self { 
        self.cacr_reg().write(value);
        self
    }

    #[doc="Set the CACR register."]
    #[inline] pub fn set_cacr<F: FnOnce(Cacr) -> Cacr>(&self, f: F) -> &Self {
        self.cacr_reg().set(f);
        self
    }

    #[doc="Modify the CACR register."]
    #[inline] pub fn with_cacr<F: FnOnce(Cacr) -> Cacr>(&self, f: F) -> &Self {
        self.cacr_reg().with(f);
        self
    }

    #[doc="Get the DBGR Register."]
    #[inline] pub fn dbgr_reg(&self) -> ::bobbin_mcu::register::Register<Dbgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgr, 0x234)
    }

    #[doc="Get the *mut pointer for the DBGR register."]
    #[inline] pub fn dbgr_mut(&self) -> *mut Dbgr { 
        self.dbgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DBGR register."]
    #[inline] pub fn dbgr_ptr(&self) -> *const Dbgr { 
        self.dbgr_reg().ptr()
    }

    #[doc="Read the DBGR register."]
    #[inline] pub fn dbgr(&self) -> Dbgr { 
        self.dbgr_reg().read()
    }

    #[doc="Write the DBGR register."]
    #[inline] pub fn write_dbgr(&self, value: Dbgr) -> &Self { 
        self.dbgr_reg().write(value);
        self
    }

    #[doc="Set the DBGR register."]
    #[inline] pub fn set_dbgr<F: FnOnce(Dbgr) -> Dbgr>(&self, f: F) -> &Self {
        self.dbgr_reg().set(f);
        self
    }

    #[doc="Modify the DBGR register."]
    #[inline] pub fn with_dbgr<F: FnOnce(Dbgr) -> Dbgr>(&self, f: F) -> &Self {
        self.dbgr_reg().with(f);
        self
    }

}

#[doc="SDMA System Address / Argument 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssar(pub u32);
impl Ssar {
    #[doc="SDMA System Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ssar {
    #[inline]
    fn from(other: u32) -> Self {
         Ssar(other)
    }
}

impl ::core::fmt::Display for Ssar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDMA System Address / Argument 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SsarCmd23(pub u32);
impl SsarCmd23 {
    #[doc="Argument 2"]
    #[inline] pub fn arg2(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ARG2 != 0"]
    #[inline] pub fn test_arg2(&self) -> bool {
        self.arg2() != 0
    }

    #[doc="Sets the ARG2 field."]
    #[inline] pub fn set_arg2<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for SsarCmd23 {
    #[inline]
    fn from(other: u32) -> Self {
         SsarCmd23(other)
    }
}

impl ::core::fmt::Display for SsarCmd23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for SsarCmd23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Block Size"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bsr(pub u16);
impl Bsr {
    #[doc="Transfer Block Size"]
    #[inline] pub fn blocksize(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if BLOCKSIZE != 0"]
    #[inline] pub fn test_blocksize(&self) -> bool {
        self.blocksize() != 0
    }

    #[doc="Sets the BLOCKSIZE field."]
    #[inline] pub fn set_blocksize<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SDMA Buffer Boundary"]
    #[inline] pub fn boundary(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if BOUNDARY != 0"]
    #[inline] pub fn test_boundary(&self) -> bool {
        self.boundary() != 0
    }

    #[doc="Sets the BOUNDARY field."]
    #[inline] pub fn set_boundary<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Bsr {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.blocksize() != 0 { try!(write!(f, " blocksize=0x{:x}", self.blocksize()))}
        if self.boundary() != 0 { try!(write!(f, " boundary=0x{:x}", self.boundary()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Block Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr(pub u16);
impl Bcr {
    #[doc="Blocks Count for Current Transfer"]
    #[inline] pub fn bcnt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if BCNT != 0"]
    #[inline] pub fn test_bcnt(&self) -> bool {
        self.bcnt() != 0
    }

    #[doc="Sets the BCNT field."]
    #[inline] pub fn set_bcnt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Bcr {
    #[inline]
    fn from(other: u16) -> Self {
         Bcr(other)
    }
}

impl ::core::fmt::Display for Bcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bcnt() != 0 { try!(write!(f, " bcnt=0x{:x}", self.bcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Argument 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Arg1r(pub u32);
impl Arg1r {
    #[doc="Argument 1"]
    #[inline] pub fn arg(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ARG != 0"]
    #[inline] pub fn test_arg(&self) -> bool {
        self.arg() != 0
    }

    #[doc="Sets the ARG field."]
    #[inline] pub fn set_arg<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Arg1r {
    #[inline]
    fn from(other: u32) -> Self {
         Arg1r(other)
    }
}

impl ::core::fmt::Display for Arg1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Arg1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transfer Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmr(pub u16);
impl Tmr {
    #[doc="DMA Enable"]
    #[inline] pub fn dmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Block Count Enable"]
    #[inline] pub fn bcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BCEN != 0"]
    #[inline] pub fn test_bcen(&self) -> bool {
        self.bcen() != 0
    }

    #[doc="Sets the BCEN field."]
    #[inline] pub fn set_bcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Auto Command Enable"]
    #[inline] pub fn acmden(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if ACMDEN != 0"]
    #[inline] pub fn test_acmden(&self) -> bool {
        self.acmden() != 0
    }

    #[doc="Sets the ACMDEN field."]
    #[inline] pub fn set_acmden<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data Transfer Direction Selection"]
    #[inline] pub fn dtdsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DTDSEL != 0"]
    #[inline] pub fn test_dtdsel(&self) -> bool {
        self.dtdsel() != 0
    }

    #[doc="Sets the DTDSEL field."]
    #[inline] pub fn set_dtdsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Multi/Single Block Selection"]
    #[inline] pub fn msbsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MSBSEL != 0"]
    #[inline] pub fn test_msbsel(&self) -> bool {
        self.msbsel() != 0
    }

    #[doc="Sets the MSBSEL field."]
    #[inline] pub fn set_msbsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u16> for Tmr {
    #[inline]
    fn from(other: u16) -> Self {
         Tmr(other)
    }
}

impl ::core::fmt::Display for Tmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.bcen() != 0 { try!(write!(f, " bcen"))}
        if self.acmden() != 0 { try!(write!(f, " acmden=0x{:x}", self.acmden()))}
        if self.dtdsel() != 0 { try!(write!(f, " dtdsel"))}
        if self.msbsel() != 0 { try!(write!(f, " msbsel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Command"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u16);
impl Cr {
    #[doc="Response Type"]
    #[inline] pub fn resptyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if RESPTYP != 0"]
    #[inline] pub fn test_resptyp(&self) -> bool {
        self.resptyp() != 0
    }

    #[doc="Sets the RESPTYP field."]
    #[inline] pub fn set_resptyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command CRC Check Enable"]
    #[inline] pub fn cmdccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMDCCEN != 0"]
    #[inline] pub fn test_cmdccen(&self) -> bool {
        self.cmdccen() != 0
    }

    #[doc="Sets the CMDCCEN field."]
    #[inline] pub fn set_cmdccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Command Index Check Enable"]
    #[inline] pub fn cmdicen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CMDICEN != 0"]
    #[inline] pub fn test_cmdicen(&self) -> bool {
        self.cmdicen() != 0
    }

    #[doc="Sets the CMDICEN field."]
    #[inline] pub fn set_cmdicen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Present Select"]
    #[inline] pub fn dpsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DPSEL != 0"]
    #[inline] pub fn test_dpsel(&self) -> bool {
        self.dpsel() != 0
    }

    #[doc="Sets the DPSEL field."]
    #[inline] pub fn set_dpsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Command Type"]
    #[inline] pub fn cmdtyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CMDTYP != 0"]
    #[inline] pub fn test_cmdtyp(&self) -> bool {
        self.cmdtyp() != 0
    }

    #[doc="Sets the CMDTYP field."]
    #[inline] pub fn set_cmdtyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Command Index"]
    #[inline] pub fn cmdidx(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if CMDIDX != 0"]
    #[inline] pub fn test_cmdidx(&self) -> bool {
        self.cmdidx() != 0
    }

    #[doc="Sets the CMDIDX field."]
    #[inline] pub fn set_cmdidx<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Cr {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.resptyp() != 0 { try!(write!(f, " resptyp=0x{:x}", self.resptyp()))}
        if self.cmdccen() != 0 { try!(write!(f, " cmdccen"))}
        if self.cmdicen() != 0 { try!(write!(f, " cmdicen"))}
        if self.dpsel() != 0 { try!(write!(f, " dpsel"))}
        if self.cmdtyp() != 0 { try!(write!(f, " cmdtyp=0x{:x}", self.cmdtyp()))}
        if self.cmdidx() != 0 { try!(write!(f, " cmdidx=0x{:x}", self.cmdidx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Response"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rr(pub u32);
impl Rr {
    #[doc="Command Response"]
    #[inline] pub fn cmdresp(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CMDRESP != 0"]
    #[inline] pub fn test_cmdresp(&self) -> bool {
        self.cmdresp() != 0
    }

    #[doc="Sets the CMDRESP field."]
    #[inline] pub fn set_cmdresp<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rr {
    #[inline]
    fn from(other: u32) -> Self {
         Rr(other)
    }
}

impl ::core::fmt::Display for Rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Buffer Data Port"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdpr(pub u32);
impl Bdpr {
    #[doc="Buffer Data"]
    #[inline] pub fn bufdata(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if BUFDATA != 0"]
    #[inline] pub fn test_bufdata(&self) -> bool {
        self.bufdata() != 0
    }

    #[doc="Sets the BUFDATA field."]
    #[inline] pub fn set_bufdata<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bdpr {
    #[inline]
    fn from(other: u32) -> Self {
         Bdpr(other)
    }
}

impl ::core::fmt::Display for Bdpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Present State"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc="Command Inhibit (CMD)"]
    #[inline] pub fn cmdinhc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDINHC != 0"]
    #[inline] pub fn test_cmdinhc(&self) -> bool {
        self.cmdinhc() != 0
    }

    #[doc="Sets the CMDINHC field."]
    #[inline] pub fn set_cmdinhc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command Inhibit (DAT)"]
    #[inline] pub fn cmdinhd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDINHD != 0"]
    #[inline] pub fn test_cmdinhd(&self) -> bool {
        self.cmdinhd() != 0
    }

    #[doc="Sets the CMDINHD field."]
    #[inline] pub fn set_cmdinhd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DAT Line Active"]
    #[inline] pub fn dlact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DLACT != 0"]
    #[inline] pub fn test_dlact(&self) -> bool {
        self.dlact() != 0
    }

    #[doc="Sets the DLACT field."]
    #[inline] pub fn set_dlact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Re-Tuning Request"]
    #[inline] pub fn rtreq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTREQ != 0"]
    #[inline] pub fn test_rtreq(&self) -> bool {
        self.rtreq() != 0
    }

    #[doc="Sets the RTREQ field."]
    #[inline] pub fn set_rtreq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Write Transfer Active"]
    #[inline] pub fn wtact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WTACT != 0"]
    #[inline] pub fn test_wtact(&self) -> bool {
        self.wtact() != 0
    }

    #[doc="Sets the WTACT field."]
    #[inline] pub fn set_wtact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Read Transfer Active"]
    #[inline] pub fn rtact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RTACT != 0"]
    #[inline] pub fn test_rtact(&self) -> bool {
        self.rtact() != 0
    }

    #[doc="Sets the RTACT field."]
    #[inline] pub fn set_rtact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Buffer Write Enable"]
    #[inline] pub fn bufwren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BUFWREN != 0"]
    #[inline] pub fn test_bufwren(&self) -> bool {
        self.bufwren() != 0
    }

    #[doc="Sets the BUFWREN field."]
    #[inline] pub fn set_bufwren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Buffer Read Enable"]
    #[inline] pub fn bufrden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BUFRDEN != 0"]
    #[inline] pub fn test_bufrden(&self) -> bool {
        self.bufrden() != 0
    }

    #[doc="Sets the BUFRDEN field."]
    #[inline] pub fn set_bufrden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Card Inserted"]
    #[inline] pub fn cardins(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CARDINS != 0"]
    #[inline] pub fn test_cardins(&self) -> bool {
        self.cardins() != 0
    }

    #[doc="Sets the CARDINS field."]
    #[inline] pub fn set_cardins<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Card State Stable"]
    #[inline] pub fn cardss(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CARDSS != 0"]
    #[inline] pub fn test_cardss(&self) -> bool {
        self.cardss() != 0
    }

    #[doc="Sets the CARDSS field."]
    #[inline] pub fn set_cardss<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Card Detect Pin Level"]
    #[inline] pub fn carddpl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CARDDPL != 0"]
    #[inline] pub fn test_carddpl(&self) -> bool {
        self.carddpl() != 0
    }

    #[doc="Sets the CARDDPL field."]
    #[inline] pub fn set_carddpl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Write Protect Pin Level"]
    #[inline] pub fn wrppl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if WRPPL != 0"]
    #[inline] pub fn test_wrppl(&self) -> bool {
        self.wrppl() != 0
    }

    #[doc="Sets the WRPPL field."]
    #[inline] pub fn set_wrppl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DAT[3:0] Line Level"]
    #[inline] pub fn datll(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if DATLL != 0"]
    #[inline] pub fn test_datll(&self) -> bool {
        self.datll() != 0
    }

    #[doc="Sets the DATLL field."]
    #[inline] pub fn set_datll<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="CMD Line Level"]
    #[inline] pub fn cmdll(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CMDLL != 0"]
    #[inline] pub fn test_cmdll(&self) -> bool {
        self.cmdll() != 0
    }

    #[doc="Sets the CMDLL field."]
    #[inline] pub fn set_cmdll<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Psr {
    #[inline]
    fn from(other: u32) -> Self {
         Psr(other)
    }
}

impl ::core::fmt::Display for Psr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdinhc() != 0 { try!(write!(f, " cmdinhc"))}
        if self.cmdinhd() != 0 { try!(write!(f, " cmdinhd"))}
        if self.dlact() != 0 { try!(write!(f, " dlact"))}
        if self.rtreq() != 0 { try!(write!(f, " rtreq"))}
        if self.wtact() != 0 { try!(write!(f, " wtact"))}
        if self.rtact() != 0 { try!(write!(f, " rtact"))}
        if self.bufwren() != 0 { try!(write!(f, " bufwren"))}
        if self.bufrden() != 0 { try!(write!(f, " bufrden"))}
        if self.cardins() != 0 { try!(write!(f, " cardins"))}
        if self.cardss() != 0 { try!(write!(f, " cardss"))}
        if self.carddpl() != 0 { try!(write!(f, " carddpl"))}
        if self.wrppl() != 0 { try!(write!(f, " wrppl"))}
        if self.datll() != 0 { try!(write!(f, " datll=0x{:x}", self.datll()))}
        if self.cmdll() != 0 { try!(write!(f, " cmdll"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Host Control 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hc1r(pub u8);
impl Hc1r {
    #[doc="LED Control"]
    #[inline] pub fn ledctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LEDCTRL != 0"]
    #[inline] pub fn test_ledctrl(&self) -> bool {
        self.ledctrl() != 0
    }

    #[doc="Sets the LEDCTRL field."]
    #[inline] pub fn set_ledctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Width"]
    #[inline] pub fn dw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DW != 0"]
    #[inline] pub fn test_dw(&self) -> bool {
        self.dw() != 0
    }

    #[doc="Sets the DW field."]
    #[inline] pub fn set_dw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="High Speed Enable"]
    #[inline] pub fn hsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSEN != 0"]
    #[inline] pub fn test_hsen(&self) -> bool {
        self.hsen() != 0
    }

    #[doc="Sets the HSEN field."]
    #[inline] pub fn set_hsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA Select"]
    #[inline] pub fn dmasel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if DMASEL != 0"]
    #[inline] pub fn test_dmasel(&self) -> bool {
        self.dmasel() != 0
    }

    #[doc="Sets the DMASEL field."]
    #[inline] pub fn set_dmasel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Card Detect Test Level"]
    #[inline] pub fn carddtl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CARDDTL != 0"]
    #[inline] pub fn test_carddtl(&self) -> bool {
        self.carddtl() != 0
    }

    #[doc="Sets the CARDDTL field."]
    #[inline] pub fn set_carddtl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Card Detect Signal Selection"]
    #[inline] pub fn carddsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CARDDSEL != 0"]
    #[inline] pub fn test_carddsel(&self) -> bool {
        self.carddsel() != 0
    }

    #[doc="Sets the CARDDSEL field."]
    #[inline] pub fn set_carddsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Hc1r {
    #[inline]
    fn from(other: u8) -> Self {
         Hc1r(other)
    }
}

impl ::core::fmt::Display for Hc1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hc1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ledctrl() != 0 { try!(write!(f, " ledctrl"))}
        if self.dw() != 0 { try!(write!(f, " dw"))}
        if self.hsen() != 0 { try!(write!(f, " hsen"))}
        if self.dmasel() != 0 { try!(write!(f, " dmasel=0x{:x}", self.dmasel()))}
        if self.carddtl() != 0 { try!(write!(f, " carddtl"))}
        if self.carddsel() != 0 { try!(write!(f, " carddsel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Host Control 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hc1rEmmc(pub u8);
impl Hc1rEmmc {
    #[doc="Data Width"]
    #[inline] pub fn dw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DW != 0"]
    #[inline] pub fn test_dw(&self) -> bool {
        self.dw() != 0
    }

    #[doc="Sets the DW field."]
    #[inline] pub fn set_dw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="High Speed Enable"]
    #[inline] pub fn hsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSEN != 0"]
    #[inline] pub fn test_hsen(&self) -> bool {
        self.hsen() != 0
    }

    #[doc="Sets the HSEN field."]
    #[inline] pub fn set_hsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA Select"]
    #[inline] pub fn dmasel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if DMASEL != 0"]
    #[inline] pub fn test_dmasel(&self) -> bool {
        self.dmasel() != 0
    }

    #[doc="Sets the DMASEL field."]
    #[inline] pub fn set_dmasel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Hc1rEmmc {
    #[inline]
    fn from(other: u8) -> Self {
         Hc1rEmmc(other)
    }
}

impl ::core::fmt::Display for Hc1rEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hc1rEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dw() != 0 { try!(write!(f, " dw"))}
        if self.hsen() != 0 { try!(write!(f, " hsen"))}
        if self.dmasel() != 0 { try!(write!(f, " dmasel=0x{:x}", self.dmasel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcr(pub u8);
impl Pcr {
    #[doc="SD Bus Power"]
    #[inline] pub fn sdbpwr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SDBPWR != 0"]
    #[inline] pub fn test_sdbpwr(&self) -> bool {
        self.sdbpwr() != 0
    }

    #[doc="Sets the SDBPWR field."]
    #[inline] pub fn set_sdbpwr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SD Bus Voltage Select"]
    #[inline] pub fn sdbvsel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if SDBVSEL != 0"]
    #[inline] pub fn test_sdbvsel(&self) -> bool {
        self.sdbvsel() != 0
    }

    #[doc="Sets the SDBVSEL field."]
    #[inline] pub fn set_sdbvsel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Pcr {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.sdbpwr() != 0 { try!(write!(f, " sdbpwr"))}
        if self.sdbvsel() != 0 { try!(write!(f, " sdbvsel=0x{:x}", self.sdbvsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Block Gap Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bgcr(pub u8);
impl Bgcr {
    #[doc="Stop at Block Gap Request"]
    #[inline] pub fn stpbgr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STPBGR != 0"]
    #[inline] pub fn test_stpbgr(&self) -> bool {
        self.stpbgr() != 0
    }

    #[doc="Sets the STPBGR field."]
    #[inline] pub fn set_stpbgr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Continue Request"]
    #[inline] pub fn contr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CONTR != 0"]
    #[inline] pub fn test_contr(&self) -> bool {
        self.contr() != 0
    }

    #[doc="Sets the CONTR field."]
    #[inline] pub fn set_contr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Read Wait Control"]
    #[inline] pub fn rwctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RWCTRL != 0"]
    #[inline] pub fn test_rwctrl(&self) -> bool {
        self.rwctrl() != 0
    }

    #[doc="Sets the RWCTRL field."]
    #[inline] pub fn set_rwctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Interrupt at Block Gap"]
    #[inline] pub fn intbg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if INTBG != 0"]
    #[inline] pub fn test_intbg(&self) -> bool {
        self.intbg() != 0
    }

    #[doc="Sets the INTBG field."]
    #[inline] pub fn set_intbg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Bgcr {
    #[inline]
    fn from(other: u8) -> Self {
         Bgcr(other)
    }
}

impl ::core::fmt::Display for Bgcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bgcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stpbgr() != 0 { try!(write!(f, " stpbgr"))}
        if self.contr() != 0 { try!(write!(f, " contr"))}
        if self.rwctrl() != 0 { try!(write!(f, " rwctrl"))}
        if self.intbg() != 0 { try!(write!(f, " intbg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Block Gap Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BgcrEmmc(pub u8);
impl BgcrEmmc {
    #[doc="Stop at Block Gap Request"]
    #[inline] pub fn stpbgr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STPBGR != 0"]
    #[inline] pub fn test_stpbgr(&self) -> bool {
        self.stpbgr() != 0
    }

    #[doc="Sets the STPBGR field."]
    #[inline] pub fn set_stpbgr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Continue Request"]
    #[inline] pub fn contr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CONTR != 0"]
    #[inline] pub fn test_contr(&self) -> bool {
        self.contr() != 0
    }

    #[doc="Sets the CONTR field."]
    #[inline] pub fn set_contr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for BgcrEmmc {
    #[inline]
    fn from(other: u8) -> Self {
         BgcrEmmc(other)
    }
}

impl ::core::fmt::Display for BgcrEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for BgcrEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stpbgr() != 0 { try!(write!(f, " stpbgr"))}
        if self.contr() != 0 { try!(write!(f, " contr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Wakeup Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wcr(pub u8);
impl Wcr {
    #[doc="Wakeup Event Enable on Card Interrupt"]
    #[inline] pub fn wkencint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WKENCINT != 0"]
    #[inline] pub fn test_wkencint(&self) -> bool {
        self.wkencint() != 0
    }

    #[doc="Sets the WKENCINT field."]
    #[inline] pub fn set_wkencint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Wakeup Event Enable on Card Insertion"]
    #[inline] pub fn wkencins(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WKENCINS != 0"]
    #[inline] pub fn test_wkencins(&self) -> bool {
        self.wkencins() != 0
    }

    #[doc="Sets the WKENCINS field."]
    #[inline] pub fn set_wkencins<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wakeup Event Enable on Card Removal"]
    #[inline] pub fn wkencrem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WKENCREM != 0"]
    #[inline] pub fn test_wkencrem(&self) -> bool {
        self.wkencrem() != 0
    }

    #[doc="Sets the WKENCREM field."]
    #[inline] pub fn set_wkencrem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Wcr {
    #[inline]
    fn from(other: u8) -> Self {
         Wcr(other)
    }
}

impl ::core::fmt::Display for Wcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wkencint() != 0 { try!(write!(f, " wkencint"))}
        if self.wkencins() != 0 { try!(write!(f, " wkencins"))}
        if self.wkencrem() != 0 { try!(write!(f, " wkencrem"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u16);
impl Ccr {
    #[doc="Internal Clock Enable"]
    #[inline] pub fn intclken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INTCLKEN != 0"]
    #[inline] pub fn test_intclken(&self) -> bool {
        self.intclken() != 0
    }

    #[doc="Sets the INTCLKEN field."]
    #[inline] pub fn set_intclken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Internal Clock Stable"]
    #[inline] pub fn intclks(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INTCLKS != 0"]
    #[inline] pub fn test_intclks(&self) -> bool {
        self.intclks() != 0
    }

    #[doc="Sets the INTCLKS field."]
    #[inline] pub fn set_intclks<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SD Clock Enable"]
    #[inline] pub fn sdclken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SDCLKEN != 0"]
    #[inline] pub fn test_sdclken(&self) -> bool {
        self.sdclken() != 0
    }

    #[doc="Sets the SDCLKEN field."]
    #[inline] pub fn set_sdclken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clock Generator Select"]
    #[inline] pub fn clkgsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CLKGSEL != 0"]
    #[inline] pub fn test_clkgsel(&self) -> bool {
        self.clkgsel() != 0
    }

    #[doc="Sets the CLKGSEL field."]
    #[inline] pub fn set_clkgsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Upper Bits of SDCLK Frequency Select"]
    #[inline] pub fn usdclkfsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if USDCLKFSEL != 0"]
    #[inline] pub fn test_usdclkfsel(&self) -> bool {
        self.usdclkfsel() != 0
    }

    #[doc="Sets the USDCLKFSEL field."]
    #[inline] pub fn set_usdclkfsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SDCLK Frequency Select"]
    #[inline] pub fn sdclkfsel(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if SDCLKFSEL != 0"]
    #[inline] pub fn test_sdclkfsel(&self) -> bool {
        self.sdclkfsel() != 0
    }

    #[doc="Sets the SDCLKFSEL field."]
    #[inline] pub fn set_sdclkfsel<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Ccr {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.intclken() != 0 { try!(write!(f, " intclken"))}
        if self.intclks() != 0 { try!(write!(f, " intclks"))}
        if self.sdclken() != 0 { try!(write!(f, " sdclken"))}
        if self.clkgsel() != 0 { try!(write!(f, " clkgsel"))}
        if self.usdclkfsel() != 0 { try!(write!(f, " usdclkfsel=0x{:x}", self.usdclkfsel()))}
        if self.sdclkfsel() != 0 { try!(write!(f, " sdclkfsel=0x{:x}", self.sdclkfsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timeout Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcr(pub u8);
impl Tcr {
    #[doc="Data Timeout Counter Value"]
    #[inline] pub fn dtcval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DTCVAL != 0"]
    #[inline] pub fn test_dtcval(&self) -> bool {
        self.dtcval() != 0
    }

    #[doc="Sets the DTCVAL field."]
    #[inline] pub fn set_dtcval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Tcr {
    #[inline]
    fn from(other: u8) -> Self {
         Tcr(other)
    }
}

impl ::core::fmt::Display for Tcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtcval() != 0 { try!(write!(f, " dtcval=0x{:x}", self.dtcval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software Reset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srr(pub u8);
impl Srr {
    #[doc="Software Reset For All"]
    #[inline] pub fn swrstall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRSTALL != 0"]
    #[inline] pub fn test_swrstall(&self) -> bool {
        self.swrstall() != 0
    }

    #[doc="Sets the SWRSTALL field."]
    #[inline] pub fn set_swrstall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Software Reset For CMD Line"]
    #[inline] pub fn swrstcmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SWRSTCMD != 0"]
    #[inline] pub fn test_swrstcmd(&self) -> bool {
        self.swrstcmd() != 0
    }

    #[doc="Sets the SWRSTCMD field."]
    #[inline] pub fn set_swrstcmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Software Reset For DAT Line"]
    #[inline] pub fn swrstdat(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SWRSTDAT != 0"]
    #[inline] pub fn test_swrstdat(&self) -> bool {
        self.swrstdat() != 0
    }

    #[doc="Sets the SWRSTDAT field."]
    #[inline] pub fn set_swrstdat<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Srr {
    #[inline]
    fn from(other: u8) -> Self {
         Srr(other)
    }
}

impl ::core::fmt::Display for Srr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrstall() != 0 { try!(write!(f, " swrstall"))}
        if self.swrstcmd() != 0 { try!(write!(f, " swrstcmd"))}
        if self.swrstdat() != 0 { try!(write!(f, " swrstdat"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Normal Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nistr(pub u16);
impl Nistr {
    #[doc="Command Complete"]
    #[inline] pub fn cmdc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDC != 0"]
    #[inline] pub fn test_cmdc(&self) -> bool {
        self.cmdc() != 0
    }

    #[doc="Sets the CMDC field."]
    #[inline] pub fn set_cmdc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete"]
    #[inline] pub fn trfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRFC != 0"]
    #[inline] pub fn test_trfc(&self) -> bool {
        self.trfc() != 0
    }

    #[doc="Sets the TRFC field."]
    #[inline] pub fn set_trfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Block Gap Event"]
    #[inline] pub fn blkge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BLKGE != 0"]
    #[inline] pub fn test_blkge(&self) -> bool {
        self.blkge() != 0
    }

    #[doc="Sets the BLKGE field."]
    #[inline] pub fn set_blkge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA Interrupt"]
    #[inline] pub fn dmaint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAINT != 0"]
    #[inline] pub fn test_dmaint(&self) -> bool {
        self.dmaint() != 0
    }

    #[doc="Sets the DMAINT field."]
    #[inline] pub fn set_dmaint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Buffer Write Ready"]
    #[inline] pub fn bwrrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BWRRDY != 0"]
    #[inline] pub fn test_bwrrdy(&self) -> bool {
        self.bwrrdy() != 0
    }

    #[doc="Sets the BWRRDY field."]
    #[inline] pub fn set_bwrrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Buffer Read Ready"]
    #[inline] pub fn brdrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BRDRDY != 0"]
    #[inline] pub fn test_brdrdy(&self) -> bool {
        self.brdrdy() != 0
    }

    #[doc="Sets the BRDRDY field."]
    #[inline] pub fn set_brdrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Card Insertion"]
    #[inline] pub fn cins(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CINS != 0"]
    #[inline] pub fn test_cins(&self) -> bool {
        self.cins() != 0
    }

    #[doc="Sets the CINS field."]
    #[inline] pub fn set_cins<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Card Removal"]
    #[inline] pub fn crem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CREM != 0"]
    #[inline] pub fn test_crem(&self) -> bool {
        self.crem() != 0
    }

    #[doc="Sets the CREM field."]
    #[inline] pub fn set_crem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Card Interrupt"]
    #[inline] pub fn cint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CINT != 0"]
    #[inline] pub fn test_cint(&self) -> bool {
        self.cint() != 0
    }

    #[doc="Sets the CINT field."]
    #[inline] pub fn set_cint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Error Interrupt"]
    #[inline] pub fn errint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ERRINT != 0"]
    #[inline] pub fn test_errint(&self) -> bool {
        self.errint() != 0
    }

    #[doc="Sets the ERRINT field."]
    #[inline] pub fn set_errint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Nistr {
    #[inline]
    fn from(other: u16) -> Self {
         Nistr(other)
    }
}

impl ::core::fmt::Display for Nistr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nistr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdc() != 0 { try!(write!(f, " cmdc"))}
        if self.trfc() != 0 { try!(write!(f, " trfc"))}
        if self.blkge() != 0 { try!(write!(f, " blkge"))}
        if self.dmaint() != 0 { try!(write!(f, " dmaint"))}
        if self.bwrrdy() != 0 { try!(write!(f, " bwrrdy"))}
        if self.brdrdy() != 0 { try!(write!(f, " brdrdy"))}
        if self.cins() != 0 { try!(write!(f, " cins"))}
        if self.crem() != 0 { try!(write!(f, " crem"))}
        if self.cint() != 0 { try!(write!(f, " cint"))}
        if self.errint() != 0 { try!(write!(f, " errint"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Normal Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct NistrEmmc(pub u16);
impl NistrEmmc {
    #[doc="Command Complete"]
    #[inline] pub fn cmdc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDC != 0"]
    #[inline] pub fn test_cmdc(&self) -> bool {
        self.cmdc() != 0
    }

    #[doc="Sets the CMDC field."]
    #[inline] pub fn set_cmdc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete"]
    #[inline] pub fn trfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRFC != 0"]
    #[inline] pub fn test_trfc(&self) -> bool {
        self.trfc() != 0
    }

    #[doc="Sets the TRFC field."]
    #[inline] pub fn set_trfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Block Gap Event"]
    #[inline] pub fn blkge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BLKGE != 0"]
    #[inline] pub fn test_blkge(&self) -> bool {
        self.blkge() != 0
    }

    #[doc="Sets the BLKGE field."]
    #[inline] pub fn set_blkge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA Interrupt"]
    #[inline] pub fn dmaint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAINT != 0"]
    #[inline] pub fn test_dmaint(&self) -> bool {
        self.dmaint() != 0
    }

    #[doc="Sets the DMAINT field."]
    #[inline] pub fn set_dmaint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Buffer Write Ready"]
    #[inline] pub fn bwrrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BWRRDY != 0"]
    #[inline] pub fn test_bwrrdy(&self) -> bool {
        self.bwrrdy() != 0
    }

    #[doc="Sets the BWRRDY field."]
    #[inline] pub fn set_bwrrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Buffer Read Ready"]
    #[inline] pub fn brdrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BRDRDY != 0"]
    #[inline] pub fn test_brdrdy(&self) -> bool {
        self.brdrdy() != 0
    }

    #[doc="Sets the BRDRDY field."]
    #[inline] pub fn set_brdrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Boot Acknowledge Received"]
    #[inline] pub fn bootar(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if BOOTAR != 0"]
    #[inline] pub fn test_bootar(&self) -> bool {
        self.bootar() != 0
    }

    #[doc="Sets the BOOTAR field."]
    #[inline] pub fn set_bootar<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Error Interrupt"]
    #[inline] pub fn errint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ERRINT != 0"]
    #[inline] pub fn test_errint(&self) -> bool {
        self.errint() != 0
    }

    #[doc="Sets the ERRINT field."]
    #[inline] pub fn set_errint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for NistrEmmc {
    #[inline]
    fn from(other: u16) -> Self {
         NistrEmmc(other)
    }
}

impl ::core::fmt::Display for NistrEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for NistrEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdc() != 0 { try!(write!(f, " cmdc"))}
        if self.trfc() != 0 { try!(write!(f, " trfc"))}
        if self.blkge() != 0 { try!(write!(f, " blkge"))}
        if self.dmaint() != 0 { try!(write!(f, " dmaint"))}
        if self.bwrrdy() != 0 { try!(write!(f, " bwrrdy"))}
        if self.brdrdy() != 0 { try!(write!(f, " brdrdy"))}
        if self.bootar() != 0 { try!(write!(f, " bootar"))}
        if self.errint() != 0 { try!(write!(f, " errint"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eistr(pub u16);
impl Eistr {
    #[doc="Command Timeout Error"]
    #[inline] pub fn cmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDTEO != 0"]
    #[inline] pub fn test_cmdteo(&self) -> bool {
        self.cmdteo() != 0
    }

    #[doc="Sets the CMDTEO field."]
    #[inline] pub fn set_cmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command CRC Error"]
    #[inline] pub fn cmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDCRC != 0"]
    #[inline] pub fn test_cmdcrc(&self) -> bool {
        self.cmdcrc() != 0
    }

    #[doc="Sets the CMDCRC field."]
    #[inline] pub fn set_cmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Command End Bit Error"]
    #[inline] pub fn cmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CMDEND != 0"]
    #[inline] pub fn test_cmdend(&self) -> bool {
        self.cmdend() != 0
    }

    #[doc="Sets the CMDEND field."]
    #[inline] pub fn set_cmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command Index Error"]
    #[inline] pub fn cmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMDIDX != 0"]
    #[inline] pub fn test_cmdidx(&self) -> bool {
        self.cmdidx() != 0
    }

    #[doc="Sets the CMDIDX field."]
    #[inline] pub fn set_cmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data Timeout Error"]
    #[inline] pub fn datteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DATTEO != 0"]
    #[inline] pub fn test_datteo(&self) -> bool {
        self.datteo() != 0
    }

    #[doc="Sets the DATTEO field."]
    #[inline] pub fn set_datteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data CRC Error"]
    #[inline] pub fn datcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DATCRC != 0"]
    #[inline] pub fn test_datcrc(&self) -> bool {
        self.datcrc() != 0
    }

    #[doc="Sets the DATCRC field."]
    #[inline] pub fn set_datcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Data End Bit Error"]
    #[inline] pub fn datend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DATEND != 0"]
    #[inline] pub fn test_datend(&self) -> bool {
        self.datend() != 0
    }

    #[doc="Sets the DATEND field."]
    #[inline] pub fn set_datend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Current Limit Error"]
    #[inline] pub fn curlim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CURLIM != 0"]
    #[inline] pub fn test_curlim(&self) -> bool {
        self.curlim() != 0
    }

    #[doc="Sets the CURLIM field."]
    #[inline] pub fn set_curlim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Auto CMD Error"]
    #[inline] pub fn acmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ACMD != 0"]
    #[inline] pub fn test_acmd(&self) -> bool {
        self.acmd() != 0
    }

    #[doc="Sets the ACMD field."]
    #[inline] pub fn set_acmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADMA Error"]
    #[inline] pub fn adma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADMA != 0"]
    #[inline] pub fn test_adma(&self) -> bool {
        self.adma() != 0
    }

    #[doc="Sets the ADMA field."]
    #[inline] pub fn set_adma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u16> for Eistr {
    #[inline]
    fn from(other: u16) -> Self {
         Eistr(other)
    }
}

impl ::core::fmt::Display for Eistr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eistr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdteo() != 0 { try!(write!(f, " cmdteo"))}
        if self.cmdcrc() != 0 { try!(write!(f, " cmdcrc"))}
        if self.cmdend() != 0 { try!(write!(f, " cmdend"))}
        if self.cmdidx() != 0 { try!(write!(f, " cmdidx"))}
        if self.datteo() != 0 { try!(write!(f, " datteo"))}
        if self.datcrc() != 0 { try!(write!(f, " datcrc"))}
        if self.datend() != 0 { try!(write!(f, " datend"))}
        if self.curlim() != 0 { try!(write!(f, " curlim"))}
        if self.acmd() != 0 { try!(write!(f, " acmd"))}
        if self.adma() != 0 { try!(write!(f, " adma"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct EistrEmmc(pub u16);
impl EistrEmmc {
    #[doc="Command Timeout Error"]
    #[inline] pub fn cmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDTEO != 0"]
    #[inline] pub fn test_cmdteo(&self) -> bool {
        self.cmdteo() != 0
    }

    #[doc="Sets the CMDTEO field."]
    #[inline] pub fn set_cmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command CRC Error"]
    #[inline] pub fn cmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDCRC != 0"]
    #[inline] pub fn test_cmdcrc(&self) -> bool {
        self.cmdcrc() != 0
    }

    #[doc="Sets the CMDCRC field."]
    #[inline] pub fn set_cmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Command End Bit Error"]
    #[inline] pub fn cmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CMDEND != 0"]
    #[inline] pub fn test_cmdend(&self) -> bool {
        self.cmdend() != 0
    }

    #[doc="Sets the CMDEND field."]
    #[inline] pub fn set_cmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command Index Error"]
    #[inline] pub fn cmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMDIDX != 0"]
    #[inline] pub fn test_cmdidx(&self) -> bool {
        self.cmdidx() != 0
    }

    #[doc="Sets the CMDIDX field."]
    #[inline] pub fn set_cmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data Timeout Error"]
    #[inline] pub fn datteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DATTEO != 0"]
    #[inline] pub fn test_datteo(&self) -> bool {
        self.datteo() != 0
    }

    #[doc="Sets the DATTEO field."]
    #[inline] pub fn set_datteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data CRC Error"]
    #[inline] pub fn datcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DATCRC != 0"]
    #[inline] pub fn test_datcrc(&self) -> bool {
        self.datcrc() != 0
    }

    #[doc="Sets the DATCRC field."]
    #[inline] pub fn set_datcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Data End Bit Error"]
    #[inline] pub fn datend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DATEND != 0"]
    #[inline] pub fn test_datend(&self) -> bool {
        self.datend() != 0
    }

    #[doc="Sets the DATEND field."]
    #[inline] pub fn set_datend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Current Limit Error"]
    #[inline] pub fn curlim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CURLIM != 0"]
    #[inline] pub fn test_curlim(&self) -> bool {
        self.curlim() != 0
    }

    #[doc="Sets the CURLIM field."]
    #[inline] pub fn set_curlim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Auto CMD Error"]
    #[inline] pub fn acmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ACMD != 0"]
    #[inline] pub fn test_acmd(&self) -> bool {
        self.acmd() != 0
    }

    #[doc="Sets the ACMD field."]
    #[inline] pub fn set_acmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADMA Error"]
    #[inline] pub fn adma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADMA != 0"]
    #[inline] pub fn test_adma(&self) -> bool {
        self.adma() != 0
    }

    #[doc="Sets the ADMA field."]
    #[inline] pub fn set_adma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Boot Acknowledge Error"]
    #[inline] pub fn bootae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if BOOTAE != 0"]
    #[inline] pub fn test_bootae(&self) -> bool {
        self.bootae() != 0
    }

    #[doc="Sets the BOOTAE field."]
    #[inline] pub fn set_bootae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for EistrEmmc {
    #[inline]
    fn from(other: u16) -> Self {
         EistrEmmc(other)
    }
}

impl ::core::fmt::Display for EistrEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for EistrEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdteo() != 0 { try!(write!(f, " cmdteo"))}
        if self.cmdcrc() != 0 { try!(write!(f, " cmdcrc"))}
        if self.cmdend() != 0 { try!(write!(f, " cmdend"))}
        if self.cmdidx() != 0 { try!(write!(f, " cmdidx"))}
        if self.datteo() != 0 { try!(write!(f, " datteo"))}
        if self.datcrc() != 0 { try!(write!(f, " datcrc"))}
        if self.datend() != 0 { try!(write!(f, " datend"))}
        if self.curlim() != 0 { try!(write!(f, " curlim"))}
        if self.acmd() != 0 { try!(write!(f, " acmd"))}
        if self.adma() != 0 { try!(write!(f, " adma"))}
        if self.bootae() != 0 { try!(write!(f, " bootae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Normal Interrupt Status Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nister(pub u16);
impl Nister {
    #[doc="Command Complete Status Enable"]
    #[inline] pub fn cmdc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDC != 0"]
    #[inline] pub fn test_cmdc(&self) -> bool {
        self.cmdc() != 0
    }

    #[doc="Sets the CMDC field."]
    #[inline] pub fn set_cmdc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete Status Enable"]
    #[inline] pub fn trfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRFC != 0"]
    #[inline] pub fn test_trfc(&self) -> bool {
        self.trfc() != 0
    }

    #[doc="Sets the TRFC field."]
    #[inline] pub fn set_trfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Block Gap Event Status Enable"]
    #[inline] pub fn blkge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BLKGE != 0"]
    #[inline] pub fn test_blkge(&self) -> bool {
        self.blkge() != 0
    }

    #[doc="Sets the BLKGE field."]
    #[inline] pub fn set_blkge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA Interrupt Status Enable"]
    #[inline] pub fn dmaint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAINT != 0"]
    #[inline] pub fn test_dmaint(&self) -> bool {
        self.dmaint() != 0
    }

    #[doc="Sets the DMAINT field."]
    #[inline] pub fn set_dmaint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Buffer Write Ready Status Enable"]
    #[inline] pub fn bwrrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BWRRDY != 0"]
    #[inline] pub fn test_bwrrdy(&self) -> bool {
        self.bwrrdy() != 0
    }

    #[doc="Sets the BWRRDY field."]
    #[inline] pub fn set_bwrrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Buffer Read Ready Status Enable"]
    #[inline] pub fn brdrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BRDRDY != 0"]
    #[inline] pub fn test_brdrdy(&self) -> bool {
        self.brdrdy() != 0
    }

    #[doc="Sets the BRDRDY field."]
    #[inline] pub fn set_brdrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Card Insertion Status Enable"]
    #[inline] pub fn cins(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CINS != 0"]
    #[inline] pub fn test_cins(&self) -> bool {
        self.cins() != 0
    }

    #[doc="Sets the CINS field."]
    #[inline] pub fn set_cins<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Card Removal Status Enable"]
    #[inline] pub fn crem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CREM != 0"]
    #[inline] pub fn test_crem(&self) -> bool {
        self.crem() != 0
    }

    #[doc="Sets the CREM field."]
    #[inline] pub fn set_crem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Card Interrupt Status Enable"]
    #[inline] pub fn cint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CINT != 0"]
    #[inline] pub fn test_cint(&self) -> bool {
        self.cint() != 0
    }

    #[doc="Sets the CINT field."]
    #[inline] pub fn set_cint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Nister {
    #[inline]
    fn from(other: u16) -> Self {
         Nister(other)
    }
}

impl ::core::fmt::Display for Nister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdc() != 0 { try!(write!(f, " cmdc"))}
        if self.trfc() != 0 { try!(write!(f, " trfc"))}
        if self.blkge() != 0 { try!(write!(f, " blkge"))}
        if self.dmaint() != 0 { try!(write!(f, " dmaint"))}
        if self.bwrrdy() != 0 { try!(write!(f, " bwrrdy"))}
        if self.brdrdy() != 0 { try!(write!(f, " brdrdy"))}
        if self.cins() != 0 { try!(write!(f, " cins"))}
        if self.crem() != 0 { try!(write!(f, " crem"))}
        if self.cint() != 0 { try!(write!(f, " cint"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Normal Interrupt Status Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct NisterEmmc(pub u16);
impl NisterEmmc {
    #[doc="Command Complete Status Enable"]
    #[inline] pub fn cmdc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDC != 0"]
    #[inline] pub fn test_cmdc(&self) -> bool {
        self.cmdc() != 0
    }

    #[doc="Sets the CMDC field."]
    #[inline] pub fn set_cmdc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete Status Enable"]
    #[inline] pub fn trfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRFC != 0"]
    #[inline] pub fn test_trfc(&self) -> bool {
        self.trfc() != 0
    }

    #[doc="Sets the TRFC field."]
    #[inline] pub fn set_trfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Block Gap Event Status Enable"]
    #[inline] pub fn blkge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BLKGE != 0"]
    #[inline] pub fn test_blkge(&self) -> bool {
        self.blkge() != 0
    }

    #[doc="Sets the BLKGE field."]
    #[inline] pub fn set_blkge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA Interrupt Status Enable"]
    #[inline] pub fn dmaint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAINT != 0"]
    #[inline] pub fn test_dmaint(&self) -> bool {
        self.dmaint() != 0
    }

    #[doc="Sets the DMAINT field."]
    #[inline] pub fn set_dmaint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Buffer Write Ready Status Enable"]
    #[inline] pub fn bwrrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BWRRDY != 0"]
    #[inline] pub fn test_bwrrdy(&self) -> bool {
        self.bwrrdy() != 0
    }

    #[doc="Sets the BWRRDY field."]
    #[inline] pub fn set_bwrrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Buffer Read Ready Status Enable"]
    #[inline] pub fn brdrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BRDRDY != 0"]
    #[inline] pub fn test_brdrdy(&self) -> bool {
        self.brdrdy() != 0
    }

    #[doc="Sets the BRDRDY field."]
    #[inline] pub fn set_brdrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Boot Acknowledge Received Status Enable"]
    #[inline] pub fn bootar(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if BOOTAR != 0"]
    #[inline] pub fn test_bootar(&self) -> bool {
        self.bootar() != 0
    }

    #[doc="Sets the BOOTAR field."]
    #[inline] pub fn set_bootar<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u16> for NisterEmmc {
    #[inline]
    fn from(other: u16) -> Self {
         NisterEmmc(other)
    }
}

impl ::core::fmt::Display for NisterEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for NisterEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdc() != 0 { try!(write!(f, " cmdc"))}
        if self.trfc() != 0 { try!(write!(f, " trfc"))}
        if self.blkge() != 0 { try!(write!(f, " blkge"))}
        if self.dmaint() != 0 { try!(write!(f, " dmaint"))}
        if self.bwrrdy() != 0 { try!(write!(f, " bwrrdy"))}
        if self.brdrdy() != 0 { try!(write!(f, " brdrdy"))}
        if self.bootar() != 0 { try!(write!(f, " bootar"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Interrupt Status Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eister(pub u16);
impl Eister {
    #[doc="Command Timeout Error Status Enable"]
    #[inline] pub fn cmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDTEO != 0"]
    #[inline] pub fn test_cmdteo(&self) -> bool {
        self.cmdteo() != 0
    }

    #[doc="Sets the CMDTEO field."]
    #[inline] pub fn set_cmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command CRC Error Status Enable"]
    #[inline] pub fn cmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDCRC != 0"]
    #[inline] pub fn test_cmdcrc(&self) -> bool {
        self.cmdcrc() != 0
    }

    #[doc="Sets the CMDCRC field."]
    #[inline] pub fn set_cmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Command End Bit Error Status Enable"]
    #[inline] pub fn cmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CMDEND != 0"]
    #[inline] pub fn test_cmdend(&self) -> bool {
        self.cmdend() != 0
    }

    #[doc="Sets the CMDEND field."]
    #[inline] pub fn set_cmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command Index Error Status Enable"]
    #[inline] pub fn cmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMDIDX != 0"]
    #[inline] pub fn test_cmdidx(&self) -> bool {
        self.cmdidx() != 0
    }

    #[doc="Sets the CMDIDX field."]
    #[inline] pub fn set_cmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data Timeout Error Status Enable"]
    #[inline] pub fn datteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DATTEO != 0"]
    #[inline] pub fn test_datteo(&self) -> bool {
        self.datteo() != 0
    }

    #[doc="Sets the DATTEO field."]
    #[inline] pub fn set_datteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data CRC Error Status Enable"]
    #[inline] pub fn datcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DATCRC != 0"]
    #[inline] pub fn test_datcrc(&self) -> bool {
        self.datcrc() != 0
    }

    #[doc="Sets the DATCRC field."]
    #[inline] pub fn set_datcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Data End Bit Error Status Enable"]
    #[inline] pub fn datend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DATEND != 0"]
    #[inline] pub fn test_datend(&self) -> bool {
        self.datend() != 0
    }

    #[doc="Sets the DATEND field."]
    #[inline] pub fn set_datend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Current Limit Error Status Enable"]
    #[inline] pub fn curlim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CURLIM != 0"]
    #[inline] pub fn test_curlim(&self) -> bool {
        self.curlim() != 0
    }

    #[doc="Sets the CURLIM field."]
    #[inline] pub fn set_curlim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Auto CMD Error Status Enable"]
    #[inline] pub fn acmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ACMD != 0"]
    #[inline] pub fn test_acmd(&self) -> bool {
        self.acmd() != 0
    }

    #[doc="Sets the ACMD field."]
    #[inline] pub fn set_acmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADMA Error Status Enable"]
    #[inline] pub fn adma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADMA != 0"]
    #[inline] pub fn test_adma(&self) -> bool {
        self.adma() != 0
    }

    #[doc="Sets the ADMA field."]
    #[inline] pub fn set_adma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u16> for Eister {
    #[inline]
    fn from(other: u16) -> Self {
         Eister(other)
    }
}

impl ::core::fmt::Display for Eister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdteo() != 0 { try!(write!(f, " cmdteo"))}
        if self.cmdcrc() != 0 { try!(write!(f, " cmdcrc"))}
        if self.cmdend() != 0 { try!(write!(f, " cmdend"))}
        if self.cmdidx() != 0 { try!(write!(f, " cmdidx"))}
        if self.datteo() != 0 { try!(write!(f, " datteo"))}
        if self.datcrc() != 0 { try!(write!(f, " datcrc"))}
        if self.datend() != 0 { try!(write!(f, " datend"))}
        if self.curlim() != 0 { try!(write!(f, " curlim"))}
        if self.acmd() != 0 { try!(write!(f, " acmd"))}
        if self.adma() != 0 { try!(write!(f, " adma"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Interrupt Status Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct EisterEmmc(pub u16);
impl EisterEmmc {
    #[doc="Command Timeout Error Status Enable"]
    #[inline] pub fn cmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDTEO != 0"]
    #[inline] pub fn test_cmdteo(&self) -> bool {
        self.cmdteo() != 0
    }

    #[doc="Sets the CMDTEO field."]
    #[inline] pub fn set_cmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command CRC Error Status Enable"]
    #[inline] pub fn cmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDCRC != 0"]
    #[inline] pub fn test_cmdcrc(&self) -> bool {
        self.cmdcrc() != 0
    }

    #[doc="Sets the CMDCRC field."]
    #[inline] pub fn set_cmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Command End Bit Error Status Enable"]
    #[inline] pub fn cmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CMDEND != 0"]
    #[inline] pub fn test_cmdend(&self) -> bool {
        self.cmdend() != 0
    }

    #[doc="Sets the CMDEND field."]
    #[inline] pub fn set_cmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command Index Error Status Enable"]
    #[inline] pub fn cmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMDIDX != 0"]
    #[inline] pub fn test_cmdidx(&self) -> bool {
        self.cmdidx() != 0
    }

    #[doc="Sets the CMDIDX field."]
    #[inline] pub fn set_cmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data Timeout Error Status Enable"]
    #[inline] pub fn datteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DATTEO != 0"]
    #[inline] pub fn test_datteo(&self) -> bool {
        self.datteo() != 0
    }

    #[doc="Sets the DATTEO field."]
    #[inline] pub fn set_datteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data CRC Error Status Enable"]
    #[inline] pub fn datcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DATCRC != 0"]
    #[inline] pub fn test_datcrc(&self) -> bool {
        self.datcrc() != 0
    }

    #[doc="Sets the DATCRC field."]
    #[inline] pub fn set_datcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Data End Bit Error Status Enable"]
    #[inline] pub fn datend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DATEND != 0"]
    #[inline] pub fn test_datend(&self) -> bool {
        self.datend() != 0
    }

    #[doc="Sets the DATEND field."]
    #[inline] pub fn set_datend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Current Limit Error Status Enable"]
    #[inline] pub fn curlim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CURLIM != 0"]
    #[inline] pub fn test_curlim(&self) -> bool {
        self.curlim() != 0
    }

    #[doc="Sets the CURLIM field."]
    #[inline] pub fn set_curlim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Auto CMD Error Status Enable"]
    #[inline] pub fn acmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ACMD != 0"]
    #[inline] pub fn test_acmd(&self) -> bool {
        self.acmd() != 0
    }

    #[doc="Sets the ACMD field."]
    #[inline] pub fn set_acmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADMA Error Status Enable"]
    #[inline] pub fn adma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADMA != 0"]
    #[inline] pub fn test_adma(&self) -> bool {
        self.adma() != 0
    }

    #[doc="Sets the ADMA field."]
    #[inline] pub fn set_adma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Boot Acknowledge Error Status Enable"]
    #[inline] pub fn bootae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if BOOTAE != 0"]
    #[inline] pub fn test_bootae(&self) -> bool {
        self.bootae() != 0
    }

    #[doc="Sets the BOOTAE field."]
    #[inline] pub fn set_bootae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for EisterEmmc {
    #[inline]
    fn from(other: u16) -> Self {
         EisterEmmc(other)
    }
}

impl ::core::fmt::Display for EisterEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for EisterEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdteo() != 0 { try!(write!(f, " cmdteo"))}
        if self.cmdcrc() != 0 { try!(write!(f, " cmdcrc"))}
        if self.cmdend() != 0 { try!(write!(f, " cmdend"))}
        if self.cmdidx() != 0 { try!(write!(f, " cmdidx"))}
        if self.datteo() != 0 { try!(write!(f, " datteo"))}
        if self.datcrc() != 0 { try!(write!(f, " datcrc"))}
        if self.datend() != 0 { try!(write!(f, " datend"))}
        if self.curlim() != 0 { try!(write!(f, " curlim"))}
        if self.acmd() != 0 { try!(write!(f, " acmd"))}
        if self.adma() != 0 { try!(write!(f, " adma"))}
        if self.bootae() != 0 { try!(write!(f, " bootae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Normal Interrupt Signal Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nisier(pub u16);
impl Nisier {
    #[doc="Command Complete Signal Enable"]
    #[inline] pub fn cmdc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDC != 0"]
    #[inline] pub fn test_cmdc(&self) -> bool {
        self.cmdc() != 0
    }

    #[doc="Sets the CMDC field."]
    #[inline] pub fn set_cmdc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete Signal Enable"]
    #[inline] pub fn trfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRFC != 0"]
    #[inline] pub fn test_trfc(&self) -> bool {
        self.trfc() != 0
    }

    #[doc="Sets the TRFC field."]
    #[inline] pub fn set_trfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Block Gap Event Signal Enable"]
    #[inline] pub fn blkge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BLKGE != 0"]
    #[inline] pub fn test_blkge(&self) -> bool {
        self.blkge() != 0
    }

    #[doc="Sets the BLKGE field."]
    #[inline] pub fn set_blkge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA Interrupt Signal Enable"]
    #[inline] pub fn dmaint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAINT != 0"]
    #[inline] pub fn test_dmaint(&self) -> bool {
        self.dmaint() != 0
    }

    #[doc="Sets the DMAINT field."]
    #[inline] pub fn set_dmaint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Buffer Write Ready Signal Enable"]
    #[inline] pub fn bwrrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BWRRDY != 0"]
    #[inline] pub fn test_bwrrdy(&self) -> bool {
        self.bwrrdy() != 0
    }

    #[doc="Sets the BWRRDY field."]
    #[inline] pub fn set_bwrrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Buffer Read Ready Signal Enable"]
    #[inline] pub fn brdrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BRDRDY != 0"]
    #[inline] pub fn test_brdrdy(&self) -> bool {
        self.brdrdy() != 0
    }

    #[doc="Sets the BRDRDY field."]
    #[inline] pub fn set_brdrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Card Insertion Signal Enable"]
    #[inline] pub fn cins(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CINS != 0"]
    #[inline] pub fn test_cins(&self) -> bool {
        self.cins() != 0
    }

    #[doc="Sets the CINS field."]
    #[inline] pub fn set_cins<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Card Removal Signal Enable"]
    #[inline] pub fn crem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CREM != 0"]
    #[inline] pub fn test_crem(&self) -> bool {
        self.crem() != 0
    }

    #[doc="Sets the CREM field."]
    #[inline] pub fn set_crem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Card Interrupt Signal Enable"]
    #[inline] pub fn cint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CINT != 0"]
    #[inline] pub fn test_cint(&self) -> bool {
        self.cint() != 0
    }

    #[doc="Sets the CINT field."]
    #[inline] pub fn set_cint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Nisier {
    #[inline]
    fn from(other: u16) -> Self {
         Nisier(other)
    }
}

impl ::core::fmt::Display for Nisier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nisier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdc() != 0 { try!(write!(f, " cmdc"))}
        if self.trfc() != 0 { try!(write!(f, " trfc"))}
        if self.blkge() != 0 { try!(write!(f, " blkge"))}
        if self.dmaint() != 0 { try!(write!(f, " dmaint"))}
        if self.bwrrdy() != 0 { try!(write!(f, " bwrrdy"))}
        if self.brdrdy() != 0 { try!(write!(f, " brdrdy"))}
        if self.cins() != 0 { try!(write!(f, " cins"))}
        if self.crem() != 0 { try!(write!(f, " crem"))}
        if self.cint() != 0 { try!(write!(f, " cint"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Normal Interrupt Signal Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct NisierEmmc(pub u16);
impl NisierEmmc {
    #[doc="Command Complete Signal Enable"]
    #[inline] pub fn cmdc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDC != 0"]
    #[inline] pub fn test_cmdc(&self) -> bool {
        self.cmdc() != 0
    }

    #[doc="Sets the CMDC field."]
    #[inline] pub fn set_cmdc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete Signal Enable"]
    #[inline] pub fn trfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRFC != 0"]
    #[inline] pub fn test_trfc(&self) -> bool {
        self.trfc() != 0
    }

    #[doc="Sets the TRFC field."]
    #[inline] pub fn set_trfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Block Gap Event Signal Enable"]
    #[inline] pub fn blkge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BLKGE != 0"]
    #[inline] pub fn test_blkge(&self) -> bool {
        self.blkge() != 0
    }

    #[doc="Sets the BLKGE field."]
    #[inline] pub fn set_blkge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA Interrupt Signal Enable"]
    #[inline] pub fn dmaint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAINT != 0"]
    #[inline] pub fn test_dmaint(&self) -> bool {
        self.dmaint() != 0
    }

    #[doc="Sets the DMAINT field."]
    #[inline] pub fn set_dmaint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Buffer Write Ready Signal Enable"]
    #[inline] pub fn bwrrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BWRRDY != 0"]
    #[inline] pub fn test_bwrrdy(&self) -> bool {
        self.bwrrdy() != 0
    }

    #[doc="Sets the BWRRDY field."]
    #[inline] pub fn set_bwrrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Buffer Read Ready Signal Enable"]
    #[inline] pub fn brdrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BRDRDY != 0"]
    #[inline] pub fn test_brdrdy(&self) -> bool {
        self.brdrdy() != 0
    }

    #[doc="Sets the BRDRDY field."]
    #[inline] pub fn set_brdrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Boot Acknowledge Received Signal Enable"]
    #[inline] pub fn bootar(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if BOOTAR != 0"]
    #[inline] pub fn test_bootar(&self) -> bool {
        self.bootar() != 0
    }

    #[doc="Sets the BOOTAR field."]
    #[inline] pub fn set_bootar<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u16> for NisierEmmc {
    #[inline]
    fn from(other: u16) -> Self {
         NisierEmmc(other)
    }
}

impl ::core::fmt::Display for NisierEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for NisierEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdc() != 0 { try!(write!(f, " cmdc"))}
        if self.trfc() != 0 { try!(write!(f, " trfc"))}
        if self.blkge() != 0 { try!(write!(f, " blkge"))}
        if self.dmaint() != 0 { try!(write!(f, " dmaint"))}
        if self.bwrrdy() != 0 { try!(write!(f, " bwrrdy"))}
        if self.brdrdy() != 0 { try!(write!(f, " brdrdy"))}
        if self.bootar() != 0 { try!(write!(f, " bootar"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Interrupt Signal Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eisier(pub u16);
impl Eisier {
    #[doc="Command Timeout Error Signal Enable"]
    #[inline] pub fn cmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDTEO != 0"]
    #[inline] pub fn test_cmdteo(&self) -> bool {
        self.cmdteo() != 0
    }

    #[doc="Sets the CMDTEO field."]
    #[inline] pub fn set_cmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command CRC Error Signal Enable"]
    #[inline] pub fn cmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDCRC != 0"]
    #[inline] pub fn test_cmdcrc(&self) -> bool {
        self.cmdcrc() != 0
    }

    #[doc="Sets the CMDCRC field."]
    #[inline] pub fn set_cmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Command End Bit Error Signal Enable"]
    #[inline] pub fn cmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CMDEND != 0"]
    #[inline] pub fn test_cmdend(&self) -> bool {
        self.cmdend() != 0
    }

    #[doc="Sets the CMDEND field."]
    #[inline] pub fn set_cmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command Index Error Signal Enable"]
    #[inline] pub fn cmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMDIDX != 0"]
    #[inline] pub fn test_cmdidx(&self) -> bool {
        self.cmdidx() != 0
    }

    #[doc="Sets the CMDIDX field."]
    #[inline] pub fn set_cmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data Timeout Error Signal Enable"]
    #[inline] pub fn datteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DATTEO != 0"]
    #[inline] pub fn test_datteo(&self) -> bool {
        self.datteo() != 0
    }

    #[doc="Sets the DATTEO field."]
    #[inline] pub fn set_datteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data CRC Error Signal Enable"]
    #[inline] pub fn datcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DATCRC != 0"]
    #[inline] pub fn test_datcrc(&self) -> bool {
        self.datcrc() != 0
    }

    #[doc="Sets the DATCRC field."]
    #[inline] pub fn set_datcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Data End Bit Error Signal Enable"]
    #[inline] pub fn datend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DATEND != 0"]
    #[inline] pub fn test_datend(&self) -> bool {
        self.datend() != 0
    }

    #[doc="Sets the DATEND field."]
    #[inline] pub fn set_datend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Current Limit Error Signal Enable"]
    #[inline] pub fn curlim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CURLIM != 0"]
    #[inline] pub fn test_curlim(&self) -> bool {
        self.curlim() != 0
    }

    #[doc="Sets the CURLIM field."]
    #[inline] pub fn set_curlim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Auto CMD Error Signal Enable"]
    #[inline] pub fn acmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ACMD != 0"]
    #[inline] pub fn test_acmd(&self) -> bool {
        self.acmd() != 0
    }

    #[doc="Sets the ACMD field."]
    #[inline] pub fn set_acmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADMA Error Signal Enable"]
    #[inline] pub fn adma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADMA != 0"]
    #[inline] pub fn test_adma(&self) -> bool {
        self.adma() != 0
    }

    #[doc="Sets the ADMA field."]
    #[inline] pub fn set_adma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u16> for Eisier {
    #[inline]
    fn from(other: u16) -> Self {
         Eisier(other)
    }
}

impl ::core::fmt::Display for Eisier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eisier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdteo() != 0 { try!(write!(f, " cmdteo"))}
        if self.cmdcrc() != 0 { try!(write!(f, " cmdcrc"))}
        if self.cmdend() != 0 { try!(write!(f, " cmdend"))}
        if self.cmdidx() != 0 { try!(write!(f, " cmdidx"))}
        if self.datteo() != 0 { try!(write!(f, " datteo"))}
        if self.datcrc() != 0 { try!(write!(f, " datcrc"))}
        if self.datend() != 0 { try!(write!(f, " datend"))}
        if self.curlim() != 0 { try!(write!(f, " curlim"))}
        if self.acmd() != 0 { try!(write!(f, " acmd"))}
        if self.adma() != 0 { try!(write!(f, " adma"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Interrupt Signal Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct EisierEmmc(pub u16);
impl EisierEmmc {
    #[doc="Command Timeout Error Signal Enable"]
    #[inline] pub fn cmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDTEO != 0"]
    #[inline] pub fn test_cmdteo(&self) -> bool {
        self.cmdteo() != 0
    }

    #[doc="Sets the CMDTEO field."]
    #[inline] pub fn set_cmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command CRC Error Signal Enable"]
    #[inline] pub fn cmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDCRC != 0"]
    #[inline] pub fn test_cmdcrc(&self) -> bool {
        self.cmdcrc() != 0
    }

    #[doc="Sets the CMDCRC field."]
    #[inline] pub fn set_cmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Command End Bit Error Signal Enable"]
    #[inline] pub fn cmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CMDEND != 0"]
    #[inline] pub fn test_cmdend(&self) -> bool {
        self.cmdend() != 0
    }

    #[doc="Sets the CMDEND field."]
    #[inline] pub fn set_cmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command Index Error Signal Enable"]
    #[inline] pub fn cmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMDIDX != 0"]
    #[inline] pub fn test_cmdidx(&self) -> bool {
        self.cmdidx() != 0
    }

    #[doc="Sets the CMDIDX field."]
    #[inline] pub fn set_cmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data Timeout Error Signal Enable"]
    #[inline] pub fn datteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DATTEO != 0"]
    #[inline] pub fn test_datteo(&self) -> bool {
        self.datteo() != 0
    }

    #[doc="Sets the DATTEO field."]
    #[inline] pub fn set_datteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data CRC Error Signal Enable"]
    #[inline] pub fn datcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DATCRC != 0"]
    #[inline] pub fn test_datcrc(&self) -> bool {
        self.datcrc() != 0
    }

    #[doc="Sets the DATCRC field."]
    #[inline] pub fn set_datcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Data End Bit Error Signal Enable"]
    #[inline] pub fn datend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DATEND != 0"]
    #[inline] pub fn test_datend(&self) -> bool {
        self.datend() != 0
    }

    #[doc="Sets the DATEND field."]
    #[inline] pub fn set_datend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Current Limit Error Signal Enable"]
    #[inline] pub fn curlim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CURLIM != 0"]
    #[inline] pub fn test_curlim(&self) -> bool {
        self.curlim() != 0
    }

    #[doc="Sets the CURLIM field."]
    #[inline] pub fn set_curlim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Auto CMD Error Signal Enable"]
    #[inline] pub fn acmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ACMD != 0"]
    #[inline] pub fn test_acmd(&self) -> bool {
        self.acmd() != 0
    }

    #[doc="Sets the ACMD field."]
    #[inline] pub fn set_acmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADMA Error Signal Enable"]
    #[inline] pub fn adma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADMA != 0"]
    #[inline] pub fn test_adma(&self) -> bool {
        self.adma() != 0
    }

    #[doc="Sets the ADMA field."]
    #[inline] pub fn set_adma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Boot Acknowledge Error Signal Enable"]
    #[inline] pub fn bootae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if BOOTAE != 0"]
    #[inline] pub fn test_bootae(&self) -> bool {
        self.bootae() != 0
    }

    #[doc="Sets the BOOTAE field."]
    #[inline] pub fn set_bootae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for EisierEmmc {
    #[inline]
    fn from(other: u16) -> Self {
         EisierEmmc(other)
    }
}

impl ::core::fmt::Display for EisierEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for EisierEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdteo() != 0 { try!(write!(f, " cmdteo"))}
        if self.cmdcrc() != 0 { try!(write!(f, " cmdcrc"))}
        if self.cmdend() != 0 { try!(write!(f, " cmdend"))}
        if self.cmdidx() != 0 { try!(write!(f, " cmdidx"))}
        if self.datteo() != 0 { try!(write!(f, " datteo"))}
        if self.datcrc() != 0 { try!(write!(f, " datcrc"))}
        if self.datend() != 0 { try!(write!(f, " datend"))}
        if self.curlim() != 0 { try!(write!(f, " curlim"))}
        if self.acmd() != 0 { try!(write!(f, " acmd"))}
        if self.adma() != 0 { try!(write!(f, " adma"))}
        if self.bootae() != 0 { try!(write!(f, " bootae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Auto CMD Error Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acesr(pub u16);
impl Acesr {
    #[doc="Auto CMD12 Not Executed"]
    #[inline] pub fn acmd12ne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ACMD12NE != 0"]
    #[inline] pub fn test_acmd12ne(&self) -> bool {
        self.acmd12ne() != 0
    }

    #[doc="Sets the ACMD12NE field."]
    #[inline] pub fn set_acmd12ne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Auto CMD Timeout Error"]
    #[inline] pub fn acmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ACMDTEO != 0"]
    #[inline] pub fn test_acmdteo(&self) -> bool {
        self.acmdteo() != 0
    }

    #[doc="Sets the ACMDTEO field."]
    #[inline] pub fn set_acmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Auto CMD CRC Error"]
    #[inline] pub fn acmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ACMDCRC != 0"]
    #[inline] pub fn test_acmdcrc(&self) -> bool {
        self.acmdcrc() != 0
    }

    #[doc="Sets the ACMDCRC field."]
    #[inline] pub fn set_acmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Auto CMD End Bit Error"]
    #[inline] pub fn acmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ACMDEND != 0"]
    #[inline] pub fn test_acmdend(&self) -> bool {
        self.acmdend() != 0
    }

    #[doc="Sets the ACMDEND field."]
    #[inline] pub fn set_acmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Auto CMD Index Error"]
    #[inline] pub fn acmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ACMDIDX != 0"]
    #[inline] pub fn test_acmdidx(&self) -> bool {
        self.acmdidx() != 0
    }

    #[doc="Sets the ACMDIDX field."]
    #[inline] pub fn set_acmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Command not Issued By Auto CMD12 Error"]
    #[inline] pub fn cmdni(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CMDNI != 0"]
    #[inline] pub fn test_cmdni(&self) -> bool {
        self.cmdni() != 0
    }

    #[doc="Sets the CMDNI field."]
    #[inline] pub fn set_cmdni<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u16> for Acesr {
    #[inline]
    fn from(other: u16) -> Self {
         Acesr(other)
    }
}

impl ::core::fmt::Display for Acesr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acesr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.acmd12ne() != 0 { try!(write!(f, " acmd12ne"))}
        if self.acmdteo() != 0 { try!(write!(f, " acmdteo"))}
        if self.acmdcrc() != 0 { try!(write!(f, " acmdcrc"))}
        if self.acmdend() != 0 { try!(write!(f, " acmdend"))}
        if self.acmdidx() != 0 { try!(write!(f, " acmdidx"))}
        if self.cmdni() != 0 { try!(write!(f, " cmdni"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Host Control 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hc2r(pub u16);
impl Hc2r {
    #[doc="UHS Mode Select"]
    #[inline] pub fn uhsms(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if UHSMS != 0"]
    #[inline] pub fn test_uhsms(&self) -> bool {
        self.uhsms() != 0
    }

    #[doc="Sets the UHSMS field."]
    #[inline] pub fn set_uhsms<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="1.8V Signaling Enable"]
    #[inline] pub fn vs18en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VS18EN != 0"]
    #[inline] pub fn test_vs18en(&self) -> bool {
        self.vs18en() != 0
    }

    #[doc="Sets the VS18EN field."]
    #[inline] pub fn set_vs18en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Driver Strength Select"]
    #[inline] pub fn drvsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DRVSEL != 0"]
    #[inline] pub fn test_drvsel(&self) -> bool {
        self.drvsel() != 0
    }

    #[doc="Sets the DRVSEL field."]
    #[inline] pub fn set_drvsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Execute Tuning"]
    #[inline] pub fn extun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EXTUN != 0"]
    #[inline] pub fn test_extun(&self) -> bool {
        self.extun() != 0
    }

    #[doc="Sets the EXTUN field."]
    #[inline] pub fn set_extun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Sampling Clock Select"]
    #[inline] pub fn slcksel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SLCKSEL != 0"]
    #[inline] pub fn test_slcksel(&self) -> bool {
        self.slcksel() != 0
    }

    #[doc="Sets the SLCKSEL field."]
    #[inline] pub fn set_slcksel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Asynchronous Interrupt Enable"]
    #[inline] pub fn asinten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ASINTEN != 0"]
    #[inline] pub fn test_asinten(&self) -> bool {
        self.asinten() != 0
    }

    #[doc="Sets the ASINTEN field."]
    #[inline] pub fn set_asinten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Preset Value Enable"]
    #[inline] pub fn pvalen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PVALEN != 0"]
    #[inline] pub fn test_pvalen(&self) -> bool {
        self.pvalen() != 0
    }

    #[doc="Sets the PVALEN field."]
    #[inline] pub fn set_pvalen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Hc2r {
    #[inline]
    fn from(other: u16) -> Self {
         Hc2r(other)
    }
}

impl ::core::fmt::Display for Hc2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hc2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uhsms() != 0 { try!(write!(f, " uhsms=0x{:x}", self.uhsms()))}
        if self.vs18en() != 0 { try!(write!(f, " vs18en"))}
        if self.drvsel() != 0 { try!(write!(f, " drvsel=0x{:x}", self.drvsel()))}
        if self.extun() != 0 { try!(write!(f, " extun"))}
        if self.slcksel() != 0 { try!(write!(f, " slcksel"))}
        if self.asinten() != 0 { try!(write!(f, " asinten"))}
        if self.pvalen() != 0 { try!(write!(f, " pvalen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Host Control 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hc2rEmmc(pub u16);
impl Hc2rEmmc {
    #[doc="HS200 Mode Enable"]
    #[inline] pub fn hs200en(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if HS200EN != 0"]
    #[inline] pub fn test_hs200en(&self) -> bool {
        self.hs200en() != 0
    }

    #[doc="Sets the HS200EN field."]
    #[inline] pub fn set_hs200en<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Driver Strength Select"]
    #[inline] pub fn drvsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DRVSEL != 0"]
    #[inline] pub fn test_drvsel(&self) -> bool {
        self.drvsel() != 0
    }

    #[doc="Sets the DRVSEL field."]
    #[inline] pub fn set_drvsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Execute Tuning"]
    #[inline] pub fn extun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EXTUN != 0"]
    #[inline] pub fn test_extun(&self) -> bool {
        self.extun() != 0
    }

    #[doc="Sets the EXTUN field."]
    #[inline] pub fn set_extun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Sampling Clock Select"]
    #[inline] pub fn slcksel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SLCKSEL != 0"]
    #[inline] pub fn test_slcksel(&self) -> bool {
        self.slcksel() != 0
    }

    #[doc="Sets the SLCKSEL field."]
    #[inline] pub fn set_slcksel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Preset Value Enable"]
    #[inline] pub fn pvalen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PVALEN != 0"]
    #[inline] pub fn test_pvalen(&self) -> bool {
        self.pvalen() != 0
    }

    #[doc="Sets the PVALEN field."]
    #[inline] pub fn set_pvalen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Hc2rEmmc {
    #[inline]
    fn from(other: u16) -> Self {
         Hc2rEmmc(other)
    }
}

impl ::core::fmt::Display for Hc2rEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hc2rEmmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hs200en() != 0 { try!(write!(f, " hs200en=0x{:x}", self.hs200en()))}
        if self.drvsel() != 0 { try!(write!(f, " drvsel=0x{:x}", self.drvsel()))}
        if self.extun() != 0 { try!(write!(f, " extun"))}
        if self.slcksel() != 0 { try!(write!(f, " slcksel"))}
        if self.pvalen() != 0 { try!(write!(f, " pvalen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capabilities 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ca0r(pub u32);
impl Ca0r {
    #[doc="Timeout Clock Frequency"]
    #[inline] pub fn teoclkf(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if TEOCLKF != 0"]
    #[inline] pub fn test_teoclkf(&self) -> bool {
        self.teoclkf() != 0
    }

    #[doc="Sets the TEOCLKF field."]
    #[inline] pub fn set_teoclkf<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timeout Clock Unit"]
    #[inline] pub fn teoclku(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TEOCLKU != 0"]
    #[inline] pub fn test_teoclku(&self) -> bool {
        self.teoclku() != 0
    }

    #[doc="Sets the TEOCLKU field."]
    #[inline] pub fn set_teoclku<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Base Clock Frequency"]
    #[inline] pub fn baseclkf(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if BASECLKF != 0"]
    #[inline] pub fn test_baseclkf(&self) -> bool {
        self.baseclkf() != 0
    }

    #[doc="Sets the BASECLKF field."]
    #[inline] pub fn set_baseclkf<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Max Block Length"]
    #[inline] pub fn maxblkl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if MAXBLKL != 0"]
    #[inline] pub fn test_maxblkl(&self) -> bool {
        self.maxblkl() != 0
    }

    #[doc="Sets the MAXBLKL field."]
    #[inline] pub fn set_maxblkl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="8-bit Support for Embedded Device"]
    #[inline] pub fn ed8sup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ED8SUP != 0"]
    #[inline] pub fn test_ed8sup(&self) -> bool {
        self.ed8sup() != 0
    }

    #[doc="Sets the ED8SUP field."]
    #[inline] pub fn set_ed8sup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="ADMA2 Support"]
    #[inline] pub fn adma2sup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if ADMA2SUP != 0"]
    #[inline] pub fn test_adma2sup(&self) -> bool {
        self.adma2sup() != 0
    }

    #[doc="Sets the ADMA2SUP field."]
    #[inline] pub fn set_adma2sup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="High Speed Support"]
    #[inline] pub fn hssup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if HSSUP != 0"]
    #[inline] pub fn test_hssup(&self) -> bool {
        self.hssup() != 0
    }

    #[doc="Sets the HSSUP field."]
    #[inline] pub fn set_hssup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SDMA Support"]
    #[inline] pub fn sdmasup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SDMASUP != 0"]
    #[inline] pub fn test_sdmasup(&self) -> bool {
        self.sdmasup() != 0
    }

    #[doc="Sets the SDMASUP field."]
    #[inline] pub fn set_sdmasup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Suspend/Resume Support"]
    #[inline] pub fn srsup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if SRSUP != 0"]
    #[inline] pub fn test_srsup(&self) -> bool {
        self.srsup() != 0
    }

    #[doc="Sets the SRSUP field."]
    #[inline] pub fn set_srsup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Voltage Support 3.3V"]
    #[inline] pub fn v33vsup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if V33VSUP != 0"]
    #[inline] pub fn test_v33vsup(&self) -> bool {
        self.v33vsup() != 0
    }

    #[doc="Sets the V33VSUP field."]
    #[inline] pub fn set_v33vsup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Voltage Support 3.0V"]
    #[inline] pub fn v30vsup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if V30VSUP != 0"]
    #[inline] pub fn test_v30vsup(&self) -> bool {
        self.v30vsup() != 0
    }

    #[doc="Sets the V30VSUP field."]
    #[inline] pub fn set_v30vsup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Voltage Support 1.8V"]
    #[inline] pub fn v18vsup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if V18VSUP != 0"]
    #[inline] pub fn test_v18vsup(&self) -> bool {
        self.v18vsup() != 0
    }

    #[doc="Sets the V18VSUP field."]
    #[inline] pub fn set_v18vsup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="64-Bit System Bus Support"]
    #[inline] pub fn sb64sup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SB64SUP != 0"]
    #[inline] pub fn test_sb64sup(&self) -> bool {
        self.sb64sup() != 0
    }

    #[doc="Sets the SB64SUP field."]
    #[inline] pub fn set_sb64sup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Asynchronous Interrupt Support"]
    #[inline] pub fn asintsup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ASINTSUP != 0"]
    #[inline] pub fn test_asintsup(&self) -> bool {
        self.asintsup() != 0
    }

    #[doc="Sets the ASINTSUP field."]
    #[inline] pub fn set_asintsup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Slot Type"]
    #[inline] pub fn sltype(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if SLTYPE != 0"]
    #[inline] pub fn test_sltype(&self) -> bool {
        self.sltype() != 0
    }

    #[doc="Sets the SLTYPE field."]
    #[inline] pub fn set_sltype<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Ca0r {
    #[inline]
    fn from(other: u32) -> Self {
         Ca0r(other)
    }
}

impl ::core::fmt::Display for Ca0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ca0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.teoclkf() != 0 { try!(write!(f, " teoclkf=0x{:x}", self.teoclkf()))}
        if self.teoclku() != 0 { try!(write!(f, " teoclku"))}
        if self.baseclkf() != 0 { try!(write!(f, " baseclkf=0x{:x}", self.baseclkf()))}
        if self.maxblkl() != 0 { try!(write!(f, " maxblkl=0x{:x}", self.maxblkl()))}
        if self.ed8sup() != 0 { try!(write!(f, " ed8sup"))}
        if self.adma2sup() != 0 { try!(write!(f, " adma2sup"))}
        if self.hssup() != 0 { try!(write!(f, " hssup"))}
        if self.sdmasup() != 0 { try!(write!(f, " sdmasup"))}
        if self.srsup() != 0 { try!(write!(f, " srsup"))}
        if self.v33vsup() != 0 { try!(write!(f, " v33vsup"))}
        if self.v30vsup() != 0 { try!(write!(f, " v30vsup"))}
        if self.v18vsup() != 0 { try!(write!(f, " v18vsup"))}
        if self.sb64sup() != 0 { try!(write!(f, " sb64sup"))}
        if self.asintsup() != 0 { try!(write!(f, " asintsup"))}
        if self.sltype() != 0 { try!(write!(f, " sltype=0x{:x}", self.sltype()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capabilities 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ca1r(pub u32);
impl Ca1r {
    #[doc="SDR50 Support"]
    #[inline] pub fn sdr50sup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SDR50SUP != 0"]
    #[inline] pub fn test_sdr50sup(&self) -> bool {
        self.sdr50sup() != 0
    }

    #[doc="Sets the SDR50SUP field."]
    #[inline] pub fn set_sdr50sup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SDR104 Support"]
    #[inline] pub fn sdr104sup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SDR104SUP != 0"]
    #[inline] pub fn test_sdr104sup(&self) -> bool {
        self.sdr104sup() != 0
    }

    #[doc="Sets the SDR104SUP field."]
    #[inline] pub fn set_sdr104sup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DDR50 Support"]
    #[inline] pub fn ddr50sup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DDR50SUP != 0"]
    #[inline] pub fn test_ddr50sup(&self) -> bool {
        self.ddr50sup() != 0
    }

    #[doc="Sets the DDR50SUP field."]
    #[inline] pub fn set_ddr50sup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Driver Type A Support"]
    #[inline] pub fn drvasup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DRVASUP != 0"]
    #[inline] pub fn test_drvasup(&self) -> bool {
        self.drvasup() != 0
    }

    #[doc="Sets the DRVASUP field."]
    #[inline] pub fn set_drvasup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Driver Type C Support"]
    #[inline] pub fn drvcsup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DRVCSUP != 0"]
    #[inline] pub fn test_drvcsup(&self) -> bool {
        self.drvcsup() != 0
    }

    #[doc="Sets the DRVCSUP field."]
    #[inline] pub fn set_drvcsup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Driver Type D Support"]
    #[inline] pub fn drvdsup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DRVDSUP != 0"]
    #[inline] pub fn test_drvdsup(&self) -> bool {
        self.drvdsup() != 0
    }

    #[doc="Sets the DRVDSUP field."]
    #[inline] pub fn set_drvdsup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Timer Count for Re-Tuning"]
    #[inline] pub fn tcntrt(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if TCNTRT != 0"]
    #[inline] pub fn test_tcntrt(&self) -> bool {
        self.tcntrt() != 0
    }

    #[doc="Sets the TCNTRT field."]
    #[inline] pub fn set_tcntrt<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Use Tuning for SDR50"]
    #[inline] pub fn tsdr50(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TSDR50 != 0"]
    #[inline] pub fn test_tsdr50(&self) -> bool {
        self.tsdr50() != 0
    }

    #[doc="Sets the TSDR50 field."]
    #[inline] pub fn set_tsdr50<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Clock Multiplier"]
    #[inline] pub fn clkmult(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CLKMULT != 0"]
    #[inline] pub fn test_clkmult(&self) -> bool {
        self.clkmult() != 0
    }

    #[doc="Sets the CLKMULT field."]
    #[inline] pub fn set_clkmult<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Ca1r {
    #[inline]
    fn from(other: u32) -> Self {
         Ca1r(other)
    }
}

impl ::core::fmt::Display for Ca1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ca1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sdr50sup() != 0 { try!(write!(f, " sdr50sup"))}
        if self.sdr104sup() != 0 { try!(write!(f, " sdr104sup"))}
        if self.ddr50sup() != 0 { try!(write!(f, " ddr50sup"))}
        if self.drvasup() != 0 { try!(write!(f, " drvasup"))}
        if self.drvcsup() != 0 { try!(write!(f, " drvcsup"))}
        if self.drvdsup() != 0 { try!(write!(f, " drvdsup"))}
        if self.tcntrt() != 0 { try!(write!(f, " tcntrt=0x{:x}", self.tcntrt()))}
        if self.tsdr50() != 0 { try!(write!(f, " tsdr50"))}
        if self.clkmult() != 0 { try!(write!(f, " clkmult=0x{:x}", self.clkmult()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Maximum Current Capabilities"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mccar(pub u32);
impl Mccar {
    #[doc="Maximum Current for 3.3V"]
    #[inline] pub fn maxcur33v(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MAXCUR33V != 0"]
    #[inline] pub fn test_maxcur33v(&self) -> bool {
        self.maxcur33v() != 0
    }

    #[doc="Sets the MAXCUR33V field."]
    #[inline] pub fn set_maxcur33v<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Maximum Current for 3.0V"]
    #[inline] pub fn maxcur30v(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if MAXCUR30V != 0"]
    #[inline] pub fn test_maxcur30v(&self) -> bool {
        self.maxcur30v() != 0
    }

    #[doc="Sets the MAXCUR30V field."]
    #[inline] pub fn set_maxcur30v<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Maximum Current for 1.8V"]
    #[inline] pub fn maxcur18v(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MAXCUR18V != 0"]
    #[inline] pub fn test_maxcur18v(&self) -> bool {
        self.maxcur18v() != 0
    }

    #[doc="Sets the MAXCUR18V field."]
    #[inline] pub fn set_maxcur18v<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Mccar {
    #[inline]
    fn from(other: u32) -> Self {
         Mccar(other)
    }
}

impl ::core::fmt::Display for Mccar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mccar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.maxcur33v() != 0 { try!(write!(f, " maxcur33v=0x{:x}", self.maxcur33v()))}
        if self.maxcur30v() != 0 { try!(write!(f, " maxcur30v=0x{:x}", self.maxcur30v()))}
        if self.maxcur18v() != 0 { try!(write!(f, " maxcur18v=0x{:x}", self.maxcur18v()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Force Event for Auto CMD Error Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Feraces(pub u16);
impl Feraces {
    #[doc="Force Event for Auto CMD12 Not Executed"]
    #[inline] pub fn acmd12ne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ACMD12NE != 0"]
    #[inline] pub fn test_acmd12ne(&self) -> bool {
        self.acmd12ne() != 0
    }

    #[doc="Sets the ACMD12NE field."]
    #[inline] pub fn set_acmd12ne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Force Event for Auto CMD Timeout Error"]
    #[inline] pub fn acmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ACMDTEO != 0"]
    #[inline] pub fn test_acmdteo(&self) -> bool {
        self.acmdteo() != 0
    }

    #[doc="Sets the ACMDTEO field."]
    #[inline] pub fn set_acmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Force Event for Auto CMD CRC Error"]
    #[inline] pub fn acmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ACMDCRC != 0"]
    #[inline] pub fn test_acmdcrc(&self) -> bool {
        self.acmdcrc() != 0
    }

    #[doc="Sets the ACMDCRC field."]
    #[inline] pub fn set_acmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Force Event for Auto CMD End Bit Error"]
    #[inline] pub fn acmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ACMDEND != 0"]
    #[inline] pub fn test_acmdend(&self) -> bool {
        self.acmdend() != 0
    }

    #[doc="Sets the ACMDEND field."]
    #[inline] pub fn set_acmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Force Event for Auto CMD Index Error"]
    #[inline] pub fn acmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ACMDIDX != 0"]
    #[inline] pub fn test_acmdidx(&self) -> bool {
        self.acmdidx() != 0
    }

    #[doc="Sets the ACMDIDX field."]
    #[inline] pub fn set_acmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline] pub fn cmdni(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CMDNI != 0"]
    #[inline] pub fn test_cmdni(&self) -> bool {
        self.cmdni() != 0
    }

    #[doc="Sets the CMDNI field."]
    #[inline] pub fn set_cmdni<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u16> for Feraces {
    #[inline]
    fn from(other: u16) -> Self {
         Feraces(other)
    }
}

impl ::core::fmt::Display for Feraces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Feraces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.acmd12ne() != 0 { try!(write!(f, " acmd12ne"))}
        if self.acmdteo() != 0 { try!(write!(f, " acmdteo"))}
        if self.acmdcrc() != 0 { try!(write!(f, " acmdcrc"))}
        if self.acmdend() != 0 { try!(write!(f, " acmdend"))}
        if self.acmdidx() != 0 { try!(write!(f, " acmdidx"))}
        if self.cmdni() != 0 { try!(write!(f, " cmdni"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Force Event for Error Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fereis(pub u16);
impl Fereis {
    #[doc="Force Event for Command Timeout Error"]
    #[inline] pub fn cmdteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDTEO != 0"]
    #[inline] pub fn test_cmdteo(&self) -> bool {
        self.cmdteo() != 0
    }

    #[doc="Sets the CMDTEO field."]
    #[inline] pub fn set_cmdteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Force Event for Command CRC Error"]
    #[inline] pub fn cmdcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDCRC != 0"]
    #[inline] pub fn test_cmdcrc(&self) -> bool {
        self.cmdcrc() != 0
    }

    #[doc="Sets the CMDCRC field."]
    #[inline] pub fn set_cmdcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Force Event for Command End Bit Error"]
    #[inline] pub fn cmdend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CMDEND != 0"]
    #[inline] pub fn test_cmdend(&self) -> bool {
        self.cmdend() != 0
    }

    #[doc="Sets the CMDEND field."]
    #[inline] pub fn set_cmdend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Force Event for Command Index Error"]
    #[inline] pub fn cmdidx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMDIDX != 0"]
    #[inline] pub fn test_cmdidx(&self) -> bool {
        self.cmdidx() != 0
    }

    #[doc="Sets the CMDIDX field."]
    #[inline] pub fn set_cmdidx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Force Event for Data Timeout Error"]
    #[inline] pub fn datteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DATTEO != 0"]
    #[inline] pub fn test_datteo(&self) -> bool {
        self.datteo() != 0
    }

    #[doc="Sets the DATTEO field."]
    #[inline] pub fn set_datteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Force Event for Data CRC Error"]
    #[inline] pub fn datcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DATCRC != 0"]
    #[inline] pub fn test_datcrc(&self) -> bool {
        self.datcrc() != 0
    }

    #[doc="Sets the DATCRC field."]
    #[inline] pub fn set_datcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Force Event for Data End Bit Error"]
    #[inline] pub fn datend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DATEND != 0"]
    #[inline] pub fn test_datend(&self) -> bool {
        self.datend() != 0
    }

    #[doc="Sets the DATEND field."]
    #[inline] pub fn set_datend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Force Event for Current Limit Error"]
    #[inline] pub fn curlim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CURLIM != 0"]
    #[inline] pub fn test_curlim(&self) -> bool {
        self.curlim() != 0
    }

    #[doc="Sets the CURLIM field."]
    #[inline] pub fn set_curlim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Force Event for Auto CMD Error"]
    #[inline] pub fn acmd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ACMD != 0"]
    #[inline] pub fn test_acmd(&self) -> bool {
        self.acmd() != 0
    }

    #[doc="Sets the ACMD field."]
    #[inline] pub fn set_acmd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Force Event for ADMA Error"]
    #[inline] pub fn adma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADMA != 0"]
    #[inline] pub fn test_adma(&self) -> bool {
        self.adma() != 0
    }

    #[doc="Sets the ADMA field."]
    #[inline] pub fn set_adma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Force Event for Boot Acknowledge Error"]
    #[inline] pub fn bootae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if BOOTAE != 0"]
    #[inline] pub fn test_bootae(&self) -> bool {
        self.bootae() != 0
    }

    #[doc="Sets the BOOTAE field."]
    #[inline] pub fn set_bootae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Fereis {
    #[inline]
    fn from(other: u16) -> Self {
         Fereis(other)
    }
}

impl ::core::fmt::Display for Fereis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fereis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdteo() != 0 { try!(write!(f, " cmdteo"))}
        if self.cmdcrc() != 0 { try!(write!(f, " cmdcrc"))}
        if self.cmdend() != 0 { try!(write!(f, " cmdend"))}
        if self.cmdidx() != 0 { try!(write!(f, " cmdidx"))}
        if self.datteo() != 0 { try!(write!(f, " datteo"))}
        if self.datcrc() != 0 { try!(write!(f, " datcrc"))}
        if self.datend() != 0 { try!(write!(f, " datend"))}
        if self.curlim() != 0 { try!(write!(f, " curlim"))}
        if self.acmd() != 0 { try!(write!(f, " acmd"))}
        if self.adma() != 0 { try!(write!(f, " adma"))}
        if self.bootae() != 0 { try!(write!(f, " bootae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADMA Error Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aesr(pub u8);
impl Aesr {
    #[doc="ADMA Error State"]
    #[inline] pub fn errst(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ERRST != 0"]
    #[inline] pub fn test_errst(&self) -> bool {
        self.errst() != 0
    }

    #[doc="Sets the ERRST field."]
    #[inline] pub fn set_errst<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADMA Length Mismatch Error"]
    #[inline] pub fn lmis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LMIS != 0"]
    #[inline] pub fn test_lmis(&self) -> bool {
        self.lmis() != 0
    }

    #[doc="Sets the LMIS field."]
    #[inline] pub fn set_lmis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Aesr {
    #[inline]
    fn from(other: u8) -> Self {
         Aesr(other)
    }
}

impl ::core::fmt::Display for Aesr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aesr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.errst() != 0 { try!(write!(f, " errst=0x{:x}", self.errst()))}
        if self.lmis() != 0 { try!(write!(f, " lmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADMA System Address n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Asar(pub u32);
impl Asar {
    #[doc="ADMA System Address"]
    #[inline] pub fn admasa(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADMASA != 0"]
    #[inline] pub fn test_admasa(&self) -> bool {
        self.admasa() != 0
    }

    #[doc="Sets the ADMASA field."]
    #[inline] pub fn set_admasa<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Asar {
    #[inline]
    fn from(other: u32) -> Self {
         Asar(other)
    }
}

impl ::core::fmt::Display for Asar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Asar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Preset Value n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pvr(pub u16);
impl Pvr {
    #[doc="SDCLK Frequency Select Value for Initialization"]
    #[inline] pub fn sdclkfsel(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if SDCLKFSEL != 0"]
    #[inline] pub fn test_sdclkfsel(&self) -> bool {
        self.sdclkfsel() != 0
    }

    #[doc="Sets the SDCLKFSEL field."]
    #[inline] pub fn set_sdclkfsel<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Generator Select Value for Initialization"]
    #[inline] pub fn clkgsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CLKGSEL != 0"]
    #[inline] pub fn test_clkgsel(&self) -> bool {
        self.clkgsel() != 0
    }

    #[doc="Sets the CLKGSEL field."]
    #[inline] pub fn set_clkgsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Driver Strength Select Value for Initialization"]
    #[inline] pub fn drvsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DRVSEL != 0"]
    #[inline] pub fn test_drvsel(&self) -> bool {
        self.drvsel() != 0
    }

    #[doc="Sets the DRVSEL field."]
    #[inline] pub fn set_drvsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u16> for Pvr {
    #[inline]
    fn from(other: u16) -> Self {
         Pvr(other)
    }
}

impl ::core::fmt::Display for Pvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sdclkfsel() != 0 { try!(write!(f, " sdclkfsel=0x{:x}", self.sdclkfsel()))}
        if self.clkgsel() != 0 { try!(write!(f, " clkgsel"))}
        if self.drvsel() != 0 { try!(write!(f, " drvsel=0x{:x}", self.drvsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sisr(pub u16);
impl Sisr {
    #[doc="Interrupt Signal for Each Slot"]
    #[inline] pub fn intssl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INTSSL != 0"]
    #[inline] pub fn test_intssl(&self) -> bool {
        self.intssl() != 0
    }

    #[doc="Sets the INTSSL field."]
    #[inline] pub fn set_intssl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Sisr {
    #[inline]
    fn from(other: u16) -> Self {
         Sisr(other)
    }
}

impl ::core::fmt::Display for Sisr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sisr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.intssl() != 0 { try!(write!(f, " intssl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Host Controller Version"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcvr(pub u16);
impl Hcvr {
    #[doc="Spec Version"]
    #[inline] pub fn sver(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SVER != 0"]
    #[inline] pub fn test_sver(&self) -> bool {
        self.sver() != 0
    }

    #[doc="Sets the SVER field."]
    #[inline] pub fn set_sver<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Vendor Version"]
    #[inline] pub fn vver(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if VVER != 0"]
    #[inline] pub fn test_vver(&self) -> bool {
        self.vver() != 0
    }

    #[doc="Sets the VVER field."]
    #[inline] pub fn set_vver<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Hcvr {
    #[inline]
    fn from(other: u16) -> Self {
         Hcvr(other)
    }
}

impl ::core::fmt::Display for Hcvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sver() != 0 { try!(write!(f, " sver=0x{:x}", self.sver()))}
        if self.vver() != 0 { try!(write!(f, " vver=0x{:x}", self.vver()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MMC Control 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mc1r(pub u8);
impl Mc1r {
    #[doc="e.MMC Command Type"]
    #[inline] pub fn cmdtyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CMDTYP != 0"]
    #[inline] pub fn test_cmdtyp(&self) -> bool {
        self.cmdtyp() != 0
    }

    #[doc="Sets the CMDTYP field."]
    #[inline] pub fn set_cmdtyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="e.MMC HSDDR Mode"]
    #[inline] pub fn ddr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DDR != 0"]
    #[inline] pub fn test_ddr(&self) -> bool {
        self.ddr() != 0
    }

    #[doc="Sets the DDR field."]
    #[inline] pub fn set_ddr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="e.MMC Open Drain Mode"]
    #[inline] pub fn opd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OPD != 0"]
    #[inline] pub fn test_opd(&self) -> bool {
        self.opd() != 0
    }

    #[doc="Sets the OPD field."]
    #[inline] pub fn set_opd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="e.MMC Boot Acknowledge Enable"]
    #[inline] pub fn boota(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BOOTA != 0"]
    #[inline] pub fn test_boota(&self) -> bool {
        self.boota() != 0
    }

    #[doc="Sets the BOOTA field."]
    #[inline] pub fn set_boota<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="e.MMC Reset Signal"]
    #[inline] pub fn rstn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RSTN != 0"]
    #[inline] pub fn test_rstn(&self) -> bool {
        self.rstn() != 0
    }

    #[doc="Sets the RSTN field."]
    #[inline] pub fn set_rstn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="e.MMC Force Card Detect"]
    #[inline] pub fn fcd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FCD != 0"]
    #[inline] pub fn test_fcd(&self) -> bool {
        self.fcd() != 0
    }

    #[doc="Sets the FCD field."]
    #[inline] pub fn set_fcd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Mc1r {
    #[inline]
    fn from(other: u8) -> Self {
         Mc1r(other)
    }
}

impl ::core::fmt::Display for Mc1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mc1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdtyp() != 0 { try!(write!(f, " cmdtyp=0x{:x}", self.cmdtyp()))}
        if self.ddr() != 0 { try!(write!(f, " ddr"))}
        if self.opd() != 0 { try!(write!(f, " opd"))}
        if self.boota() != 0 { try!(write!(f, " boota"))}
        if self.rstn() != 0 { try!(write!(f, " rstn"))}
        if self.fcd() != 0 { try!(write!(f, " fcd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MMC Control 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mc2r(pub u8);
impl Mc2r {
    #[doc="e.MMC Abort Wait IRQ"]
    #[inline] pub fn sresp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SRESP != 0"]
    #[inline] pub fn test_sresp(&self) -> bool {
        self.sresp() != 0
    }

    #[doc="Sets the SRESP field."]
    #[inline] pub fn set_sresp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="e.MMC Abort Boot"]
    #[inline] pub fn aboot(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ABOOT != 0"]
    #[inline] pub fn test_aboot(&self) -> bool {
        self.aboot() != 0
    }

    #[doc="Sets the ABOOT field."]
    #[inline] pub fn set_aboot<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Mc2r {
    #[inline]
    fn from(other: u8) -> Self {
         Mc2r(other)
    }
}

impl ::core::fmt::Display for Mc2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mc2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sresp() != 0 { try!(write!(f, " sresp"))}
        if self.aboot() != 0 { try!(write!(f, " aboot"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="AHB Maximum Burst"]
    #[inline] pub fn bmax(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if BMAX != 0"]
    #[inline] pub fn test_bmax(&self) -> bool {
        self.bmax() != 0
    }

    #[doc="Sets the BMAX field."]
    #[inline] pub fn set_bmax<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Acr {
    #[inline]
    fn from(other: u32) -> Self {
         Acr(other)
    }
}

impl ::core::fmt::Display for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bmax() != 0 { try!(write!(f, " bmax=0x{:x}", self.bmax()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Control 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc2r(pub u32);
impl Cc2r {
    #[doc="Force SDCK Disabled"]
    #[inline] pub fn fsdclkd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSDCLKD != 0"]
    #[inline] pub fn test_fsdclkd(&self) -> bool {
        self.fsdclkd() != 0
    }

    #[doc="Sets the FSDCLKD field."]
    #[inline] pub fn set_fsdclkd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cc2r {
    #[inline]
    fn from(other: u32) -> Self {
         Cc2r(other)
    }
}

impl ::core::fmt::Display for Cc2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cc2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsdclkd() != 0 { try!(write!(f, " fsdclkd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capabilities Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cacr(pub u32);
impl Cacr {
    #[doc="Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline] pub fn capwren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CAPWREN != 0"]
    #[inline] pub fn test_capwren(&self) -> bool {
        self.capwren() != 0
    }

    #[doc="Sets the CAPWREN field."]
    #[inline] pub fn set_capwren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Key (0x46)"]
    #[inline] pub fn key(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Cacr {
    #[inline]
    fn from(other: u32) -> Self {
         Cacr(other)
    }
}

impl ::core::fmt::Display for Cacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.capwren() != 0 { try!(write!(f, " capwren"))}
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgr(pub u8);
impl Dbgr {
    #[doc="Non-intrusive debug enable"]
    #[inline] pub fn nidbg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if NIDBG != 0"]
    #[inline] pub fn test_nidbg(&self) -> bool {
        self.nidbg() != 0
    }

    #[doc="Sets the NIDBG field."]
    #[inline] pub fn set_nidbg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Dbgr {
    #[inline]
    fn from(other: u8) -> Self {
         Dbgr(other)
    }
}

impl ::core::fmt::Display for Dbgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nidbg() != 0 { try!(write!(f, " nidbg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

