
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="EDMA Peripheral"]
pub struct EdmaPeriph(pub usize); 

pub struct EdmaCh { pub periph: EdmaPeriph, pub index: usize }

impl EdmaPeriph {
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

    #[doc="Get the *mut pointer for the ES register."]
    #[inline] pub fn es_mut(&self) -> *mut Es { 
        (self.0 + 0x4) as *mut Es
    }

    #[doc="Get the *const pointer for the ES register."]
    #[inline] pub fn es_ptr(&self) -> *const Es { 
           self.es_mut()
    }

    #[doc="Read the ES register."]
    #[inline] pub fn es(&self) -> Es { 
        unsafe {
            read_volatile(self.es_ptr())
        }
    }

    #[doc="Get the *mut pointer for the ERQ register."]
    #[inline] pub fn erq_mut(&self) -> *mut Erq { 
        (self.0 + 0xc) as *mut Erq
    }

    #[doc="Get the *const pointer for the ERQ register."]
    #[inline] pub fn erq_ptr(&self) -> *const Erq { 
           self.erq_mut()
    }

    #[doc="Read the ERQ register."]
    #[inline] pub fn erq(&self) -> Erq { 
        unsafe {
            read_volatile(self.erq_ptr())
        }
    }

    #[doc="Write the ERQ register."]
    #[inline] pub fn set_erq<F: FnOnce(Erq) -> Erq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.erq_mut(), f(Erq(0)));
        }
        self
    }

    #[doc="Modify the ERQ register."]
    #[inline] pub fn with_erq<F: FnOnce(Erq) -> Erq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.erq_mut(), f(self.erq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EEI register."]
    #[inline] pub fn eei_mut(&self) -> *mut Eei { 
        (self.0 + 0x14) as *mut Eei
    }

    #[doc="Get the *const pointer for the EEI register."]
    #[inline] pub fn eei_ptr(&self) -> *const Eei { 
           self.eei_mut()
    }

    #[doc="Read the EEI register."]
    #[inline] pub fn eei(&self) -> Eei { 
        unsafe {
            read_volatile(self.eei_ptr())
        }
    }

    #[doc="Write the EEI register."]
    #[inline] pub fn set_eei<F: FnOnce(Eei) -> Eei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.eei_mut(), f(Eei(0)));
        }
        self
    }

    #[doc="Modify the EEI register."]
    #[inline] pub fn with_eei<F: FnOnce(Eei) -> Eei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.eei_mut(), f(self.eei()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CEEI register."]
    #[inline] pub fn ceei_mut(&self) -> *mut Ceei { 
        (self.0 + 0x18) as *mut Ceei
    }

    #[doc="Get the *const pointer for the CEEI register."]
    #[inline] pub fn ceei_ptr(&self) -> *const Ceei { 
           self.ceei_mut()
    }

    #[doc="Write the CEEI register."]
    #[inline] pub fn set_ceei<F: FnOnce(Ceei) -> Ceei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ceei_mut(), f(Ceei(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SEEI register."]
    #[inline] pub fn seei_mut(&self) -> *mut Seei { 
        (self.0 + 0x19) as *mut Seei
    }

    #[doc="Get the *const pointer for the SEEI register."]
    #[inline] pub fn seei_ptr(&self) -> *const Seei { 
           self.seei_mut()
    }

    #[doc="Write the SEEI register."]
    #[inline] pub fn set_seei<F: FnOnce(Seei) -> Seei>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.seei_mut(), f(Seei(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CERQ register."]
    #[inline] pub fn cerq_mut(&self) -> *mut Cerq { 
        (self.0 + 0x1a) as *mut Cerq
    }

    #[doc="Get the *const pointer for the CERQ register."]
    #[inline] pub fn cerq_ptr(&self) -> *const Cerq { 
           self.cerq_mut()
    }

    #[doc="Write the CERQ register."]
    #[inline] pub fn set_cerq<F: FnOnce(Cerq) -> Cerq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cerq_mut(), f(Cerq(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SERQ register."]
    #[inline] pub fn serq_mut(&self) -> *mut Serq { 
        (self.0 + 0x1b) as *mut Serq
    }

    #[doc="Get the *const pointer for the SERQ register."]
    #[inline] pub fn serq_ptr(&self) -> *const Serq { 
           self.serq_mut()
    }

    #[doc="Write the SERQ register."]
    #[inline] pub fn set_serq<F: FnOnce(Serq) -> Serq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.serq_mut(), f(Serq(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CDNE register."]
    #[inline] pub fn cdne_mut(&self) -> *mut Cdne { 
        (self.0 + 0x1c) as *mut Cdne
    }

    #[doc="Get the *const pointer for the CDNE register."]
    #[inline] pub fn cdne_ptr(&self) -> *const Cdne { 
           self.cdne_mut()
    }

    #[doc="Write the CDNE register."]
    #[inline] pub fn set_cdne<F: FnOnce(Cdne) -> Cdne>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cdne_mut(), f(Cdne(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSRT register."]
    #[inline] pub fn ssrt_mut(&self) -> *mut Ssrt { 
        (self.0 + 0x1d) as *mut Ssrt
    }

    #[doc="Get the *const pointer for the SSRT register."]
    #[inline] pub fn ssrt_ptr(&self) -> *const Ssrt { 
           self.ssrt_mut()
    }

    #[doc="Write the SSRT register."]
    #[inline] pub fn set_ssrt<F: FnOnce(Ssrt) -> Ssrt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssrt_mut(), f(Ssrt(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CERR register."]
    #[inline] pub fn cerr_mut(&self) -> *mut Cerr { 
        (self.0 + 0x1e) as *mut Cerr
    }

    #[doc="Get the *const pointer for the CERR register."]
    #[inline] pub fn cerr_ptr(&self) -> *const Cerr { 
           self.cerr_mut()
    }

    #[doc="Write the CERR register."]
    #[inline] pub fn set_cerr<F: FnOnce(Cerr) -> Cerr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cerr_mut(), f(Cerr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CINT register."]
    #[inline] pub fn cint_mut(&self) -> *mut Cint { 
        (self.0 + 0x1f) as *mut Cint
    }

    #[doc="Get the *const pointer for the CINT register."]
    #[inline] pub fn cint_ptr(&self) -> *const Cint { 
           self.cint_mut()
    }

    #[doc="Write the CINT register."]
    #[inline] pub fn set_cint<F: FnOnce(Cint) -> Cint>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cint_mut(), f(Cint(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT register."]
    #[inline] pub fn int_mut(&self) -> *mut Int { 
        (self.0 + 0x24) as *mut Int
    }

    #[doc="Get the *const pointer for the INT register."]
    #[inline] pub fn int_ptr(&self) -> *const Int { 
           self.int_mut()
    }

    #[doc="Read the INT register."]
    #[inline] pub fn int(&self) -> Int { 
        unsafe {
            read_volatile(self.int_ptr())
        }
    }

    #[doc="Write the INT register."]
    #[inline] pub fn set_int<F: FnOnce(Int) -> Int>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int_mut(), f(Int(0)));
        }
        self
    }

    #[doc="Modify the INT register."]
    #[inline] pub fn with_int<F: FnOnce(Int) -> Int>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int_mut(), f(self.int()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ERR register."]
    #[inline] pub fn err_mut(&self) -> *mut Err { 
        (self.0 + 0x2c) as *mut Err
    }

    #[doc="Get the *const pointer for the ERR register."]
    #[inline] pub fn err_ptr(&self) -> *const Err { 
           self.err_mut()
    }

    #[doc="Read the ERR register."]
    #[inline] pub fn err(&self) -> Err { 
        unsafe {
            read_volatile(self.err_ptr())
        }
    }

    #[doc="Write the ERR register."]
    #[inline] pub fn set_err<F: FnOnce(Err) -> Err>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.err_mut(), f(Err(0)));
        }
        self
    }

    #[doc="Modify the ERR register."]
    #[inline] pub fn with_err<F: FnOnce(Err) -> Err>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.err_mut(), f(self.err()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HRS register."]
    #[inline] pub fn hrs_mut(&self) -> *mut Hrs { 
        (self.0 + 0x34) as *mut Hrs
    }

    #[doc="Get the *const pointer for the HRS register."]
    #[inline] pub fn hrs_ptr(&self) -> *const Hrs { 
           self.hrs_mut()
    }

    #[doc="Read the HRS register."]
    #[inline] pub fn hrs(&self) -> Hrs { 
        unsafe {
            read_volatile(self.hrs_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DCHPRI register."]
    #[inline] pub fn dchpri_mut<I: Into<bits::R16>>(&self, index: I) -> *mut Dchpri { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x100 + (index)) as *mut Dchpri
    }

    #[doc="Get the *const pointer for the DCHPRI register."]
    #[inline] pub fn dchpri_ptr<I: Into<bits::R16>>(&self, index: I) -> *const Dchpri { 
           self.dchpri_mut(index)
    }

    #[doc="Read the DCHPRI register."]
    #[inline] pub fn dchpri<I: Into<bits::R16>>(&self, index: I) -> Dchpri { 
        unsafe {
            read_volatile(self.dchpri_ptr(index))
        }
    }

    #[doc="Write the DCHPRI register."]
    #[inline] pub fn set_dchpri<I: Into<bits::R16>, F: FnOnce(Dchpri) -> Dchpri>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.dchpri_mut(index), f(Dchpri(0)));
        }
        self
    }

    #[doc="Modify the DCHPRI register."]
    #[inline] pub fn with_dchpri<I: Into<bits::R16> + Copy, F: FnOnce(Dchpri) -> Dchpri>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.dchpri_mut(index), f(self.dchpri(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_SADDR register."]
    #[inline] pub fn tcd_saddr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdSaddr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1000 + (index * 32)) as *mut TcdSaddr
    }

    #[doc="Get the *const pointer for the TCD_SADDR register."]
    #[inline] pub fn tcd_saddr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdSaddr { 
           self.tcd_saddr_mut(index)
    }

    #[doc="Read the TCD_SADDR register."]
    #[inline] pub fn tcd_saddr<I: Into<bits::R16>>(&self, index: I) -> TcdSaddr { 
        unsafe {
            read_volatile(self.tcd_saddr_ptr(index))
        }
    }

    #[doc="Write the TCD_SADDR register."]
    #[inline] pub fn set_tcd_saddr<I: Into<bits::R16>, F: FnOnce(TcdSaddr) -> TcdSaddr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_saddr_mut(index), f(TcdSaddr(0)));
        }
        self
    }

    #[doc="Modify the TCD_SADDR register."]
    #[inline] pub fn with_tcd_saddr<I: Into<bits::R16> + Copy, F: FnOnce(TcdSaddr) -> TcdSaddr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_saddr_mut(index), f(self.tcd_saddr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_SOFF register."]
    #[inline] pub fn tcd_soff_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdSoff { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1004 + (index * 32)) as *mut TcdSoff
    }

    #[doc="Get the *const pointer for the TCD_SOFF register."]
    #[inline] pub fn tcd_soff_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdSoff { 
           self.tcd_soff_mut(index)
    }

    #[doc="Read the TCD_SOFF register."]
    #[inline] pub fn tcd_soff<I: Into<bits::R16>>(&self, index: I) -> TcdSoff { 
        unsafe {
            read_volatile(self.tcd_soff_ptr(index))
        }
    }

    #[doc="Write the TCD_SOFF register."]
    #[inline] pub fn set_tcd_soff<I: Into<bits::R16>, F: FnOnce(TcdSoff) -> TcdSoff>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_soff_mut(index), f(TcdSoff(0)));
        }
        self
    }

    #[doc="Modify the TCD_SOFF register."]
    #[inline] pub fn with_tcd_soff<I: Into<bits::R16> + Copy, F: FnOnce(TcdSoff) -> TcdSoff>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_soff_mut(index), f(self.tcd_soff(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_ATTR register."]
    #[inline] pub fn tcd_attr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdAttr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1006 + (index * 32)) as *mut TcdAttr
    }

    #[doc="Get the *const pointer for the TCD_ATTR register."]
    #[inline] pub fn tcd_attr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdAttr { 
           self.tcd_attr_mut(index)
    }

    #[doc="Read the TCD_ATTR register."]
    #[inline] pub fn tcd_attr<I: Into<bits::R16>>(&self, index: I) -> TcdAttr { 
        unsafe {
            read_volatile(self.tcd_attr_ptr(index))
        }
    }

    #[doc="Write the TCD_ATTR register."]
    #[inline] pub fn set_tcd_attr<I: Into<bits::R16>, F: FnOnce(TcdAttr) -> TcdAttr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_attr_mut(index), f(TcdAttr(0)));
        }
        self
    }

    #[doc="Modify the TCD_ATTR register."]
    #[inline] pub fn with_tcd_attr<I: Into<bits::R16> + Copy, F: FnOnce(TcdAttr) -> TcdAttr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_attr_mut(index), f(self.tcd_attr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_NBYTES_MLNO register."]
    #[inline] pub fn tcd_nbytes_mlno_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdNbytesMlno { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1008 + (index * 32)) as *mut TcdNbytesMlno
    }

    #[doc="Get the *const pointer for the TCD_NBYTES_MLNO register."]
    #[inline] pub fn tcd_nbytes_mlno_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdNbytesMlno { 
           self.tcd_nbytes_mlno_mut(index)
    }

    #[doc="Read the TCD_NBYTES_MLNO register."]
    #[inline] pub fn tcd_nbytes_mlno<I: Into<bits::R16>>(&self, index: I) -> TcdNbytesMlno { 
        unsafe {
            read_volatile(self.tcd_nbytes_mlno_ptr(index))
        }
    }

    #[doc="Write the TCD_NBYTES_MLNO register."]
    #[inline] pub fn set_tcd_nbytes_mlno<I: Into<bits::R16>, F: FnOnce(TcdNbytesMlno) -> TcdNbytesMlno>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_nbytes_mlno_mut(index), f(TcdNbytesMlno(0)));
        }
        self
    }

    #[doc="Modify the TCD_NBYTES_MLNO register."]
    #[inline] pub fn with_tcd_nbytes_mlno<I: Into<bits::R16> + Copy, F: FnOnce(TcdNbytesMlno) -> TcdNbytesMlno>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_nbytes_mlno_mut(index), f(self.tcd_nbytes_mlno(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_NBYTES_MLOFFNO register."]
    #[inline] pub fn tcd_nbytes_mloffno_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdNbytesMloffno { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1008 + (index * 32)) as *mut TcdNbytesMloffno
    }

    #[doc="Get the *const pointer for the TCD_NBYTES_MLOFFNO register."]
    #[inline] pub fn tcd_nbytes_mloffno_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdNbytesMloffno { 
           self.tcd_nbytes_mloffno_mut(index)
    }

    #[doc="Read the TCD_NBYTES_MLOFFNO register."]
    #[inline] pub fn tcd_nbytes_mloffno<I: Into<bits::R16>>(&self, index: I) -> TcdNbytesMloffno { 
        unsafe {
            read_volatile(self.tcd_nbytes_mloffno_ptr(index))
        }
    }

    #[doc="Write the TCD_NBYTES_MLOFFNO register."]
    #[inline] pub fn set_tcd_nbytes_mloffno<I: Into<bits::R16>, F: FnOnce(TcdNbytesMloffno) -> TcdNbytesMloffno>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_nbytes_mloffno_mut(index), f(TcdNbytesMloffno(0)));
        }
        self
    }

    #[doc="Modify the TCD_NBYTES_MLOFFNO register."]
    #[inline] pub fn with_tcd_nbytes_mloffno<I: Into<bits::R16> + Copy, F: FnOnce(TcdNbytesMloffno) -> TcdNbytesMloffno>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_nbytes_mloffno_mut(index), f(self.tcd_nbytes_mloffno(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_NBYTES_MLOFFYES register."]
    #[inline] pub fn tcd_nbytes_mloffyes_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdNbytesMloffyes { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1008 + (index * 32)) as *mut TcdNbytesMloffyes
    }

    #[doc="Get the *const pointer for the TCD_NBYTES_MLOFFYES register."]
    #[inline] pub fn tcd_nbytes_mloffyes_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdNbytesMloffyes { 
           self.tcd_nbytes_mloffyes_mut(index)
    }

    #[doc="Read the TCD_NBYTES_MLOFFYES register."]
    #[inline] pub fn tcd_nbytes_mloffyes<I: Into<bits::R16>>(&self, index: I) -> TcdNbytesMloffyes { 
        unsafe {
            read_volatile(self.tcd_nbytes_mloffyes_ptr(index))
        }
    }

    #[doc="Write the TCD_NBYTES_MLOFFYES register."]
    #[inline] pub fn set_tcd_nbytes_mloffyes<I: Into<bits::R16>, F: FnOnce(TcdNbytesMloffyes) -> TcdNbytesMloffyes>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_nbytes_mloffyes_mut(index), f(TcdNbytesMloffyes(0)));
        }
        self
    }

    #[doc="Modify the TCD_NBYTES_MLOFFYES register."]
    #[inline] pub fn with_tcd_nbytes_mloffyes<I: Into<bits::R16> + Copy, F: FnOnce(TcdNbytesMloffyes) -> TcdNbytesMloffyes>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_nbytes_mloffyes_mut(index), f(self.tcd_nbytes_mloffyes(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_SLAST register."]
    #[inline] pub fn tcd_slast_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdSlast { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x100c + (index * 32)) as *mut TcdSlast
    }

    #[doc="Get the *const pointer for the TCD_SLAST register."]
    #[inline] pub fn tcd_slast_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdSlast { 
           self.tcd_slast_mut(index)
    }

    #[doc="Read the TCD_SLAST register."]
    #[inline] pub fn tcd_slast<I: Into<bits::R16>>(&self, index: I) -> TcdSlast { 
        unsafe {
            read_volatile(self.tcd_slast_ptr(index))
        }
    }

    #[doc="Write the TCD_SLAST register."]
    #[inline] pub fn set_tcd_slast<I: Into<bits::R16>, F: FnOnce(TcdSlast) -> TcdSlast>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_slast_mut(index), f(TcdSlast(0)));
        }
        self
    }

    #[doc="Modify the TCD_SLAST register."]
    #[inline] pub fn with_tcd_slast<I: Into<bits::R16> + Copy, F: FnOnce(TcdSlast) -> TcdSlast>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_slast_mut(index), f(self.tcd_slast(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_DADDR register."]
    #[inline] pub fn tcd_daddr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdDaddr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1010 + (index * 32)) as *mut TcdDaddr
    }

    #[doc="Get the *const pointer for the TCD_DADDR register."]
    #[inline] pub fn tcd_daddr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdDaddr { 
           self.tcd_daddr_mut(index)
    }

    #[doc="Read the TCD_DADDR register."]
    #[inline] pub fn tcd_daddr<I: Into<bits::R16>>(&self, index: I) -> TcdDaddr { 
        unsafe {
            read_volatile(self.tcd_daddr_ptr(index))
        }
    }

    #[doc="Write the TCD_DADDR register."]
    #[inline] pub fn set_tcd_daddr<I: Into<bits::R16>, F: FnOnce(TcdDaddr) -> TcdDaddr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_daddr_mut(index), f(TcdDaddr(0)));
        }
        self
    }

    #[doc="Modify the TCD_DADDR register."]
    #[inline] pub fn with_tcd_daddr<I: Into<bits::R16> + Copy, F: FnOnce(TcdDaddr) -> TcdDaddr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_daddr_mut(index), f(self.tcd_daddr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_DOFF register."]
    #[inline] pub fn tcd_doff_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdDoff { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1014 + (index * 32)) as *mut TcdDoff
    }

    #[doc="Get the *const pointer for the TCD_DOFF register."]
    #[inline] pub fn tcd_doff_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdDoff { 
           self.tcd_doff_mut(index)
    }

    #[doc="Read the TCD_DOFF register."]
    #[inline] pub fn tcd_doff<I: Into<bits::R16>>(&self, index: I) -> TcdDoff { 
        unsafe {
            read_volatile(self.tcd_doff_ptr(index))
        }
    }

    #[doc="Write the TCD_DOFF register."]
    #[inline] pub fn set_tcd_doff<I: Into<bits::R16>, F: FnOnce(TcdDoff) -> TcdDoff>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_doff_mut(index), f(TcdDoff(0)));
        }
        self
    }

    #[doc="Modify the TCD_DOFF register."]
    #[inline] pub fn with_tcd_doff<I: Into<bits::R16> + Copy, F: FnOnce(TcdDoff) -> TcdDoff>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_doff_mut(index), f(self.tcd_doff(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_CITER_ELINKNO register."]
    #[inline] pub fn tcd_citer_elinkno_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdCiterElinkno { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1016 + (index * 32)) as *mut TcdCiterElinkno
    }

    #[doc="Get the *const pointer for the TCD_CITER_ELINKNO register."]
    #[inline] pub fn tcd_citer_elinkno_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdCiterElinkno { 
           self.tcd_citer_elinkno_mut(index)
    }

    #[doc="Read the TCD_CITER_ELINKNO register."]
    #[inline] pub fn tcd_citer_elinkno<I: Into<bits::R16>>(&self, index: I) -> TcdCiterElinkno { 
        unsafe {
            read_volatile(self.tcd_citer_elinkno_ptr(index))
        }
    }

    #[doc="Write the TCD_CITER_ELINKNO register."]
    #[inline] pub fn set_tcd_citer_elinkno<I: Into<bits::R16>, F: FnOnce(TcdCiterElinkno) -> TcdCiterElinkno>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_citer_elinkno_mut(index), f(TcdCiterElinkno(0)));
        }
        self
    }

    #[doc="Modify the TCD_CITER_ELINKNO register."]
    #[inline] pub fn with_tcd_citer_elinkno<I: Into<bits::R16> + Copy, F: FnOnce(TcdCiterElinkno) -> TcdCiterElinkno>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_citer_elinkno_mut(index), f(self.tcd_citer_elinkno(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_CITER_ELINKYES register."]
    #[inline] pub fn tcd_citer_elinkyes_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdCiterElinkyes { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1016 + (index * 32)) as *mut TcdCiterElinkyes
    }

    #[doc="Get the *const pointer for the TCD_CITER_ELINKYES register."]
    #[inline] pub fn tcd_citer_elinkyes_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdCiterElinkyes { 
           self.tcd_citer_elinkyes_mut(index)
    }

    #[doc="Read the TCD_CITER_ELINKYES register."]
    #[inline] pub fn tcd_citer_elinkyes<I: Into<bits::R16>>(&self, index: I) -> TcdCiterElinkyes { 
        unsafe {
            read_volatile(self.tcd_citer_elinkyes_ptr(index))
        }
    }

    #[doc="Write the TCD_CITER_ELINKYES register."]
    #[inline] pub fn set_tcd_citer_elinkyes<I: Into<bits::R16>, F: FnOnce(TcdCiterElinkyes) -> TcdCiterElinkyes>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_citer_elinkyes_mut(index), f(TcdCiterElinkyes(0)));
        }
        self
    }

    #[doc="Modify the TCD_CITER_ELINKYES register."]
    #[inline] pub fn with_tcd_citer_elinkyes<I: Into<bits::R16> + Copy, F: FnOnce(TcdCiterElinkyes) -> TcdCiterElinkyes>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_citer_elinkyes_mut(index), f(self.tcd_citer_elinkyes(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_DLASTSGA register."]
    #[inline] pub fn tcd_dlastsga_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdDlastsga { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1018 + (index * 32)) as *mut TcdDlastsga
    }

    #[doc="Get the *const pointer for the TCD_DLASTSGA register."]
    #[inline] pub fn tcd_dlastsga_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdDlastsga { 
           self.tcd_dlastsga_mut(index)
    }

    #[doc="Read the TCD_DLASTSGA register."]
    #[inline] pub fn tcd_dlastsga<I: Into<bits::R16>>(&self, index: I) -> TcdDlastsga { 
        unsafe {
            read_volatile(self.tcd_dlastsga_ptr(index))
        }
    }

    #[doc="Write the TCD_DLASTSGA register."]
    #[inline] pub fn set_tcd_dlastsga<I: Into<bits::R16>, F: FnOnce(TcdDlastsga) -> TcdDlastsga>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_dlastsga_mut(index), f(TcdDlastsga(0)));
        }
        self
    }

    #[doc="Modify the TCD_DLASTSGA register."]
    #[inline] pub fn with_tcd_dlastsga<I: Into<bits::R16> + Copy, F: FnOnce(TcdDlastsga) -> TcdDlastsga>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_dlastsga_mut(index), f(self.tcd_dlastsga(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_CSR register."]
    #[inline] pub fn tcd_csr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdCsr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x101c + (index * 32)) as *mut TcdCsr
    }

    #[doc="Get the *const pointer for the TCD_CSR register."]
    #[inline] pub fn tcd_csr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdCsr { 
           self.tcd_csr_mut(index)
    }

    #[doc="Read the TCD_CSR register."]
    #[inline] pub fn tcd_csr<I: Into<bits::R16>>(&self, index: I) -> TcdCsr { 
        unsafe {
            read_volatile(self.tcd_csr_ptr(index))
        }
    }

    #[doc="Write the TCD_CSR register."]
    #[inline] pub fn set_tcd_csr<I: Into<bits::R16>, F: FnOnce(TcdCsr) -> TcdCsr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_csr_mut(index), f(TcdCsr(0)));
        }
        self
    }

    #[doc="Modify the TCD_CSR register."]
    #[inline] pub fn with_tcd_csr<I: Into<bits::R16> + Copy, F: FnOnce(TcdCsr) -> TcdCsr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_csr_mut(index), f(self.tcd_csr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_BITER_ELINKNO register."]
    #[inline] pub fn tcd_biter_elinkno_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdBiterElinkno { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x101e + (index * 32)) as *mut TcdBiterElinkno
    }

    #[doc="Get the *const pointer for the TCD_BITER_ELINKNO register."]
    #[inline] pub fn tcd_biter_elinkno_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdBiterElinkno { 
           self.tcd_biter_elinkno_mut(index)
    }

    #[doc="Read the TCD_BITER_ELINKNO register."]
    #[inline] pub fn tcd_biter_elinkno<I: Into<bits::R16>>(&self, index: I) -> TcdBiterElinkno { 
        unsafe {
            read_volatile(self.tcd_biter_elinkno_ptr(index))
        }
    }

    #[doc="Write the TCD_BITER_ELINKNO register."]
    #[inline] pub fn set_tcd_biter_elinkno<I: Into<bits::R16>, F: FnOnce(TcdBiterElinkno) -> TcdBiterElinkno>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_biter_elinkno_mut(index), f(TcdBiterElinkno(0)));
        }
        self
    }

    #[doc="Modify the TCD_BITER_ELINKNO register."]
    #[inline] pub fn with_tcd_biter_elinkno<I: Into<bits::R16> + Copy, F: FnOnce(TcdBiterElinkno) -> TcdBiterElinkno>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_biter_elinkno_mut(index), f(self.tcd_biter_elinkno(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCD_BITER_ELINKYES register."]
    #[inline] pub fn tcd_biter_elinkyes_mut<I: Into<bits::R16>>(&self, index: I) -> *mut TcdBiterElinkyes { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x101e + (index * 32)) as *mut TcdBiterElinkyes
    }

    #[doc="Get the *const pointer for the TCD_BITER_ELINKYES register."]
    #[inline] pub fn tcd_biter_elinkyes_ptr<I: Into<bits::R16>>(&self, index: I) -> *const TcdBiterElinkyes { 
           self.tcd_biter_elinkyes_mut(index)
    }

    #[doc="Read the TCD_BITER_ELINKYES register."]
    #[inline] pub fn tcd_biter_elinkyes<I: Into<bits::R16>>(&self, index: I) -> TcdBiterElinkyes { 
        unsafe {
            read_volatile(self.tcd_biter_elinkyes_ptr(index))
        }
    }

    #[doc="Write the TCD_BITER_ELINKYES register."]
    #[inline] pub fn set_tcd_biter_elinkyes<I: Into<bits::R16>, F: FnOnce(TcdBiterElinkyes) -> TcdBiterElinkyes>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_biter_elinkyes_mut(index), f(TcdBiterElinkyes(0)));
        }
        self
    }

    #[doc="Modify the TCD_BITER_ELINKYES register."]
    #[inline] pub fn with_tcd_biter_elinkyes<I: Into<bits::R16> + Copy, F: FnOnce(TcdBiterElinkyes) -> TcdBiterElinkyes>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcd_biter_elinkyes_mut(index), f(self.tcd_biter_elinkyes(index)));
        }
        self
    }

}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Enable Debug"]
    #[inline] pub fn edbg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EDBG != 0"]
    #[inline] pub fn test_edbg(&self) -> bool {
        self.edbg() != 0
    }

    #[doc="Sets the EDBG field."]
    #[inline] pub fn set_edbg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable Round Robin Channel Arbitration"]
    #[inline] pub fn erca(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERCA != 0"]
    #[inline] pub fn test_erca(&self) -> bool {
        self.erca() != 0
    }

    #[doc="Sets the ERCA field."]
    #[inline] pub fn set_erca<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Halt On Error"]
    #[inline] pub fn hoe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HOE != 0"]
    #[inline] pub fn test_hoe(&self) -> bool {
        self.hoe() != 0
    }

    #[doc="Sets the HOE field."]
    #[inline] pub fn set_hoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Halt DMA Operations"]
    #[inline] pub fn halt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if HALT != 0"]
    #[inline] pub fn test_halt(&self) -> bool {
        self.halt() != 0
    }

    #[doc="Sets the HALT field."]
    #[inline] pub fn set_halt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Continuous Link Mode"]
    #[inline] pub fn clm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CLM != 0"]
    #[inline] pub fn test_clm(&self) -> bool {
        self.clm() != 0
    }

    #[doc="Sets the CLM field."]
    #[inline] pub fn set_clm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Enable Minor Loop Mapping"]
    #[inline] pub fn emlm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EMLM != 0"]
    #[inline] pub fn test_emlm(&self) -> bool {
        self.emlm() != 0
    }

    #[doc="Sets the EMLM field."]
    #[inline] pub fn set_emlm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Error Cancel Transfer"]
    #[inline] pub fn ecx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ECX != 0"]
    #[inline] pub fn test_ecx(&self) -> bool {
        self.ecx() != 0
    }

    #[doc="Sets the ECX field."]
    #[inline] pub fn set_ecx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Cancel Transfer"]
    #[inline] pub fn cx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CX != 0"]
    #[inline] pub fn test_cx(&self) -> bool {
        self.cx() != 0
    }

    #[doc="Sets the CX field."]
    #[inline] pub fn set_cx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
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
        if self.edbg() != 0 { try!(write!(f, " edbg"))}
        if self.erca() != 0 { try!(write!(f, " erca"))}
        if self.hoe() != 0 { try!(write!(f, " hoe"))}
        if self.halt() != 0 { try!(write!(f, " halt"))}
        if self.clm() != 0 { try!(write!(f, " clm"))}
        if self.emlm() != 0 { try!(write!(f, " emlm"))}
        if self.ecx() != 0 { try!(write!(f, " ecx"))}
        if self.cx() != 0 { try!(write!(f, " cx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Es(pub u32);
impl Es {
    #[doc="Destination Bus Error"]
    #[inline] pub fn dbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBE != 0"]
    #[inline] pub fn test_dbe(&self) -> bool {
        self.dbe() != 0
    }

    #[doc="Sets the DBE field."]
    #[inline] pub fn set_dbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Source Bus Error"]
    #[inline] pub fn sbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SBE != 0"]
    #[inline] pub fn test_sbe(&self) -> bool {
        self.sbe() != 0
    }

    #[doc="Sets the SBE field."]
    #[inline] pub fn set_sbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Scatter/Gather Configuration Error"]
    #[inline] pub fn sge(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SGE != 0"]
    #[inline] pub fn test_sge(&self) -> bool {
        self.sge() != 0
    }

    #[doc="Sets the SGE field."]
    #[inline] pub fn set_sge<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="NBYTES/CITER Configuration Error"]
    #[inline] pub fn nce(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if NCE != 0"]
    #[inline] pub fn test_nce(&self) -> bool {
        self.nce() != 0
    }

    #[doc="Sets the NCE field."]
    #[inline] pub fn set_nce<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Destination Offset Error"]
    #[inline] pub fn doe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DOE != 0"]
    #[inline] pub fn test_doe(&self) -> bool {
        self.doe() != 0
    }

    #[doc="Sets the DOE field."]
    #[inline] pub fn set_doe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Destination Address Error"]
    #[inline] pub fn dae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DAE != 0"]
    #[inline] pub fn test_dae(&self) -> bool {
        self.dae() != 0
    }

    #[doc="Sets the DAE field."]
    #[inline] pub fn set_dae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Source Offset Error"]
    #[inline] pub fn soe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SOE != 0"]
    #[inline] pub fn test_soe(&self) -> bool {
        self.soe() != 0
    }

    #[doc="Sets the SOE field."]
    #[inline] pub fn set_soe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Source Address Error"]
    #[inline] pub fn sae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SAE != 0"]
    #[inline] pub fn test_sae(&self) -> bool {
        self.sae() != 0
    }

    #[doc="Sets the SAE field."]
    #[inline] pub fn set_sae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Error Channel Number or Canceled Channel Number"]
    #[inline] pub fn errchn(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if ERRCHN != 0"]
    #[inline] pub fn test_errchn(&self) -> bool {
        self.errchn() != 0
    }

    #[doc="Sets the ERRCHN field."]
    #[inline] pub fn set_errchn<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Channel Priority Error"]
    #[inline] pub fn cpe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CPE != 0"]
    #[inline] pub fn test_cpe(&self) -> bool {
        self.cpe() != 0
    }

    #[doc="Sets the CPE field."]
    #[inline] pub fn set_cpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Transfer Canceled"]
    #[inline] pub fn ecx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ECX != 0"]
    #[inline] pub fn test_ecx(&self) -> bool {
        self.ecx() != 0
    }

    #[doc="Sets the ECX field."]
    #[inline] pub fn set_ecx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Logical OR of all ERR status bits"]
    #[inline] pub fn vld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if VLD != 0"]
    #[inline] pub fn test_vld(&self) -> bool {
        self.vld() != 0
    }

    #[doc="Sets the VLD field."]
    #[inline] pub fn set_vld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Es {
    #[inline]
    fn from(other: u32) -> Self {
         Es(other)
    }
}

