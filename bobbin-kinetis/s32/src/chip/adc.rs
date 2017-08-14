#[allow(unused_imports)] use bobbin_common::bits;
pub const ADC0: Adc0 = Periph(0x4003b000, Adc0Id {});
pub const ADC1: Adc1 = Periph(0x40027000, Adc1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc0Id {}
pub type Adc0 = Periph<Adc0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc1Id {}
pub type Adc1 = Periph<Adc1Id>;

impl super::sig::Signal<super::sig::Adc0Se0> for Adc0Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc0Se0> for Adc0Ch0 {}
impl super::sig::Signal<super::sig::Adc0Se1> for Adc0Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc0Se1> for Adc0Ch1 {}
impl super::sig::Signal<super::sig::Adc0Se2> for Adc0Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc0Se2> for Adc0Ch2 {}
impl super::sig::Signal<super::sig::Adc0Se3> for Adc0Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc0Se3> for Adc0Ch3 {}
impl super::sig::Signal<super::sig::Adc0Se4> for Adc0Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc0Se4> for Adc0Ch4 {}
impl super::sig::Signal<super::sig::Adc0Se5> for Adc0Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc0Se5> for Adc0Ch5 {}
impl super::sig::Signal<super::sig::Adc0Se6> for Adc0Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc0Se6> for Adc0Ch6 {}
impl super::sig::Signal<super::sig::Adc0Se7> for Adc0Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc0Se7> for Adc0Ch7 {}
impl super::sig::Signal<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc0Se8> for Adc0Ch8 {}
impl super::sig::Signal<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc0Se9> for Adc0Ch9 {}
impl super::sig::Signal<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc0Se10> for Adc0Ch10 {}
impl super::sig::Signal<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc0Se11> for Adc0Ch11 {}
impl super::sig::Signal<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc0Se12> for Adc0Ch12 {}
impl super::sig::Signal<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc0Se13> for Adc0Ch13 {}
impl super::sig::Signal<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc0Se14> for Adc0Ch14 {}
impl super::sig::Signal<super::sig::Adc0Se15> for Adc0Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc0Se15> for Adc0Ch15 {}

impl super::sig::Signal<super::sig::Adc1Se0> for Adc1Ch0 {}
impl super::sig::SignalAdc<super::sig::Adc1Se0> for Adc1Ch0 {}
impl super::sig::Signal<super::sig::Adc1Se1> for Adc1Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc1Se1> for Adc1Ch1 {}
impl super::sig::Signal<super::sig::Adc1Se2> for Adc1Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc1Se2> for Adc1Ch2 {}
impl super::sig::Signal<super::sig::Adc1Se3> for Adc1Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc1Se3> for Adc1Ch3 {}
impl super::sig::Signal<super::sig::Adc1Se4> for Adc1Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc1Se4> for Adc1Ch4 {}
impl super::sig::Signal<super::sig::Adc1Se5> for Adc1Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc1Se5> for Adc1Ch5 {}
impl super::sig::Signal<super::sig::Adc1Se6> for Adc1Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc1Se6> for Adc1Ch6 {}
impl super::sig::Signal<super::sig::Adc1Se7> for Adc1Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc1Se7> for Adc1Ch7 {}
impl super::sig::Signal<super::sig::Adc1Se8> for Adc1Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc1Se8> for Adc1Ch8 {}
impl super::sig::Signal<super::sig::Adc1Se9> for Adc1Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc1Se9> for Adc1Ch9 {}
impl super::sig::Signal<super::sig::Adc1Se10> for Adc1Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc1Se10> for Adc1Ch10 {}
impl super::sig::Signal<super::sig::Adc1Se11> for Adc1Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc1Se11> for Adc1Ch11 {}
impl super::sig::Signal<super::sig::Adc1Se12> for Adc1Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc1Se12> for Adc1Ch12 {}
impl super::sig::Signal<super::sig::Adc1Se13> for Adc1Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc1Se13> for Adc1Ch13 {}
impl super::sig::Signal<super::sig::Adc1Se14> for Adc1Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc1Se14> for Adc1Ch14 {}
impl super::sig::Signal<super::sig::Adc1Se15> for Adc1Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc1Se15> for Adc1Ch15 {}


impl<T> Periph<T> {
#[doc="Get the *const pointer for the SC1 register."]
  #[inline] pub fn sc1_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x0 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the SC1 register."]
  #[inline] pub fn sc1_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x0 + (index << 2)) as *mut u32
  }
#[doc="Read the SC1 register."]
  #[inline] pub fn sc1<I: Into<bits::R16>>(&self, index: I) -> Sc1 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Sc1(::core::ptr::read_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the SC1 register."]
  #[inline] pub fn set_sc1<I: Into<bits::R16>, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
     let index: bits::R16 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Sc1(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SC1 register."]
  #[inline] pub fn with_sc1<I: Into<bits::R16> + Copy, F: FnOnce(Sc1) -> Sc1>(&self, index: I, f: F) -> &Self {
     let index: bits::R16 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.sc1(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0 + (index << 2)) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CFG1 register."]
  #[inline] pub fn cfg1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the CFG1 register."]
  #[inline] pub fn cfg1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the CFG1 register."]
  #[inline] pub fn cfg1(&self) -> Cfg1 { 
     unsafe {
        Cfg1(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the CFG1 register."]
  #[inline] pub fn set_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
     let value = f(Cfg1(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFG1 register."]
  #[inline] pub fn with_cfg1<F: FnOnce(Cfg1) -> Cfg1>(&self, f: F) -> &Self {
     let tmp = self.cfg1();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CFG2 register."]
  #[inline] pub fn cfg2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the CFG2 register."]
  #[inline] pub fn cfg2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the CFG2 register."]
  #[inline] pub fn cfg2(&self) -> Cfg2 { 
     unsafe {
        Cfg2(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the CFG2 register."]
  #[inline] pub fn set_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
     let value = f(Cfg2(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFG2 register."]
  #[inline] pub fn with_cfg2<F: FnOnce(Cfg2) -> Cfg2>(&self, f: F) -> &Self {
     let tmp = self.cfg2();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the R register."]
  #[inline] pub fn r_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x48 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the R register."]
  #[inline] pub fn r_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
     let index: bits::R16 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x48 + (index << 2)) as *mut u32
  }
#[doc="Read the R register."]
  #[inline] pub fn r<I: Into<bits::R16>>(&self, index: I) -> R { 
     let index: bits::R16 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        R(::core::ptr::read_volatile(((self.0 as usize) + 0x48 + (index << 2)) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CV register."]
  #[inline] pub fn cv_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
     let index: bits::R2 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x88 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the CV register."]
  #[inline] pub fn cv_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
     let index: bits::R2 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x88 + (index << 2)) as *mut u32
  }
#[doc="Read the CV register."]
  #[inline] pub fn cv<I: Into<bits::R2>>(&self, index: I) -> Cv { 
     let index: bits::R2 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Cv(::core::ptr::read_volatile(((self.0 as usize) + 0x88 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the CV register."]
  #[inline] pub fn set_cv<I: Into<bits::R2>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Cv(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x88 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CV register."]
  #[inline] pub fn with_cv<I: Into<bits::R2> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
     let index: bits::R2 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.cv(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x88 + (index << 2)) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SC2 register."]
  #[inline] pub fn sc2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x90) as *const u32
  }
#[doc="Get the *mut pointer for the SC2 register."]
  #[inline] pub fn sc2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x90) as *mut u32
  }
#[doc="Read the SC2 register."]
  #[inline] pub fn sc2(&self) -> Sc2 { 
     unsafe {
        Sc2(::core::ptr::read_volatile(((self.0 as usize) + 0x90) as *const u32))
     }
  }
#[doc="Write the SC2 register."]
  #[inline] pub fn set_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
     let value = f(Sc2(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x90) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SC2 register."]
  #[inline] pub fn with_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
     let tmp = self.sc2();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x90) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SC3 register."]
  #[inline] pub fn sc3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x94) as *const u32
  }
#[doc="Get the *mut pointer for the SC3 register."]
  #[inline] pub fn sc3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x94) as *mut u32
  }
#[doc="Read the SC3 register."]
  #[inline] pub fn sc3(&self) -> Sc3 { 
     unsafe {
        Sc3(::core::ptr::read_volatile(((self.0 as usize) + 0x94) as *const u32))
     }
  }
#[doc="Write the SC3 register."]
  #[inline] pub fn set_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
     let value = f(Sc3(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x94) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SC3 register."]
  #[inline] pub fn with_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
     let tmp = self.sc3();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x94) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the BASE_OFS register."]
  #[inline] pub fn base_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x98) as *const u32
  }
#[doc="Get the *mut pointer for the BASE_OFS register."]
  #[inline] pub fn base_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x98) as *mut u32
  }
#[doc="Read the BASE_OFS register."]
  #[inline] pub fn base_ofs(&self) -> BaseOfs { 
     unsafe {
        BaseOfs(::core::ptr::read_volatile(((self.0 as usize) + 0x98) as *const u32))
     }
  }
#[doc="Write the BASE_OFS register."]
  #[inline] pub fn set_base_ofs<F: FnOnce(BaseOfs) -> BaseOfs>(&self, f: F) -> &Self {
     let value = f(BaseOfs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x98) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BASE_OFS register."]
  #[inline] pub fn with_base_ofs<F: FnOnce(BaseOfs) -> BaseOfs>(&self, f: F) -> &Self {
     let tmp = self.base_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x98) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the OFS register."]
  #[inline] pub fn ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x9c) as *const u32
  }
