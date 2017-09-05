//! Reset and clock control
#[allow(unused_imports)] use bobbin_common::*;

periph!(RCC, Rcc, 0x40021000);

#[doc="Reset and clock control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcc(pub usize);
impl Rcc {
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

    #[doc="Get the *mut pointer for the CRRCR register."]
    #[inline] pub fn crrcr_mut(&self) -> *mut Crrcr { 
        (self.0 + 0x8) as *mut Crrcr
    }

    #[doc="Get the *const pointer for the CRRCR register."]
    #[inline] pub fn crrcr_ptr(&self) -> *const Crrcr { 
           self.crrcr_mut()
    }

    #[doc="Read the CRRCR register."]
    #[inline] pub fn crrcr(&self) -> Crrcr { 
        unsafe {
            read_volatile(self.crrcr_ptr())
        }
    }

    #[doc="Write the CRRCR register."]
    #[inline] pub fn set_crrcr<F: FnOnce(Crrcr) -> Crrcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crrcr_mut(), f(Crrcr(0)));
        }
        self
    }

    #[doc="Modify the CRRCR register."]
    #[inline] pub fn with_crrcr<F: FnOnce(Crrcr) -> Crrcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crrcr_mut(), f(self.crrcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR register."]
    #[inline] pub fn cfgr_mut(&self) -> *mut Cfgr { 
        (self.0 + 0xc) as *mut Cfgr
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

    #[doc="Get the *mut pointer for the CIER register."]
    #[inline] pub fn cier_mut(&self) -> *mut Cier { 
        (self.0 + 0x10) as *mut Cier
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

    #[doc="Get the *mut pointer for the CIFR register."]
    #[inline] pub fn cifr_mut(&self) -> *mut Cifr { 
        (self.0 + 0x14) as *mut Cifr
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
        (self.0 + 0x18) as *mut Cicr
    }

    #[doc="Get the *const pointer for the CICR register."]
    #[inline] pub fn cicr_ptr(&self) -> *const Cicr { 
           self.cicr_mut()
    }

    #[doc="Read the CICR register."]
    #[inline] pub fn cicr(&self) -> Cicr { 
        unsafe {
            read_volatile(self.cicr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IOPRSTR register."]
    #[inline] pub fn ioprstr_mut(&self) -> *mut Ioprstr { 
        (self.0 + 0x1c) as *mut Ioprstr
    }

    #[doc="Get the *const pointer for the IOPRSTR register."]
    #[inline] pub fn ioprstr_ptr(&self) -> *const Ioprstr { 
           self.ioprstr_mut()
    }

    #[doc="Read the IOPRSTR register."]
    #[inline] pub fn ioprstr(&self) -> Ioprstr { 
        unsafe {
            read_volatile(self.ioprstr_ptr())
        }
    }

    #[doc="Write the IOPRSTR register."]
    #[inline] pub fn set_ioprstr<F: FnOnce(Ioprstr) -> Ioprstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ioprstr_mut(), f(Ioprstr(0)));
        }
        self
    }

    #[doc="Modify the IOPRSTR register."]
    #[inline] pub fn with_ioprstr<F: FnOnce(Ioprstr) -> Ioprstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ioprstr_mut(), f(self.ioprstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHBRSTR register."]
    #[inline] pub fn ahbrstr_mut(&self) -> *mut Ahbrstr { 
        (self.0 + 0x20) as *mut Ahbrstr
    }

    #[doc="Get the *const pointer for the AHBRSTR register."]
    #[inline] pub fn ahbrstr_ptr(&self) -> *const Ahbrstr { 
           self.ahbrstr_mut()
    }

    #[doc="Read the AHBRSTR register."]
    #[inline] pub fn ahbrstr(&self) -> Ahbrstr { 
        unsafe {
            read_volatile(self.ahbrstr_ptr())
        }
    }

    #[doc="Write the AHBRSTR register."]
    #[inline] pub fn set_ahbrstr<F: FnOnce(Ahbrstr) -> Ahbrstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbrstr_mut(), f(Ahbrstr(0)));
        }
        self
    }

    #[doc="Modify the AHBRSTR register."]
    #[inline] pub fn with_ahbrstr<F: FnOnce(Ahbrstr) -> Ahbrstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbrstr_mut(), f(self.ahbrstr()));
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

    #[doc="Get the *mut pointer for the APB1RSTR register."]
    #[inline] pub fn apb1rstr_mut(&self) -> *mut Apb1rstr { 
        (self.0 + 0x28) as *mut Apb1rstr
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

    #[doc="Get the *mut pointer for the IOPENR register."]
    #[inline] pub fn iopenr_mut(&self) -> *mut Iopenr { 
        (self.0 + 0x2c) as *mut Iopenr
    }

    #[doc="Get the *const pointer for the IOPENR register."]
    #[inline] pub fn iopenr_ptr(&self) -> *const Iopenr { 
           self.iopenr_mut()
    }

    #[doc="Read the IOPENR register."]
    #[inline] pub fn iopenr(&self) -> Iopenr { 
        unsafe {
            read_volatile(self.iopenr_ptr())
        }
    }

    #[doc="Write the IOPENR register."]
    #[inline] pub fn set_iopenr<F: FnOnce(Iopenr) -> Iopenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iopenr_mut(), f(Iopenr(0)));
        }
        self
    }

    #[doc="Modify the IOPENR register."]
    #[inline] pub fn with_iopenr<F: FnOnce(Iopenr) -> Iopenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iopenr_mut(), f(self.iopenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHBENR register."]
    #[inline] pub fn ahbenr_mut(&self) -> *mut Ahbenr { 
        (self.0 + 0x30) as *mut Ahbenr
    }

    #[doc="Get the *const pointer for the AHBENR register."]
    #[inline] pub fn ahbenr_ptr(&self) -> *const Ahbenr { 
           self.ahbenr_mut()
    }

    #[doc="Read the AHBENR register."]
    #[inline] pub fn ahbenr(&self) -> Ahbenr { 
        unsafe {
            read_volatile(self.ahbenr_ptr())
        }
    }

    #[doc="Write the AHBENR register."]
    #[inline] pub fn set_ahbenr<F: FnOnce(Ahbenr) -> Ahbenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbenr_mut(), f(Ahbenr(0)));
        }
        self
    }

    #[doc="Modify the AHBENR register."]
    #[inline] pub fn with_ahbenr<F: FnOnce(Ahbenr) -> Ahbenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbenr_mut(), f(self.ahbenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2ENR register."]
    #[inline] pub fn apb2enr_mut(&self) -> *mut Apb2enr { 
        (self.0 + 0x34) as *mut Apb2enr
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

    #[doc="Get the *mut pointer for the APB1ENR register."]
    #[inline] pub fn apb1enr_mut(&self) -> *mut Apb1enr { 
        (self.0 + 0x38) as *mut Apb1enr
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

    #[doc="Get the *mut pointer for the IOPSMEN register."]
    #[inline] pub fn iopsmen_mut(&self) -> *mut Iopsmen { 
        (self.0 + 0x3c) as *mut Iopsmen
    }

    #[doc="Get the *const pointer for the IOPSMEN register."]
    #[inline] pub fn iopsmen_ptr(&self) -> *const Iopsmen { 
           self.iopsmen_mut()
    }

    #[doc="Read the IOPSMEN register."]
    #[inline] pub fn iopsmen(&self) -> Iopsmen { 
        unsafe {
            read_volatile(self.iopsmen_ptr())
        }
    }

    #[doc="Write the IOPSMEN register."]
    #[inline] pub fn set_iopsmen<F: FnOnce(Iopsmen) -> Iopsmen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iopsmen_mut(), f(Iopsmen(0)));
        }
        self
    }

    #[doc="Modify the IOPSMEN register."]
    #[inline] pub fn with_iopsmen<F: FnOnce(Iopsmen) -> Iopsmen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iopsmen_mut(), f(self.iopsmen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHBSMENR register."]
    #[inline] pub fn ahbsmenr_mut(&self) -> *mut Ahbsmenr { 
        (self.0 + 0x40) as *mut Ahbsmenr
    }

    #[doc="Get the *const pointer for the AHBSMENR register."]
    #[inline] pub fn ahbsmenr_ptr(&self) -> *const Ahbsmenr { 
           self.ahbsmenr_mut()
    }

    #[doc="Read the AHBSMENR register."]
    #[inline] pub fn ahbsmenr(&self) -> Ahbsmenr { 
        unsafe {
            read_volatile(self.ahbsmenr_ptr())
        }
    }

    #[doc="Write the AHBSMENR register."]
    #[inline] pub fn set_ahbsmenr<F: FnOnce(Ahbsmenr) -> Ahbsmenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbsmenr_mut(), f(Ahbsmenr(0)));
        }
        self
    }

    #[doc="Modify the AHBSMENR register."]
    #[inline] pub fn with_ahbsmenr<F: FnOnce(Ahbsmenr) -> Ahbsmenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbsmenr_mut(), f(self.ahbsmenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2SMENR register."]
    #[inline] pub fn apb2smenr_mut(&self) -> *mut Apb2smenr { 
        (self.0 + 0x44) as *mut Apb2smenr
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

    #[doc="Get the *mut pointer for the APB1SMENR register."]
    #[inline] pub fn apb1smenr_mut(&self) -> *mut Apb1smenr { 
        (self.0 + 0x48) as *mut Apb1smenr
    }

    #[doc="Get the *const pointer for the APB1SMENR register."]
    #[inline] pub fn apb1smenr_ptr(&self) -> *const Apb1smenr { 
           self.apb1smenr_mut()
    }

    #[doc="Read the APB1SMENR register."]
    #[inline] pub fn apb1smenr(&self) -> Apb1smenr { 
        unsafe {
            read_volatile(self.apb1smenr_ptr())
        }
    }

    #[doc="Write the APB1SMENR register."]
    #[inline] pub fn set_apb1smenr<F: FnOnce(Apb1smenr) -> Apb1smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1smenr_mut(), f(Apb1smenr(0)));
        }
        self
    }

    #[doc="Modify the APB1SMENR register."]
    #[inline] pub fn with_apb1smenr<F: FnOnce(Apb1smenr) -> Apb1smenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1smenr_mut(), f(self.apb1smenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CCIPR register."]
    #[inline] pub fn ccipr_mut(&self) -> *mut Ccipr { 
        (self.0 + 0x4c) as *mut Ccipr
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

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        (self.0 + 0x50) as *mut Csr
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
    #[doc="PLL clock ready flag"]
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

    #[doc="PLL enable bit"]
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

    #[doc="TC/LCD prescaler"]
    #[inline] pub fn rtcpre(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if RTCPRE != 0"]
    #[inline] pub fn test_rtcpre(&self) -> bool {
        self.rtcpre() != 0
    }

    #[doc="Sets the RTCPRE field."]
    #[inline] pub fn set_rtcpre<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Clock security system on HSE enable bit"]
    #[inline] pub fn csslseon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CSSLSEON != 0"]
    #[inline] pub fn test_csslseon(&self) -> bool {
        self.csslseon() != 0
    }

    #[doc="Sets the CSSLSEON field."]
    #[inline] pub fn set_csslseon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="HSE clock bypass bit"]
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

    #[doc="HSE clock enable bit"]
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

    #[doc="MSI clock ready flag"]
    #[inline] pub fn msirdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if MSIRDY != 0"]
    #[inline] pub fn test_msirdy(&self) -> bool {
        self.msirdy() != 0
    }

    #[doc="Sets the MSIRDY field."]
    #[inline] pub fn set_msirdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="MSI clock enable bit"]
    #[inline] pub fn msion(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MSION != 0"]
    #[inline] pub fn test_msion(&self) -> bool {
        self.msion() != 0
    }

    #[doc="Sets the MSION field."]
    #[inline] pub fn set_msion<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HSI16DIVF"]
    #[inline] pub fn hsi16divf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HSI16DIVF != 0"]
    #[inline] pub fn test_hsi16divf(&self) -> bool {
        self.hsi16divf() != 0
    }

    #[doc="Sets the HSI16DIVF field."]
    #[inline] pub fn set_hsi16divf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSI16DIVEN"]
    #[inline] pub fn hsi16diven(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSI16DIVEN != 0"]
    #[inline] pub fn test_hsi16diven(&self) -> bool {
        self.hsi16diven() != 0
    }

    #[doc="Sets the HSI16DIVEN field."]
    #[inline] pub fn set_hsi16diven<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Internal high-speed clock ready flag"]
    #[inline] pub fn hsi16rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSI16RDYF != 0"]
    #[inline] pub fn test_hsi16rdyf(&self) -> bool {
        self.hsi16rdyf() != 0
    }

    #[doc="Sets the HSI16RDYF field."]
    #[inline] pub fn set_hsi16rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="High-speed internal clock enable bit for some IP kernels"]
    #[inline] pub fn hsi16keron(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HSI16KERON != 0"]
    #[inline] pub fn test_hsi16keron(&self) -> bool {
        self.hsi16keron() != 0
    }

    #[doc="Sets the HSI16KERON field."]
    #[inline] pub fn set_hsi16keron<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="16 MHz high-speed internal clock enable"]
    #[inline] pub fn hsi16on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HSI16ON != 0"]
    #[inline] pub fn test_hsi16on(&self) -> bool {
        self.hsi16on() != 0
    }

    #[doc="Sets the HSI16ON field."]
    #[inline] pub fn set_hsi16on<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
        if self.pllon() != 0 { try!(write!(f, " pllon"))}
        if self.rtcpre() != 0 { try!(write!(f, " rtcpre=0x{:x}", self.rtcpre()))}
        if self.csslseon() != 0 { try!(write!(f, " csslseon"))}
        if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
        if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
        if self.hseon() != 0 { try!(write!(f, " hseon"))}
        if self.msirdy() != 0 { try!(write!(f, " msirdy"))}
        if self.msion() != 0 { try!(write!(f, " msion"))}
        if self.hsi16divf() != 0 { try!(write!(f, " hsi16divf"))}
        if self.hsi16diven() != 0 { try!(write!(f, " hsi16diven"))}
        if self.hsi16rdyf() != 0 { try!(write!(f, " hsi16rdyf"))}
        if self.hsi16keron() != 0 { try!(write!(f, " hsi16keron"))}
        if self.hsi16on() != 0 { try!(write!(f, " hsi16on"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Internal clock sources calibration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icscr(pub u32);
impl Icscr {
    #[doc="MSI clock trimming"]
    #[inline] pub fn msitrim(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MSITRIM != 0"]
    #[inline] pub fn test_msitrim(&self) -> bool {
        self.msitrim() != 0
    }

    #[doc="Sets the MSITRIM field."]
    #[inline] pub fn set_msitrim<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="MSI clock calibration"]
    #[inline] pub fn msical(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MSICAL != 0"]
    #[inline] pub fn test_msical(&self) -> bool {
        self.msical() != 0
    }

    #[doc="Sets the MSICAL field."]
    #[inline] pub fn set_msical<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MSI clock ranges"]
    #[inline] pub fn msirange(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if MSIRANGE != 0"]
    #[inline] pub fn test_msirange(&self) -> bool {
        self.msirange() != 0
    }

    #[doc="Sets the MSIRANGE field."]
    #[inline] pub fn set_msirange<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="High speed internal clock trimming"]
    #[inline] pub fn hsi16trim(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if HSI16TRIM != 0"]
    #[inline] pub fn test_hsi16trim(&self) -> bool {
        self.hsi16trim() != 0
    }

    #[doc="Sets the HSI16TRIM field."]
    #[inline] pub fn set_hsi16trim<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="nternal high speed clock calibration"]
    #[inline] pub fn hsi16cal(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if HSI16CAL != 0"]
    #[inline] pub fn test_hsi16cal(&self) -> bool {
        self.hsi16cal() != 0
    }

    #[doc="Sets the HSI16CAL field."]
    #[inline] pub fn set_hsi16cal<V: Into<bits::U8>>(mut self, value: V) -> Self {
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
        if self.msitrim() != 0 { try!(write!(f, " msitrim=0x{:x}", self.msitrim()))}
        if self.msical() != 0 { try!(write!(f, " msical=0x{:x}", self.msical()))}
        if self.msirange() != 0 { try!(write!(f, " msirange=0x{:x}", self.msirange()))}
        if self.hsi16trim() != 0 { try!(write!(f, " hsi16trim=0x{:x}", self.hsi16trim()))}
        if self.hsi16cal() != 0 { try!(write!(f, " hsi16cal=0x{:x}", self.hsi16cal()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock recovery RC register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crrcr(pub u32);
impl Crrcr {
    #[doc="48 MHz HSI clock calibration"]
    #[inline] pub fn hsi48cal(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if HSI48CAL != 0"]
    #[inline] pub fn test_hsi48cal(&self) -> bool {
        self.hsi48cal() != 0
    }

    #[doc="Sets the HSI48CAL field."]
    #[inline] pub fn set_hsi48cal<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="48MHz HSI clock ready flag"]
    #[inline] pub fn hsi48rdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HSI48RDY != 0"]
    #[inline] pub fn test_hsi48rdy(&self) -> bool {
        self.hsi48rdy() != 0
    }

    #[doc="Sets the HSI48RDY field."]
    #[inline] pub fn set_hsi48rdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="48MHz HSI clock enable bit"]
    #[inline] pub fn hsi48on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HSI48ON != 0"]
    #[inline] pub fn test_hsi48on(&self) -> bool {
        self.hsi48on() != 0
    }

    #[doc="Sets the HSI48ON field."]
    #[inline] pub fn set_hsi48on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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

    #[doc="Microcontroller clock output selection"]
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

    #[doc="PLL output division"]
    #[inline] pub fn plldiv(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if PLLDIV != 0"]
    #[inline] pub fn test_plldiv(&self) -> bool {
        self.plldiv() != 0
    }

    #[doc="Sets the PLLDIV field."]
    #[inline] pub fn set_plldiv<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="PLL multiplication factor"]
    #[inline] pub fn pllmul(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0xf) as u8) } // [21:18]
    }

    #[doc="Returns true if PLLMUL != 0"]
    #[inline] pub fn test_pllmul(&self) -> bool {
        self.pllmul() != 0
    }

    #[doc="Sets the PLLMUL field."]
    #[inline] pub fn set_pllmul<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="PLL entry clock source"]
    #[inline] pub fn pllsrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PLLSRC != 0"]
    #[inline] pub fn test_pllsrc(&self) -> bool {
        self.pllsrc() != 0
    }

    #[doc="Sets the PLLSRC field."]
    #[inline] pub fn set_pllsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Wake-up from stop clock selection"]
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

    #[doc="APB low-speed prescaler (APB1)"]
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
        if self.plldiv() != 0 { try!(write!(f, " plldiv=0x{:x}", self.plldiv()))}
        if self.pllmul() != 0 { try!(write!(f, " pllmul=0x{:x}", self.pllmul()))}
        if self.pllsrc() != 0 { try!(write!(f, " pllsrc"))}
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

