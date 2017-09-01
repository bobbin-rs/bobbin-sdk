//! Ethernet MAC-NET Core
#[allow(unused_imports)] use bobbin_common::*;

periph!(ENET, Enet, 0x400c0000);

#[doc="Ethernet MAC-NET Core"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Enet(pub usize);
impl Enet {
    #[doc="Get the *mut pointer for the EIR register."]
    #[inline] pub fn eir_mut(&self) -> *mut Eir { 
        (self.0 + 0x4) as *mut Eir
    }

    #[doc="Get the *const pointer for the EIR register."]
    #[inline] pub fn eir_ptr(&self) -> *const Eir { 
           self.eir_mut()
    }

    #[doc="Read the EIR register."]
    #[inline] pub fn eir(&self) -> Eir { 
        unsafe {
            read_volatile(self.eir_ptr())
        }
    }

    #[doc="Write the EIR register."]
    #[inline] pub fn set_eir<F: FnOnce(Eir) -> Eir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.eir_mut(), f(Eir(0)));
        }
        self
    }

    #[doc="Modify the EIR register."]
    #[inline] pub fn with_eir<F: FnOnce(Eir) -> Eir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.eir_mut(), f(self.eir()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EIMR register."]
    #[inline] pub fn eimr_mut(&self) -> *mut Eimr { 
        (self.0 + 0x8) as *mut Eimr
    }

    #[doc="Get the *const pointer for the EIMR register."]
    #[inline] pub fn eimr_ptr(&self) -> *const Eimr { 
           self.eimr_mut()
    }

    #[doc="Read the EIMR register."]
    #[inline] pub fn eimr(&self) -> Eimr { 
        unsafe {
            read_volatile(self.eimr_ptr())
        }
    }

    #[doc="Write the EIMR register."]
    #[inline] pub fn set_eimr<F: FnOnce(Eimr) -> Eimr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.eimr_mut(), f(Eimr(0)));
        }
        self
    }

    #[doc="Modify the EIMR register."]
    #[inline] pub fn with_eimr<F: FnOnce(Eimr) -> Eimr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.eimr_mut(), f(self.eimr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RDAR register."]
    #[inline] pub fn rdar_mut(&self) -> *mut Rdar { 
        (self.0 + 0x10) as *mut Rdar
    }

    #[doc="Get the *const pointer for the RDAR register."]
    #[inline] pub fn rdar_ptr(&self) -> *const Rdar { 
           self.rdar_mut()
    }

    #[doc="Read the RDAR register."]
    #[inline] pub fn rdar(&self) -> Rdar { 
        unsafe {
            read_volatile(self.rdar_ptr())
        }
    }

    #[doc="Write the RDAR register."]
    #[inline] pub fn set_rdar<F: FnOnce(Rdar) -> Rdar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rdar_mut(), f(Rdar(0)));
        }
        self
    }

    #[doc="Modify the RDAR register."]
    #[inline] pub fn with_rdar<F: FnOnce(Rdar) -> Rdar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rdar_mut(), f(self.rdar()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDAR register."]
    #[inline] pub fn tdar_mut(&self) -> *mut Tdar { 
        (self.0 + 0x14) as *mut Tdar
    }

    #[doc="Get the *const pointer for the TDAR register."]
    #[inline] pub fn tdar_ptr(&self) -> *const Tdar { 
           self.tdar_mut()
    }

    #[doc="Read the TDAR register."]
    #[inline] pub fn tdar(&self) -> Tdar { 
        unsafe {
            read_volatile(self.tdar_ptr())
        }
    }

    #[doc="Write the TDAR register."]
    #[inline] pub fn set_tdar<F: FnOnce(Tdar) -> Tdar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdar_mut(), f(Tdar(0)));
        }
        self
    }

    #[doc="Modify the TDAR register."]
    #[inline] pub fn with_tdar<F: FnOnce(Tdar) -> Tdar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdar_mut(), f(self.tdar()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ECR register."]
    #[inline] pub fn ecr_mut(&self) -> *mut Ecr { 
        (self.0 + 0x24) as *mut Ecr
    }

    #[doc="Get the *const pointer for the ECR register."]
    #[inline] pub fn ecr_ptr(&self) -> *const Ecr { 
           self.ecr_mut()
    }

    #[doc="Read the ECR register."]
    #[inline] pub fn ecr(&self) -> Ecr { 
        unsafe {
            read_volatile(self.ecr_ptr())
        }
    }

    #[doc="Write the ECR register."]
    #[inline] pub fn set_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ecr_mut(), f(Ecr(0)));
        }
        self
    }

    #[doc="Modify the ECR register."]
    #[inline] pub fn with_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ecr_mut(), f(self.ecr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MMFR register."]
    #[inline] pub fn mmfr_mut(&self) -> *mut Mmfr { 
        (self.0 + 0x40) as *mut Mmfr
    }

    #[doc="Get the *const pointer for the MMFR register."]
    #[inline] pub fn mmfr_ptr(&self) -> *const Mmfr { 
           self.mmfr_mut()
    }

    #[doc="Read the MMFR register."]
    #[inline] pub fn mmfr(&self) -> Mmfr { 
        unsafe {
            read_volatile(self.mmfr_ptr())
        }
    }

    #[doc="Write the MMFR register."]
    #[inline] pub fn set_mmfr<F: FnOnce(Mmfr) -> Mmfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmfr_mut(), f(Mmfr(0)));
        }
        self
    }

    #[doc="Modify the MMFR register."]
    #[inline] pub fn with_mmfr<F: FnOnce(Mmfr) -> Mmfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmfr_mut(), f(self.mmfr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MSCR register."]
    #[inline] pub fn mscr_mut(&self) -> *mut Mscr { 
        (self.0 + 0x44) as *mut Mscr
    }

    #[doc="Get the *const pointer for the MSCR register."]
    #[inline] pub fn mscr_ptr(&self) -> *const Mscr { 
           self.mscr_mut()
    }

    #[doc="Read the MSCR register."]
    #[inline] pub fn mscr(&self) -> Mscr { 
        unsafe {
            read_volatile(self.mscr_ptr())
        }
    }

    #[doc="Write the MSCR register."]
    #[inline] pub fn set_mscr<F: FnOnce(Mscr) -> Mscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mscr_mut(), f(Mscr(0)));
        }
        self
    }

    #[doc="Modify the MSCR register."]
    #[inline] pub fn with_mscr<F: FnOnce(Mscr) -> Mscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mscr_mut(), f(self.mscr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MIBC register."]
    #[inline] pub fn mibc_mut(&self) -> *mut Mibc { 
        (self.0 + 0x64) as *mut Mibc
    }

    #[doc="Get the *const pointer for the MIBC register."]
    #[inline] pub fn mibc_ptr(&self) -> *const Mibc { 
           self.mibc_mut()
    }

    #[doc="Read the MIBC register."]
    #[inline] pub fn mibc(&self) -> Mibc { 
        unsafe {
            read_volatile(self.mibc_ptr())
        }
    }

    #[doc="Write the MIBC register."]
    #[inline] pub fn set_mibc<F: FnOnce(Mibc) -> Mibc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mibc_mut(), f(Mibc(0)));
        }
        self
    }

    #[doc="Modify the MIBC register."]
    #[inline] pub fn with_mibc<F: FnOnce(Mibc) -> Mibc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mibc_mut(), f(self.mibc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RCR register."]
    #[inline] pub fn rcr_mut(&self) -> *mut Rcr { 
        (self.0 + 0x84) as *mut Rcr
    }

    #[doc="Get the *const pointer for the RCR register."]
    #[inline] pub fn rcr_ptr(&self) -> *const Rcr { 
           self.rcr_mut()
    }

    #[doc="Read the RCR register."]
    #[inline] pub fn rcr(&self) -> Rcr { 
        unsafe {
            read_volatile(self.rcr_ptr())
        }
    }

    #[doc="Write the RCR register."]
    #[inline] pub fn set_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rcr_mut(), f(Rcr(0)));
        }
        self
    }

    #[doc="Modify the RCR register."]
    #[inline] pub fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rcr_mut(), f(self.rcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCR register."]
    #[inline] pub fn tcr_mut(&self) -> *mut Tcr { 
        (self.0 + 0xc4) as *mut Tcr
    }

    #[doc="Get the *const pointer for the TCR register."]
    #[inline] pub fn tcr_ptr(&self) -> *const Tcr { 
           self.tcr_mut()
    }

    #[doc="Read the TCR register."]
    #[inline] pub fn tcr(&self) -> Tcr { 
        unsafe {
            read_volatile(self.tcr_ptr())
        }
    }

    #[doc="Write the TCR register."]
    #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcr_mut(), f(Tcr(0)));
        }
        self
    }

    #[doc="Modify the TCR register."]
    #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcr_mut(), f(self.tcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PALR register."]
    #[inline] pub fn palr_mut(&self) -> *mut Palr { 
        (self.0 + 0xe4) as *mut Palr
    }

    #[doc="Get the *const pointer for the PALR register."]
    #[inline] pub fn palr_ptr(&self) -> *const Palr { 
           self.palr_mut()
    }

    #[doc="Read the PALR register."]
    #[inline] pub fn palr(&self) -> Palr { 
        unsafe {
            read_volatile(self.palr_ptr())
        }
    }

    #[doc="Write the PALR register."]
    #[inline] pub fn set_palr<F: FnOnce(Palr) -> Palr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.palr_mut(), f(Palr(0)));
        }
        self
    }

    #[doc="Modify the PALR register."]
    #[inline] pub fn with_palr<F: FnOnce(Palr) -> Palr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.palr_mut(), f(self.palr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PAUR register."]
    #[inline] pub fn paur_mut(&self) -> *mut Paur { 
        (self.0 + 0xe8) as *mut Paur
    }

    #[doc="Get the *const pointer for the PAUR register."]
    #[inline] pub fn paur_ptr(&self) -> *const Paur { 
           self.paur_mut()
    }

    #[doc="Read the PAUR register."]
    #[inline] pub fn paur(&self) -> Paur { 
        unsafe {
            read_volatile(self.paur_ptr())
        }
    }

    #[doc="Write the PAUR register."]
    #[inline] pub fn set_paur<F: FnOnce(Paur) -> Paur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.paur_mut(), f(Paur(0)));
        }
        self
    }

    #[doc="Modify the PAUR register."]
    #[inline] pub fn with_paur<F: FnOnce(Paur) -> Paur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.paur_mut(), f(self.paur()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPD register."]
    #[inline] pub fn opd_mut(&self) -> *mut Opd { 
        (self.0 + 0xec) as *mut Opd
    }

    #[doc="Get the *const pointer for the OPD register."]
    #[inline] pub fn opd_ptr(&self) -> *const Opd { 
           self.opd_mut()
    }

    #[doc="Read the OPD register."]
    #[inline] pub fn opd(&self) -> Opd { 
        unsafe {
            read_volatile(self.opd_ptr())
        }
    }

    #[doc="Write the OPD register."]
    #[inline] pub fn set_opd<F: FnOnce(Opd) -> Opd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opd_mut(), f(Opd(0)));
        }
        self
    }

    #[doc="Modify the OPD register."]
    #[inline] pub fn with_opd<F: FnOnce(Opd) -> Opd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opd_mut(), f(self.opd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IAUR register."]
    #[inline] pub fn iaur_mut(&self) -> *mut Iaur { 
        (self.0 + 0x118) as *mut Iaur
    }

    #[doc="Get the *const pointer for the IAUR register."]
    #[inline] pub fn iaur_ptr(&self) -> *const Iaur { 
           self.iaur_mut()
    }

    #[doc="Read the IAUR register."]
    #[inline] pub fn iaur(&self) -> Iaur { 
        unsafe {
            read_volatile(self.iaur_ptr())
        }
    }

    #[doc="Write the IAUR register."]
    #[inline] pub fn set_iaur<F: FnOnce(Iaur) -> Iaur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iaur_mut(), f(Iaur(0)));
        }
        self
    }

    #[doc="Modify the IAUR register."]
    #[inline] pub fn with_iaur<F: FnOnce(Iaur) -> Iaur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iaur_mut(), f(self.iaur()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IALR register."]
    #[inline] pub fn ialr_mut(&self) -> *mut Ialr { 
        (self.0 + 0x11c) as *mut Ialr
    }

    #[doc="Get the *const pointer for the IALR register."]
    #[inline] pub fn ialr_ptr(&self) -> *const Ialr { 
           self.ialr_mut()
    }

    #[doc="Read the IALR register."]
    #[inline] pub fn ialr(&self) -> Ialr { 
        unsafe {
            read_volatile(self.ialr_ptr())
        }
    }

    #[doc="Write the IALR register."]
    #[inline] pub fn set_ialr<F: FnOnce(Ialr) -> Ialr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ialr_mut(), f(Ialr(0)));
        }
        self
    }

    #[doc="Modify the IALR register."]
    #[inline] pub fn with_ialr<F: FnOnce(Ialr) -> Ialr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ialr_mut(), f(self.ialr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the GAUR register."]
    #[inline] pub fn gaur_mut(&self) -> *mut Gaur { 
        (self.0 + 0x120) as *mut Gaur
    }

    #[doc="Get the *const pointer for the GAUR register."]
    #[inline] pub fn gaur_ptr(&self) -> *const Gaur { 
           self.gaur_mut()
    }

    #[doc="Read the GAUR register."]
    #[inline] pub fn gaur(&self) -> Gaur { 
        unsafe {
            read_volatile(self.gaur_ptr())
        }
    }

    #[doc="Write the GAUR register."]
    #[inline] pub fn set_gaur<F: FnOnce(Gaur) -> Gaur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gaur_mut(), f(Gaur(0)));
        }
        self
    }

    #[doc="Modify the GAUR register."]
    #[inline] pub fn with_gaur<F: FnOnce(Gaur) -> Gaur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gaur_mut(), f(self.gaur()));
        }
        self
    }

    #[doc="Get the *mut pointer for the GALR register."]
    #[inline] pub fn galr_mut(&self) -> *mut Galr { 
        (self.0 + 0x124) as *mut Galr
    }

    #[doc="Get the *const pointer for the GALR register."]
    #[inline] pub fn galr_ptr(&self) -> *const Galr { 
           self.galr_mut()
    }

    #[doc="Read the GALR register."]
    #[inline] pub fn galr(&self) -> Galr { 
        unsafe {
            read_volatile(self.galr_ptr())
        }
    }

    #[doc="Write the GALR register."]
    #[inline] pub fn set_galr<F: FnOnce(Galr) -> Galr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.galr_mut(), f(Galr(0)));
        }
        self
    }

    #[doc="Modify the GALR register."]
    #[inline] pub fn with_galr<F: FnOnce(Galr) -> Galr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.galr_mut(), f(self.galr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TFWR register."]
    #[inline] pub fn tfwr_mut(&self) -> *mut Tfwr { 
        (self.0 + 0x144) as *mut Tfwr
    }

    #[doc="Get the *const pointer for the TFWR register."]
    #[inline] pub fn tfwr_ptr(&self) -> *const Tfwr { 
           self.tfwr_mut()
    }

    #[doc="Read the TFWR register."]
    #[inline] pub fn tfwr(&self) -> Tfwr { 
        unsafe {
            read_volatile(self.tfwr_ptr())
        }
    }

    #[doc="Write the TFWR register."]
    #[inline] pub fn set_tfwr<F: FnOnce(Tfwr) -> Tfwr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tfwr_mut(), f(Tfwr(0)));
        }
        self
    }

    #[doc="Modify the TFWR register."]
    #[inline] pub fn with_tfwr<F: FnOnce(Tfwr) -> Tfwr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tfwr_mut(), f(self.tfwr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RDSR register."]
    #[inline] pub fn rdsr_mut(&self) -> *mut Rdsr { 
        (self.0 + 0x180) as *mut Rdsr
    }

    #[doc="Get the *const pointer for the RDSR register."]
    #[inline] pub fn rdsr_ptr(&self) -> *const Rdsr { 
           self.rdsr_mut()
    }

    #[doc="Read the RDSR register."]
    #[inline] pub fn rdsr(&self) -> Rdsr { 
        unsafe {
            read_volatile(self.rdsr_ptr())
        }
    }

    #[doc="Write the RDSR register."]
    #[inline] pub fn set_rdsr<F: FnOnce(Rdsr) -> Rdsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rdsr_mut(), f(Rdsr(0)));
        }
        self
    }

    #[doc="Modify the RDSR register."]
    #[inline] pub fn with_rdsr<F: FnOnce(Rdsr) -> Rdsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rdsr_mut(), f(self.rdsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDSR register."]
    #[inline] pub fn tdsr_mut(&self) -> *mut Tdsr { 
        (self.0 + 0x184) as *mut Tdsr
    }

    #[doc="Get the *const pointer for the TDSR register."]
    #[inline] pub fn tdsr_ptr(&self) -> *const Tdsr { 
           self.tdsr_mut()
    }

    #[doc="Read the TDSR register."]
    #[inline] pub fn tdsr(&self) -> Tdsr { 
        unsafe {
            read_volatile(self.tdsr_ptr())
        }
    }

    #[doc="Write the TDSR register."]
    #[inline] pub fn set_tdsr<F: FnOnce(Tdsr) -> Tdsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdsr_mut(), f(Tdsr(0)));
        }
        self
    }

    #[doc="Modify the TDSR register."]
    #[inline] pub fn with_tdsr<F: FnOnce(Tdsr) -> Tdsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdsr_mut(), f(self.tdsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MRBR register."]
    #[inline] pub fn mrbr_mut(&self) -> *mut Mrbr { 
        (self.0 + 0x188) as *mut Mrbr
    }

    #[doc="Get the *const pointer for the MRBR register."]
    #[inline] pub fn mrbr_ptr(&self) -> *const Mrbr { 
           self.mrbr_mut()
    }

    #[doc="Read the MRBR register."]
    #[inline] pub fn mrbr(&self) -> Mrbr { 
        unsafe {
            read_volatile(self.mrbr_ptr())
        }
    }

    #[doc="Write the MRBR register."]
    #[inline] pub fn set_mrbr<F: FnOnce(Mrbr) -> Mrbr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mrbr_mut(), f(Mrbr(0)));
        }
        self
    }

    #[doc="Modify the MRBR register."]
    #[inline] pub fn with_mrbr<F: FnOnce(Mrbr) -> Mrbr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mrbr_mut(), f(self.mrbr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RSFL register."]
    #[inline] pub fn rsfl_mut(&self) -> *mut Rsfl { 
        (self.0 + 0x190) as *mut Rsfl
    }

    #[doc="Get the *const pointer for the RSFL register."]
    #[inline] pub fn rsfl_ptr(&self) -> *const Rsfl { 
           self.rsfl_mut()
    }

    #[doc="Read the RSFL register."]
    #[inline] pub fn rsfl(&self) -> Rsfl { 
        unsafe {
            read_volatile(self.rsfl_ptr())
        }
    }

    #[doc="Write the RSFL register."]
    #[inline] pub fn set_rsfl<F: FnOnce(Rsfl) -> Rsfl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rsfl_mut(), f(Rsfl(0)));
        }
        self
    }

    #[doc="Modify the RSFL register."]
    #[inline] pub fn with_rsfl<F: FnOnce(Rsfl) -> Rsfl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rsfl_mut(), f(self.rsfl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RSEM register."]
    #[inline] pub fn rsem_mut(&self) -> *mut Rsem { 
        (self.0 + 0x194) as *mut Rsem
    }

    #[doc="Get the *const pointer for the RSEM register."]
    #[inline] pub fn rsem_ptr(&self) -> *const Rsem { 
           self.rsem_mut()
    }

    #[doc="Read the RSEM register."]
    #[inline] pub fn rsem(&self) -> Rsem { 
        unsafe {
            read_volatile(self.rsem_ptr())
        }
    }

    #[doc="Write the RSEM register."]
    #[inline] pub fn set_rsem<F: FnOnce(Rsem) -> Rsem>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rsem_mut(), f(Rsem(0)));
        }
        self
    }

    #[doc="Modify the RSEM register."]
    #[inline] pub fn with_rsem<F: FnOnce(Rsem) -> Rsem>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rsem_mut(), f(self.rsem()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAEM register."]
    #[inline] pub fn raem_mut(&self) -> *mut Raem { 
        (self.0 + 0x198) as *mut Raem
    }

    #[doc="Get the *const pointer for the RAEM register."]
    #[inline] pub fn raem_ptr(&self) -> *const Raem { 
           self.raem_mut()
    }

    #[doc="Read the RAEM register."]
    #[inline] pub fn raem(&self) -> Raem { 
        unsafe {
            read_volatile(self.raem_ptr())
        }
    }

    #[doc="Write the RAEM register."]
    #[inline] pub fn set_raem<F: FnOnce(Raem) -> Raem>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.raem_mut(), f(Raem(0)));
        }
        self
    }

    #[doc="Modify the RAEM register."]
    #[inline] pub fn with_raem<F: FnOnce(Raem) -> Raem>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.raem_mut(), f(self.raem()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RAFL register."]
    #[inline] pub fn rafl_mut(&self) -> *mut Rafl { 
        (self.0 + 0x19c) as *mut Rafl
    }

    #[doc="Get the *const pointer for the RAFL register."]
    #[inline] pub fn rafl_ptr(&self) -> *const Rafl { 
           self.rafl_mut()
    }

    #[doc="Read the RAFL register."]
    #[inline] pub fn rafl(&self) -> Rafl { 
        unsafe {
            read_volatile(self.rafl_ptr())
        }
    }

    #[doc="Write the RAFL register."]
    #[inline] pub fn set_rafl<F: FnOnce(Rafl) -> Rafl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rafl_mut(), f(Rafl(0)));
        }
        self
    }

    #[doc="Modify the RAFL register."]
    #[inline] pub fn with_rafl<F: FnOnce(Rafl) -> Rafl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rafl_mut(), f(self.rafl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TSEM register."]
    #[inline] pub fn tsem_mut(&self) -> *mut Tsem { 
        (self.0 + 0x1a0) as *mut Tsem
    }

    #[doc="Get the *const pointer for the TSEM register."]
    #[inline] pub fn tsem_ptr(&self) -> *const Tsem { 
           self.tsem_mut()
    }

    #[doc="Read the TSEM register."]
    #[inline] pub fn tsem(&self) -> Tsem { 
        unsafe {
            read_volatile(self.tsem_ptr())
        }
    }

    #[doc="Write the TSEM register."]
    #[inline] pub fn set_tsem<F: FnOnce(Tsem) -> Tsem>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tsem_mut(), f(Tsem(0)));
        }
        self
    }

    #[doc="Modify the TSEM register."]
    #[inline] pub fn with_tsem<F: FnOnce(Tsem) -> Tsem>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tsem_mut(), f(self.tsem()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TAEM register."]
    #[inline] pub fn taem_mut(&self) -> *mut Taem { 
        (self.0 + 0x1a4) as *mut Taem
    }

    #[doc="Get the *const pointer for the TAEM register."]
    #[inline] pub fn taem_ptr(&self) -> *const Taem { 
           self.taem_mut()
    }

    #[doc="Read the TAEM register."]
    #[inline] pub fn taem(&self) -> Taem { 
        unsafe {
            read_volatile(self.taem_ptr())
        }
    }

    #[doc="Write the TAEM register."]
    #[inline] pub fn set_taem<F: FnOnce(Taem) -> Taem>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.taem_mut(), f(Taem(0)));
        }
        self
    }

    #[doc="Modify the TAEM register."]
    #[inline] pub fn with_taem<F: FnOnce(Taem) -> Taem>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.taem_mut(), f(self.taem()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TAFL register."]
    #[inline] pub fn tafl_mut(&self) -> *mut Tafl { 
        (self.0 + 0x1a8) as *mut Tafl
    }

    #[doc="Get the *const pointer for the TAFL register."]
    #[inline] pub fn tafl_ptr(&self) -> *const Tafl { 
           self.tafl_mut()
    }

    #[doc="Read the TAFL register."]
    #[inline] pub fn tafl(&self) -> Tafl { 
        unsafe {
            read_volatile(self.tafl_ptr())
        }
    }

    #[doc="Write the TAFL register."]
    #[inline] pub fn set_tafl<F: FnOnce(Tafl) -> Tafl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tafl_mut(), f(Tafl(0)));
        }
        self
    }

    #[doc="Modify the TAFL register."]
    #[inline] pub fn with_tafl<F: FnOnce(Tafl) -> Tafl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tafl_mut(), f(self.tafl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TIPG register."]
    #[inline] pub fn tipg_mut(&self) -> *mut Tipg { 
        (self.0 + 0x1ac) as *mut Tipg
    }

    #[doc="Get the *const pointer for the TIPG register."]
    #[inline] pub fn tipg_ptr(&self) -> *const Tipg { 
           self.tipg_mut()
    }

    #[doc="Read the TIPG register."]
    #[inline] pub fn tipg(&self) -> Tipg { 
        unsafe {
            read_volatile(self.tipg_ptr())
        }
    }

    #[doc="Write the TIPG register."]
    #[inline] pub fn set_tipg<F: FnOnce(Tipg) -> Tipg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tipg_mut(), f(Tipg(0)));
        }
        self
    }

    #[doc="Modify the TIPG register."]
    #[inline] pub fn with_tipg<F: FnOnce(Tipg) -> Tipg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tipg_mut(), f(self.tipg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTRL register."]
    #[inline] pub fn ftrl_mut(&self) -> *mut Ftrl { 
        (self.0 + 0x1b0) as *mut Ftrl
    }

    #[doc="Get the *const pointer for the FTRL register."]
    #[inline] pub fn ftrl_ptr(&self) -> *const Ftrl { 
           self.ftrl_mut()
    }

    #[doc="Read the FTRL register."]
    #[inline] pub fn ftrl(&self) -> Ftrl { 
        unsafe {
            read_volatile(self.ftrl_ptr())
        }
    }

    #[doc="Write the FTRL register."]
    #[inline] pub fn set_ftrl<F: FnOnce(Ftrl) -> Ftrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftrl_mut(), f(Ftrl(0)));
        }
        self
    }

    #[doc="Modify the FTRL register."]
    #[inline] pub fn with_ftrl<F: FnOnce(Ftrl) -> Ftrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftrl_mut(), f(self.ftrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TACC register."]
    #[inline] pub fn tacc_mut(&self) -> *mut Tacc { 
        (self.0 + 0x1c0) as *mut Tacc
    }

    #[doc="Get the *const pointer for the TACC register."]
    #[inline] pub fn tacc_ptr(&self) -> *const Tacc { 
           self.tacc_mut()
    }

    #[doc="Read the TACC register."]
    #[inline] pub fn tacc(&self) -> Tacc { 
        unsafe {
            read_volatile(self.tacc_ptr())
        }
    }

    #[doc="Write the TACC register."]
    #[inline] pub fn set_tacc<F: FnOnce(Tacc) -> Tacc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tacc_mut(), f(Tacc(0)));
        }
        self
    }

    #[doc="Modify the TACC register."]
    #[inline] pub fn with_tacc<F: FnOnce(Tacc) -> Tacc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tacc_mut(), f(self.tacc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RACC register."]
    #[inline] pub fn racc_mut(&self) -> *mut Racc { 
        (self.0 + 0x1c4) as *mut Racc
    }

    #[doc="Get the *const pointer for the RACC register."]
    #[inline] pub fn racc_ptr(&self) -> *const Racc { 
           self.racc_mut()
    }

    #[doc="Read the RACC register."]
    #[inline] pub fn racc(&self) -> Racc { 
        unsafe {
            read_volatile(self.racc_ptr())
        }
    }

    #[doc="Write the RACC register."]
    #[inline] pub fn set_racc<F: FnOnce(Racc) -> Racc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.racc_mut(), f(Racc(0)));
        }
        self
    }

    #[doc="Modify the RACC register."]
    #[inline] pub fn with_racc<F: FnOnce(Racc) -> Racc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.racc_mut(), f(self.racc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RMON_T_PACKETS register."]
    #[inline] pub fn rmon_t_packets_mut(&self) -> *mut RmonTPackets { 
        (self.0 + 0x204) as *mut RmonTPackets
    }

    #[doc="Get the *const pointer for the RMON_T_PACKETS register."]
    #[inline] pub fn rmon_t_packets_ptr(&self) -> *const RmonTPackets { 
           self.rmon_t_packets_mut()
    }

    #[doc="Read the RMON_T_PACKETS register."]
    #[inline] pub fn rmon_t_packets(&self) -> RmonTPackets { 
        unsafe {
            read_volatile(self.rmon_t_packets_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_BC_PKT register."]
    #[inline] pub fn rmon_t_bc_pkt_mut(&self) -> *mut RmonTBcPkt { 
        (self.0 + 0x208) as *mut RmonTBcPkt
    }

    #[doc="Get the *const pointer for the RMON_T_BC_PKT register."]
    #[inline] pub fn rmon_t_bc_pkt_ptr(&self) -> *const RmonTBcPkt { 
           self.rmon_t_bc_pkt_mut()
    }

    #[doc="Read the RMON_T_BC_PKT register."]
    #[inline] pub fn rmon_t_bc_pkt(&self) -> RmonTBcPkt { 
        unsafe {
            read_volatile(self.rmon_t_bc_pkt_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_MC_PKT register."]
    #[inline] pub fn rmon_t_mc_pkt_mut(&self) -> *mut RmonTMcPkt { 
        (self.0 + 0x20c) as *mut RmonTMcPkt
    }

    #[doc="Get the *const pointer for the RMON_T_MC_PKT register."]
    #[inline] pub fn rmon_t_mc_pkt_ptr(&self) -> *const RmonTMcPkt { 
           self.rmon_t_mc_pkt_mut()
    }

    #[doc="Read the RMON_T_MC_PKT register."]
    #[inline] pub fn rmon_t_mc_pkt(&self) -> RmonTMcPkt { 
        unsafe {
            read_volatile(self.rmon_t_mc_pkt_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_CRC_ALIGN register."]
    #[inline] pub fn rmon_t_crc_align_mut(&self) -> *mut RmonTCrcAlign { 
        (self.0 + 0x210) as *mut RmonTCrcAlign
    }

    #[doc="Get the *const pointer for the RMON_T_CRC_ALIGN register."]
    #[inline] pub fn rmon_t_crc_align_ptr(&self) -> *const RmonTCrcAlign { 
           self.rmon_t_crc_align_mut()
    }

    #[doc="Read the RMON_T_CRC_ALIGN register."]
    #[inline] pub fn rmon_t_crc_align(&self) -> RmonTCrcAlign { 
        unsafe {
            read_volatile(self.rmon_t_crc_align_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_UNDERSIZE register."]
    #[inline] pub fn rmon_t_undersize_mut(&self) -> *mut RmonTUndersize { 
        (self.0 + 0x214) as *mut RmonTUndersize
    }

    #[doc="Get the *const pointer for the RMON_T_UNDERSIZE register."]
    #[inline] pub fn rmon_t_undersize_ptr(&self) -> *const RmonTUndersize { 
           self.rmon_t_undersize_mut()
    }

    #[doc="Read the RMON_T_UNDERSIZE register."]
    #[inline] pub fn rmon_t_undersize(&self) -> RmonTUndersize { 
        unsafe {
            read_volatile(self.rmon_t_undersize_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_OVERSIZE register."]
    #[inline] pub fn rmon_t_oversize_mut(&self) -> *mut RmonTOversize { 
        (self.0 + 0x218) as *mut RmonTOversize
    }

    #[doc="Get the *const pointer for the RMON_T_OVERSIZE register."]
    #[inline] pub fn rmon_t_oversize_ptr(&self) -> *const RmonTOversize { 
           self.rmon_t_oversize_mut()
    }

    #[doc="Read the RMON_T_OVERSIZE register."]
    #[inline] pub fn rmon_t_oversize(&self) -> RmonTOversize { 
        unsafe {
            read_volatile(self.rmon_t_oversize_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_FRAG register."]
    #[inline] pub fn rmon_t_frag_mut(&self) -> *mut RmonTFrag { 
        (self.0 + 0x21c) as *mut RmonTFrag
    }

    #[doc="Get the *const pointer for the RMON_T_FRAG register."]
    #[inline] pub fn rmon_t_frag_ptr(&self) -> *const RmonTFrag { 
           self.rmon_t_frag_mut()
    }

    #[doc="Read the RMON_T_FRAG register."]
    #[inline] pub fn rmon_t_frag(&self) -> RmonTFrag { 
        unsafe {
            read_volatile(self.rmon_t_frag_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_JAB register."]
    #[inline] pub fn rmon_t_jab_mut(&self) -> *mut RmonTJab { 
        (self.0 + 0x220) as *mut RmonTJab
    }

    #[doc="Get the *const pointer for the RMON_T_JAB register."]
    #[inline] pub fn rmon_t_jab_ptr(&self) -> *const RmonTJab { 
           self.rmon_t_jab_mut()
    }

    #[doc="Read the RMON_T_JAB register."]
    #[inline] pub fn rmon_t_jab(&self) -> RmonTJab { 
        unsafe {
            read_volatile(self.rmon_t_jab_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_COL register."]
    #[inline] pub fn rmon_t_col_mut(&self) -> *mut RmonTCol { 
        (self.0 + 0x224) as *mut RmonTCol
    }

    #[doc="Get the *const pointer for the RMON_T_COL register."]
    #[inline] pub fn rmon_t_col_ptr(&self) -> *const RmonTCol { 
           self.rmon_t_col_mut()
    }

    #[doc="Read the RMON_T_COL register."]
    #[inline] pub fn rmon_t_col(&self) -> RmonTCol { 
        unsafe {
            read_volatile(self.rmon_t_col_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_P64 register."]
    #[inline] pub fn rmon_t_p64_mut(&self) -> *mut RmonTP64 { 
        (self.0 + 0x228) as *mut RmonTP64
    }

    #[doc="Get the *const pointer for the RMON_T_P64 register."]
    #[inline] pub fn rmon_t_p64_ptr(&self) -> *const RmonTP64 { 
           self.rmon_t_p64_mut()
    }

    #[doc="Read the RMON_T_P64 register."]
    #[inline] pub fn rmon_t_p64(&self) -> RmonTP64 { 
        unsafe {
            read_volatile(self.rmon_t_p64_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_P65TO127 register."]
    #[inline] pub fn rmon_t_p65to127_mut(&self) -> *mut RmonTP65to127 { 
        (self.0 + 0x22c) as *mut RmonTP65to127
    }

    #[doc="Get the *const pointer for the RMON_T_P65TO127 register."]
    #[inline] pub fn rmon_t_p65to127_ptr(&self) -> *const RmonTP65to127 { 
           self.rmon_t_p65to127_mut()
    }

    #[doc="Read the RMON_T_P65TO127 register."]
    #[inline] pub fn rmon_t_p65to127(&self) -> RmonTP65to127 { 
        unsafe {
            read_volatile(self.rmon_t_p65to127_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_P128TO255 register."]
    #[inline] pub fn rmon_t_p128to255_mut(&self) -> *mut RmonTP128to255 { 
        (self.0 + 0x230) as *mut RmonTP128to255
    }

    #[doc="Get the *const pointer for the RMON_T_P128TO255 register."]
    #[inline] pub fn rmon_t_p128to255_ptr(&self) -> *const RmonTP128to255 { 
           self.rmon_t_p128to255_mut()
    }

    #[doc="Read the RMON_T_P128TO255 register."]
    #[inline] pub fn rmon_t_p128to255(&self) -> RmonTP128to255 { 
        unsafe {
            read_volatile(self.rmon_t_p128to255_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_P256TO511 register."]
    #[inline] pub fn rmon_t_p256to511_mut(&self) -> *mut RmonTP256to511 { 
        (self.0 + 0x234) as *mut RmonTP256to511
    }

    #[doc="Get the *const pointer for the RMON_T_P256TO511 register."]
    #[inline] pub fn rmon_t_p256to511_ptr(&self) -> *const RmonTP256to511 { 
           self.rmon_t_p256to511_mut()
    }

    #[doc="Read the RMON_T_P256TO511 register."]
    #[inline] pub fn rmon_t_p256to511(&self) -> RmonTP256to511 { 
        unsafe {
            read_volatile(self.rmon_t_p256to511_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_P512TO1023 register."]
    #[inline] pub fn rmon_t_p512to1023_mut(&self) -> *mut RmonTP512to1023 { 
        (self.0 + 0x238) as *mut RmonTP512to1023
    }

    #[doc="Get the *const pointer for the RMON_T_P512TO1023 register."]
    #[inline] pub fn rmon_t_p512to1023_ptr(&self) -> *const RmonTP512to1023 { 
           self.rmon_t_p512to1023_mut()
    }

    #[doc="Read the RMON_T_P512TO1023 register."]
    #[inline] pub fn rmon_t_p512to1023(&self) -> RmonTP512to1023 { 
        unsafe {
            read_volatile(self.rmon_t_p512to1023_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_P1024TO2047 register."]
    #[inline] pub fn rmon_t_p1024to2047_mut(&self) -> *mut RmonTP1024to2047 { 
        (self.0 + 0x23c) as *mut RmonTP1024to2047
    }

    #[doc="Get the *const pointer for the RMON_T_P1024TO2047 register."]
    #[inline] pub fn rmon_t_p1024to2047_ptr(&self) -> *const RmonTP1024to2047 { 
           self.rmon_t_p1024to2047_mut()
    }

    #[doc="Read the RMON_T_P1024TO2047 register."]
    #[inline] pub fn rmon_t_p1024to2047(&self) -> RmonTP1024to2047 { 
        unsafe {
            read_volatile(self.rmon_t_p1024to2047_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_P_GTE2048 register."]
    #[inline] pub fn rmon_t_p_gte2048_mut(&self) -> *mut RmonTPGte2048 { 
        (self.0 + 0x240) as *mut RmonTPGte2048
    }

    #[doc="Get the *const pointer for the RMON_T_P_GTE2048 register."]
    #[inline] pub fn rmon_t_p_gte2048_ptr(&self) -> *const RmonTPGte2048 { 
           self.rmon_t_p_gte2048_mut()
    }

    #[doc="Read the RMON_T_P_GTE2048 register."]
    #[inline] pub fn rmon_t_p_gte2048(&self) -> RmonTPGte2048 { 
        unsafe {
            read_volatile(self.rmon_t_p_gte2048_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_T_OCTETS register."]
    #[inline] pub fn rmon_t_octets_mut(&self) -> *mut RmonTOctets { 
        (self.0 + 0x244) as *mut RmonTOctets
    }

    #[doc="Get the *const pointer for the RMON_T_OCTETS register."]
    #[inline] pub fn rmon_t_octets_ptr(&self) -> *const RmonTOctets { 
           self.rmon_t_octets_mut()
    }

    #[doc="Read the RMON_T_OCTETS register."]
    #[inline] pub fn rmon_t_octets(&self) -> RmonTOctets { 
        unsafe {
            read_volatile(self.rmon_t_octets_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_FRAME_OK register."]
    #[inline] pub fn ieee_t_frame_ok_mut(&self) -> *mut IeeeTFrameOk { 
        (self.0 + 0x24c) as *mut IeeeTFrameOk
    }

    #[doc="Get the *const pointer for the IEEE_T_FRAME_OK register."]
    #[inline] pub fn ieee_t_frame_ok_ptr(&self) -> *const IeeeTFrameOk { 
           self.ieee_t_frame_ok_mut()
    }

    #[doc="Read the IEEE_T_FRAME_OK register."]
    #[inline] pub fn ieee_t_frame_ok(&self) -> IeeeTFrameOk { 
        unsafe {
            read_volatile(self.ieee_t_frame_ok_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_1COL register."]
    #[inline] pub fn ieee_t_1col_mut(&self) -> *mut IeeeT1col { 
        (self.0 + 0x250) as *mut IeeeT1col
    }

    #[doc="Get the *const pointer for the IEEE_T_1COL register."]
    #[inline] pub fn ieee_t_1col_ptr(&self) -> *const IeeeT1col { 
           self.ieee_t_1col_mut()
    }

    #[doc="Read the IEEE_T_1COL register."]
    #[inline] pub fn ieee_t_1col(&self) -> IeeeT1col { 
        unsafe {
            read_volatile(self.ieee_t_1col_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_MCOL register."]
    #[inline] pub fn ieee_t_mcol_mut(&self) -> *mut IeeeTMcol { 
        (self.0 + 0x254) as *mut IeeeTMcol
    }

    #[doc="Get the *const pointer for the IEEE_T_MCOL register."]
    #[inline] pub fn ieee_t_mcol_ptr(&self) -> *const IeeeTMcol { 
           self.ieee_t_mcol_mut()
    }

    #[doc="Read the IEEE_T_MCOL register."]
    #[inline] pub fn ieee_t_mcol(&self) -> IeeeTMcol { 
        unsafe {
            read_volatile(self.ieee_t_mcol_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_DEF register."]
    #[inline] pub fn ieee_t_def_mut(&self) -> *mut IeeeTDef { 
        (self.0 + 0x258) as *mut IeeeTDef
    }

    #[doc="Get the *const pointer for the IEEE_T_DEF register."]
    #[inline] pub fn ieee_t_def_ptr(&self) -> *const IeeeTDef { 
           self.ieee_t_def_mut()
    }

    #[doc="Read the IEEE_T_DEF register."]
    #[inline] pub fn ieee_t_def(&self) -> IeeeTDef { 
        unsafe {
            read_volatile(self.ieee_t_def_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_LCOL register."]
    #[inline] pub fn ieee_t_lcol_mut(&self) -> *mut IeeeTLcol { 
        (self.0 + 0x25c) as *mut IeeeTLcol
    }

    #[doc="Get the *const pointer for the IEEE_T_LCOL register."]
    #[inline] pub fn ieee_t_lcol_ptr(&self) -> *const IeeeTLcol { 
           self.ieee_t_lcol_mut()
    }

    #[doc="Read the IEEE_T_LCOL register."]
    #[inline] pub fn ieee_t_lcol(&self) -> IeeeTLcol { 
        unsafe {
            read_volatile(self.ieee_t_lcol_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_EXCOL register."]
    #[inline] pub fn ieee_t_excol_mut(&self) -> *mut IeeeTExcol { 
        (self.0 + 0x260) as *mut IeeeTExcol
    }

    #[doc="Get the *const pointer for the IEEE_T_EXCOL register."]
    #[inline] pub fn ieee_t_excol_ptr(&self) -> *const IeeeTExcol { 
           self.ieee_t_excol_mut()
    }

    #[doc="Read the IEEE_T_EXCOL register."]
    #[inline] pub fn ieee_t_excol(&self) -> IeeeTExcol { 
        unsafe {
            read_volatile(self.ieee_t_excol_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_MACERR register."]
    #[inline] pub fn ieee_t_macerr_mut(&self) -> *mut IeeeTMacerr { 
        (self.0 + 0x264) as *mut IeeeTMacerr
    }

    #[doc="Get the *const pointer for the IEEE_T_MACERR register."]
    #[inline] pub fn ieee_t_macerr_ptr(&self) -> *const IeeeTMacerr { 
           self.ieee_t_macerr_mut()
    }

    #[doc="Read the IEEE_T_MACERR register."]
    #[inline] pub fn ieee_t_macerr(&self) -> IeeeTMacerr { 
        unsafe {
            read_volatile(self.ieee_t_macerr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_CSERR register."]
    #[inline] pub fn ieee_t_cserr_mut(&self) -> *mut IeeeTCserr { 
        (self.0 + 0x268) as *mut IeeeTCserr
    }

    #[doc="Get the *const pointer for the IEEE_T_CSERR register."]
    #[inline] pub fn ieee_t_cserr_ptr(&self) -> *const IeeeTCserr { 
           self.ieee_t_cserr_mut()
    }

    #[doc="Read the IEEE_T_CSERR register."]
    #[inline] pub fn ieee_t_cserr(&self) -> IeeeTCserr { 
        unsafe {
            read_volatile(self.ieee_t_cserr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_FDXFC register."]
    #[inline] pub fn ieee_t_fdxfc_mut(&self) -> *mut IeeeTFdxfc { 
        (self.0 + 0x270) as *mut IeeeTFdxfc
    }

    #[doc="Get the *const pointer for the IEEE_T_FDXFC register."]
    #[inline] pub fn ieee_t_fdxfc_ptr(&self) -> *const IeeeTFdxfc { 
           self.ieee_t_fdxfc_mut()
    }

    #[doc="Read the IEEE_T_FDXFC register."]
    #[inline] pub fn ieee_t_fdxfc(&self) -> IeeeTFdxfc { 
        unsafe {
            read_volatile(self.ieee_t_fdxfc_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_T_OCTETS_OK register."]
    #[inline] pub fn ieee_t_octets_ok_mut(&self) -> *mut IeeeTOctetsOk { 
        (self.0 + 0x274) as *mut IeeeTOctetsOk
    }

    #[doc="Get the *const pointer for the IEEE_T_OCTETS_OK register."]
    #[inline] pub fn ieee_t_octets_ok_ptr(&self) -> *const IeeeTOctetsOk { 
           self.ieee_t_octets_ok_mut()
    }

    #[doc="Read the IEEE_T_OCTETS_OK register."]
    #[inline] pub fn ieee_t_octets_ok(&self) -> IeeeTOctetsOk { 
        unsafe {
            read_volatile(self.ieee_t_octets_ok_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_PACKETS register."]
    #[inline] pub fn rmon_r_packets_mut(&self) -> *mut RmonRPackets { 
        (self.0 + 0x284) as *mut RmonRPackets
    }

    #[doc="Get the *const pointer for the RMON_R_PACKETS register."]
    #[inline] pub fn rmon_r_packets_ptr(&self) -> *const RmonRPackets { 
           self.rmon_r_packets_mut()
    }

    #[doc="Read the RMON_R_PACKETS register."]
    #[inline] pub fn rmon_r_packets(&self) -> RmonRPackets { 
        unsafe {
            read_volatile(self.rmon_r_packets_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_BC_PKT register."]
    #[inline] pub fn rmon_r_bc_pkt_mut(&self) -> *mut RmonRBcPkt { 
        (self.0 + 0x288) as *mut RmonRBcPkt
    }

    #[doc="Get the *const pointer for the RMON_R_BC_PKT register."]
    #[inline] pub fn rmon_r_bc_pkt_ptr(&self) -> *const RmonRBcPkt { 
           self.rmon_r_bc_pkt_mut()
    }

    #[doc="Read the RMON_R_BC_PKT register."]
    #[inline] pub fn rmon_r_bc_pkt(&self) -> RmonRBcPkt { 
        unsafe {
            read_volatile(self.rmon_r_bc_pkt_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_MC_PKT register."]
    #[inline] pub fn rmon_r_mc_pkt_mut(&self) -> *mut RmonRMcPkt { 
        (self.0 + 0x28c) as *mut RmonRMcPkt
    }

    #[doc="Get the *const pointer for the RMON_R_MC_PKT register."]
    #[inline] pub fn rmon_r_mc_pkt_ptr(&self) -> *const RmonRMcPkt { 
           self.rmon_r_mc_pkt_mut()
    }

    #[doc="Read the RMON_R_MC_PKT register."]
    #[inline] pub fn rmon_r_mc_pkt(&self) -> RmonRMcPkt { 
        unsafe {
            read_volatile(self.rmon_r_mc_pkt_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_CRC_ALIGN register."]
    #[inline] pub fn rmon_r_crc_align_mut(&self) -> *mut RmonRCrcAlign { 
        (self.0 + 0x290) as *mut RmonRCrcAlign
    }

    #[doc="Get the *const pointer for the RMON_R_CRC_ALIGN register."]
    #[inline] pub fn rmon_r_crc_align_ptr(&self) -> *const RmonRCrcAlign { 
           self.rmon_r_crc_align_mut()
    }

    #[doc="Read the RMON_R_CRC_ALIGN register."]
    #[inline] pub fn rmon_r_crc_align(&self) -> RmonRCrcAlign { 
        unsafe {
            read_volatile(self.rmon_r_crc_align_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_UNDERSIZE register."]
    #[inline] pub fn rmon_r_undersize_mut(&self) -> *mut RmonRUndersize { 
        (self.0 + 0x294) as *mut RmonRUndersize
    }

    #[doc="Get the *const pointer for the RMON_R_UNDERSIZE register."]
    #[inline] pub fn rmon_r_undersize_ptr(&self) -> *const RmonRUndersize { 
           self.rmon_r_undersize_mut()
    }

    #[doc="Read the RMON_R_UNDERSIZE register."]
    #[inline] pub fn rmon_r_undersize(&self) -> RmonRUndersize { 
        unsafe {
            read_volatile(self.rmon_r_undersize_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_OVERSIZE register."]
    #[inline] pub fn rmon_r_oversize_mut(&self) -> *mut RmonROversize { 
        (self.0 + 0x298) as *mut RmonROversize
    }

    #[doc="Get the *const pointer for the RMON_R_OVERSIZE register."]
    #[inline] pub fn rmon_r_oversize_ptr(&self) -> *const RmonROversize { 
           self.rmon_r_oversize_mut()
    }

    #[doc="Read the RMON_R_OVERSIZE register."]
    #[inline] pub fn rmon_r_oversize(&self) -> RmonROversize { 
        unsafe {
            read_volatile(self.rmon_r_oversize_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_FRAG register."]
    #[inline] pub fn rmon_r_frag_mut(&self) -> *mut RmonRFrag { 
        (self.0 + 0x29c) as *mut RmonRFrag
    }

    #[doc="Get the *const pointer for the RMON_R_FRAG register."]
    #[inline] pub fn rmon_r_frag_ptr(&self) -> *const RmonRFrag { 
           self.rmon_r_frag_mut()
    }

    #[doc="Read the RMON_R_FRAG register."]
    #[inline] pub fn rmon_r_frag(&self) -> RmonRFrag { 
        unsafe {
            read_volatile(self.rmon_r_frag_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_JAB register."]
    #[inline] pub fn rmon_r_jab_mut(&self) -> *mut RmonRJab { 
        (self.0 + 0x2a0) as *mut RmonRJab
    }

    #[doc="Get the *const pointer for the RMON_R_JAB register."]
    #[inline] pub fn rmon_r_jab_ptr(&self) -> *const RmonRJab { 
           self.rmon_r_jab_mut()
    }

    #[doc="Read the RMON_R_JAB register."]
    #[inline] pub fn rmon_r_jab(&self) -> RmonRJab { 
        unsafe {
            read_volatile(self.rmon_r_jab_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_P64 register."]
    #[inline] pub fn rmon_r_p64_mut(&self) -> *mut RmonRP64 { 
        (self.0 + 0x2a8) as *mut RmonRP64
    }

    #[doc="Get the *const pointer for the RMON_R_P64 register."]
    #[inline] pub fn rmon_r_p64_ptr(&self) -> *const RmonRP64 { 
           self.rmon_r_p64_mut()
    }

    #[doc="Read the RMON_R_P64 register."]
    #[inline] pub fn rmon_r_p64(&self) -> RmonRP64 { 
        unsafe {
            read_volatile(self.rmon_r_p64_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_P65TO127 register."]
    #[inline] pub fn rmon_r_p65to127_mut(&self) -> *mut RmonRP65to127 { 
        (self.0 + 0x2ac) as *mut RmonRP65to127
    }

    #[doc="Get the *const pointer for the RMON_R_P65TO127 register."]
    #[inline] pub fn rmon_r_p65to127_ptr(&self) -> *const RmonRP65to127 { 
           self.rmon_r_p65to127_mut()
    }

    #[doc="Read the RMON_R_P65TO127 register."]
    #[inline] pub fn rmon_r_p65to127(&self) -> RmonRP65to127 { 
        unsafe {
            read_volatile(self.rmon_r_p65to127_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_P128TO255 register."]
    #[inline] pub fn rmon_r_p128to255_mut(&self) -> *mut RmonRP128to255 { 
        (self.0 + 0x2b0) as *mut RmonRP128to255
    }

    #[doc="Get the *const pointer for the RMON_R_P128TO255 register."]
    #[inline] pub fn rmon_r_p128to255_ptr(&self) -> *const RmonRP128to255 { 
           self.rmon_r_p128to255_mut()
    }

    #[doc="Read the RMON_R_P128TO255 register."]
    #[inline] pub fn rmon_r_p128to255(&self) -> RmonRP128to255 { 
        unsafe {
            read_volatile(self.rmon_r_p128to255_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_P256TO511 register."]
    #[inline] pub fn rmon_r_p256to511_mut(&self) -> *mut RmonRP256to511 { 
        (self.0 + 0x2b4) as *mut RmonRP256to511
    }

    #[doc="Get the *const pointer for the RMON_R_P256TO511 register."]
    #[inline] pub fn rmon_r_p256to511_ptr(&self) -> *const RmonRP256to511 { 
           self.rmon_r_p256to511_mut()
    }

    #[doc="Read the RMON_R_P256TO511 register."]
    #[inline] pub fn rmon_r_p256to511(&self) -> RmonRP256to511 { 
        unsafe {
            read_volatile(self.rmon_r_p256to511_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_P512TO1023 register."]
    #[inline] pub fn rmon_r_p512to1023_mut(&self) -> *mut RmonRP512to1023 { 
        (self.0 + 0x2b8) as *mut RmonRP512to1023
    }

    #[doc="Get the *const pointer for the RMON_R_P512TO1023 register."]
    #[inline] pub fn rmon_r_p512to1023_ptr(&self) -> *const RmonRP512to1023 { 
           self.rmon_r_p512to1023_mut()
    }

    #[doc="Read the RMON_R_P512TO1023 register."]
    #[inline] pub fn rmon_r_p512to1023(&self) -> RmonRP512to1023 { 
        unsafe {
            read_volatile(self.rmon_r_p512to1023_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_P1024TO2047 register."]
    #[inline] pub fn rmon_r_p1024to2047_mut(&self) -> *mut RmonRP1024to2047 { 
        (self.0 + 0x2bc) as *mut RmonRP1024to2047
    }

    #[doc="Get the *const pointer for the RMON_R_P1024TO2047 register."]
    #[inline] pub fn rmon_r_p1024to2047_ptr(&self) -> *const RmonRP1024to2047 { 
           self.rmon_r_p1024to2047_mut()
    }

    #[doc="Read the RMON_R_P1024TO2047 register."]
    #[inline] pub fn rmon_r_p1024to2047(&self) -> RmonRP1024to2047 { 
        unsafe {
            read_volatile(self.rmon_r_p1024to2047_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_P_GTE2048 register."]
    #[inline] pub fn rmon_r_p_gte2048_mut(&self) -> *mut RmonRPGte2048 { 
        (self.0 + 0x2c0) as *mut RmonRPGte2048
    }

    #[doc="Get the *const pointer for the RMON_R_P_GTE2048 register."]
    #[inline] pub fn rmon_r_p_gte2048_ptr(&self) -> *const RmonRPGte2048 { 
           self.rmon_r_p_gte2048_mut()
    }

    #[doc="Read the RMON_R_P_GTE2048 register."]
    #[inline] pub fn rmon_r_p_gte2048(&self) -> RmonRPGte2048 { 
        unsafe {
            read_volatile(self.rmon_r_p_gte2048_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RMON_R_OCTETS register."]
    #[inline] pub fn rmon_r_octets_mut(&self) -> *mut RmonROctets { 
        (self.0 + 0x2c4) as *mut RmonROctets
    }

    #[doc="Get the *const pointer for the RMON_R_OCTETS register."]
    #[inline] pub fn rmon_r_octets_ptr(&self) -> *const RmonROctets { 
           self.rmon_r_octets_mut()
    }

    #[doc="Read the RMON_R_OCTETS register."]
    #[inline] pub fn rmon_r_octets(&self) -> RmonROctets { 
        unsafe {
            read_volatile(self.rmon_r_octets_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_R_DROP register."]
    #[inline] pub fn ieee_r_drop_mut(&self) -> *mut IeeeRDrop { 
        (self.0 + 0x2c8) as *mut IeeeRDrop
    }

    #[doc="Get the *const pointer for the IEEE_R_DROP register."]
    #[inline] pub fn ieee_r_drop_ptr(&self) -> *const IeeeRDrop { 
           self.ieee_r_drop_mut()
    }

    #[doc="Read the IEEE_R_DROP register."]
    #[inline] pub fn ieee_r_drop(&self) -> IeeeRDrop { 
        unsafe {
            read_volatile(self.ieee_r_drop_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_R_FRAME_OK register."]
    #[inline] pub fn ieee_r_frame_ok_mut(&self) -> *mut IeeeRFrameOk { 
        (self.0 + 0x2cc) as *mut IeeeRFrameOk
    }

    #[doc="Get the *const pointer for the IEEE_R_FRAME_OK register."]
    #[inline] pub fn ieee_r_frame_ok_ptr(&self) -> *const IeeeRFrameOk { 
           self.ieee_r_frame_ok_mut()
    }

    #[doc="Read the IEEE_R_FRAME_OK register."]
    #[inline] pub fn ieee_r_frame_ok(&self) -> IeeeRFrameOk { 
        unsafe {
            read_volatile(self.ieee_r_frame_ok_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_R_CRC register."]
    #[inline] pub fn ieee_r_crc_mut(&self) -> *mut IeeeRCrc { 
        (self.0 + 0x2d0) as *mut IeeeRCrc
    }

    #[doc="Get the *const pointer for the IEEE_R_CRC register."]
    #[inline] pub fn ieee_r_crc_ptr(&self) -> *const IeeeRCrc { 
           self.ieee_r_crc_mut()
    }

    #[doc="Read the IEEE_R_CRC register."]
    #[inline] pub fn ieee_r_crc(&self) -> IeeeRCrc { 
        unsafe {
            read_volatile(self.ieee_r_crc_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_R_ALIGN register."]
    #[inline] pub fn ieee_r_align_mut(&self) -> *mut IeeeRAlign { 
        (self.0 + 0x2d4) as *mut IeeeRAlign
    }

    #[doc="Get the *const pointer for the IEEE_R_ALIGN register."]
    #[inline] pub fn ieee_r_align_ptr(&self) -> *const IeeeRAlign { 
           self.ieee_r_align_mut()
    }

    #[doc="Read the IEEE_R_ALIGN register."]
    #[inline] pub fn ieee_r_align(&self) -> IeeeRAlign { 
        unsafe {
            read_volatile(self.ieee_r_align_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_R_MACERR register."]
    #[inline] pub fn ieee_r_macerr_mut(&self) -> *mut IeeeRMacerr { 
        (self.0 + 0x2d8) as *mut IeeeRMacerr
    }

    #[doc="Get the *const pointer for the IEEE_R_MACERR register."]
    #[inline] pub fn ieee_r_macerr_ptr(&self) -> *const IeeeRMacerr { 
           self.ieee_r_macerr_mut()
    }

    #[doc="Read the IEEE_R_MACERR register."]
    #[inline] pub fn ieee_r_macerr(&self) -> IeeeRMacerr { 
        unsafe {
            read_volatile(self.ieee_r_macerr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_R_FDXFC register."]
    #[inline] pub fn ieee_r_fdxfc_mut(&self) -> *mut IeeeRFdxfc { 
        (self.0 + 0x2dc) as *mut IeeeRFdxfc
    }

    #[doc="Get the *const pointer for the IEEE_R_FDXFC register."]
    #[inline] pub fn ieee_r_fdxfc_ptr(&self) -> *const IeeeRFdxfc { 
           self.ieee_r_fdxfc_mut()
    }

    #[doc="Read the IEEE_R_FDXFC register."]
    #[inline] pub fn ieee_r_fdxfc(&self) -> IeeeRFdxfc { 
        unsafe {
            read_volatile(self.ieee_r_fdxfc_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IEEE_R_OCTETS_OK register."]
    #[inline] pub fn ieee_r_octets_ok_mut(&self) -> *mut IeeeROctetsOk { 
        (self.0 + 0x2e0) as *mut IeeeROctetsOk
    }

    #[doc="Get the *const pointer for the IEEE_R_OCTETS_OK register."]
    #[inline] pub fn ieee_r_octets_ok_ptr(&self) -> *const IeeeROctetsOk { 
           self.ieee_r_octets_ok_mut()
    }

    #[doc="Read the IEEE_R_OCTETS_OK register."]
    #[inline] pub fn ieee_r_octets_ok(&self) -> IeeeROctetsOk { 
        unsafe {
            read_volatile(self.ieee_r_octets_ok_ptr())
        }
    }

    #[doc="Get the *mut pointer for the ATCR register."]
    #[inline] pub fn atcr_mut(&self) -> *mut Atcr { 
        (self.0 + 0x400) as *mut Atcr
    }

    #[doc="Get the *const pointer for the ATCR register."]
    #[inline] pub fn atcr_ptr(&self) -> *const Atcr { 
           self.atcr_mut()
    }

    #[doc="Read the ATCR register."]
    #[inline] pub fn atcr(&self) -> Atcr { 
        unsafe {
            read_volatile(self.atcr_ptr())
        }
    }

    #[doc="Write the ATCR register."]
    #[inline] pub fn set_atcr<F: FnOnce(Atcr) -> Atcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atcr_mut(), f(Atcr(0)));
        }
        self
    }

    #[doc="Modify the ATCR register."]
    #[inline] pub fn with_atcr<F: FnOnce(Atcr) -> Atcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atcr_mut(), f(self.atcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ATVR register."]
    #[inline] pub fn atvr_mut(&self) -> *mut Atvr { 
        (self.0 + 0x404) as *mut Atvr
    }

    #[doc="Get the *const pointer for the ATVR register."]
    #[inline] pub fn atvr_ptr(&self) -> *const Atvr { 
           self.atvr_mut()
    }

    #[doc="Read the ATVR register."]
    #[inline] pub fn atvr(&self) -> Atvr { 
        unsafe {
            read_volatile(self.atvr_ptr())
        }
    }

    #[doc="Write the ATVR register."]
    #[inline] pub fn set_atvr<F: FnOnce(Atvr) -> Atvr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atvr_mut(), f(Atvr(0)));
        }
        self
    }

    #[doc="Modify the ATVR register."]
    #[inline] pub fn with_atvr<F: FnOnce(Atvr) -> Atvr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atvr_mut(), f(self.atvr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ATOFF register."]
    #[inline] pub fn atoff_mut(&self) -> *mut Atoff { 
        (self.0 + 0x408) as *mut Atoff
    }

    #[doc="Get the *const pointer for the ATOFF register."]
    #[inline] pub fn atoff_ptr(&self) -> *const Atoff { 
           self.atoff_mut()
    }

    #[doc="Read the ATOFF register."]
    #[inline] pub fn atoff(&self) -> Atoff { 
        unsafe {
            read_volatile(self.atoff_ptr())
        }
    }

    #[doc="Write the ATOFF register."]
    #[inline] pub fn set_atoff<F: FnOnce(Atoff) -> Atoff>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atoff_mut(), f(Atoff(0)));
        }
        self
    }

    #[doc="Modify the ATOFF register."]
    #[inline] pub fn with_atoff<F: FnOnce(Atoff) -> Atoff>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atoff_mut(), f(self.atoff()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ATPER register."]
    #[inline] pub fn atper_mut(&self) -> *mut Atper { 
        (self.0 + 0x40c) as *mut Atper
    }

    #[doc="Get the *const pointer for the ATPER register."]
    #[inline] pub fn atper_ptr(&self) -> *const Atper { 
           self.atper_mut()
    }

    #[doc="Read the ATPER register."]
    #[inline] pub fn atper(&self) -> Atper { 
        unsafe {
            read_volatile(self.atper_ptr())
        }
    }

    #[doc="Write the ATPER register."]
    #[inline] pub fn set_atper<F: FnOnce(Atper) -> Atper>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atper_mut(), f(Atper(0)));
        }
        self
    }

    #[doc="Modify the ATPER register."]
    #[inline] pub fn with_atper<F: FnOnce(Atper) -> Atper>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atper_mut(), f(self.atper()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ATCOR register."]
    #[inline] pub fn atcor_mut(&self) -> *mut Atcor { 
        (self.0 + 0x410) as *mut Atcor
    }

    #[doc="Get the *const pointer for the ATCOR register."]
    #[inline] pub fn atcor_ptr(&self) -> *const Atcor { 
           self.atcor_mut()
    }

    #[doc="Read the ATCOR register."]
    #[inline] pub fn atcor(&self) -> Atcor { 
        unsafe {
            read_volatile(self.atcor_ptr())
        }
    }

    #[doc="Write the ATCOR register."]
    #[inline] pub fn set_atcor<F: FnOnce(Atcor) -> Atcor>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atcor_mut(), f(Atcor(0)));
        }
        self
    }

    #[doc="Modify the ATCOR register."]
    #[inline] pub fn with_atcor<F: FnOnce(Atcor) -> Atcor>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atcor_mut(), f(self.atcor()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ATINC register."]
    #[inline] pub fn atinc_mut(&self) -> *mut Atinc { 
        (self.0 + 0x414) as *mut Atinc
    }

    #[doc="Get the *const pointer for the ATINC register."]
    #[inline] pub fn atinc_ptr(&self) -> *const Atinc { 
           self.atinc_mut()
    }

    #[doc="Read the ATINC register."]
    #[inline] pub fn atinc(&self) -> Atinc { 
        unsafe {
            read_volatile(self.atinc_ptr())
        }
    }

    #[doc="Write the ATINC register."]
    #[inline] pub fn set_atinc<F: FnOnce(Atinc) -> Atinc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atinc_mut(), f(Atinc(0)));
        }
        self
    }

    #[doc="Modify the ATINC register."]
    #[inline] pub fn with_atinc<F: FnOnce(Atinc) -> Atinc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atinc_mut(), f(self.atinc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ATSTMP register."]
    #[inline] pub fn atstmp_mut(&self) -> *mut Atstmp { 
        (self.0 + 0x418) as *mut Atstmp
    }

    #[doc="Get the *const pointer for the ATSTMP register."]
    #[inline] pub fn atstmp_ptr(&self) -> *const Atstmp { 
           self.atstmp_mut()
    }

    #[doc="Read the ATSTMP register."]
    #[inline] pub fn atstmp(&self) -> Atstmp { 
        unsafe {
            read_volatile(self.atstmp_ptr())
        }
    }

    #[doc="Get the *mut pointer for the TGSR register."]
    #[inline] pub fn tgsr_mut(&self) -> *mut Tgsr { 
        (self.0 + 0x604) as *mut Tgsr
    }

    #[doc="Get the *const pointer for the TGSR register."]
    #[inline] pub fn tgsr_ptr(&self) -> *const Tgsr { 
           self.tgsr_mut()
    }

    #[doc="Read the TGSR register."]
    #[inline] pub fn tgsr(&self) -> Tgsr { 
        unsafe {
            read_volatile(self.tgsr_ptr())
        }
    }

    #[doc="Write the TGSR register."]
    #[inline] pub fn set_tgsr<F: FnOnce(Tgsr) -> Tgsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tgsr_mut(), f(Tgsr(0)));
        }
        self
    }

    #[doc="Modify the TGSR register."]
    #[inline] pub fn with_tgsr<F: FnOnce(Tgsr) -> Tgsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tgsr_mut(), f(self.tgsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCSR register."]
    #[inline] pub fn tcsr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Tcsr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x608 + (index << 3)) as *mut Tcsr
    }

    #[doc="Get the *const pointer for the TCSR register."]
    #[inline] pub fn tcsr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Tcsr { 
           self.tcsr_mut(index)
    }

    #[doc="Read the TCSR register."]
    #[inline] pub fn tcsr<I: Into<bits::R4>>(&self, index: I) -> Tcsr { 
        unsafe {
            read_volatile(self.tcsr_ptr(index))
        }
    }

    #[doc="Write the TCSR register."]
    #[inline] pub fn set_tcsr<I: Into<bits::R4>, F: FnOnce(Tcsr) -> Tcsr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcsr_mut(index), f(Tcsr(0)));
        }
        self
    }

    #[doc="Modify the TCSR register."]
    #[inline] pub fn with_tcsr<I: Into<bits::R4> + Copy, F: FnOnce(Tcsr) -> Tcsr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tcsr_mut(index), f(self.tcsr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TCCR register."]
    #[inline] pub fn tccr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Tccr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x60c + (index << 3)) as *mut Tccr
    }

    #[doc="Get the *const pointer for the TCCR register."]
    #[inline] pub fn tccr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Tccr { 
           self.tccr_mut(index)
    }

    #[doc="Read the TCCR register."]
    #[inline] pub fn tccr<I: Into<bits::R4>>(&self, index: I) -> Tccr { 
        unsafe {
            read_volatile(self.tccr_ptr(index))
        }
    }

    #[doc="Write the TCCR register."]
    #[inline] pub fn set_tccr<I: Into<bits::R4>, F: FnOnce(Tccr) -> Tccr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tccr_mut(index), f(Tccr(0)));
        }
        self
    }

    #[doc="Modify the TCCR register."]
    #[inline] pub fn with_tccr<I: Into<bits::R4> + Copy, F: FnOnce(Tccr) -> Tccr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tccr_mut(index), f(self.tccr(index)));
        }
        self
    }

}

