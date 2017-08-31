#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TIM_ADV Peripheral"]
pub struct TimAdvPeriph(pub usize); 


impl TimAdvPeriph {
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
         Cr1(read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the CR1 register."]
   #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
      let value = f(Cr1(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR1 register."]
   #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
      let tmp = self.cr1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
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
         Cr2(read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the CR2 register."]
   #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
      let value = f(Cr2(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR2 register."]
   #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
      let tmp = self.cr2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
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
         Smcr(read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the SMCR register."]
   #[inline] pub fn set_smcr<F: FnOnce(Smcr) -> Smcr>(&self, f: F) -> &Self {
      let value = f(Smcr(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SMCR register."]
   #[inline] pub fn with_smcr<F: FnOnce(Smcr) -> Smcr>(&self, f: F) -> &Self {
      let tmp = self.smcr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
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
         Dier(read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the DIER register."]
   #[inline] pub fn set_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
      let value = f(Dier(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DIER register."]
   #[inline] pub fn with_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
      let tmp = self.dier();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
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
         Sr(read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the SR register."]
   #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let value = f(Sr(0));
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SR register."]
   #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let tmp = self.sr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
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
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCMR_OUTPUT register."]
   #[inline] pub fn ccmr_output_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the CCMR_OUTPUT register."]
   #[inline] pub fn ccmr_output_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
   }
#[doc="Read the CCMR_OUTPUT register."]
   #[inline] pub fn ccmr_output<I: Into<bits::R2>>(&self, index: I) -> CcmrOutput { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         CcmrOutput(read_volatile((self.0 + 0x18 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the CCMR_OUTPUT register."]
   #[inline] pub fn set_ccmr_output<I: Into<bits::R2>, F: FnOnce(CcmrOutput) -> CcmrOutput>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(CcmrOutput(0));
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCMR_OUTPUT register."]
   #[inline] pub fn with_ccmr_output<I: Into<bits::R2> + Copy, F: FnOnce(CcmrOutput) -> CcmrOutput>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.ccmr_output(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCMR_INPUT register."]
   #[inline] pub fn ccmr_input_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the CCMR_INPUT register."]
   #[inline] pub fn ccmr_input_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x18 + (index << 2)) as *mut u32
   }
#[doc="Read the CCMR_INPUT register."]
   #[inline] pub fn ccmr_input<I: Into<bits::R2>>(&self, index: I) -> CcmrInput { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         CcmrInput(read_volatile((self.0 + 0x18 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the CCMR_INPUT register."]
   #[inline] pub fn set_ccmr_input<I: Into<bits::R2>, F: FnOnce(CcmrInput) -> CcmrInput>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(CcmrInput(0));
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCMR_INPUT register."]
   #[inline] pub fn with_ccmr_input<I: Into<bits::R2> + Copy, F: FnOnce(CcmrInput) -> CcmrInput>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.ccmr_input(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18 + (index << 2)) as *mut u32, value.0);
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
         Ccer(read_volatile((self.0 + 0x20) as *const u32))
      }
   }
#[doc="Write the CCER register."]
   #[inline] pub fn set_ccer<F: FnOnce(Ccer) -> Ccer>(&self, f: F) -> &Self {
      let value = f(Ccer(0));
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCER register."]
   #[inline] pub fn with_ccer<F: FnOnce(Ccer) -> Ccer>(&self, f: F) -> &Self {
      let tmp = self.ccer();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
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
         Cnt(read_volatile((self.0 + 0x24) as *const u32))
      }
   }
#[doc="Write the CNT register."]
   #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
      let value = f(Cnt(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CNT register."]
   #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
      let tmp = self.cnt();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
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
         Psc(read_volatile((self.0 + 0x28) as *const u32))
      }
   }
#[doc="Write the PSC register."]
   #[inline] pub fn set_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
      let value = f(Psc(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PSC register."]
   #[inline] pub fn with_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
      let tmp = self.psc();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
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
         Arr(read_volatile((self.0 + 0x2c) as *const u32))
      }
   }
#[doc="Write the ARR register."]
   #[inline] pub fn set_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
      let value = f(Arr(0));
      unsafe {
         write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ARR register."]
   #[inline] pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
      let tmp = self.arr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x2c) as *mut u32, value.0);
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
         Rcr(read_volatile((self.0 + 0x30) as *const u32))
      }
   }
#[doc="Write the RCR register."]
   #[inline] pub fn set_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
      let value = f(Rcr(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCR register."]
   #[inline] pub fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
      let tmp = self.rcr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCR register."]
   #[inline] pub fn ccr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x34 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the CCR register."]
   #[inline] pub fn ccr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x34 + (index << 2)) as *mut u32
   }
#[doc="Read the CCR register."]
   #[inline] pub fn ccr<I: Into<bits::R4>>(&self, index: I) -> Ccr { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Ccr(read_volatile((self.0 + 0x34 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the CCR register."]
   #[inline] pub fn set_ccr<I: Into<bits::R4>, F: FnOnce(Ccr) -> Ccr>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Ccr(0));
      unsafe {
         write_volatile((self.0 + 0x34 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCR register."]
   #[inline] pub fn with_ccr<I: Into<bits::R4> + Copy, F: FnOnce(Ccr) -> Ccr>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.ccr(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x34 + (index << 2)) as *mut u32, value.0);
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
         Bdtr(read_volatile((self.0 + 0x44) as *const u32))
      }
   }
#[doc="Write the BDTR register."]
   #[inline] pub fn set_bdtr<F: FnOnce(Bdtr) -> Bdtr>(&self, f: F) -> &Self {
      let value = f(Bdtr(0));
      unsafe {
         write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the BDTR register."]
   #[inline] pub fn with_bdtr<F: FnOnce(Bdtr) -> Bdtr>(&self, f: F) -> &Self {
      let tmp = self.bdtr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x44) as *mut u32, value.0);
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
         Dcr(read_volatile((self.0 + 0x48) as *const u32))
      }
   }
#[doc="Write the DCR register."]
   #[inline] pub fn set_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &Self {
      let value = f(Dcr(0));
      unsafe {
         write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCR register."]
   #[inline] pub fn with_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &Self {
      let tmp = self.dcr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x48) as *mut u32, value.0);
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
         Dmar(read_volatile((self.0 + 0x4c) as *const u32))
      }
   }
#[doc="Write the DMAR register."]
   #[inline] pub fn set_dmar<F: FnOnce(Dmar) -> Dmar>(&self, f: F) -> &Self {
      let value = f(Dmar(0));
      unsafe {
         write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DMAR register."]
   #[inline] pub fn with_dmar<F: FnOnce(Dmar) -> Dmar>(&self, f: F) -> &Self {
      let tmp = self.dmar();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCMR3_OUTPUT register."]
   #[inline] pub fn ccmr3_output_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x54) as *const u32
   }
#[doc="Get the *mut pointer for the CCMR3_OUTPUT register."]
   #[inline] pub fn ccmr3_output_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x54) as *mut u32
   }