#[doc="Clock interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cier(pub u32);
impl Cier {
    #[doc="LSE CSS interrupt flag"]
    #[inline] pub fn csslse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CSSLSE != 0"]
    #[inline] pub fn test_csslse(&self) -> bool {
        self.csslse() != 0
    }

    #[doc="Sets the CSSLSE field."]
    #[inline] pub fn set_csslse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="HSI48 ready interrupt flag"]
    #[inline] pub fn hsi48rdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HSI48RDYIE != 0"]
    #[inline] pub fn test_hsi48rdyie(&self) -> bool {
        self.hsi48rdyie() != 0
    }

    #[doc="Sets the HSI48RDYIE field."]
    #[inline] pub fn set_hsi48rdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MSI ready interrupt flag"]
    #[inline] pub fn msirdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MSIRDYIE != 0"]
    #[inline] pub fn test_msirdyie(&self) -> bool {
        self.msirdyie() != 0
    }

    #[doc="Sets the MSIRDYIE field."]
    #[inline] pub fn set_msirdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PLL ready interrupt flag"]
    #[inline] pub fn pllrdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PLLRDYIE != 0"]
    #[inline] pub fn test_pllrdyie(&self) -> bool {
        self.pllrdyie() != 0
    }

    #[doc="Sets the PLLRDYIE field."]
    #[inline] pub fn set_pllrdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSE ready interrupt flag"]
    #[inline] pub fn hserdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSERDYIE != 0"]
    #[inline] pub fn test_hserdyie(&self) -> bool {
        self.hserdyie() != 0
    }

    #[doc="Sets the HSERDYIE field."]
    #[inline] pub fn set_hserdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="HSI16 ready interrupt flag"]
    #[inline] pub fn hsi16rdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSI16RDYIE != 0"]
    #[inline] pub fn test_hsi16rdyie(&self) -> bool {
        self.hsi16rdyie() != 0
    }

    #[doc="Sets the HSI16RDYIE field."]
    #[inline] pub fn set_hsi16rdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE ready interrupt flag"]
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

    #[doc="LSI ready interrupt flag"]
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
        if self.csslse() != 0 { try!(write!(f, " csslse"))}
        if self.hsi48rdyie() != 0 { try!(write!(f, " hsi48rdyie"))}
        if self.msirdyie() != 0 { try!(write!(f, " msirdyie"))}
        if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
        if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
        if self.hsi16rdyie() != 0 { try!(write!(f, " hsi16rdyie"))}
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
    #[doc="Clock Security System Interrupt flag"]
    #[inline] pub fn csshsef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSSHSEF != 0"]
    #[inline] pub fn test_csshsef(&self) -> bool {
        self.csshsef() != 0
    }

    #[doc="Sets the CSSHSEF field."]
    #[inline] pub fn set_csshsef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LSE Clock Security System Interrupt flag"]
    #[inline] pub fn csslsef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CSSLSEF != 0"]
    #[inline] pub fn test_csslsef(&self) -> bool {
        self.csslsef() != 0
    }

    #[doc="Sets the CSSLSEF field."]
    #[inline] pub fn set_csslsef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="HSI48 ready interrupt flag"]
    #[inline] pub fn hsi48rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HSI48RDYF != 0"]
    #[inline] pub fn test_hsi48rdyf(&self) -> bool {
        self.hsi48rdyf() != 0
    }

    #[doc="Sets the HSI48RDYF field."]
    #[inline] pub fn set_hsi48rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MSI ready interrupt flag"]
    #[inline] pub fn msirdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MSIRDYF != 0"]
    #[inline] pub fn test_msirdyf(&self) -> bool {
        self.msirdyf() != 0
    }

    #[doc="Sets the MSIRDYF field."]
    #[inline] pub fn set_msirdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PLL ready interrupt flag"]
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

    #[doc="HSI16 ready interrupt flag"]
    #[inline] pub fn hsi16rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSI16RDYF != 0"]
    #[inline] pub fn test_hsi16rdyf(&self) -> bool {
        self.hsi16rdyf() != 0
    }

    #[doc="Sets the HSI16RDYF field."]
    #[inline] pub fn set_hsi16rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.csshsef() != 0 { try!(write!(f, " csshsef"))}
        if self.csslsef() != 0 { try!(write!(f, " csslsef"))}
        if self.hsi48rdyf() != 0 { try!(write!(f, " hsi48rdyf"))}
        if self.msirdyf() != 0 { try!(write!(f, " msirdyf"))}
        if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
        if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
        if self.hsi16rdyf() != 0 { try!(write!(f, " hsi16rdyf"))}
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
    #[doc="Clock Security System Interrupt clear"]
    #[inline] pub fn csshsec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSSHSEC != 0"]
    #[inline] pub fn test_csshsec(&self) -> bool {
        self.csshsec() != 0
    }

    #[doc="Sets the CSSHSEC field."]
    #[inline] pub fn set_csshsec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LSE Clock Security System Interrupt clear"]
    #[inline] pub fn csslsec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CSSLSEC != 0"]
    #[inline] pub fn test_csslsec(&self) -> bool {
        self.csslsec() != 0
    }

    #[doc="Sets the CSSLSEC field."]
    #[inline] pub fn set_csslsec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="HSI48 ready Interrupt clear"]
    #[inline] pub fn hsi48rdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HSI48RDYC != 0"]
    #[inline] pub fn test_hsi48rdyc(&self) -> bool {
        self.hsi48rdyc() != 0
    }

    #[doc="Sets the HSI48RDYC field."]
    #[inline] pub fn set_hsi48rdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MSI ready Interrupt clear"]
    #[inline] pub fn msirdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MSIRDYC != 0"]
    #[inline] pub fn test_msirdyc(&self) -> bool {
        self.msirdyc() != 0
    }

    #[doc="Sets the MSIRDYC field."]
    #[inline] pub fn set_msirdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PLL ready Interrupt clear"]
    #[inline] pub fn pllrdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PLLRDYC != 0"]
    #[inline] pub fn test_pllrdyc(&self) -> bool {
        self.pllrdyc() != 0
    }

    #[doc="Sets the PLLRDYC field."]
    #[inline] pub fn set_pllrdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HSE ready Interrupt clear"]
    #[inline] pub fn hserdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSERDYC != 0"]
    #[inline] pub fn test_hserdyc(&self) -> bool {
        self.hserdyc() != 0
    }

    #[doc="Sets the HSERDYC field."]
    #[inline] pub fn set_hserdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="HSI16 ready Interrupt clear"]
    #[inline] pub fn hsi16rdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSI16RDYC != 0"]
    #[inline] pub fn test_hsi16rdyc(&self) -> bool {
        self.hsi16rdyc() != 0
    }

    #[doc="Sets the HSI16RDYC field."]
    #[inline] pub fn set_hsi16rdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE ready Interrupt clear"]
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

    #[doc="LSI ready Interrupt clear"]
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
        if self.csshsec() != 0 { try!(write!(f, " csshsec"))}
        if self.csslsec() != 0 { try!(write!(f, " csslsec"))}
        if self.hsi48rdyc() != 0 { try!(write!(f, " hsi48rdyc"))}
        if self.msirdyc() != 0 { try!(write!(f, " msirdyc"))}
        if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
        if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
        if self.hsi16rdyc() != 0 { try!(write!(f, " hsi16rdyc"))}
        if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
        if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ioprstr(pub u32);
