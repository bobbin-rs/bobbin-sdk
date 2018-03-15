#[allow(unused_imports)] use bobbin_common::*;

periph!( DFSDM, Dfsdm, _DFSDM, DfsdmPeriph, 0x40016000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DFSDM Peripheral"]
pub struct DfsdmPeriph(pub usize); 

impl super::sig::Signal<super::sig::Dfsdm1Ckout> for Dfsdm {}
impl super::sig::SignalDfsdmCkout<super::sig::Dfsdm1Ckout> for Dfsdm {}
impl super::sig::Signal<super::sig::Dfsdm1Ckin0> for Dfsdm {}
impl super::sig::SignalDfsdmCkin0<super::sig::Dfsdm1Ckin0> for Dfsdm {}
impl super::sig::Signal<super::sig::Dfsdm1Ckin1> for Dfsdm {}
impl super::sig::SignalDfsdmCkin1<super::sig::Dfsdm1Ckin1> for Dfsdm {}
impl super::sig::Signal<super::sig::Dfsdm1Ckin2> for Dfsdm {}
impl super::sig::SignalDfsdmCkin2<super::sig::Dfsdm1Ckin2> for Dfsdm {}
impl super::sig::Signal<super::sig::Dfsdm1Ckin3> for Dfsdm {}
impl super::sig::SignalDfsdmCkin3<super::sig::Dfsdm1Ckin3> for Dfsdm {}
impl super::sig::Signal<super::sig::Dfsdm1Datin0> for Dfsdm {}
impl super::sig::SignalDfsdmDatin0<super::sig::Dfsdm1Datin0> for Dfsdm {}
impl super::sig::Signal<super::sig::Dfsdm2Datin1> for Dfsdm {}
impl super::sig::SignalDfsdmDatin1<super::sig::Dfsdm2Datin1> for Dfsdm {}
impl super::sig::Signal<super::sig::Dfsdm3Datin2> for Dfsdm {}
impl super::sig::SignalDfsdmDatin2<super::sig::Dfsdm3Datin2> for Dfsdm {}
impl super::sig::Signal<super::sig::Dfsdm4Datin3> for Dfsdm {}
impl super::sig::SignalDfsdmDatin3<super::sig::Dfsdm4Datin3> for Dfsdm {}


impl DfsdmPeriph {
    #[doc="Get the *mut pointer for the CHCFGR1 register."]
    #[inline] pub fn chcfgr1_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Chcfgr1 { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x0 + (index * 32)) as *mut Chcfgr1
    }

    #[doc="Get the *const pointer for the CHCFGR1 register."]
    #[inline] pub fn chcfgr1_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Chcfgr1 { 
           self.chcfgr1_mut(index)
    }

    #[doc="Read the CHCFGR1 register."]
    #[inline] pub fn chcfgr1<I: Into<bits::R4>>(&self, index: I) -> Chcfgr1 { 
        unsafe {
            read_volatile(self.chcfgr1_ptr(index))
        }
    }

    #[doc="Write the CHCFGR1 register."]
    #[inline] pub fn set_chcfgr1<I: Into<bits::R4>, F: FnOnce(Chcfgr1) -> Chcfgr1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfgr1_mut(index), f(Chcfgr1(0)));
        }
        self
    }

    #[doc="Modify the CHCFGR1 register."]
    #[inline] pub fn with_chcfgr1<I: Into<bits::R4> + Copy, F: FnOnce(Chcfgr1) -> Chcfgr1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfgr1_mut(index), f(self.chcfgr1(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFGR2 register."]
    #[inline] pub fn chcfgr2_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Chcfgr2 { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x4 + (index * 32)) as *mut Chcfgr2
    }

    #[doc="Get the *const pointer for the CHCFGR2 register."]
    #[inline] pub fn chcfgr2_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Chcfgr2 { 
           self.chcfgr2_mut(index)
    }

    #[doc="Read the CHCFGR2 register."]
    #[inline] pub fn chcfgr2<I: Into<bits::R4>>(&self, index: I) -> Chcfgr2 { 
        unsafe {
            read_volatile(self.chcfgr2_ptr(index))
        }
    }

    #[doc="Write the CHCFGR2 register."]
    #[inline] pub fn set_chcfgr2<I: Into<bits::R4>, F: FnOnce(Chcfgr2) -> Chcfgr2>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfgr2_mut(index), f(Chcfgr2(0)));
        }
        self
    }

    #[doc="Modify the CHCFGR2 register."]
    #[inline] pub fn with_chcfgr2<I: Into<bits::R4> + Copy, F: FnOnce(Chcfgr2) -> Chcfgr2>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfgr2_mut(index), f(self.chcfgr2(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCDR register."]
    #[inline] pub fn awscdr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Awscdr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x8 + (index * 32)) as *mut Awscdr
    }

    #[doc="Get the *const pointer for the AWSCDR register."]
    #[inline] pub fn awscdr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Awscdr { 
           self.awscdr_mut(index)
    }

    #[doc="Read the AWSCDR register."]
    #[inline] pub fn awscdr<I: Into<bits::R4>>(&self, index: I) -> Awscdr { 
        unsafe {
            read_volatile(self.awscdr_ptr(index))
        }
    }

    #[doc="Write the AWSCDR register."]
    #[inline] pub fn set_awscdr<I: Into<bits::R4>, F: FnOnce(Awscdr) -> Awscdr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscdr_mut(index), f(Awscdr(0)));
        }
        self
    }

    #[doc="Modify the AWSCDR register."]
    #[inline] pub fn with_awscdr<I: Into<bits::R4> + Copy, F: FnOnce(Awscdr) -> Awscdr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscdr_mut(index), f(self.awscdr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDATR register."]
    #[inline] pub fn chwdatr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Chwdatr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xc + (index * 32)) as *mut Chwdatr
    }

    #[doc="Get the *const pointer for the CHWDATR register."]
    #[inline] pub fn chwdatr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Chwdatr { 
           self.chwdatr_mut(index)
    }

    #[doc="Read the CHWDATR register."]
    #[inline] pub fn chwdatr<I: Into<bits::R4>>(&self, index: I) -> Chwdatr { 
        unsafe {
            read_volatile(self.chwdatr_ptr(index))
        }
    }

    #[doc="Write the CHWDATR register."]
    #[inline] pub fn set_chwdatr<I: Into<bits::R4>, F: FnOnce(Chwdatr) -> Chwdatr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdatr_mut(index), f(Chwdatr(0)));
        }
        self
    }

    #[doc="Modify the CHWDATR register."]
    #[inline] pub fn with_chwdatr<I: Into<bits::R4> + Copy, F: FnOnce(Chwdatr) -> Chwdatr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdatr_mut(index), f(self.chwdatr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATINR register."]
    #[inline] pub fn chdatinr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Chdatinr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x10 + (index * 32)) as *mut Chdatinr
    }

    #[doc="Get the *const pointer for the CHDATINR register."]
    #[inline] pub fn chdatinr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Chdatinr { 
           self.chdatinr_mut(index)
    }

    #[doc="Read the CHDATINR register."]
    #[inline] pub fn chdatinr<I: Into<bits::R4>>(&self, index: I) -> Chdatinr { 
        unsafe {
            read_volatile(self.chdatinr_ptr(index))
        }
    }

    #[doc="Write the CHDATINR register."]
    #[inline] pub fn set_chdatinr<I: Into<bits::R4>, F: FnOnce(Chdatinr) -> Chdatinr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatinr_mut(index), f(Chdatinr(0)));
        }
        self
    }

    #[doc="Modify the CHDATINR register."]
    #[inline] pub fn with_chdatinr<I: Into<bits::R4> + Copy, F: FnOnce(Chdatinr) -> Chdatinr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatinr_mut(index), f(self.chdatinr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTCR1 register."]
    #[inline] pub fn fltcr1_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltcr1 { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x100 + (index * 128)) as *mut Fltcr1
    }

    #[doc="Get the *const pointer for the FLTCR1 register."]
    #[inline] pub fn fltcr1_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltcr1 { 
           self.fltcr1_mut(index)
    }

    #[doc="Read the FLTCR1 register."]
    #[inline] pub fn fltcr1<I: Into<bits::R4>>(&self, index: I) -> Fltcr1 { 
        unsafe {
            read_volatile(self.fltcr1_ptr(index))
        }
    }

    #[doc="Write the FLTCR1 register."]
    #[inline] pub fn set_fltcr1<I: Into<bits::R4>, F: FnOnce(Fltcr1) -> Fltcr1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltcr1_mut(index), f(Fltcr1(0)));
        }
        self
    }

    #[doc="Modify the FLTCR1 register."]
    #[inline] pub fn with_fltcr1<I: Into<bits::R4> + Copy, F: FnOnce(Fltcr1) -> Fltcr1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltcr1_mut(index), f(self.fltcr1(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTCR2 register."]
    #[inline] pub fn fltcr2_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltcr2 { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x104 + (index * 128)) as *mut Fltcr2
    }

    #[doc="Get the *const pointer for the FLTCR2 register."]
    #[inline] pub fn fltcr2_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltcr2 { 
           self.fltcr2_mut(index)
    }

    #[doc="Read the FLTCR2 register."]
    #[inline] pub fn fltcr2<I: Into<bits::R4>>(&self, index: I) -> Fltcr2 { 
        unsafe {
            read_volatile(self.fltcr2_ptr(index))
        }
    }

    #[doc="Write the FLTCR2 register."]
    #[inline] pub fn set_fltcr2<I: Into<bits::R4>, F: FnOnce(Fltcr2) -> Fltcr2>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltcr2_mut(index), f(Fltcr2(0)));
        }
        self
    }

    #[doc="Modify the FLTCR2 register."]
    #[inline] pub fn with_fltcr2<I: Into<bits::R4> + Copy, F: FnOnce(Fltcr2) -> Fltcr2>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltcr2_mut(index), f(self.fltcr2(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTISR register."]
    #[inline] pub fn fltisr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltisr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x108 + (index * 128)) as *mut Fltisr
    }

    #[doc="Get the *const pointer for the FLTISR register."]
    #[inline] pub fn fltisr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltisr { 
           self.fltisr_mut(index)
    }

    #[doc="Read the FLTISR register."]
    #[inline] pub fn fltisr<I: Into<bits::R4>>(&self, index: I) -> Fltisr { 
        unsafe {
            read_volatile(self.fltisr_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the FLTICR register."]
    #[inline] pub fn flticr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Flticr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x10c + (index * 128)) as *mut Flticr
    }

    #[doc="Get the *const pointer for the FLTICR register."]
    #[inline] pub fn flticr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Flticr { 
           self.flticr_mut(index)
    }

    #[doc="Read the FLTICR register."]
    #[inline] pub fn flticr<I: Into<bits::R4>>(&self, index: I) -> Flticr { 
        unsafe {
            read_volatile(self.flticr_ptr(index))
        }
    }

    #[doc="Write the FLTICR register."]
    #[inline] pub fn set_flticr<I: Into<bits::R4>, F: FnOnce(Flticr) -> Flticr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.flticr_mut(index), f(Flticr(0)));
        }
        self
    }

    #[doc="Modify the FLTICR register."]
    #[inline] pub fn with_flticr<I: Into<bits::R4> + Copy, F: FnOnce(Flticr) -> Flticr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.flticr_mut(index), f(self.flticr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTJCHGR register."]
    #[inline] pub fn fltjchgr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltjchgr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x110 + (index * 128)) as *mut Fltjchgr
    }

    #[doc="Get the *const pointer for the FLTJCHGR register."]
    #[inline] pub fn fltjchgr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltjchgr { 
           self.fltjchgr_mut(index)
    }

    #[doc="Read the FLTJCHGR register."]
    #[inline] pub fn fltjchgr<I: Into<bits::R4>>(&self, index: I) -> Fltjchgr { 
        unsafe {
            read_volatile(self.fltjchgr_ptr(index))
        }
    }

    #[doc="Write the FLTJCHGR register."]
    #[inline] pub fn set_fltjchgr<I: Into<bits::R4>, F: FnOnce(Fltjchgr) -> Fltjchgr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltjchgr_mut(index), f(Fltjchgr(0)));
        }
        self
    }

    #[doc="Modify the FLTJCHGR register."]
    #[inline] pub fn with_fltjchgr<I: Into<bits::R4> + Copy, F: FnOnce(Fltjchgr) -> Fltjchgr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltjchgr_mut(index), f(self.fltjchgr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTFCR register."]
    #[inline] pub fn fltfcr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltfcr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x114 + (index * 128)) as *mut Fltfcr
    }

    #[doc="Get the *const pointer for the FLTFCR register."]
    #[inline] pub fn fltfcr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltfcr { 
           self.fltfcr_mut(index)
    }

    #[doc="Read the FLTFCR register."]
    #[inline] pub fn fltfcr<I: Into<bits::R4>>(&self, index: I) -> Fltfcr { 
        unsafe {
            read_volatile(self.fltfcr_ptr(index))
        }
    }

    #[doc="Write the FLTFCR register."]
    #[inline] pub fn set_fltfcr<I: Into<bits::R4>, F: FnOnce(Fltfcr) -> Fltfcr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltfcr_mut(index), f(Fltfcr(0)));
        }
        self
    }

    #[doc="Modify the FLTFCR register."]
    #[inline] pub fn with_fltfcr<I: Into<bits::R4> + Copy, F: FnOnce(Fltfcr) -> Fltfcr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltfcr_mut(index), f(self.fltfcr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTJDATAR register."]
    #[inline] pub fn fltjdatar_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltjdatar { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x118 + (index * 128)) as *mut Fltjdatar
    }

    #[doc="Get the *const pointer for the FLTJDATAR register."]
    #[inline] pub fn fltjdatar_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltjdatar { 
           self.fltjdatar_mut(index)
    }

    #[doc="Read the FLTJDATAR register."]
    #[inline] pub fn fltjdatar<I: Into<bits::R4>>(&self, index: I) -> Fltjdatar { 
        unsafe {
            read_volatile(self.fltjdatar_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the FLTRDATAR register."]
    #[inline] pub fn fltrdatar_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltrdatar { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x11c + (index * 128)) as *mut Fltrdatar
    }

    #[doc="Get the *const pointer for the FLTRDATAR register."]
    #[inline] pub fn fltrdatar_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltrdatar { 
           self.fltrdatar_mut(index)
    }

    #[doc="Read the FLTRDATAR register."]
    #[inline] pub fn fltrdatar<I: Into<bits::R4>>(&self, index: I) -> Fltrdatar { 
        unsafe {
            read_volatile(self.fltrdatar_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the FLTAWHTR register."]
    #[inline] pub fn fltawhtr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltawhtr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x120 + (index * 128)) as *mut Fltawhtr
    }

    #[doc="Get the *const pointer for the FLTAWHTR register."]
    #[inline] pub fn fltawhtr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltawhtr { 
           self.fltawhtr_mut(index)
    }

    #[doc="Read the FLTAWHTR register."]
    #[inline] pub fn fltawhtr<I: Into<bits::R4>>(&self, index: I) -> Fltawhtr { 
        unsafe {
            read_volatile(self.fltawhtr_ptr(index))
        }
    }

    #[doc="Write the FLTAWHTR register."]
    #[inline] pub fn set_fltawhtr<I: Into<bits::R4>, F: FnOnce(Fltawhtr) -> Fltawhtr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltawhtr_mut(index), f(Fltawhtr(0)));
        }
        self
    }

    #[doc="Modify the FLTAWHTR register."]
    #[inline] pub fn with_fltawhtr<I: Into<bits::R4> + Copy, F: FnOnce(Fltawhtr) -> Fltawhtr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltawhtr_mut(index), f(self.fltawhtr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTAWLTR register."]
    #[inline] pub fn fltawltr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltawltr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x124 + (index * 128)) as *mut Fltawltr
    }

    #[doc="Get the *const pointer for the FLTAWLTR register."]
    #[inline] pub fn fltawltr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltawltr { 
           self.fltawltr_mut(index)
    }

    #[doc="Read the FLTAWLTR register."]
    #[inline] pub fn fltawltr<I: Into<bits::R4>>(&self, index: I) -> Fltawltr { 
        unsafe {
            read_volatile(self.fltawltr_ptr(index))
        }
    }

    #[doc="Write the FLTAWLTR register."]
    #[inline] pub fn set_fltawltr<I: Into<bits::R4>, F: FnOnce(Fltawltr) -> Fltawltr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltawltr_mut(index), f(Fltawltr(0)));
        }
        self
    }

    #[doc="Modify the FLTAWLTR register."]
    #[inline] pub fn with_fltawltr<I: Into<bits::R4> + Copy, F: FnOnce(Fltawltr) -> Fltawltr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltawltr_mut(index), f(self.fltawltr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTAWSR register."]
    #[inline] pub fn fltawsr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltawsr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x128 + (index * 128)) as *mut Fltawsr
    }

    #[doc="Get the *const pointer for the FLTAWSR register."]
    #[inline] pub fn fltawsr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltawsr { 
           self.fltawsr_mut(index)
    }

    #[doc="Read the FLTAWSR register."]
    #[inline] pub fn fltawsr<I: Into<bits::R4>>(&self, index: I) -> Fltawsr { 
        unsafe {
            read_volatile(self.fltawsr_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the FLTAWCFR register."]
    #[inline] pub fn fltawcfr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltawcfr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x12c + (index * 128)) as *mut Fltawcfr
    }

    #[doc="Get the *const pointer for the FLTAWCFR register."]
    #[inline] pub fn fltawcfr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltawcfr { 
           self.fltawcfr_mut(index)
    }

    #[doc="Read the FLTAWCFR register."]
    #[inline] pub fn fltawcfr<I: Into<bits::R4>>(&self, index: I) -> Fltawcfr { 
        unsafe {
            read_volatile(self.fltawcfr_ptr(index))
        }
    }

    #[doc="Write the FLTAWCFR register."]
    #[inline] pub fn set_fltawcfr<I: Into<bits::R4>, F: FnOnce(Fltawcfr) -> Fltawcfr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltawcfr_mut(index), f(Fltawcfr(0)));
        }
        self
    }

    #[doc="Modify the FLTAWCFR register."]
    #[inline] pub fn with_fltawcfr<I: Into<bits::R4> + Copy, F: FnOnce(Fltawcfr) -> Fltawcfr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltawcfr_mut(index), f(self.fltawcfr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTEXMAX register."]
    #[inline] pub fn fltexmax_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltexmax { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x130 + (index * 128)) as *mut Fltexmax
    }

    #[doc="Get the *const pointer for the FLTEXMAX register."]
    #[inline] pub fn fltexmax_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltexmax { 
           self.fltexmax_mut(index)
    }

    #[doc="Read the FLTEXMAX register."]
    #[inline] pub fn fltexmax<I: Into<bits::R4>>(&self, index: I) -> Fltexmax { 
        unsafe {
            read_volatile(self.fltexmax_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the FLTEXMIN register."]
    #[inline] pub fn fltexmin_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltexmin { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x134 + (index * 128)) as *mut Fltexmin
    }

    #[doc="Get the *const pointer for the FLTEXMIN register."]
    #[inline] pub fn fltexmin_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltexmin { 
           self.fltexmin_mut(index)
    }

    #[doc="Read the FLTEXMIN register."]
    #[inline] pub fn fltexmin<I: Into<bits::R4>>(&self, index: I) -> Fltexmin { 
        unsafe {
            read_volatile(self.fltexmin_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the FLTCNVTIMR register."]
    #[inline] pub fn fltcnvtimr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fltcnvtimr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x138 + (index * 128)) as *mut Fltcnvtimr
    }

    #[doc="Get the *const pointer for the FLTCNVTIMR register."]
    #[inline] pub fn fltcnvtimr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fltcnvtimr { 
           self.fltcnvtimr_mut(index)
    }

    #[doc="Read the FLTCNVTIMR register."]
    #[inline] pub fn fltcnvtimr<I: Into<bits::R4>>(&self, index: I) -> Fltcnvtimr { 
        unsafe {
            read_volatile(self.fltcnvtimr_ptr(index))
        }
    }

}

