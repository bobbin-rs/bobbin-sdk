#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TIM_GEN Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the CR1 register."]
  #[inline] pub fn cr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CR1 register."]
  #[inline] pub fn cr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CR1 register."]
  #[inline] pub fn cr1(&self) -> Cr1 { 
     unsafe {
        Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CR1 register."]
  #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
     let value = f(Cr1(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR1 register."]
  #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
     let tmp = self.cr1();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CR2 register."]
  #[inline] pub fn cr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CR2 register."]
  #[inline] pub fn cr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CR2 register."]
  #[inline] pub fn cr2(&self) -> Cr2 { 
     unsafe {
        Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CR2 register."]
  #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
     let value = f(Cr2(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR2 register."]
  #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
     let tmp = self.cr2();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SMCR register."]
  #[inline] pub fn smcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the SMCR register."]
  #[inline] pub fn smcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the SMCR register."]
  #[inline] pub fn smcr(&self) -> Smcr { 
     unsafe {
        Smcr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the SMCR register."]
  #[inline] pub fn set_smcr<F: FnOnce(Smcr) -> Smcr>(&self, f: F) -> &Self {
     let value = f(Smcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SMCR register."]
  #[inline] pub fn with_smcr<F: FnOnce(Smcr) -> Smcr>(&self, f: F) -> &Self {
     let tmp = self.smcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DIER register."]
  #[inline] pub fn dier_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the DIER register."]
  #[inline] pub fn dier_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the DIER register."]
  #[inline] pub fn dier(&self) -> Dier { 
     unsafe {
        Dier(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the DIER register."]
  #[inline] pub fn set_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
     let value = f(Dier(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DIER register."]
  #[inline] pub fn with_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
     let tmp = self.dier();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SR register."]
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the SR register."]
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the SR register."]
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the SR register."]
  #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let value = f(Sr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR register."]
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the EGR register."]
  #[inline] pub fn egr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the EGR register."]
  #[inline] pub fn egr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Write the EGR register."]
  #[inline] pub fn set_egr<F: FnOnce(Egr) -> Egr>(&self, f: F) -> &Self {
     let value = f(Egr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CCMR_OUTPUT register."]
  #[inline] pub fn ccmr_output_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the CCMR_OUTPUT register."]
  #[inline] pub fn ccmr_output_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
  }
#[doc="Read the CCMR_OUTPUT register."]
  #[inline] pub fn ccmr_output<I: Into<bits::R2>>(&self, index: I) -> CcmrOutput { 
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     unsafe {
        CcmrOutput(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the CCMR_OUTPUT register."]
  #[inline] pub fn set_ccmr_output<I: Into<bits::R2>, F: FnOnce(CcmrOutput) -> CcmrOutput>(&self, index: I, f: F) -> &Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value = f(CcmrOutput(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCMR_OUTPUT register."]
  #[inline] pub fn with_ccmr_output<I: Into<bits::R2> + Copy, F: FnOnce(CcmrOutput) -> CcmrOutput>(&self, index: I, f: F) -> &Self {
     let tmp = self.ccmr_output(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CCMR_INPUT register."]
  #[inline] pub fn ccmr_input_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the CCMR_INPUT register."]
  #[inline] pub fn ccmr_input_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
  }
#[doc="Read the CCMR_INPUT register."]
  #[inline] pub fn ccmr_input<I: Into<bits::R2>>(&self, index: I) -> CcmrInput { 
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     unsafe {
        CcmrInput(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the CCMR_INPUT register."]
  #[inline] pub fn set_ccmr_input<I: Into<bits::R2>, F: FnOnce(CcmrInput) -> CcmrInput>(&self, index: I, f: F) -> &Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value = f(CcmrInput(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCMR_INPUT register."]
  #[inline] pub fn with_ccmr_input<I: Into<bits::R2> + Copy, F: FnOnce(CcmrInput) -> CcmrInput>(&self, index: I, f: F) -> &Self {
     let tmp = self.ccmr_input(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CCER register."]
  #[inline] pub fn ccer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the CCER register."]
  #[inline] pub fn ccer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the CCER register."]
  #[inline] pub fn ccer(&self) -> Ccer { 
     unsafe {
        Ccer(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the CCER register."]
  #[inline] pub fn set_ccer<F: FnOnce(Ccer) -> Ccer>(&self, f: F) -> &Self {
     let value = f(Ccer(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCER register."]
  #[inline] pub fn with_ccer<F: FnOnce(Ccer) -> Ccer>(&self, f: F) -> &Self {
     let tmp = self.ccer();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CNT register."]
  #[inline] pub fn cnt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the CNT register."]
  #[inline] pub fn cnt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the CNT register."]
  #[inline] pub fn cnt(&self) -> Cnt { 
     unsafe {
        Cnt(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the CNT register."]
  #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
     let value = f(Cnt(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CNT register."]
  #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
     let tmp = self.cnt();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PSC register."]
  #[inline] pub fn psc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the PSC register."]
  #[inline] pub fn psc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the PSC register."]
  #[inline] pub fn psc(&self) -> Psc { 
     unsafe {
        Psc(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the PSC register."]
  #[inline] pub fn set_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
     let value = f(Psc(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PSC register."]
  #[inline] pub fn with_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
     let tmp = self.psc();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ARR register."]
  #[inline] pub fn arr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the ARR register."]
  #[inline] pub fn arr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Read the ARR register."]
  #[inline] pub fn arr(&self) -> Arr { 
     unsafe {
        Arr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
#[doc="Write the ARR register."]
  #[inline] pub fn set_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
     let value = f(Arr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ARR register."]
  #[inline] pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
     let tmp = self.arr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the RCR register."]
  #[inline] pub fn rcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the RCR register."]
  #[inline] pub fn rcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the RCR register."]
  #[inline] pub fn rcr(&self) -> Rcr { 
     unsafe {
        Rcr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
#[doc="Write the RCR register."]
  #[inline] pub fn set_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
     let value = f(Rcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RCR register."]
  #[inline] pub fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
     let tmp = self.rcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the BDTR register."]
  #[inline] pub fn bdtr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the BDTR register."]
  #[inline] pub fn bdtr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the BDTR register."]
  #[inline] pub fn bdtr(&self) -> Bdtr { 
     unsafe {
        Bdtr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the BDTR register."]
  #[inline] pub fn set_bdtr<F: FnOnce(Bdtr) -> Bdtr>(&self, f: F) -> &Self {
     let value = f(Bdtr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BDTR register."]
  #[inline] pub fn with_bdtr<F: FnOnce(Bdtr) -> Bdtr>(&self, f: F) -> &Self {
     let tmp = self.bdtr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CCR register."]
  #[inline] pub fn ccr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x34 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the CCR register."]
  #[inline] pub fn ccr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x34 + (index << 2)) as *mut u32
  }
#[doc="Read the CCR register."]
  #[inline] pub fn ccr<I: Into<bits::R4>>(&self, index: I) -> Ccr { 
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     unsafe {
        Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x34 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the CCR register."]
  #[inline] pub fn set_ccr<I: Into<bits::R4>, F: FnOnce(Ccr) -> Ccr>(&self, index: I, f: F) -> &Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value = f(Ccr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCR register."]
  #[inline] pub fn with_ccr<I: Into<bits::R4> + Copy, F: FnOnce(Ccr) -> Ccr>(&self, index: I, f: F) -> &Self {
     let tmp = self.ccr(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DCR register."]
  #[inline] pub fn dcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
#[doc="Get the *mut pointer for the DCR register."]
  #[inline] pub fn dcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
#[doc="Read the DCR register."]
  #[inline] pub fn dcr(&self) -> Dcr { 
     unsafe {
        Dcr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
#[doc="Write the DCR register."]
  #[inline] pub fn set_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &Self {
     let value = f(Dcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DCR register."]
  #[inline] pub fn with_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &Self {
     let tmp = self.dcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DMAR register."]
  #[inline] pub fn dmar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
#[doc="Get the *mut pointer for the DMAR register."]
  #[inline] pub fn dmar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
#[doc="Read the DMAR register."]
  #[inline] pub fn dmar(&self) -> Dmar { 
     unsafe {
        Dmar(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
#[doc="Write the DMAR register."]
  #[inline] pub fn set_dmar<F: FnOnce(Dmar) -> Dmar>(&self, f: F) -> &Self {
     let value = f(Dmar(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DMAR register."]
  #[inline] pub fn with_dmar<F: FnOnce(Dmar) -> Dmar>(&self, f: F) -> &Self {
     let tmp = self.dmar();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OR register."]
  #[inline] pub fn or_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
#[doc="Get the *mut pointer for the OR register."]
  #[inline] pub fn or_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
#[doc="Read the OR register."]
  #[inline] pub fn or(&self) -> Or { 
     unsafe {
        Or(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
#[doc="Write the OR register."]
  #[inline] pub fn set_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &Self {
     let value = f(Or(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OR register."]
  #[inline] pub fn with_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &Self {
     let tmp = self.or();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }

}

#[doc="control register 1"]
#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
#[doc="Counter enable"]
  #[inline] pub fn cen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Counter enable"]
  #[inline] pub fn set_cen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Update disable"]
  #[inline] pub fn udis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Update disable"]
  #[inline] pub fn set_udis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Update request source"]
  #[inline] pub fn urs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Update request source"]
  #[inline] pub fn set_urs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="One-pulse mode"]
  #[inline] pub fn opm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="One-pulse mode"]
  #[inline] pub fn set_opm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Direction"]
  #[inline] pub fn dir(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Direction"]
  #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Center-aligned mode selection"]
  #[inline] pub fn cms(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
  }
#[doc="Center-aligned mode selection"]
  #[inline] pub fn set_cms<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Auto-reload preload enable"]
  #[inline] pub fn arpe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Auto-reload preload enable"]
  #[inline] pub fn set_arpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Clock division"]
  #[inline] pub fn ckd(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
  }
#[doc="Clock division"]
  #[inline] pub fn set_ckd<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="UIF status bit remapping"]
  #[inline] pub fn uifremap(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="UIF status bit remapping"]
  #[inline] pub fn set_uifremap<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
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
      if self.cen() != 0 { try!(write!(f, " cen"))}
      if self.udis() != 0 { try!(write!(f, " udis"))}
      if self.urs() != 0 { try!(write!(f, " urs"))}
      if self.opm() != 0 { try!(write!(f, " opm"))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.cms() != 0 { try!(write!(f, " cms=0x{:x}", self.cms()))}
      if self.arpe() != 0 { try!(write!(f, " arpe"))}
      if self.ckd() != 0 { try!(write!(f, " ckd=0x{:x}", self.ckd()))}
      if self.uifremap() != 0 { try!(write!(f, " uifremap"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="control register 2"]
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
#[doc="TI1 selection"]
  #[inline] pub fn ti1s(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="TI1 selection"]
  #[inline] pub fn set_ti1s<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Master mode selection"]
  #[inline] pub fn mms(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
  }
#[doc="Master mode selection"]
  #[inline] pub fn set_mms<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Capture/compare DMA selection"]
  #[inline] pub fn ccds(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Capture/compare DMA selection"]
  #[inline] pub fn set_ccds<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
      if self.ti1s() != 0 { try!(write!(f, " ti1s"))}
      if self.mms() != 0 { try!(write!(f, " mms=0x{:x}", self.mms()))}
      if self.ccds() != 0 { try!(write!(f, " ccds"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="slave mode control register"]
#[derive(PartialEq, Eq)]
pub struct Smcr(pub u32);
impl Smcr {
#[doc="Slave mode selection"]
  #[inline] pub fn sms(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
  }
#[doc="Slave mode selection"]
  #[inline] pub fn set_sms<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="OCREF clear selection"]
  #[inline] pub fn occs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="OCREF clear selection"]
  #[inline] pub fn set_occs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Trigger selection"]
  #[inline] pub fn ts(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
  }
#[doc="Trigger selection"]
  #[inline] pub fn set_ts<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Master/Slave mode"]
  #[inline] pub fn msm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Master/Slave mode"]
  #[inline] pub fn set_msm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="External trigger filter"]
  #[inline] pub fn etf(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="External trigger filter"]
  #[inline] pub fn set_etf<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="External trigger prescaler"]
  #[inline] pub fn etps(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
  }
#[doc="External trigger prescaler"]
  #[inline] pub fn set_etps<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="External clock enable"]
  #[inline] pub fn ece(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="External clock enable"]
  #[inline] pub fn set_ece<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="External trigger polarity"]
  #[inline] pub fn etp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="External trigger polarity"]
  #[inline] pub fn set_etp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Slave mode selection bit3"]
  #[inline] pub fn sms_3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Slave mode selection bit3"]
  #[inline] pub fn set_sms_3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Smcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Smcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sms() != 0 { try!(write!(f, " sms=0x{:x}", self.sms()))}
      if self.occs() != 0 { try!(write!(f, " occs"))}
      if self.ts() != 0 { try!(write!(f, " ts=0x{:x}", self.ts()))}
      if self.msm() != 0 { try!(write!(f, " msm"))}
      if self.etf() != 0 { try!(write!(f, " etf=0x{:x}", self.etf()))}
      if self.etps() != 0 { try!(write!(f, " etps=0x{:x}", self.etps()))}
      if self.ece() != 0 { try!(write!(f, " ece"))}
      if self.etp() != 0 { try!(write!(f, " etp"))}
      if self.sms_3() != 0 { try!(write!(f, " sms_3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA/Interrupt enable register"]
#[derive(PartialEq, Eq)]
pub struct Dier(pub u32);
impl Dier {
#[doc="Trigger DMA request enable"]
  #[inline] pub fn tde(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="Trigger DMA request enable"]
  #[inline] pub fn set_tde<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Capture/Compare n DMA request enable"]
  #[inline] pub fn ccde<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 9 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [9]
  }
#[doc="Capture/Compare n DMA request enable"]
  #[inline] pub fn set_ccde<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 9 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Update DMA request enable"]
  #[inline] pub fn ude(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Update DMA request enable"]
  #[inline] pub fn set_ude<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Trigger interrupt enable"]
  #[inline] pub fn tie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Trigger interrupt enable"]
  #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Capture/Compare n interrupt enable"]
  #[inline] pub fn ccie<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 1 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
  }
#[doc="Capture/Compare n interrupt enable"]
  #[inline] pub fn set_ccie<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 1 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Update interrupt enable"]
  #[inline] pub fn uie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Update interrupt enable"]
  #[inline] pub fn set_uie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tde() != 0 { try!(write!(f, " tde"))}
      if self.ccde(0) != 0 { try!(write!(f, " ccde[0]"))}
      if self.ccde(1) != 0 { try!(write!(f, " ccde[1]"))}
      if self.ccde(2) != 0 { try!(write!(f, " ccde[2]"))}
      if self.ccde(3) != 0 { try!(write!(f, " ccde[3]"))}
      if self.ude() != 0 { try!(write!(f, " ude"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.ccie(0) != 0 { try!(write!(f, " ccie[0]"))}
      if self.ccie(1) != 0 { try!(write!(f, " ccie[1]"))}
      if self.ccie(2) != 0 { try!(write!(f, " ccie[2]"))}
      if self.ccie(3) != 0 { try!(write!(f, " ccie[3]"))}
      if self.uie() != 0 { try!(write!(f, " uie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="status register"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="Capture/Compare n overcapture flag"]
  #[inline] pub fn ccof<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 9 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [9]
  }
#[doc="Capture/Compare n overcapture flag"]
  #[inline] pub fn set_ccof<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 9 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Trigger interrupt flag"]
  #[inline] pub fn tif(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Trigger interrupt flag"]
  #[inline] pub fn set_tif<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Capture/compare n interrupt flag"]
  #[inline] pub fn ccif<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 1 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
  }
#[doc="Capture/compare n interrupt flag"]
  #[inline] pub fn set_ccif<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 1 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Update interrupt flag"]
  #[inline] pub fn uif(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Update interrupt flag"]
  #[inline] pub fn set_uif<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccof(0) != 0 { try!(write!(f, " ccof[0]"))}
      if self.ccof(1) != 0 { try!(write!(f, " ccof[1]"))}
      if self.ccof(2) != 0 { try!(write!(f, " ccof[2]"))}
      if self.ccof(3) != 0 { try!(write!(f, " ccof[3]"))}
      if self.tif() != 0 { try!(write!(f, " tif"))}
      if self.ccif(0) != 0 { try!(write!(f, " ccif[0]"))}
      if self.ccif(1) != 0 { try!(write!(f, " ccif[1]"))}
      if self.ccif(2) != 0 { try!(write!(f, " ccif[2]"))}
      if self.ccif(3) != 0 { try!(write!(f, " ccif[3]"))}
      if self.uif() != 0 { try!(write!(f, " uif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="event generation register"]
#[derive(PartialEq, Eq)]
pub struct Egr(pub u32);
impl Egr {
#[doc="Trigger generation"]
  #[inline] pub fn tg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Trigger generation"]
  #[inline] pub fn set_tg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Capture/compare n generation"]
  #[inline] pub fn ccg<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 1 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
  }
#[doc="Capture/compare n generation"]
  #[inline] pub fn set_ccg<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 1 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Update generation"]
  #[inline] pub fn ug(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Update generation"]
  #[inline] pub fn set_ug<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Egr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Egr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tg() != 0 { try!(write!(f, " tg"))}
      if self.ccg(0) != 0 { try!(write!(f, " ccg[0]"))}
      if self.ccg(1) != 0 { try!(write!(f, " ccg[1]"))}
      if self.ccg(2) != 0 { try!(write!(f, " ccg[2]"))}
      if self.ccg(3) != 0 { try!(write!(f, " ccg[3]"))}
      if self.ug() != 0 { try!(write!(f, " ug"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare mode register 1 (output mode)"]
#[derive(PartialEq, Eq)]
pub struct CcmrOutput(pub u32);
impl CcmrOutput {
#[doc="Capture/Compare n selection"]
  #[inline] pub fn ccs<I: Into<bits::R2>>(&self, index: I) -> bits::U2 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
  }
#[doc="Capture/Compare n selection"]
  #[inline] pub fn set_ccs<I: Into<bits::R2>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Output compare n fast enable"]
  #[inline] pub fn ocfe<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 2 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
  }
#[doc="Output compare n fast enable"]
  #[inline] pub fn set_ocfe<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Output compare n preload enable"]
  #[inline] pub fn ocpe<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 3 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
  }
#[doc="Output compare n preload enable"]
  #[inline] pub fn set_ocpe<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 3 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Output compare n mode"]
  #[inline] pub fn ocm<I: Into<bits::R2>>(&self, index: I) -> bits::U3 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 4 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x7) as u8) } // [6:4]
  }
#[doc="Output compare n mode"]
  #[inline] pub fn set_ocm<I: Into<bits::R2>, V: Into<bits::U3>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x7 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Output compare n clear enable"]
  #[inline] pub fn occe<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 7 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [7]
  }
#[doc="Output compare n clear enable"]
  #[inline] pub fn set_occe<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 7 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Output compare n mode bit 3"]
  #[inline] pub fn ocm_3<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 16 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
  }
#[doc="Output compare n mode bit 3"]
  #[inline] pub fn set_ocm_3<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 16 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for CcmrOutput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for CcmrOutput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccs(0) != 0 { try!(write!(f, " ccs[0]=0x{:x}", self.ccs(0)))}
      if self.ccs(1) != 0 { try!(write!(f, " ccs[1]=0x{:x}", self.ccs(1)))}
      if self.ocfe(0) != 0 { try!(write!(f, " ocfe[0]"))}
      if self.ocfe(1) != 0 { try!(write!(f, " ocfe[1]"))}
      if self.ocpe(0) != 0 { try!(write!(f, " ocpe[0]"))}
      if self.ocpe(1) != 0 { try!(write!(f, " ocpe[1]"))}
      if self.ocm(0) != 0 { try!(write!(f, " ocm[0]=0x{:x}", self.ocm(0)))}
      if self.ocm(1) != 0 { try!(write!(f, " ocm[1]=0x{:x}", self.ocm(1)))}
      if self.occe(0) != 0 { try!(write!(f, " occe[0]"))}
      if self.occe(1) != 0 { try!(write!(f, " occe[1]"))}
      if self.ocm_3(0) != 0 { try!(write!(f, " ocm_3[0]"))}
      if self.ocm_3(1) != 0 { try!(write!(f, " ocm_3[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare mode register n (input mode)"]
#[derive(PartialEq, Eq)]
pub struct CcmrInput(pub u32);
impl CcmrInput {
#[doc="Input capture n filter"]
  #[inline] pub fn icf<I: Into<bits::R2>>(&self, index: I) -> bits::U4 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 4 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [7:4]
  }
#[doc="Input capture n filter"]
  #[inline] pub fn set_icf<I: Into<bits::R2>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Input capture n prescaler"]
  #[inline] pub fn icpsc<I: Into<bits::R2>>(&self, index: I) -> bits::U2 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 2 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [3:2]
  }
#[doc="Input capture n prescaler"]
  #[inline] pub fn set_icpsc<I: Into<bits::R2>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Capture/Compare n selection"]
  #[inline] pub fn ccs<I: Into<bits::R2>>(&self, index: I) -> bits::U2 {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + (index << 3);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
  }
#[doc="Capture/Compare n selection"]
  #[inline] pub fn set_ccs<I: Into<bits::R2>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value();
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for CcmrInput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for CcmrInput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.icf(0) != 0 { try!(write!(f, " icf[0]=0x{:x}", self.icf(0)))}
      if self.icf(1) != 0 { try!(write!(f, " icf[1]=0x{:x}", self.icf(1)))}
      if self.icpsc(0) != 0 { try!(write!(f, " icpsc[0]=0x{:x}", self.icpsc(0)))}
      if self.icpsc(1) != 0 { try!(write!(f, " icpsc[1]=0x{:x}", self.icpsc(1)))}
      if self.ccs(0) != 0 { try!(write!(f, " ccs[0]=0x{:x}", self.ccs(0)))}
      if self.ccs(1) != 0 { try!(write!(f, " ccs[1]=0x{:x}", self.ccs(1)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare enable register"]
#[derive(PartialEq, Eq)]
pub struct Ccer(pub u32);
impl Ccer {
#[doc="Capture/Compare n output enable"]
  #[inline] pub fn cce<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + (index << 2);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Capture/Compare n output enable"]
  #[inline] pub fn set_cce<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Capture/Compare n output Polarity"]
  #[inline] pub fn ccp<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 1 + (index << 2);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
  }
#[doc="Capture/Compare n output Polarity"]
  #[inline] pub fn set_ccp<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 1 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Capture/Compare n output Polarity"]
  #[inline] pub fn ccnp<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let shift: usize = 3 + (index << 2);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
  }
#[doc="Capture/Compare n output Polarity"]
  #[inline] pub fn set_ccnp<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R4 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     let shift: usize = 3 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ccer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cce(0) != 0 { try!(write!(f, " cce[0]"))}
      if self.cce(1) != 0 { try!(write!(f, " cce[1]"))}
      if self.cce(2) != 0 { try!(write!(f, " cce[2]"))}
      if self.cce(3) != 0 { try!(write!(f, " cce[3]"))}
      if self.ccp(0) != 0 { try!(write!(f, " ccp[0]"))}
      if self.ccp(1) != 0 { try!(write!(f, " ccp[1]"))}
      if self.ccp(2) != 0 { try!(write!(f, " ccp[2]"))}
      if self.ccp(3) != 0 { try!(write!(f, " ccp[3]"))}
      if self.ccnp(0) != 0 { try!(write!(f, " ccnp[0]"))}
      if self.ccnp(1) != 0 { try!(write!(f, " ccnp[1]"))}
      if self.ccnp(2) != 0 { try!(write!(f, " ccnp[2]"))}
      if self.ccnp(3) != 0 { try!(write!(f, " ccnp[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="counter"]
#[derive(PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
#[doc="Low counter value"]
  #[inline] pub fn cntl(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Low counter value"]
  #[inline] pub fn set_cntl<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="High counter value"]
  #[inline] pub fn cnth(&self) -> bits::U15 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7fff) as u16) } // [30:16]
  }
#[doc="High counter value"]
  #[inline] pub fn set_cnth<V: Into<bits::U15>>(mut self, value: V) -> Self {
     let value: bits::U15 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7fff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access"]
  #[inline] pub fn cnt_or_uifcpy(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
  }
#[doc="if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access"]
  #[inline] pub fn set_cnt_or_uifcpy<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cntl() != 0 { try!(write!(f, " cntl=0x{:x}", self.cntl()))}
      if self.cnth() != 0 { try!(write!(f, " cnth=0x{:x}", self.cnth()))}
      if self.cnt_or_uifcpy() != 0 { try!(write!(f, " cnt_or_uifcpy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="prescaler"]
#[derive(PartialEq, Eq)]
pub struct Psc(pub u32);
impl Psc {
#[doc="Prescaler value"]
  #[inline] pub fn psc(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Prescaler value"]
  #[inline] pub fn set_psc<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Psc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Psc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="auto-reload register"]
#[derive(PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
#[doc="Low Auto-reload value"]
  #[inline] pub fn arrl(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Low Auto-reload value"]
  #[inline] pub fn set_arrl<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="High Auto-reload value"]
  #[inline] pub fn arrh(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
  }
#[doc="High Auto-reload value"]
  #[inline] pub fn set_arrh<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Arr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Arr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.arrl() != 0 { try!(write!(f, " arrl=0x{:x}", self.arrl()))}
      if self.arrh() != 0 { try!(write!(f, " arrh=0x{:x}", self.arrh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="repetition counter register"]
#[derive(PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
#[doc="Repetition counter value"]
  #[inline] pub fn rep(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Repetition counter value"]
  #[inline] pub fn set_rep<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
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
      if self.rep() != 0 { try!(write!(f, " rep=0x{:x}", self.rep()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="break and dead-time register"]
#[derive(PartialEq, Eq)]
pub struct Bdtr(pub u32);
impl Bdtr {
#[doc="Main output enable"]
  #[inline] pub fn moe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Main output enable"]
  #[inline] pub fn set_moe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Automatic output enable"]
  #[inline] pub fn aoe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="Automatic output enable"]
  #[inline] pub fn set_aoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Break polarity"]
  #[inline] pub fn bkp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="Break polarity"]
  #[inline] pub fn set_bkp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Break enable"]
  #[inline] pub fn bke(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Break enable"]
  #[inline] pub fn set_bke<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Off-state selection for Run mode"]
  #[inline] pub fn ossr(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Off-state selection for Run mode"]
  #[inline] pub fn set_ossr<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Off-state selection for Idle mode"]
  #[inline] pub fn ossi(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Off-state selection for Idle mode"]
  #[inline] pub fn set_ossi<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Lock configuration"]
  #[inline] pub fn lock(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
  }
#[doc="Lock configuration"]
  #[inline] pub fn set_lock<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Dead-time generator setup"]
  #[inline] pub fn dtg(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Dead-time generator setup"]
  #[inline] pub fn set_dtg<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Break filter"]
  #[inline] pub fn bkf(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="Break filter"]
  #[inline] pub fn set_bkf<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Bdtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bdtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.moe() != 0 { try!(write!(f, " moe"))}
      if self.aoe() != 0 { try!(write!(f, " aoe"))}
      if self.bkp() != 0 { try!(write!(f, " bkp"))}
      if self.bke() != 0 { try!(write!(f, " bke"))}
      if self.ossr() != 0 { try!(write!(f, " ossr"))}
      if self.ossi() != 0 { try!(write!(f, " ossi"))}
      if self.lock() != 0 { try!(write!(f, " lock=0x{:x}", self.lock()))}
      if self.dtg() != 0 { try!(write!(f, " dtg=0x{:x}", self.dtg()))}
      if self.bkf() != 0 { try!(write!(f, " bkf=0x{:x}", self.bkf()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare register n"]
#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="Low Capture/Compare n value"]
  #[inline] pub fn ccrl(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Low Capture/Compare n value"]
  #[inline] pub fn set_ccrl<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="High Capture/Compare n value (on TIM2)"]
  #[inline] pub fn ccrh(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
  }
#[doc="High Capture/Compare n value (on TIM2)"]
  #[inline] pub fn set_ccrh<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccrl() != 0 { try!(write!(f, " ccrl=0x{:x}", self.ccrl()))}
      if self.ccrh() != 0 { try!(write!(f, " ccrh=0x{:x}", self.ccrh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA control register"]
#[derive(PartialEq, Eq)]
pub struct Dcr(pub u32);
impl Dcr {
#[doc="DMA burst length"]
  #[inline] pub fn dbl(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
  }
#[doc="DMA burst length"]
  #[inline] pub fn set_dbl<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

#[doc="DMA base address"]
  #[inline] pub fn dba(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
  }
#[doc="DMA base address"]
  #[inline] pub fn set_dba<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbl() != 0 { try!(write!(f, " dbl=0x{:x}", self.dbl()))}
      if self.dba() != 0 { try!(write!(f, " dba=0x{:x}", self.dba()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA address for full transfer"]
#[derive(PartialEq, Eq)]
pub struct Dmar(pub u32);
impl Dmar {
#[doc="DMA register for burst accesses"]
  #[inline] pub fn dmab(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="DMA register for burst accesses"]
  #[inline] pub fn set_dmab<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dmab() != 0 { try!(write!(f, " dmab=0x{:x}", self.dmab()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="option register"]
#[derive(PartialEq, Eq)]
pub struct Or(pub u32);
impl Or {
}
impl ::core::fmt::Display for Or {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Or {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="TIM_GEN Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