#[doc="Interrupt Event Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eir(pub u32);
impl Eir {
    #[doc="Timestamp Timer"]
    #[inline] pub fn ts_timer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Timestamp Timer"]
    #[inline] pub fn test_ts_timer(&self) -> bool {
        self.ts_timer() != 0
    }

    #[doc="Timestamp Timer"]
    #[inline] pub fn set_ts_timer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Transmit Timestamp Available"]
    #[inline] pub fn ts_avail(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Transmit Timestamp Available"]
    #[inline] pub fn test_ts_avail(&self) -> bool {
        self.ts_avail() != 0
    }

    #[doc="Transmit Timestamp Available"]
    #[inline] pub fn set_ts_avail<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Node Wakeup Request Indication"]
    #[inline] pub fn wakeup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Node Wakeup Request Indication"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="Node Wakeup Request Indication"]
    #[inline] pub fn set_wakeup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Payload Receive Error"]
    #[inline] pub fn plr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Payload Receive Error"]
    #[inline] pub fn test_plr(&self) -> bool {
        self.plr() != 0
    }

    #[doc="Payload Receive Error"]
    #[inline] pub fn set_plr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Transmit FIFO Underrun"]
    #[inline] pub fn un(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Transmit FIFO Underrun"]
    #[inline] pub fn test_un(&self) -> bool {
        self.un() != 0
    }

    #[doc="Transmit FIFO Underrun"]
    #[inline] pub fn set_un<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Collision Retry Limit"]
    #[inline] pub fn rl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Collision Retry Limit"]
    #[inline] pub fn test_rl(&self) -> bool {
        self.rl() != 0
    }

    #[doc="Collision Retry Limit"]
    #[inline] pub fn set_rl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Late Collision"]
    #[inline] pub fn lc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Late Collision"]
    #[inline] pub fn test_lc(&self) -> bool {
        self.lc() != 0
    }

    #[doc="Late Collision"]
    #[inline] pub fn set_lc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Ethernet Bus Error"]
    #[inline] pub fn eberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Ethernet Bus Error"]
    #[inline] pub fn test_eberr(&self) -> bool {
        self.eberr() != 0
    }

    #[doc="Ethernet Bus Error"]
    #[inline] pub fn set_eberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="MII Interrupt."]
    #[inline] pub fn mii(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="MII Interrupt."]
    #[inline] pub fn test_mii(&self) -> bool {
        self.mii() != 0
    }

    #[doc="MII Interrupt."]
    #[inline] pub fn set_mii<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Receive Buffer Interrupt"]
    #[inline] pub fn rxb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Receive Buffer Interrupt"]
    #[inline] pub fn test_rxb(&self) -> bool {
        self.rxb() != 0
    }

    #[doc="Receive Buffer Interrupt"]
    #[inline] pub fn set_rxb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Receive Frame Interrupt"]
    #[inline] pub fn rxf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Receive Frame Interrupt"]
    #[inline] pub fn test_rxf(&self) -> bool {
        self.rxf() != 0
    }

    #[doc="Receive Frame Interrupt"]
    #[inline] pub fn set_rxf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Transmit Buffer Interrupt"]
    #[inline] pub fn txb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Transmit Buffer Interrupt"]
    #[inline] pub fn test_txb(&self) -> bool {
        self.txb() != 0
    }

    #[doc="Transmit Buffer Interrupt"]
    #[inline] pub fn set_txb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Transmit Frame Interrupt"]
    #[inline] pub fn txf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Transmit Frame Interrupt"]
    #[inline] pub fn test_txf(&self) -> bool {
        self.txf() != 0
    }

    #[doc="Transmit Frame Interrupt"]
    #[inline] pub fn set_txf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Graceful Stop Complete"]
    #[inline] pub fn gra(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Graceful Stop Complete"]
    #[inline] pub fn test_gra(&self) -> bool {
        self.gra() != 0
    }

    #[doc="Graceful Stop Complete"]
    #[inline] pub fn set_gra<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Babbling Transmit Error"]
    #[inline] pub fn babt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Babbling Transmit Error"]
    #[inline] pub fn test_babt(&self) -> bool {
        self.babt() != 0
    }

    #[doc="Babbling Transmit Error"]
    #[inline] pub fn set_babt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Babbling Receive Error"]
    #[inline] pub fn babr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Babbling Receive Error"]
    #[inline] pub fn test_babr(&self) -> bool {
        self.babr() != 0
    }

    #[doc="Babbling Receive Error"]
    #[inline] pub fn set_babr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Eir {
    #[inline]
    fn from(other: u32) -> Self {
         Eir(other)
    }
}