#[doc="Get the *mut pointer for the OFS register."]
  #[inline] pub fn ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x9c) as *mut u32
  }
#[doc="Read the OFS register."]
  #[inline] pub fn ofs(&self) -> Ofs { 
     unsafe {
        Ofs(::core::ptr::read_volatile(((self.0 as usize) + 0x9c) as *const u32))
     }
  }
#[doc="Write the OFS register."]
  #[inline] pub fn set_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
     let value = f(Ofs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x9c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the OFS register."]
  #[inline] pub fn with_ofs<F: FnOnce(Ofs) -> Ofs>(&self, f: F) -> &Self {
     let tmp = self.ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x9c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the USR_OFS register."]
  #[inline] pub fn usr_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa0) as *const u32
  }
#[doc="Get the *mut pointer for the USR_OFS register."]
  #[inline] pub fn usr_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa0) as *mut u32
  }
#[doc="Read the USR_OFS register."]
  #[inline] pub fn usr_ofs(&self) -> UsrOfs { 
     unsafe {
        UsrOfs(::core::ptr::read_volatile(((self.0 as usize) + 0xa0) as *const u32))
     }
  }
#[doc="Write the USR_OFS register."]
  #[inline] pub fn set_usr_ofs<F: FnOnce(UsrOfs) -> UsrOfs>(&self, f: F) -> &Self {
     let value = f(UsrOfs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the USR_OFS register."]
  #[inline] pub fn with_usr_ofs<F: FnOnce(UsrOfs) -> UsrOfs>(&self, f: F) -> &Self {
     let tmp = self.usr_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the XOFS register."]
  #[inline] pub fn xofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa4) as *const u32
  }
#[doc="Get the *mut pointer for the XOFS register."]
  #[inline] pub fn xofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa4) as *mut u32
  }
#[doc="Read the XOFS register."]
  #[inline] pub fn xofs(&self) -> Xofs { 
     unsafe {
        Xofs(::core::ptr::read_volatile(((self.0 as usize) + 0xa4) as *const u32))
     }
  }
#[doc="Write the XOFS register."]
  #[inline] pub fn set_xofs<F: FnOnce(Xofs) -> Xofs>(&self, f: F) -> &Self {
     let value = f(Xofs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the XOFS register."]
  #[inline] pub fn with_xofs<F: FnOnce(Xofs) -> Xofs>(&self, f: F) -> &Self {
     let tmp = self.xofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the YOFS register."]
  #[inline] pub fn yofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa8) as *const u32
  }
#[doc="Get the *mut pointer for the YOFS register."]
  #[inline] pub fn yofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa8) as *mut u32
  }
#[doc="Read the YOFS register."]
  #[inline] pub fn yofs(&self) -> Yofs { 
     unsafe {
        Yofs(::core::ptr::read_volatile(((self.0 as usize) + 0xa8) as *const u32))
     }
  }
#[doc="Write the YOFS register."]
  #[inline] pub fn set_yofs<F: FnOnce(Yofs) -> Yofs>(&self, f: F) -> &Self {
     let value = f(Yofs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the YOFS register."]
  #[inline] pub fn with_yofs<F: FnOnce(Yofs) -> Yofs>(&self, f: F) -> &Self {
     let tmp = self.yofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the G register."]
  #[inline] pub fn g_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xac) as *const u32
  }
#[doc="Get the *mut pointer for the G register."]
  #[inline] pub fn g_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xac) as *mut u32
  }
