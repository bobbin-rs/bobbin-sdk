//! Reset and clock control

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Reset and clock control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RccPeriph(pub usize);
impl RccPeriph {
    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x0) as *mut Cr
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
           self.cr_mut()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile(self.cr_ptr())
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(self.cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICSCR register."]
    #[inline] pub fn icscr_mut(&self) -> *mut Icscr { 
        (self.0 + 0x4) as *mut Icscr
    }

    #[doc="Get the *const pointer for the ICSCR register."]
    #[inline] pub fn icscr_ptr(&self) -> *const Icscr { 
           self.icscr_mut()
    }

    #[doc="Read the ICSCR register."]
    #[inline] pub fn icscr(&self) -> Icscr { 
        unsafe {
            read_volatile(self.icscr_ptr())
        }
    }

    #[doc="Write the ICSCR register."]
    #[inline] pub fn set_icscr<F: FnOnce(Icscr) -> Icscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icscr_mut(), f(Icscr(0)));
        }
        self
    }

    #[doc="Modify the ICSCR register."]
    #[inline] pub fn with_icscr<F: FnOnce(Icscr) -> Icscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icscr_mut(), f(self.icscr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR register."]
    #[inline] pub fn cfgr_mut(&self) -> *mut Cfgr { 
        (self.0 + 0x8) as *mut Cfgr
    }

    #[doc="Get the *const pointer for the CFGR register."]
    #[inline] pub fn cfgr_ptr(&self) -> *const Cfgr { 
           self.cfgr_mut()
    }

    #[doc="Read the CFGR register."]
    #[inline] pub fn cfgr(&self) -> Cfgr { 
        unsafe {
            read_volatile(self.cfgr_ptr())
        }
    }

    #[doc="Write the CFGR register."]
    #[inline] pub fn set_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr_mut(), f(Cfgr(0)));
        }
        self
    }

    #[doc="Modify the CFGR register."]
    #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr_mut(), f(self.cfgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PLLCFGR register."]
    #[inline] pub fn pllcfgr_mut(&self) -> *mut Pllcfgr { 
        (self.0 + 0xc) as *mut Pllcfgr
    }

    #[doc="Get the *const pointer for the PLLCFGR register."]
    #[inline] pub fn pllcfgr_ptr(&self) -> *const Pllcfgr { 
           self.pllcfgr_mut()
    }

    #[doc="Read the PLLCFGR register."]
    #[inline] pub fn pllcfgr(&self) -> Pllcfgr { 
        unsafe {
            read_volatile(self.pllcfgr_ptr())
        }
    }

    #[doc="Write the PLLCFGR register."]
    #[inline] pub fn set_pllcfgr<F: FnOnce(Pllcfgr) -> Pllcfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pllcfgr_mut(), f(Pllcfgr(0)));
        }
        self
    }

    #[doc="Modify the PLLCFGR register."]
    #[inline] pub fn with_pllcfgr<F: FnOnce(Pllcfgr) -> Pllcfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pllcfgr_mut(), f(self.pllcfgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PLLSAI1CFGR register."]
    #[inline] pub fn pllsai1cfgr_mut(&self) -> *mut Pllsai1cfgr { 
        (self.0 + 0x10) as *mut Pllsai1cfgr
    }

    #[doc="Get the *const pointer for the PLLSAI1CFGR register."]
    #[inline] pub fn pllsai1cfgr_ptr(&self) -> *const Pllsai1cfgr { 
           self.pllsai1cfgr_mut()
    }

    #[doc="Read the PLLSAI1CFGR register."]
    #[inline] pub fn pllsai1cfgr(&self) -> Pllsai1cfgr { 
        unsafe {
            read_volatile(self.pllsai1cfgr_ptr())
        }
    }

    #[doc="Write the PLLSAI1CFGR register."]
    #[inline] pub fn set_pllsai1cfgr<F: FnOnce(Pllsai1cfgr) -> Pllsai1cfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pllsai1cfgr_mut(), f(Pllsai1cfgr(0)));
        }
        self
    }

    #[doc="Modify the PLLSAI1CFGR register."]
    #[inline] pub fn with_pllsai1cfgr<F: FnOnce(Pllsai1cfgr) -> Pllsai1cfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pllsai1cfgr_mut(), f(self.pllsai1cfgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PLLSAI2CFGR register."]
    #[inline] pub fn pllsai2cfgr_mut(&self) -> *mut Pllsai2cfgr { 
        (self.0 + 0x14) as *mut Pllsai2cfgr
    }

    #[doc="Get the *const pointer for the PLLSAI2CFGR register."]
    #[inline] pub fn pllsai2cfgr_ptr(&self) -> *const Pllsai2cfgr { 
           self.pllsai2cfgr_mut()
    }

    #[doc="Read the PLLSAI2CFGR register."]
    #[inline] pub fn pllsai2cfgr(&self) -> Pllsai2cfgr { 
        unsafe {
            read_volatile(self.pllsai2cfgr_ptr())
        }
    }

    #[doc="Write the PLLSAI2CFGR register."]
    #[inline] pub fn set_pllsai2cfgr<F: FnOnce(Pllsai2cfgr) -> Pllsai2cfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pllsai2cfgr_mut(), f(Pllsai2cfgr(0)));
        }
        self
    }

    #[doc="Modify the PLLSAI2CFGR register."]
    #[inline] pub fn with_pllsai2cfgr<F: FnOnce(Pllsai2cfgr) -> Pllsai2cfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pllsai2cfgr_mut(), f(self.pllsai2cfgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CIER register."]
    #[inline] pub fn cier_mut(&self) -> *mut Cier { 
        (self.0 + 0x18) as *mut Cier
    }

    #[doc="Get the *const pointer for the CIER register."]
    #[inline] pub fn cier_ptr(&self) -> *const Cier { 
           self.cier_mut()
    }

    #[doc="Read the CIER register."]
    #[inline] pub fn cier(&self) -> Cier { 
        unsafe {
            read_volatile(self.cier_ptr())
        }
    }

    #[doc="Write the CIER register."]
    #[inline] pub fn set_cier<F: FnOnce(Cier) -> Cier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cier_mut(), f(Cier(0)));
        }
        self
    }

    #[doc="Modify the CIER register."]
    #[inline] pub fn with_cier<F: FnOnce(Cier) -> Cier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cier_mut(), f(self.cier()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CIFR register."]
    #[inline] pub fn cifr_mut(&self) -> *mut Cifr { 
        (self.0 + 0x1c) as *mut Cifr
    }

    #[doc="Get the *const pointer for the CIFR register."]
    #[inline] pub fn cifr_ptr(&self) -> *const Cifr { 
           self.cifr_mut()
    }

    #[doc="Read the CIFR register."]
    #[inline] pub fn cifr(&self) -> Cifr { 
        unsafe {
            read_volatile(self.cifr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the CICR register."]
    #[inline] pub fn cicr_mut(&self) -> *mut Cicr { 
        (self.0 + 0x20) as *mut Cicr
    }

    #[doc="Get the *const pointer for the CICR register."]
    #[inline] pub fn cicr_ptr(&self) -> *const Cicr { 
           self.cicr_mut()
    }

    #[doc="Write the CICR register."]
    #[inline] pub fn set_cicr<F: FnOnce(Cicr) -> Cicr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cicr_mut(), f(Cicr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB1RSTR register."]
    #[inline] pub fn ahb1rstr_mut(&self) -> *mut Ahb1rstr { 
        (self.0 + 0x28) as *mut Ahb1rstr
    }

    #[doc="Get the *const pointer for the AHB1RSTR register."]
    #[inline] pub fn ahb1rstr_ptr(&self) -> *const Ahb1rstr { 
           self.ahb1rstr_mut()
    }

    #[doc="Read the AHB1RSTR register."]
    #[inline] pub fn ahb1rstr(&self) -> Ahb1rstr { 
        unsafe {
            read_volatile(self.ahb1rstr_ptr())
        }
    }

    #[doc="Write the AHB1RSTR register."]
    #[inline] pub fn set_ahb1rstr<F: FnOnce(Ahb1rstr) -> Ahb1rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb1rstr_mut(), f(Ahb1rstr(0)));
        }
        self
    }

    #[doc="Modify the AHB1RSTR register."]
    #[inline] pub fn with_ahb1rstr<F: FnOnce(Ahb1rstr) -> Ahb1rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb1rstr_mut(), f(self.ahb1rstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB2RSTR register."]
    #[inline] pub fn ahb2rstr_mut(&self) -> *mut Ahb2rstr { 
        (self.0 + 0x2c) as *mut Ahb2rstr
    }

    #[doc="Get the *const pointer for the AHB2RSTR register."]
    #[inline] pub fn ahb2rstr_ptr(&self) -> *const Ahb2rstr { 
           self.ahb2rstr_mut()
    }

    #[doc="Read the AHB2RSTR register."]
    #[inline] pub fn ahb2rstr(&self) -> Ahb2rstr { 
        unsafe {
            read_volatile(self.ahb2rstr_ptr())
        }
    }

    #[doc="Write the AHB2RSTR register."]
    #[inline] pub fn set_ahb2rstr<F: FnOnce(Ahb2rstr) -> Ahb2rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb2rstr_mut(), f(Ahb2rstr(0)));
        }
        self
    }

    #[doc="Modify the AHB2RSTR register."]
    #[inline] pub fn with_ahb2rstr<F: FnOnce(Ahb2rstr) -> Ahb2rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb2rstr_mut(), f(self.ahb2rstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB3RSTR register."]
    #[inline] pub fn ahb3rstr_mut(&self) -> *mut Ahb3rstr { 
        (self.0 + 0x30) as *mut Ahb3rstr
    }

    #[doc="Get the *const pointer for the AHB3RSTR register."]
    #[inline] pub fn ahb3rstr_ptr(&self) -> *const Ahb3rstr { 
           self.ahb3rstr_mut()
    }

    #[doc="Read the AHB3RSTR register."]
    #[inline] pub fn ahb3rstr(&self) -> Ahb3rstr { 
        unsafe {
            read_volatile(self.ahb3rstr_ptr())
        }
    }

    #[doc="Write the AHB3RSTR register."]
    #[inline] pub fn set_ahb3rstr<F: FnOnce(Ahb3rstr) -> Ahb3rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb3rstr_mut(), f(Ahb3rstr(0)));
        }
        self
    }

    #[doc="Modify the AHB3RSTR register."]
    #[inline] pub fn with_ahb3rstr<F: FnOnce(Ahb3rstr) -> Ahb3rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb3rstr_mut(), f(self.ahb3rstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1RSTR1 register."]
    #[inline] pub fn apb1rstr1_mut(&self) -> *mut Apb1rstr1 { 
        (self.0 + 0x38) as *mut Apb1rstr1
    }

    #[doc="Get the *const pointer for the APB1RSTR1 register."]
    #[inline] pub fn apb1rstr1_ptr(&self) -> *const Apb1rstr1 { 
           self.apb1rstr1_mut()
    }

    #[doc="Read the APB1RSTR1 register."]
    #[inline] pub fn apb1rstr1(&self) -> Apb1rstr1 { 
        unsafe {
            read_volatile(self.apb1rstr1_ptr())
        }
    }

    #[doc="Write the APB1RSTR1 register."]
    #[inline] pub fn set_apb1rstr1<F: FnOnce(Apb1rstr1) -> Apb1rstr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1rstr1_mut(), f(Apb1rstr1(0)));
        }
        self
    }

    #[doc="Modify the APB1RSTR1 register."]
    #[inline] pub fn with_apb1rstr1<F: FnOnce(Apb1rstr1) -> Apb1rstr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1rstr1_mut(), f(self.apb1rstr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1RSTR2 register."]
    #[inline] pub fn apb1rstr2_mut(&self) -> *mut Apb1rstr2 { 
        (self.0 + 0x3c) as *mut Apb1rstr2
    }

    #[doc="Get the *const pointer for the APB1RSTR2 register."]
    #[inline] pub fn apb1rstr2_ptr(&self) -> *const Apb1rstr2 { 
           self.apb1rstr2_mut()
    }

    #[doc="Read the APB1RSTR2 register."]
    #[inline] pub fn apb1rstr2(&self) -> Apb1rstr2 { 
        unsafe {
            read_volatile(self.apb1rstr2_ptr())
        }
    }

    #[doc="Write the APB1RSTR2 register."]
    #[inline] pub fn set_apb1rstr2<F: FnOnce(Apb1rstr2) -> Apb1rstr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1rstr2_mut(), f(Apb1rstr2(0)));
        }
        self
    }

    #[doc="Modify the APB1RSTR2 register."]
    #[inline] pub fn with_apb1rstr2<F: FnOnce(Apb1rstr2) -> Apb1rstr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1rstr2_mut(), f(self.apb1rstr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2RSTR register."]
    #[inline] pub fn apb2rstr_mut(&self) -> *mut Apb2rstr { 
        (self.0 + 0x40) as *mut Apb2rstr
    }

    #[doc="Get the *const pointer for the APB2RSTR register."]
    #[inline] pub fn apb2rstr_ptr(&self) -> *const Apb2rstr { 
           self.apb2rstr_mut()
    }

    #[doc="Read the APB2RSTR register."]
    #[inline] pub fn apb2rstr(&self) -> Apb2rstr { 
        unsafe {
            read_volatile(self.apb2rstr_ptr())
        }
    }

    #[doc="Write the APB2RSTR register."]
    #[inline] pub fn set_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2rstr_mut(), f(Apb2rstr(0)));
        }
        self
    }

    #[doc="Modify the APB2RSTR register."]
    #[inline] pub fn with_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2rstr_mut(), f(self.apb2rstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB1ENR register."]
    #[inline] pub fn ahb1enr_mut(&self) -> *mut Ahb1enr { 
        (self.0 + 0x48) as *mut Ahb1enr
    }

    #[doc="Get the *const pointer for the AHB1ENR register."]
    #[inline] pub fn ahb1enr_ptr(&self) -> *const Ahb1enr { 
           self.ahb1enr_mut()
    }

    #[doc="Read the AHB1ENR register."]
    #[inline] pub fn ahb1enr(&self) -> Ahb1enr { 
        unsafe {
            read_volatile(self.ahb1enr_ptr())
        }
    }

    #[doc="Write the AHB1ENR register."]
    #[inline] pub fn set_ahb1enr<F: FnOnce(Ahb1enr) -> Ahb1enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb1enr_mut(), f(Ahb1enr(0)));
        }
        self
    }

    #[doc="Modify the AHB1ENR register."]
    #[inline] pub fn with_ahb1enr<F: FnOnce(Ahb1enr) -> Ahb1enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb1enr_mut(), f(self.ahb1enr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB2ENR register."]
    #[inline] pub fn ahb2enr_mut(&self) -> *mut Ahb2enr { 
        (self.0 + 0x4c) as *mut Ahb2enr
    }

    #[doc="Get the *const pointer for the AHB2ENR register."]
    #[inline] pub fn ahb2enr_ptr(&self) -> *const Ahb2enr { 
           self.ahb2enr_mut()
    }

    #[doc="Read the AHB2ENR register."]
    #[inline] pub fn ahb2enr(&self) -> Ahb2enr { 
        unsafe {
            read_volatile(self.ahb2enr_ptr())
        }
    }

    #[doc="Write the AHB2ENR register."]
    #[inline] pub fn set_ahb2enr<F: FnOnce(Ahb2enr) -> Ahb2enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb2enr_mut(), f(Ahb2enr(0)));
        }
        self
    }

    #[doc="Modify the AHB2ENR register."]
    #[inline] pub fn with_ahb2enr<F: FnOnce(Ahb2enr) -> Ahb2enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb2enr_mut(), f(self.ahb2enr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB3ENR register."]
    #[inline] pub fn ahb3enr_mut(&self) -> *mut Ahb3enr { 
        (self.0 + 0x50) as *mut Ahb3enr
    }

    #[doc="Get the *const pointer for the AHB3ENR register."]
    #[inline] pub fn ahb3enr_ptr(&self) -> *const Ahb3enr { 
           self.ahb3enr_mut()
    }

    #[doc="Read the AHB3ENR register."]
    #[inline] pub fn ahb3enr(&self) -> Ahb3enr { 
        unsafe {
            read_volatile(self.ahb3enr_ptr())
        }
    }

    #[doc="Write the AHB3ENR register."]
    #[inline] pub fn set_ahb3enr<F: FnOnce(Ahb3enr) -> Ahb3enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb3enr_mut(), f(Ahb3enr(0)));
        }
        self
    }

    #[doc="Modify the AHB3ENR register."]
    #[inline] pub fn with_ahb3enr<F: FnOnce(Ahb3enr) -> Ahb3enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb3enr_mut(), f(self.ahb3enr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1ENR1 register."]
    #[inline] pub fn apb1enr1_mut(&self) -> *mut Apb1enr1 { 
        (self.0 + 0x58) as *mut Apb1enr1
    }

    #[doc="Get the *const pointer for the APB1ENR1 register."]
    #[inline] pub fn apb1enr1_ptr(&self) -> *const Apb1enr1 { 
           self.apb1enr1_mut()
    }

    #[doc="Read the APB1ENR1 register."]
    #[inline] pub fn apb1enr1(&self) -> Apb1enr1 { 
        unsafe {
            read_volatile(self.apb1enr1_ptr())
        }
    }

    #[doc="Write the APB1ENR1 register."]
    #[inline] pub fn set_apb1enr1<F: FnOnce(Apb1enr1) -> Apb1enr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1enr1_mut(), f(Apb1enr1(0)));
        }
        self
    }

    #[doc="Modify the APB1ENR1 register."]
    #[inline] pub fn with_apb1enr1<F: FnOnce(Apb1enr1) -> Apb1enr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1enr1_mut(), f(self.apb1enr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1ENR2 register."]
    #[inline] pub fn apb1enr2_mut(&self) -> *mut Apb1enr2 { 
        (self.0 + 0x5c) as *mut Apb1enr2
    }

    #[doc="Get the *const pointer for the APB1ENR2 register."]
    #[inline] pub fn apb1enr2_ptr(&self) -> *const Apb1enr2 { 
           self.apb1enr2_mut()
    }

    #[doc="Read the APB1ENR2 register."]
    #[inline] pub fn apb1enr2(&self) -> Apb1enr2 { 
        unsafe {
            read_volatile(self.apb1enr2_ptr())
        }
    }

    #[doc="Write the APB1ENR2 register."]
    #[inline] pub fn set_apb1enr2<F: FnOnce(Apb1enr2) -> Apb1enr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1enr2_mut(), f(Apb1enr2(0)));
        }
        self
    }

    #[doc="Modify the APB1ENR2 register."]
    #[inline] pub fn with_apb1enr2<F: FnOnce(Apb1enr2) -> Apb1enr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1enr2_mut(), f(self.apb1enr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2ENR register."]
    #[inline] pub fn apb2enr_mut(&self) -> *mut Apb2enr { 
        (self.0 + 0x60) as *mut Apb2enr
    }

    #[doc="Get the *const pointer for the APB2ENR register."]
    #[inline] pub fn apb2enr_ptr(&self) -> *const Apb2enr { 
           self.apb2enr_mut()
    }

    #[doc="Read the APB2ENR register."]
    #[inline] pub fn apb2enr(&self) -> Apb2enr { 
        unsafe {
            read_volatile(self.apb2enr_ptr())
        }
    }

    #[doc="Write the APB2ENR register."]
    #[inline] pub fn set_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2enr_mut(), f(Apb2enr(0)));
        }
        self
    }

    #[doc="Modify the APB2ENR register."]
    #[inline] pub fn with_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2enr_mut(), f(self.apb2enr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB1SMENR register."]
    #[inline] pub fn ahb1smenr_mut(&self) -> *mut Ahb1smenr { 
        (self.0 + 0x68) as *mut Ahb1smenr
    }

    #[doc="Get the *const pointer for the AHB1SMENR register."]
    #[inline] pub fn ahb1smenr_ptr(&self) -> *const Ahb1smenr { 
           self.ahb1smenr_mut()
    }

    #[doc="Read the AHB1SMENR register."]
    #[inline] pub fn ahb1smenr(&self) -> Ahb1smenr { 
        unsafe {
            read_volatile(self.ahb1smenr_ptr())
        }
    }

    #[doc="Write the AHB1SMENR register."]
    #[inline] pub fn set_ahb1smenr<F: FnOnce(Ahb1smenr) -> Ahb1smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb1smenr_mut(), f(Ahb1smenr(0)));
        }
        self
    }

    #[doc="Modify the AHB1SMENR register."]
    #[inline] pub fn with_ahb1smenr<F: FnOnce(Ahb1smenr) -> Ahb1smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb1smenr_mut(), f(self.ahb1smenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB2SMENR register."]
    #[inline] pub fn ahb2smenr_mut(&self) -> *mut Ahb2smenr { 
        (self.0 + 0x6c) as *mut Ahb2smenr
    }

    #[doc="Get the *const pointer for the AHB2SMENR register."]
    #[inline] pub fn ahb2smenr_ptr(&self) -> *const Ahb2smenr { 
           self.ahb2smenr_mut()
    }

    #[doc="Read the AHB2SMENR register."]
    #[inline] pub fn ahb2smenr(&self) -> Ahb2smenr { 
        unsafe {
            read_volatile(self.ahb2smenr_ptr())
        }
    }

    #[doc="Write the AHB2SMENR register."]
    #[inline] pub fn set_ahb2smenr<F: FnOnce(Ahb2smenr) -> Ahb2smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb2smenr_mut(), f(Ahb2smenr(0)));
        }
        self
    }

    #[doc="Modify the AHB2SMENR register."]
    #[inline] pub fn with_ahb2smenr<F: FnOnce(Ahb2smenr) -> Ahb2smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb2smenr_mut(), f(self.ahb2smenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB3SMENR register."]
    #[inline] pub fn ahb3smenr_mut(&self) -> *mut Ahb3smenr { 
        (self.0 + 0x70) as *mut Ahb3smenr
    }

    #[doc="Get the *const pointer for the AHB3SMENR register."]
    #[inline] pub fn ahb3smenr_ptr(&self) -> *const Ahb3smenr { 
           self.ahb3smenr_mut()
    }

    #[doc="Read the AHB3SMENR register."]
    #[inline] pub fn ahb3smenr(&self) -> Ahb3smenr { 
        unsafe {
            read_volatile(self.ahb3smenr_ptr())
        }
    }

    #[doc="Write the AHB3SMENR register."]
    #[inline] pub fn set_ahb3smenr<F: FnOnce(Ahb3smenr) -> Ahb3smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb3smenr_mut(), f(Ahb3smenr(0)));
        }
        self
    }

    #[doc="Modify the AHB3SMENR register."]
    #[inline] pub fn with_ahb3smenr<F: FnOnce(Ahb3smenr) -> Ahb3smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb3smenr_mut(), f(self.ahb3smenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1SMENR1 register."]
    #[inline] pub fn apb1smenr1_mut(&self) -> *mut Apb1smenr1 { 
        (self.0 + 0x78) as *mut Apb1smenr1
    }

    #[doc="Get the *const pointer for the APB1SMENR1 register."]
    #[inline] pub fn apb1smenr1_ptr(&self) -> *const Apb1smenr1 { 
           self.apb1smenr1_mut()
    }

    #[doc="Read the APB1SMENR1 register."]
    #[inline] pub fn apb1smenr1(&self) -> Apb1smenr1 { 
        unsafe {
            read_volatile(self.apb1smenr1_ptr())
        }
    }

    #[doc="Write the APB1SMENR1 register."]
    #[inline] pub fn set_apb1smenr1<F: FnOnce(Apb1smenr1) -> Apb1smenr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1smenr1_mut(), f(Apb1smenr1(0)));
        }
        self
    }

    #[doc="Modify the APB1SMENR1 register."]
    #[inline] pub fn with_apb1smenr1<F: FnOnce(Apb1smenr1) -> Apb1smenr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1smenr1_mut(), f(self.apb1smenr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1SMENR2 register."]
    #[inline] pub fn apb1smenr2_mut(&self) -> *mut Apb1smenr2 { 
        (self.0 + 0x7c) as *mut Apb1smenr2
    }

    #[doc="Get the *const pointer for the APB1SMENR2 register."]
    #[inline] pub fn apb1smenr2_ptr(&self) -> *const Apb1smenr2 { 
           self.apb1smenr2_mut()
    }

    #[doc="Read the APB1SMENR2 register."]
    #[inline] pub fn apb1smenr2(&self) -> Apb1smenr2 { 
        unsafe {
            read_volatile(self.apb1smenr2_ptr())
        }
    }

    #[doc="Write the APB1SMENR2 register."]
    #[inline] pub fn set_apb1smenr2<F: FnOnce(Apb1smenr2) -> Apb1smenr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1smenr2_mut(), f(Apb1smenr2(0)));
        }
        self
    }

    #[doc="Modify the APB1SMENR2 register."]
    #[inline] pub fn with_apb1smenr2<F: FnOnce(Apb1smenr2) -> Apb1smenr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1smenr2_mut(), f(self.apb1smenr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2SMENR register."]
    #[inline] pub fn apb2smenr_mut(&self) -> *mut Apb2smenr { 
        (self.0 + 0x80) as *mut Apb2smenr
    }

    #[doc="Get the *const pointer for the APB2SMENR register."]
    #[inline] pub fn apb2smenr_ptr(&self) -> *const Apb2smenr { 
           self.apb2smenr_mut()
    }

    #[doc="Read the APB2SMENR register."]
    #[inline] pub fn apb2smenr(&self) -> Apb2smenr { 
        unsafe {
            read_volatile(self.apb2smenr_ptr())
        }
    }

    #[doc="Write the APB2SMENR register."]
    #[inline] pub fn set_apb2smenr<F: FnOnce(Apb2smenr) -> Apb2smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2smenr_mut(), f(Apb2smenr(0)));
        }
        self
    }

    #[doc="Modify the APB2SMENR register."]
    #[inline] pub fn with_apb2smenr<F: FnOnce(Apb2smenr) -> Apb2smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2smenr_mut(), f(self.apb2smenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CCIPR register."]
    #[inline] pub fn ccipr_mut(&self) -> *mut Ccipr { 
        (self.0 + 0x88) as *mut Ccipr
    }

    #[doc="Get the *const pointer for the CCIPR register."]
    #[inline] pub fn ccipr_ptr(&self) -> *const Ccipr { 
           self.ccipr_mut()
    }

    #[doc="Read the CCIPR register."]
    #[inline] pub fn ccipr(&self) -> Ccipr { 
        unsafe {
            read_volatile(self.ccipr_ptr())
        }
    }

    #[doc="Write the CCIPR register."]
    #[inline] pub fn set_ccipr<F: FnOnce(Ccipr) -> Ccipr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccipr_mut(), f(Ccipr(0)));
        }
        self
    }

    #[doc="Modify the CCIPR register."]
    #[inline] pub fn with_ccipr<F: FnOnce(Ccipr) -> Ccipr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccipr_mut(), f(self.ccipr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BDCR register."]
    #[inline] pub fn bdcr_mut(&self) -> *mut Bdcr { 
        (self.0 + 0x90) as *mut Bdcr
    }

    #[doc="Get the *const pointer for the BDCR register."]
    #[inline] pub fn bdcr_ptr(&self) -> *const Bdcr { 
           self.bdcr_mut()
    }

    #[doc="Read the BDCR register."]
    #[inline] pub fn bdcr(&self) -> Bdcr { 
        unsafe {
            read_volatile(self.bdcr_ptr())
        }
    }

    #[doc="Write the BDCR register."]
    #[inline] pub fn set_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bdcr_mut(), f(Bdcr(0)));
        }
        self
    }

    #[doc="Modify the BDCR register."]
    #[inline] pub fn with_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bdcr_mut(), f(self.bdcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        (self.0 + 0x94) as *mut Csr
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
           self.csr_mut()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        unsafe {
            read_volatile(self.csr_ptr())
        }
    }

    #[doc="Write the CSR register."]
    #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr_mut(), f(Csr(0)));
        }
        self
    }

    #[doc="Modify the CSR register."]
    #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr_mut(), f(self.csr()));
        }
        self
    }

}