impl ::core::fmt::Display for Eir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ts_timer() != 0 { try!(write!(f, " ts_timer"))}
        if self.ts_avail() != 0 { try!(write!(f, " ts_avail"))}
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.plr() != 0 { try!(write!(f, " plr"))}
        if self.un() != 0 { try!(write!(f, " un"))}
        if self.rl() != 0 { try!(write!(f, " rl"))}
        if self.lc() != 0 { try!(write!(f, " lc"))}
        if self.eberr() != 0 { try!(write!(f, " eberr"))}
        if self.mii() != 0 { try!(write!(f, " mii"))}
        if self.rxb() != 0 { try!(write!(f, " rxb"))}
        if self.rxf() != 0 { try!(write!(f, " rxf"))}
        if self.txb() != 0 { try!(write!(f, " txb"))}
        if self.txf() != 0 { try!(write!(f, " txf"))}
        if self.gra() != 0 { try!(write!(f, " gra"))}
        if self.babt() != 0 { try!(write!(f, " babt"))}
        if self.babr() != 0 { try!(write!(f, " babr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Mask Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eimr(pub u32);
impl Eimr {
    #[doc="TS_TIMER Interrupt Mask"]
    #[inline] pub fn ts_timer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="TS_TIMER Interrupt Mask"]
    #[inline] pub fn test_ts_timer(&self) -> bool {
        self.ts_timer() != 0
    }

    #[doc="TS_TIMER Interrupt Mask"]
    #[inline] pub fn set_ts_timer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="TS_AVAIL Interrupt Mask"]
    #[inline] pub fn ts_avail(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="TS_AVAIL Interrupt Mask"]
    #[inline] pub fn test_ts_avail(&self) -> bool {
        self.ts_avail() != 0
    }

    #[doc="TS_AVAIL Interrupt Mask"]
    #[inline] pub fn set_ts_avail<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="WAKEUP Interrupt Mask"]
    #[inline] pub fn wakeup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="WAKEUP Interrupt Mask"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="WAKEUP Interrupt Mask"]
    #[inline] pub fn set_wakeup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PLR Interrupt Mask"]
    #[inline] pub fn plr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="PLR Interrupt Mask"]
    #[inline] pub fn test_plr(&self) -> bool {
        self.plr() != 0
    }

    #[doc="PLR Interrupt Mask"]
    #[inline] pub fn set_plr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="UN Interrupt Mask"]
    #[inline] pub fn un(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="UN Interrupt Mask"]
    #[inline] pub fn test_un(&self) -> bool {
        self.un() != 0
    }

    #[doc="UN Interrupt Mask"]
    #[inline] pub fn set_un<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="RL Interrupt Mask"]
    #[inline] pub fn rl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="RL Interrupt Mask"]
    #[inline] pub fn test_rl(&self) -> bool {
        self.rl() != 0
    }

    #[doc="RL Interrupt Mask"]
    #[inline] pub fn set_rl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="LC Interrupt Mask"]
    #[inline] pub fn lc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="LC Interrupt Mask"]
    #[inline] pub fn test_lc(&self) -> bool {
        self.lc() != 0
    }

    #[doc="LC Interrupt Mask"]
    #[inline] pub fn set_lc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EBERR Interrupt Mask"]
    #[inline] pub fn eberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="EBERR Interrupt Mask"]
    #[inline] pub fn test_eberr(&self) -> bool {
        self.eberr() != 0
    }

    #[doc="EBERR Interrupt Mask"]
    #[inline] pub fn set_eberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="MII Interrupt Mask"]
    #[inline] pub fn mii(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="MII Interrupt Mask"]
    #[inline] pub fn test_mii(&self) -> bool {
        self.mii() != 0
    }

    #[doc="MII Interrupt Mask"]
    #[inline] pub fn set_mii<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="RXB Interrupt Mask"]
    #[inline] pub fn rxb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="RXB Interrupt Mask"]
    #[inline] pub fn test_rxb(&self) -> bool {
        self.rxb() != 0
    }

    #[doc="RXB Interrupt Mask"]
    #[inline] pub fn set_rxb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="RXF Interrupt Mask"]
    #[inline] pub fn rxf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="RXF Interrupt Mask"]
    #[inline] pub fn test_rxf(&self) -> bool {
        self.rxf() != 0
    }

    #[doc="RXF Interrupt Mask"]
    #[inline] pub fn set_rxf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="TXB Interrupt Mask"]
    #[inline] pub fn txb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="TXB Interrupt Mask"]
    #[inline] pub fn test_txb(&self) -> bool {
        self.txb() != 0
    }

    #[doc="TXB Interrupt Mask"]
    #[inline] pub fn set_txb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="TXF Interrupt Mask"]
    #[inline] pub fn txf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="TXF Interrupt Mask"]
    #[inline] pub fn test_txf(&self) -> bool {
        self.txf() != 0
    }

    #[doc="TXF Interrupt Mask"]
    #[inline] pub fn set_txf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GRA Interrupt Mask"]
    #[inline] pub fn gra(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="GRA Interrupt Mask"]
    #[inline] pub fn test_gra(&self) -> bool {
        self.gra() != 0
    }

    #[doc="GRA Interrupt Mask"]
    #[inline] pub fn set_gra<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="BABT Interrupt Mask"]
    #[inline] pub fn babt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="BABT Interrupt Mask"]
    #[inline] pub fn test_babt(&self) -> bool {
        self.babt() != 0
    }

    #[doc="BABT Interrupt Mask"]
    #[inline] pub fn set_babt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="BABR Interrupt Mask"]
    #[inline] pub fn babr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="BABR Interrupt Mask"]
    #[inline] pub fn test_babr(&self) -> bool {
        self.babr() != 0
    }

    #[doc="BABR Interrupt Mask"]
    #[inline] pub fn set_babr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Eimr {
    #[inline]
    fn from(other: u32) -> Self {
         Eimr(other)
    }
}