#[doc="channel configuration y register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfgr1(pub u32);
impl Chcfgr1 {
    #[doc="DFSDMEN"]
    #[inline] pub fn dfsdmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DFSDMEN != 0"]
    #[inline] pub fn test_dfsdmen(&self) -> bool {
        self.dfsdmen() != 0
    }

    #[doc="Sets the DFSDMEN field."]
    #[inline] pub fn set_dfsdmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="CKOUTSRC"]
    #[inline] pub fn ckoutsrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CKOUTSRC != 0"]
    #[inline] pub fn test_ckoutsrc(&self) -> bool {
        self.ckoutsrc() != 0
    }

    #[doc="Sets the CKOUTSRC field."]
    #[inline] pub fn set_ckoutsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="CKOUTDIV"]
    #[inline] pub fn ckoutdiv(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CKOUTDIV != 0"]
    #[inline] pub fn test_ckoutdiv(&self) -> bool {
        self.ckoutdiv() != 0
    }

    #[doc="Sets the CKOUTDIV field."]
    #[inline] pub fn set_ckoutdiv<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfgr1(other)
    }
}

impl ::core::fmt::Display for Chcfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dfsdmen() != 0 { try!(write!(f, " dfsdmen"))}
        if self.ckoutsrc() != 0 { try!(write!(f, " ckoutsrc"))}
        if self.ckoutdiv() != 0 { try!(write!(f, " ckoutdiv=0x{:x}", self.ckoutdiv()))}
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel configuration y register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfgr2(pub u32);
impl Chcfgr2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfgr2(other)
    }
}

