pub const GPIOA: Gpioa = Periph(0x40004000, GpioaId {});
pub const GPIOB: Gpiob = Periph(0x40005000, GpiobId {});
pub const GPIOC: Gpioc = Periph(0x40006000, GpiocId {});
pub const GPIOD: Gpiod = Periph(0x40007000, GpiodId {});
pub const GPIOE: Gpioe = Periph(0x40024000, GpioeId {});
pub const GPIOF: Gpiof = Periph(0x40025000, GpiofId {});
pub const GPIOG: Gpiog = Periph(0x40026000, GpiogId {});
pub const GPIOH: Gpioh = Periph(0x40027000, GpiohId {});
pub const GPIOJ: Gpioj = Periph(0x4003d000, GpiojId {});
pub const GPIOA_AHB: GpioaAhb = Periph(0x40058000, GpioaAhbId {});
pub const GPIOB_AHB: GpiobAhb = Periph(0x40059000, GpiobAhbId {});
pub const GPIOC_AHB: GpiocAhb = Periph(0x4005a000, GpiocAhbId {});
pub const GPIOD_AHB: GpiodAhb = Periph(0x4005b000, GpiodAhbId {});
pub const GPIOE_AHB: GpioeAhb = Periph(0x4005c000, GpioeAhbId {});
pub const GPIOF_AHB: GpiofAhb = Periph(0x4005d000, GpiofAhbId {});
pub const GPIOG_AHB: GpiogAhb = Periph(0x4005e000, GpiogAhbId {});
pub const GPIOH_AHB: GpiohAhb = Periph(0x4005f000, GpiohAhbId {});
pub const GPIOJ_AHB: GpiojAhb = Periph(0x40060000, GpiojAhbId {});
pub const GPIOK: Gpiok = Periph(0x40061000, GpiokId {});
pub const GPIOL: Gpiol = Periph(0x40062000, GpiolId {});
pub const GPIOM: Gpiom = Periph(0x40063000, GpiomId {});
pub const GPION: Gpion = Periph(0x40064000, GpionId {});
pub const GPIOP: Gpiop = Periph(0x40065000, GpiopId {});
pub const GPIOQ: Gpioq = Periph(0x40066000, GpioqId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioaId {}
pub type Gpioa = Periph<GpioaId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiobId {}
pub type Gpiob = Periph<GpiobId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiocId {}
pub type Gpioc = Periph<GpiocId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiodId {}
pub type Gpiod = Periph<GpiodId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioeId {}
pub type Gpioe = Periph<GpioeId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiofId {}
pub type Gpiof = Periph<GpiofId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiogId {}
pub type Gpiog = Periph<GpiogId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiohId {}
pub type Gpioh = Periph<GpiohId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiojId {}
pub type Gpioj = Periph<GpiojId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioaAhbId {}
pub type GpioaAhb = Periph<GpioaAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiobAhbId {}
pub type GpiobAhb = Periph<GpiobAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiocAhbId {}
pub type GpiocAhb = Periph<GpiocAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiodAhbId {}
pub type GpiodAhb = Periph<GpiodAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioeAhbId {}
pub type GpioeAhb = Periph<GpioeAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiofAhbId {}
pub type GpiofAhb = Periph<GpiofAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiogAhbId {}
pub type GpiogAhb = Periph<GpiogAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiohAhbId {}
pub type GpiohAhb = Periph<GpiohAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiojAhbId {}
pub type GpiojAhb = Periph<GpiojAhbId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiokId {}
pub type Gpiok = Periph<GpiokId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiolId {}
pub type Gpiol = Periph<GpiolId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiomId {}
pub type Gpiom = Periph<GpiomId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpionId {}
pub type Gpion = Periph<GpionId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpiopId {}
pub type Gpiop = Periph<GpiopId>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioqId {}
pub type Gpioq = Periph<GpioqId>;


























impl<T> Periph<T> {
  #[inline] pub fn data_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3fc) as *const u32
  }
  #[inline] pub fn data_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3fc) as *mut u32
  }
  #[inline] pub fn data(&self) -> Data { 
     unsafe {
        Data(::core::ptr::read_volatile(((self.0 as usize) + 0x3fc) as *const u32))
     }
  }
  #[inline] pub fn set_data(&self, value: Data) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3fc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
     let tmp = self.data();
     self.set_data(f(tmp))
  }

  #[inline] pub fn dir_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x400) as *const u32
  }
  #[inline] pub fn dir_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x400) as *mut u32
  }
  #[inline] pub fn dir(&self) -> Dir { 
     unsafe {
        Dir(::core::ptr::read_volatile(((self.0 as usize) + 0x400) as *const u32))
     }
  }
  #[inline] pub fn set_dir(&self, value: Dir) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x400) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &Self {
     let tmp = self.dir();
     self.set_dir(f(tmp))
  }

  #[inline] pub fn is_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x404) as *const u32
  }
  #[inline] pub fn is_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x404) as *mut u32
  }
  #[inline] pub fn is(&self) -> Is { 
     unsafe {
        Is(::core::ptr::read_volatile(((self.0 as usize) + 0x404) as *const u32))
     }
  }
  #[inline] pub fn set_is(&self, value: Is) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x404) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_is<F: FnOnce(Is) -> Is>(&self, f: F) -> &Self {
     let tmp = self.is();
     self.set_is(f(tmp))
  }

  #[inline] pub fn ibe_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x408) as *const u32
  }
  #[inline] pub fn ibe_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x408) as *mut u32
  }
  #[inline] pub fn ibe(&self) -> Ibe { 
     unsafe {
        Ibe(::core::ptr::read_volatile(((self.0 as usize) + 0x408) as *const u32))
     }
  }
  #[inline] pub fn set_ibe(&self, value: Ibe) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x408) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ibe<F: FnOnce(Ibe) -> Ibe>(&self, f: F) -> &Self {
     let tmp = self.ibe();
     self.set_ibe(f(tmp))
  }

  #[inline] pub fn iev_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40c) as *const u32
  }
  #[inline] pub fn iev_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40c) as *mut u32
  }
  #[inline] pub fn iev(&self) -> Iev { 
     unsafe {
        Iev(::core::ptr::read_volatile(((self.0 as usize) + 0x40c) as *const u32))
     }
  }
  #[inline] pub fn set_iev(&self, value: Iev) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_iev<F: FnOnce(Iev) -> Iev>(&self, f: F) -> &Self {
     let tmp = self.iev();
     self.set_iev(f(tmp))
  }

  #[inline] pub fn im_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x410) as *const u32
  }
  #[inline] pub fn im_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x410) as *mut u32
  }
  #[inline] pub fn im(&self) -> Im { 
     unsafe {
        Im(::core::ptr::read_volatile(((self.0 as usize) + 0x410) as *const u32))
     }
  }
  #[inline] pub fn set_im(&self, value: Im) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x410) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
     let tmp = self.im();
     self.set_im(f(tmp))
  }

  #[inline] pub fn ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x414) as *const u32
  }
  #[inline] pub fn ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x414) as *mut u32
  }
  #[inline] pub fn ris(&self) -> Ris { 
     unsafe {
        Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x414) as *const u32))
     }
  }
  #[inline] pub fn set_ris(&self, value: Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x414) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  #[inline] pub fn mis_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x418) as *const u32
  }
  #[inline] pub fn mis_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x418) as *mut u32
  }
  #[inline] pub fn mis(&self) -> Mis { 
     unsafe {
        Mis(::core::ptr::read_volatile(((self.0 as usize) + 0x418) as *const u32))
     }
  }
  #[inline] pub fn set_mis(&self, value: Mis) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x418) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
     let tmp = self.mis();
     self.set_mis(f(tmp))
  }

  #[inline] pub fn icr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x41c) as *const u32
  }
  #[inline] pub fn icr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x41c) as *mut u32
  }
  #[inline] pub fn set_icr(&self, value: Icr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x41c) as *mut u32, value.0);
     }
     self
  }

  #[inline] pub fn afsel_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x420) as *const u32
  }
  #[inline] pub fn afsel_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x420) as *mut u32
  }
  #[inline] pub fn afsel(&self) -> Afsel { 
     unsafe {
        Afsel(::core::ptr::read_volatile(((self.0 as usize) + 0x420) as *const u32))
     }
  }
  #[inline] pub fn set_afsel(&self, value: Afsel) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x420) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_afsel<F: FnOnce(Afsel) -> Afsel>(&self, f: F) -> &Self {
     let tmp = self.afsel();
     self.set_afsel(f(tmp))
  }

  #[inline] pub fn dr2r_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x500) as *const u32
  }
  #[inline] pub fn dr2r_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x500) as *mut u32
  }
  #[inline] pub fn dr2r(&self) -> Dr2r { 
     unsafe {
        Dr2r(::core::ptr::read_volatile(((self.0 as usize) + 0x500) as *const u32))
     }
  }
  #[inline] pub fn set_dr2r(&self, value: Dr2r) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x500) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dr2r<F: FnOnce(Dr2r) -> Dr2r>(&self, f: F) -> &Self {
     let tmp = self.dr2r();
     self.set_dr2r(f(tmp))
  }

  #[inline] pub fn dr4r_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x504) as *const u32
  }
  #[inline] pub fn dr4r_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x504) as *mut u32
  }
  #[inline] pub fn dr4r(&self) -> Dr4r { 
     unsafe {
        Dr4r(::core::ptr::read_volatile(((self.0 as usize) + 0x504) as *const u32))
     }
  }
  #[inline] pub fn set_dr4r(&self, value: Dr4r) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x504) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dr4r<F: FnOnce(Dr4r) -> Dr4r>(&self, f: F) -> &Self {
     let tmp = self.dr4r();
     self.set_dr4r(f(tmp))
  }

  #[inline] pub fn dr8r_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x508) as *const u32
  }
  #[inline] pub fn dr8r_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x508) as *mut u32
  }
  #[inline] pub fn dr8r(&self) -> Dr8r { 
     unsafe {
        Dr8r(::core::ptr::read_volatile(((self.0 as usize) + 0x508) as *const u32))
     }
  }
  #[inline] pub fn set_dr8r(&self, value: Dr8r) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x508) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dr8r<F: FnOnce(Dr8r) -> Dr8r>(&self, f: F) -> &Self {
     let tmp = self.dr8r();
     self.set_dr8r(f(tmp))
  }

  #[inline] pub fn odr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50c) as *const u32
  }
  #[inline] pub fn odr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50c) as *mut u32
  }
  #[inline] pub fn odr(&self) -> Odr { 
     unsafe {
        Odr(::core::ptr::read_volatile(((self.0 as usize) + 0x50c) as *const u32))
     }
  }
  #[inline] pub fn set_odr(&self, value: Odr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
     let tmp = self.odr();
     self.set_odr(f(tmp))
  }

  #[inline] pub fn pur_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x510) as *const u32
  }
  #[inline] pub fn pur_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x510) as *mut u32
  }
  #[inline] pub fn pur(&self) -> Pur { 
     unsafe {
        Pur(::core::ptr::read_volatile(((self.0 as usize) + 0x510) as *const u32))
     }
  }
  #[inline] pub fn set_pur(&self, value: Pur) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x510) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pur<F: FnOnce(Pur) -> Pur>(&self, f: F) -> &Self {
     let tmp = self.pur();
     self.set_pur(f(tmp))
  }

  #[inline] pub fn pdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x514) as *const u32
  }
  #[inline] pub fn pdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x514) as *mut u32
  }
  #[inline] pub fn pdr(&self) -> Pdr { 
     unsafe {
        Pdr(::core::ptr::read_volatile(((self.0 as usize) + 0x514) as *const u32))
     }
  }
  #[inline] pub fn set_pdr(&self, value: Pdr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x514) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pdr<F: FnOnce(Pdr) -> Pdr>(&self, f: F) -> &Self {
     let tmp = self.pdr();
     self.set_pdr(f(tmp))
  }

  #[inline] pub fn slr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x518) as *const u32
  }
  #[inline] pub fn slr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x518) as *mut u32
  }
  #[inline] pub fn slr(&self) -> Slr { 
     unsafe {
        Slr(::core::ptr::read_volatile(((self.0 as usize) + 0x518) as *const u32))
     }
  }
  #[inline] pub fn set_slr(&self, value: Slr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x518) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_slr<F: FnOnce(Slr) -> Slr>(&self, f: F) -> &Self {
     let tmp = self.slr();
     self.set_slr(f(tmp))
  }

  #[inline] pub fn den_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x51c) as *const u32
  }
  #[inline] pub fn den_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x51c) as *mut u32
  }
  #[inline] pub fn den(&self) -> Den { 
     unsafe {
        Den(::core::ptr::read_volatile(((self.0 as usize) + 0x51c) as *const u32))
     }
  }
  #[inline] pub fn set_den(&self, value: Den) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x51c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_den<F: FnOnce(Den) -> Den>(&self, f: F) -> &Self {
     let tmp = self.den();
     self.set_den(f(tmp))
  }

  #[inline] pub fn lock_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x520) as *const u32
  }
  #[inline] pub fn lock_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x520) as *mut u32
  }
  #[inline] pub fn lock(&self) -> Lock { 
     unsafe {
        Lock(::core::ptr::read_volatile(((self.0 as usize) + 0x520) as *const u32))
     }
  }
  #[inline] pub fn set_lock(&self, value: Lock) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x520) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
     let tmp = self.lock();
     self.set_lock(f(tmp))
  }

  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x524) as *const u32
  }
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x524) as *mut u32
  }
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x524) as *const u32))
     }
  }

  #[inline] pub fn amsel_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x528) as *const u32
  }
  #[inline] pub fn amsel_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x528) as *mut u32
  }
  #[inline] pub fn amsel(&self) -> Amsel { 
     unsafe {
        Amsel(::core::ptr::read_volatile(((self.0 as usize) + 0x528) as *const u32))
     }
  }
  #[inline] pub fn set_amsel(&self, value: Amsel) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x528) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_amsel<F: FnOnce(Amsel) -> Amsel>(&self, f: F) -> &Self {
     let tmp = self.amsel();
     self.set_amsel(f(tmp))
  }

  #[inline] pub fn pctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x52c) as *const u32
  }
  #[inline] pub fn pctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x52c) as *mut u32
  }
  #[inline] pub fn pctl(&self) -> Pctl { 
     unsafe {
        Pctl(::core::ptr::read_volatile(((self.0 as usize) + 0x52c) as *const u32))
     }
  }
  #[inline] pub fn set_pctl(&self, value: Pctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x52c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_pctl<F: FnOnce(Pctl) -> Pctl>(&self, f: F) -> &Self {
     let tmp = self.pctl();
     self.set_pctl(f(tmp))
  }

  #[inline] pub fn adcctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x530) as *const u32
  }
  #[inline] pub fn adcctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x530) as *mut u32
  }
  #[inline] pub fn adcctl(&self) -> Adcctl { 
     unsafe {
        Adcctl(::core::ptr::read_volatile(((self.0 as usize) + 0x530) as *const u32))
     }
  }
  #[inline] pub fn set_adcctl(&self, value: Adcctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x530) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_adcctl<F: FnOnce(Adcctl) -> Adcctl>(&self, f: F) -> &Self {
     let tmp = self.adcctl();
     self.set_adcctl(f(tmp))
  }

  #[inline] pub fn dmactl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x534) as *const u32
  }
  #[inline] pub fn dmactl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x534) as *mut u32
  }
  #[inline] pub fn dmactl(&self) -> Dmactl { 
     unsafe {
        Dmactl(::core::ptr::read_volatile(((self.0 as usize) + 0x534) as *const u32))
     }
  }
  #[inline] pub fn set_dmactl(&self, value: Dmactl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x534) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
     let tmp = self.dmactl();
     self.set_dmactl(f(tmp))
  }

  #[inline] pub fn gpiosi_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x539) as *const u32
  }
  #[inline] pub fn gpiosi_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x539) as *mut u32
  }
  #[inline] pub fn gpiosi(&self) -> Gpiosi { 
     unsafe {
        Gpiosi(::core::ptr::read_volatile(((self.0 as usize) + 0x539) as *const u32))
     }
  }
  #[inline] pub fn set_gpiosi(&self, value: Gpiosi) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x539) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_gpiosi<F: FnOnce(Gpiosi) -> Gpiosi>(&self, f: F) -> &Self {
     let tmp = self.gpiosi();
     self.set_gpiosi(f(tmp))
  }

  #[inline] pub fn gpiodr12r_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x53c) as *const u32
  }
  #[inline] pub fn gpiodr12r_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x53c) as *mut u32
  }
  #[inline] pub fn gpiodr12r(&self) -> Gpiodr12r { 
     unsafe {
        Gpiodr12r(::core::ptr::read_volatile(((self.0 as usize) + 0x53c) as *const u32))
     }
  }
  #[inline] pub fn set_gpiodr12r(&self, value: Gpiodr12r) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x53c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_gpiodr12r<F: FnOnce(Gpiodr12r) -> Gpiodr12r>(&self, f: F) -> &Self {
     let tmp = self.gpiodr12r();
     self.set_gpiodr12r(f(tmp))
  }

  #[inline] pub fn gpiowakepen_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x540) as *const u32
  }
  #[inline] pub fn gpiowakepen_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x540) as *mut u32
  }
  #[inline] pub fn gpiowakepen(&self) -> Gpiowakepen { 
     unsafe {
        Gpiowakepen(::core::ptr::read_volatile(((self.0 as usize) + 0x540) as *const u32))
     }
  }
  #[inline] pub fn set_gpiowakepen(&self, value: Gpiowakepen) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x540) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_gpiowakepen<F: FnOnce(Gpiowakepen) -> Gpiowakepen>(&self, f: F) -> &Self {
     let tmp = self.gpiowakepen();
     self.set_gpiowakepen(f(tmp))
  }

  #[inline] pub fn gpiowakelvl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x544) as *const u32
  }
  #[inline] pub fn gpiowakelvl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x544) as *mut u32
  }
  #[inline] pub fn gpiowakelvl(&self) -> Gpiowakelvl { 
     unsafe {
        Gpiowakelvl(::core::ptr::read_volatile(((self.0 as usize) + 0x544) as *const u32))
     }
  }
  #[inline] pub fn set_gpiowakelvl(&self, value: Gpiowakelvl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x544) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_gpiowakelvl<F: FnOnce(Gpiowakelvl) -> Gpiowakelvl>(&self, f: F) -> &Self {
     let tmp = self.gpiowakelvl();
     self.set_gpiowakelvl(f(tmp))
  }

  #[inline] pub fn gpiowakestat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x548) as *const u32
  }
  #[inline] pub fn gpiowakestat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x548) as *mut u32
  }
  #[inline] pub fn gpiowakestat(&self) -> Gpiowakestat { 
     unsafe {
        Gpiowakestat(::core::ptr::read_volatile(((self.0 as usize) + 0x548) as *const u32))
     }
  }
  #[inline] pub fn set_gpiowakestat(&self, value: Gpiowakestat) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x548) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_gpiowakestat<F: FnOnce(Gpiowakestat) -> Gpiowakestat>(&self, f: F) -> &Self {
     let tmp = self.gpiowakestat();
     self.set_gpiowakestat(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
  #[inline] pub fn data(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_data(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn dir(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_dir(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn is(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_is(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn ibe(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ibe(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn iev(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_iev(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn im(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_im(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dmaime(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_dmaime(mut self, value: u32) -> Self {
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
  #[inline] pub fn ris(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ris(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dmaris(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_dmaris(mut self, value: u32) -> Self {
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
  #[inline] pub fn mis(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_mis(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dmamis(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_dmamis(mut self, value: u32) -> Self {
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
  #[inline] pub fn icr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_icr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  #[inline] pub fn dmamic(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_dmamic(mut self, value: u32) -> Self {
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
  #[inline] pub fn afsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_afsel(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn drv2(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_drv2(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn drv4(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_drv4(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn drv8(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_drv8(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn ode(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_ode(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn pue(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_pue(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn pde(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_pde(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn slr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_slr(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn den(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_den(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn gpio_lock(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_gpio_lock(mut self, value: u32) -> Self {
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
  #[inline] pub fn cr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_cr(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn gpioamsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_gpioamsel(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn pmc(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  #[inline] pub fn set_pmc(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn adcen(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_adcen(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn dmaen(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_dmaen(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn sum(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_sum(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn drv12(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_drv12(mut self, index: usize, value: u32) -> Self {
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
  #[inline] pub fn wakep7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_wakep7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn wakep6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_wakep6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn wakep5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_wakep5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn wakep4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_wakep4(mut self, value: u32) -> Self {
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
  #[inline] pub fn wakelvl7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_wakelvl7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn wakelvl6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_wakelvl6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn wakelvl5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_wakelvl5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn wakelvl4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_wakelvl4(mut self, value: u32) -> Self {
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
  #[inline] pub fn wakestat7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_wakestat7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn wakestat6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_wakestat6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn wakestat5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_wakestat5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn wakestat4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_wakestat4(mut self, value: u32) -> Self {
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
pub struct Pin<P, T> { pub port: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Pin<P,T> {
   #[inline] pub fn port(&self) -> &Periph<T> { &self.port }
   #[inline] pub fn index(&self) -> usize { self.index }
}
pub trait AltFn<T> {
   fn alt_fn(&self) -> usize;
}

pub const PA0: Pin<Pa0Id, GpioaId> = Pin { port: GPIOA, index: 0, id: Pa0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa0Id {}
pub type Pa0 = Pin<Pa0Id, GpioaId>;
impl AltFn<super::sig::U0rx> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c9scl> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T0ccp0> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can0rx> for Pa0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA1: Pin<Pa1Id, GpioaId> = Pin { port: GPIOA, index: 1, id: Pa1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa1Id {}
pub type Pa1 = Pin<Pa1Id, GpioaId>;
impl AltFn<super::sig::U0tx> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c9sda> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T0ccp1> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can0tx> for Pa1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PA2: Pin<Pa2Id, GpioaId> = Pin { port: GPIOA, index: 2, id: Pa2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa2Id {}
pub type Pa2 = Pin<Pa2Id, GpioaId>;
impl AltFn<super::sig::U4rx> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c8scl> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T1ccp0> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi0clk> for Pa2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA3: Pin<Pa3Id, GpioaId> = Pin { port: GPIOA, index: 3, id: Pa3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa3Id {}
pub type Pa3 = Pin<Pa3Id, GpioaId>;
impl AltFn<super::sig::U4tx> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c8sda> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T1ccp1> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi0fss> for Pa3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA4: Pin<Pa4Id, GpioaId> = Pin { port: GPIOA, index: 4, id: Pa4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa4Id {}
pub type Pa4 = Pin<Pa4Id, GpioaId>;
impl AltFn<super::sig::U3rx> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c7scl> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T2ccp0> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi0xdat0> for Pa4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA5: Pin<Pa5Id, GpioaId> = Pin { port: GPIOA, index: 5, id: Pa5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa5Id {}
pub type Pa5 = Pin<Pa5Id, GpioaId>;
impl AltFn<super::sig::U3tx> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c7sda> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T2ccp1> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi0xdat1> for Pa5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA6: Pin<Pa6Id, GpioaId> = Pin { port: GPIOA, index: 6, id: Pa6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa6Id {}
pub type Pa6 = Pin<Pa6Id, GpioaId>;
impl AltFn<super::sig::U2rx> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c6scl> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T3ccp0> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0epen> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi0xdat2> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Epi0s8> for Pa6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PA7: Pin<Pa7Id, GpioaId> = Pin { port: GPIOA, index: 7, id: Pa7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pa7Id {}
pub type Pa7 = Pin<Pa7Id, GpioaId>;
impl AltFn<super::sig::U2tx> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c6sda> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T3ccp1> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0pflt> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Usb0epen> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 11 }
}

impl AltFn<super::sig::Ssi0xdat3> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 13 }
}

impl AltFn<super::sig::Epi0s9> for Pa7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB0: Pin<Pb0Id, GpiobId> = Pin { port: GPIOB, index: 0, id: Pb0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb0Id {}
pub type Pb0 = Pin<Pb0Id, GpiobId>;
impl AltFn<super::sig::Usb0id> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1rx> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c5scl> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T4ccp0> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can1rx> for Pb0Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB1: Pin<Pb1Id, GpiobId> = Pin { port: GPIOB, index: 1, id: Pb1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb1Id {}
pub type Pb1 = Pin<Pb1Id, GpiobId>;
impl AltFn<super::sig::Usb0vbus> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1tx> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c5sda> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T4ccp1> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Can1tx> for Pb1Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

pub const PB2: Pin<Pb2Id, GpiobId> = Pin { port: GPIOB, index: 2, id: Pb2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb2Id {}
pub type Pb2 = Pin<Pb2Id, GpiobId>;
impl AltFn<super::sig::I2c0scl> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T5ccp0> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0stp> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s27> for Pb2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB3: Pin<Pb3Id, GpiobId> = Pin { port: GPIOB, index: 3, id: Pb3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb3Id {}
pub type Pb3 = Pin<Pb3Id, GpiobId>;
impl AltFn<super::sig::I2c0sda> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T5ccp1> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0clk> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s28> for Pb3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB4: Pin<Pb4Id, GpiobId> = Pin { port: GPIOB, index: 4, id: Pb4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb4Id {}
pub type Pb4 = Pin<Pb4Id, GpiobId>;
impl AltFn<super::sig::Ain10> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0cts> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c5scl> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ssi1fss> for Pb4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PB5: Pin<Pb5Id, GpiobId> = Pin { port: GPIOB, index: 5, id: Pb5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pb5Id {}
pub type Pb5 = Pin<Pb5Id, GpiobId>;
impl AltFn<super::sig::Ain11> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0rts> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c5sda> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Ssi1clk> for Pb5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC0: Pin<Pc0Id, GpiocId> = Pin { port: GPIOC, index: 0, id: Pc0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc0Id {}
pub type Pc0 = Pin<Pc0Id, GpiocId>;
impl AltFn<super::sig::Tck> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Swclk> for Pc0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC1: Pin<Pc1Id, GpiocId> = Pin { port: GPIOC, index: 1, id: Pc1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc1Id {}
pub type Pc1 = Pin<Pc1Id, GpiocId>;
impl AltFn<super::sig::Tms> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Swdio> for Pc1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC2: Pin<Pc2Id, GpiocId> = Pin { port: GPIOC, index: 2, id: Pc2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc2Id {}
pub type Pc2 = Pin<Pc2Id, GpiocId>;
impl AltFn<super::sig::Tdi> for Pc2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC3: Pin<Pc3Id, GpiocId> = Pin { port: GPIOC, index: 3, id: Pc3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc3Id {}
pub type Pc3 = Pin<Pc3Id, GpiocId>;
impl AltFn<super::sig::Tdo> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Swo> for Pc3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PC4: Pin<Pc4Id, GpiocId> = Pin { port: GPIOC, index: 4, id: Pc4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc4Id {}
pub type Pc4 = Pin<Pc4Id, GpiocId>;
impl AltFn<super::sig::C1Neg> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U7rx> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s7> for Pc4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC5: Pin<Pc5Id, GpiocId> = Pin { port: GPIOC, index: 5, id: Pc5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc5Id {}
pub type Pc5 = Pin<Pc5Id, GpiocId>;
impl AltFn<super::sig::C1Pos> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U7tx> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Rtcclk> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Epi0s6> for Pc5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC6: Pin<Pc6Id, GpiocId> = Pin { port: GPIOC, index: 6, id: Pc6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc6Id {}
pub type Pc6 = Pin<Pc6Id, GpiocId>;
impl AltFn<super::sig::C0Neg> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U5rx> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s5> for Pc6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PC7: Pin<Pc7Id, GpiocId> = Pin { port: GPIOC, index: 7, id: Pc7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pc7Id {}
pub type Pc7 = Pin<Pc7Id, GpiocId>;
impl AltFn<super::sig::C0Pos> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U5tx> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s4> for Pc7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD0: Pin<Pd0Id, GpiodId> = Pin { port: GPIOD, index: 0, id: Pd0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd0Id {}
pub type Pd0 = Pin<Pd0Id, GpiodId>;
impl AltFn<super::sig::Ain15> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c7scl> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T0ccp0> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::C0o> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi2xdat1> for Pd0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD1: Pin<Pd1Id, GpiodId> = Pin { port: GPIOD, index: 1, id: Pd1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd1Id {}
pub type Pd1 = Pin<Pd1Id, GpiodId>;
impl AltFn<super::sig::Ain14> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c7sda> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T0ccp1> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::C1o> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi2xdat0> for Pd1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD2: Pin<Pd2Id, GpiodId> = Pin { port: GPIOD, index: 2, id: Pd2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd2Id {}
pub type Pd2 = Pin<Pd2Id, GpiodId>;
impl AltFn<super::sig::Ain13> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c8scl> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T1ccp0> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::C2o> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi2fss> for Pd2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD3: Pin<Pd3Id, GpiodId> = Pin { port: GPIOD, index: 3, id: Pd3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd3Id {}
pub type Pd3 = Pin<Pd3Id, GpiodId>;
impl AltFn<super::sig::Ain12> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::I2c8sda> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::T1ccp1> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi2clk> for Pd3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD4: Pin<Pd4Id, GpiodId> = Pin { port: GPIOD, index: 4, id: Pd4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd4Id {}
pub type Pd4 = Pin<Pd4Id, GpiodId>;
impl AltFn<super::sig::Ain7> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U2rx> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T3ccp0> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi1xdat2> for Pd4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD5: Pin<Pd5Id, GpiodId> = Pin { port: GPIOD, index: 5, id: Pd5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd5Id {}
pub type Pd5 = Pin<Pd5Id, GpiodId>;
impl AltFn<super::sig::Ain6> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U2tx> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T3ccp1> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Ssi1xdat3> for Pd5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD6: Pin<Pd6Id, GpiodId> = Pin { port: GPIOD, index: 6, id: Pd6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd6Id {}
pub type Pd6 = Pin<Pd6Id, GpiodId>;
impl AltFn<super::sig::Ain5> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U2rts> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T4ccp0> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0epen> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Ssi2xdat3> for Pd6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PD7: Pin<Pd7Id, GpiodId> = Pin { port: GPIOD, index: 7, id: Pd7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pd7Id {}
pub type Pd7 = Pin<Pd7Id, GpiodId>;
impl AltFn<super::sig::Ain4> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U2cts> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T4ccp1> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0pflt> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Nmi> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 8 }
}

impl AltFn<super::sig::Ssi2xdat2> for Pd7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE0: Pin<Pe0Id, GpioeId> = Pin { port: GPIOE, index: 0, id: Pe0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe0Id {}
pub type Pe0 = Pin<Pe0Id, GpioeId>;
impl AltFn<super::sig::Ain3> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1rts> for Pe0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PE1: Pin<Pe1Id, GpioeId> = Pin { port: GPIOE, index: 1, id: Pe1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe1Id {}
pub type Pe1 = Pin<Pe1Id, GpioeId>;
impl AltFn<super::sig::Ain2> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1dsr> for Pe1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PE2: Pin<Pe2Id, GpioeId> = Pin { port: GPIOE, index: 2, id: Pe2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe2Id {}
pub type Pe2 = Pin<Pe2Id, GpioeId>;
impl AltFn<super::sig::Ain1> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1dcd> for Pe2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PE3: Pin<Pe3Id, GpioeId> = Pin { port: GPIOE, index: 3, id: Pe3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe3Id {}
pub type Pe3 = Pin<Pe3Id, GpioeId>;
impl AltFn<super::sig::Ain0> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1dtr> for Pe3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PE4: Pin<Pe4Id, GpioeId> = Pin { port: GPIOE, index: 4, id: Pe4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe4Id {}
pub type Pe4 = Pin<Pe4Id, GpioeId>;
impl AltFn<super::sig::Ain9> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U1ri> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ssi1xdat0> for Pe4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PE5: Pin<Pe5Id, GpioeId> = Pin { port: GPIOE, index: 5, id: Pe5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pe5Id {}
pub type Pe5 = Pin<Pe5Id, GpioeId>;
impl AltFn<super::sig::Ain8> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::Ssi1xdat1> for Pe5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF0: Pin<Pf0Id, GpiofId> = Pin { port: GPIOF, index: 0, id: Pf0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf0Id {}
pub type Pf0 = Pin<Pf0Id, GpiofId>;
impl AltFn<super::sig::En0led0> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm0> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3xdat1> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trd2> for Pf0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF1: Pin<Pf1Id, GpiofId> = Pin { port: GPIOF, index: 1, id: Pf1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf1Id {}
pub type Pf1 = Pin<Pf1Id, GpiofId>;
impl AltFn<super::sig::En0led2> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm1> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3xdat0> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trd1> for Pf1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF2: Pin<Pf2Id, GpiofId> = Pin { port: GPIOF, index: 2, id: Pf2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf2Id {}
pub type Pf2 = Pin<Pf2Id, GpiofId>;
impl AltFn<super::sig::M0pwm2> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3fss> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trd0> for Pf2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF3: Pin<Pf3Id, GpiofId> = Pin { port: GPIOF, index: 3, id: Pf3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf3Id {}
pub type Pf3 = Pin<Pf3Id, GpiofId>;
impl AltFn<super::sig::M0pwm3> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3clk> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trclk> for Pf3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PF4: Pin<Pf4Id, GpiofId> = Pin { port: GPIOF, index: 4, id: Pf4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pf4Id {}
pub type Pf4 = Pin<Pf4Id, GpiofId>;
impl AltFn<super::sig::En0led1> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0fault0> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Ssi3xdat2> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Trd3> for Pf4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG0: Pin<Pg0Id, GpiogId> = Pin { port: GPIOG, index: 0, id: Pg0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg0Id {}
pub type Pg0 = Pin<Pg0Id, GpiogId>;
impl AltFn<super::sig::I2c1scl> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::En0pps> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm4> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s11> for Pg0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PG1: Pin<Pg1Id, GpiogId> = Pin { port: GPIOG, index: 1, id: Pg1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pg1Id {}
pub type Pg1 = Pin<Pg1Id, GpiogId>;
impl AltFn<super::sig::I2c1sda> for Pg1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::M0pwm5> for Pg1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s10> for Pg1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH0: Pin<Ph0Id, GpiohId> = Pin { port: GPIOH, index: 0, id: Ph0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ph0Id {}
pub type Ph0 = Pin<Ph0Id, GpiohId>;
impl AltFn<super::sig::U0rts> for Ph0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s0> for Ph0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH1: Pin<Ph1Id, GpiohId> = Pin { port: GPIOH, index: 1, id: Ph1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ph1Id {}
pub type Ph1 = Pin<Ph1Id, GpiohId>;
impl AltFn<super::sig::U0cts> for Ph1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s1> for Ph1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH2: Pin<Ph2Id, GpiohId> = Pin { port: GPIOH, index: 2, id: Ph2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ph2Id {}
pub type Ph2 = Pin<Ph2Id, GpiohId>;
impl AltFn<super::sig::U0dcd> for Ph2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s2> for Ph2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PH3: Pin<Ph3Id, GpiohId> = Pin { port: GPIOH, index: 3, id: Ph3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Ph3Id {}
pub type Ph3 = Pin<Ph3Id, GpiohId>;
impl AltFn<super::sig::U0dsr> for Ph3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s3> for Ph3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PJ0: Pin<Pj0Id, GpiojId> = Pin { port: GPIOJ, index: 0, id: Pj0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pj0Id {}
pub type Pj0 = Pin<Pj0Id, GpiojId>;
impl AltFn<super::sig::U3rx> for Pj0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::En0pps> for Pj0Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

pub const PJ1: Pin<Pj1Id, GpiojId> = Pin { port: GPIOJ, index: 1, id: Pj1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pj1Id {}
pub type Pj1 = Pin<Pj1Id, GpiojId>;
impl AltFn<super::sig::U3tx> for Pj1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PK0: Pin<Pk0Id, GpiokId> = Pin { port: GPIOK, index: 0, id: Pk0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pk0Id {}
pub type Pk0 = Pin<Pk0Id, GpiokId>;
impl AltFn<super::sig::Ain16> for Pk0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U4rx> for Pk0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s0> for Pk0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK1: Pin<Pk1Id, GpiokId> = Pin { port: GPIOK, index: 1, id: Pk1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pk1Id {}
pub type Pk1 = Pin<Pk1Id, GpiokId>;
impl AltFn<super::sig::Ain17> for Pk1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U4tx> for Pk1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s1> for Pk1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK2: Pin<Pk2Id, GpiokId> = Pin { port: GPIOK, index: 2, id: Pk2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pk2Id {}
pub type Pk2 = Pin<Pk2Id, GpiokId>;
impl AltFn<super::sig::Ain18> for Pk2Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U4rts> for Pk2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s2> for Pk2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK3: Pin<Pk3Id, GpiokId> = Pin { port: GPIOK, index: 3, id: Pk3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pk3Id {}
pub type Pk3 = Pin<Pk3Id, GpiokId>;
impl AltFn<super::sig::Ain19> for Pk3Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U4cts> for Pk3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Epi0s3> for Pk3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK4: Pin<Pk4Id, GpiokId> = Pin { port: GPIOK, index: 4, id: Pk4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pk4Id {}
pub type Pk4 = Pin<Pk4Id, GpiokId>;
impl AltFn<super::sig::I2c3scl> for Pk4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::En0led0> for Pk4Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm6> for Pk4Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s32> for Pk4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK5: Pin<Pk5Id, GpiokId> = Pin { port: GPIOK, index: 5, id: Pk5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pk5Id {}
pub type Pk5 = Pin<Pk5Id, GpiokId>;
impl AltFn<super::sig::I2c3sda> for Pk5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::En0led2> for Pk5Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0pwm7> for Pk5Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s31> for Pk5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK6: Pin<Pk6Id, GpiokId> = Pin { port: GPIOK, index: 6, id: Pk6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pk6Id {}
pub type Pk6 = Pin<Pk6Id, GpiokId>;
impl AltFn<super::sig::I2c4scl> for Pk6Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::En0led1> for Pk6Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0fault1> for Pk6Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s25> for Pk6Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PK7: Pin<Pk7Id, GpiokId> = Pin { port: GPIOK, index: 7, id: Pk7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pk7Id {}
pub type Pk7 = Pin<Pk7Id, GpiokId>;
impl AltFn<super::sig::U0ri> for Pk7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c4sda> for Pk7Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Rtcclk> for Pk7Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::M0fault2> for Pk7Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Epi0s24> for Pk7Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL0: Pin<Pl0Id, GpiolId> = Pin { port: GPIOL, index: 0, id: Pl0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pl0Id {}
pub type Pl0 = Pin<Pl0Id, GpiolId>;
impl AltFn<super::sig::I2c2sda> for Pl0Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::M0fault3> for Pl0Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0d0> for Pl0Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s16> for Pl0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL1: Pin<Pl1Id, GpiolId> = Pin { port: GPIOL, index: 1, id: Pl1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pl1Id {}
pub type Pl1 = Pin<Pl1Id, GpiolId>;
impl AltFn<super::sig::I2c2scl> for Pl1Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Pha0> for Pl1Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0d1> for Pl1Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s17> for Pl1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL2: Pin<Pl2Id, GpiolId> = Pin { port: GPIOL, index: 2, id: Pl2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pl2Id {}
pub type Pl2 = Pin<Pl2Id, GpiolId>;
impl AltFn<super::sig::C0o> for Pl2Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Phb0> for Pl2Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0d2> for Pl2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s18> for Pl2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL3: Pin<Pl3Id, GpiolId> = Pin { port: GPIOL, index: 3, id: Pl3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pl3Id {}
pub type Pl3 = Pin<Pl3Id, GpiolId>;
impl AltFn<super::sig::C1o> for Pl3Id {
   #[inline] fn alt_fn(&self) -> usize { 5 }
}

impl AltFn<super::sig::Idx0> for Pl3Id {
   #[inline] fn alt_fn(&self) -> usize { 6 }
}

impl AltFn<super::sig::Usb0d3> for Pl3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s19> for Pl3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL4: Pin<Pl4Id, GpiolId> = Pin { port: GPIOL, index: 4, id: Pl4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pl4Id {}
pub type Pl4 = Pin<Pl4Id, GpiolId>;
impl AltFn<super::sig::T0ccp0> for Pl4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0d4> for Pl4Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s26> for Pl4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL5: Pin<Pl5Id, GpiolId> = Pin { port: GPIOL, index: 5, id: Pl5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pl5Id {}
pub type Pl5 = Pin<Pl5Id, GpiolId>;
impl AltFn<super::sig::T0ccp1> for Pl5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Usb0d5> for Pl5Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s33> for Pl5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PL6: Pin<Pl6Id, GpiolId> = Pin { port: GPIOL, index: 6, id: Pl6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pl6Id {}
pub type Pl6 = Pin<Pl6Id, GpiolId>;
impl AltFn<super::sig::Usb0dp> for Pl6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::T1ccp0> for Pl6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PL7: Pin<Pl7Id, GpiolId> = Pin { port: GPIOL, index: 7, id: Pl7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pl7Id {}
pub type Pl7 = Pin<Pl7Id, GpiolId>;
impl AltFn<super::sig::Usb0dm> for Pl7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::T1ccp1> for Pl7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PM0: Pin<Pm0Id, GpiomId> = Pin { port: GPIOM, index: 0, id: Pm0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pm0Id {}
pub type Pm0 = Pin<Pm0Id, GpiomId>;
impl AltFn<super::sig::T2ccp0> for Pm0Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s15> for Pm0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PM1: Pin<Pm1Id, GpiomId> = Pin { port: GPIOM, index: 1, id: Pm1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pm1Id {}
pub type Pm1 = Pin<Pm1Id, GpiomId>;
impl AltFn<super::sig::T2ccp1> for Pm1Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s14> for Pm1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PM2: Pin<Pm2Id, GpiomId> = Pin { port: GPIOM, index: 2, id: Pm2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pm2Id {}
pub type Pm2 = Pin<Pm2Id, GpiomId>;
impl AltFn<super::sig::T3ccp0> for Pm2Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s13> for Pm2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PM3: Pin<Pm3Id, GpiomId> = Pin { port: GPIOM, index: 3, id: Pm3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pm3Id {}
pub type Pm3 = Pin<Pm3Id, GpiomId>;
impl AltFn<super::sig::T3ccp1> for Pm3Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s12> for Pm3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PM4: Pin<Pm4Id, GpiomId> = Pin { port: GPIOM, index: 4, id: Pm4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pm4Id {}
pub type Pm4 = Pin<Pm4Id, GpiomId>;
impl AltFn<super::sig::Tmpr3> for Pm4Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0cts> for Pm4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T4ccp0> for Pm4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PM5: Pin<Pm5Id, GpiomId> = Pin { port: GPIOM, index: 5, id: Pm5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pm5Id {}
pub type Pm5 = Pin<Pm5Id, GpiomId>;
impl AltFn<super::sig::Tmpr2> for Pm5Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0dcd> for Pm5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T4ccp1> for Pm5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PM6: Pin<Pm6Id, GpiomId> = Pin { port: GPIOM, index: 6, id: Pm6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pm6Id {}
pub type Pm6 = Pin<Pm6Id, GpiomId>;
impl AltFn<super::sig::Tmpr1> for Pm6Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0dsr> for Pm6Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T5ccp0> for Pm6Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PM7: Pin<Pm7Id, GpiomId> = Pin { port: GPIOM, index: 7, id: Pm7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pm7Id {}
pub type Pm7 = Pin<Pm7Id, GpiomId>;
impl AltFn<super::sig::Tmpr0> for Pm7Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U0ri> for Pm7Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::T5ccp1> for Pm7Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

pub const PN0: Pin<Pn0Id, GpionId> = Pin { port: GPION, index: 0, id: Pn0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pn0Id {}
pub type Pn0 = Pin<Pn0Id, GpionId>;
impl AltFn<super::sig::U1rts> for Pn0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PN1: Pin<Pn1Id, GpionId> = Pin { port: GPION, index: 1, id: Pn1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pn1Id {}
pub type Pn1 = Pin<Pn1Id, GpionId>;
impl AltFn<super::sig::U1cts> for Pn1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

pub const PN2: Pin<Pn2Id, GpionId> = Pin { port: GPION, index: 2, id: Pn2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pn2Id {}
pub type Pn2 = Pin<Pn2Id, GpionId>;
impl AltFn<super::sig::U1dcd> for Pn2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U2rts> for Pn2Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Epi0s29> for Pn2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PN3: Pin<Pn3Id, GpionId> = Pin { port: GPION, index: 3, id: Pn3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pn3Id {}
pub type Pn3 = Pin<Pn3Id, GpionId>;
impl AltFn<super::sig::U1dsr> for Pn3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U2cts> for Pn3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Epi0s30> for Pn3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PN4: Pin<Pn4Id, GpionId> = Pin { port: GPION, index: 4, id: Pn4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pn4Id {}
pub type Pn4 = Pin<Pn4Id, GpionId>;
impl AltFn<super::sig::U1dtr> for Pn4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U3rts> for Pn4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c2sda> for Pn4Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s34> for Pn4Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PN5: Pin<Pn5Id, GpionId> = Pin { port: GPION, index: 5, id: Pn5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pn5Id {}
pub type Pn5 = Pin<Pn5Id, GpionId>;
impl AltFn<super::sig::U1ri> for Pn5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U3cts> for Pn5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::I2c2scl> for Pn5Id {
   #[inline] fn alt_fn(&self) -> usize { 3 }
}

impl AltFn<super::sig::Epi0s35> for Pn5Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP0: Pin<Pp0Id, GpiopId> = Pin { port: GPIOP, index: 0, id: Pp0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pp0Id {}
pub type Pp0 = Pin<Pp0Id, GpiopId>;
impl AltFn<super::sig::C2Pos> for Pp0Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U6rx> for Pp0Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ssi3xdat> for Pp0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP1: Pin<Pp1Id, GpiopId> = Pin { port: GPIOP, index: 1, id: Pp1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pp1Id {}
pub type Pp1 = Pin<Pp1Id, GpiopId>;
impl AltFn<super::sig::C2Neg> for Pp1Id {
   #[inline] fn alt_fn(&self) -> usize { 0 }
}

impl AltFn<super::sig::U6tx> for Pp1Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Ssi3xdat> for Pp1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP2: Pin<Pp2Id, GpiopId> = Pin { port: GPIOP, index: 2, id: Pp2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pp2Id {}
pub type Pp2 = Pin<Pp2Id, GpiopId>;
impl AltFn<super::sig::U0dtr> for Pp2Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Usb0nxt> for Pp2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s29> for Pp2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP3: Pin<Pp3Id, GpiopId> = Pin { port: GPIOP, index: 3, id: Pp3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pp3Id {}
pub type Pp3 = Pin<Pp3Id, GpiopId>;
impl AltFn<super::sig::U1cts> for Pp3Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U0dcd> for Pp3Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Rtcclk> for Pp3Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

impl AltFn<super::sig::Usb0dir> for Pp3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s30> for Pp3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PP4: Pin<Pp4Id, GpiopId> = Pin { port: GPIOP, index: 4, id: Pp4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pp4Id {}
pub type Pp4 = Pin<Pp4Id, GpiopId>;
impl AltFn<super::sig::U3rts> for Pp4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::U0dsr> for Pp4Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usb0d7> for Pp4Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

pub const PP5: Pin<Pp5Id, GpiopId> = Pin { port: GPIOP, index: 5, id: Pp5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pp5Id {}
pub type Pp5 = Pin<Pp5Id, GpiopId>;
impl AltFn<super::sig::U3cts> for Pp5Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::I2c2scl> for Pp5Id {
   #[inline] fn alt_fn(&self) -> usize { 2 }
}

impl AltFn<super::sig::Usb0d6> for Pp5Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

pub const PQ0: Pin<Pq0Id, GpioqId> = Pin { port: GPIOQ, index: 0, id: Pq0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pq0Id {}
pub type Pq0 = Pin<Pq0Id, GpioqId>;
impl AltFn<super::sig::Ssi3clk> for Pq0Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s20> for Pq0Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PQ1: Pin<Pq1Id, GpioqId> = Pin { port: GPIOQ, index: 1, id: Pq1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pq1Id {}
pub type Pq1 = Pin<Pq1Id, GpioqId>;
impl AltFn<super::sig::Ssi3fss> for Pq1Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s21> for Pq1Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PQ2: Pin<Pq2Id, GpioqId> = Pin { port: GPIOQ, index: 2, id: Pq2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pq2Id {}
pub type Pq2 = Pin<Pq2Id, GpioqId>;
impl AltFn<super::sig::Ssi3xdat0> for Pq2Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s22> for Pq2Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PQ3: Pin<Pq3Id, GpioqId> = Pin { port: GPIOQ, index: 3, id: Pq3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pq3Id {}
pub type Pq3 = Pin<Pq3Id, GpioqId>;
impl AltFn<super::sig::Ssi3xdat1> for Pq3Id {
   #[inline] fn alt_fn(&self) -> usize { 14 }
}

impl AltFn<super::sig::Epi0s23> for Pq3Id {
   #[inline] fn alt_fn(&self) -> usize { 15 }
}

pub const PQ4: Pin<Pq4Id, GpioqId> = Pin { port: GPIOQ, index: 4, id: Pq4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Pq4Id {}
pub type Pq4 = Pin<Pq4Id, GpioqId>;
impl AltFn<super::sig::U1rx> for Pq4Id {
   #[inline] fn alt_fn(&self) -> usize { 1 }
}

impl AltFn<super::sig::Divsclk> for Pq4Id {
   #[inline] fn alt_fn(&self) -> usize { 7 }
}