impl ::core::fmt::Display for Eimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ts_timer() != 0 { try!(write!(f, " ts_timer"))}
        if self.ts_avail() != 0 { try!(write!(f, " ts_avail"))}
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.plr() != 0 { try!(write!(f, " plr"))}
        if self.un() != 0 { try!(write!(f, " un"))}
        if self.rl() != 0 { try!(write!(f, " rl"))}
        if self.lc() != 0 { try!(write!(f, " lc"))}
        if self.eberr() != 0 { try!(write!(f, " eberr"))}
        if self.mii() != 0 { try!(write!(f, " mii"))}
        if self.rxb() != 0 { try!(write!(f, " rxb"))}
        if self.rxf() != 0 { try!(write!(f, " rxf"))}
        if self.txb() != 0 { try!(write!(f, " txb"))}
        if self.txf() != 0 { try!(write!(f, " txf"))}
        if self.gra() != 0 { try!(write!(f, " gra"))}
        if self.babt() != 0 { try!(write!(f, " babt"))}
        if self.babr() != 0 { try!(write!(f, " babr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Descriptor Active Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdar(pub u32);
impl Rdar {
    #[doc="Receive Descriptor Active"]
    #[inline] pub fn rdar(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Receive Descriptor Active"]
    #[inline] pub fn test_rdar(&self) -> bool {
        self.rdar() != 0
    }

    #[doc="Receive Descriptor Active"]
    #[inline] pub fn set_rdar<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Rdar {
    #[inline]
    fn from(other: u32) -> Self {
         Rdar(other)
    }
}

impl ::core::fmt::Display for Rdar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdar() != 0 { try!(write!(f, " rdar"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Descriptor Active Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdar(pub u32);
impl Tdar {
    #[doc="Transmit Descriptor Active"]
    #[inline] pub fn tdar(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Transmit Descriptor Active"]
    #[inline] pub fn test_tdar(&self) -> bool {
        self.tdar() != 0
    }

    #[doc="Transmit Descriptor Active"]
    #[inline] pub fn set_tdar<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Tdar {
    #[inline]
    fn from(other: u32) -> Self {
         Tdar(other)
    }
}

impl ::core::fmt::Display for Tdar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdar() != 0 { try!(write!(f, " tdar"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc="Ethernet MAC Reset"]
    #[inline] pub fn _reset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Ethernet MAC Reset"]
    #[inline] pub fn test_reset(&self) -> bool {
        self._reset() != 0
    }

    #[doc="Ethernet MAC Reset"]
    #[inline] pub fn set_reset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ethernet Enable"]
    #[inline] pub fn etheren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Ethernet Enable"]
    #[inline] pub fn test_etheren(&self) -> bool {
        self.etheren() != 0
    }

    #[doc="Ethernet Enable"]
    #[inline] pub fn set_etheren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Magic Packet Detection Enable"]
    #[inline] pub fn magicen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Magic Packet Detection Enable"]
    #[inline] pub fn test_magicen(&self) -> bool {
        self.magicen() != 0
    }

    #[doc="Magic Packet Detection Enable"]
    #[inline] pub fn set_magicen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Sleep Mode Enable"]
    #[inline] pub fn sleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Sleep Mode Enable"]
    #[inline] pub fn test_sleep(&self) -> bool {
        self.sleep() != 0
    }

    #[doc="Sleep Mode Enable"]
    #[inline] pub fn set_sleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EN1588 Enable"]
    #[inline] pub fn en1588(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="EN1588 Enable"]
    #[inline] pub fn test_en1588(&self) -> bool {
        self.en1588() != 0
    }

    #[doc="EN1588 Enable"]
    #[inline] pub fn set_en1588<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Debug Enable"]
    #[inline] pub fn dbgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Debug Enable"]
    #[inline] pub fn test_dbgen(&self) -> bool {
        self.dbgen() != 0
    }

    #[doc="Debug Enable"]
    #[inline] pub fn set_dbgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="STOPEN Signal Control"]
    #[inline] pub fn stopen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="STOPEN Signal Control"]
    #[inline] pub fn test_stopen(&self) -> bool {
        self.stopen() != 0
    }

    #[doc="STOPEN Signal Control"]
    #[inline] pub fn set_stopen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Descriptor Byte Swapping Enable"]
    #[inline] pub fn dbswp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Descriptor Byte Swapping Enable"]
    #[inline] pub fn test_dbswp(&self) -> bool {
        self.dbswp() != 0
    }

    #[doc="Descriptor Byte Swapping Enable"]
    #[inline] pub fn set_dbswp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Ecr {
    #[inline]
    fn from(other: u32) -> Self {
         Ecr(other)
    }
}