impl ::core::fmt::Display for Chcfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog and short-circuit detector register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscdr(pub u32);
impl Awscdr {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscdr {
    #[inline]
    fn from(other: u32) -> Self {
         Awscdr(other)
    }
}

impl ::core::fmt::Display for Awscdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel watchdog filter data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdatr(pub u32);
impl Chwdatr {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdatr {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdatr(other)
    }
}

impl ::core::fmt::Display for Chwdatr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdatr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel data input register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatinr(pub u32);
impl Chdatinr {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatinr {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatinr(other)
    }
}

impl ::core::fmt::Display for Chdatinr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatinr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltcr1(pub u32);
impl Fltcr1 {
    #[doc="Analog watchdog fast mode select"]
    #[inline] pub fn awfsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if AWFSEL != 0"]
    #[inline] pub fn test_awfsel(&self) -> bool {
        self.awfsel() != 0
    }

    #[doc="Sets the AWFSEL field."]
    #[inline] pub fn set_awfsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Fast conversion mode selection for regular conversions"]
    #[inline] pub fn fast(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if FAST != 0"]
    #[inline] pub fn test_fast(&self) -> bool {
        self.fast() != 0
    }

    #[doc="Sets the FAST field."]
    #[inline] pub fn set_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Regular channel selection"]
    #[inline] pub fn rch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if RCH != 0"]
    #[inline] pub fn test_rch(&self) -> bool {
        self.rch() != 0
    }

    #[doc="Sets the RCH field."]
    #[inline] pub fn set_rch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DMA channel enabled to read data for the regular conversion"]
    #[inline] pub fn rdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RDMAEN != 0"]
    #[inline] pub fn test_rdmaen(&self) -> bool {
        self.rdmaen() != 0
    }

    #[doc="Sets the RDMAEN field."]
    #[inline] pub fn set_rdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Launch regular conversion synchronously with DFSDM0"]
    #[inline] pub fn rsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RSYNC != 0"]
    #[inline] pub fn test_rsync(&self) -> bool {
        self.rsync() != 0
    }

    #[doc="Sets the RSYNC field."]
    #[inline] pub fn set_rsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Continuous mode selection for regular conversions"]
    #[inline] pub fn rcont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Software start of a conversion on the regular channel"]
    #[inline] pub fn rswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RSWSTART != 0"]
    #[inline] pub fn test_rswstart(&self) -> bool {
        self.rswstart() != 0
    }

    #[doc="Sets the RSWSTART field."]
    #[inline] pub fn set_rswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Trigger enable and trigger edge selection for injected conversions"]
    #[inline] pub fn jexten(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if JEXTEN != 0"]
    #[inline] pub fn test_jexten(&self) -> bool {
        self.jexten() != 0
    }

    #[doc="Sets the JEXTEN field."]
    #[inline] pub fn set_jexten<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Trigger signal selection for launching injected conversions"]
    #[inline] pub fn jextsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if JEXTSEL != 0"]
    #[inline] pub fn test_jextsel(&self) -> bool {
        self.jextsel() != 0
    }

    #[doc="Sets the JEXTSEL field."]
    #[inline] pub fn set_jextsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA channel enabled to read data for the injected channel group"]
    #[inline] pub fn jdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JDMAEN != 0"]
    #[inline] pub fn test_jdmaen(&self) -> bool {
        self.jdmaen() != 0
    }

    #[doc="Sets the JDMAEN field."]
    #[inline] pub fn set_jdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Scanning conversion mode for injected conversions"]
    #[inline] pub fn jscan(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if JSCAN != 0"]
    #[inline] pub fn test_jscan(&self) -> bool {
        self.jscan() != 0
    }

    #[doc="Sets the JSCAN field."]
    #[inline] pub fn set_jscan<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline] pub fn jsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JSYNC != 0"]
    #[inline] pub fn test_jsync(&self) -> bool {
        self.jsync() != 0
    }

    #[doc="Sets the JSYNC field."]
    #[inline] pub fn set_jsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start a conversion of the injected group of channels"]
    #[inline] pub fn jswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if JSWSTART != 0"]
    #[inline] pub fn test_jswstart(&self) -> bool {
        self.jswstart() != 0
    }

    #[doc="Sets the JSWSTART field."]
    #[inline] pub fn set_jswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DFSDM enable"]
    #[inline] pub fn dfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DFEN != 0"]
    #[inline] pub fn test_dfen(&self) -> bool {
        self.dfen() != 0
    }

    #[doc="Sets the DFEN field."]
    #[inline] pub fn set_dfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltcr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Fltcr1(other)
    }
}