#[doc="Clock control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="SAI2 PLL clock ready flag"]
    #[inline] pub fn pllsai2rdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PLLSAI2RDY != 0"]
    #[inline] pub fn test_pllsai2rdy(&self) -> bool {
        self.pllsai2rdy() != 0
    }

    #[doc="Sets the PLLSAI2RDY field."]
    #[inline] pub fn set_pllsai2rdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SAI2 PLL enable"]
    #[inline] pub fn pllsai2on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PLLSAI2ON != 0"]
    #[inline] pub fn test_pllsai2on(&self) -> bool {
        self.pllsai2on() != 0
    }

    #[doc="Sets the PLLSAI2ON field."]
    #[inline] pub fn set_pllsai2on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SAI1 PLL clock ready flag"]
    #[inline] pub fn pllsai1rdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PLLSAI1RDY != 0"]
    #[inline] pub fn test_pllsai1rdy(&self) -> bool {
        self.pllsai1rdy() != 0
    }

    #[doc="Sets the PLLSAI1RDY field."]
    #[inline] pub fn set_pllsai1rdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="SAI1 PLL enable"]
    #[inline] pub fn pllsai1on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PLLSAI1ON != 0"]
    #[inline] pub fn test_pllsai1on(&self) -> bool {
        self.pllsai1on() != 0
    }

    #[doc="Sets the PLLSAI1ON field."]
    #[inline] pub fn set_pllsai1on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Main PLL clock ready flag"]
    #[inline] pub fn pllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PLLRDY != 0"]
    #[inline] pub fn test_pllrdy(&self) -> bool {
        self.pllrdy() != 0
    }

    #[doc="Sets the PLLRDY field."]
    #[inline] pub fn set_pllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Main PLL enable"]
    #[inline] pub fn pllon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PLLON != 0"]
    #[inline] pub fn test_pllon(&self) -> bool {
        self.pllon() != 0
    }

    #[doc="Sets the PLLON field."]
    #[inline] pub fn set_pllon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock security system enable"]
    #[inline] pub fn csson(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CSSON != 0"]
    #[inline] pub fn test_csson(&self) -> bool {
        self.csson() != 0
    }

    #[doc="Sets the CSSON field."]
    #[inline] pub fn set_csson<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="HSE crystal oscillator bypass"]
    #[inline] pub fn hsebyp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if HSEBYP != 0"]
    #[inline] pub fn test_hsebyp(&self) -> bool {
        self.hsebyp() != 0
    }

    #[doc="Sets the HSEBYP field."]
    #[inline] pub fn set_hsebyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="HSE clock ready flag"]
    #[inline] pub fn hserdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if HSERDY != 0"]
    #[inline] pub fn test_hserdy(&self) -> bool {
        self.hserdy() != 0
    }

    #[doc="Sets the HSERDY field."]
    #[inline] pub fn set_hserdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="HSE clock enable"]
    #[inline] pub fn hseon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if HSEON != 0"]
    #[inline] pub fn test_hseon(&self) -> bool {
        self.hseon() != 0
    }

    #[doc="Sets the HSEON field."]
    #[inline] pub fn set_hseon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="HSI automatic start from Stop"]
    #[inline] pub fn hsiasfs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HSIASFS != 0"]
    #[inline] pub fn test_hsiasfs(&self) -> bool {
        self.hsiasfs() != 0
    }

    #[doc="Sets the HSIASFS field."]
    #[inline] pub fn set_hsiasfs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HSI clock ready flag"]
    #[inline] pub fn hsirdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSIRDY != 0"]
    #[inline] pub fn test_hsirdy(&self) -> bool {
        self.hsirdy() != 0
    }

    #[doc="Sets the HSIRDY field."]
    #[inline] pub fn set_hsirdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="HSI always enable for peripheral kernels"]
    #[inline] pub fn hsikeron(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HSIKERON != 0"]
    #[inline] pub fn test_hsikeron(&self) -> bool {
        self.hsikeron() != 0
    }

    #[doc="Sets the HSIKERON field."]
    #[inline] pub fn set_hsikeron<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HSI clock enable"]
    #[inline] pub fn hsion(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HSION != 0"]
    #[inline] pub fn test_hsion(&self) -> bool {
        self.hsion() != 0
    }

    #[doc="Sets the HSION field."]
    #[inline] pub fn set_hsion<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MSI clock ranges"]
    #[inline] pub fn msirange(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if MSIRANGE != 0"]
    #[inline] pub fn test_msirange(&self) -> bool {
        self.msirange() != 0
    }

    #[doc="Sets the MSIRANGE field."]
    #[inline] pub fn set_msirange<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MSI clock range selection"]
    #[inline] pub fn msirgsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MSIRGSEL != 0"]
    #[inline] pub fn test_msirgsel(&self) -> bool {
        self.msirgsel() != 0
    }

    #[doc="Sets the MSIRGSEL field."]
    #[inline] pub fn set_msirgsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MSI clock PLL enable"]
    #[inline] pub fn msipllen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSIPLLEN != 0"]
    #[inline] pub fn test_msipllen(&self) -> bool {
        self.msipllen() != 0
    }

    #[doc="Sets the MSIPLLEN field."]
    #[inline] pub fn set_msipllen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MSI clock ready flag"]
    #[inline] pub fn msirdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MSIRDY != 0"]
    #[inline] pub fn test_msirdy(&self) -> bool {
        self.msirdy() != 0
    }

    #[doc="Sets the MSIRDY field."]
    #[inline] pub fn set_msirdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MSI clock enable"]
    #[inline] pub fn msion(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MSION != 0"]
    #[inline] pub fn test_msion(&self) -> bool {
        self.msion() != 0
    }

    #[doc="Sets the MSION field."]
    #[inline] pub fn set_msion<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.pllsai2rdy() != 0 { try!(write!(f, " pllsai2rdy"))}
        if self.pllsai2on() != 0 { try!(write!(f, " pllsai2on"))}
        if self.pllsai1rdy() != 0 { try!(write!(f, " pllsai1rdy"))}
        if self.pllsai1on() != 0 { try!(write!(f, " pllsai1on"))}
        if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
        if self.pllon() != 0 { try!(write!(f, " pllon"))}
        if self.csson() != 0 { try!(write!(f, " csson"))}
        if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
        if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
        if self.hseon() != 0 { try!(write!(f, " hseon"))}
        if self.hsiasfs() != 0 { try!(write!(f, " hsiasfs"))}
        if self.hsirdy() != 0 { try!(write!(f, " hsirdy"))}
        if self.hsikeron() != 0 { try!(write!(f, " hsikeron"))}
        if self.hsion() != 0 { try!(write!(f, " hsion"))}
        if self.msirange() != 0 { try!(write!(f, " msirange=0x{:x}", self.msirange()))}
        if self.msirgsel() != 0 { try!(write!(f, " msirgsel"))}
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
    #[inline] pub fn hsitrim(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if HSITRIM != 0"]
    #[inline] pub fn test_hsitrim(&self) -> bool {
        self.hsitrim() != 0
    }

    #[doc="Sets the HSITRIM field."]
    #[inline] pub fn set_hsitrim<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="HSI clock calibration"]
    #[inline] pub fn hsical(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if HSICAL != 0"]
    #[inline] pub fn test_hsical(&self) -> bool {
        self.hsical() != 0
    }

    #[doc="Sets the HSICAL field."]
    #[inline] pub fn set_hsical<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MSI clock trimming"]
    #[inline] pub fn msitrim(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if MSITRIM != 0"]
    #[inline] pub fn test_msitrim(&self) -> bool {
        self.msitrim() != 0
    }

    #[doc="Sets the MSITRIM field."]
    #[inline] pub fn set_msitrim<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MSI clock calibration"]
    #[inline] pub fn msical(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MSICAL != 0"]
    #[inline] pub fn test_msical(&self) -> bool {
        self.msical() != 0
    }

    #[doc="Sets the MSICAL field."]
    #[inline] pub fn set_msical<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
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
    #[inline] pub fn mcopre(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if MCOPRE != 0"]
    #[inline] pub fn test_mcopre(&self) -> bool {
        self.mcopre() != 0
    }

    #[doc="Sets the MCOPRE field."]
    #[inline] pub fn set_mcopre<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Microcontroller clock output"]
    #[inline] pub fn mcosel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if MCOSEL != 0"]
    #[inline] pub fn test_mcosel(&self) -> bool {
        self.mcosel() != 0
    }

    #[doc="Sets the MCOSEL field."]
    #[inline] pub fn set_mcosel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Wakeup from Stop and CSS backup clock selection"]
    #[inline] pub fn stopwuck(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if STOPWUCK != 0"]
    #[inline] pub fn test_stopwuck(&self) -> bool {
        self.stopwuck() != 0
    }

    #[doc="Sets the STOPWUCK field."]
    #[inline] pub fn set_stopwuck<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="APB high-speed prescaler (APB2)"]
    #[inline] pub fn ppre2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PPRE2 != 0"]
    #[inline] pub fn test_ppre2(&self) -> bool {
        self.ppre2() != 0
    }

    #[doc="Sets the PPRE2 field."]
    #[inline] pub fn set_ppre2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PB low-speed prescaler (APB1)"]
    #[inline] pub fn ppre1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PPRE1 != 0"]
    #[inline] pub fn test_ppre1(&self) -> bool {
        self.ppre1() != 0
    }

    #[doc="Sets the PPRE1 field."]
    #[inline] pub fn set_ppre1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AHB prescaler"]
    #[inline] pub fn hpre(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if HPRE != 0"]
    #[inline] pub fn test_hpre(&self) -> bool {
        self.hpre() != 0
    }

    #[doc="Sets the HPRE field."]
    #[inline] pub fn set_hpre<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="System clock switch status"]
    #[inline] pub fn sws(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SWS != 0"]
    #[inline] pub fn test_sws(&self) -> bool {
        self.sws() != 0
    }

    #[doc="Sets the SWS field."]
    #[inline] pub fn set_sws<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="System clock switch"]
    #[inline] pub fn sw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SW != 0"]
    #[inline] pub fn test_sw(&self) -> bool {
        self.sw() != 0
    }

    #[doc="Sets the SW field."]
    #[inline] pub fn set_sw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
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

