
::bobbin_mcu::periph!( RCC, Rcc, RCC_PERIPH, RccPeriph, RCC_OWNED, RCC_REF_COUNT, 0x40023800, 0x00, 0x00);


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

    #[doc="Get the PLLCFGR Register."]
    #[inline] pub fn pllcfgr_reg(&self) -> ::bobbin_mcu::register::Register<Pllcfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pllcfgr, 0x4)
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

    #[doc="Get the CIR Register."]
    #[inline] pub fn cir_reg(&self) -> ::bobbin_mcu::register::Register<Cir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cir, 0xc)
    }

    #[doc="Get the *mut pointer for the CIR register."]
    #[inline] pub fn cir_mut(&self) -> *mut Cir { 
        self.cir_reg().ptr()
    }

    #[doc="Get the *const pointer for the CIR register."]
    #[inline] pub fn cir_ptr(&self) -> *const Cir { 
        self.cir_reg().ptr()
    }

    #[doc="Read the CIR register."]
    #[inline] pub fn cir(&self) -> Cir { 
        self.cir_reg().read()
    }

    #[doc="Write the CIR register."]
    #[inline] pub fn write_cir(&self, value: Cir) -> &Self { 
        self.cir_reg().write(value);
        self
    }

    #[doc="Set the CIR register."]
    #[inline] pub fn set_cir<F: FnOnce(Cir) -> Cir>(&self, f: F) -> &Self {
        self.cir_reg().set(f);
        self
    }

    #[doc="Modify the CIR register."]
    #[inline] pub fn with_cir<F: FnOnce(Cir) -> Cir>(&self, f: F) -> &Self {
        self.cir_reg().with(f);
        self
    }

    #[doc="Get the AHB1RSTR Register."]
    #[inline] pub fn ahb1rstr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb1rstr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb1rstr, 0x10)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb2rstr, 0x14)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb3rstr, 0x18)
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

    #[doc="Get the APB1RSTR Register."]
    #[inline] pub fn apb1rstr_reg(&self) -> ::bobbin_mcu::register::Register<Apb1rstr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1rstr, 0x20)
    }

    #[doc="Get the *mut pointer for the APB1RSTR register."]
    #[inline] pub fn apb1rstr_mut(&self) -> *mut Apb1rstr { 
        self.apb1rstr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1RSTR register."]
    #[inline] pub fn apb1rstr_ptr(&self) -> *const Apb1rstr { 
        self.apb1rstr_reg().ptr()
    }

    #[doc="Read the APB1RSTR register."]
    #[inline] pub fn apb1rstr(&self) -> Apb1rstr { 
        self.apb1rstr_reg().read()
    }

    #[doc="Write the APB1RSTR register."]
    #[inline] pub fn write_apb1rstr(&self, value: Apb1rstr) -> &Self { 
        self.apb1rstr_reg().write(value);
        self
    }

    #[doc="Set the APB1RSTR register."]
    #[inline] pub fn set_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Self {
        self.apb1rstr_reg().set(f);
        self
    }

    #[doc="Modify the APB1RSTR register."]
    #[inline] pub fn with_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Self {
        self.apb1rstr_reg().with(f);
        self
    }

    #[doc="Get the APB2RSTR Register."]
    #[inline] pub fn apb2rstr_reg(&self) -> ::bobbin_mcu::register::Register<Apb2rstr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb2rstr, 0x24)
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

    #[doc="Get the AHB1ENR Register."]
    #[inline] pub fn ahb1enr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb1enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb1enr, 0x30)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb2enr, 0x34)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb3enr, 0x38)
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

    #[doc="Get the APB1ENR Register."]
    #[inline] pub fn apb1enr_reg(&self) -> ::bobbin_mcu::register::Register<Apb1enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1enr, 0x40)
    }

    #[doc="Get the *mut pointer for the APB1ENR register."]
    #[inline] pub fn apb1enr_mut(&self) -> *mut Apb1enr { 
        self.apb1enr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1ENR register."]
    #[inline] pub fn apb1enr_ptr(&self) -> *const Apb1enr { 
        self.apb1enr_reg().ptr()
    }

    #[doc="Read the APB1ENR register."]
    #[inline] pub fn apb1enr(&self) -> Apb1enr { 
        self.apb1enr_reg().read()
    }

    #[doc="Write the APB1ENR register."]
    #[inline] pub fn write_apb1enr(&self, value: Apb1enr) -> &Self { 
        self.apb1enr_reg().write(value);
        self
    }

    #[doc="Set the APB1ENR register."]
    #[inline] pub fn set_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Self {
        self.apb1enr_reg().set(f);
        self
    }

    #[doc="Modify the APB1ENR register."]
    #[inline] pub fn with_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Self {
        self.apb1enr_reg().with(f);
        self
    }

    #[doc="Get the APB2ENR Register."]
    #[inline] pub fn apb2enr_reg(&self) -> ::bobbin_mcu::register::Register<Apb2enr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb2enr, 0x44)
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

    #[doc="Get the AHB1LPENR Register."]
    #[inline] pub fn ahb1lpenr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb1lpenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb1lpenr, 0x50)
    }

    #[doc="Get the *mut pointer for the AHB1LPENR register."]
    #[inline] pub fn ahb1lpenr_mut(&self) -> *mut Ahb1lpenr { 
        self.ahb1lpenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB1LPENR register."]
    #[inline] pub fn ahb1lpenr_ptr(&self) -> *const Ahb1lpenr { 
        self.ahb1lpenr_reg().ptr()
    }

    #[doc="Read the AHB1LPENR register."]
    #[inline] pub fn ahb1lpenr(&self) -> Ahb1lpenr { 
        self.ahb1lpenr_reg().read()
    }

    #[doc="Write the AHB1LPENR register."]
    #[inline] pub fn write_ahb1lpenr(&self, value: Ahb1lpenr) -> &Self { 
        self.ahb1lpenr_reg().write(value);
        self
    }

    #[doc="Set the AHB1LPENR register."]
    #[inline] pub fn set_ahb1lpenr<F: FnOnce(Ahb1lpenr) -> Ahb1lpenr>(&self, f: F) -> &Self {
        self.ahb1lpenr_reg().set(f);
        self
    }

    #[doc="Modify the AHB1LPENR register."]
    #[inline] pub fn with_ahb1lpenr<F: FnOnce(Ahb1lpenr) -> Ahb1lpenr>(&self, f: F) -> &Self {
        self.ahb1lpenr_reg().with(f);
        self
    }

    #[doc="Get the AHB2LPENR Register."]
    #[inline] pub fn ahb2lpenr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb2lpenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb2lpenr, 0x54)
    }

    #[doc="Get the *mut pointer for the AHB2LPENR register."]
    #[inline] pub fn ahb2lpenr_mut(&self) -> *mut Ahb2lpenr { 
        self.ahb2lpenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB2LPENR register."]
    #[inline] pub fn ahb2lpenr_ptr(&self) -> *const Ahb2lpenr { 
        self.ahb2lpenr_reg().ptr()
    }

    #[doc="Read the AHB2LPENR register."]
    #[inline] pub fn ahb2lpenr(&self) -> Ahb2lpenr { 
        self.ahb2lpenr_reg().read()
    }

    #[doc="Write the AHB2LPENR register."]
    #[inline] pub fn write_ahb2lpenr(&self, value: Ahb2lpenr) -> &Self { 
        self.ahb2lpenr_reg().write(value);
        self
    }

    #[doc="Set the AHB2LPENR register."]
    #[inline] pub fn set_ahb2lpenr<F: FnOnce(Ahb2lpenr) -> Ahb2lpenr>(&self, f: F) -> &Self {
        self.ahb2lpenr_reg().set(f);
        self
    }

    #[doc="Modify the AHB2LPENR register."]
    #[inline] pub fn with_ahb2lpenr<F: FnOnce(Ahb2lpenr) -> Ahb2lpenr>(&self, f: F) -> &Self {
        self.ahb2lpenr_reg().with(f);
        self
    }

    #[doc="Get the AHB3LPENR Register."]
    #[inline] pub fn ahb3lpenr_reg(&self) -> ::bobbin_mcu::register::Register<Ahb3lpenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ahb3lpenr, 0x58)
    }

    #[doc="Get the *mut pointer for the AHB3LPENR register."]
    #[inline] pub fn ahb3lpenr_mut(&self) -> *mut Ahb3lpenr { 
        self.ahb3lpenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AHB3LPENR register."]
    #[inline] pub fn ahb3lpenr_ptr(&self) -> *const Ahb3lpenr { 
        self.ahb3lpenr_reg().ptr()
    }

    #[doc="Read the AHB3LPENR register."]
    #[inline] pub fn ahb3lpenr(&self) -> Ahb3lpenr { 
        self.ahb3lpenr_reg().read()
    }

    #[doc="Write the AHB3LPENR register."]
    #[inline] pub fn write_ahb3lpenr(&self, value: Ahb3lpenr) -> &Self { 
        self.ahb3lpenr_reg().write(value);
        self
    }

    #[doc="Set the AHB3LPENR register."]
    #[inline] pub fn set_ahb3lpenr<F: FnOnce(Ahb3lpenr) -> Ahb3lpenr>(&self, f: F) -> &Self {
        self.ahb3lpenr_reg().set(f);
        self
    }

    #[doc="Modify the AHB3LPENR register."]
    #[inline] pub fn with_ahb3lpenr<F: FnOnce(Ahb3lpenr) -> Ahb3lpenr>(&self, f: F) -> &Self {
        self.ahb3lpenr_reg().with(f);
        self
    }

    #[doc="Get the APB1LPENR Register."]
    #[inline] pub fn apb1lpenr_reg(&self) -> ::bobbin_mcu::register::Register<Apb1lpenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1lpenr, 0x60)
    }

    #[doc="Get the *mut pointer for the APB1LPENR register."]
    #[inline] pub fn apb1lpenr_mut(&self) -> *mut Apb1lpenr { 
        self.apb1lpenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1LPENR register."]
    #[inline] pub fn apb1lpenr_ptr(&self) -> *const Apb1lpenr { 
        self.apb1lpenr_reg().ptr()
    }

    #[doc="Read the APB1LPENR register."]
    #[inline] pub fn apb1lpenr(&self) -> Apb1lpenr { 
        self.apb1lpenr_reg().read()
    }

    #[doc="Write the APB1LPENR register."]
    #[inline] pub fn write_apb1lpenr(&self, value: Apb1lpenr) -> &Self { 
        self.apb1lpenr_reg().write(value);
        self
    }

    #[doc="Set the APB1LPENR register."]
    #[inline] pub fn set_apb1lpenr<F: FnOnce(Apb1lpenr) -> Apb1lpenr>(&self, f: F) -> &Self {
        self.apb1lpenr_reg().set(f);
        self
    }

    #[doc="Modify the APB1LPENR register."]
    #[inline] pub fn with_apb1lpenr<F: FnOnce(Apb1lpenr) -> Apb1lpenr>(&self, f: F) -> &Self {
        self.apb1lpenr_reg().with(f);
        self
    }

    #[doc="Get the APB2LPENR Register."]
    #[inline] pub fn apb2lpenr_reg(&self) -> ::bobbin_mcu::register::Register<Apb2lpenr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb2lpenr, 0x64)
    }

    #[doc="Get the *mut pointer for the APB2LPENR register."]
    #[inline] pub fn apb2lpenr_mut(&self) -> *mut Apb2lpenr { 
        self.apb2lpenr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB2LPENR register."]
    #[inline] pub fn apb2lpenr_ptr(&self) -> *const Apb2lpenr { 
        self.apb2lpenr_reg().ptr()
    }

    #[doc="Read the APB2LPENR register."]
    #[inline] pub fn apb2lpenr(&self) -> Apb2lpenr { 
        self.apb2lpenr_reg().read()
    }

    #[doc="Write the APB2LPENR register."]
    #[inline] pub fn write_apb2lpenr(&self, value: Apb2lpenr) -> &Self { 
        self.apb2lpenr_reg().write(value);
        self
    }

    #[doc="Set the APB2LPENR register."]
    #[inline] pub fn set_apb2lpenr<F: FnOnce(Apb2lpenr) -> Apb2lpenr>(&self, f: F) -> &Self {
        self.apb2lpenr_reg().set(f);
        self
    }

    #[doc="Modify the APB2LPENR register."]
    #[inline] pub fn with_apb2lpenr<F: FnOnce(Apb2lpenr) -> Apb2lpenr>(&self, f: F) -> &Self {
        self.apb2lpenr_reg().with(f);
        self
    }

    #[doc="Get the BDCR Register."]
    #[inline] pub fn bdcr_reg(&self) -> ::bobbin_mcu::register::Register<Bdcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdcr, 0x70)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr, 0x74)
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

    #[doc="Get the SSCGR Register."]
    #[inline] pub fn sscgr_reg(&self) -> ::bobbin_mcu::register::Register<Sscgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sscgr, 0x80)
    }

    #[doc="Get the *mut pointer for the SSCGR register."]
    #[inline] pub fn sscgr_mut(&self) -> *mut Sscgr { 
        self.sscgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SSCGR register."]
    #[inline] pub fn sscgr_ptr(&self) -> *const Sscgr { 
        self.sscgr_reg().ptr()
    }

    #[doc="Read the SSCGR register."]
    #[inline] pub fn sscgr(&self) -> Sscgr { 
        self.sscgr_reg().read()
    }

    #[doc="Write the SSCGR register."]
    #[inline] pub fn write_sscgr(&self, value: Sscgr) -> &Self { 
        self.sscgr_reg().write(value);
        self
    }

    #[doc="Set the SSCGR register."]
    #[inline] pub fn set_sscgr<F: FnOnce(Sscgr) -> Sscgr>(&self, f: F) -> &Self {
        self.sscgr_reg().set(f);
        self
    }

    #[doc="Modify the SSCGR register."]
    #[inline] pub fn with_sscgr<F: FnOnce(Sscgr) -> Sscgr>(&self, f: F) -> &Self {
        self.sscgr_reg().with(f);
        self
    }

    #[doc="Get the PLLI2SCFGR Register."]
    #[inline] pub fn plli2scfgr_reg(&self) -> ::bobbin_mcu::register::Register<Plli2scfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Plli2scfgr, 0x84)
    }

    #[doc="Get the *mut pointer for the PLLI2SCFGR register."]
    #[inline] pub fn plli2scfgr_mut(&self) -> *mut Plli2scfgr { 
        self.plli2scfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PLLI2SCFGR register."]
    #[inline] pub fn plli2scfgr_ptr(&self) -> *const Plli2scfgr { 
        self.plli2scfgr_reg().ptr()
    }

    #[doc="Read the PLLI2SCFGR register."]
    #[inline] pub fn plli2scfgr(&self) -> Plli2scfgr { 
        self.plli2scfgr_reg().read()
    }

    #[doc="Write the PLLI2SCFGR register."]
    #[inline] pub fn write_plli2scfgr(&self, value: Plli2scfgr) -> &Self { 
        self.plli2scfgr_reg().write(value);
        self
    }

    #[doc="Set the PLLI2SCFGR register."]
    #[inline] pub fn set_plli2scfgr<F: FnOnce(Plli2scfgr) -> Plli2scfgr>(&self, f: F) -> &Self {
        self.plli2scfgr_reg().set(f);
        self
    }

    #[doc="Modify the PLLI2SCFGR register."]
    #[inline] pub fn with_plli2scfgr<F: FnOnce(Plli2scfgr) -> Plli2scfgr>(&self, f: F) -> &Self {
        self.plli2scfgr_reg().with(f);
        self
    }

}

