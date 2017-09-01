#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC_F3 Peripheral"]
pub struct AdcPeriph(pub usize); 


impl AdcPeriph {
   #[doc="Get the *const pointer for the ISR register."]
   #[inline] pub fn isr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }

   #[doc="Get the *mut pointer for the ISR register."]
   #[inline] pub fn isr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }

   #[doc="Read the ISR register."]
   #[inline] pub fn isr(&self) -> Isr { 
      unsafe {
         Isr(read_volatile((self.0 + 0x0) as *const u32))
      }
   }

   #[doc="Write the ISR register."]
   #[inline] pub fn set_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
      let value = f(Isr(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the ISR register."]
   #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
      let tmp = self.isr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the IER register."]
   #[inline] pub fn ier_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }

   #[doc="Get the *mut pointer for the IER register."]
   #[inline] pub fn ier_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }

   #[doc="Read the IER register."]
   #[inline] pub fn ier(&self) -> Ier { 
      unsafe {
         Ier(read_volatile((self.0 + 0x4) as *const u32))
      }
   }

   #[doc="Write the IER register."]
   #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
      let value = f(Ier(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the IER register."]
   #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
      let tmp = self.ier();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the CR register."]
   #[inline] pub fn cr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }

   #[doc="Get the *mut pointer for the CR register."]
   #[inline] pub fn cr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }

   #[doc="Read the CR register."]
   #[inline] pub fn cr(&self) -> Cr { 
      unsafe {
         Cr(read_volatile((self.0 + 0x8) as *const u32))
      }
   }

   #[doc="Write the CR register."]
   #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CR register."]
   #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = self.cr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the CFGR register."]
   #[inline] pub fn cfgr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }

   #[doc="Get the *mut pointer for the CFGR register."]
   #[inline] pub fn cfgr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }

   #[doc="Read the CFGR register."]
   #[inline] pub fn cfgr(&self) -> Cfgr { 
      unsafe {
         Cfgr(read_volatile((self.0 + 0xc) as *const u32))
      }
   }

   #[doc="Write the CFGR register."]
   #[inline] pub fn set_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
      let value = f(Cfgr(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CFGR register."]
   #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
      let tmp = self.cfgr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the SMPR1 register."]
   #[inline] pub fn smpr1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x14) as *const u32
   }

   #[doc="Get the *mut pointer for the SMPR1 register."]
   #[inline] pub fn smpr1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x14) as *mut u32
   }

   #[doc="Read the SMPR1 register."]
   #[inline] pub fn smpr1(&self) -> Smpr1 { 
      unsafe {
         Smpr1(read_volatile((self.0 + 0x14) as *const u32))
      }
   }

   #[doc="Write the SMPR1 register."]
   #[inline] pub fn set_smpr1<F: FnOnce(Smpr1) -> Smpr1>(&self, f: F) -> &Self {
      let value = f(Smpr1(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the SMPR1 register."]
   #[inline] pub fn with_smpr1<F: FnOnce(Smpr1) -> Smpr1>(&self, f: F) -> &Self {
      let tmp = self.smpr1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the SMPR2 register."]
   #[inline] pub fn smpr2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x18) as *const u32
   }

   #[doc="Get the *mut pointer for the SMPR2 register."]
   #[inline] pub fn smpr2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x18) as *mut u32
   }

   #[doc="Read the SMPR2 register."]
   #[inline] pub fn smpr2(&self) -> Smpr2 { 
      unsafe {
         Smpr2(read_volatile((self.0 + 0x18) as *const u32))
      }
   }

   #[doc="Write the SMPR2 register."]
   #[inline] pub fn set_smpr2<F: FnOnce(Smpr2) -> Smpr2>(&self, f: F) -> &Self {
      let value = f(Smpr2(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the SMPR2 register."]
   #[inline] pub fn with_smpr2<F: FnOnce(Smpr2) -> Smpr2>(&self, f: F) -> &Self {
      let tmp = self.smpr2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the TR1 register."]
   #[inline] pub fn tr1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x20) as *const u32
   }

   #[doc="Get the *mut pointer for the TR1 register."]
   #[inline] pub fn tr1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x20) as *mut u32
   }

   #[doc="Read the TR1 register."]
   #[inline] pub fn tr1(&self) -> Tr1 { 
      unsafe {
         Tr1(read_volatile((self.0 + 0x20) as *const u32))
      }
   }

   #[doc="Write the TR1 register."]
   #[inline] pub fn set_tr1<F: FnOnce(Tr1) -> Tr1>(&self, f: F) -> &Self {
      let value = f(Tr1(0));
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the TR1 register."]
   #[inline] pub fn with_tr1<F: FnOnce(Tr1) -> Tr1>(&self, f: F) -> &Self {
      let tmp = self.tr1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the TR2 register."]
   #[inline] pub fn tr2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x24) as *const u32
   }

   #[doc="Get the *mut pointer for the TR2 register."]
   #[inline] pub fn tr2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x24) as *mut u32
   }

   #[doc="Read the TR2 register."]
   #[inline] pub fn tr2(&self) -> Tr2 { 
      unsafe {
         Tr2(read_volatile((self.0 + 0x24) as *const u32))
      }
   }

   #[doc="Write the TR2 register."]
   #[inline] pub fn set_tr2<F: FnOnce(Tr2) -> Tr2>(&self, f: F) -> &Self {
      let value = f(Tr2(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the TR2 register."]
   #[inline] pub fn with_tr2<F: FnOnce(Tr2) -> Tr2>(&self, f: F) -> &Self {
      let tmp = self.tr2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the TR3 register."]
   #[inline] pub fn tr3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x28) as *const u32
   }

   #[doc="Get the *mut pointer for the TR3 register."]
   #[inline] pub fn tr3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x28) as *mut u32
   }

   #[doc="Read the TR3 register."]
   #[inline] pub fn tr3(&self) -> Tr3 { 
      unsafe {
         Tr3(read_volatile((self.0 + 0x28) as *const u32))
      }
   }

   #[doc="Write the TR3 register."]
   #[inline] pub fn set_tr3<F: FnOnce(Tr3) -> Tr3>(&self, f: F) -> &Self {
      let value = f(Tr3(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the TR3 register."]
   #[inline] pub fn with_tr3<F: FnOnce(Tr3) -> Tr3>(&self, f: F) -> &Self {
      let tmp = self.tr3();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the SQR1 register."]
   #[inline] pub fn sqr1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x30) as *const u32
   }

   #[doc="Get the *mut pointer for the SQR1 register."]
   #[inline] pub fn sqr1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x30) as *mut u32
   }

   #[doc="Read the SQR1 register."]
   #[inline] pub fn sqr1(&self) -> Sqr1 { 
      unsafe {
         Sqr1(read_volatile((self.0 + 0x30) as *const u32))
      }
   }

   #[doc="Write the SQR1 register."]
   #[inline] pub fn set_sqr1<F: FnOnce(Sqr1) -> Sqr1>(&self, f: F) -> &Self {
      let value = f(Sqr1(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the SQR1 register."]
   #[inline] pub fn with_sqr1<F: FnOnce(Sqr1) -> Sqr1>(&self, f: F) -> &Self {
      let tmp = self.sqr1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the SQR2 register."]
   #[inline] pub fn sqr2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x34) as *const u32
   }

   #[doc="Get the *mut pointer for the SQR2 register."]
   #[inline] pub fn sqr2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x34) as *mut u32
   }

   #[doc="Read the SQR2 register."]
   #[inline] pub fn sqr2(&self) -> Sqr2 { 
      unsafe {
         Sqr2(read_volatile((self.0 + 0x34) as *const u32))
      }
   }

   #[doc="Write the SQR2 register."]
   #[inline] pub fn set_sqr2<F: FnOnce(Sqr2) -> Sqr2>(&self, f: F) -> &Self {
      let value = f(Sqr2(0));
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the SQR2 register."]
   #[inline] pub fn with_sqr2<F: FnOnce(Sqr2) -> Sqr2>(&self, f: F) -> &Self {
      let tmp = self.sqr2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the SQR3 register."]
   #[inline] pub fn sqr3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x38) as *const u32
   }

   #[doc="Get the *mut pointer for the SQR3 register."]
   #[inline] pub fn sqr3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x38) as *mut u32
   }

   #[doc="Read the SQR3 register."]
   #[inline] pub fn sqr3(&self) -> Sqr3 { 
      unsafe {
         Sqr3(read_volatile((self.0 + 0x38) as *const u32))
      }
   }

   #[doc="Write the SQR3 register."]
   #[inline] pub fn set_sqr3<F: FnOnce(Sqr3) -> Sqr3>(&self, f: F) -> &Self {
      let value = f(Sqr3(0));
      unsafe {
         write_volatile((self.0 + 0x38) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the SQR3 register."]
   #[inline] pub fn with_sqr3<F: FnOnce(Sqr3) -> Sqr3>(&self, f: F) -> &Self {
      let tmp = self.sqr3();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x38) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the SQR4 register."]
   #[inline] pub fn sqr4_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x3c) as *const u32
   }

   #[doc="Get the *mut pointer for the SQR4 register."]
   #[inline] pub fn sqr4_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x3c) as *mut u32
   }

   #[doc="Read the SQR4 register."]
   #[inline] pub fn sqr4(&self) -> Sqr4 { 
      unsafe {
         Sqr4(read_volatile((self.0 + 0x3c) as *const u32))
      }
   }

   #[doc="Write the SQR4 register."]
   #[inline] pub fn set_sqr4<F: FnOnce(Sqr4) -> Sqr4>(&self, f: F) -> &Self {
      let value = f(Sqr4(0));
      unsafe {
         write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the SQR4 register."]
   #[inline] pub fn with_sqr4<F: FnOnce(Sqr4) -> Sqr4>(&self, f: F) -> &Self {
      let tmp = self.sqr4();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR register."]
   #[inline] pub fn dr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x40) as *const u32
   }

   #[doc="Get the *mut pointer for the DR register."]
   #[inline] pub fn dr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x40) as *mut u32
   }

   #[doc="Read the DR register."]
   #[inline] pub fn dr(&self) -> Dr { 
      unsafe {
         Dr(read_volatile((self.0 + 0x40) as *const u32))
      }
   }

   #[doc="Get the *const pointer for the JSQR register."]
   #[inline] pub fn jsqr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4c) as *const u32
   }

   #[doc="Get the *mut pointer for the JSQR register."]
   #[inline] pub fn jsqr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4c) as *mut u32
   }

   #[doc="Read the JSQR register."]
   #[inline] pub fn jsqr(&self) -> Jsqr { 
      unsafe {
         Jsqr(read_volatile((self.0 + 0x4c) as *const u32))
      }
   }

   #[doc="Write the JSQR register."]
   #[inline] pub fn set_jsqr<F: FnOnce(Jsqr) -> Jsqr>(&self, f: F) -> &Self {
      let value = f(Jsqr(0));
      unsafe {
         write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the JSQR register."]
   #[inline] pub fn with_jsqr<F: FnOnce(Jsqr) -> Jsqr>(&self, f: F) -> &Self {
      let tmp = self.jsqr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the OFR1 register."]
   #[inline] pub fn ofr1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x60) as *const u32
   }

   #[doc="Get the *mut pointer for the OFR1 register."]
   #[inline] pub fn ofr1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x60) as *mut u32
   }

   #[doc="Read the OFR1 register."]
   #[inline] pub fn ofr1(&self) -> Ofr1 { 
      unsafe {
         Ofr1(read_volatile((self.0 + 0x60) as *const u32))
      }
   }

   #[doc="Write the OFR1 register."]
   #[inline] pub fn set_ofr1<F: FnOnce(Ofr1) -> Ofr1>(&self, f: F) -> &Self {
      let value = f(Ofr1(0));
      unsafe {
         write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the OFR1 register."]
   #[inline] pub fn with_ofr1<F: FnOnce(Ofr1) -> Ofr1>(&self, f: F) -> &Self {
      let tmp = self.ofr1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the OFR2 register."]
   #[inline] pub fn ofr2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x64) as *const u32
   }

   #[doc="Get the *mut pointer for the OFR2 register."]
   #[inline] pub fn ofr2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x64) as *mut u32
   }

   #[doc="Read the OFR2 register."]
   #[inline] pub fn ofr2(&self) -> Ofr2 { 
      unsafe {
         Ofr2(read_volatile((self.0 + 0x64) as *const u32))
      }
   }

   #[doc="Write the OFR2 register."]
   #[inline] pub fn set_ofr2<F: FnOnce(Ofr2) -> Ofr2>(&self, f: F) -> &Self {
      let value = f(Ofr2(0));
      unsafe {
         write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the OFR2 register."]
   #[inline] pub fn with_ofr2<F: FnOnce(Ofr2) -> Ofr2>(&self, f: F) -> &Self {
      let tmp = self.ofr2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the OFR3 register."]
   #[inline] pub fn ofr3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x68) as *const u32
   }

   #[doc="Get the *mut pointer for the OFR3 register."]
   #[inline] pub fn ofr3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x68) as *mut u32
   }

   #[doc="Read the OFR3 register."]
   #[inline] pub fn ofr3(&self) -> Ofr3 { 
      unsafe {
         Ofr3(read_volatile((self.0 + 0x68) as *const u32))
      }
   }

   #[doc="Write the OFR3 register."]
   #[inline] pub fn set_ofr3<F: FnOnce(Ofr3) -> Ofr3>(&self, f: F) -> &Self {
      let value = f(Ofr3(0));
      unsafe {
         write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the OFR3 register."]
   #[inline] pub fn with_ofr3<F: FnOnce(Ofr3) -> Ofr3>(&self, f: F) -> &Self {
      let tmp = self.ofr3();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the OFR4 register."]
   #[inline] pub fn ofr4_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x6c) as *const u32
   }

   #[doc="Get the *mut pointer for the OFR4 register."]
   #[inline] pub fn ofr4_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x6c) as *mut u32
   }

   #[doc="Read the OFR4 register."]
   #[inline] pub fn ofr4(&self) -> Ofr4 { 
      unsafe {
         Ofr4(read_volatile((self.0 + 0x6c) as *const u32))
      }
   }

   #[doc="Write the OFR4 register."]
   #[inline] pub fn set_ofr4<F: FnOnce(Ofr4) -> Ofr4>(&self, f: F) -> &Self {
      let value = f(Ofr4(0));
      unsafe {
         write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the OFR4 register."]
   #[inline] pub fn with_ofr4<F: FnOnce(Ofr4) -> Ofr4>(&self, f: F) -> &Self {
      let tmp = self.ofr4();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the JDR1 register."]
   #[inline] pub fn jdr1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x80) as *const u32
   }

   #[doc="Get the *mut pointer for the JDR1 register."]
   #[inline] pub fn jdr1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x80) as *mut u32
   }

   #[doc="Read the JDR1 register."]
   #[inline] pub fn jdr1(&self) -> Jdr1 { 
      unsafe {
         Jdr1(read_volatile((self.0 + 0x80) as *const u32))
      }
   }

   #[doc="Get the *const pointer for the JDR2 register."]
   #[inline] pub fn jdr2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x84) as *const u32
   }

   #[doc="Get the *mut pointer for the JDR2 register."]
   #[inline] pub fn jdr2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x84) as *mut u32
   }

   #[doc="Read the JDR2 register."]
   #[inline] pub fn jdr2(&self) -> Jdr2 { 
      unsafe {
         Jdr2(read_volatile((self.0 + 0x84) as *const u32))
      }
   }

   #[doc="Get the *const pointer for the JDR3 register."]
   #[inline] pub fn jdr3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x88) as *const u32
   }

   #[doc="Get the *mut pointer for the JDR3 register."]
   #[inline] pub fn jdr3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x88) as *mut u32
   }

   #[doc="Read the JDR3 register."]
   #[inline] pub fn jdr3(&self) -> Jdr3 { 
      unsafe {
         Jdr3(read_volatile((self.0 + 0x88) as *const u32))
      }
   }

   #[doc="Get the *const pointer for the JDR4 register."]
   #[inline] pub fn jdr4_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8c) as *const u32
   }

   #[doc="Get the *mut pointer for the JDR4 register."]
   #[inline] pub fn jdr4_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8c) as *mut u32
   }

   #[doc="Read the JDR4 register."]
   #[inline] pub fn jdr4(&self) -> Jdr4 { 
      unsafe {
         Jdr4(read_volatile((self.0 + 0x8c) as *const u32))
      }
   }

   #[doc="Get the *const pointer for the AWD2CR register."]
   #[inline] pub fn awd2cr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa0) as *const u32
   }

   #[doc="Get the *mut pointer for the AWD2CR register."]
   #[inline] pub fn awd2cr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa0) as *mut u32
   }

   #[doc="Read the AWD2CR register."]
   #[inline] pub fn awd2cr(&self) -> Awd2cr { 
      unsafe {
         Awd2cr(read_volatile((self.0 + 0xa0) as *const u32))
      }
   }

   #[doc="Write the AWD2CR register."]
   #[inline] pub fn set_awd2cr<F: FnOnce(Awd2cr) -> Awd2cr>(&self, f: F) -> &Self {
      let value = f(Awd2cr(0));
      unsafe {
         write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the AWD2CR register."]
   #[inline] pub fn with_awd2cr<F: FnOnce(Awd2cr) -> Awd2cr>(&self, f: F) -> &Self {
      let tmp = self.awd2cr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the AWD3CR register."]
   #[inline] pub fn awd3cr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa4) as *const u32
   }

   #[doc="Get the *mut pointer for the AWD3CR register."]
   #[inline] pub fn awd3cr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa4) as *mut u32
   }

   #[doc="Read the AWD3CR register."]
   #[inline] pub fn awd3cr(&self) -> Awd3cr { 
      unsafe {
         Awd3cr(read_volatile((self.0 + 0xa4) as *const u32))
      }
   }

   #[doc="Write the AWD3CR register."]
   #[inline] pub fn set_awd3cr<F: FnOnce(Awd3cr) -> Awd3cr>(&self, f: F) -> &Self {
      let value = f(Awd3cr(0));
      unsafe {
         write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the AWD3CR register."]
   #[inline] pub fn with_awd3cr<F: FnOnce(Awd3cr) -> Awd3cr>(&self, f: F) -> &Self {
      let tmp = self.awd3cr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DIFSEL register."]
   #[inline] pub fn difsel_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb0) as *const u32
   }

   #[doc="Get the *mut pointer for the DIFSEL register."]
   #[inline] pub fn difsel_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb0) as *mut u32
   }

   #[doc="Read the DIFSEL register."]
   #[inline] pub fn difsel(&self) -> Difsel { 
      unsafe {
         Difsel(read_volatile((self.0 + 0xb0) as *const u32))
      }
   }

   #[doc="Write the DIFSEL register."]
   #[inline] pub fn set_difsel<F: FnOnce(Difsel) -> Difsel>(&self, f: F) -> &Self {
      let value = f(Difsel(0));
      unsafe {
         write_volatile((self.0 + 0xb0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DIFSEL register."]
   #[inline] pub fn with_difsel<F: FnOnce(Difsel) -> Difsel>(&self, f: F) -> &Self {
      let tmp = self.difsel();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the CALFACT register."]
   #[inline] pub fn calfact_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb4) as *const u32
   }

   #[doc="Get the *mut pointer for the CALFACT register."]
   #[inline] pub fn calfact_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb4) as *mut u32
   }

   #[doc="Read the CALFACT register."]
   #[inline] pub fn calfact(&self) -> Calfact { 
      unsafe {
         Calfact(read_volatile((self.0 + 0xb4) as *const u32))
      }
   }

   #[doc="Write the CALFACT register."]
   #[inline] pub fn set_calfact<F: FnOnce(Calfact) -> Calfact>(&self, f: F) -> &Self {
      let value = f(Calfact(0));
      unsafe {
         write_volatile((self.0 + 0xb4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CALFACT register."]
   #[inline] pub fn with_calfact<F: FnOnce(Calfact) -> Calfact>(&self, f: F) -> &Self {
      let tmp = self.calfact();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb4) as *mut u32, value.0);
      }
      self
   }

}