#[doc="PLL configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pllcfgr(pub u32);
impl Pllcfgr {
    #[doc="Main PLL division factor for PLLCLK (system clock)"]
    #[inline] pub fn pllr(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if PLLR != 0"]
    #[inline] pub fn test_pllr(&self) -> bool {
        self.pllr() != 0
    }

    #[doc="Sets the PLLR field."]
    #[inline] pub fn set_pllr<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Main PLL PLLCLK output enable"]
    #[inline] pub fn pllren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PLLREN != 0"]
    #[inline] pub fn test_pllren(&self) -> bool {
        self.pllren() != 0
    }

    #[doc="Sets the PLLREN field."]
    #[inline] pub fn set_pllren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline] pub fn pllq(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if PLLQ != 0"]
    #[inline] pub fn test_pllq(&self) -> bool {
        self.pllq() != 0
    }

    #[doc="Sets the PLLQ field."]
    #[inline] pub fn set_pllq<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Main PLL PLLUSB1CLK output enable"]
    #[inline] pub fn pllqen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PLLQEN != 0"]
    #[inline] pub fn test_pllqen(&self) -> bool {
        self.pllqen() != 0
    }

    #[doc="Sets the PLLQEN field."]
    #[inline] pub fn set_pllqen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline] pub fn pllp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PLLP != 0"]
    #[inline] pub fn test_pllp(&self) -> bool {
        self.pllp() != 0
    }

    #[doc="Sets the PLLP field."]
    #[inline] pub fn set_pllp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Main PLL PLLSAI3CLK output enable"]
    #[inline] pub fn pllpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PLLPEN != 0"]
    #[inline] pub fn test_pllpen(&self) -> bool {
        self.pllpen() != 0
    }

    #[doc="Sets the PLLPEN field."]
    #[inline] pub fn set_pllpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Main PLL multiplication factor for VCO"]
    #[inline] pub fn plln(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if PLLN != 0"]
    #[inline] pub fn test_plln(&self) -> bool {
        self.plln() != 0
    }

    #[doc="Sets the PLLN field."]
    #[inline] pub fn set_plln<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline] pub fn pllm(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if PLLM != 0"]
    #[inline] pub fn test_pllm(&self) -> bool {
        self.pllm() != 0
    }

    #[doc="Sets the PLLM field."]
    #[inline] pub fn set_pllm<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline] pub fn pllsrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PLLSRC != 0"]
    #[inline] pub fn test_pllsrc(&self) -> bool {
        self.pllsrc() != 0
    }

    #[doc="Sets the PLLSRC field."]
    #[inline] pub fn set_pllsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
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
        if self.pllp() != 0 { try!(write!(f, " pllp"))}
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
    #[doc="PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline] pub fn pllsai1r(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if PLLSAI1R != 0"]
    #[inline] pub fn test_pllsai1r(&self) -> bool {
        self.pllsai1r() != 0
    }

    #[doc="Sets the PLLSAI1R field."]
    #[inline] pub fn set_pllsai1r<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="PLLSAI1 PLLADC1CLK output enable"]
    #[inline] pub fn pllsai1ren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PLLSAI1REN != 0"]
    #[inline] pub fn test_pllsai1ren(&self) -> bool {
        self.pllsai1ren() != 0
    }

    #[doc="Sets the PLLSAI1REN field."]
    #[inline] pub fn set_pllsai1ren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline] pub fn pllsai1q(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if PLLSAI1Q != 0"]
    #[inline] pub fn test_pllsai1q(&self) -> bool {
        self.pllsai1q() != 0
    }

    #[doc="Sets the PLLSAI1Q field."]
    #[inline] pub fn set_pllsai1q<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SAI1PLL PLLUSB2CLK output enable"]
    #[inline] pub fn pllsai1qen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PLLSAI1QEN != 0"]
    #[inline] pub fn test_pllsai1qen(&self) -> bool {
        self.pllsai1qen() != 0
    }

    #[doc="Sets the PLLSAI1QEN field."]
    #[inline] pub fn set_pllsai1qen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline] pub fn pllsai1p(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PLLSAI1P != 0"]
    #[inline] pub fn test_pllsai1p(&self) -> bool {
        self.pllsai1p() != 0
    }

    #[doc="Sets the PLLSAI1P field."]
    #[inline] pub fn set_pllsai1p<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SAI1PLL PLLSAI1CLK output enable"]
    #[inline] pub fn pllsai1pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PLLSAI1PEN != 0"]
    #[inline] pub fn test_pllsai1pen(&self) -> bool {
        self.pllsai1pen() != 0
    }

    #[doc="Sets the PLLSAI1PEN field."]
    #[inline] pub fn set_pllsai1pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SAI1PLL multiplication factor for VCO"]
    #[inline] pub fn pllsai1n(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if PLLSAI1N != 0"]
    #[inline] pub fn test_pllsai1n(&self) -> bool {
        self.pllsai1n() != 0
    }

    #[doc="Sets the PLLSAI1N field."]
    #[inline] pub fn set_pllsai1n<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
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
        if self.pllsai1r() != 0 { try!(write!(f, " pllsai1r=0x{:x}", self.pllsai1r()))}
        if self.pllsai1ren() != 0 { try!(write!(f, " pllsai1ren"))}
        if self.pllsai1q() != 0 { try!(write!(f, " pllsai1q=0x{:x}", self.pllsai1q()))}
        if self.pllsai1qen() != 0 { try!(write!(f, " pllsai1qen"))}
        if self.pllsai1p() != 0 { try!(write!(f, " pllsai1p"))}
        if self.pllsai1pen() != 0 { try!(write!(f, " pllsai1pen"))}
        if self.pllsai1n() != 0 { try!(write!(f, " pllsai1n=0x{:x}", self.pllsai1n()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PLLSAI2 configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pllsai2cfgr(pub u32);
impl Pllsai2cfgr {
    #[doc="PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline] pub fn pllsai2r(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if PLLSAI2R != 0"]
    #[inline] pub fn test_pllsai2r(&self) -> bool {
        self.pllsai2r() != 0
    }

    #[doc="Sets the PLLSAI2R field."]
    #[inline] pub fn set_pllsai2r<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="PLLSAI2 PLLADC2CLK output enable"]
    #[inline] pub fn pllsai2ren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PLLSAI2REN != 0"]
    #[inline] pub fn test_pllsai2ren(&self) -> bool {
        self.pllsai2ren() != 0
    }

    #[doc="Sets the PLLSAI2REN field."]
    #[inline] pub fn set_pllsai2ren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline] pub fn pllsai2p(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PLLSAI2P != 0"]
    #[inline] pub fn test_pllsai2p(&self) -> bool {
        self.pllsai2p() != 0
    }

    #[doc="Sets the PLLSAI2P field."]
    #[inline] pub fn set_pllsai2p<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SAI2PLL PLLSAI2CLK output enable"]
    #[inline] pub fn pllsai2pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PLLSAI2PEN != 0"]
    #[inline] pub fn test_pllsai2pen(&self) -> bool {
        self.pllsai2pen() != 0
    }

    #[doc="Sets the PLLSAI2PEN field."]
    #[inline] pub fn set_pllsai2pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SAI2PLL multiplication factor for VCO"]
    #[inline] pub fn pllsai2n(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if PLLSAI2N != 0"]
    #[inline] pub fn test_pllsai2n(&self) -> bool {
        self.pllsai2n() != 0
    }

    #[doc="Sets the PLLSAI2N field."]
    #[inline] pub fn set_pllsai2n<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Pllsai2cfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Pllsai2cfgr(other)
    }
}

