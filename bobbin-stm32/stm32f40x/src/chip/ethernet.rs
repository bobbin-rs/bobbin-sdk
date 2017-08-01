//! Ethernet: media access control (MAC)
pub const ETHERNET_MAC: Ethernet = Ethernet(0x40028000);

#[doc="Ethernet: media access control (MAC)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ethernet(pub u32);
impl Ethernet {
#[doc="Get the *const pointer for the MACCR register."]
  #[inline] pub fn maccr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the MACCR register."]
  #[inline] pub fn maccr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the MACCR register."]
  #[inline] pub fn maccr(&self) -> Maccr { 
     unsafe {
        Maccr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the MACCR register."]
  #[inline] pub fn set_maccr(&self, value: Maccr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACCR register."]
  #[inline] pub fn with_maccr<F: FnOnce(Maccr) -> Maccr>(&self, f: F) -> &Self {
     let tmp = self.maccr();
     self.set_maccr(f(tmp))
  }

#[doc="Get the *const pointer for the MACFFR register."]
  #[inline] pub fn macffr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the MACFFR register."]
  #[inline] pub fn macffr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the MACFFR register."]
  #[inline] pub fn macffr(&self) -> Macffr { 
     unsafe {
        Macffr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the MACFFR register."]
  #[inline] pub fn set_macffr(&self, value: Macffr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACFFR register."]
  #[inline] pub fn with_macffr<F: FnOnce(Macffr) -> Macffr>(&self, f: F) -> &Self {
     let tmp = self.macffr();
     self.set_macffr(f(tmp))
  }

#[doc="Get the *const pointer for the MACHTHR register."]
  #[inline] pub fn machthr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the MACHTHR register."]
  #[inline] pub fn machthr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the MACHTHR register."]
  #[inline] pub fn machthr(&self) -> Machthr { 
     unsafe {
        Machthr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the MACHTHR register."]
  #[inline] pub fn set_machthr(&self, value: Machthr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACHTHR register."]
  #[inline] pub fn with_machthr<F: FnOnce(Machthr) -> Machthr>(&self, f: F) -> &Self {
     let tmp = self.machthr();
     self.set_machthr(f(tmp))
  }

#[doc="Get the *const pointer for the MACHTLR register."]
  #[inline] pub fn machtlr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the MACHTLR register."]
  #[inline] pub fn machtlr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the MACHTLR register."]
  #[inline] pub fn machtlr(&self) -> Machtlr { 
     unsafe {
        Machtlr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the MACHTLR register."]
  #[inline] pub fn set_machtlr(&self, value: Machtlr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACHTLR register."]
  #[inline] pub fn with_machtlr<F: FnOnce(Machtlr) -> Machtlr>(&self, f: F) -> &Self {
     let tmp = self.machtlr();
     self.set_machtlr(f(tmp))
  }

#[doc="Get the *const pointer for the MACMIIAR register."]
  #[inline] pub fn macmiiar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the MACMIIAR register."]
  #[inline] pub fn macmiiar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the MACMIIAR register."]
  #[inline] pub fn macmiiar(&self) -> Macmiiar { 
     unsafe {
        Macmiiar(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the MACMIIAR register."]
  #[inline] pub fn set_macmiiar(&self, value: Macmiiar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACMIIAR register."]
  #[inline] pub fn with_macmiiar<F: FnOnce(Macmiiar) -> Macmiiar>(&self, f: F) -> &Self {
     let tmp = self.macmiiar();
     self.set_macmiiar(f(tmp))
  }

#[doc="Get the *const pointer for the MACMIIDR register."]
  #[inline] pub fn macmiidr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the MACMIIDR register."]
  #[inline] pub fn macmiidr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the MACMIIDR register."]
  #[inline] pub fn macmiidr(&self) -> Macmiidr { 
     unsafe {
        Macmiidr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the MACMIIDR register."]
  #[inline] pub fn set_macmiidr(&self, value: Macmiidr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACMIIDR register."]
  #[inline] pub fn with_macmiidr<F: FnOnce(Macmiidr) -> Macmiidr>(&self, f: F) -> &Self {
     let tmp = self.macmiidr();
     self.set_macmiidr(f(tmp))
  }

#[doc="Get the *const pointer for the MACFCR register."]
  #[inline] pub fn macfcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the MACFCR register."]
  #[inline] pub fn macfcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the MACFCR register."]
  #[inline] pub fn macfcr(&self) -> Macfcr { 
     unsafe {
        Macfcr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the MACFCR register."]
  #[inline] pub fn set_macfcr(&self, value: Macfcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACFCR register."]
  #[inline] pub fn with_macfcr<F: FnOnce(Macfcr) -> Macfcr>(&self, f: F) -> &Self {
     let tmp = self.macfcr();
     self.set_macfcr(f(tmp))
  }

#[doc="Get the *const pointer for the MACVLANTR register."]
  #[inline] pub fn macvlantr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the MACVLANTR register."]
  #[inline] pub fn macvlantr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the MACVLANTR register."]
  #[inline] pub fn macvlantr(&self) -> Macvlantr { 
     unsafe {
        Macvlantr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the MACVLANTR register."]
  #[inline] pub fn set_macvlantr(&self, value: Macvlantr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACVLANTR register."]
  #[inline] pub fn with_macvlantr<F: FnOnce(Macvlantr) -> Macvlantr>(&self, f: F) -> &Self {
     let tmp = self.macvlantr();
     self.set_macvlantr(f(tmp))
  }

#[doc="Get the *const pointer for the MACPMTCSR register."]
  #[inline] pub fn macpmtcsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the MACPMTCSR register."]
  #[inline] pub fn macpmtcsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Read the MACPMTCSR register."]
  #[inline] pub fn macpmtcsr(&self) -> Macpmtcsr { 
     unsafe {
        Macpmtcsr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
#[doc="Write the MACPMTCSR register."]
  #[inline] pub fn set_macpmtcsr(&self, value: Macpmtcsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACPMTCSR register."]
  #[inline] pub fn with_macpmtcsr<F: FnOnce(Macpmtcsr) -> Macpmtcsr>(&self, f: F) -> &Self {
     let tmp = self.macpmtcsr();
     self.set_macpmtcsr(f(tmp))
  }

#[doc="Get the *const pointer for the MACDBGR register."]
  #[inline] pub fn macdbgr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the MACDBGR register."]
  #[inline] pub fn macdbgr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Read the MACDBGR register."]
  #[inline] pub fn macdbgr(&self) -> Macdbgr { 
     unsafe {
        Macdbgr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }

#[doc="Get the *const pointer for the MACSR register."]
  #[inline] pub fn macsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
#[doc="Get the *mut pointer for the MACSR register."]
  #[inline] pub fn macsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
#[doc="Read the MACSR register."]
  #[inline] pub fn macsr(&self) -> Macsr { 
     unsafe {
        Macsr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
#[doc="Write the MACSR register."]
  #[inline] pub fn set_macsr(&self, value: Macsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACSR register."]
  #[inline] pub fn with_macsr<F: FnOnce(Macsr) -> Macsr>(&self, f: F) -> &Self {
     let tmp = self.macsr();
     self.set_macsr(f(tmp))
  }

#[doc="Get the *const pointer for the MACIMR register."]
  #[inline] pub fn macimr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
#[doc="Get the *mut pointer for the MACIMR register."]
  #[inline] pub fn macimr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
#[doc="Read the MACIMR register."]
  #[inline] pub fn macimr(&self) -> Macimr { 
     unsafe {
        Macimr(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
#[doc="Write the MACIMR register."]
  #[inline] pub fn set_macimr(&self, value: Macimr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACIMR register."]
  #[inline] pub fn with_macimr<F: FnOnce(Macimr) -> Macimr>(&self, f: F) -> &Self {
     let tmp = self.macimr();
     self.set_macimr(f(tmp))
  }

#[doc="Get the *const pointer for the MACA0HR register."]
  #[inline] pub fn maca0hr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the MACA0HR register."]
  #[inline] pub fn maca0hr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the MACA0HR register."]
  #[inline] pub fn maca0hr(&self) -> Maca0hr { 
     unsafe {
        Maca0hr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the MACA0HR register."]
  #[inline] pub fn set_maca0hr(&self, value: Maca0hr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACA0HR register."]
  #[inline] pub fn with_maca0hr<F: FnOnce(Maca0hr) -> Maca0hr>(&self, f: F) -> &Self {
     let tmp = self.maca0hr();
     self.set_maca0hr(f(tmp))
  }

#[doc="Get the *const pointer for the MACA0LR register."]
  #[inline] pub fn maca0lr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the MACA0LR register."]
  #[inline] pub fn maca0lr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the MACA0LR register."]
  #[inline] pub fn maca0lr(&self) -> Maca0lr { 
     unsafe {
        Maca0lr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the MACA0LR register."]
  #[inline] pub fn set_maca0lr(&self, value: Maca0lr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACA0LR register."]
  #[inline] pub fn with_maca0lr<F: FnOnce(Maca0lr) -> Maca0lr>(&self, f: F) -> &Self {
     let tmp = self.maca0lr();
     self.set_maca0lr(f(tmp))
  }

#[doc="Get the *const pointer for the MACA1HR register."]
  #[inline] pub fn maca1hr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
#[doc="Get the *mut pointer for the MACA1HR register."]
  #[inline] pub fn maca1hr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
#[doc="Read the MACA1HR register."]
  #[inline] pub fn maca1hr(&self) -> Maca1hr { 
     unsafe {
        Maca1hr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
#[doc="Write the MACA1HR register."]
  #[inline] pub fn set_maca1hr(&self, value: Maca1hr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACA1HR register."]
  #[inline] pub fn with_maca1hr<F: FnOnce(Maca1hr) -> Maca1hr>(&self, f: F) -> &Self {
     let tmp = self.maca1hr();
     self.set_maca1hr(f(tmp))
  }

#[doc="Get the *const pointer for the MACA1LR register."]
  #[inline] pub fn maca1lr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
#[doc="Get the *mut pointer for the MACA1LR register."]
  #[inline] pub fn maca1lr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
#[doc="Read the MACA1LR register."]
  #[inline] pub fn maca1lr(&self) -> Maca1lr { 
     unsafe {
        Maca1lr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
#[doc="Write the MACA1LR register."]
  #[inline] pub fn set_maca1lr(&self, value: Maca1lr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACA1LR register."]
  #[inline] pub fn with_maca1lr<F: FnOnce(Maca1lr) -> Maca1lr>(&self, f: F) -> &Self {
     let tmp = self.maca1lr();
     self.set_maca1lr(f(tmp))
  }

#[doc="Get the *const pointer for the MACA2HR register."]
  #[inline] pub fn maca2hr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
#[doc="Get the *mut pointer for the MACA2HR register."]
  #[inline] pub fn maca2hr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
#[doc="Read the MACA2HR register."]
  #[inline] pub fn maca2hr(&self) -> Maca2hr { 
     unsafe {
        Maca2hr(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
#[doc="Write the MACA2HR register."]
  #[inline] pub fn set_maca2hr(&self, value: Maca2hr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACA2HR register."]
  #[inline] pub fn with_maca2hr<F: FnOnce(Maca2hr) -> Maca2hr>(&self, f: F) -> &Self {
     let tmp = self.maca2hr();
     self.set_maca2hr(f(tmp))
  }

#[doc="Get the *const pointer for the MACA2LR register."]
  #[inline] pub fn maca2lr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
#[doc="Get the *mut pointer for the MACA2LR register."]
  #[inline] pub fn maca2lr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
#[doc="Read the MACA2LR register."]
  #[inline] pub fn maca2lr(&self) -> Maca2lr { 
     unsafe {
        Maca2lr(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
#[doc="Write the MACA2LR register."]
  #[inline] pub fn set_maca2lr(&self, value: Maca2lr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACA2LR register."]
  #[inline] pub fn with_maca2lr<F: FnOnce(Maca2lr) -> Maca2lr>(&self, f: F) -> &Self {
     let tmp = self.maca2lr();
     self.set_maca2lr(f(tmp))
  }

#[doc="Get the *const pointer for the MACA3HR register."]
  #[inline] pub fn maca3hr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
#[doc="Get the *mut pointer for the MACA3HR register."]
  #[inline] pub fn maca3hr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
#[doc="Read the MACA3HR register."]
  #[inline] pub fn maca3hr(&self) -> Maca3hr { 
     unsafe {
        Maca3hr(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
#[doc="Write the MACA3HR register."]
  #[inline] pub fn set_maca3hr(&self, value: Maca3hr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACA3HR register."]
  #[inline] pub fn with_maca3hr<F: FnOnce(Maca3hr) -> Maca3hr>(&self, f: F) -> &Self {
     let tmp = self.maca3hr();
     self.set_maca3hr(f(tmp))
  }

#[doc="Get the *const pointer for the MACA3LR register."]
  #[inline] pub fn maca3lr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
#[doc="Get the *mut pointer for the MACA3LR register."]
  #[inline] pub fn maca3lr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
#[doc="Read the MACA3LR register."]
  #[inline] pub fn maca3lr(&self) -> Maca3lr { 
     unsafe {
        Maca3lr(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }
#[doc="Write the MACA3LR register."]
  #[inline] pub fn set_maca3lr(&self, value: Maca3lr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MACA3LR register."]
  #[inline] pub fn with_maca3lr<F: FnOnce(Maca3lr) -> Maca3lr>(&self, f: F) -> &Self {
     let tmp = self.maca3lr();
     self.set_maca3lr(f(tmp))
  }

}

#[doc="Ethernet MAC configuration register"]
#[derive(PartialEq, Eq)]
pub struct Maccr(pub u32);
impl Maccr {
#[doc="RE"]
  #[inline] pub fn re(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="RE"]
  #[inline] pub fn set_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="TE"]
  #[inline] pub fn te(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="TE"]
  #[inline] pub fn set_te(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="DC"]
  #[inline] pub fn dc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="DC"]
  #[inline] pub fn set_dc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="BL"]
  #[inline] pub fn bl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
#[doc="BL"]
  #[inline] pub fn set_bl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="APCS"]
  #[inline] pub fn apcs(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="APCS"]
  #[inline] pub fn set_apcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="RD"]
  #[inline] pub fn rd(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="RD"]
  #[inline] pub fn set_rd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="IPCO"]
  #[inline] pub fn ipco(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="IPCO"]
  #[inline] pub fn set_ipco(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="DM"]
  #[inline] pub fn dm(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="DM"]
  #[inline] pub fn set_dm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="LM"]
  #[inline] pub fn lm(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="LM"]
  #[inline] pub fn set_lm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="ROD"]
  #[inline] pub fn rod(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="ROD"]
  #[inline] pub fn set_rod(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="FES"]
  #[inline] pub fn fes(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="FES"]
  #[inline] pub fn set_fes(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="CSD"]
  #[inline] pub fn csd(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="CSD"]
  #[inline] pub fn set_csd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="IFG"]
  #[inline] pub fn ifg(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7 // [19:17]
  }
#[doc="IFG"]
  #[inline] pub fn set_ifg(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="JD"]
  #[inline] pub fn jd(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="JD"]
  #[inline] pub fn set_jd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="WD"]
  #[inline] pub fn wd(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="WD"]
  #[inline] pub fn set_wd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="CSTF"]
  #[inline] pub fn cstf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="CSTF"]
  #[inline] pub fn set_cstf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

}
impl ::core::fmt::Display for Maccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.re() != 0 { try!(write!(f, " re"))}
      if self.te() != 0 { try!(write!(f, " te"))}
      if self.dc() != 0 { try!(write!(f, " dc"))}
      if self.bl() != 0 { try!(write!(f, " bl=0x{:x}", self.bl()))}
      if self.apcs() != 0 { try!(write!(f, " apcs"))}
      if self.rd() != 0 { try!(write!(f, " rd"))}
      if self.ipco() != 0 { try!(write!(f, " ipco"))}
      if self.dm() != 0 { try!(write!(f, " dm"))}
      if self.lm() != 0 { try!(write!(f, " lm"))}
      if self.rod() != 0 { try!(write!(f, " rod"))}
      if self.fes() != 0 { try!(write!(f, " fes"))}
      if self.csd() != 0 { try!(write!(f, " csd"))}
      if self.ifg() != 0 { try!(write!(f, " ifg=0x{:x}", self.ifg()))}
      if self.jd() != 0 { try!(write!(f, " jd"))}
      if self.wd() != 0 { try!(write!(f, " wd"))}
      if self.cstf() != 0 { try!(write!(f, " cstf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC frame filter register"]
#[derive(PartialEq, Eq)]
pub struct Macffr(pub u32);
impl Macffr {
#[doc="no description available"]
  #[inline] pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="no description available"]
  #[inline] pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="no description available"]
  #[inline] pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="no description available"]
  #[inline] pub fn hm(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="no description available"]
  #[inline] pub fn set_hm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="no description available"]
  #[inline] pub fn daif(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="no description available"]
  #[inline] pub fn set_daif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="no description available"]
  #[inline] pub fn ram(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="no description available"]
  #[inline] pub fn set_ram(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="no description available"]
  #[inline] pub fn bfd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="no description available"]
  #[inline] pub fn set_bfd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="no description available"]
  #[inline] pub fn pcf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="no description available"]
  #[inline] pub fn set_pcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="no description available"]
  #[inline] pub fn saif(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="no description available"]
  #[inline] pub fn set_saif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="no description available"]
  #[inline] pub fn saf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="no description available"]
  #[inline] pub fn set_saf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="no description available"]
  #[inline] pub fn hpf(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="no description available"]
  #[inline] pub fn set_hpf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="no description available"]
  #[inline] pub fn ra(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="no description available"]
  #[inline] pub fn set_ra(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Macffr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macffr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.hu() != 0 { try!(write!(f, " hu"))}
      if self.hm() != 0 { try!(write!(f, " hm"))}
      if self.daif() != 0 { try!(write!(f, " daif"))}
      if self.ram() != 0 { try!(write!(f, " ram"))}
      if self.bfd() != 0 { try!(write!(f, " bfd"))}
      if self.pcf() != 0 { try!(write!(f, " pcf"))}
      if self.saif() != 0 { try!(write!(f, " saif"))}
      if self.saf() != 0 { try!(write!(f, " saf"))}
      if self.hpf() != 0 { try!(write!(f, " hpf"))}
      if self.ra() != 0 { try!(write!(f, " ra"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC hash table high register"]
#[derive(PartialEq, Eq)]
pub struct Machthr(pub u32);
impl Machthr {
#[doc="no description available"]
  #[inline] pub fn hth(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_hth(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Machthr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Machthr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC hash table low register"]
#[derive(PartialEq, Eq)]
pub struct Machtlr(pub u32);
impl Machtlr {
#[doc="no description available"]
  #[inline] pub fn htl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_htl(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Machtlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Machtlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC MII address register"]
#[derive(PartialEq, Eq)]
pub struct Macmiiar(pub u32);
impl Macmiiar {
#[doc="no description available"]
  #[inline] pub fn mb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="no description available"]
  #[inline] pub fn set_mb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mw(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="no description available"]
  #[inline] pub fn set_mw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="no description available"]
  #[inline] pub fn cr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x7 // [4:2]
  }
#[doc="no description available"]
  #[inline] pub fn set_cr(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1f // [10:6]
  }
#[doc="no description available"]
  #[inline] pub fn set_mr(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 6);
     self.0 |= value << 6;
     self
  }

#[doc="no description available"]
  #[inline] pub fn pa(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1f // [15:11]
  }
#[doc="no description available"]
  #[inline] pub fn set_pa(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Macmiiar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macmiiar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mb() != 0 { try!(write!(f, " mb"))}
      if self.mw() != 0 { try!(write!(f, " mw"))}
      if self.cr() != 0 { try!(write!(f, " cr=0x{:x}", self.cr()))}
      if self.mr() != 0 { try!(write!(f, " mr=0x{:x}", self.mr()))}
      if self.pa() != 0 { try!(write!(f, " pa=0x{:x}", self.pa()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC MII data register"]
#[derive(PartialEq, Eq)]
pub struct Macmiidr(pub u32);
impl Macmiidr {
#[doc="no description available"]
  #[inline] pub fn td(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_td(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Macmiidr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macmiidr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.td() != 0 { try!(write!(f, " td=0x{:x}", self.td()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC flow control register"]
#[derive(PartialEq, Eq)]
pub struct Macfcr(pub u32);
impl Macfcr {
#[doc="no description available"]
  #[inline] pub fn fcb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="no description available"]
  #[inline] pub fn set_fcb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tfce(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="no description available"]
  #[inline] pub fn set_tfce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="no description available"]
  #[inline] pub fn rfce(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="no description available"]
  #[inline] pub fn set_rfce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="no description available"]
  #[inline] pub fn upfd(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="no description available"]
  #[inline] pub fn set_upfd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="no description available"]
  #[inline] pub fn plt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="no description available"]
  #[inline] pub fn set_plt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="no description available"]
  #[inline] pub fn zqpd(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="no description available"]
  #[inline] pub fn set_zqpd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="no description available"]
  #[inline] pub fn pt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
#[doc="no description available"]
  #[inline] pub fn set_pt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Macfcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macfcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fcb() != 0 { try!(write!(f, " fcb"))}
      if self.tfce() != 0 { try!(write!(f, " tfce"))}
      if self.rfce() != 0 { try!(write!(f, " rfce"))}
      if self.upfd() != 0 { try!(write!(f, " upfd"))}
      if self.plt() != 0 { try!(write!(f, " plt=0x{:x}", self.plt()))}
      if self.zqpd() != 0 { try!(write!(f, " zqpd"))}
      if self.pt() != 0 { try!(write!(f, " pt=0x{:x}", self.pt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC VLAN tag register"]
#[derive(PartialEq, Eq)]
pub struct Macvlantr(pub u32);
impl Macvlantr {
#[doc="no description available"]
  #[inline] pub fn vlanti(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_vlanti(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn vlantc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="no description available"]
  #[inline] pub fn set_vlantc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Macvlantr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macvlantr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vlanti() != 0 { try!(write!(f, " vlanti=0x{:x}", self.vlanti()))}
      if self.vlantc() != 0 { try!(write!(f, " vlantc"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC PMT control and status register"]
#[derive(PartialEq, Eq)]
pub struct Macpmtcsr(pub u32);
impl Macpmtcsr {
#[doc="no description available"]
  #[inline] pub fn pd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="no description available"]
  #[inline] pub fn set_pd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mpe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="no description available"]
  #[inline] pub fn set_mpe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="no description available"]
  #[inline] pub fn wfe(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="no description available"]
  #[inline] pub fn set_wfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mpr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="no description available"]
  #[inline] pub fn set_mpr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="no description available"]
  #[inline] pub fn wfr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="no description available"]
  #[inline] pub fn set_wfr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="no description available"]
  #[inline] pub fn gu(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="no description available"]
  #[inline] pub fn set_gu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="no description available"]
  #[inline] pub fn wffrpr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="no description available"]
  #[inline] pub fn set_wffrpr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Macpmtcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macpmtcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pd() != 0 { try!(write!(f, " pd"))}
      if self.mpe() != 0 { try!(write!(f, " mpe"))}
      if self.wfe() != 0 { try!(write!(f, " wfe"))}
      if self.mpr() != 0 { try!(write!(f, " mpr"))}
      if self.wfr() != 0 { try!(write!(f, " wfr"))}
      if self.gu() != 0 { try!(write!(f, " gu"))}
      if self.wffrpr() != 0 { try!(write!(f, " wffrpr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC debug register"]
#[derive(PartialEq, Eq)]
pub struct Macdbgr(pub u32);
impl Macdbgr {
#[doc="CR"]
  #[inline] pub fn cr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="CR"]
  #[inline] pub fn set_cr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="CSR"]
  #[inline] pub fn csr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="CSR"]
  #[inline] pub fn set_csr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="ROR"]
  #[inline] pub fn ror(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="ROR"]
  #[inline] pub fn set_ror(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="MCF"]
  #[inline] pub fn mcf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="MCF"]
  #[inline] pub fn set_mcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="MCP"]
  #[inline] pub fn mcp(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="MCP"]
  #[inline] pub fn set_mcp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="MCFHP"]
  #[inline] pub fn mcfhp(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="MCFHP"]
  #[inline] pub fn set_mcfhp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Macdbgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macdbgr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cr() != 0 { try!(write!(f, " cr"))}
      if self.csr() != 0 { try!(write!(f, " csr"))}
      if self.ror() != 0 { try!(write!(f, " ror"))}
      if self.mcf() != 0 { try!(write!(f, " mcf"))}
      if self.mcp() != 0 { try!(write!(f, " mcp"))}
      if self.mcfhp() != 0 { try!(write!(f, " mcfhp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC interrupt status register"]
#[derive(PartialEq, Eq)]
pub struct Macsr(pub u32);
impl Macsr {
#[doc="no description available"]
  #[inline] pub fn pmts(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="no description available"]
  #[inline] pub fn set_pmts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mmcs(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="no description available"]
  #[inline] pub fn set_mmcs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mmcrs(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="no description available"]
  #[inline] pub fn set_mmcrs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mmcts(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="no description available"]
  #[inline] pub fn set_mmcts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tsts(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="no description available"]
  #[inline] pub fn set_tsts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

}
impl ::core::fmt::Display for Macsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pmts() != 0 { try!(write!(f, " pmts"))}
      if self.mmcs() != 0 { try!(write!(f, " mmcs"))}
      if self.mmcrs() != 0 { try!(write!(f, " mmcrs"))}
      if self.mmcts() != 0 { try!(write!(f, " mmcts"))}
      if self.tsts() != 0 { try!(write!(f, " tsts"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC interrupt mask register"]
#[derive(PartialEq, Eq)]
pub struct Macimr(pub u32);
impl Macimr {
#[doc="no description available"]
  #[inline] pub fn pmtim(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="no description available"]
  #[inline] pub fn set_pmtim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="no description available"]
  #[inline] pub fn tstim(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="no description available"]
  #[inline] pub fn set_tstim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

}
impl ::core::fmt::Display for Macimr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Macimr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pmtim() != 0 { try!(write!(f, " pmtim"))}
      if self.tstim() != 0 { try!(write!(f, " tstim"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC address 0 high register"]
#[derive(PartialEq, Eq)]
pub struct Maca0hr(pub u32);
impl Maca0hr {
#[doc="MAC address0 high"]
  #[inline] pub fn maca0h(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="MAC address0 high"]
  #[inline] pub fn set_maca0h(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Always 1"]
  #[inline] pub fn mo(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Always 1"]
  #[inline] pub fn set_mo(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Maca0hr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maca0hr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maca0h() != 0 { try!(write!(f, " maca0h=0x{:x}", self.maca0h()))}
      if self.mo() != 0 { try!(write!(f, " mo"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC address 0 low register"]
#[derive(PartialEq, Eq)]
pub struct Maca0lr(pub u32);
impl Maca0lr {
#[doc="0"]
  #[inline] pub fn maca0l(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="0"]
  #[inline] pub fn set_maca0l(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Maca0lr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maca0lr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC address 1 high register"]
#[derive(PartialEq, Eq)]
pub struct Maca1hr(pub u32);
impl Maca1hr {
#[doc="no description available"]
  #[inline] pub fn maca1h(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_maca1h(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
#[doc="no description available"]
  #[inline] pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

#[doc="no description available"]
  #[inline] pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="no description available"]
  #[inline] pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="no description available"]
  #[inline] pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="no description available"]
  #[inline] pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Maca1hr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maca1hr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maca1h() != 0 { try!(write!(f, " maca1h=0x{:x}", self.maca1h()))}
      if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
      if self.sa() != 0 { try!(write!(f, " sa"))}
      if self.ae() != 0 { try!(write!(f, " ae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC address1 low register"]
#[derive(PartialEq, Eq)]
pub struct Maca1lr(pub u32);
impl Maca1lr {
#[doc="no description available"]
  #[inline] pub fn maca1lr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_maca1lr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Maca1lr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maca1lr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC address 2 high register"]
#[derive(PartialEq, Eq)]
pub struct Maca2hr(pub u32);
impl Maca2hr {
#[doc="no description available"]
  #[inline] pub fn mac2ah(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_mac2ah(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
#[doc="no description available"]
  #[inline] pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

#[doc="no description available"]
  #[inline] pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="no description available"]
  #[inline] pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="no description available"]
  #[inline] pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="no description available"]
  #[inline] pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Maca2hr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maca2hr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mac2ah() != 0 { try!(write!(f, " mac2ah=0x{:x}", self.mac2ah()))}
      if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
      if self.sa() != 0 { try!(write!(f, " sa"))}
      if self.ae() != 0 { try!(write!(f, " ae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC address 2 low register"]
#[derive(PartialEq, Eq)]
pub struct Maca2lr(pub u32);
impl Maca2lr {
#[doc="no description available"]
  #[inline] pub fn maca2l(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_maca2l(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Maca2lr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maca2lr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maca2l() != 0 { try!(write!(f, " maca2l=0x{:x}", self.maca2l()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC address 3 high register"]
#[derive(PartialEq, Eq)]
pub struct Maca3hr(pub u32);
impl Maca3hr {
#[doc="no description available"]
  #[inline] pub fn maca3h(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_maca3h(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="no description available"]
  #[inline] pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
#[doc="no description available"]
  #[inline] pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

#[doc="no description available"]
  #[inline] pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="no description available"]
  #[inline] pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="no description available"]
  #[inline] pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="no description available"]
  #[inline] pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Maca3hr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maca3hr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maca3h() != 0 { try!(write!(f, " maca3h=0x{:x}", self.maca3h()))}
      if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
      if self.sa() != 0 { try!(write!(f, " sa"))}
      if self.ae() != 0 { try!(write!(f, " ae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC address 3 low register"]
#[derive(PartialEq, Eq)]
pub struct Maca3lr(pub u32);
impl Maca3lr {
#[doc="no description available"]
  #[inline] pub fn mbca3l(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="no description available"]
  #[inline] pub fn set_mbca3l(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Maca3lr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Maca3lr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