impl Ioprstr {
    #[doc="I/O port H reset"]
    #[inline] pub fn iophrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IOPHRST != 0"]
    #[inline] pub fn test_iophrst(&self) -> bool {
        self.iophrst() != 0
    }

    #[doc="Sets the IOPHRST field."]
    #[inline] pub fn set_iophrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I/O port C reset"]
    #[inline] pub fn iopcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IOPCRST != 0"]
    #[inline] pub fn test_iopcrst(&self) -> bool {
        self.iopcrst() != 0
    }

    #[doc="Sets the IOPCRST field."]
    #[inline] pub fn set_iopcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I/O port B reset"]
    #[inline] pub fn iopbrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IOPBRST != 0"]
    #[inline] pub fn test_iopbrst(&self) -> bool {
        self.iopbrst() != 0
    }

    #[doc="Sets the IOPBRST field."]
    #[inline] pub fn set_iopbrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="I/O port A reset"]
    #[inline] pub fn ioparst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IOPARST != 0"]
    #[inline] pub fn test_ioparst(&self) -> bool {
        self.ioparst() != 0
    }

    #[doc="Sets the IOPARST field."]
    #[inline] pub fn set_ioparst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ioprstr {
    #[inline]
    fn from(other: u32) -> Self {
         Ioprstr(other)
    }
}

impl ::core::fmt::Display for Ioprstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ioprstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iophrst() != 0 { try!(write!(f, " iophrst"))}
        if self.iopcrst() != 0 { try!(write!(f, " iopcrst"))}
        if self.iopbrst() != 0 { try!(write!(f, " iopbrst"))}
        if self.ioparst() != 0 { try!(write!(f, " ioparst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbrstr(pub u32);
impl Ahbrstr {
    #[doc="Crypto module reset"]
    #[inline] pub fn cryprst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CRYPRST != 0"]
    #[inline] pub fn test_cryprst(&self) -> bool {
        self.cryprst() != 0
    }

    #[doc="Sets the CRYPRST field."]
    #[inline] pub fn set_cryprst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Random Number Generator module reset"]
    #[inline] pub fn rngrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if RNGRST != 0"]
    #[inline] pub fn test_rngrst(&self) -> bool {
        self.rngrst() != 0
    }

    #[doc="Sets the RNGRST field."]
    #[inline] pub fn set_rngrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Touch Sensing reset"]
    #[inline] pub fn touchrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TOUCHRST != 0"]
    #[inline] pub fn test_touchrst(&self) -> bool {
        self.touchrst() != 0
    }

    #[doc="Sets the TOUCHRST field."]
    #[inline] pub fn set_touchrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Test integration module reset"]
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

    #[doc="Memory interface reset"]
    #[inline] pub fn mifrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MIFRST != 0"]
    #[inline] pub fn test_mifrst(&self) -> bool {
        self.mifrst() != 0
    }

    #[doc="Sets the MIFRST field."]
    #[inline] pub fn set_mifrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA reset"]
    #[inline] pub fn dmarst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMARST != 0"]
    #[inline] pub fn test_dmarst(&self) -> bool {
        self.dmarst() != 0
    }

    #[doc="Sets the DMARST field."]
    #[inline] pub fn set_dmarst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahbrstr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbrstr(other)
    }
}