#[doc="Read the G register."]
  #[inline] pub fn g(&self) -> G { 
     unsafe {
        G(::core::ptr::read_volatile(((self.0 as usize) + 0xac) as *const u32))
     }
  }
#[doc="Write the G register."]
  #[inline] pub fn set_g<F: FnOnce(G) -> G>(&self, f: F) -> &Self {
     let value = f(G(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xac) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the G register."]
  #[inline] pub fn with_g<F: FnOnce(G) -> G>(&self, f: F) -> &Self {
     let tmp = self.g();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xac) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the UG register."]
  #[inline] pub fn ug_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb0) as *const u32
  }
#[doc="Get the *mut pointer for the UG register."]
  #[inline] pub fn ug_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb0) as *mut u32
  }
#[doc="Read the UG register."]
  #[inline] pub fn ug(&self) -> Ug { 
     unsafe {
        Ug(::core::ptr::read_volatile(((self.0 as usize) + 0xb0) as *const u32))
     }
  }
#[doc="Write the UG register."]
  #[inline] pub fn set_ug<F: FnOnce(Ug) -> Ug>(&self, f: F) -> &Self {
     let value = f(Ug(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the UG register."]
  #[inline] pub fn with_ug<F: FnOnce(Ug) -> Ug>(&self, f: F) -> &Self {
     let tmp = self.ug();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLPS register."]
  #[inline] pub fn clps_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb4) as *const u32
  }
#[doc="Get the *mut pointer for the CLPS register."]
  #[inline] pub fn clps_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb4) as *mut u32
  }
#[doc="Read the CLPS register."]
  #[inline] pub fn clps(&self) -> Clps { 
     unsafe {
        Clps(::core::ptr::read_volatile(((self.0 as usize) + 0xb4) as *const u32))
     }
  }
#[doc="Write the CLPS register."]
  #[inline] pub fn set_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
     let value = f(Clps(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLPS register."]
  #[inline] pub fn with_clps<F: FnOnce(Clps) -> Clps>(&self, f: F) -> &Self {
     let tmp = self.clps();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP3 register."]
  #[inline] pub fn clp3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb8) as *const u32
  }
#[doc="Get the *mut pointer for the CLP3 register."]
  #[inline] pub fn clp3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb8) as *mut u32
  }
#[doc="Read the CLP3 register."]
  #[inline] pub fn clp3(&self) -> Clp3 { 
     unsafe {
        Clp3(::core::ptr::read_volatile(((self.0 as usize) + 0xb8) as *const u32))
     }
  }
#[doc="Write the CLP3 register."]
  #[inline] pub fn set_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
     let value = f(Clp3(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP3 register."]
  #[inline] pub fn with_clp3<F: FnOnce(Clp3) -> Clp3>(&self, f: F) -> &Self {
     let tmp = self.clp3();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP2 register."]
  #[inline] pub fn clp2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xbc) as *const u32
  }
#[doc="Get the *mut pointer for the CLP2 register."]
  #[inline] pub fn clp2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xbc) as *mut u32
  }
#[doc="Read the CLP2 register."]
  #[inline] pub fn clp2(&self) -> Clp2 { 
     unsafe {
        Clp2(::core::ptr::read_volatile(((self.0 as usize) + 0xbc) as *const u32))
     }
  }
#[doc="Write the CLP2 register."]
  #[inline] pub fn set_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
     let value = f(Clp2(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xbc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP2 register."]
  #[inline] pub fn with_clp2<F: FnOnce(Clp2) -> Clp2>(&self, f: F) -> &Self {
     let tmp = self.clp2();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xbc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP1 register."]
  #[inline] pub fn clp1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc0) as *const u32
  }
#[doc="Get the *mut pointer for the CLP1 register."]
  #[inline] pub fn clp1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc0) as *mut u32
  }
#[doc="Read the CLP1 register."]
  #[inline] pub fn clp1(&self) -> Clp1 { 
     unsafe {
        Clp1(::core::ptr::read_volatile(((self.0 as usize) + 0xc0) as *const u32))
     }
  }
#[doc="Write the CLP1 register."]
  #[inline] pub fn set_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
     let value = f(Clp1(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP1 register."]
  #[inline] pub fn with_clp1<F: FnOnce(Clp1) -> Clp1>(&self, f: F) -> &Self {
     let tmp = self.clp1();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP0 register."]
  #[inline] pub fn clp0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc4) as *const u32
  }
#[doc="Get the *mut pointer for the CLP0 register."]
  #[inline] pub fn clp0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc4) as *mut u32
  }
#[doc="Read the CLP0 register."]
  #[inline] pub fn clp0(&self) -> Clp0 { 
     unsafe {
        Clp0(::core::ptr::read_volatile(((self.0 as usize) + 0xc4) as *const u32))
     }
  }
#[doc="Write the CLP0 register."]
  #[inline] pub fn set_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
     let value = f(Clp0(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP0 register."]
  #[inline] pub fn with_clp0<F: FnOnce(Clp0) -> Clp0>(&self, f: F) -> &Self {
     let tmp = self.clp0();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLPX register."]
  #[inline] pub fn clpx_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc8) as *const u32
  }
#[doc="Get the *mut pointer for the CLPX register."]
  #[inline] pub fn clpx_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc8) as *mut u32
  }
#[doc="Read the CLPX register."]
  #[inline] pub fn clpx(&self) -> Clpx { 
     unsafe {
        Clpx(::core::ptr::read_volatile(((self.0 as usize) + 0xc8) as *const u32))
     }
  }
#[doc="Write the CLPX register."]
  #[inline] pub fn set_clpx<F: FnOnce(Clpx) -> Clpx>(&self, f: F) -> &Self {
     let value = f(Clpx(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLPX register."]
  #[inline] pub fn with_clpx<F: FnOnce(Clpx) -> Clpx>(&self, f: F) -> &Self {
     let tmp = self.clpx();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP9 register."]
  #[inline] pub fn clp9_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xcc) as *const u32
  }
#[doc="Get the *mut pointer for the CLP9 register."]
  #[inline] pub fn clp9_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xcc) as *mut u32
  }
#[doc="Read the CLP9 register."]
  #[inline] pub fn clp9(&self) -> Clp9 { 
     unsafe {
        Clp9(::core::ptr::read_volatile(((self.0 as usize) + 0xcc) as *const u32))
     }
  }
