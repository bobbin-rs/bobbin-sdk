//! Reset and clock control
pub const RCC: Rcc = Rcc(0x40023800);

#[doc="Reset and clock control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcc(pub u32);
impl Rcc {
#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the PLLCFGR register."]
  #[inline] pub fn pllcfgr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PLLCFGR register."]
  #[inline] pub fn pllcfgr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the PLLCFGR register."]
  #[inline] pub fn pllcfgr(&self) -> Pllcfgr { 
     unsafe {
        Pllcfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the PLLCFGR register."]
  #[inline] pub fn set_pllcfgr(&self, value: Pllcfgr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PLLCFGR register."]
  #[inline] pub fn with_pllcfgr<F: FnOnce(Pllcfgr) -> Pllcfgr>(&self, f: F) -> &Self {
     let tmp = self.pllcfgr();
     self.set_pllcfgr(f(tmp))
  }

#[doc="Get the *const pointer for the CFGR register."]
  #[inline] pub fn cfgr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR register."]
  #[inline] pub fn cfgr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CFGR register."]
  #[inline] pub fn cfgr(&self) -> Cfgr { 
     unsafe {
        Cfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CFGR register."]
  #[inline] pub fn set_cfgr(&self, value: Cfgr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR register."]
  #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
     let tmp = self.cfgr();
     self.set_cfgr(f(tmp))
  }

#[doc="Get the *const pointer for the CIR register."]
  #[inline] pub fn cir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the CIR register."]
  #[inline] pub fn cir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the CIR register."]
  #[inline] pub fn cir(&self) -> Cir { 
     unsafe {
        Cir(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the CIR register."]
  #[inline] pub fn set_cir(&self, value: Cir) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CIR register."]
  #[inline] pub fn with_cir<F: FnOnce(Cir) -> Cir>(&self, f: F) -> &Self {
     let tmp = self.cir();
     self.set_cir(f(tmp))
  }

#[doc="Get the *const pointer for the AHB1RSTR register."]
  #[inline] pub fn ahb1rstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the AHB1RSTR register."]
  #[inline] pub fn ahb1rstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the AHB1RSTR register."]
  #[inline] pub fn ahb1rstr(&self) -> Ahb1rstr { 
     unsafe {
        Ahb1rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the AHB1RSTR register."]
  #[inline] pub fn set_ahb1rstr(&self, value: Ahb1rstr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB1RSTR register."]
  #[inline] pub fn with_ahb1rstr<F: FnOnce(Ahb1rstr) -> Ahb1rstr>(&self, f: F) -> &Self {
     let tmp = self.ahb1rstr();
     self.set_ahb1rstr(f(tmp))
  }

#[doc="Get the *const pointer for the AHB2RSTR register."]
  #[inline] pub fn ahb2rstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the AHB2RSTR register."]
  #[inline] pub fn ahb2rstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the AHB2RSTR register."]
  #[inline] pub fn ahb2rstr(&self) -> Ahb2rstr { 
     unsafe {
        Ahb2rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the AHB2RSTR register."]
  #[inline] pub fn set_ahb2rstr(&self, value: Ahb2rstr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB2RSTR register."]
  #[inline] pub fn with_ahb2rstr<F: FnOnce(Ahb2rstr) -> Ahb2rstr>(&self, f: F) -> &Self {
     let tmp = self.ahb2rstr();
     self.set_ahb2rstr(f(tmp))
  }

#[doc="Get the *const pointer for the AHB3RSTR register."]
  #[inline] pub fn ahb3rstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the AHB3RSTR register."]
  #[inline] pub fn ahb3rstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the AHB3RSTR register."]
  #[inline] pub fn ahb3rstr(&self) -> Ahb3rstr { 
     unsafe {
        Ahb3rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the AHB3RSTR register."]
  #[inline] pub fn set_ahb3rstr(&self, value: Ahb3rstr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB3RSTR register."]
  #[inline] pub fn with_ahb3rstr<F: FnOnce(Ahb3rstr) -> Ahb3rstr>(&self, f: F) -> &Self {
     let tmp = self.ahb3rstr();
     self.set_ahb3rstr(f(tmp))
  }

#[doc="Get the *const pointer for the APB1RSTR register."]
  #[inline] pub fn apb1rstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the APB1RSTR register."]
  #[inline] pub fn apb1rstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the APB1RSTR register."]
  #[inline] pub fn apb1rstr(&self) -> Apb1rstr { 
     unsafe {
        Apb1rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the APB1RSTR register."]
  #[inline] pub fn set_apb1rstr(&self, value: Apb1rstr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB1RSTR register."]
  #[inline] pub fn with_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Self {
     let tmp = self.apb1rstr();
     self.set_apb1rstr(f(tmp))
  }

#[doc="Get the *const pointer for the APB2RSTR register."]
  #[inline] pub fn apb2rstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the APB2RSTR register."]
  #[inline] pub fn apb2rstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the APB2RSTR register."]
  #[inline] pub fn apb2rstr(&self) -> Apb2rstr { 
     unsafe {
        Apb2rstr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the APB2RSTR register."]
  #[inline] pub fn set_apb2rstr(&self, value: Apb2rstr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB2RSTR register."]
  #[inline] pub fn with_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Self {
     let tmp = self.apb2rstr();
     self.set_apb2rstr(f(tmp))
  }

#[doc="Get the *const pointer for the AHB1ENR register."]
  #[inline] pub fn ahb1enr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the AHB1ENR register."]
  #[inline] pub fn ahb1enr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the AHB1ENR register."]
  #[inline] pub fn ahb1enr(&self) -> Ahb1enr { 
     unsafe {
        Ahb1enr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
#[doc="Write the AHB1ENR register."]
  #[inline] pub fn set_ahb1enr(&self, value: Ahb1enr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB1ENR register."]
  #[inline] pub fn with_ahb1enr<F: FnOnce(Ahb1enr) -> Ahb1enr>(&self, f: F) -> &Self {
     let tmp = self.ahb1enr();
     self.set_ahb1enr(f(tmp))
  }

#[doc="Get the *const pointer for the AHB2ENR register."]
  #[inline] pub fn ahb2enr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the AHB2ENR register."]
  #[inline] pub fn ahb2enr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Read the AHB2ENR register."]
  #[inline] pub fn ahb2enr(&self) -> Ahb2enr { 
     unsafe {
        Ahb2enr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
#[doc="Write the AHB2ENR register."]
  #[inline] pub fn set_ahb2enr(&self, value: Ahb2enr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB2ENR register."]
  #[inline] pub fn with_ahb2enr<F: FnOnce(Ahb2enr) -> Ahb2enr>(&self, f: F) -> &Self {
     let tmp = self.ahb2enr();
     self.set_ahb2enr(f(tmp))
  }

#[doc="Get the *const pointer for the AHB3ENR register."]
  #[inline] pub fn ahb3enr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
#[doc="Get the *mut pointer for the AHB3ENR register."]
  #[inline] pub fn ahb3enr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
#[doc="Read the AHB3ENR register."]
  #[inline] pub fn ahb3enr(&self) -> Ahb3enr { 
     unsafe {
        Ahb3enr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
#[doc="Write the AHB3ENR register."]
  #[inline] pub fn set_ahb3enr(&self, value: Ahb3enr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB3ENR register."]
  #[inline] pub fn with_ahb3enr<F: FnOnce(Ahb3enr) -> Ahb3enr>(&self, f: F) -> &Self {
     let tmp = self.ahb3enr();
     self.set_ahb3enr(f(tmp))
  }

#[doc="Get the *const pointer for the APB1ENR register."]
  #[inline] pub fn apb1enr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the APB1ENR register."]
  #[inline] pub fn apb1enr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the APB1ENR register."]
  #[inline] pub fn apb1enr(&self) -> Apb1enr { 
     unsafe {
        Apb1enr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the APB1ENR register."]
  #[inline] pub fn set_apb1enr(&self, value: Apb1enr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB1ENR register."]
  #[inline] pub fn with_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Self {
     let tmp = self.apb1enr();
     self.set_apb1enr(f(tmp))
  }

#[doc="Get the *const pointer for the APB2ENR register."]
  #[inline] pub fn apb2enr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the APB2ENR register."]
  #[inline] pub fn apb2enr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the APB2ENR register."]
  #[inline] pub fn apb2enr(&self) -> Apb2enr { 
     unsafe {
        Apb2enr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the APB2ENR register."]
  #[inline] pub fn set_apb2enr(&self, value: Apb2enr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB2ENR register."]
  #[inline] pub fn with_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Self {
     let tmp = self.apb2enr();
     self.set_apb2enr(f(tmp))
  }

#[doc="Get the *const pointer for the AHB1LPENR register."]
  #[inline] pub fn ahb1lpenr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
#[doc="Get the *mut pointer for the AHB1LPENR register."]
  #[inline] pub fn ahb1lpenr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
#[doc="Read the AHB1LPENR register."]
  #[inline] pub fn ahb1lpenr(&self) -> Ahb1lpenr { 
     unsafe {
        Ahb1lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
#[doc="Write the AHB1LPENR register."]
  #[inline] pub fn set_ahb1lpenr(&self, value: Ahb1lpenr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB1LPENR register."]
  #[inline] pub fn with_ahb1lpenr<F: FnOnce(Ahb1lpenr) -> Ahb1lpenr>(&self, f: F) -> &Self {
     let tmp = self.ahb1lpenr();
     self.set_ahb1lpenr(f(tmp))
  }

#[doc="Get the *const pointer for the AHB2LPENR register."]
  #[inline] pub fn ahb2lpenr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
#[doc="Get the *mut pointer for the AHB2LPENR register."]
  #[inline] pub fn ahb2lpenr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
#[doc="Read the AHB2LPENR register."]
  #[inline] pub fn ahb2lpenr(&self) -> Ahb2lpenr { 
     unsafe {
        Ahb2lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
#[doc="Write the AHB2LPENR register."]
  #[inline] pub fn set_ahb2lpenr(&self, value: Ahb2lpenr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB2LPENR register."]
  #[inline] pub fn with_ahb2lpenr<F: FnOnce(Ahb2lpenr) -> Ahb2lpenr>(&self, f: F) -> &Self {
     let tmp = self.ahb2lpenr();
     self.set_ahb2lpenr(f(tmp))
  }

#[doc="Get the *const pointer for the AHB3LPENR register."]
  #[inline] pub fn ahb3lpenr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
#[doc="Get the *mut pointer for the AHB3LPENR register."]
  #[inline] pub fn ahb3lpenr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
#[doc="Read the AHB3LPENR register."]
  #[inline] pub fn ahb3lpenr(&self) -> Ahb3lpenr { 
     unsafe {
        Ahb3lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
#[doc="Write the AHB3LPENR register."]
  #[inline] pub fn set_ahb3lpenr(&self, value: Ahb3lpenr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AHB3LPENR register."]
  #[inline] pub fn with_ahb3lpenr<F: FnOnce(Ahb3lpenr) -> Ahb3lpenr>(&self, f: F) -> &Self {
     let tmp = self.ahb3lpenr();
     self.set_ahb3lpenr(f(tmp))
  }

#[doc="Get the *const pointer for the APB1LPENR register."]
  #[inline] pub fn apb1lpenr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
#[doc="Get the *mut pointer for the APB1LPENR register."]
  #[inline] pub fn apb1lpenr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
#[doc="Read the APB1LPENR register."]
  #[inline] pub fn apb1lpenr(&self) -> Apb1lpenr { 
     unsafe {
        Apb1lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
#[doc="Write the APB1LPENR register."]
  #[inline] pub fn set_apb1lpenr(&self, value: Apb1lpenr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB1LPENR register."]
  #[inline] pub fn with_apb1lpenr<F: FnOnce(Apb1lpenr) -> Apb1lpenr>(&self, f: F) -> &Self {
     let tmp = self.apb1lpenr();
     self.set_apb1lpenr(f(tmp))
  }

#[doc="Get the *const pointer for the APB2LPENR register."]
  #[inline] pub fn apb2lpenr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x64) as *const u32
  }
#[doc="Get the *mut pointer for the APB2LPENR register."]
  #[inline] pub fn apb2lpenr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x64) as *mut u32
  }
