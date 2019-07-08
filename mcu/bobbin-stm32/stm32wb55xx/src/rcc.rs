
::bobbin_mcu::periph!( RCC, Rcc, RCC_PERIPH, RccPeriph, RCC_OWNED, RCC_REF_COUNT, 0x58000000, 0x00, 0x02);


#[doc="Reset and clock control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RccPeriph(pub usize);
impl RccPeriph {
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

    #[doc="Get the ICSCR Register."]
    #[inline] pub fn icscr_reg(&self) -> ::bobbin_mcu::register::Register<Icscr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Icscr, 0x4)
    }

    #[doc="Get the *mut pointer for the ICSCR register."]
    #[inline] pub fn icscr_mut(&self) -> *mut Icscr { 
        self.icscr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ICSCR register."]
    #[inline] pub fn icscr_ptr(&self) -> *const Icscr { 
        self.icscr_reg().ptr()
    }

    #[doc="Read the ICSCR register."]
    #[inline] pub fn icscr(&self) -> Icscr { 
        self.icscr_reg().read()
    }

    #[doc="Write the ICSCR register."]
    #[inline] pub fn write_icscr(&self, value: Icscr) -> &Self { 
        self.icscr_reg().write(value);
        self
    }

    #[doc="Set the ICSCR register."]
    #[inline] pub fn set_icscr<F: FnOnce(Icscr) -> Icscr>(&self, f: F) -> &Self {
        self.icscr_reg().set(f);
        self
    }

    #[doc="Modify the ICSCR register."]
    #[inline] pub fn with_icscr<F: FnOnce(Icscr) -> Icscr>(&self, f: F) -> &Self {
        self.icscr_reg().with(f);
        self
    }

    #[doc="Get the CFGR Register."]
    #[inline] pub fn cfgr_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr, 0x8)
    }

    #[doc="Get the *mut pointer for the CFGR register."]
    #[inline] pub fn cfgr_mut(&self) -> *mut Cfgr { 
        self.cfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR register."]
    #[inline] pub fn cfgr_ptr(&self) -> *const Cfgr { 
        self.cfgr_reg().ptr()
    }

    #[doc="Read the CFGR register."]
    #[inline] pub fn cfgr(&self) -> Cfgr { 
        self.cfgr_reg().read()
    }

    #[doc="Write the CFGR register."]
    #[inline] pub fn write_cfgr(&self, value: Cfgr) -> &Self { 
        self.cfgr_reg().write(value);
        self
    }

    #[doc="Set the CFGR register."]
    #[inline] pub fn set_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        self.cfgr_reg().set(f);
        self
    }

    #[doc="Modify the CFGR register."]
    #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        self.cfgr_reg().with(f);
        self
    }

    #[doc="Get the PLLCFGR Register."]
    #[inline] pub fn pllcfgr_reg(&self) -> ::bobbin_mcu::register::Register<Pllcfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pllcfgr, 0xc)
    }

    #[doc="Get the *mut pointer for the PLLCFGR register."]
    #[inline] pub fn pllcfgr_mut(&self) -> *mut Pllcfgr { 
        self.pllcfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PLLCFGR register."]
    #[inline] pub fn pllcfgr_ptr(&self) -> *const Pllcfgr { 
        self.pllcfgr_reg().ptr()
    }

    #[doc="Read the PLLCFGR register."]
    #[inline] pub fn pllcfgr(&self) -> Pllcfgr { 
        self.pllcfgr_reg().read()
    }

    #[doc="Write the PLLCFGR register."]
    #[inline] pub fn write_pllcfgr(&self, value: Pllcfgr) -> &Self { 
        self.pllcfgr_reg().write(value);
        self
    }

    #[doc="Set the PLLCFGR register."]
    #[inline] pub fn set_pllcfgr<F: FnOnce(Pllcfgr) -> Pllcfgr>(&self, f: F) -> &Self {
        self.pllcfgr_reg().set(f);
        self
    }

    #[doc="Modify the PLLCFGR register."]
    #[inline] pub fn with_pllcfgr<F: FnOnce(Pllcfgr) -> Pllcfgr>(&self, f: F) -> &Self {
        self.pllcfgr_reg().with(f);
        self
    }

    #[doc="Get the PLLSAI1CFGR Register."]
    #[inline] pub fn pllsai1cfgr_reg(&self) -> ::bobbin_mcu::register::Register<Pllsai1cfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pllsai1cfgr, 0x10)
    }

    #[doc="Get the *mut pointer for the PLLSAI1CFGR register."]
    #[inline] pub fn pllsai1cfgr_mut(&self) -> *mut Pllsai1cfgr { 
        self.pllsai1cfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PLLSAI1CFGR register."]
    #[inline] pub fn pllsai1cfgr_ptr(&self) -> *const Pllsai1cfgr { 
        self.pllsai1cfgr_reg().ptr()
    }

    #[doc="Read the PLLSAI1CFGR register."]
    #[inline] pub fn pllsai1cfgr(&self) -> Pllsai1cfgr { 
        self.pllsai1cfgr_reg().read()
    }

    #[doc="Write the PLLSAI1CFGR register."]
    #[inline] pub fn write_pllsai1cfgr(&self, value: Pllsai1cfgr) -> &Self { 
        self.pllsai1cfgr_reg().write(value);
        self
    }

    #[doc="Set the PLLSAI1CFGR register."]
    #[inline] pub fn set_pllsai1cfgr<F: FnOnce(Pllsai1cfgr) -> Pllsai1cfgr>(&self, f: F) -> &Self {
        self.pllsai1cfgr_reg().set(f);
        self
    }

    #[doc="Modify the PLLSAI1CFGR register."]
    #[inline] pub fn with_pllsai1cfgr<F: FnOnce(Pllsai1cfgr) -> Pllsai1cfgr>(&self, f: F) -> &Self {
        self.pllsai1cfgr_reg().with(f);
        self
    }

    #[doc="Get the CIER Register."]
    #[inline] pub fn cier_reg(&self) -> ::bobbin_mcu::register::Register<Cier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cier, 0x18)
    }

    #[doc="Get the *mut pointer for the CIER register."]
    #[inline] pub fn cier_mut(&self) -> *mut Cier { 
        self.cier_reg().ptr()
    }

    #[doc="Get the *const pointer for the CIER register."]
    #[inline] pub fn cier_ptr(&self) -> *const Cier { 
        self.cier_reg().ptr()
    }

    #[doc="Read the CIER register."]
    #[inline] pub fn cier(&self) -> Cier { 
        self.cier_reg().read()
    }

    #[doc="Write the CIER register."]
    #[inline] pub fn write_cier(&self, value: Cier) -> &Self { 
        self.cier_reg().write(value);
        self
    }

    #[doc="Set the CIER register."]
    #[inline] pub fn set_cier<F: FnOnce(Cier) -> Cier>(&self, f: F) -> &Self {
        self.cier_reg().set(f);
        self
    }

    #[doc="Modify the CIER register."]
    #[inline] pub fn with_cier<F: FnOnce(Cier) -> Cier>(&self, f: F) -> &Self {
        self.cier_reg().with(f);
        self
    }

    #[doc="Get the CIFR Register."]
    #[inline] pub fn cifr_reg(&self) -> ::bobbin_mcu::register::Register<Cifr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cifr, 0x1c)
    }

    #[doc="Get the *mut pointer for the CIFR register."]
    #[inline] pub fn cifr_mut(&self) -> *mut Cifr { 
        self.cifr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CIFR register."]
    #[inline] pub fn cifr_ptr(&self) -> *const Cifr { 
        self.cifr_reg().ptr()
    }

    #[doc="Read the CIFR register."]
    #[inline] pub fn cifr(&self) -> Cifr { 
        self.cifr_reg().read()
    }

    #[doc="Get the CICR Register."]
    #[inline] pub fn cicr_reg(&self) -> ::bobbin_mcu::register::Register<Cicr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cicr, 0x20)
    }

    #[doc="Get the *mut pointer for the CICR register."]
    #[inline] pub fn cicr_mut(&self) -> *mut Cicr { 
        self.cicr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CICR register."]
    #[inline] pub fn cicr_ptr(&self) -> *const Cicr { 
        self.cicr_reg().ptr()
    }

    #[doc="Write the CICR register."]
    #[inline] pub fn write_cicr(&self, value: Cicr) -> &Self { 
        self.cicr_reg().write(value);
        self
    }

    #[doc="Set the CICR register."]
    #[inline] pub fn set_cicr<F: FnOnce(Cicr) -> Cicr>(&self, f: F) -> &Self {
        self.cicr_reg().set(f);
        self
    }

    #[doc="Get the SMPSCR Register."]
    #[inline] pub fn smpscr_reg(&self) -> ::bobbin_mcu::register::Register<Smpscr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Smpscr, 0x24)
    }

    #[doc="Get the *mut pointer for the SMPSCR register."]
    #[inline] pub fn smpscr_mut(&self) -> *mut Smpscr { 
        self.smpscr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SMPSCR register."]
    #[inline] pub fn smpscr_ptr(&self) -> *const Smpscr { 
        self.smpscr_reg().ptr()
    }

    #[doc="Read the SMPSCR register."]
    #[inline] pub fn smpscr(&self) -> Smpscr { 
        self.smpscr_reg().read()
    }

    #[doc="Write the SMPSCR register."]
    #[inline] pub fn write_smpscr(&self, value: Smpscr) -> &Self { 
        self.smpscr_reg().write(value);
        self
    }

    #[doc="Set the SMPSCR register."]
    #[inline] pub fn set_smpscr<F: FnOnce(Smpscr) -> Smpscr>(&self, f: F) -> &Self {
        self.smpscr_reg().set(f);
        self
    }

    #[doc="Modify the SMPSCR register."]
    #[inline] pub fn with_smpscr<F: FnOnce(Smpscr) -> Smpscr>(&self, f: F) -> &Self {
        self.smpscr_reg().with(f);
        self
    }

    #[doc="Get the AHB1RSTR Register."]
    #[inline] pub fn ahb1rstr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb1rstr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb1rstr, 0x28)
    }

    #[doc="Get the *mut pointer for the AHB1RSTR register."]
    #[inline] pub fn ahb1rstr_mut(&self) -> *mut Ahb1rstr { 
        self.ahb1rstr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB1RSTR register."]
    #[inline] pub fn ahb1rstr_ptr(&self) -> *const Ahb1rstr { 
        self.ahb1rstr_reg().ptr()
    }

    #[doc="Read the AHB1RSTR register."]
    #[inline] pub fn ahb1rstr(&self) -> Ahb1rstr { 
        self.ahb1rstr_reg().read()
    }

    #[doc="Write the AHB1RSTR register."]
    #[inline] pub fn write_ahb1rstr(&self, value: Ahb1rstr) -> &Self { 
        self.ahb1rstr_reg().write(value);
        self
    }

    #[doc="Set the AHB1RSTR register."]
    #[inline] pub fn set_ahb1rstr<F: FnOnce(Ahb1rstr) -> Ahb1rstr>(&self, f: F) -> &Self {
        self.ahb1rstr_reg().set(f);
        self
    }

    #[doc="Modify the AHB1RSTR register."]
    #[inline] pub fn with_ahb1rstr<F: FnOnce(Ahb1rstr) -> Ahb1rstr>(&self, f: F) -> &Self {
        self.ahb1rstr_reg().with(f);
        self
    }

    #[doc="Get the AHB2RSTR Register."]
    #[inline] pub fn ahb2rstr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb2rstr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb2rstr, 0x2c)
    }

    #[doc="Get the *mut pointer for the AHB2RSTR register."]
    #[inline] pub fn ahb2rstr_mut(&self) -> *mut Ahb2rstr { 
        self.ahb2rstr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB2RSTR register."]
    #[inline] pub fn ahb2rstr_ptr(&self) -> *const Ahb2rstr { 
        self.ahb2rstr_reg().ptr()
    }

    #[doc="Read the AHB2RSTR register."]
    #[inline] pub fn ahb2rstr(&self) -> Ahb2rstr { 
        self.ahb2rstr_reg().read()
    }

    #[doc="Write the AHB2RSTR register."]
    #[inline] pub fn write_ahb2rstr(&self, value: Ahb2rstr) -> &Self { 
        self.ahb2rstr_reg().write(value);
        self
    }

    #[doc="Set the AHB2RSTR register."]
    #[inline] pub fn set_ahb2rstr<F: FnOnce(Ahb2rstr) -> Ahb2rstr>(&self, f: F) -> &Self {
        self.ahb2rstr_reg().set(f);
        self
    }

    #[doc="Modify the AHB2RSTR register."]
    #[inline] pub fn with_ahb2rstr<F: FnOnce(Ahb2rstr) -> Ahb2rstr>(&self, f: F) -> &Self {
        self.ahb2rstr_reg().with(f);
        self
    }

    #[doc="Get the AHB3RSTR Register."]
    #[inline] pub fn ahb3rstr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb3rstr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb3rstr, 0x30)
    }

    #[doc="Get the *mut pointer for the AHB3RSTR register."]
    #[inline] pub fn ahb3rstr_mut(&self) -> *mut Ahb3rstr { 
        self.ahb3rstr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB3RSTR register."]
    #[inline] pub fn ahb3rstr_ptr(&self) -> *const Ahb3rstr { 
        self.ahb3rstr_reg().ptr()
    }

    #[doc="Read the AHB3RSTR register."]
    #[inline] pub fn ahb3rstr(&self) -> Ahb3rstr { 
        self.ahb3rstr_reg().read()
    }

    #[doc="Write the AHB3RSTR register."]
    #[inline] pub fn write_ahb3rstr(&self, value: Ahb3rstr) -> &Self { 
        self.ahb3rstr_reg().write(value);
        self
    }

    #[doc="Set the AHB3RSTR register."]
    #[inline] pub fn set_ahb3rstr<F: FnOnce(Ahb3rstr) -> Ahb3rstr>(&self, f: F) -> &Self {
        self.ahb3rstr_reg().set(f);
        self
    }

    #[doc="Modify the AHB3RSTR register."]
    #[inline] pub fn with_ahb3rstr<F: FnOnce(Ahb3rstr) -> Ahb3rstr>(&self, f: F) -> &Self {
        self.ahb3rstr_reg().with(f);
        self
    }

    #[doc="Get the APB1RSTR1 Register."]
    #[inline] pub fn apb1rstr1_reg(&self) -> ::bobbin_mcu::register::Register<Apb1rstr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1rstr1, 0x38)
    }

    #[doc="Get the *mut pointer for the APB1RSTR1 register."]
    #[inline] pub fn apb1rstr1_mut(&self) -> *mut Apb1rstr1 { 
        self.apb1rstr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1RSTR1 register."]
    #[inline] pub fn apb1rstr1_ptr(&self) -> *const Apb1rstr1 { 
        self.apb1rstr1_reg().ptr()
    }

    #[doc="Read the APB1RSTR1 register."]
    #[inline] pub fn apb1rstr1(&self) -> Apb1rstr1 { 
        self.apb1rstr1_reg().read()
    }

    #[doc="Write the APB1RSTR1 register."]
    #[inline] pub fn write_apb1rstr1(&self, value: Apb1rstr1) -> &Self { 
        self.apb1rstr1_reg().write(value);
        self
    }

    #[doc="Set the APB1RSTR1 register."]
    #[inline] pub fn set_apb1rstr1<F: FnOnce(Apb1rstr1) -> Apb1rstr1>(&self, f: F) -> &Self {
        self.apb1rstr1_reg().set(f);
        self
    }

    #[doc="Modify the APB1RSTR1 register."]
    #[inline] pub fn with_apb1rstr1<F: FnOnce(Apb1rstr1) -> Apb1rstr1>(&self, f: F) -> &Self {
        self.apb1rstr1_reg().with(f);
        self
    }

    #[doc="Get the APB1RSTR2 Register."]
    #[inline] pub fn apb1rstr2_reg(&self) -> ::bobbin_mcu::register::Register<Apb1rstr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1rstr2, 0x3c)
    }

    #[doc="Get the *mut pointer for the APB1RSTR2 register."]
    #[inline] pub fn apb1rstr2_mut(&self) -> *mut Apb1rstr2 { 
        self.apb1rstr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1RSTR2 register."]
    #[inline] pub fn apb1rstr2_ptr(&self) -> *const Apb1rstr2 { 
        self.apb1rstr2_reg().ptr()
    }

    #[doc="Read the APB1RSTR2 register."]
    #[inline] pub fn apb1rstr2(&self) -> Apb1rstr2 { 
        self.apb1rstr2_reg().read()
    }

    #[doc="Write the APB1RSTR2 register."]
    #[inline] pub fn write_apb1rstr2(&self, value: Apb1rstr2) -> &Self { 
        self.apb1rstr2_reg().write(value);
        self
    }

    #[doc="Set the APB1RSTR2 register."]
    #[inline] pub fn set_apb1rstr2<F: FnOnce(Apb1rstr2) -> Apb1rstr2>(&self, f: F) -> &Self {
        self.apb1rstr2_reg().set(f);
        self
    }

    #[doc="Modify the APB1RSTR2 register."]
    #[inline] pub fn with_apb1rstr2<F: FnOnce(Apb1rstr2) -> Apb1rstr2>(&self, f: F) -> &Self {
        self.apb1rstr2_reg().with(f);
        self
    }

    #[doc="Get the APB2RSTR Register."]
    #[inline] pub fn apb2rstr_reg(&self) -> ::bobbin_mcu::register::Register<Apb2rstr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb2rstr, 0x40)
    }

    #[doc="Get the *mut pointer for the APB2RSTR register."]
    #[inline] pub fn apb2rstr_mut(&self) -> *mut Apb2rstr { 
        self.apb2rstr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB2RSTR register."]
    #[inline] pub fn apb2rstr_ptr(&self) -> *const Apb2rstr { 
        self.apb2rstr_reg().ptr()
    }

    #[doc="Read the APB2RSTR register."]
    #[inline] pub fn apb2rstr(&self) -> Apb2rstr { 
        self.apb2rstr_reg().read()
    }

    #[doc="Write the APB2RSTR register."]
    #[inline] pub fn write_apb2rstr(&self, value: Apb2rstr) -> &Self { 
        self.apb2rstr_reg().write(value);
        self
    }

    #[doc="Set the APB2RSTR register."]
    #[inline] pub fn set_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Self {
        self.apb2rstr_reg().set(f);
        self
    }

    #[doc="Modify the APB2RSTR register."]
    #[inline] pub fn with_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Self {
        self.apb2rstr_reg().with(f);
        self
    }

    #[doc="Get the APB3RSTR Register."]
    #[inline] pub fn apb3rstr_reg(&self) -> ::bobbin_mcu::register::Register<Apb3rstr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb3rstr, 0x44)
    }

    #[doc="Get the *mut pointer for the APB3RSTR register."]
    #[inline] pub fn apb3rstr_mut(&self) -> *mut Apb3rstr { 
        self.apb3rstr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB3RSTR register."]
    #[inline] pub fn apb3rstr_ptr(&self) -> *const Apb3rstr { 
        self.apb3rstr_reg().ptr()
    }

    #[doc="Read the APB3RSTR register."]
    #[inline] pub fn apb3rstr(&self) -> Apb3rstr { 
        self.apb3rstr_reg().read()
    }

    #[doc="Write the APB3RSTR register."]
    #[inline] pub fn write_apb3rstr(&self, value: Apb3rstr) -> &Self { 
        self.apb3rstr_reg().write(value);
        self
    }

    #[doc="Set the APB3RSTR register."]
    #[inline] pub fn set_apb3rstr<F: FnOnce(Apb3rstr) -> Apb3rstr>(&self, f: F) -> &Self {
        self.apb3rstr_reg().set(f);
        self
    }

    #[doc="Modify the APB3RSTR register."]
    #[inline] pub fn with_apb3rstr<F: FnOnce(Apb3rstr) -> Apb3rstr>(&self, f: F) -> &Self {
        self.apb3rstr_reg().with(f);
        self
    }

    #[doc="Get the AHB1ENR Register."]
    #[inline] pub fn ahb1enr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb1enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb1enr, 0x48)
    }

    #[doc="Get the *mut pointer for the AHB1ENR register."]
    #[inline] pub fn ahb1enr_mut(&self) -> *mut Ahb1enr { 
        self.ahb1enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB1ENR register."]
    #[inline] pub fn ahb1enr_ptr(&self) -> *const Ahb1enr { 
        self.ahb1enr_reg().ptr()
    }

    #[doc="Read the AHB1ENR register."]
    #[inline] pub fn ahb1enr(&self) -> Ahb1enr { 
        self.ahb1enr_reg().read()
    }

    #[doc="Write the AHB1ENR register."]
    #[inline] pub fn write_ahb1enr(&self, value: Ahb1enr) -> &Self { 
        self.ahb1enr_reg().write(value);
        self
    }

    #[doc="Set the AHB1ENR register."]
    #[inline] pub fn set_ahb1enr<F: FnOnce(Ahb1enr) -> Ahb1enr>(&self, f: F) -> &Self {
        self.ahb1enr_reg().set(f);
        self
    }

    #[doc="Modify the AHB1ENR register."]
    #[inline] pub fn with_ahb1enr<F: FnOnce(Ahb1enr) -> Ahb1enr>(&self, f: F) -> &Self {
        self.ahb1enr_reg().with(f);
        self
    }

    #[doc="Get the AHB2ENR Register."]
    #[inline] pub fn ahb2enr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb2enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb2enr, 0x4c)
    }

    #[doc="Get the *mut pointer for the AHB2ENR register."]
    #[inline] pub fn ahb2enr_mut(&self) -> *mut Ahb2enr { 
        self.ahb2enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB2ENR register."]
    #[inline] pub fn ahb2enr_ptr(&self) -> *const Ahb2enr { 
        self.ahb2enr_reg().ptr()
    }

    #[doc="Read the AHB2ENR register."]
    #[inline] pub fn ahb2enr(&self) -> Ahb2enr { 
        self.ahb2enr_reg().read()
    }

    #[doc="Write the AHB2ENR register."]
    #[inline] pub fn write_ahb2enr(&self, value: Ahb2enr) -> &Self { 
        self.ahb2enr_reg().write(value);
        self
    }

    #[doc="Set the AHB2ENR register."]
    #[inline] pub fn set_ahb2enr<F: FnOnce(Ahb2enr) -> Ahb2enr>(&self, f: F) -> &Self {
        self.ahb2enr_reg().set(f);
        self
    }

    #[doc="Modify the AHB2ENR register."]
    #[inline] pub fn with_ahb2enr<F: FnOnce(Ahb2enr) -> Ahb2enr>(&self, f: F) -> &Self {
        self.ahb2enr_reg().with(f);
        self
    }

    #[doc="Get the AHB3ENR Register."]
    #[inline] pub fn ahb3enr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb3enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb3enr, 0x50)
    }

    #[doc="Get the *mut pointer for the AHB3ENR register."]
    #[inline] pub fn ahb3enr_mut(&self) -> *mut Ahb3enr { 
        self.ahb3enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB3ENR register."]
    #[inline] pub fn ahb3enr_ptr(&self) -> *const Ahb3enr { 
        self.ahb3enr_reg().ptr()
    }

    #[doc="Read the AHB3ENR register."]
    #[inline] pub fn ahb3enr(&self) -> Ahb3enr { 
        self.ahb3enr_reg().read()
    }

    #[doc="Write the AHB3ENR register."]
    #[inline] pub fn write_ahb3enr(&self, value: Ahb3enr) -> &Self { 
        self.ahb3enr_reg().write(value);
        self
    }

    #[doc="Set the AHB3ENR register."]
    #[inline] pub fn set_ahb3enr<F: FnOnce(Ahb3enr) -> Ahb3enr>(&self, f: F) -> &Self {
        self.ahb3enr_reg().set(f);
        self
    }

    #[doc="Modify the AHB3ENR register."]
    #[inline] pub fn with_ahb3enr<F: FnOnce(Ahb3enr) -> Ahb3enr>(&self, f: F) -> &Self {
        self.ahb3enr_reg().with(f);
        self
    }

    #[doc="Get the APB1ENR1 Register."]
    #[inline] pub fn apb1enr1_reg(&self) -> ::bobbin_mcu::register::Register<Apb1enr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1enr1, 0x58)
    }

    #[doc="Get the *mut pointer for the APB1ENR1 register."]
    #[inline] pub fn apb1enr1_mut(&self) -> *mut Apb1enr1 { 
        self.apb1enr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1ENR1 register."]
    #[inline] pub fn apb1enr1_ptr(&self) -> *const Apb1enr1 { 
        self.apb1enr1_reg().ptr()
    }

    #[doc="Read the APB1ENR1 register."]
    #[inline] pub fn apb1enr1(&self) -> Apb1enr1 { 
        self.apb1enr1_reg().read()
    }

    #[doc="Write the APB1ENR1 register."]
    #[inline] pub fn write_apb1enr1(&self, value: Apb1enr1) -> &Self { 
        self.apb1enr1_reg().write(value);
        self
    }

    #[doc="Set the APB1ENR1 register."]
    #[inline] pub fn set_apb1enr1<F: FnOnce(Apb1enr1) -> Apb1enr1>(&self, f: F) -> &Self {
        self.apb1enr1_reg().set(f);
        self
    }

    #[doc="Modify the APB1ENR1 register."]
    #[inline] pub fn with_apb1enr1<F: FnOnce(Apb1enr1) -> Apb1enr1>(&self, f: F) -> &Self {
        self.apb1enr1_reg().with(f);
        self
    }

    #[doc="Get the APB1ENR2 Register."]
    #[inline] pub fn apb1enr2_reg(&self) -> ::bobbin_mcu::register::Register<Apb1enr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1enr2, 0x5c)
    }

    #[doc="Get the *mut pointer for the APB1ENR2 register."]
    #[inline] pub fn apb1enr2_mut(&self) -> *mut Apb1enr2 { 
        self.apb1enr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1ENR2 register."]
    #[inline] pub fn apb1enr2_ptr(&self) -> *const Apb1enr2 { 
        self.apb1enr2_reg().ptr()
    }

    #[doc="Read the APB1ENR2 register."]
    #[inline] pub fn apb1enr2(&self) -> Apb1enr2 { 
        self.apb1enr2_reg().read()
    }

    #[doc="Write the APB1ENR2 register."]
    #[inline] pub fn write_apb1enr2(&self, value: Apb1enr2) -> &Self { 
        self.apb1enr2_reg().write(value);
        self
    }

    #[doc="Set the APB1ENR2 register."]
    #[inline] pub fn set_apb1enr2<F: FnOnce(Apb1enr2) -> Apb1enr2>(&self, f: F) -> &Self {
        self.apb1enr2_reg().set(f);
        self
    }

    #[doc="Modify the APB1ENR2 register."]
    #[inline] pub fn with_apb1enr2<F: FnOnce(Apb1enr2) -> Apb1enr2>(&self, f: F) -> &Self {
        self.apb1enr2_reg().with(f);
        self
    }

    #[doc="Get the APB2ENR Register."]
    #[inline] pub fn apb2enr_reg(&self) -> ::bobbin_mcu::register::Register<Apb2enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb2enr, 0x60)
    }

    #[doc="Get the *mut pointer for the APB2ENR register."]
    #[inline] pub fn apb2enr_mut(&self) -> *mut Apb2enr { 
        self.apb2enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB2ENR register."]
    #[inline] pub fn apb2enr_ptr(&self) -> *const Apb2enr { 
        self.apb2enr_reg().ptr()
    }

    #[doc="Read the APB2ENR register."]
    #[inline] pub fn apb2enr(&self) -> Apb2enr { 
        self.apb2enr_reg().read()
    }

    #[doc="Write the APB2ENR register."]
    #[inline] pub fn write_apb2enr(&self, value: Apb2enr) -> &Self { 
        self.apb2enr_reg().write(value);
        self
    }

    #[doc="Set the APB2ENR register."]
    #[inline] pub fn set_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Self {
        self.apb2enr_reg().set(f);
        self
    }

    #[doc="Modify the APB2ENR register."]
    #[inline] pub fn with_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Self {
        self.apb2enr_reg().with(f);
        self
    }

    #[doc="Get the AHB1SMENR Register."]
    #[inline] pub fn ahb1smenr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb1smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb1smenr, 0x68)
    }

    #[doc="Get the *mut pointer for the AHB1SMENR register."]
    #[inline] pub fn ahb1smenr_mut(&self) -> *mut Ahb1smenr { 
        self.ahb1smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB1SMENR register."]
    #[inline] pub fn ahb1smenr_ptr(&self) -> *const Ahb1smenr { 
        self.ahb1smenr_reg().ptr()
    }

    #[doc="Read the AHB1SMENR register."]
    #[inline] pub fn ahb1smenr(&self) -> Ahb1smenr { 
        self.ahb1smenr_reg().read()
    }

    #[doc="Write the AHB1SMENR register."]
    #[inline] pub fn write_ahb1smenr(&self, value: Ahb1smenr) -> &Self { 
        self.ahb1smenr_reg().write(value);
        self
    }

    #[doc="Set the AHB1SMENR register."]
    #[inline] pub fn set_ahb1smenr<F: FnOnce(Ahb1smenr) -> Ahb1smenr>(&self, f: F) -> &Self {
        self.ahb1smenr_reg().set(f);
        self
    }

    #[doc="Modify the AHB1SMENR register."]
    #[inline] pub fn with_ahb1smenr<F: FnOnce(Ahb1smenr) -> Ahb1smenr>(&self, f: F) -> &Self {
        self.ahb1smenr_reg().with(f);
        self
    }

    #[doc="Get the AHB2SMENR Register."]
    #[inline] pub fn ahb2smenr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb2smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb2smenr, 0x6c)
    }

    #[doc="Get the *mut pointer for the AHB2SMENR register."]
    #[inline] pub fn ahb2smenr_mut(&self) -> *mut Ahb2smenr { 
        self.ahb2smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB2SMENR register."]
    #[inline] pub fn ahb2smenr_ptr(&self) -> *const Ahb2smenr { 
        self.ahb2smenr_reg().ptr()
    }

    #[doc="Read the AHB2SMENR register."]
    #[inline] pub fn ahb2smenr(&self) -> Ahb2smenr { 
        self.ahb2smenr_reg().read()
    }

    #[doc="Write the AHB2SMENR register."]
    #[inline] pub fn write_ahb2smenr(&self, value: Ahb2smenr) -> &Self { 
        self.ahb2smenr_reg().write(value);
        self
    }

    #[doc="Set the AHB2SMENR register."]
    #[inline] pub fn set_ahb2smenr<F: FnOnce(Ahb2smenr) -> Ahb2smenr>(&self, f: F) -> &Self {
        self.ahb2smenr_reg().set(f);
        self
    }

    #[doc="Modify the AHB2SMENR register."]
    #[inline] pub fn with_ahb2smenr<F: FnOnce(Ahb2smenr) -> Ahb2smenr>(&self, f: F) -> &Self {
        self.ahb2smenr_reg().with(f);
        self
    }

    #[doc="Get the AHB3SMENR Register."]
    #[inline] pub fn ahb3smenr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb3smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb3smenr, 0x70)
    }

    #[doc="Get the *mut pointer for the AHB3SMENR register."]
    #[inline] pub fn ahb3smenr_mut(&self) -> *mut Ahb3smenr { 
        self.ahb3smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB3SMENR register."]
    #[inline] pub fn ahb3smenr_ptr(&self) -> *const Ahb3smenr { 
        self.ahb3smenr_reg().ptr()
    }

    #[doc="Read the AHB3SMENR register."]
    #[inline] pub fn ahb3smenr(&self) -> Ahb3smenr { 
        self.ahb3smenr_reg().read()
    }

    #[doc="Write the AHB3SMENR register."]
    #[inline] pub fn write_ahb3smenr(&self, value: Ahb3smenr) -> &Self { 
        self.ahb3smenr_reg().write(value);
        self
    }

    #[doc="Set the AHB3SMENR register."]
    #[inline] pub fn set_ahb3smenr<F: FnOnce(Ahb3smenr) -> Ahb3smenr>(&self, f: F) -> &Self {
        self.ahb3smenr_reg().set(f);
        self
    }

    #[doc="Modify the AHB3SMENR register."]
    #[inline] pub fn with_ahb3smenr<F: FnOnce(Ahb3smenr) -> Ahb3smenr>(&self, f: F) -> &Self {
        self.ahb3smenr_reg().with(f);
        self
    }

    #[doc="Get the APB1SMENR1 Register."]
    #[inline] pub fn apb1smenr1_reg(&self) -> ::bobbin_mcu::register::Register<Apb1smenr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1smenr1, 0x78)
    }

    #[doc="Get the *mut pointer for the APB1SMENR1 register."]
    #[inline] pub fn apb1smenr1_mut(&self) -> *mut Apb1smenr1 { 
        self.apb1smenr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1SMENR1 register."]
    #[inline] pub fn apb1smenr1_ptr(&self) -> *const Apb1smenr1 { 
        self.apb1smenr1_reg().ptr()
    }

    #[doc="Read the APB1SMENR1 register."]
    #[inline] pub fn apb1smenr1(&self) -> Apb1smenr1 { 
        self.apb1smenr1_reg().read()
    }

    #[doc="Write the APB1SMENR1 register."]
    #[inline] pub fn write_apb1smenr1(&self, value: Apb1smenr1) -> &Self { 
        self.apb1smenr1_reg().write(value);
        self
    }

    #[doc="Set the APB1SMENR1 register."]
    #[inline] pub fn set_apb1smenr1<F: FnOnce(Apb1smenr1) -> Apb1smenr1>(&self, f: F) -> &Self {
        self.apb1smenr1_reg().set(f);
        self
    }

    #[doc="Modify the APB1SMENR1 register."]
    #[inline] pub fn with_apb1smenr1<F: FnOnce(Apb1smenr1) -> Apb1smenr1>(&self, f: F) -> &Self {
        self.apb1smenr1_reg().with(f);
        self
    }

    #[doc="Get the APB1SMENR2 Register."]
    #[inline] pub fn apb1smenr2_reg(&self) -> ::bobbin_mcu::register::Register<Apb1smenr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1smenr2, 0x7c)
    }

    #[doc="Get the *mut pointer for the APB1SMENR2 register."]
    #[inline] pub fn apb1smenr2_mut(&self) -> *mut Apb1smenr2 { 
        self.apb1smenr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1SMENR2 register."]
    #[inline] pub fn apb1smenr2_ptr(&self) -> *const Apb1smenr2 { 
        self.apb1smenr2_reg().ptr()
    }

    #[doc="Read the APB1SMENR2 register."]
    #[inline] pub fn apb1smenr2(&self) -> Apb1smenr2 { 
        self.apb1smenr2_reg().read()
    }

    #[doc="Write the APB1SMENR2 register."]
    #[inline] pub fn write_apb1smenr2(&self, value: Apb1smenr2) -> &Self { 
        self.apb1smenr2_reg().write(value);
        self
    }

    #[doc="Set the APB1SMENR2 register."]
    #[inline] pub fn set_apb1smenr2<F: FnOnce(Apb1smenr2) -> Apb1smenr2>(&self, f: F) -> &Self {
        self.apb1smenr2_reg().set(f);
        self
    }

    #[doc="Modify the APB1SMENR2 register."]
    #[inline] pub fn with_apb1smenr2<F: FnOnce(Apb1smenr2) -> Apb1smenr2>(&self, f: F) -> &Self {
        self.apb1smenr2_reg().with(f);
        self
    }

    #[doc="Get the APB2SMENR Register."]
    #[inline] pub fn apb2smenr_reg(&self) -> ::bobbin_mcu::register::Register<Apb2smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb2smenr, 0x80)
    }

    #[doc="Get the *mut pointer for the APB2SMENR register."]
    #[inline] pub fn apb2smenr_mut(&self) -> *mut Apb2smenr { 
        self.apb2smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB2SMENR register."]
    #[inline] pub fn apb2smenr_ptr(&self) -> *const Apb2smenr { 
        self.apb2smenr_reg().ptr()
    }

    #[doc="Read the APB2SMENR register."]
    #[inline] pub fn apb2smenr(&self) -> Apb2smenr { 
        self.apb2smenr_reg().read()
    }

    #[doc="Write the APB2SMENR register."]
    #[inline] pub fn write_apb2smenr(&self, value: Apb2smenr) -> &Self { 
        self.apb2smenr_reg().write(value);
        self
    }

    #[doc="Set the APB2SMENR register."]
    #[inline] pub fn set_apb2smenr<F: FnOnce(Apb2smenr) -> Apb2smenr>(&self, f: F) -> &Self {
        self.apb2smenr_reg().set(f);
        self
    }

    #[doc="Modify the APB2SMENR register."]
    #[inline] pub fn with_apb2smenr<F: FnOnce(Apb2smenr) -> Apb2smenr>(&self, f: F) -> &Self {
        self.apb2smenr_reg().with(f);
        self
    }

    #[doc="Get the CCIPR Register."]
    #[inline] pub fn ccipr_reg(&self) -> ::bobbin_mcu::register::Register<Ccipr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccipr, 0x88)
    }

    #[doc="Get the *mut pointer for the CCIPR register."]
    #[inline] pub fn ccipr_mut(&self) -> *mut Ccipr { 
        self.ccipr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCIPR register."]
    #[inline] pub fn ccipr_ptr(&self) -> *const Ccipr { 
        self.ccipr_reg().ptr()
    }

    #[doc="Read the CCIPR register."]
    #[inline] pub fn ccipr(&self) -> Ccipr { 
        self.ccipr_reg().read()
    }

    #[doc="Write the CCIPR register."]
    #[inline] pub fn write_ccipr(&self, value: Ccipr) -> &Self { 
        self.ccipr_reg().write(value);
        self
    }

    #[doc="Set the CCIPR register."]
    #[inline] pub fn set_ccipr<F: FnOnce(Ccipr) -> Ccipr>(&self, f: F) -> &Self {
        self.ccipr_reg().set(f);
        self
    }

    #[doc="Modify the CCIPR register."]
    #[inline] pub fn with_ccipr<F: FnOnce(Ccipr) -> Ccipr>(&self, f: F) -> &Self {
        self.ccipr_reg().with(f);
        self
    }

    #[doc="Get the BDCR Register."]
    #[inline] pub fn bdcr_reg(&self) -> ::bobbin_mcu::register::Register<Bdcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdcr, 0x90)
    }

    #[doc="Get the *mut pointer for the BDCR register."]
    #[inline] pub fn bdcr_mut(&self) -> *mut Bdcr { 
        self.bdcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDCR register."]
    #[inline] pub fn bdcr_ptr(&self) -> *const Bdcr { 
        self.bdcr_reg().ptr()
    }

    #[doc="Read the BDCR register."]
    #[inline] pub fn bdcr(&self) -> Bdcr { 
        self.bdcr_reg().read()
    }

    #[doc="Write the BDCR register."]
    #[inline] pub fn write_bdcr(&self, value: Bdcr) -> &Self { 
        self.bdcr_reg().write(value);
        self
    }

    #[doc="Set the BDCR register."]
    #[inline] pub fn set_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&self, f: F) -> &Self {
        self.bdcr_reg().set(f);
        self
    }

    #[doc="Modify the BDCR register."]
    #[inline] pub fn with_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&self, f: F) -> &Self {
        self.bdcr_reg().with(f);
        self
    }

    #[doc="Get the CSR Register."]
    #[inline] pub fn csr_reg(&self) -> ::bobbin_mcu::register::Register<Csr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr, 0x94)
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        self.csr_reg().read()
    }

    #[doc="Write the CSR register."]
    #[inline] pub fn write_csr(&self, value: Csr) -> &Self { 
        self.csr_reg().write(value);
        self
    }

    #[doc="Set the CSR register."]
    #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        self.csr_reg().set(f);
        self
    }

    #[doc="Modify the CSR register."]
    #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        self.csr_reg().with(f);
        self
    }

    #[doc="Get the CRRCR Register."]
    #[inline] pub fn crrcr_reg(&self) -> ::bobbin_mcu::register::Register<Crrcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Crrcr, 0x98)
    }

    #[doc="Get the *mut pointer for the CRRCR register."]
    #[inline] pub fn crrcr_mut(&self) -> *mut Crrcr { 
        self.crrcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CRRCR register."]
    #[inline] pub fn crrcr_ptr(&self) -> *const Crrcr { 
        self.crrcr_reg().ptr()
    }

    #[doc="Read the CRRCR register."]
    #[inline] pub fn crrcr(&self) -> Crrcr { 
        self.crrcr_reg().read()
    }

    #[doc="Write the CRRCR register."]
    #[inline] pub fn write_crrcr(&self, value: Crrcr) -> &Self { 
        self.crrcr_reg().write(value);
        self
    }

    #[doc="Set the CRRCR register."]
    #[inline] pub fn set_crrcr<F: FnOnce(Crrcr) -> Crrcr>(&self, f: F) -> &Self {
        self.crrcr_reg().set(f);
        self
    }

    #[doc="Modify the CRRCR register."]
    #[inline] pub fn with_crrcr<F: FnOnce(Crrcr) -> Crrcr>(&self, f: F) -> &Self {
        self.crrcr_reg().with(f);
        self
    }

    #[doc="Get the HSECR Register."]
    #[inline] pub fn hsecr_reg(&self) -> ::bobbin_mcu::register::Register<Hsecr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hsecr, 0x9c)
    }

    #[doc="Get the *mut pointer for the HSECR register."]
    #[inline] pub fn hsecr_mut(&self) -> *mut Hsecr { 
        self.hsecr_reg().ptr()
    }

    #[doc="Get the *const pointer for the HSECR register."]
    #[inline] pub fn hsecr_ptr(&self) -> *const Hsecr { 
        self.hsecr_reg().ptr()
    }

    #[doc="Read the HSECR register."]
    #[inline] pub fn hsecr(&self) -> Hsecr { 
        self.hsecr_reg().read()
    }

    #[doc="Write the HSECR register."]
    #[inline] pub fn write_hsecr(&self, value: Hsecr) -> &Self { 
        self.hsecr_reg().write(value);
        self
    }

    #[doc="Set the HSECR register."]
    #[inline] pub fn set_hsecr<F: FnOnce(Hsecr) -> Hsecr>(&self, f: F) -> &Self {
        self.hsecr_reg().set(f);
        self
    }

    #[doc="Modify the HSECR register."]
    #[inline] pub fn with_hsecr<F: FnOnce(Hsecr) -> Hsecr>(&self, f: F) -> &Self {
        self.hsecr_reg().with(f);
        self
    }

    #[doc="Get the EXTCFGR Register."]
    #[inline] pub fn extcfgr_reg(&self) -> ::bobbin_mcu::register::Register<Extcfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Extcfgr, 0x108)
    }

    #[doc="Get the *mut pointer for the EXTCFGR register."]
    #[inline] pub fn extcfgr_mut(&self) -> *mut Extcfgr { 
        self.extcfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTCFGR register."]
    #[inline] pub fn extcfgr_ptr(&self) -> *const Extcfgr { 
        self.extcfgr_reg().ptr()
    }

    #[doc="Read the EXTCFGR register."]
    #[inline] pub fn extcfgr(&self) -> Extcfgr { 
        self.extcfgr_reg().read()
    }

    #[doc="Write the EXTCFGR register."]
    #[inline] pub fn write_extcfgr(&self, value: Extcfgr) -> &Self { 
        self.extcfgr_reg().write(value);
        self
    }

    #[doc="Set the EXTCFGR register."]
    #[inline] pub fn set_extcfgr<F: FnOnce(Extcfgr) -> Extcfgr>(&self, f: F) -> &Self {
        self.extcfgr_reg().set(f);
        self
    }

    #[doc="Modify the EXTCFGR register."]
    #[inline] pub fn with_extcfgr<F: FnOnce(Extcfgr) -> Extcfgr>(&self, f: F) -> &Self {
        self.extcfgr_reg().with(f);
        self
    }

    #[doc="Get the C2AHB1ENR Register."]
    #[inline] pub fn c2ahb1enr_reg(&self) -> ::bobbin_mcu::register::Register<C2ahb1enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2ahb1enr, 0x148)
    }

    #[doc="Get the *mut pointer for the C2AHB1ENR register."]
    #[inline] pub fn c2ahb1enr_mut(&self) -> *mut C2ahb1enr { 
        self.c2ahb1enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2AHB1ENR register."]
    #[inline] pub fn c2ahb1enr_ptr(&self) -> *const C2ahb1enr { 
        self.c2ahb1enr_reg().ptr()
    }

    #[doc="Read the C2AHB1ENR register."]
    #[inline] pub fn c2ahb1enr(&self) -> C2ahb1enr { 
        self.c2ahb1enr_reg().read()
    }

    #[doc="Write the C2AHB1ENR register."]
    #[inline] pub fn write_c2ahb1enr(&self, value: C2ahb1enr) -> &Self { 
        self.c2ahb1enr_reg().write(value);
        self
    }

    #[doc="Set the C2AHB1ENR register."]
    #[inline] pub fn set_c2ahb1enr<F: FnOnce(C2ahb1enr) -> C2ahb1enr>(&self, f: F) -> &Self {
        self.c2ahb1enr_reg().set(f);
        self
    }

    #[doc="Modify the C2AHB1ENR register."]
    #[inline] pub fn with_c2ahb1enr<F: FnOnce(C2ahb1enr) -> C2ahb1enr>(&self, f: F) -> &Self {
        self.c2ahb1enr_reg().with(f);
        self
    }

    #[doc="Get the C2AHB2ENR Register."]
    #[inline] pub fn c2ahb2enr_reg(&self) -> ::bobbin_mcu::register::Register<C2ahb2enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2ahb2enr, 0x14c)
    }

    #[doc="Get the *mut pointer for the C2AHB2ENR register."]
    #[inline] pub fn c2ahb2enr_mut(&self) -> *mut C2ahb2enr { 
        self.c2ahb2enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2AHB2ENR register."]
    #[inline] pub fn c2ahb2enr_ptr(&self) -> *const C2ahb2enr { 
        self.c2ahb2enr_reg().ptr()
    }

    #[doc="Read the C2AHB2ENR register."]
    #[inline] pub fn c2ahb2enr(&self) -> C2ahb2enr { 
        self.c2ahb2enr_reg().read()
    }

    #[doc="Write the C2AHB2ENR register."]
    #[inline] pub fn write_c2ahb2enr(&self, value: C2ahb2enr) -> &Self { 
        self.c2ahb2enr_reg().write(value);
        self
    }

    #[doc="Set the C2AHB2ENR register."]
    #[inline] pub fn set_c2ahb2enr<F: FnOnce(C2ahb2enr) -> C2ahb2enr>(&self, f: F) -> &Self {
        self.c2ahb2enr_reg().set(f);
        self
    }

    #[doc="Modify the C2AHB2ENR register."]
    #[inline] pub fn with_c2ahb2enr<F: FnOnce(C2ahb2enr) -> C2ahb2enr>(&self, f: F) -> &Self {
        self.c2ahb2enr_reg().with(f);
        self
    }

    #[doc="Get the C2AHB3ENR Register."]
    #[inline] pub fn c2ahb3enr_reg(&self) -> ::bobbin_mcu::register::Register<C2ahb3enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2ahb3enr, 0x150)
    }

    #[doc="Get the *mut pointer for the C2AHB3ENR register."]
    #[inline] pub fn c2ahb3enr_mut(&self) -> *mut C2ahb3enr { 
        self.c2ahb3enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2AHB3ENR register."]
    #[inline] pub fn c2ahb3enr_ptr(&self) -> *const C2ahb3enr { 
        self.c2ahb3enr_reg().ptr()
    }

    #[doc="Read the C2AHB3ENR register."]
    #[inline] pub fn c2ahb3enr(&self) -> C2ahb3enr { 
        self.c2ahb3enr_reg().read()
    }

    #[doc="Write the C2AHB3ENR register."]
    #[inline] pub fn write_c2ahb3enr(&self, value: C2ahb3enr) -> &Self { 
        self.c2ahb3enr_reg().write(value);
        self
    }

    #[doc="Set the C2AHB3ENR register."]
    #[inline] pub fn set_c2ahb3enr<F: FnOnce(C2ahb3enr) -> C2ahb3enr>(&self, f: F) -> &Self {
        self.c2ahb3enr_reg().set(f);
        self
    }

    #[doc="Modify the C2AHB3ENR register."]
    #[inline] pub fn with_c2ahb3enr<F: FnOnce(C2ahb3enr) -> C2ahb3enr>(&self, f: F) -> &Self {
        self.c2ahb3enr_reg().with(f);
        self
    }

    #[doc="Get the C2APB1ENR1 Register."]
    #[inline] pub fn c2apb1enr1_reg(&self) -> ::bobbin_mcu::register::Register<C2apb1enr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb1enr1, 0x158)
    }

    #[doc="Get the *mut pointer for the C2APB1ENR1 register."]
    #[inline] pub fn c2apb1enr1_mut(&self) -> *mut C2apb1enr1 { 
        self.c2apb1enr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB1ENR1 register."]
    #[inline] pub fn c2apb1enr1_ptr(&self) -> *const C2apb1enr1 { 
        self.c2apb1enr1_reg().ptr()
    }

    #[doc="Read the C2APB1ENR1 register."]
    #[inline] pub fn c2apb1enr1(&self) -> C2apb1enr1 { 
        self.c2apb1enr1_reg().read()
    }

    #[doc="Write the C2APB1ENR1 register."]
    #[inline] pub fn write_c2apb1enr1(&self, value: C2apb1enr1) -> &Self { 
        self.c2apb1enr1_reg().write(value);
        self
    }

    #[doc="Set the C2APB1ENR1 register."]
    #[inline] pub fn set_c2apb1enr1<F: FnOnce(C2apb1enr1) -> C2apb1enr1>(&self, f: F) -> &Self {
        self.c2apb1enr1_reg().set(f);
        self
    }

    #[doc="Modify the C2APB1ENR1 register."]
    #[inline] pub fn with_c2apb1enr1<F: FnOnce(C2apb1enr1) -> C2apb1enr1>(&self, f: F) -> &Self {
        self.c2apb1enr1_reg().with(f);
        self
    }

    #[doc="Get the C2APB1ENR2 Register."]
    #[inline] pub fn c2apb1enr2_reg(&self) -> ::bobbin_mcu::register::Register<C2apb1enr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb1enr2, 0x15c)
    }

    #[doc="Get the *mut pointer for the C2APB1ENR2 register."]
    #[inline] pub fn c2apb1enr2_mut(&self) -> *mut C2apb1enr2 { 
        self.c2apb1enr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB1ENR2 register."]
    #[inline] pub fn c2apb1enr2_ptr(&self) -> *const C2apb1enr2 { 
        self.c2apb1enr2_reg().ptr()
    }

    #[doc="Read the C2APB1ENR2 register."]
    #[inline] pub fn c2apb1enr2(&self) -> C2apb1enr2 { 
        self.c2apb1enr2_reg().read()
    }

    #[doc="Write the C2APB1ENR2 register."]
    #[inline] pub fn write_c2apb1enr2(&self, value: C2apb1enr2) -> &Self { 
        self.c2apb1enr2_reg().write(value);
        self
    }

    #[doc="Set the C2APB1ENR2 register."]
    #[inline] pub fn set_c2apb1enr2<F: FnOnce(C2apb1enr2) -> C2apb1enr2>(&self, f: F) -> &Self {
        self.c2apb1enr2_reg().set(f);
        self
    }

    #[doc="Modify the C2APB1ENR2 register."]
    #[inline] pub fn with_c2apb1enr2<F: FnOnce(C2apb1enr2) -> C2apb1enr2>(&self, f: F) -> &Self {
        self.c2apb1enr2_reg().with(f);
        self
    }

    #[doc="Get the C2APB2ENR Register."]
    #[inline] pub fn c2apb2enr_reg(&self) -> ::bobbin_mcu::register::Register<C2apb2enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb2enr, 0x160)
    }

    #[doc="Get the *mut pointer for the C2APB2ENR register."]
    #[inline] pub fn c2apb2enr_mut(&self) -> *mut C2apb2enr { 
        self.c2apb2enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB2ENR register."]
    #[inline] pub fn c2apb2enr_ptr(&self) -> *const C2apb2enr { 
        self.c2apb2enr_reg().ptr()
    }

    #[doc="Read the C2APB2ENR register."]
    #[inline] pub fn c2apb2enr(&self) -> C2apb2enr { 
        self.c2apb2enr_reg().read()
    }

    #[doc="Write the C2APB2ENR register."]
    #[inline] pub fn write_c2apb2enr(&self, value: C2apb2enr) -> &Self { 
        self.c2apb2enr_reg().write(value);
        self
    }

    #[doc="Set the C2APB2ENR register."]
    #[inline] pub fn set_c2apb2enr<F: FnOnce(C2apb2enr) -> C2apb2enr>(&self, f: F) -> &Self {
        self.c2apb2enr_reg().set(f);
        self
    }

    #[doc="Modify the C2APB2ENR register."]
    #[inline] pub fn with_c2apb2enr<F: FnOnce(C2apb2enr) -> C2apb2enr>(&self, f: F) -> &Self {
        self.c2apb2enr_reg().with(f);
        self
    }

    #[doc="Get the C2APB3ENR Register."]
    #[inline] pub fn c2apb3enr_reg(&self) -> ::bobbin_mcu::register::Register<C2apb3enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb3enr, 0x164)
    }

    #[doc="Get the *mut pointer for the C2APB3ENR register."]
    #[inline] pub fn c2apb3enr_mut(&self) -> *mut C2apb3enr { 
        self.c2apb3enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB3ENR register."]
    #[inline] pub fn c2apb3enr_ptr(&self) -> *const C2apb3enr { 
        self.c2apb3enr_reg().ptr()
    }

    #[doc="Read the C2APB3ENR register."]
    #[inline] pub fn c2apb3enr(&self) -> C2apb3enr { 
        self.c2apb3enr_reg().read()
    }

    #[doc="Write the C2APB3ENR register."]
    #[inline] pub fn write_c2apb3enr(&self, value: C2apb3enr) -> &Self { 
        self.c2apb3enr_reg().write(value);
        self
    }

    #[doc="Set the C2APB3ENR register."]
    #[inline] pub fn set_c2apb3enr<F: FnOnce(C2apb3enr) -> C2apb3enr>(&self, f: F) -> &Self {
        self.c2apb3enr_reg().set(f);
        self
    }

    #[doc="Modify the C2APB3ENR register."]
    #[inline] pub fn with_c2apb3enr<F: FnOnce(C2apb3enr) -> C2apb3enr>(&self, f: F) -> &Self {
        self.c2apb3enr_reg().with(f);
        self
    }

    #[doc="Get the C2AHB1SMENR Register."]
    #[inline] pub fn c2ahb1smenr_reg(&self) -> ::bobbin_mcu::register::Register<C2ahb1smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2ahb1smenr, 0x168)
    }

    #[doc="Get the *mut pointer for the C2AHB1SMENR register."]
    #[inline] pub fn c2ahb1smenr_mut(&self) -> *mut C2ahb1smenr { 
        self.c2ahb1smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2AHB1SMENR register."]
    #[inline] pub fn c2ahb1smenr_ptr(&self) -> *const C2ahb1smenr { 
        self.c2ahb1smenr_reg().ptr()
    }

    #[doc="Read the C2AHB1SMENR register."]
    #[inline] pub fn c2ahb1smenr(&self) -> C2ahb1smenr { 
        self.c2ahb1smenr_reg().read()
    }

    #[doc="Write the C2AHB1SMENR register."]
    #[inline] pub fn write_c2ahb1smenr(&self, value: C2ahb1smenr) -> &Self { 
        self.c2ahb1smenr_reg().write(value);
        self
    }

    #[doc="Set the C2AHB1SMENR register."]
    #[inline] pub fn set_c2ahb1smenr<F: FnOnce(C2ahb1smenr) -> C2ahb1smenr>(&self, f: F) -> &Self {
        self.c2ahb1smenr_reg().set(f);
        self
    }

    #[doc="Modify the C2AHB1SMENR register."]
    #[inline] pub fn with_c2ahb1smenr<F: FnOnce(C2ahb1smenr) -> C2ahb1smenr>(&self, f: F) -> &Self {
        self.c2ahb1smenr_reg().with(f);
        self
    }

    #[doc="Get the C2AHB2SMENR Register."]
    #[inline] pub fn c2ahb2smenr_reg(&self) -> ::bobbin_mcu::register::Register<C2ahb2smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2ahb2smenr, 0x16c)
    }

    #[doc="Get the *mut pointer for the C2AHB2SMENR register."]
    #[inline] pub fn c2ahb2smenr_mut(&self) -> *mut C2ahb2smenr { 
        self.c2ahb2smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2AHB2SMENR register."]
    #[inline] pub fn c2ahb2smenr_ptr(&self) -> *const C2ahb2smenr { 
        self.c2ahb2smenr_reg().ptr()
    }

    #[doc="Read the C2AHB2SMENR register."]
    #[inline] pub fn c2ahb2smenr(&self) -> C2ahb2smenr { 
        self.c2ahb2smenr_reg().read()
    }

    #[doc="Write the C2AHB2SMENR register."]
    #[inline] pub fn write_c2ahb2smenr(&self, value: C2ahb2smenr) -> &Self { 
        self.c2ahb2smenr_reg().write(value);
        self
    }

    #[doc="Set the C2AHB2SMENR register."]
    #[inline] pub fn set_c2ahb2smenr<F: FnOnce(C2ahb2smenr) -> C2ahb2smenr>(&self, f: F) -> &Self {
        self.c2ahb2smenr_reg().set(f);
        self
    }

    #[doc="Modify the C2AHB2SMENR register."]
    #[inline] pub fn with_c2ahb2smenr<F: FnOnce(C2ahb2smenr) -> C2ahb2smenr>(&self, f: F) -> &Self {
        self.c2ahb2smenr_reg().with(f);
        self
    }

    #[doc="Get the C2AHB3SMENR Register."]
    #[inline] pub fn c2ahb3smenr_reg(&self) -> ::bobbin_mcu::register::Register<C2ahb3smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2ahb3smenr, 0x170)
    }

    #[doc="Get the *mut pointer for the C2AHB3SMENR register."]
    #[inline] pub fn c2ahb3smenr_mut(&self) -> *mut C2ahb3smenr { 
        self.c2ahb3smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2AHB3SMENR register."]
    #[inline] pub fn c2ahb3smenr_ptr(&self) -> *const C2ahb3smenr { 
        self.c2ahb3smenr_reg().ptr()
    }

    #[doc="Read the C2AHB3SMENR register."]
    #[inline] pub fn c2ahb3smenr(&self) -> C2ahb3smenr { 
        self.c2ahb3smenr_reg().read()
    }

    #[doc="Write the C2AHB3SMENR register."]
    #[inline] pub fn write_c2ahb3smenr(&self, value: C2ahb3smenr) -> &Self { 
        self.c2ahb3smenr_reg().write(value);
        self
    }

    #[doc="Set the C2AHB3SMENR register."]
    #[inline] pub fn set_c2ahb3smenr<F: FnOnce(C2ahb3smenr) -> C2ahb3smenr>(&self, f: F) -> &Self {
        self.c2ahb3smenr_reg().set(f);
        self
    }

    #[doc="Modify the C2AHB3SMENR register."]
    #[inline] pub fn with_c2ahb3smenr<F: FnOnce(C2ahb3smenr) -> C2ahb3smenr>(&self, f: F) -> &Self {
        self.c2ahb3smenr_reg().with(f);
        self
    }

    #[doc="Get the C2APB1SMENR1 Register."]
    #[inline] pub fn c2apb1smenr1_reg(&self) -> ::bobbin_mcu::register::Register<C2apb1smenr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb1smenr1, 0x178)
    }

    #[doc="Get the *mut pointer for the C2APB1SMENR1 register."]
    #[inline] pub fn c2apb1smenr1_mut(&self) -> *mut C2apb1smenr1 { 
        self.c2apb1smenr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB1SMENR1 register."]
    #[inline] pub fn c2apb1smenr1_ptr(&self) -> *const C2apb1smenr1 { 
        self.c2apb1smenr1_reg().ptr()
    }

    #[doc="Read the C2APB1SMENR1 register."]
    #[inline] pub fn c2apb1smenr1(&self) -> C2apb1smenr1 { 
        self.c2apb1smenr1_reg().read()
    }

    #[doc="Write the C2APB1SMENR1 register."]
    #[inline] pub fn write_c2apb1smenr1(&self, value: C2apb1smenr1) -> &Self { 
        self.c2apb1smenr1_reg().write(value);
        self
    }

    #[doc="Set the C2APB1SMENR1 register."]
    #[inline] pub fn set_c2apb1smenr1<F: FnOnce(C2apb1smenr1) -> C2apb1smenr1>(&self, f: F) -> &Self {
        self.c2apb1smenr1_reg().set(f);
        self
    }

    #[doc="Modify the C2APB1SMENR1 register."]
    #[inline] pub fn with_c2apb1smenr1<F: FnOnce(C2apb1smenr1) -> C2apb1smenr1>(&self, f: F) -> &Self {
        self.c2apb1smenr1_reg().with(f);
        self
    }

    #[doc="Get the C2APB1SMENR2 Register."]
    #[inline] pub fn c2apb1smenr2_reg(&self) -> ::bobbin_mcu::register::Register<C2apb1smenr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb1smenr2, 0x17c)
    }

    #[doc="Get the *mut pointer for the C2APB1SMENR2 register."]
    #[inline] pub fn c2apb1smenr2_mut(&self) -> *mut C2apb1smenr2 { 
        self.c2apb1smenr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB1SMENR2 register."]
    #[inline] pub fn c2apb1smenr2_ptr(&self) -> *const C2apb1smenr2 { 
        self.c2apb1smenr2_reg().ptr()
    }

    #[doc="Read the C2APB1SMENR2 register."]
    #[inline] pub fn c2apb1smenr2(&self) -> C2apb1smenr2 { 
        self.c2apb1smenr2_reg().read()
    }

    #[doc="Write the C2APB1SMENR2 register."]
    #[inline] pub fn write_c2apb1smenr2(&self, value: C2apb1smenr2) -> &Self { 
        self.c2apb1smenr2_reg().write(value);
        self
    }

    #[doc="Set the C2APB1SMENR2 register."]
    #[inline] pub fn set_c2apb1smenr2<F: FnOnce(C2apb1smenr2) -> C2apb1smenr2>(&self, f: F) -> &Self {
        self.c2apb1smenr2_reg().set(f);
        self
    }

    #[doc="Modify the C2APB1SMENR2 register."]
    #[inline] pub fn with_c2apb1smenr2<F: FnOnce(C2apb1smenr2) -> C2apb1smenr2>(&self, f: F) -> &Self {
        self.c2apb1smenr2_reg().with(f);
        self
    }

    #[doc="Get the C2APB2SMENR Register."]
    #[inline] pub fn c2apb2smenr_reg(&self) -> ::bobbin_mcu::register::Register<C2apb2smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb2smenr, 0x180)
    }

    #[doc="Get the *mut pointer for the C2APB2SMENR register."]
    #[inline] pub fn c2apb2smenr_mut(&self) -> *mut C2apb2smenr { 
        self.c2apb2smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB2SMENR register."]
    #[inline] pub fn c2apb2smenr_ptr(&self) -> *const C2apb2smenr { 
        self.c2apb2smenr_reg().ptr()
    }

    #[doc="Read the C2APB2SMENR register."]
    #[inline] pub fn c2apb2smenr(&self) -> C2apb2smenr { 
        self.c2apb2smenr_reg().read()
    }

    #[doc="Write the C2APB2SMENR register."]
    #[inline] pub fn write_c2apb2smenr(&self, value: C2apb2smenr) -> &Self { 
        self.c2apb2smenr_reg().write(value);
        self
    }

    #[doc="Set the C2APB2SMENR register."]
    #[inline] pub fn set_c2apb2smenr<F: FnOnce(C2apb2smenr) -> C2apb2smenr>(&self, f: F) -> &Self {
        self.c2apb2smenr_reg().set(f);
        self
    }

    #[doc="Modify the C2APB2SMENR register."]
    #[inline] pub fn with_c2apb2smenr<F: FnOnce(C2apb2smenr) -> C2apb2smenr>(&self, f: F) -> &Self {
        self.c2apb2smenr_reg().with(f);
        self
    }

    #[doc="Get the C2APB3SMENR Register."]
    #[inline] pub fn c2apb3smenr_reg(&self) -> ::bobbin_mcu::register::Register<C2apb3smenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb3smenr, 0x184)
    }

    #[doc="Get the *mut pointer for the C2APB3SMENR register."]
    #[inline] pub fn c2apb3smenr_mut(&self) -> *mut C2apb3smenr { 
        self.c2apb3smenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB3SMENR register."]
    #[inline] pub fn c2apb3smenr_ptr(&self) -> *const C2apb3smenr { 
        self.c2apb3smenr_reg().ptr()
    }

    #[doc="Read the C2APB3SMENR register."]
    #[inline] pub fn c2apb3smenr(&self) -> C2apb3smenr { 
        self.c2apb3smenr_reg().read()
    }

    #[doc="Write the C2APB3SMENR register."]
    #[inline] pub fn write_c2apb3smenr(&self, value: C2apb3smenr) -> &Self { 
        self.c2apb3smenr_reg().write(value);
        self
    }

    #[doc="Set the C2APB3SMENR register."]
    #[inline] pub fn set_c2apb3smenr<F: FnOnce(C2apb3smenr) -> C2apb3smenr>(&self, f: F) -> &Self {
        self.c2apb3smenr_reg().set(f);
        self
    }

    #[doc="Modify the C2APB3SMENR register."]
    #[inline] pub fn with_c2apb3smenr<F: FnOnce(C2apb3smenr) -> C2apb3smenr>(&self, f: F) -> &Self {
        self.c2apb3smenr_reg().with(f);
        self
    }

}