impl ::core::fmt::Display for Ahbrstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahbrstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cryprst() != 0 { try!(write!(f, " cryprst"))}
        if self.rngrst() != 0 { try!(write!(f, " rngrst"))}
        if self.touchrst() != 0 { try!(write!(f, " touchrst"))}
        if self.crcrst() != 0 { try!(write!(f, " crcrst"))}
        if self.mifrst() != 0 { try!(write!(f, " mifrst"))}
        if self.dmarst() != 0 { try!(write!(f, " dmarst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
    #[doc="DBG reset"]
    #[inline] pub fn dbgrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DBGRST != 0"]
    #[inline] pub fn test_dbgrst(&self) -> bool {
        self.dbgrst() != 0
    }

    #[doc="Sets the DBGRST field."]
    #[inline] pub fn set_dbgrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
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

    #[doc="ADC interface reset"]
    #[inline] pub fn adcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADCRST != 0"]
    #[inline] pub fn test_adcrst(&self) -> bool {
        self.adcrst() != 0
    }

    #[doc="Sets the ADCRST field."]
    #[inline] pub fn set_adcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TIM22 timer reset"]
    #[inline] pub fn tim22rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM22RST != 0"]
    #[inline] pub fn test_tim22rst(&self) -> bool {
        self.tim22rst() != 0
    }

    #[doc="Sets the TIM22RST field."]
    #[inline] pub fn set_tim22rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM21 timer reset"]
    #[inline] pub fn tim21rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM21RST != 0"]
    #[inline] pub fn test_tim21rst(&self) -> bool {
        self.tim21rst() != 0
    }

    #[doc="Sets the TIM21RST field."]
    #[inline] pub fn set_tim21rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="System configuration controller reset"]
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
        if self.dbgrst() != 0 { try!(write!(f, " dbgrst"))}
        if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
        if self.adcrst() != 0 { try!(write!(f, " adcrst"))}
        if self.tim22rst() != 0 { try!(write!(f, " tim22rst"))}
        if self.tim21rst() != 0 { try!(write!(f, " tim21rst"))}
        if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
    #[doc="Low power timer reset"]
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

    #[doc="DAC interface reset"]
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

    #[doc="Clock recovery system reset"]
    #[inline] pub fn crsrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if CRSRST != 0"]
    #[inline] pub fn test_crsrst(&self) -> bool {
        self.crsrst() != 0
    }

    #[doc="Sets the CRSRST field."]
    #[inline] pub fn set_crsrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="USB reset"]
    #[inline] pub fn usbrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if USBRST != 0"]
    #[inline] pub fn test_usbrst(&self) -> bool {
        self.usbrst() != 0
    }

    #[doc="Sets the USBRST field."]
    #[inline] pub fn set_usbrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
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

    #[doc="LPUART1 reset"]
    #[inline] pub fn lpuart1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if LPUART1RST != 0"]
    #[inline] pub fn test_lpuart1rst(&self) -> bool {
        self.lpuart1rst() != 0
    }

    #[doc="Sets the LPUART1RST field."]
    #[inline] pub fn set_lpuart1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
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

    #[doc="Window watchdog reset"]
    #[inline] pub fn wwdrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDRST != 0"]
    #[inline] pub fn test_wwdrst(&self) -> bool {
        self.wwdrst() != 0
    }

    #[doc="Sets the WWDRST field."]
    #[inline] pub fn set_wwdrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Timer2 reset"]
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
        if self.lptim1rst() != 0 { try!(write!(f, " lptim1rst"))}
        if self.dacrst() != 0 { try!(write!(f, " dacrst"))}
        if self.pwrrst() != 0 { try!(write!(f, " pwrrst"))}
        if self.crsrst() != 0 { try!(write!(f, " crsrst"))}
        if self.usbrst() != 0 { try!(write!(f, " usbrst"))}
        if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
        if self.lpuart1rst() != 0 { try!(write!(f, " lpuart1rst"))}
        if self.usart2rst() != 0 { try!(write!(f, " usart2rst"))}
        if self.wwdrst() != 0 { try!(write!(f, " wwdrst"))}
        if self.tim2rst() != 0 { try!(write!(f, " tim2rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iopenr(pub u32);
impl Iopenr {
    #[doc="Gets the IOPAEN field."]
    #[inline] pub fn iopaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IOPAEN != 0"]
    #[inline] pub fn test_iopaen(&self) -> bool {
        self.iopaen() != 0
    }

    #[doc="Sets the IOPAEN field."]
    #[inline] pub fn set_iopaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Gets the IOPBEN field."]
    #[inline] pub fn iopben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IOPBEN != 0"]
    #[inline] pub fn test_iopben(&self) -> bool {
        self.iopben() != 0
    }

    #[doc="Sets the IOPBEN field."]
    #[inline] pub fn set_iopben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Gets the IOPCEN field."]
    #[inline] pub fn iopcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IOPCEN != 0"]
    #[inline] pub fn test_iopcen(&self) -> bool {
        self.iopcen() != 0
    }

    #[doc="Sets the IOPCEN field."]
    #[inline] pub fn set_iopcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Gets the IOPHEN field."]
    #[inline] pub fn iophen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IOPHEN != 0"]
    #[inline] pub fn test_iophen(&self) -> bool {
        self.iophen() != 0
    }

    #[doc="Sets the IOPHEN field."]
    #[inline] pub fn set_iophen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Iopenr {
    #[inline]
    fn from(other: u32) -> Self {
         Iopenr(other)
    }
}

impl ::core::fmt::Display for Iopenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iopenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iopaen() != 0 { try!(write!(f, " iopaen"))}
        if self.iopben() != 0 { try!(write!(f, " iopben"))}
        if self.iopcen() != 0 { try!(write!(f, " iopcen"))}
        if self.iophen() != 0 { try!(write!(f, " iophen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbenr(pub u32);
impl Ahbenr {
    #[doc="Crypto clock enable bit"]
    #[inline] pub fn crypen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CRYPEN != 0"]
    #[inline] pub fn test_crypen(&self) -> bool {
        self.crypen() != 0
    }

    #[doc="Sets the CRYPEN field."]
    #[inline] pub fn set_crypen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Random Number Generator clock enable bit"]
    #[inline] pub fn rngen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if RNGEN != 0"]
    #[inline] pub fn test_rngen(&self) -> bool {
        self.rngen() != 0
    }

    #[doc="Sets the RNGEN field."]
    #[inline] pub fn set_rngen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Touch Sensing clock enable bit"]
    #[inline] pub fn touchen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TOUCHEN != 0"]
    #[inline] pub fn test_touchen(&self) -> bool {
        self.touchen() != 0
    }

    #[doc="Sets the TOUCHEN field."]
    #[inline] pub fn set_touchen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CRC clock enable bit"]
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

    #[doc="NVM interface clock enable bit"]
    #[inline] pub fn mifen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MIFEN != 0"]
    #[inline] pub fn test_mifen(&self) -> bool {
        self.mifen() != 0
    }

    #[doc="Sets the MIFEN field."]
    #[inline] pub fn set_mifen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA clock enable bit"]
    #[inline] pub fn dmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahbenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbenr(other)
    }
}

impl ::core::fmt::Display for Ahbenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahbenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.crypen() != 0 { try!(write!(f, " crypen"))}
        if self.rngen() != 0 { try!(write!(f, " rngen"))}
        if self.touchen() != 0 { try!(write!(f, " touchen"))}
        if self.crcen() != 0 { try!(write!(f, " crcen"))}
        if self.mifen() != 0 { try!(write!(f, " mifen"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
    #[doc="DBG clock enable bit"]
    #[inline] pub fn dbgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DBGEN != 0"]
    #[inline] pub fn test_dbgen(&self) -> bool {
        self.dbgen() != 0
    }

    #[doc="Sets the DBGEN field."]
    #[inline] pub fn set_dbgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="SPI1 clock enable bit"]
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

    #[doc="ADC clock enable bit"]
    #[inline] pub fn adcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADCEN != 0"]
    #[inline] pub fn test_adcen(&self) -> bool {
        self.adcen() != 0
    }

    #[doc="Sets the ADCEN field."]
    #[inline] pub fn set_adcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="MiFaRe Firewall clock enable bit"]
    #[inline] pub fn mifien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MIFIEN != 0"]
    #[inline] pub fn test_mifien(&self) -> bool {
        self.mifien() != 0
    }

    #[doc="Sets the MIFIEN field."]
    #[inline] pub fn set_mifien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TIM22 timer clock enable bit"]
    #[inline] pub fn tim22en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM22EN != 0"]
    #[inline] pub fn test_tim22en(&self) -> bool {
        self.tim22en() != 0
    }

    #[doc="Sets the TIM22EN field."]
    #[inline] pub fn set_tim22en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM21 timer clock enable bit"]
    #[inline] pub fn tim21en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM21EN != 0"]
    #[inline] pub fn test_tim21en(&self) -> bool {
        self.tim21en() != 0
    }

    #[doc="Sets the TIM21EN field."]
    #[inline] pub fn set_tim21en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="System configuration controller clock enable bit"]
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
        if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
        if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
        if self.adcen() != 0 { try!(write!(f, " adcen"))}
        if self.mifien() != 0 { try!(write!(f, " mifien"))}
        if self.tim22en() != 0 { try!(write!(f, " tim22en"))}
        if self.tim21en() != 0 { try!(write!(f, " tim21en"))}
        if self.syscfgen() != 0 { try!(write!(f, " syscfgen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clock enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
    #[doc="Low power timer clock enable bit"]
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

    #[doc="DAC interface clock enable bit"]
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

    #[doc="Power interface clock enable bit"]
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

    #[doc="Clock recovery system clock enable bit"]
    #[inline] pub fn crsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if CRSEN != 0"]
    #[inline] pub fn test_crsen(&self) -> bool {
        self.crsen() != 0
    }

    #[doc="Sets the CRSEN field."]
    #[inline] pub fn set_crsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="USB clock enable bit"]
    #[inline] pub fn usben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if USBEN != 0"]
    #[inline] pub fn test_usben(&self) -> bool {
        self.usben() != 0
    }

    #[doc="Sets the USBEN field."]
    #[inline] pub fn set_usben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C1 clock enable bit"]
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

    #[doc="LPUART1 clock enable bit"]
    #[inline] pub fn lpuart1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if LPUART1EN != 0"]
    #[inline] pub fn test_lpuart1en(&self) -> bool {
        self.lpuart1en() != 0
    }

    #[doc="Sets the LPUART1EN field."]
    #[inline] pub fn set_lpuart1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="UART2 clock enable bit"]
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

    #[doc="Window watchdog clock enable bit"]
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

    #[doc="Timer2 clock enable bit"]
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
        if self.lptim1en() != 0 { try!(write!(f, " lptim1en"))}
        if self.dacen() != 0 { try!(write!(f, " dacen"))}
        if self.pwren() != 0 { try!(write!(f, " pwren"))}
        if self.crsen() != 0 { try!(write!(f, " crsen"))}
        if self.usben() != 0 { try!(write!(f, " usben"))}
        if self.i2c1en() != 0 { try!(write!(f, " i2c1en"))}
        if self.lpuart1en() != 0 { try!(write!(f, " lpuart1en"))}
        if self.usart2en() != 0 { try!(write!(f, " usart2en"))}
        if self.wwdgen() != 0 { try!(write!(f, " wwdgen"))}
        if self.tim2en() != 0 { try!(write!(f, " tim2en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO clock enable in sleep mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iopsmen(pub u32);
impl Iopsmen {
    #[doc="IOPHSMEN"]
    #[inline] pub fn iophsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IOPHSMEN != 0"]
    #[inline] pub fn test_iophsmen(&self) -> bool {
        self.iophsmen() != 0
    }

    #[doc="Sets the IOPHSMEN field."]
    #[inline] pub fn set_iophsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IOPDSMEN"]
    #[inline] pub fn iopdsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IOPDSMEN != 0"]
    #[inline] pub fn test_iopdsmen(&self) -> bool {
        self.iopdsmen() != 0
    }

    #[doc="Sets the IOPDSMEN field."]
    #[inline] pub fn set_iopdsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IOPCSMEN"]
    #[inline] pub fn iopcsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IOPCSMEN != 0"]
    #[inline] pub fn test_iopcsmen(&self) -> bool {
        self.iopcsmen() != 0
    }

    #[doc="Sets the IOPCSMEN field."]
    #[inline] pub fn set_iopcsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IOPBSMEN"]
    #[inline] pub fn iopbsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IOPBSMEN != 0"]
    #[inline] pub fn test_iopbsmen(&self) -> bool {
        self.iopbsmen() != 0
    }

    #[doc="Sets the IOPBSMEN field."]
    #[inline] pub fn set_iopbsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IOPASMEN"]
    #[inline] pub fn iopasmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IOPASMEN != 0"]
    #[inline] pub fn test_iopasmen(&self) -> bool {
        self.iopasmen() != 0
    }

    #[doc="Sets the IOPASMEN field."]
    #[inline] pub fn set_iopasmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iopsmen {
    #[inline]
    fn from(other: u32) -> Self {
         Iopsmen(other)
    }
}