#[doc="clock control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="PLLI2S clock ready flag"]
    #[inline] pub fn plli2srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PLLI2SRDY != 0"]
    #[inline] pub fn test_plli2srdy(&self) -> bool {
        self.plli2srdy() != 0
    }

    #[doc="Sets the PLLI2SRDY field."]
    #[inline] pub fn set_plli2srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="PLLI2S enable"]
    #[inline] pub fn plli2son(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PLLI2SON != 0"]
    #[inline] pub fn test_plli2son(&self) -> bool {
        self.plli2son() != 0
    }

    #[doc="Sets the PLLI2SON field."]
    #[inline] pub fn set_plli2son<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Main PLL (PLL) clock ready flag"]
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

    #[doc="Main PLL (PLL) enable"]
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

    #[doc="Clock security system enable"]
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

    #[doc="HSE clock bypass"]
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

    #[doc="HSE clock enable"]
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

    #[doc="Internal high-speed clock calibration"]
    #[inline] pub fn hsical(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if HSICAL != 0"]
    #[inline] pub fn test_hsical(&self) -> bool {
        self.hsical() != 0
    }

    #[doc="Sets the HSICAL field."]
    #[inline] pub fn set_hsical<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Internal high-speed clock trimming"]
    #[inline] pub fn hsitrim(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if HSITRIM != 0"]
    #[inline] pub fn test_hsitrim(&self) -> bool {
        self.hsitrim() != 0
    }

    #[doc="Sets the HSITRIM field."]
    #[inline] pub fn set_hsitrim<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Internal high-speed clock ready flag"]
    #[inline] pub fn hsirdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HSIRDY != 0"]
    #[inline] pub fn test_hsirdy(&self) -> bool {
        self.hsirdy() != 0
    }

    #[doc="Sets the HSIRDY field."]
    #[inline] pub fn set_hsirdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Internal high-speed clock enable"]
    #[inline] pub fn hsion(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HSION != 0"]
    #[inline] pub fn test_hsion(&self) -> bool {
        self.hsion() != 0
    }

    #[doc="Sets the HSION field."]
    #[inline] pub fn set_hsion<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.plli2srdy() != 0 { try!(write!(f, " plli2srdy"))}
        if self.plli2son() != 0 { try!(write!(f, " plli2son"))}
        if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
        if self.pllon() != 0 { try!(write!(f, " pllon"))}
        if self.csson() != 0 { try!(write!(f, " csson"))}
        if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
        if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
        if self.hseon() != 0 { try!(write!(f, " hseon"))}
        if self.hsical() != 0 { try!(write!(f, " hsical=0x{:x}", self.hsical()))}
        if self.hsitrim() != 0 { try!(write!(f, " hsitrim=0x{:x}", self.hsitrim()))}
        if self.hsirdy() != 0 { try!(write!(f, " hsirdy"))}
        if self.hsion() != 0 { try!(write!(f, " hsion"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PLL configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pllcfgr(pub u32);
impl Pllcfgr {
    #[doc="Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline] pub fn pllq(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if PLLQ != 0"]
    #[inline] pub fn test_pllq(&self) -> bool {
        self.pllq() != 0
    }

    #[doc="Sets the PLLQ field."]
    #[inline] pub fn set_pllq<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline] pub fn pllsrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PLLSRC != 0"]
    #[inline] pub fn test_pllsrc(&self) -> bool {
        self.pllsrc() != 0
    }

    #[doc="Sets the PLLSRC field."]
    #[inline] pub fn set_pllsrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Main PLL (PLL) division factor for main system clock"]
    #[inline] pub fn pllp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PLLP != 0"]
    #[inline] pub fn test_pllp(&self) -> bool {
        self.pllp() != 0
    }

    #[doc="Sets the PLLP field."]
    #[inline] pub fn set_pllp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Main PLL (PLL) multiplication factor for VCO"]
    #[inline] pub fn plln(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1ff) as u16) } // [14:6]
    }

    #[doc="Returns true if PLLN != 0"]
    #[inline] pub fn test_plln(&self) -> bool {
        self.plln() != 0
    }

    #[doc="Sets the PLLN field."]
    #[inline] pub fn set_plln<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline] pub fn pllm(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if PLLM != 0"]
    #[inline] pub fn test_pllm(&self) -> bool {
        self.pllm() != 0
    }

    #[doc="Sets the PLLM field."]
    #[inline] pub fn set_pllm<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
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
        if self.pllq() != 0 { try!(write!(f, " pllq=0x{:x}", self.pllq()))}
        if self.pllsrc() != 0 { try!(write!(f, " pllsrc"))}
        if self.pllp() != 0 { try!(write!(f, " pllp=0x{:x}", self.pllp()))}
        if self.plln() != 0 { try!(write!(f, " plln=0x{:x}", self.plln()))}
        if self.pllm() != 0 { try!(write!(f, " pllm=0x{:x}", self.pllm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="clock configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc="Microcontroller clock output 2"]
    #[inline] pub fn mco2(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if MCO2 != 0"]
    #[inline] pub fn test_mco2(&self) -> bool {
        self.mco2() != 0
    }

    #[doc="Sets the MCO2 field."]
    #[inline] pub fn set_mco2<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="MCO2 prescaler"]
    #[inline] pub fn mco2pre(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if MCO2PRE != 0"]
    #[inline] pub fn test_mco2pre(&self) -> bool {
        self.mco2pre() != 0
    }

    #[doc="Sets the MCO2PRE field."]
    #[inline] pub fn set_mco2pre<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="MCO1 prescaler"]
    #[inline] pub fn mco1pre(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if MCO1PRE != 0"]
    #[inline] pub fn test_mco1pre(&self) -> bool {
        self.mco1pre() != 0
    }

    #[doc="Sets the MCO1PRE field."]
    #[inline] pub fn set_mco1pre<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="I2S clock selection"]
    #[inline] pub fn i2ssrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2SSRC != 0"]
    #[inline] pub fn test_i2ssrc(&self) -> bool {
        self.i2ssrc() != 0
    }

    #[doc="Sets the I2SSRC field."]
    #[inline] pub fn set_i2ssrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Microcontroller clock output 1"]
    #[inline] pub fn mco1(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if MCO1 != 0"]
    #[inline] pub fn test_mco1(&self) -> bool {
        self.mco1() != 0
    }

    #[doc="Sets the MCO1 field."]
    #[inline] pub fn set_mco1<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="HSE division factor for RTC clock"]
    #[inline] pub fn rtcpre(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if RTCPRE != 0"]
    #[inline] pub fn test_rtcpre(&self) -> bool {
        self.rtcpre() != 0
    }

    #[doc="Sets the RTCPRE field."]
    #[inline] pub fn set_rtcpre<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="APB high-speed prescaler (APB2)"]
    #[inline] pub fn ppre2(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if PPRE2 != 0"]
    #[inline] pub fn test_ppre2(&self) -> bool {
        self.ppre2() != 0
    }

    #[doc="Sets the PPRE2 field."]
    #[inline] pub fn set_ppre2<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="APB Low speed prescaler (APB1)"]
    #[inline] pub fn ppre1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x7) as u8) } // [12:10]
    }

    #[doc="Returns true if PPRE1 != 0"]
    #[inline] pub fn test_ppre1(&self) -> bool {
        self.ppre1() != 0
    }

    #[doc="Sets the PPRE1 field."]
    #[inline] pub fn set_ppre1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 10);
        self.0 |= value << 10;
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
        if self.mco2() != 0 { try!(write!(f, " mco2=0x{:x}", self.mco2()))}
        if self.mco2pre() != 0 { try!(write!(f, " mco2pre=0x{:x}", self.mco2pre()))}
        if self.mco1pre() != 0 { try!(write!(f, " mco1pre=0x{:x}", self.mco1pre()))}
        if self.i2ssrc() != 0 { try!(write!(f, " i2ssrc"))}
        if self.mco1() != 0 { try!(write!(f, " mco1=0x{:x}", self.mco1()))}
        if self.rtcpre() != 0 { try!(write!(f, " rtcpre=0x{:x}", self.rtcpre()))}
        if self.ppre2() != 0 { try!(write!(f, " ppre2=0x{:x}", self.ppre2()))}
        if self.ppre1() != 0 { try!(write!(f, " ppre1=0x{:x}", self.ppre1()))}
        if self.hpre() != 0 { try!(write!(f, " hpre=0x{:x}", self.hpre()))}
        if self.sws() != 0 { try!(write!(f, " sws=0x{:x}", self.sws()))}
        if self.sw() != 0 { try!(write!(f, " sw=0x{:x}", self.sw()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="clock interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cir(pub u32);
impl Cir {
    #[doc="Clock security system interrupt clear"]
    #[inline] pub fn cssc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if CSSC != 0"]
    #[inline] pub fn test_cssc(&self) -> bool {
        self.cssc() != 0
    }

    #[doc="Sets the CSSC field."]
    #[inline] pub fn set_cssc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="PLLSAI Ready Interrupt Clear"]
    #[inline] pub fn pllsairdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PLLSAIRDYC != 0"]
    #[inline] pub fn test_pllsairdyc(&self) -> bool {
        self.pllsairdyc() != 0
    }

    #[doc="Sets the PLLSAIRDYC field."]
    #[inline] pub fn set_pllsairdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="PLLI2S ready interrupt clear"]
    #[inline] pub fn plli2srdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PLLI2SRDYC != 0"]
    #[inline] pub fn test_plli2srdyc(&self) -> bool {
        self.plli2srdyc() != 0
    }

    #[doc="Sets the PLLI2SRDYC field."]
    #[inline] pub fn set_plli2srdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Main PLL(PLL) ready interrupt clear"]
    #[inline] pub fn pllrdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PLLRDYC != 0"]
    #[inline] pub fn test_pllrdyc(&self) -> bool {
        self.pllrdyc() != 0
    }

    #[doc="Sets the PLLRDYC field."]
    #[inline] pub fn set_pllrdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="HSE ready interrupt clear"]
    #[inline] pub fn hserdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HSERDYC != 0"]
    #[inline] pub fn test_hserdyc(&self) -> bool {
        self.hserdyc() != 0
    }

    #[doc="Sets the HSERDYC field."]
    #[inline] pub fn set_hserdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="HSI ready interrupt clear"]
    #[inline] pub fn hsirdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if HSIRDYC != 0"]
    #[inline] pub fn test_hsirdyc(&self) -> bool {
        self.hsirdyc() != 0
    }

    #[doc="Sets the HSIRDYC field."]
    #[inline] pub fn set_hsirdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="LSE ready interrupt clear"]
    #[inline] pub fn lserdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSERDYC != 0"]
    #[inline] pub fn test_lserdyc(&self) -> bool {
        self.lserdyc() != 0
    }

    #[doc="Sets the LSERDYC field."]
    #[inline] pub fn set_lserdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="LSI ready interrupt clear"]
    #[inline] pub fn lsirdyc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if LSIRDYC != 0"]
    #[inline] pub fn test_lsirdyc(&self) -> bool {
        self.lsirdyc() != 0
    }

    #[doc="Sets the LSIRDYC field."]
    #[inline] pub fn set_lsirdyc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="PLLSAI Ready Interrupt Enable"]
    #[inline] pub fn pllsairdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PLLSAIRDYIE != 0"]
    #[inline] pub fn test_pllsairdyie(&self) -> bool {
        self.pllsairdyie() != 0
    }

    #[doc="Sets the PLLSAIRDYIE field."]
    #[inline] pub fn set_pllsairdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="PLLI2S ready interrupt enable"]
    #[inline] pub fn plli2srdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PLLI2SRDYIE != 0"]
    #[inline] pub fn test_plli2srdyie(&self) -> bool {
        self.plli2srdyie() != 0
    }

    #[doc="Sets the PLLI2SRDYIE field."]
    #[inline] pub fn set_plli2srdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Main PLL (PLL) ready interrupt enable"]
    #[inline] pub fn pllrdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PLLRDYIE != 0"]
    #[inline] pub fn test_pllrdyie(&self) -> bool {
        self.pllrdyie() != 0
    }

    #[doc="Sets the PLLRDYIE field."]
    #[inline] pub fn set_pllrdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="HSE ready interrupt enable"]
    #[inline] pub fn hserdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HSERDYIE != 0"]
    #[inline] pub fn test_hserdyie(&self) -> bool {
        self.hserdyie() != 0
    }

    #[doc="Sets the HSERDYIE field."]
    #[inline] pub fn set_hserdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HSI ready interrupt enable"]
    #[inline] pub fn hsirdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSIRDYIE != 0"]
    #[inline] pub fn test_hsirdyie(&self) -> bool {
        self.hsirdyie() != 0
    }

    #[doc="Sets the HSIRDYIE field."]
    #[inline] pub fn set_hsirdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="LSE ready interrupt enable"]
    #[inline] pub fn lserdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSERDYIE != 0"]
    #[inline] pub fn test_lserdyie(&self) -> bool {
        self.lserdyie() != 0
    }

    #[doc="Sets the LSERDYIE field."]
    #[inline] pub fn set_lserdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="LSI ready interrupt enable"]
    #[inline] pub fn lsirdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LSIRDYIE != 0"]
    #[inline] pub fn test_lsirdyie(&self) -> bool {
        self.lsirdyie() != 0
    }

    #[doc="Sets the LSIRDYIE field."]
    #[inline] pub fn set_lsirdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock security system interrupt flag"]
    #[inline] pub fn cssf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CSSF != 0"]
    #[inline] pub fn test_cssf(&self) -> bool {
        self.cssf() != 0
    }

    #[doc="Sets the CSSF field."]
    #[inline] pub fn set_cssf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="PLLSAI ready interrupt flag"]
    #[inline] pub fn pllsairdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLSAIRDYF != 0"]
    #[inline] pub fn test_pllsairdyf(&self) -> bool {
        self.pllsairdyf() != 0
    }

    #[doc="Sets the PLLSAIRDYF field."]
    #[inline] pub fn set_pllsairdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PLLI2S ready interrupt flag"]
    #[inline] pub fn plli2srdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLI2SRDYF != 0"]
    #[inline] pub fn test_plli2srdyf(&self) -> bool {
        self.plli2srdyf() != 0
    }

    #[doc="Sets the PLLI2SRDYF field."]
    #[inline] pub fn set_plli2srdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Main PLL (PLL) ready interrupt flag"]
    #[inline] pub fn pllrdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PLLRDYF != 0"]
    #[inline] pub fn test_pllrdyf(&self) -> bool {
        self.pllrdyf() != 0
    }

    #[doc="Sets the PLLRDYF field."]
    #[inline] pub fn set_pllrdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSE ready interrupt flag"]
    #[inline] pub fn hserdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSERDYF != 0"]
    #[inline] pub fn test_hserdyf(&self) -> bool {
        self.hserdyf() != 0
    }

    #[doc="Sets the HSERDYF field."]
    #[inline] pub fn set_hserdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="HSI ready interrupt flag"]
    #[inline] pub fn hsirdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSIRDYF != 0"]
    #[inline] pub fn test_hsirdyf(&self) -> bool {
        self.hsirdyf() != 0
    }

    #[doc="Sets the HSIRDYF field."]
    #[inline] pub fn set_hsirdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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

    #[doc="LSI ready interrupt flag"]
    #[inline] pub fn lsirdyf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSIRDYF != 0"]
    #[inline] pub fn test_lsirdyf(&self) -> bool {
        self.lsirdyf() != 0
    }

    #[doc="Sets the LSIRDYF field."]
    #[inline] pub fn set_lsirdyf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cir {
    #[inline]
    fn from(other: u32) -> Self {
         Cir(other)
    }
}

impl ::core::fmt::Display for Cir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cssc() != 0 { try!(write!(f, " cssc"))}
        if self.pllsairdyc() != 0 { try!(write!(f, " pllsairdyc"))}
        if self.plli2srdyc() != 0 { try!(write!(f, " plli2srdyc"))}
        if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
        if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
        if self.hsirdyc() != 0 { try!(write!(f, " hsirdyc"))}
        if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
        if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
        if self.pllsairdyie() != 0 { try!(write!(f, " pllsairdyie"))}
        if self.plli2srdyie() != 0 { try!(write!(f, " plli2srdyie"))}
        if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
        if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
        if self.hsirdyie() != 0 { try!(write!(f, " hsirdyie"))}
        if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
        if self.lsirdyie() != 0 { try!(write!(f, " lsirdyie"))}
        if self.cssf() != 0 { try!(write!(f, " cssf"))}
        if self.pllsairdyf() != 0 { try!(write!(f, " pllsairdyf"))}
        if self.plli2srdyf() != 0 { try!(write!(f, " plli2srdyf"))}
        if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
        if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
        if self.hsirdyf() != 0 { try!(write!(f, " hsirdyf"))}
        if self.lserdyf() != 0 { try!(write!(f, " lserdyf"))}
        if self.lsirdyf() != 0 { try!(write!(f, " lsirdyf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1rstr(pub u32);
impl Ahb1rstr {
    #[doc="USB OTG HS module reset"]
    #[inline] pub fn otghsrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if OTGHSRST != 0"]
    #[inline] pub fn test_otghsrst(&self) -> bool {
        self.otghsrst() != 0
    }

    #[doc="Sets the OTGHSRST field."]
    #[inline] pub fn set_otghsrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Ethernet MAC reset"]
    #[inline] pub fn ethmacrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ETHMACRST != 0"]
    #[inline] pub fn test_ethmacrst(&self) -> bool {
        self.ethmacrst() != 0
    }

    #[doc="Sets the ETHMACRST field."]
    #[inline] pub fn set_ethmacrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DMA2D reset"]
    #[inline] pub fn dma2drst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DMA2DRST != 0"]
    #[inline] pub fn test_dma2drst(&self) -> bool {
        self.dma2drst() != 0
    }

    #[doc="Sets the DMA2DRST field."]
    #[inline] pub fn set_dma2drst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="DMA2 reset"]
    #[inline] pub fn dma2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DMA2RST != 0"]
    #[inline] pub fn test_dma2rst(&self) -> bool {
        self.dma2rst() != 0
    }

    #[doc="Sets the DMA2RST field."]
    #[inline] pub fn set_dma2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DMA2 reset"]
    #[inline] pub fn dma1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DMA1RST != 0"]
    #[inline] pub fn test_dma1rst(&self) -> bool {
        self.dma1rst() != 0
    }

    #[doc="Sets the DMA1RST field."]
    #[inline] pub fn set_dma1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
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

    #[doc="IO port K reset"]
    #[inline] pub fn gpiokrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIOKRST != 0"]
    #[inline] pub fn test_gpiokrst(&self) -> bool {
        self.gpiokrst() != 0
    }

    #[doc="Sets the GPIOKRST field."]
    #[inline] pub fn set_gpiokrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="IO port J reset"]
    #[inline] pub fn gpiojrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIOJRST != 0"]
    #[inline] pub fn test_gpiojrst(&self) -> bool {
        self.gpiojrst() != 0
    }

    #[doc="Sets the GPIOJRST field."]
    #[inline] pub fn set_gpiojrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IO port I reset"]
    #[inline] pub fn gpioirst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIOIRST != 0"]
    #[inline] pub fn test_gpioirst(&self) -> bool {
        self.gpioirst() != 0
    }

    #[doc="Sets the GPIOIRST field."]
    #[inline] pub fn set_gpioirst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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

    #[doc="IO port G reset"]
    #[inline] pub fn gpiogrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIOGRST != 0"]
    #[inline] pub fn test_gpiogrst(&self) -> bool {
        self.gpiogrst() != 0
    }

    #[doc="Sets the GPIOGRST field."]
    #[inline] pub fn set_gpiogrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IO port F reset"]
    #[inline] pub fn gpiofrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIOFRST != 0"]
    #[inline] pub fn test_gpiofrst(&self) -> bool {
        self.gpiofrst() != 0
    }

    #[doc="Sets the GPIOFRST field."]
    #[inline] pub fn set_gpiofrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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
        if self.otghsrst() != 0 { try!(write!(f, " otghsrst"))}
        if self.ethmacrst() != 0 { try!(write!(f, " ethmacrst"))}
        if self.dma2drst() != 0 { try!(write!(f, " dma2drst"))}
        if self.dma2rst() != 0 { try!(write!(f, " dma2rst"))}
        if self.dma1rst() != 0 { try!(write!(f, " dma1rst"))}
        if self.crcrst() != 0 { try!(write!(f, " crcrst"))}
        if self.gpiokrst() != 0 { try!(write!(f, " gpiokrst"))}
        if self.gpiojrst() != 0 { try!(write!(f, " gpiojrst"))}
        if self.gpioirst() != 0 { try!(write!(f, " gpioirst"))}
        if self.gpiohrst() != 0 { try!(write!(f, " gpiohrst"))}
        if self.gpiogrst() != 0 { try!(write!(f, " gpiogrst"))}
        if self.gpiofrst() != 0 { try!(write!(f, " gpiofrst"))}
        if self.gpioerst() != 0 { try!(write!(f, " gpioerst"))}
        if self.gpiodrst() != 0 { try!(write!(f, " gpiodrst"))}
        if self.gpiocrst() != 0 { try!(write!(f, " gpiocrst"))}
        if self.gpiobrst() != 0 { try!(write!(f, " gpiobrst"))}
        if self.gpioarst() != 0 { try!(write!(f, " gpioarst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB2 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb2rstr(pub u32);
impl Ahb2rstr {
    #[doc="USB OTG FS module reset"]
    #[inline] pub fn otgfsrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OTGFSRST != 0"]
    #[inline] pub fn test_otgfsrst(&self) -> bool {
        self.otgfsrst() != 0
    }

    #[doc="Sets the OTGFSRST field."]
    #[inline] pub fn set_otgfsrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Random number generator module reset"]
    #[inline] pub fn rngrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RNGRST != 0"]
    #[inline] pub fn test_rngrst(&self) -> bool {
        self.rngrst() != 0
    }

    #[doc="Sets the RNGRST field."]
    #[inline] pub fn set_rngrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Camera interface reset"]
    #[inline] pub fn dcmirst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCMIRST != 0"]
    #[inline] pub fn test_dcmirst(&self) -> bool {
        self.dcmirst() != 0
    }

    #[doc="Sets the DCMIRST field."]
    #[inline] pub fn set_dcmirst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.otgfsrst() != 0 { try!(write!(f, " otgfsrst"))}
        if self.rngrst() != 0 { try!(write!(f, " rngrst"))}
        if self.dcmirst() != 0 { try!(write!(f, " dcmirst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB3 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb3rstr(pub u32);
impl Ahb3rstr {
    #[doc="Flexible memory controller module reset"]
    #[inline] pub fn fmcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FMCRST != 0"]
    #[inline] pub fn test_fmcrst(&self) -> bool {
        self.fmcrst() != 0
    }

    #[doc="Sets the FMCRST field."]
    #[inline] pub fn set_fmcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.fmcrst() != 0 { try!(write!(f, " fmcrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
    #[doc="UART8 reset"]
    #[inline] pub fn uart8rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if UART8RST != 0"]
    #[inline] pub fn test_uart8rst(&self) -> bool {
        self.uart8rst() != 0
    }

    #[doc="Sets the UART8RST field."]
    #[inline] pub fn set_uart8rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="UART7 reset"]
    #[inline] pub fn uart7rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if UART7RST != 0"]
    #[inline] pub fn test_uart7rst(&self) -> bool {
        self.uart7rst() != 0
    }

    #[doc="Sets the UART7RST field."]
    #[inline] pub fn set_uart7rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DAC reset"]
    #[inline] pub fn dacrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACRST != 0"]
    #[inline] pub fn test_dacrst(&self) -> bool {
        self.dacrst() != 0
    }

    #[doc="Sets the DACRST field."]
    #[inline] pub fn set_dacrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Power interface reset"]
    #[inline] pub fn pwrrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWRRST != 0"]
    #[inline] pub fn test_pwrrst(&self) -> bool {
        self.pwrrst() != 0
    }

    #[doc="Sets the PWRRST field."]
    #[inline] pub fn set_pwrrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="CAN2 reset"]
    #[inline] pub fn can2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CAN2RST != 0"]
    #[inline] pub fn test_can2rst(&self) -> bool {
        self.can2rst() != 0
    }

    #[doc="Sets the CAN2RST field."]
    #[inline] pub fn set_can2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CAN1 reset"]
    #[inline] pub fn can1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CAN1RST != 0"]
    #[inline] pub fn test_can1rst(&self) -> bool {
        self.can1rst() != 0
    }

    #[doc="Sets the CAN1RST field."]
    #[inline] pub fn set_can1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
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

    #[doc="I2C 2 reset"]
    #[inline] pub fn i2c2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2RST != 0"]
    #[inline] pub fn test_i2c2rst(&self) -> bool {
        self.i2c2rst() != 0
    }

    #[doc="Sets the I2C2RST field."]
    #[inline] pub fn set_i2c2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C 1 reset"]
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

    #[doc="USART 5 reset"]
    #[inline] pub fn uart5rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5RST != 0"]
    #[inline] pub fn test_uart5rst(&self) -> bool {
        self.uart5rst() != 0
    }

    #[doc="Sets the UART5RST field."]
    #[inline] pub fn set_uart5rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="USART 4 reset"]
    #[inline] pub fn uart4rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4RST != 0"]
    #[inline] pub fn test_uart4rst(&self) -> bool {
        self.uart4rst() != 0
    }

    #[doc="Sets the UART4RST field."]
    #[inline] pub fn set_uart4rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="USART 3 reset"]
    #[inline] pub fn uart3rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if UART3RST != 0"]
    #[inline] pub fn test_uart3rst(&self) -> bool {
        self.uart3rst() != 0
    }

    #[doc="Sets the UART3RST field."]
    #[inline] pub fn set_uart3rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USART 2 reset"]
    #[inline] pub fn uart2rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if UART2RST != 0"]
    #[inline] pub fn test_uart2rst(&self) -> bool {
        self.uart2rst() != 0
    }

    #[doc="Sets the UART2RST field."]
    #[inline] pub fn set_uart2rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI 3 reset"]
    #[inline] pub fn spi3rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SPI3RST != 0"]
    #[inline] pub fn test_spi3rst(&self) -> bool {
        self.spi3rst() != 0
    }

    #[doc="Sets the SPI3RST field."]
    #[inline] pub fn set_spi3rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SPI 2 reset"]
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

    #[doc="Window watchdog reset"]
    #[inline] pub fn wwdgrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGRST != 0"]
    #[inline] pub fn test_wwdgrst(&self) -> bool {
        self.wwdgrst() != 0
    }

    #[doc="Sets the WWDGRST field."]
    #[inline] pub fn set_wwdgrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TIM14 reset"]
    #[inline] pub fn tim14rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TIM14RST != 0"]
    #[inline] pub fn test_tim14rst(&self) -> bool {
        self.tim14rst() != 0
    }

    #[doc="Sets the TIM14RST field."]
    #[inline] pub fn set_tim14rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM13 reset"]
    #[inline] pub fn tim13rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIM13RST != 0"]
    #[inline] pub fn test_tim13rst(&self) -> bool {
        self.tim13rst() != 0
    }

    #[doc="Sets the TIM13RST field."]
    #[inline] pub fn set_tim13rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TIM12 reset"]
    #[inline] pub fn tim12rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIM12RST != 0"]
    #[inline] pub fn test_tim12rst(&self) -> bool {
        self.tim12rst() != 0
    }

    #[doc="Sets the TIM12RST field."]
    #[inline] pub fn set_tim12rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TIM7 reset"]
    #[inline] pub fn tim7rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7RST != 0"]
    #[inline] pub fn test_tim7rst(&self) -> bool {
        self.tim7rst() != 0
    }

    #[doc="Sets the TIM7RST field."]
    #[inline] pub fn set_tim7rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM6 reset"]
    #[inline] pub fn tim6rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6RST != 0"]
    #[inline] pub fn test_tim6rst(&self) -> bool {
        self.tim6rst() != 0
    }

    #[doc="Sets the TIM6RST field."]
    #[inline] pub fn set_tim6rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM5 reset"]
    #[inline] pub fn tim5rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TIM5RST != 0"]
    #[inline] pub fn test_tim5rst(&self) -> bool {
        self.tim5rst() != 0
    }

    #[doc="Sets the TIM5RST field."]
    #[inline] pub fn set_tim5rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TIM4 reset"]
    #[inline] pub fn tim4rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4RST != 0"]
    #[inline] pub fn test_tim4rst(&self) -> bool {
        self.tim4rst() != 0
    }

    #[doc="Sets the TIM4RST field."]
    #[inline] pub fn set_tim4rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TIM3 reset"]
    #[inline] pub fn tim3rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3RST != 0"]
    #[inline] pub fn test_tim3rst(&self) -> bool {
        self.tim3rst() != 0
    }

    #[doc="Sets the TIM3RST field."]
    #[inline] pub fn set_tim3rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM2 reset"]
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

impl From<u32> for Apb1rstr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1rstr(other)
    }
}

impl ::core::fmt::Display for Apb1rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uart8rst() != 0 { try!(write!(f, " uart8rst"))}
        if self.uart7rst() != 0 { try!(write!(f, " uart7rst"))}
        if self.dacrst() != 0 { try!(write!(f, " dacrst"))}
        if self.pwrrst() != 0 { try!(write!(f, " pwrrst"))}
        if self.can2rst() != 0 { try!(write!(f, " can2rst"))}
        if self.can1rst() != 0 { try!(write!(f, " can1rst"))}
        if self.i2c3rst() != 0 { try!(write!(f, " i2c3rst"))}
        if self.i2c2rst() != 0 { try!(write!(f, " i2c2rst"))}
        if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
        if self.uart5rst() != 0 { try!(write!(f, " uart5rst"))}
        if self.uart4rst() != 0 { try!(write!(f, " uart4rst"))}
        if self.uart3rst() != 0 { try!(write!(f, " uart3rst"))}
        if self.uart2rst() != 0 { try!(write!(f, " uart2rst"))}
        if self.spi3rst() != 0 { try!(write!(f, " spi3rst"))}
        if self.spi2rst() != 0 { try!(write!(f, " spi2rst"))}
        if self.wwdgrst() != 0 { try!(write!(f, " wwdgrst"))}
        if self.tim14rst() != 0 { try!(write!(f, " tim14rst"))}
        if self.tim13rst() != 0 { try!(write!(f, " tim13rst"))}
        if self.tim12rst() != 0 { try!(write!(f, " tim12rst"))}
        if self.tim7rst() != 0 { try!(write!(f, " tim7rst"))}
        if self.tim6rst() != 0 { try!(write!(f, " tim6rst"))}
        if self.tim5rst() != 0 { try!(write!(f, " tim5rst"))}
        if self.tim4rst() != 0 { try!(write!(f, " tim4rst"))}
        if self.tim3rst() != 0 { try!(write!(f, " tim3rst"))}
        if self.tim2rst() != 0 { try!(write!(f, " tim2rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
    #[doc="LTDC reset"]
    #[inline] pub fn ltdcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if LTDCRST != 0"]
    #[inline] pub fn test_ltdcrst(&self) -> bool {
        self.ltdcrst() != 0
    }

    #[doc="Sets the LTDCRST field."]
    #[inline] pub fn set_ltdcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="SAI1 reset"]
    #[inline] pub fn sai1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SAI1RST != 0"]
    #[inline] pub fn test_sai1rst(&self) -> bool {
        self.sai1rst() != 0
    }

    #[doc="Sets the SAI1RST field."]
    #[inline] pub fn set_sai1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="SPI6 reset"]
    #[inline] pub fn spi6rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SPI6RST != 0"]
    #[inline] pub fn test_spi6rst(&self) -> bool {
        self.spi6rst() != 0
    }

    #[doc="Sets the SPI6RST field."]
    #[inline] pub fn set_spi6rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SPI5 reset"]
    #[inline] pub fn spi5rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SPI5RST != 0"]
    #[inline] pub fn test_spi5rst(&self) -> bool {
        self.spi5rst() != 0
    }

    #[doc="Sets the SPI5RST field."]
    #[inline] pub fn set_spi5rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="TIM11 reset"]
    #[inline] pub fn tim11rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM11RST != 0"]
    #[inline] pub fn test_tim11rst(&self) -> bool {
        self.tim11rst() != 0
    }

    #[doc="Sets the TIM11RST field."]
    #[inline] pub fn set_tim11rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM10 reset"]
    #[inline] pub fn tim10rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM10RST != 0"]
    #[inline] pub fn test_tim10rst(&self) -> bool {
        self.tim10rst() != 0
    }

    #[doc="Sets the TIM10RST field."]
    #[inline] pub fn set_tim10rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM9 reset"]
    #[inline] pub fn tim9rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM9RST != 0"]
    #[inline] pub fn test_tim9rst(&self) -> bool {
        self.tim9rst() != 0
    }

    #[doc="Sets the TIM9RST field."]
    #[inline] pub fn set_tim9rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System configuration controller reset"]
    #[inline] pub fn syscfgrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SYSCFGRST != 0"]
    #[inline] pub fn test_syscfgrst(&self) -> bool {
        self.syscfgrst() != 0
    }

    #[doc="Sets the SYSCFGRST field."]
    #[inline] pub fn set_syscfgrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI4 reset"]
    #[inline] pub fn spi4rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SPI4RST != 0"]
    #[inline] pub fn test_spi4rst(&self) -> bool {
        self.spi4rst() != 0
    }

    #[doc="Sets the SPI4RST field."]
    #[inline] pub fn set_spi4rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="SPI 1 reset"]
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

    #[doc="SDIO reset"]
    #[inline] pub fn sdiorst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SDIORST != 0"]
    #[inline] pub fn test_sdiorst(&self) -> bool {
        self.sdiorst() != 0
    }

    #[doc="Sets the SDIORST field."]
    #[inline] pub fn set_sdiorst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ADC interface reset (common to all ADCs)"]
    #[inline] pub fn adcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADCRST != 0"]
    #[inline] pub fn test_adcrst(&self) -> bool {
        self.adcrst() != 0
    }

    #[doc="Sets the ADCRST field."]
    #[inline] pub fn set_adcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="USART6 reset"]
    #[inline] pub fn usart6rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USART6RST != 0"]
    #[inline] pub fn test_usart6rst(&self) -> bool {
        self.usart6rst() != 0
    }

    #[doc="Sets the USART6RST field."]
    #[inline] pub fn set_usart6rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="USART1 reset"]
    #[inline] pub fn usart1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if USART1RST != 0"]
    #[inline] pub fn test_usart1rst(&self) -> bool {
        self.usart1rst() != 0
    }

    #[doc="Sets the USART1RST field."]
    #[inline] pub fn set_usart1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM8 reset"]
    #[inline] pub fn tim8rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM8RST != 0"]
    #[inline] pub fn test_tim8rst(&self) -> bool {
        self.tim8rst() != 0
    }

    #[doc="Sets the TIM8RST field."]
    #[inline] pub fn set_tim8rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM1 reset"]
    #[inline] pub fn tim1rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM1RST != 0"]
    #[inline] pub fn test_tim1rst(&self) -> bool {
        self.tim1rst() != 0
    }

    #[doc="Sets the TIM1RST field."]
    #[inline] pub fn set_tim1rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.ltdcrst() != 0 { try!(write!(f, " ltdcrst"))}
        if self.sai1rst() != 0 { try!(write!(f, " sai1rst"))}
        if self.spi6rst() != 0 { try!(write!(f, " spi6rst"))}
        if self.spi5rst() != 0 { try!(write!(f, " spi5rst"))}
        if self.tim11rst() != 0 { try!(write!(f, " tim11rst"))}
        if self.tim10rst() != 0 { try!(write!(f, " tim10rst"))}
        if self.tim9rst() != 0 { try!(write!(f, " tim9rst"))}
        if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
        if self.spi4rst() != 0 { try!(write!(f, " spi4rst"))}
        if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
        if self.sdiorst() != 0 { try!(write!(f, " sdiorst"))}
        if self.adcrst() != 0 { try!(write!(f, " adcrst"))}
        if self.usart6rst() != 0 { try!(write!(f, " usart6rst"))}
        if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
        if self.tim8rst() != 0 { try!(write!(f, " tim8rst"))}
        if self.tim1rst() != 0 { try!(write!(f, " tim1rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral clock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1enr(pub u32);
impl Ahb1enr {
    #[doc="USB OTG HSULPI clock enable"]
    #[inline] pub fn otghsulpien(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OTGHSULPIEN != 0"]
    #[inline] pub fn test_otghsulpien(&self) -> bool {
        self.otghsulpien() != 0
    }

    #[doc="Sets the OTGHSULPIEN field."]
    #[inline] pub fn set_otghsulpien<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="USB OTG HS clock enable"]
    #[inline] pub fn otghsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if OTGHSEN != 0"]
    #[inline] pub fn test_otghsen(&self) -> bool {
        self.otghsen() != 0
    }

    #[doc="Sets the OTGHSEN field."]
    #[inline] pub fn set_otghsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Ethernet PTP clock enable"]
    #[inline] pub fn ethmacptpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if ETHMACPTPEN != 0"]
    #[inline] pub fn test_ethmacptpen(&self) -> bool {
        self.ethmacptpen() != 0
    }

    #[doc="Sets the ETHMACPTPEN field."]
    #[inline] pub fn set_ethmacptpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Ethernet Reception clock enable"]
    #[inline] pub fn ethmacrxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if ETHMACRXEN != 0"]
    #[inline] pub fn test_ethmacrxen(&self) -> bool {
        self.ethmacrxen() != 0
    }

    #[doc="Sets the ETHMACRXEN field."]
    #[inline] pub fn set_ethmacrxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Ethernet Transmission clock enable"]
    #[inline] pub fn ethmactxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if ETHMACTXEN != 0"]
    #[inline] pub fn test_ethmactxen(&self) -> bool {
        self.ethmactxen() != 0
    }

    #[doc="Sets the ETHMACTXEN field."]
    #[inline] pub fn set_ethmactxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Ethernet MAC clock enable"]
    #[inline] pub fn ethmacen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ETHMACEN != 0"]
    #[inline] pub fn test_ethmacen(&self) -> bool {
        self.ethmacen() != 0
    }

    #[doc="Sets the ETHMACEN field."]
    #[inline] pub fn set_ethmacen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DMA2D clock enable"]
    #[inline] pub fn dma2den(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DMA2DEN != 0"]
    #[inline] pub fn test_dma2den(&self) -> bool {
        self.dma2den() != 0
    }

    #[doc="Sets the DMA2DEN field."]
    #[inline] pub fn set_dma2den<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="DMA2 clock enable"]
    #[inline] pub fn dma2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DMA2EN != 0"]
    #[inline] pub fn test_dma2en(&self) -> bool {
        self.dma2en() != 0
    }

    #[doc="Sets the DMA2EN field."]
    #[inline] pub fn set_dma2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DMA1 clock enable"]
    #[inline] pub fn dma1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DMA1EN != 0"]
    #[inline] pub fn test_dma1en(&self) -> bool {
        self.dma1en() != 0
    }

    #[doc="Sets the DMA1EN field."]
    #[inline] pub fn set_dma1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="CCM data RAM clock enable"]
    #[inline] pub fn ccmdataramen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CCMDATARAMEN != 0"]
    #[inline] pub fn test_ccmdataramen(&self) -> bool {
        self.ccmdataramen() != 0
    }

    #[doc="Sets the CCMDATARAMEN field."]
    #[inline] pub fn set_ccmdataramen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Backup SRAM interface clock enable"]
    #[inline] pub fn bkpsramen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if BKPSRAMEN != 0"]
    #[inline] pub fn test_bkpsramen(&self) -> bool {
        self.bkpsramen() != 0
    }

    #[doc="Sets the BKPSRAMEN field."]
    #[inline] pub fn set_bkpsramen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="CRC clock enable"]
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

    #[doc="IO port K clock enable"]
    #[inline] pub fn gpioken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIOKEN != 0"]
    #[inline] pub fn test_gpioken(&self) -> bool {
        self.gpioken() != 0
    }

    #[doc="Sets the GPIOKEN field."]
    #[inline] pub fn set_gpioken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="IO port J clock enable"]
    #[inline] pub fn gpiojen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIOJEN != 0"]
    #[inline] pub fn test_gpiojen(&self) -> bool {
        self.gpiojen() != 0
    }

    #[doc="Sets the GPIOJEN field."]
    #[inline] pub fn set_gpiojen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IO port I clock enable"]
    #[inline] pub fn gpioien(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIOIEN != 0"]
    #[inline] pub fn test_gpioien(&self) -> bool {
        self.gpioien() != 0
    }

    #[doc="Sets the GPIOIEN field."]
    #[inline] pub fn set_gpioien<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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

    #[doc="IO port G clock enable"]
    #[inline] pub fn gpiogen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIOGEN != 0"]
    #[inline] pub fn test_gpiogen(&self) -> bool {
        self.gpiogen() != 0
    }

    #[doc="Sets the GPIOGEN field."]
    #[inline] pub fn set_gpiogen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IO port F clock enable"]
    #[inline] pub fn gpiofen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIOFEN != 0"]
    #[inline] pub fn test_gpiofen(&self) -> bool {
        self.gpiofen() != 0
    }

    #[doc="Sets the GPIOFEN field."]
    #[inline] pub fn set_gpiofen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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
        if self.otghsulpien() != 0 { try!(write!(f, " otghsulpien"))}
        if self.otghsen() != 0 { try!(write!(f, " otghsen"))}
        if self.ethmacptpen() != 0 { try!(write!(f, " ethmacptpen"))}
        if self.ethmacrxen() != 0 { try!(write!(f, " ethmacrxen"))}
        if self.ethmactxen() != 0 { try!(write!(f, " ethmactxen"))}
        if self.ethmacen() != 0 { try!(write!(f, " ethmacen"))}
        if self.dma2den() != 0 { try!(write!(f, " dma2den"))}
        if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
        if self.dma1en() != 0 { try!(write!(f, " dma1en"))}
        if self.ccmdataramen() != 0 { try!(write!(f, " ccmdataramen"))}
        if self.bkpsramen() != 0 { try!(write!(f, " bkpsramen"))}
        if self.crcen() != 0 { try!(write!(f, " crcen"))}
        if self.gpioken() != 0 { try!(write!(f, " gpioken"))}
        if self.gpiojen() != 0 { try!(write!(f, " gpiojen"))}
        if self.gpioien() != 0 { try!(write!(f, " gpioien"))}
        if self.gpiohen() != 0 { try!(write!(f, " gpiohen"))}
        if self.gpiogen() != 0 { try!(write!(f, " gpiogen"))}
        if self.gpiofen() != 0 { try!(write!(f, " gpiofen"))}
        if self.gpioeen() != 0 { try!(write!(f, " gpioeen"))}
        if self.gpioden() != 0 { try!(write!(f, " gpioden"))}
        if self.gpiocen() != 0 { try!(write!(f, " gpiocen"))}
        if self.gpioben() != 0 { try!(write!(f, " gpioben"))}
        if self.gpioaen() != 0 { try!(write!(f, " gpioaen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB2 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb2enr(pub u32);
impl Ahb2enr {
    #[doc="USB OTG FS clock enable"]
    #[inline] pub fn otgfsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OTGFSEN != 0"]
    #[inline] pub fn test_otgfsen(&self) -> bool {
        self.otgfsen() != 0
    }

    #[doc="Sets the OTGFSEN field."]
    #[inline] pub fn set_otgfsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Random number generator clock enable"]
    #[inline] pub fn rngen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RNGEN != 0"]
    #[inline] pub fn test_rngen(&self) -> bool {
        self.rngen() != 0
    }

    #[doc="Sets the RNGEN field."]
    #[inline] pub fn set_rngen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Camera interface enable"]
    #[inline] pub fn dcmien(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCMIEN != 0"]
    #[inline] pub fn test_dcmien(&self) -> bool {
        self.dcmien() != 0
    }

    #[doc="Sets the DCMIEN field."]
    #[inline] pub fn set_dcmien<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.otgfsen() != 0 { try!(write!(f, " otgfsen"))}
        if self.rngen() != 0 { try!(write!(f, " rngen"))}
        if self.dcmien() != 0 { try!(write!(f, " dcmien"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB3 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb3enr(pub u32);
impl Ahb3enr {
    #[doc="Flexible memory controller module clock enable"]
    #[inline] pub fn fmcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FMCEN != 0"]
    #[inline] pub fn test_fmcen(&self) -> bool {
        self.fmcen() != 0
    }

    #[doc="Sets the FMCEN field."]
    #[inline] pub fn set_fmcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.fmcen() != 0 { try!(write!(f, " fmcen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
    #[doc="UART8 clock enable"]
    #[inline] pub fn uart8enr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if UART8ENR != 0"]
    #[inline] pub fn test_uart8enr(&self) -> bool {
        self.uart8enr() != 0
    }

    #[doc="Sets the UART8ENR field."]
    #[inline] pub fn set_uart8enr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="UART7 clock enable"]
    #[inline] pub fn uart7enr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if UART7ENR != 0"]
    #[inline] pub fn test_uart7enr(&self) -> bool {
        self.uart7enr() != 0
    }

    #[doc="Sets the UART7ENR field."]
    #[inline] pub fn set_uart7enr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DAC interface clock enable"]
    #[inline] pub fn dacen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACEN != 0"]
    #[inline] pub fn test_dacen(&self) -> bool {
        self.dacen() != 0
    }

    #[doc="Sets the DACEN field."]
    #[inline] pub fn set_dacen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Power interface clock enable"]
    #[inline] pub fn pwren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWREN != 0"]
    #[inline] pub fn test_pwren(&self) -> bool {
        self.pwren() != 0
    }

    #[doc="Sets the PWREN field."]
    #[inline] pub fn set_pwren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="CAN 2 clock enable"]
    #[inline] pub fn can2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CAN2EN != 0"]
    #[inline] pub fn test_can2en(&self) -> bool {
        self.can2en() != 0
    }

    #[doc="Sets the CAN2EN field."]
    #[inline] pub fn set_can2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CAN 1 clock enable"]
    #[inline] pub fn can1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CAN1EN != 0"]
    #[inline] pub fn test_can1en(&self) -> bool {
        self.can1en() != 0
    }

    #[doc="Sets the CAN1EN field."]
    #[inline] pub fn set_can1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="I2C3 clock enable"]
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

    #[doc="I2C2 clock enable"]
    #[inline] pub fn i2c2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2EN != 0"]
    #[inline] pub fn test_i2c2en(&self) -> bool {
        self.i2c2en() != 0
    }

    #[doc="Sets the I2C2EN field."]
    #[inline] pub fn set_i2c2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C1 clock enable"]
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

    #[doc="UART5 clock enable"]
    #[inline] pub fn uart5en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5EN != 0"]
    #[inline] pub fn test_uart5en(&self) -> bool {
        self.uart5en() != 0
    }

    #[doc="Sets the UART5EN field."]
    #[inline] pub fn set_uart5en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="UART4 clock enable"]
    #[inline] pub fn uart4en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4EN != 0"]
    #[inline] pub fn test_uart4en(&self) -> bool {
        self.uart4en() != 0
    }

    #[doc="Sets the UART4EN field."]
    #[inline] pub fn set_uart4en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="USART3 clock enable"]
    #[inline] pub fn usart3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USART3EN != 0"]
    #[inline] pub fn test_usart3en(&self) -> bool {
        self.usart3en() != 0
    }

    #[doc="Sets the USART3EN field."]
    #[inline] pub fn set_usart3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USART 2 clock enable"]
    #[inline] pub fn usart2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if USART2EN != 0"]
    #[inline] pub fn test_usart2en(&self) -> bool {
        self.usart2en() != 0
    }

    #[doc="Sets the USART2EN field."]
    #[inline] pub fn set_usart2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI3 clock enable"]
    #[inline] pub fn spi3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SPI3EN != 0"]
    #[inline] pub fn test_spi3en(&self) -> bool {
        self.spi3en() != 0
    }

    #[doc="Sets the SPI3EN field."]
    #[inline] pub fn set_spi3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SPI2 clock enable"]
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

    #[doc="Window watchdog clock enable"]
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

    #[doc="TIM14 clock enable"]
    #[inline] pub fn tim14en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TIM14EN != 0"]
    #[inline] pub fn test_tim14en(&self) -> bool {
        self.tim14en() != 0
    }

    #[doc="Sets the TIM14EN field."]
    #[inline] pub fn set_tim14en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM13 clock enable"]
    #[inline] pub fn tim13en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIM13EN != 0"]
    #[inline] pub fn test_tim13en(&self) -> bool {
        self.tim13en() != 0
    }

    #[doc="Sets the TIM13EN field."]
    #[inline] pub fn set_tim13en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TIM12 clock enable"]
    #[inline] pub fn tim12en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIM12EN != 0"]
    #[inline] pub fn test_tim12en(&self) -> bool {
        self.tim12en() != 0
    }

    #[doc="Sets the TIM12EN field."]
    #[inline] pub fn set_tim12en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TIM7 clock enable"]
    #[inline] pub fn tim7en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7EN != 0"]
    #[inline] pub fn test_tim7en(&self) -> bool {
        self.tim7en() != 0
    }

    #[doc="Sets the TIM7EN field."]
    #[inline] pub fn set_tim7en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM6 clock enable"]
    #[inline] pub fn tim6en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6EN != 0"]
    #[inline] pub fn test_tim6en(&self) -> bool {
        self.tim6en() != 0
    }

    #[doc="Sets the TIM6EN field."]
    #[inline] pub fn set_tim6en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM5 clock enable"]
    #[inline] pub fn tim5en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TIM5EN != 0"]
    #[inline] pub fn test_tim5en(&self) -> bool {
        self.tim5en() != 0
    }

    #[doc="Sets the TIM5EN field."]
    #[inline] pub fn set_tim5en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TIM4 clock enable"]
    #[inline] pub fn tim4en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4EN != 0"]
    #[inline] pub fn test_tim4en(&self) -> bool {
        self.tim4en() != 0
    }

    #[doc="Sets the TIM4EN field."]
    #[inline] pub fn set_tim4en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TIM3 clock enable"]
    #[inline] pub fn tim3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3EN != 0"]
    #[inline] pub fn test_tim3en(&self) -> bool {
        self.tim3en() != 0
    }

    #[doc="Sets the TIM3EN field."]
    #[inline] pub fn set_tim3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM2 clock enable"]
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

impl From<u32> for Apb1enr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1enr(other)
    }
}

impl ::core::fmt::Display for Apb1enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uart8enr() != 0 { try!(write!(f, " uart8enr"))}
        if self.uart7enr() != 0 { try!(write!(f, " uart7enr"))}
        if self.dacen() != 0 { try!(write!(f, " dacen"))}
        if self.pwren() != 0 { try!(write!(f, " pwren"))}
        if self.can2en() != 0 { try!(write!(f, " can2en"))}
        if self.can1en() != 0 { try!(write!(f, " can1en"))}
        if self.i2c3en() != 0 { try!(write!(f, " i2c3en"))}
        if self.i2c2en() != 0 { try!(write!(f, " i2c2en"))}
        if self.i2c1en() != 0 { try!(write!(f, " i2c1en"))}
        if self.uart5en() != 0 { try!(write!(f, " uart5en"))}
        if self.uart4en() != 0 { try!(write!(f, " uart4en"))}
        if self.usart3en() != 0 { try!(write!(f, " usart3en"))}
        if self.usart2en() != 0 { try!(write!(f, " usart2en"))}
        if self.spi3en() != 0 { try!(write!(f, " spi3en"))}
        if self.spi2en() != 0 { try!(write!(f, " spi2en"))}
        if self.wwdgen() != 0 { try!(write!(f, " wwdgen"))}
        if self.tim14en() != 0 { try!(write!(f, " tim14en"))}
        if self.tim13en() != 0 { try!(write!(f, " tim13en"))}
        if self.tim12en() != 0 { try!(write!(f, " tim12en"))}
        if self.tim7en() != 0 { try!(write!(f, " tim7en"))}
        if self.tim6en() != 0 { try!(write!(f, " tim6en"))}
        if self.tim5en() != 0 { try!(write!(f, " tim5en"))}
        if self.tim4en() != 0 { try!(write!(f, " tim4en"))}
        if self.tim3en() != 0 { try!(write!(f, " tim3en"))}
        if self.tim2en() != 0 { try!(write!(f, " tim2en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
    #[doc="LTDC clock enable"]
    #[inline] pub fn ltdcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if LTDCEN != 0"]
    #[inline] pub fn test_ltdcen(&self) -> bool {
        self.ltdcen() != 0
    }

    #[doc="Sets the LTDCEN field."]
    #[inline] pub fn set_ltdcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="SAI1 clock enable"]
    #[inline] pub fn sai1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SAI1EN != 0"]
    #[inline] pub fn test_sai1en(&self) -> bool {
        self.sai1en() != 0
    }

    #[doc="Sets the SAI1EN field."]
    #[inline] pub fn set_sai1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="SPI6 clock enable"]
    #[inline] pub fn spi6en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SPI6EN != 0"]
    #[inline] pub fn test_spi6en(&self) -> bool {
        self.spi6en() != 0
    }

    #[doc="Sets the SPI6EN field."]
    #[inline] pub fn set_spi6en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SPI5 clock enable"]
    #[inline] pub fn spi5en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SPI5EN != 0"]
    #[inline] pub fn test_spi5en(&self) -> bool {
        self.spi5en() != 0
    }

    #[doc="Sets the SPI5EN field."]
    #[inline] pub fn set_spi5en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="TIM11 clock enable"]
    #[inline] pub fn tim11en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM11EN != 0"]
    #[inline] pub fn test_tim11en(&self) -> bool {
        self.tim11en() != 0
    }

    #[doc="Sets the TIM11EN field."]
    #[inline] pub fn set_tim11en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM10 clock enable"]
    #[inline] pub fn tim10en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM10EN != 0"]
    #[inline] pub fn test_tim10en(&self) -> bool {
        self.tim10en() != 0
    }

    #[doc="Sets the TIM10EN field."]
    #[inline] pub fn set_tim10en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM9 clock enable"]
    #[inline] pub fn tim9en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM9EN != 0"]
    #[inline] pub fn test_tim9en(&self) -> bool {
        self.tim9en() != 0
    }

    #[doc="Sets the TIM9EN field."]
    #[inline] pub fn set_tim9en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System configuration controller clock enable"]
    #[inline] pub fn syscfgen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SYSCFGEN != 0"]
    #[inline] pub fn test_syscfgen(&self) -> bool {
        self.syscfgen() != 0
    }

    #[doc="Sets the SYSCFGEN field."]
    #[inline] pub fn set_syscfgen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI4 clock enable"]
    #[inline] pub fn spi4en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SPI4EN != 0"]
    #[inline] pub fn test_spi4en(&self) -> bool {
        self.spi4en() != 0
    }

    #[doc="Sets the SPI4EN field."]
    #[inline] pub fn set_spi4en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="SPI1 clock enable"]
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

    #[doc="SDIO clock enable"]
    #[inline] pub fn sdioen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SDIOEN != 0"]
    #[inline] pub fn test_sdioen(&self) -> bool {
        self.sdioen() != 0
    }

    #[doc="Sets the SDIOEN field."]
    #[inline] pub fn set_sdioen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ADC3 clock enable"]
    #[inline] pub fn adc3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ADC3EN != 0"]
    #[inline] pub fn test_adc3en(&self) -> bool {
        self.adc3en() != 0
    }

    #[doc="Sets the ADC3EN field."]
    #[inline] pub fn set_adc3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ADC2 clock enable"]
    #[inline] pub fn adc2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADC2EN != 0"]
    #[inline] pub fn test_adc2en(&self) -> bool {
        self.adc2en() != 0
    }

    #[doc="Sets the ADC2EN field."]
    #[inline] pub fn set_adc2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="ADC1 clock enable"]
    #[inline] pub fn adc1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1EN != 0"]
    #[inline] pub fn test_adc1en(&self) -> bool {
        self.adc1en() != 0
    }

    #[doc="Sets the ADC1EN field."]
    #[inline] pub fn set_adc1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="USART6 clock enable"]
    #[inline] pub fn usart6en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USART6EN != 0"]
    #[inline] pub fn test_usart6en(&self) -> bool {
        self.usart6en() != 0
    }

    #[doc="Sets the USART6EN field."]
    #[inline] pub fn set_usart6en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="USART1 clock enable"]
    #[inline] pub fn usart1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if USART1EN != 0"]
    #[inline] pub fn test_usart1en(&self) -> bool {
        self.usart1en() != 0
    }

    #[doc="Sets the USART1EN field."]
    #[inline] pub fn set_usart1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM8 clock enable"]
    #[inline] pub fn tim8en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM8EN != 0"]
    #[inline] pub fn test_tim8en(&self) -> bool {
        self.tim8en() != 0
    }

    #[doc="Sets the TIM8EN field."]
    #[inline] pub fn set_tim8en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM1 clock enable"]
    #[inline] pub fn tim1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM1EN != 0"]
    #[inline] pub fn test_tim1en(&self) -> bool {
        self.tim1en() != 0
    }

    #[doc="Sets the TIM1EN field."]
    #[inline] pub fn set_tim1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.ltdcen() != 0 { try!(write!(f, " ltdcen"))}
        if self.sai1en() != 0 { try!(write!(f, " sai1en"))}
        if self.spi6en() != 0 { try!(write!(f, " spi6en"))}
        if self.spi5en() != 0 { try!(write!(f, " spi5en"))}
        if self.tim11en() != 0 { try!(write!(f, " tim11en"))}
        if self.tim10en() != 0 { try!(write!(f, " tim10en"))}
        if self.tim9en() != 0 { try!(write!(f, " tim9en"))}
        if self.syscfgen() != 0 { try!(write!(f, " syscfgen"))}
        if self.spi4en() != 0 { try!(write!(f, " spi4en"))}
        if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
        if self.sdioen() != 0 { try!(write!(f, " sdioen"))}
        if self.adc3en() != 0 { try!(write!(f, " adc3en"))}
        if self.adc2en() != 0 { try!(write!(f, " adc2en"))}
        if self.adc1en() != 0 { try!(write!(f, " adc1en"))}
        if self.usart6en() != 0 { try!(write!(f, " usart6en"))}
        if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
        if self.tim8en() != 0 { try!(write!(f, " tim8en"))}
        if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral clock enable in low power mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1lpenr(pub u32);
impl Ahb1lpenr {
    #[doc="USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline] pub fn otghsulpilpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OTGHSULPILPEN != 0"]
    #[inline] pub fn test_otghsulpilpen(&self) -> bool {
        self.otghsulpilpen() != 0
    }

    #[doc="Sets the OTGHSULPILPEN field."]
    #[inline] pub fn set_otghsulpilpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="USB OTG HS clock enable during Sleep mode"]
    #[inline] pub fn otghslpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if OTGHSLPEN != 0"]
    #[inline] pub fn test_otghslpen(&self) -> bool {
        self.otghslpen() != 0
    }

    #[doc="Sets the OTGHSLPEN field."]
    #[inline] pub fn set_otghslpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Ethernet PTP clock enable during Sleep mode"]
    #[inline] pub fn ethmacptplpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if ETHMACPTPLPEN != 0"]
    #[inline] pub fn test_ethmacptplpen(&self) -> bool {
        self.ethmacptplpen() != 0
    }

    #[doc="Sets the ETHMACPTPLPEN field."]
    #[inline] pub fn set_ethmacptplpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Ethernet reception clock enable during Sleep mode"]
    #[inline] pub fn ethmacrxlpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if ETHMACRXLPEN != 0"]
    #[inline] pub fn test_ethmacrxlpen(&self) -> bool {
        self.ethmacrxlpen() != 0
    }

    #[doc="Sets the ETHMACRXLPEN field."]
    #[inline] pub fn set_ethmacrxlpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Ethernet transmission clock enable during Sleep mode"]
    #[inline] pub fn ethmactxlpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if ETHMACTXLPEN != 0"]
    #[inline] pub fn test_ethmactxlpen(&self) -> bool {
        self.ethmactxlpen() != 0
    }

    #[doc="Sets the ETHMACTXLPEN field."]
    #[inline] pub fn set_ethmactxlpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Ethernet MAC clock enable during Sleep mode"]
    #[inline] pub fn ethmaclpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ETHMACLPEN != 0"]
    #[inline] pub fn test_ethmaclpen(&self) -> bool {
        self.ethmaclpen() != 0
    }

    #[doc="Sets the ETHMACLPEN field."]
    #[inline] pub fn set_ethmaclpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DMA2D clock enable during Sleep mode"]
    #[inline] pub fn dma2dlpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DMA2DLPEN != 0"]
    #[inline] pub fn test_dma2dlpen(&self) -> bool {
        self.dma2dlpen() != 0
    }

    #[doc="Sets the DMA2DLPEN field."]
    #[inline] pub fn set_dma2dlpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="DMA2 clock enable during Sleep mode"]
    #[inline] pub fn dma2lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DMA2LPEN != 0"]
    #[inline] pub fn test_dma2lpen(&self) -> bool {
        self.dma2lpen() != 0
    }

    #[doc="Sets the DMA2LPEN field."]
    #[inline] pub fn set_dma2lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DMA1 clock enable during Sleep mode"]
    #[inline] pub fn dma1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DMA1LPEN != 0"]
    #[inline] pub fn test_dma1lpen(&self) -> bool {
        self.dma1lpen() != 0
    }

    #[doc="Sets the DMA1LPEN field."]
    #[inline] pub fn set_dma1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SRAM 3 interface clock enable during Sleep mode"]
    #[inline] pub fn sram3lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if SRAM3LPEN != 0"]
    #[inline] pub fn test_sram3lpen(&self) -> bool {
        self.sram3lpen() != 0
    }

    #[doc="Sets the SRAM3LPEN field."]
    #[inline] pub fn set_sram3lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Backup SRAM interface clock enable during Sleep mode"]
    #[inline] pub fn bkpsramlpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if BKPSRAMLPEN != 0"]
    #[inline] pub fn test_bkpsramlpen(&self) -> bool {
        self.bkpsramlpen() != 0
    }

    #[doc="Sets the BKPSRAMLPEN field."]
    #[inline] pub fn set_bkpsramlpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SRAM 2 interface clock enable during Sleep mode"]
    #[inline] pub fn sram2lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SRAM2LPEN != 0"]
    #[inline] pub fn test_sram2lpen(&self) -> bool {
        self.sram2lpen() != 0
    }

    #[doc="Sets the SRAM2LPEN field."]
    #[inline] pub fn set_sram2lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SRAM 1interface clock enable during Sleep mode"]
    #[inline] pub fn sram1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SRAM1LPEN != 0"]
    #[inline] pub fn test_sram1lpen(&self) -> bool {
        self.sram1lpen() != 0
    }

    #[doc="Sets the SRAM1LPEN field."]
    #[inline] pub fn set_sram1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Flash interface clock enable during Sleep mode"]
    #[inline] pub fn flitflpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FLITFLPEN != 0"]
    #[inline] pub fn test_flitflpen(&self) -> bool {
        self.flitflpen() != 0
    }

    #[doc="Sets the FLITFLPEN field."]
    #[inline] pub fn set_flitflpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="CRC clock enable during Sleep mode"]
    #[inline] pub fn crclpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCLPEN != 0"]
    #[inline] pub fn test_crclpen(&self) -> bool {
        self.crclpen() != 0
    }

    #[doc="Sets the CRCLPEN field."]
    #[inline] pub fn set_crclpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IO port K clock enable during Sleep mode"]
    #[inline] pub fn gpioklpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIOKLPEN != 0"]
    #[inline] pub fn test_gpioklpen(&self) -> bool {
        self.gpioklpen() != 0
    }

    #[doc="Sets the GPIOKLPEN field."]
    #[inline] pub fn set_gpioklpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="IO port J clock enable during Sleep mode"]
    #[inline] pub fn gpiojlpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIOJLPEN != 0"]
    #[inline] pub fn test_gpiojlpen(&self) -> bool {
        self.gpiojlpen() != 0
    }

    #[doc="Sets the GPIOJLPEN field."]
    #[inline] pub fn set_gpiojlpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IO port I clock enable during Sleep mode"]
    #[inline] pub fn gpioilpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIOILPEN != 0"]
    #[inline] pub fn test_gpioilpen(&self) -> bool {
        self.gpioilpen() != 0
    }

    #[doc="Sets the GPIOILPEN field."]
    #[inline] pub fn set_gpioilpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IO port H clock enable during Sleep mode"]
    #[inline] pub fn gpiohlpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHLPEN != 0"]
    #[inline] pub fn test_gpiohlpen(&self) -> bool {
        self.gpiohlpen() != 0
    }

    #[doc="Sets the GPIOHLPEN field."]
    #[inline] pub fn set_gpiohlpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IO port G clock enable during Sleep mode"]
    #[inline] pub fn gpioglpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIOGLPEN != 0"]
    #[inline] pub fn test_gpioglpen(&self) -> bool {
        self.gpioglpen() != 0
    }

    #[doc="Sets the GPIOGLPEN field."]
    #[inline] pub fn set_gpioglpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IO port F clock enable during Sleep mode"]
    #[inline] pub fn gpioflpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIOFLPEN != 0"]
    #[inline] pub fn test_gpioflpen(&self) -> bool {
        self.gpioflpen() != 0
    }

    #[doc="Sets the GPIOFLPEN field."]
    #[inline] pub fn set_gpioflpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IO port E clock enable during Sleep mode"]
    #[inline] pub fn gpioelpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOELPEN != 0"]
    #[inline] pub fn test_gpioelpen(&self) -> bool {
        self.gpioelpen() != 0
    }

    #[doc="Sets the GPIOELPEN field."]
    #[inline] pub fn set_gpioelpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IO port D clock enable during Sleep mode"]
    #[inline] pub fn gpiodlpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODLPEN != 0"]
    #[inline] pub fn test_gpiodlpen(&self) -> bool {
        self.gpiodlpen() != 0
    }

    #[doc="Sets the GPIODLPEN field."]
    #[inline] pub fn set_gpiodlpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IO port C clock enable during Sleep mode"]
    #[inline] pub fn gpioclpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCLPEN != 0"]
    #[inline] pub fn test_gpioclpen(&self) -> bool {
        self.gpioclpen() != 0
    }

    #[doc="Sets the GPIOCLPEN field."]
    #[inline] pub fn set_gpioclpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IO port B clock enable during Sleep mode"]
    #[inline] pub fn gpioblpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBLPEN != 0"]
    #[inline] pub fn test_gpioblpen(&self) -> bool {
        self.gpioblpen() != 0
    }

    #[doc="Sets the GPIOBLPEN field."]
    #[inline] pub fn set_gpioblpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IO port A clock enable during sleep mode"]
    #[inline] pub fn gpioalpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOALPEN != 0"]
    #[inline] pub fn test_gpioalpen(&self) -> bool {
        self.gpioalpen() != 0
    }

    #[doc="Sets the GPIOALPEN field."]
    #[inline] pub fn set_gpioalpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb1lpenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb1lpenr(other)
    }
}

impl ::core::fmt::Display for Ahb1lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb1lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.otghsulpilpen() != 0 { try!(write!(f, " otghsulpilpen"))}
        if self.otghslpen() != 0 { try!(write!(f, " otghslpen"))}
        if self.ethmacptplpen() != 0 { try!(write!(f, " ethmacptplpen"))}
        if self.ethmacrxlpen() != 0 { try!(write!(f, " ethmacrxlpen"))}
        if self.ethmactxlpen() != 0 { try!(write!(f, " ethmactxlpen"))}
        if self.ethmaclpen() != 0 { try!(write!(f, " ethmaclpen"))}
        if self.dma2dlpen() != 0 { try!(write!(f, " dma2dlpen"))}
        if self.dma2lpen() != 0 { try!(write!(f, " dma2lpen"))}
        if self.dma1lpen() != 0 { try!(write!(f, " dma1lpen"))}
        if self.sram3lpen() != 0 { try!(write!(f, " sram3lpen"))}
        if self.bkpsramlpen() != 0 { try!(write!(f, " bkpsramlpen"))}
        if self.sram2lpen() != 0 { try!(write!(f, " sram2lpen"))}
        if self.sram1lpen() != 0 { try!(write!(f, " sram1lpen"))}
        if self.flitflpen() != 0 { try!(write!(f, " flitflpen"))}
        if self.crclpen() != 0 { try!(write!(f, " crclpen"))}
        if self.gpioklpen() != 0 { try!(write!(f, " gpioklpen"))}
        if self.gpiojlpen() != 0 { try!(write!(f, " gpiojlpen"))}
        if self.gpioilpen() != 0 { try!(write!(f, " gpioilpen"))}
        if self.gpiohlpen() != 0 { try!(write!(f, " gpiohlpen"))}
        if self.gpioglpen() != 0 { try!(write!(f, " gpioglpen"))}
        if self.gpioflpen() != 0 { try!(write!(f, " gpioflpen"))}
        if self.gpioelpen() != 0 { try!(write!(f, " gpioelpen"))}
        if self.gpiodlpen() != 0 { try!(write!(f, " gpiodlpen"))}
        if self.gpioclpen() != 0 { try!(write!(f, " gpioclpen"))}
        if self.gpioblpen() != 0 { try!(write!(f, " gpioblpen"))}
        if self.gpioalpen() != 0 { try!(write!(f, " gpioalpen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB2 peripheral clock enable in low power mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb2lpenr(pub u32);
impl Ahb2lpenr {
    #[doc="USB OTG FS clock enable during Sleep mode"]
    #[inline] pub fn otgfslpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OTGFSLPEN != 0"]
    #[inline] pub fn test_otgfslpen(&self) -> bool {
        self.otgfslpen() != 0
    }

    #[doc="Sets the OTGFSLPEN field."]
    #[inline] pub fn set_otgfslpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Random number generator clock enable during Sleep mode"]
    #[inline] pub fn rnglpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RNGLPEN != 0"]
    #[inline] pub fn test_rnglpen(&self) -> bool {
        self.rnglpen() != 0
    }

    #[doc="Sets the RNGLPEN field."]
    #[inline] pub fn set_rnglpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Camera interface enable during Sleep mode"]
    #[inline] pub fn dcmilpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCMILPEN != 0"]
    #[inline] pub fn test_dcmilpen(&self) -> bool {
        self.dcmilpen() != 0
    }

    #[doc="Sets the DCMILPEN field."]
    #[inline] pub fn set_dcmilpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb2lpenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb2lpenr(other)
    }
}

impl ::core::fmt::Display for Ahb2lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb2lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.otgfslpen() != 0 { try!(write!(f, " otgfslpen"))}
        if self.rnglpen() != 0 { try!(write!(f, " rnglpen"))}
        if self.dcmilpen() != 0 { try!(write!(f, " dcmilpen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB3 peripheral clock enable in low power mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb3lpenr(pub u32);
impl Ahb3lpenr {
    #[doc="Flexible memory controller module clock enable during Sleep mode"]
    #[inline] pub fn fmclpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FMCLPEN != 0"]
    #[inline] pub fn test_fmclpen(&self) -> bool {
        self.fmclpen() != 0
    }

    #[doc="Sets the FMCLPEN field."]
    #[inline] pub fn set_fmclpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahb3lpenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahb3lpenr(other)
    }
}

impl ::core::fmt::Display for Ahb3lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahb3lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fmclpen() != 0 { try!(write!(f, " fmclpen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clock enable in low power mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1lpenr(pub u32);
impl Apb1lpenr {
    #[doc="UART8 clock enable during Sleep mode"]
    #[inline] pub fn uart8lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if UART8LPEN != 0"]
    #[inline] pub fn test_uart8lpen(&self) -> bool {
        self.uart8lpen() != 0
    }

    #[doc="Sets the UART8LPEN field."]
    #[inline] pub fn set_uart8lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="UART7 clock enable during Sleep mode"]
    #[inline] pub fn uart7lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if UART7LPEN != 0"]
    #[inline] pub fn test_uart7lpen(&self) -> bool {
        self.uart7lpen() != 0
    }

    #[doc="Sets the UART7LPEN field."]
    #[inline] pub fn set_uart7lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DAC interface clock enable during Sleep mode"]
    #[inline] pub fn daclpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACLPEN != 0"]
    #[inline] pub fn test_daclpen(&self) -> bool {
        self.daclpen() != 0
    }

    #[doc="Sets the DACLPEN field."]
    #[inline] pub fn set_daclpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Power interface clock enable during Sleep mode"]
    #[inline] pub fn pwrlpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWRLPEN != 0"]
    #[inline] pub fn test_pwrlpen(&self) -> bool {
        self.pwrlpen() != 0
    }

    #[doc="Sets the PWRLPEN field."]
    #[inline] pub fn set_pwrlpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="CAN 2 clock enable during Sleep mode"]
    #[inline] pub fn can2lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CAN2LPEN != 0"]
    #[inline] pub fn test_can2lpen(&self) -> bool {
        self.can2lpen() != 0
    }

    #[doc="Sets the CAN2LPEN field."]
    #[inline] pub fn set_can2lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CAN 1 clock enable during Sleep mode"]
    #[inline] pub fn can1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CAN1LPEN != 0"]
    #[inline] pub fn test_can1lpen(&self) -> bool {
        self.can1lpen() != 0
    }

    #[doc="Sets the CAN1LPEN field."]
    #[inline] pub fn set_can1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="I2C3 clock enable during Sleep mode"]
    #[inline] pub fn i2c3lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3LPEN != 0"]
    #[inline] pub fn test_i2c3lpen(&self) -> bool {
        self.i2c3lpen() != 0
    }

    #[doc="Sets the I2C3LPEN field."]
    #[inline] pub fn set_i2c3lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C2 clock enable during Sleep mode"]
    #[inline] pub fn i2c2lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2LPEN != 0"]
    #[inline] pub fn test_i2c2lpen(&self) -> bool {
        self.i2c2lpen() != 0
    }

    #[doc="Sets the I2C2LPEN field."]
    #[inline] pub fn set_i2c2lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C1 clock enable during Sleep mode"]
    #[inline] pub fn i2c1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1LPEN != 0"]
    #[inline] pub fn test_i2c1lpen(&self) -> bool {
        self.i2c1lpen() != 0
    }

    #[doc="Sets the I2C1LPEN field."]
    #[inline] pub fn set_i2c1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="UART5 clock enable during Sleep mode"]
    #[inline] pub fn uart5lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5LPEN != 0"]
    #[inline] pub fn test_uart5lpen(&self) -> bool {
        self.uart5lpen() != 0
    }

    #[doc="Sets the UART5LPEN field."]
    #[inline] pub fn set_uart5lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="UART4 clock enable during Sleep mode"]
    #[inline] pub fn uart4lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4LPEN != 0"]
    #[inline] pub fn test_uart4lpen(&self) -> bool {
        self.uart4lpen() != 0
    }

    #[doc="Sets the UART4LPEN field."]
    #[inline] pub fn set_uart4lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="USART3 clock enable during Sleep mode"]
    #[inline] pub fn usart3lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USART3LPEN != 0"]
    #[inline] pub fn test_usart3lpen(&self) -> bool {
        self.usart3lpen() != 0
    }

    #[doc="Sets the USART3LPEN field."]
    #[inline] pub fn set_usart3lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USART2 clock enable during Sleep mode"]
    #[inline] pub fn usart2lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if USART2LPEN != 0"]
    #[inline] pub fn test_usart2lpen(&self) -> bool {
        self.usart2lpen() != 0
    }

    #[doc="Sets the USART2LPEN field."]
    #[inline] pub fn set_usart2lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI3 clock enable during Sleep mode"]
    #[inline] pub fn spi3lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SPI3LPEN != 0"]
    #[inline] pub fn test_spi3lpen(&self) -> bool {
        self.spi3lpen() != 0
    }

    #[doc="Sets the SPI3LPEN field."]
    #[inline] pub fn set_spi3lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SPI2 clock enable during Sleep mode"]
    #[inline] pub fn spi2lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2LPEN != 0"]
    #[inline] pub fn test_spi2lpen(&self) -> bool {
        self.spi2lpen() != 0
    }

    #[doc="Sets the SPI2LPEN field."]
    #[inline] pub fn set_spi2lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Window watchdog clock enable during Sleep mode"]
    #[inline] pub fn wwdglpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGLPEN != 0"]
    #[inline] pub fn test_wwdglpen(&self) -> bool {
        self.wwdglpen() != 0
    }

    #[doc="Sets the WWDGLPEN field."]
    #[inline] pub fn set_wwdglpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TIM14 clock enable during Sleep mode"]
    #[inline] pub fn tim14lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TIM14LPEN != 0"]
    #[inline] pub fn test_tim14lpen(&self) -> bool {
        self.tim14lpen() != 0
    }

    #[doc="Sets the TIM14LPEN field."]
    #[inline] pub fn set_tim14lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM13 clock enable during Sleep mode"]
    #[inline] pub fn tim13lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIM13LPEN != 0"]
    #[inline] pub fn test_tim13lpen(&self) -> bool {
        self.tim13lpen() != 0
    }

    #[doc="Sets the TIM13LPEN field."]
    #[inline] pub fn set_tim13lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TIM12 clock enable during Sleep mode"]
    #[inline] pub fn tim12lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIM12LPEN != 0"]
    #[inline] pub fn test_tim12lpen(&self) -> bool {
        self.tim12lpen() != 0
    }

    #[doc="Sets the TIM12LPEN field."]
    #[inline] pub fn set_tim12lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TIM7 clock enable during Sleep mode"]
    #[inline] pub fn tim7lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7LPEN != 0"]
    #[inline] pub fn test_tim7lpen(&self) -> bool {
        self.tim7lpen() != 0
    }

    #[doc="Sets the TIM7LPEN field."]
    #[inline] pub fn set_tim7lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM6 clock enable during Sleep mode"]
    #[inline] pub fn tim6lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6LPEN != 0"]
    #[inline] pub fn test_tim6lpen(&self) -> bool {
        self.tim6lpen() != 0
    }

    #[doc="Sets the TIM6LPEN field."]
    #[inline] pub fn set_tim6lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM5 clock enable during Sleep mode"]
    #[inline] pub fn tim5lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TIM5LPEN != 0"]
    #[inline] pub fn test_tim5lpen(&self) -> bool {
        self.tim5lpen() != 0
    }

    #[doc="Sets the TIM5LPEN field."]
    #[inline] pub fn set_tim5lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TIM4 clock enable during Sleep mode"]
    #[inline] pub fn tim4lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4LPEN != 0"]
    #[inline] pub fn test_tim4lpen(&self) -> bool {
        self.tim4lpen() != 0
    }

    #[doc="Sets the TIM4LPEN field."]
    #[inline] pub fn set_tim4lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TIM3 clock enable during Sleep mode"]
    #[inline] pub fn tim3lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3LPEN != 0"]
    #[inline] pub fn test_tim3lpen(&self) -> bool {
        self.tim3lpen() != 0
    }

    #[doc="Sets the TIM3LPEN field."]
    #[inline] pub fn set_tim3lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM2 clock enable during Sleep mode"]
    #[inline] pub fn tim2lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2LPEN != 0"]
    #[inline] pub fn test_tim2lpen(&self) -> bool {
        self.tim2lpen() != 0
    }

    #[doc="Sets the TIM2LPEN field."]
    #[inline] pub fn set_tim2lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb1lpenr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1lpenr(other)
    }
}

impl ::core::fmt::Display for Apb1lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uart8lpen() != 0 { try!(write!(f, " uart8lpen"))}
        if self.uart7lpen() != 0 { try!(write!(f, " uart7lpen"))}
        if self.daclpen() != 0 { try!(write!(f, " daclpen"))}
        if self.pwrlpen() != 0 { try!(write!(f, " pwrlpen"))}
        if self.can2lpen() != 0 { try!(write!(f, " can2lpen"))}
        if self.can1lpen() != 0 { try!(write!(f, " can1lpen"))}
        if self.i2c3lpen() != 0 { try!(write!(f, " i2c3lpen"))}
        if self.i2c2lpen() != 0 { try!(write!(f, " i2c2lpen"))}
        if self.i2c1lpen() != 0 { try!(write!(f, " i2c1lpen"))}
        if self.uart5lpen() != 0 { try!(write!(f, " uart5lpen"))}
        if self.uart4lpen() != 0 { try!(write!(f, " uart4lpen"))}
        if self.usart3lpen() != 0 { try!(write!(f, " usart3lpen"))}
        if self.usart2lpen() != 0 { try!(write!(f, " usart2lpen"))}
        if self.spi3lpen() != 0 { try!(write!(f, " spi3lpen"))}
        if self.spi2lpen() != 0 { try!(write!(f, " spi2lpen"))}
        if self.wwdglpen() != 0 { try!(write!(f, " wwdglpen"))}
        if self.tim14lpen() != 0 { try!(write!(f, " tim14lpen"))}
        if self.tim13lpen() != 0 { try!(write!(f, " tim13lpen"))}
        if self.tim12lpen() != 0 { try!(write!(f, " tim12lpen"))}
        if self.tim7lpen() != 0 { try!(write!(f, " tim7lpen"))}
        if self.tim6lpen() != 0 { try!(write!(f, " tim6lpen"))}
        if self.tim5lpen() != 0 { try!(write!(f, " tim5lpen"))}
        if self.tim4lpen() != 0 { try!(write!(f, " tim4lpen"))}
        if self.tim3lpen() != 0 { try!(write!(f, " tim3lpen"))}
        if self.tim2lpen() != 0 { try!(write!(f, " tim2lpen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral clock enabled in low power mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2lpenr(pub u32);
impl Apb2lpenr {
    #[doc="LTDC clock enable"]
    #[inline] pub fn ltdclpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if LTDCLPEN != 0"]
    #[inline] pub fn test_ltdclpen(&self) -> bool {
        self.ltdclpen() != 0
    }

    #[doc="Sets the LTDCLPEN field."]
    #[inline] pub fn set_ltdclpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="SAI1 clock enable"]
    #[inline] pub fn sai1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SAI1LPEN != 0"]
    #[inline] pub fn test_sai1lpen(&self) -> bool {
        self.sai1lpen() != 0
    }

    #[doc="Sets the SAI1LPEN field."]
    #[inline] pub fn set_sai1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="SPI 6 clock enable during Sleep mode"]
    #[inline] pub fn spi6lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SPI6LPEN != 0"]
    #[inline] pub fn test_spi6lpen(&self) -> bool {
        self.spi6lpen() != 0
    }

    #[doc="Sets the SPI6LPEN field."]
    #[inline] pub fn set_spi6lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SPI 5 clock enable during Sleep mode"]
    #[inline] pub fn spi5lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SPI5LPEN != 0"]
    #[inline] pub fn test_spi5lpen(&self) -> bool {
        self.spi5lpen() != 0
    }

    #[doc="Sets the SPI5LPEN field."]
    #[inline] pub fn set_spi5lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="TIM11 clock enable during Sleep mode"]
    #[inline] pub fn tim11lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM11LPEN != 0"]
    #[inline] pub fn test_tim11lpen(&self) -> bool {
        self.tim11lpen() != 0
    }

    #[doc="Sets the TIM11LPEN field."]
    #[inline] pub fn set_tim11lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM10 clock enable during Sleep mode"]
    #[inline] pub fn tim10lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM10LPEN != 0"]
    #[inline] pub fn test_tim10lpen(&self) -> bool {
        self.tim10lpen() != 0
    }

    #[doc="Sets the TIM10LPEN field."]
    #[inline] pub fn set_tim10lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM9 clock enable during sleep mode"]
    #[inline] pub fn tim9lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM9LPEN != 0"]
    #[inline] pub fn test_tim9lpen(&self) -> bool {
        self.tim9lpen() != 0
    }

    #[doc="Sets the TIM9LPEN field."]
    #[inline] pub fn set_tim9lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System configuration controller clock enable during Sleep mode"]
    #[inline] pub fn syscfglpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SYSCFGLPEN != 0"]
    #[inline] pub fn test_syscfglpen(&self) -> bool {
        self.syscfglpen() != 0
    }

    #[doc="Sets the SYSCFGLPEN field."]
    #[inline] pub fn set_syscfglpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI 4 clock enable during Sleep mode"]
    #[inline] pub fn spi4lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SPI4LPEN != 0"]
    #[inline] pub fn test_spi4lpen(&self) -> bool {
        self.spi4lpen() != 0
    }

    #[doc="Sets the SPI4LPEN field."]
    #[inline] pub fn set_spi4lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="SPI 1 clock enable during Sleep mode"]
    #[inline] pub fn spi1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1LPEN != 0"]
    #[inline] pub fn test_spi1lpen(&self) -> bool {
        self.spi1lpen() != 0
    }

    #[doc="Sets the SPI1LPEN field."]
    #[inline] pub fn set_spi1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SDIO clock enable during Sleep mode"]
    #[inline] pub fn sdiolpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SDIOLPEN != 0"]
    #[inline] pub fn test_sdiolpen(&self) -> bool {
        self.sdiolpen() != 0
    }

    #[doc="Sets the SDIOLPEN field."]
    #[inline] pub fn set_sdiolpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ADC 3 clock enable during Sleep mode"]
    #[inline] pub fn adc3lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ADC3LPEN != 0"]
    #[inline] pub fn test_adc3lpen(&self) -> bool {
        self.adc3lpen() != 0
    }

    #[doc="Sets the ADC3LPEN field."]
    #[inline] pub fn set_adc3lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ADC2 clock enable during Sleep mode"]
    #[inline] pub fn adc2lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADC2LPEN != 0"]
    #[inline] pub fn test_adc2lpen(&self) -> bool {
        self.adc2lpen() != 0
    }

    #[doc="Sets the ADC2LPEN field."]
    #[inline] pub fn set_adc2lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="ADC1 clock enable during Sleep mode"]
    #[inline] pub fn adc1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1LPEN != 0"]
    #[inline] pub fn test_adc1lpen(&self) -> bool {
        self.adc1lpen() != 0
    }

    #[doc="Sets the ADC1LPEN field."]
    #[inline] pub fn set_adc1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="USART6 clock enable during Sleep mode"]
    #[inline] pub fn usart6lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USART6LPEN != 0"]
    #[inline] pub fn test_usart6lpen(&self) -> bool {
        self.usart6lpen() != 0
    }

    #[doc="Sets the USART6LPEN field."]
    #[inline] pub fn set_usart6lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="USART1 clock enable during Sleep mode"]
    #[inline] pub fn usart1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if USART1LPEN != 0"]
    #[inline] pub fn test_usart1lpen(&self) -> bool {
        self.usart1lpen() != 0
    }

    #[doc="Sets the USART1LPEN field."]
    #[inline] pub fn set_usart1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM8 clock enable during Sleep mode"]
    #[inline] pub fn tim8lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM8LPEN != 0"]
    #[inline] pub fn test_tim8lpen(&self) -> bool {
        self.tim8lpen() != 0
    }

    #[doc="Sets the TIM8LPEN field."]
    #[inline] pub fn set_tim8lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM1 clock enable during Sleep mode"]
    #[inline] pub fn tim1lpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM1LPEN != 0"]
    #[inline] pub fn test_tim1lpen(&self) -> bool {
        self.tim1lpen() != 0
    }

    #[doc="Sets the TIM1LPEN field."]
    #[inline] pub fn set_tim1lpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apb2lpenr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb2lpenr(other)
    }
}

impl ::core::fmt::Display for Apb2lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb2lpenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ltdclpen() != 0 { try!(write!(f, " ltdclpen"))}
        if self.sai1lpen() != 0 { try!(write!(f, " sai1lpen"))}
        if self.spi6lpen() != 0 { try!(write!(f, " spi6lpen"))}
        if self.spi5lpen() != 0 { try!(write!(f, " spi5lpen"))}
        if self.tim11lpen() != 0 { try!(write!(f, " tim11lpen"))}
        if self.tim10lpen() != 0 { try!(write!(f, " tim10lpen"))}
        if self.tim9lpen() != 0 { try!(write!(f, " tim9lpen"))}
        if self.syscfglpen() != 0 { try!(write!(f, " syscfglpen"))}
        if self.spi4lpen() != 0 { try!(write!(f, " spi4lpen"))}
        if self.spi1lpen() != 0 { try!(write!(f, " spi1lpen"))}
        if self.sdiolpen() != 0 { try!(write!(f, " sdiolpen"))}
        if self.adc3lpen() != 0 { try!(write!(f, " adc3lpen"))}
        if self.adc2lpen() != 0 { try!(write!(f, " adc2lpen"))}
        if self.adc1lpen() != 0 { try!(write!(f, " adc1lpen"))}
        if self.usart6lpen() != 0 { try!(write!(f, " usart6lpen"))}
        if self.usart1lpen() != 0 { try!(write!(f, " usart1lpen"))}
        if self.tim8lpen() != 0 { try!(write!(f, " tim8lpen"))}
        if self.tim1lpen() != 0 { try!(write!(f, " tim1lpen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup domain control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdcr(pub u32);
impl Bdcr {
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
    #[inline] pub fn rtcsel1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RTCSEL1 != 0"]
    #[inline] pub fn test_rtcsel1(&self) -> bool {
        self.rtcsel1() != 0
    }

    #[doc="Sets the RTCSEL1 field."]
    #[inline] pub fn set_rtcsel1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="RTC clock source selection"]
    #[inline] pub fn rtcsel0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RTCSEL0 != 0"]
    #[inline] pub fn test_rtcsel0(&self) -> bool {
        self.rtcsel0() != 0
    }

    #[doc="Sets the RTCSEL0 field."]
    #[inline] pub fn set_rtcsel0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="External low-speed oscillator bypass"]
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

    #[doc="External low-speed oscillator ready"]
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

    #[doc="External low-speed oscillator enable"]
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
        if self.bdrst() != 0 { try!(write!(f, " bdrst"))}
        if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
        if self.rtcsel1() != 0 { try!(write!(f, " rtcsel1"))}
        if self.rtcsel0() != 0 { try!(write!(f, " rtcsel0"))}
        if self.lsebyp() != 0 { try!(write!(f, " lsebyp"))}
        if self.lserdy() != 0 { try!(write!(f, " lserdy"))}
        if self.lseon() != 0 { try!(write!(f, " lseon"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="clock control & status register"]
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

    #[doc="Independent watchdog reset flag"]
    #[inline] pub fn wdgrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if WDGRSTF != 0"]
    #[inline] pub fn test_wdgrstf(&self) -> bool {
        self.wdgrstf() != 0
    }

    #[doc="Sets the WDGRSTF field."]
    #[inline] pub fn set_wdgrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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

    #[doc="POR/PDR reset flag"]
    #[inline] pub fn porrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PORRSTF != 0"]
    #[inline] pub fn test_porrstf(&self) -> bool {
        self.porrstf() != 0
    }

    #[doc="Sets the PORRSTF field."]
    #[inline] pub fn set_porrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="PIN reset flag"]
    #[inline] pub fn padrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PADRSTF != 0"]
    #[inline] pub fn test_padrstf(&self) -> bool {
        self.padrstf() != 0
    }

    #[doc="Sets the PADRSTF field."]
    #[inline] pub fn set_padrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="BOR reset flag"]
    #[inline] pub fn borrstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if BORRSTF != 0"]
    #[inline] pub fn test_borrstf(&self) -> bool {
        self.borrstf() != 0
    }

    #[doc="Sets the BORRSTF field."]
    #[inline] pub fn set_borrstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Remove reset flag"]
    #[inline] pub fn rmvf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RMVF != 0"]
    #[inline] pub fn test_rmvf(&self) -> bool {
        self.rmvf() != 0
    }

    #[doc="Sets the RMVF field."]
    #[inline] pub fn set_rmvf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Internal low-speed oscillator ready"]
    #[inline] pub fn lsirdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSIRDY != 0"]
    #[inline] pub fn test_lsirdy(&self) -> bool {
        self.lsirdy() != 0
    }

    #[doc="Sets the LSIRDY field."]
    #[inline] pub fn set_lsirdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Internal low-speed oscillator enable"]
    #[inline] pub fn lsion(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSION != 0"]
    #[inline] pub fn test_lsion(&self) -> bool {
        self.lsion() != 0
    }

    #[doc="Sets the LSION field."]
    #[inline] pub fn set_lsion<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.wdgrstf() != 0 { try!(write!(f, " wdgrstf"))}
        if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
        if self.porrstf() != 0 { try!(write!(f, " porrstf"))}
        if self.padrstf() != 0 { try!(write!(f, " padrstf"))}
        if self.borrstf() != 0 { try!(write!(f, " borrstf"))}
        if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
        if self.lsirdy() != 0 { try!(write!(f, " lsirdy"))}
        if self.lsion() != 0 { try!(write!(f, " lsion"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="spread spectrum clock generation register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sscgr(pub u32);
impl Sscgr {
    #[doc="Spread spectrum modulation enable"]
    #[inline] pub fn sscgen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SSCGEN != 0"]
    #[inline] pub fn test_sscgen(&self) -> bool {
        self.sscgen() != 0
    }

    #[doc="Sets the SSCGEN field."]
    #[inline] pub fn set_sscgen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Spread Select"]
    #[inline] pub fn spreadsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SPREADSEL != 0"]
    #[inline] pub fn test_spreadsel(&self) -> bool {
        self.spreadsel() != 0
    }

    #[doc="Sets the SPREADSEL field."]
    #[inline] pub fn set_spreadsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Incrementation step"]
    #[inline] pub fn incstep(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7fff) as u16) } // [27:13]
    }

    #[doc="Returns true if INCSTEP != 0"]
    #[inline] pub fn test_incstep(&self) -> bool {
        self.incstep() != 0
    }

    #[doc="Sets the INCSTEP field."]
    #[inline] pub fn set_incstep<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Modulation period"]
    #[inline] pub fn modper(&self) -> ::bobbin_bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if MODPER != 0"]
    #[inline] pub fn test_modper(&self) -> bool {
        self.modper() != 0
    }

    #[doc="Sets the MODPER field."]
    #[inline] pub fn set_modper<V: Into<::bobbin_bits::U13>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sscgr {
    #[inline]
    fn from(other: u32) -> Self {
         Sscgr(other)
    }
}

impl ::core::fmt::Display for Sscgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sscgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sscgen() != 0 { try!(write!(f, " sscgen"))}
        if self.spreadsel() != 0 { try!(write!(f, " spreadsel"))}
        if self.incstep() != 0 { try!(write!(f, " incstep=0x{:x}", self.incstep()))}
        if self.modper() != 0 { try!(write!(f, " modper=0x{:x}", self.modper()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PLLI2S configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Plli2scfgr(pub u32);
impl Plli2scfgr {
    #[doc="PLLI2S division factor for I2S clocks"]
    #[inline] pub fn plli2sr(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if PLLI2SR != 0"]
    #[inline] pub fn test_plli2sr(&self) -> bool {
        self.plli2sr() != 0
    }

    #[doc="Sets the PLLI2SR field."]
    #[inline] pub fn set_plli2sr<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="PLLI2S division factor for SAI1 clock"]
    #[inline] pub fn plli2sq(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if PLLI2SQ != 0"]
    #[inline] pub fn test_plli2sq(&self) -> bool {
        self.plli2sq() != 0
    }

    #[doc="Sets the PLLI2SQ field."]
    #[inline] pub fn set_plli2sq<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="PLLI2S multiplication factor for VCO"]
    #[inline] pub fn plli2sn(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1ff) as u16) } // [14:6]
    }

    #[doc="Returns true if PLLI2SN != 0"]
    #[inline] pub fn test_plli2sn(&self) -> bool {
        self.plli2sn() != 0
    }

    #[doc="Sets the PLLI2SN field."]
    #[inline] pub fn set_plli2sn<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Plli2scfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Plli2scfgr(other)
    }
}

impl ::core::fmt::Display for Plli2scfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Plli2scfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.plli2sr() != 0 { try!(write!(f, " plli2sr=0x{:x}", self.plli2sr()))}
        if self.plli2sq() != 0 { try!(write!(f, " plli2sq=0x{:x}", self.plli2sq()))}
        if self.plli2sn() != 0 { try!(write!(f, " plli2sn=0x{:x}", self.plli2sn()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

