#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMA Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the LISR register."]
  #[inline] pub fn lisr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the LISR register."]
  #[inline] pub fn lisr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the LISR register."]
  #[inline] pub fn lisr(&self) -> Lisr { 
     unsafe {
        Lisr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the HISR register."]
  #[inline] pub fn hisr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the HISR register."]
  #[inline] pub fn hisr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the HISR register."]
  #[inline] pub fn hisr(&self) -> Hisr { 
     unsafe {
        Hisr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }

#[doc="Get the *const pointer for the LIFCR register."]
  #[inline] pub fn lifcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the LIFCR register."]
  #[inline] pub fn lifcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the LIFCR register."]
  #[inline] pub fn lifcr(&self) -> Lifcr { 
     unsafe {
        Lifcr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the LIFCR register."]
  #[inline] pub fn set_lifcr<F: FnOnce(Lifcr) -> Lifcr>(&self, f: F) -> &Self {
     let value = f(Lifcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LIFCR register."]
  #[inline] pub fn with_lifcr<F: FnOnce(Lifcr) -> Lifcr>(&self, f: F) -> &Self {
     let tmp = self.lifcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the HIFCR register."]
  #[inline] pub fn hifcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the HIFCR register."]
  #[inline] pub fn hifcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the HIFCR register."]
  #[inline] pub fn hifcr(&self) -> Hifcr { 
     unsafe {
        Hifcr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the HIFCR register."]
  #[inline] pub fn set_hifcr<F: FnOnce(Hifcr) -> Hifcr>(&self, f: F) -> &Self {
     let value = f(Hifcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the HIFCR register."]
  #[inline] pub fn with_hifcr<F: FnOnce(Hifcr) -> Hifcr>(&self, f: F) -> &Self {
     let tmp = self.hifcr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SCR register."]
  #[inline] pub fn scr_ptr<I: Into<bits::R8>>(&self, index: I) -> *const u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x10 + (index * 24)) as *const u32
  }
#[doc="Get the *mut pointer for the SCR register."]
  #[inline] pub fn scr_mut<I: Into<bits::R8>>(&self, index: I) -> *mut u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x10 + (index * 24)) as *mut u32
  }
#[doc="Read the SCR register."]
  #[inline] pub fn scr<I: Into<bits::R8>>(&self, index: I) -> Scr { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Scr(::core::ptr::read_volatile(((self.0 as usize) + 0x10 + (index * 24)) as *const u32))
     }
  }
#[doc="Write the SCR register."]
  #[inline] pub fn set_scr<I: Into<bits::R8>, F: FnOnce(Scr) -> Scr>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Scr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10 + (index * 24)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SCR register."]
  #[inline] pub fn with_scr<I: Into<bits::R8> + Copy, F: FnOnce(Scr) -> Scr>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.scr(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10 + (index * 24)) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SNDTR register."]
  #[inline] pub fn sndtr_ptr<I: Into<bits::R8>>(&self, index: I) -> *const u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x14 + (index * 24)) as *const u32
  }
#[doc="Get the *mut pointer for the SNDTR register."]
  #[inline] pub fn sndtr_mut<I: Into<bits::R8>>(&self, index: I) -> *mut u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x14 + (index * 24)) as *mut u32
  }
#[doc="Read the SNDTR register."]
  #[inline] pub fn sndtr<I: Into<bits::R8>>(&self, index: I) -> Sndtr { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Sndtr(::core::ptr::read_volatile(((self.0 as usize) + 0x14 + (index * 24)) as *const u32))
     }
  }
#[doc="Write the SNDTR register."]
  #[inline] pub fn set_sndtr<I: Into<bits::R8>, F: FnOnce(Sndtr) -> Sndtr>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Sndtr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14 + (index * 24)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SNDTR register."]
  #[inline] pub fn with_sndtr<I: Into<bits::R8> + Copy, F: FnOnce(Sndtr) -> Sndtr>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.sndtr(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14 + (index * 24)) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SPAR register."]
  #[inline] pub fn spar_ptr<I: Into<bits::R8>>(&self, index: I) -> *const u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x18 + (index * 24)) as *const u32
  }
#[doc="Get the *mut pointer for the SPAR register."]
  #[inline] pub fn spar_mut<I: Into<bits::R8>>(&self, index: I) -> *mut u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x18 + (index * 24)) as *mut u32
  }
#[doc="Read the SPAR register."]
  #[inline] pub fn spar<I: Into<bits::R8>>(&self, index: I) -> Spar { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Spar(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index * 24)) as *const u32))
     }
  }
