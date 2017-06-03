pub const GPIOA: Gpioa = Gpioa {};
pub const GPIOA_REF: &Gpioa = &GPIOA;
pub const GPIOA_IMPL: GpioImpl = GpioImpl(0x40004000);
pub const GPIOA_IMPL_REF: &GpioImpl = &GPIOA_IMPL;

pub struct Gpioa {}
impl ::core::ops::Deref for Gpioa {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOA_IMPL_REF }
}


pub const GPIOB: Gpiob = Gpiob {};
pub const GPIOB_REF: &Gpiob = &GPIOB;
pub const GPIOB_IMPL: GpioImpl = GpioImpl(0x40005000);
pub const GPIOB_IMPL_REF: &GpioImpl = &GPIOB_IMPL;

pub struct Gpiob {}
impl ::core::ops::Deref for Gpiob {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOB_IMPL_REF }
}


pub const GPIOC: Gpioc = Gpioc {};
pub const GPIOC_REF: &Gpioc = &GPIOC;
pub const GPIOC_IMPL: GpioImpl = GpioImpl(0x40006000);
pub const GPIOC_IMPL_REF: &GpioImpl = &GPIOC_IMPL;

pub struct Gpioc {}
impl ::core::ops::Deref for Gpioc {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOC_IMPL_REF }
}


pub const GPIOD: Gpiod = Gpiod {};
pub const GPIOD_REF: &Gpiod = &GPIOD;
pub const GPIOD_IMPL: GpioImpl = GpioImpl(0x40007000);
pub const GPIOD_IMPL_REF: &GpioImpl = &GPIOD_IMPL;

pub struct Gpiod {}
impl ::core::ops::Deref for Gpiod {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOD_IMPL_REF }
}


pub const GPIOE: Gpioe = Gpioe {};
pub const GPIOE_REF: &Gpioe = &GPIOE;
pub const GPIOE_IMPL: GpioImpl = GpioImpl(0x40024000);
pub const GPIOE_IMPL_REF: &GpioImpl = &GPIOE_IMPL;

pub struct Gpioe {}
impl ::core::ops::Deref for Gpioe {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOE_IMPL_REF }
}


pub const GPIOF: Gpiof = Gpiof {};
pub const GPIOF_REF: &Gpiof = &GPIOF;
pub const GPIOF_IMPL: GpioImpl = GpioImpl(0x40025000);
pub const GPIOF_IMPL_REF: &GpioImpl = &GPIOF_IMPL;

pub struct Gpiof {}
impl ::core::ops::Deref for Gpiof {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOF_IMPL_REF }
}


pub const GPIOG: Gpiog = Gpiog {};
pub const GPIOG_REF: &Gpiog = &GPIOG;
pub const GPIOG_IMPL: GpioImpl = GpioImpl(0x40026000);
pub const GPIOG_IMPL_REF: &GpioImpl = &GPIOG_IMPL;

pub struct Gpiog {}
impl ::core::ops::Deref for Gpiog {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOG_IMPL_REF }
}


pub const GPIOH: Gpioh = Gpioh {};
pub const GPIOH_REF: &Gpioh = &GPIOH;
pub const GPIOH_IMPL: GpioImpl = GpioImpl(0x40027000);
pub const GPIOH_IMPL_REF: &GpioImpl = &GPIOH_IMPL;

pub struct Gpioh {}
impl ::core::ops::Deref for Gpioh {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOH_IMPL_REF }
}


pub const GPIOJ: Gpioj = Gpioj {};
pub const GPIOJ_REF: &Gpioj = &GPIOJ;
pub const GPIOJ_IMPL: GpioImpl = GpioImpl(0x4003d000);
pub const GPIOJ_IMPL_REF: &GpioImpl = &GPIOJ_IMPL;

pub struct Gpioj {}
impl ::core::ops::Deref for Gpioj {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOJ_IMPL_REF }
}


pub const GPIOA_AHB: GpioaAhb = GpioaAhb {};
pub const GPIOA_AHB_REF: &GpioaAhb = &GPIOA_AHB;
pub const GPIOA_AHB_IMPL: GpioImpl = GpioImpl(0x40058000);
pub const GPIOA_AHB_IMPL_REF: &GpioImpl = &GPIOA_AHB_IMPL;

pub struct GpioaAhb {}
impl ::core::ops::Deref for GpioaAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOA_AHB_IMPL_REF }
}


pub const GPIOB_AHB: GpiobAhb = GpiobAhb {};
pub const GPIOB_AHB_REF: &GpiobAhb = &GPIOB_AHB;
pub const GPIOB_AHB_IMPL: GpioImpl = GpioImpl(0x40059000);
pub const GPIOB_AHB_IMPL_REF: &GpioImpl = &GPIOB_AHB_IMPL;

pub struct GpiobAhb {}
impl ::core::ops::Deref for GpiobAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOB_AHB_IMPL_REF }
}


pub const GPIOC_AHB: GpiocAhb = GpiocAhb {};
pub const GPIOC_AHB_REF: &GpiocAhb = &GPIOC_AHB;
pub const GPIOC_AHB_IMPL: GpioImpl = GpioImpl(0x4005a000);
pub const GPIOC_AHB_IMPL_REF: &GpioImpl = &GPIOC_AHB_IMPL;

pub struct GpiocAhb {}
impl ::core::ops::Deref for GpiocAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOC_AHB_IMPL_REF }
}


pub const GPIOD_AHB: GpiodAhb = GpiodAhb {};
pub const GPIOD_AHB_REF: &GpiodAhb = &GPIOD_AHB;
pub const GPIOD_AHB_IMPL: GpioImpl = GpioImpl(0x4005b000);
pub const GPIOD_AHB_IMPL_REF: &GpioImpl = &GPIOD_AHB_IMPL;

pub struct GpiodAhb {}
impl ::core::ops::Deref for GpiodAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOD_AHB_IMPL_REF }
}


pub const GPIOE_AHB: GpioeAhb = GpioeAhb {};
pub const GPIOE_AHB_REF: &GpioeAhb = &GPIOE_AHB;
pub const GPIOE_AHB_IMPL: GpioImpl = GpioImpl(0x4005c000);
pub const GPIOE_AHB_IMPL_REF: &GpioImpl = &GPIOE_AHB_IMPL;

pub struct GpioeAhb {}
impl ::core::ops::Deref for GpioeAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOE_AHB_IMPL_REF }
}


pub const GPIOF_AHB: GpiofAhb = GpiofAhb {};
pub const GPIOF_AHB_REF: &GpiofAhb = &GPIOF_AHB;
pub const GPIOF_AHB_IMPL: GpioImpl = GpioImpl(0x4005d000);
pub const GPIOF_AHB_IMPL_REF: &GpioImpl = &GPIOF_AHB_IMPL;

pub struct GpiofAhb {}
impl ::core::ops::Deref for GpiofAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOF_AHB_IMPL_REF }
}


pub const GPIOG_AHB: GpiogAhb = GpiogAhb {};
pub const GPIOG_AHB_REF: &GpiogAhb = &GPIOG_AHB;
pub const GPIOG_AHB_IMPL: GpioImpl = GpioImpl(0x4005e000);
pub const GPIOG_AHB_IMPL_REF: &GpioImpl = &GPIOG_AHB_IMPL;

pub struct GpiogAhb {}
impl ::core::ops::Deref for GpiogAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOG_AHB_IMPL_REF }
}


pub const GPIOH_AHB: GpiohAhb = GpiohAhb {};
pub const GPIOH_AHB_REF: &GpiohAhb = &GPIOH_AHB;
pub const GPIOH_AHB_IMPL: GpioImpl = GpioImpl(0x4005f000);
pub const GPIOH_AHB_IMPL_REF: &GpioImpl = &GPIOH_AHB_IMPL;

pub struct GpiohAhb {}
impl ::core::ops::Deref for GpiohAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOH_AHB_IMPL_REF }
}


pub const GPIOJ_AHB: GpiojAhb = GpiojAhb {};
pub const GPIOJ_AHB_REF: &GpiojAhb = &GPIOJ_AHB;
pub const GPIOJ_AHB_IMPL: GpioImpl = GpioImpl(0x40060000);
pub const GPIOJ_AHB_IMPL_REF: &GpioImpl = &GPIOJ_AHB_IMPL;

pub struct GpiojAhb {}
impl ::core::ops::Deref for GpiojAhb {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOJ_AHB_IMPL_REF }
}


pub const GPIOK: Gpiok = Gpiok {};
pub const GPIOK_REF: &Gpiok = &GPIOK;
pub const GPIOK_IMPL: GpioImpl = GpioImpl(0x40061000);
pub const GPIOK_IMPL_REF: &GpioImpl = &GPIOK_IMPL;

pub struct Gpiok {}
impl ::core::ops::Deref for Gpiok {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOK_IMPL_REF }
}


pub const GPIOL: Gpiol = Gpiol {};
pub const GPIOL_REF: &Gpiol = &GPIOL;
pub const GPIOL_IMPL: GpioImpl = GpioImpl(0x40062000);
pub const GPIOL_IMPL_REF: &GpioImpl = &GPIOL_IMPL;

pub struct Gpiol {}
impl ::core::ops::Deref for Gpiol {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOL_IMPL_REF }
}


pub const GPIOM: Gpiom = Gpiom {};
pub const GPIOM_REF: &Gpiom = &GPIOM;
pub const GPIOM_IMPL: GpioImpl = GpioImpl(0x40063000);
pub const GPIOM_IMPL_REF: &GpioImpl = &GPIOM_IMPL;

pub struct Gpiom {}
impl ::core::ops::Deref for Gpiom {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOM_IMPL_REF }
}


pub const GPION: Gpion = Gpion {};
pub const GPION_REF: &Gpion = &GPION;
pub const GPION_IMPL: GpioImpl = GpioImpl(0x40064000);
pub const GPION_IMPL_REF: &GpioImpl = &GPION_IMPL;

pub struct Gpion {}
impl ::core::ops::Deref for Gpion {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPION_IMPL_REF }
}


pub const GPIOP: Gpiop = Gpiop {};
pub const GPIOP_REF: &Gpiop = &GPIOP;
pub const GPIOP_IMPL: GpioImpl = GpioImpl(0x40065000);
pub const GPIOP_IMPL_REF: &GpioImpl = &GPIOP_IMPL;

pub struct Gpiop {}
impl ::core::ops::Deref for Gpiop {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOP_IMPL_REF }
}


pub const GPIOQ: Gpioq = Gpioq {};
pub const GPIOQ_REF: &Gpioq = &GPIOQ;
pub const GPIOQ_IMPL: GpioImpl = GpioImpl(0x40066000);
pub const GPIOQ_IMPL_REF: &GpioImpl = &GPIOQ_IMPL;