#[doc="Write the CLP9 register."]
  #[inline] pub fn set_clp9<F: FnOnce(Clp9) -> Clp9>(&self, f: F) -> &Self {
     let value = f(Clp9(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xcc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP9 register."]
  #[inline] pub fn with_clp9<F: FnOnce(Clp9) -> Clp9>(&self, f: F) -> &Self {
     let tmp = self.clp9();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xcc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLPS_OFS register."]
  #[inline] pub fn clps_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd0) as *const u32
  }
#[doc="Get the *mut pointer for the CLPS_OFS register."]
  #[inline] pub fn clps_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd0) as *mut u32
  }
#[doc="Read the CLPS_OFS register."]
  #[inline] pub fn clps_ofs(&self) -> ClpsOfs { 
     unsafe {
        ClpsOfs(::core::ptr::read_volatile(((self.0 as usize) + 0xd0) as *const u32))
     }
  }
#[doc="Write the CLPS_OFS register."]
  #[inline] pub fn set_clps_ofs<F: FnOnce(ClpsOfs) -> ClpsOfs>(&self, f: F) -> &Self {
     let value = f(ClpsOfs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLPS_OFS register."]
  #[inline] pub fn with_clps_ofs<F: FnOnce(ClpsOfs) -> ClpsOfs>(&self, f: F) -> &Self {
     let tmp = self.clps_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP3_OFS register."]
  #[inline] pub fn clp3_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd4) as *const u32
  }
#[doc="Get the *mut pointer for the CLP3_OFS register."]
  #[inline] pub fn clp3_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd4) as *mut u32
  }
#[doc="Read the CLP3_OFS register."]
  #[inline] pub fn clp3_ofs(&self) -> Clp3Ofs { 
     unsafe {
        Clp3Ofs(::core::ptr::read_volatile(((self.0 as usize) + 0xd4) as *const u32))
     }
  }
#[doc="Write the CLP3_OFS register."]
  #[inline] pub fn set_clp3_ofs<F: FnOnce(Clp3Ofs) -> Clp3Ofs>(&self, f: F) -> &Self {
     let value = f(Clp3Ofs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP3_OFS register."]
  #[inline] pub fn with_clp3_ofs<F: FnOnce(Clp3Ofs) -> Clp3Ofs>(&self, f: F) -> &Self {
     let tmp = self.clp3_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP2_OFS register."]
  #[inline] pub fn clp2_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd8) as *const u32
  }
#[doc="Get the *mut pointer for the CLP2_OFS register."]
  #[inline] pub fn clp2_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd8) as *mut u32
  }
#[doc="Read the CLP2_OFS register."]
  #[inline] pub fn clp2_ofs(&self) -> Clp2Ofs { 
     unsafe {
        Clp2Ofs(::core::ptr::read_volatile(((self.0 as usize) + 0xd8) as *const u32))
     }
  }
#[doc="Write the CLP2_OFS register."]
  #[inline] pub fn set_clp2_ofs<F: FnOnce(Clp2Ofs) -> Clp2Ofs>(&self, f: F) -> &Self {
     let value = f(Clp2Ofs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP2_OFS register."]
  #[inline] pub fn with_clp2_ofs<F: FnOnce(Clp2Ofs) -> Clp2Ofs>(&self, f: F) -> &Self {
     let tmp = self.clp2_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP1_OFS register."]
  #[inline] pub fn clp1_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xdc) as *const u32
  }
#[doc="Get the *mut pointer for the CLP1_OFS register."]
  #[inline] pub fn clp1_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xdc) as *mut u32
  }
#[doc="Read the CLP1_OFS register."]
  #[inline] pub fn clp1_ofs(&self) -> Clp1Ofs { 
     unsafe {
        Clp1Ofs(::core::ptr::read_volatile(((self.0 as usize) + 0xdc) as *const u32))
     }
  }
#[doc="Write the CLP1_OFS register."]
  #[inline] pub fn set_clp1_ofs<F: FnOnce(Clp1Ofs) -> Clp1Ofs>(&self, f: F) -> &Self {
     let value = f(Clp1Ofs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xdc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP1_OFS register."]
  #[inline] pub fn with_clp1_ofs<F: FnOnce(Clp1Ofs) -> Clp1Ofs>(&self, f: F) -> &Self {
     let tmp = self.clp1_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xdc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP0_OFS register."]
  #[inline] pub fn clp0_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe0) as *const u32
  }
#[doc="Get the *mut pointer for the CLP0_OFS register."]
  #[inline] pub fn clp0_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe0) as *mut u32
  }
#[doc="Read the CLP0_OFS register."]
  #[inline] pub fn clp0_ofs(&self) -> Clp0Ofs { 
     unsafe {
        Clp0Ofs(::core::ptr::read_volatile(((self.0 as usize) + 0xe0) as *const u32))
     }
  }
#[doc="Write the CLP0_OFS register."]
  #[inline] pub fn set_clp0_ofs<F: FnOnce(Clp0Ofs) -> Clp0Ofs>(&self, f: F) -> &Self {
     let value = f(Clp0Ofs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP0_OFS register."]
  #[inline] pub fn with_clp0_ofs<F: FnOnce(Clp0Ofs) -> Clp0Ofs>(&self, f: F) -> &Self {
     let tmp = self.clp0_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLPX_OFS register."]
  #[inline] pub fn clpx_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe4) as *const u32
  }
#[doc="Get the *mut pointer for the CLPX_OFS register."]
  #[inline] pub fn clpx_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe4) as *mut u32
  }
#[doc="Read the CLPX_OFS register."]
  #[inline] pub fn clpx_ofs(&self) -> ClpxOfs { 
     unsafe {
        ClpxOfs(::core::ptr::read_volatile(((self.0 as usize) + 0xe4) as *const u32))
     }
  }
#[doc="Write the CLPX_OFS register."]
  #[inline] pub fn set_clpx_ofs<F: FnOnce(ClpxOfs) -> ClpxOfs>(&self, f: F) -> &Self {
     let value = f(ClpxOfs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLPX_OFS register."]
  #[inline] pub fn with_clpx_ofs<F: FnOnce(ClpxOfs) -> ClpxOfs>(&self, f: F) -> &Self {
     let tmp = self.clpx_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CLP9_OFS register."]
  #[inline] pub fn clp9_ofs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe8) as *const u32
  }
#[doc="Get the *mut pointer for the CLP9_OFS register."]
  #[inline] pub fn clp9_ofs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe8) as *mut u32
  }
#[doc="Read the CLP9_OFS register."]
  #[inline] pub fn clp9_ofs(&self) -> Clp9Ofs { 
     unsafe {
        Clp9Ofs(::core::ptr::read_volatile(((self.0 as usize) + 0xe8) as *const u32))
     }
  }
