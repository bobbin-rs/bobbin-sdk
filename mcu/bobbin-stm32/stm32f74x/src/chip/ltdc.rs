//! LCD-TFT Controller
#[allow(unused_imports)] use bobbin_common::*;

periph!(LTDC, Ltdc, 0x40016800);

#[doc="LCD-TFT Controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ltdc(pub usize);
impl Ltdc {
    #[doc="Get the *mut pointer for the SSCR register."]
    #[inline] pub fn sscr_mut(&self) -> *mut Sscr { 
        (self.0 + 0x8) as *mut Sscr
    }

    #[doc="Get the *const pointer for the SSCR register."]
    #[inline] pub fn sscr_ptr(&self) -> *const Sscr { 
           self.sscr_mut()
    }

    #[doc="Read the SSCR register."]
    #[inline] pub fn sscr(&self) -> Sscr { 
        unsafe {
            read_volatile(self.sscr_ptr())
        }
    }

    #[doc="Write the SSCR register."]
    #[inline] pub fn set_sscr<F: FnOnce(Sscr) -> Sscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sscr_mut(), f(Sscr(0)));
        }
        self
    }

    #[doc="Modify the SSCR register."]
    #[inline] pub fn with_sscr<F: FnOnce(Sscr) -> Sscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sscr_mut(), f(self.sscr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BPCR register."]
    #[inline] pub fn bpcr_mut(&self) -> *mut Bpcr { 
        (self.0 + 0xc) as *mut Bpcr
    }

    #[doc="Get the *const pointer for the BPCR register."]
    #[inline] pub fn bpcr_ptr(&self) -> *const Bpcr { 
           self.bpcr_mut()
    }

    #[doc="Read the BPCR register."]
    #[inline] pub fn bpcr(&self) -> Bpcr { 
        unsafe {
            read_volatile(self.bpcr_ptr())
        }
    }

    #[doc="Write the BPCR register."]
    #[inline] pub fn set_bpcr<F: FnOnce(Bpcr) -> Bpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bpcr_mut(), f(Bpcr(0)));
        }
        self
    }

    #[doc="Modify the BPCR register."]
    #[inline] pub fn with_bpcr<F: FnOnce(Bpcr) -> Bpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bpcr_mut(), f(self.bpcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWCR register."]
    #[inline] pub fn awcr_mut(&self) -> *mut Awcr { 
        (self.0 + 0x10) as *mut Awcr
    }

    #[doc="Get the *const pointer for the AWCR register."]
    #[inline] pub fn awcr_ptr(&self) -> *const Awcr { 
           self.awcr_mut()
    }

    #[doc="Read the AWCR register."]
    #[inline] pub fn awcr(&self) -> Awcr { 
        unsafe {
            read_volatile(self.awcr_ptr())
        }
    }

    #[doc="Write the AWCR register."]
    #[inline] pub fn set_awcr<F: FnOnce(Awcr) -> Awcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awcr_mut(), f(Awcr(0)));
        }
        self
    }

    #[doc="Modify the AWCR register."]
    #[inline] pub fn with_awcr<F: FnOnce(Awcr) -> Awcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awcr_mut(), f(self.awcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TWCR register."]
    #[inline] pub fn twcr_mut(&self) -> *mut Twcr { 
        (self.0 + 0x14) as *mut Twcr
    }

    #[doc="Get the *const pointer for the TWCR register."]
    #[inline] pub fn twcr_ptr(&self) -> *const Twcr { 
           self.twcr_mut()
    }

    #[doc="Read the TWCR register."]
    #[inline] pub fn twcr(&self) -> Twcr { 
        unsafe {
            read_volatile(self.twcr_ptr())
        }
    }

    #[doc="Write the TWCR register."]
    #[inline] pub fn set_twcr<F: FnOnce(Twcr) -> Twcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.twcr_mut(), f(Twcr(0)));
        }
        self
    }

    #[doc="Modify the TWCR register."]
    #[inline] pub fn with_twcr<F: FnOnce(Twcr) -> Twcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.twcr_mut(), f(self.twcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the GCR register."]
    #[inline] pub fn gcr_mut(&self) -> *mut Gcr { 
        (self.0 + 0x18) as *mut Gcr
    }

    #[doc="Get the *const pointer for the GCR register."]
    #[inline] pub fn gcr_ptr(&self) -> *const Gcr { 
           self.gcr_mut()
    }

    #[doc="Read the GCR register."]
    #[inline] pub fn gcr(&self) -> Gcr { 
        unsafe {
            read_volatile(self.gcr_ptr())
        }
    }

    #[doc="Write the GCR register."]
    #[inline] pub fn set_gcr<F: FnOnce(Gcr) -> Gcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gcr_mut(), f(Gcr(0)));
        }
        self
    }

    #[doc="Modify the GCR register."]
    #[inline] pub fn with_gcr<F: FnOnce(Gcr) -> Gcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gcr_mut(), f(self.gcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SRCR register."]
    #[inline] pub fn srcr_mut(&self) -> *mut Srcr { 
        (self.0 + 0x24) as *mut Srcr
    }

    #[doc="Get the *const pointer for the SRCR register."]
    #[inline] pub fn srcr_ptr(&self) -> *const Srcr { 
           self.srcr_mut()
    }

    #[doc="Read the SRCR register."]
    #[inline] pub fn srcr(&self) -> Srcr { 
        unsafe {
            read_volatile(self.srcr_ptr())
        }
    }

    #[doc="Write the SRCR register."]
    #[inline] pub fn set_srcr<F: FnOnce(Srcr) -> Srcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.srcr_mut(), f(Srcr(0)));
        }
        self
    }

    #[doc="Modify the SRCR register."]
    #[inline] pub fn with_srcr<F: FnOnce(Srcr) -> Srcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.srcr_mut(), f(self.srcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BCCR register."]
    #[inline] pub fn bccr_mut(&self) -> *mut Bccr { 
        (self.0 + 0x2c) as *mut Bccr
    }

    #[doc="Get the *const pointer for the BCCR register."]
    #[inline] pub fn bccr_ptr(&self) -> *const Bccr { 
           self.bccr_mut()
    }

    #[doc="Read the BCCR register."]
    #[inline] pub fn bccr(&self) -> Bccr { 
        unsafe {
            read_volatile(self.bccr_ptr())
        }
    }

    #[doc="Write the BCCR register."]
    #[inline] pub fn set_bccr<F: FnOnce(Bccr) -> Bccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bccr_mut(), f(Bccr(0)));
        }
        self
    }

    #[doc="Modify the BCCR register."]
    #[inline] pub fn with_bccr<F: FnOnce(Bccr) -> Bccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bccr_mut(), f(self.bccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        (self.0 + 0x34) as *mut Ier
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
           self.ier_mut()
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        unsafe {
            read_volatile(self.ier_ptr())
        }
    }

    #[doc="Write the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ier_mut(), f(Ier(0)));
        }
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ier_mut(), f(self.ier()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        (self.0 + 0x38) as *mut Isr
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
           self.isr_mut()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        unsafe {
            read_volatile(self.isr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0x3c) as *mut Icr
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
           self.icr_mut()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(Icr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the LIPCR register."]
    #[inline] pub fn lipcr_mut(&self) -> *mut Lipcr { 
        (self.0 + 0x40) as *mut Lipcr
    }

    #[doc="Get the *const pointer for the LIPCR register."]
    #[inline] pub fn lipcr_ptr(&self) -> *const Lipcr { 
           self.lipcr_mut()
    }

    #[doc="Read the LIPCR register."]
    #[inline] pub fn lipcr(&self) -> Lipcr { 
        unsafe {
            read_volatile(self.lipcr_ptr())
        }
    }

    #[doc="Write the LIPCR register."]
    #[inline] pub fn set_lipcr<F: FnOnce(Lipcr) -> Lipcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lipcr_mut(), f(Lipcr(0)));
        }
        self
    }

    #[doc="Modify the LIPCR register."]
    #[inline] pub fn with_lipcr<F: FnOnce(Lipcr) -> Lipcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lipcr_mut(), f(self.lipcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CPSR register."]
    #[inline] pub fn cpsr_mut(&self) -> *mut Cpsr { 
        (self.0 + 0x44) as *mut Cpsr
    }

    #[doc="Get the *const pointer for the CPSR register."]
    #[inline] pub fn cpsr_ptr(&self) -> *const Cpsr { 
           self.cpsr_mut()
    }

    #[doc="Read the CPSR register."]
    #[inline] pub fn cpsr(&self) -> Cpsr { 
        unsafe {
            read_volatile(self.cpsr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the CDSR register."]
    #[inline] pub fn cdsr_mut(&self) -> *mut Cdsr { 
        (self.0 + 0x48) as *mut Cdsr
    }

    #[doc="Get the *const pointer for the CDSR register."]
    #[inline] pub fn cdsr_ptr(&self) -> *const Cdsr { 
           self.cdsr_mut()
    }

    #[doc="Read the CDSR register."]
    #[inline] pub fn cdsr(&self) -> Cdsr { 
        unsafe {
            read_volatile(self.cdsr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the L1CR register."]
    #[inline] pub fn l1cr_mut(&self) -> *mut L1cr { 
        (self.0 + 0x84) as *mut L1cr
    }

    #[doc="Get the *const pointer for the L1CR register."]
    #[inline] pub fn l1cr_ptr(&self) -> *const L1cr { 
           self.l1cr_mut()
    }

    #[doc="Read the L1CR register."]
    #[inline] pub fn l1cr(&self) -> L1cr { 
        unsafe {
            read_volatile(self.l1cr_ptr())
        }
    }

    #[doc="Write the L1CR register."]
    #[inline] pub fn set_l1cr<F: FnOnce(L1cr) -> L1cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cr_mut(), f(L1cr(0)));
        }
        self
    }

    #[doc="Modify the L1CR register."]
    #[inline] pub fn with_l1cr<F: FnOnce(L1cr) -> L1cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cr_mut(), f(self.l1cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1WHPCR register."]
    #[inline] pub fn l1whpcr_mut(&self) -> *mut L1whpcr { 
        (self.0 + 0x88) as *mut L1whpcr
    }

    #[doc="Get the *const pointer for the L1WHPCR register."]
    #[inline] pub fn l1whpcr_ptr(&self) -> *const L1whpcr { 
           self.l1whpcr_mut()
    }

    #[doc="Read the L1WHPCR register."]
    #[inline] pub fn l1whpcr(&self) -> L1whpcr { 
        unsafe {
            read_volatile(self.l1whpcr_ptr())
        }
    }

    #[doc="Write the L1WHPCR register."]
    #[inline] pub fn set_l1whpcr<F: FnOnce(L1whpcr) -> L1whpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1whpcr_mut(), f(L1whpcr(0)));
        }
        self
    }

    #[doc="Modify the L1WHPCR register."]
    #[inline] pub fn with_l1whpcr<F: FnOnce(L1whpcr) -> L1whpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1whpcr_mut(), f(self.l1whpcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1WVPCR register."]
    #[inline] pub fn l1wvpcr_mut(&self) -> *mut L1wvpcr { 
        (self.0 + 0x8c) as *mut L1wvpcr
    }

    #[doc="Get the *const pointer for the L1WVPCR register."]
    #[inline] pub fn l1wvpcr_ptr(&self) -> *const L1wvpcr { 
           self.l1wvpcr_mut()
    }

    #[doc="Read the L1WVPCR register."]
    #[inline] pub fn l1wvpcr(&self) -> L1wvpcr { 
        unsafe {
            read_volatile(self.l1wvpcr_ptr())
        }
    }

    #[doc="Write the L1WVPCR register."]
    #[inline] pub fn set_l1wvpcr<F: FnOnce(L1wvpcr) -> L1wvpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1wvpcr_mut(), f(L1wvpcr(0)));
        }
        self
    }

    #[doc="Modify the L1WVPCR register."]
    #[inline] pub fn with_l1wvpcr<F: FnOnce(L1wvpcr) -> L1wvpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1wvpcr_mut(), f(self.l1wvpcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1CKCR register."]
    #[inline] pub fn l1ckcr_mut(&self) -> *mut L1ckcr { 
        (self.0 + 0x90) as *mut L1ckcr
    }

    #[doc="Get the *const pointer for the L1CKCR register."]
    #[inline] pub fn l1ckcr_ptr(&self) -> *const L1ckcr { 
           self.l1ckcr_mut()
    }

    #[doc="Read the L1CKCR register."]
    #[inline] pub fn l1ckcr(&self) -> L1ckcr { 
        unsafe {
            read_volatile(self.l1ckcr_ptr())
        }
    }

    #[doc="Write the L1CKCR register."]
    #[inline] pub fn set_l1ckcr<F: FnOnce(L1ckcr) -> L1ckcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1ckcr_mut(), f(L1ckcr(0)));
        }
        self
    }

    #[doc="Modify the L1CKCR register."]
    #[inline] pub fn with_l1ckcr<F: FnOnce(L1ckcr) -> L1ckcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1ckcr_mut(), f(self.l1ckcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1PFCR register."]
    #[inline] pub fn l1pfcr_mut(&self) -> *mut L1pfcr { 
        (self.0 + 0x94) as *mut L1pfcr
    }

    #[doc="Get the *const pointer for the L1PFCR register."]
    #[inline] pub fn l1pfcr_ptr(&self) -> *const L1pfcr { 
           self.l1pfcr_mut()
    }

    #[doc="Read the L1PFCR register."]
    #[inline] pub fn l1pfcr(&self) -> L1pfcr { 
        unsafe {
            read_volatile(self.l1pfcr_ptr())
        }
    }

    #[doc="Write the L1PFCR register."]
    #[inline] pub fn set_l1pfcr<F: FnOnce(L1pfcr) -> L1pfcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1pfcr_mut(), f(L1pfcr(0)));
        }
        self
    }

    #[doc="Modify the L1PFCR register."]
    #[inline] pub fn with_l1pfcr<F: FnOnce(L1pfcr) -> L1pfcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1pfcr_mut(), f(self.l1pfcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1CACR register."]
    #[inline] pub fn l1cacr_mut(&self) -> *mut L1cacr { 
        (self.0 + 0x98) as *mut L1cacr
    }

    #[doc="Get the *const pointer for the L1CACR register."]
    #[inline] pub fn l1cacr_ptr(&self) -> *const L1cacr { 
           self.l1cacr_mut()
    }

    #[doc="Read the L1CACR register."]
    #[inline] pub fn l1cacr(&self) -> L1cacr { 
        unsafe {
            read_volatile(self.l1cacr_ptr())
        }
    }

    #[doc="Write the L1CACR register."]
    #[inline] pub fn set_l1cacr<F: FnOnce(L1cacr) -> L1cacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cacr_mut(), f(L1cacr(0)));
        }
        self
    }

    #[doc="Modify the L1CACR register."]
    #[inline] pub fn with_l1cacr<F: FnOnce(L1cacr) -> L1cacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cacr_mut(), f(self.l1cacr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1DCCR register."]
    #[inline] pub fn l1dccr_mut(&self) -> *mut L1dccr { 
        (self.0 + 0x9c) as *mut L1dccr
    }

    #[doc="Get the *const pointer for the L1DCCR register."]
    #[inline] pub fn l1dccr_ptr(&self) -> *const L1dccr { 
           self.l1dccr_mut()
    }

    #[doc="Read the L1DCCR register."]
    #[inline] pub fn l1dccr(&self) -> L1dccr { 
        unsafe {
            read_volatile(self.l1dccr_ptr())
        }
    }

    #[doc="Write the L1DCCR register."]
    #[inline] pub fn set_l1dccr<F: FnOnce(L1dccr) -> L1dccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1dccr_mut(), f(L1dccr(0)));
        }
        self
    }

    #[doc="Modify the L1DCCR register."]
    #[inline] pub fn with_l1dccr<F: FnOnce(L1dccr) -> L1dccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1dccr_mut(), f(self.l1dccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1BFCR register."]
    #[inline] pub fn l1bfcr_mut(&self) -> *mut L1bfcr { 
        (self.0 + 0xa0) as *mut L1bfcr
    }

    #[doc="Get the *const pointer for the L1BFCR register."]
    #[inline] pub fn l1bfcr_ptr(&self) -> *const L1bfcr { 
           self.l1bfcr_mut()
    }

    #[doc="Read the L1BFCR register."]
    #[inline] pub fn l1bfcr(&self) -> L1bfcr { 
        unsafe {
            read_volatile(self.l1bfcr_ptr())
        }
    }

    #[doc="Write the L1BFCR register."]
    #[inline] pub fn set_l1bfcr<F: FnOnce(L1bfcr) -> L1bfcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1bfcr_mut(), f(L1bfcr(0)));
        }
        self
    }

    #[doc="Modify the L1BFCR register."]
    #[inline] pub fn with_l1bfcr<F: FnOnce(L1bfcr) -> L1bfcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1bfcr_mut(), f(self.l1bfcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1CFBAR register."]
    #[inline] pub fn l1cfbar_mut(&self) -> *mut L1cfbar { 
        (self.0 + 0xac) as *mut L1cfbar
    }

    #[doc="Get the *const pointer for the L1CFBAR register."]
    #[inline] pub fn l1cfbar_ptr(&self) -> *const L1cfbar { 
           self.l1cfbar_mut()
    }

    #[doc="Read the L1CFBAR register."]
    #[inline] pub fn l1cfbar(&self) -> L1cfbar { 
        unsafe {
            read_volatile(self.l1cfbar_ptr())
        }
    }

    #[doc="Write the L1CFBAR register."]
    #[inline] pub fn set_l1cfbar<F: FnOnce(L1cfbar) -> L1cfbar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cfbar_mut(), f(L1cfbar(0)));
        }
        self
    }

    #[doc="Modify the L1CFBAR register."]
    #[inline] pub fn with_l1cfbar<F: FnOnce(L1cfbar) -> L1cfbar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cfbar_mut(), f(self.l1cfbar()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1CFBLR register."]
    #[inline] pub fn l1cfblr_mut(&self) -> *mut L1cfblr { 
        (self.0 + 0xb0) as *mut L1cfblr
    }

    #[doc="Get the *const pointer for the L1CFBLR register."]
    #[inline] pub fn l1cfblr_ptr(&self) -> *const L1cfblr { 
           self.l1cfblr_mut()
    }

    #[doc="Read the L1CFBLR register."]
    #[inline] pub fn l1cfblr(&self) -> L1cfblr { 
        unsafe {
            read_volatile(self.l1cfblr_ptr())
        }
    }

    #[doc="Write the L1CFBLR register."]
    #[inline] pub fn set_l1cfblr<F: FnOnce(L1cfblr) -> L1cfblr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cfblr_mut(), f(L1cfblr(0)));
        }
        self
    }

    #[doc="Modify the L1CFBLR register."]
    #[inline] pub fn with_l1cfblr<F: FnOnce(L1cfblr) -> L1cfblr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cfblr_mut(), f(self.l1cfblr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1CFBLNR register."]
    #[inline] pub fn l1cfblnr_mut(&self) -> *mut L1cfblnr { 
        (self.0 + 0xb4) as *mut L1cfblnr
    }

    #[doc="Get the *const pointer for the L1CFBLNR register."]
    #[inline] pub fn l1cfblnr_ptr(&self) -> *const L1cfblnr { 
           self.l1cfblnr_mut()
    }

    #[doc="Read the L1CFBLNR register."]
    #[inline] pub fn l1cfblnr(&self) -> L1cfblnr { 
        unsafe {
            read_volatile(self.l1cfblnr_ptr())
        }
    }

    #[doc="Write the L1CFBLNR register."]
    #[inline] pub fn set_l1cfblnr<F: FnOnce(L1cfblnr) -> L1cfblnr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cfblnr_mut(), f(L1cfblnr(0)));
        }
        self
    }

    #[doc="Modify the L1CFBLNR register."]
    #[inline] pub fn with_l1cfblnr<F: FnOnce(L1cfblnr) -> L1cfblnr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1cfblnr_mut(), f(self.l1cfblnr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L1CLUTWR register."]
    #[inline] pub fn l1clutwr_mut(&self) -> *mut L1clutwr { 
        (self.0 + 0xc4) as *mut L1clutwr
    }

    #[doc="Get the *const pointer for the L1CLUTWR register."]
    #[inline] pub fn l1clutwr_ptr(&self) -> *const L1clutwr { 
           self.l1clutwr_mut()
    }

    #[doc="Write the L1CLUTWR register."]
    #[inline] pub fn set_l1clutwr<F: FnOnce(L1clutwr) -> L1clutwr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l1clutwr_mut(), f(L1clutwr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2CR register."]
    #[inline] pub fn l2cr_mut(&self) -> *mut L2cr { 
        (self.0 + 0x104) as *mut L2cr
    }

    #[doc="Get the *const pointer for the L2CR register."]
    #[inline] pub fn l2cr_ptr(&self) -> *const L2cr { 
           self.l2cr_mut()
    }

    #[doc="Read the L2CR register."]
    #[inline] pub fn l2cr(&self) -> L2cr { 
        unsafe {
            read_volatile(self.l2cr_ptr())
        }
    }

    #[doc="Write the L2CR register."]
    #[inline] pub fn set_l2cr<F: FnOnce(L2cr) -> L2cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cr_mut(), f(L2cr(0)));
        }
        self
    }

    #[doc="Modify the L2CR register."]
    #[inline] pub fn with_l2cr<F: FnOnce(L2cr) -> L2cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cr_mut(), f(self.l2cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2WHPCR register."]
    #[inline] pub fn l2whpcr_mut(&self) -> *mut L2whpcr { 
        (self.0 + 0x108) as *mut L2whpcr
    }

    #[doc="Get the *const pointer for the L2WHPCR register."]
    #[inline] pub fn l2whpcr_ptr(&self) -> *const L2whpcr { 
           self.l2whpcr_mut()
    }

    #[doc="Read the L2WHPCR register."]
    #[inline] pub fn l2whpcr(&self) -> L2whpcr { 
        unsafe {
            read_volatile(self.l2whpcr_ptr())
        }
    }

    #[doc="Write the L2WHPCR register."]
    #[inline] pub fn set_l2whpcr<F: FnOnce(L2whpcr) -> L2whpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2whpcr_mut(), f(L2whpcr(0)));
        }
        self
    }

    #[doc="Modify the L2WHPCR register."]
    #[inline] pub fn with_l2whpcr<F: FnOnce(L2whpcr) -> L2whpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2whpcr_mut(), f(self.l2whpcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2WVPCR register."]
    #[inline] pub fn l2wvpcr_mut(&self) -> *mut L2wvpcr { 
        (self.0 + 0x10c) as *mut L2wvpcr
    }

    #[doc="Get the *const pointer for the L2WVPCR register."]
    #[inline] pub fn l2wvpcr_ptr(&self) -> *const L2wvpcr { 
           self.l2wvpcr_mut()
    }

    #[doc="Read the L2WVPCR register."]
    #[inline] pub fn l2wvpcr(&self) -> L2wvpcr { 
        unsafe {
            read_volatile(self.l2wvpcr_ptr())
        }
    }

    #[doc="Write the L2WVPCR register."]
    #[inline] pub fn set_l2wvpcr<F: FnOnce(L2wvpcr) -> L2wvpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2wvpcr_mut(), f(L2wvpcr(0)));
        }
        self
    }

    #[doc="Modify the L2WVPCR register."]
    #[inline] pub fn with_l2wvpcr<F: FnOnce(L2wvpcr) -> L2wvpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2wvpcr_mut(), f(self.l2wvpcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2CKCR register."]
    #[inline] pub fn l2ckcr_mut(&self) -> *mut L2ckcr { 
        (self.0 + 0x110) as *mut L2ckcr
    }

    #[doc="Get the *const pointer for the L2CKCR register."]
    #[inline] pub fn l2ckcr_ptr(&self) -> *const L2ckcr { 
           self.l2ckcr_mut()
    }

    #[doc="Read the L2CKCR register."]
    #[inline] pub fn l2ckcr(&self) -> L2ckcr { 
        unsafe {
            read_volatile(self.l2ckcr_ptr())
        }
    }

    #[doc="Write the L2CKCR register."]
    #[inline] pub fn set_l2ckcr<F: FnOnce(L2ckcr) -> L2ckcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2ckcr_mut(), f(L2ckcr(0)));
        }
        self
    }

    #[doc="Modify the L2CKCR register."]
    #[inline] pub fn with_l2ckcr<F: FnOnce(L2ckcr) -> L2ckcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2ckcr_mut(), f(self.l2ckcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2PFCR register."]
    #[inline] pub fn l2pfcr_mut(&self) -> *mut L2pfcr { 
        (self.0 + 0x114) as *mut L2pfcr
    }

    #[doc="Get the *const pointer for the L2PFCR register."]
    #[inline] pub fn l2pfcr_ptr(&self) -> *const L2pfcr { 
           self.l2pfcr_mut()
    }

    #[doc="Read the L2PFCR register."]
    #[inline] pub fn l2pfcr(&self) -> L2pfcr { 
        unsafe {
            read_volatile(self.l2pfcr_ptr())
        }
    }

    #[doc="Write the L2PFCR register."]
    #[inline] pub fn set_l2pfcr<F: FnOnce(L2pfcr) -> L2pfcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2pfcr_mut(), f(L2pfcr(0)));
        }
        self
    }

    #[doc="Modify the L2PFCR register."]
    #[inline] pub fn with_l2pfcr<F: FnOnce(L2pfcr) -> L2pfcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2pfcr_mut(), f(self.l2pfcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2CACR register."]
    #[inline] pub fn l2cacr_mut(&self) -> *mut L2cacr { 
        (self.0 + 0x118) as *mut L2cacr
    }

    #[doc="Get the *const pointer for the L2CACR register."]
    #[inline] pub fn l2cacr_ptr(&self) -> *const L2cacr { 
           self.l2cacr_mut()
    }

    #[doc="Read the L2CACR register."]
    #[inline] pub fn l2cacr(&self) -> L2cacr { 
        unsafe {
            read_volatile(self.l2cacr_ptr())
        }
    }

    #[doc="Write the L2CACR register."]
    #[inline] pub fn set_l2cacr<F: FnOnce(L2cacr) -> L2cacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cacr_mut(), f(L2cacr(0)));
        }
        self
    }

    #[doc="Modify the L2CACR register."]
    #[inline] pub fn with_l2cacr<F: FnOnce(L2cacr) -> L2cacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cacr_mut(), f(self.l2cacr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2DCCR register."]
    #[inline] pub fn l2dccr_mut(&self) -> *mut L2dccr { 
        (self.0 + 0x11c) as *mut L2dccr
    }

    #[doc="Get the *const pointer for the L2DCCR register."]
    #[inline] pub fn l2dccr_ptr(&self) -> *const L2dccr { 
           self.l2dccr_mut()
    }

    #[doc="Read the L2DCCR register."]
    #[inline] pub fn l2dccr(&self) -> L2dccr { 
        unsafe {
            read_volatile(self.l2dccr_ptr())
        }
    }

    #[doc="Write the L2DCCR register."]
    #[inline] pub fn set_l2dccr<F: FnOnce(L2dccr) -> L2dccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2dccr_mut(), f(L2dccr(0)));
        }
        self
    }

    #[doc="Modify the L2DCCR register."]
    #[inline] pub fn with_l2dccr<F: FnOnce(L2dccr) -> L2dccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2dccr_mut(), f(self.l2dccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2BFCR register."]
    #[inline] pub fn l2bfcr_mut(&self) -> *mut L2bfcr { 
        (self.0 + 0x120) as *mut L2bfcr
    }

    #[doc="Get the *const pointer for the L2BFCR register."]
    #[inline] pub fn l2bfcr_ptr(&self) -> *const L2bfcr { 
           self.l2bfcr_mut()
    }

    #[doc="Read the L2BFCR register."]
    #[inline] pub fn l2bfcr(&self) -> L2bfcr { 
        unsafe {
            read_volatile(self.l2bfcr_ptr())
        }
    }

    #[doc="Write the L2BFCR register."]
    #[inline] pub fn set_l2bfcr<F: FnOnce(L2bfcr) -> L2bfcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2bfcr_mut(), f(L2bfcr(0)));
        }
        self
    }

    #[doc="Modify the L2BFCR register."]
    #[inline] pub fn with_l2bfcr<F: FnOnce(L2bfcr) -> L2bfcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2bfcr_mut(), f(self.l2bfcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2CFBAR register."]
    #[inline] pub fn l2cfbar_mut(&self) -> *mut L2cfbar { 
        (self.0 + 0x12c) as *mut L2cfbar
    }

    #[doc="Get the *const pointer for the L2CFBAR register."]
    #[inline] pub fn l2cfbar_ptr(&self) -> *const L2cfbar { 
           self.l2cfbar_mut()
    }

    #[doc="Read the L2CFBAR register."]
    #[inline] pub fn l2cfbar(&self) -> L2cfbar { 
        unsafe {
            read_volatile(self.l2cfbar_ptr())
        }
    }

    #[doc="Write the L2CFBAR register."]
    #[inline] pub fn set_l2cfbar<F: FnOnce(L2cfbar) -> L2cfbar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cfbar_mut(), f(L2cfbar(0)));
        }
        self
    }

    #[doc="Modify the L2CFBAR register."]
    #[inline] pub fn with_l2cfbar<F: FnOnce(L2cfbar) -> L2cfbar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cfbar_mut(), f(self.l2cfbar()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2CFBLR register."]
    #[inline] pub fn l2cfblr_mut(&self) -> *mut L2cfblr { 
        (self.0 + 0x130) as *mut L2cfblr
    }

    #[doc="Get the *const pointer for the L2CFBLR register."]
    #[inline] pub fn l2cfblr_ptr(&self) -> *const L2cfblr { 
           self.l2cfblr_mut()
    }

    #[doc="Read the L2CFBLR register."]
    #[inline] pub fn l2cfblr(&self) -> L2cfblr { 
        unsafe {
            read_volatile(self.l2cfblr_ptr())
        }
    }

    #[doc="Write the L2CFBLR register."]
    #[inline] pub fn set_l2cfblr<F: FnOnce(L2cfblr) -> L2cfblr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cfblr_mut(), f(L2cfblr(0)));
        }
        self
    }

    #[doc="Modify the L2CFBLR register."]
    #[inline] pub fn with_l2cfblr<F: FnOnce(L2cfblr) -> L2cfblr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cfblr_mut(), f(self.l2cfblr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2CFBLNR register."]
    #[inline] pub fn l2cfblnr_mut(&self) -> *mut L2cfblnr { 
        (self.0 + 0x134) as *mut L2cfblnr
    }

    #[doc="Get the *const pointer for the L2CFBLNR register."]
    #[inline] pub fn l2cfblnr_ptr(&self) -> *const L2cfblnr { 
           self.l2cfblnr_mut()
    }

    #[doc="Read the L2CFBLNR register."]
    #[inline] pub fn l2cfblnr(&self) -> L2cfblnr { 
        unsafe {
            read_volatile(self.l2cfblnr_ptr())
        }
    }

    #[doc="Write the L2CFBLNR register."]
    #[inline] pub fn set_l2cfblnr<F: FnOnce(L2cfblnr) -> L2cfblnr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cfblnr_mut(), f(L2cfblnr(0)));
        }
        self
    }

    #[doc="Modify the L2CFBLNR register."]
    #[inline] pub fn with_l2cfblnr<F: FnOnce(L2cfblnr) -> L2cfblnr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2cfblnr_mut(), f(self.l2cfblnr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the L2CLUTWR register."]
    #[inline] pub fn l2clutwr_mut(&self) -> *mut L2clutwr { 
        (self.0 + 0x144) as *mut L2clutwr
    }

    #[doc="Get the *const pointer for the L2CLUTWR register."]
    #[inline] pub fn l2clutwr_ptr(&self) -> *const L2clutwr { 
           self.l2clutwr_mut()
    }

    #[doc="Write the L2CLUTWR register."]
    #[inline] pub fn set_l2clutwr<F: FnOnce(L2clutwr) -> L2clutwr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.l2clutwr_mut(), f(L2clutwr(0)));
        }
        self
    }

}