pub struct Gpioq {}
impl ::core::ops::Deref for Gpioq {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOQ_IMPL_REF }
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioImpl(pub u32);
impl GpioImpl {
  #[inline]
  pub fn data_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3fc) as *const u32
  }
  #[inline]
  pub fn data_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3fc) as *mut u32
  }
  #[inline]
  pub fn data(&self) -> Data { 
     unsafe {
       Data(::core::ptr::read_volatile(((self.0 as usize) + 0x3fc) as *const u32))
     }
  }
  #[inline]
  pub fn set_data(&self, value: Data) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x3fc) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &GpioImpl {
     let tmp = self.data();
     self.set_data(f(tmp))
  }

  #[inline]
  pub fn dir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x400) as *const u32
  }
  #[inline]
  pub fn dir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x400) as *mut u32
  }
  #[inline]
  pub fn dir(&self) -> Dir { 
     unsafe {
       Dir(::core::ptr::read_volatile(((self.0 as usize) + 0x400) as *const u32))
     }
  }
  #[inline]
  pub fn set_dir(&self, value: Dir) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x400) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &GpioImpl {
     let tmp = self.dir();
     self.set_dir(f(tmp))
  }

  #[inline]
  pub fn is_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x404) as *const u32
  }
  #[inline]
  pub fn is_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x404) as *mut u32
  }
  #[inline]
  pub fn is(&self) -> Is { 
     unsafe {
       Is(::core::ptr::read_volatile(((self.0 as usize) + 0x404) as *const u32))
     }
  }
  #[inline]
  pub fn set_is(&self, value: Is) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x404) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_is<F: FnOnce(Is) -> Is>(&self, f: F) -> &GpioImpl {
     let tmp = self.is();
     self.set_is(f(tmp))
  }

  #[inline]
  pub fn ibe_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x408) as *const u32
  }
  #[inline]
  pub fn ibe_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x408) as *mut u32
  }
  #[inline]
  pub fn ibe(&self) -> Ibe { 
     unsafe {
       Ibe(::core::ptr::read_volatile(((self.0 as usize) + 0x408) as *const u32))
     }
  }
  #[inline]
  pub fn set_ibe(&self, value: Ibe) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x408) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ibe<F: FnOnce(Ibe) -> Ibe>(&self, f: F) -> &GpioImpl {
     let tmp = self.ibe();
     self.set_ibe(f(tmp))
  }

  #[inline]
  pub fn iev_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40c) as *const u32
  }
  #[inline]
  pub fn iev_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40c) as *mut u32
  }
  #[inline]
  pub fn iev(&self) -> Iev { 
     unsafe {
       Iev(::core::ptr::read_volatile(((self.0 as usize) + 0x40c) as *const u32))
     }
  }
  #[inline]
  pub fn set_iev(&self, value: Iev) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x40c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_iev<F: FnOnce(Iev) -> Iev>(&self, f: F) -> &GpioImpl {
     let tmp = self.iev();
     self.set_iev(f(tmp))
  }

  #[inline]
  pub fn im_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x410) as *const u32
  }
  #[inline]
  pub fn im_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x410) as *mut u32
  }
  #[inline]
  pub fn im(&self) -> Im { 
     unsafe {
       Im(::core::ptr::read_volatile(((self.0 as usize) + 0x410) as *const u32))
     }
  }
  #[inline]
  pub fn set_im(&self, value: Im) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x410) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &GpioImpl {
     let tmp = self.im();
     self.set_im(f(tmp))
  }

  #[inline]
  pub fn ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x414) as *const u32
  }
  #[inline]
  pub fn ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x414) as *mut u32
  }
  #[inline]
  pub fn ris(&self) -> Ris { 
     unsafe {
       Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x414) as *const u32))
     }
  }
  #[inline]
  pub fn set_ris(&self, value: Ris) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x414) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &GpioImpl {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  #[inline]
  pub fn mis_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x418) as *const u32
  }
  #[inline]
  pub fn mis_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x418) as *mut u32
  }
  #[inline]
  pub fn mis(&self) -> Mis { 
     unsafe {
       Mis(::core::ptr::read_volatile(((self.0 as usize) + 0x418) as *const u32))
     }
  }
  #[inline]
  pub fn set_mis(&self, value: Mis) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x418) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &GpioImpl {
     let tmp = self.mis();
     self.set_mis(f(tmp))
  }

  #[inline]
  pub fn icr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x41c) as *const u32
  }
  #[inline]
  pub fn icr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x41c) as *mut u32
  }
  #[inline]
  pub fn set_icr(&self, value: Icr) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x41c) as *mut u32, value.0);
     }
     self
  }

  #[inline]
  pub fn afsel_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x420) as *const u32
  }
  #[inline]
  pub fn afsel_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x420) as *mut u32
  }
  #[inline]
  pub fn afsel(&self) -> Afsel { 
     unsafe {
       Afsel(::core::ptr::read_volatile(((self.0 as usize) + 0x420) as *const u32))
     }
  }
  #[inline]
  pub fn set_afsel(&self, value: Afsel) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x420) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_afsel<F: FnOnce(Afsel) -> Afsel>(&self, f: F) -> &GpioImpl {
     let tmp = self.afsel();
     self.set_afsel(f(tmp))
  }

  #[inline]
  pub fn dr2r_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x500) as *const u32
  }
  #[inline]
  pub fn dr2r_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x500) as *mut u32
  }
  #[inline]
  pub fn dr2r(&self) -> Dr2r { 
     unsafe {
       Dr2r(::core::ptr::read_volatile(((self.0 as usize) + 0x500) as *const u32))
     }
  }
  #[inline]
  pub fn set_dr2r(&self, value: Dr2r) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x500) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dr2r<F: FnOnce(Dr2r) -> Dr2r>(&self, f: F) -> &GpioImpl {
     let tmp = self.dr2r();
     self.set_dr2r(f(tmp))
  }

  #[inline]
  pub fn dr4r_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x504) as *const u32
  }
  #[inline]
  pub fn dr4r_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x504) as *mut u32
  }
  #[inline]
  pub fn dr4r(&self) -> Dr4r { 
     unsafe {
       Dr4r(::core::ptr::read_volatile(((self.0 as usize) + 0x504) as *const u32))
     }
  }
  #[inline]
  pub fn set_dr4r(&self, value: Dr4r) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x504) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dr4r<F: FnOnce(Dr4r) -> Dr4r>(&self, f: F) -> &GpioImpl {
     let tmp = self.dr4r();
     self.set_dr4r(f(tmp))
  }

  #[inline]
  pub fn dr8r_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x508) as *const u32
  }
  #[inline]
  pub fn dr8r_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x508) as *mut u32
  }
  #[inline]
  pub fn dr8r(&self) -> Dr8r { 
     unsafe {
       Dr8r(::core::ptr::read_volatile(((self.0 as usize) + 0x508) as *const u32))
     }
  }
  #[inline]
  pub fn set_dr8r(&self, value: Dr8r) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x508) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dr8r<F: FnOnce(Dr8r) -> Dr8r>(&self, f: F) -> &GpioImpl {
     let tmp = self.dr8r();
     self.set_dr8r(f(tmp))
  }

  #[inline]
  pub fn odr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50c) as *const u32
  }
  #[inline]
  pub fn odr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50c) as *mut u32
  }
  #[inline]
  pub fn odr(&self) -> Odr { 
     unsafe {
       Odr(::core::ptr::read_volatile(((self.0 as usize) + 0x50c) as *const u32))
     }
  }
  #[inline]
  pub fn set_odr(&self, value: Odr) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x50c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &GpioImpl {
     let tmp = self.odr();
     self.set_odr(f(tmp))
  }

  #[inline]
  pub fn pur_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x510) as *const u32
  }
  #[inline]
  pub fn pur_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x510) as *mut u32
  }
  #[inline]
  pub fn pur(&self) -> Pur { 
     unsafe {
       Pur(::core::ptr::read_volatile(((self.0 as usize) + 0x510) as *const u32))
     }
  }
  #[inline]
  pub fn set_pur(&self, value: Pur) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x510) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pur<F: FnOnce(Pur) -> Pur>(&self, f: F) -> &GpioImpl {
     let tmp = self.pur();
     self.set_pur(f(tmp))
  }

  #[inline]
  pub fn pdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x514) as *const u32
  }
  #[inline]
  pub fn pdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x514) as *mut u32
  }
  #[inline]
  pub fn pdr(&self) -> Pdr { 
     unsafe {
       Pdr(::core::ptr::read_volatile(((self.0 as usize) + 0x514) as *const u32))
     }
  }
  #[inline]
  pub fn set_pdr(&self, value: Pdr) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x514) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pdr<F: FnOnce(Pdr) -> Pdr>(&self, f: F) -> &GpioImpl {
     let tmp = self.pdr();
     self.set_pdr(f(tmp))
  }

  #[inline]
  pub fn slr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x518) as *const u32
  }
  #[inline]
  pub fn slr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x518) as *mut u32
  }
  #[inline]
  pub fn slr(&self) -> Slr { 
     unsafe {
       Slr(::core::ptr::read_volatile(((self.0 as usize) + 0x518) as *const u32))
     }
  }
  #[inline]
  pub fn set_slr(&self, value: Slr) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x518) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_slr<F: FnOnce(Slr) -> Slr>(&self, f: F) -> &GpioImpl {
     let tmp = self.slr();
     self.set_slr(f(tmp))
  }

  #[inline]
  pub fn den_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x51c) as *const u32
  }
  #[inline]
  pub fn den_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x51c) as *mut u32
  }
  #[inline]
  pub fn den(&self) -> Den { 
     unsafe {
       Den(::core::ptr::read_volatile(((self.0 as usize) + 0x51c) as *const u32))
     }
  }
  #[inline]
  pub fn set_den(&self, value: Den) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x51c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_den<F: FnOnce(Den) -> Den>(&self, f: F) -> &GpioImpl {
     let tmp = self.den();
     self.set_den(f(tmp))
  }

  #[inline]
  pub fn lock_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x520) as *const u32
  }
  #[inline]
  pub fn lock_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x520) as *mut u32
  }
  #[inline]
  pub fn lock(&self) -> Lock { 
     unsafe {
       Lock(::core::ptr::read_volatile(((self.0 as usize) + 0x520) as *const u32))
     }
  }
  #[inline]
  pub fn set_lock(&self, value: Lock) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x520) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &GpioImpl {
     let tmp = self.lock();
     self.set_lock(f(tmp))
  }

  #[inline]
  pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x524) as *const u32
  }
  #[inline]
  pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x524) as *mut u32
  }
  #[inline]
  pub fn cr(&self) -> Cr { 
     unsafe {
       Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x524) as *const u32))
     }
  }

  #[inline]
  pub fn amsel_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x528) as *const u32
  }
  #[inline]
  pub fn amsel_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x528) as *mut u32
  }
  #[inline]
  pub fn amsel(&self) -> Amsel { 
     unsafe {
       Amsel(::core::ptr::read_volatile(((self.0 as usize) + 0x528) as *const u32))
     }
  }
  #[inline]
  pub fn set_amsel(&self, value: Amsel) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x528) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_amsel<F: FnOnce(Amsel) -> Amsel>(&self, f: F) -> &GpioImpl {
     let tmp = self.amsel();
     self.set_amsel(f(tmp))
  }

  #[inline]
  pub fn pctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x52c) as *const u32
  }
  #[inline]
  pub fn pctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x52c) as *mut u32
  }
  #[inline]
  pub fn pctl(&self) -> Pctl { 
     unsafe {
       Pctl(::core::ptr::read_volatile(((self.0 as usize) + 0x52c) as *const u32))
     }
  }
  #[inline]
  pub fn set_pctl(&self, value: Pctl) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x52c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pctl<F: FnOnce(Pctl) -> Pctl>(&self, f: F) -> &GpioImpl {
     let tmp = self.pctl();
     self.set_pctl(f(tmp))
  }

  #[inline]
  pub fn adcctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x530) as *const u32
  }
  #[inline]
  pub fn adcctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x530) as *mut u32
  }
  #[inline]
  pub fn adcctl(&self) -> Adcctl { 
     unsafe {
       Adcctl(::core::ptr::read_volatile(((self.0 as usize) + 0x530) as *const u32))
     }
  }
  #[inline]
  pub fn set_adcctl(&self, value: Adcctl) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x530) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_adcctl<F: FnOnce(Adcctl) -> Adcctl>(&self, f: F) -> &GpioImpl {
     let tmp = self.adcctl();
     self.set_adcctl(f(tmp))
  }

  #[inline]
  pub fn dmactl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x534) as *const u32
  }
  #[inline]
  pub fn dmactl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x534) as *mut u32
  }
  #[inline]
  pub fn dmactl(&self) -> Dmactl { 
     unsafe {
       Dmactl(::core::ptr::read_volatile(((self.0 as usize) + 0x534) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmactl(&self, value: Dmactl) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x534) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &GpioImpl {
     let tmp = self.dmactl();
     self.set_dmactl(f(tmp))
  }

  #[inline]
  pub fn gpiosi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x539) as *const u32
  }
  #[inline]
  pub fn gpiosi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x539) as *mut u32
  }
  #[inline]
  pub fn gpiosi(&self) -> Gpiosi { 
     unsafe {
       Gpiosi(::core::ptr::read_volatile(((self.0 as usize) + 0x539) as *const u32))
     }
  }
  #[inline]
  pub fn set_gpiosi(&self, value: Gpiosi) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x539) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_gpiosi<F: FnOnce(Gpiosi) -> Gpiosi>(&self, f: F) -> &GpioImpl {
     let tmp = self.gpiosi();
     self.set_gpiosi(f(tmp))
  }

  #[inline]
  pub fn gpiodr12r_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x53c) as *const u32
  }
  #[inline]
  pub fn gpiodr12r_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x53c) as *mut u32
  }
  #[inline]
  pub fn gpiodr12r(&self) -> Gpiodr12r { 
     unsafe {
       Gpiodr12r(::core::ptr::read_volatile(((self.0 as usize) + 0x53c) as *const u32))
     }
  }
  #[inline]
  pub fn set_gpiodr12r(&self, value: Gpiodr12r) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x53c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_gpiodr12r<F: FnOnce(Gpiodr12r) -> Gpiodr12r>(&self, f: F) -> &GpioImpl {
     let tmp = self.gpiodr12r();
     self.set_gpiodr12r(f(tmp))
  }

  #[inline]
  pub fn gpiowakepen_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x540) as *const u32
  }
  #[inline]
  pub fn gpiowakepen_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x540) as *mut u32
  }
  #[inline]
  pub fn gpiowakepen(&self) -> Gpiowakepen { 
     unsafe {
       Gpiowakepen(::core::ptr::read_volatile(((self.0 as usize) + 0x540) as *const u32))
     }
  }
  #[inline]
  pub fn set_gpiowakepen(&self, value: Gpiowakepen) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x540) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_gpiowakepen<F: FnOnce(Gpiowakepen) -> Gpiowakepen>(&self, f: F) -> &GpioImpl {
     let tmp = self.gpiowakepen();
     self.set_gpiowakepen(f(tmp))
  }

  #[inline]
  pub fn gpiowakelvl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x544) as *const u32
  }
  #[inline]
  pub fn gpiowakelvl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x544) as *mut u32
  }
  #[inline]
  pub fn gpiowakelvl(&self) -> Gpiowakelvl { 
     unsafe {
       Gpiowakelvl(::core::ptr::read_volatile(((self.0 as usize) + 0x544) as *const u32))
     }
  }
  #[inline]
  pub fn set_gpiowakelvl(&self, value: Gpiowakelvl) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x544) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_gpiowakelvl<F: FnOnce(Gpiowakelvl) -> Gpiowakelvl>(&self, f: F) -> &GpioImpl {
     let tmp = self.gpiowakelvl();
     self.set_gpiowakelvl(f(tmp))
  }

  #[inline]
  pub fn gpiowakestat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x548) as *const u32
  }
  #[inline]
  pub fn gpiowakestat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x548) as *mut u32
  }
  #[inline]
  pub fn gpiowakestat(&self) -> Gpiowakestat { 
     unsafe {
       Gpiowakestat(::core::ptr::read_volatile(((self.0 as usize) + 0x548) as *const u32))
     }
  }
  #[inline]
  pub fn set_gpiowakestat(&self, value: Gpiowakestat) -> &GpioImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x548) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_gpiowakestat<F: FnOnce(Gpiowakestat) -> Gpiowakestat>(&self, f: F) -> &GpioImpl {
     let tmp = self.gpiowakestat();
     self.set_gpiowakestat(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
  #[inline]
  pub fn data(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_data(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data(0) != 0 { try!(write!(f, " data[0]"))}
      if self.data(1) != 0 { try!(write!(f, " data[1]"))}
      if self.data(2) != 0 { try!(write!(f, " data[2]"))}
      if self.data(3) != 0 { try!(write!(f, " data[3]"))}
      if self.data(4) != 0 { try!(write!(f, " data[4]"))}
      if self.data(5) != 0 { try!(write!(f, " data[5]"))}
      if self.data(6) != 0 { try!(write!(f, " data[6]"))}
      if self.data(7) != 0 { try!(write!(f, " data[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dir(pub u32);
impl Dir {
  #[inline]
  pub fn dir(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dir(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Dir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir(0) != 0 { try!(write!(f, " dir[0]"))}
      if self.dir(1) != 0 { try!(write!(f, " dir[1]"))}
      if self.dir(2) != 0 { try!(write!(f, " dir[2]"))}
      if self.dir(3) != 0 { try!(write!(f, " dir[3]"))}
      if self.dir(4) != 0 { try!(write!(f, " dir[4]"))}
      if self.dir(5) != 0 { try!(write!(f, " dir[5]"))}
      if self.dir(6) != 0 { try!(write!(f, " dir[6]"))}
      if self.dir(7) != 0 { try!(write!(f, " dir[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Is(pub u32);
impl Is {
  #[inline]
  pub fn is(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_is(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Is {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Is {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.is(0) != 0 { try!(write!(f, " is[0]"))}
      if self.is(1) != 0 { try!(write!(f, " is[1]"))}
      if self.is(2) != 0 { try!(write!(f, " is[2]"))}
      if self.is(3) != 0 { try!(write!(f, " is[3]"))}
      if self.is(4) != 0 { try!(write!(f, " is[4]"))}
      if self.is(5) != 0 { try!(write!(f, " is[5]"))}
      if self.is(6) != 0 { try!(write!(f, " is[6]"))}
      if self.is(7) != 0 { try!(write!(f, " is[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ibe(pub u32);
impl Ibe {
  #[inline]
  pub fn ibe(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ibe(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ibe {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ibe {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ibe(0) != 0 { try!(write!(f, " ibe[0]"))}
      if self.ibe(1) != 0 { try!(write!(f, " ibe[1]"))}
      if self.ibe(2) != 0 { try!(write!(f, " ibe[2]"))}
      if self.ibe(3) != 0 { try!(write!(f, " ibe[3]"))}
      if self.ibe(4) != 0 { try!(write!(f, " ibe[4]"))}
      if self.ibe(5) != 0 { try!(write!(f, " ibe[5]"))}
      if self.ibe(6) != 0 { try!(write!(f, " ibe[6]"))}
      if self.ibe(7) != 0 { try!(write!(f, " ibe[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Iev(pub u32);
impl Iev {
  #[inline]
  pub fn iev(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_iev(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Iev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Iev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iev(0) != 0 { try!(write!(f, " iev[0]"))}
      if self.iev(1) != 0 { try!(write!(f, " iev[1]"))}
      if self.iev(2) != 0 { try!(write!(f, " iev[2]"))}
      if self.iev(3) != 0 { try!(write!(f, " iev[3]"))}
      if self.iev(4) != 0 { try!(write!(f, " iev[4]"))}
      if self.iev(5) != 0 { try!(write!(f, " iev[5]"))}
      if self.iev(6) != 0 { try!(write!(f, " iev[6]"))}
      if self.iev(7) != 0 { try!(write!(f, " iev[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Im(pub u32);
impl Im {
  #[inline]
  pub fn im(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_im(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn dmaime(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_dmaime(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.im(0) != 0 { try!(write!(f, " im[0]"))}
      if self.im(1) != 0 { try!(write!(f, " im[1]"))}
      if self.im(2) != 0 { try!(write!(f, " im[2]"))}
      if self.im(3) != 0 { try!(write!(f, " im[3]"))}
      if self.im(4) != 0 { try!(write!(f, " im[4]"))}
      if self.im(5) != 0 { try!(write!(f, " im[5]"))}
      if self.im(6) != 0 { try!(write!(f, " im[6]"))}
      if self.im(7) != 0 { try!(write!(f, " im[7]"))}
      if self.dmaime() != 0 { try!(write!(f, " dmaime"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
  #[inline]
  pub fn ris(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ris(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn dmaris(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_dmaris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ris(0) != 0 { try!(write!(f, " ris[0]"))}
      if self.ris(1) != 0 { try!(write!(f, " ris[1]"))}
      if self.ris(2) != 0 { try!(write!(f, " ris[2]"))}
      if self.ris(3) != 0 { try!(write!(f, " ris[3]"))}
      if self.ris(4) != 0 { try!(write!(f, " ris[4]"))}
      if self.ris(5) != 0 { try!(write!(f, " ris[5]"))}
      if self.ris(6) != 0 { try!(write!(f, " ris[6]"))}
      if self.ris(7) != 0 { try!(write!(f, " ris[7]"))}
      if self.dmaris() != 0 { try!(write!(f, " dmaris"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
  #[inline]
  pub fn mis(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_mis(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn dmamis(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_dmamis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mis(0) != 0 { try!(write!(f, " mis[0]"))}
      if self.mis(1) != 0 { try!(write!(f, " mis[1]"))}
      if self.mis(2) != 0 { try!(write!(f, " mis[2]"))}
      if self.mis(3) != 0 { try!(write!(f, " mis[3]"))}
      if self.mis(4) != 0 { try!(write!(f, " mis[4]"))}
      if self.mis(5) != 0 { try!(write!(f, " mis[5]"))}
      if self.mis(6) != 0 { try!(write!(f, " mis[6]"))}
      if self.mis(7) != 0 { try!(write!(f, " mis[7]"))}
      if self.dmamis() != 0 { try!(write!(f, " dmamis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
  #[inline]
  pub fn icr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_icr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline]
  pub fn dmamic(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_dmamic(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.icr(0) != 0 { try!(write!(f, " icr[0]"))}
      if self.icr(1) != 0 { try!(write!(f, " icr[1]"))}
      if self.icr(2) != 0 { try!(write!(f, " icr[2]"))}
      if self.icr(3) != 0 { try!(write!(f, " icr[3]"))}
      if self.icr(4) != 0 { try!(write!(f, " icr[4]"))}
      if self.icr(5) != 0 { try!(write!(f, " icr[5]"))}
      if self.icr(6) != 0 { try!(write!(f, " icr[6]"))}
      if self.icr(7) != 0 { try!(write!(f, " icr[7]"))}
      if self.dmamic() != 0 { try!(write!(f, " dmamic"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Afsel(pub u32);
impl Afsel {
  #[inline]
  pub fn afsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_afsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Afsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Afsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.afsel(0) != 0 { try!(write!(f, " afsel[0]"))}
      if self.afsel(1) != 0 { try!(write!(f, " afsel[1]"))}
      if self.afsel(2) != 0 { try!(write!(f, " afsel[2]"))}
      if self.afsel(3) != 0 { try!(write!(f, " afsel[3]"))}
      if self.afsel(4) != 0 { try!(write!(f, " afsel[4]"))}
      if self.afsel(5) != 0 { try!(write!(f, " afsel[5]"))}
      if self.afsel(6) != 0 { try!(write!(f, " afsel[6]"))}
      if self.afsel(7) != 0 { try!(write!(f, " afsel[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dr2r(pub u32);
impl Dr2r {
  #[inline]
  pub fn drv2(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_drv2(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Dr2r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr2r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv2(0) != 0 { try!(write!(f, " drv2[0]"))}
      if self.drv2(1) != 0 { try!(write!(f, " drv2[1]"))}
      if self.drv2(2) != 0 { try!(write!(f, " drv2[2]"))}
      if self.drv2(3) != 0 { try!(write!(f, " drv2[3]"))}
      if self.drv2(4) != 0 { try!(write!(f, " drv2[4]"))}
      if self.drv2(5) != 0 { try!(write!(f, " drv2[5]"))}
      if self.drv2(6) != 0 { try!(write!(f, " drv2[6]"))}
      if self.drv2(7) != 0 { try!(write!(f, " drv2[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dr4r(pub u32);
impl Dr4r {
  #[inline]
  pub fn drv4(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_drv4(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Dr4r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr4r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv4(0) != 0 { try!(write!(f, " drv4[0]"))}
      if self.drv4(1) != 0 { try!(write!(f, " drv4[1]"))}
      if self.drv4(2) != 0 { try!(write!(f, " drv4[2]"))}
      if self.drv4(3) != 0 { try!(write!(f, " drv4[3]"))}
      if self.drv4(4) != 0 { try!(write!(f, " drv4[4]"))}
      if self.drv4(5) != 0 { try!(write!(f, " drv4[5]"))}
      if self.drv4(6) != 0 { try!(write!(f, " drv4[6]"))}
      if self.drv4(7) != 0 { try!(write!(f, " drv4[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dr8r(pub u32);
impl Dr8r {
  #[inline]
  pub fn drv8(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_drv8(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Dr8r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr8r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv8(0) != 0 { try!(write!(f, " drv8[0]"))}
      if self.drv8(1) != 0 { try!(write!(f, " drv8[1]"))}
      if self.drv8(2) != 0 { try!(write!(f, " drv8[2]"))}
      if self.drv8(3) != 0 { try!(write!(f, " drv8[3]"))}
      if self.drv8(4) != 0 { try!(write!(f, " drv8[4]"))}
      if self.drv8(5) != 0 { try!(write!(f, " drv8[5]"))}
      if self.drv8(6) != 0 { try!(write!(f, " drv8[6]"))}
      if self.drv8(7) != 0 { try!(write!(f, " drv8[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Odr(pub u32);
impl Odr {
  #[inline]
  pub fn ode(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ode(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ode(0) != 0 { try!(write!(f, " ode[0]"))}
      if self.ode(1) != 0 { try!(write!(f, " ode[1]"))}
      if self.ode(2) != 0 { try!(write!(f, " ode[2]"))}
      if self.ode(3) != 0 { try!(write!(f, " ode[3]"))}
      if self.ode(4) != 0 { try!(write!(f, " ode[4]"))}
      if self.ode(5) != 0 { try!(write!(f, " ode[5]"))}
      if self.ode(6) != 0 { try!(write!(f, " ode[6]"))}
      if self.ode(7) != 0 { try!(write!(f, " ode[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pur(pub u32);
impl Pur {
  #[inline]
  pub fn pue(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pue(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pue(0) != 0 { try!(write!(f, " pue[0]"))}
      if self.pue(1) != 0 { try!(write!(f, " pue[1]"))}
      if self.pue(2) != 0 { try!(write!(f, " pue[2]"))}
      if self.pue(3) != 0 { try!(write!(f, " pue[3]"))}
      if self.pue(4) != 0 { try!(write!(f, " pue[4]"))}
      if self.pue(5) != 0 { try!(write!(f, " pue[5]"))}
      if self.pue(6) != 0 { try!(write!(f, " pue[6]"))}
      if self.pue(7) != 0 { try!(write!(f, " pue[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pdr(pub u32);
impl Pdr {
  #[inline]
  pub fn pde(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pde(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pde(0) != 0 { try!(write!(f, " pde[0]"))}
      if self.pde(1) != 0 { try!(write!(f, " pde[1]"))}
      if self.pde(2) != 0 { try!(write!(f, " pde[2]"))}
      if self.pde(3) != 0 { try!(write!(f, " pde[3]"))}
      if self.pde(4) != 0 { try!(write!(f, " pde[4]"))}
      if self.pde(5) != 0 { try!(write!(f, " pde[5]"))}
      if self.pde(6) != 0 { try!(write!(f, " pde[6]"))}
      if self.pde(7) != 0 { try!(write!(f, " pde[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Slr(pub u32);
impl Slr {
  #[inline]
  pub fn slr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_slr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Slr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Slr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.slr(0) != 0 { try!(write!(f, " slr[0]"))}
      if self.slr(1) != 0 { try!(write!(f, " slr[1]"))}
      if self.slr(2) != 0 { try!(write!(f, " slr[2]"))}
      if self.slr(3) != 0 { try!(write!(f, " slr[3]"))}
      if self.slr(4) != 0 { try!(write!(f, " slr[4]"))}
      if self.slr(5) != 0 { try!(write!(f, " slr[5]"))}
      if self.slr(6) != 0 { try!(write!(f, " slr[6]"))}
      if self.slr(7) != 0 { try!(write!(f, " slr[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Den(pub u32);
impl Den {
  #[inline]
  pub fn den(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_den(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Den {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Den {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.den(0) != 0 { try!(write!(f, " den[0]"))}
      if self.den(1) != 0 { try!(write!(f, " den[1]"))}
      if self.den(2) != 0 { try!(write!(f, " den[2]"))}
      if self.den(3) != 0 { try!(write!(f, " den[3]"))}
      if self.den(4) != 0 { try!(write!(f, " den[4]"))}
      if self.den(5) != 0 { try!(write!(f, " den[5]"))}
      if self.den(6) != 0 { try!(write!(f, " den[6]"))}
      if self.den(7) != 0 { try!(write!(f, " den[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Lock(pub u32);
impl Lock {
  #[inline]
  pub fn gpio_lock(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
  pub fn set_gpio_lock(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Lock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
  #[inline]
  pub fn cr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_cr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.cr(0) != 0 { try!(write!(f, " cr[0]"))}
      if self.cr(1) != 0 { try!(write!(f, " cr[1]"))}
      if self.cr(2) != 0 { try!(write!(f, " cr[2]"))}
      if self.cr(3) != 0 { try!(write!(f, " cr[3]"))}
      if self.cr(4) != 0 { try!(write!(f, " cr[4]"))}
      if self.cr(5) != 0 { try!(write!(f, " cr[5]"))}
      if self.cr(6) != 0 { try!(write!(f, " cr[6]"))}
      if self.cr(7) != 0 { try!(write!(f, " cr[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Amsel(pub u32);
impl Amsel {
  #[inline]
  pub fn gpioamsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_gpioamsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Amsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Amsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gpioamsel(0) != 0 { try!(write!(f, " gpioamsel[0]"))}
      if self.gpioamsel(1) != 0 { try!(write!(f, " gpioamsel[1]"))}
      if self.gpioamsel(2) != 0 { try!(write!(f, " gpioamsel[2]"))}
      if self.gpioamsel(3) != 0 { try!(write!(f, " gpioamsel[3]"))}
      if self.gpioamsel(4) != 0 { try!(write!(f, " gpioamsel[4]"))}
      if self.gpioamsel(5) != 0 { try!(write!(f, " gpioamsel[5]"))}
      if self.gpioamsel(6) != 0 { try!(write!(f, " gpioamsel[6]"))}
      if self.gpioamsel(7) != 0 { try!(write!(f, " gpioamsel[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pctl(pub u32);
impl Pctl {
  #[inline]
  pub fn pmc(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_pmc(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pmc(0) != 0 { try!(write!(f, " pmc[0]=0x{:x}", self.pmc(0)))}
      if self.pmc(1) != 0 { try!(write!(f, " pmc[1]=0x{:x}", self.pmc(1)))}
      if self.pmc(2) != 0 { try!(write!(f, " pmc[2]=0x{:x}", self.pmc(2)))}
      if self.pmc(3) != 0 { try!(write!(f, " pmc[3]=0x{:x}", self.pmc(3)))}
      if self.pmc(4) != 0 { try!(write!(f, " pmc[4]=0x{:x}", self.pmc(4)))}
      if self.pmc(5) != 0 { try!(write!(f, " pmc[5]=0x{:x}", self.pmc(5)))}
      if self.pmc(6) != 0 { try!(write!(f, " pmc[6]=0x{:x}", self.pmc(6)))}
      if self.pmc(7) != 0 { try!(write!(f, " pmc[7]=0x{:x}", self.pmc(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Adcctl(pub u32);
impl Adcctl {
  #[inline]
  pub fn adcen(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_adcen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Adcctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Adcctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adcen(0) != 0 { try!(write!(f, " adcen[0]"))}
      if self.adcen(1) != 0 { try!(write!(f, " adcen[1]"))}
      if self.adcen(2) != 0 { try!(write!(f, " adcen[2]"))}
      if self.adcen(3) != 0 { try!(write!(f, " adcen[3]"))}
      if self.adcen(4) != 0 { try!(write!(f, " adcen[4]"))}
      if self.adcen(5) != 0 { try!(write!(f, " adcen[5]"))}
      if self.adcen(6) != 0 { try!(write!(f, " adcen[6]"))}
      if self.adcen(7) != 0 { try!(write!(f, " adcen[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmactl(pub u32);
impl Dmactl {
  #[inline]
  pub fn dmaen(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_dmaen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dmaen(0) != 0 { try!(write!(f, " dmaen[0]"))}
      if self.dmaen(1) != 0 { try!(write!(f, " dmaen[1]"))}
      if self.dmaen(2) != 0 { try!(write!(f, " dmaen[2]"))}
      if self.dmaen(3) != 0 { try!(write!(f, " dmaen[3]"))}
      if self.dmaen(4) != 0 { try!(write!(f, " dmaen[4]"))}
      if self.dmaen(5) != 0 { try!(write!(f, " dmaen[5]"))}
      if self.dmaen(6) != 0 { try!(write!(f, " dmaen[6]"))}
      if self.dmaen(7) != 0 { try!(write!(f, " dmaen[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Gpiosi(pub u32);
impl Gpiosi {
  #[inline]
  pub fn sum(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_sum(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Gpiosi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiosi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sum(0) != 0 { try!(write!(f, " sum[0]"))}
      if self.sum(1) != 0 { try!(write!(f, " sum[1]"))}
      if self.sum(2) != 0 { try!(write!(f, " sum[2]"))}
      if self.sum(3) != 0 { try!(write!(f, " sum[3]"))}
      if self.sum(4) != 0 { try!(write!(f, " sum[4]"))}
      if self.sum(5) != 0 { try!(write!(f, " sum[5]"))}
      if self.sum(6) != 0 { try!(write!(f, " sum[6]"))}
      if self.sum(7) != 0 { try!(write!(f, " sum[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Gpiodr12r(pub u32);
impl Gpiodr12r {
  #[inline]
  pub fn drv12(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline]
  pub fn set_drv12(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Gpiodr12r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiodr12r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv12(0) != 0 { try!(write!(f, " drv12[0]"))}
      if self.drv12(1) != 0 { try!(write!(f, " drv12[1]"))}
      if self.drv12(2) != 0 { try!(write!(f, " drv12[2]"))}
      if self.drv12(3) != 0 { try!(write!(f, " drv12[3]"))}
      if self.drv12(4) != 0 { try!(write!(f, " drv12[4]"))}
      if self.drv12(5) != 0 { try!(write!(f, " drv12[5]"))}
      if self.drv12(6) != 0 { try!(write!(f, " drv12[6]"))}
      if self.drv12(7) != 0 { try!(write!(f, " drv12[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Gpiowakepen(pub u32);
impl Gpiowakepen {
  #[inline]
  pub fn wakep7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_wakep7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn wakep6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_wakep6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn wakep5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_wakep5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn wakep4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_wakep4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}
impl ::core::fmt::Display for Gpiowakepen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiowakepen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakep7() != 0 { try!(write!(f, " wakep7"))}
      if self.wakep6() != 0 { try!(write!(f, " wakep6"))}
      if self.wakep5() != 0 { try!(write!(f, " wakep5"))}
      if self.wakep4() != 0 { try!(write!(f, " wakep4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Gpiowakelvl(pub u32);
impl Gpiowakelvl {
  #[inline]
  pub fn wakelvl7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_wakelvl7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn wakelvl6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_wakelvl6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn wakelvl5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_wakelvl5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn wakelvl4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_wakelvl4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}
impl ::core::fmt::Display for Gpiowakelvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiowakelvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakelvl7() != 0 { try!(write!(f, " wakelvl7"))}
      if self.wakelvl6() != 0 { try!(write!(f, " wakelvl6"))}
      if self.wakelvl5() != 0 { try!(write!(f, " wakelvl5"))}
      if self.wakelvl4() != 0 { try!(write!(f, " wakelvl4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Gpiowakestat(pub u32);
impl Gpiowakestat {
  #[inline]
  pub fn wakestat7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_wakestat7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn wakestat6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_wakestat6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn wakestat5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_wakestat5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn wakestat4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_wakestat4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}
impl ::core::fmt::Display for Gpiowakestat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gpiowakestat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakestat7() != 0 { try!(write!(f, " wakestat7"))}
      if self.wakestat6() != 0 { try!(write!(f, " wakestat6"))}
      if self.wakestat5() != 0 { try!(write!(f, " wakestat5"))}
      if self.wakestat4() != 0 { try!(write!(f, " wakestat4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct PinImpl {
  pub port: GpioImpl,
  pub index: usize,
}

pub trait Pin<T> {
   fn port(&self) -> T;
   fn index(&self) -> usize;
}

pub trait AltFn<T> {
   fn alt_fn(&self) -> usize;
}

pub const PA0: Pa0 = Pa0 {}; 
pub const PA0_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 0 };
pub const PA0_IMPL_REF: &PinImpl = &PA0_IMPL;

impl ::core::ops::Deref for Pa0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa0 {}

impl Pin<Gpioa> for Pa0 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0rx> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c9scl> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T0ccp0> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can0rx> for Pa0 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA1: Pa1 = Pa1 {}; 
pub const PA1_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 1 };
pub const PA1_IMPL_REF: &PinImpl = &PA1_IMPL;

impl ::core::ops::Deref for Pa1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa1 {}

impl Pin<Gpioa> for Pa1 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::U0tx> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c9sda> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T0ccp1> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can0tx> for Pa1 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA2: Pa2 = Pa2 {}; 
pub const PA2_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 2 };
pub const PA2_IMPL_REF: &PinImpl = &PA2_IMPL;

impl ::core::ops::Deref for Pa2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa2 {}

impl Pin<Gpioa> for Pa2 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::U4rx> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c8scl> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T1ccp0> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi0clk> for Pa2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA3: Pa3 = Pa3 {}; 
pub const PA3_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 3 };
pub const PA3_IMPL_REF: &PinImpl = &PA3_IMPL;

impl ::core::ops::Deref for Pa3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa3 {}

impl Pin<Gpioa> for Pa3 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::U4tx> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c8sda> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T1ccp1> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi0fss> for Pa3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA4: Pa4 = Pa4 {}; 
pub const PA4_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 4 };
pub const PA4_IMPL_REF: &PinImpl = &PA4_IMPL;

impl ::core::ops::Deref for Pa4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa4 {}

impl Pin<Gpioa> for Pa4 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::U3rx> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c7scl> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T2ccp0> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi0xdat0> for Pa4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA5: Pa5 = Pa5 {}; 
pub const PA5_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 5 };
pub const PA5_IMPL_REF: &PinImpl = &PA5_IMPL;

impl ::core::ops::Deref for Pa5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa5 {}

impl Pin<Gpioa> for Pa5 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::U3tx> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c7sda> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T2ccp1> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi0xdat1> for Pa5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA6: Pa6 = Pa6 {}; 
pub const PA6_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 6 };
pub const PA6_IMPL_REF: &PinImpl = &PA6_IMPL;

impl ::core::ops::Deref for Pa6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa6 {}

impl Pin<Gpioa> for Pa6 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::U2rx> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c6scl> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T3ccp0> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0epen> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi0xdat2> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Epi0s8> for Pa6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA7: Pa7 = Pa7 {}; 
pub const PA7_IMPL: PinImpl = PinImpl { port: GPIOA_IMPL, index: 7 };
pub const PA7_IMPL_REF: &PinImpl = &PA7_IMPL;

impl ::core::ops::Deref for Pa7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PA7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pa7 {}

impl Pin<Gpioa> for Pa7 {
   #[inline]
   fn port(&self) -> Gpioa { GPIOA }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::U2tx> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c6sda> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T3ccp1> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0pflt> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usb0epen> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Ssi0xdat3> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Epi0s9> for Pa7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB0: Pb0 = Pb0 {}; 
pub const PB0_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 0 };
pub const PB0_IMPL_REF: &PinImpl = &PB0_IMPL;

impl ::core::ops::Deref for Pb0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb0 {}

impl Pin<Gpiob> for Pb0 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Usb0id> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1rx> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c5scl> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T4ccp0> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can1rx> for Pb0 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB1: Pb1 = Pb1 {}; 
pub const PB1_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 1 };
pub const PB1_IMPL_REF: &PinImpl = &PB1_IMPL;

impl ::core::ops::Deref for Pb1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb1 {}

impl Pin<Gpiob> for Pb1 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usb0vbus> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1tx> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c5sda> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T4ccp1> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can1tx> for Pb1 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB2: Pb2 = Pb2 {}; 
pub const PB2_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 2 };
pub const PB2_IMPL_REF: &PinImpl = &PB2_IMPL;

impl ::core::ops::Deref for Pb2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb2 {}

impl Pin<Gpiob> for Pb2 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c0scl> for Pb2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T5ccp0> for Pb2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0stp> for Pb2 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s27> for Pb2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB3: Pb3 = Pb3 {}; 
pub const PB3_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 3 };
pub const PB3_IMPL_REF: &PinImpl = &PB3_IMPL;

impl ::core::ops::Deref for Pb3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb3 {}

impl Pin<Gpiob> for Pb3 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::I2c0sda> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T5ccp1> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0clk> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s28> for Pb3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB4: Pb4 = Pb4 {}; 
pub const PB4_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 4 };
pub const PB4_IMPL_REF: &PinImpl = &PB4_IMPL;

impl ::core::ops::Deref for Pb4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb4 {}

impl Pin<Gpiob> for Pb4 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ain10> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0cts> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c5scl> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ssi1fss> for Pb4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB5: Pb5 = Pb5 {}; 
pub const PB5_IMPL: PinImpl = PinImpl { port: GPIOB_IMPL, index: 5 };
pub const PB5_IMPL_REF: &PinImpl = &PB5_IMPL;

impl ::core::ops::Deref for Pb5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PB5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pb5 {}

impl Pin<Gpiob> for Pb5 {
   #[inline]
   fn port(&self) -> Gpiob { GPIOB }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ain11> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0rts> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c5sda> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ssi1clk> for Pb5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC0: Pc0 = Pc0 {}; 
pub const PC0_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 0 };
pub const PC0_IMPL_REF: &PinImpl = &PC0_IMPL;

impl ::core::ops::Deref for Pc0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc0 {}

impl Pin<Gpioc> for Pc0 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Tck> for Pc0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Swclk> for Pc0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC1: Pc1 = Pc1 {}; 
pub const PC1_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 1 };
pub const PC1_IMPL_REF: &PinImpl = &PC1_IMPL;

impl ::core::ops::Deref for Pc1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc1 {}

impl Pin<Gpioc> for Pc1 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Tms> for Pc1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Swdio> for Pc1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC2: Pc2 = Pc2 {}; 
pub const PC2_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 2 };
pub const PC2_IMPL_REF: &PinImpl = &PC2_IMPL;

impl ::core::ops::Deref for Pc2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc2 {}

impl Pin<Gpioc> for Pc2 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Tdi> for Pc2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC3: Pc3 = Pc3 {}; 
pub const PC3_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 3 };
pub const PC3_IMPL_REF: &PinImpl = &PC3_IMPL;

impl ::core::ops::Deref for Pc3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc3 {}

impl Pin<Gpioc> for Pc3 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Tdo> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Swo> for Pc3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC4: Pc4 = Pc4 {}; 
pub const PC4_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 4 };
pub const PC4_IMPL_REF: &PinImpl = &PC4_IMPL;

impl ::core::ops::Deref for Pc4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc4 {}

impl Pin<Gpioc> for Pc4 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::C1Neg> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U7rx> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s7> for Pc4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC5: Pc5 = Pc5 {}; 
pub const PC5_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 5 };
pub const PC5_IMPL_REF: &PinImpl = &PC5_IMPL;

impl ::core::ops::Deref for Pc5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc5 {}

impl Pin<Gpioc> for Pc5 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::C1Pos> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U7tx> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Rtcclk> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Epi0s6> for Pc5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC6: Pc6 = Pc6 {}; 
pub const PC6_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 6 };
pub const PC6_IMPL_REF: &PinImpl = &PC6_IMPL;

impl ::core::ops::Deref for Pc6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc6 {}

impl Pin<Gpioc> for Pc6 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::C0Neg> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U5rx> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s5> for Pc6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC7: Pc7 = Pc7 {}; 
pub const PC7_IMPL: PinImpl = PinImpl { port: GPIOC_IMPL, index: 7 };
pub const PC7_IMPL_REF: &PinImpl = &PC7_IMPL;

impl ::core::ops::Deref for Pc7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PC7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pc7 {}

impl Pin<Gpioc> for Pc7 {
   #[inline]
   fn port(&self) -> Gpioc { GPIOC }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::C0Pos> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U5tx> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s4> for Pc7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD0: Pd0 = Pd0 {}; 
pub const PD0_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 0 };
pub const PD0_IMPL_REF: &PinImpl = &PD0_IMPL;

impl ::core::ops::Deref for Pd0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd0 {}

impl Pin<Gpiod> for Pd0 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain15> for Pd0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c7scl> for Pd0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T0ccp0> for Pd0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::C0o> for Pd0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi2xdat1> for Pd0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD1: Pd1 = Pd1 {}; 
pub const PD1_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 1 };
pub const PD1_IMPL_REF: &PinImpl = &PD1_IMPL;

impl ::core::ops::Deref for Pd1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd1 {}

impl Pin<Gpiod> for Pd1 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain14> for Pd1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c7sda> for Pd1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T0ccp1> for Pd1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::C1o> for Pd1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi2xdat0> for Pd1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD2: Pd2 = Pd2 {}; 
pub const PD2_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 2 };
pub const PD2_IMPL_REF: &PinImpl = &PD2_IMPL;

impl ::core::ops::Deref for Pd2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd2 {}

impl Pin<Gpiod> for Pd2 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ain13> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c8scl> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T1ccp0> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::C2o> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi2fss> for Pd2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD3: Pd3 = Pd3 {}; 
pub const PD3_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 3 };
pub const PD3_IMPL_REF: &PinImpl = &PD3_IMPL;

impl ::core::ops::Deref for Pd3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd3 {}

impl Pin<Gpiod> for Pd3 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ain12> for Pd3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c8sda> for Pd3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T1ccp1> for Pd3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi2clk> for Pd3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD4: Pd4 = Pd4 {}; 
pub const PD4_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 4 };
pub const PD4_IMPL_REF: &PinImpl = &PD4_IMPL;

impl ::core::ops::Deref for Pd4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd4 {}

impl Pin<Gpiod> for Pd4 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ain7> for Pd4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U2rx> for Pd4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T3ccp0> for Pd4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi1xdat2> for Pd4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD5: Pd5 = Pd5 {}; 
pub const PD5_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 5 };
pub const PD5_IMPL_REF: &PinImpl = &PD5_IMPL;

impl ::core::ops::Deref for Pd5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd5 {}

impl Pin<Gpiod> for Pd5 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ain6> for Pd5 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U2tx> for Pd5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T3ccp1> for Pd5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi1xdat3> for Pd5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD6: Pd6 = Pd6 {}; 
pub const PD6_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 6 };
pub const PD6_IMPL_REF: &PinImpl = &PD6_IMPL;

impl ::core::ops::Deref for Pd6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd6 {}

impl Pin<Gpiod> for Pd6 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ain5> for Pd6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U2rts> for Pd6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T4ccp0> for Pd6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0epen> for Pd6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi2xdat3> for Pd6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD7: Pd7 = Pd7 {}; 
pub const PD7_IMPL: PinImpl = PinImpl { port: GPIOD_IMPL, index: 7 };
pub const PD7_IMPL_REF: &PinImpl = &PD7_IMPL;

impl ::core::ops::Deref for Pd7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PD7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pd7 {}

impl Pin<Gpiod> for Pd7 {
   #[inline]
   fn port(&self) -> Gpiod { GPIOD }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Ain4> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U2cts> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T4ccp1> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0pflt> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Nmi> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Ssi2xdat2> for Pd7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE0: Pe0 = Pe0 {}; 
pub const PE0_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 0 };
pub const PE0_IMPL_REF: &PinImpl = &PE0_IMPL;

impl ::core::ops::Deref for Pe0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe0 {}

impl Pin<Gpioe> for Pe0 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain3> for Pe0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1rts> for Pe0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PE1: Pe1 = Pe1 {}; 
pub const PE1_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 1 };
pub const PE1_IMPL_REF: &PinImpl = &PE1_IMPL;

impl ::core::ops::Deref for Pe1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe1 {}

impl Pin<Gpioe> for Pe1 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain2> for Pe1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1dsr> for Pe1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PE2: Pe2 = Pe2 {}; 
pub const PE2_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 2 };
pub const PE2_IMPL_REF: &PinImpl = &PE2_IMPL;

impl ::core::ops::Deref for Pe2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe2 {}

impl Pin<Gpioe> for Pe2 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ain1> for Pe2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1dcd> for Pe2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PE3: Pe3 = Pe3 {}; 
pub const PE3_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 3 };
pub const PE3_IMPL_REF: &PinImpl = &PE3_IMPL;

impl ::core::ops::Deref for Pe3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe3 {}

impl Pin<Gpioe> for Pe3 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ain0> for Pe3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1dtr> for Pe3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PE4: Pe4 = Pe4 {}; 
pub const PE4_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 4 };
pub const PE4_IMPL_REF: &PinImpl = &PE4_IMPL;

impl ::core::ops::Deref for Pe4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe4 {}

impl Pin<Gpioe> for Pe4 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Ain9> for Pe4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1ri> for Pe4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ssi1xdat0> for Pe4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE5: Pe5 = Pe5 {}; 
pub const PE5_IMPL: PinImpl = PinImpl { port: GPIOE_IMPL, index: 5 };
pub const PE5_IMPL_REF: &PinImpl = &PE5_IMPL;

impl ::core::ops::Deref for Pe5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PE5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pe5 {}

impl Pin<Gpioe> for Pe5 {
   #[inline]
   fn port(&self) -> Gpioe { GPIOE }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ain8> for Pe5 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ssi1xdat1> for Pe5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF0: Pf0 = Pf0 {}; 
pub const PF0_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 0 };
pub const PF0_IMPL_REF: &PinImpl = &PF0_IMPL;

impl ::core::ops::Deref for Pf0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf0 {}

impl Pin<Gpiof> for Pf0 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::En0led0> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm0> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3xdat1> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trd2> for Pf0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF1: Pf1 = Pf1 {}; 
pub const PF1_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 1 };
pub const PF1_IMPL_REF: &PinImpl = &PF1_IMPL;

impl ::core::ops::Deref for Pf1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf1 {}

impl Pin<Gpiof> for Pf1 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::En0led2> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm1> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3xdat0> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trd1> for Pf1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF2: Pf2 = Pf2 {}; 
pub const PF2_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 2 };
pub const PF2_IMPL_REF: &PinImpl = &PF2_IMPL;

impl ::core::ops::Deref for Pf2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf2 {}

impl Pin<Gpiof> for Pf2 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::M0pwm2> for Pf2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3fss> for Pf2 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trd0> for Pf2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF3: Pf3 = Pf3 {}; 
pub const PF3_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 3 };
pub const PF3_IMPL_REF: &PinImpl = &PF3_IMPL;

impl ::core::ops::Deref for Pf3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf3 {}

impl Pin<Gpiof> for Pf3 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::M0pwm3> for Pf3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3clk> for Pf3 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trclk> for Pf3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF4: Pf4 = Pf4 {}; 
pub const PF4_IMPL: PinImpl = PinImpl { port: GPIOF_IMPL, index: 4 };
pub const PF4_IMPL_REF: &PinImpl = &PF4_IMPL;

impl ::core::ops::Deref for Pf4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PF4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pf4 {}

impl Pin<Gpiof> for Pf4 {
   #[inline]
   fn port(&self) -> Gpiof { GPIOF }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::En0led1> for Pf4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0fault0> for Pf4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3xdat2> for Pf4 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trd3> for Pf4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG0: Pg0 = Pg0 {}; 
pub const PG0_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 0 };
pub const PG0_IMPL_REF: &PinImpl = &PG0_IMPL;

impl ::core::ops::Deref for Pg0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg0 {}

impl Pin<Gpiog> for Pg0 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c1scl> for Pg0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::En0pps> for Pg0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm4> for Pg0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s11> for Pg0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG1: Pg1 = Pg1 {}; 
pub const PG1_IMPL: PinImpl = PinImpl { port: GPIOG_IMPL, index: 1 };
pub const PG1_IMPL_REF: &PinImpl = &PG1_IMPL;

impl ::core::ops::Deref for Pg1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PG1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pg1 {}

impl Pin<Gpiog> for Pg1 {
   #[inline]
   fn port(&self) -> Gpiog { GPIOG }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c1sda> for Pg1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::M0pwm5> for Pg1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s10> for Pg1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH0: Ph0 = Ph0 {}; 
pub const PH0_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 0 };
pub const PH0_IMPL_REF: &PinImpl = &PH0_IMPL;

impl ::core::ops::Deref for Ph0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph0 {}

impl Pin<Gpioh> for Ph0 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0rts> for Ph0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s0> for Ph0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH1: Ph1 = Ph1 {}; 
pub const PH1_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 1 };
pub const PH1_IMPL_REF: &PinImpl = &PH1_IMPL;

impl ::core::ops::Deref for Ph1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph1 {}

impl Pin<Gpioh> for Ph1 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::U0cts> for Ph1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s1> for Ph1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH2: Ph2 = Ph2 {}; 
pub const PH2_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 2 };
pub const PH2_IMPL_REF: &PinImpl = &PH2_IMPL;

impl ::core::ops::Deref for Ph2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph2 {}

impl Pin<Gpioh> for Ph2 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::U0dcd> for Ph2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s2> for Ph2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH3: Ph3 = Ph3 {}; 
pub const PH3_IMPL: PinImpl = PinImpl { port: GPIOH_IMPL, index: 3 };
pub const PH3_IMPL_REF: &PinImpl = &PH3_IMPL;

impl ::core::ops::Deref for Ph3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PH3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ph3 {}

impl Pin<Gpioh> for Ph3 {
   #[inline]
   fn port(&self) -> Gpioh { GPIOH }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::U0dsr> for Ph3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s3> for Ph3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ0: Pj0 = Pj0 {}; 
pub const PJ0_IMPL: PinImpl = PinImpl { port: GPIOJ_IMPL, index: 0 };
pub const PJ0_IMPL_REF: &PinImpl = &PJ0_IMPL;

impl ::core::ops::Deref for Pj0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PJ0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pj0 {}

impl Pin<Gpioj> for Pj0 {
   #[inline]
   fn port(&self) -> Gpioj { GPIOJ }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::U3rx> for Pj0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::En0pps> for Pj0 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PJ1: Pj1 = Pj1 {}; 
pub const PJ1_IMPL: PinImpl = PinImpl { port: GPIOJ_IMPL, index: 1 };
pub const PJ1_IMPL_REF: &PinImpl = &PJ1_IMPL;

impl ::core::ops::Deref for Pj1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PJ1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pj1 {}

impl Pin<Gpioj> for Pj1 {
   #[inline]
   fn port(&self) -> Gpioj { GPIOJ }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::U3tx> for Pj1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PK0: Pk0 = Pk0 {}; 
pub const PK0_IMPL: PinImpl = PinImpl { port: GPIOK_IMPL, index: 0 };
pub const PK0_IMPL_REF: &PinImpl = &PK0_IMPL;

impl ::core::ops::Deref for Pk0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PK0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pk0 {}

impl Pin<Gpiok> for Pk0 {
   #[inline]
   fn port(&self) -> Gpiok { GPIOK }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ain16> for Pk0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U4rx> for Pk0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s0> for Pk0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK1: Pk1 = Pk1 {}; 
pub const PK1_IMPL: PinImpl = PinImpl { port: GPIOK_IMPL, index: 1 };
pub const PK1_IMPL_REF: &PinImpl = &PK1_IMPL;

impl ::core::ops::Deref for Pk1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PK1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pk1 {}

impl Pin<Gpiok> for Pk1 {
   #[inline]
   fn port(&self) -> Gpiok { GPIOK }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ain17> for Pk1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U4tx> for Pk1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s1> for Pk1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK2: Pk2 = Pk2 {}; 
pub const PK2_IMPL: PinImpl = PinImpl { port: GPIOK_IMPL, index: 2 };
pub const PK2_IMPL_REF: &PinImpl = &PK2_IMPL;

impl ::core::ops::Deref for Pk2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PK2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pk2 {}

impl Pin<Gpiok> for Pk2 {
   #[inline]
   fn port(&self) -> Gpiok { GPIOK }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ain18> for Pk2 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U4rts> for Pk2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s2> for Pk2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK3: Pk3 = Pk3 {}; 
pub const PK3_IMPL: PinImpl = PinImpl { port: GPIOK_IMPL, index: 3 };
pub const PK3_IMPL_REF: &PinImpl = &PK3_IMPL;

impl ::core::ops::Deref for Pk3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PK3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pk3 {}

impl Pin<Gpiok> for Pk3 {
   #[inline]
   fn port(&self) -> Gpiok { GPIOK }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ain19> for Pk3 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U4cts> for Pk3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s3> for Pk3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK4: Pk4 = Pk4 {}; 
pub const PK4_IMPL: PinImpl = PinImpl { port: GPIOK_IMPL, index: 4 };
pub const PK4_IMPL_REF: &PinImpl = &PK4_IMPL;

impl ::core::ops::Deref for Pk4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PK4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pk4 {}

impl Pin<Gpiok> for Pk4 {
   #[inline]
   fn port(&self) -> Gpiok { GPIOK }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::I2c3scl> for Pk4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::En0led0> for Pk4 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm6> for Pk4 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s32> for Pk4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK5: Pk5 = Pk5 {}; 
pub const PK5_IMPL: PinImpl = PinImpl { port: GPIOK_IMPL, index: 5 };
pub const PK5_IMPL_REF: &PinImpl = &PK5_IMPL;

impl ::core::ops::Deref for Pk5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PK5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pk5 {}

impl Pin<Gpiok> for Pk5 {
   #[inline]
   fn port(&self) -> Gpiok { GPIOK }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::I2c3sda> for Pk5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::En0led2> for Pk5 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm7> for Pk5 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s31> for Pk5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK6: Pk6 = Pk6 {}; 
pub const PK6_IMPL: PinImpl = PinImpl { port: GPIOK_IMPL, index: 6 };
pub const PK6_IMPL_REF: &PinImpl = &PK6_IMPL;

impl ::core::ops::Deref for Pk6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PK6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pk6 {}

impl Pin<Gpiok> for Pk6 {
   #[inline]
   fn port(&self) -> Gpiok { GPIOK }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::I2c4scl> for Pk6 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::En0led1> for Pk6 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0fault1> for Pk6 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s25> for Pk6 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK7: Pk7 = Pk7 {}; 
pub const PK7_IMPL: PinImpl = PinImpl { port: GPIOK_IMPL, index: 7 };
pub const PK7_IMPL_REF: &PinImpl = &PK7_IMPL;

impl ::core::ops::Deref for Pk7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PK7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pk7 {}

impl Pin<Gpiok> for Pk7 {
   #[inline]
   fn port(&self) -> Gpiok { GPIOK }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::U0ri> for Pk7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c4sda> for Pk7 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Rtcclk> for Pk7 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0fault2> for Pk7 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s24> for Pk7 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL0: Pl0 = Pl0 {}; 
pub const PL0_IMPL: PinImpl = PinImpl { port: GPIOL_IMPL, index: 0 };
pub const PL0_IMPL_REF: &PinImpl = &PL0_IMPL;

impl ::core::ops::Deref for Pl0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PL0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pl0 {}

impl Pin<Gpiol> for Pl0 {
   #[inline]
   fn port(&self) -> Gpiol { GPIOL }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c2sda> for Pl0 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::M0fault3> for Pl0 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0d0> for Pl0 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s16> for Pl0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL1: Pl1 = Pl1 {}; 
pub const PL1_IMPL: PinImpl = PinImpl { port: GPIOL_IMPL, index: 1 };
pub const PL1_IMPL_REF: &PinImpl = &PL1_IMPL;

impl ::core::ops::Deref for Pl1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PL1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pl1 {}

impl Pin<Gpiol> for Pl1 {
   #[inline]
   fn port(&self) -> Gpiol { GPIOL }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2scl> for Pl1 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Pha0> for Pl1 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0d1> for Pl1 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s17> for Pl1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL2: Pl2 = Pl2 {}; 
pub const PL2_IMPL: PinImpl = PinImpl { port: GPIOL_IMPL, index: 2 };
pub const PL2_IMPL_REF: &PinImpl = &PL2_IMPL;

impl ::core::ops::Deref for Pl2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PL2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pl2 {}

impl Pin<Gpiol> for Pl2 {
   #[inline]
   fn port(&self) -> Gpiol { GPIOL }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::C0o> for Pl2 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Phb0> for Pl2 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0d2> for Pl2 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s18> for Pl2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL3: Pl3 = Pl3 {}; 
pub const PL3_IMPL: PinImpl = PinImpl { port: GPIOL_IMPL, index: 3 };
pub const PL3_IMPL_REF: &PinImpl = &PL3_IMPL;

impl ::core::ops::Deref for Pl3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PL3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pl3 {}

impl Pin<Gpiol> for Pl3 {
   #[inline]
   fn port(&self) -> Gpiol { GPIOL }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::C1o> for Pl3 {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Idx0> for Pl3 {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0d3> for Pl3 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s19> for Pl3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL4: Pl4 = Pl4 {}; 
pub const PL4_IMPL: PinImpl = PinImpl { port: GPIOL_IMPL, index: 4 };
pub const PL4_IMPL_REF: &PinImpl = &PL4_IMPL;

impl ::core::ops::Deref for Pl4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PL4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pl4 {}

impl Pin<Gpiol> for Pl4 {
   #[inline]
   fn port(&self) -> Gpiol { GPIOL }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::T0ccp0> for Pl4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0d4> for Pl4 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s26> for Pl4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL5: Pl5 = Pl5 {}; 
pub const PL5_IMPL: PinImpl = PinImpl { port: GPIOL_IMPL, index: 5 };
pub const PL5_IMPL_REF: &PinImpl = &PL5_IMPL;

impl ::core::ops::Deref for Pl5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PL5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pl5 {}

impl Pin<Gpiol> for Pl5 {
   #[inline]
   fn port(&self) -> Gpiol { GPIOL }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::T0ccp1> for Pl5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0d5> for Pl5 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s33> for Pl5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL6: Pl6 = Pl6 {}; 
pub const PL6_IMPL: PinImpl = PinImpl { port: GPIOL_IMPL, index: 6 };
pub const PL6_IMPL_REF: &PinImpl = &PL6_IMPL;

impl ::core::ops::Deref for Pl6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PL6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pl6 {}

impl Pin<Gpiol> for Pl6 {
   #[inline]
   fn port(&self) -> Gpiol { GPIOL }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0dp> for Pl6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::T1ccp0> for Pl6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PL7: Pl7 = Pl7 {}; 
pub const PL7_IMPL: PinImpl = PinImpl { port: GPIOL_IMPL, index: 7 };
pub const PL7_IMPL_REF: &PinImpl = &PL7_IMPL;

impl ::core::ops::Deref for Pl7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PL7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pl7 {}

impl Pin<Gpiol> for Pl7 {
   #[inline]
   fn port(&self) -> Gpiol { GPIOL }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Usb0dm> for Pl7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::T1ccp1> for Pl7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PM0: Pm0 = Pm0 {}; 
pub const PM0_IMPL: PinImpl = PinImpl { port: GPIOM_IMPL, index: 0 };
pub const PM0_IMPL_REF: &PinImpl = &PM0_IMPL;

impl ::core::ops::Deref for Pm0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PM0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pm0 {}

impl Pin<Gpiom> for Pm0 {
   #[inline]
   fn port(&self) -> Gpiom { GPIOM }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::T2ccp0> for Pm0 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s15> for Pm0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PM1: Pm1 = Pm1 {}; 
pub const PM1_IMPL: PinImpl = PinImpl { port: GPIOM_IMPL, index: 1 };
pub const PM1_IMPL_REF: &PinImpl = &PM1_IMPL;

impl ::core::ops::Deref for Pm1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PM1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pm1 {}

impl Pin<Gpiom> for Pm1 {
   #[inline]
   fn port(&self) -> Gpiom { GPIOM }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::T2ccp1> for Pm1 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s14> for Pm1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PM2: Pm2 = Pm2 {}; 
pub const PM2_IMPL: PinImpl = PinImpl { port: GPIOM_IMPL, index: 2 };
pub const PM2_IMPL_REF: &PinImpl = &PM2_IMPL;

impl ::core::ops::Deref for Pm2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PM2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pm2 {}

impl Pin<Gpiom> for Pm2 {
   #[inline]
   fn port(&self) -> Gpiom { GPIOM }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::T3ccp0> for Pm2 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s13> for Pm2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PM3: Pm3 = Pm3 {}; 
pub const PM3_IMPL: PinImpl = PinImpl { port: GPIOM_IMPL, index: 3 };
pub const PM3_IMPL_REF: &PinImpl = &PM3_IMPL;

impl ::core::ops::Deref for Pm3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PM3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pm3 {}

impl Pin<Gpiom> for Pm3 {
   #[inline]
   fn port(&self) -> Gpiom { GPIOM }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::T3ccp1> for Pm3 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s12> for Pm3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PM4: Pm4 = Pm4 {}; 
pub const PM4_IMPL: PinImpl = PinImpl { port: GPIOM_IMPL, index: 4 };
pub const PM4_IMPL_REF: &PinImpl = &PM4_IMPL;

impl ::core::ops::Deref for Pm4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PM4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pm4 {}

impl Pin<Gpiom> for Pm4 {
   #[inline]
   fn port(&self) -> Gpiom { GPIOM }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::Tmpr3> for Pm4 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0cts> for Pm4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T4ccp0> for Pm4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PM5: Pm5 = Pm5 {}; 
pub const PM5_IMPL: PinImpl = PinImpl { port: GPIOM_IMPL, index: 5 };
pub const PM5_IMPL_REF: &PinImpl = &PM5_IMPL;

impl ::core::ops::Deref for Pm5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PM5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pm5 {}

impl Pin<Gpiom> for Pm5 {
   #[inline]
   fn port(&self) -> Gpiom { GPIOM }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::Tmpr2> for Pm5 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0dcd> for Pm5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T4ccp1> for Pm5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PM6: Pm6 = Pm6 {}; 
pub const PM6_IMPL: PinImpl = PinImpl { port: GPIOM_IMPL, index: 6 };
pub const PM6_IMPL_REF: &PinImpl = &PM6_IMPL;

impl ::core::ops::Deref for Pm6 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PM6_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pm6 {}

impl Pin<Gpiom> for Pm6 {
   #[inline]
   fn port(&self) -> Gpiom { GPIOM }
   #[inline]
   fn index(&self) -> usize { 6 }
}

impl AltFn<super::sig::Tmpr1> for Pm6 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0dsr> for Pm6 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T5ccp0> for Pm6 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PM7: Pm7 = Pm7 {}; 
pub const PM7_IMPL: PinImpl = PinImpl { port: GPIOM_IMPL, index: 7 };
pub const PM7_IMPL_REF: &PinImpl = &PM7_IMPL;

impl ::core::ops::Deref for Pm7 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PM7_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pm7 {}

impl Pin<Gpiom> for Pm7 {
   #[inline]
   fn port(&self) -> Gpiom { GPIOM }
   #[inline]
   fn index(&self) -> usize { 7 }
}

impl AltFn<super::sig::Tmpr0> for Pm7 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0ri> for Pm7 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T5ccp1> for Pm7 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PN0: Pn0 = Pn0 {}; 
pub const PN0_IMPL: PinImpl = PinImpl { port: GPION_IMPL, index: 0 };
pub const PN0_IMPL_REF: &PinImpl = &PN0_IMPL;

impl ::core::ops::Deref for Pn0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PN0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pn0 {}

impl Pin<Gpion> for Pn0 {
   #[inline]
   fn port(&self) -> Gpion { GPION }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1rts> for Pn0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PN1: Pn1 = Pn1 {}; 
pub const PN1_IMPL: PinImpl = PinImpl { port: GPION_IMPL, index: 1 };
pub const PN1_IMPL_REF: &PinImpl = &PN1_IMPL;

impl ::core::ops::Deref for Pn1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PN1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pn1 {}

impl Pin<Gpion> for Pn1 {
   #[inline]
   fn port(&self) -> Gpion { GPION }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::U1cts> for Pn1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PN2: Pn2 = Pn2 {}; 
pub const PN2_IMPL: PinImpl = PinImpl { port: GPION_IMPL, index: 2 };
pub const PN2_IMPL_REF: &PinImpl = &PN2_IMPL;

impl ::core::ops::Deref for Pn2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PN2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pn2 {}

impl Pin<Gpion> for Pn2 {
   #[inline]
   fn port(&self) -> Gpion { GPION }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::U1dcd> for Pn2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U2rts> for Pn2 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Epi0s29> for Pn2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PN3: Pn3 = Pn3 {}; 
pub const PN3_IMPL: PinImpl = PinImpl { port: GPION_IMPL, index: 3 };
pub const PN3_IMPL_REF: &PinImpl = &PN3_IMPL;

impl ::core::ops::Deref for Pn3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PN3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pn3 {}

impl Pin<Gpion> for Pn3 {
   #[inline]
   fn port(&self) -> Gpion { GPION }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::U1dsr> for Pn3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U2cts> for Pn3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Epi0s30> for Pn3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PN4: Pn4 = Pn4 {}; 
pub const PN4_IMPL: PinImpl = PinImpl { port: GPION_IMPL, index: 4 };
pub const PN4_IMPL_REF: &PinImpl = &PN4_IMPL;

impl ::core::ops::Deref for Pn4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PN4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pn4 {}

impl Pin<Gpion> for Pn4 {
   #[inline]
   fn port(&self) -> Gpion { GPION }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::U1dtr> for Pn4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U3rts> for Pn4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c2sda> for Pn4 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s34> for Pn4 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PN5: Pn5 = Pn5 {}; 
pub const PN5_IMPL: PinImpl = PinImpl { port: GPION_IMPL, index: 5 };
pub const PN5_IMPL_REF: &PinImpl = &PN5_IMPL;

impl ::core::ops::Deref for Pn5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PN5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pn5 {}

impl Pin<Gpion> for Pn5 {
   #[inline]
   fn port(&self) -> Gpion { GPION }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::U1ri> for Pn5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U3cts> for Pn5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c2scl> for Pn5 {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s35> for Pn5 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP0: Pp0 = Pp0 {}; 
pub const PP0_IMPL: PinImpl = PinImpl { port: GPIOP_IMPL, index: 0 };
pub const PP0_IMPL_REF: &PinImpl = &PP0_IMPL;

impl ::core::ops::Deref for Pp0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PP0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pp0 {}

impl Pin<Gpiop> for Pp0 {
   #[inline]
   fn port(&self) -> Gpiop { GPIOP }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::C2Pos> for Pp0 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U6rx> for Pp0 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ssi3xdat> for Pp0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP1: Pp1 = Pp1 {}; 
pub const PP1_IMPL: PinImpl = PinImpl { port: GPIOP_IMPL, index: 1 };
pub const PP1_IMPL_REF: &PinImpl = &PP1_IMPL;

impl ::core::ops::Deref for Pp1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PP1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pp1 {}

impl Pin<Gpiop> for Pp1 {
   #[inline]
   fn port(&self) -> Gpiop { GPIOP }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::C2Neg> for Pp1 {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U6tx> for Pp1 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ssi3xdat> for Pp1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP2: Pp2 = Pp2 {}; 
pub const PP2_IMPL: PinImpl = PinImpl { port: GPIOP_IMPL, index: 2 };
pub const PP2_IMPL_REF: &PinImpl = &PP2_IMPL;

impl ::core::ops::Deref for Pp2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PP2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pp2 {}

impl Pin<Gpiop> for Pp2 {
   #[inline]
   fn port(&self) -> Gpiop { GPIOP }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::U0dtr> for Pp2 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usb0nxt> for Pp2 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s29> for Pp2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP3: Pp3 = Pp3 {}; 
pub const PP3_IMPL: PinImpl = PinImpl { port: GPIOP_IMPL, index: 3 };
pub const PP3_IMPL_REF: &PinImpl = &PP3_IMPL;

impl ::core::ops::Deref for Pp3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PP3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pp3 {}

impl Pin<Gpiop> for Pp3 {
   #[inline]
   fn port(&self) -> Gpiop { GPIOP }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::U1cts> for Pp3 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U0dcd> for Pp3 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Rtcclk> for Pp3 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Usb0dir> for Pp3 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s30> for Pp3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP4: Pp4 = Pp4 {}; 
pub const PP4_IMPL: PinImpl = PinImpl { port: GPIOP_IMPL, index: 4 };
pub const PP4_IMPL_REF: &PinImpl = &PP4_IMPL;

impl ::core::ops::Deref for Pp4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PP4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pp4 {}

impl Pin<Gpiop> for Pp4 {
   #[inline]
   fn port(&self) -> Gpiop { GPIOP }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::U3rts> for Pp4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U0dsr> for Pp4 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usb0d7> for Pp4 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

pub const PP5: Pp5 = Pp5 {}; 
pub const PP5_IMPL: PinImpl = PinImpl { port: GPIOP_IMPL, index: 5 };
pub const PP5_IMPL_REF: &PinImpl = &PP5_IMPL;

impl ::core::ops::Deref for Pp5 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PP5_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pp5 {}

impl Pin<Gpiop> for Pp5 {
   #[inline]
   fn port(&self) -> Gpiop { GPIOP }
   #[inline]
   fn index(&self) -> usize { 5 }
}

impl AltFn<super::sig::U3cts> for Pp5 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2scl> for Pp5 {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usb0d6> for Pp5 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

pub const PQ0: Pq0 = Pq0 {}; 
pub const PQ0_IMPL: PinImpl = PinImpl { port: GPIOQ_IMPL, index: 0 };
pub const PQ0_IMPL_REF: &PinImpl = &PQ0_IMPL;

impl ::core::ops::Deref for Pq0 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PQ0_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pq0 {}

impl Pin<Gpioq> for Pq0 {
   #[inline]
   fn port(&self) -> Gpioq { GPIOQ }
   #[inline]
   fn index(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ssi3clk> for Pq0 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s20> for Pq0 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PQ1: Pq1 = Pq1 {}; 
pub const PQ1_IMPL: PinImpl = PinImpl { port: GPIOQ_IMPL, index: 1 };
pub const PQ1_IMPL_REF: &PinImpl = &PQ1_IMPL;

impl ::core::ops::Deref for Pq1 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PQ1_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pq1 {}

impl Pin<Gpioq> for Pq1 {
   #[inline]
   fn port(&self) -> Gpioq { GPIOQ }
   #[inline]
   fn index(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ssi3fss> for Pq1 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s21> for Pq1 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PQ2: Pq2 = Pq2 {}; 
pub const PQ2_IMPL: PinImpl = PinImpl { port: GPIOQ_IMPL, index: 2 };
pub const PQ2_IMPL_REF: &PinImpl = &PQ2_IMPL;

impl ::core::ops::Deref for Pq2 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PQ2_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pq2 {}

impl Pin<Gpioq> for Pq2 {
   #[inline]
   fn port(&self) -> Gpioq { GPIOQ }
   #[inline]
   fn index(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ssi3xdat0> for Pq2 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s22> for Pq2 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PQ3: Pq3 = Pq3 {}; 
pub const PQ3_IMPL: PinImpl = PinImpl { port: GPIOQ_IMPL, index: 3 };
pub const PQ3_IMPL_REF: &PinImpl = &PQ3_IMPL;

impl ::core::ops::Deref for Pq3 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PQ3_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pq3 {}

impl Pin<Gpioq> for Pq3 {
   #[inline]
   fn port(&self) -> Gpioq { GPIOQ }
   #[inline]
   fn index(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi3xdat1> for Pq3 {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s23> for Pq3 {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PQ4: Pq4 = Pq4 {}; 
pub const PQ4_IMPL: PinImpl = PinImpl { port: GPIOQ_IMPL, index: 4 };
pub const PQ4_IMPL_REF: &PinImpl = &PQ4_IMPL;

impl ::core::ops::Deref for Pq4 {
   type Target = PinImpl;
   #[inline]
   fn deref(&self) -> &PinImpl { PQ4_IMPL_REF }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pq4 {}

impl Pin<Gpioq> for Pq4 {
   #[inline]
   fn port(&self) -> Gpioq { GPIOQ }
   #[inline]
   fn index(&self) -> usize { 4 }
}

impl AltFn<super::sig::U1rx> for Pq4 {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Divsclk> for Pq4 {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