#[doc="Write the CLP9_OFS register."]
  #[inline] pub fn set_clp9_ofs<F: FnOnce(Clp9Ofs) -> Clp9Ofs>(&self, f: F) -> &Self {
     let value = f(Clp9Ofs(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLP9_OFS register."]
  #[inline] pub fn with_clp9_ofs<F: FnOnce(Clp9Ofs) -> Clp9Ofs>(&self, f: F) -> &Self {
     let tmp = self.clp9_ofs();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe8) as *mut u32, value.0);
     }
     self
  }

}

#[doc="ADC Status and Control Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc1(pub u32);
impl Sc1 {
#[doc="Input channel select"]
  #[inline] pub fn adch(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
  }
#[doc="Input channel select"]
  #[inline] pub fn set_adch<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Interrupt Enable"]
  #[inline] pub fn aien(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Interrupt Enable"]
  #[inline] pub fn set_aien<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Conversion Complete Flag"]
  #[inline] pub fn coco(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Conversion Complete Flag"]
  #[inline] pub fn set_coco<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Sc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adch() != 0 { try!(write!(f, " adch=0x{:x}", self.adch()))}
      if self.aien() != 0 { try!(write!(f, " aien"))}
      if self.coco() != 0 { try!(write!(f, " coco"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Configuration Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
#[doc="Input Clock Select"]
  #[inline] pub fn adiclk(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
  }
#[doc="Input Clock Select"]
  #[inline] pub fn set_adiclk<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Conversion mode selection"]
  #[inline] pub fn mode(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
  }
#[doc="Conversion mode selection"]
  #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Clock Divide Select"]
  #[inline] pub fn adiv(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
  }
