#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Power control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PwrPeriph(pub usize);
impl PwrPeriph {
    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        (self.0 + 0x0) as *mut Cr1
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
           self.cr1_mut()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        unsafe {
            read_volatile(self.cr1_ptr())
        }
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr1_mut(), f(Cr1(0)));
        }
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr1_mut(), f(self.cr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        (self.0 + 0x4) as *mut Cr2
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
           self.cr2_mut()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        unsafe {
            read_volatile(self.cr2_ptr())
        }
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr2_mut(), f(Cr2(0)));
        }
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr2_mut(), f(self.cr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR3 register."]
    #[inline] pub fn cr3_mut(&self) -> *mut Cr3 { 
        (self.0 + 0x8) as *mut Cr3
    }

    #[doc="Get the *const pointer for the CR3 register."]
    #[inline] pub fn cr3_ptr(&self) -> *const Cr3 { 
           self.cr3_mut()
    }

    #[doc="Read the CR3 register."]
    #[inline] pub fn cr3(&self) -> Cr3 { 
        unsafe {
            read_volatile(self.cr3_ptr())
        }
    }

    #[doc="Write the CR3 register."]
    #[inline] pub fn set_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr3_mut(), f(Cr3(0)));
        }
        self
    }

    #[doc="Modify the CR3 register."]
    #[inline] pub fn with_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr3_mut(), f(self.cr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR4 register."]
    #[inline] pub fn cr4_mut(&self) -> *mut Cr4 { 
        (self.0 + 0xc) as *mut Cr4
    }

    #[doc="Get the *const pointer for the CR4 register."]
    #[inline] pub fn cr4_ptr(&self) -> *const Cr4 { 
           self.cr4_mut()
    }

    #[doc="Read the CR4 register."]
    #[inline] pub fn cr4(&self) -> Cr4 { 
        unsafe {
            read_volatile(self.cr4_ptr())
        }
    }

    #[doc="Write the CR4 register."]
    #[inline] pub fn set_cr4<F: FnOnce(Cr4) -> Cr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr4_mut(), f(Cr4(0)));
        }
        self
    }

    #[doc="Modify the CR4 register."]
    #[inline] pub fn with_cr4<F: FnOnce(Cr4) -> Cr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr4_mut(), f(self.cr4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SR1 register."]
    #[inline] pub fn sr1_mut(&self) -> *mut Sr1 { 
        (self.0 + 0x10) as *mut Sr1
    }

    #[doc="Get the *const pointer for the SR1 register."]
    #[inline] pub fn sr1_ptr(&self) -> *const Sr1 { 
           self.sr1_mut()
    }

    #[doc="Read the SR1 register."]
    #[inline] pub fn sr1(&self) -> Sr1 { 
        unsafe {
            read_volatile(self.sr1_ptr())
        }
    }

    #[doc="Get the *mut pointer for the SR2 register."]
    #[inline] pub fn sr2_mut(&self) -> *mut Sr2 { 
        (self.0 + 0x14) as *mut Sr2
    }

    #[doc="Get the *const pointer for the SR2 register."]
    #[inline] pub fn sr2_ptr(&self) -> *const Sr2 { 
           self.sr2_mut()
    }

    #[doc="Read the SR2 register."]
    #[inline] pub fn sr2(&self) -> Sr2 { 
        unsafe {
            read_volatile(self.sr2_ptr())
        }
    }

    #[doc="Get the *mut pointer for the SCR register."]
    #[inline] pub fn scr_mut(&self) -> *mut Scr { 
        (self.0 + 0x18) as *mut Scr
    }

    #[doc="Get the *const pointer for the SCR register."]
    #[inline] pub fn scr_ptr(&self) -> *const Scr { 
           self.scr_mut()
    }

    #[doc="Write the SCR register."]
    #[inline] pub fn set_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scr_mut(), f(Scr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the PUCRA register."]
    #[inline] pub fn pucra_mut(&self) -> *mut Pucra { 
        (self.0 + 0x20) as *mut Pucra
    }

    #[doc="Get the *const pointer for the PUCRA register."]
    #[inline] pub fn pucra_ptr(&self) -> *const Pucra { 
           self.pucra_mut()
    }

    #[doc="Read the PUCRA register."]
    #[inline] pub fn pucra(&self) -> Pucra { 
        unsafe {
            read_volatile(self.pucra_ptr())
        }
    }

    #[doc="Write the PUCRA register."]
    #[inline] pub fn set_pucra<F: FnOnce(Pucra) -> Pucra>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucra_mut(), f(Pucra(0)));
        }
        self
    }

    #[doc="Modify the PUCRA register."]
    #[inline] pub fn with_pucra<F: FnOnce(Pucra) -> Pucra>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucra_mut(), f(self.pucra()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDCRA register."]
    #[inline] pub fn pdcra_mut(&self) -> *mut Pdcra { 
        (self.0 + 0x24) as *mut Pdcra
    }

    #[doc="Get the *const pointer for the PDCRA register."]
    #[inline] pub fn pdcra_ptr(&self) -> *const Pdcra { 
           self.pdcra_mut()
    }

    #[doc="Read the PDCRA register."]
    #[inline] pub fn pdcra(&self) -> Pdcra { 
        unsafe {
            read_volatile(self.pdcra_ptr())
        }
    }

    #[doc="Write the PDCRA register."]
    #[inline] pub fn set_pdcra<F: FnOnce(Pdcra) -> Pdcra>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcra_mut(), f(Pdcra(0)));
        }
        self
    }

    #[doc="Modify the PDCRA register."]
    #[inline] pub fn with_pdcra<F: FnOnce(Pdcra) -> Pdcra>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcra_mut(), f(self.pdcra()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PUCRB register."]
    #[inline] pub fn pucrb_mut(&self) -> *mut Pucrb { 
        (self.0 + 0x28) as *mut Pucrb
    }

    #[doc="Get the *const pointer for the PUCRB register."]
    #[inline] pub fn pucrb_ptr(&self) -> *const Pucrb { 
           self.pucrb_mut()
    }

    #[doc="Read the PUCRB register."]
    #[inline] pub fn pucrb(&self) -> Pucrb { 
        unsafe {
            read_volatile(self.pucrb_ptr())
        }
    }

    #[doc="Write the PUCRB register."]
    #[inline] pub fn set_pucrb<F: FnOnce(Pucrb) -> Pucrb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrb_mut(), f(Pucrb(0)));
        }
        self
    }

    #[doc="Modify the PUCRB register."]
    #[inline] pub fn with_pucrb<F: FnOnce(Pucrb) -> Pucrb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrb_mut(), f(self.pucrb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDCRB register."]
    #[inline] pub fn pdcrb_mut(&self) -> *mut Pdcrb { 
        (self.0 + 0x2c) as *mut Pdcrb
    }

    #[doc="Get the *const pointer for the PDCRB register."]
    #[inline] pub fn pdcrb_ptr(&self) -> *const Pdcrb { 
           self.pdcrb_mut()
    }

    #[doc="Read the PDCRB register."]
    #[inline] pub fn pdcrb(&self) -> Pdcrb { 
        unsafe {
            read_volatile(self.pdcrb_ptr())
        }
    }

    #[doc="Write the PDCRB register."]
    #[inline] pub fn set_pdcrb<F: FnOnce(Pdcrb) -> Pdcrb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrb_mut(), f(Pdcrb(0)));
        }
        self
    }

    #[doc="Modify the PDCRB register."]
    #[inline] pub fn with_pdcrb<F: FnOnce(Pdcrb) -> Pdcrb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrb_mut(), f(self.pdcrb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PUCRC register."]
    #[inline] pub fn pucrc_mut(&self) -> *mut Pucrc { 
        (self.0 + 0x30) as *mut Pucrc
    }

    #[doc="Get the *const pointer for the PUCRC register."]
    #[inline] pub fn pucrc_ptr(&self) -> *const Pucrc { 
           self.pucrc_mut()
    }

    #[doc="Read the PUCRC register."]
    #[inline] pub fn pucrc(&self) -> Pucrc { 
        unsafe {
            read_volatile(self.pucrc_ptr())
        }
    }

    #[doc="Write the PUCRC register."]
    #[inline] pub fn set_pucrc<F: FnOnce(Pucrc) -> Pucrc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrc_mut(), f(Pucrc(0)));
        }
        self
    }

    #[doc="Modify the PUCRC register."]
    #[inline] pub fn with_pucrc<F: FnOnce(Pucrc) -> Pucrc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrc_mut(), f(self.pucrc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDCRC register."]
    #[inline] pub fn pdcrc_mut(&self) -> *mut Pdcrc { 
        (self.0 + 0x34) as *mut Pdcrc
    }

    #[doc="Get the *const pointer for the PDCRC register."]
    #[inline] pub fn pdcrc_ptr(&self) -> *const Pdcrc { 
           self.pdcrc_mut()
    }

    #[doc="Read the PDCRC register."]
    #[inline] pub fn pdcrc(&self) -> Pdcrc { 
        unsafe {
            read_volatile(self.pdcrc_ptr())
        }
    }

    #[doc="Write the PDCRC register."]
    #[inline] pub fn set_pdcrc<F: FnOnce(Pdcrc) -> Pdcrc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrc_mut(), f(Pdcrc(0)));
        }
        self
    }

    #[doc="Modify the PDCRC register."]
    #[inline] pub fn with_pdcrc<F: FnOnce(Pdcrc) -> Pdcrc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrc_mut(), f(self.pdcrc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PUCRD register."]
    #[inline] pub fn pucrd_mut(&self) -> *mut Pucrd { 
        (self.0 + 0x38) as *mut Pucrd
    }

    #[doc="Get the *const pointer for the PUCRD register."]
    #[inline] pub fn pucrd_ptr(&self) -> *const Pucrd { 
           self.pucrd_mut()
    }

    #[doc="Read the PUCRD register."]
    #[inline] pub fn pucrd(&self) -> Pucrd { 
        unsafe {
            read_volatile(self.pucrd_ptr())
        }
    }

    #[doc="Write the PUCRD register."]
    #[inline] pub fn set_pucrd<F: FnOnce(Pucrd) -> Pucrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrd_mut(), f(Pucrd(0)));
        }
        self
    }

    #[doc="Modify the PUCRD register."]
    #[inline] pub fn with_pucrd<F: FnOnce(Pucrd) -> Pucrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrd_mut(), f(self.pucrd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDCRD register."]
    #[inline] pub fn pdcrd_mut(&self) -> *mut Pdcrd { 
        (self.0 + 0x3c) as *mut Pdcrd
    }

    #[doc="Get the *const pointer for the PDCRD register."]
    #[inline] pub fn pdcrd_ptr(&self) -> *const Pdcrd { 
           self.pdcrd_mut()
    }

    #[doc="Read the PDCRD register."]
    #[inline] pub fn pdcrd(&self) -> Pdcrd { 
        unsafe {
            read_volatile(self.pdcrd_ptr())
        }
    }

    #[doc="Write the PDCRD register."]
    #[inline] pub fn set_pdcrd<F: FnOnce(Pdcrd) -> Pdcrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrd_mut(), f(Pdcrd(0)));
        }
        self
    }

    #[doc="Modify the PDCRD register."]
    #[inline] pub fn with_pdcrd<F: FnOnce(Pdcrd) -> Pdcrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrd_mut(), f(self.pdcrd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PUCRE register."]
    #[inline] pub fn pucre_mut(&self) -> *mut Pucre { 
        (self.0 + 0x40) as *mut Pucre
    }

    #[doc="Get the *const pointer for the PUCRE register."]
    #[inline] pub fn pucre_ptr(&self) -> *const Pucre { 
           self.pucre_mut()
    }

    #[doc="Read the PUCRE register."]
    #[inline] pub fn pucre(&self) -> Pucre { 
        unsafe {
            read_volatile(self.pucre_ptr())
        }
    }

    #[doc="Write the PUCRE register."]
    #[inline] pub fn set_pucre<F: FnOnce(Pucre) -> Pucre>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucre_mut(), f(Pucre(0)));
        }
        self
    }

    #[doc="Modify the PUCRE register."]
    #[inline] pub fn with_pucre<F: FnOnce(Pucre) -> Pucre>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucre_mut(), f(self.pucre()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDCRE register."]
    #[inline] pub fn pdcre_mut(&self) -> *mut Pdcre { 
        (self.0 + 0x44) as *mut Pdcre
    }

    #[doc="Get the *const pointer for the PDCRE register."]
    #[inline] pub fn pdcre_ptr(&self) -> *const Pdcre { 
           self.pdcre_mut()
    }

    #[doc="Read the PDCRE register."]
    #[inline] pub fn pdcre(&self) -> Pdcre { 
        unsafe {
            read_volatile(self.pdcre_ptr())
        }
    }

    #[doc="Write the PDCRE register."]
    #[inline] pub fn set_pdcre<F: FnOnce(Pdcre) -> Pdcre>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcre_mut(), f(Pdcre(0)));
        }
        self
    }

    #[doc="Modify the PDCRE register."]
    #[inline] pub fn with_pdcre<F: FnOnce(Pdcre) -> Pdcre>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcre_mut(), f(self.pdcre()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PUCRF register."]
    #[inline] pub fn pucrf_mut(&self) -> *mut Pucrf { 
        (self.0 + 0x48) as *mut Pucrf
    }

    #[doc="Get the *const pointer for the PUCRF register."]
    #[inline] pub fn pucrf_ptr(&self) -> *const Pucrf { 
           self.pucrf_mut()
    }

    #[doc="Read the PUCRF register."]
    #[inline] pub fn pucrf(&self) -> Pucrf { 
        unsafe {
            read_volatile(self.pucrf_ptr())
        }
    }

    #[doc="Write the PUCRF register."]
    #[inline] pub fn set_pucrf<F: FnOnce(Pucrf) -> Pucrf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrf_mut(), f(Pucrf(0)));
        }
        self
    }

    #[doc="Modify the PUCRF register."]
    #[inline] pub fn with_pucrf<F: FnOnce(Pucrf) -> Pucrf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrf_mut(), f(self.pucrf()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDCRF register."]
    #[inline] pub fn pdcrf_mut(&self) -> *mut Pdcrf { 
        (self.0 + 0x4c) as *mut Pdcrf
    }

    #[doc="Get the *const pointer for the PDCRF register."]
    #[inline] pub fn pdcrf_ptr(&self) -> *const Pdcrf { 
           self.pdcrf_mut()
    }

    #[doc="Read the PDCRF register."]
    #[inline] pub fn pdcrf(&self) -> Pdcrf { 
        unsafe {
            read_volatile(self.pdcrf_ptr())
        }
    }

    #[doc="Write the PDCRF register."]
    #[inline] pub fn set_pdcrf<F: FnOnce(Pdcrf) -> Pdcrf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrf_mut(), f(Pdcrf(0)));
        }
        self
    }

    #[doc="Modify the PDCRF register."]
    #[inline] pub fn with_pdcrf<F: FnOnce(Pdcrf) -> Pdcrf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrf_mut(), f(self.pdcrf()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PUCRG register."]
    #[inline] pub fn pucrg_mut(&self) -> *mut Pucrg { 
        (self.0 + 0x50) as *mut Pucrg
    }

    #[doc="Get the *const pointer for the PUCRG register."]
    #[inline] pub fn pucrg_ptr(&self) -> *const Pucrg { 
           self.pucrg_mut()
    }

    #[doc="Read the PUCRG register."]
    #[inline] pub fn pucrg(&self) -> Pucrg { 
        unsafe {
            read_volatile(self.pucrg_ptr())
        }
    }

    #[doc="Write the PUCRG register."]
    #[inline] pub fn set_pucrg<F: FnOnce(Pucrg) -> Pucrg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrg_mut(), f(Pucrg(0)));
        }
        self
    }

    #[doc="Modify the PUCRG register."]
    #[inline] pub fn with_pucrg<F: FnOnce(Pucrg) -> Pucrg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrg_mut(), f(self.pucrg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDCRG register."]
    #[inline] pub fn pdcrg_mut(&self) -> *mut Pdcrg { 
        (self.0 + 0x54) as *mut Pdcrg
    }

    #[doc="Get the *const pointer for the PDCRG register."]
    #[inline] pub fn pdcrg_ptr(&self) -> *const Pdcrg { 
           self.pdcrg_mut()
    }

    #[doc="Read the PDCRG register."]
    #[inline] pub fn pdcrg(&self) -> Pdcrg { 
        unsafe {
            read_volatile(self.pdcrg_ptr())
        }
    }

    #[doc="Write the PDCRG register."]
    #[inline] pub fn set_pdcrg<F: FnOnce(Pdcrg) -> Pdcrg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrg_mut(), f(Pdcrg(0)));
        }
        self
    }

    #[doc="Modify the PDCRG register."]
    #[inline] pub fn with_pdcrg<F: FnOnce(Pdcrg) -> Pdcrg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrg_mut(), f(self.pdcrg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PUCRH register."]
    #[inline] pub fn pucrh_mut(&self) -> *mut Pucrh { 
        (self.0 + 0x58) as *mut Pucrh
    }

    #[doc="Get the *const pointer for the PUCRH register."]
    #[inline] pub fn pucrh_ptr(&self) -> *const Pucrh { 
           self.pucrh_mut()
    }

    #[doc="Read the PUCRH register."]
    #[inline] pub fn pucrh(&self) -> Pucrh { 
        unsafe {
            read_volatile(self.pucrh_ptr())
        }
    }

    #[doc="Write the PUCRH register."]
    #[inline] pub fn set_pucrh<F: FnOnce(Pucrh) -> Pucrh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrh_mut(), f(Pucrh(0)));
        }
        self
    }

    #[doc="Modify the PUCRH register."]
    #[inline] pub fn with_pucrh<F: FnOnce(Pucrh) -> Pucrh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pucrh_mut(), f(self.pucrh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDCRH register."]
    #[inline] pub fn pdcrh_mut(&self) -> *mut Pdcrh { 
        (self.0 + 0x5c) as *mut Pdcrh
    }

    #[doc="Get the *const pointer for the PDCRH register."]
    #[inline] pub fn pdcrh_ptr(&self) -> *const Pdcrh { 
           self.pdcrh_mut()
    }

    #[doc="Read the PDCRH register."]
    #[inline] pub fn pdcrh(&self) -> Pdcrh { 
        unsafe {
            read_volatile(self.pdcrh_ptr())
        }
    }

    #[doc="Write the PDCRH register."]
    #[inline] pub fn set_pdcrh<F: FnOnce(Pdcrh) -> Pdcrh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrh_mut(), f(Pdcrh(0)));
        }
        self
    }

    #[doc="Modify the PDCRH register."]
    #[inline] pub fn with_pdcrh<F: FnOnce(Pdcrh) -> Pdcrh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdcrh_mut(), f(self.pdcrh()));
        }
        self
    }

}