impl ::core::fmt::Display for Fltcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awfsel() != 0 { try!(write!(f, " awfsel"))}
        if self.fast() != 0 { try!(write!(f, " fast"))}
        if self.rch() != 0 { try!(write!(f, " rch=0x{:x}", self.rch()))}
        if self.rdmaen() != 0 { try!(write!(f, " rdmaen"))}
        if self.rsync() != 0 { try!(write!(f, " rsync"))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rswstart() != 0 { try!(write!(f, " rswstart"))}
        if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
        if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
        if self.jdmaen() != 0 { try!(write!(f, " jdmaen"))}
        if self.jscan() != 0 { try!(write!(f, " jscan"))}
        if self.jsync() != 0 { try!(write!(f, " jsync"))}
        if self.jswstart() != 0 { try!(write!(f, " jswstart"))}
        if self.dfen() != 0 { try!(write!(f, " dfen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltcr2(pub u32);
impl Fltcr2 {
    #[doc="Analog watchdog channel selection"]
    #[inline] pub fn awdch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if AWDCH != 0"]
    #[inline] pub fn test_awdch(&self) -> bool {
        self.awdch() != 0
    }

    #[doc="Sets the AWDCH field."]
    #[inline] pub fn set_awdch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Extremes detector channel selection"]
    #[inline] pub fn exch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if EXCH != 0"]
    #[inline] pub fn test_exch(&self) -> bool {
        self.exch() != 0
    }

    #[doc="Sets the EXCH field."]
    #[inline] pub fn set_exch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock absence interrupt enable"]
    #[inline] pub fn ckabie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABIE != 0"]
    #[inline] pub fn test_ckabie(&self) -> bool {
        self.ckabie() != 0
    }

    #[doc="Sets the CKABIE field."]
    #[inline] pub fn set_ckabie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Short-circuit detector interrupt enable"]
    #[inline] pub fn scdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDIE != 0"]
    #[inline] pub fn test_scdie(&self) -> bool {
        self.scdie() != 0
    }

    #[doc="Sets the SCDIE field."]
    #[inline] pub fn set_scdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Analog watchdog interrupt enable"]
    #[inline] pub fn awdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDIE != 0"]
    #[inline] pub fn test_awdie(&self) -> bool {
        self.awdie() != 0
    }

    #[doc="Sets the AWDIE field."]
    #[inline] pub fn set_awdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular data overrun interrupt enable"]
    #[inline] pub fn rovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRIE != 0"]
    #[inline] pub fn test_rovrie(&self) -> bool {
        self.rovrie() != 0
    }

    #[doc="Sets the ROVRIE field."]
    #[inline] pub fn set_rovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected data overrun interrupt enable"]
    #[inline] pub fn jovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRIE != 0"]
    #[inline] pub fn test_jovrie(&self) -> bool {
        self.jovrie() != 0
    }

    #[doc="Sets the JOVRIE field."]
    #[inline] pub fn set_jovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Regular end of conversion interrupt enable"]
    #[inline] pub fn reocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCIE != 0"]
    #[inline] pub fn test_reocie(&self) -> bool {
        self.reocie() != 0
    }

    #[doc="Sets the REOCIE field."]
    #[inline] pub fn set_reocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Injected end of conversion interrupt enable"]
    #[inline] pub fn jeocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCIE != 0"]
    #[inline] pub fn test_jeocie(&self) -> bool {
        self.jeocie() != 0
    }

    #[doc="Sets the JEOCIE field."]
    #[inline] pub fn set_jeocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltcr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Fltcr2(other)
    }
}