#[doc="Write the SPAR register."]
  #[inline] pub fn set_spar<I: Into<bits::R8>, F: FnOnce(Spar) -> Spar>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Spar(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index * 24)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SPAR register."]
  #[inline] pub fn with_spar<I: Into<bits::R8> + Copy, F: FnOnce(Spar) -> Spar>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.spar(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index * 24)) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SM0AR register."]
  #[inline] pub fn sm0ar_ptr<I: Into<bits::R8>>(&self, index: I) -> *const u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x1c + (index * 24)) as *const u32
  }
#[doc="Get the *mut pointer for the SM0AR register."]
  #[inline] pub fn sm0ar_mut<I: Into<bits::R8>>(&self, index: I) -> *mut u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x1c + (index * 24)) as *mut u32
  }
#[doc="Read the SM0AR register."]
  #[inline] pub fn sm0ar<I: Into<bits::R8>>(&self, index: I) -> Sm0ar { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Sm0ar(::core::ptr::read_volatile(((self.0 as usize) + 0x1c + (index * 24)) as *const u32))
     }
  }
#[doc="Write the SM0AR register."]
  #[inline] pub fn set_sm0ar<I: Into<bits::R8>, F: FnOnce(Sm0ar) -> Sm0ar>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Sm0ar(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c + (index * 24)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SM0AR register."]
  #[inline] pub fn with_sm0ar<I: Into<bits::R8> + Copy, F: FnOnce(Sm0ar) -> Sm0ar>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.sm0ar(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c + (index * 24)) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SM1AR register."]
  #[inline] pub fn sm1ar_ptr<I: Into<bits::R8>>(&self, index: I) -> *const u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x20 + (index * 24)) as *const u32
  }
#[doc="Get the *mut pointer for the SM1AR register."]
  #[inline] pub fn sm1ar_mut<I: Into<bits::R8>>(&self, index: I) -> *mut u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x20 + (index * 24)) as *mut u32
  }
#[doc="Read the SM1AR register."]
  #[inline] pub fn sm1ar<I: Into<bits::R8>>(&self, index: I) -> Sm1ar { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Sm1ar(::core::ptr::read_volatile(((self.0 as usize) + 0x20 + (index * 24)) as *const u32))
     }
  }
#[doc="Write the SM1AR register."]
  #[inline] pub fn set_sm1ar<I: Into<bits::R8>, F: FnOnce(Sm1ar) -> Sm1ar>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Sm1ar(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20 + (index * 24)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SM1AR register."]
  #[inline] pub fn with_sm1ar<I: Into<bits::R8> + Copy, F: FnOnce(Sm1ar) -> Sm1ar>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.sm1ar(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20 + (index * 24)) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SFCR register."]
  #[inline] pub fn sfcr_ptr<I: Into<bits::R8>>(&self, index: I) -> *const u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x24 + (index * 24)) as *const u32
  }
#[doc="Get the *mut pointer for the SFCR register."]
  #[inline] pub fn sfcr_mut<I: Into<bits::R8>>(&self, index: I) -> *mut u32 { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     ((self.0 as usize) + 0x24 + (index * 24)) as *mut u32
  }
#[doc="Read the SFCR register."]
  #[inline] pub fn sfcr<I: Into<bits::R8>>(&self, index: I) -> Sfcr { 
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     unsafe {
        Sfcr(::core::ptr::read_volatile(((self.0 as usize) + 0x24 + (index * 24)) as *const u32))
     }
  }
#[doc="Write the SFCR register."]
  #[inline] pub fn set_sfcr<I: Into<bits::R8>, F: FnOnce(Sfcr) -> Sfcr>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let value = f(Sfcr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24 + (index * 24)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SFCR register."]
  #[inline] pub fn with_sfcr<I: Into<bits::R8> + Copy, F: FnOnce(Sfcr) -> Sfcr>(&self, index: I, f: F) -> &Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value() as usize;
     let tmp = self.sfcr(index);
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24 + (index * 24)) as *mut u32, value.0);
     }
     self
  }

}

#[doc="low interrupt status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lisr(pub u32);
impl Lisr {
#[doc="Stream x transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn tcif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
  }
#[doc="Stream x transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn set_tcif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Stream x half transfer interrupt flag (x=3..0)"]
  #[inline] pub fn htif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
  }
#[doc="Stream x half transfer interrupt flag (x=3..0)"]
  #[inline] pub fn set_htif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Stream x transfer error interrupt flag (x=3..0)"]
  #[inline] pub fn teif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
  }