impl ::core::fmt::Display for Iopsmen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iopsmen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iophsmen() != 0 { try!(write!(f, " iophsmen"))}
        if self.iopdsmen() != 0 { try!(write!(f, " iopdsmen"))}
        if self.iopcsmen() != 0 { try!(write!(f, " iopcsmen"))}
        if self.iopbsmen() != 0 { try!(write!(f, " iopbsmen"))}
        if self.iopasmen() != 0 { try!(write!(f, " iopasmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB peripheral clock enable in sleep mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbsmenr(pub u32);
impl Ahbsmenr {
    #[doc="Crypto clock enable during sleep mode bit"]
    #[inline] pub fn crypsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CRYPSMEN != 0"]
    #[inline] pub fn test_crypsmen(&self) -> bool {
        self.crypsmen() != 0
    }

    #[doc="Sets the CRYPSMEN field."]
    #[inline] pub fn set_crypsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Random Number Generator clock enable during sleep mode bit"]
    #[inline] pub fn rngsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if RNGSMEN != 0"]
    #[inline] pub fn test_rngsmen(&self) -> bool {
        self.rngsmen() != 0
    }

    #[doc="Sets the RNGSMEN field."]
    #[inline] pub fn set_rngsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Touch Sensing clock enable during sleep mode bit"]
    #[inline] pub fn touchsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TOUCHSMEN != 0"]
    #[inline] pub fn test_touchsmen(&self) -> bool {
        self.touchsmen() != 0
    }

    #[doc="Sets the TOUCHSMEN field."]
    #[inline] pub fn set_touchsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CRC clock enable during sleep mode bit"]
    #[inline] pub fn crcsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCSMEN != 0"]
    #[inline] pub fn test_crcsmen(&self) -> bool {
        self.crcsmen() != 0
    }

    #[doc="Sets the CRCSMEN field."]
    #[inline] pub fn set_crcsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SRAM interface clock enable during sleep mode bit"]
    #[inline] pub fn sramsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SRAMSMEN != 0"]
    #[inline] pub fn test_sramsmen(&self) -> bool {
        self.sramsmen() != 0
    }

    #[doc="Sets the SRAMSMEN field."]
    #[inline] pub fn set_sramsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="NVM interface clock enable during sleep mode bit"]
    #[inline] pub fn mifsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MIFSMEN != 0"]
    #[inline] pub fn test_mifsmen(&self) -> bool {
        self.mifsmen() != 0
    }

    #[doc="Sets the MIFSMEN field."]
    #[inline] pub fn set_mifsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA clock enable during sleep mode bit"]
    #[inline] pub fn dmasmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMASMEN != 0"]
    #[inline] pub fn test_dmasmen(&self) -> bool {
        self.dmasmen() != 0
    }

    #[doc="Sets the DMASMEN field."]
    #[inline] pub fn set_dmasmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ahbsmenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbsmenr(other)
    }
}