impl ::core::fmt::Display for Fltcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awdch() != 0 { try!(write!(f, " awdch=0x{:x}", self.awdch()))}
        if self.exch() != 0 { try!(write!(f, " exch=0x{:x}", self.exch()))}
        if self.ckabie() != 0 { try!(write!(f, " ckabie"))}
        if self.scdie() != 0 { try!(write!(f, " scdie"))}
        if self.awdie() != 0 { try!(write!(f, " awdie"))}
        if self.rovrie() != 0 { try!(write!(f, " rovrie"))}
        if self.jovrie() != 0 { try!(write!(f, " jovrie"))}
        if self.reocie() != 0 { try!(write!(f, " reocie"))}
        if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltisr(pub u32);
impl Fltisr {
    #[doc="short-circuit detector flag"]
    #[inline] pub fn scdf<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 24 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SCDF != 0"]
    #[inline] pub fn test_scdf<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.scdf(index) != 0
    }

    #[doc="Sets the SCDF field."]
    #[inline] pub fn set_scdf<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 24 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Clock absence flag"]
    #[inline] pub fn ckabf<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CKABF != 0"]
    #[inline] pub fn test_ckabf<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.ckabf(index) != 0
    }

    #[doc="Sets the CKABF field."]
    #[inline] pub fn set_ckabf<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Regular conversion in progress status"]
    #[inline] pub fn rcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCIP != 0"]
    #[inline] pub fn test_rcip(&self) -> bool {
        self.rcip() != 0
    }

    #[doc="Sets the RCIP field."]
    #[inline] pub fn set_rcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Injected conversion in progress status"]
    #[inline] pub fn jcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if JCIP != 0"]
    #[inline] pub fn test_jcip(&self) -> bool {
        self.jcip() != 0
    }

    #[doc="Sets the JCIP field."]
    #[inline] pub fn set_jcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Analog watchdog"]
    #[inline] pub fn awdf<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 4 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDF != 0"]
    #[inline] pub fn test_awdf<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.awdf(index) != 0
    }

    #[doc="Sets the AWDF field."]
    #[inline] pub fn set_awdf<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 4 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Regular conversion overrun flag"]
    #[inline] pub fn rovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRF != 0"]
    #[inline] pub fn test_rovrf(&self) -> bool {
        self.rovrf() != 0
    }

    #[doc="Sets the ROVRF field."]
    #[inline] pub fn set_rovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected conversion overrun flag"]
    #[inline] pub fn jovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRF != 0"]
    #[inline] pub fn test_jovrf(&self) -> bool {
        self.jovrf() != 0
    }

    #[doc="Sets the JOVRF field."]
    #[inline] pub fn set_jovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of regular conversion flag"]
    #[inline] pub fn reocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCF != 0"]
    #[inline] pub fn test_reocf(&self) -> bool {
        self.reocf() != 0
    }

    #[doc="Sets the REOCF field."]
    #[inline] pub fn set_reocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of injected conversion flag"]
    #[inline] pub fn jeocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCF != 0"]
    #[inline] pub fn test_jeocf(&self) -> bool {
        self.jeocf() != 0
    }

    #[doc="Sets the JEOCF field."]
    #[inline] pub fn set_jeocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltisr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltisr(other)
    }
}

