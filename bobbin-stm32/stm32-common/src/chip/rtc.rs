#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="RTC Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the TR register."]
  #[inline] pub fn tr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the TR register."]
  #[inline] pub fn tr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the TR register."]
  #[inline] pub fn tr(&self) -> Tr { 
     unsafe {
        Tr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the TR register."]
  #[inline] pub fn set_tr<F: FnOnce(Tr) -> Tr>(&self, f: F) -> &Self {
     let value = f(Tr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TR register."]
  #[inline] pub fn with_tr<F: FnOnce(Tr) -> Tr>(&self, f: F) -> &Self {
     let tmp = self.tr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DR register."]
  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the DR register."]
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the DR register."]
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the DR register."]
  #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let value = f(Dr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DR register."]
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
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
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let value = f(Cr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ISR register."]
  #[inline] pub fn isr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the ISR register."]
  #[inline] pub fn isr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the ISR register."]
  #[inline] pub fn isr(&self) -> Isr { 
     unsafe {
        Isr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the ISR register."]
  #[inline] pub fn set_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
     let value = f(Isr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ISR register."]
  #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
     let tmp = self.isr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PRER register."]
  #[inline] pub fn prer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the PRER register."]
  #[inline] pub fn prer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the PRER register."]
  #[inline] pub fn prer(&self) -> Prer { 
     unsafe {
        Prer(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the PRER register."]
  #[inline] pub fn set_prer<F: FnOnce(Prer) -> Prer>(&self, f: F) -> &Self {
     let value = f(Prer(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PRER register."]
  #[inline] pub fn with_prer<F: FnOnce(Prer) -> Prer>(&self, f: F) -> &Self {
     let tmp = self.prer();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the WUTR register."]
  #[inline] pub fn wutr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the WUTR register."]
  #[inline] pub fn wutr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the WUTR register."]
  #[inline] pub fn wutr(&self) -> Wutr { 
     unsafe {
        Wutr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the WUTR register."]
  #[inline] pub fn set_wutr<F: FnOnce(Wutr) -> Wutr>(&self, f: F) -> &Self {
     let value = f(Wutr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the WUTR register."]
  #[inline] pub fn with_wutr<F: FnOnce(Wutr) -> Wutr>(&self, f: F) -> &Self {
     let tmp = self.wutr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ALRMAR register."]
  #[inline] pub fn alrmar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the ALRMAR register."]
  #[inline] pub fn alrmar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the ALRMAR register."]
  #[inline] pub fn alrmar(&self) -> Alrmar { 
     unsafe {
        Alrmar(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the ALRMAR register."]
  #[inline] pub fn set_alrmar<F: FnOnce(Alrmar) -> Alrmar>(&self, f: F) -> &Self {
     let value = f(Alrmar(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALRMAR register."]
  #[inline] pub fn with_alrmar<F: FnOnce(Alrmar) -> Alrmar>(&self, f: F) -> &Self {
     let tmp = self.alrmar();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ALRMBR register."]
  #[inline] pub fn alrmbr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the ALRMBR register."]
  #[inline] pub fn alrmbr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the ALRMBR register."]
  #[inline] pub fn alrmbr(&self) -> Alrmbr { 
     unsafe {
        Alrmbr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the ALRMBR register."]
  #[inline] pub fn set_alrmbr<F: FnOnce(Alrmbr) -> Alrmbr>(&self, f: F) -> &Self {
     let value = f(Alrmbr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALRMBR register."]
  #[inline] pub fn with_alrmbr<F: FnOnce(Alrmbr) -> Alrmbr>(&self, f: F) -> &Self {
     let tmp = self.alrmbr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the WPR register."]
  #[inline] pub fn wpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the WPR register."]
  #[inline] pub fn wpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Write the WPR register."]
  #[inline] pub fn set_wpr<F: FnOnce(Wpr) -> Wpr>(&self, f: F) -> &Self {
     let value = f(Wpr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SSR register."]
  #[inline] pub fn ssr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the SSR register."]
  #[inline] pub fn ssr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the SSR register."]
  #[inline] pub fn ssr(&self) -> Ssr { 
     unsafe {
        Ssr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }

#[doc="Get the *const pointer for the SHIFTR register."]
  #[inline] pub fn shiftr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the SHIFTR register."]
  #[inline] pub fn shiftr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Write the SHIFTR register."]
  #[inline] pub fn set_shiftr<F: FnOnce(Shiftr) -> Shiftr>(&self, f: F) -> &Self {
     let value = f(Shiftr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the TSTR register."]
  #[inline] pub fn tstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the TSTR register."]
  #[inline] pub fn tstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the TSTR register."]
  #[inline] pub fn tstr(&self) -> Tstr { 
     unsafe {
        Tstr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TSDR register."]
  #[inline] pub fn tsdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the TSDR register."]
  #[inline] pub fn tsdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Read the TSDR register."]
  #[inline] pub fn tsdr(&self) -> Tsdr { 
     unsafe {
        Tsdr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TSSSR register."]
  #[inline] pub fn tsssr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
#[doc="Get the *mut pointer for the TSSSR register."]
  #[inline] pub fn tsssr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
#[doc="Read the TSSSR register."]
  #[inline] pub fn tsssr(&self) -> Tsssr { 
     unsafe {
        Tsssr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CALR register."]
  #[inline] pub fn calr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
#[doc="Get the *mut pointer for the CALR register."]
  #[inline] pub fn calr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
#[doc="Read the CALR register."]
  #[inline] pub fn calr(&self) -> Calr { 
     unsafe {
        Calr(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
#[doc="Write the CALR register."]
  #[inline] pub fn set_calr<F: FnOnce(Calr) -> Calr>(&self, f: F) -> &Self {
     let value = f(Calr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CALR register."]
  #[inline] pub fn with_calr<F: FnOnce(Calr) -> Calr>(&self, f: F) -> &Self {
     let tmp = self.calr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the TAMPCR register."]
  #[inline] pub fn tampcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the TAMPCR register."]
  #[inline] pub fn tampcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the TAMPCR register."]
  #[inline] pub fn tampcr(&self) -> Tampcr { 
     unsafe {
        Tampcr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the TAMPCR register."]
  #[inline] pub fn set_tampcr<F: FnOnce(Tampcr) -> Tampcr>(&self, f: F) -> &Self {
     let value = f(Tampcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TAMPCR register."]
  #[inline] pub fn with_tampcr<F: FnOnce(Tampcr) -> Tampcr>(&self, f: F) -> &Self {
     let tmp = self.tampcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ALRMASSR register."]
  #[inline] pub fn alrmassr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the ALRMASSR register."]
  #[inline] pub fn alrmassr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the ALRMASSR register."]
  #[inline] pub fn alrmassr(&self) -> Alrmassr { 
     unsafe {
        Alrmassr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the ALRMASSR register."]
  #[inline] pub fn set_alrmassr<F: FnOnce(Alrmassr) -> Alrmassr>(&self, f: F) -> &Self {
     let value = f(Alrmassr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALRMASSR register."]
  #[inline] pub fn with_alrmassr<F: FnOnce(Alrmassr) -> Alrmassr>(&self, f: F) -> &Self {
     let tmp = self.alrmassr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ALRMBSSR register."]
  #[inline] pub fn alrmbssr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
#[doc="Get the *mut pointer for the ALRMBSSR register."]
  #[inline] pub fn alrmbssr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
#[doc="Read the ALRMBSSR register."]
  #[inline] pub fn alrmbssr(&self) -> Alrmbssr { 
     unsafe {
        Alrmbssr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
#[doc="Write the ALRMBSSR register."]
  #[inline] pub fn set_alrmbssr<F: FnOnce(Alrmbssr) -> Alrmbssr>(&self, f: F) -> &Self {
     let value = f(Alrmbssr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALRMBSSR register."]
  #[inline] pub fn with_alrmbssr<F: FnOnce(Alrmbssr) -> Alrmbssr>(&self, f: F) -> &Self {
     let tmp = self.alrmbssr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OR register."]
  #[inline] pub fn or_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
#[doc="Get the *mut pointer for the OR register."]
  #[inline] pub fn or_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
#[doc="Read the OR register."]
  #[inline] pub fn or(&self) -> Or { 
     unsafe {
        Or(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
#[doc="Write the OR register."]
  #[inline] pub fn set_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &Self {
     let value = f(Or(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OR register."]
  #[inline] pub fn with_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &Self {
     let tmp = self.or();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the BKPR register."]
  #[inline] pub fn bkpr_ptr<I: Into<bits::R5>>(&self, index: I) -> *const u32 { 
     let index: bits::R5 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x50 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the BKPR register."]
  #[inline] pub fn bkpr_mut<I: Into<bits::R5>>(&self, index: I) -> *mut u32 { 
     let index: bits::R5 = index.into();
     let index: usize = index.value();
     ((self.0 as usize) + 0x50 + (index << 2)) as *mut u32
  }
#[doc="Read the BKPR register."]
  #[inline] pub fn bkpr<I: Into<bits::R5>>(&self, index: I) -> Bkpr { 
     let index: bits::R5 = index.into();
     let index: usize = index.value();
     unsafe {
        Bkpr(::core::ptr::read_volatile(((self.0 as usize) + 0x50 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the BKPR register."]
  #[inline] pub fn set_bkpr<I: Into<bits::R5>, F: FnOnce(Bkpr) -> Bkpr>(&self, index: I, f: F) -> &Self {
     let index: bits::R5 = index.into();
     let index: usize = index.value();
     let value = f(Bkpr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BKPR register."]
  #[inline] pub fn with_bkpr<I: Into<bits::R5> + Copy, F: FnOnce(Bkpr) -> Bkpr>(&self, index: I, f: F) -> &Self {
     let tmp = self.bkpr(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }

}

#[doc="RTC time register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tr(pub u32);
impl Tr {
#[doc="AM/PM notation"]
  #[inline] pub fn pm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="AM/PM notation"]
  #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Hour tens in BCD format"]
  #[inline] pub fn ht(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
  }
#[doc="Hour tens in BCD format"]
  #[inline] pub fn set_ht<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Hour units in BCD format"]
  #[inline] pub fn hu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="Hour units in BCD format"]
  #[inline] pub fn set_hu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Minute tens in BCD format"]
  #[inline] pub fn mnt(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
  }
#[doc="Minute tens in BCD format"]
  #[inline] pub fn set_mnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Minute units in BCD format"]
  #[inline] pub fn mnu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="Minute units in BCD format"]
  #[inline] pub fn set_mnu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Second tens in BCD format"]
  #[inline] pub fn st(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
  }
#[doc="Second tens in BCD format"]
  #[inline] pub fn set_st<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Second units in BCD format"]
  #[inline] pub fn su(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="Second units in BCD format"]
  #[inline] pub fn set_su<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC date register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="Year tens in BCD format"]
  #[inline] pub fn yt(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
  }
#[doc="Year tens in BCD format"]
  #[inline] pub fn set_yt<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Year units in BCD format"]
  #[inline] pub fn yu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="Year units in BCD format"]
  #[inline] pub fn set_yu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Week day units"]
  #[inline] pub fn wdu(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
  }
#[doc="Week day units"]
  #[inline] pub fn set_wdu<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Month tens in BCD format"]
  #[inline] pub fn mt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Month tens in BCD format"]
  #[inline] pub fn set_mt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Month units in BCD format"]
  #[inline] pub fn mu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="Month units in BCD format"]
  #[inline] pub fn set_mu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Date tens in BCD format"]
  #[inline] pub fn dt(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="Date tens in BCD format"]
  #[inline] pub fn set_dt<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Date units in BCD format"]
  #[inline] pub fn du(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="Date units in BCD format"]
  #[inline] pub fn set_du<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
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
      if self.yt() != 0 { try!(write!(f, " yt=0x{:x}", self.yt()))}
      if self.yu() != 0 { try!(write!(f, " yu=0x{:x}", self.yu()))}
      if self.wdu() != 0 { try!(write!(f, " wdu=0x{:x}", self.wdu()))}
      if self.mt() != 0 { try!(write!(f, " mt"))}
      if self.mu() != 0 { try!(write!(f, " mu=0x{:x}", self.mu()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Calibration output enable"]
  #[inline] pub fn coe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="Calibration output enable"]
  #[inline] pub fn set_coe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Output selection"]
  #[inline] pub fn osel(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
  }
#[doc="Output selection"]
  #[inline] pub fn set_osel<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Output polarity"]
  #[inline] pub fn pol(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="Output polarity"]
  #[inline] pub fn set_pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Calibration output selection"]
  #[inline] pub fn cosel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Calibration output selection"]
  #[inline] pub fn set_cosel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Backup"]
  #[inline] pub fn bkp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Backup"]
  #[inline] pub fn set_bkp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Subtract 1 hour (winter time change)"]
  #[inline] pub fn sub1h(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
  }
#[doc="Subtract 1 hour (winter time change)"]
  #[inline] pub fn set_sub1h<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Add 1 hour (summer time change)"]
  #[inline] pub fn add1h(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Add 1 hour (summer time change)"]
  #[inline] pub fn set_add1h<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Time-stamp interrupt enable"]
  #[inline] pub fn tsie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Time-stamp interrupt enable"]
  #[inline] pub fn set_tsie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Wakeup timer interrupt enable"]
  #[inline] pub fn wutie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="Wakeup timer interrupt enable"]
  #[inline] pub fn set_wutie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Alarm B interrupt enable"]
  #[inline] pub fn alrbie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="Alarm B interrupt enable"]
  #[inline] pub fn set_alrbie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Alarm A interrupt enable"]
  #[inline] pub fn alraie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Alarm A interrupt enable"]
  #[inline] pub fn set_alraie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="timestamp enable"]
  #[inline] pub fn tse(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="timestamp enable"]
  #[inline] pub fn set_tse<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Wakeup timer enable"]
  #[inline] pub fn wute(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Wakeup timer enable"]
  #[inline] pub fn set_wute<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Alarm B enable"]
  #[inline] pub fn alrbe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Alarm B enable"]
  #[inline] pub fn set_alrbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Alarm A enable"]
  #[inline] pub fn alrae(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Alarm A enable"]
  #[inline] pub fn set_alrae<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Hour format"]
  #[inline] pub fn fmt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Hour format"]
  #[inline] pub fn set_fmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Bypass the shadow registers"]
  #[inline] pub fn bypshad(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Bypass the shadow registers"]
  #[inline] pub fn set_bypshad<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
  #[inline] pub fn refckon(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
  #[inline] pub fn set_refckon<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Time-stamp event active edge"]
  #[inline] pub fn tsedge(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Time-stamp event active edge"]
  #[inline] pub fn set_tsedge<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Wakeup clock selection"]
  #[inline] pub fn wucksel(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
  }
#[doc="Wakeup clock selection"]
  #[inline] pub fn set_wucksel<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 0);
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
      if self.coe() != 0 { try!(write!(f, " coe"))}
      if self.osel() != 0 { try!(write!(f, " osel=0x{:x}", self.osel()))}
      if self.pol() != 0 { try!(write!(f, " pol"))}
      if self.cosel() != 0 { try!(write!(f, " cosel"))}
      if self.bkp() != 0 { try!(write!(f, " bkp"))}
      if self.sub1h() != 0 { try!(write!(f, " sub1h"))}
      if self.add1h() != 0 { try!(write!(f, " add1h"))}
      if self.tsie() != 0 { try!(write!(f, " tsie"))}
      if self.wutie() != 0 { try!(write!(f, " wutie"))}
      if self.alrbie() != 0 { try!(write!(f, " alrbie"))}
      if self.alraie() != 0 { try!(write!(f, " alraie"))}
      if self.tse() != 0 { try!(write!(f, " tse"))}
      if self.wute() != 0 { try!(write!(f, " wute"))}
      if self.alrbe() != 0 { try!(write!(f, " alrbe"))}
      if self.alrae() != 0 { try!(write!(f, " alrae"))}
      if self.fmt() != 0 { try!(write!(f, " fmt"))}
      if self.bypshad() != 0 { try!(write!(f, " bypshad"))}
      if self.refckon() != 0 { try!(write!(f, " refckon"))}
      if self.tsedge() != 0 { try!(write!(f, " tsedge"))}
      if self.wucksel() != 0 { try!(write!(f, " wucksel=0x{:x}", self.wucksel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC initialization and status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="RTC_TAMP2 detection flag"]
  #[inline] pub fn tamp2f(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="RTC_TAMP2 detection flag"]
  #[inline] pub fn set_tamp2f<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="RTC_TAMP1 detection flag"]
  #[inline] pub fn tamp1f(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="RTC_TAMP1 detection flag"]
  #[inline] pub fn set_tamp1f<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Time-stamp overflow flag"]
  #[inline] pub fn tsovf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Time-stamp overflow flag"]
  #[inline] pub fn set_tsovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Time-stamp flag"]
  #[inline] pub fn tsf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Time-stamp flag"]
  #[inline] pub fn set_tsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Wakeup timer flag"]
  #[inline] pub fn wutf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Wakeup timer flag"]
  #[inline] pub fn set_wutf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Alarm B flag"]
  #[inline] pub fn alrbf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Alarm B flag"]
  #[inline] pub fn set_alrbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Alarm A flag"]
  #[inline] pub fn alraf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Alarm A flag"]
  #[inline] pub fn set_alraf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Initialization mode"]
  #[inline] pub fn init(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Initialization mode"]
  #[inline] pub fn set_init<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Initialization flag"]
  #[inline] pub fn initf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Initialization flag"]
  #[inline] pub fn set_initf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Registers synchronization flag"]
  #[inline] pub fn rsf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Registers synchronization flag"]
  #[inline] pub fn set_rsf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Initialization status flag"]
  #[inline] pub fn inits(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Initialization status flag"]
  #[inline] pub fn set_inits<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Shift operation pending"]
  #[inline] pub fn shpf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Shift operation pending"]
  #[inline] pub fn set_shpf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Wakeup timer write flag"]
  #[inline] pub fn wutwf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Wakeup timer write flag"]
  #[inline] pub fn set_wutwf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Alarm B write flag"]
  #[inline] pub fn alrbwf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Alarm B write flag"]
  #[inline] pub fn set_alrbwf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Alarm A write flag"]
  #[inline] pub fn alrawf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Alarm A write flag"]
  #[inline] pub fn set_alrawf<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
      if self.tamp2f() != 0 { try!(write!(f, " tamp2f"))}
      if self.tamp1f() != 0 { try!(write!(f, " tamp1f"))}
      if self.tsovf() != 0 { try!(write!(f, " tsovf"))}
      if self.tsf() != 0 { try!(write!(f, " tsf"))}
      if self.wutf() != 0 { try!(write!(f, " wutf"))}
      if self.alrbf() != 0 { try!(write!(f, " alrbf"))}
      if self.alraf() != 0 { try!(write!(f, " alraf"))}
      if self.init() != 0 { try!(write!(f, " init"))}
      if self.initf() != 0 { try!(write!(f, " initf"))}
      if self.rsf() != 0 { try!(write!(f, " rsf"))}
      if self.inits() != 0 { try!(write!(f, " inits"))}
      if self.shpf() != 0 { try!(write!(f, " shpf"))}
      if self.wutwf() != 0 { try!(write!(f, " wutwf"))}
      if self.alrbwf() != 0 { try!(write!(f, " alrbwf"))}
      if self.alrawf() != 0 { try!(write!(f, " alrawf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC prescaler register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prer(pub u32);
impl Prer {
#[doc="Asynchronous prescaler factor"]
  #[inline] pub fn prediv_a(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
  }
#[doc="Asynchronous prescaler factor"]
  #[inline] pub fn set_prediv_a<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Synchronous prescaler factor"]
  #[inline] pub fn prediv_s(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Synchronous prescaler factor"]
  #[inline] pub fn set_prediv_s<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Prer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prediv_a() != 0 { try!(write!(f, " prediv_a=0x{:x}", self.prediv_a()))}
      if self.prediv_s() != 0 { try!(write!(f, " prediv_s=0x{:x}", self.prediv_s()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC wakeup timer register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wutr(pub u32);
impl Wutr {
#[doc="Wakeup auto-reload value bits"]
  #[inline] pub fn wut(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Wakeup auto-reload value bits"]
  #[inline] pub fn set_wut<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Wutr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wutr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wut() != 0 { try!(write!(f, " wut=0x{:x}", self.wut()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC alarm A register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Alrmar(pub u32);
impl Alrmar {
#[doc="Alarm A date mask"]
  #[inline] pub fn msk4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
  }
#[doc="Alarm A date mask"]
  #[inline] pub fn set_msk4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Week day selection"]
  #[inline] pub fn wdsel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
  }
#[doc="Week day selection"]
  #[inline] pub fn set_wdsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Date tens in BCD format."]
  #[inline] pub fn dt(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="Date tens in BCD format."]
  #[inline] pub fn set_dt<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Date units or day in BCD format."]
  #[inline] pub fn du(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Date units or day in BCD format."]
  #[inline] pub fn set_du<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Alarm A hours mask"]
  #[inline] pub fn msk3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="Alarm A hours mask"]
  #[inline] pub fn set_msk3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="AM/PM notation"]
  #[inline] pub fn pm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="AM/PM notation"]
  #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Hour tens in BCD format."]
  #[inline] pub fn ht(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
  }
#[doc="Hour tens in BCD format."]
  #[inline] pub fn set_ht<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Hour units in BCD format."]
  #[inline] pub fn hu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="Hour units in BCD format."]
  #[inline] pub fn set_hu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Alarm A minutes mask"]
  #[inline] pub fn msk2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Alarm A minutes mask"]
  #[inline] pub fn set_msk2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Minute tens in BCD format."]
  #[inline] pub fn mnt(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
  }
#[doc="Minute tens in BCD format."]
  #[inline] pub fn set_mnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Minute units in BCD format."]
  #[inline] pub fn mnu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="Minute units in BCD format."]
  #[inline] pub fn set_mnu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Alarm A seconds mask"]
  #[inline] pub fn msk1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Alarm A seconds mask"]
  #[inline] pub fn set_msk1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Second tens in BCD format."]
  #[inline] pub fn st(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
  }
#[doc="Second tens in BCD format."]
  #[inline] pub fn set_st<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Second units in BCD format."]
  #[inline] pub fn su(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="Second units in BCD format."]
  #[inline] pub fn set_su<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Alrmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Alrmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.msk4() != 0 { try!(write!(f, " msk4"))}
      if self.wdsel() != 0 { try!(write!(f, " wdsel"))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      if self.msk3() != 0 { try!(write!(f, " msk3"))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.msk2() != 0 { try!(write!(f, " msk2"))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.msk1() != 0 { try!(write!(f, " msk1"))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC alarm B register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Alrmbr(pub u32);
impl Alrmbr {
#[doc="Alarm B date mask"]
  #[inline] pub fn msk4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
  }
#[doc="Alarm B date mask"]
  #[inline] pub fn set_msk4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Week day selection"]
  #[inline] pub fn wdsel(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
  }
#[doc="Week day selection"]
  #[inline] pub fn set_wdsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Date tens in BCD format"]
  #[inline] pub fn dt(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="Date tens in BCD format"]
  #[inline] pub fn set_dt<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Date units or day in BCD format"]
  #[inline] pub fn du(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Date units or day in BCD format"]
  #[inline] pub fn set_du<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Alarm B hours mask"]
  #[inline] pub fn msk3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
  }
#[doc="Alarm B hours mask"]
  #[inline] pub fn set_msk3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="AM/PM notation"]
  #[inline] pub fn pm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="AM/PM notation"]
  #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Hour tens in BCD format"]
  #[inline] pub fn ht(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
  }
#[doc="Hour tens in BCD format"]
  #[inline] pub fn set_ht<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Hour units in BCD format"]
  #[inline] pub fn hu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="Hour units in BCD format"]
  #[inline] pub fn set_hu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Alarm B minutes mask"]
  #[inline] pub fn msk2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Alarm B minutes mask"]
  #[inline] pub fn set_msk2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Minute tens in BCD format"]
  #[inline] pub fn mnt(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
  }
#[doc="Minute tens in BCD format"]
  #[inline] pub fn set_mnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Minute units in BCD format"]
  #[inline] pub fn mnu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="Minute units in BCD format"]
  #[inline] pub fn set_mnu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Alarm B seconds mask"]
  #[inline] pub fn msk1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Alarm B seconds mask"]
  #[inline] pub fn set_msk1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Second tens in BCD format"]
  #[inline] pub fn st(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
  }
#[doc="Second tens in BCD format"]
  #[inline] pub fn set_st<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Second units in BCD format"]
  #[inline] pub fn su(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="Second units in BCD format"]
  #[inline] pub fn set_su<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Alrmbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Alrmbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.msk4() != 0 { try!(write!(f, " msk4"))}
      if self.wdsel() != 0 { try!(write!(f, " wdsel"))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      if self.msk3() != 0 { try!(write!(f, " msk3"))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.msk2() != 0 { try!(write!(f, " msk2"))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.msk1() != 0 { try!(write!(f, " msk1"))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="write protection register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wpr(pub u32);
impl Wpr {
#[doc="Write protection key"]
  #[inline] pub fn key(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Write protection key"]
  #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Wpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC sub second register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ssr(pub u32);
impl Ssr {
#[doc="Sub second value"]
  #[inline] pub fn ss(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Sub second value"]
  #[inline] pub fn set_ss<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC shift control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Shiftr(pub u32);
impl Shiftr {
#[doc="Add one second"]
  #[inline] pub fn add1s(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
  }
#[doc="Add one second"]
  #[inline] pub fn set_add1s<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Subtract a fraction of a second"]
  #[inline] pub fn subfs(&self) -> bits::U15 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
  }
#[doc="Subtract a fraction of a second"]
  #[inline] pub fn set_subfs<V: Into<bits::U15>>(mut self, value: V) -> Self {
     let value: bits::U15 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Shiftr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Shiftr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.add1s() != 0 { try!(write!(f, " add1s"))}
      if self.subfs() != 0 { try!(write!(f, " subfs=0x{:x}", self.subfs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC timestamp time register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tstr(pub u32);
impl Tstr {
#[doc="AM/PM notation"]
  #[inline] pub fn pm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="AM/PM notation"]
  #[inline] pub fn set_pm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Hour tens in BCD format."]
  #[inline] pub fn ht(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
  }
#[doc="Hour tens in BCD format."]
  #[inline] pub fn set_ht<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Hour units in BCD format."]
  #[inline] pub fn hu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="Hour units in BCD format."]
  #[inline] pub fn set_hu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Minute tens in BCD format."]
  #[inline] pub fn mnt(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
  }
#[doc="Minute tens in BCD format."]
  #[inline] pub fn set_mnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Minute units in BCD format."]
  #[inline] pub fn mnu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="Minute units in BCD format."]
  #[inline] pub fn set_mnu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Second tens in BCD format."]
  #[inline] pub fn st(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
  }
#[doc="Second tens in BCD format."]
  #[inline] pub fn set_st<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Second units in BCD format."]
  #[inline] pub fn su(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="Second units in BCD format."]
  #[inline] pub fn set_su<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC timestamp date register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tsdr(pub u32);
impl Tsdr {
#[doc="Week day units"]
  #[inline] pub fn wdu(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
  }
#[doc="Week day units"]
  #[inline] pub fn set_wdu<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Month tens in BCD format"]
  #[inline] pub fn mt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Month tens in BCD format"]
  #[inline] pub fn set_mt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Month units in BCD format"]
  #[inline] pub fn mu(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
  }
#[doc="Month units in BCD format"]
  #[inline] pub fn set_mu<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Date tens in BCD format"]
  #[inline] pub fn dt(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="Date tens in BCD format"]
  #[inline] pub fn set_dt<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Date units in BCD format"]
  #[inline] pub fn du(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="Date units in BCD format"]
  #[inline] pub fn set_du<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tsdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tsdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdu() != 0 { try!(write!(f, " wdu=0x{:x}", self.wdu()))}
      if self.mt() != 0 { try!(write!(f, " mt"))}
      if self.mu() != 0 { try!(write!(f, " mu=0x{:x}", self.mu()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC time-stamp sub second register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tsssr(pub u32);
impl Tsssr {
#[doc="Sub second value"]
  #[inline] pub fn ss(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Sub second value"]
  #[inline] pub fn set_ss<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tsssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tsssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC calibration register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Calr(pub u32);
impl Calr {
#[doc="Use an 8-second calibration cycle period"]
  #[inline] pub fn calp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Use an 8-second calibration cycle period"]
  #[inline] pub fn set_calp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Use a 16-second calibration cycle period"]
  #[inline] pub fn calw8(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="Use a 16-second calibration cycle period"]
  #[inline] pub fn set_calw8<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Reserved"]
  #[inline] pub fn calw16(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="Reserved"]
  #[inline] pub fn set_calw16<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Calibration minus"]
  #[inline] pub fn calm(&self) -> bits::U9 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
  }
#[doc="Calibration minus"]
  #[inline] pub fn set_calm<V: Into<bits::U9>>(mut self, value: V) -> Self {
     let value: bits::U9 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Calr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Calr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.calp() != 0 { try!(write!(f, " calp"))}
      if self.calw8() != 0 { try!(write!(f, " calw8"))}
      if self.calw16() != 0 { try!(write!(f, " calw16"))}
      if self.calm() != 0 { try!(write!(f, " calm=0x{:x}", self.calm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC tamper configuration register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tampcr(pub u32);
impl Tampcr {
#[doc="Tamper 2 mask flag"]
  #[inline] pub fn tamp2mf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
  }
#[doc="Tamper 2 mask flag"]
  #[inline] pub fn set_tamp2mf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Tamper 2 no erase"]
  #[inline] pub fn tamp2noerase(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="Tamper 2 no erase"]
  #[inline] pub fn set_tamp2noerase<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Tamper 2 interrupt enable"]
  #[inline] pub fn tamp2ie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Tamper 2 interrupt enable"]
  #[inline] pub fn set_tamp2ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Tamper 1 mask flag"]
  #[inline] pub fn tamp1mf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Tamper 1 mask flag"]
  #[inline] pub fn set_tamp1mf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Tamper 1 no erase"]
  #[inline] pub fn tamp1noerase(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
  }
#[doc="Tamper 1 no erase"]
  #[inline] pub fn set_tamp1noerase<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Tamper 1 interrupt enable"]
  #[inline] pub fn tamp1ie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Tamper 1 interrupt enable"]
  #[inline] pub fn set_tamp1ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="RTC_TAMPx pull-up disable"]
  #[inline] pub fn tamppudis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="RTC_TAMPx pull-up disable"]
  #[inline] pub fn set_tamppudis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="RTC_TAMPx precharge duration"]
  #[inline] pub fn tampprch(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
  }
#[doc="RTC_TAMPx precharge duration"]
  #[inline] pub fn set_tampprch<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="RTC_TAMPx filter count"]
  #[inline] pub fn tampflt(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
  }
#[doc="RTC_TAMPx filter count"]
  #[inline] pub fn set_tampflt<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Tamper sampling frequency"]
  #[inline] pub fn tampfreq(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
  }
#[doc="Tamper sampling frequency"]
  #[inline] pub fn set_tampfreq<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Activate timestamp on tamper detection event"]
  #[inline] pub fn tampts(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Activate timestamp on tamper detection event"]
  #[inline] pub fn set_tampts<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Active level for RTC_TAMP2 input"]
  #[inline] pub fn tamp2_trg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Active level for RTC_TAMP2 input"]
  #[inline] pub fn set_tamp2_trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="RTC_TAMP2 input detection enable"]
  #[inline] pub fn tamp2e(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="RTC_TAMP2 input detection enable"]
  #[inline] pub fn set_tamp2e<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Tamper interrupt enable"]
  #[inline] pub fn tampie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Tamper interrupt enable"]
  #[inline] pub fn set_tampie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Active level for RTC_TAMP1 input"]
  #[inline] pub fn tamp1trg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Active level for RTC_TAMP1 input"]
  #[inline] pub fn set_tamp1trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="RTC_TAMP1 input detection enable"]
  #[inline] pub fn tamp1e(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="RTC_TAMP1 input detection enable"]
  #[inline] pub fn set_tamp1e<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tampcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tampcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tamp2mf() != 0 { try!(write!(f, " tamp2mf"))}
      if self.tamp2noerase() != 0 { try!(write!(f, " tamp2noerase"))}
      if self.tamp2ie() != 0 { try!(write!(f, " tamp2ie"))}
      if self.tamp1mf() != 0 { try!(write!(f, " tamp1mf"))}
      if self.tamp1noerase() != 0 { try!(write!(f, " tamp1noerase"))}
      if self.tamp1ie() != 0 { try!(write!(f, " tamp1ie"))}
      if self.tamppudis() != 0 { try!(write!(f, " tamppudis"))}
      if self.tampprch() != 0 { try!(write!(f, " tampprch=0x{:x}", self.tampprch()))}
      if self.tampflt() != 0 { try!(write!(f, " tampflt=0x{:x}", self.tampflt()))}
      if self.tampfreq() != 0 { try!(write!(f, " tampfreq=0x{:x}", self.tampfreq()))}
      if self.tampts() != 0 { try!(write!(f, " tampts"))}
      if self.tamp2_trg() != 0 { try!(write!(f, " tamp2_trg"))}
      if self.tamp2e() != 0 { try!(write!(f, " tamp2e"))}
      if self.tampie() != 0 { try!(write!(f, " tampie"))}
      if self.tamp1trg() != 0 { try!(write!(f, " tamp1trg"))}
      if self.tamp1e() != 0 { try!(write!(f, " tamp1e"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC alarm A sub second register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Alrmassr(pub u32);
impl Alrmassr {
#[doc="Mask the most-significant bits starting at this bit"]
  #[inline] pub fn maskss(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Mask the most-significant bits starting at this bit"]
  #[inline] pub fn set_maskss<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Sub seconds value"]
  #[inline] pub fn ss(&self) -> bits::U15 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
  }
#[doc="Sub seconds value"]
  #[inline] pub fn set_ss<V: Into<bits::U15>>(mut self, value: V) -> Self {
     let value: bits::U15 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Alrmassr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Alrmassr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maskss() != 0 { try!(write!(f, " maskss=0x{:x}", self.maskss()))}
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC alarm B sub second register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Alrmbssr(pub u32);
impl Alrmbssr {
#[doc="Mask the most-significant bits starting at this bit"]
  #[inline] pub fn maskss(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Mask the most-significant bits starting at this bit"]
  #[inline] pub fn set_maskss<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Sub seconds value"]
  #[inline] pub fn ss(&self) -> bits::U15 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
  }
#[doc="Sub seconds value"]
  #[inline] pub fn set_ss<V: Into<bits::U15>>(mut self, value: V) -> Self {
     let value: bits::U15 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Alrmbssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Alrmbssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maskss() != 0 { try!(write!(f, " maskss=0x{:x}", self.maskss()))}
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="option register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Or(pub u32);
impl Or {
#[doc="RTC_ALARM on PC13 output type"]
  #[inline] pub fn rtc_out_rmp(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="RTC_ALARM on PC13 output type"]
  #[inline] pub fn set_rtc_out_rmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="RTC_ALARM on PC13 output type"]
  #[inline] pub fn rtc_alarm_type(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="RTC_ALARM on PC13 output type"]
  #[inline] pub fn set_rtc_alarm_type<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.rtc_out_rmp() != 0 { try!(write!(f, " rtc_out_rmp"))}
      if self.rtc_alarm_type() != 0 { try!(write!(f, " rtc_alarm_type"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="RTC backup register n"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bkpr(pub u32);
impl Bkpr {
#[doc="BKP"]
  #[inline] pub fn bkp(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="BKP"]
  #[inline] pub fn set_bkp<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bkpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bkpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