#[doc="Read the CCMR3_OUTPUT register."]
   #[inline] pub fn ccmr3_output(&self) -> Ccmr3Output { 
      unsafe {
         Ccmr3Output(read_volatile((self.0 + 0x54) as *const u32))
      }
   }
#[doc="Write the CCMR3_OUTPUT register."]
   #[inline] pub fn set_ccmr3_output<F: FnOnce(Ccmr3Output) -> Ccmr3Output>(&self, f: F) -> &Self {
      let value = f(Ccmr3Output(0));
      unsafe {
         write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCMR3_OUTPUT register."]
   #[inline] pub fn with_ccmr3_output<F: FnOnce(Ccmr3Output) -> Ccmr3Output>(&self, f: F) -> &Self {
      let tmp = self.ccmr3_output();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCR5 register."]
   #[inline] pub fn ccr5_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x58) as *const u32
   }
#[doc="Get the *mut pointer for the CCR5 register."]
   #[inline] pub fn ccr5_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x58) as *mut u32
   }
#[doc="Read the CCR5 register."]
   #[inline] pub fn ccr5(&self) -> Ccr5 { 
      unsafe {
         Ccr5(read_volatile((self.0 + 0x58) as *const u32))
      }
   }
#[doc="Write the CCR5 register."]
   #[inline] pub fn set_ccr5<F: FnOnce(Ccr5) -> Ccr5>(&self, f: F) -> &Self {
      let value = f(Ccr5(0));
      unsafe {
         write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCR5 register."]
   #[inline] pub fn with_ccr5<F: FnOnce(Ccr5) -> Ccr5>(&self, f: F) -> &Self {
      let tmp = self.ccr5();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCR6 register."]
   #[inline] pub fn ccr6_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x5c) as *const u32
   }