impl ::core::fmt::Display for Fltisr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltisr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scdf(0) != 0 { try!(write!(f, " scdf[0]"))}
        if self.scdf(1) != 0 { try!(write!(f, " scdf[1]"))}
        if self.scdf(2) != 0 { try!(write!(f, " scdf[2]"))}
        if self.scdf(3) != 0 { try!(write!(f, " scdf[3]"))}
        if self.scdf(4) != 0 { try!(write!(f, " scdf[4]"))}
        if self.scdf(5) != 0 { try!(write!(f, " scdf[5]"))}
        if self.scdf(6) != 0 { try!(write!(f, " scdf[6]"))}
        if self.scdf(7) != 0 { try!(write!(f, " scdf[7]"))}
        if self.ckabf(0) != 0 { try!(write!(f, " ckabf[0]"))}
        if self.ckabf(1) != 0 { try!(write!(f, " ckabf[1]"))}
        if self.ckabf(2) != 0 { try!(write!(f, " ckabf[2]"))}
        if self.ckabf(3) != 0 { try!(write!(f, " ckabf[3]"))}
        if self.ckabf(4) != 0 { try!(write!(f, " ckabf[4]"))}
        if self.ckabf(5) != 0 { try!(write!(f, " ckabf[5]"))}
        if self.ckabf(6) != 0 { try!(write!(f, " ckabf[6]"))}
        if self.ckabf(7) != 0 { try!(write!(f, " ckabf[7]"))}
        if self.rcip() != 0 { try!(write!(f, " rcip"))}
        if self.jcip() != 0 { try!(write!(f, " jcip"))}
        if self.awdf(0) != 0 { try!(write!(f, " awdf[0]"))}
        if self.awdf(1) != 0 { try!(write!(f, " awdf[1]"))}
        if self.awdf(2) != 0 { try!(write!(f, " awdf[2]"))}
        if self.awdf(3) != 0 { try!(write!(f, " awdf[3]"))}
        if self.rovrf() != 0 { try!(write!(f, " rovrf"))}
        if self.jovrf() != 0 { try!(write!(f, " jovrf"))}
        if self.reocf() != 0 { try!(write!(f, " reocf"))}
        if self.jeocf() != 0 { try!(write!(f, " jeocf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flticr(pub u32);
impl Flticr {
    #[doc="Clear the short-circuit detector flag"]
    #[inline] pub fn clrscdf<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 24 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CLRSCDF != 0"]
    #[inline] pub fn test_clrscdf<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.clrscdf(index) != 0
    }

    #[doc="Sets the CLRSCDF field."]
    #[inline] pub fn set_clrscdf<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 24 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Clear the clock absence flag"]
    #[inline] pub fn clrckabf<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CLRCKABF != 0"]
    #[inline] pub fn test_clrckabf<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.clrckabf(index) != 0
    }

    #[doc="Sets the CLRCKABF field."]
    #[inline] pub fn set_clrckabf<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Clear the regular conversion overrun flag"]
    #[inline] pub fn clrrovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CLRROVRF != 0"]
    #[inline] pub fn test_clrrovrf(&self) -> bool {
        self.clrrovrf() != 0
    }

    #[doc="Sets the CLRROVRF field."]
    #[inline] pub fn set_clrrovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear the injected conversion overrun flag"]
    #[inline] pub fn clrjovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CLRJOVRF != 0"]
    #[inline] pub fn test_clrjovrf(&self) -> bool {
        self.clrjovrf() != 0
    }

    #[doc="Sets the CLRJOVRF field."]
    #[inline] pub fn set_clrjovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Flticr {
    #[inline]
    fn from(other: u32) -> Self {
         Flticr(other)
    }
}