#[doc="interrupt and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
   #[doc="JQOVF"]
   #[inline] pub fn jqovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }

   #[doc="JQOVF"]
   #[inline] pub fn test_jqovf(&self) -> bool {
      self.jqovf != 0
   }

   #[doc="JQOVF"]
   #[inline] pub fn set_jqovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

   #[doc="AWD3"]
   #[inline] pub fn awd3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="AWD3"]
   #[inline] pub fn test_awd3(&self) -> bool {
      self.awd3 != 0
   }

   #[doc="AWD3"]
   #[inline] pub fn set_awd3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="AWD2"]
   #[inline] pub fn awd2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="AWD2"]
   #[inline] pub fn test_awd2(&self) -> bool {
      self.awd2 != 0
   }

   #[doc="AWD2"]
   #[inline] pub fn set_awd2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="AWD1"]
   #[inline] pub fn awd1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="AWD1"]
   #[inline] pub fn test_awd1(&self) -> bool {
      self.awd1 != 0
   }

   #[doc="AWD1"]
   #[inline] pub fn set_awd1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="JEOS"]
   #[inline] pub fn jeos(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }

   #[doc="JEOS"]
   #[inline] pub fn test_jeos(&self) -> bool {
      self.jeos != 0
   }

   #[doc="JEOS"]
   #[inline] pub fn set_jeos<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="JEOC"]
   #[inline] pub fn jeoc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }

   #[doc="JEOC"]
   #[inline] pub fn test_jeoc(&self) -> bool {
      self.jeoc != 0
   }

   #[doc="JEOC"]
   #[inline] pub fn set_jeoc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

   #[doc="OVR"]
   #[inline] pub fn ovr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }

   #[doc="OVR"]
   #[inline] pub fn test_ovr(&self) -> bool {
      self.ovr != 0
   }

   #[doc="OVR"]
   #[inline] pub fn set_ovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="EOS"]
   #[inline] pub fn eos(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="EOS"]
   #[inline] pub fn test_eos(&self) -> bool {
      self.eos != 0
   }

   #[doc="EOS"]
   #[inline] pub fn set_eos<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="EOC"]
   #[inline] pub fn eoc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="EOC"]
   #[inline] pub fn test_eoc(&self) -> bool {
      self.eoc != 0
   }

   #[doc="EOC"]
   #[inline] pub fn set_eoc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="EOSMP"]
   #[inline] pub fn eosmp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="EOSMP"]
   #[inline] pub fn test_eosmp(&self) -> bool {
      self.eosmp != 0
   }

   #[doc="EOSMP"]
   #[inline] pub fn set_eosmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="ADRDY"]
   #[inline] pub fn adrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="ADRDY"]
   #[inline] pub fn test_adrdy(&self) -> bool {
      self.adrdy != 0
   }

   #[doc="ADRDY"]
   #[inline] pub fn set_adrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
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
      if self.jqovf() != 0 { try!(write!(f, " jqovf"))}
      if self.awd3() != 0 { try!(write!(f, " awd3"))}
      if self.awd2() != 0 { try!(write!(f, " awd2"))}
      if self.awd1() != 0 { try!(write!(f, " awd1"))}
      if self.jeos() != 0 { try!(write!(f, " jeos"))}
      if self.jeoc() != 0 { try!(write!(f, " jeoc"))}
      if self.ovr() != 0 { try!(write!(f, " ovr"))}
      if self.eos() != 0 { try!(write!(f, " eos"))}
      if self.eoc() != 0 { try!(write!(f, " eoc"))}
      if self.eosmp() != 0 { try!(write!(f, " eosmp"))}
      if self.adrdy() != 0 { try!(write!(f, " adrdy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
   #[doc="JQOVFIE"]
   #[inline] pub fn jqovfie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }

   #[doc="JQOVFIE"]
   #[inline] pub fn test_jqovfie(&self) -> bool {
      self.jqovfie != 0
   }

   #[doc="JQOVFIE"]
   #[inline] pub fn set_jqovfie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

   #[doc="AWD3IE"]
   #[inline] pub fn awd3ie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="AWD3IE"]
   #[inline] pub fn test_awd3ie(&self) -> bool {
      self.awd3ie != 0
   }

   #[doc="AWD3IE"]
   #[inline] pub fn set_awd3ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="AWD2IE"]
   #[inline] pub fn awd2ie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="AWD2IE"]
   #[inline] pub fn test_awd2ie(&self) -> bool {
      self.awd2ie != 0
   }

   #[doc="AWD2IE"]
   #[inline] pub fn set_awd2ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="AWD1IE"]
   #[inline] pub fn awd1ie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="AWD1IE"]
   #[inline] pub fn test_awd1ie(&self) -> bool {
      self.awd1ie != 0
   }

   #[doc="AWD1IE"]
   #[inline] pub fn set_awd1ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="JEOSIE"]
   #[inline] pub fn jeosie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }

   #[doc="JEOSIE"]
   #[inline] pub fn test_jeosie(&self) -> bool {
      self.jeosie != 0
   }

   #[doc="JEOSIE"]
   #[inline] pub fn set_jeosie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="JEOCIE"]
   #[inline] pub fn jeocie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }

   #[doc="JEOCIE"]
   #[inline] pub fn test_jeocie(&self) -> bool {
      self.jeocie != 0
   }

   #[doc="JEOCIE"]
   #[inline] pub fn set_jeocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

   #[doc="OVRIE"]
   #[inline] pub fn ovrie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }

   #[doc="OVRIE"]
   #[inline] pub fn test_ovrie(&self) -> bool {
      self.ovrie != 0
   }

   #[doc="OVRIE"]
   #[inline] pub fn set_ovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="EOSIE"]
   #[inline] pub fn eosie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="EOSIE"]
   #[inline] pub fn test_eosie(&self) -> bool {
      self.eosie != 0
   }

   #[doc="EOSIE"]
   #[inline] pub fn set_eosie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="EOCIE"]
   #[inline] pub fn eocie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="EOCIE"]
   #[inline] pub fn test_eocie(&self) -> bool {
      self.eocie != 0
   }

   #[doc="EOCIE"]
   #[inline] pub fn set_eocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="EOSMPIE"]
   #[inline] pub fn eosmpie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="EOSMPIE"]
   #[inline] pub fn test_eosmpie(&self) -> bool {
      self.eosmpie != 0
   }

   #[doc="EOSMPIE"]
   #[inline] pub fn set_eosmpie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="ADRDYIE"]
   #[inline] pub fn adrdyie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="ADRDYIE"]
   #[inline] pub fn test_adrdyie(&self) -> bool {
      self.adrdyie != 0
   }

   #[doc="ADRDYIE"]
   #[inline] pub fn set_adrdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
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
      if self.jqovfie() != 0 { try!(write!(f, " jqovfie"))}
      if self.awd3ie() != 0 { try!(write!(f, " awd3ie"))}
      if self.awd2ie() != 0 { try!(write!(f, " awd2ie"))}
      if self.awd1ie() != 0 { try!(write!(f, " awd1ie"))}
      if self.jeosie() != 0 { try!(write!(f, " jeosie"))}
      if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
      if self.ovrie() != 0 { try!(write!(f, " ovrie"))}
      if self.eosie() != 0 { try!(write!(f, " eosie"))}
      if self.eocie() != 0 { try!(write!(f, " eocie"))}
      if self.eosmpie() != 0 { try!(write!(f, " eosmpie"))}
      if self.adrdyie() != 0 { try!(write!(f, " adrdyie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
   #[doc="ADCAL"]
   #[inline] pub fn adcal(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }

   #[doc="ADCAL"]
   #[inline] pub fn test_adcal(&self) -> bool {
      self.adcal != 0
   }

   #[doc="ADCAL"]
   #[inline] pub fn set_adcal<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

   #[doc="ADCALDIF"]
   #[inline] pub fn adcaldif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }

   #[doc="ADCALDIF"]
   #[inline] pub fn test_adcaldif(&self) -> bool {
      self.adcaldif != 0
   }

   #[doc="ADCALDIF"]
   #[inline] pub fn set_adcaldif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

   #[doc="ADVREGEN"]
   #[inline] pub fn advregen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
   }

   #[doc="ADVREGEN"]
   #[inline] pub fn test_advregen(&self) -> bool {
      self.advregen != 0
   }

   #[doc="ADVREGEN"]
   #[inline] pub fn set_advregen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 28);
      self.0 |= value << 28;
      self
   }

   #[doc="JADSTP"]
   #[inline] pub fn jadstp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }

   #[doc="JADSTP"]
   #[inline] pub fn test_jadstp(&self) -> bool {
      self.jadstp != 0
   }

   #[doc="JADSTP"]
   #[inline] pub fn set_jadstp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

   #[doc="ADSTP"]
   #[inline] pub fn adstp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }

   #[doc="ADSTP"]
   #[inline] pub fn test_adstp(&self) -> bool {
      self.adstp != 0
   }

   #[doc="ADSTP"]
   #[inline] pub fn set_adstp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="JADSTART"]
   #[inline] pub fn jadstart(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="JADSTART"]
   #[inline] pub fn test_jadstart(&self) -> bool {
      self.jadstart != 0
   }

   #[doc="JADSTART"]
   #[inline] pub fn set_jadstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="ADSTART"]
   #[inline] pub fn adstart(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="ADSTART"]
   #[inline] pub fn test_adstart(&self) -> bool {
      self.adstart != 0
   }

   #[doc="ADSTART"]
   #[inline] pub fn set_adstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="ADDIS"]
   #[inline] pub fn addis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="ADDIS"]
   #[inline] pub fn test_addis(&self) -> bool {
      self.addis != 0
   }

   #[doc="ADDIS"]
   #[inline] pub fn set_addis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="ADEN"]
   #[inline] pub fn aden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="ADEN"]
   #[inline] pub fn test_aden(&self) -> bool {
      self.aden != 0
   }

   #[doc="ADEN"]
   #[inline] pub fn set_aden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
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
      if self.adcal() != 0 { try!(write!(f, " adcal"))}
      if self.adcaldif() != 0 { try!(write!(f, " adcaldif"))}
      if self.advregen() != 0 { try!(write!(f, " advregen=0x{:x}", self.advregen()))}
      if self.jadstp() != 0 { try!(write!(f, " jadstp"))}
      if self.adstp() != 0 { try!(write!(f, " adstp"))}
      if self.jadstart() != 0 { try!(write!(f, " jadstart"))}
      if self.adstart() != 0 { try!(write!(f, " adstart"))}
      if self.addis() != 0 { try!(write!(f, " addis"))}
      if self.aden() != 0 { try!(write!(f, " aden"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
   #[doc="AWDCH1CH"]
   #[inline] pub fn awdch1ch(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
   }

   #[doc="AWDCH1CH"]
   #[inline] pub fn test_awdch1ch(&self) -> bool {
      self.awdch1ch != 0
   }

   #[doc="AWDCH1CH"]
   #[inline] pub fn set_awdch1ch<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 26);
      self.0 |= value << 26;
      self
   }

   #[doc="JAUTO"]
   #[inline] pub fn jauto(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }

   #[doc="JAUTO"]
   #[inline] pub fn test_jauto(&self) -> bool {
      self.jauto != 0
   }

   #[doc="JAUTO"]
   #[inline] pub fn set_jauto<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

   #[doc="JAWD1EN"]
   #[inline] pub fn jawd1en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }

   #[doc="JAWD1EN"]
   #[inline] pub fn test_jawd1en(&self) -> bool {
      self.jawd1en != 0
   }

   #[doc="JAWD1EN"]
   #[inline] pub fn set_jawd1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="AWD1EN"]
   #[inline] pub fn awd1en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }

   #[doc="AWD1EN"]
   #[inline] pub fn test_awd1en(&self) -> bool {
      self.awd1en != 0
   }

   #[doc="AWD1EN"]
   #[inline] pub fn set_awd1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

   #[doc="AWD1SGL"]
   #[inline] pub fn awd1sgl(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }

   #[doc="AWD1SGL"]
   #[inline] pub fn test_awd1sgl(&self) -> bool {
      self.awd1sgl != 0
   }

   #[doc="AWD1SGL"]
   #[inline] pub fn set_awd1sgl<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

   #[doc="JQM"]
   #[inline] pub fn jqm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }

   #[doc="JQM"]
   #[inline] pub fn test_jqm(&self) -> bool {
      self.jqm != 0
   }

   #[doc="JQM"]
   #[inline] pub fn set_jqm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

   #[doc="JDISCEN"]
   #[inline] pub fn jdiscen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }

   #[doc="JDISCEN"]
   #[inline] pub fn test_jdiscen(&self) -> bool {
      self.jdiscen != 0
   }

   #[doc="JDISCEN"]
   #[inline] pub fn set_jdiscen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

   #[doc="DISCNUM"]
   #[inline] pub fn discnum(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
   }

   #[doc="DISCNUM"]
   #[inline] pub fn test_discnum(&self) -> bool {
      self.discnum != 0
   }

   #[doc="DISCNUM"]
   #[inline] pub fn set_discnum<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 17);
      self.0 |= value << 17;
      self
   }

   #[doc="DISCEN"]
   #[inline] pub fn discen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }

   #[doc="DISCEN"]
   #[inline] pub fn test_discen(&self) -> bool {
      self.discen != 0
   }

   #[doc="DISCEN"]
   #[inline] pub fn set_discen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="AUTOFF"]
   #[inline] pub fn autoff(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }

   #[doc="AUTOFF"]
   #[inline] pub fn test_autoff(&self) -> bool {
      self.autoff != 0
   }

   #[doc="AUTOFF"]
   #[inline] pub fn set_autoff<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

   #[doc="AUTDLY"]
   #[inline] pub fn autdly(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }

   #[doc="AUTDLY"]
   #[inline] pub fn test_autdly(&self) -> bool {
      self.autdly != 0
   }

   #[doc="AUTDLY"]
   #[inline] pub fn set_autdly<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

   #[doc="CONT"]
   #[inline] pub fn cont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }

   #[doc="CONT"]
   #[inline] pub fn test_cont(&self) -> bool {
      self.cont != 0
   }

   #[doc="CONT"]
   #[inline] pub fn set_cont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

   #[doc="OVRMOD"]
   #[inline] pub fn ovrmod(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }

   #[doc="OVRMOD"]
   #[inline] pub fn test_ovrmod(&self) -> bool {
      self.ovrmod != 0
   }

   #[doc="OVRMOD"]
   #[inline] pub fn set_ovrmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

   #[doc="EXTEN"]
   #[inline] pub fn exten(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
   }

   #[doc="EXTEN"]
   #[inline] pub fn test_exten(&self) -> bool {
      self.exten != 0
   }

   #[doc="EXTEN"]
   #[inline] pub fn set_exten<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 10);
      self.0 |= value << 10;
      self
   }

   #[doc="EXTSEL"]
   #[inline] pub fn extsel(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0xf) as u8) } // [9:6]
   }

   #[doc="EXTSEL"]
   #[inline] pub fn test_extsel(&self) -> bool {
      self.extsel != 0
   }

   #[doc="EXTSEL"]
   #[inline] pub fn set_extsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="ALIGN"]
   #[inline] pub fn align(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }

   #[doc="ALIGN"]
   #[inline] pub fn test_align(&self) -> bool {
      self.align != 0
   }

   #[doc="ALIGN"]
   #[inline] pub fn set_align<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

   #[doc="RES"]
   #[inline] pub fn res(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
   }

   #[doc="RES"]
   #[inline] pub fn test_res(&self) -> bool {
      self.res != 0
   }

   #[doc="RES"]
   #[inline] pub fn set_res<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="DMACFG"]
   #[inline] pub fn dmacfg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="DMACFG"]
   #[inline] pub fn test_dmacfg(&self) -> bool {
      self.dmacfg != 0
   }

   #[doc="DMACFG"]
   #[inline] pub fn set_dmacfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="DMAEN"]
   #[inline] pub fn dmaen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="DMAEN"]
   #[inline] pub fn test_dmaen(&self) -> bool {
      self.dmaen != 0
   }

   #[doc="DMAEN"]
   #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
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
      if self.awdch1ch() != 0 { try!(write!(f, " awdch1ch=0x{:x}", self.awdch1ch()))}
      if self.jauto() != 0 { try!(write!(f, " jauto"))}
      if self.jawd1en() != 0 { try!(write!(f, " jawd1en"))}
      if self.awd1en() != 0 { try!(write!(f, " awd1en"))}
      if self.awd1sgl() != 0 { try!(write!(f, " awd1sgl"))}
      if self.jqm() != 0 { try!(write!(f, " jqm"))}
      if self.jdiscen() != 0 { try!(write!(f, " jdiscen"))}
      if self.discnum() != 0 { try!(write!(f, " discnum=0x{:x}", self.discnum()))}
      if self.discen() != 0 { try!(write!(f, " discen"))}
      if self.autoff() != 0 { try!(write!(f, " autoff"))}
      if self.autdly() != 0 { try!(write!(f, " autdly"))}
      if self.cont() != 0 { try!(write!(f, " cont"))}
      if self.ovrmod() != 0 { try!(write!(f, " ovrmod"))}
      if self.exten() != 0 { try!(write!(f, " exten=0x{:x}", self.exten()))}
      if self.extsel() != 0 { try!(write!(f, " extsel=0x{:x}", self.extsel()))}
      if self.align() != 0 { try!(write!(f, " align"))}
      if self.res() != 0 { try!(write!(f, " res=0x{:x}", self.res()))}
      if self.dmacfg() != 0 { try!(write!(f, " dmacfg"))}
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="sample time register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smpr1(pub u32);
impl Smpr1 {
   #[doc="SMP9"]
   #[inline] pub fn smp9(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
   }

   #[doc="SMP9"]
   #[inline] pub fn test_smp9(&self) -> bool {
      self.smp9 != 0
   }

   #[doc="SMP9"]
   #[inline] pub fn set_smp9<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 27);
      self.0 |= value << 27;
      self
   }

   #[doc="SMP8"]
   #[inline] pub fn smp8(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }

   #[doc="SMP8"]
   #[inline] pub fn test_smp8(&self) -> bool {
      self.smp8 != 0
   }

   #[doc="SMP8"]
   #[inline] pub fn set_smp8<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="SMP7"]
   #[inline] pub fn smp7(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7) as u8) } // [23:21]
   }

   #[doc="SMP7"]
   #[inline] pub fn test_smp7(&self) -> bool {
      self.smp7 != 0
   }

   #[doc="SMP7"]
   #[inline] pub fn set_smp7<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 21);
      self.0 |= value << 21;
      self
   }

   #[doc="SMP6"]
   #[inline] pub fn smp6(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7) as u8) } // [20:18]
   }

   #[doc="SMP6"]
   #[inline] pub fn test_smp6(&self) -> bool {
      self.smp6 != 0
   }

   #[doc="SMP6"]
   #[inline] pub fn set_smp6<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 18);
      self.0 |= value << 18;
      self
   }

   #[doc="SMP5"]
   #[inline] pub fn smp5(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x7) as u8) } // [17:15]
   }

   #[doc="SMP5"]
   #[inline] pub fn test_smp5(&self) -> bool {
      self.smp5 != 0
   }

   #[doc="SMP5"]
   #[inline] pub fn set_smp5<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 15);
      self.0 |= value << 15;
      self
   }

   #[doc="SMP4"]
   #[inline] pub fn smp4(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
   }

   #[doc="SMP4"]
   #[inline] pub fn test_smp4(&self) -> bool {
      self.smp4 != 0
   }

   #[doc="SMP4"]
   #[inline] pub fn set_smp4<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 12);
      self.0 |= value << 12;
      self
   }

   #[doc="SMP3"]
   #[inline] pub fn smp3(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x7) as u8) } // [11:9]
   }

   #[doc="SMP3"]
   #[inline] pub fn test_smp3(&self) -> bool {
      self.smp3 != 0
   }

   #[doc="SMP3"]
   #[inline] pub fn set_smp3<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="SMP2"]
   #[inline] pub fn smp2(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
   }

   #[doc="SMP2"]
   #[inline] pub fn test_smp2(&self) -> bool {
      self.smp2 != 0
   }

   #[doc="SMP2"]
   #[inline] pub fn set_smp2<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="SMP1"]
   #[inline] pub fn smp1(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
   }

   #[doc="SMP1"]
   #[inline] pub fn test_smp1(&self) -> bool {
      self.smp1 != 0
   }

   #[doc="SMP1"]
   #[inline] pub fn set_smp1<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 3);
      self.0 |= value << 3;
      self
   }

}