impl ::core::fmt::Display for Ecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._reset() != 0 { try!(write!(f, " _reset"))}
        if self.etheren() != 0 { try!(write!(f, " etheren"))}
        if self.magicen() != 0 { try!(write!(f, " magicen"))}
        if self.sleep() != 0 { try!(write!(f, " sleep"))}
        if self.en1588() != 0 { try!(write!(f, " en1588"))}
        if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
        if self.stopen() != 0 { try!(write!(f, " stopen"))}
        if self.dbswp() != 0 { try!(write!(f, " dbswp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MII Management Frame Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmfr(pub u32);
impl Mmfr {
    #[doc="Management Frame Data"]
    #[inline] pub fn data(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Management Frame Data"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Management Frame Data"]
    #[inline] pub fn set_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Turn Around"]
    #[inline] pub fn ta(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Turn Around"]
    #[inline] pub fn test_ta(&self) -> bool {
        self.ta() != 0
    }

    #[doc="Turn Around"]
    #[inline] pub fn set_ta<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Register Address"]
    #[inline] pub fn ra(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
    }

    #[doc="Register Address"]
    #[inline] pub fn test_ra(&self) -> bool {
        self.ra() != 0
    }

    #[doc="Register Address"]
    #[inline] pub fn set_ra<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="PHY Address"]
    #[inline] pub fn pa(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1f) as u8) } // [27:23]
    }

    #[doc="PHY Address"]
    #[inline] pub fn test_pa(&self) -> bool {
        self.pa() != 0
    }

    #[doc="PHY Address"]
    #[inline] pub fn set_pa<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Operation Code"]
    #[inline] pub fn op(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Operation Code"]
    #[inline] pub fn test_op(&self) -> bool {
        self.op() != 0
    }

    #[doc="Operation Code"]
    #[inline] pub fn set_op<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Start Of Frame Delimiter"]
    #[inline] pub fn st(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Start Of Frame Delimiter"]
    #[inline] pub fn test_st(&self) -> bool {
        self.st() != 0
    }

    #[doc="Start Of Frame Delimiter"]
    #[inline] pub fn set_st<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Mmfr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmfr(other)
    }
}

impl ::core::fmt::Display for Mmfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        if self.ta() != 0 { try!(write!(f, " ta=0x{:x}", self.ta()))}
        if self.ra() != 0 { try!(write!(f, " ra=0x{:x}", self.ra()))}
        if self.pa() != 0 { try!(write!(f, " pa=0x{:x}", self.pa()))}
        if self.op() != 0 { try!(write!(f, " op=0x{:x}", self.op()))}
        if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MII Speed Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mscr(pub u32);
impl Mscr {
    #[doc="MII Speed"]
    #[inline] pub fn mii_speed(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3f) as u8) } // [6:1]
    }

    #[doc="MII Speed"]
    #[inline] pub fn test_mii_speed(&self) -> bool {
        self.mii_speed() != 0
    }

    #[doc="MII Speed"]
    #[inline] pub fn set_mii_speed<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Disable Preamble"]
    #[inline] pub fn dis_pre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Disable Preamble"]
    #[inline] pub fn test_dis_pre(&self) -> bool {
        self.dis_pre() != 0
    }

    #[doc="Disable Preamble"]
    #[inline] pub fn set_dis_pre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Hold time On MDIO Output"]
    #[inline] pub fn holdtime(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Hold time On MDIO Output"]
    #[inline] pub fn test_holdtime(&self) -> bool {
        self.holdtime() != 0
    }

    #[doc="Hold time On MDIO Output"]
    #[inline] pub fn set_holdtime<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Mscr {
    #[inline]
    fn from(other: u32) -> Self {
         Mscr(other)
    }
}

impl ::core::fmt::Display for Mscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mii_speed() != 0 { try!(write!(f, " mii_speed=0x{:x}", self.mii_speed()))}
        if self.dis_pre() != 0 { try!(write!(f, " dis_pre"))}
        if self.holdtime() != 0 { try!(write!(f, " holdtime=0x{:x}", self.holdtime()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MIB Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mibc(pub u32);
impl Mibc {
    #[doc="MIB Clear"]
    #[inline] pub fn mib_clear(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="MIB Clear"]
    #[inline] pub fn test_mib_clear(&self) -> bool {
        self.mib_clear() != 0
    }

    #[doc="MIB Clear"]
    #[inline] pub fn set_mib_clear<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="MIB Idle"]
    #[inline] pub fn mib_idle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="MIB Idle"]
    #[inline] pub fn test_mib_idle(&self) -> bool {
        self.mib_idle() != 0
    }

    #[doc="MIB Idle"]
    #[inline] pub fn set_mib_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Disable MIB Logic"]
    #[inline] pub fn mib_dis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Disable MIB Logic"]
    #[inline] pub fn test_mib_dis(&self) -> bool {
        self.mib_dis() != 0
    }

    #[doc="Disable MIB Logic"]
    #[inline] pub fn set_mib_dis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Mibc {
    #[inline]
    fn from(other: u32) -> Self {
         Mibc(other)
    }
}

impl ::core::fmt::Display for Mibc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mibc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mib_clear() != 0 { try!(write!(f, " mib_clear"))}
        if self.mib_idle() != 0 { try!(write!(f, " mib_idle"))}
        if self.mib_dis() != 0 { try!(write!(f, " mib_dis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc="Internal Loopback"]
    #[inline] pub fn _loop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Internal Loopback"]
    #[inline] pub fn test_loop(&self) -> bool {
        self._loop() != 0
    }

    #[doc="Internal Loopback"]
    #[inline] pub fn set_loop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Disable Receive On Transmit"]
    #[inline] pub fn drt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Disable Receive On Transmit"]
    #[inline] pub fn test_drt(&self) -> bool {
        self.drt() != 0
    }

    #[doc="Disable Receive On Transmit"]
    #[inline] pub fn set_drt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Media Independent Interface Mode"]
    #[inline] pub fn mii_mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Media Independent Interface Mode"]
    #[inline] pub fn test_mii_mode(&self) -> bool {
        self.mii_mode() != 0
    }

    #[doc="Media Independent Interface Mode"]
    #[inline] pub fn set_mii_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Promiscuous Mode"]
    #[inline] pub fn prom(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Promiscuous Mode"]
    #[inline] pub fn test_prom(&self) -> bool {
        self.prom() != 0
    }

    #[doc="Promiscuous Mode"]
    #[inline] pub fn set_prom<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Broadcast Frame Reject"]
    #[inline] pub fn bc_rej(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Broadcast Frame Reject"]
    #[inline] pub fn test_bc_rej(&self) -> bool {
        self.bc_rej() != 0
    }

    #[doc="Broadcast Frame Reject"]
    #[inline] pub fn set_bc_rej<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Flow Control Enable"]
    #[inline] pub fn fce(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Flow Control Enable"]
    #[inline] pub fn test_fce(&self) -> bool {
        self.fce() != 0
    }

    #[doc="Flow Control Enable"]
    #[inline] pub fn set_fce<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RMII Mode Enable"]
    #[inline] pub fn rmii_mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="RMII Mode Enable"]
    #[inline] pub fn test_rmii_mode(&self) -> bool {
        self.rmii_mode() != 0
    }

    #[doc="RMII Mode Enable"]
    #[inline] pub fn set_rmii_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enables 10-Mbps mode of the RMII ."]
    #[inline] pub fn rmii_10t(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Enables 10-Mbps mode of the RMII ."]
    #[inline] pub fn test_rmii_10t(&self) -> bool {
        self.rmii_10t() != 0
    }

    #[doc="Enables 10-Mbps mode of the RMII ."]
    #[inline] pub fn set_rmii_10t<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable Frame Padding Remove On Receive"]
    #[inline] pub fn paden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Enable Frame Padding Remove On Receive"]
    #[inline] pub fn test_paden(&self) -> bool {
        self.paden() != 0
    }

    #[doc="Enable Frame Padding Remove On Receive"]
    #[inline] pub fn set_paden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Terminate/Forward Pause Frames"]
    #[inline] pub fn paufwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Terminate/Forward Pause Frames"]
    #[inline] pub fn test_paufwd(&self) -> bool {
        self.paufwd() != 0
    }

    #[doc="Terminate/Forward Pause Frames"]
    #[inline] pub fn set_paufwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Terminate/Forward Received CRC"]
    #[inline] pub fn crcfwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Terminate/Forward Received CRC"]
    #[inline] pub fn test_crcfwd(&self) -> bool {
        self.crcfwd() != 0
    }

    #[doc="Terminate/Forward Received CRC"]
    #[inline] pub fn set_crcfwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="MAC Control Frame Enable"]
    #[inline] pub fn cfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="MAC Control Frame Enable"]
    #[inline] pub fn test_cfen(&self) -> bool {
        self.cfen() != 0
    }

    #[doc="MAC Control Frame Enable"]
    #[inline] pub fn set_cfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Maximum Frame Length"]
    #[inline] pub fn max_fl(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3fff) as u16) } // [29:16]
    }

    #[doc="Maximum Frame Length"]
    #[inline] pub fn test_max_fl(&self) -> bool {
        self.max_fl() != 0
    }

    #[doc="Maximum Frame Length"]
    #[inline] pub fn set_max_fl<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Payload Length Check Disable"]
    #[inline] pub fn nlc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Payload Length Check Disable"]
    #[inline] pub fn test_nlc(&self) -> bool {
        self.nlc() != 0
    }

    #[doc="Payload Length Check Disable"]
    #[inline] pub fn set_nlc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Graceful Receive Stopped"]
    #[inline] pub fn grs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Graceful Receive Stopped"]
    #[inline] pub fn test_grs(&self) -> bool {
        self.grs() != 0
    }

    #[doc="Graceful Receive Stopped"]
    #[inline] pub fn set_grs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Rcr {
    #[inline]
    fn from(other: u32) -> Self {
         Rcr(other)
    }
}

impl ::core::fmt::Display for Rcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._loop() != 0 { try!(write!(f, " _loop"))}
        if self.drt() != 0 { try!(write!(f, " drt"))}
        if self.mii_mode() != 0 { try!(write!(f, " mii_mode"))}
        if self.prom() != 0 { try!(write!(f, " prom"))}
        if self.bc_rej() != 0 { try!(write!(f, " bc_rej"))}
        if self.fce() != 0 { try!(write!(f, " fce"))}
        if self.rmii_mode() != 0 { try!(write!(f, " rmii_mode"))}
        if self.rmii_10t() != 0 { try!(write!(f, " rmii_10t"))}
        if self.paden() != 0 { try!(write!(f, " paden"))}
        if self.paufwd() != 0 { try!(write!(f, " paufwd"))}
        if self.crcfwd() != 0 { try!(write!(f, " crcfwd"))}
        if self.cfen() != 0 { try!(write!(f, " cfen"))}
        if self.max_fl() != 0 { try!(write!(f, " max_fl=0x{:x}", self.max_fl()))}
        if self.nlc() != 0 { try!(write!(f, " nlc"))}
        if self.grs() != 0 { try!(write!(f, " grs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc="Graceful Transmit Stop"]
    #[inline] pub fn gts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Graceful Transmit Stop"]
    #[inline] pub fn test_gts(&self) -> bool {
        self.gts() != 0
    }

    #[doc="Graceful Transmit Stop"]
    #[inline] pub fn set_gts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Full-Duplex Enable"]
    #[inline] pub fn fden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Full-Duplex Enable"]
    #[inline] pub fn test_fden(&self) -> bool {
        self.fden() != 0
    }

    #[doc="Full-Duplex Enable"]
    #[inline] pub fn set_fden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit Frame Control Pause"]
    #[inline] pub fn tfc_pause(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Transmit Frame Control Pause"]
    #[inline] pub fn test_tfc_pause(&self) -> bool {
        self.tfc_pause() != 0
    }

    #[doc="Transmit Frame Control Pause"]
    #[inline] pub fn set_tfc_pause<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receive Frame Control Pause"]
    #[inline] pub fn rfc_pause(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Receive Frame Control Pause"]
    #[inline] pub fn test_rfc_pause(&self) -> bool {
        self.rfc_pause() != 0
    }

    #[doc="Receive Frame Control Pause"]
    #[inline] pub fn set_rfc_pause<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Source MAC Address Select On Transmit"]
    #[inline] pub fn addsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Source MAC Address Select On Transmit"]
    #[inline] pub fn test_addsel(&self) -> bool {
        self.addsel() != 0
    }

    #[doc="Source MAC Address Select On Transmit"]
    #[inline] pub fn set_addsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Set MAC Address On Transmit"]
    #[inline] pub fn addins(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Set MAC Address On Transmit"]
    #[inline] pub fn test_addins(&self) -> bool {
        self.addins() != 0
    }

    #[doc="Set MAC Address On Transmit"]
    #[inline] pub fn set_addins<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Forward Frame From Application With CRC"]
    #[inline] pub fn crcfwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Forward Frame From Application With CRC"]
    #[inline] pub fn test_crcfwd(&self) -> bool {
        self.crcfwd() != 0
    }

    #[doc="Forward Frame From Application With CRC"]
    #[inline] pub fn set_crcfwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Tcr {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.gts() != 0 { try!(write!(f, " gts"))}
        if self.fden() != 0 { try!(write!(f, " fden"))}
        if self.tfc_pause() != 0 { try!(write!(f, " tfc_pause"))}
        if self.rfc_pause() != 0 { try!(write!(f, " rfc_pause"))}
        if self.addsel() != 0 { try!(write!(f, " addsel=0x{:x}", self.addsel()))}
        if self.addins() != 0 { try!(write!(f, " addins"))}
        if self.crcfwd() != 0 { try!(write!(f, " crcfwd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Physical Address Lower Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Palr(pub u32);
impl Palr {
    #[doc="Pause Address"]
    #[inline] pub fn paddr1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Pause Address"]
    #[inline] pub fn test_paddr1(&self) -> bool {
        self.paddr1() != 0
    }

    #[doc="Pause Address"]
    #[inline] pub fn set_paddr1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Palr {
    #[inline]
    fn from(other: u32) -> Self {
         Palr(other)
    }
}

impl ::core::fmt::Display for Palr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Palr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Physical Address Upper Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Paur(pub u32);
impl Paur {
    #[doc="Type Field In PAUSE Frames"]
    #[inline] pub fn _type(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Type Field In PAUSE Frames"]
    #[inline] pub fn test_type(&self) -> bool {
        self._type() != 0
    }

    #[doc="Type Field In PAUSE Frames"]
    #[inline] pub fn set_type<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline] pub fn paddr2(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline] pub fn test_paddr2(&self) -> bool {
        self.paddr2() != 0
    }

    #[doc="Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline] pub fn set_paddr2<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Paur {
    #[inline]
    fn from(other: u32) -> Self {
         Paur(other)
    }
}

impl ::core::fmt::Display for Paur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Paur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._type() != 0 { try!(write!(f, " type=0x{:x}", self._type()))}
        if self.paddr2() != 0 { try!(write!(f, " paddr2=0x{:x}", self.paddr2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Opcode/Pause Duration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opd(pub u32);
impl Opd {
    #[doc="Pause Duration"]
    #[inline] pub fn pause_dur(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Pause Duration"]
    #[inline] pub fn test_pause_dur(&self) -> bool {
        self.pause_dur() != 0
    }

    #[doc="Pause Duration"]
    #[inline] pub fn set_pause_dur<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Opcode Field In PAUSE Frames"]
    #[inline] pub fn opcode(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Opcode Field In PAUSE Frames"]
    #[inline] pub fn test_opcode(&self) -> bool {
        self.opcode() != 0
    }

    #[doc="Opcode Field In PAUSE Frames"]
    #[inline] pub fn set_opcode<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Opd {
    #[inline]
    fn from(other: u32) -> Self {
         Opd(other)
    }
}

impl ::core::fmt::Display for Opd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pause_dur() != 0 { try!(write!(f, " pause_dur=0x{:x}", self.pause_dur()))}
        if self.opcode() != 0 { try!(write!(f, " opcode=0x{:x}", self.opcode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Descriptor Individual Upper Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iaur(pub u32);
impl Iaur {
    #[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline] pub fn iaddr1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline] pub fn test_iaddr1(&self) -> bool {
        self.iaddr1() != 0
    }

    #[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline] pub fn set_iaddr1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iaur {
    #[inline]
    fn from(other: u32) -> Self {
         Iaur(other)
    }
}

impl ::core::fmt::Display for Iaur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iaur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Descriptor Individual Lower Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ialr(pub u32);
impl Ialr {
    #[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline] pub fn iaddr2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline] pub fn test_iaddr2(&self) -> bool {
        self.iaddr2() != 0
    }

    #[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline] pub fn set_iaddr2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ialr {
    #[inline]
    fn from(other: u32) -> Self {
         Ialr(other)
    }
}

impl ::core::fmt::Display for Ialr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ialr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Descriptor Group Upper Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gaur(pub u32);
impl Gaur {
    #[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline] pub fn gaddr1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline] pub fn test_gaddr1(&self) -> bool {
        self.gaddr1() != 0
    }

    #[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline] pub fn set_gaddr1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gaur {
    #[inline]
    fn from(other: u32) -> Self {
         Gaur(other)
    }
}

impl ::core::fmt::Display for Gaur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gaur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Descriptor Group Lower Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Galr(pub u32);
impl Galr {
    #[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline] pub fn gaddr2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline] pub fn test_gaddr2(&self) -> bool {
        self.gaddr2() != 0
    }

    #[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline] pub fn set_gaddr2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Galr {
    #[inline]
    fn from(other: u32) -> Self {
         Galr(other)
    }
}

impl ::core::fmt::Display for Galr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Galr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit FIFO Watermark Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tfwr(pub u32);
impl Tfwr {
    #[doc="Transmit FIFO Write"]
    #[inline] pub fn tfwr(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Transmit FIFO Write"]
    #[inline] pub fn test_tfwr(&self) -> bool {
        self.tfwr() != 0
    }

    #[doc="Transmit FIFO Write"]
    #[inline] pub fn set_tfwr<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Store And Forward Enable"]
    #[inline] pub fn strfwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Store And Forward Enable"]
    #[inline] pub fn test_strfwd(&self) -> bool {
        self.strfwd() != 0
    }

    #[doc="Store And Forward Enable"]
    #[inline] pub fn set_strfwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Tfwr {
    #[inline]
    fn from(other: u32) -> Self {
         Tfwr(other)
    }
}