impl ::core::fmt::Display for Ahbsmenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahbsmenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.crypsmen() != 0 { try!(write!(f, " crypsmen"))}
        if self.rngsmen() != 0 { try!(write!(f, " rngsmen"))}
        if self.touchsmen() != 0 { try!(write!(f, " touchsmen"))}
        if self.crcsmen() != 0 { try!(write!(f, " crcsmen"))}
        if self.sramsmen() != 0 { try!(write!(f, " sramsmen"))}
        if self.mifsmen() != 0 { try!(write!(f, " mifsmen"))}
        if self.dmasmen() != 0 { try!(write!(f, " dmasmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral clock enable in sleep mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2smenr(pub u32);
impl Apb2smenr {
    #[doc="DBG clock enable during sleep mode bit"]
    #[inline] pub fn dbgsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DBGSMEN != 0"]
    #[inline] pub fn test_dbgsmen(&self) -> bool {
        self.dbgsmen() != 0
    }

    #[doc="Sets the DBGSMEN field."]
    #[inline] pub fn set_dbgsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="SPI1 clock enable during sleep mode bit"]
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

    #[doc="ADC clock enable during sleep mode bit"]
    #[inline] pub fn adcsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADCSMEN != 0"]
    #[inline] pub fn test_adcsmen(&self) -> bool {
        self.adcsmen() != 0
    }

    #[doc="Sets the ADCSMEN field."]
    #[inline] pub fn set_adcsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TIM22 timer clock enable during sleep mode bit"]
    #[inline] pub fn tim22smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM22SMEN != 0"]
    #[inline] pub fn test_tim22smen(&self) -> bool {
        self.tim22smen() != 0
    }

    #[doc="Sets the TIM22SMEN field."]
    #[inline] pub fn set_tim22smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM21 timer clock enable during sleep mode bit"]
    #[inline] pub fn tim21smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM21SMEN != 0"]
    #[inline] pub fn test_tim21smen(&self) -> bool {
        self.tim21smen() != 0
    }

    #[doc="Sets the TIM21SMEN field."]
    #[inline] pub fn set_tim21smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="System configuration controller clock enable during sleep mode bit"]
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
        if self.dbgsmen() != 0 { try!(write!(f, " dbgsmen"))}
        if self.spi1smen() != 0 { try!(write!(f, " spi1smen"))}
        if self.adcsmen() != 0 { try!(write!(f, " adcsmen"))}
        if self.tim22smen() != 0 { try!(write!(f, " tim22smen"))}
        if self.tim21smen() != 0 { try!(write!(f, " tim21smen"))}
        if self.syscfgsmen() != 0 { try!(write!(f, " syscfgsmen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clock enable in sleep mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1smenr(pub u32);
impl Apb1smenr {
    #[doc="Low power timer clock enable during sleep mode bit"]
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

    #[doc="DAC interface clock enable during sleep mode bit"]
    #[inline] pub fn dacsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACSMEN != 0"]
    #[inline] pub fn test_dacsmen(&self) -> bool {
        self.dacsmen() != 0
    }

    #[doc="Sets the DACSMEN field."]
    #[inline] pub fn set_dacsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Power interface clock enable during sleep mode bit"]
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

    #[doc="Clock recovery system clock enable during sleep mode bit"]
    #[inline] pub fn crssmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if CRSSMEN != 0"]
    #[inline] pub fn test_crssmen(&self) -> bool {
        self.crssmen() != 0
    }

    #[doc="Sets the CRSSMEN field."]
    #[inline] pub fn set_crssmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="USB clock enable during sleep mode bit"]
    #[inline] pub fn usbsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if USBSMEN != 0"]
    #[inline] pub fn test_usbsmen(&self) -> bool {
        self.usbsmen() != 0
    }

    #[doc="Sets the USBSMEN field."]
    #[inline] pub fn set_usbsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C1 clock enable during sleep mode bit"]
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

    #[doc="LPUART1 clock enable during sleep mode bit"]
    #[inline] pub fn lpuart1smen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if LPUART1SMEN != 0"]
    #[inline] pub fn test_lpuart1smen(&self) -> bool {
        self.lpuart1smen() != 0
    }

    #[doc="Sets the LPUART1SMEN field."]
    #[inline] pub fn set_lpuart1smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="UART2 clock enable during sleep mode bit"]
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

    #[doc="Window watchdog clock enable during sleep mode bit"]
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

    #[doc="Timer 6 clock enable during sleep mode bit"]
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

    #[doc="Timer2 clock enable during sleep mode bit"]
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

impl From<u32> for Apb1smenr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1smenr(other)
    }
}

impl ::core::fmt::Display for Apb1smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1smenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptim1smen() != 0 { try!(write!(f, " lptim1smen"))}
        if self.dacsmen() != 0 { try!(write!(f, " dacsmen"))}
        if self.pwrsmen() != 0 { try!(write!(f, " pwrsmen"))}
        if self.crssmen() != 0 { try!(write!(f, " crssmen"))}
        if self.usbsmen() != 0 { try!(write!(f, " usbsmen"))}
        if self.i2c1smen() != 0 { try!(write!(f, " i2c1smen"))}
        if self.lpuart1smen() != 0 { try!(write!(f, " lpuart1smen"))}
        if self.usart2smen() != 0 { try!(write!(f, " usart2smen"))}
        if self.wwdgsmen() != 0 { try!(write!(f, " wwdgsmen"))}
        if self.tim6smen() != 0 { try!(write!(f, " tim6smen"))}
        if self.tim2smen() != 0 { try!(write!(f, " tim2smen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccipr(pub u32);
impl Ccipr {
    #[doc="48 MHz HSI48 clock source selection bit"]
    #[inline] pub fn hsi48msel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if HSI48MSEL != 0"]
    #[inline] pub fn test_hsi48msel(&self) -> bool {
        self.hsi48msel() != 0
    }

    #[doc="Sets the HSI48MSEL field."]
    #[inline] pub fn set_hsi48msel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Low Power Timer clock source selection bits"]
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

    #[doc="I2C1 clock source selection bits"]
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

    #[doc="LPUART1 clock source selection bits"]
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

    #[doc="USART2 clock source selection bits"]
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
        if self.hsi48msel() != 0 { try!(write!(f, " hsi48msel"))}
        if self.lptim1sel() != 0 { try!(write!(f, " lptim1sel=0x{:x}", self.lptim1sel()))}
        if self.i2c1sel() != 0 { try!(write!(f, " i2c1sel=0x{:x}", self.i2c1sel()))}
        if self.lpuart1sel() != 0 { try!(write!(f, " lpuart1sel=0x{:x}", self.lpuart1sel()))}
        if self.usart2sel() != 0 { try!(write!(f, " usart2sel=0x{:x}", self.usart2sel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control and status register"]
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

    #[doc="Independent watchdog reset flag"]
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

    #[doc="OBLRSTF"]
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

    #[doc="RTC software reset bit"]
    #[inline] pub fn rtcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RTCRST != 0"]
    #[inline] pub fn test_rtcrst(&self) -> bool {
        self.rtcrst() != 0
    }

    #[doc="Sets the RTCRST field."]
    #[inline] pub fn set_rtcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="RTC clock enable bit"]
    #[inline] pub fn rtcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RTCEN != 0"]
    #[inline] pub fn test_rtcen(&self) -> bool {
        self.rtcen() != 0
    }

    #[doc="Sets the RTCEN field."]
    #[inline] pub fn set_rtcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="RTC and LCD clock source selection bits"]
    #[inline] pub fn rtcsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if RTCSEL != 0"]
    #[inline] pub fn test_rtcsel(&self) -> bool {
        self.rtcsel() != 0
    }

    #[doc="Sets the RTCSEL field."]
    #[inline] pub fn set_rtcsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CSS on LSE failure detection flag"]
    #[inline] pub fn csslsed(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CSSLSED != 0"]
    #[inline] pub fn test_csslsed(&self) -> bool {
        self.csslsed() != 0
    }

    #[doc="Sets the CSSLSED field."]
    #[inline] pub fn set_csslsed<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CSSLSEON"]
    #[inline] pub fn csslseon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CSSLSEON != 0"]
    #[inline] pub fn test_csslseon(&self) -> bool {
        self.csslseon() != 0
    }

    #[doc="Sets the CSSLSEON field."]
    #[inline] pub fn set_csslseon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="LSEDRV"]
    #[inline] pub fn lsedrv(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if LSEDRV != 0"]
    #[inline] pub fn test_lsedrv(&self) -> bool {
        self.lsedrv() != 0
    }

    #[doc="Sets the LSEDRV field."]
    #[inline] pub fn set_lsedrv<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="External low-speed oscillator bypass bit"]
    #[inline] pub fn lsebyp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if LSEBYP != 0"]
    #[inline] pub fn test_lsebyp(&self) -> bool {
        self.lsebyp() != 0
    }

    #[doc="Sets the LSEBYP field."]
    #[inline] pub fn set_lsebyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="External low-speed oscillator ready bit"]
    #[inline] pub fn lserdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSERDY != 0"]
    #[inline] pub fn test_lserdy(&self) -> bool {
        self.lserdy() != 0
    }

    #[doc="Sets the LSERDY field."]
    #[inline] pub fn set_lserdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="External low-speed oscillator enable bit"]
    #[inline] pub fn lseon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LSEON != 0"]
    #[inline] pub fn test_lseon(&self) -> bool {
        self.lseon() != 0
    }

    #[doc="Sets the LSEON field."]
    #[inline] pub fn set_lseon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Internal low-speed oscillator ready bit"]
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
        if self.lpwrstf() != 0 { try!(write!(f, " lpwrstf"))}
        if self.wwdgrstf() != 0 { try!(write!(f, " wwdgrstf"))}
        if self.iwdgrstf() != 0 { try!(write!(f, " iwdgrstf"))}
        if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
        if self.porrstf() != 0 { try!(write!(f, " porrstf"))}
        if self.pinrstf() != 0 { try!(write!(f, " pinrstf"))}
        if self.oblrstf() != 0 { try!(write!(f, " oblrstf"))}
        if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
        if self.rtcrst() != 0 { try!(write!(f, " rtcrst"))}
        if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
        if self.rtcsel() != 0 { try!(write!(f, " rtcsel=0x{:x}", self.rtcsel()))}
        if self.csslsed() != 0 { try!(write!(f, " csslsed"))}
        if self.csslseon() != 0 { try!(write!(f, " csslseon"))}
        if self.lsedrv() != 0 { try!(write!(f, " lsedrv=0x{:x}", self.lsedrv()))}
        if self.lsebyp() != 0 { try!(write!(f, " lsebyp"))}
        if self.lserdy() != 0 { try!(write!(f, " lserdy"))}
        if self.lseon() != 0 { try!(write!(f, " lseon"))}
        if self.lsirdy() != 0 { try!(write!(f, " lsirdy"))}
        if self.lsion() != 0 { try!(write!(f, " lsion"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub trait En {
    fn en(&self) -> u32;
    fn set_en(&self, value: u32);
}

impl Rcc {
    #[inline] pub fn en<P: En>(&self, p: &P) -> u32 {
        p.en()
    }
    #[inline] pub fn set_en<P: En>(&self, p: &P, value: u32) {
        p.set_en(value)
    }
}

pub trait Rst {
    fn rst(&self) -> u32;
    fn set_rst(&self, value: u32);
}

impl Rcc {
    #[inline] pub fn rst<P: Rst>(&self, p: &P) -> u32 {
        p.rst()
    }
    #[inline] pub fn set_rst<P: Rst>(&self, p: &P, value: u32) {
        p.set_rst(value)
    }
}