impl ::core::fmt::Display for Smpr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Smpr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smp9() != 0 { try!(write!(f, " smp9=0x{:x}", self.smp9()))}
      if self.smp8() != 0 { try!(write!(f, " smp8=0x{:x}", self.smp8()))}
      if self.smp7() != 0 { try!(write!(f, " smp7=0x{:x}", self.smp7()))}
      if self.smp6() != 0 { try!(write!(f, " smp6=0x{:x}", self.smp6()))}
      if self.smp5() != 0 { try!(write!(f, " smp5=0x{:x}", self.smp5()))}
      if self.smp4() != 0 { try!(write!(f, " smp4=0x{:x}", self.smp4()))}
      if self.smp3() != 0 { try!(write!(f, " smp3=0x{:x}", self.smp3()))}
      if self.smp2() != 0 { try!(write!(f, " smp2=0x{:x}", self.smp2()))}
      if self.smp1() != 0 { try!(write!(f, " smp1=0x{:x}", self.smp1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="sample time register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smpr2(pub u32);
impl Smpr2 {
   #[doc="SMP18"]
   #[inline] pub fn smp18(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }

   #[doc="SMP18"]
   #[inline] pub fn test_smp18(&self) -> bool {
      self.smp18 != 0
   }

   #[doc="SMP18"]
   #[inline] pub fn set_smp18<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="SMP17"]
   #[inline] pub fn smp17(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7) as u8) } // [23:21]
   }

   #[doc="SMP17"]
   #[inline] pub fn test_smp17(&self) -> bool {
      self.smp17 != 0
   }

   #[doc="SMP17"]
   #[inline] pub fn set_smp17<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 21);
      self.0 |= value << 21;
      self
   }

   #[doc="SMP16"]
   #[inline] pub fn smp16(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7) as u8) } // [20:18]
   }

   #[doc="SMP16"]
   #[inline] pub fn test_smp16(&self) -> bool {
      self.smp16 != 0
   }

   #[doc="SMP16"]
   #[inline] pub fn set_smp16<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 18);
      self.0 |= value << 18;
      self
   }

   #[doc="SMP15"]
   #[inline] pub fn smp15(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x7) as u8) } // [17:15]
   }

   #[doc="SMP15"]
   #[inline] pub fn test_smp15(&self) -> bool {
      self.smp15 != 0
   }

   #[doc="SMP15"]
   #[inline] pub fn set_smp15<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 15);
      self.0 |= value << 15;
      self
   }

   #[doc="SMP14"]
   #[inline] pub fn smp14(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
   }

   #[doc="SMP14"]
   #[inline] pub fn test_smp14(&self) -> bool {
      self.smp14 != 0
   }

   #[doc="SMP14"]
   #[inline] pub fn set_smp14<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 12);
      self.0 |= value << 12;
      self
   }

   #[doc="SMP13"]
   #[inline] pub fn smp13(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x7) as u8) } // [11:9]
   }

   #[doc="SMP13"]
   #[inline] pub fn test_smp13(&self) -> bool {
      self.smp13 != 0
   }

   #[doc="SMP13"]
   #[inline] pub fn set_smp13<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="SMP12"]
   #[inline] pub fn smp12(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
   }

   #[doc="SMP12"]
   #[inline] pub fn test_smp12(&self) -> bool {
      self.smp12 != 0
   }

   #[doc="SMP12"]
   #[inline] pub fn set_smp12<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="SMP11"]
   #[inline] pub fn smp11(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
   }

   #[doc="SMP11"]
   #[inline] pub fn test_smp11(&self) -> bool {
      self.smp11 != 0
   }

   #[doc="SMP11"]
   #[inline] pub fn set_smp11<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="SMP10"]
   #[inline] pub fn smp10(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }

   #[doc="SMP10"]
   #[inline] pub fn test_smp10(&self) -> bool {
      self.smp10 != 0
   }

   #[doc="SMP10"]
   #[inline] pub fn set_smp10<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Smpr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Smpr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smp18() != 0 { try!(write!(f, " smp18=0x{:x}", self.smp18()))}
      if self.smp17() != 0 { try!(write!(f, " smp17=0x{:x}", self.smp17()))}
      if self.smp16() != 0 { try!(write!(f, " smp16=0x{:x}", self.smp16()))}
      if self.smp15() != 0 { try!(write!(f, " smp15=0x{:x}", self.smp15()))}
      if self.smp14() != 0 { try!(write!(f, " smp14=0x{:x}", self.smp14()))}
      if self.smp13() != 0 { try!(write!(f, " smp13=0x{:x}", self.smp13()))}
      if self.smp12() != 0 { try!(write!(f, " smp12=0x{:x}", self.smp12()))}
      if self.smp11() != 0 { try!(write!(f, " smp11=0x{:x}", self.smp11()))}
      if self.smp10() != 0 { try!(write!(f, " smp10=0x{:x}", self.smp10()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="watchdog threshold register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tr1(pub u32);
impl Tr1 {
   #[doc="HT1"]
   #[inline] pub fn ht1(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
   }

   #[doc="HT1"]
   #[inline] pub fn test_ht1(&self) -> bool {
      self.ht1 != 0
   }

   #[doc="HT1"]
   #[inline] pub fn set_ht1<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="LT1"]
   #[inline] pub fn lt1(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }

   #[doc="LT1"]
   #[inline] pub fn test_lt1(&self) -> bool {
      self.lt1 != 0
   }

   #[doc="LT1"]
   #[inline] pub fn set_lt1<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Tr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ht1() != 0 { try!(write!(f, " ht1=0x{:x}", self.ht1()))}
      if self.lt1() != 0 { try!(write!(f, " lt1=0x{:x}", self.lt1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="watchdog threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tr2(pub u32);
impl Tr2 {
   #[doc="HT2"]
   #[inline] pub fn ht2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }

   #[doc="HT2"]
   #[inline] pub fn test_ht2(&self) -> bool {
      self.ht2 != 0
   }

   #[doc="HT2"]
   #[inline] pub fn set_ht2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="LT2"]
   #[inline] pub fn lt2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="LT2"]
   #[inline] pub fn test_lt2(&self) -> bool {
      self.lt2 != 0
   }

   #[doc="LT2"]
   #[inline] pub fn set_lt2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Tr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ht2() != 0 { try!(write!(f, " ht2=0x{:x}", self.ht2()))}
      if self.lt2() != 0 { try!(write!(f, " lt2=0x{:x}", self.lt2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="watchdog threshold register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tr3(pub u32);
impl Tr3 {
   #[doc="HT3"]
   #[inline] pub fn ht3(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }

   #[doc="HT3"]
   #[inline] pub fn test_ht3(&self) -> bool {
      self.ht3 != 0
   }

   #[doc="HT3"]
   #[inline] pub fn set_ht3<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="LT3"]
   #[inline] pub fn lt3(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="LT3"]
   #[inline] pub fn test_lt3(&self) -> bool {
      self.lt3 != 0
   }

   #[doc="LT3"]
   #[inline] pub fn set_lt3<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Tr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ht3() != 0 { try!(write!(f, " ht3=0x{:x}", self.ht3()))}
      if self.lt3() != 0 { try!(write!(f, " lt3=0x{:x}", self.lt3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="regular sequence register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr1(pub u32);
impl Sqr1 {
   #[doc="SQ4"]
   #[inline] pub fn sq4(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
   }

   #[doc="SQ4"]
   #[inline] pub fn test_sq4(&self) -> bool {
      self.sq4 != 0
   }

   #[doc="SQ4"]
   #[inline] pub fn set_sq4<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="SQ3"]
   #[inline] pub fn sq3(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
   }

   #[doc="SQ3"]
   #[inline] pub fn test_sq3(&self) -> bool {
      self.sq3 != 0
   }

   #[doc="SQ3"]
   #[inline] pub fn set_sq3<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 18);
      self.0 |= value << 18;
      self
   }

   #[doc="SQ2"]
   #[inline] pub fn sq2(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
   }

   #[doc="SQ2"]
   #[inline] pub fn test_sq2(&self) -> bool {
      self.sq2 != 0
   }

   #[doc="SQ2"]
   #[inline] pub fn set_sq2<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 12);
      self.0 |= value << 12;
      self
   }

   #[doc="SQ1"]
   #[inline] pub fn sq1(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
   }

   #[doc="SQ1"]
   #[inline] pub fn test_sq1(&self) -> bool {
      self.sq1 != 0
   }

   #[doc="SQ1"]
   #[inline] pub fn set_sq1<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="L"]
   #[inline] pub fn l(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }

   #[doc="L"]
   #[inline] pub fn test_l(&self) -> bool {
      self.l != 0
   }

   #[doc="L"]
   #[inline] pub fn set_l<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Sqr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sqr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sq4() != 0 { try!(write!(f, " sq4=0x{:x}", self.sq4()))}
      if self.sq3() != 0 { try!(write!(f, " sq3=0x{:x}", self.sq3()))}
      if self.sq2() != 0 { try!(write!(f, " sq2=0x{:x}", self.sq2()))}
      if self.sq1() != 0 { try!(write!(f, " sq1=0x{:x}", self.sq1()))}
      if self.l() != 0 { try!(write!(f, " l=0x{:x}", self.l()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="regular sequence register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr2(pub u32);
impl Sqr2 {
   #[doc="SQ9"]
   #[inline] pub fn sq9(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
   }

   #[doc="SQ9"]
   #[inline] pub fn test_sq9(&self) -> bool {
      self.sq9 != 0
   }

   #[doc="SQ9"]
   #[inline] pub fn set_sq9<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="SQ8"]
   #[inline] pub fn sq8(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
   }

   #[doc="SQ8"]
   #[inline] pub fn test_sq8(&self) -> bool {
      self.sq8 != 0
   }

   #[doc="SQ8"]
   #[inline] pub fn set_sq8<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 18);
      self.0 |= value << 18;
      self
   }

   #[doc="SQ7"]
   #[inline] pub fn sq7(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
   }

   #[doc="SQ7"]
   #[inline] pub fn test_sq7(&self) -> bool {
      self.sq7 != 0
   }

   #[doc="SQ7"]
   #[inline] pub fn set_sq7<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 12);
      self.0 |= value << 12;
      self
   }

   #[doc="SQ6"]
   #[inline] pub fn sq6(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
   }

   #[doc="SQ6"]
   #[inline] pub fn test_sq6(&self) -> bool {
      self.sq6 != 0
   }

   #[doc="SQ6"]
   #[inline] pub fn set_sq6<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="SQ5"]
   #[inline] pub fn sq5(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }

   #[doc="SQ5"]
   #[inline] pub fn test_sq5(&self) -> bool {
      self.sq5 != 0
   }

   #[doc="SQ5"]
   #[inline] pub fn set_sq5<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Sqr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sqr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sq9() != 0 { try!(write!(f, " sq9=0x{:x}", self.sq9()))}
      if self.sq8() != 0 { try!(write!(f, " sq8=0x{:x}", self.sq8()))}
      if self.sq7() != 0 { try!(write!(f, " sq7=0x{:x}", self.sq7()))}
      if self.sq6() != 0 { try!(write!(f, " sq6=0x{:x}", self.sq6()))}
      if self.sq5() != 0 { try!(write!(f, " sq5=0x{:x}", self.sq5()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="regular sequence register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr3(pub u32);
impl Sqr3 {
   #[doc="SQ14"]
   #[inline] pub fn sq14(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
   }

   #[doc="SQ14"]
   #[inline] pub fn test_sq14(&self) -> bool {
      self.sq14 != 0
   }

   #[doc="SQ14"]
   #[inline] pub fn set_sq14<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="SQ13"]
   #[inline] pub fn sq13(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
   }

   #[doc="SQ13"]
   #[inline] pub fn test_sq13(&self) -> bool {
      self.sq13 != 0
   }

   #[doc="SQ13"]
   #[inline] pub fn set_sq13<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 18);
      self.0 |= value << 18;
      self
   }

   #[doc="SQ12"]
   #[inline] pub fn sq12(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
   }

   #[doc="SQ12"]
   #[inline] pub fn test_sq12(&self) -> bool {
      self.sq12 != 0
   }

   #[doc="SQ12"]
   #[inline] pub fn set_sq12<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 12);
      self.0 |= value << 12;
      self
   }

   #[doc="SQ11"]
   #[inline] pub fn sq11(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
   }

   #[doc="SQ11"]
   #[inline] pub fn test_sq11(&self) -> bool {
      self.sq11 != 0
   }

   #[doc="SQ11"]
   #[inline] pub fn set_sq11<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="SQ10"]
   #[inline] pub fn sq10(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }

   #[doc="SQ10"]
   #[inline] pub fn test_sq10(&self) -> bool {
      self.sq10 != 0
   }

   #[doc="SQ10"]
   #[inline] pub fn set_sq10<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Sqr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sqr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sq14() != 0 { try!(write!(f, " sq14=0x{:x}", self.sq14()))}
      if self.sq13() != 0 { try!(write!(f, " sq13=0x{:x}", self.sq13()))}
      if self.sq12() != 0 { try!(write!(f, " sq12=0x{:x}", self.sq12()))}
      if self.sq11() != 0 { try!(write!(f, " sq11=0x{:x}", self.sq11()))}
      if self.sq10() != 0 { try!(write!(f, " sq10=0x{:x}", self.sq10()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="regular sequence register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr4(pub u32);
impl Sqr4 {
   #[doc="SQ16"]
   #[inline] pub fn sq16(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
   }

   #[doc="SQ16"]
   #[inline] pub fn test_sq16(&self) -> bool {
      self.sq16 != 0
   }

   #[doc="SQ16"]
   #[inline] pub fn set_sq16<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="SQ15"]
   #[inline] pub fn sq15(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }

   #[doc="SQ15"]
   #[inline] pub fn test_sq15(&self) -> bool {
      self.sq15 != 0
   }

   #[doc="SQ15"]
   #[inline] pub fn set_sq15<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Sqr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sqr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sq16() != 0 { try!(write!(f, " sq16=0x{:x}", self.sq16()))}
      if self.sq15() != 0 { try!(write!(f, " sq15=0x{:x}", self.sq15()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
   #[doc="DATA"]
   #[inline] pub fn data(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="DATA"]
   #[inline] pub fn test_data(&self) -> bool {
      self.data != 0
   }

   #[doc="DATA"]
   #[inline] pub fn set_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="DATA (16 bit)"]
   #[inline] pub fn data_16(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="DATA (16 bit)"]
   #[inline] pub fn test_data_16(&self) -> bool {
      self.data_16 != 0
   }

   #[doc="DATA (16 bit)"]
   #[inline] pub fn set_data_16<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="DATA (12 bit)"]
   #[inline] pub fn data_12(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }

   #[doc="DATA (12 bit)"]
   #[inline] pub fn test_data_12(&self) -> bool {
      self.data_12 != 0
   }

   #[doc="DATA (12 bit)"]
   #[inline] pub fn set_data_12<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="DATA (10 bit)"]
   #[inline] pub fn data_10(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }

   #[doc="DATA (10 bit)"]
   #[inline] pub fn test_data_10(&self) -> bool {
      self.data_10 != 0
   }

   #[doc="DATA (10 bit)"]
   #[inline] pub fn set_data_10<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="DATA (8 bit)"]
   #[inline] pub fn data_8(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="DATA (8 bit)"]
   #[inline] pub fn test_data_8(&self) -> bool {
      self.data_8 != 0
   }

   #[doc="DATA (8 bit)"]
   #[inline] pub fn set_data_8<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="DATA (6 bit)"]
   #[inline] pub fn data_6(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }

   #[doc="DATA (6 bit)"]
   #[inline] pub fn test_data_6(&self) -> bool {
      self.data_6 != 0
   }

   #[doc="DATA (6 bit)"]
   #[inline] pub fn set_data_6<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      if self.data_16() != 0 { try!(write!(f, " data_16=0x{:x}", self.data_16()))}
      if self.data_12() != 0 { try!(write!(f, " data_12=0x{:x}", self.data_12()))}
      if self.data_10() != 0 { try!(write!(f, " data_10=0x{:x}", self.data_10()))}
      if self.data_8() != 0 { try!(write!(f, " data_8=0x{:x}", self.data_8()))}
      if self.data_6() != 0 { try!(write!(f, " data_6=0x{:x}", self.data_6()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="injected sequence register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jsqr(pub u32);
impl Jsqr {
   #[doc="JSQ4"]
   #[inline] pub fn jsq4(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
   }

   #[doc="JSQ4"]
   #[inline] pub fn test_jsq4(&self) -> bool {
      self.jsq4 != 0
   }

   #[doc="JSQ4"]
   #[inline] pub fn set_jsq4<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 26);
      self.0 |= value << 26;
      self
   }

   #[doc="JSQ3"]
   #[inline] pub fn jsq3(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1f) as u8) } // [24:20]
   }

   #[doc="JSQ3"]
   #[inline] pub fn test_jsq3(&self) -> bool {
      self.jsq3 != 0
   }

   #[doc="JSQ3"]
   #[inline] pub fn set_jsq3<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 20);
      self.0 |= value << 20;
      self
   }

   #[doc="JSQ2"]
   #[inline] pub fn jsq2(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1f) as u8) } // [18:14]
   }

   #[doc="JSQ2"]
   #[inline] pub fn test_jsq2(&self) -> bool {
      self.jsq2 != 0
   }

   #[doc="JSQ2"]
   #[inline] pub fn set_jsq2<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 14);
      self.0 |= value << 14;
      self
   }

   #[doc="JSQ1"]
   #[inline] pub fn jsq1(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
   }

   #[doc="JSQ1"]
   #[inline] pub fn test_jsq1(&self) -> bool {
      self.jsq1 != 0
   }

   #[doc="JSQ1"]
   #[inline] pub fn set_jsq1<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="JEXTEN"]
   #[inline] pub fn jexten(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }

   #[doc="JEXTEN"]
   #[inline] pub fn test_jexten(&self) -> bool {
      self.jexten != 0
   }

   #[doc="JEXTEN"]
   #[inline] pub fn set_jexten<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="JEXTSEL"]
   #[inline] pub fn jextsel(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0xf) as u8) } // [5:2]
   }

   #[doc="JEXTSEL"]
   #[inline] pub fn test_jextsel(&self) -> bool {
      self.jextsel != 0
   }

   #[doc="JEXTSEL"]
   #[inline] pub fn set_jextsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="JL"]
   #[inline] pub fn jl(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }

   #[doc="JL"]
   #[inline] pub fn test_jl(&self) -> bool {
      self.jl != 0
   }

   #[doc="JL"]
   #[inline] pub fn set_jl<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Jsqr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jsqr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jsq4() != 0 { try!(write!(f, " jsq4=0x{:x}", self.jsq4()))}
      if self.jsq3() != 0 { try!(write!(f, " jsq3=0x{:x}", self.jsq3()))}
      if self.jsq2() != 0 { try!(write!(f, " jsq2=0x{:x}", self.jsq2()))}
      if self.jsq1() != 0 { try!(write!(f, " jsq1=0x{:x}", self.jsq1()))}
      if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
      if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
      if self.jl() != 0 { try!(write!(f, " jl=0x{:x}", self.jl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="offset register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofr1(pub u32);
impl Ofr1 {
   #[doc="OFFSET1_EN"]
   #[inline] pub fn offset1_en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }

   #[doc="OFFSET1_EN"]
   #[inline] pub fn test_offset1_en(&self) -> bool {
      self.offset1_en != 0
   }

   #[doc="OFFSET1_EN"]
   #[inline] pub fn set_offset1_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

   #[doc="OFFSET1_CH"]
   #[inline] pub fn offset1_ch(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
   }

   #[doc="OFFSET1_CH"]
   #[inline] pub fn test_offset1_ch(&self) -> bool {
      self.offset1_ch != 0
   }

   #[doc="OFFSET1_CH"]
   #[inline] pub fn set_offset1_ch<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 26);
      self.0 |= value << 26;
      self
   }

   #[doc="OFFSET1"]
   #[inline] pub fn offset1(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }

   #[doc="OFFSET1"]
   #[inline] pub fn test_offset1(&self) -> bool {
      self.offset1 != 0
   }

   #[doc="OFFSET1"]
   #[inline] pub fn set_offset1<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Ofr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ofr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offset1_en() != 0 { try!(write!(f, " offset1_en"))}
      if self.offset1_ch() != 0 { try!(write!(f, " offset1_ch=0x{:x}", self.offset1_ch()))}
      if self.offset1() != 0 { try!(write!(f, " offset1=0x{:x}", self.offset1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="offset register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofr2(pub u32);
impl Ofr2 {
   #[doc="OFFSET2_EN"]
   #[inline] pub fn offset2_en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }

   #[doc="OFFSET2_EN"]
   #[inline] pub fn test_offset2_en(&self) -> bool {
      self.offset2_en != 0
   }

   #[doc="OFFSET2_EN"]
   #[inline] pub fn set_offset2_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

   #[doc="OFFSET2_CH"]
   #[inline] pub fn offset2_ch(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
   }

   #[doc="OFFSET2_CH"]
   #[inline] pub fn test_offset2_ch(&self) -> bool {
      self.offset2_ch != 0
   }

   #[doc="OFFSET2_CH"]
   #[inline] pub fn set_offset2_ch<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 26);
      self.0 |= value << 26;
      self
   }

   #[doc="OFFSET2"]
   #[inline] pub fn offset2(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }

   #[doc="OFFSET2"]
   #[inline] pub fn test_offset2(&self) -> bool {
      self.offset2 != 0
   }

   #[doc="OFFSET2"]
   #[inline] pub fn set_offset2<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Ofr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ofr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offset2_en() != 0 { try!(write!(f, " offset2_en"))}
      if self.offset2_ch() != 0 { try!(write!(f, " offset2_ch=0x{:x}", self.offset2_ch()))}
      if self.offset2() != 0 { try!(write!(f, " offset2=0x{:x}", self.offset2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="offset register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofr3(pub u32);
impl Ofr3 {
   #[doc="OFFSET3_EN"]
   #[inline] pub fn offset3_en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }

   #[doc="OFFSET3_EN"]
   #[inline] pub fn test_offset3_en(&self) -> bool {
      self.offset3_en != 0
   }

   #[doc="OFFSET3_EN"]
   #[inline] pub fn set_offset3_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

   #[doc="OFFSET3_CH"]
   #[inline] pub fn offset3_ch(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
   }

   #[doc="OFFSET3_CH"]
   #[inline] pub fn test_offset3_ch(&self) -> bool {
      self.offset3_ch != 0
   }

   #[doc="OFFSET3_CH"]
   #[inline] pub fn set_offset3_ch<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 26);
      self.0 |= value << 26;
      self
   }

   #[doc="OFFSET3"]
   #[inline] pub fn offset3(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }

   #[doc="OFFSET3"]
   #[inline] pub fn test_offset3(&self) -> bool {
      self.offset3 != 0
   }

   #[doc="OFFSET3"]
   #[inline] pub fn set_offset3<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Ofr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ofr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offset3_en() != 0 { try!(write!(f, " offset3_en"))}
      if self.offset3_ch() != 0 { try!(write!(f, " offset3_ch=0x{:x}", self.offset3_ch()))}
      if self.offset3() != 0 { try!(write!(f, " offset3=0x{:x}", self.offset3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="offset register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofr4(pub u32);
impl Ofr4 {
   #[doc="OFFSET4_EN"]
   #[inline] pub fn offset4_en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }

   #[doc="OFFSET4_EN"]
   #[inline] pub fn test_offset4_en(&self) -> bool {
      self.offset4_en != 0
   }

   #[doc="OFFSET4_EN"]
   #[inline] pub fn set_offset4_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

   #[doc="OFFSET4_CH"]
   #[inline] pub fn offset4_ch(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
   }

   #[doc="OFFSET4_CH"]
   #[inline] pub fn test_offset4_ch(&self) -> bool {
      self.offset4_ch != 0
   }

   #[doc="OFFSET4_CH"]
   #[inline] pub fn set_offset4_ch<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 26);
      self.0 |= value << 26;
      self
   }

   #[doc="OFFSET4"]
   #[inline] pub fn offset4(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }

   #[doc="OFFSET4"]
   #[inline] pub fn test_offset4(&self) -> bool {
      self.offset4 != 0
   }

   #[doc="OFFSET4"]
   #[inline] pub fn set_offset4<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Ofr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ofr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.offset4_en() != 0 { try!(write!(f, " offset4_en"))}
      if self.offset4_ch() != 0 { try!(write!(f, " offset4_ch=0x{:x}", self.offset4_ch()))}
      if self.offset4() != 0 { try!(write!(f, " offset4=0x{:x}", self.offset4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="injected data register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr1(pub u32);
impl Jdr1 {
   #[doc="JDATA1"]
   #[inline] pub fn jdata1(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="JDATA1"]
   #[inline] pub fn test_jdata1(&self) -> bool {
      self.jdata1 != 0
   }

   #[doc="JDATA1"]
   #[inline] pub fn set_jdata1<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Jdr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jdr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jdata1() != 0 { try!(write!(f, " jdata1=0x{:x}", self.jdata1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="injected data register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr2(pub u32);
impl Jdr2 {
   #[doc="JDATA2"]
   #[inline] pub fn jdata2(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="JDATA2"]
   #[inline] pub fn test_jdata2(&self) -> bool {
      self.jdata2 != 0
   }

   #[doc="JDATA2"]
   #[inline] pub fn set_jdata2<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Jdr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jdr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jdata2() != 0 { try!(write!(f, " jdata2=0x{:x}", self.jdata2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="injected data register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr3(pub u32);
impl Jdr3 {
   #[doc="JDATA3"]
   #[inline] pub fn jdata3(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="JDATA3"]
   #[inline] pub fn test_jdata3(&self) -> bool {
      self.jdata3 != 0
   }

   #[doc="JDATA3"]
   #[inline] pub fn set_jdata3<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Jdr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jdr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jdata3() != 0 { try!(write!(f, " jdata3=0x{:x}", self.jdata3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="injected data register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr4(pub u32);
impl Jdr4 {
   #[doc="JDATA4"]
   #[inline] pub fn jdata4(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="JDATA4"]
   #[inline] pub fn test_jdata4(&self) -> bool {
      self.jdata4 != 0
   }

   #[doc="JDATA4"]
   #[inline] pub fn set_jdata4<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Jdr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Jdr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.jdata4() != 0 { try!(write!(f, " jdata4=0x{:x}", self.jdata4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Analog Watchdog 2 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awd2cr(pub u32);
impl Awd2cr {
   #[doc="AWD2CH"]
   #[inline] pub fn awd2ch(&self) -> bits::U18 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3ffff) as u32) } // [18:1]
   }

   #[doc="AWD2CH"]
   #[inline] pub fn test_awd2ch(&self) -> bool {
      self.awd2ch != 0
   }

   #[doc="AWD2CH"]
   #[inline] pub fn set_awd2ch<V: Into<bits::U18>>(mut self, value: V) -> Self {
      let value: bits::U18 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ffff << 1);
      self.0 |= value << 1;
      self
   }

}