#[doc="Read the APB2LPENR register."]
  #[inline] pub fn apb2lpenr(&self) -> Apb2lpenr { 
     unsafe {
        Apb2lpenr(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
     }
  }
#[doc="Write the APB2LPENR register."]
  #[inline] pub fn set_apb2lpenr(&self, value: Apb2lpenr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the APB2LPENR register."]
  #[inline] pub fn with_apb2lpenr<F: FnOnce(Apb2lpenr) -> Apb2lpenr>(&self, f: F) -> &Self {
     let tmp = self.apb2lpenr();
     self.set_apb2lpenr(f(tmp))
  }

#[doc="Get the *const pointer for the BDCR register."]
  #[inline] pub fn bdcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70) as *const u32
  }
#[doc="Get the *mut pointer for the BDCR register."]
  #[inline] pub fn bdcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70) as *mut u32
  }
#[doc="Read the BDCR register."]
  #[inline] pub fn bdcr(&self) -> Bdcr { 
     unsafe {
        Bdcr(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
     }
  }
#[doc="Write the BDCR register."]
  #[inline] pub fn set_bdcr(&self, value: Bdcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x70) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BDCR register."]
  #[inline] pub fn with_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&self, f: F) -> &Self {
     let tmp = self.bdcr();
     self.set_bdcr(f(tmp))
  }

#[doc="Get the *const pointer for the CSR register."]
  #[inline] pub fn csr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x74) as *const u32
  }
#[doc="Get the *mut pointer for the CSR register."]
  #[inline] pub fn csr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x74) as *mut u32
  }
#[doc="Read the CSR register."]
  #[inline] pub fn csr(&self) -> Csr { 
     unsafe {
        Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x74) as *const u32))
     }
  }
#[doc="Write the CSR register."]
  #[inline] pub fn set_csr(&self, value: Csr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x74) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CSR register."]
  #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
     let tmp = self.csr();
     self.set_csr(f(tmp))
  }

#[doc="Get the *const pointer for the SSCGR register."]
  #[inline] pub fn sscgr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x80) as *const u32
  }
#[doc="Get the *mut pointer for the SSCGR register."]
  #[inline] pub fn sscgr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x80) as *mut u32
  }
#[doc="Read the SSCGR register."]
  #[inline] pub fn sscgr(&self) -> Sscgr { 
     unsafe {
        Sscgr(::core::ptr::read_volatile(((self.0 as usize) + 0x80) as *const u32))
     }
  }
#[doc="Write the SSCGR register."]
  #[inline] pub fn set_sscgr(&self, value: Sscgr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x80) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SSCGR register."]
  #[inline] pub fn with_sscgr<F: FnOnce(Sscgr) -> Sscgr>(&self, f: F) -> &Self {
     let tmp = self.sscgr();
     self.set_sscgr(f(tmp))
  }

#[doc="Get the *const pointer for the PLLI2SCFGR register."]
  #[inline] pub fn plli2scfgr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x84) as *const u32
  }
#[doc="Get the *mut pointer for the PLLI2SCFGR register."]
  #[inline] pub fn plli2scfgr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x84) as *mut u32
  }
#[doc="Read the PLLI2SCFGR register."]
  #[inline] pub fn plli2scfgr(&self) -> Plli2scfgr { 
     unsafe {
        Plli2scfgr(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
     }
  }
#[doc="Write the PLLI2SCFGR register."]
  #[inline] pub fn set_plli2scfgr(&self, value: Plli2scfgr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PLLI2SCFGR register."]
  #[inline] pub fn with_plli2scfgr<F: FnOnce(Plli2scfgr) -> Plli2scfgr>(&self, f: F) -> &Self {
     let tmp = self.plli2scfgr();
     self.set_plli2scfgr(f(tmp))
  }

}

#[doc="clock control register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="PLLI2S clock ready flag"]
  #[inline] pub fn plli2srdy(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="PLLI2S clock ready flag"]
  #[inline] pub fn set_plli2srdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="PLLI2S enable"]
  #[inline] pub fn plli2son(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="PLLI2S enable"]
  #[inline] pub fn set_plli2son(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Main PLL (PLL) clock ready flag"]
  #[inline] pub fn pllrdy(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Main PLL (PLL) clock ready flag"]
  #[inline] pub fn set_pllrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Main PLL (PLL) enable"]
  #[inline] pub fn pllon(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Main PLL (PLL) enable"]
  #[inline] pub fn set_pllon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock security system enable"]
  #[inline] pub fn csson(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Clock security system enable"]
  #[inline] pub fn set_csson(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="HSE clock bypass"]
  #[inline] pub fn hsebyp(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="HSE clock bypass"]
  #[inline] pub fn set_hsebyp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="HSE clock ready flag"]
  #[inline] pub fn hserdy(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="HSE clock ready flag"]
  #[inline] pub fn set_hserdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="HSE clock enable"]
  #[inline] pub fn hseon(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="HSE clock enable"]
  #[inline] pub fn set_hseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Internal high-speed clock calibration"]
  #[inline] pub fn hsical(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
#[doc="Internal high-speed clock calibration"]
  #[inline] pub fn set_hsical(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Internal high-speed clock trimming"]
  #[inline] pub fn hsitrim(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1f // [7:3]
  }
#[doc="Internal high-speed clock trimming"]
  #[inline] pub fn set_hsitrim(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Internal high-speed clock ready flag"]
  #[inline] pub fn hsirdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Internal high-speed clock ready flag"]
  #[inline] pub fn set_hsirdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Internal high-speed clock enable"]
  #[inline] pub fn hsion(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Internal high-speed clock enable"]
  #[inline] pub fn set_hsion(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.plli2srdy() != 0 { try!(write!(f, " plli2srdy"))}
      if self.plli2son() != 0 { try!(write!(f, " plli2son"))}
      if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
      if self.pllon() != 0 { try!(write!(f, " pllon"))}
      if self.csson() != 0 { try!(write!(f, " csson"))}
      if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
      if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
      if self.hseon() != 0 { try!(write!(f, " hseon"))}
      if self.hsical() != 0 { try!(write!(f, " hsical=0x{:x}", self.hsical()))}
      if self.hsitrim() != 0 { try!(write!(f, " hsitrim=0x{:x}", self.hsitrim()))}
      if self.hsirdy() != 0 { try!(write!(f, " hsirdy"))}
      if self.hsion() != 0 { try!(write!(f, " hsion"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PLL configuration register"]
#[derive(PartialEq, Eq)]
pub struct Pllcfgr(pub u32);
impl Pllcfgr {
#[doc="Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
  #[inline] pub fn pllq(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
  #[inline] pub fn set_pllq(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
  #[inline] pub fn pllsrc(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
  #[inline] pub fn set_pllsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Main PLL (PLL) division factor for main system clock"]
  #[inline] pub fn pllp(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
#[doc="Main PLL (PLL) division factor for main system clock"]
  #[inline] pub fn set_pllp(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Main PLL (PLL) multiplication factor for VCO"]
  #[inline] pub fn plln(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1ff // [14:6]
  }
#[doc="Main PLL (PLL) multiplication factor for VCO"]
  #[inline] pub fn set_plln(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
  #[inline] pub fn pllm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3f // [5:0]
  }
#[doc="Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
  #[inline] pub fn set_pllm(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pllcfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pllcfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pllq() != 0 { try!(write!(f, " pllq=0x{:x}", self.pllq()))}
      if self.pllsrc() != 0 { try!(write!(f, " pllsrc"))}
      if self.pllp() != 0 { try!(write!(f, " pllp=0x{:x}", self.pllp()))}
      if self.plln() != 0 { try!(write!(f, " plln=0x{:x}", self.plln()))}
      if self.pllm() != 0 { try!(write!(f, " pllm=0x{:x}", self.pllm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="clock configuration register"]
#[derive(PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
#[doc="Microcontroller clock output 2"]
  #[inline] pub fn mco2(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
#[doc="Microcontroller clock output 2"]
  #[inline] pub fn set_mco2(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="MCO2 prescaler"]
  #[inline] pub fn mco2pre(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x7 // [29:27]
  }
#[doc="MCO2 prescaler"]
  #[inline] pub fn set_mco2pre(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="MCO1 prescaler"]
  #[inline] pub fn mco1pre(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="MCO1 prescaler"]
  #[inline] pub fn set_mco1pre(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="I2S clock selection"]
  #[inline] pub fn i2ssrc(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="I2S clock selection"]
  #[inline] pub fn set_i2ssrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Microcontroller clock output 1"]
  #[inline] pub fn mco1(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x3 // [22:21]
  }
#[doc="Microcontroller clock output 1"]
  #[inline] pub fn set_mco1(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="HSE division factor for RTC clock"]
  #[inline] pub fn rtcpre(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1f // [20:16]
  }
#[doc="HSE division factor for RTC clock"]
  #[inline] pub fn set_rtcpre(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
  }