impl ::core::fmt::Display for Pllsai2cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pllsai2cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pllsai2r() != 0 { try!(write!(f, " pllsai2r=0x{:x}", self.pllsai2r()))}
        if self.pllsai2ren() != 0 { try!(write!(f, " pllsai2ren"))}
        if self.pllsai2p() != 0 { try!(write!(f, " pllsai2p"))}
        if self.pllsai2pen() != 0 { try!(write!(f, " pllsai2pen"))}
        if self.pllsai2n() != 0 { try!(write!(f, " pllsai2n=0x{:x}", self.pllsai2n()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cier(pub u32);
impl Cier {
    #[doc="LSE clock security system interrupt enable"]
    #[inline] pub fn lsecssie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSECSSIE != 0"]
    #[inline] pub fn test_lsecssie(&self) -> bool {
        self.lsecssie() != 0
    }

    #[doc="Sets the LSECSSIE field."]
    #[inline] pub fn set_lsecssie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="PLLSAI2 ready interrupt enable"]
    #[inline] pub fn pllsai2rdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PLLSAI2RDYIE != 0"]
    #[inline] pub fn test_pllsai2rdyie(&self) -> bool {
        self.pllsai2rdyie() != 0
    }

    #[doc="Sets the PLLSAI2RDYIE field."]
    #[inline] pub fn set_pllsai2rdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="PLLSAI1 ready interrupt enable"]
    #[inline] pub fn pllsai1rdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLSAI1RDYIE != 0"]
    #[inline] pub fn test_pllsai1rdyie(&self) -> bool {
        self.pllsai1rdyie() != 0
    }

    #[doc="Sets the PLLSAI1RDYIE field."]
    #[inline] pub fn set_pllsai1rdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PLL ready interrupt enable"]
    #[inline] pub fn pllrdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLRDYIE != 0"]
    #[inline] pub fn test_pllrdyie(&self) -> bool {
        self.pllrdyie() != 0
    }

    #[doc="Sets the PLLRDYIE field."]
    #[inline] pub fn set_pllrdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HSE ready interrupt enable"]
    #[inline] pub fn hserdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HSERDYIE != 0"]
    #[inline] pub fn test_hserdyie(&self) -> bool {
        self.hserdyie() != 0
    }

    #[doc="Sets the HSERDYIE field."]
    #[inline] pub fn set_hserdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSI ready interrupt enable"]
    #[inline] pub fn hsirdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSIRDYIE != 0"]
    #[inline] pub fn test_hsirdyie(&self) -> bool {
        self.hsirdyie() != 0
    }

    #[doc="Sets the HSIRDYIE field."]
    #[inline] pub fn set_hsirdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MSI ready interrupt enable"]
    #[inline] pub fn msirdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSIRDYIE != 0"]
    #[inline] pub fn test_msirdyie(&self) -> bool {
        self.msirdyie() != 0
    }

    #[doc="Sets the MSIRDYIE field."]
    #[inline] pub fn set_msirdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE ready interrupt enable"]
    #[inline] pub fn lserdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDYIE != 0"]
    #[inline] pub fn test_lserdyie(&self) -> bool {
        self.lserdyie() != 0
    }

    #[doc="Sets the LSERDYIE field."]
    #[inline] pub fn set_lserdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSI ready interrupt enable"]
    #[inline] pub fn lsirdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSIRDYIE != 0"]
    #[inline] pub fn test_lsirdyie(&self) -> bool {
        self.lsirdyie() != 0
    }

    #[doc="Sets the LSIRDYIE field."]
    #[inline] pub fn set_lsirdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.lsecssie() != 0 { try!(write!(f, " lsecssie"))}
        if self.pllsai2rdyie() != 0 { try!(write!(f, " pllsai2rdyie"))}
        if self.pllsai1rdyie() != 0 { try!(write!(f, " pllsai1rdyie"))}
        if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
        if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
        if self.hsirdyie() != 0 { try!(write!(f, " hsirdyie"))}
        if self.msirdyie() != 0 { try!(write!(f, " msirdyie"))}
        if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
        if self.lsirdyie() != 0 { try!(write!(f, " lsirdyie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock interrupt flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cifr(pub u32);
impl Cifr {
    #[doc="LSE Clock security system interrupt flag"]
    #[inline] pub fn lsecssf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSECSSF != 0"]
    #[inline] pub fn test_lsecssf(&self) -> bool {
        self.lsecssf() != 0
    }

    #[doc="Sets the LSECSSF field."]
    #[inline] pub fn set_lsecssf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clock security system interrupt flag"]
    #[inline] pub fn cssf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSSF != 0"]
    #[inline] pub fn test_cssf(&self) -> bool {
        self.cssf() != 0
    }

    #[doc="Sets the CSSF field."]
    #[inline] pub fn set_cssf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PLLSAI2 ready interrupt flag"]
    #[inline] pub fn pllsai2rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PLLSAI2RDYF != 0"]
    #[inline] pub fn test_pllsai2rdyf(&self) -> bool {
        self.pllsai2rdyf() != 0
    }

    #[doc="Sets the PLLSAI2RDYF field."]
    #[inline] pub fn set_pllsai2rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="PLLSAI1 ready interrupt flag"]
    #[inline] pub fn pllsai1rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLSAI1RDYF != 0"]
    #[inline] pub fn test_pllsai1rdyf(&self) -> bool {
        self.pllsai1rdyf() != 0
    }

    #[doc="Sets the PLLSAI1RDYF field."]
    #[inline] pub fn set_pllsai1rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PLL ready interrupt flag"]
    #[inline] pub fn pllrdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLRDYF != 0"]
    #[inline] pub fn test_pllrdyf(&self) -> bool {
        self.pllrdyf() != 0
    }

    #[doc="Sets the PLLRDYF field."]
    #[inline] pub fn set_pllrdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HSE ready interrupt flag"]
    #[inline] pub fn hserdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HSERDYF != 0"]
    #[inline] pub fn test_hserdyf(&self) -> bool {
        self.hserdyf() != 0
    }

    #[doc="Sets the HSERDYF field."]
    #[inline] pub fn set_hserdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSI ready interrupt flag"]
    #[inline] pub fn hsirdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSIRDYF != 0"]
    #[inline] pub fn test_hsirdyf(&self) -> bool {
        self.hsirdyf() != 0
    }

    #[doc="Sets the HSIRDYF field."]
    #[inline] pub fn set_hsirdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MSI ready interrupt flag"]
    #[inline] pub fn msirdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSIRDYF != 0"]
    #[inline] pub fn test_msirdyf(&self) -> bool {
        self.msirdyf() != 0
    }

    #[doc="Sets the MSIRDYF field."]
    #[inline] pub fn set_msirdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE ready interrupt flag"]
    #[inline] pub fn lserdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDYF != 0"]
    #[inline] pub fn test_lserdyf(&self) -> bool {
        self.lserdyf() != 0
    }

    #[doc="Sets the LSERDYF field."]
    #[inline] pub fn set_lserdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSI ready interrupt flag"]
    #[inline] pub fn lsirdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSIRDYF != 0"]
    #[inline] pub fn test_lsirdyf(&self) -> bool {
        self.lsirdyf() != 0
    }

    #[doc="Sets the LSIRDYF field."]
    #[inline] pub fn set_lsirdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.lsecssf() != 0 { try!(write!(f, " lsecssf"))}
        if self.cssf() != 0 { try!(write!(f, " cssf"))}
        if self.pllsai2rdyf() != 0 { try!(write!(f, " pllsai2rdyf"))}
        if self.pllsai1rdyf() != 0 { try!(write!(f, " pllsai1rdyf"))}
        if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
        if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
        if self.hsirdyf() != 0 { try!(write!(f, " hsirdyf"))}
        if self.msirdyf() != 0 { try!(write!(f, " msirdyf"))}
        if self.lserdyf() != 0 { try!(write!(f, " lserdyf"))}
        if self.lsirdyf() != 0 { try!(write!(f, " lsirdyf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cicr(pub u32);
impl Cicr {
    #[doc="LSE Clock security system interrupt clear"]
    #[inline] pub fn lsecssc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSECSSC != 0"]
    #[inline] pub fn test_lsecssc(&self) -> bool {
        self.lsecssc() != 0
    }

    #[doc="Sets the LSECSSC field."]
    #[inline] pub fn set_lsecssc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clock security system interrupt clear"]
    #[inline] pub fn cssc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSSC != 0"]
    #[inline] pub fn test_cssc(&self) -> bool {
        self.cssc() != 0
    }

    #[doc="Sets the CSSC field."]
    #[inline] pub fn set_cssc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PLLSAI2 ready interrupt clear"]
    #[inline] pub fn pllsai2rdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PLLSAI2RDYC != 0"]
    #[inline] pub fn test_pllsai2rdyc(&self) -> bool {
        self.pllsai2rdyc() != 0
    }

    #[doc="Sets the PLLSAI2RDYC field."]
    #[inline] pub fn set_pllsai2rdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="PLLSAI1 ready interrupt clear"]
    #[inline] pub fn pllsai1rdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLSAI1RDYC != 0"]
    #[inline] pub fn test_pllsai1rdyc(&self) -> bool {
        self.pllsai1rdyc() != 0
    }

    #[doc="Sets the PLLSAI1RDYC field."]
    #[inline] pub fn set_pllsai1rdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PLL ready interrupt clear"]
    #[inline] pub fn pllrdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLRDYC != 0"]
    #[inline] pub fn test_pllrdyc(&self) -> bool {
        self.pllrdyc() != 0
    }

    #[doc="Sets the PLLRDYC field."]
    #[inline] pub fn set_pllrdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="HSE ready interrupt clear"]
    #[inline] pub fn hserdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HSERDYC != 0"]
    #[inline] pub fn test_hserdyc(&self) -> bool {
        self.hserdyc() != 0
    }

    #[doc="Sets the HSERDYC field."]
    #[inline] pub fn set_hserdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSI ready interrupt clear"]
    #[inline] pub fn hsirdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSIRDYC != 0"]
    #[inline] pub fn test_hsirdyc(&self) -> bool {
        self.hsirdyc() != 0
    }

    #[doc="Sets the HSIRDYC field."]
    #[inline] pub fn set_hsirdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MSI ready interrupt clear"]
    #[inline] pub fn msirdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSIRDYC != 0"]
    #[inline] pub fn test_msirdyc(&self) -> bool {
        self.msirdyc() != 0
    }

    #[doc="Sets the MSIRDYC field."]
    #[inline] pub fn set_msirdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE ready interrupt clear"]
    #[inline] pub fn lserdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDYC != 0"]
    #[inline] pub fn test_lserdyc(&self) -> bool {
        self.lserdyc() != 0
    }

    #[doc="Sets the LSERDYC field."]
    #[inline] pub fn set_lserdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSI ready interrupt clear"]
    #[inline] pub fn lsirdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSIRDYC != 0"]
    #[inline] pub fn test_lsirdyc(&self) -> bool {
        self.lsirdyc() != 0
    }

    #[doc="Sets the LSIRDYC field."]
    #[inline] pub fn set_lsirdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.lsecssc() != 0 { try!(write!(f, " lsecssc"))}
        if self.cssc() != 0 { try!(write!(f, " cssc"))}
        if self.pllsai2rdyc() != 0 { try!(write!(f, " pllsai2rdyc"))}
        if self.pllsai1rdyc() != 0 { try!(write!(f, " pllsai1rdyc"))}
        if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
        if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
        if self.hsirdyc() != 0 { try!(write!(f, " hsirdyc"))}
        if self.msirdyc() != 0 { try!(write!(f, " msirdyc"))}
        if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
        if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1rstr(pub u32);
impl Ahb1rstr {
    #[doc="Touch Sensing Controller reset"]
    #[inline] pub fn tscrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSCRST != 0"]
    #[inline] pub fn test_tscrst(&self) -> bool {
        self.tscrst() != 0
    }

    #[doc="Sets the TSCRST field."]
    #[inline] pub fn set_tscrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Reserved"]
    #[inline] pub fn crcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CRCRST != 0"]
    #[inline] pub fn test_crcrst(&self) -> bool {
        self.crcrst() != 0
    }

    #[doc="Sets the CRCRST field."]
    #[inline] pub fn set_crcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Flash memory interface reset"]
    #[inline] pub fn flashrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FLASHRST != 0"]
    #[inline] pub fn test_flashrst(&self) -> bool {
        self.flashrst() != 0
    }

    #[doc="Sets the FLASHRST field."]
    #[inline] pub fn set_flashrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA2 reset"]
    #[inline] pub fn dma2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2RST != 0"]
    #[inline] pub fn test_dma2rst(&self) -> bool {
        self.dma2rst() != 0
    }

    #[doc="Sets the DMA2RST field."]
    #[inline] pub fn set_dma2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMA1 reset"]
    #[inline] pub fn dma1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1RST != 0"]
    #[inline] pub fn test_dma1rst(&self) -> bool {
        self.dma1rst() != 0
    }

    #[doc="Sets the DMA1RST field."]
    #[inline] pub fn set_dma1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.flashrst() != 0 { try!(write!(f, " flashrst"))}
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
    #[doc="Random number generator reset"]
    #[inline] pub fn rngrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RNGRST != 0"]
    #[inline] pub fn test_rngrst(&self) -> bool {
        self.rngrst() != 0
    }

    #[doc="Sets the RNGRST field."]
    #[inline] pub fn set_rngrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="AES hardware accelerator reset"]
    #[inline] pub fn aesrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AESRST != 0"]
    #[inline] pub fn test_aesrst(&self) -> bool {
        self.aesrst() != 0
    }

    #[doc="Sets the AESRST field."]
    #[inline] pub fn set_aesrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ADC reset"]
    #[inline] pub fn adcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCRST != 0"]
    #[inline] pub fn test_adcrst(&self) -> bool {
        self.adcrst() != 0
    }

    #[doc="Sets the ADCRST field."]
    #[inline] pub fn set_adcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="USB OTG FS reset"]
    #[inline] pub fn otgfsrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if OTGFSRST != 0"]
    #[inline] pub fn test_otgfsrst(&self) -> bool {
        self.otgfsrst() != 0
    }

    #[doc="Sets the OTGFSRST field."]
    #[inline] pub fn set_otgfsrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IO port H reset"]
    #[inline] pub fn gpiohrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHRST != 0"]
    #[inline] pub fn test_gpiohrst(&self) -> bool {
        self.gpiohrst() != 0
    }

    #[doc="Sets the GPIOHRST field."]
    #[inline] pub fn set_gpiohrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IO port G reset"]
    #[inline] pub fn gpiogrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIOGRST != 0"]
    #[inline] pub fn test_gpiogrst(&self) -> bool {
        self.gpiogrst() != 0
    }

    #[doc="Sets the GPIOGRST field."]
    #[inline] pub fn set_gpiogrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IO port F reset"]
    #[inline] pub fn gpiofrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIOFRST != 0"]
    #[inline] pub fn test_gpiofrst(&self) -> bool {
        self.gpiofrst() != 0
    }

    #[doc="Sets the GPIOFRST field."]
    #[inline] pub fn set_gpiofrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IO port E reset"]
    #[inline] pub fn gpioerst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOERST != 0"]
    #[inline] pub fn test_gpioerst(&self) -> bool {
        self.gpioerst() != 0
    }

    #[doc="Sets the GPIOERST field."]
    #[inline] pub fn set_gpioerst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IO port D reset"]
    #[inline] pub fn gpiodrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODRST != 0"]
    #[inline] pub fn test_gpiodrst(&self) -> bool {
        self.gpiodrst() != 0
    }

    #[doc="Sets the GPIODRST field."]
    #[inline] pub fn set_gpiodrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IO port C reset"]
    #[inline] pub fn gpiocrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCRST != 0"]
    #[inline] pub fn test_gpiocrst(&self) -> bool {
        self.gpiocrst() != 0
    }

    #[doc="Sets the GPIOCRST field."]
    #[inline] pub fn set_gpiocrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IO port B reset"]
    #[inline] pub fn gpiobrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBRST != 0"]
    #[inline] pub fn test_gpiobrst(&self) -> bool {
        self.gpiobrst() != 0
    }

    #[doc="Sets the GPIOBRST field."]
    #[inline] pub fn set_gpiobrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IO port A reset"]
    #[inline] pub fn gpioarst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOARST != 0"]
    #[inline] pub fn test_gpioarst(&self) -> bool {
        self.gpioarst() != 0
    }

    #[doc="Sets the GPIOARST field."]
    #[inline] pub fn set_gpioarst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.rngrst() != 0 { try!(write!(f, " rngrst"))}
        if self.aesrst() != 0 { try!(write!(f, " aesrst"))}
        if self.adcrst() != 0 { try!(write!(f, " adcrst"))}
        if self.otgfsrst() != 0 { try!(write!(f, " otgfsrst"))}
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