#[doc="Synchronization Size Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sscr(pub u32);
impl Sscr {
    #[doc="Horizontal Synchronization Width (in units of pixel clock period)"]
    #[inline] pub fn hsw(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if HSW != 0"]
    #[inline] pub fn test_hsw(&self) -> bool {
        self.hsw() != 0
    }

    #[doc="Sets the HSW field."]
    #[inline] pub fn set_hsw<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Vertical Synchronization Height (in units of horizontal scan line)"]
    #[inline] pub fn vsh(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if VSH != 0"]
    #[inline] pub fn test_vsh(&self) -> bool {
        self.vsh() != 0
    }

    #[doc="Sets the VSH field."]
    #[inline] pub fn set_vsh<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sscr {
    #[inline]
    fn from(other: u32) -> Self {
         Sscr(other)
    }
}

impl ::core::fmt::Display for Sscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hsw() != 0 { try!(write!(f, " hsw=0x{:x}", self.hsw()))}
        if self.vsh() != 0 { try!(write!(f, " vsh=0x{:x}", self.vsh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Back Porch Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bpcr(pub u32);
impl Bpcr {
    #[doc="Accumulated Horizontal back porch (in units of pixel clock period)"]
    #[inline] pub fn ahbp(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if AHBP != 0"]
    #[inline] pub fn test_ahbp(&self) -> bool {
        self.ahbp() != 0
    }

    #[doc="Sets the AHBP field."]
    #[inline] pub fn set_ahbp<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Accumulated Vertical back porch (in units of horizontal scan line)"]
    #[inline] pub fn avbp(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if AVBP != 0"]
    #[inline] pub fn test_avbp(&self) -> bool {
        self.avbp() != 0
    }

    #[doc="Sets the AVBP field."]
    #[inline] pub fn set_avbp<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bpcr {
    #[inline]
    fn from(other: u32) -> Self {
         Bpcr(other)
    }
}

impl ::core::fmt::Display for Bpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ahbp() != 0 { try!(write!(f, " ahbp=0x{:x}", self.ahbp()))}
        if self.avbp() != 0 { try!(write!(f, " avbp=0x{:x}", self.avbp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Active Width Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awcr(pub u32);
impl Awcr {
    #[doc="AAV"]
    #[inline] pub fn aav(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if AAV != 0"]
    #[inline] pub fn test_aav(&self) -> bool {
        self.aav() != 0
    }

    #[doc="Sets the AAV field."]
    #[inline] pub fn set_aav<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Accumulated Active Height (in units of horizontal scan line)"]
    #[inline] pub fn aah(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if AAH != 0"]
    #[inline] pub fn test_aah(&self) -> bool {
        self.aah() != 0
    }

    #[doc="Sets the AAH field."]
    #[inline] pub fn set_aah<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awcr {
    #[inline]
    fn from(other: u32) -> Self {
         Awcr(other)
    }
}

impl ::core::fmt::Display for Awcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aav() != 0 { try!(write!(f, " aav=0x{:x}", self.aav()))}
        if self.aah() != 0 { try!(write!(f, " aah=0x{:x}", self.aah()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Total Width Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Twcr(pub u32);
impl Twcr {
    #[doc="Total Width (in units of pixel clock period)"]
    #[inline] pub fn totalw(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if TOTALW != 0"]
    #[inline] pub fn test_totalw(&self) -> bool {
        self.totalw() != 0
    }

    #[doc="Sets the TOTALW field."]
    #[inline] pub fn set_totalw<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Total Height (in units of horizontal scan line)"]
    #[inline] pub fn totalh(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if TOTALH != 0"]
    #[inline] pub fn test_totalh(&self) -> bool {
        self.totalh() != 0
    }

    #[doc="Sets the TOTALH field."]
    #[inline] pub fn set_totalh<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Twcr {
    #[inline]
    fn from(other: u32) -> Self {
         Twcr(other)
    }
}

impl ::core::fmt::Display for Twcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Twcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.totalw() != 0 { try!(write!(f, " totalw=0x{:x}", self.totalw()))}
        if self.totalh() != 0 { try!(write!(f, " totalh=0x{:x}", self.totalh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Global Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc="Horizontal Synchronization Polarity"]
    #[inline] pub fn hspol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if HSPOL != 0"]
    #[inline] pub fn test_hspol(&self) -> bool {
        self.hspol() != 0
    }

    #[doc="Sets the HSPOL field."]
    #[inline] pub fn set_hspol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Vertical Synchronization Polarity"]
    #[inline] pub fn vspol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if VSPOL != 0"]
    #[inline] pub fn test_vspol(&self) -> bool {
        self.vspol() != 0
    }

    #[doc="Sets the VSPOL field."]
    #[inline] pub fn set_vspol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Data Enable Polarity"]
    #[inline] pub fn depol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DEPOL != 0"]
    #[inline] pub fn test_depol(&self) -> bool {
        self.depol() != 0
    }

    #[doc="Sets the DEPOL field."]
    #[inline] pub fn set_depol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Pixel Clock Polarity"]
    #[inline] pub fn pcpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PCPOL != 0"]
    #[inline] pub fn test_pcpol(&self) -> bool {
        self.pcpol() != 0
    }

    #[doc="Sets the PCPOL field."]
    #[inline] pub fn set_pcpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Dither Enable"]
    #[inline] pub fn den(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DEN != 0"]
    #[inline] pub fn test_den(&self) -> bool {
        self.den() != 0
    }

    #[doc="Sets the DEN field."]
    #[inline] pub fn set_den<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Dither Red Width"]
    #[inline] pub fn drw(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if DRW != 0"]
    #[inline] pub fn test_drw(&self) -> bool {
        self.drw() != 0
    }

    #[doc="Sets the DRW field."]
    #[inline] pub fn set_drw<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Dither Green Width"]
    #[inline] pub fn dgw(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if DGW != 0"]
    #[inline] pub fn test_dgw(&self) -> bool {
        self.dgw() != 0
    }

    #[doc="Sets the DGW field."]
    #[inline] pub fn set_dgw<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Dither Blue Width"]
    #[inline] pub fn dbw(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if DBW != 0"]
    #[inline] pub fn test_dbw(&self) -> bool {
        self.dbw() != 0
    }

    #[doc="Sets the DBW field."]
    #[inline] pub fn set_dbw<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="LCD-TFT controller enable bit"]
    #[inline] pub fn ltdcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LTDCEN != 0"]
    #[inline] pub fn test_ltdcen(&self) -> bool {
        self.ltdcen() != 0
    }

    #[doc="Sets the LTDCEN field."]
    #[inline] pub fn set_ltdcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
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
        if self.hspol() != 0 { try!(write!(f, " hspol"))}
        if self.vspol() != 0 { try!(write!(f, " vspol"))}
        if self.depol() != 0 { try!(write!(f, " depol"))}
        if self.pcpol() != 0 { try!(write!(f, " pcpol"))}
        if self.den() != 0 { try!(write!(f, " den"))}
        if self.drw() != 0 { try!(write!(f, " drw=0x{:x}", self.drw()))}
        if self.dgw() != 0 { try!(write!(f, " dgw=0x{:x}", self.dgw()))}
        if self.dbw() != 0 { try!(write!(f, " dbw=0x{:x}", self.dbw()))}
        if self.ltdcen() != 0 { try!(write!(f, " ltdcen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Shadow Reload Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srcr(pub u32);
impl Srcr {
    #[doc="Vertical Blanking Reload"]
    #[inline] pub fn vbr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if VBR != 0"]
    #[inline] pub fn test_vbr(&self) -> bool {
        self.vbr() != 0
    }

    #[doc="Sets the VBR field."]
    #[inline] pub fn set_vbr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Immediate Reload"]
    #[inline] pub fn imr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IMR != 0"]
    #[inline] pub fn test_imr(&self) -> bool {
        self.imr() != 0
    }

    #[doc="Sets the IMR field."]
    #[inline] pub fn set_imr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srcr {
    #[inline]
    fn from(other: u32) -> Self {
         Srcr(other)
    }
}

impl ::core::fmt::Display for Srcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vbr() != 0 { try!(write!(f, " vbr"))}
        if self.imr() != 0 { try!(write!(f, " imr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Background Color Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bccr(pub u32);
impl Bccr {
    #[doc="Background Color Red value"]
    #[inline] pub fn bc(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if BC != 0"]
    #[inline] pub fn test_bc(&self) -> bool {
        self.bc() != 0
    }

    #[doc="Sets the BC field."]
    #[inline] pub fn set_bc<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bccr {
    #[inline]
    fn from(other: u32) -> Self {
         Bccr(other)
    }
}

impl ::core::fmt::Display for Bccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bc() != 0 { try!(write!(f, " bc=0x{:x}", self.bc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Register Reload interrupt enable"]
    #[inline] pub fn rrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RRIE != 0"]
    #[inline] pub fn test_rrie(&self) -> bool {
        self.rrie() != 0
    }

    #[doc="Sets the RRIE field."]
    #[inline] pub fn set_rrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transfer Error Interrupt Enable"]
    #[inline] pub fn terrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TERRIE != 0"]
    #[inline] pub fn test_terrie(&self) -> bool {
        self.terrie() != 0
    }

    #[doc="Sets the TERRIE field."]
    #[inline] pub fn set_terrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FIFO Underrun Interrupt Enable"]
    #[inline] pub fn fuie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FUIE != 0"]
    #[inline] pub fn test_fuie(&self) -> bool {
        self.fuie() != 0
    }

    #[doc="Sets the FUIE field."]
    #[inline] pub fn set_fuie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Line Interrupt Enable"]
    #[inline] pub fn lie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LIE != 0"]
    #[inline] pub fn test_lie(&self) -> bool {
        self.lie() != 0
    }

    #[doc="Sets the LIE field."]
    #[inline] pub fn set_lie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rrie() != 0 { try!(write!(f, " rrie"))}
        if self.terrie() != 0 { try!(write!(f, " terrie"))}
        if self.fuie() != 0 { try!(write!(f, " fuie"))}
        if self.lie() != 0 { try!(write!(f, " lie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Register Reload Interrupt Flag"]
    #[inline] pub fn rrif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RRIF != 0"]
    #[inline] pub fn test_rrif(&self) -> bool {
        self.rrif() != 0
    }

    #[doc="Sets the RRIF field."]
    #[inline] pub fn set_rrif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transfer Error interrupt flag"]
    #[inline] pub fn terrif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TERRIF != 0"]
    #[inline] pub fn test_terrif(&self) -> bool {
        self.terrif() != 0
    }

    #[doc="Sets the TERRIF field."]
    #[inline] pub fn set_terrif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FIFO Underrun Interrupt flag"]
    #[inline] pub fn fuif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FUIF != 0"]
    #[inline] pub fn test_fuif(&self) -> bool {
        self.fuif() != 0
    }

    #[doc="Sets the FUIF field."]
    #[inline] pub fn set_fuif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Line Interrupt flag"]
    #[inline] pub fn lif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LIF != 0"]
    #[inline] pub fn test_lif(&self) -> bool {
        self.lif() != 0
    }

    #[doc="Sets the LIF field."]
    #[inline] pub fn set_lif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.rrif() != 0 { try!(write!(f, " rrif"))}
        if self.terrif() != 0 { try!(write!(f, " terrif"))}
        if self.fuif() != 0 { try!(write!(f, " fuif"))}
        if self.lif() != 0 { try!(write!(f, " lif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Clears Register Reload Interrupt Flag"]
    #[inline] pub fn crrif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CRRIF != 0"]
    #[inline] pub fn test_crrif(&self) -> bool {
        self.crrif() != 0
    }

    #[doc="Sets the CRRIF field."]
    #[inline] pub fn set_crrif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clears the Transfer Error Interrupt Flag"]
    #[inline] pub fn cterrif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTERRIF != 0"]
    #[inline] pub fn test_cterrif(&self) -> bool {
        self.cterrif() != 0
    }

    #[doc="Sets the CTERRIF field."]
    #[inline] pub fn set_cterrif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clears the FIFO Underrun Interrupt flag"]
    #[inline] pub fn cfuif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CFUIF != 0"]
    #[inline] pub fn test_cfuif(&self) -> bool {
        self.cfuif() != 0
    }

    #[doc="Sets the CFUIF field."]
    #[inline] pub fn set_cfuif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clears the Line Interrupt Flag"]
    #[inline] pub fn clif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CLIF != 0"]
    #[inline] pub fn test_clif(&self) -> bool {
        self.clif() != 0
    }

    #[doc="Sets the CLIF field."]
    #[inline] pub fn set_clif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
    }
}