#[doc="APB high-speed prescaler (APB2)"]
  #[inline] pub fn ppre2(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
#[doc="APB high-speed prescaler (APB2)"]
  #[inline] pub fn set_ppre2(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="APB Low speed prescaler (APB1)"]
  #[inline] pub fn ppre1(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x7 // [12:10]
  }
#[doc="APB Low speed prescaler (APB1)"]
  #[inline] pub fn set_ppre1(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="AHB prescaler"]
  #[inline] pub fn hpre(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
#[doc="AHB prescaler"]
  #[inline] pub fn set_hpre(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="System clock switch status"]
  #[inline] pub fn sws(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
#[doc="System clock switch status"]
  #[inline] pub fn set_sws(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="System clock switch"]
  #[inline] pub fn sw(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="System clock switch"]
  #[inline] pub fn set_sw(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
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
      if self.mco2() != 0 { try!(write!(f, " mco2=0x{:x}", self.mco2()))}
      if self.mco2pre() != 0 { try!(write!(f, " mco2pre=0x{:x}", self.mco2pre()))}
      if self.mco1pre() != 0 { try!(write!(f, " mco1pre=0x{:x}", self.mco1pre()))}
      if self.i2ssrc() != 0 { try!(write!(f, " i2ssrc"))}
      if self.mco1() != 0 { try!(write!(f, " mco1=0x{:x}", self.mco1()))}
      if self.rtcpre() != 0 { try!(write!(f, " rtcpre=0x{:x}", self.rtcpre()))}
      if self.ppre2() != 0 { try!(write!(f, " ppre2=0x{:x}", self.ppre2()))}
      if self.ppre1() != 0 { try!(write!(f, " ppre1=0x{:x}", self.ppre1()))}
      if self.hpre() != 0 { try!(write!(f, " hpre=0x{:x}", self.hpre()))}
      if self.sws() != 0 { try!(write!(f, " sws=0x{:x}", self.sws()))}
      if self.sw() != 0 { try!(write!(f, " sw=0x{:x}", self.sw()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="clock interrupt register"]
#[derive(PartialEq, Eq)]
pub struct Cir(pub u32);
impl Cir {
#[doc="Clock security system interrupt clear"]
  #[inline] pub fn cssc(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Clock security system interrupt clear"]
  #[inline] pub fn set_cssc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="PLLI2S ready interrupt clear"]
  #[inline] pub fn plli2srdyc(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="PLLI2S ready interrupt clear"]
  #[inline] pub fn set_plli2srdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Main PLL(PLL) ready interrupt clear"]
  #[inline] pub fn pllrdyc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="Main PLL(PLL) ready interrupt clear"]
  #[inline] pub fn set_pllrdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="HSE ready interrupt clear"]
  #[inline] pub fn hserdyc(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="HSE ready interrupt clear"]
  #[inline] pub fn set_hserdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="HSI ready interrupt clear"]
  #[inline] pub fn hsirdyc(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="HSI ready interrupt clear"]
  #[inline] pub fn set_hsirdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="LSE ready interrupt clear"]
  #[inline] pub fn lserdyc(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="LSE ready interrupt clear"]
  #[inline] pub fn set_lserdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="LSI ready interrupt clear"]
  #[inline] pub fn lsirdyc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="LSI ready interrupt clear"]
  #[inline] pub fn set_lsirdyc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="PLLI2S ready interrupt enable"]
  #[inline] pub fn plli2srdyie(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="PLLI2S ready interrupt enable"]
  #[inline] pub fn set_plli2srdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Main PLL (PLL) ready interrupt enable"]
  #[inline] pub fn pllrdyie(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Main PLL (PLL) ready interrupt enable"]
  #[inline] pub fn set_pllrdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="HSE ready interrupt enable"]
  #[inline] pub fn hserdyie(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="HSE ready interrupt enable"]
  #[inline] pub fn set_hserdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="HSI ready interrupt enable"]
  #[inline] pub fn hsirdyie(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="HSI ready interrupt enable"]
  #[inline] pub fn set_hsirdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="LSE ready interrupt enable"]
  #[inline] pub fn lserdyie(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="LSE ready interrupt enable"]
  #[inline] pub fn set_lserdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="LSI ready interrupt enable"]
  #[inline] pub fn lsirdyie(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="LSI ready interrupt enable"]
  #[inline] pub fn set_lsirdyie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Clock security system interrupt flag"]
  #[inline] pub fn cssf(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Clock security system interrupt flag"]
  #[inline] pub fn set_cssf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="PLLI2S ready interrupt flag"]
  #[inline] pub fn plli2srdyf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="PLLI2S ready interrupt flag"]
  #[inline] pub fn set_plli2srdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Main PLL (PLL) ready interrupt flag"]
  #[inline] pub fn pllrdyf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Main PLL (PLL) ready interrupt flag"]
  #[inline] pub fn set_pllrdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="HSE ready interrupt flag"]
  #[inline] pub fn hserdyf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="HSE ready interrupt flag"]
  #[inline] pub fn set_hserdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="HSI ready interrupt flag"]
  #[inline] pub fn hsirdyf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="HSI ready interrupt flag"]
  #[inline] pub fn set_hsirdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="LSE ready interrupt flag"]
  #[inline] pub fn lserdyf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="LSE ready interrupt flag"]
  #[inline] pub fn set_lserdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="LSI ready interrupt flag"]
  #[inline] pub fn lsirdyf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="LSI ready interrupt flag"]
  #[inline] pub fn set_lsirdyf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cssc() != 0 { try!(write!(f, " cssc"))}
      if self.plli2srdyc() != 0 { try!(write!(f, " plli2srdyc"))}
      if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
      if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
      if self.hsirdyc() != 0 { try!(write!(f, " hsirdyc"))}
      if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
      if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
      if self.plli2srdyie() != 0 { try!(write!(f, " plli2srdyie"))}
      if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
      if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
      if self.hsirdyie() != 0 { try!(write!(f, " hsirdyie"))}
      if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
      if self.lsirdyie() != 0 { try!(write!(f, " lsirdyie"))}
      if self.cssf() != 0 { try!(write!(f, " cssf"))}
      if self.plli2srdyf() != 0 { try!(write!(f, " plli2srdyf"))}
      if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
      if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
      if self.hsirdyf() != 0 { try!(write!(f, " hsirdyf"))}
      if self.lserdyf() != 0 { try!(write!(f, " lserdyf"))}
      if self.lsirdyf() != 0 { try!(write!(f, " lsirdyf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB1 peripheral reset register"]
#[derive(PartialEq, Eq)]
pub struct Ahb1rstr(pub u32);
impl Ahb1rstr {
#[doc="USB OTG HS module reset"]
  #[inline] pub fn otghsrst(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="USB OTG HS module reset"]
  #[inline] pub fn set_otghsrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Ethernet MAC reset"]
  #[inline] pub fn ethmacrst(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Ethernet MAC reset"]
  #[inline] pub fn set_ethmacrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="DMA2 reset"]
  #[inline] pub fn dma2rst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="DMA2 reset"]
  #[inline] pub fn set_dma2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="DMA2 reset"]
  #[inline] pub fn dma1rst(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="DMA2 reset"]
  #[inline] pub fn set_dma1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="CRC reset"]
  #[inline] pub fn crcrst(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="CRC reset"]
  #[inline] pub fn set_crcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="IO port I reset"]
  #[inline] pub fn gpioirst(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="IO port I reset"]
  #[inline] pub fn set_gpioirst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="IO port H reset"]
  #[inline] pub fn gpiohrst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="IO port H reset"]
  #[inline] pub fn set_gpiohrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="IO port G reset"]
  #[inline] pub fn gpiogrst(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="IO port G reset"]
  #[inline] pub fn set_gpiogrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="IO port F reset"]
  #[inline] pub fn gpiofrst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="IO port F reset"]
  #[inline] pub fn set_gpiofrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="IO port E reset"]
  #[inline] pub fn gpioerst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="IO port E reset"]
  #[inline] pub fn set_gpioerst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="IO port D reset"]
  #[inline] pub fn gpiodrst(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="IO port D reset"]
  #[inline] pub fn set_gpiodrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="IO port C reset"]
  #[inline] pub fn gpiocrst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="IO port C reset"]
  #[inline] pub fn set_gpiocrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="IO port B reset"]
  #[inline] pub fn gpiobrst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="IO port B reset"]
  #[inline] pub fn set_gpiobrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="IO port A reset"]
  #[inline] pub fn gpioarst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="IO port A reset"]
  #[inline] pub fn set_gpioarst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb1rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb1rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otghsrst() != 0 { try!(write!(f, " otghsrst"))}
      if self.ethmacrst() != 0 { try!(write!(f, " ethmacrst"))}
      if self.dma2rst() != 0 { try!(write!(f, " dma2rst"))}
      if self.dma1rst() != 0 { try!(write!(f, " dma1rst"))}
      if self.crcrst() != 0 { try!(write!(f, " crcrst"))}
      if self.gpioirst() != 0 { try!(write!(f, " gpioirst"))}
      if self.gpiohrst() != 0 { try!(write!(f, " gpiohrst"))}
      if self.gpiogrst() != 0 { try!(write!(f, " gpiogrst"))}
      if self.gpiofrst() != 0 { try!(write!(f, " gpiofrst"))}
      if self.gpioerst() != 0 { try!(write!(f, " gpioerst"))}
      if self.gpiodrst() != 0 { try!(write!(f, " gpiodrst"))}
      if self.gpiocrst() != 0 { try!(write!(f, " gpiocrst"))}
      if self.gpiobrst() != 0 { try!(write!(f, " gpiobrst"))}
      if self.gpioarst() != 0 { try!(write!(f, " gpioarst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB2 peripheral reset register"]
#[derive(PartialEq, Eq)]
pub struct Ahb2rstr(pub u32);
impl Ahb2rstr {
#[doc="USB OTG FS module reset"]
  #[inline] pub fn otgfsrst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="USB OTG FS module reset"]
  #[inline] pub fn set_otgfsrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Random number generator module reset"]
  #[inline] pub fn rngrst(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Random number generator module reset"]
  #[inline] pub fn set_rngrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Camera interface reset"]
  #[inline] pub fn dcmirst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Camera interface reset"]
  #[inline] pub fn set_dcmirst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb2rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb2rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otgfsrst() != 0 { try!(write!(f, " otgfsrst"))}
      if self.rngrst() != 0 { try!(write!(f, " rngrst"))}
      if self.dcmirst() != 0 { try!(write!(f, " dcmirst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB3 peripheral reset register"]
#[derive(PartialEq, Eq)]
pub struct Ahb3rstr(pub u32);
impl Ahb3rstr {
#[doc="Flexible static memory controller module reset"]
  #[inline] pub fn fsmcrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Flexible static memory controller module reset"]
  #[inline] pub fn set_fsmcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb3rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb3rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fsmcrst() != 0 { try!(write!(f, " fsmcrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB1 peripheral reset register"]
#[derive(PartialEq, Eq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
#[doc="DAC reset"]
  #[inline] pub fn dacrst(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="DAC reset"]
  #[inline] pub fn set_dacrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Power interface reset"]
  #[inline] pub fn pwrrst(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Power interface reset"]
  #[inline] pub fn set_pwrrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="CAN2 reset"]
  #[inline] pub fn can2rst(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="CAN2 reset"]
  #[inline] pub fn set_can2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="CAN1 reset"]
  #[inline] pub fn can1rst(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="CAN1 reset"]
  #[inline] pub fn set_can1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="I2C3 reset"]
  #[inline] pub fn i2c3rst(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="I2C3 reset"]
  #[inline] pub fn set_i2c3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="I2C 2 reset"]
  #[inline] pub fn i2c2rst(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="I2C 2 reset"]
  #[inline] pub fn set_i2c2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="I2C 1 reset"]
  #[inline] pub fn i2c1rst(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="I2C 1 reset"]
  #[inline] pub fn set_i2c1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="USART 5 reset"]
  #[inline] pub fn uart5rst(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="USART 5 reset"]
  #[inline] pub fn set_uart5rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="USART 4 reset"]
  #[inline] pub fn uart4rst(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="USART 4 reset"]
  #[inline] pub fn set_uart4rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="USART 3 reset"]
  #[inline] pub fn uart3rst(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="USART 3 reset"]
  #[inline] pub fn set_uart3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="USART 2 reset"]
  #[inline] pub fn uart2rst(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="USART 2 reset"]
  #[inline] pub fn set_uart2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="SPI 3 reset"]
  #[inline] pub fn spi3rst(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="SPI 3 reset"]
  #[inline] pub fn set_spi3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="SPI 2 reset"]
  #[inline] pub fn spi2rst(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="SPI 2 reset"]
  #[inline] pub fn set_spi2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Window watchdog reset"]
  #[inline] pub fn wwdgrst(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Window watchdog reset"]
  #[inline] pub fn set_wwdgrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="TIM14 reset"]
  #[inline] pub fn tim14rst(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="TIM14 reset"]
  #[inline] pub fn set_tim14rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="TIM13 reset"]
  #[inline] pub fn tim13rst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="TIM13 reset"]
  #[inline] pub fn set_tim13rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="TIM12 reset"]
  #[inline] pub fn tim12rst(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="TIM12 reset"]
  #[inline] pub fn set_tim12rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="TIM7 reset"]
  #[inline] pub fn tim7rst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="TIM7 reset"]
  #[inline] pub fn set_tim7rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="TIM6 reset"]
  #[inline] pub fn tim6rst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="TIM6 reset"]
  #[inline] pub fn set_tim6rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TIM5 reset"]
  #[inline] pub fn tim5rst(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="TIM5 reset"]
  #[inline] pub fn set_tim5rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="TIM4 reset"]
  #[inline] pub fn tim4rst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="TIM4 reset"]
  #[inline] pub fn set_tim4rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="TIM3 reset"]
  #[inline] pub fn tim3rst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="TIM3 reset"]
  #[inline] pub fn set_tim3rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TIM2 reset"]
  #[inline] pub fn tim2rst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="TIM2 reset"]
  #[inline] pub fn set_tim2rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb1rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb1rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacrst() != 0 { try!(write!(f, " dacrst"))}
      if self.pwrrst() != 0 { try!(write!(f, " pwrrst"))}
      if self.can2rst() != 0 { try!(write!(f, " can2rst"))}
      if self.can1rst() != 0 { try!(write!(f, " can1rst"))}
      if self.i2c3rst() != 0 { try!(write!(f, " i2c3rst"))}
      if self.i2c2rst() != 0 { try!(write!(f, " i2c2rst"))}
      if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
      if self.uart5rst() != 0 { try!(write!(f, " uart5rst"))}
      if self.uart4rst() != 0 { try!(write!(f, " uart4rst"))}
      if self.uart3rst() != 0 { try!(write!(f, " uart3rst"))}
      if self.uart2rst() != 0 { try!(write!(f, " uart2rst"))}
      if self.spi3rst() != 0 { try!(write!(f, " spi3rst"))}
      if self.spi2rst() != 0 { try!(write!(f, " spi2rst"))}
      if self.wwdgrst() != 0 { try!(write!(f, " wwdgrst"))}
      if self.tim14rst() != 0 { try!(write!(f, " tim14rst"))}
      if self.tim13rst() != 0 { try!(write!(f, " tim13rst"))}
      if self.tim12rst() != 0 { try!(write!(f, " tim12rst"))}
      if self.tim7rst() != 0 { try!(write!(f, " tim7rst"))}
      if self.tim6rst() != 0 { try!(write!(f, " tim6rst"))}
      if self.tim5rst() != 0 { try!(write!(f, " tim5rst"))}
      if self.tim4rst() != 0 { try!(write!(f, " tim4rst"))}
      if self.tim3rst() != 0 { try!(write!(f, " tim3rst"))}
      if self.tim2rst() != 0 { try!(write!(f, " tim2rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB2 peripheral reset register"]
#[derive(PartialEq, Eq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
#[doc="TIM11 reset"]
  #[inline] pub fn tim11rst(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="TIM11 reset"]
  #[inline] pub fn set_tim11rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="TIM10 reset"]
  #[inline] pub fn tim10rst(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="TIM10 reset"]
  #[inline] pub fn set_tim10rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="TIM9 reset"]
  #[inline] pub fn tim9rst(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="TIM9 reset"]
  #[inline] pub fn set_tim9rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="System configuration controller reset"]
  #[inline] pub fn syscfgrst(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="System configuration controller reset"]
  #[inline] pub fn set_syscfgrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="SPI 1 reset"]
  #[inline] pub fn spi1rst(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="SPI 1 reset"]
  #[inline] pub fn set_spi1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="SDIO reset"]
  #[inline] pub fn sdiorst(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="SDIO reset"]
  #[inline] pub fn set_sdiorst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="ADC interface reset (common to all ADCs)"]
  #[inline] pub fn adcrst(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="ADC interface reset (common to all ADCs)"]
  #[inline] pub fn set_adcrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="USART6 reset"]
  #[inline] pub fn usart6rst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="USART6 reset"]
  #[inline] pub fn set_usart6rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="USART1 reset"]
  #[inline] pub fn usart1rst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="USART1 reset"]
  #[inline] pub fn set_usart1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TIM8 reset"]
  #[inline] pub fn tim8rst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="TIM8 reset"]
  #[inline] pub fn set_tim8rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TIM1 reset"]
  #[inline] pub fn tim1rst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="TIM1 reset"]
  #[inline] pub fn set_tim1rst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb2rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb2rstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim11rst() != 0 { try!(write!(f, " tim11rst"))}
      if self.tim10rst() != 0 { try!(write!(f, " tim10rst"))}
      if self.tim9rst() != 0 { try!(write!(f, " tim9rst"))}
      if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
      if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
      if self.sdiorst() != 0 { try!(write!(f, " sdiorst"))}
      if self.adcrst() != 0 { try!(write!(f, " adcrst"))}
      if self.usart6rst() != 0 { try!(write!(f, " usart6rst"))}
      if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
      if self.tim8rst() != 0 { try!(write!(f, " tim8rst"))}
      if self.tim1rst() != 0 { try!(write!(f, " tim1rst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB1 peripheral clock register"]
#[derive(PartialEq, Eq)]
pub struct Ahb1enr(pub u32);
impl Ahb1enr {
#[doc="USB OTG HSULPI clock enable"]
  #[inline] pub fn otghsulpien(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="USB OTG HSULPI clock enable"]
  #[inline] pub fn set_otghsulpien(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="USB OTG HS clock enable"]
  #[inline] pub fn otghsen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="USB OTG HS clock enable"]
  #[inline] pub fn set_otghsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Ethernet PTP clock enable"]
  #[inline] pub fn ethmacptpen(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Ethernet PTP clock enable"]
  #[inline] pub fn set_ethmacptpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Ethernet Reception clock enable"]
  #[inline] pub fn ethmacrxen(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="Ethernet Reception clock enable"]
  #[inline] pub fn set_ethmacrxen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Ethernet Transmission clock enable"]
  #[inline] pub fn ethmactxen(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="Ethernet Transmission clock enable"]
  #[inline] pub fn set_ethmactxen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Ethernet MAC clock enable"]
  #[inline] pub fn ethmacen(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Ethernet MAC clock enable"]
  #[inline] pub fn set_ethmacen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="DMA2 clock enable"]
  #[inline] pub fn dma2en(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="DMA2 clock enable"]
  #[inline] pub fn set_dma2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="DMA1 clock enable"]
  #[inline] pub fn dma1en(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="DMA1 clock enable"]
  #[inline] pub fn set_dma1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Backup SRAM interface clock enable"]
  #[inline] pub fn bkpsramen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Backup SRAM interface clock enable"]
  #[inline] pub fn set_bkpsramen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="CRC clock enable"]
  #[inline] pub fn crcen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="CRC clock enable"]
  #[inline] pub fn set_crcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="IO port I clock enable"]
  #[inline] pub fn gpioien(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="IO port I clock enable"]
  #[inline] pub fn set_gpioien(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="IO port H clock enable"]
  #[inline] pub fn gpiohen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="IO port H clock enable"]
  #[inline] pub fn set_gpiohen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="IO port G clock enable"]
  #[inline] pub fn gpiogen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="IO port G clock enable"]
  #[inline] pub fn set_gpiogen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="IO port F clock enable"]
  #[inline] pub fn gpiofen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="IO port F clock enable"]
  #[inline] pub fn set_gpiofen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="IO port E clock enable"]
  #[inline] pub fn gpioeen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="IO port E clock enable"]
  #[inline] pub fn set_gpioeen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="IO port D clock enable"]
  #[inline] pub fn gpioden(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="IO port D clock enable"]
  #[inline] pub fn set_gpioden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="IO port C clock enable"]
  #[inline] pub fn gpiocen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="IO port C clock enable"]
  #[inline] pub fn set_gpiocen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="IO port B clock enable"]
  #[inline] pub fn gpioben(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="IO port B clock enable"]
  #[inline] pub fn set_gpioben(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="IO port A clock enable"]
  #[inline] pub fn gpioaen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="IO port A clock enable"]
  #[inline] pub fn set_gpioaen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb1enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb1enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otghsulpien() != 0 { try!(write!(f, " otghsulpien"))}
      if self.otghsen() != 0 { try!(write!(f, " otghsen"))}
      if self.ethmacptpen() != 0 { try!(write!(f, " ethmacptpen"))}
      if self.ethmacrxen() != 0 { try!(write!(f, " ethmacrxen"))}
      if self.ethmactxen() != 0 { try!(write!(f, " ethmactxen"))}
      if self.ethmacen() != 0 { try!(write!(f, " ethmacen"))}
      if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
      if self.dma1en() != 0 { try!(write!(f, " dma1en"))}
      if self.bkpsramen() != 0 { try!(write!(f, " bkpsramen"))}
      if self.crcen() != 0 { try!(write!(f, " crcen"))}
      if self.gpioien() != 0 { try!(write!(f, " gpioien"))}
      if self.gpiohen() != 0 { try!(write!(f, " gpiohen"))}
      if self.gpiogen() != 0 { try!(write!(f, " gpiogen"))}
      if self.gpiofen() != 0 { try!(write!(f, " gpiofen"))}
      if self.gpioeen() != 0 { try!(write!(f, " gpioeen"))}
      if self.gpioden() != 0 { try!(write!(f, " gpioden"))}
      if self.gpiocen() != 0 { try!(write!(f, " gpiocen"))}
      if self.gpioben() != 0 { try!(write!(f, " gpioben"))}
      if self.gpioaen() != 0 { try!(write!(f, " gpioaen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB2 peripheral clock enable register"]
#[derive(PartialEq, Eq)]
pub struct Ahb2enr(pub u32);
impl Ahb2enr {
#[doc="USB OTG FS clock enable"]
  #[inline] pub fn otgfsen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="USB OTG FS clock enable"]
  #[inline] pub fn set_otgfsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Random number generator clock enable"]
  #[inline] pub fn rngen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Random number generator clock enable"]
  #[inline] pub fn set_rngen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Camera interface enable"]
  #[inline] pub fn dcmien(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Camera interface enable"]
  #[inline] pub fn set_dcmien(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb2enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb2enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otgfsen() != 0 { try!(write!(f, " otgfsen"))}
      if self.rngen() != 0 { try!(write!(f, " rngen"))}
      if self.dcmien() != 0 { try!(write!(f, " dcmien"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB3 peripheral clock enable register"]
#[derive(PartialEq, Eq)]
pub struct Ahb3enr(pub u32);
impl Ahb3enr {
#[doc="Flexible static memory controller module clock enable"]
  #[inline] pub fn fsmcen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Flexible static memory controller module clock enable"]
  #[inline] pub fn set_fsmcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb3enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb3enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fsmcen() != 0 { try!(write!(f, " fsmcen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB1 peripheral clock enable register"]
#[derive(PartialEq, Eq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
#[doc="DAC interface clock enable"]
  #[inline] pub fn dacen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="DAC interface clock enable"]
  #[inline] pub fn set_dacen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Power interface clock enable"]
  #[inline] pub fn pwren(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Power interface clock enable"]
  #[inline] pub fn set_pwren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="CAN 2 clock enable"]
  #[inline] pub fn can2en(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="CAN 2 clock enable"]
  #[inline] pub fn set_can2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="CAN 1 clock enable"]
  #[inline] pub fn can1en(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="CAN 1 clock enable"]
  #[inline] pub fn set_can1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="I2C3 clock enable"]
  #[inline] pub fn i2c3en(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="I2C3 clock enable"]
  #[inline] pub fn set_i2c3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="I2C2 clock enable"]
  #[inline] pub fn i2c2en(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="I2C2 clock enable"]
  #[inline] pub fn set_i2c2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="I2C1 clock enable"]
  #[inline] pub fn i2c1en(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="I2C1 clock enable"]
  #[inline] pub fn set_i2c1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="UART5 clock enable"]
  #[inline] pub fn uart5en(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="UART5 clock enable"]
  #[inline] pub fn set_uart5en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="UART4 clock enable"]
  #[inline] pub fn uart4en(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="UART4 clock enable"]
  #[inline] pub fn set_uart4en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="USART3 clock enable"]
  #[inline] pub fn usart3en(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="USART3 clock enable"]
  #[inline] pub fn set_usart3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="USART 2 clock enable"]
  #[inline] pub fn usart2en(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="USART 2 clock enable"]
  #[inline] pub fn set_usart2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="SPI3 clock enable"]
  #[inline] pub fn spi3en(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="SPI3 clock enable"]
  #[inline] pub fn set_spi3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="SPI2 clock enable"]
  #[inline] pub fn spi2en(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="SPI2 clock enable"]
  #[inline] pub fn set_spi2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Window watchdog clock enable"]
  #[inline] pub fn wwdgen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Window watchdog clock enable"]
  #[inline] pub fn set_wwdgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="TIM14 clock enable"]
  #[inline] pub fn tim14en(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="TIM14 clock enable"]
  #[inline] pub fn set_tim14en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="TIM13 clock enable"]
  #[inline] pub fn tim13en(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="TIM13 clock enable"]
  #[inline] pub fn set_tim13en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="TIM12 clock enable"]
  #[inline] pub fn tim12en(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="TIM12 clock enable"]
  #[inline] pub fn set_tim12en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="TIM7 clock enable"]
  #[inline] pub fn tim7en(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="TIM7 clock enable"]
  #[inline] pub fn set_tim7en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="TIM6 clock enable"]
  #[inline] pub fn tim6en(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="TIM6 clock enable"]
  #[inline] pub fn set_tim6en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TIM5 clock enable"]
  #[inline] pub fn tim5en(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="TIM5 clock enable"]
  #[inline] pub fn set_tim5en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="TIM4 clock enable"]
  #[inline] pub fn tim4en(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="TIM4 clock enable"]
  #[inline] pub fn set_tim4en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="TIM3 clock enable"]
  #[inline] pub fn tim3en(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="TIM3 clock enable"]
  #[inline] pub fn set_tim3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TIM2 clock enable"]
  #[inline] pub fn tim2en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="TIM2 clock enable"]
  #[inline] pub fn set_tim2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb1enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb1enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacen() != 0 { try!(write!(f, " dacen"))}
      if self.pwren() != 0 { try!(write!(f, " pwren"))}
      if self.can2en() != 0 { try!(write!(f, " can2en"))}
      if self.can1en() != 0 { try!(write!(f, " can1en"))}
      if self.i2c3en() != 0 { try!(write!(f, " i2c3en"))}
      if self.i2c2en() != 0 { try!(write!(f, " i2c2en"))}
      if self.i2c1en() != 0 { try!(write!(f, " i2c1en"))}
      if self.uart5en() != 0 { try!(write!(f, " uart5en"))}
      if self.uart4en() != 0 { try!(write!(f, " uart4en"))}
      if self.usart3en() != 0 { try!(write!(f, " usart3en"))}
      if self.usart2en() != 0 { try!(write!(f, " usart2en"))}
      if self.spi3en() != 0 { try!(write!(f, " spi3en"))}
      if self.spi2en() != 0 { try!(write!(f, " spi2en"))}
      if self.wwdgen() != 0 { try!(write!(f, " wwdgen"))}
      if self.tim14en() != 0 { try!(write!(f, " tim14en"))}
      if self.tim13en() != 0 { try!(write!(f, " tim13en"))}
      if self.tim12en() != 0 { try!(write!(f, " tim12en"))}
      if self.tim7en() != 0 { try!(write!(f, " tim7en"))}
      if self.tim6en() != 0 { try!(write!(f, " tim6en"))}
      if self.tim5en() != 0 { try!(write!(f, " tim5en"))}
      if self.tim4en() != 0 { try!(write!(f, " tim4en"))}
      if self.tim3en() != 0 { try!(write!(f, " tim3en"))}
      if self.tim2en() != 0 { try!(write!(f, " tim2en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB2 peripheral clock enable register"]
#[derive(PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
#[doc="SPI6 clock enable"]
  #[inline] pub fn spi6en(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="SPI6 clock enable"]
  #[inline] pub fn set_spi6en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="SPI5 clock enable"]
  #[inline] pub fn spi5en(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="SPI5 clock enable"]
  #[inline] pub fn set_spi5en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="TIM11 clock enable"]
  #[inline] pub fn tim11en(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="TIM11 clock enable"]
  #[inline] pub fn set_tim11en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="TIM10 clock enable"]
  #[inline] pub fn tim10en(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="TIM10 clock enable"]
  #[inline] pub fn set_tim10en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="TIM9 clock enable"]
  #[inline] pub fn tim9en(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="TIM9 clock enable"]
  #[inline] pub fn set_tim9en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="System configuration controller clock enable"]
  #[inline] pub fn syscfgen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="System configuration controller clock enable"]
  #[inline] pub fn set_syscfgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="SPI4 clock enable"]
  #[inline] pub fn spi4en(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="SPI4 clock enable"]
  #[inline] pub fn set_spi4en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="SPI1 clock enable"]
  #[inline] pub fn spi1en(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="SPI1 clock enable"]
  #[inline] pub fn set_spi1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="SDIO clock enable"]
  #[inline] pub fn sdioen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="SDIO clock enable"]
  #[inline] pub fn set_sdioen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="ADC3 clock enable"]
  #[inline] pub fn adc3en(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="ADC3 clock enable"]
  #[inline] pub fn set_adc3en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="ADC2 clock enable"]
  #[inline] pub fn adc2en(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="ADC2 clock enable"]
  #[inline] pub fn set_adc2en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="ADC1 clock enable"]
  #[inline] pub fn adc1en(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="ADC1 clock enable"]
  #[inline] pub fn set_adc1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="USART6 clock enable"]
  #[inline] pub fn usart6en(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="USART6 clock enable"]
  #[inline] pub fn set_usart6en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="USART1 clock enable"]
  #[inline] pub fn usart1en(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="USART1 clock enable"]
  #[inline] pub fn set_usart1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TIM8 clock enable"]
  #[inline] pub fn tim8en(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="TIM8 clock enable"]
  #[inline] pub fn set_tim8en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TIM1 clock enable"]
  #[inline] pub fn tim1en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="TIM1 clock enable"]
  #[inline] pub fn set_tim1en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb2enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb2enr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.spi6en() != 0 { try!(write!(f, " spi6en"))}
      if self.spi5en() != 0 { try!(write!(f, " spi5en"))}
      if self.tim11en() != 0 { try!(write!(f, " tim11en"))}
      if self.tim10en() != 0 { try!(write!(f, " tim10en"))}
      if self.tim9en() != 0 { try!(write!(f, " tim9en"))}
      if self.syscfgen() != 0 { try!(write!(f, " syscfgen"))}
      if self.spi4en() != 0 { try!(write!(f, " spi4en"))}
      if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
      if self.sdioen() != 0 { try!(write!(f, " sdioen"))}
      if self.adc3en() != 0 { try!(write!(f, " adc3en"))}
      if self.adc2en() != 0 { try!(write!(f, " adc2en"))}
      if self.adc1en() != 0 { try!(write!(f, " adc1en"))}
      if self.usart6en() != 0 { try!(write!(f, " usart6en"))}
      if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
      if self.tim8en() != 0 { try!(write!(f, " tim8en"))}
      if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB1 peripheral clock enable in low power mode register"]
#[derive(PartialEq, Eq)]
pub struct Ahb1lpenr(pub u32);
impl Ahb1lpenr {
#[doc="USB OTG HS ULPI clock enable during Sleep mode"]
  #[inline] pub fn otghsulpilpen(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="USB OTG HS ULPI clock enable during Sleep mode"]
  #[inline] pub fn set_otghsulpilpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="USB OTG HS clock enable during Sleep mode"]
  #[inline] pub fn otghslpen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="USB OTG HS clock enable during Sleep mode"]
  #[inline] pub fn set_otghslpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Ethernet PTP clock enable during Sleep mode"]
  #[inline] pub fn ethmacptplpen(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Ethernet PTP clock enable during Sleep mode"]
  #[inline] pub fn set_ethmacptplpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Ethernet reception clock enable during Sleep mode"]
  #[inline] pub fn ethmacrxlpen(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="Ethernet reception clock enable during Sleep mode"]
  #[inline] pub fn set_ethmacrxlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Ethernet transmission clock enable during Sleep mode"]
  #[inline] pub fn ethmactxlpen(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="Ethernet transmission clock enable during Sleep mode"]
  #[inline] pub fn set_ethmactxlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Ethernet MAC clock enable during Sleep mode"]
  #[inline] pub fn ethmaclpen(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Ethernet MAC clock enable during Sleep mode"]
  #[inline] pub fn set_ethmaclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="DMA2 clock enable during Sleep mode"]
  #[inline] pub fn dma2lpen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="DMA2 clock enable during Sleep mode"]
  #[inline] pub fn set_dma2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="DMA1 clock enable during Sleep mode"]
  #[inline] pub fn dma1lpen(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="DMA1 clock enable during Sleep mode"]
  #[inline] pub fn set_dma1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Backup SRAM interface clock enable during Sleep mode"]
  #[inline] pub fn bkpsramlpen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Backup SRAM interface clock enable during Sleep mode"]
  #[inline] pub fn set_bkpsramlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="SRAM 2 interface clock enable during Sleep mode"]
  #[inline] pub fn sram2lpen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="SRAM 2 interface clock enable during Sleep mode"]
  #[inline] pub fn set_sram2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="SRAM 1interface clock enable during Sleep mode"]
  #[inline] pub fn sram1lpen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="SRAM 1interface clock enable during Sleep mode"]
  #[inline] pub fn set_sram1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Flash interface clock enable during Sleep mode"]
  #[inline] pub fn flitflpen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Flash interface clock enable during Sleep mode"]
  #[inline] pub fn set_flitflpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="CRC clock enable during Sleep mode"]
  #[inline] pub fn crclpen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="CRC clock enable during Sleep mode"]
  #[inline] pub fn set_crclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="IO port I clock enable during Sleep mode"]
  #[inline] pub fn gpioilpen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="IO port I clock enable during Sleep mode"]
  #[inline] pub fn set_gpioilpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="IO port H clock enable during Sleep mode"]
  #[inline] pub fn gpiohlpen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="IO port H clock enable during Sleep mode"]
  #[inline] pub fn set_gpiohlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="IO port G clock enable during Sleep mode"]
  #[inline] pub fn gpioglpen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="IO port G clock enable during Sleep mode"]
  #[inline] pub fn set_gpioglpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="IO port F clock enable during Sleep mode"]
  #[inline] pub fn gpioflpen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="IO port F clock enable during Sleep mode"]
  #[inline] pub fn set_gpioflpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="IO port E clock enable during Sleep mode"]
  #[inline] pub fn gpioelpen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="IO port E clock enable during Sleep mode"]
  #[inline] pub fn set_gpioelpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="IO port D clock enable during Sleep mode"]
  #[inline] pub fn gpiodlpen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="IO port D clock enable during Sleep mode"]
  #[inline] pub fn set_gpiodlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="IO port C clock enable during Sleep mode"]
  #[inline] pub fn gpioclpen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="IO port C clock enable during Sleep mode"]
  #[inline] pub fn set_gpioclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="IO port B clock enable during Sleep mode"]
  #[inline] pub fn gpioblpen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="IO port B clock enable during Sleep mode"]
  #[inline] pub fn set_gpioblpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="IO port A clock enable during sleep mode"]
  #[inline] pub fn gpioalpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="IO port A clock enable during sleep mode"]
  #[inline] pub fn set_gpioalpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb1lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb1lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otghsulpilpen() != 0 { try!(write!(f, " otghsulpilpen"))}
      if self.otghslpen() != 0 { try!(write!(f, " otghslpen"))}
      if self.ethmacptplpen() != 0 { try!(write!(f, " ethmacptplpen"))}
      if self.ethmacrxlpen() != 0 { try!(write!(f, " ethmacrxlpen"))}
      if self.ethmactxlpen() != 0 { try!(write!(f, " ethmactxlpen"))}
      if self.ethmaclpen() != 0 { try!(write!(f, " ethmaclpen"))}
      if self.dma2lpen() != 0 { try!(write!(f, " dma2lpen"))}
      if self.dma1lpen() != 0 { try!(write!(f, " dma1lpen"))}
      if self.bkpsramlpen() != 0 { try!(write!(f, " bkpsramlpen"))}
      if self.sram2lpen() != 0 { try!(write!(f, " sram2lpen"))}
      if self.sram1lpen() != 0 { try!(write!(f, " sram1lpen"))}
      if self.flitflpen() != 0 { try!(write!(f, " flitflpen"))}
      if self.crclpen() != 0 { try!(write!(f, " crclpen"))}
      if self.gpioilpen() != 0 { try!(write!(f, " gpioilpen"))}
      if self.gpiohlpen() != 0 { try!(write!(f, " gpiohlpen"))}
      if self.gpioglpen() != 0 { try!(write!(f, " gpioglpen"))}
      if self.gpioflpen() != 0 { try!(write!(f, " gpioflpen"))}
      if self.gpioelpen() != 0 { try!(write!(f, " gpioelpen"))}
      if self.gpiodlpen() != 0 { try!(write!(f, " gpiodlpen"))}
      if self.gpioclpen() != 0 { try!(write!(f, " gpioclpen"))}
      if self.gpioblpen() != 0 { try!(write!(f, " gpioblpen"))}
      if self.gpioalpen() != 0 { try!(write!(f, " gpioalpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB2 peripheral clock enable in low power mode register"]
#[derive(PartialEq, Eq)]
pub struct Ahb2lpenr(pub u32);
impl Ahb2lpenr {
#[doc="USB OTG FS clock enable during Sleep mode"]
  #[inline] pub fn otgfslpen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="USB OTG FS clock enable during Sleep mode"]
  #[inline] pub fn set_otgfslpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Random number generator clock enable during Sleep mode"]
  #[inline] pub fn rnglpen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Random number generator clock enable during Sleep mode"]
  #[inline] pub fn set_rnglpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Camera interface enable during Sleep mode"]
  #[inline] pub fn dcmilpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Camera interface enable during Sleep mode"]
  #[inline] pub fn set_dcmilpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb2lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb2lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.otgfslpen() != 0 { try!(write!(f, " otgfslpen"))}
      if self.rnglpen() != 0 { try!(write!(f, " rnglpen"))}
      if self.dcmilpen() != 0 { try!(write!(f, " dcmilpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="AHB3 peripheral clock enable in low power mode register"]
#[derive(PartialEq, Eq)]
pub struct Ahb3lpenr(pub u32);
impl Ahb3lpenr {
#[doc="Flexible static memory controller module clock enable during Sleep mode"]
  #[inline] pub fn fsmclpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Flexible static memory controller module clock enable during Sleep mode"]
  #[inline] pub fn set_fsmclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ahb3lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ahb3lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fsmclpen() != 0 { try!(write!(f, " fsmclpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB1 peripheral clock enable in low power mode register"]
#[derive(PartialEq, Eq)]
pub struct Apb1lpenr(pub u32);
impl Apb1lpenr {
#[doc="DAC interface clock enable during Sleep mode"]
  #[inline] pub fn daclpen(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="DAC interface clock enable during Sleep mode"]
  #[inline] pub fn set_daclpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Power interface clock enable during Sleep mode"]
  #[inline] pub fn pwrlpen(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Power interface clock enable during Sleep mode"]
  #[inline] pub fn set_pwrlpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="CAN 2 clock enable during Sleep mode"]
  #[inline] pub fn can2lpen(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="CAN 2 clock enable during Sleep mode"]
  #[inline] pub fn set_can2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="CAN 1 clock enable during Sleep mode"]
  #[inline] pub fn can1lpen(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="CAN 1 clock enable during Sleep mode"]
  #[inline] pub fn set_can1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="I2C3 clock enable during Sleep mode"]
  #[inline] pub fn i2c3lpen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="I2C3 clock enable during Sleep mode"]
  #[inline] pub fn set_i2c3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="I2C2 clock enable during Sleep mode"]
  #[inline] pub fn i2c2lpen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="I2C2 clock enable during Sleep mode"]
  #[inline] pub fn set_i2c2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="I2C1 clock enable during Sleep mode"]
  #[inline] pub fn i2c1lpen(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="I2C1 clock enable during Sleep mode"]
  #[inline] pub fn set_i2c1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="UART5 clock enable during Sleep mode"]
  #[inline] pub fn uart5lpen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="UART5 clock enable during Sleep mode"]
  #[inline] pub fn set_uart5lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="UART4 clock enable during Sleep mode"]
  #[inline] pub fn uart4lpen(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="UART4 clock enable during Sleep mode"]
  #[inline] pub fn set_uart4lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="USART3 clock enable during Sleep mode"]
  #[inline] pub fn usart3lpen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="USART3 clock enable during Sleep mode"]
  #[inline] pub fn set_usart3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="USART2 clock enable during Sleep mode"]
  #[inline] pub fn usart2lpen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="USART2 clock enable during Sleep mode"]
  #[inline] pub fn set_usart2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="SPI3 clock enable during Sleep mode"]
  #[inline] pub fn spi3lpen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="SPI3 clock enable during Sleep mode"]
  #[inline] pub fn set_spi3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="SPI2 clock enable during Sleep mode"]
  #[inline] pub fn spi2lpen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="SPI2 clock enable during Sleep mode"]
  #[inline] pub fn set_spi2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Window watchdog clock enable during Sleep mode"]
  #[inline] pub fn wwdglpen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Window watchdog clock enable during Sleep mode"]
  #[inline] pub fn set_wwdglpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="TIM14 clock enable during Sleep mode"]
  #[inline] pub fn tim14lpen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="TIM14 clock enable during Sleep mode"]
  #[inline] pub fn set_tim14lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="TIM13 clock enable during Sleep mode"]
  #[inline] pub fn tim13lpen(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="TIM13 clock enable during Sleep mode"]
  #[inline] pub fn set_tim13lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="TIM12 clock enable during Sleep mode"]
  #[inline] pub fn tim12lpen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="TIM12 clock enable during Sleep mode"]
  #[inline] pub fn set_tim12lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="TIM7 clock enable during Sleep mode"]
  #[inline] pub fn tim7lpen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="TIM7 clock enable during Sleep mode"]
  #[inline] pub fn set_tim7lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="TIM6 clock enable during Sleep mode"]
  #[inline] pub fn tim6lpen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="TIM6 clock enable during Sleep mode"]
  #[inline] pub fn set_tim6lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TIM5 clock enable during Sleep mode"]
  #[inline] pub fn tim5lpen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="TIM5 clock enable during Sleep mode"]
  #[inline] pub fn set_tim5lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="TIM4 clock enable during Sleep mode"]
  #[inline] pub fn tim4lpen(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="TIM4 clock enable during Sleep mode"]
  #[inline] pub fn set_tim4lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="TIM3 clock enable during Sleep mode"]
  #[inline] pub fn tim3lpen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="TIM3 clock enable during Sleep mode"]
  #[inline] pub fn set_tim3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TIM2 clock enable during Sleep mode"]
  #[inline] pub fn tim2lpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="TIM2 clock enable during Sleep mode"]
  #[inline] pub fn set_tim2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb1lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb1lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.daclpen() != 0 { try!(write!(f, " daclpen"))}
      if self.pwrlpen() != 0 { try!(write!(f, " pwrlpen"))}
      if self.can2lpen() != 0 { try!(write!(f, " can2lpen"))}
      if self.can1lpen() != 0 { try!(write!(f, " can1lpen"))}
      if self.i2c3lpen() != 0 { try!(write!(f, " i2c3lpen"))}
      if self.i2c2lpen() != 0 { try!(write!(f, " i2c2lpen"))}
      if self.i2c1lpen() != 0 { try!(write!(f, " i2c1lpen"))}
      if self.uart5lpen() != 0 { try!(write!(f, " uart5lpen"))}
      if self.uart4lpen() != 0 { try!(write!(f, " uart4lpen"))}
      if self.usart3lpen() != 0 { try!(write!(f, " usart3lpen"))}
      if self.usart2lpen() != 0 { try!(write!(f, " usart2lpen"))}
      if self.spi3lpen() != 0 { try!(write!(f, " spi3lpen"))}
      if self.spi2lpen() != 0 { try!(write!(f, " spi2lpen"))}
      if self.wwdglpen() != 0 { try!(write!(f, " wwdglpen"))}
      if self.tim14lpen() != 0 { try!(write!(f, " tim14lpen"))}
      if self.tim13lpen() != 0 { try!(write!(f, " tim13lpen"))}
      if self.tim12lpen() != 0 { try!(write!(f, " tim12lpen"))}
      if self.tim7lpen() != 0 { try!(write!(f, " tim7lpen"))}
      if self.tim6lpen() != 0 { try!(write!(f, " tim6lpen"))}
      if self.tim5lpen() != 0 { try!(write!(f, " tim5lpen"))}
      if self.tim4lpen() != 0 { try!(write!(f, " tim4lpen"))}
      if self.tim3lpen() != 0 { try!(write!(f, " tim3lpen"))}
      if self.tim2lpen() != 0 { try!(write!(f, " tim2lpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="APB2 peripheral clock enabled in low power mode register"]
#[derive(PartialEq, Eq)]
pub struct Apb2lpenr(pub u32);
impl Apb2lpenr {
#[doc="TIM11 clock enable during Sleep mode"]
  #[inline] pub fn tim11lpen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="TIM11 clock enable during Sleep mode"]
  #[inline] pub fn set_tim11lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="TIM10 clock enable during Sleep mode"]
  #[inline] pub fn tim10lpen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="TIM10 clock enable during Sleep mode"]
  #[inline] pub fn set_tim10lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="TIM9 clock enable during sleep mode"]
  #[inline] pub fn tim9lpen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="TIM9 clock enable during sleep mode"]
  #[inline] pub fn set_tim9lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="System configuration controller clock enable during Sleep mode"]
  #[inline] pub fn syscfglpen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="System configuration controller clock enable during Sleep mode"]
  #[inline] pub fn set_syscfglpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="SPI 1 clock enable during Sleep mode"]
  #[inline] pub fn spi1lpen(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="SPI 1 clock enable during Sleep mode"]
  #[inline] pub fn set_spi1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="SDIO clock enable during Sleep mode"]
  #[inline] pub fn sdiolpen(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="SDIO clock enable during Sleep mode"]
  #[inline] pub fn set_sdiolpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="ADC 3 clock enable during Sleep mode"]
  #[inline] pub fn adc3lpen(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="ADC 3 clock enable during Sleep mode"]
  #[inline] pub fn set_adc3lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="ADC2 clock enable during Sleep mode"]
  #[inline] pub fn adc2lpen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="ADC2 clock enable during Sleep mode"]
  #[inline] pub fn set_adc2lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="ADC1 clock enable during Sleep mode"]
  #[inline] pub fn adc1lpen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="ADC1 clock enable during Sleep mode"]
  #[inline] pub fn set_adc1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="USART6 clock enable during Sleep mode"]
  #[inline] pub fn usart6lpen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="USART6 clock enable during Sleep mode"]
  #[inline] pub fn set_usart6lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="USART1 clock enable during Sleep mode"]
  #[inline] pub fn usart1lpen(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="USART1 clock enable during Sleep mode"]
  #[inline] pub fn set_usart1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="TIM8 clock enable during Sleep mode"]
  #[inline] pub fn tim8lpen(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="TIM8 clock enable during Sleep mode"]
  #[inline] pub fn set_tim8lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="TIM1 clock enable during Sleep mode"]
  #[inline] pub fn tim1lpen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="TIM1 clock enable during Sleep mode"]
  #[inline] pub fn set_tim1lpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Apb2lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb2lpenr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim11lpen() != 0 { try!(write!(f, " tim11lpen"))}
      if self.tim10lpen() != 0 { try!(write!(f, " tim10lpen"))}
      if self.tim9lpen() != 0 { try!(write!(f, " tim9lpen"))}
      if self.syscfglpen() != 0 { try!(write!(f, " syscfglpen"))}
      if self.spi1lpen() != 0 { try!(write!(f, " spi1lpen"))}
      if self.sdiolpen() != 0 { try!(write!(f, " sdiolpen"))}
      if self.adc3lpen() != 0 { try!(write!(f, " adc3lpen"))}
      if self.adc2lpen() != 0 { try!(write!(f, " adc2lpen"))}
      if self.adc1lpen() != 0 { try!(write!(f, " adc1lpen"))}
      if self.usart6lpen() != 0 { try!(write!(f, " usart6lpen"))}
      if self.usart1lpen() != 0 { try!(write!(f, " usart1lpen"))}
      if self.tim8lpen() != 0 { try!(write!(f, " tim8lpen"))}
      if self.tim1lpen() != 0 { try!(write!(f, " tim1lpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Backup domain control register"]
#[derive(PartialEq, Eq)]
pub struct Bdcr(pub u32);
impl Bdcr {
#[doc="Backup domain software reset"]
  #[inline] pub fn bdrst(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Backup domain software reset"]
  #[inline] pub fn set_bdrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="RTC clock enable"]
  #[inline] pub fn rtcen(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="RTC clock enable"]
  #[inline] pub fn set_rtcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="RTC clock source selection"]
  #[inline] pub fn rtcsel1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="RTC clock source selection"]
  #[inline] pub fn set_rtcsel1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="RTC clock source selection"]
  #[inline] pub fn rtcsel0(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="RTC clock source selection"]
  #[inline] pub fn set_rtcsel0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="External low-speed oscillator bypass"]
  #[inline] pub fn lsebyp(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="External low-speed oscillator bypass"]
  #[inline] pub fn set_lsebyp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="External low-speed oscillator ready"]
  #[inline] pub fn lserdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="External low-speed oscillator ready"]
  #[inline] pub fn set_lserdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="External low-speed oscillator enable"]
  #[inline] pub fn lseon(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="External low-speed oscillator enable"]
  #[inline] pub fn set_lseon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bdcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bdcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bdrst() != 0 { try!(write!(f, " bdrst"))}
      if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
      if self.rtcsel1() != 0 { try!(write!(f, " rtcsel1"))}
      if self.rtcsel0() != 0 { try!(write!(f, " rtcsel0"))}
      if self.lsebyp() != 0 { try!(write!(f, " lsebyp"))}
      if self.lserdy() != 0 { try!(write!(f, " lserdy"))}
      if self.lseon() != 0 { try!(write!(f, " lseon"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="clock control & status register"]
#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
#[doc="Low-power reset flag"]
  #[inline] pub fn lpwrrstf(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Low-power reset flag"]
  #[inline] pub fn set_lpwrrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Window watchdog reset flag"]
  #[inline] pub fn wwdgrstf(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Window watchdog reset flag"]
  #[inline] pub fn set_wwdgrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Independent watchdog reset flag"]
  #[inline] pub fn wdgrstf(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="Independent watchdog reset flag"]
  #[inline] pub fn set_wdgrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Software reset flag"]
  #[inline] pub fn sftrstf(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Software reset flag"]
  #[inline] pub fn set_sftrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="POR/PDR reset flag"]
  #[inline] pub fn porrstf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="POR/PDR reset flag"]
  #[inline] pub fn set_porrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="PIN reset flag"]
  #[inline] pub fn padrstf(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="PIN reset flag"]
  #[inline] pub fn set_padrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="BOR reset flag"]
  #[inline] pub fn borrstf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="BOR reset flag"]
  #[inline] pub fn set_borrstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Remove reset flag"]
  #[inline] pub fn rmvf(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Remove reset flag"]
  #[inline] pub fn set_rmvf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Internal low-speed oscillator ready"]
  #[inline] pub fn lsirdy(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Internal low-speed oscillator ready"]
  #[inline] pub fn set_lsirdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Internal low-speed oscillator enable"]
  #[inline] pub fn lsion(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Internal low-speed oscillator enable"]
  #[inline] pub fn set_lsion(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lpwrrstf() != 0 { try!(write!(f, " lpwrrstf"))}
      if self.wwdgrstf() != 0 { try!(write!(f, " wwdgrstf"))}
      if self.wdgrstf() != 0 { try!(write!(f, " wdgrstf"))}
      if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
      if self.porrstf() != 0 { try!(write!(f, " porrstf"))}
      if self.padrstf() != 0 { try!(write!(f, " padrstf"))}
      if self.borrstf() != 0 { try!(write!(f, " borrstf"))}
      if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
      if self.lsirdy() != 0 { try!(write!(f, " lsirdy"))}
      if self.lsion() != 0 { try!(write!(f, " lsion"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="spread spectrum clock generation register"]
#[derive(PartialEq, Eq)]
pub struct Sscgr(pub u32);
impl Sscgr {
#[doc="Spread spectrum modulation enable"]
  #[inline] pub fn sscgen(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Spread spectrum modulation enable"]
  #[inline] pub fn set_sscgen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Spread Select"]
  #[inline] pub fn spreadsel(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Spread Select"]
  #[inline] pub fn set_spreadsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Incrementation step"]
  #[inline] pub fn incstep(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7fff // [27:13]
  }
#[doc="Incrementation step"]
  #[inline] pub fn set_incstep(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Modulation period"]
  #[inline] pub fn modper(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1fff // [12:0]
  }
#[doc="Modulation period"]
  #[inline] pub fn set_modper(mut self, value: u32) -> Self {
     assert!((value & !0x1fff) == 0);
     self.0 &= !(0x1fff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sscgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sscgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sscgen() != 0 { try!(write!(f, " sscgen"))}
      if self.spreadsel() != 0 { try!(write!(f, " spreadsel"))}
      if self.incstep() != 0 { try!(write!(f, " incstep=0x{:x}", self.incstep()))}
      if self.modper() != 0 { try!(write!(f, " modper=0x{:x}", self.modper()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PLLI2S configuration register"]
#[derive(PartialEq, Eq)]
pub struct Plli2scfgr(pub u32);
impl Plli2scfgr {
#[doc="PLLI2S division factor for I2S clocks"]
  #[inline] pub fn plli2sr(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x7 // [30:28]
  }
#[doc="PLLI2S division factor for I2S clocks"]
  #[inline] pub fn set_plli2sr(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="PLLI2S multiplication factor for VCO"]
  #[inline] pub fn plli2sn(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1ff // [14:6]
  }
#[doc="PLLI2S multiplication factor for VCO"]
  #[inline] pub fn set_plli2sn(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Plli2scfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Plli2scfgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.plli2sr() != 0 { try!(write!(f, " plli2sr=0x{:x}", self.plli2sr()))}
      if self.plli2sn() != 0 { try!(write!(f, " plli2sn=0x{:x}", self.plli2sn()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait En {
   fn en(&self) -> u32;
   fn set_en(&self, value: u32);
}

impl Rcc {
   #[inline] pub fn en<P: En>(&self, p: &P) -> u32 {
      p.en()
   }
   #[inline] pub fn set_en<P: En>(&self, p: &P, value: u32) {
      p.set_en(value)
   }
}

pub trait Rst {
   fn rst(&self) -> u32;
   fn set_rst(&self, value: u32);
}

impl Rcc {
   #[inline] pub fn rst<P: Rst>(&self, p: &P) -> u32 {
      p.rst()
   }
   #[inline] pub fn set_rst<P: Rst>(&self, p: &P, value: u32) {
      p.set_rst(value)
   }
}

impl En for super::dma::Dma2 {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().dma2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_dma2en(value)); }
}

impl En for super::dma::Dma1 {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().dma1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_dma1en(value)); }
}

impl En for super::crc_24::Crc {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().crcen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_crcen(value)); }
}

impl En for super::gpio::Gpioi {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpioien() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpioien(value)); }
}

impl En for super::gpio::Gpioh {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpiohen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpiohen(value)); }
}

impl En for super::gpio::Gpiog {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpiogen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpiogen(value)); }
}

impl En for super::gpio::Gpiof {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpiofen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpiofen(value)); }
}

impl En for super::gpio::Gpioe {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpioeen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpioeen(value)); }
}

impl En for super::gpio::Gpiod {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpioden() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpioden(value)); }
}

impl En for super::gpio::Gpioc {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpiocen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpiocen(value)); }
}

impl En for super::gpio::Gpiob {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpioben() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpioben(value)); }
}

impl En for super::gpio::Gpioa {
   #[inline] fn en(&self) -> u32 { RCC.ahb1enr().gpioaen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_ahb1enr(|r| r.set_gpioaen(value)); }
}

impl En for super::i2c::I2c3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().i2c3en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c3en(value)); }
}

impl En for super::i2c::I2c2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().i2c2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c2en(value)); }
}

impl En for super::i2c::I2c1 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().i2c1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_i2c1en(value)); }
}

impl En for super::usart_f24::Uart5 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().uart5en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_uart5en(value)); }
}

impl En for super::usart_f24::Uart4 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().uart4en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_uart4en(value)); }
}

impl En for super::usart_f24::Usart3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().usart3en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_usart3en(value)); }
}

impl En for super::usart_f24::Usart2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().usart2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_usart2en(value)); }
}

impl En for super::spi::Spi3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().spi3en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_spi3en(value)); }
}

impl En for super::spi::Spi2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().spi2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_spi2en(value)); }
}

