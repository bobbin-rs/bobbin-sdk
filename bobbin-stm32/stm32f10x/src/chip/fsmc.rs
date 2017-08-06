//! Flexible static memory controller
#[allow(unused_imports)] use bobbin_common::bits;
pub const FSMC: Fsmc = Fsmc(0xa0000000);

#[doc="Flexible static memory controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fsmc(pub u32);
impl Fsmc {
#[doc="Get the *const pointer for the BCR1 register."]
  #[inline] pub fn bcr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the BCR1 register."]
  #[inline] pub fn bcr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the BCR1 register."]
  #[inline] pub fn bcr1(&self) -> Bcr1 { 
     unsafe {
        Bcr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the BCR1 register."]
  #[inline] pub fn set_bcr1(&self, value: Bcr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BCR1 register."]
  #[inline] pub fn with_bcr1<F: FnOnce(Bcr1) -> Bcr1>(&self, f: F) -> &Self {
     let tmp = self.bcr1();
     self.set_bcr1(f(tmp))
  }

#[doc="Get the *const pointer for the BTR1 register."]
  #[inline] pub fn btr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the BTR1 register."]
  #[inline] pub fn btr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the BTR1 register."]
  #[inline] pub fn btr1(&self) -> Btr1 { 
     unsafe {
        Btr1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the BTR1 register."]
  #[inline] pub fn set_btr1(&self, value: Btr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BTR1 register."]
  #[inline] pub fn with_btr1<F: FnOnce(Btr1) -> Btr1>(&self, f: F) -> &Self {
     let tmp = self.btr1();
     self.set_btr1(f(tmp))
  }

#[doc="Get the *const pointer for the BCR2 register."]
  #[inline] pub fn bcr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the BCR2 register."]
  #[inline] pub fn bcr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the BCR2 register."]
  #[inline] pub fn bcr2(&self) -> Bcr2 { 
     unsafe {
        Bcr2(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the BCR2 register."]
  #[inline] pub fn set_bcr2(&self, value: Bcr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BCR2 register."]
  #[inline] pub fn with_bcr2<F: FnOnce(Bcr2) -> Bcr2>(&self, f: F) -> &Self {
     let tmp = self.bcr2();
     self.set_bcr2(f(tmp))
  }

#[doc="Get the *const pointer for the BTR2 register."]
  #[inline] pub fn btr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the BTR2 register."]
  #[inline] pub fn btr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the BTR2 register."]
  #[inline] pub fn btr2(&self) -> Btr2 { 
     unsafe {
        Btr2(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the BTR2 register."]
  #[inline] pub fn set_btr2(&self, value: Btr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BTR2 register."]
  #[inline] pub fn with_btr2<F: FnOnce(Btr2) -> Btr2>(&self, f: F) -> &Self {
     let tmp = self.btr2();
     self.set_btr2(f(tmp))
  }

#[doc="Get the *const pointer for the BCR3 register."]
  #[inline] pub fn bcr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the BCR3 register."]
  #[inline] pub fn bcr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the BCR3 register."]
  #[inline] pub fn bcr3(&self) -> Bcr3 { 
     unsafe {
        Bcr3(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the BCR3 register."]
  #[inline] pub fn set_bcr3(&self, value: Bcr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BCR3 register."]
  #[inline] pub fn with_bcr3<F: FnOnce(Bcr3) -> Bcr3>(&self, f: F) -> &Self {
     let tmp = self.bcr3();
     self.set_bcr3(f(tmp))
  }

#[doc="Get the *const pointer for the BTR3 register."]
  #[inline] pub fn btr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the BTR3 register."]
  #[inline] pub fn btr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the BTR3 register."]
  #[inline] pub fn btr3(&self) -> Btr3 { 
     unsafe {
        Btr3(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the BTR3 register."]
  #[inline] pub fn set_btr3(&self, value: Btr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BTR3 register."]
  #[inline] pub fn with_btr3<F: FnOnce(Btr3) -> Btr3>(&self, f: F) -> &Self {
     let tmp = self.btr3();
     self.set_btr3(f(tmp))
  }

#[doc="Get the *const pointer for the BCR4 register."]
  #[inline] pub fn bcr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the BCR4 register."]
  #[inline] pub fn bcr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the BCR4 register."]
  #[inline] pub fn bcr4(&self) -> Bcr4 { 
     unsafe {
        Bcr4(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the BCR4 register."]
  #[inline] pub fn set_bcr4(&self, value: Bcr4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BCR4 register."]
  #[inline] pub fn with_bcr4<F: FnOnce(Bcr4) -> Bcr4>(&self, f: F) -> &Self {
     let tmp = self.bcr4();
     self.set_bcr4(f(tmp))
  }

#[doc="Get the *const pointer for the BTR4 register."]
  #[inline] pub fn btr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the BTR4 register."]
  #[inline] pub fn btr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the BTR4 register."]
  #[inline] pub fn btr4(&self) -> Btr4 { 
     unsafe {
        Btr4(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the BTR4 register."]
  #[inline] pub fn set_btr4(&self, value: Btr4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BTR4 register."]
  #[inline] pub fn with_btr4<F: FnOnce(Btr4) -> Btr4>(&self, f: F) -> &Self {
     let tmp = self.btr4();
     self.set_btr4(f(tmp))
  }

#[doc="Get the *const pointer for the PCR2 register."]
  #[inline] pub fn pcr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
#[doc="Get the *mut pointer for the PCR2 register."]
  #[inline] pub fn pcr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
#[doc="Read the PCR2 register."]
  #[inline] pub fn pcr2(&self) -> Pcr2 { 
     unsafe {
        Pcr2(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
#[doc="Write the PCR2 register."]
  #[inline] pub fn set_pcr2(&self, value: Pcr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCR2 register."]
  #[inline] pub fn with_pcr2<F: FnOnce(Pcr2) -> Pcr2>(&self, f: F) -> &Self {
     let tmp = self.pcr2();
     self.set_pcr2(f(tmp))
  }

#[doc="Get the *const pointer for the SR2 register."]
  #[inline] pub fn sr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x64) as *const u32
  }
#[doc="Get the *mut pointer for the SR2 register."]
  #[inline] pub fn sr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x64) as *mut u32
  }
#[doc="Read the SR2 register."]
  #[inline] pub fn sr2(&self) -> Sr2 { 
     unsafe {
        Sr2(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
     }
  }
#[doc="Write the SR2 register."]
  #[inline] pub fn set_sr2(&self, value: Sr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR2 register."]
  #[inline] pub fn with_sr2<F: FnOnce(Sr2) -> Sr2>(&self, f: F) -> &Self {
     let tmp = self.sr2();
     self.set_sr2(f(tmp))
  }

#[doc="Get the *const pointer for the PMEM2 register."]
  #[inline] pub fn pmem2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x68) as *const u32
  }
#[doc="Get the *mut pointer for the PMEM2 register."]
  #[inline] pub fn pmem2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x68) as *mut u32
  }
#[doc="Read the PMEM2 register."]
  #[inline] pub fn pmem2(&self) -> Pmem2 { 
     unsafe {
        Pmem2(::core::ptr::read_volatile(((self.0 as usize) + 0x68) as *const u32))
     }
  }
#[doc="Write the PMEM2 register."]
  #[inline] pub fn set_pmem2(&self, value: Pmem2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x68) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PMEM2 register."]
  #[inline] pub fn with_pmem2<F: FnOnce(Pmem2) -> Pmem2>(&self, f: F) -> &Self {
     let tmp = self.pmem2();
     self.set_pmem2(f(tmp))
  }

#[doc="Get the *const pointer for the PATT2 register."]
  #[inline] pub fn patt2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x6c) as *const u32
  }