impl ::core::fmt::Display for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.crrif() != 0 { try!(write!(f, " crrif"))}
        if self.cterrif() != 0 { try!(write!(f, " cterrif"))}
        if self.cfuif() != 0 { try!(write!(f, " cfuif"))}
        if self.clif() != 0 { try!(write!(f, " clif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Line Interrupt Position Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lipcr(pub u32);
impl Lipcr {
    #[doc="Line Interrupt Position"]
    #[inline] pub fn lipos(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if LIPOS != 0"]
    #[inline] pub fn test_lipos(&self) -> bool {
        self.lipos() != 0
    }

    #[doc="Sets the LIPOS field."]
    #[inline] pub fn set_lipos<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lipcr {
    #[inline]
    fn from(other: u32) -> Self {
         Lipcr(other)
    }
}

impl ::core::fmt::Display for Lipcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lipcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lipos() != 0 { try!(write!(f, " lipos=0x{:x}", self.lipos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Current Position Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpsr(pub u32);
impl Cpsr {
    #[doc="Current X Position"]
    #[inline] pub fn cxpos(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CXPOS != 0"]
    #[inline] pub fn test_cxpos(&self) -> bool {
        self.cxpos() != 0
    }

    #[doc="Sets the CXPOS field."]
    #[inline] pub fn set_cxpos<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Current Y Position"]
    #[inline] pub fn cypos(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CYPOS != 0"]
    #[inline] pub fn test_cypos(&self) -> bool {
        self.cypos() != 0
    }

    #[doc="Sets the CYPOS field."]
    #[inline] pub fn set_cypos<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cpsr {
    #[inline]
    fn from(other: u32) -> Self {
         Cpsr(other)
    }
}

impl ::core::fmt::Display for Cpsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cpsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cxpos() != 0 { try!(write!(f, " cxpos=0x{:x}", self.cxpos()))}
        if self.cypos() != 0 { try!(write!(f, " cypos=0x{:x}", self.cypos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Current Display Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cdsr(pub u32);
impl Cdsr {
    #[doc="Horizontal Synchronization display Status"]
    #[inline] pub fn hsyncs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSYNCS != 0"]
    #[inline] pub fn test_hsyncs(&self) -> bool {
        self.hsyncs() != 0
    }

    #[doc="Sets the HSYNCS field."]
    #[inline] pub fn set_hsyncs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Vertical Synchronization display Status"]
    #[inline] pub fn vsyncs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if VSYNCS != 0"]
    #[inline] pub fn test_vsyncs(&self) -> bool {
        self.vsyncs() != 0
    }

    #[doc="Sets the VSYNCS field."]
    #[inline] pub fn set_vsyncs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Horizontal Data Enable display Status"]
    #[inline] pub fn hdes(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HDES != 0"]
    #[inline] pub fn test_hdes(&self) -> bool {
        self.hdes() != 0
    }

    #[doc="Sets the HDES field."]
    #[inline] pub fn set_hdes<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Vertical Data Enable display Status"]
    #[inline] pub fn vdes(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VDES != 0"]
    #[inline] pub fn test_vdes(&self) -> bool {
        self.vdes() != 0
    }

    #[doc="Sets the VDES field."]
    #[inline] pub fn set_vdes<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cdsr {
    #[inline]
    fn from(other: u32) -> Self {
         Cdsr(other)
    }
}

impl ::core::fmt::Display for Cdsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cdsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hsyncs() != 0 { try!(write!(f, " hsyncs"))}
        if self.vsyncs() != 0 { try!(write!(f, " vsyncs"))}
        if self.hdes() != 0 { try!(write!(f, " hdes"))}
        if self.vdes() != 0 { try!(write!(f, " vdes"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1cr(pub u32);
impl L1cr {
    #[doc="Color Look-Up Table Enable"]
    #[inline] pub fn cluten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CLUTEN != 0"]
    #[inline] pub fn test_cluten(&self) -> bool {
        self.cluten() != 0
    }

    #[doc="Sets the CLUTEN field."]
    #[inline] pub fn set_cluten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Color Keying Enable"]
    #[inline] pub fn colken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COLKEN != 0"]
    #[inline] pub fn test_colken(&self) -> bool {
        self.colken() != 0
    }

    #[doc="Sets the COLKEN field."]
    #[inline] pub fn set_colken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Layer Enable"]
    #[inline] pub fn len(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LEN != 0"]
    #[inline] pub fn test_len(&self) -> bool {
        self.len() != 0
    }

    #[doc="Sets the LEN field."]
    #[inline] pub fn set_len<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1cr {
    #[inline]
    fn from(other: u32) -> Self {
         L1cr(other)
    }
}

impl ::core::fmt::Display for L1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cluten() != 0 { try!(write!(f, " cluten"))}
        if self.colken() != 0 { try!(write!(f, " colken"))}
        if self.len() != 0 { try!(write!(f, " len"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Window Horizontal Position Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1whpcr(pub u32);
impl L1whpcr {
    #[doc="Window Horizontal Stop Position"]
    #[inline] pub fn whsppos(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if WHSPPOS != 0"]
    #[inline] pub fn test_whsppos(&self) -> bool {
        self.whsppos() != 0
    }

    #[doc="Sets the WHSPPOS field."]
    #[inline] pub fn set_whsppos<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Window Horizontal Start Position"]
    #[inline] pub fn whstpos(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if WHSTPOS != 0"]
    #[inline] pub fn test_whstpos(&self) -> bool {
        self.whstpos() != 0
    }

    #[doc="Sets the WHSTPOS field."]
    #[inline] pub fn set_whstpos<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1whpcr {
    #[inline]
    fn from(other: u32) -> Self {
         L1whpcr(other)
    }
}

impl ::core::fmt::Display for L1whpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1whpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.whsppos() != 0 { try!(write!(f, " whsppos=0x{:x}", self.whsppos()))}
        if self.whstpos() != 0 { try!(write!(f, " whstpos=0x{:x}", self.whstpos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Window Vertical Position Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1wvpcr(pub u32);
impl L1wvpcr {
    #[doc="Window Vertical Stop Position"]
    #[inline] pub fn wvsppos(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7ff) as u16) } // [26:16]
    }

    #[doc="Returns true if WVSPPOS != 0"]
    #[inline] pub fn test_wvsppos(&self) -> bool {
        self.wvsppos() != 0
    }

    #[doc="Sets the WVSPPOS field."]
    #[inline] pub fn set_wvsppos<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Window Vertical Start Position"]
    #[inline] pub fn wvstpos(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if WVSTPOS != 0"]
    #[inline] pub fn test_wvstpos(&self) -> bool {
        self.wvstpos() != 0
    }

    #[doc="Sets the WVSTPOS field."]
    #[inline] pub fn set_wvstpos<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1wvpcr {
    #[inline]
    fn from(other: u32) -> Self {
         L1wvpcr(other)
    }
}

impl ::core::fmt::Display for L1wvpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1wvpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wvsppos() != 0 { try!(write!(f, " wvsppos=0x{:x}", self.wvsppos()))}
        if self.wvstpos() != 0 { try!(write!(f, " wvstpos=0x{:x}", self.wvstpos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Color Keying Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1ckcr(pub u32);
impl L1ckcr {
    #[doc="Color Key Red value"]
    #[inline] pub fn ckred(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CKRED != 0"]
    #[inline] pub fn test_ckred(&self) -> bool {
        self.ckred() != 0
    }

    #[doc="Sets the CKRED field."]
    #[inline] pub fn set_ckred<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Color Key Green value"]
    #[inline] pub fn ckgreen(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CKGREEN != 0"]
    #[inline] pub fn test_ckgreen(&self) -> bool {
        self.ckgreen() != 0
    }

    #[doc="Sets the CKGREEN field."]
    #[inline] pub fn set_ckgreen<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Color Key Blue value"]
    #[inline] pub fn ckblue(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CKBLUE != 0"]
    #[inline] pub fn test_ckblue(&self) -> bool {
        self.ckblue() != 0
    }

    #[doc="Sets the CKBLUE field."]
    #[inline] pub fn set_ckblue<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1ckcr {
    #[inline]
    fn from(other: u32) -> Self {
         L1ckcr(other)
    }
}

impl ::core::fmt::Display for L1ckcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1ckcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ckred() != 0 { try!(write!(f, " ckred=0x{:x}", self.ckred()))}
        if self.ckgreen() != 0 { try!(write!(f, " ckgreen=0x{:x}", self.ckgreen()))}
        if self.ckblue() != 0 { try!(write!(f, " ckblue=0x{:x}", self.ckblue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Pixel Format Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1pfcr(pub u32);
impl L1pfcr {
    #[doc="Pixel Format"]
    #[inline] pub fn pf(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PF != 0"]
    #[inline] pub fn test_pf(&self) -> bool {
        self.pf() != 0
    }

    #[doc="Sets the PF field."]
    #[inline] pub fn set_pf<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1pfcr {
    #[inline]
    fn from(other: u32) -> Self {
         L1pfcr(other)
    }
}

impl ::core::fmt::Display for L1pfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1pfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pf() != 0 { try!(write!(f, " pf=0x{:x}", self.pf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Constant Alpha Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1cacr(pub u32);
impl L1cacr {
    #[doc="Constant Alpha"]
    #[inline] pub fn consta(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CONSTA != 0"]
    #[inline] pub fn test_consta(&self) -> bool {
        self.consta() != 0
    }

    #[doc="Sets the CONSTA field."]
    #[inline] pub fn set_consta<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1cacr {
    #[inline]
    fn from(other: u32) -> Self {
         L1cacr(other)
    }
}

impl ::core::fmt::Display for L1cacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1cacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.consta() != 0 { try!(write!(f, " consta=0x{:x}", self.consta()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Default Color Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1dccr(pub u32);
impl L1dccr {
    #[doc="Default Color Alpha"]
    #[inline] pub fn dcalpha(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DCALPHA != 0"]
    #[inline] pub fn test_dcalpha(&self) -> bool {
        self.dcalpha() != 0
    }

    #[doc="Sets the DCALPHA field."]
    #[inline] pub fn set_dcalpha<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Default Color Red"]
    #[inline] pub fn dcred(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DCRED != 0"]
    #[inline] pub fn test_dcred(&self) -> bool {
        self.dcred() != 0
    }

    #[doc="Sets the DCRED field."]
    #[inline] pub fn set_dcred<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Default Color Green"]
    #[inline] pub fn dcgreen(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DCGREEN != 0"]
    #[inline] pub fn test_dcgreen(&self) -> bool {
        self.dcgreen() != 0
    }

    #[doc="Sets the DCGREEN field."]
    #[inline] pub fn set_dcgreen<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Default Color Blue"]
    #[inline] pub fn dcblue(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DCBLUE != 0"]
    #[inline] pub fn test_dcblue(&self) -> bool {
        self.dcblue() != 0
    }

    #[doc="Sets the DCBLUE field."]
    #[inline] pub fn set_dcblue<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1dccr {
    #[inline]
    fn from(other: u32) -> Self {
         L1dccr(other)
    }
}

impl ::core::fmt::Display for L1dccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1dccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcalpha() != 0 { try!(write!(f, " dcalpha=0x{:x}", self.dcalpha()))}
        if self.dcred() != 0 { try!(write!(f, " dcred=0x{:x}", self.dcred()))}
        if self.dcgreen() != 0 { try!(write!(f, " dcgreen=0x{:x}", self.dcgreen()))}
        if self.dcblue() != 0 { try!(write!(f, " dcblue=0x{:x}", self.dcblue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Blending Factors Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1bfcr(pub u32);
impl L1bfcr {
    #[doc="Blending Factor 1"]
    #[inline] pub fn bf1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if BF1 != 0"]
    #[inline] pub fn test_bf1(&self) -> bool {
        self.bf1() != 0
    }

    #[doc="Sets the BF1 field."]
    #[inline] pub fn set_bf1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Blending Factor 2"]
    #[inline] pub fn bf2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if BF2 != 0"]
    #[inline] pub fn test_bf2(&self) -> bool {
        self.bf2() != 0
    }

    #[doc="Sets the BF2 field."]
    #[inline] pub fn set_bf2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1bfcr {
    #[inline]
    fn from(other: u32) -> Self {
         L1bfcr(other)
    }
}

impl ::core::fmt::Display for L1bfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1bfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bf1() != 0 { try!(write!(f, " bf1=0x{:x}", self.bf1()))}
        if self.bf2() != 0 { try!(write!(f, " bf2=0x{:x}", self.bf2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Color Frame Buffer Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1cfbar(pub u32);
impl L1cfbar {
    #[doc="Color Frame Buffer Start Address"]
    #[inline] pub fn cfbadd(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CFBADD != 0"]
    #[inline] pub fn test_cfbadd(&self) -> bool {
        self.cfbadd() != 0
    }

    #[doc="Sets the CFBADD field."]
    #[inline] pub fn set_cfbadd<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1cfbar {
    #[inline]
    fn from(other: u32) -> Self {
         L1cfbar(other)
    }
}

impl ::core::fmt::Display for L1cfbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1cfbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Color Frame Buffer Length Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1cfblr(pub u32);
impl L1cfblr {
    #[doc="Color Frame Buffer Pitch in bytes"]
    #[inline] pub fn cfbp(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1fff) as u16) } // [28:16]
    }

    #[doc="Returns true if CFBP != 0"]
    #[inline] pub fn test_cfbp(&self) -> bool {
        self.cfbp() != 0
    }

    #[doc="Sets the CFBP field."]
    #[inline] pub fn set_cfbp<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Color Frame Buffer Line Length"]
    #[inline] pub fn cfbll(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if CFBLL != 0"]
    #[inline] pub fn test_cfbll(&self) -> bool {
        self.cfbll() != 0
    }

    #[doc="Sets the CFBLL field."]
    #[inline] pub fn set_cfbll<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1cfblr {
    #[inline]
    fn from(other: u32) -> Self {
         L1cfblr(other)
    }
}

impl ::core::fmt::Display for L1cfblr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1cfblr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfbp() != 0 { try!(write!(f, " cfbp=0x{:x}", self.cfbp()))}
        if self.cfbll() != 0 { try!(write!(f, " cfbll=0x{:x}", self.cfbll()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx ColorFrame Buffer Line Number Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1cfblnr(pub u32);
impl L1cfblnr {
    #[doc="Frame Buffer Line Number"]
    #[inline] pub fn cfblnbr(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if CFBLNBR != 0"]
    #[inline] pub fn test_cfblnbr(&self) -> bool {
        self.cfblnbr() != 0
    }

    #[doc="Sets the CFBLNBR field."]
    #[inline] pub fn set_cfblnbr<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1cfblnr {
    #[inline]
    fn from(other: u32) -> Self {
         L1cfblnr(other)
    }
}

impl ::core::fmt::Display for L1cfblnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1cfblnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfblnbr() != 0 { try!(write!(f, " cfblnbr=0x{:x}", self.cfblnbr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx CLUT Write Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L1clutwr(pub u32);
impl L1clutwr {
    #[doc="CLUT Address"]
    #[inline] pub fn clutadd(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if CLUTADD != 0"]
    #[inline] pub fn test_clutadd(&self) -> bool {
        self.clutadd() != 0
    }

    #[doc="Sets the CLUTADD field."]
    #[inline] pub fn set_clutadd<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Red value"]
    #[inline] pub fn red(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if RED != 0"]
    #[inline] pub fn test_red(&self) -> bool {
        self.red() != 0
    }

    #[doc="Sets the RED field."]
    #[inline] pub fn set_red<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Green value"]
    #[inline] pub fn green(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GREEN != 0"]
    #[inline] pub fn test_green(&self) -> bool {
        self.green() != 0
    }

    #[doc="Sets the GREEN field."]
    #[inline] pub fn set_green<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Blue value"]
    #[inline] pub fn blue(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BLUE != 0"]
    #[inline] pub fn test_blue(&self) -> bool {
        self.blue() != 0
    }

    #[doc="Sets the BLUE field."]
    #[inline] pub fn set_blue<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L1clutwr {
    #[inline]
    fn from(other: u32) -> Self {
         L1clutwr(other)
    }
}

impl ::core::fmt::Display for L1clutwr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L1clutwr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clutadd() != 0 { try!(write!(f, " clutadd=0x{:x}", self.clutadd()))}
        if self.red() != 0 { try!(write!(f, " red=0x{:x}", self.red()))}
        if self.green() != 0 { try!(write!(f, " green=0x{:x}", self.green()))}
        if self.blue() != 0 { try!(write!(f, " blue=0x{:x}", self.blue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2cr(pub u32);
impl L2cr {
    #[doc="Color Look-Up Table Enable"]
    #[inline] pub fn cluten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CLUTEN != 0"]
    #[inline] pub fn test_cluten(&self) -> bool {
        self.cluten() != 0
    }

    #[doc="Sets the CLUTEN field."]
    #[inline] pub fn set_cluten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Color Keying Enable"]
    #[inline] pub fn colken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COLKEN != 0"]
    #[inline] pub fn test_colken(&self) -> bool {
        self.colken() != 0
    }

    #[doc="Sets the COLKEN field."]
    #[inline] pub fn set_colken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Layer Enable"]
    #[inline] pub fn len(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LEN != 0"]
    #[inline] pub fn test_len(&self) -> bool {
        self.len() != 0
    }

    #[doc="Sets the LEN field."]
    #[inline] pub fn set_len<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2cr {
    #[inline]
    fn from(other: u32) -> Self {
         L2cr(other)
    }
}

impl ::core::fmt::Display for L2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cluten() != 0 { try!(write!(f, " cluten"))}
        if self.colken() != 0 { try!(write!(f, " colken"))}
        if self.len() != 0 { try!(write!(f, " len"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Window Horizontal Position Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2whpcr(pub u32);
impl L2whpcr {
    #[doc="Window Horizontal Stop Position"]
    #[inline] pub fn whsppos(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if WHSPPOS != 0"]
    #[inline] pub fn test_whsppos(&self) -> bool {
        self.whsppos() != 0
    }

    #[doc="Sets the WHSPPOS field."]
    #[inline] pub fn set_whsppos<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Window Horizontal Start Position"]
    #[inline] pub fn whstpos(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if WHSTPOS != 0"]
    #[inline] pub fn test_whstpos(&self) -> bool {
        self.whstpos() != 0
    }

    #[doc="Sets the WHSTPOS field."]
    #[inline] pub fn set_whstpos<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2whpcr {
    #[inline]
    fn from(other: u32) -> Self {
         L2whpcr(other)
    }
}

impl ::core::fmt::Display for L2whpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2whpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.whsppos() != 0 { try!(write!(f, " whsppos=0x{:x}", self.whsppos()))}
        if self.whstpos() != 0 { try!(write!(f, " whstpos=0x{:x}", self.whstpos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Window Vertical Position Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2wvpcr(pub u32);
impl L2wvpcr {
    #[doc="Window Vertical Stop Position"]
    #[inline] pub fn wvsppos(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7ff) as u16) } // [26:16]
    }

    #[doc="Returns true if WVSPPOS != 0"]
    #[inline] pub fn test_wvsppos(&self) -> bool {
        self.wvsppos() != 0
    }

    #[doc="Sets the WVSPPOS field."]
    #[inline] pub fn set_wvsppos<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Window Vertical Start Position"]
    #[inline] pub fn wvstpos(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if WVSTPOS != 0"]
    #[inline] pub fn test_wvstpos(&self) -> bool {
        self.wvstpos() != 0
    }

    #[doc="Sets the WVSTPOS field."]
    #[inline] pub fn set_wvstpos<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2wvpcr {
    #[inline]
    fn from(other: u32) -> Self {
         L2wvpcr(other)
    }
}

impl ::core::fmt::Display for L2wvpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2wvpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wvsppos() != 0 { try!(write!(f, " wvsppos=0x{:x}", self.wvsppos()))}
        if self.wvstpos() != 0 { try!(write!(f, " wvstpos=0x{:x}", self.wvstpos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Color Keying Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2ckcr(pub u32);
impl L2ckcr {
    #[doc="Color Key Red value"]
    #[inline] pub fn ckred(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1ff) as u16) } // [23:15]
    }

    #[doc="Returns true if CKRED != 0"]
    #[inline] pub fn test_ckred(&self) -> bool {
        self.ckred() != 0
    }

    #[doc="Sets the CKRED field."]
    #[inline] pub fn set_ckred<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Color Key Green value"]
    #[inline] pub fn ckgreen(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if CKGREEN != 0"]
    #[inline] pub fn test_ckgreen(&self) -> bool {
        self.ckgreen() != 0
    }

    #[doc="Sets the CKGREEN field."]
    #[inline] pub fn set_ckgreen<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Color Key Blue value"]
    #[inline] pub fn ckblue(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CKBLUE != 0"]
    #[inline] pub fn test_ckblue(&self) -> bool {
        self.ckblue() != 0
    }

    #[doc="Sets the CKBLUE field."]
    #[inline] pub fn set_ckblue<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2ckcr {
    #[inline]
    fn from(other: u32) -> Self {
         L2ckcr(other)
    }
}

impl ::core::fmt::Display for L2ckcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2ckcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ckred() != 0 { try!(write!(f, " ckred=0x{:x}", self.ckred()))}
        if self.ckgreen() != 0 { try!(write!(f, " ckgreen=0x{:x}", self.ckgreen()))}
        if self.ckblue() != 0 { try!(write!(f, " ckblue=0x{:x}", self.ckblue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Pixel Format Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2pfcr(pub u32);
impl L2pfcr {
    #[doc="Pixel Format"]
    #[inline] pub fn pf(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PF != 0"]
    #[inline] pub fn test_pf(&self) -> bool {
        self.pf() != 0
    }

    #[doc="Sets the PF field."]
    #[inline] pub fn set_pf<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2pfcr {
    #[inline]
    fn from(other: u32) -> Self {
         L2pfcr(other)
    }
}

impl ::core::fmt::Display for L2pfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2pfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pf() != 0 { try!(write!(f, " pf=0x{:x}", self.pf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Constant Alpha Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2cacr(pub u32);
impl L2cacr {
    #[doc="Constant Alpha"]
    #[inline] pub fn consta(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CONSTA != 0"]
    #[inline] pub fn test_consta(&self) -> bool {
        self.consta() != 0
    }

    #[doc="Sets the CONSTA field."]
    #[inline] pub fn set_consta<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2cacr {
    #[inline]
    fn from(other: u32) -> Self {
         L2cacr(other)
    }
}

impl ::core::fmt::Display for L2cacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2cacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.consta() != 0 { try!(write!(f, " consta=0x{:x}", self.consta()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Default Color Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2dccr(pub u32);
impl L2dccr {
    #[doc="Default Color Alpha"]
    #[inline] pub fn dcalpha(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DCALPHA != 0"]
    #[inline] pub fn test_dcalpha(&self) -> bool {
        self.dcalpha() != 0
    }

    #[doc="Sets the DCALPHA field."]
    #[inline] pub fn set_dcalpha<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Default Color Red"]
    #[inline] pub fn dcred(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DCRED != 0"]
    #[inline] pub fn test_dcred(&self) -> bool {
        self.dcred() != 0
    }

    #[doc="Sets the DCRED field."]
    #[inline] pub fn set_dcred<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Default Color Green"]
    #[inline] pub fn dcgreen(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DCGREEN != 0"]
    #[inline] pub fn test_dcgreen(&self) -> bool {
        self.dcgreen() != 0
    }

    #[doc="Sets the DCGREEN field."]
    #[inline] pub fn set_dcgreen<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Default Color Blue"]
    #[inline] pub fn dcblue(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DCBLUE != 0"]
    #[inline] pub fn test_dcblue(&self) -> bool {
        self.dcblue() != 0
    }

    #[doc="Sets the DCBLUE field."]
    #[inline] pub fn set_dcblue<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2dccr {
    #[inline]
    fn from(other: u32) -> Self {
         L2dccr(other)
    }
}

impl ::core::fmt::Display for L2dccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2dccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcalpha() != 0 { try!(write!(f, " dcalpha=0x{:x}", self.dcalpha()))}
        if self.dcred() != 0 { try!(write!(f, " dcred=0x{:x}", self.dcred()))}
        if self.dcgreen() != 0 { try!(write!(f, " dcgreen=0x{:x}", self.dcgreen()))}
        if self.dcblue() != 0 { try!(write!(f, " dcblue=0x{:x}", self.dcblue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Blending Factors Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2bfcr(pub u32);
impl L2bfcr {
    #[doc="Blending Factor 1"]
    #[inline] pub fn bf1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if BF1 != 0"]
    #[inline] pub fn test_bf1(&self) -> bool {
        self.bf1() != 0
    }

    #[doc="Sets the BF1 field."]
    #[inline] pub fn set_bf1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Blending Factor 2"]
    #[inline] pub fn bf2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if BF2 != 0"]
    #[inline] pub fn test_bf2(&self) -> bool {
        self.bf2() != 0
    }

    #[doc="Sets the BF2 field."]
    #[inline] pub fn set_bf2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2bfcr {
    #[inline]
    fn from(other: u32) -> Self {
         L2bfcr(other)
    }
}

impl ::core::fmt::Display for L2bfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2bfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bf1() != 0 { try!(write!(f, " bf1=0x{:x}", self.bf1()))}
        if self.bf2() != 0 { try!(write!(f, " bf2=0x{:x}", self.bf2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Color Frame Buffer Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2cfbar(pub u32);
impl L2cfbar {
    #[doc="Color Frame Buffer Start Address"]
    #[inline] pub fn cfbadd(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CFBADD != 0"]
    #[inline] pub fn test_cfbadd(&self) -> bool {
        self.cfbadd() != 0
    }

    #[doc="Sets the CFBADD field."]
    #[inline] pub fn set_cfbadd<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2cfbar {
    #[inline]
    fn from(other: u32) -> Self {
         L2cfbar(other)
    }
}

impl ::core::fmt::Display for L2cfbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2cfbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx Color Frame Buffer Length Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2cfblr(pub u32);
impl L2cfblr {
    #[doc="Color Frame Buffer Pitch in bytes"]
    #[inline] pub fn cfbp(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1fff) as u16) } // [28:16]
    }

    #[doc="Returns true if CFBP != 0"]
    #[inline] pub fn test_cfbp(&self) -> bool {
        self.cfbp() != 0
    }

    #[doc="Sets the CFBP field."]
    #[inline] pub fn set_cfbp<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Color Frame Buffer Line Length"]
    #[inline] pub fn cfbll(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if CFBLL != 0"]
    #[inline] pub fn test_cfbll(&self) -> bool {
        self.cfbll() != 0
    }

    #[doc="Sets the CFBLL field."]
    #[inline] pub fn set_cfbll<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2cfblr {
    #[inline]
    fn from(other: u32) -> Self {
         L2cfblr(other)
    }
}

impl ::core::fmt::Display for L2cfblr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2cfblr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfbp() != 0 { try!(write!(f, " cfbp=0x{:x}", self.cfbp()))}
        if self.cfbll() != 0 { try!(write!(f, " cfbll=0x{:x}", self.cfbll()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx ColorFrame Buffer Line Number Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2cfblnr(pub u32);
impl L2cfblnr {
    #[doc="Frame Buffer Line Number"]
    #[inline] pub fn cfblnbr(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if CFBLNBR != 0"]
    #[inline] pub fn test_cfblnbr(&self) -> bool {
        self.cfblnbr() != 0
    }

    #[doc="Sets the CFBLNBR field."]
    #[inline] pub fn set_cfblnbr<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2cfblnr {
    #[inline]
    fn from(other: u32) -> Self {
         L2cfblnr(other)
    }
}

impl ::core::fmt::Display for L2cfblnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2cfblnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfblnbr() != 0 { try!(write!(f, " cfblnbr=0x{:x}", self.cfblnbr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Layerx CLUT Write Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct L2clutwr(pub u32);
impl L2clutwr {
    #[doc="CLUT Address"]
    #[inline] pub fn clutadd(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if CLUTADD != 0"]
    #[inline] pub fn test_clutadd(&self) -> bool {
        self.clutadd() != 0
    }

    #[doc="Sets the CLUTADD field."]
    #[inline] pub fn set_clutadd<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Red value"]
    #[inline] pub fn red(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if RED != 0"]
    #[inline] pub fn test_red(&self) -> bool {
        self.red() != 0
    }

    #[doc="Sets the RED field."]
    #[inline] pub fn set_red<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Green value"]
    #[inline] pub fn green(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GREEN != 0"]
    #[inline] pub fn test_green(&self) -> bool {
        self.green() != 0
    }

    #[doc="Sets the GREEN field."]
    #[inline] pub fn set_green<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Blue value"]
    #[inline] pub fn blue(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BLUE != 0"]
    #[inline] pub fn test_blue(&self) -> bool {
        self.blue() != 0
    }

    #[doc="Sets the BLUE field."]
    #[inline] pub fn set_blue<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for L2clutwr {
    #[inline]
    fn from(other: u32) -> Self {
         L2clutwr(other)
    }
}

impl ::core::fmt::Display for L2clutwr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for L2clutwr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clutadd() != 0 { try!(write!(f, " clutadd=0x{:x}", self.clutadd()))}
        if self.red() != 0 { try!(write!(f, " red=0x{:x}", self.red()))}
        if self.green() != 0 { try!(write!(f, " green=0x{:x}", self.green()))}
        if self.blue() != 0 { try!(write!(f, " blue=0x{:x}", self.blue()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