impl ::core::fmt::Display for Es {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Es {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbe() != 0 { try!(write!(f, " dbe"))}
        if self.sbe() != 0 { try!(write!(f, " sbe"))}
        if self.sge() != 0 { try!(write!(f, " sge"))}
        if self.nce() != 0 { try!(write!(f, " nce"))}
        if self.doe() != 0 { try!(write!(f, " doe"))}
        if self.dae() != 0 { try!(write!(f, " dae"))}
        if self.soe() != 0 { try!(write!(f, " soe"))}
        if self.sae() != 0 { try!(write!(f, " sae"))}
        if self.errchn() != 0 { try!(write!(f, " errchn=0x{:x}", self.errchn()))}
        if self.cpe() != 0 { try!(write!(f, " cpe"))}
        if self.ecx() != 0 { try!(write!(f, " ecx"))}
        if self.vld() != 0 { try!(write!(f, " vld"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Enable Request Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Erq(pub u32);
impl Erq {
    #[doc="Enable DMA Request n"]
    #[inline] pub fn erq<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ERQ != 0"]
    #[inline] pub fn test_erq<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.erq(index) != 0
    }

    #[doc="Sets the ERQ field."]
    #[inline] pub fn set_erq<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Erq {
    #[inline]
    fn from(other: u32) -> Self {
         Erq(other)
    }
}

impl ::core::fmt::Display for Erq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Erq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.erq(0) != 0 { try!(write!(f, " erq[0]"))}
        if self.erq(1) != 0 { try!(write!(f, " erq[1]"))}
        if self.erq(2) != 0 { try!(write!(f, " erq[2]"))}
        if self.erq(3) != 0 { try!(write!(f, " erq[3]"))}
        if self.erq(4) != 0 { try!(write!(f, " erq[4]"))}
        if self.erq(5) != 0 { try!(write!(f, " erq[5]"))}
        if self.erq(6) != 0 { try!(write!(f, " erq[6]"))}
        if self.erq(7) != 0 { try!(write!(f, " erq[7]"))}
        if self.erq(8) != 0 { try!(write!(f, " erq[8]"))}
        if self.erq(9) != 0 { try!(write!(f, " erq[9]"))}
        if self.erq(10) != 0 { try!(write!(f, " erq[10]"))}
        if self.erq(11) != 0 { try!(write!(f, " erq[11]"))}
        if self.erq(12) != 0 { try!(write!(f, " erq[12]"))}
        if self.erq(13) != 0 { try!(write!(f, " erq[13]"))}
        if self.erq(14) != 0 { try!(write!(f, " erq[14]"))}
        if self.erq(15) != 0 { try!(write!(f, " erq[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Enable Error Interrupt Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eei(pub u32);
impl Eei {
    #[doc="Enable Error Interrupt No"]
    #[inline] pub fn eei<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EEI != 0"]
    #[inline] pub fn test_eei<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.eei(index) != 0
    }

    #[doc="Sets the EEI field."]
    #[inline] pub fn set_eei<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Eei {
    #[inline]
    fn from(other: u32) -> Self {
         Eei(other)
    }
}

impl ::core::fmt::Display for Eei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eei(0) != 0 { try!(write!(f, " eei[0]"))}
        if self.eei(1) != 0 { try!(write!(f, " eei[1]"))}
        if self.eei(2) != 0 { try!(write!(f, " eei[2]"))}
        if self.eei(3) != 0 { try!(write!(f, " eei[3]"))}
        if self.eei(4) != 0 { try!(write!(f, " eei[4]"))}
        if self.eei(5) != 0 { try!(write!(f, " eei[5]"))}
        if self.eei(6) != 0 { try!(write!(f, " eei[6]"))}
        if self.eei(7) != 0 { try!(write!(f, " eei[7]"))}
        if self.eei(8) != 0 { try!(write!(f, " eei[8]"))}
        if self.eei(9) != 0 { try!(write!(f, " eei[9]"))}
        if self.eei(10) != 0 { try!(write!(f, " eei[10]"))}
        if self.eei(11) != 0 { try!(write!(f, " eei[11]"))}
        if self.eei(12) != 0 { try!(write!(f, " eei[12]"))}
        if self.eei(13) != 0 { try!(write!(f, " eei[13]"))}
        if self.eei(14) != 0 { try!(write!(f, " eei[14]"))}
        if self.eei(15) != 0 { try!(write!(f, " eei[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clear Enable Error Interrupt Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ceei(pub u8);
impl Ceei {
    #[doc="Clear Enable Error Interrupt"]
    #[inline] pub fn ceei(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CEEI != 0"]
    #[inline] pub fn test_ceei(&self) -> bool {
        self.ceei() != 0
    }

    #[doc="Sets the CEEI field."]
    #[inline] pub fn set_ceei<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear All Enable Error Interrupts"]
    #[inline] pub fn caee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CAEE != 0"]
    #[inline] pub fn test_caee(&self) -> bool {
        self.caee() != 0
    }

    #[doc="Sets the CAEE field."]
    #[inline] pub fn set_caee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="No Op enable"]
    #[inline] pub fn nop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOP != 0"]
    #[inline] pub fn test_nop(&self) -> bool {
        self.nop() != 0
    }

    #[doc="Sets the NOP field."]
    #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ceei {
    #[inline]
    fn from(other: u8) -> Self {
         Ceei(other)
    }
}

impl ::core::fmt::Display for Ceei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ceei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ceei() != 0 { try!(write!(f, " ceei=0x{:x}", self.ceei()))}
        if self.caee() != 0 { try!(write!(f, " caee"))}
        if self.nop() != 0 { try!(write!(f, " nop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Set Enable Error Interrupt Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Seei(pub u8);
impl Seei {
    #[doc="Set Enable Error Interrupt"]
    #[inline] pub fn seei(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SEEI != 0"]
    #[inline] pub fn test_seei(&self) -> bool {
        self.seei() != 0
    }

    #[doc="Sets the SEEI field."]
    #[inline] pub fn set_seei<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Sets All Enable Error Interrupts"]
    #[inline] pub fn saee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SAEE != 0"]
    #[inline] pub fn test_saee(&self) -> bool {
        self.saee() != 0
    }

    #[doc="Sets the SAEE field."]
    #[inline] pub fn set_saee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="No Op enable"]
    #[inline] pub fn nop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOP != 0"]
    #[inline] pub fn test_nop(&self) -> bool {
        self.nop() != 0
    }

    #[doc="Sets the NOP field."]
    #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Seei {
    #[inline]
    fn from(other: u8) -> Self {
         Seei(other)
    }
}

impl ::core::fmt::Display for Seei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Seei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.seei() != 0 { try!(write!(f, " seei=0x{:x}", self.seei()))}
        if self.saee() != 0 { try!(write!(f, " saee"))}
        if self.nop() != 0 { try!(write!(f, " nop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clear Enable Request Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cerq(pub u8);
impl Cerq {
    #[doc="Clear Enable Request"]
    #[inline] pub fn cerq(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CERQ != 0"]
    #[inline] pub fn test_cerq(&self) -> bool {
        self.cerq() != 0
    }

    #[doc="Sets the CERQ field."]
    #[inline] pub fn set_cerq<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear All Enable Requests"]
    #[inline] pub fn caer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CAER != 0"]
    #[inline] pub fn test_caer(&self) -> bool {
        self.caer() != 0
    }

    #[doc="Sets the CAER field."]
    #[inline] pub fn set_caer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="No Op enable"]
    #[inline] pub fn nop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOP != 0"]
    #[inline] pub fn test_nop(&self) -> bool {
        self.nop() != 0
    }

    #[doc="Sets the NOP field."]
    #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cerq {
    #[inline]
    fn from(other: u8) -> Self {
         Cerq(other)
    }
}

impl ::core::fmt::Display for Cerq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cerq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cerq() != 0 { try!(write!(f, " cerq=0x{:x}", self.cerq()))}
        if self.caer() != 0 { try!(write!(f, " caer"))}
        if self.nop() != 0 { try!(write!(f, " nop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Set Enable Request Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Serq(pub u8);
impl Serq {
    #[doc="Set enable request"]
    #[inline] pub fn serq(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SERQ != 0"]
    #[inline] pub fn test_serq(&self) -> bool {
        self.serq() != 0
    }

    #[doc="Sets the SERQ field."]
    #[inline] pub fn set_serq<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Set All Enable Requests"]
    #[inline] pub fn saer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SAER != 0"]
    #[inline] pub fn test_saer(&self) -> bool {
        self.saer() != 0
    }

    #[doc="Sets the SAER field."]
    #[inline] pub fn set_saer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="No Op enable"]
    #[inline] pub fn nop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOP != 0"]
    #[inline] pub fn test_nop(&self) -> bool {
        self.nop() != 0
    }

    #[doc="Sets the NOP field."]
    #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Serq {
    #[inline]
    fn from(other: u8) -> Self {
         Serq(other)
    }
}

impl ::core::fmt::Display for Serq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Serq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.serq() != 0 { try!(write!(f, " serq=0x{:x}", self.serq()))}
        if self.saer() != 0 { try!(write!(f, " saer"))}
        if self.nop() != 0 { try!(write!(f, " nop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clear DONE Status Bit Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cdne(pub u8);
impl Cdne {
    #[doc="Clear DONE Bit"]
    #[inline] pub fn cdne(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CDNE != 0"]
    #[inline] pub fn test_cdne(&self) -> bool {
        self.cdne() != 0
    }

    #[doc="Sets the CDNE field."]
    #[inline] pub fn set_cdne<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clears All DONE Bits"]
    #[inline] pub fn cadn(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CADN != 0"]
    #[inline] pub fn test_cadn(&self) -> bool {
        self.cadn() != 0
    }

    #[doc="Sets the CADN field."]
    #[inline] pub fn set_cadn<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="No Op enable"]
    #[inline] pub fn nop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOP != 0"]
    #[inline] pub fn test_nop(&self) -> bool {
        self.nop() != 0
    }

    #[doc="Sets the NOP field."]
    #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cdne {
    #[inline]
    fn from(other: u8) -> Self {
         Cdne(other)
    }
}

impl ::core::fmt::Display for Cdne {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cdne {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cdne() != 0 { try!(write!(f, " cdne=0x{:x}", self.cdne()))}
        if self.cadn() != 0 { try!(write!(f, " cadn"))}
        if self.nop() != 0 { try!(write!(f, " nop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Set START Bit Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssrt(pub u8);
impl Ssrt {
    #[doc="Set START Bit"]
    #[inline] pub fn ssrt(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SSRT != 0"]
    #[inline] pub fn test_ssrt(&self) -> bool {
        self.ssrt() != 0
    }

    #[doc="Sets the SSRT field."]
    #[inline] pub fn set_ssrt<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Set All START Bits (activates all channels)"]
    #[inline] pub fn sast(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SAST != 0"]
    #[inline] pub fn test_sast(&self) -> bool {
        self.sast() != 0
    }

    #[doc="Sets the SAST field."]
    #[inline] pub fn set_sast<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="No Op enable"]
    #[inline] pub fn nop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOP != 0"]
    #[inline] pub fn test_nop(&self) -> bool {
        self.nop() != 0
    }

    #[doc="Sets the NOP field."]
    #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ssrt {
    #[inline]
    fn from(other: u8) -> Self {
         Ssrt(other)
    }
}

impl ::core::fmt::Display for Ssrt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssrt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ssrt() != 0 { try!(write!(f, " ssrt=0x{:x}", self.ssrt()))}
        if self.sast() != 0 { try!(write!(f, " sast"))}
        if self.nop() != 0 { try!(write!(f, " nop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clear Error Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cerr(pub u8);
impl Cerr {
    #[doc="Clear Error Indicator"]
    #[inline] pub fn cerr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CERR != 0"]
    #[inline] pub fn test_cerr(&self) -> bool {
        self.cerr() != 0
    }

    #[doc="Sets the CERR field."]
    #[inline] pub fn set_cerr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear All Error Indicators"]
    #[inline] pub fn caei(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CAEI != 0"]
    #[inline] pub fn test_caei(&self) -> bool {
        self.caei() != 0
    }

    #[doc="Sets the CAEI field."]
    #[inline] pub fn set_caei<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="No Op enable"]
    #[inline] pub fn nop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOP != 0"]
    #[inline] pub fn test_nop(&self) -> bool {
        self.nop() != 0
    }

    #[doc="Sets the NOP field."]
    #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cerr {
    #[inline]
    fn from(other: u8) -> Self {
         Cerr(other)
    }
}

impl ::core::fmt::Display for Cerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cerr() != 0 { try!(write!(f, " cerr=0x{:x}", self.cerr()))}
        if self.caei() != 0 { try!(write!(f, " caei"))}
        if self.nop() != 0 { try!(write!(f, " nop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clear Interrupt Request Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cint(pub u8);
impl Cint {
    #[doc="Clear Interrupt Request"]
    #[inline] pub fn cint(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CINT != 0"]
    #[inline] pub fn test_cint(&self) -> bool {
        self.cint() != 0
    }

    #[doc="Sets the CINT field."]
    #[inline] pub fn set_cint<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear All Interrupt Requests"]
    #[inline] pub fn cair(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CAIR != 0"]
    #[inline] pub fn test_cair(&self) -> bool {
        self.cair() != 0
    }

    #[doc="Sets the CAIR field."]
    #[inline] pub fn set_cair<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="No Op enable"]
    #[inline] pub fn nop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOP != 0"]
    #[inline] pub fn test_nop(&self) -> bool {
        self.nop() != 0
    }

    #[doc="Sets the NOP field."]
    #[inline] pub fn set_nop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cint {
    #[inline]
    fn from(other: u8) -> Self {
         Cint(other)
    }
}

impl ::core::fmt::Display for Cint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cint() != 0 { try!(write!(f, " cint=0x{:x}", self.cint()))}
        if self.cair() != 0 { try!(write!(f, " cair"))}
        if self.nop() != 0 { try!(write!(f, " nop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Request Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int(pub u32);
impl Int {
    #[doc="Interrupt Request n"]
    #[inline] pub fn int<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INT != 0"]
    #[inline] pub fn test_int<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.int(index) != 0
    }

    #[doc="Sets the INT field."]
    #[inline] pub fn set_int<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Int {
    #[inline]
    fn from(other: u32) -> Self {
         Int(other)
    }
}

impl ::core::fmt::Display for Int {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.int(0) != 0 { try!(write!(f, " int[0]"))}
        if self.int(1) != 0 { try!(write!(f, " int[1]"))}
        if self.int(2) != 0 { try!(write!(f, " int[2]"))}
        if self.int(3) != 0 { try!(write!(f, " int[3]"))}
        if self.int(4) != 0 { try!(write!(f, " int[4]"))}
        if self.int(5) != 0 { try!(write!(f, " int[5]"))}
        if self.int(6) != 0 { try!(write!(f, " int[6]"))}
        if self.int(7) != 0 { try!(write!(f, " int[7]"))}
        if self.int(8) != 0 { try!(write!(f, " int[8]"))}
        if self.int(9) != 0 { try!(write!(f, " int[9]"))}
        if self.int(10) != 0 { try!(write!(f, " int[10]"))}
        if self.int(11) != 0 { try!(write!(f, " int[11]"))}
        if self.int(12) != 0 { try!(write!(f, " int[12]"))}
        if self.int(13) != 0 { try!(write!(f, " int[13]"))}
        if self.int(14) != 0 { try!(write!(f, " int[14]"))}
        if self.int(15) != 0 { try!(write!(f, " int[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Err(pub u32);
impl Err {
    #[doc="Error In Channel n"]
    #[inline] pub fn err<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.err(index) != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Err {
    #[inline]
    fn from(other: u32) -> Self {
         Err(other)
    }
}

impl ::core::fmt::Display for Err {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Err {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.err(0) != 0 { try!(write!(f, " err[0]"))}
        if self.err(1) != 0 { try!(write!(f, " err[1]"))}
        if self.err(2) != 0 { try!(write!(f, " err[2]"))}
        if self.err(3) != 0 { try!(write!(f, " err[3]"))}
        if self.err(4) != 0 { try!(write!(f, " err[4]"))}
        if self.err(5) != 0 { try!(write!(f, " err[5]"))}
        if self.err(6) != 0 { try!(write!(f, " err[6]"))}
        if self.err(7) != 0 { try!(write!(f, " err[7]"))}
        if self.err(8) != 0 { try!(write!(f, " err[8]"))}
        if self.err(9) != 0 { try!(write!(f, " err[9]"))}
        if self.err(10) != 0 { try!(write!(f, " err[10]"))}
        if self.err(11) != 0 { try!(write!(f, " err[11]"))}
        if self.err(12) != 0 { try!(write!(f, " err[12]"))}
        if self.err(13) != 0 { try!(write!(f, " err[13]"))}
        if self.err(14) != 0 { try!(write!(f, " err[14]"))}
        if self.err(15) != 0 { try!(write!(f, " err[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Hardware Request Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hrs(pub u32);
impl Hrs {
    #[doc="Hardware Request Status Channel n"]
    #[inline] pub fn hrs<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HRS != 0"]
    #[inline] pub fn test_hrs<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.hrs(index) != 0
    }

    #[doc="Sets the HRS field."]
    #[inline] pub fn set_hrs<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Hrs {
    #[inline]
    fn from(other: u32) -> Self {
         Hrs(other)
    }
}

impl ::core::fmt::Display for Hrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hrs(0) != 0 { try!(write!(f, " hrs[0]"))}
        if self.hrs(1) != 0 { try!(write!(f, " hrs[1]"))}
        if self.hrs(2) != 0 { try!(write!(f, " hrs[2]"))}
        if self.hrs(3) != 0 { try!(write!(f, " hrs[3]"))}
        if self.hrs(4) != 0 { try!(write!(f, " hrs[4]"))}
        if self.hrs(5) != 0 { try!(write!(f, " hrs[5]"))}
        if self.hrs(6) != 0 { try!(write!(f, " hrs[6]"))}
        if self.hrs(7) != 0 { try!(write!(f, " hrs[7]"))}
        if self.hrs(8) != 0 { try!(write!(f, " hrs[8]"))}
        if self.hrs(9) != 0 { try!(write!(f, " hrs[9]"))}
        if self.hrs(10) != 0 { try!(write!(f, " hrs[10]"))}
        if self.hrs(11) != 0 { try!(write!(f, " hrs[11]"))}
        if self.hrs(12) != 0 { try!(write!(f, " hrs[12]"))}
        if self.hrs(13) != 0 { try!(write!(f, " hrs[13]"))}
        if self.hrs(14) != 0 { try!(write!(f, " hrs[14]"))}
        if self.hrs(15) != 0 { try!(write!(f, " hrs[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dchpri(pub u8);
impl Dchpri {
    #[doc="Channel n Arbitration Priority"]
    #[inline] pub fn chpri(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CHPRI != 0"]
    #[inline] pub fn test_chpri(&self) -> bool {
        self.chpri() != 0
    }

    #[doc="Sets the CHPRI field."]
    #[inline] pub fn set_chpri<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Disable Preempt Ability"]
    #[inline] pub fn dpa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DPA != 0"]
    #[inline] pub fn test_dpa(&self) -> bool {
        self.dpa() != 0
    }

    #[doc="Sets the DPA field."]
    #[inline] pub fn set_dpa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Enable Channel Preemption"]
    #[inline] pub fn ecp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ECP != 0"]
    #[inline] pub fn test_ecp(&self) -> bool {
        self.ecp() != 0
    }

    #[doc="Sets the ECP field."]
    #[inline] pub fn set_ecp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Dchpri {
    #[inline]
    fn from(other: u8) -> Self {
         Dchpri(other)
    }
}

impl ::core::fmt::Display for Dchpri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dchpri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chpri() != 0 { try!(write!(f, " chpri=0x{:x}", self.chpri()))}
        if self.dpa() != 0 { try!(write!(f, " dpa"))}
        if self.ecp() != 0 { try!(write!(f, " ecp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Source Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdSaddr(pub u32);
impl TcdSaddr {
    #[doc="Source Address"]
    #[inline] pub fn saddr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SADDR != 0"]
    #[inline] pub fn test_saddr(&self) -> bool {
        self.saddr() != 0
    }

    #[doc="Sets the SADDR field."]
    #[inline] pub fn set_saddr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for TcdSaddr {
    #[inline]
    fn from(other: u32) -> Self {
         TcdSaddr(other)
    }
}

impl ::core::fmt::Display for TcdSaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdSaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Signed Source Address Offset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdSoff(pub u16);
impl TcdSoff {
    #[doc="Source address signed offset"]
    #[inline] pub fn soff(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if SOFF != 0"]
    #[inline] pub fn test_soff(&self) -> bool {
        self.soff() != 0
    }

    #[doc="Sets the SOFF field."]
    #[inline] pub fn set_soff<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for TcdSoff {
    #[inline]
    fn from(other: u16) -> Self {
         TcdSoff(other)
    }
}

impl ::core::fmt::Display for TcdSoff {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdSoff {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.soff() != 0 { try!(write!(f, " soff=0x{:x}", self.soff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Transfer Attributes"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdAttr(pub u16);
impl TcdAttr {
    #[doc="Destination Data Transfer Size"]
    #[inline] pub fn dsize(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if DSIZE != 0"]
    #[inline] pub fn test_dsize(&self) -> bool {
        self.dsize() != 0
    }

    #[doc="Sets the DSIZE field."]
    #[inline] pub fn set_dsize<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Destination Address Modulo"]
    #[inline] pub fn dmod(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DMOD != 0"]
    #[inline] pub fn test_dmod(&self) -> bool {
        self.dmod() != 0
    }

    #[doc="Sets the DMOD field."]
    #[inline] pub fn set_dmod<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Source data transfer size"]
    #[inline] pub fn ssize(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if SSIZE != 0"]
    #[inline] pub fn test_ssize(&self) -> bool {
        self.ssize() != 0
    }

    #[doc="Sets the SSIZE field."]
    #[inline] pub fn set_ssize<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Source Address Modulo."]
    #[inline] pub fn smod(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1f) as u8) } // [15:11]
    }

    #[doc="Returns true if SMOD != 0"]
    #[inline] pub fn test_smod(&self) -> bool {
        self.smod() != 0
    }

    #[doc="Sets the SMOD field."]
    #[inline] pub fn set_smod<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u16> for TcdAttr {
    #[inline]
    fn from(other: u16) -> Self {
         TcdAttr(other)
    }
}

impl ::core::fmt::Display for TcdAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dsize() != 0 { try!(write!(f, " dsize=0x{:x}", self.dsize()))}
        if self.dmod() != 0 { try!(write!(f, " dmod=0x{:x}", self.dmod()))}
        if self.ssize() != 0 { try!(write!(f, " ssize=0x{:x}", self.ssize()))}
        if self.smod() != 0 { try!(write!(f, " smod=0x{:x}", self.smod()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Minor Byte Count (Minor Loop Disabled)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdNbytesMlno(pub u32);
impl TcdNbytesMlno {
    #[doc="Minor Byte Transfer Count"]
    #[inline] pub fn nbytes(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if NBYTES != 0"]
    #[inline] pub fn test_nbytes(&self) -> bool {
        self.nbytes() != 0
    }

    #[doc="Sets the NBYTES field."]
    #[inline] pub fn set_nbytes<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for TcdNbytesMlno {
    #[inline]
    fn from(other: u32) -> Self {
         TcdNbytesMlno(other)
    }
}

impl ::core::fmt::Display for TcdNbytesMlno {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdNbytesMlno {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdNbytesMloffno(pub u32);
impl TcdNbytesMloffno {
    #[doc="Minor Byte Transfer Count"]
    #[inline] pub fn nbytes(&self) -> bits::U30 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fffffff) as u32) } // [29:0]
    }

    #[doc="Returns true if NBYTES != 0"]
    #[inline] pub fn test_nbytes(&self) -> bool {
        self.nbytes() != 0
    }

    #[doc="Sets the NBYTES field."]
    #[inline] pub fn set_nbytes<V: Into<bits::U30>>(mut self, value: V) -> Self {
        let value: bits::U30 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Destination Minor Loop Offset enable"]
    #[inline] pub fn dmloe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if DMLOE != 0"]
    #[inline] pub fn test_dmloe(&self) -> bool {
        self.dmloe() != 0
    }

    #[doc="Sets the DMLOE field."]
    #[inline] pub fn set_dmloe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Source Minor Loop Offset Enable"]
    #[inline] pub fn smloe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SMLOE != 0"]
    #[inline] pub fn test_smloe(&self) -> bool {
        self.smloe() != 0
    }

    #[doc="Sets the SMLOE field."]
    #[inline] pub fn set_smloe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for TcdNbytesMloffno {
    #[inline]
    fn from(other: u32) -> Self {
         TcdNbytesMloffno(other)
    }
}

impl ::core::fmt::Display for TcdNbytesMloffno {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdNbytesMloffno {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nbytes() != 0 { try!(write!(f, " nbytes=0x{:x}", self.nbytes()))}
        if self.dmloe() != 0 { try!(write!(f, " dmloe"))}
        if self.smloe() != 0 { try!(write!(f, " smloe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdNbytesMloffyes(pub u32);
impl TcdNbytesMloffyes {
    #[doc="Minor Byte Transfer Count"]
    #[inline] pub fn nbytes(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if NBYTES != 0"]
    #[inline] pub fn test_nbytes(&self) -> bool {
        self.nbytes() != 0
    }

    #[doc="Sets the NBYTES field."]
    #[inline] pub fn set_nbytes<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline] pub fn mloff(&self) -> bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0xfffff) as u32) } // [29:10]
    }

    #[doc="Returns true if MLOFF != 0"]
    #[inline] pub fn test_mloff(&self) -> bool {
        self.mloff() != 0
    }

    #[doc="Sets the MLOFF field."]
    #[inline] pub fn set_mloff<V: Into<bits::U20>>(mut self, value: V) -> Self {
        let value: bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Destination Minor Loop Offset enable"]
    #[inline] pub fn dmloe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if DMLOE != 0"]
    #[inline] pub fn test_dmloe(&self) -> bool {
        self.dmloe() != 0
    }

    #[doc="Sets the DMLOE field."]
    #[inline] pub fn set_dmloe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Source Minor Loop Offset Enable"]
    #[inline] pub fn smloe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SMLOE != 0"]
    #[inline] pub fn test_smloe(&self) -> bool {
        self.smloe() != 0
    }

    #[doc="Sets the SMLOE field."]
    #[inline] pub fn set_smloe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for TcdNbytesMloffyes {
    #[inline]
    fn from(other: u32) -> Self {
         TcdNbytesMloffyes(other)
    }
}

impl ::core::fmt::Display for TcdNbytesMloffyes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdNbytesMloffyes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nbytes() != 0 { try!(write!(f, " nbytes=0x{:x}", self.nbytes()))}
        if self.mloff() != 0 { try!(write!(f, " mloff=0x{:x}", self.mloff()))}
        if self.dmloe() != 0 { try!(write!(f, " dmloe"))}
        if self.smloe() != 0 { try!(write!(f, " smloe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Last Source Address Adjustment"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdSlast(pub u32);
impl TcdSlast {
    #[doc="Last source Address Adjustment"]
    #[inline] pub fn slast(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SLAST != 0"]
    #[inline] pub fn test_slast(&self) -> bool {
        self.slast() != 0
    }

    #[doc="Sets the SLAST field."]
    #[inline] pub fn set_slast<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for TcdSlast {
    #[inline]
    fn from(other: u32) -> Self {
         TcdSlast(other)
    }
}

impl ::core::fmt::Display for TcdSlast {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdSlast {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Destination Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdDaddr(pub u32);
impl TcdDaddr {
    #[doc="Destination Address"]
    #[inline] pub fn daddr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DADDR != 0"]
    #[inline] pub fn test_daddr(&self) -> bool {
        self.daddr() != 0
    }

    #[doc="Sets the DADDR field."]
    #[inline] pub fn set_daddr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for TcdDaddr {
    #[inline]
    fn from(other: u32) -> Self {
         TcdDaddr(other)
    }
}

impl ::core::fmt::Display for TcdDaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdDaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Signed Destination Address Offset"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdDoff(pub u16);
impl TcdDoff {
    #[doc="Destination Address Signed offset"]
    #[inline] pub fn doff(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DOFF != 0"]
    #[inline] pub fn test_doff(&self) -> bool {
        self.doff() != 0
    }

    #[doc="Sets the DOFF field."]
    #[inline] pub fn set_doff<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for TcdDoff {
    #[inline]
    fn from(other: u16) -> Self {
         TcdDoff(other)
    }
}

impl ::core::fmt::Display for TcdDoff {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdDoff {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.doff() != 0 { try!(write!(f, " doff=0x{:x}", self.doff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdCiterElinkno(pub u16);
impl TcdCiterElinkno {
    #[doc="Current Major Iteration Count"]
    #[inline] pub fn citer(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
    }

    #[doc="Returns true if CITER != 0"]
    #[inline] pub fn test_citer(&self) -> bool {
        self.citer() != 0
    }

    #[doc="Sets the CITER field."]
    #[inline] pub fn set_citer<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable channel-to-channel linking on minor-loop complete"]
    #[inline] pub fn elink(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ELINK != 0"]
    #[inline] pub fn test_elink(&self) -> bool {
        self.elink() != 0
    }

    #[doc="Sets the ELINK field."]
    #[inline] pub fn set_elink<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for TcdCiterElinkno {
    #[inline]
    fn from(other: u16) -> Self {
         TcdCiterElinkno(other)
    }
}

impl ::core::fmt::Display for TcdCiterElinkno {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdCiterElinkno {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.citer() != 0 { try!(write!(f, " citer=0x{:x}", self.citer()))}
        if self.elink() != 0 { try!(write!(f, " elink"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdCiterElinkyes(pub u16);
impl TcdCiterElinkyes {
    #[doc="Current Major Iteration Count"]
    #[inline] pub fn citer(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if CITER != 0"]
    #[inline] pub fn test_citer(&self) -> bool {
        self.citer() != 0
    }

    #[doc="Sets the CITER field."]
    #[inline] pub fn set_citer<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Link Channel Number"]
    #[inline] pub fn linkch(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
    }

    #[doc="Returns true if LINKCH != 0"]
    #[inline] pub fn test_linkch(&self) -> bool {
        self.linkch() != 0
    }

    #[doc="Sets the LINKCH field."]
    #[inline] pub fn set_linkch<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable channel-to-channel linking on minor-loop complete"]
    #[inline] pub fn elink(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ELINK != 0"]
    #[inline] pub fn test_elink(&self) -> bool {
        self.elink() != 0
    }

    #[doc="Sets the ELINK field."]
    #[inline] pub fn set_elink<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for TcdCiterElinkyes {
    #[inline]
    fn from(other: u16) -> Self {
         TcdCiterElinkyes(other)
    }
}

impl ::core::fmt::Display for TcdCiterElinkyes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdCiterElinkyes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.citer() != 0 { try!(write!(f, " citer=0x{:x}", self.citer()))}
        if self.linkch() != 0 { try!(write!(f, " linkch=0x{:x}", self.linkch()))}
        if self.elink() != 0 { try!(write!(f, " elink"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdDlastsga(pub u32);
impl TcdDlastsga {
    #[doc="Destination last address adjustment or the memory address for the next transfer control descriptor to be loaded into this channel (scatter/gather)"]
    #[inline] pub fn dlastsga(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DLASTSGA != 0"]
    #[inline] pub fn test_dlastsga(&self) -> bool {
        self.dlastsga() != 0
    }

    #[doc="Sets the DLASTSGA field."]
    #[inline] pub fn set_dlastsga<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for TcdDlastsga {
    #[inline]
    fn from(other: u32) -> Self {
         TcdDlastsga(other)
    }
}

impl ::core::fmt::Display for TcdDlastsga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdDlastsga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Control and Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdCsr(pub u16);
impl TcdCsr {
    #[doc="Channel Start"]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable an interrupt when major iteration count completes"]
    #[inline] pub fn intmajor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INTMAJOR != 0"]
    #[inline] pub fn test_intmajor(&self) -> bool {
        self.intmajor() != 0
    }

    #[doc="Sets the INTMAJOR field."]
    #[inline] pub fn set_intmajor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable an interrupt when major counter is half complete."]
    #[inline] pub fn inthalf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if INTHALF != 0"]
    #[inline] pub fn test_inthalf(&self) -> bool {
        self.inthalf() != 0
    }

    #[doc="Sets the INTHALF field."]
    #[inline] pub fn set_inthalf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Disable Request"]
    #[inline] pub fn dreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DREQ != 0"]
    #[inline] pub fn test_dreq(&self) -> bool {
        self.dreq() != 0
    }

    #[doc="Sets the DREQ field."]
    #[inline] pub fn set_dreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enable Scatter/Gather Processing"]
    #[inline] pub fn esg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ESG != 0"]
    #[inline] pub fn test_esg(&self) -> bool {
        self.esg() != 0
    }

    #[doc="Sets the ESG field."]
    #[inline] pub fn set_esg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enable channel-to-channel linking on major loop complete"]
    #[inline] pub fn majorelink(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MAJORELINK != 0"]
    #[inline] pub fn test_majorelink(&self) -> bool {
        self.majorelink() != 0
    }

    #[doc="Sets the MAJORELINK field."]
    #[inline] pub fn set_majorelink<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel Active"]
    #[inline] pub fn active(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ACTIVE != 0"]
    #[inline] pub fn test_active(&self) -> bool {
        self.active() != 0
    }

    #[doc="Sets the ACTIVE field."]
    #[inline] pub fn set_active<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel Done"]
    #[inline] pub fn done(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DONE != 0"]
    #[inline] pub fn test_done(&self) -> bool {
        self.done() != 0
    }

    #[doc="Sets the DONE field."]
    #[inline] pub fn set_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Link Channel Number"]
    #[inline] pub fn majorlinkch(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MAJORLINKCH != 0"]
    #[inline] pub fn test_majorlinkch(&self) -> bool {
        self.majorlinkch() != 0
    }

    #[doc="Sets the MAJORLINKCH field."]
    #[inline] pub fn set_majorlinkch<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Bandwidth Control"]
    #[inline] pub fn bwc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if BWC != 0"]
    #[inline] pub fn test_bwc(&self) -> bool {
        self.bwc() != 0
    }

    #[doc="Sets the BWC field."]
    #[inline] pub fn set_bwc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u16> for TcdCsr {
    #[inline]
    fn from(other: u16) -> Self {
         TcdCsr(other)
    }
}

impl ::core::fmt::Display for TcdCsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdCsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.intmajor() != 0 { try!(write!(f, " intmajor"))}
        if self.inthalf() != 0 { try!(write!(f, " inthalf"))}
        if self.dreq() != 0 { try!(write!(f, " dreq"))}
        if self.esg() != 0 { try!(write!(f, " esg"))}
        if self.majorelink() != 0 { try!(write!(f, " majorelink"))}
        if self.active() != 0 { try!(write!(f, " active"))}
        if self.done() != 0 { try!(write!(f, " done"))}
        if self.majorlinkch() != 0 { try!(write!(f, " majorlinkch=0x{:x}", self.majorlinkch()))}
        if self.bwc() != 0 { try!(write!(f, " bwc=0x{:x}", self.bwc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdBiterElinkno(pub u16);
impl TcdBiterElinkno {
    #[doc="Starting Major Iteration Count"]
    #[inline] pub fn biter(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
    }

    #[doc="Returns true if BITER != 0"]
    #[inline] pub fn test_biter(&self) -> bool {
        self.biter() != 0
    }

    #[doc="Sets the BITER field."]
    #[inline] pub fn set_biter<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enables channel-to-channel linking on minor loop complete"]
    #[inline] pub fn elink(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ELINK != 0"]
    #[inline] pub fn test_elink(&self) -> bool {
        self.elink() != 0
    }

    #[doc="Sets the ELINK field."]
    #[inline] pub fn set_elink<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for TcdBiterElinkno {
    #[inline]
    fn from(other: u16) -> Self {
         TcdBiterElinkno(other)
    }
}

impl ::core::fmt::Display for TcdBiterElinkno {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdBiterElinkno {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.biter() != 0 { try!(write!(f, " biter=0x{:x}", self.biter()))}
        if self.elink() != 0 { try!(write!(f, " elink"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
    #[doc="Starting Major Iteration Count"]
    #[inline] pub fn biter(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if BITER != 0"]
    #[inline] pub fn test_biter(&self) -> bool {
        self.biter() != 0
    }

    #[doc="Sets the BITER field."]
    #[inline] pub fn set_biter<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Link Channel Number"]
    #[inline] pub fn linkch(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
    }

    #[doc="Returns true if LINKCH != 0"]
    #[inline] pub fn test_linkch(&self) -> bool {
        self.linkch() != 0
    }

    #[doc="Sets the LINKCH field."]
    #[inline] pub fn set_linkch<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enables channel-to-channel linking on minor loop complete"]
    #[inline] pub fn elink(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ELINK != 0"]
    #[inline] pub fn test_elink(&self) -> bool {
        self.elink() != 0
    }

    #[doc="Sets the ELINK field."]
    #[inline] pub fn set_elink<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for TcdBiterElinkyes {
    #[inline]
    fn from(other: u16) -> Self {
         TcdBiterElinkyes(other)
    }
}

impl ::core::fmt::Display for TcdBiterElinkyes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for TcdBiterElinkyes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.biter() != 0 { try!(write!(f, " biter=0x{:x}", self.biter()))}
        if self.linkch() != 0 { try!(write!(f, " linkch=0x{:x}", self.linkch()))}
        if self.elink() != 0 { try!(write!(f, " elink"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