impl En for super::wwdg::Wwdg {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().wwdgen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

impl En for super::tim_gen::Tim14 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim14en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim14en(value)); }
}

impl En for super::tim_gen::Tim13 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim13en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim13en(value)); }
}

impl En for super::tim_gen::Tim12 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim12en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim12en(value)); }
}

impl En for super::tim_bas::Tim7 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim7en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim7en(value)); }
}

impl En for super::tim_bas::Tim6 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim6en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim6en(value)); }
}

impl En for super::tim_gen::Tim5 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim5en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim5en(value)); }
}

impl En for super::tim_gen::Tim4 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim4en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim4en(value)); }
}

impl En for super::tim_gen::Tim3 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim3en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim3en(value)); }
}

impl En for super::tim_gen::Tim2 {
   #[inline] fn en(&self) -> u32 { RCC.apb1enr().tim2en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb1enr(|r| r.set_tim2en(value)); }
}

impl En for super::tim_gen::Tim11 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim11en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim11en(value)); }
}

impl En for super::tim_gen::Tim10 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim10en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim10en(value)); }
}

impl En for super::tim_gen::Tim9 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim9en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim9en(value)); }
}

impl En for super::syscfg::Syscfg {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().syscfgen() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

impl En for super::spi::Spi1 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().spi1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_spi1en(value)); }
}

impl En for super::usart_f24::Usart6 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().usart6en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_usart6en(value)); }
}

impl En for super::usart_f24::Usart1 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().usart1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_usart1en(value)); }
}

impl En for super::tim_adv::Tim8 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim8en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim8en(value)); }
}

impl En for super::tim_adv::Tim1 {
   #[inline] fn en(&self) -> u32 { RCC.apb2enr().tim1en() }
   #[inline] fn set_en(&self, value: u32) { RCC.with_apb2enr(|r| r.set_tim1en(value)); }
}