#[doc="Clock control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="SAI1 PLL clock ready flag"]
    #[inline] pub fn pllsai1rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PLLSAI1RDY != 0"]
    #[inline] pub fn test_pllsai1rdy(&self) -> bool {
        self.pllsai1rdy() != 0
    }

    #[doc="Sets the PLLSAI1RDY field."]
    #[inline] pub fn set_pllsai1rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="SAI1 PLL enable"]
    #[inline] pub fn pllsai1on(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PLLSAI1ON != 0"]
    #[inline] pub fn test_pllsai1on(&self) -> bool {
        self.pllsai1on() != 0
    }

    #[doc="Sets the PLLSAI1ON field."]
    #[inline] pub fn set_pllsai1on<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Main PLL clock ready flag"]
    #[inline] pub fn pllrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PLLRDY != 0"]
    #[inline] pub fn test_pllrdy(&self) -> bool {
        self.pllrdy() != 0
    }

    #[doc="Sets the PLLRDY field."]
    #[inline] pub fn set_pllrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Main PLL enable"]
    #[inline] pub fn pllon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PLLON != 0"]
    #[inline] pub fn test_pllon(&self) -> bool {
        self.pllon() != 0
    }

    #[doc="Sets the PLLON field."]
    #[inline] pub fn set_pllon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="HSE sysclk and PLL M divider prescaler"]
    #[inline] pub fn hsepre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if HSEPRE != 0"]
    #[inline] pub fn test_hsepre(&self) -> bool {
        self.hsepre() != 0
    }

    #[doc="Sets the HSEPRE field."]
    #[inline] pub fn set_hsepre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="HSE Clock security system enable"]
    #[inline] pub fn csson(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CSSON != 0"]
    #[inline] pub fn test_csson(&self) -> bool {
        self.csson() != 0
    }

    #[doc="Sets the CSSON field."]
    #[inline] pub fn set_csson<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="HSE crystal oscillator bypass"]
    #[inline] pub fn hsebyp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if HSEBYP != 0"]
    #[inline] pub fn test_hsebyp(&self) -> bool {
        self.hsebyp() != 0
    }

    #[doc="Sets the HSEBYP field."]
    #[inline] pub fn set_hsebyp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="HSE clock ready flag"]
    #[inline] pub fn hserdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if HSERDY != 0"]
    #[inline] pub fn test_hserdy(&self) -> bool {
        self.hserdy() != 0
    }

    #[doc="Sets the HSERDY field."]
    #[inline] pub fn set_hserdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="HSE clock enabled"]
    #[inline] pub fn hseon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if HSEON != 0"]
    #[inline] pub fn test_hseon(&self) -> bool {
        self.hseon() != 0
    }

    #[doc="Sets the HSEON field."]
    #[inline] pub fn set_hseon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="HSI kernel clock ready flag for peripherals requests"]
    #[inline] pub fn hsikerdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if HSIKERDY != 0"]
    #[inline] pub fn test_hsikerdy(&self) -> bool {
        self.hsikerdy() != 0
    }

    #[doc="Sets the HSIKERDY field."]
    #[inline] pub fn set_hsikerdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="HSI automatic start from Stop"]
    #[inline] pub fn hsiasfs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HSIASFS != 0"]
    #[inline] pub fn test_hsiasfs(&self) -> bool {
        self.hsiasfs() != 0
    }

    #[doc="Sets the HSIASFS field."]
    #[inline] pub fn set_hsiasfs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HSI clock ready flag"]
    #[inline] pub fn hsirdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSIRDY != 0"]
    #[inline] pub fn test_hsirdy(&self) -> bool {
        self.hsirdy() != 0
    }

    #[doc="Sets the HSIRDY field."]
    #[inline] pub fn set_hsirdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="HSI always enable for peripheral kernels"]
    #[inline] pub fn hsikeron(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HSIKERON != 0"]
    #[inline] pub fn test_hsikeron(&self) -> bool {
        self.hsikeron() != 0
    }

    #[doc="Sets the HSIKERON field."]
    #[inline] pub fn set_hsikeron<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HSI clock enabled"]
    #[inline] pub fn hsion(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HSION != 0"]
    #[inline] pub fn test_hsion(&self) -> bool {
        self.hsion() != 0
    }

    #[doc="Sets the HSION field."]
    #[inline] pub fn set_hsion<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MSI clock ranges"]
    #[inline] pub fn msirange(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if MSIRANGE != 0"]
    #[inline] pub fn test_msirange(&self) -> bool {
        self.msirange() != 0
    }

    #[doc="Sets the MSIRANGE field."]
    #[inline] pub fn set_msirange<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MSI clock PLL enable"]
    #[inline] pub fn msipllen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSIPLLEN != 0"]
    #[inline] pub fn test_msipllen(&self) -> bool {
        self.msipllen() != 0
    }

    #[doc="Sets the MSIPLLEN field."]
    #[inline] pub fn set_msipllen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MSI clock ready flag"]
    #[inline] pub fn msirdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MSIRDY != 0"]
    #[inline] pub fn test_msirdy(&self) -> bool {
        self.msirdy() != 0
    }

    #[doc="Sets the MSIRDY field."]
    #[inline] pub fn set_msirdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MSI clock enable"]
    #[inline] pub fn msion(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MSION != 0"]
    #[inline] pub fn test_msion(&self) -> bool {
        self.msion() != 0
    }

    #[doc="Sets the MSION field."]
    #[inline] pub fn set_msion<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.pllsai1rdy() != 0 { try!(write!(f, " pllsai1rdy"))}
        if self.pllsai1on() != 0 { try!(write!(f, " pllsai1on"))}
        if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
        if self.pllon() != 0 { try!(write!(f, " pllon"))}
        if self.hsepre() != 0 { try!(write!(f, " hsepre"))}
        if self.csson() != 0 { try!(write!(f, " csson"))}
        if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
        if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
        if self.hseon() != 0 { try!(write!(f, " hseon"))}
        if self.hsikerdy() != 0 { try!(write!(f, " hsikerdy"))}
        if self.hsiasfs() != 0 { try!(write!(f, " hsiasfs"))}
        if self.hsirdy() != 0 { try!(write!(f, " hsirdy"))}
        if self.hsikeron() != 0 { try!(write!(f, " hsikeron"))}
        if self.hsion() != 0 { try!(write!(f, " hsion"))}
        if self.msirange() != 0 { try!(write!(f, " msirange=0x{:x}", self.msirange()))}
        if self.msipllen() != 0 { try!(write!(f, " msipllen"))}
        if self.msirdy() != 0 { try!(write!(f, " msirdy"))}
        if self.msion() != 0 { try!(write!(f, " msion"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Internal clock sources calibration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icscr(pub u32);
impl Icscr {
    #[doc="HSI clock trimming"]
    #[inline] pub fn hsitrim(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7f) as u8) } // [30:24]
    }

    #[doc="Returns true if HSITRIM != 0"]
    #[inline] pub fn test_hsitrim(&self) -> bool {
        self.hsitrim() != 0
    }

    #[doc="Sets the HSITRIM field."]
    #[inline] pub fn set_hsitrim<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="HSI clock calibration"]
    #[inline] pub fn hsical(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if HSICAL != 0"]
    #[inline] pub fn test_hsical(&self) -> bool {
        self.hsical() != 0
    }

    #[doc="Sets the HSICAL field."]
    #[inline] pub fn set_hsical<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MSI clock trimming"]
    #[inline] pub fn msitrim(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if MSITRIM != 0"]
    #[inline] pub fn test_msitrim(&self) -> bool {
        self.msitrim() != 0
    }

    #[doc="Sets the MSITRIM field."]
    #[inline] pub fn set_msitrim<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MSI clock calibration"]
    #[inline] pub fn msical(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MSICAL != 0"]
    #[inline] pub fn test_msical(&self) -> bool {
        self.msical() != 0
    }

    #[doc="Sets the MSICAL field."]
    #[inline] pub fn set_msical<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icscr {
    #[inline]
    fn from(other: u32) -> Self {
         Icscr(other)
    }
}

impl ::core::fmt::Display for Icscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hsitrim() != 0 { try!(write!(f, " hsitrim=0x{:x}", self.hsitrim()))}
        if self.hsical() != 0 { try!(write!(f, " hsical=0x{:x}", self.hsical()))}
        if self.msitrim() != 0 { try!(write!(f, " msitrim=0x{:x}", self.msitrim()))}
        if self.msical() != 0 { try!(write!(f, " msical=0x{:x}", self.msical()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc="Microcontroller clock output prescaler"]
    #[inline] pub fn mcopre(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if MCOPRE != 0"]
    #[inline] pub fn test_mcopre(&self) -> bool {
        self.mcopre() != 0
    }

    #[doc="Sets the MCOPRE field."]
    #[inline] pub fn set_mcopre<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Microcontroller clock output"]
    #[inline] pub fn mcosel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if MCOSEL != 0"]
    #[inline] pub fn test_mcosel(&self) -> bool {
        self.mcosel() != 0
    }

    #[doc="Sets the MCOSEL field."]
    #[inline] pub fn set_mcosel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="APB2 prescaler flag"]
    #[inline] pub fn ppre2f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PPRE2F != 0"]
    #[inline] pub fn test_ppre2f(&self) -> bool {
        self.ppre2f() != 0
    }

    #[doc="Sets the PPRE2F field."]
    #[inline] pub fn set_ppre2f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="APB1 prescaler flag"]
    #[inline] pub fn ppre1f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PPRE1F != 0"]
    #[inline] pub fn test_ppre1f(&self) -> bool {
        self.ppre1f() != 0
    }

    #[doc="Sets the PPRE1F field."]
    #[inline] pub fn set_ppre1f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="AHB prescaler flag"]
    #[inline] pub fn hpref(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if HPREF != 0"]
    #[inline] pub fn test_hpref(&self) -> bool {
        self.hpref() != 0
    }

    #[doc="Sets the HPREF field."]
    #[inline] pub fn set_hpref<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Wakeup from Stop and CSS backup clock selection"]
    #[inline] pub fn stopwuck(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if STOPWUCK != 0"]
    #[inline] pub fn test_stopwuck(&self) -> bool {
        self.stopwuck() != 0
    }

    #[doc="Sets the STOPWUCK field."]
    #[inline] pub fn set_stopwuck<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="APB high-speed prescaler (APB2)"]
    #[inline] pub fn ppre2(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PPRE2 != 0"]
    #[inline] pub fn test_ppre2(&self) -> bool {
        self.ppre2() != 0
    }

    #[doc="Sets the PPRE2 field."]
    #[inline] pub fn set_ppre2<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PB low-speed prescaler (APB1)"]
    #[inline] pub fn ppre1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PPRE1 != 0"]
    #[inline] pub fn test_ppre1(&self) -> bool {
        self.ppre1() != 0
    }

    #[doc="Sets the PPRE1 field."]
    #[inline] pub fn set_ppre1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AHB prescaler"]
    #[inline] pub fn hpre(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if HPRE != 0"]
    #[inline] pub fn test_hpre(&self) -> bool {
        self.hpre() != 0
    }

    #[doc="Sets the HPRE field."]
    #[inline] pub fn set_hpre<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="System clock switch status"]
    #[inline] pub fn sws(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SWS != 0"]
    #[inline] pub fn test_sws(&self) -> bool {
        self.sws() != 0
    }

    #[doc="Sets the SWS field."]
    #[inline] pub fn set_sws<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="System clock switch"]
    #[inline] pub fn sw(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SW != 0"]
    #[inline] pub fn test_sw(&self) -> bool {
        self.sw() != 0
    }

    #[doc="Sets the SW field."]
    #[inline] pub fn set_sw<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr(other)
    }
}

impl ::core::fmt::Display for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcopre() != 0 { try!(write!(f, " mcopre=0x{:x}", self.mcopre()))}
        if self.mcosel() != 0 { try!(write!(f, " mcosel=0x{:x}", self.mcosel()))}
        if self.ppre2f() != 0 { try!(write!(f, " ppre2f"))}
        if self.ppre1f() != 0 { try!(write!(f, " ppre1f"))}
        if self.hpref() != 0 { try!(write!(f, " hpref"))}
        if self.stopwuck() != 0 { try!(write!(f, " stopwuck"))}
        if self.ppre2() != 0 { try!(write!(f, " ppre2=0x{:x}", self.ppre2()))}
        if self.ppre1() != 0 { try!(write!(f, " ppre1=0x{:x}", self.ppre1()))}
        if self.hpre() != 0 { try!(write!(f, " hpre=0x{:x}", self.hpre()))}
        if self.sws() != 0 { try!(write!(f, " sws=0x{:x}", self.sws()))}
        if self.sw() != 0 { try!(write!(f, " sw=0x{:x}", self.sw()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PLLSYS configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pllcfgr(pub u32);
impl Pllcfgr {
    #[doc="Main PLLSYS division factor R for SYSCLK (system clock)"]
    #[inline] pub fn pllr(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if PLLR != 0"]
    #[inline] pub fn test_pllr(&self) -> bool {
        self.pllr() != 0
    }

    #[doc="Sets the PLLR field."]
    #[inline] pub fn set_pllr<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Main PLLSYSR PLLCLK output enable"]
    #[inline] pub fn pllren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PLLREN != 0"]
    #[inline] pub fn test_pllren(&self) -> bool {
        self.pllren() != 0
    }

    #[doc="Sets the PLLREN field."]
    #[inline] pub fn set_pllren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Main PLLSYS division factor Q for PLLSYSUSBCLK"]
    #[inline] pub fn pllq(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x7) as u8) } // [27:25]
    }

    #[doc="Returns true if PLLQ != 0"]
    #[inline] pub fn test_pllq(&self) -> bool {
        self.pllq() != 0
    }

    #[doc="Sets the PLLQ field."]
    #[inline] pub fn set_pllq<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Main PLLSYSQ output enable"]
    #[inline] pub fn pllqen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PLLQEN != 0"]
    #[inline] pub fn test_pllqen(&self) -> bool {
        self.pllqen() != 0
    }

    #[doc="Sets the PLLQEN field."]
    #[inline] pub fn set_pllqen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Main PLL division factor P for PPLSYSSAICLK"]
    #[inline] pub fn pllp(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if PLLP != 0"]
    #[inline] pub fn test_pllp(&self) -> bool {
        self.pllp() != 0
    }

    #[doc="Sets the PLLP field."]
    #[inline] pub fn set_pllp<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Main PLLSYSP output enable"]
    #[inline] pub fn pllpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PLLPEN != 0"]
    #[inline] pub fn test_pllpen(&self) -> bool {
        self.pllpen() != 0
    }

    #[doc="Sets the PLLPEN field."]
    #[inline] pub fn set_pllpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Main PLLSYS multiplication factor N"]
    #[inline] pub fn plln(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if PLLN != 0"]
    #[inline] pub fn test_plln(&self) -> bool {
        self.plln() != 0
    }

    #[doc="Sets the PLLN field."]
    #[inline] pub fn set_plln<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline] pub fn pllm(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if PLLM != 0"]
    #[inline] pub fn test_pllm(&self) -> bool {
        self.pllm() != 0
    }

    #[doc="Sets the PLLM field."]
    #[inline] pub fn set_pllm<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline] pub fn pllsrc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PLLSRC != 0"]
    #[inline] pub fn test_pllsrc(&self) -> bool {
        self.pllsrc() != 0
    }

    #[doc="Sets the PLLSRC field."]
    #[inline] pub fn set_pllsrc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pllcfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Pllcfgr(other)
    }
}

impl ::core::fmt::Display for Pllcfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pllcfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pllr() != 0 { try!(write!(f, " pllr=0x{:x}", self.pllr()))}
        if self.pllren() != 0 { try!(write!(f, " pllren"))}
        if self.pllq() != 0 { try!(write!(f, " pllq=0x{:x}", self.pllq()))}
        if self.pllqen() != 0 { try!(write!(f, " pllqen"))}
        if self.pllp() != 0 { try!(write!(f, " pllp=0x{:x}", self.pllp()))}
        if self.pllpen() != 0 { try!(write!(f, " pllpen"))}
        if self.plln() != 0 { try!(write!(f, " plln=0x{:x}", self.plln()))}
        if self.pllm() != 0 { try!(write!(f, " pllm=0x{:x}", self.pllm()))}
        if self.pllsrc() != 0 { try!(write!(f, " pllsrc=0x{:x}", self.pllsrc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PLLSAI1 configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pllsai1cfgr(pub u32);
impl Pllsai1cfgr {
    #[doc="PLLSAI division factor R for PLLADC1CLK (ADC clock)"]
    #[inline] pub fn pllr(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if PLLR != 0"]
    #[inline] pub fn test_pllr(&self) -> bool {
        self.pllr() != 0
    }

    #[doc="Sets the PLLR field."]
    #[inline] pub fn set_pllr<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="PLLSAI PLLADC1CLK output enable"]
    #[inline] pub fn pllren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PLLREN != 0"]
    #[inline] pub fn test_pllren(&self) -> bool {
        self.pllren() != 0
    }

    #[doc="Sets the PLLREN field."]
    #[inline] pub fn set_pllren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)"]
    #[inline] pub fn pllq(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x7) as u8) } // [27:25]
    }

    #[doc="Returns true if PLLQ != 0"]
    #[inline] pub fn test_pllq(&self) -> bool {
        self.pllq() != 0
    }

    #[doc="Sets the PLLQ field."]
    #[inline] pub fn set_pllq<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="SAIPLL PLLSAIUSBCLK output enable"]
    #[inline] pub fn pllqen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PLLQEN != 0"]
    #[inline] pub fn test_pllqen(&self) -> bool {
        self.pllqen() != 0
    }

    #[doc="Sets the PLLQEN field."]
    #[inline] pub fn set_pllqen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SAI1PLL division factor P for PLLSAICLK (SAI1clock)"]
    #[inline] pub fn pllp(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if PLLP != 0"]
    #[inline] pub fn test_pllp(&self) -> bool {
        self.pllp() != 0
    }

    #[doc="Sets the PLLP field."]
    #[inline] pub fn set_pllp<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SAIPLL PLLSAI1CLK output enable"]
    #[inline] pub fn pllpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PLLPEN != 0"]
    #[inline] pub fn test_pllpen(&self) -> bool {
        self.pllpen() != 0
    }

    #[doc="Sets the PLLPEN field."]
    #[inline] pub fn set_pllpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SAIPLL multiplication factor for VCO"]
    #[inline] pub fn plln(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if PLLN != 0"]
    #[inline] pub fn test_plln(&self) -> bool {
        self.plln() != 0
    }

    #[doc="Sets the PLLN field."]
    #[inline] pub fn set_plln<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Pllsai1cfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Pllsai1cfgr(other)
    }
}

impl ::core::fmt::Display for Pllsai1cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pllsai1cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pllr() != 0 { try!(write!(f, " pllr=0x{:x}", self.pllr()))}
        if self.pllren() != 0 { try!(write!(f, " pllren"))}
        if self.pllq() != 0 { try!(write!(f, " pllq=0x{:x}", self.pllq()))}
        if self.pllqen() != 0 { try!(write!(f, " pllqen"))}
        if self.pllp() != 0 { try!(write!(f, " pllp=0x{:x}", self.pllp()))}
        if self.pllpen() != 0 { try!(write!(f, " pllpen"))}
        if self.plln() != 0 { try!(write!(f, " plln=0x{:x}", self.plln()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cier(pub u32);
impl Cier {
    #[doc="LSI2 ready interrupt enable"]
    #[inline] pub fn lsi2rdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LSI2RDYIE != 0"]
    #[inline] pub fn test_lsi2rdyie(&self) -> bool {
        self.lsi2rdyie() != 0
    }

    #[doc="Sets the LSI2RDYIE field."]
    #[inline] pub fn set_lsi2rdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HSI48 ready interrupt enable"]
    #[inline] pub fn hsi48rdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSI48RDYIE != 0"]
    #[inline] pub fn test_hsi48rdyie(&self) -> bool {
        self.hsi48rdyie() != 0
    }

    #[doc="Sets the HSI48RDYIE field."]
    #[inline] pub fn set_hsi48rdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="LSE clock security system interrupt enable"]
    #[inline] pub fn lsecssie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSECSSIE != 0"]
    #[inline] pub fn test_lsecssie(&self) -> bool {
        self.lsecssie() != 0
    }

    #[doc="Sets the LSECSSIE field."]
    #[inline] pub fn set_lsecssie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="PLLSAI1 ready interrupt enable"]
    #[inline] pub fn pllsai1rdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLSAI1RDYIE != 0"]
    #[inline] pub fn test_pllsai1rdyie(&self) -> bool {
        self.pllsai1rdyie() != 0
    }

    #[doc="Sets the PLLSAI1RDYIE field."]
    #[inline] pub fn set_pllsai1rdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PLLSYS ready interrupt enable"]
    #[inline] pub fn pllrdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLRDYIE != 0"]
    #[inline] pub fn test_pllrdyie(&self) -> bool {
        self.pllrdyie() != 0
    }

    #[doc="Sets the PLLRDYIE field."]
    #[inline] pub fn set_pllrdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HSE ready interrupt enable"]
    #[inline] pub fn hserdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HSERDYIE != 0"]
    #[inline] pub fn test_hserdyie(&self) -> bool {
        self.hserdyie() != 0
    }

    #[doc="Sets the HSERDYIE field."]
    #[inline] pub fn set_hserdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSI ready interrupt enable"]
    #[inline] pub fn hsirdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSIRDYIE != 0"]
    #[inline] pub fn test_hsirdyie(&self) -> bool {
        self.hsirdyie() != 0
    }

    #[doc="Sets the HSIRDYIE field."]
    #[inline] pub fn set_hsirdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MSI ready interrupt enable"]
    #[inline] pub fn msirdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSIRDYIE != 0"]
    #[inline] pub fn test_msirdyie(&self) -> bool {
        self.msirdyie() != 0
    }

    #[doc="Sets the MSIRDYIE field."]
    #[inline] pub fn set_msirdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE ready interrupt enable"]
    #[inline] pub fn lserdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDYIE != 0"]
    #[inline] pub fn test_lserdyie(&self) -> bool {
        self.lserdyie() != 0
    }

    #[doc="Sets the LSERDYIE field."]
    #[inline] pub fn set_lserdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSI1 ready interrupt enable"]
    #[inline] pub fn lsi1rdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSI1RDYIE != 0"]
    #[inline] pub fn test_lsi1rdyie(&self) -> bool {
        self.lsi1rdyie() != 0
    }

    #[doc="Sets the LSI1RDYIE field."]
    #[inline] pub fn set_lsi1rdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cier {
    #[inline]
    fn from(other: u32) -> Self {
         Cier(other)
    }
}

impl ::core::fmt::Display for Cier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lsi2rdyie() != 0 { try!(write!(f, " lsi2rdyie"))}
        if self.hsi48rdyie() != 0 { try!(write!(f, " hsi48rdyie"))}
        if self.lsecssie() != 0 { try!(write!(f, " lsecssie"))}
        if self.pllsai1rdyie() != 0 { try!(write!(f, " pllsai1rdyie"))}
        if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
        if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
        if self.hsirdyie() != 0 { try!(write!(f, " hsirdyie"))}
        if self.msirdyie() != 0 { try!(write!(f, " msirdyie"))}
        if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
        if self.lsi1rdyie() != 0 { try!(write!(f, " lsi1rdyie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock interrupt flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cifr(pub u32);
impl Cifr {
    #[doc="LSI2 ready interrupt flag"]
    #[inline] pub fn lsi2rdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LSI2RDYF != 0"]
    #[inline] pub fn test_lsi2rdyf(&self) -> bool {
        self.lsi2rdyf() != 0
    }

    #[doc="Sets the LSI2RDYF field."]
    #[inline] pub fn set_lsi2rdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HSI48 ready interrupt flag"]
    #[inline] pub fn hsi48rdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSI48RDYF != 0"]
    #[inline] pub fn test_hsi48rdyf(&self) -> bool {
        self.hsi48rdyf() != 0
    }

    #[doc="Sets the HSI48RDYF field."]
    #[inline] pub fn set_hsi48rdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="LSE Clock security system interrupt flag"]
    #[inline] pub fn lsecssf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSECSSF != 0"]
    #[inline] pub fn test_lsecssf(&self) -> bool {
        self.lsecssf() != 0
    }

    #[doc="Sets the LSECSSF field."]
    #[inline] pub fn set_lsecssf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HSE Clock security system interrupt flag"]
    #[inline] pub fn hsecssf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HSECSSF != 0"]
    #[inline] pub fn test_hsecssf(&self) -> bool {
        self.hsecssf() != 0
    }

    #[doc="Sets the HSECSSF field."]
    #[inline] pub fn set_hsecssf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PLLSAI1 ready interrupt flag"]
    #[inline] pub fn pllsai1rdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLSAI1RDYF != 0"]
    #[inline] pub fn test_pllsai1rdyf(&self) -> bool {
        self.pllsai1rdyf() != 0
    }

    #[doc="Sets the PLLSAI1RDYF field."]
    #[inline] pub fn set_pllsai1rdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PLL ready interrupt flag"]
    #[inline] pub fn pllrdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLRDYF != 0"]
    #[inline] pub fn test_pllrdyf(&self) -> bool {
        self.pllrdyf() != 0
    }

    #[doc="Sets the PLLRDYF field."]
    #[inline] pub fn set_pllrdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HSE ready interrupt flag"]
    #[inline] pub fn hserdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HSERDYF != 0"]
    #[inline] pub fn test_hserdyf(&self) -> bool {
        self.hserdyf() != 0
    }

    #[doc="Sets the HSERDYF field."]
    #[inline] pub fn set_hserdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSI ready interrupt flag"]
    #[inline] pub fn hsirdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSIRDYF != 0"]
    #[inline] pub fn test_hsirdyf(&self) -> bool {
        self.hsirdyf() != 0
    }

    #[doc="Sets the HSIRDYF field."]
    #[inline] pub fn set_hsirdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MSI ready interrupt flag"]
    #[inline] pub fn msirdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSIRDYF != 0"]
    #[inline] pub fn test_msirdyf(&self) -> bool {
        self.msirdyf() != 0
    }

    #[doc="Sets the MSIRDYF field."]
    #[inline] pub fn set_msirdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE ready interrupt flag"]
    #[inline] pub fn lserdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDYF != 0"]
    #[inline] pub fn test_lserdyf(&self) -> bool {
        self.lserdyf() != 0
    }

    #[doc="Sets the LSERDYF field."]
    #[inline] pub fn set_lserdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSI1 ready interrupt flag"]
    #[inline] pub fn lsi1rdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSI1RDYF != 0"]
    #[inline] pub fn test_lsi1rdyf(&self) -> bool {
        self.lsi1rdyf() != 0
    }

    #[doc="Sets the LSI1RDYF field."]
    #[inline] pub fn set_lsi1rdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cifr {
    #[inline]
    fn from(other: u32) -> Self {
         Cifr(other)
    }
}

impl ::core::fmt::Display for Cifr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cifr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lsi2rdyf() != 0 { try!(write!(f, " lsi2rdyf"))}
        if self.hsi48rdyf() != 0 { try!(write!(f, " hsi48rdyf"))}
        if self.lsecssf() != 0 { try!(write!(f, " lsecssf"))}
        if self.hsecssf() != 0 { try!(write!(f, " hsecssf"))}
        if self.pllsai1rdyf() != 0 { try!(write!(f, " pllsai1rdyf"))}
        if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
        if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
        if self.hsirdyf() != 0 { try!(write!(f, " hsirdyf"))}
        if self.msirdyf() != 0 { try!(write!(f, " msirdyf"))}
        if self.lserdyf() != 0 { try!(write!(f, " lserdyf"))}
        if self.lsi1rdyf() != 0 { try!(write!(f, " lsi1rdyf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cicr(pub u32);
impl Cicr {
    #[doc="LSI2 ready interrupt clear"]
    #[inline] pub fn lsi2rdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LSI2RDYC != 0"]
    #[inline] pub fn test_lsi2rdyc(&self) -> bool {
        self.lsi2rdyc() != 0
    }

    #[doc="Sets the LSI2RDYC field."]
    #[inline] pub fn set_lsi2rdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HSI48 ready interrupt clear"]
    #[inline] pub fn hsi48rdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSI48RDYC != 0"]
    #[inline] pub fn test_hsi48rdyc(&self) -> bool {
        self.hsi48rdyc() != 0
    }

    #[doc="Sets the HSI48RDYC field."]
    #[inline] pub fn set_hsi48rdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="LSE Clock security system interrupt clear"]
    #[inline] pub fn lsecssc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSECSSC != 0"]
    #[inline] pub fn test_lsecssc(&self) -> bool {
        self.lsecssc() != 0
    }

    #[doc="Sets the LSECSSC field."]
    #[inline] pub fn set_lsecssc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HSE Clock security system interrupt clear"]
    #[inline] pub fn hsecssc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HSECSSC != 0"]
    #[inline] pub fn test_hsecssc(&self) -> bool {
        self.hsecssc() != 0
    }

    #[doc="Sets the HSECSSC field."]
    #[inline] pub fn set_hsecssc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PLLSAI1 ready interrupt clear"]
    #[inline] pub fn pllsai1rdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLSAI1RDYC != 0"]
    #[inline] pub fn test_pllsai1rdyc(&self) -> bool {
        self.pllsai1rdyc() != 0
    }

    #[doc="Sets the PLLSAI1RDYC field."]
    #[inline] pub fn set_pllsai1rdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PLL ready interrupt clear"]
    #[inline] pub fn pllrdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLRDYC != 0"]
    #[inline] pub fn test_pllrdyc(&self) -> bool {
        self.pllrdyc() != 0
    }

    #[doc="Sets the PLLRDYC field."]
    #[inline] pub fn set_pllrdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HSE ready interrupt clear"]
    #[inline] pub fn hserdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HSERDYC != 0"]
    #[inline] pub fn test_hserdyc(&self) -> bool {
        self.hserdyc() != 0
    }

    #[doc="Sets the HSERDYC field."]
    #[inline] pub fn set_hserdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSI ready interrupt clear"]
    #[inline] pub fn hsirdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSIRDYC != 0"]
    #[inline] pub fn test_hsirdyc(&self) -> bool {
        self.hsirdyc() != 0
    }

    #[doc="Sets the HSIRDYC field."]
    #[inline] pub fn set_hsirdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MSI ready interrupt clear"]
    #[inline] pub fn msirdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSIRDYC != 0"]
    #[inline] pub fn test_msirdyc(&self) -> bool {
        self.msirdyc() != 0
    }

    #[doc="Sets the MSIRDYC field."]
    #[inline] pub fn set_msirdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE ready interrupt clear"]
    #[inline] pub fn lserdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDYC != 0"]
    #[inline] pub fn test_lserdyc(&self) -> bool {
        self.lserdyc() != 0
    }

    #[doc="Sets the LSERDYC field."]
    #[inline] pub fn set_lserdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSI1 ready interrupt clear"]
    #[inline] pub fn lsi1rdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSI1RDYC != 0"]
    #[inline] pub fn test_lsi1rdyc(&self) -> bool {
        self.lsi1rdyc() != 0
    }

    #[doc="Sets the LSI1RDYC field."]
    #[inline] pub fn set_lsi1rdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cicr {
    #[inline]
    fn from(other: u32) -> Self {
         Cicr(other)
    }
}

impl ::core::fmt::Display for Cicr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cicr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lsi2rdyc() != 0 { try!(write!(f, " lsi2rdyc"))}
        if self.hsi48rdyc() != 0 { try!(write!(f, " hsi48rdyc"))}
        if self.lsecssc() != 0 { try!(write!(f, " lsecssc"))}
        if self.hsecssc() != 0 { try!(write!(f, " hsecssc"))}
        if self.pllsai1rdyc() != 0 { try!(write!(f, " pllsai1rdyc"))}
        if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
        if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
        if self.hsirdyc() != 0 { try!(write!(f, " hsirdyc"))}
        if self.msirdyc() != 0 { try!(write!(f, " msirdyc"))}
        if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
        if self.lsi1rdyc() != 0 { try!(write!(f, " lsi1rdyc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Step Down converter control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smpscr(pub u32);
impl Smpscr {
    #[doc="Step Down converter clock switch status"]
    #[inline] pub fn smpssws(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if SMPSSWS != 0"]
    #[inline] pub fn test_smpssws(&self) -> bool {
        self.smpssws() != 0
    }

    #[doc="Sets the SMPSSWS field."]
    #[inline] pub fn set_smpssws<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Step Down converter clock prescaler"]
    #[inline] pub fn smpsdiv(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if SMPSDIV != 0"]
    #[inline] pub fn test_smpsdiv(&self) -> bool {
        self.smpsdiv() != 0
    }

    #[doc="Sets the SMPSDIV field."]
    #[inline] pub fn set_smpsdiv<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Step Down converter clock selection"]
    #[inline] pub fn smpssel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SMPSSEL != 0"]
    #[inline] pub fn test_smpssel(&self) -> bool {
        self.smpssel() != 0
    }

    #[doc="Sets the SMPSSEL field."]
    #[inline] pub fn set_smpssel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Smpscr {
    #[inline]
    fn from(other: u32) -> Self {
         Smpscr(other)
    }
}

impl ::core::fmt::Display for Smpscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Smpscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.smpssws() != 0 { try!(write!(f, " smpssws=0x{:x}", self.smpssws()))}
        if self.smpsdiv() != 0 { try!(write!(f, " smpsdiv=0x{:x}", self.smpsdiv()))}
        if self.smpssel() != 0 { try!(write!(f, " smpssel=0x{:x}", self.smpssel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1rstr(pub u32);
impl Ahb1rstr {
    #[doc="Touch Sensing Controller reset"]
    #[inline] pub fn tscrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSCRST != 0"]
    #[inline] pub fn test_tscrst(&self) -> bool {
        self.tscrst() != 0
    }

    #[doc="Sets the TSCRST field."]
    #[inline] pub fn set_tscrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CRC reset"]
    #[inline] pub fn crcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCRST != 0"]
    #[inline] pub fn test_crcrst(&self) -> bool {
        self.crcrst() != 0
    }

    #[doc="Sets the CRCRST field."]
    #[inline] pub fn set_crcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DMAMUX reset"]
    #[inline] pub fn dmamuxrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAMUXRST != 0"]
    #[inline] pub fn test_dmamuxrst(&self) -> bool {
        self.dmamuxrst() != 0
    }

    #[doc="Sets the DMAMUXRST field."]
    #[inline] pub fn set_dmamuxrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA2 reset"]
    #[inline] pub fn dma2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2RST != 0"]
    #[inline] pub fn test_dma2rst(&self) -> bool {
        self.dma2rst() != 0
    }

    #[doc="Sets the DMA2RST field."]
    #[inline] pub fn set_dma2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMA1 reset"]
    #[inline] pub fn dma1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1RST != 0"]
    #[inline] pub fn test_dma1rst(&self) -> bool {
        self.dma1rst() != 0
    }

    #[doc="Sets the DMA1RST field."]
    #[inline] pub fn set_dma1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb1rstr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb1rstr(other)
    }
}

impl ::core::fmt::Display for Ahb1rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb1rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tscrst() != 0 { try!(write!(f, " tscrst"))}
        if self.crcrst() != 0 { try!(write!(f, " crcrst"))}
        if self.dmamuxrst() != 0 { try!(write!(f, " dmamuxrst"))}
        if self.dma2rst() != 0 { try!(write!(f, " dma2rst"))}
        if self.dma1rst() != 0 { try!(write!(f, " dma1rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB2 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb2rstr(pub u32);
impl Ahb2rstr {
    #[doc="AES1 hardware accelerator reset"]
    #[inline] pub fn aes1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AES1RST != 0"]
    #[inline] pub fn test_aes1rst(&self) -> bool {
        self.aes1rst() != 0
    }

    #[doc="Sets the AES1RST field."]
    #[inline] pub fn set_aes1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ADC reset"]
    #[inline] pub fn adcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCRST != 0"]
    #[inline] pub fn test_adcrst(&self) -> bool {
        self.adcrst() != 0
    }

    #[doc="Sets the ADCRST field."]
    #[inline] pub fn set_adcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="IO port H reset"]
    #[inline] pub fn gpiohrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHRST != 0"]
    #[inline] pub fn test_gpiohrst(&self) -> bool {
        self.gpiohrst() != 0
    }

    #[doc="Sets the GPIOHRST field."]
    #[inline] pub fn set_gpiohrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IO port E reset"]
    #[inline] pub fn gpioerst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOERST != 0"]
    #[inline] pub fn test_gpioerst(&self) -> bool {
        self.gpioerst() != 0
    }

    #[doc="Sets the GPIOERST field."]
    #[inline] pub fn set_gpioerst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IO port D reset"]
    #[inline] pub fn gpiodrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODRST != 0"]
    #[inline] pub fn test_gpiodrst(&self) -> bool {
        self.gpiodrst() != 0
    }

    #[doc="Sets the GPIODRST field."]
    #[inline] pub fn set_gpiodrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IO port C reset"]
    #[inline] pub fn gpiocrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCRST != 0"]
    #[inline] pub fn test_gpiocrst(&self) -> bool {
        self.gpiocrst() != 0
    }

    #[doc="Sets the GPIOCRST field."]
    #[inline] pub fn set_gpiocrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IO port B reset"]
    #[inline] pub fn gpiobrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBRST != 0"]
    #[inline] pub fn test_gpiobrst(&self) -> bool {
        self.gpiobrst() != 0
    }

    #[doc="Sets the GPIOBRST field."]
    #[inline] pub fn set_gpiobrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IO port A reset"]
    #[inline] pub fn gpioarst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOARST != 0"]
    #[inline] pub fn test_gpioarst(&self) -> bool {
        self.gpioarst() != 0
    }

    #[doc="Sets the GPIOARST field."]
    #[inline] pub fn set_gpioarst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb2rstr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb2rstr(other)
    }
}

impl ::core::fmt::Display for Ahb2rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb2rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aes1rst() != 0 { try!(write!(f, " aes1rst"))}
        if self.adcrst() != 0 { try!(write!(f, " adcrst"))}
        if self.gpiohrst() != 0 { try!(write!(f, " gpiohrst"))}
        if self.gpioerst() != 0 { try!(write!(f, " gpioerst"))}
        if self.gpiodrst() != 0 { try!(write!(f, " gpiodrst"))}
        if self.gpiocrst() != 0 { try!(write!(f, " gpiocrst"))}
        if self.gpiobrst() != 0 { try!(write!(f, " gpiobrst"))}
        if self.gpioarst() != 0 { try!(write!(f, " gpioarst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB3 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb3rstr(pub u32);
impl Ahb3rstr {
    #[doc="Flash interface reset"]
    #[inline] pub fn flashrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FLASHRST != 0"]
    #[inline] pub fn test_flashrst(&self) -> bool {
        self.flashrst() != 0
    }

    #[doc="Sets the FLASHRST field."]
    #[inline] pub fn set_flashrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="IPCC interface reset"]
    #[inline] pub fn ipccrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IPCCRST != 0"]
    #[inline] pub fn test_ipccrst(&self) -> bool {
        self.ipccrst() != 0
    }

    #[doc="Sets the IPCCRST field."]
    #[inline] pub fn set_ipccrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="HSEM interface reset"]
    #[inline] pub fn hsemrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HSEMRST != 0"]
    #[inline] pub fn test_hsemrst(&self) -> bool {
        self.hsemrst() != 0
    }

    #[doc="Sets the HSEMRST field."]
    #[inline] pub fn set_hsemrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="RNG interface reset"]
    #[inline] pub fn rngrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RNGRST != 0"]
    #[inline] pub fn test_rngrst(&self) -> bool {
        self.rngrst() != 0
    }

    #[doc="Sets the RNGRST field."]
    #[inline] pub fn set_rngrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="AES2 interface reset"]
    #[inline] pub fn aes2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if AES2RST != 0"]
    #[inline] pub fn test_aes2rst(&self) -> bool {
        self.aes2rst() != 0
    }

    #[doc="Sets the AES2RST field."]
    #[inline] pub fn set_aes2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PKA interface reset"]
    #[inline] pub fn pkarst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PKARST != 0"]
    #[inline] pub fn test_pkarst(&self) -> bool {
        self.pkarst() != 0
    }

    #[doc="Sets the PKARST field."]
    #[inline] pub fn set_pkarst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Quad SPI memory interface reset"]
    #[inline] pub fn qspirst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if QSPIRST != 0"]
    #[inline] pub fn test_qspirst(&self) -> bool {
        self.qspirst() != 0
    }

    #[doc="Sets the QSPIRST field."]
    #[inline] pub fn set_qspirst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Ahb3rstr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb3rstr(other)
    }
}

impl ::core::fmt::Display for Ahb3rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb3rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flashrst() != 0 { try!(write!(f, " flashrst"))}
        if self.ipccrst() != 0 { try!(write!(f, " ipccrst"))}
        if self.hsemrst() != 0 { try!(write!(f, " hsemrst"))}
        if self.rngrst() != 0 { try!(write!(f, " rngrst"))}
        if self.aes2rst() != 0 { try!(write!(f, " aes2rst"))}
        if self.pkarst() != 0 { try!(write!(f, " pkarst"))}
        if self.qspirst() != 0 { try!(write!(f, " qspirst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral reset register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1rstr1(pub u32);
impl Apb1rstr1 {
    #[doc="Low Power Timer 1 reset"]
    #[inline] pub fn lptim1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPTIM1RST != 0"]
    #[inline] pub fn test_lptim1rst(&self) -> bool {
        self.lptim1rst() != 0
    }

    #[doc="Sets the LPTIM1RST field."]
    #[inline] pub fn set_lptim1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="USB FS reset"]
    #[inline] pub fn usbfsrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if USBFSRST != 0"]
    #[inline] pub fn test_usbfsrst(&self) -> bool {
        self.usbfsrst() != 0
    }

    #[doc="Sets the USBFSRST field."]
    #[inline] pub fn set_usbfsrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CRS reset"]
    #[inline] pub fn crsrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CRSRST != 0"]
    #[inline] pub fn test_crsrst(&self) -> bool {
        self.crsrst() != 0
    }

    #[doc="Sets the CRSRST field."]
    #[inline] pub fn set_crsrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="I2C3 reset"]
    #[inline] pub fn i2c3rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3RST != 0"]
    #[inline] pub fn test_i2c3rst(&self) -> bool {
        self.i2c3rst() != 0
    }

    #[doc="Sets the I2C3RST field."]
    #[inline] pub fn set_i2c3rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C1 reset"]
    #[inline] pub fn i2c1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1RST != 0"]
    #[inline] pub fn test_i2c1rst(&self) -> bool {
        self.i2c1rst() != 0
    }

    #[doc="Sets the I2C1RST field."]
    #[inline] pub fn set_i2c1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SPI2 reset"]
    #[inline] pub fn spi2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2RST != 0"]
    #[inline] pub fn test_spi2rst(&self) -> bool {
        self.spi2rst() != 0
    }

    #[doc="Sets the SPI2RST field."]
    #[inline] pub fn set_spi2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="LCD interface reset"]
    #[inline] pub fn lcdrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LCDRST != 0"]
    #[inline] pub fn test_lcdrst(&self) -> bool {
        self.lcdrst() != 0
    }

    #[doc="Sets the LCDRST field."]
    #[inline] pub fn set_lcdrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TIM2 timer reset"]
    #[inline] pub fn tim2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2RST != 0"]
    #[inline] pub fn test_tim2rst(&self) -> bool {
        self.tim2rst() != 0
    }

    #[doc="Sets the TIM2RST field."]
    #[inline] pub fn set_tim2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb1rstr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1rstr1(other)
    }
}

impl ::core::fmt::Display for Apb1rstr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1rstr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim1rst() != 0 { try!(write!(f, " lptim1rst"))}
        if self.usbfsrst() != 0 { try!(write!(f, " usbfsrst"))}
        if self.crsrst() != 0 { try!(write!(f, " crsrst"))}
        if self.i2c3rst() != 0 { try!(write!(f, " i2c3rst"))}
        if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
        if self.spi2rst() != 0 { try!(write!(f, " spi2rst"))}
        if self.lcdrst() != 0 { try!(write!(f, " lcdrst"))}
        if self.tim2rst() != 0 { try!(write!(f, " tim2rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral reset register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1rstr2(pub u32);
impl Apb1rstr2 {
    #[doc="Low-power timer 2 reset"]
    #[inline] pub fn lptim2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LPTIM2RST != 0"]
    #[inline] pub fn test_lptim2rst(&self) -> bool {
        self.lptim2rst() != 0
    }

    #[doc="Sets the LPTIM2RST field."]
    #[inline] pub fn set_lptim2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Low-power UART 1 reset"]
    #[inline] pub fn lpuart1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPUART1RST != 0"]
    #[inline] pub fn test_lpuart1rst(&self) -> bool {
        self.lpuart1rst() != 0
    }

    #[doc="Sets the LPUART1RST field."]
    #[inline] pub fn set_lpuart1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb1rstr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1rstr2(other)
    }
}

impl ::core::fmt::Display for Apb1rstr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1rstr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim2rst() != 0 { try!(write!(f, " lptim2rst"))}
        if self.lpuart1rst() != 0 { try!(write!(f, " lpuart1rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
    #[doc="Serial audio interface 1 (SAI1) reset"]
    #[inline] pub fn sai1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SAI1RST != 0"]
    #[inline] pub fn test_sai1rst(&self) -> bool {
        self.sai1rst() != 0
    }

    #[doc="Sets the SAI1RST field."]
    #[inline] pub fn set_sai1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="TIM17 timer reset"]
    #[inline] pub fn tim17rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17RST != 0"]
    #[inline] pub fn test_tim17rst(&self) -> bool {
        self.tim17rst() != 0
    }

    #[doc="Sets the TIM17RST field."]
    #[inline] pub fn set_tim17rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM16 timer reset"]
    #[inline] pub fn tim16rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16RST != 0"]
    #[inline] pub fn test_tim16rst(&self) -> bool {
        self.tim16rst() != 0
    }

    #[doc="Sets the TIM16RST field."]
    #[inline] pub fn set_tim16rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="USART1 reset"]
    #[inline] pub fn usart1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1RST != 0"]
    #[inline] pub fn test_usart1rst(&self) -> bool {
        self.usart1rst() != 0
    }

    #[doc="Sets the USART1RST field."]
    #[inline] pub fn set_usart1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI1 reset"]
    #[inline] pub fn spi1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1RST != 0"]
    #[inline] pub fn test_spi1rst(&self) -> bool {
        self.spi1rst() != 0
    }

    #[doc="Sets the SPI1RST field."]
    #[inline] pub fn set_spi1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM1 timer reset"]
    #[inline] pub fn tim1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1RST != 0"]
    #[inline] pub fn test_tim1rst(&self) -> bool {
        self.tim1rst() != 0
    }

    #[doc="Sets the TIM1RST field."]
    #[inline] pub fn set_tim1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Apb2rstr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb2rstr(other)
    }
}

impl ::core::fmt::Display for Apb2rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb2rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sai1rst() != 0 { try!(write!(f, " sai1rst"))}
        if self.tim17rst() != 0 { try!(write!(f, " tim17rst"))}
        if self.tim16rst() != 0 { try!(write!(f, " tim16rst"))}
        if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
        if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
        if self.tim1rst() != 0 { try!(write!(f, " tim1rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB3 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb3rstr(pub u32);
impl Apb3rstr {
    #[doc="Radio system BLE reset"]
    #[inline] pub fn rfrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RFRST != 0"]
    #[inline] pub fn test_rfrst(&self) -> bool {
        self.rfrst() != 0
    }

    #[doc="Sets the RFRST field."]
    #[inline] pub fn set_rfrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb3rstr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb3rstr(other)
    }
}

impl ::core::fmt::Display for Apb3rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb3rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfrst() != 0 { try!(write!(f, " rfrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1enr(pub u32);
impl Ahb1enr {
    #[doc="Touch Sensing Controller clock enable"]
    #[inline] pub fn tscen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSCEN != 0"]
    #[inline] pub fn test_tscen(&self) -> bool {
        self.tscen() != 0
    }

    #[doc="Sets the TSCEN field."]
    #[inline] pub fn set_tscen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CPU1 CRC clock enable"]
    #[inline] pub fn crcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCEN != 0"]
    #[inline] pub fn test_crcen(&self) -> bool {
        self.crcen() != 0
    }

    #[doc="Sets the CRCEN field."]
    #[inline] pub fn set_crcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DMAMUX clock enable"]
    #[inline] pub fn dmamuxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAMUXEN != 0"]
    #[inline] pub fn test_dmamuxen(&self) -> bool {
        self.dmamuxen() != 0
    }

    #[doc="Sets the DMAMUXEN field."]
    #[inline] pub fn set_dmamuxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA2 clock enable"]
    #[inline] pub fn dma2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2EN != 0"]
    #[inline] pub fn test_dma2en(&self) -> bool {
        self.dma2en() != 0
    }

    #[doc="Sets the DMA2EN field."]
    #[inline] pub fn set_dma2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMA1 clock enable"]
    #[inline] pub fn dma1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1EN != 0"]
    #[inline] pub fn test_dma1en(&self) -> bool {
        self.dma1en() != 0
    }

    #[doc="Sets the DMA1EN field."]
    #[inline] pub fn set_dma1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb1enr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb1enr(other)
    }
}

impl ::core::fmt::Display for Ahb1enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb1enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tscen() != 0 { try!(write!(f, " tscen"))}
        if self.crcen() != 0 { try!(write!(f, " crcen"))}
        if self.dmamuxen() != 0 { try!(write!(f, " dmamuxen"))}
        if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
        if self.dma1en() != 0 { try!(write!(f, " dma1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB2 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb2enr(pub u32);
impl Ahb2enr {
    #[doc="AES1 accelerator clock enable"]
    #[inline] pub fn aes1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AES1EN != 0"]
    #[inline] pub fn test_aes1en(&self) -> bool {
        self.aes1en() != 0
    }

    #[doc="Sets the AES1EN field."]
    #[inline] pub fn set_aes1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ADC clock enable"]
    #[inline] pub fn adcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCEN != 0"]
    #[inline] pub fn test_adcen(&self) -> bool {
        self.adcen() != 0
    }

    #[doc="Sets the ADCEN field."]
    #[inline] pub fn set_adcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="IO port H clock enable"]
    #[inline] pub fn gpiohen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHEN != 0"]
    #[inline] pub fn test_gpiohen(&self) -> bool {
        self.gpiohen() != 0
    }

    #[doc="Sets the GPIOHEN field."]
    #[inline] pub fn set_gpiohen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IO port E clock enable"]
    #[inline] pub fn gpioeen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOEEN != 0"]
    #[inline] pub fn test_gpioeen(&self) -> bool {
        self.gpioeen() != 0
    }

    #[doc="Sets the GPIOEEN field."]
    #[inline] pub fn set_gpioeen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IO port D clock enable"]
    #[inline] pub fn gpioden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODEN != 0"]
    #[inline] pub fn test_gpioden(&self) -> bool {
        self.gpioden() != 0
    }

    #[doc="Sets the GPIODEN field."]
    #[inline] pub fn set_gpioden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IO port C clock enable"]
    #[inline] pub fn gpiocen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCEN != 0"]
    #[inline] pub fn test_gpiocen(&self) -> bool {
        self.gpiocen() != 0
    }

    #[doc="Sets the GPIOCEN field."]
    #[inline] pub fn set_gpiocen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IO port B clock enable"]
    #[inline] pub fn gpioben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBEN != 0"]
    #[inline] pub fn test_gpioben(&self) -> bool {
        self.gpioben() != 0
    }

    #[doc="Sets the GPIOBEN field."]
    #[inline] pub fn set_gpioben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IO port A clock enable"]
    #[inline] pub fn gpioaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOAEN != 0"]
    #[inline] pub fn test_gpioaen(&self) -> bool {
        self.gpioaen() != 0
    }

    #[doc="Sets the GPIOAEN field."]
    #[inline] pub fn set_gpioaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb2enr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb2enr(other)
    }
}

impl ::core::fmt::Display for Ahb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aes1en() != 0 { try!(write!(f, " aes1en"))}
        if self.adcen() != 0 { try!(write!(f, " adcen"))}
        if self.gpiohen() != 0 { try!(write!(f, " gpiohen"))}
        if self.gpioeen() != 0 { try!(write!(f, " gpioeen"))}
        if self.gpioden() != 0 { try!(write!(f, " gpioden"))}
        if self.gpiocen() != 0 { try!(write!(f, " gpiocen"))}
        if self.gpioben() != 0 { try!(write!(f, " gpioben"))}
        if self.gpioaen() != 0 { try!(write!(f, " gpioaen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB3 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb3enr(pub u32);
impl Ahb3enr {
    #[doc="FLASHEN"]
    #[inline] pub fn flashen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FLASHEN != 0"]
    #[inline] pub fn test_flashen(&self) -> bool {
        self.flashen() != 0
    }

    #[doc="Sets the FLASHEN field."]
    #[inline] pub fn set_flashen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="IPCCEN"]
    #[inline] pub fn ipccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IPCCEN != 0"]
    #[inline] pub fn test_ipccen(&self) -> bool {
        self.ipccen() != 0
    }

    #[doc="Sets the IPCCEN field."]
    #[inline] pub fn set_ipccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="HSEMEN"]
    #[inline] pub fn hsemen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HSEMEN != 0"]
    #[inline] pub fn test_hsemen(&self) -> bool {
        self.hsemen() != 0
    }

    #[doc="Sets the HSEMEN field."]
    #[inline] pub fn set_hsemen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="RNGEN"]
    #[inline] pub fn rngen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RNGEN != 0"]
    #[inline] pub fn test_rngen(&self) -> bool {
        self.rngen() != 0
    }

    #[doc="Sets the RNGEN field."]
    #[inline] pub fn set_rngen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="AES2EN"]
    #[inline] pub fn aes2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if AES2EN != 0"]
    #[inline] pub fn test_aes2en(&self) -> bool {
        self.aes2en() != 0
    }

    #[doc="Sets the AES2EN field."]
    #[inline] pub fn set_aes2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PKAEN"]
    #[inline] pub fn pkaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PKAEN != 0"]
    #[inline] pub fn test_pkaen(&self) -> bool {
        self.pkaen() != 0
    }

    #[doc="Sets the PKAEN field."]
    #[inline] pub fn set_pkaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="QSPIEN"]
    #[inline] pub fn qspien(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if QSPIEN != 0"]
    #[inline] pub fn test_qspien(&self) -> bool {
        self.qspien() != 0
    }

    #[doc="Sets the QSPIEN field."]
    #[inline] pub fn set_qspien<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Ahb3enr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb3enr(other)
    }
}

impl ::core::fmt::Display for Ahb3enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb3enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flashen() != 0 { try!(write!(f, " flashen"))}
        if self.ipccen() != 0 { try!(write!(f, " ipccen"))}
        if self.hsemen() != 0 { try!(write!(f, " hsemen"))}
        if self.rngen() != 0 { try!(write!(f, " rngen"))}
        if self.aes2en() != 0 { try!(write!(f, " aes2en"))}
        if self.pkaen() != 0 { try!(write!(f, " pkaen"))}
        if self.qspien() != 0 { try!(write!(f, " qspien"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1ENR1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1enr1(pub u32);
impl Apb1enr1 {
    #[doc="CPU1 Low power timer 1 clock enable"]
    #[inline] pub fn lptim1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPTIM1EN != 0"]
    #[inline] pub fn test_lptim1en(&self) -> bool {
        self.lptim1en() != 0
    }

    #[doc="Sets the LPTIM1EN field."]
    #[inline] pub fn set_lptim1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="CPU1 USB clock enable"]
    #[inline] pub fn usben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if USBEN != 0"]
    #[inline] pub fn test_usben(&self) -> bool {
        self.usben() != 0
    }

    #[doc="Sets the USBEN field."]
    #[inline] pub fn set_usben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CPU1 CRS clock enable"]
    #[inline] pub fn crsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CRSEN != 0"]
    #[inline] pub fn test_crsen(&self) -> bool {
        self.crsen() != 0
    }

    #[doc="Sets the CRSEN field."]
    #[inline] pub fn set_crsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CPU1 I2C3 clock enable"]
    #[inline] pub fn i2c3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3EN != 0"]
    #[inline] pub fn test_i2c3en(&self) -> bool {
        self.i2c3en() != 0
    }

    #[doc="Sets the I2C3EN field."]
    #[inline] pub fn set_i2c3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="CPU1 I2C1 clock enable"]
    #[inline] pub fn i2c1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1EN != 0"]
    #[inline] pub fn test_i2c1en(&self) -> bool {
        self.i2c1en() != 0
    }

    #[doc="Sets the I2C1EN field."]
    #[inline] pub fn set_i2c1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="CPU1 SPI2 clock enable"]
    #[inline] pub fn spi2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2EN != 0"]
    #[inline] pub fn test_spi2en(&self) -> bool {
        self.spi2en() != 0
    }

    #[doc="Sets the SPI2EN field."]
    #[inline] pub fn set_spi2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CPU1 Window watchdog clock enable"]
    #[inline] pub fn wwdgen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGEN != 0"]
    #[inline] pub fn test_wwdgen(&self) -> bool {
        self.wwdgen() != 0
    }

    #[doc="Sets the WWDGEN field."]
    #[inline] pub fn set_wwdgen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CPU1 RTC APB clock enable"]
    #[inline] pub fn rtcapben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RTCAPBEN != 0"]
    #[inline] pub fn test_rtcapben(&self) -> bool {
        self.rtcapben() != 0
    }

    #[doc="Sets the RTCAPBEN field."]
    #[inline] pub fn set_rtcapben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CPU1 LCD clock enable"]
    #[inline] pub fn lcden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LCDEN != 0"]
    #[inline] pub fn test_lcden(&self) -> bool {
        self.lcden() != 0
    }

    #[doc="Sets the LCDEN field."]
    #[inline] pub fn set_lcden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="CPU1 TIM2 timer clock enable"]
    #[inline] pub fn tim2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2EN != 0"]
    #[inline] pub fn test_tim2en(&self) -> bool {
        self.tim2en() != 0
    }

    #[doc="Sets the TIM2EN field."]
    #[inline] pub fn set_tim2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb1enr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1enr1(other)
    }
}

impl ::core::fmt::Display for Apb1enr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1enr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim1en() != 0 { try!(write!(f, " lptim1en"))}
        if self.usben() != 0 { try!(write!(f, " usben"))}
        if self.crsen() != 0 { try!(write!(f, " crsen"))}
        if self.i2c3en() != 0 { try!(write!(f, " i2c3en"))}
        if self.i2c1en() != 0 { try!(write!(f, " i2c1en"))}
        if self.spi2en() != 0 { try!(write!(f, " spi2en"))}
        if self.wwdgen() != 0 { try!(write!(f, " wwdgen"))}
        if self.rtcapben() != 0 { try!(write!(f, " rtcapben"))}
        if self.lcden() != 0 { try!(write!(f, " lcden"))}
        if self.tim2en() != 0 { try!(write!(f, " tim2en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clock enable register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1enr2(pub u32);
impl Apb1enr2 {
    #[doc="CPU1 LPTIM2EN"]
    #[inline] pub fn lptim2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LPTIM2EN != 0"]
    #[inline] pub fn test_lptim2en(&self) -> bool {
        self.lptim2en() != 0
    }

    #[doc="Sets the LPTIM2EN field."]
    #[inline] pub fn set_lptim2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="CPU1 Low power UART 1 clock enable"]
    #[inline] pub fn lpuart1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPUART1EN != 0"]
    #[inline] pub fn test_lpuart1en(&self) -> bool {
        self.lpuart1en() != 0
    }

    #[doc="Sets the LPUART1EN field."]
    #[inline] pub fn set_lpuart1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb1enr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1enr2(other)
    }
}

impl ::core::fmt::Display for Apb1enr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1enr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim2en() != 0 { try!(write!(f, " lptim2en"))}
        if self.lpuart1en() != 0 { try!(write!(f, " lpuart1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2ENR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
    #[doc="CPU1 SAI1 clock enable"]
    #[inline] pub fn sai1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SAI1EN != 0"]
    #[inline] pub fn test_sai1en(&self) -> bool {
        self.sai1en() != 0
    }

    #[doc="Sets the SAI1EN field."]
    #[inline] pub fn set_sai1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="CPU1 TIM17 timer clock enable"]
    #[inline] pub fn tim17en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17EN != 0"]
    #[inline] pub fn test_tim17en(&self) -> bool {
        self.tim17en() != 0
    }

    #[doc="Sets the TIM17EN field."]
    #[inline] pub fn set_tim17en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="CPU1 TIM16 timer clock enable"]
    #[inline] pub fn tim16en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16EN != 0"]
    #[inline] pub fn test_tim16en(&self) -> bool {
        self.tim16en() != 0
    }

    #[doc="Sets the TIM16EN field."]
    #[inline] pub fn set_tim16en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="CPU1 USART1clock enable"]
    #[inline] pub fn usart1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1EN != 0"]
    #[inline] pub fn test_usart1en(&self) -> bool {
        self.usart1en() != 0
    }

    #[doc="Sets the USART1EN field."]
    #[inline] pub fn set_usart1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CPU1 SPI1 clock enable"]
    #[inline] pub fn spi1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1EN != 0"]
    #[inline] pub fn test_spi1en(&self) -> bool {
        self.spi1en() != 0
    }

    #[doc="Sets the SPI1EN field."]
    #[inline] pub fn set_spi1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CPU1 TIM1 timer clock enable"]
    #[inline] pub fn tim1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1EN != 0"]
    #[inline] pub fn test_tim1en(&self) -> bool {
        self.tim1en() != 0
    }

    #[doc="Sets the TIM1EN field."]
    #[inline] pub fn set_tim1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Apb2enr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb2enr(other)
    }
}

impl ::core::fmt::Display for Apb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sai1en() != 0 { try!(write!(f, " sai1en"))}
        if self.tim17en() != 0 { try!(write!(f, " tim17en"))}
        if self.tim16en() != 0 { try!(write!(f, " tim16en"))}
        if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
        if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
        if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral clocks enable in Sleep and Stop modes register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1smenr(pub u32);
impl Ahb1smenr {
    #[doc="CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tscsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSCSMEN != 0"]
    #[inline] pub fn test_tscsmen(&self) -> bool {
        self.tscsmen() != 0
    }

    #[doc="Sets the TSCSMEN field."]
    #[inline] pub fn set_tscsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CPU1 CRCSMEN"]
    #[inline] pub fn crcsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCSMEN != 0"]
    #[inline] pub fn test_crcsmen(&self) -> bool {
        self.crcsmen() != 0
    }

    #[doc="Sets the CRCSMEN field."]
    #[inline] pub fn set_crcsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline] pub fn sram1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SRAM1SMEN != 0"]
    #[inline] pub fn test_sram1smen(&self) -> bool {
        self.sram1smen() != 0
    }

    #[doc="Sets the SRAM1SMEN field."]
    #[inline] pub fn set_sram1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dmamuxsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAMUXSMEN != 0"]
    #[inline] pub fn test_dmamuxsmen(&self) -> bool {
        self.dmamuxsmen() != 0
    }

    #[doc="Sets the DMAMUXSMEN field."]
    #[inline] pub fn set_dmamuxsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CPU1 DMA2 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dma2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2SMEN != 0"]
    #[inline] pub fn test_dma2smen(&self) -> bool {
        self.dma2smen() != 0
    }

    #[doc="Sets the DMA2SMEN field."]
    #[inline] pub fn set_dma2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CPU1 DMA1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dma1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1SMEN != 0"]
    #[inline] pub fn test_dma1smen(&self) -> bool {
        self.dma1smen() != 0
    }

    #[doc="Sets the DMA1SMEN field."]
    #[inline] pub fn set_dma1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb1smenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb1smenr(other)
    }
}

impl ::core::fmt::Display for Ahb1smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb1smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tscsmen() != 0 { try!(write!(f, " tscsmen"))}
        if self.crcsmen() != 0 { try!(write!(f, " crcsmen"))}
        if self.sram1smen() != 0 { try!(write!(f, " sram1smen"))}
        if self.dmamuxsmen() != 0 { try!(write!(f, " dmamuxsmen"))}
        if self.dma2smen() != 0 { try!(write!(f, " dma2smen"))}
        if self.dma1smen() != 0 { try!(write!(f, " dma1smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB2 peripheral clocks enable in Sleep and Stop modes register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb2smenr(pub u32);
impl Ahb2smenr {
    #[doc="CPU1 AES1 accelerator clocks enable during Sleep and Stop modes"]
    #[inline] pub fn aes1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AES1SMEN != 0"]
    #[inline] pub fn test_aes1smen(&self) -> bool {
        self.aes1smen() != 0
    }

    #[doc="Sets the AES1SMEN field."]
    #[inline] pub fn set_aes1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CPU1 ADC clocks enable during Sleep and Stop modes"]
    #[inline] pub fn adcfssmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCFSSMEN != 0"]
    #[inline] pub fn test_adcfssmen(&self) -> bool {
        self.adcfssmen() != 0
    }

    #[doc="Sets the ADCFSSMEN field."]
    #[inline] pub fn set_adcfssmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CPU1 IO port H clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiohsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHSMEN != 0"]
    #[inline] pub fn test_gpiohsmen(&self) -> bool {
        self.gpiohsmen() != 0
    }

    #[doc="Sets the GPIOHSMEN field."]
    #[inline] pub fn set_gpiohsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CPU1 IO port E clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpioesmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOESMEN != 0"]
    #[inline] pub fn test_gpioesmen(&self) -> bool {
        self.gpioesmen() != 0
    }

    #[doc="Sets the GPIOESMEN field."]
    #[inline] pub fn set_gpioesmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CPU1 IO port D clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiodsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODSMEN != 0"]
    #[inline] pub fn test_gpiodsmen(&self) -> bool {
        self.gpiodsmen() != 0
    }

    #[doc="Sets the GPIODSMEN field."]
    #[inline] pub fn set_gpiodsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="CPU1 IO port C clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiocsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCSMEN != 0"]
    #[inline] pub fn test_gpiocsmen(&self) -> bool {
        self.gpiocsmen() != 0
    }

    #[doc="Sets the GPIOCSMEN field."]
    #[inline] pub fn set_gpiocsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CPU1 IO port B clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiobsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBSMEN != 0"]
    #[inline] pub fn test_gpiobsmen(&self) -> bool {
        self.gpiobsmen() != 0
    }

    #[doc="Sets the GPIOBSMEN field."]
    #[inline] pub fn set_gpiobsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CPU1 IO port A clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpioasmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOASMEN != 0"]
    #[inline] pub fn test_gpioasmen(&self) -> bool {
        self.gpioasmen() != 0
    }

    #[doc="Sets the GPIOASMEN field."]
    #[inline] pub fn set_gpioasmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb2smenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb2smenr(other)
    }
}

impl ::core::fmt::Display for Ahb2smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb2smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aes1smen() != 0 { try!(write!(f, " aes1smen"))}
        if self.adcfssmen() != 0 { try!(write!(f, " adcfssmen"))}
        if self.gpiohsmen() != 0 { try!(write!(f, " gpiohsmen"))}
        if self.gpioesmen() != 0 { try!(write!(f, " gpioesmen"))}
        if self.gpiodsmen() != 0 { try!(write!(f, " gpiodsmen"))}
        if self.gpiocsmen() != 0 { try!(write!(f, " gpiocsmen"))}
        if self.gpiobsmen() != 0 { try!(write!(f, " gpiobsmen"))}
        if self.gpioasmen() != 0 { try!(write!(f, " gpioasmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB3 peripheral clocks enable in Sleep and Stop modes register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb3smenr(pub u32);
impl Ahb3smenr {
    #[doc="Flash interface clocks enable during CPU1 sleep mode"]
    #[inline] pub fn flashsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FLASHSMEN != 0"]
    #[inline] pub fn test_flashsmen(&self) -> bool {
        self.flashsmen() != 0
    }

    #[doc="Sets the FLASHSMEN field."]
    #[inline] pub fn set_flashsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode"]
    #[inline] pub fn sram2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SRAM2SMEN != 0"]
    #[inline] pub fn test_sram2smen(&self) -> bool {
        self.sram2smen() != 0
    }

    #[doc="Sets the SRAM2SMEN field."]
    #[inline] pub fn set_sram2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="True RNG clocks enable during CPU1 sleep mode"]
    #[inline] pub fn rngsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RNGSMEN != 0"]
    #[inline] pub fn test_rngsmen(&self) -> bool {
        self.rngsmen() != 0
    }

    #[doc="Sets the RNGSMEN field."]
    #[inline] pub fn set_rngsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="AES2 accelerator clocks enable during CPU1 sleep mode"]
    #[inline] pub fn aes2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if AES2SMEN != 0"]
    #[inline] pub fn test_aes2smen(&self) -> bool {
        self.aes2smen() != 0
    }

    #[doc="Sets the AES2SMEN field."]
    #[inline] pub fn set_aes2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PKA accelerator clocks enable during CPU1 sleep mode"]
    #[inline] pub fn pkasmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PKASMEN != 0"]
    #[inline] pub fn test_pkasmen(&self) -> bool {
        self.pkasmen() != 0
    }

    #[doc="Sets the PKASMEN field."]
    #[inline] pub fn set_pkasmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="QSPISMEN"]
    #[inline] pub fn qspismen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if QSPISMEN != 0"]
    #[inline] pub fn test_qspismen(&self) -> bool {
        self.qspismen() != 0
    }

    #[doc="Sets the QSPISMEN field."]
    #[inline] pub fn set_qspismen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Ahb3smenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb3smenr(other)
    }
}

impl ::core::fmt::Display for Ahb3smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb3smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flashsmen() != 0 { try!(write!(f, " flashsmen"))}
        if self.sram2smen() != 0 { try!(write!(f, " sram2smen"))}
        if self.rngsmen() != 0 { try!(write!(f, " rngsmen"))}
        if self.aes2smen() != 0 { try!(write!(f, " aes2smen"))}
        if self.pkasmen() != 0 { try!(write!(f, " pkasmen"))}
        if self.qspismen() != 0 { try!(write!(f, " qspismen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1SMENR1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1smenr1(pub u32);
impl Apb1smenr1 {
    #[doc="Low power timer 1 clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn lptim1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPTIM1SMEN != 0"]
    #[inline] pub fn test_lptim1smen(&self) -> bool {
        self.lptim1smen() != 0
    }

    #[doc="Sets the LPTIM1SMEN field."]
    #[inline] pub fn set_lptim1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="USB FS clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn usbsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if USBSMEN != 0"]
    #[inline] pub fn test_usbsmen(&self) -> bool {
        self.usbsmen() != 0
    }

    #[doc="Sets the USBSMEN field."]
    #[inline] pub fn set_usbsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CRS clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn crsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CRSMEN != 0"]
    #[inline] pub fn test_crsmen(&self) -> bool {
        self.crsmen() != 0
    }

    #[doc="Sets the CRSMEN field."]
    #[inline] pub fn set_crsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="I2C3 clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn i2c3smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3SMEN != 0"]
    #[inline] pub fn test_i2c3smen(&self) -> bool {
        self.i2c3smen() != 0
    }

    #[doc="Sets the I2C3SMEN field."]
    #[inline] pub fn set_i2c3smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C1 clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn i2c1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1SMEN != 0"]
    #[inline] pub fn test_i2c1smen(&self) -> bool {
        self.i2c1smen() != 0
    }

    #[doc="Sets the I2C1SMEN field."]
    #[inline] pub fn set_i2c1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SPI2 clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn spi2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2SMEN != 0"]
    #[inline] pub fn test_spi2smen(&self) -> bool {
        self.spi2smen() != 0
    }

    #[doc="Sets the SPI2SMEN field."]
    #[inline] pub fn set_spi2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Window watchdog clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn wwdgsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGSMEN != 0"]
    #[inline] pub fn test_wwdgsmen(&self) -> bool {
        self.wwdgsmen() != 0
    }

    #[doc="Sets the WWDGSMEN field."]
    #[inline] pub fn set_wwdgsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="RTC APB clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn rtcapbsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RTCAPBSMEN != 0"]
    #[inline] pub fn test_rtcapbsmen(&self) -> bool {
        self.rtcapbsmen() != 0
    }

    #[doc="Sets the RTCAPBSMEN field."]
    #[inline] pub fn set_rtcapbsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="LCD clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn lcdsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LCDSMEN != 0"]
    #[inline] pub fn test_lcdsmen(&self) -> bool {
        self.lcdsmen() != 0
    }

    #[doc="Sets the LCDSMEN field."]
    #[inline] pub fn set_lcdsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TIM2 timer clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn tim2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2SMEN != 0"]
    #[inline] pub fn test_tim2smen(&self) -> bool {
        self.tim2smen() != 0
    }

    #[doc="Sets the TIM2SMEN field."]
    #[inline] pub fn set_tim2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb1smenr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1smenr1(other)
    }
}

impl ::core::fmt::Display for Apb1smenr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1smenr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim1smen() != 0 { try!(write!(f, " lptim1smen"))}
        if self.usbsmen() != 0 { try!(write!(f, " usbsmen"))}
        if self.crsmen() != 0 { try!(write!(f, " crsmen"))}
        if self.i2c3smen() != 0 { try!(write!(f, " i2c3smen"))}
        if self.i2c1smen() != 0 { try!(write!(f, " i2c1smen"))}
        if self.spi2smen() != 0 { try!(write!(f, " spi2smen"))}
        if self.wwdgsmen() != 0 { try!(write!(f, " wwdgsmen"))}
        if self.rtcapbsmen() != 0 { try!(write!(f, " rtcapbsmen"))}
        if self.lcdsmen() != 0 { try!(write!(f, " lcdsmen"))}
        if self.tim2smen() != 0 { try!(write!(f, " tim2smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1smenr2(pub u32);
impl Apb1smenr2 {
    #[doc="Low power timer 2 clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn lptim2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LPTIM2SMEN != 0"]
    #[inline] pub fn test_lptim2smen(&self) -> bool {
        self.lptim2smen() != 0
    }

    #[doc="Sets the LPTIM2SMEN field."]
    #[inline] pub fn set_lptim2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Low power UART 1 clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn lpuart1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPUART1SMEN != 0"]
    #[inline] pub fn test_lpuart1smen(&self) -> bool {
        self.lpuart1smen() != 0
    }

    #[doc="Sets the LPUART1SMEN field."]
    #[inline] pub fn set_lpuart1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb1smenr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1smenr2(other)
    }
}

impl ::core::fmt::Display for Apb1smenr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1smenr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim2smen() != 0 { try!(write!(f, " lptim2smen"))}
        if self.lpuart1smen() != 0 { try!(write!(f, " lpuart1smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2SMENR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2smenr(pub u32);
impl Apb2smenr {
    #[doc="SAI1 clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn sai1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SAI1SMEN != 0"]
    #[inline] pub fn test_sai1smen(&self) -> bool {
        self.sai1smen() != 0
    }

    #[doc="Sets the SAI1SMEN field."]
    #[inline] pub fn set_sai1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="TIM17 timer clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn tim17smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17SMEN != 0"]
    #[inline] pub fn test_tim17smen(&self) -> bool {
        self.tim17smen() != 0
    }

    #[doc="Sets the TIM17SMEN field."]
    #[inline] pub fn set_tim17smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM16 timer clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn tim16smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16SMEN != 0"]
    #[inline] pub fn test_tim16smen(&self) -> bool {
        self.tim16smen() != 0
    }

    #[doc="Sets the TIM16SMEN field."]
    #[inline] pub fn set_tim16smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="USART1clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn usart1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1SMEN != 0"]
    #[inline] pub fn test_usart1smen(&self) -> bool {
        self.usart1smen() != 0
    }

    #[doc="Sets the USART1SMEN field."]
    #[inline] pub fn set_usart1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI1 clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn spi1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1SMEN != 0"]
    #[inline] pub fn test_spi1smen(&self) -> bool {
        self.spi1smen() != 0
    }

    #[doc="Sets the SPI1SMEN field."]
    #[inline] pub fn set_spi1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM1 timer clocks enable during CPU1 Sleep mode"]
    #[inline] pub fn tim1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1SMEN != 0"]
    #[inline] pub fn test_tim1smen(&self) -> bool {
        self.tim1smen() != 0
    }

    #[doc="Sets the TIM1SMEN field."]
    #[inline] pub fn set_tim1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Apb2smenr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb2smenr(other)
    }
}

impl ::core::fmt::Display for Apb2smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb2smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sai1smen() != 0 { try!(write!(f, " sai1smen"))}
        if self.tim17smen() != 0 { try!(write!(f, " tim17smen"))}
        if self.tim16smen() != 0 { try!(write!(f, " tim16smen"))}
        if self.usart1smen() != 0 { try!(write!(f, " usart1smen"))}
        if self.spi1smen() != 0 { try!(write!(f, " spi1smen"))}
        if self.tim1smen() != 0 { try!(write!(f, " tim1smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CCIPR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccipr(pub u32);
impl Ccipr {
    #[doc="RNG clock source selection"]
    #[inline] pub fn rngsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if RNGSEL != 0"]
    #[inline] pub fn test_rngsel(&self) -> bool {
        self.rngsel() != 0
    }

    #[doc="Sets the RNGSEL field."]
    #[inline] pub fn set_rngsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="ADCs clock source selection"]
    #[inline] pub fn adcsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ADCSEL != 0"]
    #[inline] pub fn test_adcsel(&self) -> bool {
        self.adcsel() != 0
    }

    #[doc="Sets the ADCSEL field."]
    #[inline] pub fn set_adcsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="48 MHz clock source selection"]
    #[inline] pub fn clk48sel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3) as u8) } // [27:26]
    }

    #[doc="Returns true if CLK48SEL != 0"]
    #[inline] pub fn test_clk48sel(&self) -> bool {
        self.clk48sel() != 0
    }

    #[doc="Sets the CLK48SEL field."]
    #[inline] pub fn set_clk48sel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="SAI1 clock source selection"]
    #[inline] pub fn sai1sel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if SAI1SEL != 0"]
    #[inline] pub fn test_sai1sel(&self) -> bool {
        self.sai1sel() != 0
    }

    #[doc="Sets the SAI1SEL field."]
    #[inline] pub fn set_sai1sel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Low power timer 2 clock source selection"]
    #[inline] pub fn lptim2sel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if LPTIM2SEL != 0"]
    #[inline] pub fn test_lptim2sel(&self) -> bool {
        self.lptim2sel() != 0
    }

    #[doc="Sets the LPTIM2SEL field."]
    #[inline] pub fn set_lptim2sel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Low power timer 1 clock source selection"]
    #[inline] pub fn lptim1sel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if LPTIM1SEL != 0"]
    #[inline] pub fn test_lptim1sel(&self) -> bool {
        self.lptim1sel() != 0
    }

    #[doc="Sets the LPTIM1SEL field."]
    #[inline] pub fn set_lptim1sel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="I2C3 clock source selection"]
    #[inline] pub fn i2c3sel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if I2C3SEL != 0"]
    #[inline] pub fn test_i2c3sel(&self) -> bool {
        self.i2c3sel() != 0
    }

    #[doc="Sets the I2C3SEL field."]
    #[inline] pub fn set_i2c3sel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="I2C1 clock source selection"]
    #[inline] pub fn i2c1sel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if I2C1SEL != 0"]
    #[inline] pub fn test_i2c1sel(&self) -> bool {
        self.i2c1sel() != 0
    }

    #[doc="Sets the I2C1SEL field."]
    #[inline] pub fn set_i2c1sel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="LPUART1 clock source selection"]
    #[inline] pub fn lpuart1sel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if LPUART1SEL != 0"]
    #[inline] pub fn test_lpuart1sel(&self) -> bool {
        self.lpuart1sel() != 0
    }

    #[doc="Sets the LPUART1SEL field."]
    #[inline] pub fn set_lpuart1sel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="USART1 clock source selection"]
    #[inline] pub fn usart1sel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if USART1SEL != 0"]
    #[inline] pub fn test_usart1sel(&self) -> bool {
        self.usart1sel() != 0
    }

    #[doc="Sets the USART1SEL field."]
    #[inline] pub fn set_usart1sel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccipr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccipr(other)
    }
}

impl ::core::fmt::Display for Ccipr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccipr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rngsel() != 0 { try!(write!(f, " rngsel=0x{:x}", self.rngsel()))}
        if self.adcsel() != 0 { try!(write!(f, " adcsel=0x{:x}", self.adcsel()))}
        if self.clk48sel() != 0 { try!(write!(f, " clk48sel=0x{:x}", self.clk48sel()))}
        if self.sai1sel() != 0 { try!(write!(f, " sai1sel=0x{:x}", self.sai1sel()))}
        if self.lptim2sel() != 0 { try!(write!(f, " lptim2sel=0x{:x}", self.lptim2sel()))}
        if self.lptim1sel() != 0 { try!(write!(f, " lptim1sel=0x{:x}", self.lptim1sel()))}
        if self.i2c3sel() != 0 { try!(write!(f, " i2c3sel=0x{:x}", self.i2c3sel()))}
        if self.i2c1sel() != 0 { try!(write!(f, " i2c1sel=0x{:x}", self.i2c1sel()))}
        if self.lpuart1sel() != 0 { try!(write!(f, " lpuart1sel=0x{:x}", self.lpuart1sel()))}
        if self.usart1sel() != 0 { try!(write!(f, " usart1sel=0x{:x}", self.usart1sel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BDCR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdcr(pub u32);
impl Bdcr {
    #[doc="Low speed clock output selection"]
    #[inline] pub fn lscosel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if LSCOSEL != 0"]
    #[inline] pub fn test_lscosel(&self) -> bool {
        self.lscosel() != 0
    }

    #[doc="Sets the LSCOSEL field."]
    #[inline] pub fn set_lscosel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Low speed clock output enable"]
    #[inline] pub fn lscoen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if LSCOEN != 0"]
    #[inline] pub fn test_lscoen(&self) -> bool {
        self.lscoen() != 0
    }

    #[doc="Sets the LSCOEN field."]
    #[inline] pub fn set_lscoen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Backup domain software reset"]
    #[inline] pub fn bdrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BDRST != 0"]
    #[inline] pub fn test_bdrst(&self) -> bool {
        self.bdrst() != 0
    }

    #[doc="Sets the BDRST field."]
    #[inline] pub fn set_bdrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="RTC clock enable"]
    #[inline] pub fn rtcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RTCEN != 0"]
    #[inline] pub fn test_rtcen(&self) -> bool {
        self.rtcen() != 0
    }

    #[doc="Sets the RTCEN field."]
    #[inline] pub fn set_rtcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="RTC clock source selection"]
    #[inline] pub fn rtcsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if RTCSEL != 0"]
    #[inline] pub fn test_rtcsel(&self) -> bool {
        self.rtcsel() != 0
    }

    #[doc="Sets the RTCSEL field."]
    #[inline] pub fn set_rtcsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CSS on LSE failure detection"]
    #[inline] pub fn lsecssd_(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LSECSSD_ != 0"]
    #[inline] pub fn test_lsecssd_(&self) -> bool {
        self.lsecssd_() != 0
    }

    #[doc="Sets the LSECSSD_ field."]
    #[inline] pub fn set_lsecssd_<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LSECSSON"]
    #[inline] pub fn lsecsson(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LSECSSON != 0"]
    #[inline] pub fn test_lsecsson(&self) -> bool {
        self.lsecsson() != 0
    }

    #[doc="Sets the LSECSSON field."]
    #[inline] pub fn set_lsecsson<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SE oscillator drive capability"]
    #[inline] pub fn lsedrv(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if LSEDRV != 0"]
    #[inline] pub fn test_lsedrv(&self) -> bool {
        self.lsedrv() != 0
    }

    #[doc="Sets the LSEDRV field."]
    #[inline] pub fn set_lsedrv<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="LSE oscillator bypass"]
    #[inline] pub fn lsebyp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LSEBYP != 0"]
    #[inline] pub fn test_lsebyp(&self) -> bool {
        self.lsebyp() != 0
    }

    #[doc="Sets the LSEBYP field."]
    #[inline] pub fn set_lsebyp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE oscillator ready"]
    #[inline] pub fn lserdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDY != 0"]
    #[inline] pub fn test_lserdy(&self) -> bool {
        self.lserdy() != 0
    }

    #[doc="Sets the LSERDY field."]
    #[inline] pub fn set_lserdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSE oscillator enable"]
    #[inline] pub fn lseon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSEON != 0"]
    #[inline] pub fn test_lseon(&self) -> bool {
        self.lseon() != 0
    }

    #[doc="Sets the LSEON field."]
    #[inline] pub fn set_lseon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bdcr {
    #[inline]
    fn from(other: u32) -> Self {
         Bdcr(other)
    }
}

impl ::core::fmt::Display for Bdcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lscosel() != 0 { try!(write!(f, " lscosel=0x{:x}", self.lscosel()))}
        if self.lscoen() != 0 { try!(write!(f, " lscoen"))}
        if self.bdrst() != 0 { try!(write!(f, " bdrst"))}
        if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
        if self.rtcsel() != 0 { try!(write!(f, " rtcsel=0x{:x}", self.rtcsel()))}
        if self.lsecssd_() != 0 { try!(write!(f, " lsecssd_"))}
        if self.lsecsson() != 0 { try!(write!(f, " lsecsson"))}
        if self.lsedrv() != 0 { try!(write!(f, " lsedrv=0x{:x}", self.lsedrv()))}
        if self.lsebyp() != 0 { try!(write!(f, " lsebyp"))}
        if self.lserdy() != 0 { try!(write!(f, " lserdy"))}
        if self.lseon() != 0 { try!(write!(f, " lseon"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CSR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Low-power reset flag"]
    #[inline] pub fn lpwrrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPWRRSTF != 0"]
    #[inline] pub fn test_lpwrrstf(&self) -> bool {
        self.lpwrrstf() != 0
    }

    #[doc="Sets the LPWRRSTF field."]
    #[inline] pub fn set_lpwrrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Window watchdog reset flag"]
    #[inline] pub fn wwdgrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if WWDGRSTF != 0"]
    #[inline] pub fn test_wwdgrstf(&self) -> bool {
        self.wwdgrstf() != 0
    }

    #[doc="Sets the WWDGRSTF field."]
    #[inline] pub fn set_wwdgrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Independent window watchdog reset flag"]
    #[inline] pub fn iwdgrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if IWDGRSTF != 0"]
    #[inline] pub fn test_iwdgrstf(&self) -> bool {
        self.iwdgrstf() != 0
    }

    #[doc="Sets the IWDGRSTF field."]
    #[inline] pub fn set_iwdgrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Software reset flag"]
    #[inline] pub fn sftrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SFTRSTF != 0"]
    #[inline] pub fn test_sftrstf(&self) -> bool {
        self.sftrstf() != 0
    }

    #[doc="Sets the SFTRSTF field."]
    #[inline] pub fn set_sftrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="BOR flag"]
    #[inline] pub fn borrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if BORRSTF != 0"]
    #[inline] pub fn test_borrstf(&self) -> bool {
        self.borrstf() != 0
    }

    #[doc="Sets the BORRSTF field."]
    #[inline] pub fn set_borrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pin reset flag"]
    #[inline] pub fn pinrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PINRSTF != 0"]
    #[inline] pub fn test_pinrstf(&self) -> bool {
        self.pinrstf() != 0
    }

    #[doc="Sets the PINRSTF field."]
    #[inline] pub fn set_pinrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Option byte loader reset flag"]
    #[inline] pub fn oblrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if OBLRSTF != 0"]
    #[inline] pub fn test_oblrstf(&self) -> bool {
        self.oblrstf() != 0
    }

    #[doc="Sets the OBLRSTF field."]
    #[inline] pub fn set_oblrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Remove reset flag"]
    #[inline] pub fn rmvf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if RMVF != 0"]
    #[inline] pub fn test_rmvf(&self) -> bool {
        self.rmvf() != 0
    }

    #[doc="Sets the RMVF field."]
    #[inline] pub fn set_rmvf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="RF system wakeup clock source selection"]
    #[inline] pub fn rfwkpsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if RFWKPSEL != 0"]
    #[inline] pub fn test_rfwkpsel(&self) -> bool {
        self.rfwkpsel() != 0
    }

    #[doc="Sets the RFWKPSEL field."]
    #[inline] pub fn set_rfwkpsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="LSI2 oscillator bias configuration"]
    #[inline] pub fn lsi2bw(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if LSI2BW != 0"]
    #[inline] pub fn test_lsi2bw(&self) -> bool {
        self.lsi2bw() != 0
    }

    #[doc="Sets the LSI2BW field."]
    #[inline] pub fn set_lsi2bw<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LSI2 oscillator trim OK"]
    #[inline] pub fn lsi2trimok(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LSI2TRIMOK != 0"]
    #[inline] pub fn test_lsi2trimok(&self) -> bool {
        self.lsi2trimok() != 0
    }

    #[doc="Sets the LSI2TRIMOK field."]
    #[inline] pub fn set_lsi2trimok<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="LSI2 oscillator trimming enable"]
    #[inline] pub fn lsi2trimen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LSI2TRIMEN != 0"]
    #[inline] pub fn test_lsi2trimen(&self) -> bool {
        self.lsi2trimen() != 0
    }

    #[doc="Sets the LSI2TRIMEN field."]
    #[inline] pub fn set_lsi2trimen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="LSI2 oscillator ready"]
    #[inline] pub fn lsi2rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LSI2RDY != 0"]
    #[inline] pub fn test_lsi2rdy(&self) -> bool {
        self.lsi2rdy() != 0
    }

    #[doc="Sets the LSI2RDY field."]
    #[inline] pub fn set_lsi2rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="LSI2 oscillator enabled"]
    #[inline] pub fn lsi2on(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LSI2ON != 0"]
    #[inline] pub fn test_lsi2on(&self) -> bool {
        self.lsi2on() != 0
    }

    #[doc="Sets the LSI2ON field."]
    #[inline] pub fn set_lsi2on<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSI1 oscillator ready"]
    #[inline] pub fn lsi1rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSI1RDY != 0"]
    #[inline] pub fn test_lsi1rdy(&self) -> bool {
        self.lsi1rdy() != 0
    }

    #[doc="Sets the LSI1RDY field."]
    #[inline] pub fn set_lsi1rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSI1 oscillator enabled"]
    #[inline] pub fn lsi1on(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSI1ON != 0"]
    #[inline] pub fn test_lsi1on(&self) -> bool {
        self.lsi1on() != 0
    }

    #[doc="Sets the LSI1ON field."]
    #[inline] pub fn set_lsi1on<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Radio system BLE and 802.15.4 reset status"]
    #[inline] pub fn rfrsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RFRSTS != 0"]
    #[inline] pub fn test_rfrsts(&self) -> bool {
        self.rfrsts() != 0
    }

    #[doc="Sets the RFRSTS field."]
    #[inline] pub fn set_rfrsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Csr(other)
    }
}

impl ::core::fmt::Display for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lpwrrstf() != 0 { try!(write!(f, " lpwrrstf"))}
        if self.wwdgrstf() != 0 { try!(write!(f, " wwdgrstf"))}
        if self.iwdgrstf() != 0 { try!(write!(f, " iwdgrstf"))}
        if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
        if self.borrstf() != 0 { try!(write!(f, " borrstf"))}
        if self.pinrstf() != 0 { try!(write!(f, " pinrstf"))}
        if self.oblrstf() != 0 { try!(write!(f, " oblrstf"))}
        if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
        if self.rfwkpsel() != 0 { try!(write!(f, " rfwkpsel=0x{:x}", self.rfwkpsel()))}
        if self.lsi2bw() != 0 { try!(write!(f, " lsi2bw=0x{:x}", self.lsi2bw()))}
        if self.lsi2trimok() != 0 { try!(write!(f, " lsi2trimok"))}
        if self.lsi2trimen() != 0 { try!(write!(f, " lsi2trimen"))}
        if self.lsi2rdy() != 0 { try!(write!(f, " lsi2rdy"))}
        if self.lsi2on() != 0 { try!(write!(f, " lsi2on"))}
        if self.lsi1rdy() != 0 { try!(write!(f, " lsi1rdy"))}
        if self.lsi1on() != 0 { try!(write!(f, " lsi1on"))}
        if self.rfrsts() != 0 { try!(write!(f, " rfrsts"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock recovery RC register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crrcr(pub u32);
impl Crrcr {
    #[doc="HSI48 clock calibration"]
    #[inline] pub fn hsi48cal(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1ff) as u16) } // [15:7]
    }

    #[doc="Returns true if HSI48CAL != 0"]
    #[inline] pub fn test_hsi48cal(&self) -> bool {
        self.hsi48cal() != 0
    }

    #[doc="Sets the HSI48CAL field."]
    #[inline] pub fn set_hsi48cal<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="HSI48 clock ready"]
    #[inline] pub fn hsi48rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HSI48RDY != 0"]
    #[inline] pub fn test_hsi48rdy(&self) -> bool {
        self.hsi48rdy() != 0
    }

    #[doc="Sets the HSI48RDY field."]
    #[inline] pub fn set_hsi48rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HSI48 oscillator enabled"]
    #[inline] pub fn hsi48on(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HSI48ON != 0"]
    #[inline] pub fn test_hsi48on(&self) -> bool {
        self.hsi48on() != 0
    }

    #[doc="Sets the HSI48ON field."]
    #[inline] pub fn set_hsi48on<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Crrcr {
    #[inline]
    fn from(other: u32) -> Self {
         Crrcr(other)
    }
}

impl ::core::fmt::Display for Crrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hsi48cal() != 0 { try!(write!(f, " hsi48cal=0x{:x}", self.hsi48cal()))}
        if self.hsi48rdy() != 0 { try!(write!(f, " hsi48rdy"))}
        if self.hsi48on() != 0 { try!(write!(f, " hsi48on"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock HSE register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hsecr(pub u32);
impl Hsecr {
    #[doc="HSE capacitor tuning"]
    #[inline] pub fn hsetune(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if HSETUNE != 0"]
    #[inline] pub fn test_hsetune(&self) -> bool {
        self.hsetune() != 0
    }

    #[doc="Sets the HSETUNE field."]
    #[inline] pub fn set_hsetune<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HSE current control"]
    #[inline] pub fn hsegmc(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if HSEGMC != 0"]
    #[inline] pub fn test_hsegmc(&self) -> bool {
        self.hsegmc() != 0
    }

    #[doc="Sets the HSEGMC field."]
    #[inline] pub fn set_hsegmc<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSE Sense amplifier threshold"]
    #[inline] pub fn hses(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSES != 0"]
    #[inline] pub fn test_hses(&self) -> bool {
        self.hses() != 0
    }

    #[doc="Sets the HSES field."]
    #[inline] pub fn set_hses<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Register lock system"]
    #[inline] pub fn unlocked(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UNLOCKED != 0"]
    #[inline] pub fn test_unlocked(&self) -> bool {
        self.unlocked() != 0
    }

    #[doc="Sets the UNLOCKED field."]
    #[inline] pub fn set_unlocked<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hsecr {
    #[inline]
    fn from(other: u32) -> Self {
         Hsecr(other)
    }
}

impl ::core::fmt::Display for Hsecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hsecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hsetune() != 0 { try!(write!(f, " hsetune=0x{:x}", self.hsetune()))}
        if self.hsegmc() != 0 { try!(write!(f, " hsegmc=0x{:x}", self.hsegmc()))}
        if self.hses() != 0 { try!(write!(f, " hses"))}
        if self.unlocked() != 0 { try!(write!(f, " unlocked"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extended clock recovery register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extcfgr(pub u32);
impl Extcfgr {
    #[doc="RF clock source selected"]
    #[inline] pub fn rfcss(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if RFCSS != 0"]
    #[inline] pub fn test_rfcss(&self) -> bool {
        self.rfcss() != 0
    }

    #[doc="Sets the RFCSS field."]
    #[inline] pub fn set_rfcss<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="CPU2 AHB prescaler flag"]
    #[inline] pub fn c2hpref(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if C2HPREF != 0"]
    #[inline] pub fn test_c2hpref(&self) -> bool {
        self.c2hpref() != 0
    }

    #[doc="Sets the C2HPREF field."]
    #[inline] pub fn set_c2hpref<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Shared AHB prescaler flag"]
    #[inline] pub fn shdhpref(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SHDHPREF != 0"]
    #[inline] pub fn test_shdhpref(&self) -> bool {
        self.shdhpref() != 0
    }

    #[doc="Sets the SHDHPREF field."]
    #[inline] pub fn set_shdhpref<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CPU2 AHB prescaler"]
    #[inline] pub fn c2hpre(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if C2HPRE != 0"]
    #[inline] pub fn test_c2hpre(&self) -> bool {
        self.c2hpre() != 0
    }

    #[doc="Sets the C2HPRE field."]
    #[inline] pub fn set_c2hpre<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Shared AHB prescaler"]
    #[inline] pub fn shdhpre(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SHDHPRE != 0"]
    #[inline] pub fn test_shdhpre(&self) -> bool {
        self.shdhpre() != 0
    }

    #[doc="Sets the SHDHPRE field."]
    #[inline] pub fn set_shdhpre<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Extcfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Extcfgr(other)
    }
}

impl ::core::fmt::Display for Extcfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extcfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfcss() != 0 { try!(write!(f, " rfcss"))}
        if self.c2hpref() != 0 { try!(write!(f, " c2hpref"))}
        if self.shdhpref() != 0 { try!(write!(f, " shdhpref"))}
        if self.c2hpre() != 0 { try!(write!(f, " c2hpre=0x{:x}", self.c2hpre()))}
        if self.shdhpre() != 0 { try!(write!(f, " shdhpre=0x{:x}", self.shdhpre()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 AHB1 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2ahb1enr(pub u32);
impl C2ahb1enr {
    #[doc="CPU2 Touch Sensing Controller clock enable"]
    #[inline] pub fn tscen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSCEN != 0"]
    #[inline] pub fn test_tscen(&self) -> bool {
        self.tscen() != 0
    }

    #[doc="Sets the TSCEN field."]
    #[inline] pub fn set_tscen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CPU2 CRC clock enable"]
    #[inline] pub fn crcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCEN != 0"]
    #[inline] pub fn test_crcen(&self) -> bool {
        self.crcen() != 0
    }

    #[doc="Sets the CRCEN field."]
    #[inline] pub fn set_crcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CPU2 SRAM1 clock enable"]
    #[inline] pub fn sram1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SRAM1EN != 0"]
    #[inline] pub fn test_sram1en(&self) -> bool {
        self.sram1en() != 0
    }

    #[doc="Sets the SRAM1EN field."]
    #[inline] pub fn set_sram1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="CPU2 DMAMUX clock enable"]
    #[inline] pub fn dmamuxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAMUXEN != 0"]
    #[inline] pub fn test_dmamuxen(&self) -> bool {
        self.dmamuxen() != 0
    }

    #[doc="Sets the DMAMUXEN field."]
    #[inline] pub fn set_dmamuxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CPU2 DMA2 clock enable"]
    #[inline] pub fn dma2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2EN != 0"]
    #[inline] pub fn test_dma2en(&self) -> bool {
        self.dma2en() != 0
    }

    #[doc="Sets the DMA2EN field."]
    #[inline] pub fn set_dma2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CPU2 DMA1 clock enable"]
    #[inline] pub fn dma1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1EN != 0"]
    #[inline] pub fn test_dma1en(&self) -> bool {
        self.dma1en() != 0
    }

    #[doc="Sets the DMA1EN field."]
    #[inline] pub fn set_dma1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2ahb1enr {
    #[inline]
    fn from(other: u32) -> Self {
         C2ahb1enr(other)
    }
}

impl ::core::fmt::Display for C2ahb1enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2ahb1enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tscen() != 0 { try!(write!(f, " tscen"))}
        if self.crcen() != 0 { try!(write!(f, " crcen"))}
        if self.sram1en() != 0 { try!(write!(f, " sram1en"))}
        if self.dmamuxen() != 0 { try!(write!(f, " dmamuxen"))}
        if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
        if self.dma1en() != 0 { try!(write!(f, " dma1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 AHB2 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2ahb2enr(pub u32);
impl C2ahb2enr {
    #[doc="CPU2 AES1 accelerator clock enable"]
    #[inline] pub fn aes1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AES1EN != 0"]
    #[inline] pub fn test_aes1en(&self) -> bool {
        self.aes1en() != 0
    }

    #[doc="Sets the AES1EN field."]
    #[inline] pub fn set_aes1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CPU2 ADC clock enable"]
    #[inline] pub fn adcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCEN != 0"]
    #[inline] pub fn test_adcen(&self) -> bool {
        self.adcen() != 0
    }

    #[doc="Sets the ADCEN field."]
    #[inline] pub fn set_adcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CPU2 IO port H clock enable"]
    #[inline] pub fn gpiohen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHEN != 0"]
    #[inline] pub fn test_gpiohen(&self) -> bool {
        self.gpiohen() != 0
    }

    #[doc="Sets the GPIOHEN field."]
    #[inline] pub fn set_gpiohen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CPU2 IO port E clock enable"]
    #[inline] pub fn gpioeen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOEEN != 0"]
    #[inline] pub fn test_gpioeen(&self) -> bool {
        self.gpioeen() != 0
    }

    #[doc="Sets the GPIOEEN field."]
    #[inline] pub fn set_gpioeen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CPU2 IO port D clock enable"]
    #[inline] pub fn gpioden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODEN != 0"]
    #[inline] pub fn test_gpioden(&self) -> bool {
        self.gpioden() != 0
    }

    #[doc="Sets the GPIODEN field."]
    #[inline] pub fn set_gpioden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="CPU2 IO port C clock enable"]
    #[inline] pub fn gpiocen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCEN != 0"]
    #[inline] pub fn test_gpiocen(&self) -> bool {
        self.gpiocen() != 0
    }

    #[doc="Sets the GPIOCEN field."]
    #[inline] pub fn set_gpiocen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CPU2 IO port B clock enable"]
    #[inline] pub fn gpioben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBEN != 0"]
    #[inline] pub fn test_gpioben(&self) -> bool {
        self.gpioben() != 0
    }

    #[doc="Sets the GPIOBEN field."]
    #[inline] pub fn set_gpioben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CPU2 IO port A clock enable"]
    #[inline] pub fn gpioaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOAEN != 0"]
    #[inline] pub fn test_gpioaen(&self) -> bool {
        self.gpioaen() != 0
    }

    #[doc="Sets the GPIOAEN field."]
    #[inline] pub fn set_gpioaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2ahb2enr {
    #[inline]
    fn from(other: u32) -> Self {
         C2ahb2enr(other)
    }
}

impl ::core::fmt::Display for C2ahb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2ahb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aes1en() != 0 { try!(write!(f, " aes1en"))}
        if self.adcen() != 0 { try!(write!(f, " adcen"))}
        if self.gpiohen() != 0 { try!(write!(f, " gpiohen"))}
        if self.gpioeen() != 0 { try!(write!(f, " gpioeen"))}
        if self.gpioden() != 0 { try!(write!(f, " gpioden"))}
        if self.gpiocen() != 0 { try!(write!(f, " gpiocen"))}
        if self.gpioben() != 0 { try!(write!(f, " gpioben"))}
        if self.gpioaen() != 0 { try!(write!(f, " gpioaen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 AHB3 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2ahb3enr(pub u32);
impl C2ahb3enr {
    #[doc="CPU2 FLASHEN"]
    #[inline] pub fn flashen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FLASHEN != 0"]
    #[inline] pub fn test_flashen(&self) -> bool {
        self.flashen() != 0
    }

    #[doc="Sets the FLASHEN field."]
    #[inline] pub fn set_flashen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="CPU2 IPCCEN"]
    #[inline] pub fn ipccen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IPCCEN != 0"]
    #[inline] pub fn test_ipccen(&self) -> bool {
        self.ipccen() != 0
    }

    #[doc="Sets the IPCCEN field."]
    #[inline] pub fn set_ipccen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="CPU2 HSEMEN"]
    #[inline] pub fn hsemen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HSEMEN != 0"]
    #[inline] pub fn test_hsemen(&self) -> bool {
        self.hsemen() != 0
    }

    #[doc="Sets the HSEMEN field."]
    #[inline] pub fn set_hsemen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="CPU2 RNGEN"]
    #[inline] pub fn rngen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RNGEN != 0"]
    #[inline] pub fn test_rngen(&self) -> bool {
        self.rngen() != 0
    }

    #[doc="Sets the RNGEN field."]
    #[inline] pub fn set_rngen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="CPU2 AES2EN"]
    #[inline] pub fn aes2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if AES2EN != 0"]
    #[inline] pub fn test_aes2en(&self) -> bool {
        self.aes2en() != 0
    }

    #[doc="Sets the AES2EN field."]
    #[inline] pub fn set_aes2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="CPU2 PKAEN"]
    #[inline] pub fn pkaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PKAEN != 0"]
    #[inline] pub fn test_pkaen(&self) -> bool {
        self.pkaen() != 0
    }

    #[doc="Sets the PKAEN field."]
    #[inline] pub fn set_pkaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for C2ahb3enr {
    #[inline]
    fn from(other: u32) -> Self {
         C2ahb3enr(other)
    }
}

impl ::core::fmt::Display for C2ahb3enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2ahb3enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flashen() != 0 { try!(write!(f, " flashen"))}
        if self.ipccen() != 0 { try!(write!(f, " ipccen"))}
        if self.hsemen() != 0 { try!(write!(f, " hsemen"))}
        if self.rngen() != 0 { try!(write!(f, " rngen"))}
        if self.aes2en() != 0 { try!(write!(f, " aes2en"))}
        if self.pkaen() != 0 { try!(write!(f, " pkaen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 APB1ENR1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb1enr1(pub u32);
impl C2apb1enr1 {
    #[doc="CPU2 Low power timer 1 clock enable"]
    #[inline] pub fn lptim1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPTIM1EN != 0"]
    #[inline] pub fn test_lptim1en(&self) -> bool {
        self.lptim1en() != 0
    }

    #[doc="Sets the LPTIM1EN field."]
    #[inline] pub fn set_lptim1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="CPU2 USB clock enable"]
    #[inline] pub fn usben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if USBEN != 0"]
    #[inline] pub fn test_usben(&self) -> bool {
        self.usben() != 0
    }

    #[doc="Sets the USBEN field."]
    #[inline] pub fn set_usben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CPU2 CRS clock enable"]
    #[inline] pub fn crsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CRSEN != 0"]
    #[inline] pub fn test_crsen(&self) -> bool {
        self.crsen() != 0
    }

    #[doc="Sets the CRSEN field."]
    #[inline] pub fn set_crsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="CPU2 I2C3 clock enable"]
    #[inline] pub fn i2c3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3EN != 0"]
    #[inline] pub fn test_i2c3en(&self) -> bool {
        self.i2c3en() != 0
    }

    #[doc="Sets the I2C3EN field."]
    #[inline] pub fn set_i2c3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="CPU2 I2C1 clock enable"]
    #[inline] pub fn i2c1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1EN != 0"]
    #[inline] pub fn test_i2c1en(&self) -> bool {
        self.i2c1en() != 0
    }

    #[doc="Sets the I2C1EN field."]
    #[inline] pub fn set_i2c1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="CPU2 SPI2 clock enable"]
    #[inline] pub fn spi2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2EN != 0"]
    #[inline] pub fn test_spi2en(&self) -> bool {
        self.spi2en() != 0
    }

    #[doc="Sets the SPI2EN field."]
    #[inline] pub fn set_spi2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CPU2 RTC APB clock enable"]
    #[inline] pub fn rtcapben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RTCAPBEN != 0"]
    #[inline] pub fn test_rtcapben(&self) -> bool {
        self.rtcapben() != 0
    }

    #[doc="Sets the RTCAPBEN field."]
    #[inline] pub fn set_rtcapben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CPU2 LCD clock enable"]
    #[inline] pub fn lcden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LCDEN != 0"]
    #[inline] pub fn test_lcden(&self) -> bool {
        self.lcden() != 0
    }

    #[doc="Sets the LCDEN field."]
    #[inline] pub fn set_lcden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="CPU2 TIM2 timer clock enable"]
    #[inline] pub fn tim2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2EN != 0"]
    #[inline] pub fn test_tim2en(&self) -> bool {
        self.tim2en() != 0
    }

    #[doc="Sets the TIM2EN field."]
    #[inline] pub fn set_tim2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2apb1enr1 {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb1enr1(other)
    }
}

impl ::core::fmt::Display for C2apb1enr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb1enr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim1en() != 0 { try!(write!(f, " lptim1en"))}
        if self.usben() != 0 { try!(write!(f, " usben"))}
        if self.crsen() != 0 { try!(write!(f, " crsen"))}
        if self.i2c3en() != 0 { try!(write!(f, " i2c3en"))}
        if self.i2c1en() != 0 { try!(write!(f, " i2c1en"))}
        if self.spi2en() != 0 { try!(write!(f, " spi2en"))}
        if self.rtcapben() != 0 { try!(write!(f, " rtcapben"))}
        if self.lcden() != 0 { try!(write!(f, " lcden"))}
        if self.tim2en() != 0 { try!(write!(f, " tim2en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 APB1 peripheral clock enable register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb1enr2(pub u32);
impl C2apb1enr2 {
    #[doc="CPU2 LPTIM2EN"]
    #[inline] pub fn lptim2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LPTIM2EN != 0"]
    #[inline] pub fn test_lptim2en(&self) -> bool {
        self.lptim2en() != 0
    }

    #[doc="Sets the LPTIM2EN field."]
    #[inline] pub fn set_lptim2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="CPU2 Low power UART 1 clock enable"]
    #[inline] pub fn lpuart1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPUART1EN != 0"]
    #[inline] pub fn test_lpuart1en(&self) -> bool {
        self.lpuart1en() != 0
    }

    #[doc="Sets the LPUART1EN field."]
    #[inline] pub fn set_lpuart1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2apb1enr2 {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb1enr2(other)
    }
}

impl ::core::fmt::Display for C2apb1enr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb1enr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim2en() != 0 { try!(write!(f, " lptim2en"))}
        if self.lpuart1en() != 0 { try!(write!(f, " lpuart1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 APB2ENR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb2enr(pub u32);
impl C2apb2enr {
    #[doc="CPU2 SAI1 clock enable"]
    #[inline] pub fn sai1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SAI1EN != 0"]
    #[inline] pub fn test_sai1en(&self) -> bool {
        self.sai1en() != 0
    }

    #[doc="Sets the SAI1EN field."]
    #[inline] pub fn set_sai1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="CPU2 TIM17 timer clock enable"]
    #[inline] pub fn tim17en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17EN != 0"]
    #[inline] pub fn test_tim17en(&self) -> bool {
        self.tim17en() != 0
    }

    #[doc="Sets the TIM17EN field."]
    #[inline] pub fn set_tim17en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="CPU2 TIM16 timer clock enable"]
    #[inline] pub fn tim16en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16EN != 0"]
    #[inline] pub fn test_tim16en(&self) -> bool {
        self.tim16en() != 0
    }

    #[doc="Sets the TIM16EN field."]
    #[inline] pub fn set_tim16en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="CPU2 USART1clock enable"]
    #[inline] pub fn usart1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1EN != 0"]
    #[inline] pub fn test_usart1en(&self) -> bool {
        self.usart1en() != 0
    }

    #[doc="Sets the USART1EN field."]
    #[inline] pub fn set_usart1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CPU2 SPI1 clock enable"]
    #[inline] pub fn spi1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1EN != 0"]
    #[inline] pub fn test_spi1en(&self) -> bool {
        self.spi1en() != 0
    }

    #[doc="Sets the SPI1EN field."]
    #[inline] pub fn set_spi1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CPU2 TIM1 timer clock enable"]
    #[inline] pub fn tim1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1EN != 0"]
    #[inline] pub fn test_tim1en(&self) -> bool {
        self.tim1en() != 0
    }

    #[doc="Sets the TIM1EN field."]
    #[inline] pub fn set_tim1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for C2apb2enr {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb2enr(other)
    }
}

impl ::core::fmt::Display for C2apb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sai1en() != 0 { try!(write!(f, " sai1en"))}
        if self.tim17en() != 0 { try!(write!(f, " tim17en"))}
        if self.tim16en() != 0 { try!(write!(f, " tim16en"))}
        if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
        if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
        if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 APB3ENR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb3enr(pub u32);
impl C2apb3enr {
    #[doc="CPU2 802.15.4 interface clock enable"]
    #[inline] pub fn en802(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EN802 != 0"]
    #[inline] pub fn test_en802(&self) -> bool {
        self.en802() != 0
    }

    #[doc="Sets the EN802 field."]
    #[inline] pub fn set_en802<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CPU2 BLE interface clock enable"]
    #[inline] pub fn bleen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BLEEN != 0"]
    #[inline] pub fn test_bleen(&self) -> bool {
        self.bleen() != 0
    }

    #[doc="Sets the BLEEN field."]
    #[inline] pub fn set_bleen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2apb3enr {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb3enr(other)
    }
}

impl ::core::fmt::Display for C2apb3enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb3enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.en802() != 0 { try!(write!(f, " en802"))}
        if self.bleen() != 0 { try!(write!(f, " bleen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 AHB1 peripheral clocks enable in Sleep and Stop modes register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2ahb1smenr(pub u32);
impl C2ahb1smenr {
    #[doc="CPU2 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tscsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSCSMEN != 0"]
    #[inline] pub fn test_tscsmen(&self) -> bool {
        self.tscsmen() != 0
    }

    #[doc="Sets the TSCSMEN field."]
    #[inline] pub fn set_tscsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CPU2 CRCSMEN"]
    #[inline] pub fn crcsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCSMEN != 0"]
    #[inline] pub fn test_crcsmen(&self) -> bool {
        self.crcsmen() != 0
    }

    #[doc="Sets the CRCSMEN field."]
    #[inline] pub fn set_crcsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SRAM1 interface clock enable during CPU1 CSleep mode"]
    #[inline] pub fn sram1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SRAM1SMEN != 0"]
    #[inline] pub fn test_sram1smen(&self) -> bool {
        self.sram1smen() != 0
    }

    #[doc="Sets the SRAM1SMEN field."]
    #[inline] pub fn set_sram1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="CPU2 DMAMUX clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dmamuxsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAMUXSMEN != 0"]
    #[inline] pub fn test_dmamuxsmen(&self) -> bool {
        self.dmamuxsmen() != 0
    }

    #[doc="Sets the DMAMUXSMEN field."]
    #[inline] pub fn set_dmamuxsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CPU2 DMA2 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dma2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2SMEN != 0"]
    #[inline] pub fn test_dma2smen(&self) -> bool {
        self.dma2smen() != 0
    }

    #[doc="Sets the DMA2SMEN field."]
    #[inline] pub fn set_dma2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CPU2 DMA1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dma1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1SMEN != 0"]
    #[inline] pub fn test_dma1smen(&self) -> bool {
        self.dma1smen() != 0
    }

    #[doc="Sets the DMA1SMEN field."]
    #[inline] pub fn set_dma1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2ahb1smenr {
    #[inline]
    fn from(other: u32) -> Self {
         C2ahb1smenr(other)
    }
}

impl ::core::fmt::Display for C2ahb1smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2ahb1smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tscsmen() != 0 { try!(write!(f, " tscsmen"))}
        if self.crcsmen() != 0 { try!(write!(f, " crcsmen"))}
        if self.sram1smen() != 0 { try!(write!(f, " sram1smen"))}
        if self.dmamuxsmen() != 0 { try!(write!(f, " dmamuxsmen"))}
        if self.dma2smen() != 0 { try!(write!(f, " dma2smen"))}
        if self.dma1smen() != 0 { try!(write!(f, " dma1smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2ahb2smenr(pub u32);
impl C2ahb2smenr {
    #[doc="CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
    #[inline] pub fn aes1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AES1SMEN != 0"]
    #[inline] pub fn test_aes1smen(&self) -> bool {
        self.aes1smen() != 0
    }

    #[doc="Sets the AES1SMEN field."]
    #[inline] pub fn set_aes1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CPU2 ADC clocks enable during Sleep and Stop modes"]
    #[inline] pub fn adcfssmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCFSSMEN != 0"]
    #[inline] pub fn test_adcfssmen(&self) -> bool {
        self.adcfssmen() != 0
    }

    #[doc="Sets the ADCFSSMEN field."]
    #[inline] pub fn set_adcfssmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CPU2 IO port H clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiohsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHSMEN != 0"]
    #[inline] pub fn test_gpiohsmen(&self) -> bool {
        self.gpiohsmen() != 0
    }

    #[doc="Sets the GPIOHSMEN field."]
    #[inline] pub fn set_gpiohsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CPU2 IO port E clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpioesmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOESMEN != 0"]
    #[inline] pub fn test_gpioesmen(&self) -> bool {
        self.gpioesmen() != 0
    }

    #[doc="Sets the GPIOESMEN field."]
    #[inline] pub fn set_gpioesmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CPU2 IO port D clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiodsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODSMEN != 0"]
    #[inline] pub fn test_gpiodsmen(&self) -> bool {
        self.gpiodsmen() != 0
    }

    #[doc="Sets the GPIODSMEN field."]
    #[inline] pub fn set_gpiodsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="CPU2 IO port C clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiocsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCSMEN != 0"]
    #[inline] pub fn test_gpiocsmen(&self) -> bool {
        self.gpiocsmen() != 0
    }

    #[doc="Sets the GPIOCSMEN field."]
    #[inline] pub fn set_gpiocsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CPU2 IO port B clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiobsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBSMEN != 0"]
    #[inline] pub fn test_gpiobsmen(&self) -> bool {
        self.gpiobsmen() != 0
    }

    #[doc="Sets the GPIOBSMEN field."]
    #[inline] pub fn set_gpiobsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CPU2 IO port A clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpioasmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOASMEN != 0"]
    #[inline] pub fn test_gpioasmen(&self) -> bool {
        self.gpioasmen() != 0
    }

    #[doc="Sets the GPIOASMEN field."]
    #[inline] pub fn set_gpioasmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2ahb2smenr {
    #[inline]
    fn from(other: u32) -> Self {
         C2ahb2smenr(other)
    }
}

impl ::core::fmt::Display for C2ahb2smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2ahb2smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aes1smen() != 0 { try!(write!(f, " aes1smen"))}
        if self.adcfssmen() != 0 { try!(write!(f, " adcfssmen"))}
        if self.gpiohsmen() != 0 { try!(write!(f, " gpiohsmen"))}
        if self.gpioesmen() != 0 { try!(write!(f, " gpioesmen"))}
        if self.gpiodsmen() != 0 { try!(write!(f, " gpiodsmen"))}
        if self.gpiocsmen() != 0 { try!(write!(f, " gpiocsmen"))}
        if self.gpiobsmen() != 0 { try!(write!(f, " gpiobsmen"))}
        if self.gpioasmen() != 0 { try!(write!(f, " gpioasmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2ahb3smenr(pub u32);
impl C2ahb3smenr {
    #[doc="Flash interface clocks enable during CPU2 sleep modes"]
    #[inline] pub fn flashsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FLASHSMEN != 0"]
    #[inline] pub fn test_flashsmen(&self) -> bool {
        self.flashsmen() != 0
    }

    #[doc="Sets the FLASHSMEN field."]
    #[inline] pub fn set_flashsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
    #[inline] pub fn sram2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SRAM2SMEN != 0"]
    #[inline] pub fn test_sram2smen(&self) -> bool {
        self.sram2smen() != 0
    }

    #[doc="Sets the SRAM2SMEN field."]
    #[inline] pub fn set_sram2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="True RNG clocks enable during CPU2 sleep modes"]
    #[inline] pub fn rngsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RNGSMEN != 0"]
    #[inline] pub fn test_rngsmen(&self) -> bool {
        self.rngsmen() != 0
    }

    #[doc="Sets the RNGSMEN field."]
    #[inline] pub fn set_rngsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="AES2 accelerator clocks enable during CPU2 sleep modes"]
    #[inline] pub fn aes2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if AES2SMEN != 0"]
    #[inline] pub fn test_aes2smen(&self) -> bool {
        self.aes2smen() != 0
    }

    #[doc="Sets the AES2SMEN field."]
    #[inline] pub fn set_aes2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PKA accelerator clocks enable during CPU2 sleep modes"]
    #[inline] pub fn pkasmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PKASMEN != 0"]
    #[inline] pub fn test_pkasmen(&self) -> bool {
        self.pkasmen() != 0
    }

    #[doc="Sets the PKASMEN field."]
    #[inline] pub fn set_pkasmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for C2ahb3smenr {
    #[inline]
    fn from(other: u32) -> Self {
         C2ahb3smenr(other)
    }
}

impl ::core::fmt::Display for C2ahb3smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2ahb3smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flashsmen() != 0 { try!(write!(f, " flashsmen"))}
        if self.sram2smen() != 0 { try!(write!(f, " sram2smen"))}
        if self.rngsmen() != 0 { try!(write!(f, " rngsmen"))}
        if self.aes2smen() != 0 { try!(write!(f, " aes2smen"))}
        if self.pkasmen() != 0 { try!(write!(f, " pkasmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 APB1SMENR1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb1smenr1(pub u32);
impl C2apb1smenr1 {
    #[doc="Low power timer 1 clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn lptim1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPTIM1SMEN != 0"]
    #[inline] pub fn test_lptim1smen(&self) -> bool {
        self.lptim1smen() != 0
    }

    #[doc="Sets the LPTIM1SMEN field."]
    #[inline] pub fn set_lptim1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="USB FS clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn usbsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if USBSMEN != 0"]
    #[inline] pub fn test_usbsmen(&self) -> bool {
        self.usbsmen() != 0
    }

    #[doc="Sets the USBSMEN field."]
    #[inline] pub fn set_usbsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CRS clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn crsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CRSMEN != 0"]
    #[inline] pub fn test_crsmen(&self) -> bool {
        self.crsmen() != 0
    }

    #[doc="Sets the CRSMEN field."]
    #[inline] pub fn set_crsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="I2C3 clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn i2c3smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3SMEN != 0"]
    #[inline] pub fn test_i2c3smen(&self) -> bool {
        self.i2c3smen() != 0
    }

    #[doc="Sets the I2C3SMEN field."]
    #[inline] pub fn set_i2c3smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C1 clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn i2c1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1SMEN != 0"]
    #[inline] pub fn test_i2c1smen(&self) -> bool {
        self.i2c1smen() != 0
    }

    #[doc="Sets the I2C1SMEN field."]
    #[inline] pub fn set_i2c1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SPI2 clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn spi2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2SMEN != 0"]
    #[inline] pub fn test_spi2smen(&self) -> bool {
        self.spi2smen() != 0
    }

    #[doc="Sets the SPI2SMEN field."]
    #[inline] pub fn set_spi2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="RTC APB clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn rtcapbsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RTCAPBSMEN != 0"]
    #[inline] pub fn test_rtcapbsmen(&self) -> bool {
        self.rtcapbsmen() != 0
    }

    #[doc="Sets the RTCAPBSMEN field."]
    #[inline] pub fn set_rtcapbsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="LCD clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn lcdsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LCDSMEN != 0"]
    #[inline] pub fn test_lcdsmen(&self) -> bool {
        self.lcdsmen() != 0
    }

    #[doc="Sets the LCDSMEN field."]
    #[inline] pub fn set_lcdsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TIM2 timer clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn tim2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2SMEN != 0"]
    #[inline] pub fn test_tim2smen(&self) -> bool {
        self.tim2smen() != 0
    }

    #[doc="Sets the TIM2SMEN field."]
    #[inline] pub fn set_tim2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2apb1smenr1 {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb1smenr1(other)
    }
}

impl ::core::fmt::Display for C2apb1smenr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb1smenr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim1smen() != 0 { try!(write!(f, " lptim1smen"))}
        if self.usbsmen() != 0 { try!(write!(f, " usbsmen"))}
        if self.crsmen() != 0 { try!(write!(f, " crsmen"))}
        if self.i2c3smen() != 0 { try!(write!(f, " i2c3smen"))}
        if self.i2c1smen() != 0 { try!(write!(f, " i2c1smen"))}
        if self.spi2smen() != 0 { try!(write!(f, " spi2smen"))}
        if self.rtcapbsmen() != 0 { try!(write!(f, " rtcapbsmen"))}
        if self.lcdsmen() != 0 { try!(write!(f, " lcdsmen"))}
        if self.tim2smen() != 0 { try!(write!(f, " tim2smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb1smenr2(pub u32);
impl C2apb1smenr2 {
    #[doc="Low power timer 2 clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn lptim2smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LPTIM2SMEN != 0"]
    #[inline] pub fn test_lptim2smen(&self) -> bool {
        self.lptim2smen() != 0
    }

    #[doc="Sets the LPTIM2SMEN field."]
    #[inline] pub fn set_lptim2smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Low power UART 1 clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn lpuart1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPUART1SMEN != 0"]
    #[inline] pub fn test_lpuart1smen(&self) -> bool {
        self.lpuart1smen() != 0
    }

    #[doc="Sets the LPUART1SMEN field."]
    #[inline] pub fn set_lpuart1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2apb1smenr2 {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb1smenr2(other)
    }
}

impl ::core::fmt::Display for C2apb1smenr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb1smenr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim2smen() != 0 { try!(write!(f, " lptim2smen"))}
        if self.lpuart1smen() != 0 { try!(write!(f, " lpuart1smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 APB2SMENR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb2smenr(pub u32);
impl C2apb2smenr {
    #[doc="SAI1 clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn sai1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SAI1SMEN != 0"]
    #[inline] pub fn test_sai1smen(&self) -> bool {
        self.sai1smen() != 0
    }

    #[doc="Sets the SAI1SMEN field."]
    #[inline] pub fn set_sai1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="TIM17 timer clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn tim17smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17SMEN != 0"]
    #[inline] pub fn test_tim17smen(&self) -> bool {
        self.tim17smen() != 0
    }

    #[doc="Sets the TIM17SMEN field."]
    #[inline] pub fn set_tim17smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM16 timer clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn tim16smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16SMEN != 0"]
    #[inline] pub fn test_tim16smen(&self) -> bool {
        self.tim16smen() != 0
    }

    #[doc="Sets the TIM16SMEN field."]
    #[inline] pub fn set_tim16smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="USART1clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn usart1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1SMEN != 0"]
    #[inline] pub fn test_usart1smen(&self) -> bool {
        self.usart1smen() != 0
    }

    #[doc="Sets the USART1SMEN field."]
    #[inline] pub fn set_usart1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI1 clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn spi1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1SMEN != 0"]
    #[inline] pub fn test_spi1smen(&self) -> bool {
        self.spi1smen() != 0
    }

    #[doc="Sets the SPI1SMEN field."]
    #[inline] pub fn set_spi1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM1 timer clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn tim1smen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1SMEN != 0"]
    #[inline] pub fn test_tim1smen(&self) -> bool {
        self.tim1smen() != 0
    }

    #[doc="Sets the TIM1SMEN field."]
    #[inline] pub fn set_tim1smen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for C2apb2smenr {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb2smenr(other)
    }
}

impl ::core::fmt::Display for C2apb2smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb2smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sai1smen() != 0 { try!(write!(f, " sai1smen"))}
        if self.tim17smen() != 0 { try!(write!(f, " tim17smen"))}
        if self.tim16smen() != 0 { try!(write!(f, " tim16smen"))}
        if self.usart1smen() != 0 { try!(write!(f, " usart1smen"))}
        if self.spi1smen() != 0 { try!(write!(f, " spi1smen"))}
        if self.tim1smen() != 0 { try!(write!(f, " tim1smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 APB3SMENR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb3smenr(pub u32);
impl C2apb3smenr {
    #[doc="802.15.4 interface clocks enable during CPU2 Sleep modes"]
    #[inline] pub fn smen802(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SMEN802 != 0"]
    #[inline] pub fn test_smen802(&self) -> bool {
        self.smen802() != 0
    }

    #[doc="Sets the SMEN802 field."]
    #[inline] pub fn set_smen802<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BLE interface clocks enable during CPU2 Sleep mode"]
    #[inline] pub fn blesmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BLESMEN != 0"]
    #[inline] pub fn test_blesmen(&self) -> bool {
        self.blesmen() != 0
    }

    #[doc="Sets the BLESMEN field."]
    #[inline] pub fn set_blesmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2apb3smenr {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb3smenr(other)
    }
}

impl ::core::fmt::Display for C2apb3smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb3smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.smen802() != 0 { try!(write!(f, " smen802"))}
        if self.blesmen() != 0 { try!(write!(f, " blesmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