impl ::core::fmt::Display for Awd2cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Awd2cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.awd2ch() != 0 { try!(write!(f, " awd2ch=0x{:x}", self.awd2ch()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Analog Watchdog 3 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awd3cr(pub u32);
impl Awd3cr {
   #[doc="AWD3CH"]
   #[inline] pub fn awd3ch(&self) -> bits::U18 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3ffff) as u32) } // [18:1]
   }

   #[doc="AWD3CH"]
   #[inline] pub fn test_awd3ch(&self) -> bool {
      self.awd3ch != 0
   }

   #[doc="AWD3CH"]
   #[inline] pub fn set_awd3ch<V: Into<bits::U18>>(mut self, value: V) -> Self {
      let value: bits::U18 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ffff << 1);
      self.0 |= value << 1;
      self
   }

}

impl ::core::fmt::Display for Awd3cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Awd3cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.awd3ch() != 0 { try!(write!(f, " awd3ch=0x{:x}", self.awd3ch()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Differential Mode Selection Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Difsel(pub u32);
impl Difsel {
   #[doc="Differential mode for channels 15 to 1"]
   #[inline] pub fn difsel_1_15(&self) -> bits::U15 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
   }

   #[doc="Differential mode for channels 15 to 1"]
   #[inline] pub fn test_difsel_1_15(&self) -> bool {
      self.difsel_1_15 != 0
   }

   #[doc="Differential mode for channels 15 to 1"]
   #[inline] pub fn set_difsel_1_15<V: Into<bits::U15>>(mut self, value: V) -> Self {
      let value: bits::U15 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7fff << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Differential mode for channels 18 to 16"]
   #[inline] pub fn difsel_16_18(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
   }