#[doc="AHB3 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb3rstr(pub u32);
impl Ahb3rstr {
    #[doc="Quad SPI memory interface reset"]
    #[inline] pub fn qspirst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if QSPIRST != 0"]
    #[inline] pub fn test_qspirst(&self) -> bool {
        self.qspirst() != 0
    }

    #[doc="Sets the QSPIRST field."]
    #[inline] pub fn set_qspirst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Flexible memory controller reset"]
    #[inline] pub fn fmcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FMCRST != 0"]
    #[inline] pub fn test_fmcrst(&self) -> bool {
        self.fmcrst() != 0
    }

    #[doc="Sets the FMCRST field."]
    #[inline] pub fn set_fmcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.qspirst() != 0 { try!(write!(f, " qspirst"))}
        if self.fmcrst() != 0 { try!(write!(f, " fmcrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral reset register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1rstr1(pub u32);
impl Apb1rstr1 {
    #[doc="Low Power Timer 1 reset"]
    #[inline] pub fn lptim1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPTIM1RST != 0"]
    #[inline] pub fn test_lptim1rst(&self) -> bool {
        self.lptim1rst() != 0
    }

    #[doc="Sets the LPTIM1RST field."]
    #[inline] pub fn set_lptim1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="OPAMP interface reset"]
    #[inline] pub fn opamprst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OPAMPRST != 0"]
    #[inline] pub fn test_opamprst(&self) -> bool {
        self.opamprst() != 0
    }

    #[doc="Sets the OPAMPRST field."]
    #[inline] pub fn set_opamprst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DAC1 interface reset"]
    #[inline] pub fn dac1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DAC1RST != 0"]
    #[inline] pub fn test_dac1rst(&self) -> bool {
        self.dac1rst() != 0
    }

    #[doc="Sets the DAC1RST field."]
    #[inline] pub fn set_dac1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Power interface reset"]
    #[inline] pub fn pwrrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWRRST != 0"]
    #[inline] pub fn test_pwrrst(&self) -> bool {
        self.pwrrst() != 0
    }

    #[doc="Sets the PWRRST field."]
    #[inline] pub fn set_pwrrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="CAN1 reset"]
    #[inline] pub fn can1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CAN1RST != 0"]
    #[inline] pub fn test_can1rst(&self) -> bool {
        self.can1rst() != 0
    }

    #[doc="Sets the CAN1RST field."]
    #[inline] pub fn set_can1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="I2C3 reset"]
    #[inline] pub fn i2c3rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3RST != 0"]
    #[inline] pub fn test_i2c3rst(&self) -> bool {
        self.i2c3rst() != 0
    }

    #[doc="Sets the I2C3RST field."]
    #[inline] pub fn set_i2c3rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C2 reset"]
    #[inline] pub fn i2c2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2RST != 0"]
    #[inline] pub fn test_i2c2rst(&self) -> bool {
        self.i2c2rst() != 0
    }

    #[doc="Sets the I2C2RST field."]
    #[inline] pub fn set_i2c2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C1 reset"]
    #[inline] pub fn i2c1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1RST != 0"]
    #[inline] pub fn test_i2c1rst(&self) -> bool {
        self.i2c1rst() != 0
    }

    #[doc="Sets the I2C1RST field."]
    #[inline] pub fn set_i2c1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="UART5 reset"]
    #[inline] pub fn uart5rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5RST != 0"]
    #[inline] pub fn test_uart5rst(&self) -> bool {
        self.uart5rst() != 0
    }

    #[doc="Sets the UART5RST field."]
    #[inline] pub fn set_uart5rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="UART4 reset"]
    #[inline] pub fn uart4rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4RST != 0"]
    #[inline] pub fn test_uart4rst(&self) -> bool {
        self.uart4rst() != 0
    }

    #[doc="Sets the UART4RST field."]
    #[inline] pub fn set_uart4rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="USART3 reset"]
    #[inline] pub fn usart3rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USART3RST != 0"]
    #[inline] pub fn test_usart3rst(&self) -> bool {
        self.usart3rst() != 0
    }

    #[doc="Sets the USART3RST field."]
    #[inline] pub fn set_usart3rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USART2 reset"]
    #[inline] pub fn usart2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if USART2RST != 0"]
    #[inline] pub fn test_usart2rst(&self) -> bool {
        self.usart2rst() != 0
    }

    #[doc="Sets the USART2RST field."]
    #[inline] pub fn set_usart2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI3 reset"]
    #[inline] pub fn spi3rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SPI3RST != 0"]
    #[inline] pub fn test_spi3rst(&self) -> bool {
        self.spi3rst() != 0
    }

    #[doc="Sets the SPI3RST field."]
    #[inline] pub fn set_spi3rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SPI2 reset"]
    #[inline] pub fn spi2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2RST != 0"]
    #[inline] pub fn test_spi2rst(&self) -> bool {
        self.spi2rst() != 0
    }

    #[doc="Sets the SPI2RST field."]
    #[inline] pub fn set_spi2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="LCD interface reset"]
    #[inline] pub fn lcdrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LCDRST != 0"]
    #[inline] pub fn test_lcdrst(&self) -> bool {
        self.lcdrst() != 0
    }

    #[doc="Sets the LCDRST field."]
    #[inline] pub fn set_lcdrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TIM7 timer reset"]
    #[inline] pub fn tim7rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7RST != 0"]
    #[inline] pub fn test_tim7rst(&self) -> bool {
        self.tim7rst() != 0
    }

    #[doc="Sets the TIM7RST field."]
    #[inline] pub fn set_tim7rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM6 timer reset"]
    #[inline] pub fn tim6rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6RST != 0"]
    #[inline] pub fn test_tim6rst(&self) -> bool {
        self.tim6rst() != 0
    }

    #[doc="Sets the TIM6RST field."]
    #[inline] pub fn set_tim6rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM5 timer reset"]
    #[inline] pub fn tim5rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TIM5RST != 0"]
    #[inline] pub fn test_tim5rst(&self) -> bool {
        self.tim5rst() != 0
    }

    #[doc="Sets the TIM5RST field."]
    #[inline] pub fn set_tim5rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TIM3 timer reset"]
    #[inline] pub fn tim4rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4RST != 0"]
    #[inline] pub fn test_tim4rst(&self) -> bool {
        self.tim4rst() != 0
    }

    #[doc="Sets the TIM4RST field."]
    #[inline] pub fn set_tim4rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TIM3 timer reset"]
    #[inline] pub fn tim3rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3RST != 0"]
    #[inline] pub fn test_tim3rst(&self) -> bool {
        self.tim3rst() != 0
    }

    #[doc="Sets the TIM3RST field."]
    #[inline] pub fn set_tim3rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM2 timer reset"]
    #[inline] pub fn tim2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2RST != 0"]
    #[inline] pub fn test_tim2rst(&self) -> bool {
        self.tim2rst() != 0
    }

    #[doc="Sets the TIM2RST field."]
    #[inline] pub fn set_tim2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.opamprst() != 0 { try!(write!(f, " opamprst"))}
        if self.dac1rst() != 0 { try!(write!(f, " dac1rst"))}
        if self.pwrrst() != 0 { try!(write!(f, " pwrrst"))}
        if self.can1rst() != 0 { try!(write!(f, " can1rst"))}
        if self.i2c3rst() != 0 { try!(write!(f, " i2c3rst"))}
        if self.i2c2rst() != 0 { try!(write!(f, " i2c2rst"))}
        if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
        if self.uart5rst() != 0 { try!(write!(f, " uart5rst"))}
        if self.uart4rst() != 0 { try!(write!(f, " uart4rst"))}
        if self.usart3rst() != 0 { try!(write!(f, " usart3rst"))}
        if self.usart2rst() != 0 { try!(write!(f, " usart2rst"))}
        if self.spi3rst() != 0 { try!(write!(f, " spi3rst"))}
        if self.spi2rst() != 0 { try!(write!(f, " spi2rst"))}
        if self.lcdrst() != 0 { try!(write!(f, " lcdrst"))}
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

#[doc="APB1 peripheral reset register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1rstr2(pub u32);
impl Apb1rstr2 {
    #[doc="Low-power timer 2 reset"]
    #[inline] pub fn lptim2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LPTIM2RST != 0"]
    #[inline] pub fn test_lptim2rst(&self) -> bool {
        self.lptim2rst() != 0
    }

    #[doc="Sets the LPTIM2RST field."]
    #[inline] pub fn set_lptim2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Single wire protocol reset"]
    #[inline] pub fn swpmi1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SWPMI1RST != 0"]
    #[inline] pub fn test_swpmi1rst(&self) -> bool {
        self.swpmi1rst() != 0
    }

    #[doc="Sets the SWPMI1RST field."]
    #[inline] pub fn set_swpmi1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Low-power UART 1 reset"]
    #[inline] pub fn lpuart1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPUART1RST != 0"]
    #[inline] pub fn test_lpuart1rst(&self) -> bool {
        self.lpuart1rst() != 0
    }

    #[doc="Sets the LPUART1RST field."]
    #[inline] pub fn set_lpuart1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.swpmi1rst() != 0 { try!(write!(f, " swpmi1rst"))}
        if self.lpuart1rst() != 0 { try!(write!(f, " lpuart1rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
    #[doc="Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline] pub fn dfsdmrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DFSDMRST != 0"]
    #[inline] pub fn test_dfsdmrst(&self) -> bool {
        self.dfsdmrst() != 0
    }

    #[doc="Sets the DFSDMRST field."]
    #[inline] pub fn set_dfsdmrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Serial audio interface 2 (SAI2) reset"]
    #[inline] pub fn sai2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SAI2RST != 0"]
    #[inline] pub fn test_sai2rst(&self) -> bool {
        self.sai2rst() != 0
    }

    #[doc="Sets the SAI2RST field."]
    #[inline] pub fn set_sai2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Serial audio interface 1 (SAI1) reset"]
    #[inline] pub fn sai1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SAI1RST != 0"]
    #[inline] pub fn test_sai1rst(&self) -> bool {
        self.sai1rst() != 0
    }

    #[doc="Sets the SAI1RST field."]
    #[inline] pub fn set_sai1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="TIM17 timer reset"]
    #[inline] pub fn tim17rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17RST != 0"]
    #[inline] pub fn test_tim17rst(&self) -> bool {
        self.tim17rst() != 0
    }

    #[doc="Sets the TIM17RST field."]
    #[inline] pub fn set_tim17rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM16 timer reset"]
    #[inline] pub fn tim16rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16RST != 0"]
    #[inline] pub fn test_tim16rst(&self) -> bool {
        self.tim16rst() != 0
    }

    #[doc="Sets the TIM16RST field."]
    #[inline] pub fn set_tim16rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM15 timer reset"]
    #[inline] pub fn tim15rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM15RST != 0"]
    #[inline] pub fn test_tim15rst(&self) -> bool {
        self.tim15rst() != 0
    }

    #[doc="Sets the TIM15RST field."]
    #[inline] pub fn set_tim15rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USART1 reset"]
    #[inline] pub fn usart1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1RST != 0"]
    #[inline] pub fn test_usart1rst(&self) -> bool {
        self.usart1rst() != 0
    }

    #[doc="Sets the USART1RST field."]
    #[inline] pub fn set_usart1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TIM8 timer reset"]
    #[inline] pub fn tim8rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TIM8RST != 0"]
    #[inline] pub fn test_tim8rst(&self) -> bool {
        self.tim8rst() != 0
    }

    #[doc="Sets the TIM8RST field."]
    #[inline] pub fn set_tim8rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="SPI1 reset"]
    #[inline] pub fn spi1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1RST != 0"]
    #[inline] pub fn test_spi1rst(&self) -> bool {
        self.spi1rst() != 0
    }

    #[doc="Sets the SPI1RST field."]
    #[inline] pub fn set_spi1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM1 timer reset"]
    #[inline] pub fn tim1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1RST != 0"]
    #[inline] pub fn test_tim1rst(&self) -> bool {
        self.tim1rst() != 0
    }

    #[doc="Sets the TIM1RST field."]
    #[inline] pub fn set_tim1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SDMMC reset"]
    #[inline] pub fn sdmmcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SDMMCRST != 0"]
    #[inline] pub fn test_sdmmcrst(&self) -> bool {
        self.sdmmcrst() != 0
    }

    #[doc="Sets the SDMMCRST field."]
    #[inline] pub fn set_sdmmcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="System configuration (SYSCFG) reset"]
    #[inline] pub fn syscfgrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYSCFGRST != 0"]
    #[inline] pub fn test_syscfgrst(&self) -> bool {
        self.syscfgrst() != 0
    }

    #[doc="Sets the SYSCFGRST field."]
    #[inline] pub fn set_syscfgrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.dfsdmrst() != 0 { try!(write!(f, " dfsdmrst"))}
        if self.sai2rst() != 0 { try!(write!(f, " sai2rst"))}
        if self.sai1rst() != 0 { try!(write!(f, " sai1rst"))}
        if self.tim17rst() != 0 { try!(write!(f, " tim17rst"))}
        if self.tim16rst() != 0 { try!(write!(f, " tim16rst"))}
        if self.tim15rst() != 0 { try!(write!(f, " tim15rst"))}
        if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
        if self.tim8rst() != 0 { try!(write!(f, " tim8rst"))}
        if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
        if self.tim1rst() != 0 { try!(write!(f, " tim1rst"))}
        if self.sdmmcrst() != 0 { try!(write!(f, " sdmmcrst"))}
        if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1enr(pub u32);
impl Ahb1enr {
    #[doc="Touch Sensing Controller clock enable"]
    #[inline] pub fn tscen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSCEN != 0"]
    #[inline] pub fn test_tscen(&self) -> bool {
        self.tscen() != 0
    }

    #[doc="Sets the TSCEN field."]
    #[inline] pub fn set_tscen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Reserved"]
    #[inline] pub fn crcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CRCEN != 0"]
    #[inline] pub fn test_crcen(&self) -> bool {
        self.crcen() != 0
    }

    #[doc="Sets the CRCEN field."]
    #[inline] pub fn set_crcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Flash memory interface clock enable"]
    #[inline] pub fn flashen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FLASHEN != 0"]
    #[inline] pub fn test_flashen(&self) -> bool {
        self.flashen() != 0
    }

    #[doc="Sets the FLASHEN field."]
    #[inline] pub fn set_flashen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA2 clock enable"]
    #[inline] pub fn dma2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2EN != 0"]
    #[inline] pub fn test_dma2en(&self) -> bool {
        self.dma2en() != 0
    }

    #[doc="Sets the DMA2EN field."]
    #[inline] pub fn set_dma2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMA1 clock enable"]
    #[inline] pub fn dma1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1EN != 0"]
    #[inline] pub fn test_dma1en(&self) -> bool {
        self.dma1en() != 0
    }

    #[doc="Sets the DMA1EN field."]
    #[inline] pub fn set_dma1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.flashen() != 0 { try!(write!(f, " flashen"))}
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
    #[doc="Random Number Generator clock enable"]
    #[inline] pub fn rngen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RNGEN != 0"]
    #[inline] pub fn test_rngen(&self) -> bool {
        self.rngen() != 0
    }

    #[doc="Sets the RNGEN field."]
    #[inline] pub fn set_rngen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="AES accelerator clock enable"]
    #[inline] pub fn aesen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AESEN != 0"]
    #[inline] pub fn test_aesen(&self) -> bool {
        self.aesen() != 0
    }

    #[doc="Sets the AESEN field."]
    #[inline] pub fn set_aesen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ADC clock enable"]
    #[inline] pub fn adcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCEN != 0"]
    #[inline] pub fn test_adcen(&self) -> bool {
        self.adcen() != 0
    }

    #[doc="Sets the ADCEN field."]
    #[inline] pub fn set_adcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="OTG full speed clock enable"]
    #[inline] pub fn otgfsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if OTGFSEN != 0"]
    #[inline] pub fn test_otgfsen(&self) -> bool {
        self.otgfsen() != 0
    }

    #[doc="Sets the OTGFSEN field."]
    #[inline] pub fn set_otgfsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IO port H clock enable"]
    #[inline] pub fn gpiohen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHEN != 0"]
    #[inline] pub fn test_gpiohen(&self) -> bool {
        self.gpiohen() != 0
    }

    #[doc="Sets the GPIOHEN field."]
    #[inline] pub fn set_gpiohen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IO port G clock enable"]
    #[inline] pub fn gpiogen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIOGEN != 0"]
    #[inline] pub fn test_gpiogen(&self) -> bool {
        self.gpiogen() != 0
    }

    #[doc="Sets the GPIOGEN field."]
    #[inline] pub fn set_gpiogen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IO port F clock enable"]
    #[inline] pub fn gpiofen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIOFEN != 0"]
    #[inline] pub fn test_gpiofen(&self) -> bool {
        self.gpiofen() != 0
    }

    #[doc="Sets the GPIOFEN field."]
    #[inline] pub fn set_gpiofen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IO port E clock enable"]
    #[inline] pub fn gpioeen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOEEN != 0"]
    #[inline] pub fn test_gpioeen(&self) -> bool {
        self.gpioeen() != 0
    }

    #[doc="Sets the GPIOEEN field."]
    #[inline] pub fn set_gpioeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IO port D clock enable"]
    #[inline] pub fn gpioden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODEN != 0"]
    #[inline] pub fn test_gpioden(&self) -> bool {
        self.gpioden() != 0
    }

    #[doc="Sets the GPIODEN field."]
    #[inline] pub fn set_gpioden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IO port C clock enable"]
    #[inline] pub fn gpiocen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCEN != 0"]
    #[inline] pub fn test_gpiocen(&self) -> bool {
        self.gpiocen() != 0
    }

    #[doc="Sets the GPIOCEN field."]
    #[inline] pub fn set_gpiocen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IO port B clock enable"]
    #[inline] pub fn gpioben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBEN != 0"]
    #[inline] pub fn test_gpioben(&self) -> bool {
        self.gpioben() != 0
    }

    #[doc="Sets the GPIOBEN field."]
    #[inline] pub fn set_gpioben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IO port A clock enable"]
    #[inline] pub fn gpioaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOAEN != 0"]
    #[inline] pub fn test_gpioaen(&self) -> bool {
        self.gpioaen() != 0
    }

    #[doc="Sets the GPIOAEN field."]
    #[inline] pub fn set_gpioaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.rngen() != 0 { try!(write!(f, " rngen"))}
        if self.aesen() != 0 { try!(write!(f, " aesen"))}
        if self.adcen() != 0 { try!(write!(f, " adcen"))}
        if self.otgfsen() != 0 { try!(write!(f, " otgfsen"))}
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

#[doc="AHB3 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb3enr(pub u32);
impl Ahb3enr {
    #[doc="QSPIEN"]
    #[inline] pub fn qspien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if QSPIEN != 0"]
    #[inline] pub fn test_qspien(&self) -> bool {
        self.qspien() != 0
    }

    #[doc="Sets the QSPIEN field."]
    #[inline] pub fn set_qspien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Flexible memory controller clock enable"]
    #[inline] pub fn fmcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FMCEN != 0"]
    #[inline] pub fn test_fmcen(&self) -> bool {
        self.fmcen() != 0
    }

    #[doc="Sets the FMCEN field."]
    #[inline] pub fn set_fmcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.qspien() != 0 { try!(write!(f, " qspien"))}
        if self.fmcen() != 0 { try!(write!(f, " fmcen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1ENR1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1enr1(pub u32);
impl Apb1enr1 {
    #[doc="Low power timer 1 clock enable"]
    #[inline] pub fn lptim1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPTIM1EN != 0"]
    #[inline] pub fn test_lptim1en(&self) -> bool {
        self.lptim1en() != 0
    }

    #[doc="Sets the LPTIM1EN field."]
    #[inline] pub fn set_lptim1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="OPAMP interface clock enable"]
    #[inline] pub fn opampen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OPAMPEN != 0"]
    #[inline] pub fn test_opampen(&self) -> bool {
        self.opampen() != 0
    }

    #[doc="Sets the OPAMPEN field."]
    #[inline] pub fn set_opampen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DAC1 interface clock enable"]
    #[inline] pub fn dac1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DAC1EN != 0"]
    #[inline] pub fn test_dac1en(&self) -> bool {
        self.dac1en() != 0
    }

    #[doc="Sets the DAC1EN field."]
    #[inline] pub fn set_dac1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Power interface clock enable"]
    #[inline] pub fn pwren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWREN != 0"]
    #[inline] pub fn test_pwren(&self) -> bool {
        self.pwren() != 0
    }

    #[doc="Sets the PWREN field."]
    #[inline] pub fn set_pwren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="CAN1 clock enable"]
    #[inline] pub fn can1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CAN1EN != 0"]
    #[inline] pub fn test_can1en(&self) -> bool {
        self.can1en() != 0
    }

    #[doc="Sets the CAN1EN field."]
    #[inline] pub fn set_can1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="I2C3 clock enable"]
    #[inline] pub fn i2c3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3EN != 0"]
    #[inline] pub fn test_i2c3en(&self) -> bool {
        self.i2c3en() != 0
    }

    #[doc="Sets the I2C3EN field."]
    #[inline] pub fn set_i2c3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C2 clock enable"]
    #[inline] pub fn i2c2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2EN != 0"]
    #[inline] pub fn test_i2c2en(&self) -> bool {
        self.i2c2en() != 0
    }

    #[doc="Sets the I2C2EN field."]
    #[inline] pub fn set_i2c2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C1 clock enable"]
    #[inline] pub fn i2c1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1EN != 0"]
    #[inline] pub fn test_i2c1en(&self) -> bool {
        self.i2c1en() != 0
    }

    #[doc="Sets the I2C1EN field."]
    #[inline] pub fn set_i2c1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="UART5 clock enable"]
    #[inline] pub fn uart5en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5EN != 0"]
    #[inline] pub fn test_uart5en(&self) -> bool {
        self.uart5en() != 0
    }

    #[doc="Sets the UART5EN field."]
    #[inline] pub fn set_uart5en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="UART4 clock enable"]
    #[inline] pub fn uart4en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4EN != 0"]
    #[inline] pub fn test_uart4en(&self) -> bool {
        self.uart4en() != 0
    }

    #[doc="Sets the UART4EN field."]
    #[inline] pub fn set_uart4en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="USART3 clock enable"]
    #[inline] pub fn usart3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USART3EN != 0"]
    #[inline] pub fn test_usart3en(&self) -> bool {
        self.usart3en() != 0
    }

    #[doc="Sets the USART3EN field."]
    #[inline] pub fn set_usart3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USART2 clock enable"]
    #[inline] pub fn usart2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if USART2EN != 0"]
    #[inline] pub fn test_usart2en(&self) -> bool {
        self.usart2en() != 0
    }

    #[doc="Sets the USART2EN field."]
    #[inline] pub fn set_usart2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI3 clock enable"]
    #[inline] pub fn spi3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SPI3EN != 0"]
    #[inline] pub fn test_spi3en(&self) -> bool {
        self.spi3en() != 0
    }

    #[doc="Sets the SPI3EN field."]
    #[inline] pub fn set_spi3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SPI2 clock enable"]
    #[inline] pub fn spi2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2EN != 0"]
    #[inline] pub fn test_spi2en(&self) -> bool {
        self.spi2en() != 0
    }

    #[doc="Sets the SPI2EN field."]
    #[inline] pub fn set_spi2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Window watchdog clock enable"]
    #[inline] pub fn wwdgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGEN != 0"]
    #[inline] pub fn test_wwdgen(&self) -> bool {
        self.wwdgen() != 0
    }

    #[doc="Sets the WWDGEN field."]
    #[inline] pub fn set_wwdgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="LCD clock enable"]
    #[inline] pub fn lcden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LCDEN != 0"]
    #[inline] pub fn test_lcden(&self) -> bool {
        self.lcden() != 0
    }

    #[doc="Sets the LCDEN field."]
    #[inline] pub fn set_lcden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TIM7 timer clock enable"]
    #[inline] pub fn tim7en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7EN != 0"]
    #[inline] pub fn test_tim7en(&self) -> bool {
        self.tim7en() != 0
    }

    #[doc="Sets the TIM7EN field."]
    #[inline] pub fn set_tim7en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM6 timer clock enable"]
    #[inline] pub fn tim6en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6EN != 0"]
    #[inline] pub fn test_tim6en(&self) -> bool {
        self.tim6en() != 0
    }

    #[doc="Sets the TIM6EN field."]
    #[inline] pub fn set_tim6en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Reserved"]
    #[inline] pub fn tim5en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TIM5EN != 0"]
    #[inline] pub fn test_tim5en(&self) -> bool {
        self.tim5en() != 0
    }

    #[doc="Sets the TIM5EN field."]
    #[inline] pub fn set_tim5en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TIM4 timer clock enable"]
    #[inline] pub fn tim4en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4EN != 0"]
    #[inline] pub fn test_tim4en(&self) -> bool {
        self.tim4en() != 0
    }

    #[doc="Sets the TIM4EN field."]
    #[inline] pub fn set_tim4en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TIM3 timer clock enable"]
    #[inline] pub fn tim3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3EN != 0"]
    #[inline] pub fn test_tim3en(&self) -> bool {
        self.tim3en() != 0
    }

    #[doc="Sets the TIM3EN field."]
    #[inline] pub fn set_tim3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM2 timer clock enable"]
    #[inline] pub fn tim2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2EN != 0"]
    #[inline] pub fn test_tim2en(&self) -> bool {
        self.tim2en() != 0
    }

    #[doc="Sets the TIM2EN field."]
    #[inline] pub fn set_tim2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.opampen() != 0 { try!(write!(f, " opampen"))}
        if self.dac1en() != 0 { try!(write!(f, " dac1en"))}
        if self.pwren() != 0 { try!(write!(f, " pwren"))}
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
        if self.lcden() != 0 { try!(write!(f, " lcden"))}
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

