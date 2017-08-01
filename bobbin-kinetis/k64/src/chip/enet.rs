//! Ethernet MAC-NET Core
pub const ENET: Enet = Enet(0x400c0000);

#[doc="Ethernet MAC-NET Core"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Enet(pub u32);
impl Enet {
#[doc="Get the *const pointer for the EIR register."]
  #[inline] pub fn eir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the EIR register."]
  #[inline] pub fn eir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the EIR register."]
  #[inline] pub fn eir(&self) -> Eir { 
     unsafe {
        Eir(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the EIR register."]
  #[inline] pub fn set_eir(&self, value: Eir) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EIR register."]
  #[inline] pub fn with_eir<F: FnOnce(Eir) -> Eir>(&self, f: F) -> &Self {
     let tmp = self.eir();
     self.set_eir(f(tmp))
  }

#[doc="Get the *const pointer for the EIMR register."]
  #[inline] pub fn eimr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the EIMR register."]
  #[inline] pub fn eimr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the EIMR register."]
  #[inline] pub fn eimr(&self) -> Eimr { 
     unsafe {
        Eimr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the EIMR register."]
  #[inline] pub fn set_eimr(&self, value: Eimr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EIMR register."]
  #[inline] pub fn with_eimr<F: FnOnce(Eimr) -> Eimr>(&self, f: F) -> &Self {
     let tmp = self.eimr();
     self.set_eimr(f(tmp))
  }

#[doc="Get the *const pointer for the RDAR register."]
  #[inline] pub fn rdar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the RDAR register."]
  #[inline] pub fn rdar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the RDAR register."]
  #[inline] pub fn rdar(&self) -> Rdar { 
     unsafe {
        Rdar(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the RDAR register."]
  #[inline] pub fn set_rdar(&self, value: Rdar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RDAR register."]
  #[inline] pub fn with_rdar<F: FnOnce(Rdar) -> Rdar>(&self, f: F) -> &Self {
     let tmp = self.rdar();
     self.set_rdar(f(tmp))
  }

#[doc="Get the *const pointer for the TDAR register."]
  #[inline] pub fn tdar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the TDAR register."]
  #[inline] pub fn tdar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the TDAR register."]
  #[inline] pub fn tdar(&self) -> Tdar { 
     unsafe {
        Tdar(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the TDAR register."]
  #[inline] pub fn set_tdar(&self, value: Tdar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TDAR register."]
  #[inline] pub fn with_tdar<F: FnOnce(Tdar) -> Tdar>(&self, f: F) -> &Self {
     let tmp = self.tdar();
     self.set_tdar(f(tmp))
  }

#[doc="Get the *const pointer for the ECR register."]
  #[inline] pub fn ecr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the ECR register."]
  #[inline] pub fn ecr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the ECR register."]
  #[inline] pub fn ecr(&self) -> Ecr { 
     unsafe {
        Ecr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the ECR register."]
  #[inline] pub fn set_ecr(&self, value: Ecr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ECR register."]
  #[inline] pub fn with_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
     let tmp = self.ecr();
     self.set_ecr(f(tmp))
  }

#[doc="Get the *const pointer for the MMFR register."]
  #[inline] pub fn mmfr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the MMFR register."]
  #[inline] pub fn mmfr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the MMFR register."]
  #[inline] pub fn mmfr(&self) -> Mmfr { 
     unsafe {
        Mmfr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the MMFR register."]
  #[inline] pub fn set_mmfr(&self, value: Mmfr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MMFR register."]
  #[inline] pub fn with_mmfr<F: FnOnce(Mmfr) -> Mmfr>(&self, f: F) -> &Self {
     let tmp = self.mmfr();
     self.set_mmfr(f(tmp))
  }

#[doc="Get the *const pointer for the MSCR register."]
  #[inline] pub fn mscr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the MSCR register."]
  #[inline] pub fn mscr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the MSCR register."]
  #[inline] pub fn mscr(&self) -> Mscr { 
     unsafe {
        Mscr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the MSCR register."]
  #[inline] pub fn set_mscr(&self, value: Mscr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MSCR register."]
  #[inline] pub fn with_mscr<F: FnOnce(Mscr) -> Mscr>(&self, f: F) -> &Self {
     let tmp = self.mscr();
     self.set_mscr(f(tmp))
  }

#[doc="Get the *const pointer for the MIBC register."]
  #[inline] pub fn mibc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x64) as *const u32
  }
#[doc="Get the *mut pointer for the MIBC register."]
  #[inline] pub fn mibc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x64) as *mut u32
  }
#[doc="Read the MIBC register."]
  #[inline] pub fn mibc(&self) -> Mibc { 
     unsafe {
        Mibc(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
     }
  }
#[doc="Write the MIBC register."]
  #[inline] pub fn set_mibc(&self, value: Mibc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MIBC register."]
  #[inline] pub fn with_mibc<F: FnOnce(Mibc) -> Mibc>(&self, f: F) -> &Self {
     let tmp = self.mibc();
     self.set_mibc(f(tmp))
  }

#[doc="Get the *const pointer for the RCR register."]
  #[inline] pub fn rcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x84) as *const u32
  }
#[doc="Get the *mut pointer for the RCR register."]
  #[inline] pub fn rcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x84) as *mut u32
  }
#[doc="Read the RCR register."]
  #[inline] pub fn rcr(&self) -> Rcr { 
     unsafe {
        Rcr(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
     }
  }
#[doc="Write the RCR register."]
  #[inline] pub fn set_rcr(&self, value: Rcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RCR register."]
  #[inline] pub fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
     let tmp = self.rcr();
     self.set_rcr(f(tmp))
  }

#[doc="Get the *const pointer for the TCR register."]
  #[inline] pub fn tcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc4) as *const u32
  }
#[doc="Get the *mut pointer for the TCR register."]
  #[inline] pub fn tcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc4) as *mut u32
  }
#[doc="Read the TCR register."]
  #[inline] pub fn tcr(&self) -> Tcr { 
     unsafe {
        Tcr(::core::ptr::read_volatile(((self.0 as usize) + 0xc4) as *const u32))
     }
  }
#[doc="Write the TCR register."]
  #[inline] pub fn set_tcr(&self, value: Tcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TCR register."]
  #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
     let tmp = self.tcr();
     self.set_tcr(f(tmp))
  }

#[doc="Get the *const pointer for the PALR register."]
  #[inline] pub fn palr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe4) as *const u32
  }
#[doc="Get the *mut pointer for the PALR register."]
  #[inline] pub fn palr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe4) as *mut u32
  }
#[doc="Read the PALR register."]
  #[inline] pub fn palr(&self) -> Palr { 
     unsafe {
        Palr(::core::ptr::read_volatile(((self.0 as usize) + 0xe4) as *const u32))
     }
  }
#[doc="Write the PALR register."]
  #[inline] pub fn set_palr(&self, value: Palr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PALR register."]
  #[inline] pub fn with_palr<F: FnOnce(Palr) -> Palr>(&self, f: F) -> &Self {
     let tmp = self.palr();
     self.set_palr(f(tmp))
  }

#[doc="Get the *const pointer for the PAUR register."]
  #[inline] pub fn paur_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe8) as *const u32
  }
#[doc="Get the *mut pointer for the PAUR register."]
  #[inline] pub fn paur_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe8) as *mut u32
  }
#[doc="Read the PAUR register."]
  #[inline] pub fn paur(&self) -> Paur { 
     unsafe {
        Paur(::core::ptr::read_volatile(((self.0 as usize) + 0xe8) as *const u32))
     }
  }
#[doc="Write the PAUR register."]
  #[inline] pub fn set_paur(&self, value: Paur) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PAUR register."]
  #[inline] pub fn with_paur<F: FnOnce(Paur) -> Paur>(&self, f: F) -> &Self {
     let tmp = self.paur();
     self.set_paur(f(tmp))
  }

#[doc="Get the *const pointer for the OPD register."]
  #[inline] pub fn opd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xec) as *const u32
  }
#[doc="Get the *mut pointer for the OPD register."]
  #[inline] pub fn opd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xec) as *mut u32
  }
#[doc="Read the OPD register."]
  #[inline] pub fn opd(&self) -> Opd { 
     unsafe {
        Opd(::core::ptr::read_volatile(((self.0 as usize) + 0xec) as *const u32))
     }
  }
#[doc="Write the OPD register."]
  #[inline] pub fn set_opd(&self, value: Opd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xec) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OPD register."]
  #[inline] pub fn with_opd<F: FnOnce(Opd) -> Opd>(&self, f: F) -> &Self {
     let tmp = self.opd();
     self.set_opd(f(tmp))
  }

#[doc="Get the *const pointer for the IAUR register."]
  #[inline] pub fn iaur_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x118) as *const u32
  }
#[doc="Get the *mut pointer for the IAUR register."]
  #[inline] pub fn iaur_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x118) as *mut u32
  }
#[doc="Read the IAUR register."]
  #[inline] pub fn iaur(&self) -> Iaur { 
     unsafe {
        Iaur(::core::ptr::read_volatile(((self.0 as usize) + 0x118) as *const u32))
     }
  }
#[doc="Write the IAUR register."]
  #[inline] pub fn set_iaur(&self, value: Iaur) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x118) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the IAUR register."]
  #[inline] pub fn with_iaur<F: FnOnce(Iaur) -> Iaur>(&self, f: F) -> &Self {
     let tmp = self.iaur();
     self.set_iaur(f(tmp))
  }

#[doc="Get the *const pointer for the IALR register."]
  #[inline] pub fn ialr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x11c) as *const u32
  }
#[doc="Get the *mut pointer for the IALR register."]
  #[inline] pub fn ialr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x11c) as *mut u32
  }
#[doc="Read the IALR register."]
  #[inline] pub fn ialr(&self) -> Ialr { 
     unsafe {
        Ialr(::core::ptr::read_volatile(((self.0 as usize) + 0x11c) as *const u32))
     }
  }
#[doc="Write the IALR register."]
  #[inline] pub fn set_ialr(&self, value: Ialr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x11c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the IALR register."]
  #[inline] pub fn with_ialr<F: FnOnce(Ialr) -> Ialr>(&self, f: F) -> &Self {
     let tmp = self.ialr();
     self.set_ialr(f(tmp))
  }

#[doc="Get the *const pointer for the GAUR register."]
  #[inline] pub fn gaur_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x120) as *const u32
  }
#[doc="Get the *mut pointer for the GAUR register."]
  #[inline] pub fn gaur_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x120) as *mut u32
  }
#[doc="Read the GAUR register."]
  #[inline] pub fn gaur(&self) -> Gaur { 
     unsafe {
        Gaur(::core::ptr::read_volatile(((self.0 as usize) + 0x120) as *const u32))
     }
  }
#[doc="Write the GAUR register."]
  #[inline] pub fn set_gaur(&self, value: Gaur) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x120) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the GAUR register."]
  #[inline] pub fn with_gaur<F: FnOnce(Gaur) -> Gaur>(&self, f: F) -> &Self {
     let tmp = self.gaur();
     self.set_gaur(f(tmp))
  }

#[doc="Get the *const pointer for the GALR register."]
  #[inline] pub fn galr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x124) as *const u32
  }
#[doc="Get the *mut pointer for the GALR register."]
  #[inline] pub fn galr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x124) as *mut u32
  }
#[doc="Read the GALR register."]
  #[inline] pub fn galr(&self) -> Galr { 
     unsafe {
        Galr(::core::ptr::read_volatile(((self.0 as usize) + 0x124) as *const u32))
     }
  }
#[doc="Write the GALR register."]
  #[inline] pub fn set_galr(&self, value: Galr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x124) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the GALR register."]
  #[inline] pub fn with_galr<F: FnOnce(Galr) -> Galr>(&self, f: F) -> &Self {
     let tmp = self.galr();
     self.set_galr(f(tmp))
  }

#[doc="Get the *const pointer for the TFWR register."]
  #[inline] pub fn tfwr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x144) as *const u32
  }
#[doc="Get the *mut pointer for the TFWR register."]
  #[inline] pub fn tfwr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x144) as *mut u32
  }
#[doc="Read the TFWR register."]
  #[inline] pub fn tfwr(&self) -> Tfwr { 
     unsafe {
        Tfwr(::core::ptr::read_volatile(((self.0 as usize) + 0x144) as *const u32))
     }
  }
#[doc="Write the TFWR register."]
  #[inline] pub fn set_tfwr(&self, value: Tfwr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x144) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TFWR register."]
  #[inline] pub fn with_tfwr<F: FnOnce(Tfwr) -> Tfwr>(&self, f: F) -> &Self {
     let tmp = self.tfwr();
     self.set_tfwr(f(tmp))
  }

#[doc="Get the *const pointer for the RDSR register."]
  #[inline] pub fn rdsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x180) as *const u32
  }
#[doc="Get the *mut pointer for the RDSR register."]
  #[inline] pub fn rdsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x180) as *mut u32
  }
#[doc="Read the RDSR register."]
  #[inline] pub fn rdsr(&self) -> Rdsr { 
     unsafe {
        Rdsr(::core::ptr::read_volatile(((self.0 as usize) + 0x180) as *const u32))
     }
  }
#[doc="Write the RDSR register."]
  #[inline] pub fn set_rdsr(&self, value: Rdsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x180) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RDSR register."]
  #[inline] pub fn with_rdsr<F: FnOnce(Rdsr) -> Rdsr>(&self, f: F) -> &Self {
     let tmp = self.rdsr();
     self.set_rdsr(f(tmp))
  }

#[doc="Get the *const pointer for the TDSR register."]
  #[inline] pub fn tdsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x184) as *const u32
  }
#[doc="Get the *mut pointer for the TDSR register."]
  #[inline] pub fn tdsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x184) as *mut u32
  }
#[doc="Read the TDSR register."]
  #[inline] pub fn tdsr(&self) -> Tdsr { 
     unsafe {
        Tdsr(::core::ptr::read_volatile(((self.0 as usize) + 0x184) as *const u32))
     }
  }
#[doc="Write the TDSR register."]
  #[inline] pub fn set_tdsr(&self, value: Tdsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x184) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TDSR register."]
  #[inline] pub fn with_tdsr<F: FnOnce(Tdsr) -> Tdsr>(&self, f: F) -> &Self {
     let tmp = self.tdsr();
     self.set_tdsr(f(tmp))
  }

#[doc="Get the *const pointer for the MRBR register."]
  #[inline] pub fn mrbr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x188) as *const u32
  }
#[doc="Get the *mut pointer for the MRBR register."]
  #[inline] pub fn mrbr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x188) as *mut u32
  }
#[doc="Read the MRBR register."]
  #[inline] pub fn mrbr(&self) -> Mrbr { 
     unsafe {
        Mrbr(::core::ptr::read_volatile(((self.0 as usize) + 0x188) as *const u32))
     }
  }
#[doc="Write the MRBR register."]
  #[inline] pub fn set_mrbr(&self, value: Mrbr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x188) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MRBR register."]
  #[inline] pub fn with_mrbr<F: FnOnce(Mrbr) -> Mrbr>(&self, f: F) -> &Self {
     let tmp = self.mrbr();
     self.set_mrbr(f(tmp))
  }

#[doc="Get the *const pointer for the RSFL register."]
  #[inline] pub fn rsfl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x190) as *const u32
  }
#[doc="Get the *mut pointer for the RSFL register."]
  #[inline] pub fn rsfl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x190) as *mut u32
  }
#[doc="Read the RSFL register."]
  #[inline] pub fn rsfl(&self) -> Rsfl { 
     unsafe {
        Rsfl(::core::ptr::read_volatile(((self.0 as usize) + 0x190) as *const u32))
     }
  }
#[doc="Write the RSFL register."]
  #[inline] pub fn set_rsfl(&self, value: Rsfl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x190) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RSFL register."]
  #[inline] pub fn with_rsfl<F: FnOnce(Rsfl) -> Rsfl>(&self, f: F) -> &Self {
     let tmp = self.rsfl();
     self.set_rsfl(f(tmp))
  }

#[doc="Get the *const pointer for the RSEM register."]
  #[inline] pub fn rsem_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x194) as *const u32
  }
#[doc="Get the *mut pointer for the RSEM register."]
  #[inline] pub fn rsem_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x194) as *mut u32
  }
#[doc="Read the RSEM register."]
  #[inline] pub fn rsem(&self) -> Rsem { 
     unsafe {
        Rsem(::core::ptr::read_volatile(((self.0 as usize) + 0x194) as *const u32))
     }
  }
#[doc="Write the RSEM register."]
  #[inline] pub fn set_rsem(&self, value: Rsem) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x194) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RSEM register."]
  #[inline] pub fn with_rsem<F: FnOnce(Rsem) -> Rsem>(&self, f: F) -> &Self {
     let tmp = self.rsem();
     self.set_rsem(f(tmp))
  }

#[doc="Get the *const pointer for the RAEM register."]
  #[inline] pub fn raem_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x198) as *const u32
  }
#[doc="Get the *mut pointer for the RAEM register."]
  #[inline] pub fn raem_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x198) as *mut u32
  }
#[doc="Read the RAEM register."]
  #[inline] pub fn raem(&self) -> Raem { 
     unsafe {
        Raem(::core::ptr::read_volatile(((self.0 as usize) + 0x198) as *const u32))
     }
  }
#[doc="Write the RAEM register."]
  #[inline] pub fn set_raem(&self, value: Raem) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x198) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RAEM register."]
  #[inline] pub fn with_raem<F: FnOnce(Raem) -> Raem>(&self, f: F) -> &Self {
     let tmp = self.raem();
     self.set_raem(f(tmp))
  }

#[doc="Get the *const pointer for the RAFL register."]
  #[inline] pub fn rafl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x19c) as *const u32
  }
#[doc="Get the *mut pointer for the RAFL register."]
  #[inline] pub fn rafl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x19c) as *mut u32
  }
#[doc="Read the RAFL register."]
  #[inline] pub fn rafl(&self) -> Rafl { 
     unsafe {
        Rafl(::core::ptr::read_volatile(((self.0 as usize) + 0x19c) as *const u32))
     }
  }
#[doc="Write the RAFL register."]
  #[inline] pub fn set_rafl(&self, value: Rafl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x19c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RAFL register."]
  #[inline] pub fn with_rafl<F: FnOnce(Rafl) -> Rafl>(&self, f: F) -> &Self {
     let tmp = self.rafl();
     self.set_rafl(f(tmp))
  }

#[doc="Get the *const pointer for the TSEM register."]
  #[inline] pub fn tsem_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1a0) as *const u32
  }
#[doc="Get the *mut pointer for the TSEM register."]
  #[inline] pub fn tsem_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1a0) as *mut u32
  }
#[doc="Read the TSEM register."]
  #[inline] pub fn tsem(&self) -> Tsem { 
     unsafe {
        Tsem(::core::ptr::read_volatile(((self.0 as usize) + 0x1a0) as *const u32))
     }
  }
#[doc="Write the TSEM register."]
  #[inline] pub fn set_tsem(&self, value: Tsem) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TSEM register."]
  #[inline] pub fn with_tsem<F: FnOnce(Tsem) -> Tsem>(&self, f: F) -> &Self {
     let tmp = self.tsem();
     self.set_tsem(f(tmp))
  }

#[doc="Get the *const pointer for the TAEM register."]
  #[inline] pub fn taem_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1a4) as *const u32
  }
#[doc="Get the *mut pointer for the TAEM register."]
  #[inline] pub fn taem_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1a4) as *mut u32
  }
#[doc="Read the TAEM register."]
  #[inline] pub fn taem(&self) -> Taem { 
     unsafe {
        Taem(::core::ptr::read_volatile(((self.0 as usize) + 0x1a4) as *const u32))
     }
  }
#[doc="Write the TAEM register."]
  #[inline] pub fn set_taem(&self, value: Taem) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TAEM register."]
  #[inline] pub fn with_taem<F: FnOnce(Taem) -> Taem>(&self, f: F) -> &Self {
     let tmp = self.taem();
     self.set_taem(f(tmp))
  }

#[doc="Get the *const pointer for the TAFL register."]
  #[inline] pub fn tafl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1a8) as *const u32
  }
#[doc="Get the *mut pointer for the TAFL register."]
  #[inline] pub fn tafl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1a8) as *mut u32
  }
#[doc="Read the TAFL register."]
  #[inline] pub fn tafl(&self) -> Tafl { 
     unsafe {
        Tafl(::core::ptr::read_volatile(((self.0 as usize) + 0x1a8) as *const u32))
     }
  }
#[doc="Write the TAFL register."]
  #[inline] pub fn set_tafl(&self, value: Tafl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TAFL register."]
  #[inline] pub fn with_tafl<F: FnOnce(Tafl) -> Tafl>(&self, f: F) -> &Self {
     let tmp = self.tafl();
     self.set_tafl(f(tmp))
  }

#[doc="Get the *const pointer for the TIPG register."]
  #[inline] pub fn tipg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1ac) as *const u32
  }
#[doc="Get the *mut pointer for the TIPG register."]
  #[inline] pub fn tipg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1ac) as *mut u32
  }
#[doc="Read the TIPG register."]
  #[inline] pub fn tipg(&self) -> Tipg { 
     unsafe {
        Tipg(::core::ptr::read_volatile(((self.0 as usize) + 0x1ac) as *const u32))
     }
  }
#[doc="Write the TIPG register."]
  #[inline] pub fn set_tipg(&self, value: Tipg) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1ac) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TIPG register."]
  #[inline] pub fn with_tipg<F: FnOnce(Tipg) -> Tipg>(&self, f: F) -> &Self {
     let tmp = self.tipg();
     self.set_tipg(f(tmp))
  }

#[doc="Get the *const pointer for the FTRL register."]
  #[inline] pub fn ftrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1b0) as *const u32
  }
#[doc="Get the *mut pointer for the FTRL register."]
  #[inline] pub fn ftrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1b0) as *mut u32
  }
#[doc="Read the FTRL register."]
  #[inline] pub fn ftrl(&self) -> Ftrl { 
     unsafe {
        Ftrl(::core::ptr::read_volatile(((self.0 as usize) + 0x1b0) as *const u32))
     }
  }
#[doc="Write the FTRL register."]
  #[inline] pub fn set_ftrl(&self, value: Ftrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1b0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FTRL register."]
  #[inline] pub fn with_ftrl<F: FnOnce(Ftrl) -> Ftrl>(&self, f: F) -> &Self {
     let tmp = self.ftrl();
     self.set_ftrl(f(tmp))
  }

#[doc="Get the *const pointer for the TACC register."]
  #[inline] pub fn tacc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c0) as *const u32
  }
#[doc="Get the *mut pointer for the TACC register."]
  #[inline] pub fn tacc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c0) as *mut u32
  }
#[doc="Read the TACC register."]
  #[inline] pub fn tacc(&self) -> Tacc { 
     unsafe {
        Tacc(::core::ptr::read_volatile(((self.0 as usize) + 0x1c0) as *const u32))
     }
  }
#[doc="Write the TACC register."]
  #[inline] pub fn set_tacc(&self, value: Tacc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TACC register."]
  #[inline] pub fn with_tacc<F: FnOnce(Tacc) -> Tacc>(&self, f: F) -> &Self {
     let tmp = self.tacc();
     self.set_tacc(f(tmp))
  }

#[doc="Get the *const pointer for the RACC register."]
  #[inline] pub fn racc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c4) as *const u32
  }
#[doc="Get the *mut pointer for the RACC register."]
  #[inline] pub fn racc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c4) as *mut u32
  }
#[doc="Read the RACC register."]
  #[inline] pub fn racc(&self) -> Racc { 
     unsafe {
        Racc(::core::ptr::read_volatile(((self.0 as usize) + 0x1c4) as *const u32))
     }
  }
#[doc="Write the RACC register."]
  #[inline] pub fn set_racc(&self, value: Racc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RACC register."]
  #[inline] pub fn with_racc<F: FnOnce(Racc) -> Racc>(&self, f: F) -> &Self {
     let tmp = self.racc();
     self.set_racc(f(tmp))
  }

#[doc="Get the *const pointer for the RMON_T_PACKETS register."]
  #[inline] pub fn rmon_t_packets_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x204) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_PACKETS register."]
  #[inline] pub fn rmon_t_packets_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x204) as *mut u32
  }
#[doc="Read the RMON_T_PACKETS register."]
  #[inline] pub fn rmon_t_packets(&self) -> RmonTPackets { 
     unsafe {
        RmonTPackets(::core::ptr::read_volatile(((self.0 as usize) + 0x204) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_BC_PKT register."]
  #[inline] pub fn rmon_t_bc_pkt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x208) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_BC_PKT register."]
  #[inline] pub fn rmon_t_bc_pkt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x208) as *mut u32
  }
#[doc="Read the RMON_T_BC_PKT register."]
  #[inline] pub fn rmon_t_bc_pkt(&self) -> RmonTBcPkt { 
     unsafe {
        RmonTBcPkt(::core::ptr::read_volatile(((self.0 as usize) + 0x208) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_MC_PKT register."]
  #[inline] pub fn rmon_t_mc_pkt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20c) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_MC_PKT register."]
  #[inline] pub fn rmon_t_mc_pkt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20c) as *mut u32
  }
#[doc="Read the RMON_T_MC_PKT register."]
  #[inline] pub fn rmon_t_mc_pkt(&self) -> RmonTMcPkt { 
     unsafe {
        RmonTMcPkt(::core::ptr::read_volatile(((self.0 as usize) + 0x20c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_CRC_ALIGN register."]
  #[inline] pub fn rmon_t_crc_align_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x210) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_CRC_ALIGN register."]
  #[inline] pub fn rmon_t_crc_align_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x210) as *mut u32
  }
#[doc="Read the RMON_T_CRC_ALIGN register."]
  #[inline] pub fn rmon_t_crc_align(&self) -> RmonTCrcAlign { 
     unsafe {
        RmonTCrcAlign(::core::ptr::read_volatile(((self.0 as usize) + 0x210) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_UNDERSIZE register."]
  #[inline] pub fn rmon_t_undersize_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x214) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_UNDERSIZE register."]
  #[inline] pub fn rmon_t_undersize_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x214) as *mut u32
  }
#[doc="Read the RMON_T_UNDERSIZE register."]
  #[inline] pub fn rmon_t_undersize(&self) -> RmonTUndersize { 
     unsafe {
        RmonTUndersize(::core::ptr::read_volatile(((self.0 as usize) + 0x214) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_OVERSIZE register."]
  #[inline] pub fn rmon_t_oversize_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x218) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_OVERSIZE register."]
  #[inline] pub fn rmon_t_oversize_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x218) as *mut u32
  }
#[doc="Read the RMON_T_OVERSIZE register."]
  #[inline] pub fn rmon_t_oversize(&self) -> RmonTOversize { 
     unsafe {
        RmonTOversize(::core::ptr::read_volatile(((self.0 as usize) + 0x218) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_FRAG register."]
  #[inline] pub fn rmon_t_frag_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x21c) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_FRAG register."]
  #[inline] pub fn rmon_t_frag_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x21c) as *mut u32
  }
#[doc="Read the RMON_T_FRAG register."]
  #[inline] pub fn rmon_t_frag(&self) -> RmonTFrag { 
     unsafe {
        RmonTFrag(::core::ptr::read_volatile(((self.0 as usize) + 0x21c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_JAB register."]
  #[inline] pub fn rmon_t_jab_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x220) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_JAB register."]
  #[inline] pub fn rmon_t_jab_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x220) as *mut u32
  }
#[doc="Read the RMON_T_JAB register."]
  #[inline] pub fn rmon_t_jab(&self) -> RmonTJab { 
     unsafe {
        RmonTJab(::core::ptr::read_volatile(((self.0 as usize) + 0x220) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_COL register."]
  #[inline] pub fn rmon_t_col_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x224) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_COL register."]
  #[inline] pub fn rmon_t_col_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x224) as *mut u32
  }
#[doc="Read the RMON_T_COL register."]
  #[inline] pub fn rmon_t_col(&self) -> RmonTCol { 
     unsafe {
        RmonTCol(::core::ptr::read_volatile(((self.0 as usize) + 0x224) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_P64 register."]
  #[inline] pub fn rmon_t_p64_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x228) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_P64 register."]
  #[inline] pub fn rmon_t_p64_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x228) as *mut u32
  }
#[doc="Read the RMON_T_P64 register."]
  #[inline] pub fn rmon_t_p64(&self) -> RmonTP64 { 
     unsafe {
        RmonTP64(::core::ptr::read_volatile(((self.0 as usize) + 0x228) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_P65TO127 register."]
  #[inline] pub fn rmon_t_p65to127_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x22c) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_P65TO127 register."]
  #[inline] pub fn rmon_t_p65to127_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x22c) as *mut u32
  }
#[doc="Read the RMON_T_P65TO127 register."]
  #[inline] pub fn rmon_t_p65to127(&self) -> RmonTP65to127 { 
     unsafe {
        RmonTP65to127(::core::ptr::read_volatile(((self.0 as usize) + 0x22c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_P128TO255 register."]
  #[inline] pub fn rmon_t_p128to255_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x230) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_P128TO255 register."]
  #[inline] pub fn rmon_t_p128to255_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x230) as *mut u32
  }
#[doc="Read the RMON_T_P128TO255 register."]
  #[inline] pub fn rmon_t_p128to255(&self) -> RmonTP128to255 { 
     unsafe {
        RmonTP128to255(::core::ptr::read_volatile(((self.0 as usize) + 0x230) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_P256TO511 register."]
  #[inline] pub fn rmon_t_p256to511_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x234) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_P256TO511 register."]
  #[inline] pub fn rmon_t_p256to511_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x234) as *mut u32
  }
#[doc="Read the RMON_T_P256TO511 register."]
  #[inline] pub fn rmon_t_p256to511(&self) -> RmonTP256to511 { 
     unsafe {
        RmonTP256to511(::core::ptr::read_volatile(((self.0 as usize) + 0x234) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_P512TO1023 register."]
  #[inline] pub fn rmon_t_p512to1023_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x238) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_P512TO1023 register."]
  #[inline] pub fn rmon_t_p512to1023_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x238) as *mut u32
  }
#[doc="Read the RMON_T_P512TO1023 register."]
  #[inline] pub fn rmon_t_p512to1023(&self) -> RmonTP512to1023 { 
     unsafe {
        RmonTP512to1023(::core::ptr::read_volatile(((self.0 as usize) + 0x238) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_P1024TO2047 register."]
  #[inline] pub fn rmon_t_p1024to2047_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x23c) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_P1024TO2047 register."]
  #[inline] pub fn rmon_t_p1024to2047_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x23c) as *mut u32
  }
#[doc="Read the RMON_T_P1024TO2047 register."]
  #[inline] pub fn rmon_t_p1024to2047(&self) -> RmonTP1024to2047 { 
     unsafe {
        RmonTP1024to2047(::core::ptr::read_volatile(((self.0 as usize) + 0x23c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_P_GTE2048 register."]
  #[inline] pub fn rmon_t_p_gte2048_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x240) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_P_GTE2048 register."]
  #[inline] pub fn rmon_t_p_gte2048_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x240) as *mut u32
  }
#[doc="Read the RMON_T_P_GTE2048 register."]
  #[inline] pub fn rmon_t_p_gte2048(&self) -> RmonTPGte2048 { 
     unsafe {
        RmonTPGte2048(::core::ptr::read_volatile(((self.0 as usize) + 0x240) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_T_OCTETS register."]
  #[inline] pub fn rmon_t_octets_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x244) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_T_OCTETS register."]
  #[inline] pub fn rmon_t_octets_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x244) as *mut u32
  }
#[doc="Read the RMON_T_OCTETS register."]
  #[inline] pub fn rmon_t_octets(&self) -> RmonTOctets { 
     unsafe {
        RmonTOctets(::core::ptr::read_volatile(((self.0 as usize) + 0x244) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_FRAME_OK register."]
  #[inline] pub fn ieee_t_frame_ok_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24c) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_FRAME_OK register."]
  #[inline] pub fn ieee_t_frame_ok_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24c) as *mut u32
  }
#[doc="Read the IEEE_T_FRAME_OK register."]
  #[inline] pub fn ieee_t_frame_ok(&self) -> IeeeTFrameOk { 
     unsafe {
        IeeeTFrameOk(::core::ptr::read_volatile(((self.0 as usize) + 0x24c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_1COL register."]
  #[inline] pub fn ieee_t_1col_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x250) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_1COL register."]
  #[inline] pub fn ieee_t_1col_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x250) as *mut u32
  }
#[doc="Read the IEEE_T_1COL register."]
  #[inline] pub fn ieee_t_1col(&self) -> IeeeT1col { 
     unsafe {
        IeeeT1col(::core::ptr::read_volatile(((self.0 as usize) + 0x250) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_MCOL register."]
  #[inline] pub fn ieee_t_mcol_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x254) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_MCOL register."]
  #[inline] pub fn ieee_t_mcol_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x254) as *mut u32
  }
#[doc="Read the IEEE_T_MCOL register."]
  #[inline] pub fn ieee_t_mcol(&self) -> IeeeTMcol { 
     unsafe {
        IeeeTMcol(::core::ptr::read_volatile(((self.0 as usize) + 0x254) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_DEF register."]
  #[inline] pub fn ieee_t_def_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x258) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_DEF register."]
  #[inline] pub fn ieee_t_def_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x258) as *mut u32
  }
#[doc="Read the IEEE_T_DEF register."]
  #[inline] pub fn ieee_t_def(&self) -> IeeeTDef { 
     unsafe {
        IeeeTDef(::core::ptr::read_volatile(((self.0 as usize) + 0x258) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_LCOL register."]
  #[inline] pub fn ieee_t_lcol_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x25c) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_LCOL register."]
  #[inline] pub fn ieee_t_lcol_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x25c) as *mut u32
  }
#[doc="Read the IEEE_T_LCOL register."]
  #[inline] pub fn ieee_t_lcol(&self) -> IeeeTLcol { 
     unsafe {
        IeeeTLcol(::core::ptr::read_volatile(((self.0 as usize) + 0x25c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_EXCOL register."]
  #[inline] pub fn ieee_t_excol_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x260) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_EXCOL register."]
  #[inline] pub fn ieee_t_excol_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x260) as *mut u32
  }
#[doc="Read the IEEE_T_EXCOL register."]
  #[inline] pub fn ieee_t_excol(&self) -> IeeeTExcol { 
     unsafe {
        IeeeTExcol(::core::ptr::read_volatile(((self.0 as usize) + 0x260) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_MACERR register."]
  #[inline] pub fn ieee_t_macerr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x264) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_MACERR register."]
  #[inline] pub fn ieee_t_macerr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x264) as *mut u32
  }
#[doc="Read the IEEE_T_MACERR register."]
  #[inline] pub fn ieee_t_macerr(&self) -> IeeeTMacerr { 
     unsafe {
        IeeeTMacerr(::core::ptr::read_volatile(((self.0 as usize) + 0x264) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_CSERR register."]
  #[inline] pub fn ieee_t_cserr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x268) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_CSERR register."]
  #[inline] pub fn ieee_t_cserr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x268) as *mut u32
  }
#[doc="Read the IEEE_T_CSERR register."]
  #[inline] pub fn ieee_t_cserr(&self) -> IeeeTCserr { 
     unsafe {
        IeeeTCserr(::core::ptr::read_volatile(((self.0 as usize) + 0x268) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_FDXFC register."]
  #[inline] pub fn ieee_t_fdxfc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x270) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_FDXFC register."]
  #[inline] pub fn ieee_t_fdxfc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x270) as *mut u32
  }
#[doc="Read the IEEE_T_FDXFC register."]
  #[inline] pub fn ieee_t_fdxfc(&self) -> IeeeTFdxfc { 
     unsafe {
        IeeeTFdxfc(::core::ptr::read_volatile(((self.0 as usize) + 0x270) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_T_OCTETS_OK register."]
  #[inline] pub fn ieee_t_octets_ok_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x274) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_T_OCTETS_OK register."]
  #[inline] pub fn ieee_t_octets_ok_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x274) as *mut u32
  }
#[doc="Read the IEEE_T_OCTETS_OK register."]
  #[inline] pub fn ieee_t_octets_ok(&self) -> IeeeTOctetsOk { 
     unsafe {
        IeeeTOctetsOk(::core::ptr::read_volatile(((self.0 as usize) + 0x274) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_PACKETS register."]
  #[inline] pub fn rmon_r_packets_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x284) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_PACKETS register."]
  #[inline] pub fn rmon_r_packets_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x284) as *mut u32
  }
#[doc="Read the RMON_R_PACKETS register."]
  #[inline] pub fn rmon_r_packets(&self) -> RmonRPackets { 
     unsafe {
        RmonRPackets(::core::ptr::read_volatile(((self.0 as usize) + 0x284) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_BC_PKT register."]
  #[inline] pub fn rmon_r_bc_pkt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x288) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_BC_PKT register."]
  #[inline] pub fn rmon_r_bc_pkt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x288) as *mut u32
  }
#[doc="Read the RMON_R_BC_PKT register."]
  #[inline] pub fn rmon_r_bc_pkt(&self) -> RmonRBcPkt { 
     unsafe {
        RmonRBcPkt(::core::ptr::read_volatile(((self.0 as usize) + 0x288) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_MC_PKT register."]
  #[inline] pub fn rmon_r_mc_pkt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28c) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_MC_PKT register."]
  #[inline] pub fn rmon_r_mc_pkt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28c) as *mut u32
  }
#[doc="Read the RMON_R_MC_PKT register."]
  #[inline] pub fn rmon_r_mc_pkt(&self) -> RmonRMcPkt { 
     unsafe {
        RmonRMcPkt(::core::ptr::read_volatile(((self.0 as usize) + 0x28c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_CRC_ALIGN register."]
  #[inline] pub fn rmon_r_crc_align_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x290) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_CRC_ALIGN register."]
  #[inline] pub fn rmon_r_crc_align_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x290) as *mut u32
  }
#[doc="Read the RMON_R_CRC_ALIGN register."]
  #[inline] pub fn rmon_r_crc_align(&self) -> RmonRCrcAlign { 
     unsafe {
        RmonRCrcAlign(::core::ptr::read_volatile(((self.0 as usize) + 0x290) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_UNDERSIZE register."]
  #[inline] pub fn rmon_r_undersize_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x294) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_UNDERSIZE register."]
  #[inline] pub fn rmon_r_undersize_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x294) as *mut u32
  }
#[doc="Read the RMON_R_UNDERSIZE register."]
  #[inline] pub fn rmon_r_undersize(&self) -> RmonRUndersize { 
     unsafe {
        RmonRUndersize(::core::ptr::read_volatile(((self.0 as usize) + 0x294) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_OVERSIZE register."]
  #[inline] pub fn rmon_r_oversize_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x298) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_OVERSIZE register."]
  #[inline] pub fn rmon_r_oversize_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x298) as *mut u32
  }
#[doc="Read the RMON_R_OVERSIZE register."]
  #[inline] pub fn rmon_r_oversize(&self) -> RmonROversize { 
     unsafe {
        RmonROversize(::core::ptr::read_volatile(((self.0 as usize) + 0x298) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_FRAG register."]
  #[inline] pub fn rmon_r_frag_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x29c) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_FRAG register."]
  #[inline] pub fn rmon_r_frag_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x29c) as *mut u32
  }
#[doc="Read the RMON_R_FRAG register."]
  #[inline] pub fn rmon_r_frag(&self) -> RmonRFrag { 
     unsafe {
        RmonRFrag(::core::ptr::read_volatile(((self.0 as usize) + 0x29c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_JAB register."]
  #[inline] pub fn rmon_r_jab_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2a0) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_JAB register."]
  #[inline] pub fn rmon_r_jab_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2a0) as *mut u32
  }
#[doc="Read the RMON_R_JAB register."]
  #[inline] pub fn rmon_r_jab(&self) -> RmonRJab { 
     unsafe {
        RmonRJab(::core::ptr::read_volatile(((self.0 as usize) + 0x2a0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_P64 register."]
  #[inline] pub fn rmon_r_p64_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2a8) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_P64 register."]
  #[inline] pub fn rmon_r_p64_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2a8) as *mut u32
  }
#[doc="Read the RMON_R_P64 register."]
  #[inline] pub fn rmon_r_p64(&self) -> RmonRP64 { 
     unsafe {
        RmonRP64(::core::ptr::read_volatile(((self.0 as usize) + 0x2a8) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_P65TO127 register."]
  #[inline] pub fn rmon_r_p65to127_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2ac) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_P65TO127 register."]
  #[inline] pub fn rmon_r_p65to127_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2ac) as *mut u32
  }
#[doc="Read the RMON_R_P65TO127 register."]
  #[inline] pub fn rmon_r_p65to127(&self) -> RmonRP65to127 { 
     unsafe {
        RmonRP65to127(::core::ptr::read_volatile(((self.0 as usize) + 0x2ac) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_P128TO255 register."]
  #[inline] pub fn rmon_r_p128to255_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2b0) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_P128TO255 register."]
  #[inline] pub fn rmon_r_p128to255_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2b0) as *mut u32
  }
#[doc="Read the RMON_R_P128TO255 register."]
  #[inline] pub fn rmon_r_p128to255(&self) -> RmonRP128to255 { 
     unsafe {
        RmonRP128to255(::core::ptr::read_volatile(((self.0 as usize) + 0x2b0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_P256TO511 register."]
  #[inline] pub fn rmon_r_p256to511_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2b4) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_P256TO511 register."]
  #[inline] pub fn rmon_r_p256to511_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2b4) as *mut u32
  }
#[doc="Read the RMON_R_P256TO511 register."]
  #[inline] pub fn rmon_r_p256to511(&self) -> RmonRP256to511 { 
     unsafe {
        RmonRP256to511(::core::ptr::read_volatile(((self.0 as usize) + 0x2b4) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_P512TO1023 register."]
  #[inline] pub fn rmon_r_p512to1023_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2b8) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_P512TO1023 register."]
  #[inline] pub fn rmon_r_p512to1023_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2b8) as *mut u32
  }
#[doc="Read the RMON_R_P512TO1023 register."]
  #[inline] pub fn rmon_r_p512to1023(&self) -> RmonRP512to1023 { 
     unsafe {
        RmonRP512to1023(::core::ptr::read_volatile(((self.0 as usize) + 0x2b8) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_P1024TO2047 register."]
  #[inline] pub fn rmon_r_p1024to2047_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2bc) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_P1024TO2047 register."]
  #[inline] pub fn rmon_r_p1024to2047_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2bc) as *mut u32
  }
#[doc="Read the RMON_R_P1024TO2047 register."]
  #[inline] pub fn rmon_r_p1024to2047(&self) -> RmonRP1024to2047 { 
     unsafe {
        RmonRP1024to2047(::core::ptr::read_volatile(((self.0 as usize) + 0x2bc) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_P_GTE2048 register."]
  #[inline] pub fn rmon_r_p_gte2048_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c0) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_P_GTE2048 register."]
  #[inline] pub fn rmon_r_p_gte2048_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c0) as *mut u32
  }
#[doc="Read the RMON_R_P_GTE2048 register."]
  #[inline] pub fn rmon_r_p_gte2048(&self) -> RmonRPGte2048 { 
     unsafe {
        RmonRPGte2048(::core::ptr::read_volatile(((self.0 as usize) + 0x2c0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RMON_R_OCTETS register."]
  #[inline] pub fn rmon_r_octets_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c4) as *const u32
  }
#[doc="Get the *mut pointer for the RMON_R_OCTETS register."]
  #[inline] pub fn rmon_r_octets_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c4) as *mut u32
  }
#[doc="Read the RMON_R_OCTETS register."]
  #[inline] pub fn rmon_r_octets(&self) -> RmonROctets { 
     unsafe {
        RmonROctets(::core::ptr::read_volatile(((self.0 as usize) + 0x2c4) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_R_DROP register."]
  #[inline] pub fn ieee_r_drop_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c8) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_R_DROP register."]
  #[inline] pub fn ieee_r_drop_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c8) as *mut u32
  }
#[doc="Read the IEEE_R_DROP register."]
  #[inline] pub fn ieee_r_drop(&self) -> IeeeRDrop { 
     unsafe {
        IeeeRDrop(::core::ptr::read_volatile(((self.0 as usize) + 0x2c8) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_R_FRAME_OK register."]
  #[inline] pub fn ieee_r_frame_ok_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2cc) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_R_FRAME_OK register."]
  #[inline] pub fn ieee_r_frame_ok_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2cc) as *mut u32
  }
#[doc="Read the IEEE_R_FRAME_OK register."]
  #[inline] pub fn ieee_r_frame_ok(&self) -> IeeeRFrameOk { 
     unsafe {
        IeeeRFrameOk(::core::ptr::read_volatile(((self.0 as usize) + 0x2cc) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_R_CRC register."]
  #[inline] pub fn ieee_r_crc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2d0) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_R_CRC register."]
  #[inline] pub fn ieee_r_crc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2d0) as *mut u32
  }
#[doc="Read the IEEE_R_CRC register."]
  #[inline] pub fn ieee_r_crc(&self) -> IeeeRCrc { 
     unsafe {
        IeeeRCrc(::core::ptr::read_volatile(((self.0 as usize) + 0x2d0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_R_ALIGN register."]
  #[inline] pub fn ieee_r_align_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2d4) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_R_ALIGN register."]
  #[inline] pub fn ieee_r_align_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2d4) as *mut u32
  }
#[doc="Read the IEEE_R_ALIGN register."]
  #[inline] pub fn ieee_r_align(&self) -> IeeeRAlign { 
     unsafe {
        IeeeRAlign(::core::ptr::read_volatile(((self.0 as usize) + 0x2d4) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_R_MACERR register."]
  #[inline] pub fn ieee_r_macerr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2d8) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_R_MACERR register."]
  #[inline] pub fn ieee_r_macerr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2d8) as *mut u32
  }
#[doc="Read the IEEE_R_MACERR register."]
  #[inline] pub fn ieee_r_macerr(&self) -> IeeeRMacerr { 
     unsafe {
        IeeeRMacerr(::core::ptr::read_volatile(((self.0 as usize) + 0x2d8) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_R_FDXFC register."]
  #[inline] pub fn ieee_r_fdxfc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2dc) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_R_FDXFC register."]
  #[inline] pub fn ieee_r_fdxfc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2dc) as *mut u32
  }
#[doc="Read the IEEE_R_FDXFC register."]
  #[inline] pub fn ieee_r_fdxfc(&self) -> IeeeRFdxfc { 
     unsafe {
        IeeeRFdxfc(::core::ptr::read_volatile(((self.0 as usize) + 0x2dc) as *const u32))
     }
  }

#[doc="Get the *const pointer for the IEEE_R_OCTETS_OK register."]
  #[inline] pub fn ieee_r_octets_ok_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2e0) as *const u32
  }
#[doc="Get the *mut pointer for the IEEE_R_OCTETS_OK register."]
  #[inline] pub fn ieee_r_octets_ok_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2e0) as *mut u32
  }
#[doc="Read the IEEE_R_OCTETS_OK register."]
  #[inline] pub fn ieee_r_octets_ok(&self) -> IeeeROctetsOk { 
     unsafe {
        IeeeROctetsOk(::core::ptr::read_volatile(((self.0 as usize) + 0x2e0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the ATCR register."]
  #[inline] pub fn atcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x400) as *const u32
  }
#[doc="Get the *mut pointer for the ATCR register."]
  #[inline] pub fn atcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x400) as *mut u32
  }
#[doc="Read the ATCR register."]
  #[inline] pub fn atcr(&self) -> Atcr { 
     unsafe {
        Atcr(::core::ptr::read_volatile(((self.0 as usize) + 0x400) as *const u32))
     }
  }
#[doc="Write the ATCR register."]
  #[inline] pub fn set_atcr(&self, value: Atcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x400) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ATCR register."]
  #[inline] pub fn with_atcr<F: FnOnce(Atcr) -> Atcr>(&self, f: F) -> &Self {
     let tmp = self.atcr();
     self.set_atcr(f(tmp))
  }

#[doc="Get the *const pointer for the ATVR register."]
  #[inline] pub fn atvr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x404) as *const u32
  }
#[doc="Get the *mut pointer for the ATVR register."]
  #[inline] pub fn atvr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x404) as *mut u32
  }
#[doc="Read the ATVR register."]
  #[inline] pub fn atvr(&self) -> Atvr { 
     unsafe {
        Atvr(::core::ptr::read_volatile(((self.0 as usize) + 0x404) as *const u32))
     }
  }
#[doc="Write the ATVR register."]
  #[inline] pub fn set_atvr(&self, value: Atvr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x404) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ATVR register."]
  #[inline] pub fn with_atvr<F: FnOnce(Atvr) -> Atvr>(&self, f: F) -> &Self {
     let tmp = self.atvr();
     self.set_atvr(f(tmp))
  }

#[doc="Get the *const pointer for the ATOFF register."]
  #[inline] pub fn atoff_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x408) as *const u32
  }
#[doc="Get the *mut pointer for the ATOFF register."]
  #[inline] pub fn atoff_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x408) as *mut u32
  }
#[doc="Read the ATOFF register."]
  #[inline] pub fn atoff(&self) -> Atoff { 
     unsafe {
        Atoff(::core::ptr::read_volatile(((self.0 as usize) + 0x408) as *const u32))
     }
  }
#[doc="Write the ATOFF register."]
  #[inline] pub fn set_atoff(&self, value: Atoff) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x408) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ATOFF register."]
  #[inline] pub fn with_atoff<F: FnOnce(Atoff) -> Atoff>(&self, f: F) -> &Self {
     let tmp = self.atoff();
     self.set_atoff(f(tmp))
  }

#[doc="Get the *const pointer for the ATPER register."]
  #[inline] pub fn atper_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40c) as *const u32
  }
#[doc="Get the *mut pointer for the ATPER register."]
  #[inline] pub fn atper_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40c) as *mut u32
  }
#[doc="Read the ATPER register."]
  #[inline] pub fn atper(&self) -> Atper { 
     unsafe {
        Atper(::core::ptr::read_volatile(((self.0 as usize) + 0x40c) as *const u32))
     }
  }
#[doc="Write the ATPER register."]
  #[inline] pub fn set_atper(&self, value: Atper) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ATPER register."]
  #[inline] pub fn with_atper<F: FnOnce(Atper) -> Atper>(&self, f: F) -> &Self {
     let tmp = self.atper();
     self.set_atper(f(tmp))
  }

#[doc="Get the *const pointer for the ATCOR register."]
  #[inline] pub fn atcor_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x410) as *const u32
  }
#[doc="Get the *mut pointer for the ATCOR register."]
  #[inline] pub fn atcor_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x410) as *mut u32
  }
#[doc="Read the ATCOR register."]
  #[inline] pub fn atcor(&self) -> Atcor { 
     unsafe {
        Atcor(::core::ptr::read_volatile(((self.0 as usize) + 0x410) as *const u32))
     }
  }
#[doc="Write the ATCOR register."]
  #[inline] pub fn set_atcor(&self, value: Atcor) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x410) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ATCOR register."]
  #[inline] pub fn with_atcor<F: FnOnce(Atcor) -> Atcor>(&self, f: F) -> &Self {
     let tmp = self.atcor();
     self.set_atcor(f(tmp))
  }

#[doc="Get the *const pointer for the ATINC register."]
  #[inline] pub fn atinc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x414) as *const u32
  }
#[doc="Get the *mut pointer for the ATINC register."]
  #[inline] pub fn atinc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x414) as *mut u32
  }
#[doc="Read the ATINC register."]
  #[inline] pub fn atinc(&self) -> Atinc { 
     unsafe {
        Atinc(::core::ptr::read_volatile(((self.0 as usize) + 0x414) as *const u32))
     }
  }
#[doc="Write the ATINC register."]
  #[inline] pub fn set_atinc(&self, value: Atinc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x414) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ATINC register."]
  #[inline] pub fn with_atinc<F: FnOnce(Atinc) -> Atinc>(&self, f: F) -> &Self {
     let tmp = self.atinc();
     self.set_atinc(f(tmp))
  }

#[doc="Get the *const pointer for the ATSTMP register."]
  #[inline] pub fn atstmp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x418) as *const u32
  }
#[doc="Get the *mut pointer for the ATSTMP register."]
  #[inline] pub fn atstmp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x418) as *mut u32
  }
#[doc="Read the ATSTMP register."]
  #[inline] pub fn atstmp(&self) -> Atstmp { 
     unsafe {
        Atstmp(::core::ptr::read_volatile(((self.0 as usize) + 0x418) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TGSR register."]
  #[inline] pub fn tgsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x604) as *const u32
  }
#[doc="Get the *mut pointer for the TGSR register."]
  #[inline] pub fn tgsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x604) as *mut u32
  }
#[doc="Read the TGSR register."]
  #[inline] pub fn tgsr(&self) -> Tgsr { 
     unsafe {
        Tgsr(::core::ptr::read_volatile(((self.0 as usize) + 0x604) as *const u32))
     }
  }
#[doc="Write the TGSR register."]
  #[inline] pub fn set_tgsr(&self, value: Tgsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x604) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TGSR register."]
  #[inline] pub fn with_tgsr<F: FnOnce(Tgsr) -> Tgsr>(&self, f: F) -> &Self {
     let tmp = self.tgsr();
     self.set_tgsr(f(tmp))
  }

#[doc="Get the *const pointer for the TCSR register."]
  #[inline] pub fn tcsr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x608 + (index << 3)) as *const u32
  }
#[doc="Get the *mut pointer for the TCSR register."]
  #[inline] pub fn tcsr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x608 + (index << 3)) as *mut u32
  }
#[doc="Read the TCSR register."]
  #[inline] pub fn tcsr(&self, index: usize) -> Tcsr { 
     assert!(index < 4);
     unsafe {
        Tcsr(::core::ptr::read_volatile(((self.0 as usize) + 0x608 + (index << 3)) as *const u32))
     }
  }
#[doc="Write the TCSR register."]
  #[inline] pub fn set_tcsr(&self, index: usize, value: Tcsr) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x608 + (index << 3)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TCSR register."]
  #[inline] pub fn with_tcsr<F: FnOnce(Tcsr) -> Tcsr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcsr(index);
     self.set_tcsr(index, f(tmp))
  }

#[doc="Get the *const pointer for the TCCR register."]
  #[inline] pub fn tccr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x60c + (index << 3)) as *const u32
  }
#[doc="Get the *mut pointer for the TCCR register."]
  #[inline] pub fn tccr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x60c + (index << 3)) as *mut u32
  }
#[doc="Read the TCCR register."]
  #[inline] pub fn tccr(&self, index: usize) -> Tccr { 
     assert!(index < 4);
     unsafe {
        Tccr(::core::ptr::read_volatile(((self.0 as usize) + 0x60c + (index << 3)) as *const u32))
     }
  }
#[doc="Write the TCCR register."]
  #[inline] pub fn set_tccr(&self, index: usize, value: Tccr) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60c + (index << 3)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TCCR register."]
  #[inline] pub fn with_tccr<F: FnOnce(Tccr) -> Tccr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tccr(index);
     self.set_tccr(index, f(tmp))
  }

}

#[doc="Interrupt Event Register"]
#[derive(PartialEq, Eq)]
pub struct Eir(pub u32);
impl Eir {
#[doc="Timestamp Timer"]
  #[inline] pub fn ts_timer(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Timestamp Timer"]
  #[inline] pub fn set_ts_timer(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Transmit Timestamp Available"]
  #[inline] pub fn ts_avail(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Transmit Timestamp Available"]
  #[inline] pub fn set_ts_avail(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Node Wakeup Request Indication"]
  #[inline] pub fn wakeup(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Node Wakeup Request Indication"]
  #[inline] pub fn set_wakeup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Payload Receive Error"]
  #[inline] pub fn plr(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Payload Receive Error"]
  #[inline] pub fn set_plr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Transmit FIFO Underrun"]
  #[inline] pub fn un(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Transmit FIFO Underrun"]
  #[inline] pub fn set_un(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Collision Retry Limit"]
  #[inline] pub fn rl(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="Collision Retry Limit"]
  #[inline] pub fn set_rl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Late Collision"]
  #[inline] pub fn lc(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="Late Collision"]
  #[inline] pub fn set_lc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Ethernet Bus Error"]
  #[inline] pub fn eberr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="Ethernet Bus Error"]
  #[inline] pub fn set_eberr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="MII Interrupt."]
  #[inline] pub fn mii(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="MII Interrupt."]
  #[inline] pub fn set_mii(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Receive Buffer Interrupt"]
  #[inline] pub fn rxb(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Receive Buffer Interrupt"]
  #[inline] pub fn set_rxb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Receive Frame Interrupt"]
  #[inline] pub fn rxf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Receive Frame Interrupt"]
  #[inline] pub fn set_rxf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Transmit Buffer Interrupt"]
  #[inline] pub fn txb(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="Transmit Buffer Interrupt"]
  #[inline] pub fn set_txb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Transmit Frame Interrupt"]
  #[inline] pub fn txf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="Transmit Frame Interrupt"]
  #[inline] pub fn set_txf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Graceful Stop Complete"]
  #[inline] pub fn gra(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Graceful Stop Complete"]
  #[inline] pub fn set_gra(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Babbling Transmit Error"]
  #[inline] pub fn babt(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="Babbling Transmit Error"]
  #[inline] pub fn set_babt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Babbling Receive Error"]
  #[inline] pub fn babr(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Babbling Receive Error"]
  #[inline] pub fn set_babr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
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
#[derive(PartialEq, Eq)]
pub struct Eimr(pub u32);
impl Eimr {
#[doc="TS_TIMER Interrupt Mask"]
  #[inline] pub fn ts_timer(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="TS_TIMER Interrupt Mask"]
  #[inline] pub fn set_ts_timer(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="TS_AVAIL Interrupt Mask"]
  #[inline] pub fn ts_avail(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="TS_AVAIL Interrupt Mask"]
  #[inline] pub fn set_ts_avail(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="WAKEUP Interrupt Mask"]
  #[inline] pub fn wakeup(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="WAKEUP Interrupt Mask"]
  #[inline] pub fn set_wakeup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="PLR Interrupt Mask"]
  #[inline] pub fn plr(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="PLR Interrupt Mask"]
  #[inline] pub fn set_plr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="UN Interrupt Mask"]
  #[inline] pub fn un(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="UN Interrupt Mask"]
  #[inline] pub fn set_un(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="RL Interrupt Mask"]
  #[inline] pub fn rl(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="RL Interrupt Mask"]
  #[inline] pub fn set_rl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="LC Interrupt Mask"]
  #[inline] pub fn lc(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="LC Interrupt Mask"]
  #[inline] pub fn set_lc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="EBERR Interrupt Mask"]
  #[inline] pub fn eberr(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="EBERR Interrupt Mask"]
  #[inline] pub fn set_eberr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="MII Interrupt Mask"]
  #[inline] pub fn mii(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="MII Interrupt Mask"]
  #[inline] pub fn set_mii(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="RXB Interrupt Mask"]
  #[inline] pub fn rxb(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="RXB Interrupt Mask"]
  #[inline] pub fn set_rxb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="RXF Interrupt Mask"]
  #[inline] pub fn rxf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="RXF Interrupt Mask"]
  #[inline] pub fn set_rxf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="TXB Interrupt Mask"]
  #[inline] pub fn txb(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="TXB Interrupt Mask"]
  #[inline] pub fn set_txb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="TXF Interrupt Mask"]
  #[inline] pub fn txf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="TXF Interrupt Mask"]
  #[inline] pub fn set_txf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="GRA Interrupt Mask"]
  #[inline] pub fn gra(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="GRA Interrupt Mask"]
  #[inline] pub fn set_gra(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="BABT Interrupt Mask"]
  #[inline] pub fn babt(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="BABT Interrupt Mask"]
  #[inline] pub fn set_babt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="BABR Interrupt Mask"]
  #[inline] pub fn babr(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="BABR Interrupt Mask"]
  #[inline] pub fn set_babr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rdar(pub u32);
impl Rdar {
#[doc="Receive Descriptor Active"]
  #[inline] pub fn rdar(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Receive Descriptor Active"]
  #[inline] pub fn set_rdar(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tdar(pub u32);
impl Tdar {
#[doc="Transmit Descriptor Active"]
  #[inline] pub fn tdar(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Transmit Descriptor Active"]
  #[inline] pub fn set_tdar(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ecr(pub u32);
impl Ecr {
#[doc="Ethernet MAC Reset"]
  #[inline] pub fn _reset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Ethernet MAC Reset"]
  #[inline] pub fn set_reset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Ethernet Enable"]
  #[inline] pub fn etheren(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Ethernet Enable"]
  #[inline] pub fn set_etheren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Magic Packet Detection Enable"]
  #[inline] pub fn magicen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Magic Packet Detection Enable"]
  #[inline] pub fn set_magicen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Sleep Mode Enable"]
  #[inline] pub fn sleep(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Sleep Mode Enable"]
  #[inline] pub fn set_sleep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="EN1588 Enable"]
  #[inline] pub fn en1588(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="EN1588 Enable"]
  #[inline] pub fn set_en1588(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Debug Enable"]
  #[inline] pub fn dbgen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Debug Enable"]
  #[inline] pub fn set_dbgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="STOPEN Signal Control"]
  #[inline] pub fn stopen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="STOPEN Signal Control"]
  #[inline] pub fn set_stopen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Descriptor Byte Swapping Enable"]
  #[inline] pub fn dbswp(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Descriptor Byte Swapping Enable"]
  #[inline] pub fn set_dbswp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Mmfr(pub u32);
impl Mmfr {
#[doc="Management Frame Data"]
  #[inline] pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Management Frame Data"]
  #[inline] pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Turn Around"]
  #[inline] pub fn ta(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
#[doc="Turn Around"]
  #[inline] pub fn set_ta(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Register Address"]
  #[inline] pub fn ra(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1f // [22:18]
  }
#[doc="Register Address"]
  #[inline] pub fn set_ra(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 18);
     self.0 |= value << 18;
     self
  }

#[doc="PHY Address"]
  #[inline] pub fn pa(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1f // [27:23]
  }
#[doc="PHY Address"]
  #[inline] pub fn set_pa(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Operation Code"]
  #[inline] pub fn op(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
#[doc="Operation Code"]
  #[inline] pub fn set_op(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Start Of Frame Delimiter"]
  #[inline] pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
#[doc="Start Of Frame Delimiter"]
  #[inline] pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
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
#[derive(PartialEq, Eq)]
pub struct Mscr(pub u32);
impl Mscr {
#[doc="MII Speed"]
  #[inline] pub fn mii_speed(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3f // [6:1]
  }
#[doc="MII Speed"]
  #[inline] pub fn set_mii_speed(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Disable Preamble"]
  #[inline] pub fn dis_pre(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Disable Preamble"]
  #[inline] pub fn set_dis_pre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Hold time On MDIO Output"]
  #[inline] pub fn holdtime(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="Hold time On MDIO Output"]
  #[inline] pub fn set_holdtime(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Mibc(pub u32);
impl Mibc {
#[doc="MIB Clear"]
  #[inline] pub fn mib_clear(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="MIB Clear"]
  #[inline] pub fn set_mib_clear(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="MIB Idle"]
  #[inline] pub fn mib_idle(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="MIB Idle"]
  #[inline] pub fn set_mib_idle(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Disable MIB Logic"]
  #[inline] pub fn mib_dis(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Disable MIB Logic"]
  #[inline] pub fn set_mib_dis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
#[doc="Internal Loopback"]
  #[inline] pub fn _loop(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Internal Loopback"]
  #[inline] pub fn set_loop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Disable Receive On Transmit"]
  #[inline] pub fn drt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Disable Receive On Transmit"]
  #[inline] pub fn set_drt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Media Independent Interface Mode"]
  #[inline] pub fn mii_mode(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Media Independent Interface Mode"]
  #[inline] pub fn set_mii_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Promiscuous Mode"]
  #[inline] pub fn prom(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Promiscuous Mode"]
  #[inline] pub fn set_prom(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Broadcast Frame Reject"]
  #[inline] pub fn bc_rej(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Broadcast Frame Reject"]
  #[inline] pub fn set_bc_rej(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Flow Control Enable"]
  #[inline] pub fn fce(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Flow Control Enable"]
  #[inline] pub fn set_fce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="RMII Mode Enable"]
  #[inline] pub fn rmii_mode(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="RMII Mode Enable"]
  #[inline] pub fn set_rmii_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Enables 10-Mbps mode of the RMII ."]
  #[inline] pub fn rmii_10t(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Enables 10-Mbps mode of the RMII ."]
  #[inline] pub fn set_rmii_10t(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Enable Frame Padding Remove On Receive"]
  #[inline] pub fn paden(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Enable Frame Padding Remove On Receive"]
  #[inline] pub fn set_paden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Terminate/Forward Pause Frames"]
  #[inline] pub fn paufwd(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Terminate/Forward Pause Frames"]
  #[inline] pub fn set_paufwd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Terminate/Forward Received CRC"]
  #[inline] pub fn crcfwd(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Terminate/Forward Received CRC"]
  #[inline] pub fn set_crcfwd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="MAC Control Frame Enable"]
  #[inline] pub fn cfen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="MAC Control Frame Enable"]
  #[inline] pub fn set_cfen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Maximum Frame Length"]
  #[inline] pub fn max_fl(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3fff // [29:16]
  }
#[doc="Maximum Frame Length"]
  #[inline] pub fn set_max_fl(mut self, value: u32) -> Self {
     assert!((value & !0x3fff) == 0);
     self.0 &= !(0x3fff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Payload Length Check Disable"]
  #[inline] pub fn nlc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Payload Length Check Disable"]
  #[inline] pub fn set_nlc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Graceful Receive Stopped"]
  #[inline] pub fn grs(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Graceful Receive Stopped"]
  #[inline] pub fn set_grs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
#[doc="Graceful Transmit Stop"]
  #[inline] pub fn gts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Graceful Transmit Stop"]
  #[inline] pub fn set_gts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Full-Duplex Enable"]
  #[inline] pub fn fden(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Full-Duplex Enable"]
  #[inline] pub fn set_fden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Transmit Frame Control Pause"]
  #[inline] pub fn tfc_pause(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Transmit Frame Control Pause"]
  #[inline] pub fn set_tfc_pause(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Receive Frame Control Pause"]
  #[inline] pub fn rfc_pause(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Receive Frame Control Pause"]
  #[inline] pub fn set_rfc_pause(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Source MAC Address Select On Transmit"]
  #[inline] pub fn addsel(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7 // [7:5]
  }
#[doc="Source MAC Address Select On Transmit"]
  #[inline] pub fn set_addsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Set MAC Address On Transmit"]
  #[inline] pub fn addins(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Set MAC Address On Transmit"]
  #[inline] pub fn set_addins(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Forward Frame From Application With CRC"]
  #[inline] pub fn crcfwd(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Forward Frame From Application With CRC"]
  #[inline] pub fn set_crcfwd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
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
#[derive(PartialEq, Eq)]
pub struct Palr(pub u32);
impl Palr {
#[doc="Pause Address"]
  #[inline] pub fn paddr1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Pause Address"]
  #[inline] pub fn set_paddr1(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Paur(pub u32);
impl Paur {
#[doc="Type Field In PAUSE Frames"]
  #[inline] pub fn _type(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Type Field In PAUSE Frames"]
  #[inline] pub fn set_type(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
  #[inline] pub fn paddr2(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
#[doc="Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
  #[inline] pub fn set_paddr2(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
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
#[derive(PartialEq, Eq)]
pub struct Opd(pub u32);
impl Opd {
#[doc="Pause Duration"]
  #[inline] pub fn pause_dur(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Pause Duration"]
  #[inline] pub fn set_pause_dur(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Opcode Field In PAUSE Frames"]
  #[inline] pub fn opcode(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
#[doc="Opcode Field In PAUSE Frames"]
  #[inline] pub fn set_opcode(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
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
#[derive(PartialEq, Eq)]
pub struct Iaur(pub u32);
impl Iaur {
#[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
  #[inline] pub fn iaddr1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
  #[inline] pub fn set_iaddr1(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ialr(pub u32);
impl Ialr {
#[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
  #[inline] pub fn iaddr2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
  #[inline] pub fn set_iaddr2(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Gaur(pub u32);
impl Gaur {
#[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
  #[inline] pub fn gaddr1(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
  #[inline] pub fn set_gaddr1(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Galr(pub u32);
impl Galr {
#[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
  #[inline] pub fn gaddr2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
  #[inline] pub fn set_gaddr2(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tfwr(pub u32);
impl Tfwr {
#[doc="Transmit FIFO Write"]
  #[inline] pub fn tfwr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3f // [5:0]
  }
#[doc="Transmit FIFO Write"]
  #[inline] pub fn set_tfwr(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Store And Forward Enable"]
  #[inline] pub fn strfwd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Store And Forward Enable"]
  #[inline] pub fn set_strfwd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rdsr(pub u32);
impl Rdsr {
#[doc="Pointer to the beginning of the receive buffer descriptor queue."]
  #[inline] pub fn r_des_start(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1fffffff // [31:3]
  }
#[doc="Pointer to the beginning of the receive buffer descriptor queue."]
  #[inline] pub fn set_r_des_start(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tdsr(pub u32);
impl Tdsr {
#[doc="Pointer to the beginning of the transmit buffer descriptor queue."]
  #[inline] pub fn x_des_start(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1fffffff // [31:3]
  }
#[doc="Pointer to the beginning of the transmit buffer descriptor queue."]
  #[inline] pub fn set_x_des_start(mut self, value: u32) -> Self {
     assert!((value & !0x1fffffff) == 0);
     self.0 &= !(0x1fffffff << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Mrbr(pub u32);
impl Mrbr {
#[doc="Receive buffer size in bytes."]
  #[inline] pub fn r_buf_size(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3ff // [13:4]
  }
#[doc="Receive buffer size in bytes."]
  #[inline] pub fn set_r_buf_size(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 4);
     self.0 |= value << 4;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rsfl(pub u32);
impl Rsfl {
#[doc="Value Of Receive FIFO Section Full Threshold"]
  #[inline] pub fn rx_section_full(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Value Of Receive FIFO Section Full Threshold"]
  #[inline] pub fn set_rx_section_full(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rsem(pub u32);
impl Rsem {
#[doc="Value Of The Receive FIFO Section Empty Threshold"]
  #[inline] pub fn rx_section_empty(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Value Of The Receive FIFO Section Empty Threshold"]
  #[inline] pub fn set_rx_section_empty(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="RX Status FIFO Section Empty Threshold"]
  #[inline] pub fn stat_section_empty(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1f // [20:16]
  }
#[doc="RX Status FIFO Section Empty Threshold"]
  #[inline] pub fn set_stat_section_empty(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
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
#[derive(PartialEq, Eq)]
pub struct Raem(pub u32);
impl Raem {
#[doc="Value Of The Receive FIFO Almost Empty Threshold"]
  #[inline] pub fn rx_almost_empty(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Value Of The Receive FIFO Almost Empty Threshold"]
  #[inline] pub fn set_rx_almost_empty(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Rafl(pub u32);
impl Rafl {
#[doc="Value Of The Receive FIFO Almost Full Threshold"]
  #[inline] pub fn rx_almost_full(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Value Of The Receive FIFO Almost Full Threshold"]
  #[inline] pub fn set_rx_almost_full(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tsem(pub u32);
impl Tsem {
#[doc="Value Of The Transmit FIFO Section Empty Threshold"]
  #[inline] pub fn tx_section_empty(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Value Of The Transmit FIFO Section Empty Threshold"]
  #[inline] pub fn set_tx_section_empty(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Taem(pub u32);
impl Taem {
#[doc="Value of Transmit FIFO Almost Empty Threshold"]
  #[inline] pub fn tx_almost_empty(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Value of Transmit FIFO Almost Empty Threshold"]
  #[inline] pub fn set_tx_almost_empty(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tafl(pub u32);
impl Tafl {
#[doc="Value Of The Transmit FIFO Almost Full Threshold"]
  #[inline] pub fn tx_almost_full(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Value Of The Transmit FIFO Almost Full Threshold"]
  #[inline] pub fn set_tx_almost_full(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tipg(pub u32);
impl Tipg {
#[doc="Transmit Inter-Packet Gap"]
  #[inline] pub fn ipg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
#[doc="Transmit Inter-Packet Gap"]
  #[inline] pub fn set_ipg(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ftrl(pub u32);
impl Ftrl {
#[doc="Frame Truncation Length"]
  #[inline] pub fn trunc_fl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3fff // [13:0]
  }
#[doc="Frame Truncation Length"]
  #[inline] pub fn set_trunc_fl(mut self, value: u32) -> Self {
     assert!((value & !0x3fff) == 0);
     self.0 &= !(0x3fff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tacc(pub u32);
impl Tacc {
#[doc="TX FIFO Shift-16"]
  #[inline] pub fn shift16(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="TX FIFO Shift-16"]
  #[inline] pub fn set_shift16(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enables insertion of IP header checksum."]
  #[inline] pub fn ipchk(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Enables insertion of IP header checksum."]
  #[inline] pub fn set_ipchk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Enables insertion of protocol checksum."]
  #[inline] pub fn prochk(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Enables insertion of protocol checksum."]
  #[inline] pub fn set_prochk(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
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
#[derive(PartialEq, Eq)]
pub struct Racc(pub u32);
impl Racc {
#[doc="Enable Padding Removal For Short IP Frames"]
  #[inline] pub fn padrem(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Enable Padding Removal For Short IP Frames"]
  #[inline] pub fn set_padrem(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
  #[inline] pub fn ipdis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
  #[inline] pub fn set_ipdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Enable Discard Of Frames With Wrong Protocol Checksum"]
  #[inline] pub fn prodis(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Enable Discard Of Frames With Wrong Protocol Checksum"]
  #[inline] pub fn set_prodis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Enable Discard Of Frames With MAC Layer Errors"]
  #[inline] pub fn linedis(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Enable Discard Of Frames With MAC Layer Errors"]
  #[inline] pub fn set_linedis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="RX FIFO Shift-16"]
  #[inline] pub fn shift16(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="RX FIFO Shift-16"]
  #[inline] pub fn set_shift16(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTPackets(pub u32);
impl RmonTPackets {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTBcPkt(pub u32);
impl RmonTBcPkt {
#[doc="Broadcast packets"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Broadcast packets"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTMcPkt(pub u32);
impl RmonTMcPkt {
#[doc="Multicast packets"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Multicast packets"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTCrcAlign(pub u32);
impl RmonTCrcAlign {
#[doc="Packets with CRC/align error"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packets with CRC/align error"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTUndersize(pub u32);
impl RmonTUndersize {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTOversize(pub u32);
impl RmonTOversize {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTFrag(pub u32);
impl RmonTFrag {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTJab(pub u32);
impl RmonTJab {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTCol(pub u32);
impl RmonTCol {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTP64(pub u32);
impl RmonTP64 {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTP65to127(pub u32);
impl RmonTP65to127 {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTP128to255(pub u32);
impl RmonTP128to255 {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTP256to511(pub u32);
impl RmonTP256to511 {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTP512to1023(pub u32);
impl RmonTP512to1023 {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTP1024to2047(pub u32);
impl RmonTP1024to2047 {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTPGte2048(pub u32);
impl RmonTPGte2048 {
#[doc="Packet count"]
  #[inline] pub fn txpkts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_txpkts(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonTOctets(pub u32);
impl RmonTOctets {
#[doc="Octet count"]
  #[inline] pub fn txocts(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Octet count"]
  #[inline] pub fn set_txocts(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTFrameOk(pub u32);
impl IeeeTFrameOk {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeT1col(pub u32);
impl IeeeT1col {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTMcol(pub u32);
impl IeeeTMcol {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTDef(pub u32);
impl IeeeTDef {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTLcol(pub u32);
impl IeeeTLcol {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTExcol(pub u32);
impl IeeeTExcol {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTMacerr(pub u32);
impl IeeeTMacerr {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTCserr(pub u32);
impl IeeeTCserr {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTFdxfc(pub u32);
impl IeeeTFdxfc {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeTOctetsOk(pub u32);
impl IeeeTOctetsOk {
#[doc="Octet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Octet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRPackets(pub u32);
impl RmonRPackets {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRBcPkt(pub u32);
impl RmonRBcPkt {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRMcPkt(pub u32);
impl RmonRMcPkt {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRCrcAlign(pub u32);
impl RmonRCrcAlign {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRUndersize(pub u32);
impl RmonRUndersize {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonROversize(pub u32);
impl RmonROversize {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRFrag(pub u32);
impl RmonRFrag {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRJab(pub u32);
impl RmonRJab {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRP64(pub u32);
impl RmonRP64 {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRP65to127(pub u32);
impl RmonRP65to127 {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRP128to255(pub u32);
impl RmonRP128to255 {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRP256to511(pub u32);
impl RmonRP256to511 {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRP512to1023(pub u32);
impl RmonRP512to1023 {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRP1024to2047(pub u32);
impl RmonRP1024to2047 {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonRPGte2048(pub u32);
impl RmonRPGte2048 {
#[doc="Packet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Packet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct RmonROctets(pub u32);
impl RmonROctets {
#[doc="Octet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Octet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeRDrop(pub u32);
impl IeeeRDrop {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeRFrameOk(pub u32);
impl IeeeRFrameOk {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeRCrc(pub u32);
impl IeeeRCrc {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeRAlign(pub u32);
impl IeeeRAlign {
#[doc="Frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeRMacerr(pub u32);
impl IeeeRMacerr {
#[doc="Count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeRFdxfc(pub u32);
impl IeeeRFdxfc {
#[doc="Pause frame count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Pause frame count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct IeeeROctetsOk(pub u32);
impl IeeeROctetsOk {
#[doc="Octet count"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Octet count"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Atcr(pub u32);
impl Atcr {
#[doc="Enable Timer"]
  #[inline] pub fn en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Enable Timer"]
  #[inline] pub fn set_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable One-Shot Offset Event"]
  #[inline] pub fn offen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Enable One-Shot Offset Event"]
  #[inline] pub fn set_offen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Reset Timer On Offset Event"]
  #[inline] pub fn offrst(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Reset Timer On Offset Event"]
  #[inline] pub fn set_offrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Enable Periodical Event"]
  #[inline] pub fn peren(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Enable Periodical Event"]
  #[inline] pub fn set_peren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Enables event signal output assertion on period event"]
  #[inline] pub fn pinper(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Enables event signal output assertion on period event"]
  #[inline] pub fn set_pinper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Reset Timer"]
  #[inline] pub fn restart(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Reset Timer"]
  #[inline] pub fn set_restart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Capture Timer Value"]
  #[inline] pub fn capture(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Capture Timer Value"]
  #[inline] pub fn set_capture(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Enable Timer Slave Mode"]
  #[inline] pub fn slave(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Enable Timer Slave Mode"]
  #[inline] pub fn set_slave(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
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
#[derive(PartialEq, Eq)]
pub struct Atvr(pub u32);
impl Atvr {
#[doc="A write sets the timer"]
  #[inline] pub fn atime(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="A write sets the timer"]
  #[inline] pub fn set_atime(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Atoff(pub u32);
impl Atoff {
#[doc="Offset value for one-shot event generation"]
  #[inline] pub fn offset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Offset value for one-shot event generation"]
  #[inline] pub fn set_offset(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Atper(pub u32);
impl Atper {
#[doc="Value for generating periodic events"]
  #[inline] pub fn period(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Value for generating periodic events"]
  #[inline] pub fn set_period(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Atcor(pub u32);
impl Atcor {
#[doc="Correction Counter Wrap-Around Value"]
  #[inline] pub fn cor(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
#[doc="Correction Counter Wrap-Around Value"]
  #[inline] pub fn set_cor(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Atinc(pub u32);
impl Atinc {
#[doc="Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
  #[inline] pub fn inc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
#[doc="Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
  #[inline] pub fn set_inc(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Correction Increment Value"]
  #[inline] pub fn inc_corr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7f // [14:8]
  }
#[doc="Correction Increment Value"]
  #[inline] pub fn set_inc_corr(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 8);
     self.0 |= value << 8;
     self
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
#[derive(PartialEq, Eq)]
pub struct Atstmp(pub u32);
impl Atstmp {
#[doc="Timestamp of the last frame transmitted by the core that had TxBD[TS] set"]
  #[inline] pub fn timestamp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Timestamp of the last frame transmitted by the core that had TxBD[TS] set"]
  #[inline] pub fn set_timestamp(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tgsr(pub u32);
impl Tgsr {
#[doc="Copy Of Timer Flag For Channel 0"]
  #[inline] pub fn tf0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Copy Of Timer Flag For Channel 0"]
  #[inline] pub fn set_tf0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Copy Of Timer Flag For Channel 1"]
  #[inline] pub fn tf1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Copy Of Timer Flag For Channel 1"]
  #[inline] pub fn set_tf1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Copy Of Timer Flag For Channel 2"]
  #[inline] pub fn tf2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Copy Of Timer Flag For Channel 2"]
  #[inline] pub fn set_tf2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Copy Of Timer Flag For Channel 3"]
  #[inline] pub fn tf3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Copy Of Timer Flag For Channel 3"]
  #[inline] pub fn set_tf3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tcsr(pub u32);
impl Tcsr {
#[doc="Timer DMA Request Enable"]
  #[inline] pub fn tdre(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Timer DMA Request Enable"]
  #[inline] pub fn set_tdre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Timer Mode"]
  #[inline] pub fn tmode(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0xf // [5:2]
  }
#[doc="Timer Mode"]
  #[inline] pub fn set_tmode(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Timer Interrupt Enable"]
  #[inline] pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Timer Interrupt Enable"]
  #[inline] pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Timer Flag"]
  #[inline] pub fn tf(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Timer Flag"]
  #[inline] pub fn set_tf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Tccr(pub u32);
impl Tccr {
#[doc="Timer Capture Compare"]
  #[inline] pub fn tcc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Timer Capture Compare"]
  #[inline] pub fn set_tcc(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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