   #[doc="Differential mode for channels 18 to 16"]
   #[inline] pub fn test_difsel_16_18(&self) -> bool {
      self.difsel_16_18 != 0
   }

   #[doc="Differential mode for channels 18 to 16"]
   #[inline] pub fn set_difsel_16_18<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 16);
      self.0 |= value << 16;
      self
   }

}

impl ::core::fmt::Display for Difsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Difsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.difsel_1_15() != 0 { try!(write!(f, " difsel_1_15=0x{:x}", self.difsel_1_15()))}
      if self.difsel_16_18() != 0 { try!(write!(f, " difsel_16_18=0x{:x}", self.difsel_16_18()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Calibration Factors"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calfact(pub u32);
impl Calfact {
   #[doc="CALFACT_D"]
   #[inline] pub fn calfact_d(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
   }

   #[doc="CALFACT_D"]
   #[inline] pub fn test_calfact_d(&self) -> bool {
      self.calfact_d != 0
   }

   #[doc="CALFACT_D"]
   #[inline] pub fn set_calfact_d<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="CALFACT_S"]
   #[inline] pub fn calfact_s(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }

   #[doc="CALFACT_S"]
   #[inline] pub fn test_calfact_s(&self) -> bool {
      self.calfact_s != 0
   }

   #[doc="CALFACT_S"]
   #[inline] pub fn set_calfact_s<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Calfact {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Calfact {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.calfact_d() != 0 { try!(write!(f, " calfact_d=0x{:x}", self.calfact_d()))}
      if self.calfact_s() != 0 { try!(write!(f, " calfact_s=0x{:x}", self.calfact_s()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }

