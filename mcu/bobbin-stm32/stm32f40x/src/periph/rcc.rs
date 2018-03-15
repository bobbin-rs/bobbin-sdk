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

    #[doc="Get the *mut pointer for the PLLCFGR register."]
    #[inline] pub fn pllcfgr_mut(&self) -> *mut Pllcfgr { 
        (self.0 + 0x4) as *mut Pllcfgr
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

    #[doc="Get the *mut pointer for the CIR register."]
    #[inline] pub fn cir_mut(&self) -> *mut Cir { 
        (self.0 + 0xc) as *mut Cir
    }

    #[doc="Get the *const pointer for the CIR register."]
    #[inline] pub fn cir_ptr(&self) -> *const Cir { 
           self.cir_mut()
    }

    #[doc="Read the CIR register."]
    #[inline] pub fn cir(&self) -> Cir { 
        unsafe {
            read_volatile(self.cir_ptr())
        }
    }

    #[doc="Write the CIR register."]
    #[inline] pub fn set_cir<F: FnOnce(Cir) -> Cir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cir_mut(), f(Cir(0)));
        }
        self
    }

    #[doc="Modify the CIR register."]
    #[inline] pub fn with_cir<F: FnOnce(Cir) -> Cir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cir_mut(), f(self.cir()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB1RSTR register."]
    #[inline] pub fn ahb1rstr_mut(&self) -> *mut Ahb1rstr { 
        (self.0 + 0x10) as *mut Ahb1rstr
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
        (self.0 + 0x14) as *mut Ahb2rstr
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
        (self.0 + 0x18) as *mut Ahb3rstr
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

    #[doc="Get the *mut pointer for the APB1RSTR register."]
    #[inline] pub fn apb1rstr_mut(&self) -> *mut Apb1rstr { 
        (self.0 + 0x20) as *mut Apb1rstr
    }

    #[doc="Get the *const pointer for the APB1RSTR register."]
    #[inline] pub fn apb1rstr_ptr(&self) -> *const Apb1rstr { 
           self.apb1rstr_mut()
    }

    #[doc="Read the APB1RSTR register."]
    #[inline] pub fn apb1rstr(&self) -> Apb1rstr { 
        unsafe {
            read_volatile(self.apb1rstr_ptr())
        }
    }

    #[doc="Write the APB1RSTR register."]
    #[inline] pub fn set_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1rstr_mut(), f(Apb1rstr(0)));
        }
        self
    }

    #[doc="Modify the APB1RSTR register."]
    #[inline] pub fn with_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1rstr_mut(), f(self.apb1rstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2RSTR register."]
    #[inline] pub fn apb2rstr_mut(&self) -> *mut Apb2rstr { 
        (self.0 + 0x24) as *mut Apb2rstr
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
        (self.0 + 0x30) as *mut Ahb1enr
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
        (self.0 + 0x34) as *mut Ahb2enr
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
        (self.0 + 0x38) as *mut Ahb3enr
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

    #[doc="Get the *mut pointer for the APB1ENR register."]
    #[inline] pub fn apb1enr_mut(&self) -> *mut Apb1enr { 
        (self.0 + 0x40) as *mut Apb1enr
    }

    #[doc="Get the *const pointer for the APB1ENR register."]
    #[inline] pub fn apb1enr_ptr(&self) -> *const Apb1enr { 
           self.apb1enr_mut()
    }

    #[doc="Read the APB1ENR register."]
    #[inline] pub fn apb1enr(&self) -> Apb1enr { 
        unsafe {
            read_volatile(self.apb1enr_ptr())
        }
    }

    #[doc="Write the APB1ENR register."]
    #[inline] pub fn set_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1enr_mut(), f(Apb1enr(0)));
        }
        self
    }

    #[doc="Modify the APB1ENR register."]
    #[inline] pub fn with_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1enr_mut(), f(self.apb1enr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2ENR register."]
    #[inline] pub fn apb2enr_mut(&self) -> *mut Apb2enr { 
        (self.0 + 0x44) as *mut Apb2enr
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

    #[doc="Get the *mut pointer for the AHB1LPENR register."]
    #[inline] pub fn ahb1lpenr_mut(&self) -> *mut Ahb1lpenr { 
        (self.0 + 0x50) as *mut Ahb1lpenr
    }

    #[doc="Get the *const pointer for the AHB1LPENR register."]
    #[inline] pub fn ahb1lpenr_ptr(&self) -> *const Ahb1lpenr { 
           self.ahb1lpenr_mut()
    }

    #[doc="Read the AHB1LPENR register."]
    #[inline] pub fn ahb1lpenr(&self) -> Ahb1lpenr { 
        unsafe {
            read_volatile(self.ahb1lpenr_ptr())
        }
    }

    #[doc="Write the AHB1LPENR register."]
    #[inline] pub fn set_ahb1lpenr<F: FnOnce(Ahb1lpenr) -> Ahb1lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb1lpenr_mut(), f(Ahb1lpenr(0)));
        }
        self
    }

    #[doc="Modify the AHB1LPENR register."]
    #[inline] pub fn with_ahb1lpenr<F: FnOnce(Ahb1lpenr) -> Ahb1lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb1lpenr_mut(), f(self.ahb1lpenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB2LPENR register."]
    #[inline] pub fn ahb2lpenr_mut(&self) -> *mut Ahb2lpenr { 
        (self.0 + 0x54) as *mut Ahb2lpenr
    }

    #[doc="Get the *const pointer for the AHB2LPENR register."]
    #[inline] pub fn ahb2lpenr_ptr(&self) -> *const Ahb2lpenr { 
           self.ahb2lpenr_mut()
    }

    #[doc="Read the AHB2LPENR register."]
    #[inline] pub fn ahb2lpenr(&self) -> Ahb2lpenr { 
        unsafe {
            read_volatile(self.ahb2lpenr_ptr())
        }
    }

    #[doc="Write the AHB2LPENR register."]
    #[inline] pub fn set_ahb2lpenr<F: FnOnce(Ahb2lpenr) -> Ahb2lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb2lpenr_mut(), f(Ahb2lpenr(0)));
        }
        self
    }

    #[doc="Modify the AHB2LPENR register."]
    #[inline] pub fn with_ahb2lpenr<F: FnOnce(Ahb2lpenr) -> Ahb2lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb2lpenr_mut(), f(self.ahb2lpenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHB3LPENR register."]
    #[inline] pub fn ahb3lpenr_mut(&self) -> *mut Ahb3lpenr { 
        (self.0 + 0x58) as *mut Ahb3lpenr
    }

    #[doc="Get the *const pointer for the AHB3LPENR register."]
    #[inline] pub fn ahb3lpenr_ptr(&self) -> *const Ahb3lpenr { 
           self.ahb3lpenr_mut()
    }

    #[doc="Read the AHB3LPENR register."]
    #[inline] pub fn ahb3lpenr(&self) -> Ahb3lpenr { 
        unsafe {
            read_volatile(self.ahb3lpenr_ptr())
        }
    }

    #[doc="Write the AHB3LPENR register."]
    #[inline] pub fn set_ahb3lpenr<F: FnOnce(Ahb3lpenr) -> Ahb3lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb3lpenr_mut(), f(Ahb3lpenr(0)));
        }
        self
    }

    #[doc="Modify the AHB3LPENR register."]
    #[inline] pub fn with_ahb3lpenr<F: FnOnce(Ahb3lpenr) -> Ahb3lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahb3lpenr_mut(), f(self.ahb3lpenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1LPENR register."]
    #[inline] pub fn apb1lpenr_mut(&self) -> *mut Apb1lpenr { 
        (self.0 + 0x60) as *mut Apb1lpenr
    }

    #[doc="Get the *const pointer for the APB1LPENR register."]
    #[inline] pub fn apb1lpenr_ptr(&self) -> *const Apb1lpenr { 
           self.apb1lpenr_mut()
    }

    #[doc="Read the APB1LPENR register."]
    #[inline] pub fn apb1lpenr(&self) -> Apb1lpenr { 
        unsafe {
            read_volatile(self.apb1lpenr_ptr())
        }
    }

    #[doc="Write the APB1LPENR register."]
    #[inline] pub fn set_apb1lpenr<F: FnOnce(Apb1lpenr) -> Apb1lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1lpenr_mut(), f(Apb1lpenr(0)));
        }
        self
    }

    #[doc="Modify the APB1LPENR register."]
    #[inline] pub fn with_apb1lpenr<F: FnOnce(Apb1lpenr) -> Apb1lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1lpenr_mut(), f(self.apb1lpenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2LPENR register."]
    #[inline] pub fn apb2lpenr_mut(&self) -> *mut Apb2lpenr { 
        (self.0 + 0x64) as *mut Apb2lpenr
    }

    #[doc="Get the *const pointer for the APB2LPENR register."]
    #[inline] pub fn apb2lpenr_ptr(&self) -> *const Apb2lpenr { 
           self.apb2lpenr_mut()
    }

    #[doc="Read the APB2LPENR register."]
    #[inline] pub fn apb2lpenr(&self) -> Apb2lpenr { 
        unsafe {
            read_volatile(self.apb2lpenr_ptr())
        }
    }

    #[doc="Write the APB2LPENR register."]
    #[inline] pub fn set_apb2lpenr<F: FnOnce(Apb2lpenr) -> Apb2lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2lpenr_mut(), f(Apb2lpenr(0)));
        }
        self
    }

    #[doc="Modify the APB2LPENR register."]
    #[inline] pub fn with_apb2lpenr<F: FnOnce(Apb2lpenr) -> Apb2lpenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2lpenr_mut(), f(self.apb2lpenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BDCR register."]
    #[inline] pub fn bdcr_mut(&self) -> *mut Bdcr { 
        (self.0 + 0x70) as *mut Bdcr
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
        (self.0 + 0x74) as *mut Csr
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

    #[doc="Get the *mut pointer for the SSCGR register."]
    #[inline] pub fn sscgr_mut(&self) -> *mut Sscgr { 
        (self.0 + 0x80) as *mut Sscgr
    }

    #[doc="Get the *const pointer for the SSCGR register."]
    #[inline] pub fn sscgr_ptr(&self) -> *const Sscgr { 
           self.sscgr_mut()
    }

    #[doc="Read the SSCGR register."]
    #[inline] pub fn sscgr(&self) -> Sscgr { 
        unsafe {
            read_volatile(self.sscgr_ptr())
        }
    }

    #[doc="Write the SSCGR register."]
    #[inline] pub fn set_sscgr<F: FnOnce(Sscgr) -> Sscgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sscgr_mut(), f(Sscgr(0)));
        }
        self
    }

    #[doc="Modify the SSCGR register."]
    #[inline] pub fn with_sscgr<F: FnOnce(Sscgr) -> Sscgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sscgr_mut(), f(self.sscgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PLLI2SCFGR register."]
    #[inline] pub fn plli2scfgr_mut(&self) -> *mut Plli2scfgr { 
        (self.0 + 0x84) as *mut Plli2scfgr
    }

    #[doc="Get the *const pointer for the PLLI2SCFGR register."]
    #[inline] pub fn plli2scfgr_ptr(&self) -> *const Plli2scfgr { 
           self.plli2scfgr_mut()
    }

    #[doc="Read the PLLI2SCFGR register."]
    #[inline] pub fn plli2scfgr(&self) -> Plli2scfgr { 
        unsafe {
            read_volatile(self.plli2scfgr_ptr())
        }
    }

    #[doc="Write the PLLI2SCFGR register."]
    #[inline] pub fn set_plli2scfgr<F: FnOnce(Plli2scfgr) -> Plli2scfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.plli2scfgr_mut(), f(Plli2scfgr(0)));
        }
        self
    }

    #[doc="Modify the PLLI2SCFGR register."]
    #[inline] pub fn with_plli2scfgr<F: FnOnce(Plli2scfgr) -> Plli2scfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.plli2scfgr_mut(), f(self.plli2scfgr()));
        }
        self
    }

}