#[doc="Clock Divide Select"]
  #[inline] pub fn set_adiv<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Clear Latch Trigger in Trigger Handler Block"]
  #[inline] pub fn clrltrg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Clear Latch Trigger in Trigger Handler Block"]
  #[inline] pub fn set_clrltrg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Cfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adiclk() != 0 { try!(write!(f, " adiclk=0x{:x}", self.adiclk()))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.adiv() != 0 { try!(write!(f, " adiv=0x{:x}", self.adiv()))}
      if self.clrltrg() != 0 { try!(write!(f, " clrltrg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Configuration Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
#[doc="Sample Time Select"]
  #[inline] pub fn smplts(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Sample Time Select"]
  #[inline] pub fn set_smplts<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfg2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfg2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smplts() != 0 { try!(write!(f, " smplts=0x{:x}", self.smplts()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Data Result Registers"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct R(pub u32);
impl R {
#[doc="Data result"]
  #[inline] pub fn d(&self) -> bits::U12 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
  }
#[doc="Data result"]
  #[inline] pub fn set_d<V: Into<bits::U12>>(mut self, value: V) -> Self {
     let value: bits::U12 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Data result (12 bit)"]
  #[inline] pub fn d12(&self) -> bits::U12 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
  }
#[doc="Data result (12 bit)"]
  #[inline] pub fn set_d12<V: Into<bits::U12>>(mut self, value: V) -> Self {
     let value: bits::U12 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Data result (10 bit)"]
  #[inline] pub fn d10(&self) -> bits::U10 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
  }
#[doc="Data result (10 bit)"]
  #[inline] pub fn set_d10<V: Into<bits::U10>>(mut self, value: V) -> Self {
     let value: bits::U10 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Data result (8 bit)"]
  #[inline] pub fn d8(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Data result (8 bit)"]
  #[inline] pub fn set_d8<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for R {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for R {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d() != 0 { try!(write!(f, " d=0x{:x}", self.d()))}
      if self.d12() != 0 { try!(write!(f, " d12=0x{:x}", self.d12()))}
      if self.d10() != 0 { try!(write!(f, " d10=0x{:x}", self.d10()))}
      if self.d8() != 0 { try!(write!(f, " d8=0x{:x}", self.d8()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Compare Value Registers"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
#[doc="Compare Value."]
  #[inline] pub fn cv(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Compare Value."]
  #[inline] pub fn set_cv<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cv {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cv() != 0 { try!(write!(f, " cv=0x{:x}", self.cv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status and Control Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc2(pub u32);
impl Sc2 {
#[doc="Voltage Reference Selection"]
  #[inline] pub fn refsel(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
  }
#[doc="Voltage Reference Selection"]
  #[inline] pub fn set_refsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="DMA Enable"]
  #[inline] pub fn dmaen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="DMA Enable"]
  #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Compare Function Range Enable"]
  #[inline] pub fn acren(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Compare Function Range Enable"]
  #[inline] pub fn set_acren<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Compare Function Greater Than Enable"]
  #[inline] pub fn acfgt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Compare Function Greater Than Enable"]
  #[inline] pub fn set_acfgt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Compare Function Enable"]
  #[inline] pub fn acfe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Compare Function Enable"]
  #[inline] pub fn set_acfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Conversion Trigger Select"]
  #[inline] pub fn adtrg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Conversion Trigger Select"]
  #[inline] pub fn set_adtrg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Conversion Active"]
  #[inline] pub fn adact(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Conversion Active"]
  #[inline] pub fn set_adact<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Trigger Process Number"]
  #[inline] pub fn trgprnum(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
  }
#[doc="Trigger Process Number"]
  #[inline] pub fn set_trgprnum<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Trigger Status"]
  #[inline] pub fn trgstlat(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
  }
#[doc="Trigger Status"]
  #[inline] pub fn set_trgstlat<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Error in Multiplexed Trigger Request"]
  #[inline] pub fn trgsterr(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
  }
#[doc="Error in Multiplexed Trigger Request"]
  #[inline] pub fn set_trgsterr<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Sc2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      if self.acren() != 0 { try!(write!(f, " acren"))}
      if self.acfgt() != 0 { try!(write!(f, " acfgt"))}
      if self.acfe() != 0 { try!(write!(f, " acfe"))}
      if self.adtrg() != 0 { try!(write!(f, " adtrg"))}
      if self.adact() != 0 { try!(write!(f, " adact"))}
      if self.trgprnum() != 0 { try!(write!(f, " trgprnum=0x{:x}", self.trgprnum()))}
      if self.trgstlat() != 0 { try!(write!(f, " trgstlat=0x{:x}", self.trgstlat()))}
      if self.trgsterr() != 0 { try!(write!(f, " trgsterr=0x{:x}", self.trgsterr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status and Control Register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sc3(pub u32);
impl Sc3 {
#[doc="Hardware Average Select"]
  #[inline] pub fn avgs(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
  }
#[doc="Hardware Average Select"]
  #[inline] pub fn set_avgs<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Hardware Average Enable"]
  #[inline] pub fn avge(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Hardware Average Enable"]
  #[inline] pub fn set_avge<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Continuous Conversion Enable"]
  #[inline] pub fn adco(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Continuous Conversion Enable"]
  #[inline] pub fn set_adco<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Calibration"]
  #[inline] pub fn cal(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Calibration"]
  #[inline] pub fn set_cal<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Sc3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sc3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.avgs() != 0 { try!(write!(f, " avgs=0x{:x}", self.avgs()))}
      if self.avge() != 0 { try!(write!(f, " avge"))}
      if self.adco() != 0 { try!(write!(f, " adco"))}
      if self.cal() != 0 { try!(write!(f, " cal"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="BASE Offset Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BaseOfs(pub u32);
impl BaseOfs {
#[doc="Base Offset Error Correction Value"]
  #[inline] pub fn ba_ofs(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Base Offset Error Correction Value"]
  #[inline] pub fn set_ba_ofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for BaseOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for BaseOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ba_ofs() != 0 { try!(write!(f, " ba_ofs=0x{:x}", self.ba_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Offset Correction Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ofs(pub u32);
impl Ofs {
#[doc="Offset Error Correction Value"]
  #[inline] pub fn ofs(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Offset Error Correction Value"]
  #[inline] pub fn set_ofs<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ofs() != 0 { try!(write!(f, " ofs=0x{:x}", self.ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USER Offset Correction Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UsrOfs(pub u32);
impl UsrOfs {
#[doc="USER Offset Error Correction Value"]
  #[inline] pub fn usr_ofs(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="USER Offset Error Correction Value"]
  #[inline] pub fn set_usr_ofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for UsrOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for UsrOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.usr_ofs() != 0 { try!(write!(f, " usr_ofs=0x{:x}", self.usr_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC X Offset Correction Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Xofs(pub u32);
impl Xofs {
#[doc="X offset error correction value"]
  #[inline] pub fn xofs(&self) -> bits::U6 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
  }
#[doc="X offset error correction value"]
  #[inline] pub fn set_xofs<V: Into<bits::U6>>(mut self, value: V) -> Self {
     let value: bits::U6 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Xofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Xofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.xofs() != 0 { try!(write!(f, " xofs=0x{:x}", self.xofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Y Offset Correction Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Yofs(pub u32);
impl Yofs {
#[doc="Y offset error correction value"]
  #[inline] pub fn yofs(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Y offset error correction value"]
  #[inline] pub fn set_yofs<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Yofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Yofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.yofs() != 0 { try!(write!(f, " yofs=0x{:x}", self.yofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Gain Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct G(pub u32);
impl G {
#[doc="Gain error adjustment factor for the overall conversion"]
  #[inline] pub fn g(&self) -> bits::U11 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
  }
#[doc="Gain error adjustment factor for the overall conversion"]
  #[inline] pub fn set_g<V: Into<bits::U11>>(mut self, value: V) -> Self {
     let value: bits::U11 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for G {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for G {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.g() != 0 { try!(write!(f, " g=0x{:x}", self.g()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC User Gain Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ug(pub u32);
impl Ug {
#[doc="User gain error correction value"]
  #[inline] pub fn ug(&self) -> bits::U10 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
  }
#[doc="User gain error correction value"]
  #[inline] pub fn set_ug<V: Into<bits::U10>>(mut self, value: V) -> Self {
     let value: bits::U10 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ug {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ug {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ug() != 0 { try!(write!(f, " ug=0x{:x}", self.ug()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC General Calibration Value Register S"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clps(pub u32);
impl Clps {
#[doc="Calibration Value"]
  #[inline] pub fn clps(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
  }
#[doc="Calibration Value"]
  #[inline] pub fn set_clps<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clps {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clps() != 0 { try!(write!(f, " clps=0x{:x}", self.clps()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp3(pub u32);
impl Clp3 {
#[doc="Calibration Value"]
  #[inline] pub fn clp3(&self) -> bits::U10 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
  }
#[doc="Calibration Value"]
  #[inline] pub fn set_clp3<V: Into<bits::U10>>(mut self, value: V) -> Self {
     let value: bits::U10 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp3() != 0 { try!(write!(f, " clp3=0x{:x}", self.clp3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp2(pub u32);
impl Clp2 {
#[doc="Calibration Value"]
  #[inline] pub fn clp2(&self) -> bits::U10 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
  }
#[doc="Calibration Value"]
  #[inline] pub fn set_clp2<V: Into<bits::U10>>(mut self, value: V) -> Self {
     let value: bits::U10 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp2() != 0 { try!(write!(f, " clp2=0x{:x}", self.clp2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp1(pub u32);
impl Clp1 {
#[doc="Calibration Value"]
  #[inline] pub fn clp1(&self) -> bits::U9 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
  }
#[doc="Calibration Value"]
  #[inline] pub fn set_clp1<V: Into<bits::U9>>(mut self, value: V) -> Self {
     let value: bits::U9 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp1() != 0 { try!(write!(f, " clp1=0x{:x}", self.clp1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp0(pub u32);
impl Clp0 {
#[doc="Calibration Value"]
  #[inline] pub fn clp0(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Calibration Value"]
  #[inline] pub fn set_clp0<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp0() != 0 { try!(write!(f, " clp0=0x{:x}", self.clp0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register X"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clpx(pub u32);
impl Clpx {
#[doc="Calibration Value"]
  #[inline] pub fn clpx(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
  }
#[doc="Calibration Value"]
  #[inline] pub fn set_clpx<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clpx {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clpx {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clpx() != 0 { try!(write!(f, " clpx=0x{:x}", self.clpx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Value Register 9"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp9(pub u32);
impl Clp9 {
#[doc="Calibration Value"]
  #[inline] pub fn clp9(&self) -> bits::U7 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
  }
#[doc="Calibration Value"]
  #[inline] pub fn set_clp9<V: Into<bits::U7>>(mut self, value: V) -> Self {
     let value: bits::U7 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp9() != 0 { try!(write!(f, " clp9=0x{:x}", self.clp9()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC General Calibration Offset Value Register S"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClpsOfs(pub u32);
impl ClpsOfs {
#[doc="CLPS Offset"]
  #[inline] pub fn clps_ofs(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="CLPS Offset"]
  #[inline] pub fn set_clps_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ClpsOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ClpsOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clps_ofs() != 0 { try!(write!(f, " clps_ofs=0x{:x}", self.clps_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp3Ofs(pub u32);
impl Clp3Ofs {
#[doc="CLP3 Offset"]
  #[inline] pub fn clp3_ofs(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="CLP3 Offset"]
  #[inline] pub fn set_clp3_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp3Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp3Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp3_ofs() != 0 { try!(write!(f, " clp3_ofs=0x{:x}", self.clp3_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp2Ofs(pub u32);
impl Clp2Ofs {
#[doc="CLP2 Offset"]
  #[inline] pub fn clp2_ofs(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="CLP2 Offset"]
  #[inline] pub fn set_clp2_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp2Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp2Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp2_ofs() != 0 { try!(write!(f, " clp2_ofs=0x{:x}", self.clp2_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp1Ofs(pub u32);
impl Clp1Ofs {
#[doc="CLP1 Offset"]
  #[inline] pub fn clp1_ofs(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="CLP1 Offset"]
  #[inline] pub fn set_clp1_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp1Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp1Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp1_ofs() != 0 { try!(write!(f, " clp1_ofs=0x{:x}", self.clp1_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp0Ofs(pub u32);
impl Clp0Ofs {
#[doc="CLP0 Offset"]
  #[inline] pub fn clp0_ofs(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="CLP0 Offset"]
  #[inline] pub fn set_clp0_ofs<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp0Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp0Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp0_ofs() != 0 { try!(write!(f, " clp0_ofs=0x{:x}", self.clp0_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register X"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClpxOfs(pub u32);
impl ClpxOfs {
#[doc="CLPX Offset"]
  #[inline] pub fn clpx_ofs(&self) -> bits::U12 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
  }
#[doc="CLPX Offset"]
  #[inline] pub fn set_clpx_ofs<V: Into<bits::U12>>(mut self, value: V) -> Self {
     let value: bits::U12 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ClpxOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ClpxOfs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clpx_ofs() != 0 { try!(write!(f, " clpx_ofs=0x{:x}", self.clpx_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Plus-Side General Calibration Offset Value Register 9"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clp9Ofs(pub u32);
impl Clp9Ofs {
#[doc="CLP9 Offset"]
  #[inline] pub fn clp9_ofs(&self) -> bits::U12 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
  }
#[doc="CLP9 Offset"]
  #[inline] pub fn set_clp9_ofs<V: Into<bits::U12>>(mut self, value: V) -> Self {
     let value: bits::U12 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clp9Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clp9Ofs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clp9_ofs() != 0 { try!(write!(f, " clp9_ofs=0x{:x}", self.clp9_ofs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="ADC Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

pub const ADC0_CH0: Channel<Adc0Ch0Id, Adc0Id> = Channel { periph: ADC0, index: 0, id: Adc0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch0Id {}
pub type Adc0Ch0 = Channel<Adc0Ch0Id, Adc0Id>;

pub const ADC0_CH1: Channel<Adc0Ch1Id, Adc0Id> = Channel { periph: ADC0, index: 1, id: Adc0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch1Id {}
pub type Adc0Ch1 = Channel<Adc0Ch1Id, Adc0Id>;

pub const ADC0_CH2: Channel<Adc0Ch2Id, Adc0Id> = Channel { periph: ADC0, index: 2, id: Adc0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch2Id {}
pub type Adc0Ch2 = Channel<Adc0Ch2Id, Adc0Id>;

pub const ADC0_CH3: Channel<Adc0Ch3Id, Adc0Id> = Channel { periph: ADC0, index: 3, id: Adc0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch3Id {}
pub type Adc0Ch3 = Channel<Adc0Ch3Id, Adc0Id>;

pub const ADC0_CH4: Channel<Adc0Ch4Id, Adc0Id> = Channel { periph: ADC0, index: 4, id: Adc0Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch4Id {}
pub type Adc0Ch4 = Channel<Adc0Ch4Id, Adc0Id>;

pub const ADC0_CH5: Channel<Adc0Ch5Id, Adc0Id> = Channel { periph: ADC0, index: 5, id: Adc0Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch5Id {}
pub type Adc0Ch5 = Channel<Adc0Ch5Id, Adc0Id>;

pub const ADC0_CH6: Channel<Adc0Ch6Id, Adc0Id> = Channel { periph: ADC0, index: 6, id: Adc0Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch6Id {}
pub type Adc0Ch6 = Channel<Adc0Ch6Id, Adc0Id>;

pub const ADC0_CH7: Channel<Adc0Ch7Id, Adc0Id> = Channel { periph: ADC0, index: 7, id: Adc0Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch7Id {}
pub type Adc0Ch7 = Channel<Adc0Ch7Id, Adc0Id>;

pub const ADC0_CH8: Channel<Adc0Ch8Id, Adc0Id> = Channel { periph: ADC0, index: 8, id: Adc0Ch8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch8Id {}
pub type Adc0Ch8 = Channel<Adc0Ch8Id, Adc0Id>;

pub const ADC0_CH9: Channel<Adc0Ch9Id, Adc0Id> = Channel { periph: ADC0, index: 9, id: Adc0Ch9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch9Id {}
pub type Adc0Ch9 = Channel<Adc0Ch9Id, Adc0Id>;

pub const ADC0_CH10: Channel<Adc0Ch10Id, Adc0Id> = Channel { periph: ADC0, index: 10, id: Adc0Ch10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch10Id {}
pub type Adc0Ch10 = Channel<Adc0Ch10Id, Adc0Id>;

pub const ADC0_CH11: Channel<Adc0Ch11Id, Adc0Id> = Channel { periph: ADC0, index: 11, id: Adc0Ch11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch11Id {}
pub type Adc0Ch11 = Channel<Adc0Ch11Id, Adc0Id>;

pub const ADC0_CH12: Channel<Adc0Ch12Id, Adc0Id> = Channel { periph: ADC0, index: 12, id: Adc0Ch12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch12Id {}
pub type Adc0Ch12 = Channel<Adc0Ch12Id, Adc0Id>;

pub const ADC0_CH13: Channel<Adc0Ch13Id, Adc0Id> = Channel { periph: ADC0, index: 13, id: Adc0Ch13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch13Id {}
pub type Adc0Ch13 = Channel<Adc0Ch13Id, Adc0Id>;

pub const ADC0_CH14: Channel<Adc0Ch14Id, Adc0Id> = Channel { periph: ADC0, index: 14, id: Adc0Ch14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch14Id {}
pub type Adc0Ch14 = Channel<Adc0Ch14Id, Adc0Id>;

pub const ADC0_CH15: Channel<Adc0Ch15Id, Adc0Id> = Channel { periph: ADC0, index: 15, id: Adc0Ch15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch15Id {}
pub type Adc0Ch15 = Channel<Adc0Ch15Id, Adc0Id>;

pub const ADC0_IN0: Channel<Adc0In0Id, Adc0Id> = Channel { periph: ADC0, index: 21, id: Adc0In0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0In0Id {}
pub type Adc0In0 = Channel<Adc0In0Id, Adc0Id>;

pub const ADC0_BANDGAP: Channel<Adc0BandgapId, Adc0Id> = Channel { periph: ADC0, index: 27, id: Adc0BandgapId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0BandgapId {}
pub type Adc0Bandgap = Channel<Adc0BandgapId, Adc0Id>;

pub const ADC0_REFSH: Channel<Adc0RefshId, Adc0Id> = Channel { periph: ADC0, index: 29, id: Adc0RefshId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0RefshId {}
pub type Adc0Refsh = Channel<Adc0RefshId, Adc0Id>;

pub const ADC0_REFSL: Channel<Adc0RefslId, Adc0Id> = Channel { periph: ADC0, index: 30, id: Adc0RefslId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0RefslId {}
pub type Adc0Refsl = Channel<Adc0RefslId, Adc0Id>;

pub const ADC1_CH0: Channel<Adc1Ch0Id, Adc1Id> = Channel { periph: ADC1, index: 0, id: Adc1Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch0Id {}
pub type Adc1Ch0 = Channel<Adc1Ch0Id, Adc1Id>;

pub const ADC1_CH1: Channel<Adc1Ch1Id, Adc1Id> = Channel { periph: ADC1, index: 1, id: Adc1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch1Id {}
pub type Adc1Ch1 = Channel<Adc1Ch1Id, Adc1Id>;

pub const ADC1_CH2: Channel<Adc1Ch2Id, Adc1Id> = Channel { periph: ADC1, index: 2, id: Adc1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch2Id {}
pub type Adc1Ch2 = Channel<Adc1Ch2Id, Adc1Id>;

pub const ADC1_CH3: Channel<Adc1Ch3Id, Adc1Id> = Channel { periph: ADC1, index: 3, id: Adc1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch3Id {}
pub type Adc1Ch3 = Channel<Adc1Ch3Id, Adc1Id>;

pub const ADC1_CH4: Channel<Adc1Ch4Id, Adc1Id> = Channel { periph: ADC1, index: 4, id: Adc1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch4Id {}
pub type Adc1Ch4 = Channel<Adc1Ch4Id, Adc1Id>;

pub const ADC1_CH5: Channel<Adc1Ch5Id, Adc1Id> = Channel { periph: ADC1, index: 5, id: Adc1Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch5Id {}
pub type Adc1Ch5 = Channel<Adc1Ch5Id, Adc1Id>;

pub const ADC1_CH6: Channel<Adc1Ch6Id, Adc1Id> = Channel { periph: ADC1, index: 6, id: Adc1Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch6Id {}
pub type Adc1Ch6 = Channel<Adc1Ch6Id, Adc1Id>;

pub const ADC1_CH7: Channel<Adc1Ch7Id, Adc1Id> = Channel { periph: ADC1, index: 7, id: Adc1Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch7Id {}
pub type Adc1Ch7 = Channel<Adc1Ch7Id, Adc1Id>;

pub const ADC1_CH8: Channel<Adc1Ch8Id, Adc1Id> = Channel { periph: ADC1, index: 8, id: Adc1Ch8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch8Id {}
pub type Adc1Ch8 = Channel<Adc1Ch8Id, Adc1Id>;

pub const ADC1_CH9: Channel<Adc1Ch9Id, Adc1Id> = Channel { periph: ADC1, index: 9, id: Adc1Ch9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch9Id {}
pub type Adc1Ch9 = Channel<Adc1Ch9Id, Adc1Id>;

pub const ADC1_CH10: Channel<Adc1Ch10Id, Adc1Id> = Channel { periph: ADC1, index: 10, id: Adc1Ch10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch10Id {}
pub type Adc1Ch10 = Channel<Adc1Ch10Id, Adc1Id>;

pub const ADC1_CH11: Channel<Adc1Ch11Id, Adc1Id> = Channel { periph: ADC1, index: 11, id: Adc1Ch11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch11Id {}
pub type Adc1Ch11 = Channel<Adc1Ch11Id, Adc1Id>;

pub const ADC1_CH12: Channel<Adc1Ch12Id, Adc1Id> = Channel { periph: ADC1, index: 12, id: Adc1Ch12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch12Id {}
pub type Adc1Ch12 = Channel<Adc1Ch12Id, Adc1Id>;

pub const ADC1_CH13: Channel<Adc1Ch13Id, Adc1Id> = Channel { periph: ADC1, index: 13, id: Adc1Ch13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch13Id {}
pub type Adc1Ch13 = Channel<Adc1Ch13Id, Adc1Id>;

pub const ADC1_CH14: Channel<Adc1Ch14Id, Adc1Id> = Channel { periph: ADC1, index: 14, id: Adc1Ch14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch14Id {}
pub type Adc1Ch14 = Channel<Adc1Ch14Id, Adc1Id>;

pub const ADC1_CH15: Channel<Adc1Ch15Id, Adc1Id> = Channel { periph: ADC1, index: 15, id: Adc1Ch15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch15Id {}
pub type Adc1Ch15 = Channel<Adc1Ch15Id, Adc1Id>;

pub const ADC1_IN0: Channel<Adc1In0Id, Adc1Id> = Channel { periph: ADC1, index: 21, id: Adc1In0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1In0Id {}
pub type Adc1In0 = Channel<Adc1In0Id, Adc1Id>;

pub const ADC1_BANDGAP: Channel<Adc1BandgapId, Adc1Id> = Channel { periph: ADC1, index: 27, id: Adc1BandgapId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1BandgapId {}
pub type Adc1Bandgap = Channel<Adc1BandgapId, Adc1Id>;

pub const ADC1_REFSH: Channel<Adc1RefshId, Adc1Id> = Channel { periph: ADC1, index: 29, id: Adc1RefshId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1RefshId {}
pub type Adc1Refsh = Channel<Adc1RefshId, Adc1Id>;

pub const ADC1_REFSL: Channel<Adc1RefslId, Adc1Id> = Channel { periph: ADC1, index: 30, id: Adc1RefslId {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1RefslId {}
pub type Adc1Refsl = Channel<Adc1RefslId, Adc1Id>;