#[doc="Stream x transfer error interrupt flag (x=3..0)"]
  #[inline] pub fn set_teif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Stream x direct mode error interrupt flag (x=3..0)"]
  #[inline] pub fn dmeif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
  }
#[doc="Stream x direct mode error interrupt flag (x=3..0)"]
  #[inline] pub fn set_dmeif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Stream x FIFO error interrupt flag (x=3..0)"]
  #[inline] pub fn feif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="Stream x FIFO error interrupt flag (x=3..0)"]
  #[inline] pub fn set_feif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Stream x transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn tcif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
  }
#[doc="Stream x transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn set_tcif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Stream x half transfer interrupt flag (x=3..0)"]
  #[inline] pub fn htif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="Stream x half transfer interrupt flag (x=3..0)"]
  #[inline] pub fn set_htif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Stream x transfer error interrupt flag (x=3..0)"]
  #[inline] pub fn teif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Stream x transfer error interrupt flag (x=3..0)"]
  #[inline] pub fn set_teif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Stream x direct mode error interrupt flag (x=3..0)"]
  #[inline] pub fn dmeif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Stream x direct mode error interrupt flag (x=3..0)"]
  #[inline] pub fn set_dmeif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Stream x FIFO error interrupt flag (x=3..0)"]
  #[inline] pub fn feif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Stream x FIFO error interrupt flag (x=3..0)"]
  #[inline] pub fn set_feif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Stream x transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn tcif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Stream x transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn set_tcif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Stream x half transfer interrupt flag (x=3..0)"]
  #[inline] pub fn htif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Stream x half transfer interrupt flag (x=3..0)"]
  #[inline] pub fn set_htif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Stream x transfer error interrupt flag (x=3..0)"]
  #[inline] pub fn teif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Stream x transfer error interrupt flag (x=3..0)"]
  #[inline] pub fn set_teif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Stream x direct mode error interrupt flag (x=3..0)"]
  #[inline] pub fn dmeif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Stream x direct mode error interrupt flag (x=3..0)"]
  #[inline] pub fn set_dmeif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Stream x FIFO error interrupt flag (x=3..0)"]
  #[inline] pub fn feif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Stream x FIFO error interrupt flag (x=3..0)"]
  #[inline] pub fn set_feif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Stream x transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn tcif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Stream x transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn set_tcif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Stream x half transfer interrupt flag (x=3..0)"]
  #[inline] pub fn htif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Stream x half transfer interrupt flag (x=3..0)"]
  #[inline] pub fn set_htif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Stream x transfer error interrupt flag (x=3..0)"]
  #[inline] pub fn teif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Stream x transfer error interrupt flag (x=3..0)"]
  #[inline] pub fn set_teif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Stream x direct mode error interrupt flag (x=3..0)"]
  #[inline] pub fn dmeif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Stream x direct mode error interrupt flag (x=3..0)"]
  #[inline] pub fn set_dmeif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Stream x FIFO error interrupt flag (x=3..0)"]
  #[inline] pub fn feif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Stream x FIFO error interrupt flag (x=3..0)"]
  #[inline] pub fn set_feif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Lisr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lisr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tcif3() != 0 { try!(write!(f, " tcif3"))}
      if self.htif3() != 0 { try!(write!(f, " htif3"))}
      if self.teif3() != 0 { try!(write!(f, " teif3"))}
      if self.dmeif3() != 0 { try!(write!(f, " dmeif3"))}
      if self.feif3() != 0 { try!(write!(f, " feif3"))}
      if self.tcif2() != 0 { try!(write!(f, " tcif2"))}
      if self.htif2() != 0 { try!(write!(f, " htif2"))}
      if self.teif2() != 0 { try!(write!(f, " teif2"))}
      if self.dmeif2() != 0 { try!(write!(f, " dmeif2"))}
      if self.feif2() != 0 { try!(write!(f, " feif2"))}
      if self.tcif1() != 0 { try!(write!(f, " tcif1"))}
      if self.htif1() != 0 { try!(write!(f, " htif1"))}
      if self.teif1() != 0 { try!(write!(f, " teif1"))}
      if self.dmeif1() != 0 { try!(write!(f, " dmeif1"))}
      if self.feif1() != 0 { try!(write!(f, " feif1"))}
      if self.tcif0() != 0 { try!(write!(f, " tcif0"))}
      if self.htif0() != 0 { try!(write!(f, " htif0"))}
      if self.teif0() != 0 { try!(write!(f, " teif0"))}
      if self.dmeif0() != 0 { try!(write!(f, " dmeif0"))}
      if self.feif0() != 0 { try!(write!(f, " feif0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="high interrupt status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Hisr(pub u32);
impl Hisr {
#[doc="Stream x transfer complete interrupt flag (x=7..4)"]
  #[inline] pub fn tcif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
  }
#[doc="Stream x transfer complete interrupt flag (x=7..4)"]
  #[inline] pub fn set_tcif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Stream x half transfer interrupt flag (x=7..4)"]
  #[inline] pub fn htif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
  }
#[doc="Stream x half transfer interrupt flag (x=7..4)"]
  #[inline] pub fn set_htif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Stream x transfer error interrupt flag (x=7..4)"]
  #[inline] pub fn teif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
  }
#[doc="Stream x transfer error interrupt flag (x=7..4)"]
  #[inline] pub fn set_teif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Stream x direct mode error interrupt flag (x=7..4)"]
  #[inline] pub fn dmeif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
  }