#[doc="Get the *mut pointer for the PATT2 register."]
  #[inline] pub fn patt2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x6c) as *mut u32
  }
#[doc="Read the PATT2 register."]
  #[inline] pub fn patt2(&self) -> Patt2 { 
     unsafe {
        Patt2(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
     }
  }
#[doc="Write the PATT2 register."]
  #[inline] pub fn set_patt2(&self, value: Patt2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PATT2 register."]
  #[inline] pub fn with_patt2<F: FnOnce(Patt2) -> Patt2>(&self, f: F) -> &Self {
     let tmp = self.patt2();
     self.set_patt2(f(tmp))
  }

#[doc="Get the *const pointer for the ECCR2 register."]
  #[inline] pub fn eccr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x74) as *const u32
  }
#[doc="Get the *mut pointer for the ECCR2 register."]
  #[inline] pub fn eccr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x74) as *mut u32
  }
#[doc="Read the ECCR2 register."]
  #[inline] pub fn eccr2(&self) -> Eccr2 { 
     unsafe {
        Eccr2(::core::ptr::read_volatile(((self.0 as usize) + 0x74) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PCR3 register."]
  #[inline] pub fn pcr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x80) as *const u32
  }
#[doc="Get the *mut pointer for the PCR3 register."]
  #[inline] pub fn pcr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x80) as *mut u32
  }
#[doc="Read the PCR3 register."]
  #[inline] pub fn pcr3(&self) -> Pcr3 { 
     unsafe {
        Pcr3(::core::ptr::read_volatile(((self.0 as usize) + 0x80) as *const u32))
     }
  }
#[doc="Write the PCR3 register."]
  #[inline] pub fn set_pcr3(&self, value: Pcr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x80) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCR3 register."]
  #[inline] pub fn with_pcr3<F: FnOnce(Pcr3) -> Pcr3>(&self, f: F) -> &Self {
     let tmp = self.pcr3();
     self.set_pcr3(f(tmp))
  }

#[doc="Get the *const pointer for the SR3 register."]
  #[inline] pub fn sr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x84) as *const u32
  }
#[doc="Get the *mut pointer for the SR3 register."]
  #[inline] pub fn sr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x84) as *mut u32
  }
#[doc="Read the SR3 register."]
  #[inline] pub fn sr3(&self) -> Sr3 { 
     unsafe {
        Sr3(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
     }
  }
#[doc="Write the SR3 register."]
  #[inline] pub fn set_sr3(&self, value: Sr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR3 register."]
  #[inline] pub fn with_sr3<F: FnOnce(Sr3) -> Sr3>(&self, f: F) -> &Self {
     let tmp = self.sr3();
     self.set_sr3(f(tmp))
  }

#[doc="Get the *const pointer for the PMEM3 register."]
  #[inline] pub fn pmem3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x88) as *const u32
  }
#[doc="Get the *mut pointer for the PMEM3 register."]
  #[inline] pub fn pmem3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x88) as *mut u32
  }
#[doc="Read the PMEM3 register."]
  #[inline] pub fn pmem3(&self) -> Pmem3 { 
     unsafe {
        Pmem3(::core::ptr::read_volatile(((self.0 as usize) + 0x88) as *const u32))
     }
  }
#[doc="Write the PMEM3 register."]
  #[inline] pub fn set_pmem3(&self, value: Pmem3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x88) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PMEM3 register."]
  #[inline] pub fn with_pmem3<F: FnOnce(Pmem3) -> Pmem3>(&self, f: F) -> &Self {
     let tmp = self.pmem3();
     self.set_pmem3(f(tmp))
  }

#[doc="Get the *const pointer for the PATT3 register."]
  #[inline] pub fn patt3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8c) as *const u32
  }
#[doc="Get the *mut pointer for the PATT3 register."]
  #[inline] pub fn patt3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8c) as *mut u32
  }
#[doc="Read the PATT3 register."]
  #[inline] pub fn patt3(&self) -> Patt3 { 
     unsafe {
        Patt3(::core::ptr::read_volatile(((self.0 as usize) + 0x8c) as *const u32))
     }
  }
#[doc="Write the PATT3 register."]
  #[inline] pub fn set_patt3(&self, value: Patt3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PATT3 register."]
  #[inline] pub fn with_patt3<F: FnOnce(Patt3) -> Patt3>(&self, f: F) -> &Self {
     let tmp = self.patt3();
     self.set_patt3(f(tmp))
  }

#[doc="Get the *const pointer for the ECCR3 register."]
  #[inline] pub fn eccr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x94) as *const u32
  }
#[doc="Get the *mut pointer for the ECCR3 register."]
  #[inline] pub fn eccr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x94) as *mut u32
  }
#[doc="Read the ECCR3 register."]
  #[inline] pub fn eccr3(&self) -> Eccr3 { 
     unsafe {
        Eccr3(::core::ptr::read_volatile(((self.0 as usize) + 0x94) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PCR4 register."]
  #[inline] pub fn pcr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa0) as *const u32
  }
#[doc="Get the *mut pointer for the PCR4 register."]
  #[inline] pub fn pcr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa0) as *mut u32
  }
#[doc="Read the PCR4 register."]
  #[inline] pub fn pcr4(&self) -> Pcr4 { 
     unsafe {
        Pcr4(::core::ptr::read_volatile(((self.0 as usize) + 0xa0) as *const u32))
     }
  }
#[doc="Write the PCR4 register."]
  #[inline] pub fn set_pcr4(&self, value: Pcr4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCR4 register."]
  #[inline] pub fn with_pcr4<F: FnOnce(Pcr4) -> Pcr4>(&self, f: F) -> &Self {
     let tmp = self.pcr4();
     self.set_pcr4(f(tmp))
  }

#[doc="Get the *const pointer for the SR4 register."]
  #[inline] pub fn sr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa4) as *const u32
  }
#[doc="Get the *mut pointer for the SR4 register."]
  #[inline] pub fn sr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa4) as *mut u32
  }
#[doc="Read the SR4 register."]
  #[inline] pub fn sr4(&self) -> Sr4 { 
     unsafe {
        Sr4(::core::ptr::read_volatile(((self.0 as usize) + 0xa4) as *const u32))
     }
  }
#[doc="Write the SR4 register."]
  #[inline] pub fn set_sr4(&self, value: Sr4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR4 register."]
  #[inline] pub fn with_sr4<F: FnOnce(Sr4) -> Sr4>(&self, f: F) -> &Self {
     let tmp = self.sr4();
     self.set_sr4(f(tmp))
  }

#[doc="Get the *const pointer for the PMEM4 register."]
  #[inline] pub fn pmem4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa8) as *const u32
  }
#[doc="Get the *mut pointer for the PMEM4 register."]
  #[inline] pub fn pmem4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa8) as *mut u32
  }
#[doc="Read the PMEM4 register."]
  #[inline] pub fn pmem4(&self) -> Pmem4 { 
     unsafe {
        Pmem4(::core::ptr::read_volatile(((self.0 as usize) + 0xa8) as *const u32))
     }
  }
#[doc="Write the PMEM4 register."]
  #[inline] pub fn set_pmem4(&self, value: Pmem4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PMEM4 register."]
  #[inline] pub fn with_pmem4<F: FnOnce(Pmem4) -> Pmem4>(&self, f: F) -> &Self {
     let tmp = self.pmem4();
     self.set_pmem4(f(tmp))
  }

#[doc="Get the *const pointer for the PATT4 register."]
  #[inline] pub fn patt4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xac) as *const u32
  }
#[doc="Get the *mut pointer for the PATT4 register."]
  #[inline] pub fn patt4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xac) as *mut u32
  }
#[doc="Read the PATT4 register."]
  #[inline] pub fn patt4(&self) -> Patt4 { 
     unsafe {
        Patt4(::core::ptr::read_volatile(((self.0 as usize) + 0xac) as *const u32))
     }
  }
#[doc="Write the PATT4 register."]
  #[inline] pub fn set_patt4(&self, value: Patt4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xac) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PATT4 register."]
  #[inline] pub fn with_patt4<F: FnOnce(Patt4) -> Patt4>(&self, f: F) -> &Self {
     let tmp = self.patt4();
     self.set_patt4(f(tmp))
  }

#[doc="Get the *const pointer for the PIO4 register."]
  #[inline] pub fn pio4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb0) as *const u32
  }
#[doc="Get the *mut pointer for the PIO4 register."]
  #[inline] pub fn pio4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb0) as *mut u32
  }
#[doc="Read the PIO4 register."]
  #[inline] pub fn pio4(&self) -> Pio4 { 
     unsafe {
        Pio4(::core::ptr::read_volatile(((self.0 as usize) + 0xb0) as *const u32))
     }
  }
#[doc="Write the PIO4 register."]
  #[inline] pub fn set_pio4(&self, value: Pio4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PIO4 register."]
  #[inline] pub fn with_pio4<F: FnOnce(Pio4) -> Pio4>(&self, f: F) -> &Self {
     let tmp = self.pio4();
     self.set_pio4(f(tmp))
  }

#[doc="Get the *const pointer for the BWTR1 register."]
  #[inline] pub fn bwtr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x104) as *const u32
  }
#[doc="Get the *mut pointer for the BWTR1 register."]
  #[inline] pub fn bwtr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x104) as *mut u32
  }
#[doc="Read the BWTR1 register."]
  #[inline] pub fn bwtr1(&self) -> Bwtr1 { 
     unsafe {
        Bwtr1(::core::ptr::read_volatile(((self.0 as usize) + 0x104) as *const u32))
     }
  }
#[doc="Write the BWTR1 register."]
  #[inline] pub fn set_bwtr1(&self, value: Bwtr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x104) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BWTR1 register."]
  #[inline] pub fn with_bwtr1<F: FnOnce(Bwtr1) -> Bwtr1>(&self, f: F) -> &Self {
     let tmp = self.bwtr1();
     self.set_bwtr1(f(tmp))
  }

#[doc="Get the *const pointer for the BWTR2 register."]
  #[inline] pub fn bwtr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10c) as *const u32
  }
#[doc="Get the *mut pointer for the BWTR2 register."]
  #[inline] pub fn bwtr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10c) as *mut u32
  }
#[doc="Read the BWTR2 register."]
  #[inline] pub fn bwtr2(&self) -> Bwtr2 { 
     unsafe {
        Bwtr2(::core::ptr::read_volatile(((self.0 as usize) + 0x10c) as *const u32))
     }
  }
#[doc="Write the BWTR2 register."]
  #[inline] pub fn set_bwtr2(&self, value: Bwtr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BWTR2 register."]
  #[inline] pub fn with_bwtr2<F: FnOnce(Bwtr2) -> Bwtr2>(&self, f: F) -> &Self {
     let tmp = self.bwtr2();
     self.set_bwtr2(f(tmp))
  }

#[doc="Get the *const pointer for the BWTR3 register."]
  #[inline] pub fn bwtr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x114) as *const u32
  }
#[doc="Get the *mut pointer for the BWTR3 register."]
  #[inline] pub fn bwtr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x114) as *mut u32
  }
#[doc="Read the BWTR3 register."]
  #[inline] pub fn bwtr3(&self) -> Bwtr3 { 
     unsafe {
        Bwtr3(::core::ptr::read_volatile(((self.0 as usize) + 0x114) as *const u32))
     }
  }
#[doc="Write the BWTR3 register."]
  #[inline] pub fn set_bwtr3(&self, value: Bwtr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x114) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BWTR3 register."]
  #[inline] pub fn with_bwtr3<F: FnOnce(Bwtr3) -> Bwtr3>(&self, f: F) -> &Self {
     let tmp = self.bwtr3();
     self.set_bwtr3(f(tmp))
  }

#[doc="Get the *const pointer for the BWTR4 register."]
  #[inline] pub fn bwtr4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x11c) as *const u32
  }
#[doc="Get the *mut pointer for the BWTR4 register."]
  #[inline] pub fn bwtr4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x11c) as *mut u32
  }
#[doc="Read the BWTR4 register."]
  #[inline] pub fn bwtr4(&self) -> Bwtr4 { 
     unsafe {
        Bwtr4(::core::ptr::read_volatile(((self.0 as usize) + 0x11c) as *const u32))
     }
  }
#[doc="Write the BWTR4 register."]
  #[inline] pub fn set_bwtr4(&self, value: Bwtr4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x11c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BWTR4 register."]
  #[inline] pub fn with_bwtr4<F: FnOnce(Bwtr4) -> Bwtr4>(&self, f: F) -> &Self {
     let tmp = self.bwtr4();
     self.set_bwtr4(f(tmp))
  }

}

#[doc="SRAM/NOR-Flash chip-select control register 1"]
#[derive(PartialEq, Eq)]
pub struct Bcr1(pub u32);
impl Bcr1 {
#[doc="CBURSTRW"]
  #[inline] pub fn cburstrw(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="CBURSTRW"]
  #[inline] pub fn set_cburstrw<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="ASYNCWAIT"]
  #[inline] pub fn asyncwait(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="ASYNCWAIT"]
  #[inline] pub fn set_asyncwait<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="EXTMOD"]
  #[inline] pub fn extmod(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="EXTMOD"]
  #[inline] pub fn set_extmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="WAITEN"]
  #[inline] pub fn waiten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="WAITEN"]
  #[inline] pub fn set_waiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="WREN"]
  #[inline] pub fn wren(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="WREN"]
  #[inline] pub fn set_wren<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="WAITCFG"]
  #[inline] pub fn waitcfg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="WAITCFG"]
  #[inline] pub fn set_waitcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="WAITPOL"]
  #[inline] pub fn waitpol(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="WAITPOL"]
  #[inline] pub fn set_waitpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="BURSTEN"]
  #[inline] pub fn bursten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="BURSTEN"]
  #[inline] pub fn set_bursten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="FACCEN"]
  #[inline] pub fn faccen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="FACCEN"]
  #[inline] pub fn set_faccen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="MWID"]
  #[inline] pub fn mwid(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="MWID"]
  #[inline] pub fn set_mwid<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="MTYP"]
  #[inline] pub fn mtyp(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
  }
#[doc="MTYP"]
  #[inline] pub fn set_mtyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="MUXEN"]
  #[inline] pub fn muxen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="MUXEN"]
  #[inline] pub fn set_muxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="MBKEN"]
  #[inline] pub fn mbken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="MBKEN"]
  #[inline] pub fn set_mbken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bcr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bcr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cburstrw() != 0 { try!(write!(f, " cburstrw"))}
      if self.asyncwait() != 0 { try!(write!(f, " asyncwait"))}
      if self.extmod() != 0 { try!(write!(f, " extmod"))}
      if self.waiten() != 0 { try!(write!(f, " waiten"))}
      if self.wren() != 0 { try!(write!(f, " wren"))}
      if self.waitcfg() != 0 { try!(write!(f, " waitcfg"))}
      if self.waitpol() != 0 { try!(write!(f, " waitpol"))}
      if self.bursten() != 0 { try!(write!(f, " bursten"))}
      if self.faccen() != 0 { try!(write!(f, " faccen"))}
      if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
      if self.mtyp() != 0 { try!(write!(f, " mtyp=0x{:x}", self.mtyp()))}
      if self.muxen() != 0 { try!(write!(f, " muxen"))}
      if self.mbken() != 0 { try!(write!(f, " mbken"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash chip-select timing register 1"]
#[derive(PartialEq, Eq)]
pub struct Btr1(pub u32);
impl Btr1 {
#[doc="ACCMOD"]
  #[inline] pub fn accmod(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="ACCMOD"]
  #[inline] pub fn set_accmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DATLAT"]
  #[inline] pub fn datlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="DATLAT"]
  #[inline] pub fn set_datlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="CLKDIV"]
  #[inline] pub fn clkdiv(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="CLKDIV"]
  #[inline] pub fn set_clkdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="BUSTURN"]
  #[inline] pub fn busturn(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="BUSTURN"]
  #[inline] pub fn set_busturn<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="DATAST"]
  #[inline] pub fn datast(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="DATAST"]
  #[inline] pub fn set_datast<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADDHLD"]
  #[inline] pub fn addhld(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="ADDHLD"]
  #[inline] pub fn set_addhld<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADDSET"]
  #[inline] pub fn addset(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADDSET"]
  #[inline] pub fn set_addset<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Btr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Btr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
      if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
      if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
      if self.busturn() != 0 { try!(write!(f, " busturn=0x{:x}", self.busturn()))}
      if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
      if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
      if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash chip-select control register 2"]
#[derive(PartialEq, Eq)]
pub struct Bcr2(pub u32);
impl Bcr2 {
#[doc="CBURSTRW"]
  #[inline] pub fn cburstrw(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="CBURSTRW"]
  #[inline] pub fn set_cburstrw<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="ASYNCWAIT"]
  #[inline] pub fn asyncwait(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="ASYNCWAIT"]
  #[inline] pub fn set_asyncwait<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="EXTMOD"]
  #[inline] pub fn extmod(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="EXTMOD"]
  #[inline] pub fn set_extmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="WAITEN"]
  #[inline] pub fn waiten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="WAITEN"]
  #[inline] pub fn set_waiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="WREN"]
  #[inline] pub fn wren(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="WREN"]
  #[inline] pub fn set_wren<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="WAITCFG"]
  #[inline] pub fn waitcfg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="WAITCFG"]
  #[inline] pub fn set_waitcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="WRAPMOD"]
  #[inline] pub fn wrapmod(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="WRAPMOD"]
  #[inline] pub fn set_wrapmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="WAITPOL"]
  #[inline] pub fn waitpol(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="WAITPOL"]
  #[inline] pub fn set_waitpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="BURSTEN"]
  #[inline] pub fn bursten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="BURSTEN"]
  #[inline] pub fn set_bursten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="FACCEN"]
  #[inline] pub fn faccen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="FACCEN"]
  #[inline] pub fn set_faccen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="MWID"]
  #[inline] pub fn mwid(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="MWID"]
  #[inline] pub fn set_mwid<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="MTYP"]
  #[inline] pub fn mtyp(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
  }
#[doc="MTYP"]
  #[inline] pub fn set_mtyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="MUXEN"]
  #[inline] pub fn muxen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="MUXEN"]
  #[inline] pub fn set_muxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="MBKEN"]
  #[inline] pub fn mbken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="MBKEN"]
  #[inline] pub fn set_mbken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bcr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bcr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cburstrw() != 0 { try!(write!(f, " cburstrw"))}
      if self.asyncwait() != 0 { try!(write!(f, " asyncwait"))}
      if self.extmod() != 0 { try!(write!(f, " extmod"))}
      if self.waiten() != 0 { try!(write!(f, " waiten"))}
      if self.wren() != 0 { try!(write!(f, " wren"))}
      if self.waitcfg() != 0 { try!(write!(f, " waitcfg"))}
      if self.wrapmod() != 0 { try!(write!(f, " wrapmod"))}
      if self.waitpol() != 0 { try!(write!(f, " waitpol"))}
      if self.bursten() != 0 { try!(write!(f, " bursten"))}
      if self.faccen() != 0 { try!(write!(f, " faccen"))}
      if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
      if self.mtyp() != 0 { try!(write!(f, " mtyp=0x{:x}", self.mtyp()))}
      if self.muxen() != 0 { try!(write!(f, " muxen"))}
      if self.mbken() != 0 { try!(write!(f, " mbken"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash chip-select timing register 2"]
#[derive(PartialEq, Eq)]
pub struct Btr2(pub u32);
impl Btr2 {
#[doc="ACCMOD"]
  #[inline] pub fn accmod(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="ACCMOD"]
  #[inline] pub fn set_accmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DATLAT"]
  #[inline] pub fn datlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="DATLAT"]
  #[inline] pub fn set_datlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="CLKDIV"]
  #[inline] pub fn clkdiv(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="CLKDIV"]
  #[inline] pub fn set_clkdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="BUSTURN"]
  #[inline] pub fn busturn(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="BUSTURN"]
  #[inline] pub fn set_busturn<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="DATAST"]
  #[inline] pub fn datast(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="DATAST"]
  #[inline] pub fn set_datast<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADDHLD"]
  #[inline] pub fn addhld(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="ADDHLD"]
  #[inline] pub fn set_addhld<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADDSET"]
  #[inline] pub fn addset(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADDSET"]
  #[inline] pub fn set_addset<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Btr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Btr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
      if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
      if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
      if self.busturn() != 0 { try!(write!(f, " busturn=0x{:x}", self.busturn()))}
      if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
      if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
      if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash chip-select control register 3"]
#[derive(PartialEq, Eq)]
pub struct Bcr3(pub u32);
impl Bcr3 {
#[doc="CBURSTRW"]
  #[inline] pub fn cburstrw(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="CBURSTRW"]
  #[inline] pub fn set_cburstrw<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="ASYNCWAIT"]
  #[inline] pub fn asyncwait(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="ASYNCWAIT"]
  #[inline] pub fn set_asyncwait<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="EXTMOD"]
  #[inline] pub fn extmod(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="EXTMOD"]
  #[inline] pub fn set_extmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="WAITEN"]
  #[inline] pub fn waiten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="WAITEN"]
  #[inline] pub fn set_waiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="WREN"]
  #[inline] pub fn wren(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="WREN"]
  #[inline] pub fn set_wren<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="WAITCFG"]
  #[inline] pub fn waitcfg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="WAITCFG"]
  #[inline] pub fn set_waitcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="WRAPMOD"]
  #[inline] pub fn wrapmod(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="WRAPMOD"]
  #[inline] pub fn set_wrapmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="WAITPOL"]
  #[inline] pub fn waitpol(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="WAITPOL"]
  #[inline] pub fn set_waitpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="BURSTEN"]
  #[inline] pub fn bursten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="BURSTEN"]
  #[inline] pub fn set_bursten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="FACCEN"]
  #[inline] pub fn faccen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="FACCEN"]
  #[inline] pub fn set_faccen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="MWID"]
  #[inline] pub fn mwid(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="MWID"]
  #[inline] pub fn set_mwid<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="MTYP"]
  #[inline] pub fn mtyp(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
  }
#[doc="MTYP"]
  #[inline] pub fn set_mtyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="MUXEN"]
  #[inline] pub fn muxen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="MUXEN"]
  #[inline] pub fn set_muxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="MBKEN"]
  #[inline] pub fn mbken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="MBKEN"]
  #[inline] pub fn set_mbken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bcr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bcr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cburstrw() != 0 { try!(write!(f, " cburstrw"))}
      if self.asyncwait() != 0 { try!(write!(f, " asyncwait"))}
      if self.extmod() != 0 { try!(write!(f, " extmod"))}
      if self.waiten() != 0 { try!(write!(f, " waiten"))}
      if self.wren() != 0 { try!(write!(f, " wren"))}
      if self.waitcfg() != 0 { try!(write!(f, " waitcfg"))}
      if self.wrapmod() != 0 { try!(write!(f, " wrapmod"))}
      if self.waitpol() != 0 { try!(write!(f, " waitpol"))}
      if self.bursten() != 0 { try!(write!(f, " bursten"))}
      if self.faccen() != 0 { try!(write!(f, " faccen"))}
      if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
      if self.mtyp() != 0 { try!(write!(f, " mtyp=0x{:x}", self.mtyp()))}
      if self.muxen() != 0 { try!(write!(f, " muxen"))}
      if self.mbken() != 0 { try!(write!(f, " mbken"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash chip-select timing register 3"]
#[derive(PartialEq, Eq)]
pub struct Btr3(pub u32);
impl Btr3 {
#[doc="ACCMOD"]
  #[inline] pub fn accmod(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="ACCMOD"]
  #[inline] pub fn set_accmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DATLAT"]
  #[inline] pub fn datlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="DATLAT"]
  #[inline] pub fn set_datlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="CLKDIV"]
  #[inline] pub fn clkdiv(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="CLKDIV"]
  #[inline] pub fn set_clkdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="BUSTURN"]
  #[inline] pub fn busturn(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="BUSTURN"]
  #[inline] pub fn set_busturn<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="DATAST"]
  #[inline] pub fn datast(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="DATAST"]
  #[inline] pub fn set_datast<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADDHLD"]
  #[inline] pub fn addhld(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="ADDHLD"]
  #[inline] pub fn set_addhld<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADDSET"]
  #[inline] pub fn addset(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADDSET"]
  #[inline] pub fn set_addset<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Btr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Btr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
      if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
      if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
      if self.busturn() != 0 { try!(write!(f, " busturn=0x{:x}", self.busturn()))}
      if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
      if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
      if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash chip-select control register 4"]
#[derive(PartialEq, Eq)]
pub struct Bcr4(pub u32);
impl Bcr4 {
#[doc="CBURSTRW"]
  #[inline] pub fn cburstrw(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="CBURSTRW"]
  #[inline] pub fn set_cburstrw<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="ASYNCWAIT"]
  #[inline] pub fn asyncwait(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="ASYNCWAIT"]
  #[inline] pub fn set_asyncwait<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="EXTMOD"]
  #[inline] pub fn extmod(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="EXTMOD"]
  #[inline] pub fn set_extmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="WAITEN"]
  #[inline] pub fn waiten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="WAITEN"]
  #[inline] pub fn set_waiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="WREN"]
  #[inline] pub fn wren(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="WREN"]
  #[inline] pub fn set_wren<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="WAITCFG"]
  #[inline] pub fn waitcfg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="WAITCFG"]
  #[inline] pub fn set_waitcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="WRAPMOD"]
  #[inline] pub fn wrapmod(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="WRAPMOD"]
  #[inline] pub fn set_wrapmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="WAITPOL"]
  #[inline] pub fn waitpol(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="WAITPOL"]
  #[inline] pub fn set_waitpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="BURSTEN"]
  #[inline] pub fn bursten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="BURSTEN"]
  #[inline] pub fn set_bursten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="FACCEN"]
  #[inline] pub fn faccen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="FACCEN"]
  #[inline] pub fn set_faccen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="MWID"]
  #[inline] pub fn mwid(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="MWID"]
  #[inline] pub fn set_mwid<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="MTYP"]
  #[inline] pub fn mtyp(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
  }
#[doc="MTYP"]
  #[inline] pub fn set_mtyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="MUXEN"]
  #[inline] pub fn muxen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="MUXEN"]
  #[inline] pub fn set_muxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="MBKEN"]
  #[inline] pub fn mbken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="MBKEN"]
  #[inline] pub fn set_mbken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bcr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bcr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cburstrw() != 0 { try!(write!(f, " cburstrw"))}
      if self.asyncwait() != 0 { try!(write!(f, " asyncwait"))}
      if self.extmod() != 0 { try!(write!(f, " extmod"))}
      if self.waiten() != 0 { try!(write!(f, " waiten"))}
      if self.wren() != 0 { try!(write!(f, " wren"))}
      if self.waitcfg() != 0 { try!(write!(f, " waitcfg"))}
      if self.wrapmod() != 0 { try!(write!(f, " wrapmod"))}
      if self.waitpol() != 0 { try!(write!(f, " waitpol"))}
      if self.bursten() != 0 { try!(write!(f, " bursten"))}
      if self.faccen() != 0 { try!(write!(f, " faccen"))}
      if self.mwid() != 0 { try!(write!(f, " mwid=0x{:x}", self.mwid()))}
      if self.mtyp() != 0 { try!(write!(f, " mtyp=0x{:x}", self.mtyp()))}
      if self.muxen() != 0 { try!(write!(f, " muxen"))}
      if self.mbken() != 0 { try!(write!(f, " mbken"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash chip-select timing register 4"]
#[derive(PartialEq, Eq)]
pub struct Btr4(pub u32);
impl Btr4 {
#[doc="ACCMOD"]
  #[inline] pub fn accmod(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="ACCMOD"]
  #[inline] pub fn set_accmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DATLAT"]
  #[inline] pub fn datlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="DATLAT"]
  #[inline] pub fn set_datlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="CLKDIV"]
  #[inline] pub fn clkdiv(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="CLKDIV"]
  #[inline] pub fn set_clkdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="BUSTURN"]
  #[inline] pub fn busturn(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="BUSTURN"]
  #[inline] pub fn set_busturn<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="DATAST"]
  #[inline] pub fn datast(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="DATAST"]
  #[inline] pub fn set_datast<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADDHLD"]
  #[inline] pub fn addhld(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="ADDHLD"]
  #[inline] pub fn set_addhld<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADDSET"]
  #[inline] pub fn addset(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADDSET"]
  #[inline] pub fn set_addset<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Btr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Btr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
      if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
      if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
      if self.busturn() != 0 { try!(write!(f, " busturn=0x{:x}", self.busturn()))}
      if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
      if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
      if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PC Card/NAND Flash control register 2"]
#[derive(PartialEq, Eq)]
pub struct Pcr2(pub u32);
impl Pcr2 {
#[doc="ECCPS"]
  #[inline] pub fn eccps(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
  }
#[doc="ECCPS"]
  #[inline] pub fn set_eccps<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="TAR"]
  #[inline] pub fn tar(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
  }
#[doc="TAR"]
  #[inline] pub fn set_tar<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 13);
     self.0 |= value << 13;
     self
  }

#[doc="TCLR"]
  #[inline] pub fn tclr(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
  }
#[doc="TCLR"]
  #[inline] pub fn set_tclr<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 9);
     self.0 |= value << 9;
     self
  }

#[doc="ECCEN"]
  #[inline] pub fn eccen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="ECCEN"]
  #[inline] pub fn set_eccen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="PWID"]
  #[inline] pub fn pwid(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="PWID"]
  #[inline] pub fn set_pwid<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="PTYP"]
  #[inline] pub fn ptyp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="PTYP"]
  #[inline] pub fn set_ptyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="PBKEN"]
  #[inline] pub fn pbken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="PBKEN"]
  #[inline] pub fn set_pbken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="PWAITEN"]
  #[inline] pub fn pwaiten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="PWAITEN"]
  #[inline] pub fn set_pwaiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Pcr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.eccps() != 0 { try!(write!(f, " eccps=0x{:x}", self.eccps()))}
      if self.tar() != 0 { try!(write!(f, " tar=0x{:x}", self.tar()))}
      if self.tclr() != 0 { try!(write!(f, " tclr=0x{:x}", self.tclr()))}
      if self.eccen() != 0 { try!(write!(f, " eccen"))}
      if self.pwid() != 0 { try!(write!(f, " pwid=0x{:x}", self.pwid()))}
      if self.ptyp() != 0 { try!(write!(f, " ptyp"))}
      if self.pbken() != 0 { try!(write!(f, " pbken"))}
      if self.pwaiten() != 0 { try!(write!(f, " pwaiten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FIFO status and interrupt register 2"]
#[derive(PartialEq, Eq)]
pub struct Sr2(pub u32);
impl Sr2 {
#[doc="FEMPT"]
  #[inline] pub fn fempt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="FEMPT"]
  #[inline] pub fn set_fempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="IFEN"]
  #[inline] pub fn ifen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="IFEN"]
  #[inline] pub fn set_ifen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="ILEN"]
  #[inline] pub fn ilen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="ILEN"]
  #[inline] pub fn set_ilen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="IREN"]
  #[inline] pub fn iren(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="IREN"]
  #[inline] pub fn set_iren<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="IFS"]
  #[inline] pub fn ifs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="IFS"]
  #[inline] pub fn set_ifs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="ILS"]
  #[inline] pub fn ils(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="ILS"]
  #[inline] pub fn set_ils<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="IRS"]
  #[inline] pub fn irs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="IRS"]
  #[inline] pub fn set_irs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
      if self.fempt() != 0 { try!(write!(f, " fempt"))}
      if self.ifen() != 0 { try!(write!(f, " ifen"))}
      if self.ilen() != 0 { try!(write!(f, " ilen"))}
      if self.iren() != 0 { try!(write!(f, " iren"))}
      if self.ifs() != 0 { try!(write!(f, " ifs"))}
      if self.ils() != 0 { try!(write!(f, " ils"))}
      if self.irs() != 0 { try!(write!(f, " irs"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Common memory space timing register 2"]
#[derive(PartialEq, Eq)]
pub struct Pmem2(pub u32);
impl Pmem2 {
#[doc="MEMHIZx"]
  #[inline] pub fn memhizx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
  }
#[doc="MEMHIZx"]
  #[inline] pub fn set_memhizx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="MEMHOLDx"]
  #[inline] pub fn memholdx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="MEMHOLDx"]
  #[inline] pub fn set_memholdx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="MEMWAITx"]
  #[inline] pub fn memwaitx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="MEMWAITx"]
  #[inline] pub fn set_memwaitx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="MEMSETx"]
  #[inline] pub fn memsetx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="MEMSETx"]
  #[inline] pub fn set_memsetx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pmem2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pmem2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.memhizx() != 0 { try!(write!(f, " memhizx=0x{:x}", self.memhizx()))}
      if self.memholdx() != 0 { try!(write!(f, " memholdx=0x{:x}", self.memholdx()))}
      if self.memwaitx() != 0 { try!(write!(f, " memwaitx=0x{:x}", self.memwaitx()))}
      if self.memsetx() != 0 { try!(write!(f, " memsetx=0x{:x}", self.memsetx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Attribute memory space timing register 2"]
#[derive(PartialEq, Eq)]
pub struct Patt2(pub u32);
impl Patt2 {
#[doc="Attribute memory x databus HiZ time"]
  #[inline] pub fn atthizx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
  }
#[doc="Attribute memory x databus HiZ time"]
  #[inline] pub fn set_atthizx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Attribute memory x hold time"]
  #[inline] pub fn attholdx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="Attribute memory x hold time"]
  #[inline] pub fn set_attholdx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Attribute memory x wait time"]
  #[inline] pub fn attwaitx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="Attribute memory x wait time"]
  #[inline] pub fn set_attwaitx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Attribute memory x setup time"]
  #[inline] pub fn attsetx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Attribute memory x setup time"]
  #[inline] pub fn set_attsetx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Patt2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Patt2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.atthizx() != 0 { try!(write!(f, " atthizx=0x{:x}", self.atthizx()))}
      if self.attholdx() != 0 { try!(write!(f, " attholdx=0x{:x}", self.attholdx()))}
      if self.attwaitx() != 0 { try!(write!(f, " attwaitx=0x{:x}", self.attwaitx()))}
      if self.attsetx() != 0 { try!(write!(f, " attsetx=0x{:x}", self.attsetx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ECC result register 2"]
#[derive(PartialEq, Eq)]
pub struct Eccr2(pub u32);
impl Eccr2 {
#[doc="ECC result"]
  #[inline] pub fn eccx(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="ECC result"]
  #[inline] pub fn set_eccx<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Eccr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Eccr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PC Card/NAND Flash control register 3"]
#[derive(PartialEq, Eq)]
pub struct Pcr3(pub u32);
impl Pcr3 {
#[doc="ECCPS"]
  #[inline] pub fn eccps(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
  }
#[doc="ECCPS"]
  #[inline] pub fn set_eccps<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="TAR"]
  #[inline] pub fn tar(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
  }
#[doc="TAR"]
  #[inline] pub fn set_tar<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 13);
     self.0 |= value << 13;
     self
  }

#[doc="TCLR"]
  #[inline] pub fn tclr(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
  }
#[doc="TCLR"]
  #[inline] pub fn set_tclr<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 9);
     self.0 |= value << 9;
     self
  }

#[doc="ECCEN"]
  #[inline] pub fn eccen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="ECCEN"]
  #[inline] pub fn set_eccen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="PWID"]
  #[inline] pub fn pwid(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="PWID"]
  #[inline] pub fn set_pwid<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="PTYP"]
  #[inline] pub fn ptyp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="PTYP"]
  #[inline] pub fn set_ptyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="PBKEN"]
  #[inline] pub fn pbken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="PBKEN"]
  #[inline] pub fn set_pbken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="PWAITEN"]
  #[inline] pub fn pwaiten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="PWAITEN"]
  #[inline] pub fn set_pwaiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Pcr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.eccps() != 0 { try!(write!(f, " eccps=0x{:x}", self.eccps()))}
      if self.tar() != 0 { try!(write!(f, " tar=0x{:x}", self.tar()))}
      if self.tclr() != 0 { try!(write!(f, " tclr=0x{:x}", self.tclr()))}
      if self.eccen() != 0 { try!(write!(f, " eccen"))}
      if self.pwid() != 0 { try!(write!(f, " pwid=0x{:x}", self.pwid()))}
      if self.ptyp() != 0 { try!(write!(f, " ptyp"))}
      if self.pbken() != 0 { try!(write!(f, " pbken"))}
      if self.pwaiten() != 0 { try!(write!(f, " pwaiten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FIFO status and interrupt register 3"]
#[derive(PartialEq, Eq)]
pub struct Sr3(pub u32);
impl Sr3 {
#[doc="FEMPT"]
  #[inline] pub fn fempt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="FEMPT"]
  #[inline] pub fn set_fempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="IFEN"]
  #[inline] pub fn ifen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="IFEN"]
  #[inline] pub fn set_ifen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="ILEN"]
  #[inline] pub fn ilen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="ILEN"]
  #[inline] pub fn set_ilen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="IREN"]
  #[inline] pub fn iren(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="IREN"]
  #[inline] pub fn set_iren<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="IFS"]
  #[inline] pub fn ifs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="IFS"]
  #[inline] pub fn set_ifs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="ILS"]
  #[inline] pub fn ils(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="ILS"]
  #[inline] pub fn set_ils<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="IRS"]
  #[inline] pub fn irs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="IRS"]
  #[inline] pub fn set_irs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fempt() != 0 { try!(write!(f, " fempt"))}
      if self.ifen() != 0 { try!(write!(f, " ifen"))}
      if self.ilen() != 0 { try!(write!(f, " ilen"))}
      if self.iren() != 0 { try!(write!(f, " iren"))}
      if self.ifs() != 0 { try!(write!(f, " ifs"))}
      if self.ils() != 0 { try!(write!(f, " ils"))}
      if self.irs() != 0 { try!(write!(f, " irs"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Common memory space timing register 3"]
#[derive(PartialEq, Eq)]
pub struct Pmem3(pub u32);
impl Pmem3 {
#[doc="MEMHIZx"]
  #[inline] pub fn memhizx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
  }
#[doc="MEMHIZx"]
  #[inline] pub fn set_memhizx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="MEMHOLDx"]
  #[inline] pub fn memholdx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="MEMHOLDx"]
  #[inline] pub fn set_memholdx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="MEMWAITx"]
  #[inline] pub fn memwaitx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="MEMWAITx"]
  #[inline] pub fn set_memwaitx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="MEMSETx"]
  #[inline] pub fn memsetx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="MEMSETx"]
  #[inline] pub fn set_memsetx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pmem3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pmem3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.memhizx() != 0 { try!(write!(f, " memhizx=0x{:x}", self.memhizx()))}
      if self.memholdx() != 0 { try!(write!(f, " memholdx=0x{:x}", self.memholdx()))}
      if self.memwaitx() != 0 { try!(write!(f, " memwaitx=0x{:x}", self.memwaitx()))}
      if self.memsetx() != 0 { try!(write!(f, " memsetx=0x{:x}", self.memsetx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Attribute memory space timing register 3"]
#[derive(PartialEq, Eq)]
pub struct Patt3(pub u32);
impl Patt3 {
#[doc="ATTHIZx"]
  #[inline] pub fn atthizx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
  }
#[doc="ATTHIZx"]
  #[inline] pub fn set_atthizx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="ATTHOLDx"]
  #[inline] pub fn attholdx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="ATTHOLDx"]
  #[inline] pub fn set_attholdx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="ATTWAITx"]
  #[inline] pub fn attwaitx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="ATTWAITx"]
  #[inline] pub fn set_attwaitx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ATTSETx"]
  #[inline] pub fn attsetx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="ATTSETx"]
  #[inline] pub fn set_attsetx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Patt3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Patt3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.atthizx() != 0 { try!(write!(f, " atthizx=0x{:x}", self.atthizx()))}
      if self.attholdx() != 0 { try!(write!(f, " attholdx=0x{:x}", self.attholdx()))}
      if self.attwaitx() != 0 { try!(write!(f, " attwaitx=0x{:x}", self.attwaitx()))}
      if self.attsetx() != 0 { try!(write!(f, " attsetx=0x{:x}", self.attsetx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ECC result register 3"]
#[derive(PartialEq, Eq)]
pub struct Eccr3(pub u32);
impl Eccr3 {
#[doc="ECCx"]
  #[inline] pub fn eccx(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="ECCx"]
  #[inline] pub fn set_eccx<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Eccr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Eccr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PC Card/NAND Flash control register 4"]
#[derive(PartialEq, Eq)]
pub struct Pcr4(pub u32);
impl Pcr4 {
#[doc="ECCPS"]
  #[inline] pub fn eccps(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
  }
#[doc="ECCPS"]
  #[inline] pub fn set_eccps<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="TAR"]
  #[inline] pub fn tar(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
  }
#[doc="TAR"]
  #[inline] pub fn set_tar<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 13);
     self.0 |= value << 13;
     self
  }

#[doc="TCLR"]
  #[inline] pub fn tclr(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0xf) as u8) } // [12:9]
  }
#[doc="TCLR"]
  #[inline] pub fn set_tclr<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 9);
     self.0 |= value << 9;
     self
  }

#[doc="ECCEN"]
  #[inline] pub fn eccen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="ECCEN"]
  #[inline] pub fn set_eccen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="PWID"]
  #[inline] pub fn pwid(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="PWID"]
  #[inline] pub fn set_pwid<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="PTYP"]
  #[inline] pub fn ptyp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="PTYP"]
  #[inline] pub fn set_ptyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="PBKEN"]
  #[inline] pub fn pbken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="PBKEN"]
  #[inline] pub fn set_pbken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="PWAITEN"]
  #[inline] pub fn pwaiten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="PWAITEN"]
  #[inline] pub fn set_pwaiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Pcr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.eccps() != 0 { try!(write!(f, " eccps=0x{:x}", self.eccps()))}
      if self.tar() != 0 { try!(write!(f, " tar=0x{:x}", self.tar()))}
      if self.tclr() != 0 { try!(write!(f, " tclr=0x{:x}", self.tclr()))}
      if self.eccen() != 0 { try!(write!(f, " eccen"))}
      if self.pwid() != 0 { try!(write!(f, " pwid=0x{:x}", self.pwid()))}
      if self.ptyp() != 0 { try!(write!(f, " ptyp"))}
      if self.pbken() != 0 { try!(write!(f, " pbken"))}
      if self.pwaiten() != 0 { try!(write!(f, " pwaiten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FIFO status and interrupt register 4"]
#[derive(PartialEq, Eq)]
pub struct Sr4(pub u32);
impl Sr4 {
#[doc="FEMPT"]
  #[inline] pub fn fempt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="FEMPT"]
  #[inline] pub fn set_fempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="IFEN"]
  #[inline] pub fn ifen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="IFEN"]
  #[inline] pub fn set_ifen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="ILEN"]
  #[inline] pub fn ilen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="ILEN"]
  #[inline] pub fn set_ilen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="IREN"]
  #[inline] pub fn iren(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="IREN"]
  #[inline] pub fn set_iren<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="IFS"]
  #[inline] pub fn ifs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="IFS"]
  #[inline] pub fn set_ifs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="ILS"]
  #[inline] pub fn ils(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="ILS"]
  #[inline] pub fn set_ils<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="IRS"]
  #[inline] pub fn irs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="IRS"]
  #[inline] pub fn set_irs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fempt() != 0 { try!(write!(f, " fempt"))}
      if self.ifen() != 0 { try!(write!(f, " ifen"))}
      if self.ilen() != 0 { try!(write!(f, " ilen"))}
      if self.iren() != 0 { try!(write!(f, " iren"))}
      if self.ifs() != 0 { try!(write!(f, " ifs"))}
      if self.ils() != 0 { try!(write!(f, " ils"))}
      if self.irs() != 0 { try!(write!(f, " irs"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Common memory space timing register 4"]
#[derive(PartialEq, Eq)]
pub struct Pmem4(pub u32);
impl Pmem4 {
#[doc="MEMHIZx"]
  #[inline] pub fn memhizx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
  }
#[doc="MEMHIZx"]
  #[inline] pub fn set_memhizx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="MEMHOLDx"]
  #[inline] pub fn memholdx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="MEMHOLDx"]
  #[inline] pub fn set_memholdx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="MEMWAITx"]
  #[inline] pub fn memwaitx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="MEMWAITx"]
  #[inline] pub fn set_memwaitx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="MEMSETx"]
  #[inline] pub fn memsetx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="MEMSETx"]
  #[inline] pub fn set_memsetx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pmem4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pmem4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.memhizx() != 0 { try!(write!(f, " memhizx=0x{:x}", self.memhizx()))}
      if self.memholdx() != 0 { try!(write!(f, " memholdx=0x{:x}", self.memholdx()))}
      if self.memwaitx() != 0 { try!(write!(f, " memwaitx=0x{:x}", self.memwaitx()))}
      if self.memsetx() != 0 { try!(write!(f, " memsetx=0x{:x}", self.memsetx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Attribute memory space timing register 4"]
#[derive(PartialEq, Eq)]
pub struct Patt4(pub u32);
impl Patt4 {
#[doc="ATTHIZx"]
  #[inline] pub fn atthizx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
  }
#[doc="ATTHIZx"]
  #[inline] pub fn set_atthizx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="ATTHOLDx"]
  #[inline] pub fn attholdx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="ATTHOLDx"]
  #[inline] pub fn set_attholdx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="ATTWAITx"]
  #[inline] pub fn attwaitx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="ATTWAITx"]
  #[inline] pub fn set_attwaitx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ATTSETx"]
  #[inline] pub fn attsetx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="ATTSETx"]
  #[inline] pub fn set_attsetx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Patt4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Patt4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.atthizx() != 0 { try!(write!(f, " atthizx=0x{:x}", self.atthizx()))}
      if self.attholdx() != 0 { try!(write!(f, " attholdx=0x{:x}", self.attholdx()))}
      if self.attwaitx() != 0 { try!(write!(f, " attwaitx=0x{:x}", self.attwaitx()))}
      if self.attsetx() != 0 { try!(write!(f, " attsetx=0x{:x}", self.attsetx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I/O space timing register 4"]
#[derive(PartialEq, Eq)]
pub struct Pio4(pub u32);
impl Pio4 {
#[doc="IOHIZx"]
  #[inline] pub fn iohizx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
  }
#[doc="IOHIZx"]
  #[inline] pub fn set_iohizx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="IOHOLDx"]
  #[inline] pub fn ioholdx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
  }
#[doc="IOHOLDx"]
  #[inline] pub fn set_ioholdx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="IOWAITx"]
  #[inline] pub fn iowaitx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="IOWAITx"]
  #[inline] pub fn set_iowaitx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="IOSETx"]
  #[inline] pub fn iosetx(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="IOSETx"]
  #[inline] pub fn set_iosetx<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pio4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pio4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iohizx() != 0 { try!(write!(f, " iohizx=0x{:x}", self.iohizx()))}
      if self.ioholdx() != 0 { try!(write!(f, " ioholdx=0x{:x}", self.ioholdx()))}
      if self.iowaitx() != 0 { try!(write!(f, " iowaitx=0x{:x}", self.iowaitx()))}
      if self.iosetx() != 0 { try!(write!(f, " iosetx=0x{:x}", self.iosetx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash write timing registers 1"]
#[derive(PartialEq, Eq)]
pub struct Bwtr1(pub u32);
impl Bwtr1 {
#[doc="ACCMOD"]
  #[inline] pub fn accmod(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="ACCMOD"]
  #[inline] pub fn set_accmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DATLAT"]
  #[inline] pub fn datlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="DATLAT"]
  #[inline] pub fn set_datlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="CLKDIV"]
  #[inline] pub fn clkdiv(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="CLKDIV"]
  #[inline] pub fn set_clkdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="DATAST"]
  #[inline] pub fn datast(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="DATAST"]
  #[inline] pub fn set_datast<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADDHLD"]
  #[inline] pub fn addhld(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="ADDHLD"]
  #[inline] pub fn set_addhld<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADDSET"]
  #[inline] pub fn addset(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADDSET"]
  #[inline] pub fn set_addset<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bwtr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bwtr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
      if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
      if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
      if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
      if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
      if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash write timing registers 2"]
#[derive(PartialEq, Eq)]
pub struct Bwtr2(pub u32);
impl Bwtr2 {
#[doc="ACCMOD"]
  #[inline] pub fn accmod(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="ACCMOD"]
  #[inline] pub fn set_accmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DATLAT"]
  #[inline] pub fn datlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="DATLAT"]
  #[inline] pub fn set_datlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="CLKDIV"]
  #[inline] pub fn clkdiv(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="CLKDIV"]
  #[inline] pub fn set_clkdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="DATAST"]
  #[inline] pub fn datast(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="DATAST"]
  #[inline] pub fn set_datast<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADDHLD"]
  #[inline] pub fn addhld(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="ADDHLD"]
  #[inline] pub fn set_addhld<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADDSET"]
  #[inline] pub fn addset(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADDSET"]
  #[inline] pub fn set_addset<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bwtr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bwtr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
      if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
      if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
      if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
      if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
      if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash write timing registers 3"]
#[derive(PartialEq, Eq)]
pub struct Bwtr3(pub u32);
impl Bwtr3 {
#[doc="ACCMOD"]
  #[inline] pub fn accmod(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="ACCMOD"]
  #[inline] pub fn set_accmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DATLAT"]
  #[inline] pub fn datlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="DATLAT"]
  #[inline] pub fn set_datlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="CLKDIV"]
  #[inline] pub fn clkdiv(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="CLKDIV"]
  #[inline] pub fn set_clkdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="DATAST"]
  #[inline] pub fn datast(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="DATAST"]
  #[inline] pub fn set_datast<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADDHLD"]
  #[inline] pub fn addhld(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="ADDHLD"]
  #[inline] pub fn set_addhld<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADDSET"]
  #[inline] pub fn addset(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADDSET"]
  #[inline] pub fn set_addset<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bwtr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bwtr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
      if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
      if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
      if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
      if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
      if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SRAM/NOR-Flash write timing registers 4"]
#[derive(PartialEq, Eq)]
pub struct Bwtr4(pub u32);
impl Bwtr4 {
#[doc="ACCMOD"]
  #[inline] pub fn accmod(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="ACCMOD"]
  #[inline] pub fn set_accmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="DATLAT"]
  #[inline] pub fn datlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="DATLAT"]
  #[inline] pub fn set_datlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="CLKDIV"]
  #[inline] pub fn clkdiv(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="CLKDIV"]
  #[inline] pub fn set_clkdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="DATAST"]
  #[inline] pub fn datast(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="DATAST"]
  #[inline] pub fn set_datast<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADDHLD"]
  #[inline] pub fn addhld(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="ADDHLD"]
  #[inline] pub fn set_addhld<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADDSET"]
  #[inline] pub fn addset(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="ADDSET"]
  #[inline] pub fn set_addset<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bwtr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bwtr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.accmod() != 0 { try!(write!(f, " accmod=0x{:x}", self.accmod()))}
      if self.datlat() != 0 { try!(write!(f, " datlat=0x{:x}", self.datlat()))}
      if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
      if self.datast() != 0 { try!(write!(f, " datast=0x{:x}", self.datast()))}
      if self.addhld() != 0 { try!(write!(f, " addhld=0x{:x}", self.addhld()))}
      if self.addset() != 0 { try!(write!(f, " addset=0x{:x}", self.addset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