#[doc="clock control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="PLLI2S clock ready flag"]
    #[inline] pub fn plli2srdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PLLI2SRDY != 0"]
    #[inline] pub fn test_plli2srdy(&self) -> bool {
        self.plli2srdy() != 0
    }

    #[doc="Sets the PLLI2SRDY field."]
    #[inline] pub fn set_plli2srdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="PLLI2S enable"]
    #[inline] pub fn plli2son(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PLLI2SON != 0"]
    #[inline] pub fn test_plli2son(&self) -> bool {
        self.plli2son() != 0
    }

    #[doc="Sets the PLLI2SON field."]
    #[inline] pub fn set_plli2son<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Main PLL (PLL) clock ready flag"]
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

    #[doc="Main PLL (PLL) enable"]
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

    #[doc="HSE clock bypass"]
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

    #[doc="Internal high-speed clock calibration"]
    #[inline] pub fn hsical(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if HSICAL != 0"]
    #[inline] pub fn test_hsical(&self) -> bool {
        self.hsical() != 0
    }

    #[doc="Sets the HSICAL field."]
    #[inline] pub fn set_hsical<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Internal high-speed clock trimming"]
    #[inline] pub fn hsitrim(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if HSITRIM != 0"]
    #[inline] pub fn test_hsitrim(&self) -> bool {
        self.hsitrim() != 0
    }

    #[doc="Sets the HSITRIM field."]
    #[inline] pub fn set_hsitrim<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Internal high-speed clock ready flag"]
    #[inline] pub fn hsirdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HSIRDY != 0"]
    #[inline] pub fn test_hsirdy(&self) -> bool {
        self.hsirdy() != 0
    }

    #[doc="Sets the HSIRDY field."]
    #[inline] pub fn set_hsirdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Internal high-speed clock enable"]
    #[inline] pub fn hsion(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HSION != 0"]
    #[inline] pub fn test_hsion(&self) -> bool {
        self.hsion() != 0
    }

    #[doc="Sets the HSION field."]
    #[inline] pub fn set_hsion<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
    #[inline] pub fn pllq(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if PLLQ != 0"]
    #[inline] pub fn test_pllq(&self) -> bool {
        self.pllq() != 0
    }

    #[doc="Sets the PLLQ field."]
    #[inline] pub fn set_pllq<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline] pub fn pllsrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PLLSRC != 0"]
    #[inline] pub fn test_pllsrc(&self) -> bool {
        self.pllsrc() != 0
    }

    #[doc="Sets the PLLSRC field."]
    #[inline] pub fn set_pllsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Main PLL (PLL) division factor for main system clock"]
    #[inline] pub fn pllp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PLLP != 0"]
    #[inline] pub fn test_pllp(&self) -> bool {
        self.pllp() != 0
    }

    #[doc="Sets the PLLP field."]
    #[inline] pub fn set_pllp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Main PLL (PLL) multiplication factor for VCO"]
    #[inline] pub fn plln(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1ff) as u16) } // [14:6]
    }

    #[doc="Returns true if PLLN != 0"]
    #[inline] pub fn test_plln(&self) -> bool {
        self.plln() != 0
    }

    #[doc="Sets the PLLN field."]
    #[inline] pub fn set_plln<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline] pub fn pllm(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if PLLM != 0"]
    #[inline] pub fn test_pllm(&self) -> bool {
        self.pllm() != 0
    }

    #[doc="Sets the PLLM field."]
    #[inline] pub fn set_pllm<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
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
    #[inline] pub fn mco2(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if MCO2 != 0"]
    #[inline] pub fn test_mco2(&self) -> bool {
        self.mco2() != 0
    }

    #[doc="Sets the MCO2 field."]
    #[inline] pub fn set_mco2<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="MCO2 prescaler"]
    #[inline] pub fn mco2pre(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if MCO2PRE != 0"]
    #[inline] pub fn test_mco2pre(&self) -> bool {
        self.mco2pre() != 0
    }

    #[doc="Sets the MCO2PRE field."]
    #[inline] pub fn set_mco2pre<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="MCO1 prescaler"]
    #[inline] pub fn mco1pre(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if MCO1PRE != 0"]
    #[inline] pub fn test_mco1pre(&self) -> bool {
        self.mco1pre() != 0
    }

    #[doc="Sets the MCO1PRE field."]
    #[inline] pub fn set_mco1pre<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="I2S clock selection"]
    #[inline] pub fn i2ssrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2SSRC != 0"]
    #[inline] pub fn test_i2ssrc(&self) -> bool {
        self.i2ssrc() != 0
    }

    #[doc="Sets the I2SSRC field."]
    #[inline] pub fn set_i2ssrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Microcontroller clock output 1"]
    #[inline] pub fn mco1(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if MCO1 != 0"]
    #[inline] pub fn test_mco1(&self) -> bool {
        self.mco1() != 0
    }

    #[doc="Sets the MCO1 field."]
    #[inline] pub fn set_mco1<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="HSE division factor for RTC clock"]
    #[inline] pub fn rtcpre(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if RTCPRE != 0"]
    #[inline] pub fn test_rtcpre(&self) -> bool {
        self.rtcpre() != 0
    }

    #[doc="Sets the RTCPRE field."]
    #[inline] pub fn set_rtcpre<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="APB high-speed prescaler (APB2)"]
    #[inline] pub fn ppre2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if PPRE2 != 0"]
    #[inline] pub fn test_ppre2(&self) -> bool {
        self.ppre2() != 0
    }

    #[doc="Sets the PPRE2 field."]
    #[inline] pub fn set_ppre2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="APB Low speed prescaler (APB1)"]
    #[inline] pub fn ppre1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x7) as u8) } // [12:10]
    }

    #[doc="Returns true if PPRE1 != 0"]
    #[inline] pub fn test_ppre1(&self) -> bool {
        self.ppre1() != 0
    }

    #[doc="Sets the PPRE1 field."]
    #[inline] pub fn set_ppre1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 10);
        self.0 |= value << 10;
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
    #[inline] pub fn cssc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if CSSC != 0"]
    #[inline] pub fn test_cssc(&self) -> bool {
        self.cssc() != 0
    }

    #[doc="Sets the CSSC field."]
    #[inline] pub fn set_cssc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="PLLI2S ready interrupt clear"]
    #[inline] pub fn plli2srdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PLLI2SRDYC != 0"]
    #[inline] pub fn test_plli2srdyc(&self) -> bool {
        self.plli2srdyc() != 0
    }

    #[doc="Sets the PLLI2SRDYC field."]
    #[inline] pub fn set_plli2srdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Main PLL(PLL) ready interrupt clear"]
    #[inline] pub fn pllrdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PLLRDYC != 0"]
    #[inline] pub fn test_pllrdyc(&self) -> bool {
        self.pllrdyc() != 0
    }

    #[doc="Sets the PLLRDYC field."]
    #[inline] pub fn set_pllrdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="HSE ready interrupt clear"]
    #[inline] pub fn hserdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HSERDYC != 0"]
    #[inline] pub fn test_hserdyc(&self) -> bool {
        self.hserdyc() != 0
    }

    #[doc="Sets the HSERDYC field."]
    #[inline] pub fn set_hserdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="HSI ready interrupt clear"]
    #[inline] pub fn hsirdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if HSIRDYC != 0"]
    #[inline] pub fn test_hsirdyc(&self) -> bool {
        self.hsirdyc() != 0
    }

    #[doc="Sets the HSIRDYC field."]
    #[inline] pub fn set_hsirdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="LSE ready interrupt clear"]
    #[inline] pub fn lserdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSERDYC != 0"]
    #[inline] pub fn test_lserdyc(&self) -> bool {
        self.lserdyc() != 0
    }

    #[doc="Sets the LSERDYC field."]
    #[inline] pub fn set_lserdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="LSI ready interrupt clear"]
    #[inline] pub fn lsirdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if LSIRDYC != 0"]
    #[inline] pub fn test_lsirdyc(&self) -> bool {
        self.lsirdyc() != 0
    }

    #[doc="Sets the LSIRDYC field."]
    #[inline] pub fn set_lsirdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="PLLI2S ready interrupt enable"]
    #[inline] pub fn plli2srdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PLLI2SRDYIE != 0"]
    #[inline] pub fn test_plli2srdyie(&self) -> bool {
        self.plli2srdyie() != 0
    }

    #[doc="Sets the PLLI2SRDYIE field."]
    #[inline] pub fn set_plli2srdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Main PLL (PLL) ready interrupt enable"]
    #[inline] pub fn pllrdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PLLRDYIE != 0"]
    #[inline] pub fn test_pllrdyie(&self) -> bool {
        self.pllrdyie() != 0
    }

    #[doc="Sets the PLLRDYIE field."]
    #[inline] pub fn set_pllrdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="HSE ready interrupt enable"]
    #[inline] pub fn hserdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HSERDYIE != 0"]
    #[inline] pub fn test_hserdyie(&self) -> bool {
        self.hserdyie() != 0
    }

    #[doc="Sets the HSERDYIE field."]
    #[inline] pub fn set_hserdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="HSI ready interrupt enable"]
    #[inline] pub fn hsirdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSIRDYIE != 0"]
    #[inline] pub fn test_hsirdyie(&self) -> bool {
        self.hsirdyie() != 0
    }

    #[doc="Sets the HSIRDYIE field."]
    #[inline] pub fn set_hsirdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="LSE ready interrupt enable"]
    #[inline] pub fn lserdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSERDYIE != 0"]
    #[inline] pub fn test_lserdyie(&self) -> bool {
        self.lserdyie() != 0
    }

    #[doc="Sets the LSERDYIE field."]
    #[inline] pub fn set_lserdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="LSI ready interrupt enable"]
    #[inline] pub fn lsirdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LSIRDYIE != 0"]
    #[inline] pub fn test_lsirdyie(&self) -> bool {
        self.lsirdyie() != 0
    }

    #[doc="Sets the LSIRDYIE field."]
    #[inline] pub fn set_lsirdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock security system interrupt flag"]
    #[inline] pub fn cssf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CSSF != 0"]
    #[inline] pub fn test_cssf(&self) -> bool {
        self.cssf() != 0
    }

    #[doc="Sets the CSSF field."]
    #[inline] pub fn set_cssf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="PLLI2S ready interrupt flag"]
    #[inline] pub fn plli2srdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLI2SRDYF != 0"]
    #[inline] pub fn test_plli2srdyf(&self) -> bool {
        self.plli2srdyf() != 0
    }

    #[doc="Sets the PLLI2SRDYF field."]
    #[inline] pub fn set_plli2srdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Main PLL (PLL) ready interrupt flag"]
    #[inline] pub fn pllrdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PLLRDYF != 0"]
    #[inline] pub fn test_pllrdyf(&self) -> bool {
        self.pllrdyf() != 0
    }

    #[doc="Sets the PLLRDYF field."]
    #[inline] pub fn set_pllrdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSE ready interrupt flag"]
    #[inline] pub fn hserdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSERDYF != 0"]
    #[inline] pub fn test_hserdyf(&self) -> bool {
        self.hserdyf() != 0
    }

    #[doc="Sets the HSERDYF field."]
    #[inline] pub fn set_hserdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="HSI ready interrupt flag"]
    #[inline] pub fn hsirdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSIRDYF != 0"]
    #[inline] pub fn test_hsirdyf(&self) -> bool {
        self.hsirdyf() != 0
    }

    #[doc="Sets the HSIRDYF field."]
    #[inline] pub fn set_hsirdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.plli2srdyc() != 0 { try!(write!(f, " plli2srdyc"))}
        if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
        if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
        if self.hsirdyc() != 0 { try!(write!(f, " hsirdyc"))}
        if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
        if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
        if self.plli2srdyie() != 0 { try!(write!(f, " plli2srdyie"))}
        if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
        if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
        if self.hsirdyie() != 0 { try!(write!(f, " hsirdyie"))}
        if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
        if self.lsirdyie() != 0 { try!(write!(f, " lsirdyie"))}
        if self.cssf() != 0 { try!(write!(f, " cssf"))}
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
    #[inline] pub fn otghsrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if OTGHSRST != 0"]
    #[inline] pub fn test_otghsrst(&self) -> bool {
        self.otghsrst() != 0
    }

    #[doc="Sets the OTGHSRST field."]
    #[inline] pub fn set_otghsrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Ethernet MAC reset"]
    #[inline] pub fn ethmacrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ETHMACRST != 0"]
    #[inline] pub fn test_ethmacrst(&self) -> bool {
        self.ethmacrst() != 0
    }

    #[doc="Sets the ETHMACRST field."]
    #[inline] pub fn set_ethmacrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DMA2 reset"]
    #[inline] pub fn dma2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DMA2RST != 0"]
    #[inline] pub fn test_dma2rst(&self) -> bool {
        self.dma2rst() != 0
    }

    #[doc="Sets the DMA2RST field."]
    #[inline] pub fn set_dma2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DMA2 reset"]
    #[inline] pub fn dma1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DMA1RST != 0"]
    #[inline] pub fn test_dma1rst(&self) -> bool {
        self.dma1rst() != 0
    }

    #[doc="Sets the DMA1RST field."]
    #[inline] pub fn set_dma1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="CRC reset"]
    #[inline] pub fn crcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCRST != 0"]
    #[inline] pub fn test_crcrst(&self) -> bool {
        self.crcrst() != 0
    }

    #[doc="Sets the CRCRST field."]
    #[inline] pub fn set_crcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IO port I reset"]
    #[inline] pub fn gpioirst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIOIRST != 0"]
    #[inline] pub fn test_gpioirst(&self) -> bool {
        self.gpioirst() != 0
    }

    #[doc="Sets the GPIOIRST field."]
    #[inline] pub fn set_gpioirst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        if self.dma2rst() != 0 { try!(write!(f, " dma2rst"))}
        if self.dma1rst() != 0 { try!(write!(f, " dma1rst"))}
        if self.crcrst() != 0 { try!(write!(f, " crcrst"))}
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
    #[inline] pub fn otgfsrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OTGFSRST != 0"]
    #[inline] pub fn test_otgfsrst(&self) -> bool {
        self.otgfsrst() != 0
    }

    #[doc="Sets the OTGFSRST field."]
    #[inline] pub fn set_otgfsrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Random number generator module reset"]
    #[inline] pub fn rngrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RNGRST != 0"]
    #[inline] pub fn test_rngrst(&self) -> bool {
        self.rngrst() != 0
    }

    #[doc="Sets the RNGRST field."]
    #[inline] pub fn set_rngrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Camera interface reset"]
    #[inline] pub fn dcmirst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCMIRST != 0"]
    #[inline] pub fn test_dcmirst(&self) -> bool {
        self.dcmirst() != 0
    }

    #[doc="Sets the DCMIRST field."]
    #[inline] pub fn set_dcmirst<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
    #[doc="Flexible static memory controller module reset"]
    #[inline] pub fn fsmcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSMCRST != 0"]
    #[inline] pub fn test_fsmcrst(&self) -> bool {
        self.fsmcrst() != 0
    }

    #[doc="Sets the FSMCRST field."]
    #[inline] pub fn set_fsmcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.fsmcrst() != 0 { try!(write!(f, " fsmcrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
    #[doc="DAC reset"]
    #[inline] pub fn dacrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACRST != 0"]
    #[inline] pub fn test_dacrst(&self) -> bool {
        self.dacrst() != 0
    }

    #[doc="Sets the DACRST field."]
    #[inline] pub fn set_dacrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
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

    #[doc="CAN2 reset"]
    #[inline] pub fn can2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CAN2RST != 0"]
    #[inline] pub fn test_can2rst(&self) -> bool {
        self.can2rst() != 0
    }

    #[doc="Sets the CAN2RST field."]
    #[inline] pub fn set_can2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
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

    #[doc="I2C 2 reset"]
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

    #[doc="I2C 1 reset"]
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

    #[doc="USART 5 reset"]
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

    #[doc="USART 4 reset"]
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

    #[doc="USART 3 reset"]
    #[inline] pub fn uart3rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if UART3RST != 0"]
    #[inline] pub fn test_uart3rst(&self) -> bool {
        self.uart3rst() != 0
    }

    #[doc="Sets the UART3RST field."]
    #[inline] pub fn set_uart3rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USART 2 reset"]
    #[inline] pub fn uart2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if UART2RST != 0"]
    #[inline] pub fn test_uart2rst(&self) -> bool {
        self.uart2rst() != 0
    }

    #[doc="Sets the UART2RST field."]
    #[inline] pub fn set_uart2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI 3 reset"]
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

    #[doc="SPI 2 reset"]
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

    #[doc="Window watchdog reset"]
    #[inline] pub fn wwdgrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGRST != 0"]
    #[inline] pub fn test_wwdgrst(&self) -> bool {
        self.wwdgrst() != 0
    }

    #[doc="Sets the WWDGRST field."]
    #[inline] pub fn set_wwdgrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TIM14 reset"]
    #[inline] pub fn tim14rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TIM14RST != 0"]
    #[inline] pub fn test_tim14rst(&self) -> bool {
        self.tim14rst() != 0
    }

    #[doc="Sets the TIM14RST field."]
    #[inline] pub fn set_tim14rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM13 reset"]
    #[inline] pub fn tim13rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIM13RST != 0"]
    #[inline] pub fn test_tim13rst(&self) -> bool {
        self.tim13rst() != 0
    }

    #[doc="Sets the TIM13RST field."]
    #[inline] pub fn set_tim13rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TIM12 reset"]
    #[inline] pub fn tim12rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIM12RST != 0"]
    #[inline] pub fn test_tim12rst(&self) -> bool {
        self.tim12rst() != 0
    }

    #[doc="Sets the TIM12RST field."]
    #[inline] pub fn set_tim12rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TIM7 reset"]
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

    #[doc="TIM6 reset"]
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

    #[doc="TIM5 reset"]
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

    #[doc="TIM4 reset"]
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

    #[doc="TIM3 reset"]
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

    #[doc="TIM2 reset"]
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
    #[doc="TIM11 reset"]
    #[inline] pub fn tim11rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM11RST != 0"]
    #[inline] pub fn test_tim11rst(&self) -> bool {
        self.tim11rst() != 0
    }

    #[doc="Sets the TIM11RST field."]
    #[inline] pub fn set_tim11rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM10 reset"]
    #[inline] pub fn tim10rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM10RST != 0"]
    #[inline] pub fn test_tim10rst(&self) -> bool {
        self.tim10rst() != 0
    }

    #[doc="Sets the TIM10RST field."]
    #[inline] pub fn set_tim10rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM9 reset"]
    #[inline] pub fn tim9rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM9RST != 0"]
    #[inline] pub fn test_tim9rst(&self) -> bool {
        self.tim9rst() != 0
    }

    #[doc="Sets the TIM9RST field."]
    #[inline] pub fn set_tim9rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System configuration controller reset"]
    #[inline] pub fn syscfgrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SYSCFGRST != 0"]
    #[inline] pub fn test_syscfgrst(&self) -> bool {
        self.syscfgrst() != 0
    }

    #[doc="Sets the SYSCFGRST field."]
    #[inline] pub fn set_syscfgrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI 1 reset"]
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

    #[doc="SDIO reset"]
    #[inline] pub fn sdiorst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SDIORST != 0"]
    #[inline] pub fn test_sdiorst(&self) -> bool {
        self.sdiorst() != 0
    }

    #[doc="Sets the SDIORST field."]
    #[inline] pub fn set_sdiorst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ADC interface reset (common to all ADCs)"]
    #[inline] pub fn adcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADCRST != 0"]
    #[inline] pub fn test_adcrst(&self) -> bool {
        self.adcrst() != 0
    }

    #[doc="Sets the ADCRST field."]
    #[inline] pub fn set_adcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="USART6 reset"]
    #[inline] pub fn usart6rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USART6RST != 0"]
    #[inline] pub fn test_usart6rst(&self) -> bool {
        self.usart6rst() != 0
    }

    #[doc="Sets the USART6RST field."]
    #[inline] pub fn set_usart6rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="USART1 reset"]
    #[inline] pub fn usart1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if USART1RST != 0"]
    #[inline] pub fn test_usart1rst(&self) -> bool {
        self.usart1rst() != 0
    }

    #[doc="Sets the USART1RST field."]
    #[inline] pub fn set_usart1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM8 reset"]
    #[inline] pub fn tim8rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM8RST != 0"]
    #[inline] pub fn test_tim8rst(&self) -> bool {
        self.tim8rst() != 0
    }

    #[doc="Sets the TIM8RST field."]
    #[inline] pub fn set_tim8rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM1 reset"]
    #[inline] pub fn tim1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM1RST != 0"]
    #[inline] pub fn test_tim1rst(&self) -> bool {
        self.tim1rst() != 0
    }

    #[doc="Sets the TIM1RST field."]
    #[inline] pub fn set_tim1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.tim11rst() != 0 { try!(write!(f, " tim11rst"))}
        if self.tim10rst() != 0 { try!(write!(f, " tim10rst"))}
        if self.tim9rst() != 0 { try!(write!(f, " tim9rst"))}
        if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
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
    #[inline] pub fn otghsulpien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OTGHSULPIEN != 0"]
    #[inline] pub fn test_otghsulpien(&self) -> bool {
        self.otghsulpien() != 0
    }

    #[doc="Sets the OTGHSULPIEN field."]
    #[inline] pub fn set_otghsulpien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="USB OTG HS clock enable"]
    #[inline] pub fn otghsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if OTGHSEN != 0"]
    #[inline] pub fn test_otghsen(&self) -> bool {
        self.otghsen() != 0
    }

    #[doc="Sets the OTGHSEN field."]
    #[inline] pub fn set_otghsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Ethernet PTP clock enable"]
    #[inline] pub fn ethmacptpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if ETHMACPTPEN != 0"]
    #[inline] pub fn test_ethmacptpen(&self) -> bool {
        self.ethmacptpen() != 0
    }

    #[doc="Sets the ETHMACPTPEN field."]
    #[inline] pub fn set_ethmacptpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Ethernet Reception clock enable"]
    #[inline] pub fn ethmacrxen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if ETHMACRXEN != 0"]
    #[inline] pub fn test_ethmacrxen(&self) -> bool {
        self.ethmacrxen() != 0
    }

    #[doc="Sets the ETHMACRXEN field."]
    #[inline] pub fn set_ethmacrxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Ethernet Transmission clock enable"]
    #[inline] pub fn ethmactxen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if ETHMACTXEN != 0"]
    #[inline] pub fn test_ethmactxen(&self) -> bool {
        self.ethmactxen() != 0
    }

    #[doc="Sets the ETHMACTXEN field."]
    #[inline] pub fn set_ethmactxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Ethernet MAC clock enable"]
    #[inline] pub fn ethmacen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ETHMACEN != 0"]
    #[inline] pub fn test_ethmacen(&self) -> bool {
        self.ethmacen() != 0
    }

    #[doc="Sets the ETHMACEN field."]
    #[inline] pub fn set_ethmacen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DMA2 clock enable"]
    #[inline] pub fn dma2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DMA2EN != 0"]
    #[inline] pub fn test_dma2en(&self) -> bool {
        self.dma2en() != 0
    }

    #[doc="Sets the DMA2EN field."]
    #[inline] pub fn set_dma2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DMA1 clock enable"]
    #[inline] pub fn dma1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DMA1EN != 0"]
    #[inline] pub fn test_dma1en(&self) -> bool {
        self.dma1en() != 0
    }

    #[doc="Sets the DMA1EN field."]
    #[inline] pub fn set_dma1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Backup SRAM interface clock enable"]
    #[inline] pub fn bkpsramen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if BKPSRAMEN != 0"]
    #[inline] pub fn test_bkpsramen(&self) -> bool {
        self.bkpsramen() != 0
    }

    #[doc="Sets the BKPSRAMEN field."]
    #[inline] pub fn set_bkpsramen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="CRC clock enable"]
    #[inline] pub fn crcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCEN != 0"]
    #[inline] pub fn test_crcen(&self) -> bool {
        self.crcen() != 0
    }

    #[doc="Sets the CRCEN field."]
    #[inline] pub fn set_crcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IO port I clock enable"]
    #[inline] pub fn gpioien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIOIEN != 0"]
    #[inline] pub fn test_gpioien(&self) -> bool {
        self.gpioien() != 0
    }

    #[doc="Sets the GPIOIEN field."]
    #[inline] pub fn set_gpioien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
        if self.dma1en() != 0 { try!(write!(f, " dma1en"))}
        if self.bkpsramen() != 0 { try!(write!(f, " bkpsramen"))}
        if self.crcen() != 0 { try!(write!(f, " crcen"))}
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
    #[inline] pub fn otgfsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OTGFSEN != 0"]
    #[inline] pub fn test_otgfsen(&self) -> bool {
        self.otgfsen() != 0
    }

    #[doc="Sets the OTGFSEN field."]
    #[inline] pub fn set_otgfsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Random number generator clock enable"]
    #[inline] pub fn rngen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RNGEN != 0"]
    #[inline] pub fn test_rngen(&self) -> bool {
        self.rngen() != 0
    }

    #[doc="Sets the RNGEN field."]
    #[inline] pub fn set_rngen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Camera interface enable"]
    #[inline] pub fn dcmien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCMIEN != 0"]
    #[inline] pub fn test_dcmien(&self) -> bool {
        self.dcmien() != 0
    }

    #[doc="Sets the DCMIEN field."]
    #[inline] pub fn set_dcmien<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
    #[doc="Flexible static memory controller module clock enable"]
    #[inline] pub fn fsmcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSMCEN != 0"]
    #[inline] pub fn test_fsmcen(&self) -> bool {
        self.fsmcen() != 0
    }

    #[doc="Sets the FSMCEN field."]
    #[inline] pub fn set_fsmcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.fsmcen() != 0 { try!(write!(f, " fsmcen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
    #[doc="DAC interface clock enable"]
    #[inline] pub fn dacen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACEN != 0"]
    #[inline] pub fn test_dacen(&self) -> bool {
        self.dacen() != 0
    }

    #[doc="Sets the DACEN field."]
    #[inline] pub fn set_dacen<V: Into<bits::U1>>(mut self, value: V) -> Self {
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

    #[doc="CAN 2 clock enable"]
    #[inline] pub fn can2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CAN2EN != 0"]
    #[inline] pub fn test_can2en(&self) -> bool {
        self.can2en() != 0
    }

    #[doc="Sets the CAN2EN field."]
    #[inline] pub fn set_can2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CAN 1 clock enable"]
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

    #[doc="USART 2 clock enable"]
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

    #[doc="TIM14 clock enable"]
    #[inline] pub fn tim14en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TIM14EN != 0"]
    #[inline] pub fn test_tim14en(&self) -> bool {
        self.tim14en() != 0
    }

    #[doc="Sets the TIM14EN field."]
    #[inline] pub fn set_tim14en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM13 clock enable"]
    #[inline] pub fn tim13en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIM13EN != 0"]
    #[inline] pub fn test_tim13en(&self) -> bool {
        self.tim13en() != 0
    }

    #[doc="Sets the TIM13EN field."]
    #[inline] pub fn set_tim13en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TIM12 clock enable"]
    #[inline] pub fn tim12en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIM12EN != 0"]
    #[inline] pub fn test_tim12en(&self) -> bool {
        self.tim12en() != 0
    }

    #[doc="Sets the TIM12EN field."]
    #[inline] pub fn set_tim12en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TIM7 clock enable"]
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

    #[doc="TIM6 clock enable"]
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

    #[doc="TIM5 clock enable"]
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

    #[doc="TIM4 clock enable"]
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

    #[doc="TIM3 clock enable"]
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

    #[doc="TIM2 clock enable"]
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
    #[doc="SPI6 clock enable"]
    #[inline] pub fn spi6en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SPI6EN != 0"]
    #[inline] pub fn test_spi6en(&self) -> bool {
        self.spi6en() != 0
    }

    #[doc="Sets the SPI6EN field."]
    #[inline] pub fn set_spi6en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SPI5 clock enable"]
    #[inline] pub fn spi5en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SPI5EN != 0"]
    #[inline] pub fn test_spi5en(&self) -> bool {
        self.spi5en() != 0
    }

    #[doc="Sets the SPI5EN field."]
    #[inline] pub fn set_spi5en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="TIM11 clock enable"]
    #[inline] pub fn tim11en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM11EN != 0"]
    #[inline] pub fn test_tim11en(&self) -> bool {
        self.tim11en() != 0
    }

    #[doc="Sets the TIM11EN field."]
    #[inline] pub fn set_tim11en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM10 clock enable"]
    #[inline] pub fn tim10en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM10EN != 0"]
    #[inline] pub fn test_tim10en(&self) -> bool {
        self.tim10en() != 0
    }

    #[doc="Sets the TIM10EN field."]
    #[inline] pub fn set_tim10en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM9 clock enable"]
    #[inline] pub fn tim9en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM9EN != 0"]
    #[inline] pub fn test_tim9en(&self) -> bool {
        self.tim9en() != 0
    }

    #[doc="Sets the TIM9EN field."]
    #[inline] pub fn set_tim9en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System configuration controller clock enable"]
    #[inline] pub fn syscfgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SYSCFGEN != 0"]
    #[inline] pub fn test_syscfgen(&self) -> bool {
        self.syscfgen() != 0
    }

    #[doc="Sets the SYSCFGEN field."]
    #[inline] pub fn set_syscfgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI4 clock enable"]
    #[inline] pub fn spi4en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SPI4EN != 0"]
    #[inline] pub fn test_spi4en(&self) -> bool {
        self.spi4en() != 0
    }

    #[doc="Sets the SPI4EN field."]
    #[inline] pub fn set_spi4en<V: Into<bits::U1>>(mut self, value: V) -> Self {
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

    #[doc="SDIO clock enable"]
    #[inline] pub fn sdioen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SDIOEN != 0"]
    #[inline] pub fn test_sdioen(&self) -> bool {
        self.sdioen() != 0
    }

    #[doc="Sets the SDIOEN field."]
    #[inline] pub fn set_sdioen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ADC3 clock enable"]
    #[inline] pub fn adc3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ADC3EN != 0"]
    #[inline] pub fn test_adc3en(&self) -> bool {
        self.adc3en() != 0
    }

    #[doc="Sets the ADC3EN field."]
    #[inline] pub fn set_adc3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ADC2 clock enable"]
    #[inline] pub fn adc2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADC2EN != 0"]
    #[inline] pub fn test_adc2en(&self) -> bool {
        self.adc2en() != 0
    }

    #[doc="Sets the ADC2EN field."]
    #[inline] pub fn set_adc2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="ADC1 clock enable"]
    #[inline] pub fn adc1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1EN != 0"]
    #[inline] pub fn test_adc1en(&self) -> bool {
        self.adc1en() != 0
    }

    #[doc="Sets the ADC1EN field."]
    #[inline] pub fn set_adc1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="USART6 clock enable"]
    #[inline] pub fn usart6en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USART6EN != 0"]
    #[inline] pub fn test_usart6en(&self) -> bool {
        self.usart6en() != 0
    }

    #[doc="Sets the USART6EN field."]
    #[inline] pub fn set_usart6en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="USART1 clock enable"]
    #[inline] pub fn usart1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if USART1EN != 0"]
    #[inline] pub fn test_usart1en(&self) -> bool {
        self.usart1en() != 0
    }

    #[doc="Sets the USART1EN field."]
    #[inline] pub fn set_usart1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM8 clock enable"]
    #[inline] pub fn tim8en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM8EN != 0"]
    #[inline] pub fn test_tim8en(&self) -> bool {
        self.tim8en() != 0
    }

    #[doc="Sets the TIM8EN field."]
    #[inline] pub fn set_tim8en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM1 clock enable"]
    #[inline] pub fn tim1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM1EN != 0"]
    #[inline] pub fn test_tim1en(&self) -> bool {
        self.tim1en() != 0
    }

    #[doc="Sets the TIM1EN field."]
    #[inline] pub fn set_tim1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
    #[inline] pub fn otghsulpilpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OTGHSULPILPEN != 0"]
    #[inline] pub fn test_otghsulpilpen(&self) -> bool {
        self.otghsulpilpen() != 0
    }

    #[doc="Sets the OTGHSULPILPEN field."]
    #[inline] pub fn set_otghsulpilpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="USB OTG HS clock enable during Sleep mode"]
    #[inline] pub fn otghslpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if OTGHSLPEN != 0"]
    #[inline] pub fn test_otghslpen(&self) -> bool {
        self.otghslpen() != 0
    }

    #[doc="Sets the OTGHSLPEN field."]
    #[inline] pub fn set_otghslpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Ethernet PTP clock enable during Sleep mode"]
    #[inline] pub fn ethmacptplpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if ETHMACPTPLPEN != 0"]
    #[inline] pub fn test_ethmacptplpen(&self) -> bool {
        self.ethmacptplpen() != 0
    }

    #[doc="Sets the ETHMACPTPLPEN field."]
    #[inline] pub fn set_ethmacptplpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Ethernet reception clock enable during Sleep mode"]
    #[inline] pub fn ethmacrxlpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if ETHMACRXLPEN != 0"]
    #[inline] pub fn test_ethmacrxlpen(&self) -> bool {
        self.ethmacrxlpen() != 0
    }

    #[doc="Sets the ETHMACRXLPEN field."]
    #[inline] pub fn set_ethmacrxlpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Ethernet transmission clock enable during Sleep mode"]
    #[inline] pub fn ethmactxlpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if ETHMACTXLPEN != 0"]
    #[inline] pub fn test_ethmactxlpen(&self) -> bool {
        self.ethmactxlpen() != 0
    }

    #[doc="Sets the ETHMACTXLPEN field."]
    #[inline] pub fn set_ethmactxlpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Ethernet MAC clock enable during Sleep mode"]
    #[inline] pub fn ethmaclpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ETHMACLPEN != 0"]
    #[inline] pub fn test_ethmaclpen(&self) -> bool {
        self.ethmaclpen() != 0
    }

    #[doc="Sets the ETHMACLPEN field."]
    #[inline] pub fn set_ethmaclpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DMA2 clock enable during Sleep mode"]
    #[inline] pub fn dma2lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DMA2LPEN != 0"]
    #[inline] pub fn test_dma2lpen(&self) -> bool {
        self.dma2lpen() != 0
    }

    #[doc="Sets the DMA2LPEN field."]
    #[inline] pub fn set_dma2lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DMA1 clock enable during Sleep mode"]
    #[inline] pub fn dma1lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DMA1LPEN != 0"]
    #[inline] pub fn test_dma1lpen(&self) -> bool {
        self.dma1lpen() != 0
    }

    #[doc="Sets the DMA1LPEN field."]
    #[inline] pub fn set_dma1lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Backup SRAM interface clock enable during Sleep mode"]
    #[inline] pub fn bkpsramlpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if BKPSRAMLPEN != 0"]
    #[inline] pub fn test_bkpsramlpen(&self) -> bool {
        self.bkpsramlpen() != 0
    }

    #[doc="Sets the BKPSRAMLPEN field."]
    #[inline] pub fn set_bkpsramlpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SRAM 2 interface clock enable during Sleep mode"]
    #[inline] pub fn sram2lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SRAM2LPEN != 0"]
    #[inline] pub fn test_sram2lpen(&self) -> bool {
        self.sram2lpen() != 0
    }

    #[doc="Sets the SRAM2LPEN field."]
    #[inline] pub fn set_sram2lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SRAM 1interface clock enable during Sleep mode"]
    #[inline] pub fn sram1lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SRAM1LPEN != 0"]
    #[inline] pub fn test_sram1lpen(&self) -> bool {
        self.sram1lpen() != 0
    }

    #[doc="Sets the SRAM1LPEN field."]
    #[inline] pub fn set_sram1lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Flash interface clock enable during Sleep mode"]
    #[inline] pub fn flitflpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FLITFLPEN != 0"]
    #[inline] pub fn test_flitflpen(&self) -> bool {
        self.flitflpen() != 0
    }

    #[doc="Sets the FLITFLPEN field."]
    #[inline] pub fn set_flitflpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="CRC clock enable during Sleep mode"]
    #[inline] pub fn crclpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCLPEN != 0"]
    #[inline] pub fn test_crclpen(&self) -> bool {
        self.crclpen() != 0
    }

    #[doc="Sets the CRCLPEN field."]
    #[inline] pub fn set_crclpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="IO port I clock enable during Sleep mode"]
    #[inline] pub fn gpioilpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIOILPEN != 0"]
    #[inline] pub fn test_gpioilpen(&self) -> bool {
        self.gpioilpen() != 0
    }

    #[doc="Sets the GPIOILPEN field."]
    #[inline] pub fn set_gpioilpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IO port H clock enable during Sleep mode"]
    #[inline] pub fn gpiohlpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIOHLPEN != 0"]
    #[inline] pub fn test_gpiohlpen(&self) -> bool {
        self.gpiohlpen() != 0
    }

    #[doc="Sets the GPIOHLPEN field."]
    #[inline] pub fn set_gpiohlpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IO port G clock enable during Sleep mode"]
    #[inline] pub fn gpioglpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIOGLPEN != 0"]
    #[inline] pub fn test_gpioglpen(&self) -> bool {
        self.gpioglpen() != 0
    }

    #[doc="Sets the GPIOGLPEN field."]
    #[inline] pub fn set_gpioglpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="IO port F clock enable during Sleep mode"]
    #[inline] pub fn gpioflpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIOFLPEN != 0"]
    #[inline] pub fn test_gpioflpen(&self) -> bool {
        self.gpioflpen() != 0
    }

    #[doc="Sets the GPIOFLPEN field."]
    #[inline] pub fn set_gpioflpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IO port E clock enable during Sleep mode"]
    #[inline] pub fn gpioelpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIOELPEN != 0"]
    #[inline] pub fn test_gpioelpen(&self) -> bool {
        self.gpioelpen() != 0
    }

    #[doc="Sets the GPIOELPEN field."]
    #[inline] pub fn set_gpioelpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IO port D clock enable during Sleep mode"]
    #[inline] pub fn gpiodlpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIODLPEN != 0"]
    #[inline] pub fn test_gpiodlpen(&self) -> bool {
        self.gpiodlpen() != 0
    }

    #[doc="Sets the GPIODLPEN field."]
    #[inline] pub fn set_gpiodlpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IO port C clock enable during Sleep mode"]
    #[inline] pub fn gpioclpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIOCLPEN != 0"]
    #[inline] pub fn test_gpioclpen(&self) -> bool {
        self.gpioclpen() != 0
    }

    #[doc="Sets the GPIOCLPEN field."]
    #[inline] pub fn set_gpioclpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IO port B clock enable during Sleep mode"]
    #[inline] pub fn gpioblpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIOBLPEN != 0"]
    #[inline] pub fn test_gpioblpen(&self) -> bool {
        self.gpioblpen() != 0
    }

    #[doc="Sets the GPIOBLPEN field."]
    #[inline] pub fn set_gpioblpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IO port A clock enable during sleep mode"]
    #[inline] pub fn gpioalpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOALPEN != 0"]
    #[inline] pub fn test_gpioalpen(&self) -> bool {
        self.gpioalpen() != 0
    }

    #[doc="Sets the GPIOALPEN field."]
    #[inline] pub fn set_gpioalpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.dma2lpen() != 0 { try!(write!(f, " dma2lpen"))}
        if self.dma1lpen() != 0 { try!(write!(f, " dma1lpen"))}
        if self.bkpsramlpen() != 0 { try!(write!(f, " bkpsramlpen"))}
        if self.sram2lpen() != 0 { try!(write!(f, " sram2lpen"))}
        if self.sram1lpen() != 0 { try!(write!(f, " sram1lpen"))}
        if self.flitflpen() != 0 { try!(write!(f, " flitflpen"))}
        if self.crclpen() != 0 { try!(write!(f, " crclpen"))}
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
    #[inline] pub fn otgfslpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OTGFSLPEN != 0"]
    #[inline] pub fn test_otgfslpen(&self) -> bool {
        self.otgfslpen() != 0
    }

    #[doc="Sets the OTGFSLPEN field."]
    #[inline] pub fn set_otgfslpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Random number generator clock enable during Sleep mode"]
    #[inline] pub fn rnglpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RNGLPEN != 0"]
    #[inline] pub fn test_rnglpen(&self) -> bool {
        self.rnglpen() != 0
    }

    #[doc="Sets the RNGLPEN field."]
    #[inline] pub fn set_rnglpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Camera interface enable during Sleep mode"]
    #[inline] pub fn dcmilpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCMILPEN != 0"]
    #[inline] pub fn test_dcmilpen(&self) -> bool {
        self.dcmilpen() != 0
    }

    #[doc="Sets the DCMILPEN field."]
    #[inline] pub fn set_dcmilpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[doc="Flexible static memory controller module clock enable during Sleep mode"]
    #[inline] pub fn fsmclpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSMCLPEN != 0"]
    #[inline] pub fn test_fsmclpen(&self) -> bool {
        self.fsmclpen() != 0
    }

    #[doc="Sets the FSMCLPEN field."]
    #[inline] pub fn set_fsmclpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.fsmclpen() != 0 { try!(write!(f, " fsmclpen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clock enable in low power mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1lpenr(pub u32);