impl ::core::fmt::Display for Flticr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flticr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrscdf(0) != 0 { try!(write!(f, " clrscdf[0]"))}
        if self.clrscdf(1) != 0 { try!(write!(f, " clrscdf[1]"))}
        if self.clrscdf(2) != 0 { try!(write!(f, " clrscdf[2]"))}
        if self.clrscdf(3) != 0 { try!(write!(f, " clrscdf[3]"))}
        if self.clrscdf(4) != 0 { try!(write!(f, " clrscdf[4]"))}
        if self.clrscdf(5) != 0 { try!(write!(f, " clrscdf[5]"))}
        if self.clrscdf(6) != 0 { try!(write!(f, " clrscdf[6]"))}
        if self.clrscdf(7) != 0 { try!(write!(f, " clrscdf[7]"))}
        if self.clrckabf(0) != 0 { try!(write!(f, " clrckabf[0]"))}
        if self.clrckabf(1) != 0 { try!(write!(f, " clrckabf[1]"))}
        if self.clrckabf(2) != 0 { try!(write!(f, " clrckabf[2]"))}
        if self.clrckabf(3) != 0 { try!(write!(f, " clrckabf[3]"))}
        if self.clrckabf(4) != 0 { try!(write!(f, " clrckabf[4]"))}
        if self.clrckabf(5) != 0 { try!(write!(f, " clrckabf[5]"))}
        if self.clrckabf(6) != 0 { try!(write!(f, " clrckabf[6]"))}
        if self.clrckabf(7) != 0 { try!(write!(f, " clrckabf[7]"))}
        if self.clrrovrf() != 0 { try!(write!(f, " clrrovrf"))}
        if self.clrjovrf() != 0 { try!(write!(f, " clrjovrf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected channel group selection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltjchgr(pub u32);
impl Fltjchgr {
    #[doc="Injected channel group selection"]
    #[inline] pub fn jchg(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if JCHG != 0"]
    #[inline] pub fn test_jchg(&self) -> bool {
        self.jchg() != 0
    }

    #[doc="Sets the JCHG field."]
    #[inline] pub fn set_jchg<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltjchgr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltjchgr(other)
    }
}

impl ::core::fmt::Display for Fltjchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltjchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jchg() != 0 { try!(write!(f, " jchg=0x{:x}", self.jchg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltfcr(pub u32);
impl Fltfcr {
    #[doc="Sinc filter order"]
    #[inline] pub fn ford(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if FORD != 0"]
    #[inline] pub fn test_ford(&self) -> bool {
        self.ford() != 0
    }

    #[doc="Sets the FORD field."]
    #[inline] pub fn set_ford<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Sinc filter oversampling ratio (decimation rate)"]
    #[inline] pub fn fosr(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if FOSR != 0"]
    #[inline] pub fn test_fosr(&self) -> bool {
        self.fosr() != 0
    }

    #[doc="Sets the FOSR field."]
    #[inline] pub fn set_fosr<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Integrator oversampling ratio (averaging length)"]
    #[inline] pub fn iosr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IOSR != 0"]
    #[inline] pub fn test_iosr(&self) -> bool {
        self.iosr() != 0
    }

    #[doc="Sets the IOSR field."]
    #[inline] pub fn set_iosr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltfcr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltfcr(other)
    }
}

impl ::core::fmt::Display for Fltfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ford() != 0 { try!(write!(f, " ford=0x{:x}", self.ford()))}
        if self.fosr() != 0 { try!(write!(f, " fosr=0x{:x}", self.fosr()))}
        if self.iosr() != 0 { try!(write!(f, " iosr=0x{:x}", self.iosr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for injected group"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltjdatar(pub u32);
impl Fltjdatar {
    #[doc="Injected group conversion data"]
    #[inline] pub fn jdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if JDATA != 0"]
    #[inline] pub fn test_jdata(&self) -> bool {
        self.jdata() != 0
    }

    #[doc="Sets the JDATA field."]
    #[inline] pub fn set_jdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Injected channel most recently converted"]
    #[inline] pub fn jdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if JDATACH != 0"]
    #[inline] pub fn test_jdatach(&self) -> bool {
        self.jdatach() != 0
    }

    #[doc="Sets the JDATACH field."]
    #[inline] pub fn set_jdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltjdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Fltjdatar(other)
    }
}

impl ::core::fmt::Display for Fltjdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltjdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
        if self.jdatach() != 0 { try!(write!(f, " jdatach=0x{:x}", self.jdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for the regular channel"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltrdatar(pub u32);
impl Fltrdatar {
    #[doc="Regular channel conversion data"]
    #[inline] pub fn rdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if RDATA != 0"]
    #[inline] pub fn test_rdata(&self) -> bool {
        self.rdata() != 0
    }

    #[doc="Sets the RDATA field."]
    #[inline] pub fn set_rdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Regular channel pending data"]
    #[inline] pub fn rpend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RPEND != 0"]
    #[inline] pub fn test_rpend(&self) -> bool {
        self.rpend() != 0
    }

    #[doc="Sets the RPEND field."]
    #[inline] pub fn set_rpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular channel most recently converted"]
    #[inline] pub fn rdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RDATACH != 0"]
    #[inline] pub fn test_rdatach(&self) -> bool {
        self.rdatach() != 0
    }

    #[doc="Sets the RDATACH field."]
    #[inline] pub fn set_rdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltrdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Fltrdatar(other)
    }
}

impl ::core::fmt::Display for Fltrdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltrdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdata() != 0 { try!(write!(f, " rdata=0x{:x}", self.rdata()))}
        if self.rpend() != 0 { try!(write!(f, " rpend"))}
        if self.rdatach() != 0 { try!(write!(f, " rdatach=0x{:x}", self.rdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog high threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltawhtr(pub u32);
impl Fltawhtr {
    #[doc="Analog watchdog high threshold"]
    #[inline] pub fn awht(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWHT != 0"]
    #[inline] pub fn test_awht(&self) -> bool {
        self.awht() != 0
    }

    #[doc="Sets the AWHT field."]
    #[inline] pub fn set_awht<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog high threshold event"]
    #[inline] pub fn bkawh(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWH != 0"]
    #[inline] pub fn test_bkawh(&self) -> bool {
        self.bkawh() != 0
    }

    #[doc="Sets the BKAWH field."]
    #[inline] pub fn set_bkawh<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltawhtr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltawhtr(other)
    }
}

impl ::core::fmt::Display for Fltawhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltawhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awht() != 0 { try!(write!(f, " awht=0x{:x}", self.awht()))}
        if self.bkawh() != 0 { try!(write!(f, " bkawh=0x{:x}", self.bkawh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog low threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltawltr(pub u32);
impl Fltawltr {
    #[doc="Analog watchdog low threshold"]
    #[inline] pub fn awlt(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWLT != 0"]
    #[inline] pub fn test_awlt(&self) -> bool {
        self.awlt() != 0
    }

    #[doc="Sets the AWLT field."]
    #[inline] pub fn set_awlt<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog low threshold event"]
    #[inline] pub fn bkawl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWL != 0"]
    #[inline] pub fn test_bkawl(&self) -> bool {
        self.bkawl() != 0
    }

    #[doc="Sets the BKAWL field."]
    #[inline] pub fn set_bkawl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltawltr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltawltr(other)
    }
}

impl ::core::fmt::Display for Fltawltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltawltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awlt() != 0 { try!(write!(f, " awlt=0x{:x}", self.awlt()))}
        if self.bkawl() != 0 { try!(write!(f, " bkawl=0x{:x}", self.bkawl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltawsr(pub u32);
impl Fltawsr {
    #[doc="Analog watchdog high threshold flag"]
    #[inline] pub fn awhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if AWHTF != 0"]
    #[inline] pub fn test_awhtf(&self) -> bool {
        self.awhtf() != 0
    }

    #[doc="Sets the AWHTF field."]
    #[inline] pub fn set_awhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Analog watchdog low threshold flag"]
    #[inline] pub fn awltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AWLTF != 0"]
    #[inline] pub fn test_awltf(&self) -> bool {
        self.awltf() != 0
    }

    #[doc="Sets the AWLTF field."]
    #[inline] pub fn set_awltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltawsr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltawsr(other)
    }
}

impl ::core::fmt::Display for Fltawsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltawsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awhtf() != 0 { try!(write!(f, " awhtf=0x{:x}", self.awhtf()))}
        if self.awltf() != 0 { try!(write!(f, " awltf=0x{:x}", self.awltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog clear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltawcfr(pub u32);
impl Fltawcfr {
    #[doc="Clear the analog watchdog high threshold flag"]
    #[inline] pub fn clrawhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CLRAWHTF != 0"]
    #[inline] pub fn test_clrawhtf(&self) -> bool {
        self.clrawhtf() != 0
    }

    #[doc="Sets the CLRAWHTF field."]
    #[inline] pub fn set_clrawhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear the analog watchdog low threshold flag"]
    #[inline] pub fn clrawltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLRAWLTF != 0"]
    #[inline] pub fn test_clrawltf(&self) -> bool {
        self.clrawltf() != 0
    }

    #[doc="Sets the CLRAWLTF field."]
    #[inline] pub fn set_clrawltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltawcfr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltawcfr(other)
    }
}

impl ::core::fmt::Display for Fltawcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltawcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrawhtf() != 0 { try!(write!(f, " clrawhtf=0x{:x}", self.clrawhtf()))}
        if self.clrawltf() != 0 { try!(write!(f, " clrawltf=0x{:x}", self.clrawltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector maximum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltexmax(pub u32);
impl Fltexmax {
    #[doc="Extremes detector maximum value"]
    #[inline] pub fn exmax(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMAX != 0"]
    #[inline] pub fn test_exmax(&self) -> bool {
        self.exmax() != 0
    }

    #[doc="Sets the EXMAX field."]
    #[inline] pub fn set_exmax<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector maximum data channel"]
    #[inline] pub fn exmaxch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMAXCH != 0"]
    #[inline] pub fn test_exmaxch(&self) -> bool {
        self.exmaxch() != 0
    }

    #[doc="Sets the EXMAXCH field."]
    #[inline] pub fn set_exmaxch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltexmax {
    #[inline]
    fn from(other: u32) -> Self {
         Fltexmax(other)
    }
}

impl ::core::fmt::Display for Fltexmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltexmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmax() != 0 { try!(write!(f, " exmax=0x{:x}", self.exmax()))}
        if self.exmaxch() != 0 { try!(write!(f, " exmaxch=0x{:x}", self.exmaxch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector minimum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltexmin(pub u32);
impl Fltexmin {
    #[doc="EXMIN"]
    #[inline] pub fn exmin(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMIN != 0"]
    #[inline] pub fn test_exmin(&self) -> bool {
        self.exmin() != 0
    }

    #[doc="Sets the EXMIN field."]
    #[inline] pub fn set_exmin<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector minimum data channel"]
    #[inline] pub fn exminch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMINCH != 0"]
    #[inline] pub fn test_exminch(&self) -> bool {
        self.exminch() != 0
    }

    #[doc="Sets the EXMINCH field."]
    #[inline] pub fn set_exminch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fltexmin {
    #[inline]
    fn from(other: u32) -> Self {
         Fltexmin(other)
    }
}

impl ::core::fmt::Display for Fltexmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltexmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmin() != 0 { try!(write!(f, " exmin=0x{:x}", self.exmin()))}
        if self.exminch() != 0 { try!(write!(f, " exminch=0x{:x}", self.exminch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="conversion timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltcnvtimr(pub u32);
impl Fltcnvtimr {
    #[doc="28-bit timer counting conversion time t = CNVCNT[27:0] / fFLTCKIN"]
    #[inline] pub fn cnvcnt(&self) -> bits::U28 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffffff) as u32) } // [31:4]
    }

    #[doc="Returns true if CNVCNT != 0"]
    #[inline] pub fn test_cnvcnt(&self) -> bool {
        self.cnvcnt() != 0
    }

    #[doc="Sets the CNVCNT field."]
    #[inline] pub fn set_cnvcnt<V: Into<bits::U28>>(mut self, value: V) -> Self {
        let value: bits::U28 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Fltcnvtimr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltcnvtimr(other)
    }
}

impl ::core::fmt::Display for Fltcnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltcnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnvcnt() != 0 { try!(write!(f, " cnvcnt=0x{:x}", self.cnvcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


pub trait IrqDfsdmFlt<T> {
    fn irq_dfsdm_flt(&self) -> T;
}

impl IrqDfsdmFlt<super::irq::IrqDfsdm1Flt0> for Dfsdm {
    fn irq_dfsdm_flt(&self) -> super::irq::IrqDfsdm1Flt0 { super::irq::IRQ_DFSDM1_FLT0 }
}

impl IrqDfsdmFlt<super::irq::IrqDfsdm2Flt1> for Dfsdm {
    fn irq_dfsdm_flt(&self) -> super::irq::IrqDfsdm2Flt1 { super::irq::IRQ_DFSDM2_FLT1 }
}