pub trait Sel {
    fn sel(&self) -> u32;
    fn set_sel(&self, value: u32);
}

impl Rcc {
    #[inline] pub fn sel<P: Sel>(&self, p: &P) -> u32 {
        p.sel()
    }
    #[inline] pub fn set_sel<P: Sel>(&self, p: &P, value: u32) {
        p.set_sel(value)
    }
}

impl Rst for super::gpio::Gpioh {
    #[inline] fn rst(&self) -> u32 { RCC.ioprstr().iophrst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_ioprstr(|r| r.set_iophrst(value)); }
}

impl Rst for super::gpio::Gpioc {
    #[inline] fn rst(&self) -> u32 { RCC.ioprstr().iopcrst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_ioprstr(|r| r.set_iopcrst(value)); }
}

impl Rst for super::gpio::Gpiob {
    #[inline] fn rst(&self) -> u32 { RCC.ioprstr().iopbrst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_ioprstr(|r| r.set_iopbrst(value)); }
}

impl Rst for super::gpio::Gpioa {
    #[inline] fn rst(&self) -> u32 { RCC.ioprstr().ioparst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_ioprstr(|r| r.set_ioparst(value)); }
}

impl Rst for super::crc::Crc {
    #[inline] fn rst(&self) -> u32 { RCC.ahbrstr().crcrst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_ahbrstr(|r| r.set_crcrst(value)); }
}

impl Rst for super::dma::Dma1 {
    #[inline] fn rst(&self) -> u32 { RCC.ahbrstr().dmarst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_ahbrstr(|r| r.set_dmarst(value)); }
}

impl Rst for super::spi::Spi1 {
    #[inline] fn rst(&self) -> u32 { RCC.apb2rstr().spi1rst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_spi1rst(value)); }
}

impl Rst for super::tim_gen::Tim22 {
    #[inline] fn rst(&self) -> u32 { RCC.apb2rstr().tim22rst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_tim22rst(value)); }
}

impl Rst for super::tim_gen::Tim21 {
    #[inline] fn rst(&self) -> u32 { RCC.apb2rstr().tim21rst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_tim21rst(value)); }
}

impl Rst for super::syscfg::Syscfg {
    #[inline] fn rst(&self) -> u32 { RCC.apb2rstr().syscfgrst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb2rstr(|r| r.set_syscfgrst(value)); }
}

impl Rst for super::lptim::Lptim {
    #[inline] fn rst(&self) -> u32 { RCC.apb1rstr().lptim1rst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_lptim1rst(value)); }
}

impl Rst for super::pwr::Pwr {
    #[inline] fn rst(&self) -> u32 { RCC.apb1rstr().pwrrst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_pwrrst(value)); }
}

impl Rst for super::i2c::I2c1 {
    #[inline] fn rst(&self) -> u32 { RCC.apb1rstr().i2c1rst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_i2c1rst(value)); }
}

impl Rst for super::usart::Usart2 {
    #[inline] fn rst(&self) -> u32 { RCC.apb1rstr().usart2rst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_usart2rst(value)); }
}

impl Rst for super::wwdg::Wwdg {
    #[inline] fn rst(&self) -> u32 { RCC.apb1rstr().wwdrst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_wwdrst(value)); }
}

impl Rst for super::tim_gen::Tim2 {
    #[inline] fn rst(&self) -> u32 { RCC.apb1rstr().tim2rst().into() }
    #[inline] fn set_rst(&self, value: u32) { RCC.with_apb1rstr(|r| r.set_tim2rst(value)); }
}

impl En for super::gpio::Gpioa {
    #[inline] fn en(&self) -> u32 { RCC.iopenr().iopaen().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_iopenr(|r| r.set_iopaen(value)); }
}

impl En for super::gpio::Gpiob {
    #[inline] fn en(&self) -> u32 { RCC.iopenr().iopben().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_iopenr(|r| r.set_iopben(value)); }
}

impl En for super::gpio::Gpioc {
    #[inline] fn en(&self) -> u32 { RCC.iopenr().iopcen().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_iopenr(|r| r.set_iopcen(value)); }
}

impl En for super::gpio::Gpioh {
    #[inline] fn en(&self) -> u32 { RCC.iopenr().iophen().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_iopenr(|r| r.set_iophen(value)); }
}

impl En for super::crc::Crc {
    #[inline] fn en(&self) -> u32 { RCC.ahbenr().crcen().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_crcen(value)); }
}

impl En for super::dma::Dma1 {
    #[inline] fn en(&self) -> u32 { RCC.ahbenr().dmaen().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_ahbenr(|r| r.set_dmaen(value)); }
}

impl En for super::spi::Spi1 {
    #[inline] fn en(&self) -> u32 { RCC.apb2enr().spi1en().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::adc::Adc1 {
    #[inline] fn en(&self) -> u32 { RCC.apb2enr().adcen().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_adcen(value)); }
}

impl En for super::tim_gen::Tim22 {
    #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim22en().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim22en(value)); }
}

impl En for super::tim_gen::Tim21 {
    #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim21en().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim21en(value)); }
}

impl En for super::syscfg::Syscfg {
    #[inline] fn en(&self) -> u32 { RCC.apb2enr().syscfgen().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

impl En for super::lptim::Lptim {
    #[inline] fn en(&self) -> u32 { RCC.apb1enr().lptim1en().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_lptim1en(value)); }
}

impl En for super::pwr::Pwr {
    #[inline] fn en(&self) -> u32 { RCC.apb1enr().pwren().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_pwren(value)); }
}

impl En for super::i2c::I2c1 {
    #[inline] fn en(&self) -> u32 { RCC.apb1enr().i2c1en().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c1en(value)); }
}

impl En for super::lpuart::Lpuart1 {
    #[inline] fn en(&self) -> u32 { RCC.apb1enr().lpuart1en().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_lpuart1en(value)); }
}

impl En for super::usart::Usart2 {
    #[inline] fn en(&self) -> u32 { RCC.apb1enr().usart2en().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_usart2en(value)); }
}

impl En for super::wwdg::Wwdg {
    #[inline] fn en(&self) -> u32 { RCC.apb1enr().wwdgen().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

impl En for super::tim_gen::Tim2 {
    #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim2en().into() }
    #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim2en(value)); }
}

impl Sel for super::lptim::Lptim {
    #[inline] fn sel(&self) -> u32 { RCC.ccipr().lptim1sel().into() }
    #[inline] fn set_sel(&self, value: u32) { RCC.with_ccipr(|r| r.set_lptim1sel(value)); }
}