#[doc="Get the *mut pointer for the CCR6 register."]
   #[inline] pub fn ccr6_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x5c) as *mut u32
   }
#[doc="Read the CCR6 register."]
   #[inline] pub fn ccr6(&self) -> Ccr6 { 
      unsafe {
         Ccr6(read_volatile((self.0 + 0x5c) as *const u32))
      }
   }
#[doc="Write the CCR6 register."]
   #[inline] pub fn set_ccr6<F: FnOnce(Ccr6) -> Ccr6>(&self, f: F) -> &Self {
      let value = f(Ccr6(0));
      unsafe {
         write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCR6 register."]
   #[inline] pub fn with_ccr6<F: FnOnce(Ccr6) -> Ccr6>(&self, f: F) -> &Self {
      let tmp = self.ccr6();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the OR register."]
   #[inline] pub fn or_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x60) as *const u32
   }
#[doc="Get the *mut pointer for the OR register."]
   #[inline] pub fn or_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x60) as *mut u32
   }
#[doc="Read the OR register."]
   #[inline] pub fn or(&self) -> Or { 
      unsafe {
         Or(read_volatile((self.0 + 0x60) as *const u32))
      }
   }
#[doc="Write the OR register."]
   #[inline] pub fn set_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &Self {
      let value = f(Or(0));
      unsafe {
         write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the OR register."]
   #[inline] pub fn with_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &Self {
      let tmp = self.or();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }

}

#[doc="control register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
#[doc="Capture/compare preloaded control"]
   #[inline] pub fn ccpc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Capture/compare preloaded control"]
   #[inline] pub fn set_ccpc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Capture/compare control update selection"]
   #[inline] pub fn ccus(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Capture/compare control update selection"]
   #[inline] pub fn set_ccus<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
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

#[doc="Output Idle state 1"]
   #[inline] pub fn ois1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Output Idle state 1"]
   #[inline] pub fn set_ois1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Output Idle state 1"]
   #[inline] pub fn ois1n(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Output Idle state 1"]
   #[inline] pub fn set_ois1n<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Output Idle state 2"]
   #[inline] pub fn ois2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Output Idle state 2"]
   #[inline] pub fn set_ois2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Output Idle state 2"]
   #[inline] pub fn ois2n(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Output Idle state 2"]
   #[inline] pub fn set_ois2n<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Output Idle state 3"]
   #[inline] pub fn ois3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Output Idle state 3"]
   #[inline] pub fn set_ois3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Output Idle state 3"]
   #[inline] pub fn ois3n(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Output Idle state 3"]
   #[inline] pub fn set_ois3n<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Output Idle state 4"]
   #[inline] pub fn ois4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Output Idle state 4"]
   #[inline] pub fn set_ois4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Output Idle state 5"]
   #[inline] pub fn ois5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Output Idle state 5"]
   #[inline] pub fn set_ois5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Output Idle state 6"]
   #[inline] pub fn ois6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Output Idle state 6"]
   #[inline] pub fn set_ois6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="Master mode selection 2"]
   #[inline] pub fn mms2(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
   }
#[doc="Master mode selection 2"]
   #[inline] pub fn set_mms2<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 20);
      self.0 |= value << 20;
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
      if self.ccpc() != 0 { try!(write!(f, " ccpc"))}
      if self.ccus() != 0 { try!(write!(f, " ccus"))}
      if self.ccds() != 0 { try!(write!(f, " ccds"))}
      if self.mms() != 0 { try!(write!(f, " mms=0x{:x}", self.mms()))}
      if self.ti1s() != 0 { try!(write!(f, " ti1s"))}
      if self.ois1() != 0 { try!(write!(f, " ois1"))}
      if self.ois1n() != 0 { try!(write!(f, " ois1n"))}
      if self.ois2() != 0 { try!(write!(f, " ois2"))}
      if self.ois2n() != 0 { try!(write!(f, " ois2n"))}
      if self.ois3() != 0 { try!(write!(f, " ois3"))}
      if self.ois3n() != 0 { try!(write!(f, " ois3n"))}
      if self.ois4() != 0 { try!(write!(f, " ois4"))}
      if self.ois5() != 0 { try!(write!(f, " ois5"))}
      if self.ois6() != 0 { try!(write!(f, " ois6"))}
      if self.mms2() != 0 { try!(write!(f, " mms2=0x{:x}", self.mms2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="slave mode control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
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

#[doc="Slave mode selection bit 3"]
   #[inline] pub fn sms3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Slave mode selection bit 3"]
   #[inline] pub fn set_sms3<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
      if self.sms3() != 0 { try!(write!(f, " sms3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA/Interrupt enable register"]
#[derive(Clone, Copy, PartialEq, Eq)]
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

#[doc="Reserved"]
   #[inline] pub fn comde(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Reserved"]
   #[inline] pub fn set_comde<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
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

#[doc="Break interrupt enable"]
   #[inline] pub fn bie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Break interrupt enable"]
   #[inline] pub fn set_bie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
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

#[doc="COM interrupt enable"]
   #[inline] pub fn comie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="COM interrupt enable"]
   #[inline] pub fn set_comie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
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
      if self.comde() != 0 { try!(write!(f, " comde"))}
      if self.ccde(0) != 0 { try!(write!(f, " ccde[0]"))}
      if self.ccde(1) != 0 { try!(write!(f, " ccde[1]"))}
      if self.ccde(2) != 0 { try!(write!(f, " ccde[2]"))}
      if self.ccde(3) != 0 { try!(write!(f, " ccde[3]"))}
      if self.ude() != 0 { try!(write!(f, " ude"))}
      if self.bie() != 0 { try!(write!(f, " bie"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.comie() != 0 { try!(write!(f, " comie"))}
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
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

#[doc="Capture/compare 1 interrupt flag"]
   #[inline] pub fn cc1if(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Capture/compare 1 interrupt flag"]
   #[inline] pub fn set_cc1if<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture/Compare 2 interrupt flag"]
   #[inline] pub fn cc2if(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Capture/Compare 2 interrupt flag"]
   #[inline] pub fn set_cc2if<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Capture/Compare 3 interrupt flag"]
   #[inline] pub fn cc3if(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Capture/Compare 3 interrupt flag"]
   #[inline] pub fn set_cc3if<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Capture/Compare 4 interrupt flag"]
   #[inline] pub fn cc4if(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture/Compare 4 interrupt flag"]
   #[inline] pub fn set_cc4if<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="COM interrupt flag"]
   #[inline] pub fn comif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="COM interrupt flag"]
   #[inline] pub fn set_comif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
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

#[doc="Break interrupt flag"]
   #[inline] pub fn bif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Break interrupt flag"]
   #[inline] pub fn set_bif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Break 2 interrupt flag"]
   #[inline] pub fn b2if(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Break 2 interrupt flag"]
   #[inline] pub fn set_b2if<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Capture/Compare 1 overcapture flag"]
   #[inline] pub fn cc1of(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Capture/Compare 1 overcapture flag"]
   #[inline] pub fn set_cc1of<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Capture/compare 2 overcapture flag"]
   #[inline] pub fn cc2of(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Capture/compare 2 overcapture flag"]
   #[inline] pub fn set_cc2of<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Capture/Compare 3 overcapture flag"]
   #[inline] pub fn cc3of(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Capture/Compare 3 overcapture flag"]
   #[inline] pub fn set_cc3of<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Capture/Compare 4 overcapture flag"]
   #[inline] pub fn cc4of(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Capture/Compare 4 overcapture flag"]
   #[inline] pub fn set_cc4of<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Capture/Compare 5 interrupt flag"]
   #[inline] pub fn cc5if(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Capture/Compare 5 interrupt flag"]
   #[inline] pub fn set_cc5if<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Capture/Compare 6 interrupt flag"]
   #[inline] pub fn cc6if(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Capture/Compare 6 interrupt flag"]
   #[inline] pub fn set_cc6if<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
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
      if self.uif() != 0 { try!(write!(f, " uif"))}
      if self.cc1if() != 0 { try!(write!(f, " cc1if"))}
      if self.cc2if() != 0 { try!(write!(f, " cc2if"))}
      if self.cc3if() != 0 { try!(write!(f, " cc3if"))}
      if self.cc4if() != 0 { try!(write!(f, " cc4if"))}
      if self.comif() != 0 { try!(write!(f, " comif"))}
      if self.tif() != 0 { try!(write!(f, " tif"))}
      if self.bif() != 0 { try!(write!(f, " bif"))}
      if self.b2if() != 0 { try!(write!(f, " b2if"))}
      if self.cc1of() != 0 { try!(write!(f, " cc1of"))}
      if self.cc2of() != 0 { try!(write!(f, " cc2of"))}
      if self.cc3of() != 0 { try!(write!(f, " cc3of"))}
      if self.cc4of() != 0 { try!(write!(f, " cc4of"))}
      if self.cc5if() != 0 { try!(write!(f, " cc5if"))}
      if self.cc6if() != 0 { try!(write!(f, " cc6if"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="event generation register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Egr(pub u32);
impl Egr {
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

#[doc="Capture/compare 1 generation"]
   #[inline] pub fn cc1g(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Capture/compare 1 generation"]
   #[inline] pub fn set_cc1g<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture/compare 2 generation"]
   #[inline] pub fn cc2g(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Capture/compare 2 generation"]
   #[inline] pub fn set_cc2g<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Capture/compare 3 generation"]
   #[inline] pub fn cc3g(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Capture/compare 3 generation"]
   #[inline] pub fn set_cc3g<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Capture/compare 4 generation"]
   #[inline] pub fn cc4g(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Capture/compare 4 generation"]
   #[inline] pub fn set_cc4g<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Capture/Compare control update generation"]
   #[inline] pub fn comg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Capture/Compare control update generation"]
   #[inline] pub fn set_comg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

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

#[doc="Break generation"]
   #[inline] pub fn bg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Break generation"]
   #[inline] pub fn set_bg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Break 2 generation"]
   #[inline] pub fn b2g(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Break 2 generation"]
   #[inline] pub fn set_b2g<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
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
      if self.ug() != 0 { try!(write!(f, " ug"))}
      if self.cc1g() != 0 { try!(write!(f, " cc1g"))}
      if self.cc2g() != 0 { try!(write!(f, " cc2g"))}
      if self.cc3g() != 0 { try!(write!(f, " cc3g"))}
      if self.cc4g() != 0 { try!(write!(f, " cc4g"))}
      if self.comg() != 0 { try!(write!(f, " comg"))}
      if self.tg() != 0 { try!(write!(f, " tg"))}
      if self.bg() != 0 { try!(write!(f, " bg"))}
      if self.b2g() != 0 { try!(write!(f, " b2g"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare mode register (output mode)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CcmrOutput(pub u32);
impl CcmrOutput {
#[doc="Output Compare n clear enable"]
   #[inline] pub fn occe<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
      let index: bits::R2 = index.into();
      let index: usize = index.value();
      let shift: usize = 7 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [7]
   }
#[doc="Output Compare n clear enable"]
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

#[doc="Output Compare n mode"]
   #[inline] pub fn ocm<I: Into<bits::R2>>(&self, index: I) -> bits::U3 {
      let index: bits::R2 = index.into();
      let index: usize = index.value();
      let shift: usize = 4 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x7) as u8) } // [6:4]
   }
#[doc="Output Compare n mode"]
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

#[doc="Output Compare n preload enable"]
   #[inline] pub fn ocpe<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
      let index: bits::R2 = index.into();
      let index: usize = index.value();
      let shift: usize = 3 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
   }
#[doc="Output Compare n preload enable"]
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

#[doc="Output Compare n fast enable"]
   #[inline] pub fn ocfe<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
      let index: bits::R2 = index.into();
      let index: usize = index.value();
      let shift: usize = 2 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
   }
#[doc="Output Compare n fast enable"]
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
impl ::core::fmt::Display for CcmrOutput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for CcmrOutput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.occe(0) != 0 { try!(write!(f, " occe[0]"))}
      if self.occe(1) != 0 { try!(write!(f, " occe[1]"))}
      if self.ocm(0) != 0 { try!(write!(f, " ocm[0]=0x{:x}", self.ocm(0)))}
      if self.ocm(1) != 0 { try!(write!(f, " ocm[1]=0x{:x}", self.ocm(1)))}
      if self.ocpe(0) != 0 { try!(write!(f, " ocpe[0]"))}
      if self.ocpe(1) != 0 { try!(write!(f, " ocpe[1]"))}
      if self.ocfe(0) != 0 { try!(write!(f, " ocfe[0]"))}
      if self.ocfe(1) != 0 { try!(write!(f, " ocfe[1]"))}
      if self.ccs(0) != 0 { try!(write!(f, " ccs[0]=0x{:x}", self.ccs(0)))}
      if self.ccs(1) != 0 { try!(write!(f, " ccs[1]=0x{:x}", self.ccs(1)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare mode register n (input mode)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CcmrInput(pub u32);
impl CcmrInput {
#[doc="Input capture 1 filter"]
   #[inline] pub fn icf<I: Into<bits::R2>>(&self, index: I) -> bits::U4 {
      let index: bits::R2 = index.into();
      let index: usize = index.value();
      let shift: usize = 4 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [7:4]
   }
#[doc="Input capture 1 filter"]
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

#[doc="Input capture 1 prescaler"]
   #[inline] pub fn icpcs<I: Into<bits::R2>>(&self, index: I) -> bits::U2 {
      let index: bits::R2 = index.into();
      let index: usize = index.value();
      let shift: usize = 2 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [3:2]
   }
#[doc="Input capture 1 prescaler"]
   #[inline] pub fn set_icpcs<I: Into<bits::R2>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value();
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      let shift: usize = 2 + (index << 3);
      self.0 &= !(0x3 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Capture/Compare 1 selection"]
   #[inline] pub fn cc1s<I: Into<bits::R2>>(&self, index: I) -> bits::U2 {
      let index: bits::R2 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
   }
#[doc="Capture/Compare 1 selection"]
   #[inline] pub fn set_cc1s<I: Into<bits::R2>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
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
      if self.icpcs(0) != 0 { try!(write!(f, " icpcs[0]=0x{:x}", self.icpcs(0)))}
      if self.icpcs(1) != 0 { try!(write!(f, " icpcs[1]=0x{:x}", self.icpcs(1)))}
      if self.cc1s(0) != 0 { try!(write!(f, " cc1s[0]=0x{:x}", self.cc1s(0)))}
      if self.cc1s(1) != 0 { try!(write!(f, " cc1s[1]=0x{:x}", self.cc1s(1)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare enable register"]
#[derive(Clone, Copy, PartialEq, Eq)]
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
   #[inline] pub fn ccp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Capture/Compare n output Polarity"]
   #[inline] pub fn set_ccp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Capture/Compare n complementary output enable"]
   #[inline] pub fn ccne(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Capture/Compare n complementary output enable"]
   #[inline] pub fn set_ccne<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Capture/Compare n output Polarity"]
   #[inline] pub fn ccnp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Capture/Compare n output Polarity"]
   #[inline] pub fn set_ccnp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
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
      if self.ccp() != 0 { try!(write!(f, " ccp"))}
      if self.ccne() != 0 { try!(write!(f, " ccne"))}
      if self.ccnp() != 0 { try!(write!(f, " ccnp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="counter"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
#[doc="counter value"]
   #[inline] pub fn cnt(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="counter value"]
   #[inline] pub fn set_cnt<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UIF copy"]
   #[inline] pub fn uifcpy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="UIF copy"]
   #[inline] pub fn set_uifcpy<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
      if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
      if self.uifcpy() != 0 { try!(write!(f, " uifcpy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="prescaler"]
#[derive(Clone, Copy, PartialEq, Eq)]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
#[doc="Auto-reload value"]
   #[inline] pub fn arr(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Auto-reload value"]
   #[inline] pub fn set_arr<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
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
      if self.arr() != 0 { try!(write!(f, " arr=0x{:x}", self.arr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="repetition counter register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
#[doc="Repetition counter value"]
   #[inline] pub fn rep(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Repetition counter value"]
   #[inline] pub fn set_rep<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
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
#[doc="capture/compare register n"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="Capture/Compare n value"]
   #[inline] pub fn ccr(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Capture/Compare n value"]
   #[inline] pub fn set_ccr<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
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
      if self.ccr() != 0 { try!(write!(f, " ccr=0x{:x}", self.ccr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="break and dead-time register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bdtr(pub u32);
impl Bdtr {
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

#[doc="Break 2 filter"]
   #[inline] pub fn bk2f(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
   }
#[doc="Break 2 filter"]
   #[inline] pub fn set_bk2f<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Break 2 enable"]
   #[inline] pub fn bk2e(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Break 2 enable"]
   #[inline] pub fn set_bk2e<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Break 2 polarity"]
   #[inline] pub fn bk2p(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Break 2 polarity"]
   #[inline] pub fn set_bk2p<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
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
      if self.dtg() != 0 { try!(write!(f, " dtg=0x{:x}", self.dtg()))}
      if self.lock() != 0 { try!(write!(f, " lock=0x{:x}", self.lock()))}
      if self.ossi() != 0 { try!(write!(f, " ossi"))}
      if self.ossr() != 0 { try!(write!(f, " ossr"))}
      if self.bke() != 0 { try!(write!(f, " bke"))}
      if self.bkp() != 0 { try!(write!(f, " bkp"))}
      if self.aoe() != 0 { try!(write!(f, " aoe"))}
      if self.moe() != 0 { try!(write!(f, " moe"))}
      if self.bkf() != 0 { try!(write!(f, " bkf=0x{:x}", self.bkf()))}
      if self.bk2f() != 0 { try!(write!(f, " bk2f=0x{:x}", self.bk2f()))}
      if self.bk2e() != 0 { try!(write!(f, " bk2e"))}
      if self.bk2p() != 0 { try!(write!(f, " bk2p"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
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
#[derive(Clone, Copy, PartialEq, Eq)]
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
#[doc="capture/compare mode register 3 (output mode)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ccmr3Output(pub u32);
impl Ccmr3Output {
#[doc="Output compare 5 fast enable"]
   #[inline] pub fn oc5fe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Output compare 5 fast enable"]
   #[inline] pub fn set_oc5fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Output compare 5 preload enable"]
   #[inline] pub fn oc5pe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Output compare 5 preload enable"]
   #[inline] pub fn set_oc5pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Output compare 5 mode"]
   #[inline] pub fn oc5m(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
   }
#[doc="Output compare 5 mode"]
   #[inline] pub fn set_oc5m<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Output compare 5 clear enable"]
   #[inline] pub fn oc5ce(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Output compare 5 clear enable"]
   #[inline] pub fn set_oc5ce<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Output compare 6 fast enable"]
   #[inline] pub fn oc6fe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Output compare 6 fast enable"]
   #[inline] pub fn set_oc6fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Output compare 6 preload enable"]
   #[inline] pub fn oc6pe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Output compare 6 preload enable"]
   #[inline] pub fn set_oc6pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Output compare 6 mode"]
   #[inline] pub fn oc6m(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
   }
#[doc="Output compare 6 mode"]
   #[inline] pub fn set_oc6m<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Output compare 6 clear enable"]
   #[inline] pub fn oc6ce(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Output compare 6 clear enable"]
   #[inline] pub fn set_oc6ce<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Outout Compare 5 mode bit 3"]
   #[inline] pub fn oc5m_3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Outout Compare 5 mode bit 3"]
   #[inline] pub fn set_oc5m_3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Outout Compare 6 mode bit 3"]
   #[inline] pub fn oc6m_3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Outout Compare 6 mode bit 3"]
   #[inline] pub fn set_oc6m_3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Ccmr3Output {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccmr3Output {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.oc5fe() != 0 { try!(write!(f, " oc5fe"))}
      if self.oc5pe() != 0 { try!(write!(f, " oc5pe"))}
      if self.oc5m() != 0 { try!(write!(f, " oc5m=0x{:x}", self.oc5m()))}
      if self.oc5ce() != 0 { try!(write!(f, " oc5ce"))}
      if self.oc6fe() != 0 { try!(write!(f, " oc6fe"))}
      if self.oc6pe() != 0 { try!(write!(f, " oc6pe"))}
      if self.oc6m() != 0 { try!(write!(f, " oc6m=0x{:x}", self.oc6m()))}
      if self.oc6ce() != 0 { try!(write!(f, " oc6ce"))}
      if self.oc5m_3() != 0 { try!(write!(f, " oc5m_3"))}
      if self.oc6m_3() != 0 { try!(write!(f, " oc6m_3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare register 5"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ccr5(pub u32);
impl Ccr5 {
#[doc="Capture/Compare 5 value"]
   #[inline] pub fn ccr5(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Capture/Compare 5 value"]
   #[inline] pub fn set_ccr5<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Group Channel 5 and Channel 1"]
   #[inline] pub fn gc5c1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="Group Channel 5 and Channel 1"]
   #[inline] pub fn set_gc5c1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="Group Channel 5 and Channel 2"]
   #[inline] pub fn gc5c2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Group Channel 5 and Channel 2"]
   #[inline] pub fn set_gc5c2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Group Channel 5 and Channel 3"]
   #[inline] pub fn gc5c3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Group Channel 5 and Channel 3"]
   #[inline] pub fn set_gc5c3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Ccr5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccr5() != 0 { try!(write!(f, " ccr5=0x{:x}", self.ccr5()))}
      if self.gc5c1() != 0 { try!(write!(f, " gc5c1"))}
      if self.gc5c2() != 0 { try!(write!(f, " gc5c2"))}
      if self.gc5c3() != 0 { try!(write!(f, " gc5c3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="capture/compare register 6"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ccr6(pub u32);
impl Ccr6 {
#[doc="Capture/Compare 6 value"]
   #[inline] pub fn ccr6(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Capture/Compare 6 value"]
   #[inline] pub fn set_ccr6<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ccr6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccr6() != 0 { try!(write!(f, " ccr6=0x{:x}", self.ccr6()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="option registers"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Or(pub u32);
impl Or {
#[doc="TIM1_ETR_ADC1 remapping capability"]
   #[inline] pub fn tim1_etr_adc1_rmp(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="TIM1_ETR_ADC1 remapping capability"]
   #[inline] pub fn set_tim1_etr_adc1_rmp<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TIM1_ETR_ADC4 remapping capability"]
   #[inline] pub fn tim1_etr_adc4_rmp(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="TIM1_ETR_ADC4 remapping capability"]
   #[inline] pub fn set_tim1_etr_adc4_rmp<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

}
impl ::core::fmt::Display for Or {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Or {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim1_etr_adc1_rmp() != 0 { try!(write!(f, " tim1_etr_adc1_rmp=0x{:x}", self.tim1_etr_adc1_rmp()))}
      if self.tim1_etr_adc4_rmp() != 0 { try!(write!(f, " tim1_etr_adc4_rmp=0x{:x}", self.tim1_etr_adc4_rmp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct TimAdvCh { pub periph: TimAdvPeriph, pub index: usize }