#[doc="Power control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Low-power run"]
    #[inline] pub fn lpr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LPR != 0"]
    #[inline] pub fn test_lpr(&self) -> bool {
        self.lpr() != 0
    }

    #[doc="Sets the LPR field."]
    #[inline] pub fn set_lpr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Voltage scaling range selection"]
    #[inline] pub fn vos(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if VOS != 0"]
    #[inline] pub fn test_vos(&self) -> bool {
        self.vos() != 0
    }

    #[doc="Sets the VOS field."]
    #[inline] pub fn set_vos<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Disable backup domain write protection"]
    #[inline] pub fn dbp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DBP != 0"]
    #[inline] pub fn test_dbp(&self) -> bool {
        self.dbp() != 0
    }

    #[doc="Sets the DBP field."]
    #[inline] pub fn set_dbp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Low-power mode selection"]
    #[inline] pub fn lpms(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LPMS != 0"]
    #[inline] pub fn test_lpms(&self) -> bool {
        self.lpms() != 0
    }

    #[doc="Sets the LPMS field."]
    #[inline] pub fn set_lpms<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
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
    #[inline] pub fn usv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if USV != 0"]
    #[inline] pub fn test_usv(&self) -> bool {
        self.usv() != 0
    }

    #[doc="Sets the USV field."]
    #[inline] pub fn set_usv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="VDDIO2 Independent I/Os supply valid"]
    #[inline] pub fn iosv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if IOSV != 0"]
    #[inline] pub fn test_iosv(&self) -> bool {
        self.iosv() != 0
    }

    #[doc="Sets the IOSV field."]
    #[inline] pub fn set_iosv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline] pub fn pvme4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PVME4 != 0"]
    #[inline] pub fn test_pvme4(&self) -> bool {
        self.pvme4() != 0
    }

    #[doc="Sets the PVME4 field."]
    #[inline] pub fn set_pvme4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline] pub fn pvme3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PVME3 != 0"]
    #[inline] pub fn test_pvme3(&self) -> bool {
        self.pvme3() != 0
    }

    #[doc="Sets the PVME3 field."]
    #[inline] pub fn set_pvme3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline] pub fn pvme2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PVME2 != 0"]
    #[inline] pub fn test_pvme2(&self) -> bool {
        self.pvme2() != 0
    }

    #[doc="Sets the PVME2 field."]
    #[inline] pub fn set_pvme2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline] pub fn pvme1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PVME1 != 0"]
    #[inline] pub fn test_pvme1(&self) -> bool {
        self.pvme1() != 0
    }

    #[doc="Sets the PVME1 field."]
    #[inline] pub fn set_pvme1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Power voltage detector level selection"]
    #[inline] pub fn pls(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if PLS != 0"]
    #[inline] pub fn test_pls(&self) -> bool {
        self.pls() != 0
    }

    #[doc="Sets the PLS field."]
    #[inline] pub fn set_pls<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Power voltage detector enable"]
    #[inline] pub fn pvde(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PVDE != 0"]
    #[inline] pub fn test_pvde(&self) -> bool {
        self.pvde() != 0
    }

    #[doc="Sets the PVDE field."]
    #[inline] pub fn set_pvde<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.iosv() != 0 { try!(write!(f, " iosv"))}
        if self.pvme4() != 0 { try!(write!(f, " pvme4"))}
        if self.pvme3() != 0 { try!(write!(f, " pvme3"))}
        if self.pvme2() != 0 { try!(write!(f, " pvme2"))}
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
    #[doc="Enable internal wakeup line"]
    #[inline] pub fn ewf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EWF != 0"]
    #[inline] pub fn test_ewf(&self) -> bool {
        self.ewf() != 0
    }

    #[doc="Sets the EWF field."]
    #[inline] pub fn set_ewf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Apply pull-up and pull-down configuration"]
    #[inline] pub fn apc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if APC != 0"]
    #[inline] pub fn test_apc(&self) -> bool {
        self.apc() != 0
    }

    #[doc="Sets the APC field."]
    #[inline] pub fn set_apc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="SRAM2 retention in Standby mode"]
    #[inline] pub fn rrs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RRS != 0"]
    #[inline] pub fn test_rrs(&self) -> bool {
        self.rrs() != 0
    }

    #[doc="Sets the RRS field."]
    #[inline] pub fn set_rrs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enable Wakeup pin WKUP5"]
    #[inline] pub fn ewup5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EWUP5 != 0"]
    #[inline] pub fn test_ewup5(&self) -> bool {
        self.ewup5() != 0
    }

    #[doc="Sets the EWUP5 field."]
    #[inline] pub fn set_ewup5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enable Wakeup pin WKUP4"]
    #[inline] pub fn ewup4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EWUP4 != 0"]
    #[inline] pub fn test_ewup4(&self) -> bool {
        self.ewup4() != 0
    }

    #[doc="Sets the EWUP4 field."]
    #[inline] pub fn set_ewup4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enable Wakeup pin WKUP3"]
    #[inline] pub fn ewup3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EWUP3 != 0"]
    #[inline] pub fn test_ewup3(&self) -> bool {
        self.ewup3() != 0
    }

    #[doc="Sets the EWUP3 field."]
    #[inline] pub fn set_ewup3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enable Wakeup pin WKUP2"]
    #[inline] pub fn ewup2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EWUP2 != 0"]
    #[inline] pub fn test_ewup2(&self) -> bool {
        self.ewup2() != 0
    }

    #[doc="Sets the EWUP2 field."]
    #[inline] pub fn set_ewup2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable Wakeup pin WKUP1"]
    #[inline] pub fn ewup1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EWUP1 != 0"]
    #[inline] pub fn test_ewup1(&self) -> bool {
        self.ewup1() != 0
    }

    #[doc="Sets the EWUP1 field."]
    #[inline] pub fn set_ewup1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.ewf() != 0 { try!(write!(f, " ewf"))}
        if self.apc() != 0 { try!(write!(f, " apc"))}
        if self.rrs() != 0 { try!(write!(f, " rrs"))}
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
    #[doc="VBAT battery charging resistor selection"]
    #[inline] pub fn vbrs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if VBRS != 0"]
    #[inline] pub fn test_vbrs(&self) -> bool {
        self.vbrs() != 0
    }

    #[doc="Sets the VBRS field."]
    #[inline] pub fn set_vbrs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="VBAT battery charging enable"]
    #[inline] pub fn vbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if VBE != 0"]
    #[inline] pub fn test_vbe(&self) -> bool {
        self.vbe() != 0
    }

    #[doc="Sets the VBE field."]
    #[inline] pub fn set_vbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Wakeup pin WKUP5 polarity"]
    #[inline] pub fn wp5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WP5 != 0"]
    #[inline] pub fn test_wp5(&self) -> bool {
        self.wp5() != 0
    }

    #[doc="Sets the WP5 field."]
    #[inline] pub fn set_wp5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Wakeup pin WKUP4 polarity"]
    #[inline] pub fn wp4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WP4 != 0"]
    #[inline] pub fn test_wp4(&self) -> bool {
        self.wp4() != 0
    }

    #[doc="Sets the WP4 field."]
    #[inline] pub fn set_wp4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wakeup pin WKUP3 polarity"]
    #[inline] pub fn wp3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WP3 != 0"]
    #[inline] pub fn test_wp3(&self) -> bool {
        self.wp3() != 0
    }

    #[doc="Sets the WP3 field."]
    #[inline] pub fn set_wp3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Wakeup pin WKUP2 polarity"]
    #[inline] pub fn wp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WP2 != 0"]
    #[inline] pub fn test_wp2(&self) -> bool {
        self.wp2() != 0
    }

    #[doc="Sets the WP2 field."]
    #[inline] pub fn set_wp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wakeup pin WKUP1 polarity"]
    #[inline] pub fn wp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WP1 != 0"]
    #[inline] pub fn test_wp1(&self) -> bool {
        self.wp1() != 0
    }

    #[doc="Sets the WP1 field."]
    #[inline] pub fn set_wp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[doc="Wakeup flag internal"]
    #[inline] pub fn wufi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if WUFI != 0"]
    #[inline] pub fn test_wufi(&self) -> bool {
        self.wufi() != 0
    }

    #[doc="Sets the WUFI field."]
    #[inline] pub fn set_wufi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Standby flag"]
    #[inline] pub fn csbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSBF != 0"]
    #[inline] pub fn test_csbf(&self) -> bool {
        self.csbf() != 0
    }

    #[doc="Sets the CSBF field."]
    #[inline] pub fn set_csbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Wakeup flag 5"]
    #[inline] pub fn cwuf5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CWUF5 != 0"]
    #[inline] pub fn test_cwuf5(&self) -> bool {
        self.cwuf5() != 0
    }

    #[doc="Sets the CWUF5 field."]
    #[inline] pub fn set_cwuf5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Wakeup flag 4"]
    #[inline] pub fn cwuf4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CWUF4 != 0"]
    #[inline] pub fn test_cwuf4(&self) -> bool {
        self.cwuf4() != 0
    }

    #[doc="Sets the CWUF4 field."]
    #[inline] pub fn set_cwuf4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wakeup flag 3"]
    #[inline] pub fn cwuf3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CWUF3 != 0"]
    #[inline] pub fn test_cwuf3(&self) -> bool {
        self.cwuf3() != 0
    }

    #[doc="Sets the CWUF3 field."]
    #[inline] pub fn set_cwuf3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Wakeup flag 2"]
    #[inline] pub fn cwuf2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CWUF2 != 0"]
    #[inline] pub fn test_cwuf2(&self) -> bool {
        self.cwuf2() != 0
    }

    #[doc="Sets the CWUF2 field."]
    #[inline] pub fn set_cwuf2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wakeup flag 1"]
    #[inline] pub fn cwuf1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CWUF1 != 0"]
    #[inline] pub fn test_cwuf1(&self) -> bool {
        self.cwuf1() != 0
    }

    #[doc="Sets the CWUF1 field."]
    #[inline] pub fn set_cwuf1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.csbf() != 0 { try!(write!(f, " csbf"))}
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
    #[doc="Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
    #[inline] pub fn pvmo4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PVMO4 != 0"]
    #[inline] pub fn test_pvmo4(&self) -> bool {
        self.pvmo4() != 0
    }

    #[doc="Sets the PVMO4 field."]
    #[inline] pub fn set_pvmo4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline] pub fn pvmo3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PVMO3 != 0"]
    #[inline] pub fn test_pvmo3(&self) -> bool {
        self.pvmo3() != 0
    }

    #[doc="Sets the PVMO3 field."]
    #[inline] pub fn set_pvmo3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
    #[inline] pub fn pvmo2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PVMO2 != 0"]
    #[inline] pub fn test_pvmo2(&self) -> bool {
        self.pvmo2() != 0
    }

    #[doc="Sets the PVMO2 field."]
    #[inline] pub fn set_pvmo2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
    #[inline] pub fn pvmo1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PVMO1 != 0"]
    #[inline] pub fn test_pvmo1(&self) -> bool {
        self.pvmo1() != 0
    }

    #[doc="Sets the PVMO1 field."]
    #[inline] pub fn set_pvmo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Power voltage detector output"]
    #[inline] pub fn pvdo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PVDO != 0"]
    #[inline] pub fn test_pvdo(&self) -> bool {
        self.pvdo() != 0
    }

    #[doc="Sets the PVDO field."]
    #[inline] pub fn set_pvdo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Voltage scaling flag"]
    #[inline] pub fn vosf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VOSF != 0"]
    #[inline] pub fn test_vosf(&self) -> bool {
        self.vosf() != 0
    }

    #[doc="Sets the VOSF field."]
    #[inline] pub fn set_vosf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Low-power regulator flag"]
    #[inline] pub fn reglpf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if REGLPF != 0"]
    #[inline] pub fn test_reglpf(&self) -> bool {
        self.reglpf() != 0
    }

    #[doc="Sets the REGLPF field."]
    #[inline] pub fn set_reglpf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Low-power regulator started"]
    #[inline] pub fn reglps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if REGLPS != 0"]
    #[inline] pub fn test_reglps(&self) -> bool {
        self.reglps() != 0
    }

    #[doc="Sets the REGLPS field."]
    #[inline] pub fn set_reglps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        if self.pvmo4() != 0 { try!(write!(f, " pvmo4"))}
        if self.pvmo3() != 0 { try!(write!(f, " pvmo3"))}
        if self.pvmo2() != 0 { try!(write!(f, " pvmo2"))}
        if self.pvmo1() != 0 { try!(write!(f, " pvmo1"))}
        if self.pvdo() != 0 { try!(write!(f, " pvdo"))}
        if self.vosf() != 0 { try!(write!(f, " vosf"))}
        if self.reglpf() != 0 { try!(write!(f, " reglpf"))}
        if self.reglps() != 0 { try!(write!(f, " reglps"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power status clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc="Clear standby flag"]
    #[inline] pub fn sbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SBF != 0"]
    #[inline] pub fn test_sbf(&self) -> bool {
        self.sbf() != 0
    }

    #[doc="Sets the SBF field."]
    #[inline] pub fn set_sbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear wakeup flag 5"]
    #[inline] pub fn wuf5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WUF5 != 0"]
    #[inline] pub fn test_wuf5(&self) -> bool {
        self.wuf5() != 0
    }

    #[doc="Sets the WUF5 field."]
    #[inline] pub fn set_wuf5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear wakeup flag 4"]
    #[inline] pub fn wuf4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WUF4 != 0"]
    #[inline] pub fn test_wuf4(&self) -> bool {
        self.wuf4() != 0
    }

    #[doc="Sets the WUF4 field."]
    #[inline] pub fn set_wuf4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear wakeup flag 3"]
    #[inline] pub fn wuf3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WUF3 != 0"]
    #[inline] pub fn test_wuf3(&self) -> bool {
        self.wuf3() != 0
    }

    #[doc="Sets the WUF3 field."]
    #[inline] pub fn set_wuf3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear wakeup flag 2"]
    #[inline] pub fn wuf2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WUF2 != 0"]
    #[inline] pub fn test_wuf2(&self) -> bool {
        self.wuf2() != 0
    }

    #[doc="Sets the WUF2 field."]
    #[inline] pub fn set_wuf2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear wakeup flag 1"]
    #[inline] pub fn wuf1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WUF1 != 0"]
    #[inline] pub fn test_wuf1(&self) -> bool {
        self.wuf1() != 0
    }

    #[doc="Sets the WUF1 field."]
    #[inline] pub fn set_wuf1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.sbf() != 0 { try!(write!(f, " sbf"))}
        if self.wuf5() != 0 { try!(write!(f, " wuf5"))}
        if self.wuf4() != 0 { try!(write!(f, " wuf4"))}
        if self.wuf3() != 0 { try!(write!(f, " wuf3"))}
        if self.wuf2() != 0 { try!(write!(f, " wuf2"))}
        if self.wuf1() != 0 { try!(write!(f, " wuf1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Port A pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucra(pub u32);
impl Pucra {
    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port A pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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

#[doc="Power Port A pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcra(pub u32);
impl Pdcra {
    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port A pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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

#[doc="Power Port B pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucrb(pub u32);
impl Pucrb {
    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port B pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn pd15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port B pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.pd4() != 0 { try!(write!(f, " pd4"))}
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
    #[inline] pub fn pu15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port C pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn pd15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port C pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn pu15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port D pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn pd15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port D pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn pu15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port E pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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

#[doc="Power Port E pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcre(pub u32);
impl Pdcre {
    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port E pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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

#[doc="Power Port F pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucrf(pub u32);
impl Pucrf {
    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port F pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pucrf {
    #[inline]
    fn from(other: u32) -> Self {
         Pucrf(other)
    }
}

impl ::core::fmt::Display for Pucrf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pucrf {
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

#[doc="Power Port F pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcrf(pub u32);
impl Pdcrf {
    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port F pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdcrf {
    #[inline]
    fn from(other: u32) -> Self {
         Pdcrf(other)
    }
}

impl ::core::fmt::Display for Pdcrf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdcrf {
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

#[doc="Power Port G pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucrg(pub u32);
impl Pucrg {
    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PU15 != 0"]
    #[inline] pub fn test_pu15(&self) -> bool {
        self.pu15() != 0
    }

    #[doc="Sets the PU15 field."]
    #[inline] pub fn set_pu15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PU14 != 0"]
    #[inline] pub fn test_pu14(&self) -> bool {
        self.pu14() != 0
    }

    #[doc="Sets the PU14 field."]
    #[inline] pub fn set_pu14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PU13 != 0"]
    #[inline] pub fn test_pu13(&self) -> bool {
        self.pu13() != 0
    }

    #[doc="Sets the PU13 field."]
    #[inline] pub fn set_pu13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PU12 != 0"]
    #[inline] pub fn test_pu12(&self) -> bool {
        self.pu12() != 0
    }

    #[doc="Sets the PU12 field."]
    #[inline] pub fn set_pu12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PU11 != 0"]
    #[inline] pub fn test_pu11(&self) -> bool {
        self.pu11() != 0
    }

    #[doc="Sets the PU11 field."]
    #[inline] pub fn set_pu11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PU10 != 0"]
    #[inline] pub fn test_pu10(&self) -> bool {
        self.pu10() != 0
    }

    #[doc="Sets the PU10 field."]
    #[inline] pub fn set_pu10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PU9 != 0"]
    #[inline] pub fn test_pu9(&self) -> bool {
        self.pu9() != 0
    }

    #[doc="Sets the PU9 field."]
    #[inline] pub fn set_pu9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PU8 != 0"]
    #[inline] pub fn test_pu8(&self) -> bool {
        self.pu8() != 0
    }

    #[doc="Sets the PU8 field."]
    #[inline] pub fn set_pu8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PU7 != 0"]
    #[inline] pub fn test_pu7(&self) -> bool {
        self.pu7() != 0
    }

    #[doc="Sets the PU7 field."]
    #[inline] pub fn set_pu7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PU6 != 0"]
    #[inline] pub fn test_pu6(&self) -> bool {
        self.pu6() != 0
    }

    #[doc="Sets the PU6 field."]
    #[inline] pub fn set_pu6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PU5 != 0"]
    #[inline] pub fn test_pu5(&self) -> bool {
        self.pu5() != 0
    }

    #[doc="Sets the PU5 field."]
    #[inline] pub fn set_pu5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PU4 != 0"]
    #[inline] pub fn test_pu4(&self) -> bool {
        self.pu4() != 0
    }

    #[doc="Sets the PU4 field."]
    #[inline] pub fn set_pu4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PU3 != 0"]
    #[inline] pub fn test_pu3(&self) -> bool {
        self.pu3() != 0
    }

    #[doc="Sets the PU3 field."]
    #[inline] pub fn set_pu3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PU2 != 0"]
    #[inline] pub fn test_pu2(&self) -> bool {
        self.pu2() != 0
    }

    #[doc="Sets the PU2 field."]
    #[inline] pub fn set_pu2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port G pull-up bit y (y=0..15)"]
    #[inline] pub fn pu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pucrg {
    #[inline]
    fn from(other: u32) -> Self {
         Pucrg(other)
    }
}

impl ::core::fmt::Display for Pucrg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pucrg {
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

#[doc="Power Port G pull-down control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdcrg(pub u32);
impl Pdcrg {
    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD15 != 0"]
    #[inline] pub fn test_pd15(&self) -> bool {
        self.pd15() != 0
    }

    #[doc="Sets the PD15 field."]
    #[inline] pub fn set_pd15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD14 != 0"]
    #[inline] pub fn test_pd14(&self) -> bool {
        self.pd14() != 0
    }

    #[doc="Sets the PD14 field."]
    #[inline] pub fn set_pd14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD13 != 0"]
    #[inline] pub fn test_pd13(&self) -> bool {
        self.pd13() != 0
    }

    #[doc="Sets the PD13 field."]
    #[inline] pub fn set_pd13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD12 != 0"]
    #[inline] pub fn test_pd12(&self) -> bool {
        self.pd12() != 0
    }

    #[doc="Sets the PD12 field."]
    #[inline] pub fn set_pd12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD11 != 0"]
    #[inline] pub fn test_pd11(&self) -> bool {
        self.pd11() != 0
    }

    #[doc="Sets the PD11 field."]
    #[inline] pub fn set_pd11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD10 != 0"]
    #[inline] pub fn test_pd10(&self) -> bool {
        self.pd10() != 0
    }

    #[doc="Sets the PD10 field."]
    #[inline] pub fn set_pd10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD9 != 0"]
    #[inline] pub fn test_pd9(&self) -> bool {
        self.pd9() != 0
    }

    #[doc="Sets the PD9 field."]
    #[inline] pub fn set_pd9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD8 != 0"]
    #[inline] pub fn test_pd8(&self) -> bool {
        self.pd8() != 0
    }

    #[doc="Sets the PD8 field."]
    #[inline] pub fn set_pd8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PD7 != 0"]
    #[inline] pub fn test_pd7(&self) -> bool {
        self.pd7() != 0
    }

    #[doc="Sets the PD7 field."]
    #[inline] pub fn set_pd7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD6 != 0"]
    #[inline] pub fn test_pd6(&self) -> bool {
        self.pd6() != 0
    }

    #[doc="Sets the PD6 field."]
    #[inline] pub fn set_pd6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD5 != 0"]
    #[inline] pub fn test_pd5(&self) -> bool {
        self.pd5() != 0
    }

    #[doc="Sets the PD5 field."]
    #[inline] pub fn set_pd5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD4 != 0"]
    #[inline] pub fn test_pd4(&self) -> bool {
        self.pd4() != 0
    }

    #[doc="Sets the PD4 field."]
    #[inline] pub fn set_pd4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PD3 != 0"]
    #[inline] pub fn test_pd3(&self) -> bool {
        self.pd3() != 0
    }

    #[doc="Sets the PD3 field."]
    #[inline] pub fn set_pd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PD2 != 0"]
    #[inline] pub fn test_pd2(&self) -> bool {
        self.pd2() != 0
    }

    #[doc="Sets the PD2 field."]
    #[inline] pub fn set_pd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port G pull-down bit y (y=0..15)"]
    #[inline] pub fn pd0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdcrg {
    #[inline]
    fn from(other: u32) -> Self {
         Pdcrg(other)
    }
}

impl ::core::fmt::Display for Pdcrg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdcrg {
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

#[doc="Power Port H pull-up control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pucrh(pub u32);
impl Pucrh {
    #[doc="Port H pull-up bit y (y=0..1)"]
    #[inline] pub fn pu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PU1 != 0"]
    #[inline] pub fn test_pu1(&self) -> bool {
        self.pu1() != 0
    }

    #[doc="Sets the PU1 field."]
    #[inline] pub fn set_pu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port H pull-up bit y (y=0..1)"]
    #[inline] pub fn pu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PU0 != 0"]
    #[inline] pub fn test_pu0(&self) -> bool {
        self.pu0() != 0
    }

    #[doc="Sets the PU0 field."]
    #[inline] pub fn set_pu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
    #[inline] pub fn pd1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PD1 != 0"]
    #[inline] pub fn test_pd1(&self) -> bool {
        self.pd1() != 0
    }

    #[doc="Sets the PD1 field."]
    #[inline] pub fn set_pd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port H pull-down bit y (y=0..1)"]
    #[inline] pub fn pd0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD0 != 0"]
    #[inline] pub fn test_pd0(&self) -> bool {
        self.pd0() != 0
    }

    #[doc="Sets the PD0 field."]
    #[inline] pub fn set_pd0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.pd1() != 0 { try!(write!(f, " pd1"))}
        if self.pd0() != 0 { try!(write!(f, " pd0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