#[doc="Stream x direct mode error interrupt flag (x=7..4)"]
  #[inline] pub fn set_dmeif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Stream x FIFO error interrupt flag (x=7..4)"]
  #[inline] pub fn feif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="Stream x FIFO error interrupt flag (x=7..4)"]
  #[inline] pub fn set_feif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Stream x transfer complete interrupt flag (x=7..4)"]
  #[inline] pub fn tcif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
  }
#[doc="Stream x transfer complete interrupt flag (x=7..4)"]
  #[inline] pub fn set_tcif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Stream x half transfer interrupt flag (x=7..4)"]
  #[inline] pub fn htif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="Stream x half transfer interrupt flag (x=7..4)"]
  #[inline] pub fn set_htif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Stream x transfer error interrupt flag (x=7..4)"]
  #[inline] pub fn teif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Stream x transfer error interrupt flag (x=7..4)"]
  #[inline] pub fn set_teif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Stream x direct mode error interrupt flag (x=7..4)"]
  #[inline] pub fn dmeif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Stream x direct mode error interrupt flag (x=7..4)"]
  #[inline] pub fn set_dmeif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Stream x FIFO error interrupt flag (x=7..4)"]
  #[inline] pub fn feif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Stream x FIFO error interrupt flag (x=7..4)"]
  #[inline] pub fn set_feif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Stream x transfer complete interrupt flag (x=7..4)"]
  #[inline] pub fn tcif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Stream x transfer complete interrupt flag (x=7..4)"]
  #[inline] pub fn set_tcif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Stream x half transfer interrupt flag (x=7..4)"]
  #[inline] pub fn htif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Stream x half transfer interrupt flag (x=7..4)"]
  #[inline] pub fn set_htif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Stream x transfer error interrupt flag (x=7..4)"]
  #[inline] pub fn teif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Stream x transfer error interrupt flag (x=7..4)"]
  #[inline] pub fn set_teif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Stream x direct mode error interrupt flag (x=7..4)"]
  #[inline] pub fn dmeif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Stream x direct mode error interrupt flag (x=7..4)"]
  #[inline] pub fn set_dmeif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Stream x FIFO error interrupt flag (x=7..4)"]
  #[inline] pub fn feif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Stream x FIFO error interrupt flag (x=7..4)"]
  #[inline] pub fn set_feif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Stream x transfer complete interrupt flag (x=7..4)"]
  #[inline] pub fn tcif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Stream x transfer complete interrupt flag (x=7..4)"]
  #[inline] pub fn set_tcif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Stream x half transfer interrupt flag (x=7..4)"]
  #[inline] pub fn htif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Stream x half transfer interrupt flag (x=7..4)"]
  #[inline] pub fn set_htif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Stream x transfer error interrupt flag (x=7..4)"]
  #[inline] pub fn teif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Stream x transfer error interrupt flag (x=7..4)"]
  #[inline] pub fn set_teif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Stream x direct mode error interrupt flag (x=7..4)"]
  #[inline] pub fn dmeif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Stream x direct mode error interrupt flag (x=7..4)"]
  #[inline] pub fn set_dmeif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Stream x FIFO error interrupt flag (x=7..4)"]
  #[inline] pub fn feif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Stream x FIFO error interrupt flag (x=7..4)"]
  #[inline] pub fn set_feif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Hisr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Hisr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tcif7() != 0 { try!(write!(f, " tcif7"))}
      if self.htif7() != 0 { try!(write!(f, " htif7"))}
      if self.teif7() != 0 { try!(write!(f, " teif7"))}
      if self.dmeif7() != 0 { try!(write!(f, " dmeif7"))}
      if self.feif7() != 0 { try!(write!(f, " feif7"))}
      if self.tcif6() != 0 { try!(write!(f, " tcif6"))}
      if self.htif6() != 0 { try!(write!(f, " htif6"))}
      if self.teif6() != 0 { try!(write!(f, " teif6"))}
      if self.dmeif6() != 0 { try!(write!(f, " dmeif6"))}
      if self.feif6() != 0 { try!(write!(f, " feif6"))}
      if self.tcif5() != 0 { try!(write!(f, " tcif5"))}
      if self.htif5() != 0 { try!(write!(f, " htif5"))}
      if self.teif5() != 0 { try!(write!(f, " teif5"))}
      if self.dmeif5() != 0 { try!(write!(f, " dmeif5"))}
      if self.feif5() != 0 { try!(write!(f, " feif5"))}
      if self.tcif4() != 0 { try!(write!(f, " tcif4"))}
      if self.htif4() != 0 { try!(write!(f, " htif4"))}
      if self.teif4() != 0 { try!(write!(f, " teif4"))}
      if self.dmeif4() != 0 { try!(write!(f, " dmeif4"))}
      if self.feif4() != 0 { try!(write!(f, " feif4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="low interrupt flag clear register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lifcr(pub u32);
impl Lifcr {
#[doc="Stream x clear transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn ctcif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
  }
#[doc="Stream x clear transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn set_ctcif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Stream x clear half transfer interrupt flag (x = 3..0)"]
  #[inline] pub fn chtif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
  }
#[doc="Stream x clear half transfer interrupt flag (x = 3..0)"]
  #[inline] pub fn set_chtif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Stream x clear transfer error interrupt flag (x = 3..0)"]
  #[inline] pub fn cteif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
  }
#[doc="Stream x clear transfer error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cteif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Stream x clear direct mode error interrupt flag (x = 3..0)"]
  #[inline] pub fn cdmeif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
  }
#[doc="Stream x clear direct mode error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cdmeif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Stream x clear FIFO error interrupt flag (x = 3..0)"]
  #[inline] pub fn cfeif3(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="Stream x clear FIFO error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cfeif3<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Stream x clear transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn ctcif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
  }
#[doc="Stream x clear transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn set_ctcif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Stream x clear half transfer interrupt flag (x = 3..0)"]
  #[inline] pub fn chtif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="Stream x clear half transfer interrupt flag (x = 3..0)"]
  #[inline] pub fn set_chtif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Stream x clear transfer error interrupt flag (x = 3..0)"]
  #[inline] pub fn cteif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Stream x clear transfer error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cteif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Stream x clear direct mode error interrupt flag (x = 3..0)"]
  #[inline] pub fn cdmeif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Stream x clear direct mode error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cdmeif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Stream x clear FIFO error interrupt flag (x = 3..0)"]
  #[inline] pub fn cfeif2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Stream x clear FIFO error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cfeif2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Stream x clear transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn ctcif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Stream x clear transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn set_ctcif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Stream x clear half transfer interrupt flag (x = 3..0)"]
  #[inline] pub fn chtif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Stream x clear half transfer interrupt flag (x = 3..0)"]
  #[inline] pub fn set_chtif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Stream x clear transfer error interrupt flag (x = 3..0)"]
  #[inline] pub fn cteif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Stream x clear transfer error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cteif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Stream x clear direct mode error interrupt flag (x = 3..0)"]
  #[inline] pub fn cdmeif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Stream x clear direct mode error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cdmeif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Stream x clear FIFO error interrupt flag (x = 3..0)"]
  #[inline] pub fn cfeif1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Stream x clear FIFO error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cfeif1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Stream x clear transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn ctcif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Stream x clear transfer complete interrupt flag (x = 3..0)"]
  #[inline] pub fn set_ctcif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Stream x clear half transfer interrupt flag (x = 3..0)"]
  #[inline] pub fn chtif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Stream x clear half transfer interrupt flag (x = 3..0)"]
  #[inline] pub fn set_chtif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Stream x clear transfer error interrupt flag (x = 3..0)"]
  #[inline] pub fn cteif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Stream x clear transfer error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cteif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Stream x clear direct mode error interrupt flag (x = 3..0)"]
  #[inline] pub fn cdmeif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Stream x clear direct mode error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cdmeif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Stream x clear FIFO error interrupt flag (x = 3..0)"]
  #[inline] pub fn cfeif0(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Stream x clear FIFO error interrupt flag (x = 3..0)"]
  #[inline] pub fn set_cfeif0<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Lifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ctcif3() != 0 { try!(write!(f, " ctcif3"))}
      if self.chtif3() != 0 { try!(write!(f, " chtif3"))}
      if self.cteif3() != 0 { try!(write!(f, " cteif3"))}
      if self.cdmeif3() != 0 { try!(write!(f, " cdmeif3"))}
      if self.cfeif3() != 0 { try!(write!(f, " cfeif3"))}
      if self.ctcif2() != 0 { try!(write!(f, " ctcif2"))}
      if self.chtif2() != 0 { try!(write!(f, " chtif2"))}
      if self.cteif2() != 0 { try!(write!(f, " cteif2"))}
      if self.cdmeif2() != 0 { try!(write!(f, " cdmeif2"))}
      if self.cfeif2() != 0 { try!(write!(f, " cfeif2"))}
      if self.ctcif1() != 0 { try!(write!(f, " ctcif1"))}
      if self.chtif1() != 0 { try!(write!(f, " chtif1"))}
      if self.cteif1() != 0 { try!(write!(f, " cteif1"))}
      if self.cdmeif1() != 0 { try!(write!(f, " cdmeif1"))}
      if self.cfeif1() != 0 { try!(write!(f, " cfeif1"))}
      if self.ctcif0() != 0 { try!(write!(f, " ctcif0"))}
      if self.chtif0() != 0 { try!(write!(f, " chtif0"))}
      if self.cteif0() != 0 { try!(write!(f, " cteif0"))}
      if self.cdmeif0() != 0 { try!(write!(f, " cdmeif0"))}
      if self.cfeif0() != 0 { try!(write!(f, " cfeif0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="high interrupt flag clear register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Hifcr(pub u32);
impl Hifcr {
#[doc="Stream x clear transfer complete interrupt flag (x = 7..4)"]
  #[inline] pub fn ctcif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
  }
#[doc="Stream x clear transfer complete interrupt flag (x = 7..4)"]
  #[inline] pub fn set_ctcif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Stream x clear half transfer interrupt flag (x = 7..4)"]
  #[inline] pub fn chtif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
  }
#[doc="Stream x clear half transfer interrupt flag (x = 7..4)"]
  #[inline] pub fn set_chtif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Stream x clear transfer error interrupt flag (x = 7..4)"]
  #[inline] pub fn cteif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
  }
#[doc="Stream x clear transfer error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cteif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Stream x clear direct mode error interrupt flag (x = 7..4)"]
  #[inline] pub fn cdmeif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
  }
#[doc="Stream x clear direct mode error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cdmeif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Stream x clear FIFO error interrupt flag (x = 7..4)"]
  #[inline] pub fn cfeif7(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
  }
#[doc="Stream x clear FIFO error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cfeif7<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Stream x clear transfer complete interrupt flag (x = 7..4)"]
  #[inline] pub fn ctcif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
  }
#[doc="Stream x clear transfer complete interrupt flag (x = 7..4)"]
  #[inline] pub fn set_ctcif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Stream x clear half transfer interrupt flag (x = 7..4)"]
  #[inline] pub fn chtif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
  }
#[doc="Stream x clear half transfer interrupt flag (x = 7..4)"]
  #[inline] pub fn set_chtif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Stream x clear transfer error interrupt flag (x = 7..4)"]
  #[inline] pub fn cteif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Stream x clear transfer error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cteif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Stream x clear direct mode error interrupt flag (x = 7..4)"]
  #[inline] pub fn cdmeif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Stream x clear direct mode error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cdmeif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Stream x clear FIFO error interrupt flag (x = 7..4)"]
  #[inline] pub fn cfeif6(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
  }
#[doc="Stream x clear FIFO error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cfeif6<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Stream x clear transfer complete interrupt flag (x = 7..4)"]
  #[inline] pub fn ctcif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Stream x clear transfer complete interrupt flag (x = 7..4)"]
  #[inline] pub fn set_ctcif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Stream x clear half transfer interrupt flag (x = 7..4)"]
  #[inline] pub fn chtif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Stream x clear half transfer interrupt flag (x = 7..4)"]
  #[inline] pub fn set_chtif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Stream x clear transfer error interrupt flag (x = 7..4)"]
  #[inline] pub fn cteif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Stream x clear transfer error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cteif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Stream x clear direct mode error interrupt flag (x = 7..4)"]
  #[inline] pub fn cdmeif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Stream x clear direct mode error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cdmeif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Stream x clear FIFO error interrupt flag (x = 7..4)"]
  #[inline] pub fn cfeif5(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Stream x clear FIFO error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cfeif5<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Stream x clear transfer complete interrupt flag (x = 7..4)"]
  #[inline] pub fn ctcif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Stream x clear transfer complete interrupt flag (x = 7..4)"]
  #[inline] pub fn set_ctcif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Stream x clear half transfer interrupt flag (x = 7..4)"]
  #[inline] pub fn chtif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Stream x clear half transfer interrupt flag (x = 7..4)"]
  #[inline] pub fn set_chtif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Stream x clear transfer error interrupt flag (x = 7..4)"]
  #[inline] pub fn cteif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Stream x clear transfer error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cteif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Stream x clear direct mode error interrupt flag (x = 7..4)"]
  #[inline] pub fn cdmeif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Stream x clear direct mode error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cdmeif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Stream x clear FIFO error interrupt flag (x = 7..4)"]
  #[inline] pub fn cfeif4(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Stream x clear FIFO error interrupt flag (x = 7..4)"]
  #[inline] pub fn set_cfeif4<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Hifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Hifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ctcif7() != 0 { try!(write!(f, " ctcif7"))}
      if self.chtif7() != 0 { try!(write!(f, " chtif7"))}
      if self.cteif7() != 0 { try!(write!(f, " cteif7"))}
      if self.cdmeif7() != 0 { try!(write!(f, " cdmeif7"))}
      if self.cfeif7() != 0 { try!(write!(f, " cfeif7"))}
      if self.ctcif6() != 0 { try!(write!(f, " ctcif6"))}
      if self.chtif6() != 0 { try!(write!(f, " chtif6"))}
      if self.cteif6() != 0 { try!(write!(f, " cteif6"))}
      if self.cdmeif6() != 0 { try!(write!(f, " cdmeif6"))}
      if self.cfeif6() != 0 { try!(write!(f, " cfeif6"))}
      if self.ctcif5() != 0 { try!(write!(f, " ctcif5"))}
      if self.chtif5() != 0 { try!(write!(f, " chtif5"))}
      if self.cteif5() != 0 { try!(write!(f, " cteif5"))}
      if self.cdmeif5() != 0 { try!(write!(f, " cdmeif5"))}
      if self.cfeif5() != 0 { try!(write!(f, " cfeif5"))}
      if self.ctcif4() != 0 { try!(write!(f, " ctcif4"))}
      if self.chtif4() != 0 { try!(write!(f, " chtif4"))}
      if self.cteif4() != 0 { try!(write!(f, " cteif4"))}
      if self.cdmeif4() != 0 { try!(write!(f, " cdmeif4"))}
      if self.cfeif4() != 0 { try!(write!(f, " cfeif4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="stream x configuration register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scr(pub u32);
impl Scr {
#[doc="Channel selection"]
  #[inline] pub fn chsel(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x7) as u8) } // [27:25]
  }
#[doc="Channel selection"]
  #[inline] pub fn set_chsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Memory burst transfer configuration"]
  #[inline] pub fn mburst(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x3) as u8) } // [24:23]
  }
#[doc="Memory burst transfer configuration"]
  #[inline] pub fn set_mburst<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Peripheral burst transfer configuration"]
  #[inline] pub fn pburst(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
  }
#[doc="Peripheral burst transfer configuration"]
  #[inline] pub fn set_pburst<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Current target (only in double buffer mode)"]
  #[inline] pub fn ct(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
  }
#[doc="Current target (only in double buffer mode)"]
  #[inline] pub fn set_ct<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Double buffer mode"]
  #[inline] pub fn dbm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Double buffer mode"]
  #[inline] pub fn set_dbm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Priority level"]
  #[inline] pub fn pl(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
  }
#[doc="Priority level"]
  #[inline] pub fn set_pl<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Peripheral increment offset size"]
  #[inline] pub fn pincos(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Peripheral increment offset size"]
  #[inline] pub fn set_pincos<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Memory data size"]
  #[inline] pub fn msize(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
  }
#[doc="Memory data size"]
  #[inline] pub fn set_msize<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Peripheral data size"]
  #[inline] pub fn psize(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
  }
#[doc="Peripheral data size"]
  #[inline] pub fn set_psize<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Memory increment mode"]
  #[inline] pub fn minc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Memory increment mode"]
  #[inline] pub fn set_minc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Peripheral increment mode"]
  #[inline] pub fn pinc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="Peripheral increment mode"]
  #[inline] pub fn set_pinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Circular mode"]
  #[inline] pub fn circ(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="Circular mode"]
  #[inline] pub fn set_circ<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Data transfer direction"]
  #[inline] pub fn dir(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
  }
#[doc="Data transfer direction"]
  #[inline] pub fn set_dir<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Peripheral flow controller"]
  #[inline] pub fn pfctrl(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Peripheral flow controller"]
  #[inline] pub fn set_pfctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Transfer complete interrupt enable"]
  #[inline] pub fn tcie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Transfer complete interrupt enable"]
  #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Half transfer interrupt enable"]
  #[inline] pub fn htie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Half transfer interrupt enable"]
  #[inline] pub fn set_htie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Transfer error interrupt enable"]
  #[inline] pub fn teie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Transfer error interrupt enable"]
  #[inline] pub fn set_teie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Direct mode error interrupt enable"]
  #[inline] pub fn dmeie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Direct mode error interrupt enable"]
  #[inline] pub fn set_dmeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Stream enable / flag stream ready when read low"]
  #[inline] pub fn en(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Stream enable / flag stream ready when read low"]
  #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Scr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
      if self.mburst() != 0 { try!(write!(f, " mburst=0x{:x}", self.mburst()))}
      if self.pburst() != 0 { try!(write!(f, " pburst=0x{:x}", self.pburst()))}
      if self.ct() != 0 { try!(write!(f, " ct"))}
      if self.dbm() != 0 { try!(write!(f, " dbm"))}
      if self.pl() != 0 { try!(write!(f, " pl=0x{:x}", self.pl()))}
      if self.pincos() != 0 { try!(write!(f, " pincos"))}
      if self.msize() != 0 { try!(write!(f, " msize=0x{:x}", self.msize()))}
      if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
      if self.minc() != 0 { try!(write!(f, " minc"))}
      if self.pinc() != 0 { try!(write!(f, " pinc"))}
      if self.circ() != 0 { try!(write!(f, " circ"))}
      if self.dir() != 0 { try!(write!(f, " dir=0x{:x}", self.dir()))}
      if self.pfctrl() != 0 { try!(write!(f, " pfctrl"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.htie() != 0 { try!(write!(f, " htie"))}
      if self.teie() != 0 { try!(write!(f, " teie"))}
      if self.dmeie() != 0 { try!(write!(f, " dmeie"))}
      if self.en() != 0 { try!(write!(f, " en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="stream x number of data register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sndtr(pub u32);
impl Sndtr {
#[doc="Number of data items to transfer"]
  #[inline] pub fn ndt(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="Number of data items to transfer"]
  #[inline] pub fn set_ndt<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sndtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sndtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ndt() != 0 { try!(write!(f, " ndt=0x{:x}", self.ndt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="stream x peripheral address register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spar(pub u32);
impl Spar {
#[doc="Peripheral address"]
  #[inline] pub fn pa(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Peripheral address"]
  #[inline] pub fn set_pa<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Spar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Spar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="stream x memory 0 address register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sm0ar(pub u32);
impl Sm0ar {
#[doc="Memory 0 address"]
  #[inline] pub fn m0a(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Memory 0 address"]
  #[inline] pub fn set_m0a<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sm0ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sm0ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="stream x memory 1 address register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sm1ar(pub u32);
impl Sm1ar {
#[doc="Memory 1 address (used in case of Double buffer mode)"]
  #[inline] pub fn m1a(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Memory 1 address (used in case of Double buffer mode)"]
  #[inline] pub fn set_m1a<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sm1ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sm1ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="stream x FIFO control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sfcr(pub u32);
impl Sfcr {
#[doc="FIFO error interrupt enable"]
  #[inline] pub fn feie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="FIFO error interrupt enable"]
  #[inline] pub fn set_feie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="FIFO status"]
  #[inline] pub fn fs(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
  }
#[doc="FIFO status"]
  #[inline] pub fn set_fs<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Direct mode disable"]
  #[inline] pub fn dmdis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Direct mode disable"]
  #[inline] pub fn set_dmdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="FIFO threshold selection"]
  #[inline] pub fn fth(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
  }
#[doc="FIFO threshold selection"]
  #[inline] pub fn set_fth<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sfcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sfcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.feie() != 0 { try!(write!(f, " feie"))}
      if self.fs() != 0 { try!(write!(f, " fs=0x{:x}", self.fs()))}
      if self.dmdis() != 0 { try!(write!(f, " dmdis"))}
      if self.fth() != 0 { try!(write!(f, " fth=0x{:x}", self.fth()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="DMA Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