#[doc="APB1 peripheral clock enable register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1enr2(pub u32);
impl Apb1enr2 {
    #[doc="LPTIM2EN"]
    #[inline] pub fn lptim2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LPTIM2EN != 0"]
    #[inline] pub fn test_lptim2en(&self) -> bool {
        self.lptim2en() != 0
    }

    #[doc="Sets the LPTIM2EN field."]
    #[inline] pub fn set_lptim2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Single wire protocol clock enable"]
    #[inline] pub fn swpmi1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SWPMI1EN != 0"]
    #[inline] pub fn test_swpmi1en(&self) -> bool {
        self.swpmi1en() != 0
    }

    #[doc="Sets the SWPMI1EN field."]
    #[inline] pub fn set_swpmi1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Low power UART 1 clock enable"]
    #[inline] pub fn lpuart1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPUART1EN != 0"]
    #[inline] pub fn test_lpuart1en(&self) -> bool {
        self.lpuart1en() != 0
    }

    #[doc="Sets the LPUART1EN field."]
    #[inline] pub fn set_lpuart1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.swpmi1en() != 0 { try!(write!(f, " swpmi1en"))}
        if self.lpuart1en() != 0 { try!(write!(f, " lpuart1en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2ENR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
    #[doc="DFSDM timer clock enable"]
    #[inline] pub fn dfsdmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DFSDMEN != 0"]
    #[inline] pub fn test_dfsdmen(&self) -> bool {
        self.dfsdmen() != 0
    }

    #[doc="Sets the DFSDMEN field."]
    #[inline] pub fn set_dfsdmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SAI2 clock enable"]
    #[inline] pub fn sai2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SAI2EN != 0"]
    #[inline] pub fn test_sai2en(&self) -> bool {
        self.sai2en() != 0
    }

    #[doc="Sets the SAI2EN field."]
    #[inline] pub fn set_sai2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="SAI1 clock enable"]
    #[inline] pub fn sai1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SAI1EN != 0"]
    #[inline] pub fn test_sai1en(&self) -> bool {
        self.sai1en() != 0
    }

    #[doc="Sets the SAI1EN field."]
    #[inline] pub fn set_sai1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="TIM17 timer clock enable"]
    #[inline] pub fn tim17en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17EN != 0"]
    #[inline] pub fn test_tim17en(&self) -> bool {
        self.tim17en() != 0
    }

    #[doc="Sets the TIM17EN field."]
    #[inline] pub fn set_tim17en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM16 timer clock enable"]
    #[inline] pub fn tim16en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16EN != 0"]
    #[inline] pub fn test_tim16en(&self) -> bool {
        self.tim16en() != 0
    }

    #[doc="Sets the TIM16EN field."]
    #[inline] pub fn set_tim16en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM15 timer clock enable"]
    #[inline] pub fn tim15en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM15EN != 0"]
    #[inline] pub fn test_tim15en(&self) -> bool {
        self.tim15en() != 0
    }

    #[doc="Sets the TIM15EN field."]
    #[inline] pub fn set_tim15en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USART1clock enable"]
    #[inline] pub fn usart1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1EN != 0"]
    #[inline] pub fn test_usart1en(&self) -> bool {
        self.usart1en() != 0
    }

    #[doc="Sets the USART1EN field."]
    #[inline] pub fn set_usart1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TIM8 timer clock enable"]
    #[inline] pub fn tim8en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TIM8EN != 0"]
    #[inline] pub fn test_tim8en(&self) -> bool {
        self.tim8en() != 0
    }

    #[doc="Sets the TIM8EN field."]
    #[inline] pub fn set_tim8en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="SPI1 clock enable"]
    #[inline] pub fn spi1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1EN != 0"]
    #[inline] pub fn test_spi1en(&self) -> bool {
        self.spi1en() != 0
    }

    #[doc="Sets the SPI1EN field."]
    #[inline] pub fn set_spi1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM1 timer clock enable"]
    #[inline] pub fn tim1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1EN != 0"]
    #[inline] pub fn test_tim1en(&self) -> bool {
        self.tim1en() != 0
    }

    #[doc="Sets the TIM1EN field."]
    #[inline] pub fn set_tim1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SDMMC clock enable"]
    #[inline] pub fn sdmmcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SDMMCEN != 0"]
    #[inline] pub fn test_sdmmcen(&self) -> bool {
        self.sdmmcen() != 0
    }

    #[doc="Sets the SDMMCEN field."]
    #[inline] pub fn set_sdmmcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Firewall clock enable"]
    #[inline] pub fn firewallen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FIREWALLEN != 0"]
    #[inline] pub fn test_firewallen(&self) -> bool {
        self.firewallen() != 0
    }

    #[doc="Sets the FIREWALLEN field."]
    #[inline] pub fn set_firewallen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SYSCFG clock enable"]
    #[inline] pub fn syscfgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYSCFGEN != 0"]
    #[inline] pub fn test_syscfgen(&self) -> bool {
        self.syscfgen() != 0
    }

    #[doc="Sets the SYSCFGEN field."]
    #[inline] pub fn set_syscfgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.dfsdmen() != 0 { try!(write!(f, " dfsdmen"))}
        if self.sai2en() != 0 { try!(write!(f, " sai2en"))}
        if self.sai1en() != 0 { try!(write!(f, " sai1en"))}
        if self.tim17en() != 0 { try!(write!(f, " tim17en"))}
        if self.tim16en() != 0 { try!(write!(f, " tim16en"))}
        if self.tim15en() != 0 { try!(write!(f, " tim15en"))}
        if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
        if self.tim8en() != 0 { try!(write!(f, " tim8en"))}
        if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
        if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
        if self.sdmmcen() != 0 { try!(write!(f, " sdmmcen"))}
        if self.firewallen() != 0 { try!(write!(f, " firewallen"))}
        if self.syscfgen() != 0 { try!(write!(f, " syscfgen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB1 peripheral clocks enable in Sleep and Stop modes register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahb1smenr(pub u32);
impl Ahb1smenr {
    #[doc="Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tscsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TSCSMEN != 0"]
    #[inline] pub fn test_tscsmen(&self) -> bool {
        self.tscsmen() != 0
    }

    #[doc="Sets the TSCSMEN field."]
    #[inline] pub fn set_tscsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CRCSMEN"]
    #[inline] pub fn crcsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CRCSMEN != 0"]
    #[inline] pub fn test_crcsmen(&self) -> bool {
        self.crcsmen() != 0
    }

    #[doc="Sets the CRCSMEN field."]
    #[inline] pub fn set_crcsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline] pub fn sram1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SRAM1SMEN != 0"]
    #[inline] pub fn test_sram1smen(&self) -> bool {
        self.sram1smen() != 0
    }

    #[doc="Sets the SRAM1SMEN field."]
    #[inline] pub fn set_sram1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline] pub fn flashsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FLASHSMEN != 0"]
    #[inline] pub fn test_flashsmen(&self) -> bool {
        self.flashsmen() != 0
    }

    #[doc="Sets the FLASHSMEN field."]
    #[inline] pub fn set_flashsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA2 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dma2smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2SMEN != 0"]
    #[inline] pub fn test_dma2smen(&self) -> bool {
        self.dma2smen() != 0
    }

    #[doc="Sets the DMA2SMEN field."]
    #[inline] pub fn set_dma2smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMA1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dma1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1SMEN != 0"]
    #[inline] pub fn test_dma1smen(&self) -> bool {
        self.dma1smen() != 0
    }

    #[doc="Sets the DMA1SMEN field."]
    #[inline] pub fn set_dma1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.flashsmen() != 0 { try!(write!(f, " flashsmen"))}
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
    #[doc="Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline] pub fn rngsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RNGSMEN != 0"]
    #[inline] pub fn test_rngsmen(&self) -> bool {
        self.rngsmen() != 0
    }

    #[doc="Sets the RNGSMEN field."]
    #[inline] pub fn set_rngsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline] pub fn aessmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AESSMEN != 0"]
    #[inline] pub fn test_aessmen(&self) -> bool {
        self.aessmen() != 0
    }

    #[doc="Sets the AESSMEN field."]
    #[inline] pub fn set_aessmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ADC clocks enable during Sleep and Stop modes"]
    #[inline] pub fn adcfssmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ADCFSSMEN != 0"]
    #[inline] pub fn test_adcfssmen(&self) -> bool {
        self.adcfssmen() != 0
    }

    #[doc="Sets the ADCFSSMEN field."]
    #[inline] pub fn set_adcfssmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline] pub fn otgfssmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if OTGFSSMEN != 0"]
    #[inline] pub fn test_otgfssmen(&self) -> bool {
        self.otgfssmen() != 0
    }

    #[doc="Sets the OTGFSSMEN field."]
    #[inline] pub fn set_otgfssmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline] pub fn sram2smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SRAM2SMEN != 0"]
    #[inline] pub fn test_sram2smen(&self) -> bool {
        self.sram2smen() != 0
    }

    #[doc="Sets the SRAM2SMEN field."]
    #[inline] pub fn set_sram2smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IO port H clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiohsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHSMEN != 0"]
    #[inline] pub fn test_gpiohsmen(&self) -> bool {
        self.gpiohsmen() != 0
    }

    #[doc="Sets the GPIOHSMEN field."]
    #[inline] pub fn set_gpiohsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IO port G clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiogsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIOGSMEN != 0"]
    #[inline] pub fn test_gpiogsmen(&self) -> bool {
        self.gpiogsmen() != 0
    }

    #[doc="Sets the GPIOGSMEN field."]
    #[inline] pub fn set_gpiogsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IO port F clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiofsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIOFSMEN != 0"]
    #[inline] pub fn test_gpiofsmen(&self) -> bool {
        self.gpiofsmen() != 0
    }

    #[doc="Sets the GPIOFSMEN field."]
    #[inline] pub fn set_gpiofsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IO port E clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpioesmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOESMEN != 0"]
    #[inline] pub fn test_gpioesmen(&self) -> bool {
        self.gpioesmen() != 0
    }

    #[doc="Sets the GPIOESMEN field."]
    #[inline] pub fn set_gpioesmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IO port D clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiodsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODSMEN != 0"]
    #[inline] pub fn test_gpiodsmen(&self) -> bool {
        self.gpiodsmen() != 0
    }

    #[doc="Sets the GPIODSMEN field."]
    #[inline] pub fn set_gpiodsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IO port C clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiocsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCSMEN != 0"]
    #[inline] pub fn test_gpiocsmen(&self) -> bool {
        self.gpiocsmen() != 0
    }

    #[doc="Sets the GPIOCSMEN field."]
    #[inline] pub fn set_gpiocsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IO port B clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpiobsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBSMEN != 0"]
    #[inline] pub fn test_gpiobsmen(&self) -> bool {
        self.gpiobsmen() != 0
    }

    #[doc="Sets the GPIOBSMEN field."]
    #[inline] pub fn set_gpiobsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IO port A clocks enable during Sleep and Stop modes"]
    #[inline] pub fn gpioasmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOASMEN != 0"]
    #[inline] pub fn test_gpioasmen(&self) -> bool {
        self.gpioasmen() != 0
    }

    #[doc="Sets the GPIOASMEN field."]
    #[inline] pub fn set_gpioasmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.rngsmen() != 0 { try!(write!(f, " rngsmen"))}
        if self.aessmen() != 0 { try!(write!(f, " aessmen"))}
        if self.adcfssmen() != 0 { try!(write!(f, " adcfssmen"))}
        if self.otgfssmen() != 0 { try!(write!(f, " otgfssmen"))}
        if self.sram2smen() != 0 { try!(write!(f, " sram2smen"))}
        if self.gpiohsmen() != 0 { try!(write!(f, " gpiohsmen"))}
        if self.gpiogsmen() != 0 { try!(write!(f, " gpiogsmen"))}
        if self.gpiofsmen() != 0 { try!(write!(f, " gpiofsmen"))}
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
    #[doc="QSPISMEN"]
    #[inline] pub fn qspismen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if QSPISMEN != 0"]
    #[inline] pub fn test_qspismen(&self) -> bool {
        self.qspismen() != 0
    }

    #[doc="Sets the QSPISMEN field."]
    #[inline] pub fn set_qspismen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline] pub fn fmcsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FMCSMEN != 0"]
    #[inline] pub fn test_fmcsmen(&self) -> bool {
        self.fmcsmen() != 0
    }

    #[doc="Sets the FMCSMEN field."]
    #[inline] pub fn set_fmcsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.qspismen() != 0 { try!(write!(f, " qspismen"))}
        if self.fmcsmen() != 0 { try!(write!(f, " fmcsmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1SMENR1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1smenr1(pub u32);
impl Apb1smenr1 {
    #[doc="Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn lptim1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPTIM1SMEN != 0"]
    #[inline] pub fn test_lptim1smen(&self) -> bool {
        self.lptim1smen() != 0
    }

    #[doc="Sets the LPTIM1SMEN field."]
    #[inline] pub fn set_lptim1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline] pub fn opampsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OPAMPSMEN != 0"]
    #[inline] pub fn test_opampsmen(&self) -> bool {
        self.opampsmen() != 0
    }

    #[doc="Sets the OPAMPSMEN field."]
    #[inline] pub fn set_opampsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dac1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DAC1SMEN != 0"]
    #[inline] pub fn test_dac1smen(&self) -> bool {
        self.dac1smen() != 0
    }

    #[doc="Sets the DAC1SMEN field."]
    #[inline] pub fn set_dac1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Power interface clocks enable during Sleep and Stop modes"]
    #[inline] pub fn pwrsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWRSMEN != 0"]
    #[inline] pub fn test_pwrsmen(&self) -> bool {
        self.pwrsmen() != 0
    }

    #[doc="Sets the PWRSMEN field."]
    #[inline] pub fn set_pwrsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="CAN1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn can1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CAN1SMEN != 0"]
    #[inline] pub fn test_can1smen(&self) -> bool {
        self.can1smen() != 0
    }

    #[doc="Sets the CAN1SMEN field."]
    #[inline] pub fn set_can1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="I2C3 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn i2c3smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3SMEN != 0"]
    #[inline] pub fn test_i2c3smen(&self) -> bool {
        self.i2c3smen() != 0
    }

    #[doc="Sets the I2C3SMEN field."]
    #[inline] pub fn set_i2c3smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C2 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn i2c2smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2SMEN != 0"]
    #[inline] pub fn test_i2c2smen(&self) -> bool {
        self.i2c2smen() != 0
    }

    #[doc="Sets the I2C2SMEN field."]
    #[inline] pub fn set_i2c2smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn i2c1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1SMEN != 0"]
    #[inline] pub fn test_i2c1smen(&self) -> bool {
        self.i2c1smen() != 0
    }

    #[doc="Sets the I2C1SMEN field."]
    #[inline] pub fn set_i2c1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="UART5 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn uart5smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5SMEN != 0"]
    #[inline] pub fn test_uart5smen(&self) -> bool {
        self.uart5smen() != 0
    }

    #[doc="Sets the UART5SMEN field."]
    #[inline] pub fn set_uart5smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="UART4 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn uart4smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4SMEN != 0"]
    #[inline] pub fn test_uart4smen(&self) -> bool {
        self.uart4smen() != 0
    }

    #[doc="Sets the UART4SMEN field."]
    #[inline] pub fn set_uart4smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="USART3 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn usart3smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USART3SMEN != 0"]
    #[inline] pub fn test_usart3smen(&self) -> bool {
        self.usart3smen() != 0
    }

    #[doc="Sets the USART3SMEN field."]
    #[inline] pub fn set_usart3smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USART2 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn usart2smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if USART2SMEN != 0"]
    #[inline] pub fn test_usart2smen(&self) -> bool {
        self.usart2smen() != 0
    }

    #[doc="Sets the USART2SMEN field."]
    #[inline] pub fn set_usart2smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI3 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn sp3smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SP3SMEN != 0"]
    #[inline] pub fn test_sp3smen(&self) -> bool {
        self.sp3smen() != 0
    }

    #[doc="Sets the SP3SMEN field."]
    #[inline] pub fn set_sp3smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SPI2 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn spi2smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2SMEN != 0"]
    #[inline] pub fn test_spi2smen(&self) -> bool {
        self.spi2smen() != 0
    }

    #[doc="Sets the SPI2SMEN field."]
    #[inline] pub fn set_spi2smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline] pub fn wwdgsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGSMEN != 0"]
    #[inline] pub fn test_wwdgsmen(&self) -> bool {
        self.wwdgsmen() != 0
    }

    #[doc="Sets the WWDGSMEN field."]
    #[inline] pub fn set_wwdgsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="LCD clocks enable during Sleep and Stop modes"]
    #[inline] pub fn lcdsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LCDSMEN != 0"]
    #[inline] pub fn test_lcdsmen(&self) -> bool {
        self.lcdsmen() != 0
    }

    #[doc="Sets the LCDSMEN field."]
    #[inline] pub fn set_lcdsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim7smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7SMEN != 0"]
    #[inline] pub fn test_tim7smen(&self) -> bool {
        self.tim7smen() != 0
    }

    #[doc="Sets the TIM7SMEN field."]
    #[inline] pub fn set_tim7smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim6smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6SMEN != 0"]
    #[inline] pub fn test_tim6smen(&self) -> bool {
        self.tim6smen() != 0
    }

    #[doc="Sets the TIM6SMEN field."]
    #[inline] pub fn set_tim6smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Reserved"]
    #[inline] pub fn tim5smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TIM5SMEN != 0"]
    #[inline] pub fn test_tim5smen(&self) -> bool {
        self.tim5smen() != 0
    }

    #[doc="Sets the TIM5SMEN field."]
    #[inline] pub fn set_tim5smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim4smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4SMEN != 0"]
    #[inline] pub fn test_tim4smen(&self) -> bool {
        self.tim4smen() != 0
    }

    #[doc="Sets the TIM4SMEN field."]
    #[inline] pub fn set_tim4smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim3smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3SMEN != 0"]
    #[inline] pub fn test_tim3smen(&self) -> bool {
        self.tim3smen() != 0
    }

    #[doc="Sets the TIM3SMEN field."]
    #[inline] pub fn set_tim3smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim2smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2SMEN != 0"]
    #[inline] pub fn test_tim2smen(&self) -> bool {
        self.tim2smen() != 0
    }

    #[doc="Sets the TIM2SMEN field."]
    #[inline] pub fn set_tim2smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.opampsmen() != 0 { try!(write!(f, " opampsmen"))}
        if self.dac1smen() != 0 { try!(write!(f, " dac1smen"))}
        if self.pwrsmen() != 0 { try!(write!(f, " pwrsmen"))}
        if self.can1smen() != 0 { try!(write!(f, " can1smen"))}
        if self.i2c3smen() != 0 { try!(write!(f, " i2c3smen"))}
        if self.i2c2smen() != 0 { try!(write!(f, " i2c2smen"))}
        if self.i2c1smen() != 0 { try!(write!(f, " i2c1smen"))}
        if self.uart5smen() != 0 { try!(write!(f, " uart5smen"))}
        if self.uart4smen() != 0 { try!(write!(f, " uart4smen"))}
        if self.usart3smen() != 0 { try!(write!(f, " usart3smen"))}
        if self.usart2smen() != 0 { try!(write!(f, " usart2smen"))}
        if self.sp3smen() != 0 { try!(write!(f, " sp3smen"))}
        if self.spi2smen() != 0 { try!(write!(f, " spi2smen"))}
        if self.wwdgsmen() != 0 { try!(write!(f, " wwdgsmen"))}
        if self.lcdsmen() != 0 { try!(write!(f, " lcdsmen"))}
        if self.tim7smen() != 0 { try!(write!(f, " tim7smen"))}
        if self.tim6smen() != 0 { try!(write!(f, " tim6smen"))}
        if self.tim5smen() != 0 { try!(write!(f, " tim5smen"))}
        if self.tim4smen() != 0 { try!(write!(f, " tim4smen"))}
        if self.tim3smen() != 0 { try!(write!(f, " tim3smen"))}
        if self.tim2smen() != 0 { try!(write!(f, " tim2smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1smenr2(pub u32);
impl Apb1smenr2 {
    #[doc="LPTIM2SMEN"]
    #[inline] pub fn lptim2smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LPTIM2SMEN != 0"]
    #[inline] pub fn test_lptim2smen(&self) -> bool {
        self.lptim2smen() != 0
    }

    #[doc="Sets the LPTIM2SMEN field."]
    #[inline] pub fn set_lptim2smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Single wire protocol clocks enable during Sleep and Stop modes"]
    #[inline] pub fn swpmi1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SWPMI1SMEN != 0"]
    #[inline] pub fn test_swpmi1smen(&self) -> bool {
        self.swpmi1smen() != 0
    }

    #[doc="Sets the SWPMI1SMEN field."]
    #[inline] pub fn set_swpmi1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn lpuart1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPUART1SMEN != 0"]
    #[inline] pub fn test_lpuart1smen(&self) -> bool {
        self.lpuart1smen() != 0
    }

    #[doc="Sets the LPUART1SMEN field."]
    #[inline] pub fn set_lpuart1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.swpmi1smen() != 0 { try!(write!(f, " swpmi1smen"))}
        if self.lpuart1smen() != 0 { try!(write!(f, " lpuart1smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2SMENR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2smenr(pub u32);
impl Apb2smenr {
    #[doc="DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn dfsdmsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DFSDMSMEN != 0"]
    #[inline] pub fn test_dfsdmsmen(&self) -> bool {
        self.dfsdmsmen() != 0
    }

    #[doc="Sets the DFSDMSMEN field."]
    #[inline] pub fn set_dfsdmsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SAI2 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn sai2smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SAI2SMEN != 0"]
    #[inline] pub fn test_sai2smen(&self) -> bool {
        self.sai2smen() != 0
    }

    #[doc="Sets the SAI2SMEN field."]
    #[inline] pub fn set_sai2smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="SAI1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn sai1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SAI1SMEN != 0"]
    #[inline] pub fn test_sai1smen(&self) -> bool {
        self.sai1smen() != 0
    }

    #[doc="Sets the SAI1SMEN field."]
    #[inline] pub fn set_sai1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim17smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17SMEN != 0"]
    #[inline] pub fn test_tim17smen(&self) -> bool {
        self.tim17smen() != 0
    }

    #[doc="Sets the TIM17SMEN field."]
    #[inline] pub fn set_tim17smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim16smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16SMEN != 0"]
    #[inline] pub fn test_tim16smen(&self) -> bool {
        self.tim16smen() != 0
    }

    #[doc="Sets the TIM16SMEN field."]
    #[inline] pub fn set_tim16smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim15smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM15SMEN != 0"]
    #[inline] pub fn test_tim15smen(&self) -> bool {
        self.tim15smen() != 0
    }

    #[doc="Sets the TIM15SMEN field."]
    #[inline] pub fn set_tim15smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USART1clocks enable during Sleep and Stop modes"]
    #[inline] pub fn usart1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1SMEN != 0"]
    #[inline] pub fn test_usart1smen(&self) -> bool {
        self.usart1smen() != 0
    }

    #[doc="Sets the USART1SMEN field."]
    #[inline] pub fn set_usart1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim8smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TIM8SMEN != 0"]
    #[inline] pub fn test_tim8smen(&self) -> bool {
        self.tim8smen() != 0
    }

    #[doc="Sets the TIM8SMEN field."]
    #[inline] pub fn set_tim8smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="SPI1 clocks enable during Sleep and Stop modes"]
    #[inline] pub fn spi1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1SMEN != 0"]
    #[inline] pub fn test_spi1smen(&self) -> bool {
        self.spi1smen() != 0
    }

    #[doc="Sets the SPI1SMEN field."]
    #[inline] pub fn set_spi1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline] pub fn tim1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1SMEN != 0"]
    #[inline] pub fn test_tim1smen(&self) -> bool {
        self.tim1smen() != 0
    }

    #[doc="Sets the TIM1SMEN field."]
    #[inline] pub fn set_tim1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SDMMC clocks enable during Sleep and Stop modes"]
    #[inline] pub fn sdmmcsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SDMMCSMEN != 0"]
    #[inline] pub fn test_sdmmcsmen(&self) -> bool {
        self.sdmmcsmen() != 0
    }

    #[doc="Sets the SDMMCSMEN field."]
    #[inline] pub fn set_sdmmcsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline] pub fn syscfgsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYSCFGSMEN != 0"]
    #[inline] pub fn test_syscfgsmen(&self) -> bool {
        self.syscfgsmen() != 0
    }

    #[doc="Sets the SYSCFGSMEN field."]
    #[inline] pub fn set_syscfgsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.dfsdmsmen() != 0 { try!(write!(f, " dfsdmsmen"))}
        if self.sai2smen() != 0 { try!(write!(f, " sai2smen"))}
        if self.sai1smen() != 0 { try!(write!(f, " sai1smen"))}
        if self.tim17smen() != 0 { try!(write!(f, " tim17smen"))}
        if self.tim16smen() != 0 { try!(write!(f, " tim16smen"))}
        if self.tim15smen() != 0 { try!(write!(f, " tim15smen"))}
        if self.usart1smen() != 0 { try!(write!(f, " usart1smen"))}
        if self.tim8smen() != 0 { try!(write!(f, " tim8smen"))}
        if self.spi1smen() != 0 { try!(write!(f, " spi1smen"))}
        if self.tim1smen() != 0 { try!(write!(f, " tim1smen"))}
        if self.sdmmcsmen() != 0 { try!(write!(f, " sdmmcsmen"))}
        if self.syscfgsmen() != 0 { try!(write!(f, " syscfgsmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CCIPR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccipr(pub u32);
impl Ccipr {
    #[doc="DFSDM clock source selection"]
    #[inline] pub fn dfsdmsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DFSDMSEL != 0"]
    #[inline] pub fn test_dfsdmsel(&self) -> bool {
        self.dfsdmsel() != 0
    }

    #[doc="Sets the DFSDMSEL field."]
    #[inline] pub fn set_dfsdmsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="SWPMI1 clock source selection"]
    #[inline] pub fn swpmi1sel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SWPMI1SEL != 0"]
    #[inline] pub fn test_swpmi1sel(&self) -> bool {
        self.swpmi1sel() != 0
    }

    #[doc="Sets the SWPMI1SEL field."]
    #[inline] pub fn set_swpmi1sel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="ADCs clock source selection"]
    #[inline] pub fn adcsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ADCSEL != 0"]
    #[inline] pub fn test_adcsel(&self) -> bool {
        self.adcsel() != 0
    }

    #[doc="Sets the ADCSEL field."]
    #[inline] pub fn set_adcsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="48 MHz clock source selection"]
    #[inline] pub fn clk48sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3) as u8) } // [27:26]
    }

    #[doc="Returns true if CLK48SEL != 0"]
    #[inline] pub fn test_clk48sel(&self) -> bool {
        self.clk48sel() != 0
    }

    #[doc="Sets the CLK48SEL field."]
    #[inline] pub fn set_clk48sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="SAI2 clock source selection"]
    #[inline] pub fn sai2sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if SAI2SEL != 0"]
    #[inline] pub fn test_sai2sel(&self) -> bool {
        self.sai2sel() != 0
    }

    #[doc="Sets the SAI2SEL field."]
    #[inline] pub fn set_sai2sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SAI1 clock source selection"]
    #[inline] pub fn sai1sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if SAI1SEL != 0"]
    #[inline] pub fn test_sai1sel(&self) -> bool {
        self.sai1sel() != 0
    }

    #[doc="Sets the SAI1SEL field."]
    #[inline] pub fn set_sai1sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Low power timer 2 clock source selection"]
    #[inline] pub fn lptim2sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if LPTIM2SEL != 0"]
    #[inline] pub fn test_lptim2sel(&self) -> bool {
        self.lptim2sel() != 0
    }

    #[doc="Sets the LPTIM2SEL field."]
    #[inline] pub fn set_lptim2sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Low power timer 1 clock source selection"]
    #[inline] pub fn lptim1sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if LPTIM1SEL != 0"]
    #[inline] pub fn test_lptim1sel(&self) -> bool {
        self.lptim1sel() != 0
    }

    #[doc="Sets the LPTIM1SEL field."]
    #[inline] pub fn set_lptim1sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="I2C3 clock source selection"]
    #[inline] pub fn i2c3sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if I2C3SEL != 0"]
    #[inline] pub fn test_i2c3sel(&self) -> bool {
        self.i2c3sel() != 0
    }

    #[doc="Sets the I2C3SEL field."]
    #[inline] pub fn set_i2c3sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="I2C2 clock source selection"]
    #[inline] pub fn i2c2sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if I2C2SEL != 0"]
    #[inline] pub fn test_i2c2sel(&self) -> bool {
        self.i2c2sel() != 0
    }

    #[doc="Sets the I2C2SEL field."]
    #[inline] pub fn set_i2c2sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="I2C1 clock source selection"]
    #[inline] pub fn i2c1sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if I2C1SEL != 0"]
    #[inline] pub fn test_i2c1sel(&self) -> bool {
        self.i2c1sel() != 0
    }

    #[doc="Sets the I2C1SEL field."]
    #[inline] pub fn set_i2c1sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="LPUART1 clock source selection"]
    #[inline] pub fn lpuart1sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if LPUART1SEL != 0"]
    #[inline] pub fn test_lpuart1sel(&self) -> bool {
        self.lpuart1sel() != 0
    }

    #[doc="Sets the LPUART1SEL field."]
    #[inline] pub fn set_lpuart1sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="UART5 clock source selection"]
    #[inline] pub fn uart5sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if UART5SEL != 0"]
    #[inline] pub fn test_uart5sel(&self) -> bool {
        self.uart5sel() != 0
    }

    #[doc="Sets the UART5SEL field."]
    #[inline] pub fn set_uart5sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="UART4 clock source selection"]
    #[inline] pub fn uart4sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if UART4SEL != 0"]
    #[inline] pub fn test_uart4sel(&self) -> bool {
        self.uart4sel() != 0
    }

    #[doc="Sets the UART4SEL field."]
    #[inline] pub fn set_uart4sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="USART3 clock source selection"]
    #[inline] pub fn usart3sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if USART3SEL != 0"]
    #[inline] pub fn test_usart3sel(&self) -> bool {
        self.usart3sel() != 0
    }

    #[doc="Sets the USART3SEL field."]
    #[inline] pub fn set_usart3sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="USART2 clock source selection"]
    #[inline] pub fn usart2sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if USART2SEL != 0"]
    #[inline] pub fn test_usart2sel(&self) -> bool {
        self.usart2sel() != 0
    }

    #[doc="Sets the USART2SEL field."]
    #[inline] pub fn set_usart2sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="USART1 clock source selection"]
    #[inline] pub fn usart1sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if USART1SEL != 0"]
    #[inline] pub fn test_usart1sel(&self) -> bool {
        self.usart1sel() != 0
    }

    #[doc="Sets the USART1SEL field."]
    #[inline] pub fn set_usart1sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
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
        if self.dfsdmsel() != 0 { try!(write!(f, " dfsdmsel"))}
        if self.swpmi1sel() != 0 { try!(write!(f, " swpmi1sel"))}
        if self.adcsel() != 0 { try!(write!(f, " adcsel=0x{:x}", self.adcsel()))}
        if self.clk48sel() != 0 { try!(write!(f, " clk48sel=0x{:x}", self.clk48sel()))}
        if self.sai2sel() != 0 { try!(write!(f, " sai2sel=0x{:x}", self.sai2sel()))}
        if self.sai1sel() != 0 { try!(write!(f, " sai1sel=0x{:x}", self.sai1sel()))}
        if self.lptim2sel() != 0 { try!(write!(f, " lptim2sel=0x{:x}", self.lptim2sel()))}
        if self.lptim1sel() != 0 { try!(write!(f, " lptim1sel=0x{:x}", self.lptim1sel()))}
        if self.i2c3sel() != 0 { try!(write!(f, " i2c3sel=0x{:x}", self.i2c3sel()))}
        if self.i2c2sel() != 0 { try!(write!(f, " i2c2sel=0x{:x}", self.i2c2sel()))}
        if self.i2c1sel() != 0 { try!(write!(f, " i2c1sel=0x{:x}", self.i2c1sel()))}
        if self.lpuart1sel() != 0 { try!(write!(f, " lpuart1sel=0x{:x}", self.lpuart1sel()))}
        if self.uart5sel() != 0 { try!(write!(f, " uart5sel=0x{:x}", self.uart5sel()))}
        if self.uart4sel() != 0 { try!(write!(f, " uart4sel=0x{:x}", self.uart4sel()))}
        if self.usart3sel() != 0 { try!(write!(f, " usart3sel=0x{:x}", self.usart3sel()))}
        if self.usart2sel() != 0 { try!(write!(f, " usart2sel=0x{:x}", self.usart2sel()))}
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
    #[inline] pub fn lscosel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if LSCOSEL != 0"]
    #[inline] pub fn test_lscosel(&self) -> bool {
        self.lscosel() != 0
    }

    #[doc="Sets the LSCOSEL field."]
    #[inline] pub fn set_lscosel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Low speed clock output enable"]
    #[inline] pub fn lscoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if LSCOEN != 0"]
    #[inline] pub fn test_lscoen(&self) -> bool {
        self.lscoen() != 0
    }

    #[doc="Sets the LSCOEN field."]
    #[inline] pub fn set_lscoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Backup domain software reset"]
    #[inline] pub fn bdrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BDRST != 0"]
    #[inline] pub fn test_bdrst(&self) -> bool {
        self.bdrst() != 0
    }

    #[doc="Sets the BDRST field."]
    #[inline] pub fn set_bdrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="RTC clock enable"]
    #[inline] pub fn rtcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RTCEN != 0"]
    #[inline] pub fn test_rtcen(&self) -> bool {
        self.rtcen() != 0
    }

    #[doc="Sets the RTCEN field."]
    #[inline] pub fn set_rtcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="RTC clock source selection"]
    #[inline] pub fn rtcsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if RTCSEL != 0"]
    #[inline] pub fn test_rtcsel(&self) -> bool {
        self.rtcsel() != 0
    }

    #[doc="Sets the RTCSEL field."]
    #[inline] pub fn set_rtcsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LSECSSD"]
    #[inline] pub fn lsecssd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LSECSSD != 0"]
    #[inline] pub fn test_lsecssd(&self) -> bool {
        self.lsecssd() != 0
    }

    #[doc="Sets the LSECSSD field."]
    #[inline] pub fn set_lsecssd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LSECSSON"]
    #[inline] pub fn lsecsson(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LSECSSON != 0"]
    #[inline] pub fn test_lsecsson(&self) -> bool {
        self.lsecsson() != 0
    }

    #[doc="Sets the LSECSSON field."]
    #[inline] pub fn set_lsecsson<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SE oscillator drive capability"]
    #[inline] pub fn lsedrv(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if LSEDRV != 0"]
    #[inline] pub fn test_lsedrv(&self) -> bool {
        self.lsedrv() != 0
    }

    #[doc="Sets the LSEDRV field."]
    #[inline] pub fn set_lsedrv<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="LSE oscillator bypass"]
    #[inline] pub fn lsebyp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LSEBYP != 0"]
    #[inline] pub fn test_lsebyp(&self) -> bool {
        self.lsebyp() != 0
    }

    #[doc="Sets the LSEBYP field."]
    #[inline] pub fn set_lsebyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE oscillator ready"]
    #[inline] pub fn lserdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDY != 0"]
    #[inline] pub fn test_lserdy(&self) -> bool {
        self.lserdy() != 0
    }

    #[doc="Sets the LSERDY field."]
    #[inline] pub fn set_lserdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSE oscillator enable"]
    #[inline] pub fn lseon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSEON != 0"]
    #[inline] pub fn test_lseon(&self) -> bool {
        self.lseon() != 0
    }

    #[doc="Sets the LSEON field."]
    #[inline] pub fn set_lseon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.lscosel() != 0 { try!(write!(f, " lscosel"))}
        if self.lscoen() != 0 { try!(write!(f, " lscoen"))}
        if self.bdrst() != 0 { try!(write!(f, " bdrst"))}
        if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
        if self.rtcsel() != 0 { try!(write!(f, " rtcsel=0x{:x}", self.rtcsel()))}
        if self.lsecssd() != 0 { try!(write!(f, " lsecssd"))}
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
    #[inline] pub fn lpwrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPWRSTF != 0"]
    #[inline] pub fn test_lpwrstf(&self) -> bool {
        self.lpwrstf() != 0
    }

    #[doc="Sets the LPWRSTF field."]
    #[inline] pub fn set_lpwrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Window watchdog reset flag"]
    #[inline] pub fn wwdgrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if WWDGRSTF != 0"]
    #[inline] pub fn test_wwdgrstf(&self) -> bool {
        self.wwdgrstf() != 0
    }

    #[doc="Sets the WWDGRSTF field."]
    #[inline] pub fn set_wwdgrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Independent window watchdog reset flag"]
    #[inline] pub fn iwdgrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if IWDGRSTF != 0"]
    #[inline] pub fn test_iwdgrstf(&self) -> bool {
        self.iwdgrstf() != 0
    }

    #[doc="Sets the IWDGRSTF field."]
    #[inline] pub fn set_iwdgrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Software reset flag"]
    #[inline] pub fn sftrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SFTRSTF != 0"]
    #[inline] pub fn test_sftrstf(&self) -> bool {
        self.sftrstf() != 0
    }

    #[doc="Sets the SFTRSTF field."]
    #[inline] pub fn set_sftrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="BOR flag"]
    #[inline] pub fn borrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if BORRSTF != 0"]
    #[inline] pub fn test_borrstf(&self) -> bool {
        self.borrstf() != 0
    }

    #[doc="Sets the BORRSTF field."]
    #[inline] pub fn set_borrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pin reset flag"]
    #[inline] pub fn pinrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PINRSTF != 0"]
    #[inline] pub fn test_pinrstf(&self) -> bool {
        self.pinrstf() != 0
    }

    #[doc="Sets the PINRSTF field."]
    #[inline] pub fn set_pinrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Option byte loader reset flag"]
    #[inline] pub fn oblrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if OBLRSTF != 0"]
    #[inline] pub fn test_oblrstf(&self) -> bool {
        self.oblrstf() != 0
    }

    #[doc="Sets the OBLRSTF field."]
    #[inline] pub fn set_oblrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Firewall reset flag"]
    #[inline] pub fn firewallrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FIREWALLRSTF != 0"]
    #[inline] pub fn test_firewallrstf(&self) -> bool {
        self.firewallrstf() != 0
    }

    #[doc="Sets the FIREWALLRSTF field."]
    #[inline] pub fn set_firewallrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Remove reset flag"]
    #[inline] pub fn rmvf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if RMVF != 0"]
    #[inline] pub fn test_rmvf(&self) -> bool {
        self.rmvf() != 0
    }

    #[doc="Sets the RMVF field."]
    #[inline] pub fn set_rmvf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="SI range after Standby mode"]
    #[inline] pub fn msisrange(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MSISRANGE != 0"]
    #[inline] pub fn test_msisrange(&self) -> bool {
        self.msisrange() != 0
    }

    #[doc="Sets the MSISRANGE field."]
    #[inline] pub fn set_msisrange<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LSI oscillator ready"]
    #[inline] pub fn lsirdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSIRDY != 0"]
    #[inline] pub fn test_lsirdy(&self) -> bool {
        self.lsirdy() != 0
    }

    #[doc="Sets the LSIRDY field."]
    #[inline] pub fn set_lsirdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LSI oscillator enable"]
    #[inline] pub fn lsion(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSION != 0"]
    #[inline] pub fn test_lsion(&self) -> bool {
        self.lsion() != 0
    }

    #[doc="Sets the LSION field."]
    #[inline] pub fn set_lsion<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.lpwrstf() != 0 { try!(write!(f, " lpwrstf"))}
        if self.wwdgrstf() != 0 { try!(write!(f, " wwdgrstf"))}
        if self.iwdgrstf() != 0 { try!(write!(f, " iwdgrstf"))}
        if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
        if self.borrstf() != 0 { try!(write!(f, " borrstf"))}
        if self.pinrstf() != 0 { try!(write!(f, " pinrstf"))}
        if self.oblrstf() != 0 { try!(write!(f, " oblrstf"))}
        if self.firewallrstf() != 0 { try!(write!(f, " firewallrstf"))}
        if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
        if self.msisrange() != 0 { try!(write!(f, " msisrange=0x{:x}", self.msisrange()))}
        if self.lsirdy() != 0 { try!(write!(f, " lsirdy"))}
        if self.lsion() != 0 { try!(write!(f, " lsion"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