impl Apb1lpenr {
    #[doc="DAC interface clock enable during Sleep mode"]
    #[inline] pub fn daclpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACLPEN != 0"]
    #[inline] pub fn test_daclpen(&self) -> bool {
        self.daclpen() != 0
    }

    #[doc="Sets the DACLPEN field."]
    #[inline] pub fn set_daclpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Power interface clock enable during Sleep mode"]
    #[inline] pub fn pwrlpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWRLPEN != 0"]
    #[inline] pub fn test_pwrlpen(&self) -> bool {
        self.pwrlpen() != 0
    }

    #[doc="Sets the PWRLPEN field."]
    #[inline] pub fn set_pwrlpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="CAN 2 clock enable during Sleep mode"]
    #[inline] pub fn can2lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CAN2LPEN != 0"]
    #[inline] pub fn test_can2lpen(&self) -> bool {
        self.can2lpen() != 0
    }

    #[doc="Sets the CAN2LPEN field."]
    #[inline] pub fn set_can2lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CAN 1 clock enable during Sleep mode"]
    #[inline] pub fn can1lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CAN1LPEN != 0"]
    #[inline] pub fn test_can1lpen(&self) -> bool {
        self.can1lpen() != 0
    }

    #[doc="Sets the CAN1LPEN field."]
    #[inline] pub fn set_can1lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="I2C3 clock enable during Sleep mode"]
    #[inline] pub fn i2c3lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2C3LPEN != 0"]
    #[inline] pub fn test_i2c3lpen(&self) -> bool {
        self.i2c3lpen() != 0
    }

    #[doc="Sets the I2C3LPEN field."]
    #[inline] pub fn set_i2c3lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C2 clock enable during Sleep mode"]
    #[inline] pub fn i2c2lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2LPEN != 0"]
    #[inline] pub fn test_i2c2lpen(&self) -> bool {
        self.i2c2lpen() != 0
    }

    #[doc="Sets the I2C2LPEN field."]
    #[inline] pub fn set_i2c2lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C1 clock enable during Sleep mode"]
    #[inline] pub fn i2c1lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1LPEN != 0"]
    #[inline] pub fn test_i2c1lpen(&self) -> bool {
        self.i2c1lpen() != 0
    }

    #[doc="Sets the I2C1LPEN field."]
    #[inline] pub fn set_i2c1lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="UART5 clock enable during Sleep mode"]
    #[inline] pub fn uart5lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5LPEN != 0"]
    #[inline] pub fn test_uart5lpen(&self) -> bool {
        self.uart5lpen() != 0
    }

    #[doc="Sets the UART5LPEN field."]
    #[inline] pub fn set_uart5lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="UART4 clock enable during Sleep mode"]
    #[inline] pub fn uart4lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4LPEN != 0"]
    #[inline] pub fn test_uart4lpen(&self) -> bool {
        self.uart4lpen() != 0
    }

    #[doc="Sets the UART4LPEN field."]
    #[inline] pub fn set_uart4lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="USART3 clock enable during Sleep mode"]
    #[inline] pub fn usart3lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USART3LPEN != 0"]
    #[inline] pub fn test_usart3lpen(&self) -> bool {
        self.usart3lpen() != 0
    }

    #[doc="Sets the USART3LPEN field."]
    #[inline] pub fn set_usart3lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USART2 clock enable during Sleep mode"]
    #[inline] pub fn usart2lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if USART2LPEN != 0"]
    #[inline] pub fn test_usart2lpen(&self) -> bool {
        self.usart2lpen() != 0
    }

    #[doc="Sets the USART2LPEN field."]
    #[inline] pub fn set_usart2lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI3 clock enable during Sleep mode"]
    #[inline] pub fn spi3lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SPI3LPEN != 0"]
    #[inline] pub fn test_spi3lpen(&self) -> bool {
        self.spi3lpen() != 0
    }

    #[doc="Sets the SPI3LPEN field."]
    #[inline] pub fn set_spi3lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SPI2 clock enable during Sleep mode"]
    #[inline] pub fn spi2lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2LPEN != 0"]
    #[inline] pub fn test_spi2lpen(&self) -> bool {
        self.spi2lpen() != 0
    }

    #[doc="Sets the SPI2LPEN field."]
    #[inline] pub fn set_spi2lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Window watchdog clock enable during Sleep mode"]
    #[inline] pub fn wwdglpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGLPEN != 0"]
    #[inline] pub fn test_wwdglpen(&self) -> bool {
        self.wwdglpen() != 0
    }

    #[doc="Sets the WWDGLPEN field."]
    #[inline] pub fn set_wwdglpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TIM14 clock enable during Sleep mode"]
    #[inline] pub fn tim14lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TIM14LPEN != 0"]
    #[inline] pub fn test_tim14lpen(&self) -> bool {
        self.tim14lpen() != 0
    }

    #[doc="Sets the TIM14LPEN field."]
    #[inline] pub fn set_tim14lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM13 clock enable during Sleep mode"]
    #[inline] pub fn tim13lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIM13LPEN != 0"]
    #[inline] pub fn test_tim13lpen(&self) -> bool {
        self.tim13lpen() != 0
    }

    #[doc="Sets the TIM13LPEN field."]
    #[inline] pub fn set_tim13lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TIM12 clock enable during Sleep mode"]
    #[inline] pub fn tim12lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIM12LPEN != 0"]
    #[inline] pub fn test_tim12lpen(&self) -> bool {
        self.tim12lpen() != 0
    }

    #[doc="Sets the TIM12LPEN field."]
    #[inline] pub fn set_tim12lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TIM7 clock enable during Sleep mode"]
    #[inline] pub fn tim7lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7LPEN != 0"]
    #[inline] pub fn test_tim7lpen(&self) -> bool {
        self.tim7lpen() != 0
    }

    #[doc="Sets the TIM7LPEN field."]
    #[inline] pub fn set_tim7lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM6 clock enable during Sleep mode"]
    #[inline] pub fn tim6lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6LPEN != 0"]
    #[inline] pub fn test_tim6lpen(&self) -> bool {
        self.tim6lpen() != 0
    }

    #[doc="Sets the TIM6LPEN field."]
    #[inline] pub fn set_tim6lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM5 clock enable during Sleep mode"]
    #[inline] pub fn tim5lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TIM5LPEN != 0"]
    #[inline] pub fn test_tim5lpen(&self) -> bool {
        self.tim5lpen() != 0
    }

    #[doc="Sets the TIM5LPEN field."]
    #[inline] pub fn set_tim5lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TIM4 clock enable during Sleep mode"]
    #[inline] pub fn tim4lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4LPEN != 0"]
    #[inline] pub fn test_tim4lpen(&self) -> bool {
        self.tim4lpen() != 0
    }

    #[doc="Sets the TIM4LPEN field."]
    #[inline] pub fn set_tim4lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TIM3 clock enable during Sleep mode"]
    #[inline] pub fn tim3lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3LPEN != 0"]
    #[inline] pub fn test_tim3lpen(&self) -> bool {
        self.tim3lpen() != 0
    }

    #[doc="Sets the TIM3LPEN field."]
    #[inline] pub fn set_tim3lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM2 clock enable during Sleep mode"]
    #[inline] pub fn tim2lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2LPEN != 0"]
    #[inline] pub fn test_tim2lpen(&self) -> bool {
        self.tim2lpen() != 0
    }

    #[doc="Sets the TIM2LPEN field."]
    #[inline] pub fn set_tim2lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[doc="TIM11 clock enable during Sleep mode"]
    #[inline] pub fn tim11lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM11LPEN != 0"]
    #[inline] pub fn test_tim11lpen(&self) -> bool {
        self.tim11lpen() != 0
    }

    #[doc="Sets the TIM11LPEN field."]
    #[inline] pub fn set_tim11lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM10 clock enable during Sleep mode"]
    #[inline] pub fn tim10lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM10LPEN != 0"]
    #[inline] pub fn test_tim10lpen(&self) -> bool {
        self.tim10lpen() != 0
    }

    #[doc="Sets the TIM10LPEN field."]
    #[inline] pub fn set_tim10lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM9 clock enable during sleep mode"]
    #[inline] pub fn tim9lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM9LPEN != 0"]
    #[inline] pub fn test_tim9lpen(&self) -> bool {
        self.tim9lpen() != 0
    }

    #[doc="Sets the TIM9LPEN field."]
    #[inline] pub fn set_tim9lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="System configuration controller clock enable during Sleep mode"]
    #[inline] pub fn syscfglpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SYSCFGLPEN != 0"]
    #[inline] pub fn test_syscfglpen(&self) -> bool {
        self.syscfglpen() != 0
    }

    #[doc="Sets the SYSCFGLPEN field."]
    #[inline] pub fn set_syscfglpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI 1 clock enable during Sleep mode"]
    #[inline] pub fn spi1lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1LPEN != 0"]
    #[inline] pub fn test_spi1lpen(&self) -> bool {
        self.spi1lpen() != 0
    }

    #[doc="Sets the SPI1LPEN field."]
    #[inline] pub fn set_spi1lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SDIO clock enable during Sleep mode"]
    #[inline] pub fn sdiolpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SDIOLPEN != 0"]
    #[inline] pub fn test_sdiolpen(&self) -> bool {
        self.sdiolpen() != 0
    }

    #[doc="Sets the SDIOLPEN field."]
    #[inline] pub fn set_sdiolpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ADC 3 clock enable during Sleep mode"]
    #[inline] pub fn adc3lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ADC3LPEN != 0"]
    #[inline] pub fn test_adc3lpen(&self) -> bool {
        self.adc3lpen() != 0
    }

    #[doc="Sets the ADC3LPEN field."]
    #[inline] pub fn set_adc3lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ADC2 clock enable during Sleep mode"]
    #[inline] pub fn adc2lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADC2LPEN != 0"]
    #[inline] pub fn test_adc2lpen(&self) -> bool {
        self.adc2lpen() != 0
    }

    #[doc="Sets the ADC2LPEN field."]
    #[inline] pub fn set_adc2lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="ADC1 clock enable during Sleep mode"]
    #[inline] pub fn adc1lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1LPEN != 0"]
    #[inline] pub fn test_adc1lpen(&self) -> bool {
        self.adc1lpen() != 0
    }

    #[doc="Sets the ADC1LPEN field."]
    #[inline] pub fn set_adc1lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="USART6 clock enable during Sleep mode"]
    #[inline] pub fn usart6lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USART6LPEN != 0"]
    #[inline] pub fn test_usart6lpen(&self) -> bool {
        self.usart6lpen() != 0
    }

    #[doc="Sets the USART6LPEN field."]
    #[inline] pub fn set_usart6lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="USART1 clock enable during Sleep mode"]
    #[inline] pub fn usart1lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if USART1LPEN != 0"]
    #[inline] pub fn test_usart1lpen(&self) -> bool {
        self.usart1lpen() != 0
    }

    #[doc="Sets the USART1LPEN field."]
    #[inline] pub fn set_usart1lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM8 clock enable during Sleep mode"]
    #[inline] pub fn tim8lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM8LPEN != 0"]
    #[inline] pub fn test_tim8lpen(&self) -> bool {
        self.tim8lpen() != 0
    }

    #[doc="Sets the TIM8LPEN field."]
    #[inline] pub fn set_tim8lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM1 clock enable during Sleep mode"]
    #[inline] pub fn tim1lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM1LPEN != 0"]
    #[inline] pub fn test_tim1lpen(&self) -> bool {
        self.tim1lpen() != 0
    }

    #[doc="Sets the TIM1LPEN field."]
    #[inline] pub fn set_tim1lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.tim11lpen() != 0 { try!(write!(f, " tim11lpen"))}
        if self.tim10lpen() != 0 { try!(write!(f, " tim10lpen"))}
        if self.tim9lpen() != 0 { try!(write!(f, " tim9lpen"))}
        if self.syscfglpen() != 0 { try!(write!(f, " syscfglpen"))}
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
    #[inline] pub fn rtcsel1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RTCSEL1 != 0"]
    #[inline] pub fn test_rtcsel1(&self) -> bool {
        self.rtcsel1() != 0
    }

    #[doc="Sets the RTCSEL1 field."]
    #[inline] pub fn set_rtcsel1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="RTC clock source selection"]
    #[inline] pub fn rtcsel0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RTCSEL0 != 0"]
    #[inline] pub fn test_rtcsel0(&self) -> bool {
        self.rtcsel0() != 0
    }

    #[doc="Sets the RTCSEL0 field."]
    #[inline] pub fn set_rtcsel0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="External low-speed oscillator bypass"]
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

    #[doc="External low-speed oscillator ready"]
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

    #[doc="External low-speed oscillator enable"]
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
    #[inline] pub fn lpwrrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPWRRSTF != 0"]
    #[inline] pub fn test_lpwrrstf(&self) -> bool {
        self.lpwrrstf() != 0
    }

    #[doc="Sets the LPWRRSTF field."]
    #[inline] pub fn set_lpwrrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
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

    #[doc="Independent watchdog reset flag"]
    #[inline] pub fn wdgrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if WDGRSTF != 0"]
    #[inline] pub fn test_wdgrstf(&self) -> bool {
        self.wdgrstf() != 0
    }

    #[doc="Sets the WDGRSTF field."]
    #[inline] pub fn set_wdgrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
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

    #[doc="POR/PDR reset flag"]
    #[inline] pub fn porrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PORRSTF != 0"]
    #[inline] pub fn test_porrstf(&self) -> bool {
        self.porrstf() != 0
    }

    #[doc="Sets the PORRSTF field."]
    #[inline] pub fn set_porrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="PIN reset flag"]
    #[inline] pub fn padrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PADRSTF != 0"]
    #[inline] pub fn test_padrstf(&self) -> bool {
        self.padrstf() != 0
    }

    #[doc="Sets the PADRSTF field."]
    #[inline] pub fn set_padrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="BOR reset flag"]
    #[inline] pub fn borrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if BORRSTF != 0"]
    #[inline] pub fn test_borrstf(&self) -> bool {
        self.borrstf() != 0
    }

    #[doc="Sets the BORRSTF field."]
    #[inline] pub fn set_borrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Remove reset flag"]
    #[inline] pub fn rmvf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RMVF != 0"]
    #[inline] pub fn test_rmvf(&self) -> bool {
        self.rmvf() != 0
    }

    #[doc="Sets the RMVF field."]
    #[inline] pub fn set_rmvf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Internal low-speed oscillator ready"]
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

    #[doc="Internal low-speed oscillator enable"]
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
    #[inline] pub fn sscgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SSCGEN != 0"]
    #[inline] pub fn test_sscgen(&self) -> bool {
        self.sscgen() != 0
    }

    #[doc="Sets the SSCGEN field."]
    #[inline] pub fn set_sscgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Spread Select"]
    #[inline] pub fn spreadsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SPREADSEL != 0"]
    #[inline] pub fn test_spreadsel(&self) -> bool {
        self.spreadsel() != 0
    }

    #[doc="Sets the SPREADSEL field."]
    #[inline] pub fn set_spreadsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Incrementation step"]
    #[inline] pub fn incstep(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7fff) as u16) } // [27:13]
    }

    #[doc="Returns true if INCSTEP != 0"]
    #[inline] pub fn test_incstep(&self) -> bool {
        self.incstep() != 0
    }

    #[doc="Sets the INCSTEP field."]
    #[inline] pub fn set_incstep<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Modulation period"]
    #[inline] pub fn modper(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if MODPER != 0"]
    #[inline] pub fn test_modper(&self) -> bool {
        self.modper() != 0
    }

    #[doc="Sets the MODPER field."]
    #[inline] pub fn set_modper<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
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
    #[inline] pub fn plli2sr(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if PLLI2SR != 0"]
    #[inline] pub fn test_plli2sr(&self) -> bool {
        self.plli2sr() != 0
    }

    #[doc="Sets the PLLI2SR field."]
    #[inline] pub fn set_plli2sr<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="PLLI2S multiplication factor for VCO"]
    #[inline] pub fn plli2sn(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1ff) as u16) } // [14:6]
    }

    #[doc="Returns true if PLLI2SN != 0"]
    #[inline] pub fn test_plli2sn(&self) -> bool {
        self.plli2sn() != 0
    }

    #[doc="Sets the PLLI2SN field."]
    #[inline] pub fn set_plli2sn<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
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
        if self.plli2sn() != 0 { try!(write!(f, " plli2sn=0x{:x}", self.plli2sn()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