impl ::core::fmt::Display for Tfwr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tfwr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tfwr() != 0 { try!(write!(f, " tfwr=0x{:x}", self.tfwr()))}
        if self.strfwd() != 0 { try!(write!(f, " strfwd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Descriptor Ring Start Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdsr(pub u32);
impl Rdsr {
    #[doc="Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline] pub fn r_des_start(&self) -> bits::U29 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1fffffff) as u32) } // [31:3]
    }

    #[doc="Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline] pub fn test_r_des_start(&self) -> bool {
        self.r_des_start() != 0
    }

    #[doc="Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline] pub fn set_r_des_start<V: Into<bits::U29>>(mut self, value: V) -> Self {
        let value: bits::U29 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fffffff << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Rdsr {
    #[inline]
    fn from(other: u32) -> Self {
         Rdsr(other)
    }
}

impl ::core::fmt::Display for Rdsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r_des_start() != 0 { try!(write!(f, " r_des_start=0x{:x}", self.r_des_start()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Buffer Descriptor Ring Start Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdsr(pub u32);
impl Tdsr {
    #[doc="Pointer to the beginning of the transmit buffer descriptor queue."]
    #[inline] pub fn x_des_start(&self) -> bits::U29 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1fffffff) as u32) } // [31:3]
    }

    #[doc="Pointer to the beginning of the transmit buffer descriptor queue."]
    #[inline] pub fn test_x_des_start(&self) -> bool {
        self.x_des_start() != 0
    }

    #[doc="Pointer to the beginning of the transmit buffer descriptor queue."]
    #[inline] pub fn set_x_des_start<V: Into<bits::U29>>(mut self, value: V) -> Self {
        let value: bits::U29 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fffffff << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Tdsr {
    #[inline]
    fn from(other: u32) -> Self {
         Tdsr(other)
    }
}

impl ::core::fmt::Display for Tdsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.x_des_start() != 0 { try!(write!(f, " x_des_start=0x{:x}", self.x_des_start()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Maximum Receive Buffer Size Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mrbr(pub u32);
impl Mrbr {
    #[doc="Receive buffer size in bytes."]
    #[inline] pub fn r_buf_size(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3ff) as u16) } // [13:4]
    }

    #[doc="Receive buffer size in bytes."]
    #[inline] pub fn test_r_buf_size(&self) -> bool {
        self.r_buf_size() != 0
    }

    #[doc="Receive buffer size in bytes."]
    #[inline] pub fn set_r_buf_size<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Mrbr {
    #[inline]
    fn from(other: u32) -> Self {
         Mrbr(other)
    }
}

impl ::core::fmt::Display for Mrbr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mrbr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r_buf_size() != 0 { try!(write!(f, " r_buf_size=0x{:x}", self.r_buf_size()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive FIFO Section Full Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rsfl(pub u32);
impl Rsfl {
    #[doc="Value Of Receive FIFO Section Full Threshold"]
    #[inline] pub fn rx_section_full(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Value Of Receive FIFO Section Full Threshold"]
    #[inline] pub fn test_rx_section_full(&self) -> bool {
        self.rx_section_full() != 0
    }

    #[doc="Value Of Receive FIFO Section Full Threshold"]
    #[inline] pub fn set_rx_section_full<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rsfl {
    #[inline]
    fn from(other: u32) -> Self {
         Rsfl(other)
    }
}

impl ::core::fmt::Display for Rsfl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rsfl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx_section_full() != 0 { try!(write!(f, " rx_section_full=0x{:x}", self.rx_section_full()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive FIFO Section Empty Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rsem(pub u32);
impl Rsem {
    #[doc="Value Of The Receive FIFO Section Empty Threshold"]
    #[inline] pub fn rx_section_empty(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Value Of The Receive FIFO Section Empty Threshold"]
    #[inline] pub fn test_rx_section_empty(&self) -> bool {
        self.rx_section_empty() != 0
    }

    #[doc="Value Of The Receive FIFO Section Empty Threshold"]
    #[inline] pub fn set_rx_section_empty<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RX Status FIFO Section Empty Threshold"]
    #[inline] pub fn stat_section_empty(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="RX Status FIFO Section Empty Threshold"]
    #[inline] pub fn test_stat_section_empty(&self) -> bool {
        self.stat_section_empty() != 0
    }

    #[doc="RX Status FIFO Section Empty Threshold"]
    #[inline] pub fn set_stat_section_empty<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Rsem {
    #[inline]
    fn from(other: u32) -> Self {
         Rsem(other)
    }
}

impl ::core::fmt::Display for Rsem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rsem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx_section_empty() != 0 { try!(write!(f, " rx_section_empty=0x{:x}", self.rx_section_empty()))}
        if self.stat_section_empty() != 0 { try!(write!(f, " stat_section_empty=0x{:x}", self.stat_section_empty()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive FIFO Almost Empty Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Raem(pub u32);
impl Raem {
    #[doc="Value Of The Receive FIFO Almost Empty Threshold"]
    #[inline] pub fn rx_almost_empty(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Value Of The Receive FIFO Almost Empty Threshold"]
    #[inline] pub fn test_rx_almost_empty(&self) -> bool {
        self.rx_almost_empty() != 0
    }

    #[doc="Value Of The Receive FIFO Almost Empty Threshold"]
    #[inline] pub fn set_rx_almost_empty<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Raem {
    #[inline]
    fn from(other: u32) -> Self {
         Raem(other)
    }
}

impl ::core::fmt::Display for Raem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Raem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx_almost_empty() != 0 { try!(write!(f, " rx_almost_empty=0x{:x}", self.rx_almost_empty()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive FIFO Almost Full Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rafl(pub u32);
impl Rafl {
    #[doc="Value Of The Receive FIFO Almost Full Threshold"]
    #[inline] pub fn rx_almost_full(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Value Of The Receive FIFO Almost Full Threshold"]
    #[inline] pub fn test_rx_almost_full(&self) -> bool {
        self.rx_almost_full() != 0
    }

    #[doc="Value Of The Receive FIFO Almost Full Threshold"]
    #[inline] pub fn set_rx_almost_full<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rafl {
    #[inline]
    fn from(other: u32) -> Self {
         Rafl(other)
    }
}

impl ::core::fmt::Display for Rafl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rafl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx_almost_full() != 0 { try!(write!(f, " rx_almost_full=0x{:x}", self.rx_almost_full()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit FIFO Section Empty Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tsem(pub u32);
impl Tsem {
    #[doc="Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline] pub fn tx_section_empty(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline] pub fn test_tx_section_empty(&self) -> bool {
        self.tx_section_empty() != 0
    }

    #[doc="Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline] pub fn set_tx_section_empty<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tsem {
    #[inline]
    fn from(other: u32) -> Self {
         Tsem(other)
    }
}

impl ::core::fmt::Display for Tsem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tsem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tx_section_empty() != 0 { try!(write!(f, " tx_section_empty=0x{:x}", self.tx_section_empty()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit FIFO Almost Empty Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Taem(pub u32);
impl Taem {
    #[doc="Value of Transmit FIFO Almost Empty Threshold"]
    #[inline] pub fn tx_almost_empty(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Value of Transmit FIFO Almost Empty Threshold"]
    #[inline] pub fn test_tx_almost_empty(&self) -> bool {
        self.tx_almost_empty() != 0
    }

    #[doc="Value of Transmit FIFO Almost Empty Threshold"]
    #[inline] pub fn set_tx_almost_empty<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Taem {
    #[inline]
    fn from(other: u32) -> Self {
         Taem(other)
    }
}

impl ::core::fmt::Display for Taem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Taem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tx_almost_empty() != 0 { try!(write!(f, " tx_almost_empty=0x{:x}", self.tx_almost_empty()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit FIFO Almost Full Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tafl(pub u32);
impl Tafl {
    #[doc="Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline] pub fn tx_almost_full(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline] pub fn test_tx_almost_full(&self) -> bool {
        self.tx_almost_full() != 0
    }

    #[doc="Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline] pub fn set_tx_almost_full<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tafl {
    #[inline]
    fn from(other: u32) -> Self {
         Tafl(other)
    }
}

impl ::core::fmt::Display for Tafl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tafl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tx_almost_full() != 0 { try!(write!(f, " tx_almost_full=0x{:x}", self.tx_almost_full()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Inter-Packet Gap"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tipg(pub u32);
impl Tipg {
    #[doc="Transmit Inter-Packet Gap"]
    #[inline] pub fn ipg(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Transmit Inter-Packet Gap"]
    #[inline] pub fn test_ipg(&self) -> bool {
        self.ipg() != 0
    }

    #[doc="Transmit Inter-Packet Gap"]
    #[inline] pub fn set_ipg<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tipg {
    #[inline]
    fn from(other: u32) -> Self {
         Tipg(other)
    }
}

impl ::core::fmt::Display for Tipg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tipg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipg() != 0 { try!(write!(f, " ipg=0x{:x}", self.ipg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frame Truncation Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftrl(pub u32);
impl Ftrl {
    #[doc="Frame Truncation Length"]
    #[inline] pub fn trunc_fl(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Frame Truncation Length"]
    #[inline] pub fn test_trunc_fl(&self) -> bool {
        self.trunc_fl() != 0
    }

    #[doc="Frame Truncation Length"]
    #[inline] pub fn set_trunc_fl<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ftrl {
    #[inline]
    fn from(other: u32) -> Self {
         Ftrl(other)
    }
}

impl ::core::fmt::Display for Ftrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ftrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trunc_fl() != 0 { try!(write!(f, " trunc_fl=0x{:x}", self.trunc_fl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Accelerator Function Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tacc(pub u32);
impl Tacc {
    #[doc="TX FIFO Shift-16"]
    #[inline] pub fn shift16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="TX FIFO Shift-16"]
    #[inline] pub fn test_shift16(&self) -> bool {
        self.shift16() != 0
    }

    #[doc="TX FIFO Shift-16"]
    #[inline] pub fn set_shift16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enables insertion of IP header checksum."]
    #[inline] pub fn ipchk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Enables insertion of IP header checksum."]
    #[inline] pub fn test_ipchk(&self) -> bool {
        self.ipchk() != 0
    }

    #[doc="Enables insertion of IP header checksum."]
    #[inline] pub fn set_ipchk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enables insertion of protocol checksum."]
    #[inline] pub fn prochk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Enables insertion of protocol checksum."]
    #[inline] pub fn test_prochk(&self) -> bool {
        self.prochk() != 0
    }

    #[doc="Enables insertion of protocol checksum."]
    #[inline] pub fn set_prochk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Tacc {
    #[inline]
    fn from(other: u32) -> Self {
         Tacc(other)
    }
}

impl ::core::fmt::Display for Tacc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tacc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.shift16() != 0 { try!(write!(f, " shift16"))}
        if self.ipchk() != 0 { try!(write!(f, " ipchk"))}
        if self.prochk() != 0 { try!(write!(f, " prochk"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Accelerator Function Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Racc(pub u32);
impl Racc {
    #[doc="Enable Padding Removal For Short IP Frames"]
    #[inline] pub fn padrem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Enable Padding Removal For Short IP Frames"]
    #[inline] pub fn test_padrem(&self) -> bool {
        self.padrem() != 0
    }

    #[doc="Enable Padding Removal For Short IP Frames"]
    #[inline] pub fn set_padrem<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline] pub fn ipdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline] pub fn test_ipdis(&self) -> bool {
        self.ipdis() != 0
    }

    #[doc="Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline] pub fn set_ipdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline] pub fn prodis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline] pub fn test_prodis(&self) -> bool {
        self.prodis() != 0
    }

    #[doc="Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline] pub fn set_prodis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enable Discard Of Frames With MAC Layer Errors"]
    #[inline] pub fn linedis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Enable Discard Of Frames With MAC Layer Errors"]
    #[inline] pub fn test_linedis(&self) -> bool {
        self.linedis() != 0
    }

    #[doc="Enable Discard Of Frames With MAC Layer Errors"]
    #[inline] pub fn set_linedis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="RX FIFO Shift-16"]
    #[inline] pub fn shift16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="RX FIFO Shift-16"]
    #[inline] pub fn test_shift16(&self) -> bool {
        self.shift16() != 0
    }

    #[doc="RX FIFO Shift-16"]
    #[inline] pub fn set_shift16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Racc {
    #[inline]
    fn from(other: u32) -> Self {
         Racc(other)
    }
}

impl ::core::fmt::Display for Racc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Racc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.padrem() != 0 { try!(write!(f, " padrem"))}
        if self.ipdis() != 0 { try!(write!(f, " ipdis"))}
        if self.prodis() != 0 { try!(write!(f, " prodis"))}
        if self.linedis() != 0 { try!(write!(f, " linedis"))}
        if self.shift16() != 0 { try!(write!(f, " shift16"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Packet Count Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTPackets(pub u32);
impl RmonTPackets {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTPackets {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTPackets(other)
    }
}

impl ::core::fmt::Display for RmonTPackets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTPackets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Broadcast Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTBcPkt(pub u32);
impl RmonTBcPkt {
    #[doc="Broadcast packets"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Broadcast packets"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Broadcast packets"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTBcPkt {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTBcPkt(other)
    }
}

impl ::core::fmt::Display for RmonTBcPkt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTBcPkt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Multicast Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTMcPkt(pub u32);
impl RmonTMcPkt {
    #[doc="Multicast packets"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Multicast packets"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Multicast packets"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTMcPkt {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTMcPkt(other)
    }
}

impl ::core::fmt::Display for RmonTMcPkt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTMcPkt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Packets with CRC/Align Error Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTCrcAlign(pub u32);
impl RmonTCrcAlign {
    #[doc="Packets with CRC/align error"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packets with CRC/align error"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packets with CRC/align error"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTCrcAlign {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTCrcAlign(other)
    }
}

impl ::core::fmt::Display for RmonTCrcAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTCrcAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Packets Less Than Bytes and Good CRC Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTUndersize(pub u32);
impl RmonTUndersize {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTUndersize {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTUndersize(other)
    }
}

impl ::core::fmt::Display for RmonTUndersize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTUndersize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTOversize(pub u32);
impl RmonTOversize {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTOversize {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTOversize(other)
    }
}

impl ::core::fmt::Display for RmonTOversize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTOversize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTFrag(pub u32);
impl RmonTFrag {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTFrag {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTFrag(other)
    }
}

impl ::core::fmt::Display for RmonTFrag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTFrag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTJab(pub u32);
impl RmonTJab {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTJab {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTJab(other)
    }
}

impl ::core::fmt::Display for RmonTJab {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTJab {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Collision Count Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTCol(pub u32);
impl RmonTCol {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTCol {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTCol(other)
    }
}

impl ::core::fmt::Display for RmonTCol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTCol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx 64-Byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTP64(pub u32);
impl RmonTP64 {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTP64 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTP64(other)
    }
}

impl ::core::fmt::Display for RmonTP64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTP64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx 65- to 127-byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTP65to127(pub u32);
impl RmonTP65to127 {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTP65to127 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTP65to127(other)
    }
}

impl ::core::fmt::Display for RmonTP65to127 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTP65to127 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx 128- to 255-byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTP128to255(pub u32);
impl RmonTP128to255 {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTP128to255 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTP128to255(other)
    }
}

impl ::core::fmt::Display for RmonTP128to255 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTP128to255 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx 256- to 511-byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTP256to511(pub u32);
impl RmonTP256to511 {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTP256to511 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTP256to511(other)
    }
}

impl ::core::fmt::Display for RmonTP256to511 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTP256to511 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx 512- to 1023-byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTP512to1023(pub u32);
impl RmonTP512to1023 {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTP512to1023 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTP512to1023(other)
    }
}

impl ::core::fmt::Display for RmonTP512to1023 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTP512to1023 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx 1024- to 2047-byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTP1024to2047(pub u32);
impl RmonTP1024to2047 {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTP1024to2047 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTP1024to2047(other)
    }
}

impl ::core::fmt::Display for RmonTP1024to2047 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTP1024to2047 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Packets Greater Than 2048 Bytes Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTPGte2048(pub u32);
impl RmonTPGte2048 {
    #[doc="Packet count"]
    #[inline] pub fn txpkts(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_txpkts(&self) -> bool {
        self.txpkts() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_txpkts<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTPGte2048 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTPGte2048(other)
    }
}

impl ::core::fmt::Display for RmonTPGte2048 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTPGte2048 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txpkts() != 0 { try!(write!(f, " txpkts=0x{:x}", self.txpkts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Octets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonTOctets(pub u32);
impl RmonTOctets {
    #[doc="Octet count"]
    #[inline] pub fn txocts(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Octet count"]
    #[inline] pub fn test_txocts(&self) -> bool {
        self.txocts() != 0
    }

    #[doc="Octet count"]
    #[inline] pub fn set_txocts<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonTOctets {
    #[inline]
    fn from(other: u32) -> Self {
         RmonTOctets(other)
    }
}

impl ::core::fmt::Display for RmonTOctets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonTOctets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Transmitted OK Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTFrameOk(pub u32);
impl IeeeTFrameOk {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTFrameOk {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTFrameOk(other)
    }
}

impl ::core::fmt::Display for IeeeTFrameOk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTFrameOk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Transmitted with Single Collision Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeT1col(pub u32);
impl IeeeT1col {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeT1col {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeT1col(other)
    }
}

impl ::core::fmt::Display for IeeeT1col {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeT1col {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Transmitted with Multiple Collisions Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTMcol(pub u32);
impl IeeeTMcol {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTMcol {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTMcol(other)
    }
}

impl ::core::fmt::Display for IeeeTMcol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTMcol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Transmitted after Deferral Delay Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTDef(pub u32);
impl IeeeTDef {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTDef {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTDef(other)
    }
}

impl ::core::fmt::Display for IeeeTDef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTDef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Transmitted with Late Collision Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTLcol(pub u32);
impl IeeeTLcol {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTLcol {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTLcol(other)
    }
}

impl ::core::fmt::Display for IeeeTLcol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTLcol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Transmitted with Excessive Collisions Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTExcol(pub u32);
impl IeeeTExcol {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTExcol {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTExcol(other)
    }
}

impl ::core::fmt::Display for IeeeTExcol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTExcol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Transmitted with Tx FIFO Underrun Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTMacerr(pub u32);
impl IeeeTMacerr {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTMacerr {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTMacerr(other)
    }
}

impl ::core::fmt::Display for IeeeTMacerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTMacerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Transmitted with Carrier Sense Error Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTCserr(pub u32);
impl IeeeTCserr {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTCserr {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTCserr(other)
    }
}

impl ::core::fmt::Display for IeeeTCserr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTCserr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flow Control Pause Frames Transmitted Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTFdxfc(pub u32);
impl IeeeTFdxfc {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTFdxfc {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTFdxfc(other)
    }
}

impl ::core::fmt::Display for IeeeTFdxfc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTFdxfc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Octet Count for Frames Transmitted w/o Error Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeTOctetsOk(pub u32);
impl IeeeTOctetsOk {
    #[doc="Octet count"]
    #[inline] pub fn count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Octet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Octet count"]
    #[inline] pub fn set_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeTOctetsOk {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeTOctetsOk(other)
    }
}

impl ::core::fmt::Display for IeeeTOctetsOk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeTOctetsOk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Packet Count Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRPackets(pub u32);
impl RmonRPackets {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRPackets {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRPackets(other)
    }
}

impl ::core::fmt::Display for RmonRPackets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRPackets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Broadcast Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRBcPkt(pub u32);
impl RmonRBcPkt {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRBcPkt {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRBcPkt(other)
    }
}

impl ::core::fmt::Display for RmonRBcPkt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRBcPkt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Multicast Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRMcPkt(pub u32);
impl RmonRMcPkt {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRMcPkt {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRMcPkt(other)
    }
}

impl ::core::fmt::Display for RmonRMcPkt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRMcPkt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Packets with CRC/Align Error Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRCrcAlign(pub u32);
impl RmonRCrcAlign {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRCrcAlign {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRCrcAlign(other)
    }
}

impl ::core::fmt::Display for RmonRCrcAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRCrcAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRUndersize(pub u32);
impl RmonRUndersize {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRUndersize {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRUndersize(other)
    }
}

impl ::core::fmt::Display for RmonRUndersize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRUndersize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonROversize(pub u32);
impl RmonROversize {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonROversize {
    #[inline]
    fn from(other: u32) -> Self {
         RmonROversize(other)
    }
}

impl ::core::fmt::Display for RmonROversize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonROversize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRFrag(pub u32);
impl RmonRFrag {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRFrag {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRFrag(other)
    }
}

impl ::core::fmt::Display for RmonRFrag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRFrag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRJab(pub u32);
impl RmonRJab {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRJab {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRJab(other)
    }
}

impl ::core::fmt::Display for RmonRJab {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRJab {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx 64-Byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRP64(pub u32);
impl RmonRP64 {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRP64 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRP64(other)
    }
}

impl ::core::fmt::Display for RmonRP64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRP64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx 65- to 127-Byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRP65to127(pub u32);
impl RmonRP65to127 {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRP65to127 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRP65to127(other)
    }
}

impl ::core::fmt::Display for RmonRP65to127 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRP65to127 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx 128- to 255-Byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRP128to255(pub u32);
impl RmonRP128to255 {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRP128to255 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRP128to255(other)
    }
}

impl ::core::fmt::Display for RmonRP128to255 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRP128to255 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx 256- to 511-Byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRP256to511(pub u32);
impl RmonRP256to511 {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRP256to511 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRP256to511(other)
    }
}

impl ::core::fmt::Display for RmonRP256to511 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRP256to511 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx 512- to 1023-Byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRP512to1023(pub u32);
impl RmonRP512to1023 {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRP512to1023 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRP512to1023(other)
    }
}

impl ::core::fmt::Display for RmonRP512to1023 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRP512to1023 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx 1024- to 2047-Byte Packets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRP1024to2047(pub u32);
impl RmonRP1024to2047 {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRP1024to2047 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRP1024to2047(other)
    }
}

impl ::core::fmt::Display for RmonRP1024to2047 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRP1024to2047 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Packets Greater than 2048 Bytes Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonRPGte2048(pub u32);
impl RmonRPGte2048 {
    #[doc="Packet count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Packet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Packet count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonRPGte2048 {
    #[inline]
    fn from(other: u32) -> Self {
         RmonRPGte2048(other)
    }
}

impl ::core::fmt::Display for RmonRPGte2048 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonRPGte2048 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Octets Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RmonROctets(pub u32);
impl RmonROctets {
    #[doc="Octet count"]
    #[inline] pub fn count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Octet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Octet count"]
    #[inline] pub fn set_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for RmonROctets {
    #[inline]
    fn from(other: u32) -> Self {
         RmonROctets(other)
    }
}

impl ::core::fmt::Display for RmonROctets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RmonROctets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames not Counted Correctly Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeRDrop(pub u32);
impl IeeeRDrop {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeRDrop {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeRDrop(other)
    }
}

impl ::core::fmt::Display for IeeeRDrop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeRDrop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Received OK Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeRFrameOk(pub u32);
impl IeeeRFrameOk {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeRFrameOk {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeRFrameOk(other)
    }
}

impl ::core::fmt::Display for IeeeRFrameOk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeRFrameOk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Received with CRC Error Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeRCrc(pub u32);
impl IeeeRCrc {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeRCrc {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeRCrc(other)
    }
}

impl ::core::fmt::Display for IeeeRCrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeRCrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frames Received with Alignment Error Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeRAlign(pub u32);
impl IeeeRAlign {
    #[doc="Frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeRAlign {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeRAlign(other)
    }
}

impl ::core::fmt::Display for IeeeRAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeRAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive FIFO Overflow Count Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeRMacerr(pub u32);
impl IeeeRMacerr {
    #[doc="Count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeRMacerr {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeRMacerr(other)
    }
}

impl ::core::fmt::Display for IeeeRMacerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeRMacerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flow Control Pause Frames Received Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeRFdxfc(pub u32);
impl IeeeRFdxfc {
    #[doc="Pause frame count"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Pause frame count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Pause frame count"]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeRFdxfc {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeRFdxfc(other)
    }
}

impl ::core::fmt::Display for IeeeRFdxfc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeRFdxfc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Octet Count for Frames Received without Error Statistic Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IeeeROctetsOk(pub u32);
impl IeeeROctetsOk {
    #[doc="Octet count"]
    #[inline] pub fn count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Octet count"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Octet count"]
    #[inline] pub fn set_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for IeeeROctetsOk {
    #[inline]
    fn from(other: u32) -> Self {
         IeeeROctetsOk(other)
    }
}

impl ::core::fmt::Display for IeeeROctetsOk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IeeeROctetsOk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Adjustable Timer Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atcr(pub u32);
impl Atcr {
    #[doc="Enable Timer"]
    #[inline] pub fn en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Enable Timer"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Enable Timer"]
    #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable One-Shot Offset Event"]
    #[inline] pub fn offen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Enable One-Shot Offset Event"]
    #[inline] pub fn test_offen(&self) -> bool {
        self.offen() != 0
    }

    #[doc="Enable One-Shot Offset Event"]
    #[inline] pub fn set_offen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Reset Timer On Offset Event"]
    #[inline] pub fn offrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Reset Timer On Offset Event"]
    #[inline] pub fn test_offrst(&self) -> bool {
        self.offrst() != 0
    }

    #[doc="Reset Timer On Offset Event"]
    #[inline] pub fn set_offrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enable Periodical Event"]
    #[inline] pub fn peren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Enable Periodical Event"]
    #[inline] pub fn test_peren(&self) -> bool {
        self.peren() != 0
    }

    #[doc="Enable Periodical Event"]
    #[inline] pub fn set_peren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enables event signal output assertion on period event"]
    #[inline] pub fn pinper(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Enables event signal output assertion on period event"]
    #[inline] pub fn test_pinper(&self) -> bool {
        self.pinper() != 0
    }

    #[doc="Enables event signal output assertion on period event"]
    #[inline] pub fn set_pinper<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Reset Timer"]
    #[inline] pub fn restart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Reset Timer"]
    #[inline] pub fn test_restart(&self) -> bool {
        self.restart() != 0
    }

    #[doc="Reset Timer"]
    #[inline] pub fn set_restart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Capture Timer Value"]
    #[inline] pub fn capture(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Capture Timer Value"]
    #[inline] pub fn test_capture(&self) -> bool {
        self.capture() != 0
    }

    #[doc="Capture Timer Value"]
    #[inline] pub fn set_capture<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Enable Timer Slave Mode"]
    #[inline] pub fn slave(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Enable Timer Slave Mode"]
    #[inline] pub fn test_slave(&self) -> bool {
        self.slave() != 0
    }

    #[doc="Enable Timer Slave Mode"]
    #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Atcr {
    #[inline]
    fn from(other: u32) -> Self {
         Atcr(other)
    }
}

impl ::core::fmt::Display for Atcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.en() != 0 { try!(write!(f, " en"))}
        if self.offen() != 0 { try!(write!(f, " offen"))}
        if self.offrst() != 0 { try!(write!(f, " offrst"))}
        if self.peren() != 0 { try!(write!(f, " peren"))}
        if self.pinper() != 0 { try!(write!(f, " pinper"))}
        if self.restart() != 0 { try!(write!(f, " restart"))}
        if self.capture() != 0 { try!(write!(f, " capture"))}
        if self.slave() != 0 { try!(write!(f, " slave"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atvr(pub u32);
impl Atvr {
    #[doc="A write sets the timer"]
    #[inline] pub fn atime(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="A write sets the timer"]
    #[inline] pub fn test_atime(&self) -> bool {
        self.atime() != 0
    }

    #[doc="A write sets the timer"]
    #[inline] pub fn set_atime<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Atvr {
    #[inline]
    fn from(other: u32) -> Self {
         Atvr(other)
    }
}

impl ::core::fmt::Display for Atvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Offset Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atoff(pub u32);
impl Atoff {
    #[doc="Offset value for one-shot event generation"]
    #[inline] pub fn offset(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Offset value for one-shot event generation"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Offset value for one-shot event generation"]
    #[inline] pub fn set_offset<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Atoff {
    #[inline]
    fn from(other: u32) -> Self {
         Atoff(other)
    }
}

impl ::core::fmt::Display for Atoff {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atoff {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Period Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atper(pub u32);
impl Atper {
    #[doc="Value for generating periodic events"]
    #[inline] pub fn period(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Value for generating periodic events"]
    #[inline] pub fn test_period(&self) -> bool {
        self.period() != 0
    }

    #[doc="Value for generating periodic events"]
    #[inline] pub fn set_period<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Atper {
    #[inline]
    fn from(other: u32) -> Self {
         Atper(other)
    }
}

impl ::core::fmt::Display for Atper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Correction Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atcor(pub u32);
impl Atcor {
    #[doc="Correction Counter Wrap-Around Value"]
    #[inline] pub fn cor(&self) -> bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="Correction Counter Wrap-Around Value"]
    #[inline] pub fn test_cor(&self) -> bool {
        self.cor() != 0
    }

    #[doc="Correction Counter Wrap-Around Value"]
    #[inline] pub fn set_cor<V: Into<bits::U31>>(mut self, value: V) -> Self {
        let value: bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Atcor {
    #[inline]
    fn from(other: u32) -> Self {
         Atcor(other)
    }
}

impl ::core::fmt::Display for Atcor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atcor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cor() != 0 { try!(write!(f, " cor=0x{:x}", self.cor()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Time-Stamping Clock Period Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atinc(pub u32);
impl Atinc {
    #[doc="Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline] pub fn inc(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline] pub fn test_inc(&self) -> bool {
        self.inc() != 0
    }

    #[doc="Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline] pub fn set_inc<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Correction Increment Value"]
    #[inline] pub fn inc_corr(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Correction Increment Value"]
    #[inline] pub fn test_inc_corr(&self) -> bool {
        self.inc_corr() != 0
    }

    #[doc="Correction Increment Value"]
    #[inline] pub fn set_inc_corr<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Atinc {
    #[inline]
    fn from(other: u32) -> Self {
         Atinc(other)
    }
}

impl ::core::fmt::Display for Atinc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atinc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inc() != 0 { try!(write!(f, " inc=0x{:x}", self.inc()))}
        if self.inc_corr() != 0 { try!(write!(f, " inc_corr=0x{:x}", self.inc_corr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timestamp of Last Transmitted Frame"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atstmp(pub u32);
impl Atstmp {
    #[doc="Timestamp of the last frame transmitted by the core that had TxBD[TS] set"]
    #[inline] pub fn timestamp(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Timestamp of the last frame transmitted by the core that had TxBD[TS] set"]
    #[inline] pub fn test_timestamp(&self) -> bool {
        self.timestamp() != 0
    }

    #[doc="Timestamp of the last frame transmitted by the core that had TxBD[TS] set"]
    #[inline] pub fn set_timestamp<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Atstmp {
    #[inline]
    fn from(other: u32) -> Self {
         Atstmp(other)
    }
}

impl ::core::fmt::Display for Atstmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atstmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Global Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tgsr(pub u32);
impl Tgsr {
    #[doc="Copy Of Timer Flag For Channel 0"]
    #[inline] pub fn tf0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Copy Of Timer Flag For Channel 0"]
    #[inline] pub fn test_tf0(&self) -> bool {
        self.tf0() != 0
    }

    #[doc="Copy Of Timer Flag For Channel 0"]
    #[inline] pub fn set_tf0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Copy Of Timer Flag For Channel 1"]
    #[inline] pub fn tf1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Copy Of Timer Flag For Channel 1"]
    #[inline] pub fn test_tf1(&self) -> bool {
        self.tf1() != 0
    }

    #[doc="Copy Of Timer Flag For Channel 1"]
    #[inline] pub fn set_tf1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Copy Of Timer Flag For Channel 2"]
    #[inline] pub fn tf2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Copy Of Timer Flag For Channel 2"]
    #[inline] pub fn test_tf2(&self) -> bool {
        self.tf2() != 0
    }

    #[doc="Copy Of Timer Flag For Channel 2"]
    #[inline] pub fn set_tf2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Copy Of Timer Flag For Channel 3"]
    #[inline] pub fn tf3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Copy Of Timer Flag For Channel 3"]
    #[inline] pub fn test_tf3(&self) -> bool {
        self.tf3() != 0
    }

    #[doc="Copy Of Timer Flag For Channel 3"]
    #[inline] pub fn set_tf3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Tgsr {
    #[inline]
    fn from(other: u32) -> Self {
         Tgsr(other)
    }
}

impl ::core::fmt::Display for Tgsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tgsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tf0() != 0 { try!(write!(f, " tf0"))}
        if self.tf1() != 0 { try!(write!(f, " tf1"))}
        if self.tf2() != 0 { try!(write!(f, " tf2"))}
        if self.tf3() != 0 { try!(write!(f, " tf3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Control Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcsr(pub u32);
impl Tcsr {
    #[doc="Timer DMA Request Enable"]
    #[inline] pub fn tdre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Timer DMA Request Enable"]
    #[inline] pub fn test_tdre(&self) -> bool {
        self.tdre() != 0
    }

    #[doc="Timer DMA Request Enable"]
    #[inline] pub fn set_tdre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer Mode"]
    #[inline] pub fn tmode(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0xf) as u8) } // [5:2]
    }

    #[doc="Timer Mode"]
    #[inline] pub fn test_tmode(&self) -> bool {
        self.tmode() != 0
    }

    #[doc="Timer Mode"]
    #[inline] pub fn set_tmode<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Timer Interrupt Enable"]
    #[inline] pub fn tie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Timer Interrupt Enable"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Timer Interrupt Enable"]
    #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Timer Flag"]
    #[inline] pub fn tf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Timer Flag"]
    #[inline] pub fn test_tf(&self) -> bool {
        self.tf() != 0
    }

    #[doc="Timer Flag"]
    #[inline] pub fn set_tf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Tcsr {
    #[inline]
    fn from(other: u32) -> Self {
         Tcsr(other)
    }
}

impl ::core::fmt::Display for Tcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdre() != 0 { try!(write!(f, " tdre"))}
        if self.tmode() != 0 { try!(write!(f, " tmode=0x{:x}", self.tmode()))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.tf() != 0 { try!(write!(f, " tf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timer Compare Capture Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tccr(pub u32);
impl Tccr {
    #[doc="Timer Capture Compare"]
    #[inline] pub fn tcc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Timer Capture Compare"]
    #[inline] pub fn test_tcc(&self) -> bool {
        self.tcc() != 0
    }

    #[doc="Timer Capture Compare"]
    #[inline] pub fn set_tcc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tccr {
    #[inline]
    fn from(other: u32) -> Self {
         Tccr(other)
    }
}

impl ::core::fmt::Display for Tccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


