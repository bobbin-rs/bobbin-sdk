#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="FLEXCAN Peripheral"]
pub struct FlexcanPeriph(pub usize); 


impl FlexcanPeriph {
#[doc="Get the *const pointer for the MCR register."]
   #[inline] pub fn mcr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the MCR register."]
   #[inline] pub fn mcr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the MCR register."]
   #[inline] pub fn mcr(&self) -> Mcr { 
      unsafe {
         Mcr(::core::ptr::read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the MCR register."]
   #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
      let value = f(Mcr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MCR register."]
   #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
      let tmp = self.mcr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL1 register."]
   #[inline] pub fn ctrl1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the CTRL1 register."]
   #[inline] pub fn ctrl1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the CTRL1 register."]
   #[inline] pub fn ctrl1(&self) -> Ctrl1 { 
      unsafe {
         Ctrl1(::core::ptr::read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the CTRL1 register."]
   #[inline] pub fn set_ctrl1<F: FnOnce(Ctrl1) -> Ctrl1>(&self, f: F) -> &Self {
      let value = f(Ctrl1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTRL1 register."]
   #[inline] pub fn with_ctrl1<F: FnOnce(Ctrl1) -> Ctrl1>(&self, f: F) -> &Self {
      let tmp = self.ctrl1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TIMER register."]
   #[inline] pub fn timer_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the TIMER register."]
   #[inline] pub fn timer_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the TIMER register."]
   #[inline] pub fn timer(&self) -> Timer { 
      unsafe {
         Timer(::core::ptr::read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the TIMER register."]
   #[inline] pub fn set_timer<F: FnOnce(Timer) -> Timer>(&self, f: F) -> &Self {
      let value = f(Timer(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TIMER register."]
   #[inline] pub fn with_timer<F: FnOnce(Timer) -> Timer>(&self, f: F) -> &Self {
      let tmp = self.timer();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RXMGMASK register."]
   #[inline] pub fn rxmgmask_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x10) as *const u32
   }
#[doc="Get the *mut pointer for the RXMGMASK register."]
   #[inline] pub fn rxmgmask_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x10) as *mut u32
   }
#[doc="Read the RXMGMASK register."]
   #[inline] pub fn rxmgmask(&self) -> Rxmgmask { 
      unsafe {
         Rxmgmask(::core::ptr::read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the RXMGMASK register."]
   #[inline] pub fn set_rxmgmask<F: FnOnce(Rxmgmask) -> Rxmgmask>(&self, f: F) -> &Self {
      let value = f(Rxmgmask(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RXMGMASK register."]
   #[inline] pub fn with_rxmgmask<F: FnOnce(Rxmgmask) -> Rxmgmask>(&self, f: F) -> &Self {
      let tmp = self.rxmgmask();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RX14MASK register."]
   #[inline] pub fn rx14mask_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x14) as *const u32
   }
#[doc="Get the *mut pointer for the RX14MASK register."]
   #[inline] pub fn rx14mask_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x14) as *mut u32
   }
#[doc="Read the RX14MASK register."]
   #[inline] pub fn rx14mask(&self) -> Rx14mask { 
      unsafe {
         Rx14mask(::core::ptr::read_volatile((self.0 + 0x14) as *const u32))
      }
   }
#[doc="Write the RX14MASK register."]
   #[inline] pub fn set_rx14mask<F: FnOnce(Rx14mask) -> Rx14mask>(&self, f: F) -> &Self {
      let value = f(Rx14mask(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RX14MASK register."]
   #[inline] pub fn with_rx14mask<F: FnOnce(Rx14mask) -> Rx14mask>(&self, f: F) -> &Self {
      let tmp = self.rx14mask();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RX15MASK register."]
   #[inline] pub fn rx15mask_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x18) as *const u32
   }
#[doc="Get the *mut pointer for the RX15MASK register."]
   #[inline] pub fn rx15mask_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x18) as *mut u32
   }
#[doc="Read the RX15MASK register."]
   #[inline] pub fn rx15mask(&self) -> Rx15mask { 
      unsafe {
         Rx15mask(::core::ptr::read_volatile((self.0 + 0x18) as *const u32))
      }
   }
#[doc="Write the RX15MASK register."]
   #[inline] pub fn set_rx15mask<F: FnOnce(Rx15mask) -> Rx15mask>(&self, f: F) -> &Self {
      let value = f(Rx15mask(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RX15MASK register."]
   #[inline] pub fn with_rx15mask<F: FnOnce(Rx15mask) -> Rx15mask>(&self, f: F) -> &Self {
      let tmp = self.rx15mask();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ECR register."]
   #[inline] pub fn ecr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x1c) as *const u32
   }
#[doc="Get the *mut pointer for the ECR register."]
   #[inline] pub fn ecr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x1c) as *mut u32
   }
#[doc="Read the ECR register."]
   #[inline] pub fn ecr(&self) -> Ecr { 
      unsafe {
         Ecr(::core::ptr::read_volatile((self.0 + 0x1c) as *const u32))
      }
   }
#[doc="Write the ECR register."]
   #[inline] pub fn set_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
      let value = f(Ecr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ECR register."]
   #[inline] pub fn with_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
      let tmp = self.ecr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ESR1 register."]
   #[inline] pub fn esr1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x20) as *const u32
   }
#[doc="Get the *mut pointer for the ESR1 register."]
   #[inline] pub fn esr1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x20) as *mut u32
   }
#[doc="Read the ESR1 register."]
   #[inline] pub fn esr1(&self) -> Esr1 { 
      unsafe {
         Esr1(::core::ptr::read_volatile((self.0 + 0x20) as *const u32))
      }
   }
#[doc="Write the ESR1 register."]
   #[inline] pub fn set_esr1<F: FnOnce(Esr1) -> Esr1>(&self, f: F) -> &Self {
      let value = f(Esr1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ESR1 register."]
   #[inline] pub fn with_esr1<F: FnOnce(Esr1) -> Esr1>(&self, f: F) -> &Self {
      let tmp = self.esr1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IMASK1 register."]
   #[inline] pub fn imask1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x28) as *const u32
   }
#[doc="Get the *mut pointer for the IMASK1 register."]
   #[inline] pub fn imask1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x28) as *mut u32
   }
#[doc="Read the IMASK1 register."]
   #[inline] pub fn imask1(&self) -> Imask1 { 
      unsafe {
         Imask1(::core::ptr::read_volatile((self.0 + 0x28) as *const u32))
      }
   }
#[doc="Write the IMASK1 register."]
   #[inline] pub fn set_imask1<F: FnOnce(Imask1) -> Imask1>(&self, f: F) -> &Self {
      let value = f(Imask1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IMASK1 register."]
   #[inline] pub fn with_imask1<F: FnOnce(Imask1) -> Imask1>(&self, f: F) -> &Self {
      let tmp = self.imask1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IFLAG1 register."]
   #[inline] pub fn iflag1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x30) as *const u32
   }
#[doc="Get the *mut pointer for the IFLAG1 register."]
   #[inline] pub fn iflag1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x30) as *mut u32
   }
#[doc="Read the IFLAG1 register."]
   #[inline] pub fn iflag1(&self) -> Iflag1 { 
      unsafe {
         Iflag1(::core::ptr::read_volatile((self.0 + 0x30) as *const u32))
      }
   }
#[doc="Write the IFLAG1 register."]
   #[inline] pub fn set_iflag1<F: FnOnce(Iflag1) -> Iflag1>(&self, f: F) -> &Self {
      let value = f(Iflag1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IFLAG1 register."]
   #[inline] pub fn with_iflag1<F: FnOnce(Iflag1) -> Iflag1>(&self, f: F) -> &Self {
      let tmp = self.iflag1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL2 register."]
   #[inline] pub fn ctrl2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x34) as *const u32
   }
#[doc="Get the *mut pointer for the CTRL2 register."]
   #[inline] pub fn ctrl2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x34) as *mut u32
   }
#[doc="Read the CTRL2 register."]
   #[inline] pub fn ctrl2(&self) -> Ctrl2 { 
      unsafe {
         Ctrl2(::core::ptr::read_volatile((self.0 + 0x34) as *const u32))
      }
   }
#[doc="Write the CTRL2 register."]
   #[inline] pub fn set_ctrl2<F: FnOnce(Ctrl2) -> Ctrl2>(&self, f: F) -> &Self {
      let value = f(Ctrl2(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTRL2 register."]
   #[inline] pub fn with_ctrl2<F: FnOnce(Ctrl2) -> Ctrl2>(&self, f: F) -> &Self {
      let tmp = self.ctrl2();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ESR2 register."]
   #[inline] pub fn esr2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x38) as *const u32
   }
#[doc="Get the *mut pointer for the ESR2 register."]
   #[inline] pub fn esr2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x38) as *mut u32
   }
#[doc="Read the ESR2 register."]
   #[inline] pub fn esr2(&self) -> Esr2 { 
      unsafe {
         Esr2(::core::ptr::read_volatile((self.0 + 0x38) as *const u32))
      }
   }

#[doc="Get the *const pointer for the CRCR register."]
   #[inline] pub fn crcr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x44) as *const u32
   }
#[doc="Get the *mut pointer for the CRCR register."]
   #[inline] pub fn crcr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x44) as *mut u32
   }
#[doc="Read the CRCR register."]
   #[inline] pub fn crcr(&self) -> Crcr { 
      unsafe {
         Crcr(::core::ptr::read_volatile((self.0 + 0x44) as *const u32))
      }
   }

#[doc="Get the *const pointer for the RXFGMASK register."]
   #[inline] pub fn rxfgmask_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x48) as *const u32
   }
#[doc="Get the *mut pointer for the RXFGMASK register."]
   #[inline] pub fn rxfgmask_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x48) as *mut u32
   }
#[doc="Read the RXFGMASK register."]
   #[inline] pub fn rxfgmask(&self) -> Rxfgmask { 
      unsafe {
         Rxfgmask(::core::ptr::read_volatile((self.0 + 0x48) as *const u32))
      }
   }
#[doc="Write the RXFGMASK register."]
   #[inline] pub fn set_rxfgmask<F: FnOnce(Rxfgmask) -> Rxfgmask>(&self, f: F) -> &Self {
      let value = f(Rxfgmask(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RXFGMASK register."]
   #[inline] pub fn with_rxfgmask<F: FnOnce(Rxfgmask) -> Rxfgmask>(&self, f: F) -> &Self {
      let tmp = self.rxfgmask();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RXFIR register."]
   #[inline] pub fn rxfir_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4c) as *const u32
   }
#[doc="Get the *mut pointer for the RXFIR register."]
   #[inline] pub fn rxfir_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4c) as *mut u32
   }
#[doc="Read the RXFIR register."]
   #[inline] pub fn rxfir(&self) -> Rxfir { 
      unsafe {
         Rxfir(::core::ptr::read_volatile((self.0 + 0x4c) as *const u32))
      }
   }

#[doc="Get the *const pointer for the CBT register."]
   #[inline] pub fn cbt_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x50) as *const u32
   }
#[doc="Get the *mut pointer for the CBT register."]
   #[inline] pub fn cbt_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x50) as *mut u32
   }
#[doc="Read the CBT register."]
   #[inline] pub fn cbt(&self) -> Cbt { 
      unsafe {
         Cbt(::core::ptr::read_volatile((self.0 + 0x50) as *const u32))
      }
   }
#[doc="Write the CBT register."]
   #[inline] pub fn set_cbt<F: FnOnce(Cbt) -> Cbt>(&self, f: F) -> &Self {
      let value = f(Cbt(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x50) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CBT register."]
   #[inline] pub fn with_cbt<F: FnOnce(Cbt) -> Cbt>(&self, f: F) -> &Self {
      let tmp = self.cbt();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x50) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RAM register."]
   #[inline] pub fn ram_ptr<I: Into<bits::U7>>(&self, index: I) -> *const u32 { 
      let index: bits::U7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x80 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the RAM register."]
   #[inline] pub fn ram_mut<I: Into<bits::U7>>(&self, index: I) -> *mut u32 { 
      let index: bits::U7 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x80 + (index << 2)) as *mut u32
   }
#[doc="Read the RAM register."]
   #[inline] pub fn ram<I: Into<bits::U7>>(&self, index: I) -> Ram { 
      let index: bits::U7 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Ram(::core::ptr::read_volatile((self.0 + 0x80 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the RAM register."]
   #[inline] pub fn set_ram<I: Into<bits::U7>, F: FnOnce(Ram) -> Ram>(&self, index: I, f: F) -> &Self {
      let index: bits::U7 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Ram(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x80 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RAM register."]
   #[inline] pub fn with_ram<I: Into<bits::U7> + Copy, F: FnOnce(Ram) -> Ram>(&self, index: I, f: F) -> &Self {
      let index: bits::U7 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.ram(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x80 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MB8H0 register."]
   #[inline] pub fn mb8h0_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x80 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the MB8H0 register."]
   #[inline] pub fn mb8h0_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x80 + (index << 4)) as *mut u32
   }
#[doc="Read the MB8H0 register."]
   #[inline] pub fn mb8h0<I: Into<bits::R16>>(&self, index: I) -> Mb8h0 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Mb8h0(::core::ptr::read_volatile((self.0 + 0x80 + (index << 4)) as *const u32))
      }
   }
#[doc="Write the MB8H0 register."]
   #[inline] pub fn set_mb8h0<I: Into<bits::R16>, F: FnOnce(Mb8h0) -> Mb8h0>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Mb8h0(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x80 + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MB8H0 register."]
   #[inline] pub fn with_mb8h0<I: Into<bits::R16> + Copy, F: FnOnce(Mb8h0) -> Mb8h0>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.mb8h0(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x80 + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MB8H1 register."]
   #[inline] pub fn mb8h1_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x84 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the MB8H1 register."]
   #[inline] pub fn mb8h1_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x84 + (index << 4)) as *mut u32
   }
#[doc="Read the MB8H1 register."]
   #[inline] pub fn mb8h1<I: Into<bits::R16>>(&self, index: I) -> Mb8h1 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Mb8h1(::core::ptr::read_volatile((self.0 + 0x84 + (index << 4)) as *const u32))
      }
   }
#[doc="Write the MB8H1 register."]
   #[inline] pub fn set_mb8h1<I: Into<bits::R16>, F: FnOnce(Mb8h1) -> Mb8h1>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Mb8h1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x84 + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MB8H1 register."]
   #[inline] pub fn with_mb8h1<I: Into<bits::R16> + Copy, F: FnOnce(Mb8h1) -> Mb8h1>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.mb8h1(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x84 + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MB8D0 register."]
   #[inline] pub fn mb8d0_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x88 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the MB8D0 register."]
   #[inline] pub fn mb8d0_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x88 + (index << 4)) as *mut u32
   }
#[doc="Read the MB8D0 register."]
   #[inline] pub fn mb8d0<I: Into<bits::R16>>(&self, index: I) -> Mb8d0 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Mb8d0(::core::ptr::read_volatile((self.0 + 0x88 + (index << 4)) as *const u32))
      }
   }
#[doc="Write the MB8D0 register."]
   #[inline] pub fn set_mb8d0<I: Into<bits::R16>, F: FnOnce(Mb8d0) -> Mb8d0>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Mb8d0(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x88 + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MB8D0 register."]
   #[inline] pub fn with_mb8d0<I: Into<bits::R16> + Copy, F: FnOnce(Mb8d0) -> Mb8d0>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.mb8d0(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x88 + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MB8D1 register."]
   #[inline] pub fn mb8d1_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x8c + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the MB8D1 register."]
   #[inline] pub fn mb8d1_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x8c + (index << 4)) as *mut u32
   }
#[doc="Read the MB8D1 register."]
   #[inline] pub fn mb8d1<I: Into<bits::R16>>(&self, index: I) -> Mb8d1 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Mb8d1(::core::ptr::read_volatile((self.0 + 0x8c + (index << 4)) as *const u32))
      }
   }
#[doc="Write the MB8D1 register."]
   #[inline] pub fn set_mb8d1<I: Into<bits::R16>, F: FnOnce(Mb8d1) -> Mb8d1>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Mb8d1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8c + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MB8D1 register."]
   #[inline] pub fn with_mb8d1<I: Into<bits::R16> + Copy, F: FnOnce(Mb8d1) -> Mb8d1>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.mb8d1(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8c + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RXIMR register."]
   #[inline] pub fn rximr_ptr<I: Into<bits::R16>>(&self, index: I) -> *const u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x880 + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the RXIMR register."]
   #[inline] pub fn rximr_mut<I: Into<bits::R16>>(&self, index: I) -> *mut u32 { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x880 + (index << 2)) as *mut u32
   }
#[doc="Read the RXIMR register."]
   #[inline] pub fn rximr<I: Into<bits::R16>>(&self, index: I) -> Rximr { 
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Rximr(::core::ptr::read_volatile((self.0 + 0x880 + (index << 2)) as *const u32))
      }
   }
#[doc="Write the RXIMR register."]
   #[inline] pub fn set_rximr<I: Into<bits::R16>, F: FnOnce(Rximr) -> Rximr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Rximr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x880 + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RXIMR register."]
   #[inline] pub fn with_rximr<I: Into<bits::R16> + Copy, F: FnOnce(Rximr) -> Rximr>(&self, index: I, f: F) -> &Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.rximr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x880 + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL1_PN register."]
   #[inline] pub fn ctrl1_pn_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb00) as *const u32
   }
#[doc="Get the *mut pointer for the CTRL1_PN register."]
   #[inline] pub fn ctrl1_pn_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb00) as *mut u32
   }
#[doc="Read the CTRL1_PN register."]
   #[inline] pub fn ctrl1_pn(&self) -> Ctrl1Pn { 
      unsafe {
         Ctrl1Pn(::core::ptr::read_volatile((self.0 + 0xb00) as *const u32))
      }
   }
#[doc="Write the CTRL1_PN register."]
   #[inline] pub fn set_ctrl1_pn<F: FnOnce(Ctrl1Pn) -> Ctrl1Pn>(&self, f: F) -> &Self {
      let value = f(Ctrl1Pn(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb00) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTRL1_PN register."]
   #[inline] pub fn with_ctrl1_pn<F: FnOnce(Ctrl1Pn) -> Ctrl1Pn>(&self, f: F) -> &Self {
      let tmp = self.ctrl1_pn();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb00) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTRL2_PN register."]
   #[inline] pub fn ctrl2_pn_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb04) as *const u32
   }
#[doc="Get the *mut pointer for the CTRL2_PN register."]
   #[inline] pub fn ctrl2_pn_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb04) as *mut u32
   }
#[doc="Read the CTRL2_PN register."]
   #[inline] pub fn ctrl2_pn(&self) -> Ctrl2Pn { 
      unsafe {
         Ctrl2Pn(::core::ptr::read_volatile((self.0 + 0xb04) as *const u32))
      }
   }
#[doc="Write the CTRL2_PN register."]
   #[inline] pub fn set_ctrl2_pn<F: FnOnce(Ctrl2Pn) -> Ctrl2Pn>(&self, f: F) -> &Self {
      let value = f(Ctrl2Pn(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb04) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTRL2_PN register."]
   #[inline] pub fn with_ctrl2_pn<F: FnOnce(Ctrl2Pn) -> Ctrl2Pn>(&self, f: F) -> &Self {
      let tmp = self.ctrl2_pn();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb04) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WU_MTC register."]
   #[inline] pub fn wu_mtc_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb08) as *const u32
   }
#[doc="Get the *mut pointer for the WU_MTC register."]
   #[inline] pub fn wu_mtc_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb08) as *mut u32
   }
#[doc="Read the WU_MTC register."]
   #[inline] pub fn wu_mtc(&self) -> WuMtc { 
      unsafe {
         WuMtc(::core::ptr::read_volatile((self.0 + 0xb08) as *const u32))
      }
   }
#[doc="Write the WU_MTC register."]
   #[inline] pub fn set_wu_mtc<F: FnOnce(WuMtc) -> WuMtc>(&self, f: F) -> &Self {
      let value = f(WuMtc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb08) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the WU_MTC register."]
   #[inline] pub fn with_wu_mtc<F: FnOnce(WuMtc) -> WuMtc>(&self, f: F) -> &Self {
      let tmp = self.wu_mtc();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb08) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FLT_ID1 register."]
   #[inline] pub fn flt_id1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb0c) as *const u32
   }
#[doc="Get the *mut pointer for the FLT_ID1 register."]
   #[inline] pub fn flt_id1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb0c) as *mut u32
   }
#[doc="Read the FLT_ID1 register."]
   #[inline] pub fn flt_id1(&self) -> FltId1 { 
      unsafe {
         FltId1(::core::ptr::read_volatile((self.0 + 0xb0c) as *const u32))
      }
   }
#[doc="Write the FLT_ID1 register."]
   #[inline] pub fn set_flt_id1<F: FnOnce(FltId1) -> FltId1>(&self, f: F) -> &Self {
      let value = f(FltId1(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb0c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLT_ID1 register."]
   #[inline] pub fn with_flt_id1<F: FnOnce(FltId1) -> FltId1>(&self, f: F) -> &Self {
      let tmp = self.flt_id1();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb0c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FLT_DLC register."]
   #[inline] pub fn flt_dlc_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb10) as *const u32
   }
#[doc="Get the *mut pointer for the FLT_DLC register."]
   #[inline] pub fn flt_dlc_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb10) as *mut u32
   }
#[doc="Read the FLT_DLC register."]
   #[inline] pub fn flt_dlc(&self) -> FltDlc { 
      unsafe {
         FltDlc(::core::ptr::read_volatile((self.0 + 0xb10) as *const u32))
      }
   }
#[doc="Write the FLT_DLC register."]
   #[inline] pub fn set_flt_dlc<F: FnOnce(FltDlc) -> FltDlc>(&self, f: F) -> &Self {
      let value = f(FltDlc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLT_DLC register."]
   #[inline] pub fn with_flt_dlc<F: FnOnce(FltDlc) -> FltDlc>(&self, f: F) -> &Self {
      let tmp = self.flt_dlc();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PL1_LO register."]
   #[inline] pub fn pl1_lo_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb14) as *const u32
   }
#[doc="Get the *mut pointer for the PL1_LO register."]
   #[inline] pub fn pl1_lo_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb14) as *mut u32
   }
#[doc="Read the PL1_LO register."]
   #[inline] pub fn pl1_lo(&self) -> Pl1Lo { 
      unsafe {
         Pl1Lo(::core::ptr::read_volatile((self.0 + 0xb14) as *const u32))
      }
   }
#[doc="Write the PL1_LO register."]
   #[inline] pub fn set_pl1_lo<F: FnOnce(Pl1Lo) -> Pl1Lo>(&self, f: F) -> &Self {
      let value = f(Pl1Lo(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PL1_LO register."]
   #[inline] pub fn with_pl1_lo<F: FnOnce(Pl1Lo) -> Pl1Lo>(&self, f: F) -> &Self {
      let tmp = self.pl1_lo();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PL1_HI register."]
   #[inline] pub fn pl1_hi_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb18) as *const u32
   }
#[doc="Get the *mut pointer for the PL1_HI register."]
   #[inline] pub fn pl1_hi_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb18) as *mut u32
   }
#[doc="Read the PL1_HI register."]
   #[inline] pub fn pl1_hi(&self) -> Pl1Hi { 
      unsafe {
         Pl1Hi(::core::ptr::read_volatile((self.0 + 0xb18) as *const u32))
      }
   }
#[doc="Write the PL1_HI register."]
   #[inline] pub fn set_pl1_hi<F: FnOnce(Pl1Hi) -> Pl1Hi>(&self, f: F) -> &Self {
      let value = f(Pl1Hi(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PL1_HI register."]
   #[inline] pub fn with_pl1_hi<F: FnOnce(Pl1Hi) -> Pl1Hi>(&self, f: F) -> &Self {
      let tmp = self.pl1_hi();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FLT_ID2_IDMASK register."]
   #[inline] pub fn flt_id2_idmask_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb1c) as *const u32
   }
#[doc="Get the *mut pointer for the FLT_ID2_IDMASK register."]
   #[inline] pub fn flt_id2_idmask_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb1c) as *mut u32
   }
#[doc="Read the FLT_ID2_IDMASK register."]
   #[inline] pub fn flt_id2_idmask(&self) -> FltId2Idmask { 
      unsafe {
         FltId2Idmask(::core::ptr::read_volatile((self.0 + 0xb1c) as *const u32))
      }
   }
#[doc="Write the FLT_ID2_IDMASK register."]
   #[inline] pub fn set_flt_id2_idmask<F: FnOnce(FltId2Idmask) -> FltId2Idmask>(&self, f: F) -> &Self {
      let value = f(FltId2Idmask(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLT_ID2_IDMASK register."]
   #[inline] pub fn with_flt_id2_idmask<F: FnOnce(FltId2Idmask) -> FltId2Idmask>(&self, f: F) -> &Self {
      let tmp = self.flt_id2_idmask();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb1c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PL2_PLMASK_LO register."]
   #[inline] pub fn pl2_plmask_lo_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb20) as *const u32
   }
#[doc="Get the *mut pointer for the PL2_PLMASK_LO register."]
   #[inline] pub fn pl2_plmask_lo_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb20) as *mut u32
   }
#[doc="Read the PL2_PLMASK_LO register."]
   #[inline] pub fn pl2_plmask_lo(&self) -> Pl2PlmaskLo { 
      unsafe {
         Pl2PlmaskLo(::core::ptr::read_volatile((self.0 + 0xb20) as *const u32))
      }
   }
#[doc="Write the PL2_PLMASK_LO register."]
   #[inline] pub fn set_pl2_plmask_lo<F: FnOnce(Pl2PlmaskLo) -> Pl2PlmaskLo>(&self, f: F) -> &Self {
      let value = f(Pl2PlmaskLo(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PL2_PLMASK_LO register."]
   #[inline] pub fn with_pl2_plmask_lo<F: FnOnce(Pl2PlmaskLo) -> Pl2PlmaskLo>(&self, f: F) -> &Self {
      let tmp = self.pl2_plmask_lo();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PL2_PLMASK_HI register."]
   #[inline] pub fn pl2_plmask_hi_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb24) as *const u32
   }
#[doc="Get the *mut pointer for the PL2_PLMASK_HI register."]
   #[inline] pub fn pl2_plmask_hi_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb24) as *mut u32
   }
#[doc="Read the PL2_PLMASK_HI register."]
   #[inline] pub fn pl2_plmask_hi(&self) -> Pl2PlmaskHi { 
      unsafe {
         Pl2PlmaskHi(::core::ptr::read_volatile((self.0 + 0xb24) as *const u32))
      }
   }
#[doc="Write the PL2_PLMASK_HI register."]
   #[inline] pub fn set_pl2_plmask_hi<F: FnOnce(Pl2PlmaskHi) -> Pl2PlmaskHi>(&self, f: F) -> &Self {
      let value = f(Pl2PlmaskHi(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PL2_PLMASK_HI register."]
   #[inline] pub fn with_pl2_plmask_hi<F: FnOnce(Pl2PlmaskHi) -> Pl2PlmaskHi>(&self, f: F) -> &Self {
      let tmp = self.pl2_plmask_hi();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xb24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WMB0_CS register."]
   #[inline] pub fn wmb0_cs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb40) as *const u32
   }
#[doc="Get the *mut pointer for the WMB0_CS register."]
   #[inline] pub fn wmb0_cs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb40) as *mut u32
   }
#[doc="Read the WMB0_CS register."]
   #[inline] pub fn wmb0_cs(&self) -> Wmb0Cs { 
      unsafe {
         Wmb0Cs(::core::ptr::read_volatile((self.0 + 0xb40) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB0_ID register."]
   #[inline] pub fn wmb0_id_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb44) as *const u32
   }
#[doc="Get the *mut pointer for the WMB0_ID register."]
   #[inline] pub fn wmb0_id_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb44) as *mut u32
   }
#[doc="Read the WMB0_ID register."]
   #[inline] pub fn wmb0_id(&self) -> Wmb0Id { 
      unsafe {
         Wmb0Id(::core::ptr::read_volatile((self.0 + 0xb44) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB0_D03 register."]
   #[inline] pub fn wmb0_d03_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb48) as *const u32
   }
#[doc="Get the *mut pointer for the WMB0_D03 register."]
   #[inline] pub fn wmb0_d03_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb48) as *mut u32
   }
#[doc="Read the WMB0_D03 register."]
   #[inline] pub fn wmb0_d03(&self) -> Wmb0D03 { 
      unsafe {
         Wmb0D03(::core::ptr::read_volatile((self.0 + 0xb48) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB0_D47 register."]
   #[inline] pub fn wmb0_d47_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb4c) as *const u32
   }
#[doc="Get the *mut pointer for the WMB0_D47 register."]
   #[inline] pub fn wmb0_d47_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb4c) as *mut u32
   }
#[doc="Read the WMB0_D47 register."]
   #[inline] pub fn wmb0_d47(&self) -> Wmb0D47 { 
      unsafe {
         Wmb0D47(::core::ptr::read_volatile((self.0 + 0xb4c) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB1_CS register."]
   #[inline] pub fn wmb1_cs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb50) as *const u32
   }
#[doc="Get the *mut pointer for the WMB1_CS register."]
   #[inline] pub fn wmb1_cs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb50) as *mut u32
   }
#[doc="Read the WMB1_CS register."]
   #[inline] pub fn wmb1_cs(&self) -> Wmb1Cs { 
      unsafe {
         Wmb1Cs(::core::ptr::read_volatile((self.0 + 0xb50) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB1_ID register."]
   #[inline] pub fn wmb1_id_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb54) as *const u32
   }
#[doc="Get the *mut pointer for the WMB1_ID register."]
   #[inline] pub fn wmb1_id_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb54) as *mut u32
   }
#[doc="Read the WMB1_ID register."]
   #[inline] pub fn wmb1_id(&self) -> Wmb1Id { 
      unsafe {
         Wmb1Id(::core::ptr::read_volatile((self.0 + 0xb54) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB1_D03 register."]
   #[inline] pub fn wmb1_d03_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb58) as *const u32
   }
#[doc="Get the *mut pointer for the WMB1_D03 register."]
   #[inline] pub fn wmb1_d03_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb58) as *mut u32
   }
#[doc="Read the WMB1_D03 register."]
   #[inline] pub fn wmb1_d03(&self) -> Wmb1D03 { 
      unsafe {
         Wmb1D03(::core::ptr::read_volatile((self.0 + 0xb58) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB1_D47 register."]
   #[inline] pub fn wmb1_d47_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb5c) as *const u32
   }
#[doc="Get the *mut pointer for the WMB1_D47 register."]
   #[inline] pub fn wmb1_d47_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb5c) as *mut u32
   }
#[doc="Read the WMB1_D47 register."]
   #[inline] pub fn wmb1_d47(&self) -> Wmb1D47 { 
      unsafe {
         Wmb1D47(::core::ptr::read_volatile((self.0 + 0xb5c) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB2_CS register."]
   #[inline] pub fn wmb2_cs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb60) as *const u32
   }
#[doc="Get the *mut pointer for the WMB2_CS register."]
   #[inline] pub fn wmb2_cs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb60) as *mut u32
   }
#[doc="Read the WMB2_CS register."]
   #[inline] pub fn wmb2_cs(&self) -> Wmb2Cs { 
      unsafe {
         Wmb2Cs(::core::ptr::read_volatile((self.0 + 0xb60) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB2_ID register."]
   #[inline] pub fn wmb2_id_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb64) as *const u32
   }
#[doc="Get the *mut pointer for the WMB2_ID register."]
   #[inline] pub fn wmb2_id_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb64) as *mut u32
   }
#[doc="Read the WMB2_ID register."]
   #[inline] pub fn wmb2_id(&self) -> Wmb2Id { 
      unsafe {
         Wmb2Id(::core::ptr::read_volatile((self.0 + 0xb64) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB2_D03 register."]
   #[inline] pub fn wmb2_d03_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb68) as *const u32
   }
#[doc="Get the *mut pointer for the WMB2_D03 register."]
   #[inline] pub fn wmb2_d03_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb68) as *mut u32
   }
#[doc="Read the WMB2_D03 register."]
   #[inline] pub fn wmb2_d03(&self) -> Wmb2D03 { 
      unsafe {
         Wmb2D03(::core::ptr::read_volatile((self.0 + 0xb68) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB2_D47 register."]
   #[inline] pub fn wmb2_d47_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb6c) as *const u32
   }
#[doc="Get the *mut pointer for the WMB2_D47 register."]
   #[inline] pub fn wmb2_d47_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb6c) as *mut u32
   }
#[doc="Read the WMB2_D47 register."]
   #[inline] pub fn wmb2_d47(&self) -> Wmb2D47 { 
      unsafe {
         Wmb2D47(::core::ptr::read_volatile((self.0 + 0xb6c) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB3_CS register."]
   #[inline] pub fn wmb3_cs_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb70) as *const u32
   }
#[doc="Get the *mut pointer for the WMB3_CS register."]
   #[inline] pub fn wmb3_cs_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb70) as *mut u32
   }
#[doc="Read the WMB3_CS register."]
   #[inline] pub fn wmb3_cs(&self) -> Wmb3Cs { 
      unsafe {
         Wmb3Cs(::core::ptr::read_volatile((self.0 + 0xb70) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB3_ID register."]
   #[inline] pub fn wmb3_id_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb74) as *const u32
   }
#[doc="Get the *mut pointer for the WMB3_ID register."]
   #[inline] pub fn wmb3_id_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb74) as *mut u32
   }
#[doc="Read the WMB3_ID register."]
   #[inline] pub fn wmb3_id(&self) -> Wmb3Id { 
      unsafe {
         Wmb3Id(::core::ptr::read_volatile((self.0 + 0xb74) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB3_D03 register."]
   #[inline] pub fn wmb3_d03_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb78) as *const u32
   }
#[doc="Get the *mut pointer for the WMB3_D03 register."]
   #[inline] pub fn wmb3_d03_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb78) as *mut u32
   }
#[doc="Read the WMB3_D03 register."]
   #[inline] pub fn wmb3_d03(&self) -> Wmb3D03 { 
      unsafe {
         Wmb3D03(::core::ptr::read_volatile((self.0 + 0xb78) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WMB3_D47 register."]
   #[inline] pub fn wmb3_d47_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb7c) as *const u32
   }
#[doc="Get the *mut pointer for the WMB3_D47 register."]
   #[inline] pub fn wmb3_d47_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb7c) as *mut u32
   }
#[doc="Read the WMB3_D47 register."]
   #[inline] pub fn wmb3_d47(&self) -> Wmb3D47 { 
      unsafe {
         Wmb3D47(::core::ptr::read_volatile((self.0 + 0xb7c) as *const u32))
      }
   }

#[doc="Get the *const pointer for the FDCTRL register."]
   #[inline] pub fn fdctrl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc00) as *const u32
   }
#[doc="Get the *mut pointer for the FDCTRL register."]
   #[inline] pub fn fdctrl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc00) as *mut u32
   }
#[doc="Read the FDCTRL register."]
   #[inline] pub fn fdctrl(&self) -> Fdctrl { 
      unsafe {
         Fdctrl(::core::ptr::read_volatile((self.0 + 0xc00) as *const u32))
      }
   }
#[doc="Write the FDCTRL register."]
   #[inline] pub fn set_fdctrl<F: FnOnce(Fdctrl) -> Fdctrl>(&self, f: F) -> &Self {
      let value = f(Fdctrl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc00) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FDCTRL register."]
   #[inline] pub fn with_fdctrl<F: FnOnce(Fdctrl) -> Fdctrl>(&self, f: F) -> &Self {
      let tmp = self.fdctrl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc00) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FDCBT register."]
   #[inline] pub fn fdcbt_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc04) as *const u32
   }
#[doc="Get the *mut pointer for the FDCBT register."]
   #[inline] pub fn fdcbt_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc04) as *mut u32
   }
#[doc="Read the FDCBT register."]
   #[inline] pub fn fdcbt(&self) -> Fdcbt { 
      unsafe {
         Fdcbt(::core::ptr::read_volatile((self.0 + 0xc04) as *const u32))
      }
   }
#[doc="Write the FDCBT register."]
   #[inline] pub fn set_fdcbt<F: FnOnce(Fdcbt) -> Fdcbt>(&self, f: F) -> &Self {
      let value = f(Fdcbt(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc04) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FDCBT register."]
   #[inline] pub fn with_fdcbt<F: FnOnce(Fdcbt) -> Fdcbt>(&self, f: F) -> &Self {
      let tmp = self.fdcbt();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc04) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FDCRC register."]
   #[inline] pub fn fdcrc_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc08) as *const u32
   }
#[doc="Get the *mut pointer for the FDCRC register."]
   #[inline] pub fn fdcrc_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc08) as *mut u32
   }
#[doc="Read the FDCRC register."]
   #[inline] pub fn fdcrc(&self) -> Fdcrc { 
      unsafe {
         Fdcrc(::core::ptr::read_volatile((self.0 + 0xc08) as *const u32))
      }
   }

}

#[doc="Module Configuration Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
#[doc="Number Of The Last Message Buffer"]
   #[inline] pub fn maxmb(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Number Of The Last Message Buffer"]
   #[inline] pub fn set_maxmb<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ID Acceptance Mode"]
   #[inline] pub fn idam(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="ID Acceptance Mode"]
   #[inline] pub fn set_idam<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="CAN FD operation enable"]
   #[inline] pub fn fden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="CAN FD operation enable"]
   #[inline] pub fn set_fden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Abort Enable"]
   #[inline] pub fn aen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Abort Enable"]
   #[inline] pub fn set_aen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Local Priority Enable"]
   #[inline] pub fn lprioen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Local Priority Enable"]
   #[inline] pub fn set_lprioen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Pretended Networking Enable"]
   #[inline] pub fn pnet_en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Pretended Networking Enable"]
   #[inline] pub fn set_pnet_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="DMA Enable"]
   #[inline] pub fn dma(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="DMA Enable"]
   #[inline] pub fn set_dma<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Individual Rx Masking And Queue Enable"]
   #[inline] pub fn irmq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Individual Rx Masking And Queue Enable"]
   #[inline] pub fn set_irmq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Self Reception Disable"]
   #[inline] pub fn srxdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Self Reception Disable"]
   #[inline] pub fn set_srxdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Low-Power Mode Acknowledge"]
   #[inline] pub fn lpmack(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Low-Power Mode Acknowledge"]
   #[inline] pub fn set_lpmack<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Warning Interrupt Enable"]
   #[inline] pub fn wrnen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="Warning Interrupt Enable"]
   #[inline] pub fn set_wrnen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Supervisor Mode"]
   #[inline] pub fn supv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Supervisor Mode"]
   #[inline] pub fn set_supv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="Freeze Mode Acknowledge"]
   #[inline] pub fn frzack(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Freeze Mode Acknowledge"]
   #[inline] pub fn set_frzack<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Soft Reset"]
   #[inline] pub fn softrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Soft Reset"]
   #[inline] pub fn set_softrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="FlexCAN Not Ready"]
   #[inline] pub fn notrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="FlexCAN Not Ready"]
   #[inline] pub fn set_notrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="Halt FlexCAN"]
   #[inline] pub fn halt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Halt FlexCAN"]
   #[inline] pub fn set_halt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Rx FIFO Enable"]
   #[inline] pub fn rfen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="Rx FIFO Enable"]
   #[inline] pub fn set_rfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="Freeze Enable"]
   #[inline] pub fn frz(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Freeze Enable"]
   #[inline] pub fn set_frz<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Module Disable"]
   #[inline] pub fn mdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Module Disable"]
   #[inline] pub fn set_mdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maxmb() != 0 { try!(write!(f, " maxmb=0x{:x}", self.maxmb()))}
      if self.idam() != 0 { try!(write!(f, " idam=0x{:x}", self.idam()))}
      if self.fden() != 0 { try!(write!(f, " fden"))}
      if self.aen() != 0 { try!(write!(f, " aen"))}
      if self.lprioen() != 0 { try!(write!(f, " lprioen"))}
      if self.pnet_en() != 0 { try!(write!(f, " pnet_en"))}
      if self.dma() != 0 { try!(write!(f, " dma"))}
      if self.irmq() != 0 { try!(write!(f, " irmq"))}
      if self.srxdis() != 0 { try!(write!(f, " srxdis"))}
      if self.lpmack() != 0 { try!(write!(f, " lpmack"))}
      if self.wrnen() != 0 { try!(write!(f, " wrnen"))}
      if self.supv() != 0 { try!(write!(f, " supv"))}
      if self.frzack() != 0 { try!(write!(f, " frzack"))}
      if self.softrst() != 0 { try!(write!(f, " softrst"))}
      if self.notrdy() != 0 { try!(write!(f, " notrdy"))}
      if self.halt() != 0 { try!(write!(f, " halt"))}
      if self.rfen() != 0 { try!(write!(f, " rfen"))}
      if self.frz() != 0 { try!(write!(f, " frz"))}
      if self.mdis() != 0 { try!(write!(f, " mdis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control 1 register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
#[doc="Propagation Segment"]
   #[inline] pub fn propseg(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Propagation Segment"]
   #[inline] pub fn set_propseg<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Listen-Only Mode"]
   #[inline] pub fn lom(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Listen-Only Mode"]
   #[inline] pub fn set_lom<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Lowest Buffer Transmitted First"]
   #[inline] pub fn lbuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Lowest Buffer Transmitted First"]
   #[inline] pub fn set_lbuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Timer Sync"]
   #[inline] pub fn tsyn(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Timer Sync"]
   #[inline] pub fn set_tsyn<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Bus Off Recovery"]
   #[inline] pub fn boffrec(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Bus Off Recovery"]
   #[inline] pub fn set_boffrec<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="CAN Bit Sampling"]
   #[inline] pub fn smp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="CAN Bit Sampling"]
   #[inline] pub fn set_smp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Rx Warning Interrupt Mask"]
   #[inline] pub fn rwrnmsk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Rx Warning Interrupt Mask"]
   #[inline] pub fn set_rwrnmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Tx Warning Interrupt Mask"]
   #[inline] pub fn twrnmsk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Tx Warning Interrupt Mask"]
   #[inline] pub fn set_twrnmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Loop Back Mode"]
   #[inline] pub fn lpb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Loop Back Mode"]
   #[inline] pub fn set_lpb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="CAN Engine Clock Source"]
   #[inline] pub fn clksrc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="CAN Engine Clock Source"]
   #[inline] pub fn set_clksrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Error Interrupt Mask"]
   #[inline] pub fn errmsk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Error Interrupt Mask"]
   #[inline] pub fn set_errmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Bus Off Interrupt Mask"]
   #[inline] pub fn boffmsk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Bus Off Interrupt Mask"]
   #[inline] pub fn set_boffmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Phase Segment 2"]
   #[inline] pub fn pseg2(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
   }
#[doc="Phase Segment 2"]
   #[inline] pub fn set_pseg2<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Phase Segment 1"]
   #[inline] pub fn pseg1(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
   }
#[doc="Phase Segment 1"]
   #[inline] pub fn set_pseg1<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Resync Jump Width"]
   #[inline] pub fn rjw(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
   }
#[doc="Resync Jump Width"]
   #[inline] pub fn set_rjw<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Prescaler Division Factor"]
   #[inline] pub fn presdiv(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Prescaler Division Factor"]
   #[inline] pub fn set_presdiv<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Ctrl1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrl1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.propseg() != 0 { try!(write!(f, " propseg=0x{:x}", self.propseg()))}
      if self.lom() != 0 { try!(write!(f, " lom"))}
      if self.lbuf() != 0 { try!(write!(f, " lbuf"))}
      if self.tsyn() != 0 { try!(write!(f, " tsyn"))}
      if self.boffrec() != 0 { try!(write!(f, " boffrec"))}
      if self.smp() != 0 { try!(write!(f, " smp"))}
      if self.rwrnmsk() != 0 { try!(write!(f, " rwrnmsk"))}
      if self.twrnmsk() != 0 { try!(write!(f, " twrnmsk"))}
      if self.lpb() != 0 { try!(write!(f, " lpb"))}
      if self.clksrc() != 0 { try!(write!(f, " clksrc"))}
      if self.errmsk() != 0 { try!(write!(f, " errmsk"))}
      if self.boffmsk() != 0 { try!(write!(f, " boffmsk"))}
      if self.pseg2() != 0 { try!(write!(f, " pseg2=0x{:x}", self.pseg2()))}
      if self.pseg1() != 0 { try!(write!(f, " pseg1=0x{:x}", self.pseg1()))}
      if self.rjw() != 0 { try!(write!(f, " rjw=0x{:x}", self.rjw()))}
      if self.presdiv() != 0 { try!(write!(f, " presdiv=0x{:x}", self.presdiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Free Running Timer"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Timer(pub u32);
impl Timer {
#[doc="Timer Value"]
   #[inline] pub fn timer(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Timer Value"]
   #[inline] pub fn set_timer<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Timer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Timer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.timer() != 0 { try!(write!(f, " timer=0x{:x}", self.timer()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Rx Mailboxes Global Mask Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rxmgmask(pub u32);
impl Rxmgmask {
#[doc="Rx Mailboxes Global Mask Bits"]
   #[inline] pub fn mg(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Rx Mailboxes Global Mask Bits"]
   #[inline] pub fn set_mg<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rxmgmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rxmgmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Rx 14 Mask register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rx14mask(pub u32);
impl Rx14mask {
#[doc="Rx Buffer 14 Mask Bits"]
   #[inline] pub fn rx14m(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Rx Buffer 14 Mask Bits"]
   #[inline] pub fn set_rx14m<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rx14mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rx14mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Rx 15 Mask register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rx15mask(pub u32);
impl Rx15mask {
#[doc="Rx Buffer 15 Mask Bits"]
   #[inline] pub fn rx15m(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Rx Buffer 15 Mask Bits"]
   #[inline] pub fn set_rx15m<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rx15mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rx15mask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Error Counter"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ecr(pub u32);
impl Ecr {
#[doc="Transmit Error Counter"]
   #[inline] pub fn txerrcnt(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Transmit Error Counter"]
   #[inline] pub fn set_txerrcnt<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Receive Error Counter"]
   #[inline] pub fn rxerrcnt(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Receive Error Counter"]
   #[inline] pub fn set_rxerrcnt<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Transmit Error Counter for fast bits"]
   #[inline] pub fn txerrcnt_fast(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Transmit Error Counter for fast bits"]
   #[inline] pub fn set_txerrcnt_fast<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Receive Error Counter for fast bits"]
   #[inline] pub fn rxerrcnt_fast(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Receive Error Counter for fast bits"]
   #[inline] pub fn set_rxerrcnt_fast<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Ecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txerrcnt() != 0 { try!(write!(f, " txerrcnt=0x{:x}", self.txerrcnt()))}
      if self.rxerrcnt() != 0 { try!(write!(f, " rxerrcnt=0x{:x}", self.rxerrcnt()))}
      if self.txerrcnt_fast() != 0 { try!(write!(f, " txerrcnt_fast=0x{:x}", self.txerrcnt_fast()))}
      if self.rxerrcnt_fast() != 0 { try!(write!(f, " rxerrcnt_fast=0x{:x}", self.rxerrcnt_fast()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Error and Status 1 register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Esr1(pub u32);
impl Esr1 {
#[doc="Error Interrupt"]
   #[inline] pub fn errint(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Error Interrupt"]
   #[inline] pub fn set_errint<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Bus Off Interrupt"]
   #[inline] pub fn boffint(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Bus Off Interrupt"]
   #[inline] pub fn set_boffint<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="FlexCAN In Reception"]
   #[inline] pub fn rx(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="FlexCAN In Reception"]
   #[inline] pub fn set_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Fault Confinement State"]
   #[inline] pub fn fltconf(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Fault Confinement State"]
   #[inline] pub fn set_fltconf<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="FlexCAN In Transmission"]
   #[inline] pub fn tx(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="FlexCAN In Transmission"]
   #[inline] pub fn set_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="IDLE"]
   #[inline] pub fn idle(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="IDLE"]
   #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Rx Error Warning"]
   #[inline] pub fn rxwrn(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Rx Error Warning"]
   #[inline] pub fn set_rxwrn<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="TX Error Warning"]
   #[inline] pub fn txwrn(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="TX Error Warning"]
   #[inline] pub fn set_txwrn<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Stuffing Error"]
   #[inline] pub fn stferr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Stuffing Error"]
   #[inline] pub fn set_stferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Form Error"]
   #[inline] pub fn frmerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Form Error"]
   #[inline] pub fn set_frmerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Cyclic Redundancy Check Error"]
   #[inline] pub fn crcerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Cyclic Redundancy Check Error"]
   #[inline] pub fn set_crcerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Acknowledge Error"]
   #[inline] pub fn ackerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Acknowledge Error"]
   #[inline] pub fn set_ackerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Bit0 Error"]
   #[inline] pub fn bit0err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Bit0 Error"]
   #[inline] pub fn set_bit0err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Bit1 Error"]
   #[inline] pub fn bit1err(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Bit1 Error"]
   #[inline] pub fn set_bit1err<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Rx Warning Interrupt Flag"]
   #[inline] pub fn rwrnint(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Rx Warning Interrupt Flag"]
   #[inline] pub fn set_rwrnint<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Tx Warning Interrupt Flag"]
   #[inline] pub fn twrnint(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Tx Warning Interrupt Flag"]
   #[inline] pub fn set_twrnint<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="CAN Synchronization Status"]
   #[inline] pub fn synch(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="CAN Synchronization Status"]
   #[inline] pub fn set_synch<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="Bus Off Done Interrupt"]
   #[inline] pub fn boffdoneint(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Bus Off Done Interrupt"]
   #[inline] pub fn set_boffdoneint<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Error Interrupt for errors detected in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn errint_fast(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Error Interrupt for errors detected in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn set_errint_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Error Overrun bit"]
   #[inline] pub fn errovr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="Error Overrun bit"]
   #[inline] pub fn set_errovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn stferr_fast(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn set_stferr_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn frmerr_fast(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn set_frmerr_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
   #[inline] pub fn crcerr_fast(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
   #[inline] pub fn set_crcerr_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn bit0err_fast(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn set_bit0err_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn bit1err_fast(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
   #[inline] pub fn set_bit1err_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Esr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Esr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.errint() != 0 { try!(write!(f, " errint"))}
      if self.boffint() != 0 { try!(write!(f, " boffint"))}
      if self.rx() != 0 { try!(write!(f, " rx"))}
      if self.fltconf() != 0 { try!(write!(f, " fltconf=0x{:x}", self.fltconf()))}
      if self.tx() != 0 { try!(write!(f, " tx"))}
      if self.idle() != 0 { try!(write!(f, " idle"))}
      if self.rxwrn() != 0 { try!(write!(f, " rxwrn"))}
      if self.txwrn() != 0 { try!(write!(f, " txwrn"))}
      if self.stferr() != 0 { try!(write!(f, " stferr"))}
      if self.frmerr() != 0 { try!(write!(f, " frmerr"))}
      if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
      if self.ackerr() != 0 { try!(write!(f, " ackerr"))}
      if self.bit0err() != 0 { try!(write!(f, " bit0err"))}
      if self.bit1err() != 0 { try!(write!(f, " bit1err"))}
      if self.rwrnint() != 0 { try!(write!(f, " rwrnint"))}
      if self.twrnint() != 0 { try!(write!(f, " twrnint"))}
      if self.synch() != 0 { try!(write!(f, " synch"))}
      if self.boffdoneint() != 0 { try!(write!(f, " boffdoneint"))}
      if self.errint_fast() != 0 { try!(write!(f, " errint_fast"))}
      if self.errovr() != 0 { try!(write!(f, " errovr"))}
      if self.stferr_fast() != 0 { try!(write!(f, " stferr_fast"))}
      if self.frmerr_fast() != 0 { try!(write!(f, " frmerr_fast"))}
      if self.crcerr_fast() != 0 { try!(write!(f, " crcerr_fast"))}
      if self.bit0err_fast() != 0 { try!(write!(f, " bit0err_fast"))}
      if self.bit1err_fast() != 0 { try!(write!(f, " bit1err_fast"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Masks 1 register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Imask1(pub u32);
impl Imask1 {
#[doc="Buffer MB i Mask"]
   #[inline] pub fn buf31to0m(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Buffer MB i Mask"]
   #[inline] pub fn set_buf31to0m<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Imask1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Imask1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Flags 1 register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Iflag1(pub u32);
impl Iflag1 {
#[doc="Buffer MB Interrupt"]
   #[inline] pub fn bufi<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
      let index: bits::R32 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Buffer MB Interrupt"]
   #[inline] pub fn set_bufi<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R32 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Iflag1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Iflag1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bufi(0) != 0 { try!(write!(f, " bufi[0]"))}
      if self.bufi(1) != 0 { try!(write!(f, " bufi[1]"))}
      if self.bufi(2) != 0 { try!(write!(f, " bufi[2]"))}
      if self.bufi(3) != 0 { try!(write!(f, " bufi[3]"))}
      if self.bufi(4) != 0 { try!(write!(f, " bufi[4]"))}
      if self.bufi(5) != 0 { try!(write!(f, " bufi[5]"))}
      if self.bufi(6) != 0 { try!(write!(f, " bufi[6]"))}
      if self.bufi(7) != 0 { try!(write!(f, " bufi[7]"))}
      if self.bufi(8) != 0 { try!(write!(f, " bufi[8]"))}
      if self.bufi(9) != 0 { try!(write!(f, " bufi[9]"))}
      if self.bufi(10) != 0 { try!(write!(f, " bufi[10]"))}
      if self.bufi(11) != 0 { try!(write!(f, " bufi[11]"))}
      if self.bufi(12) != 0 { try!(write!(f, " bufi[12]"))}
      if self.bufi(13) != 0 { try!(write!(f, " bufi[13]"))}
      if self.bufi(14) != 0 { try!(write!(f, " bufi[14]"))}
      if self.bufi(15) != 0 { try!(write!(f, " bufi[15]"))}
      if self.bufi(16) != 0 { try!(write!(f, " bufi[16]"))}
      if self.bufi(17) != 0 { try!(write!(f, " bufi[17]"))}
      if self.bufi(18) != 0 { try!(write!(f, " bufi[18]"))}
      if self.bufi(19) != 0 { try!(write!(f, " bufi[19]"))}
      if self.bufi(20) != 0 { try!(write!(f, " bufi[20]"))}
      if self.bufi(21) != 0 { try!(write!(f, " bufi[21]"))}
      if self.bufi(22) != 0 { try!(write!(f, " bufi[22]"))}
      if self.bufi(23) != 0 { try!(write!(f, " bufi[23]"))}
      if self.bufi(24) != 0 { try!(write!(f, " bufi[24]"))}
      if self.bufi(25) != 0 { try!(write!(f, " bufi[25]"))}
      if self.bufi(26) != 0 { try!(write!(f, " bufi[26]"))}
      if self.bufi(27) != 0 { try!(write!(f, " bufi[27]"))}
      if self.bufi(28) != 0 { try!(write!(f, " bufi[28]"))}
      if self.bufi(29) != 0 { try!(write!(f, " bufi[29]"))}
      if self.bufi(30) != 0 { try!(write!(f, " bufi[30]"))}
      if self.bufi(31) != 0 { try!(write!(f, " bufi[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control 2 register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
#[doc="Edge Filter Disable"]
   #[inline] pub fn edfltdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Edge Filter Disable"]
   #[inline] pub fn set_edfltdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="ISO CAN FD Enable"]
   #[inline] pub fn isocanfden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="ISO CAN FD Enable"]
   #[inline] pub fn set_isocanfden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Protocol Exception Enable"]
   #[inline] pub fn prexcen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Protocol Exception Enable"]
   #[inline] pub fn set_prexcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Timer Source"]
   #[inline] pub fn timer_src(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Timer Source"]
   #[inline] pub fn set_timer_src<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
   #[inline] pub fn eacen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
   #[inline] pub fn set_eacen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Remote Request Storing"]
   #[inline] pub fn rrs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Remote Request Storing"]
   #[inline] pub fn set_rrs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Mailboxes Reception Priority"]
   #[inline] pub fn mrp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Mailboxes Reception Priority"]
   #[inline] pub fn set_mrp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="Tx Arbitration Start Delay"]
   #[inline] pub fn tasd(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
   }
#[doc="Tx Arbitration Start Delay"]
   #[inline] pub fn set_tasd<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Number Of Rx FIFO Filters"]
   #[inline] pub fn rffn(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }
#[doc="Number Of Rx FIFO Filters"]
   #[inline] pub fn set_rffn<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Bus Off Done Interrupt Mask"]
   #[inline] pub fn boffdonemsk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Bus Off Done Interrupt Mask"]
   #[inline] pub fn set_boffdonemsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Error Interrupt Mask for errors detected in the Data Phase of fast CAN FD frames"]
   #[inline] pub fn errmsk_fast(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Error Interrupt Mask for errors detected in the Data Phase of fast CAN FD frames"]
   #[inline] pub fn set_errmsk_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Ctrl2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrl2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.edfltdis() != 0 { try!(write!(f, " edfltdis"))}
      if self.isocanfden() != 0 { try!(write!(f, " isocanfden"))}
      if self.prexcen() != 0 { try!(write!(f, " prexcen"))}
      if self.timer_src() != 0 { try!(write!(f, " timer_src"))}
      if self.eacen() != 0 { try!(write!(f, " eacen"))}
      if self.rrs() != 0 { try!(write!(f, " rrs"))}
      if self.mrp() != 0 { try!(write!(f, " mrp"))}
      if self.tasd() != 0 { try!(write!(f, " tasd=0x{:x}", self.tasd()))}
      if self.rffn() != 0 { try!(write!(f, " rffn=0x{:x}", self.rffn()))}
      if self.boffdonemsk() != 0 { try!(write!(f, " boffdonemsk"))}
      if self.errmsk_fast() != 0 { try!(write!(f, " errmsk_fast"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Error and Status 2 register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Esr2(pub u32);
impl Esr2 {
#[doc="Inactive Mailbox"]
   #[inline] pub fn imb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Inactive Mailbox"]
   #[inline] pub fn set_imb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Valid Priority Status"]
   #[inline] pub fn vps(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Valid Priority Status"]
   #[inline] pub fn set_vps<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Lowest Priority Tx Mailbox"]
   #[inline] pub fn lptm(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
   }
#[doc="Lowest Priority Tx Mailbox"]
   #[inline] pub fn set_lptm<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Esr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Esr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.imb() != 0 { try!(write!(f, " imb"))}
      if self.vps() != 0 { try!(write!(f, " vps"))}
      if self.lptm() != 0 { try!(write!(f, " lptm=0x{:x}", self.lptm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Crcr(pub u32);
impl Crcr {
#[doc="Transmitted CRC value"]
   #[inline] pub fn txcrc(&self) -> bits::U15 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fff) as u16) } // [14:0]
   }
#[doc="Transmitted CRC value"]
   #[inline] pub fn set_txcrc<V: Into<bits::U15>>(mut self, value: V) -> Self {
      let value: bits::U15 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7fff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CRC Mailbox"]
   #[inline] pub fn mbcrc(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
   }
#[doc="CRC Mailbox"]
   #[inline] pub fn set_mbcrc<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Crcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txcrc() != 0 { try!(write!(f, " txcrc=0x{:x}", self.txcrc()))}
      if self.mbcrc() != 0 { try!(write!(f, " mbcrc=0x{:x}", self.mbcrc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Rx FIFO Global Mask register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rxfgmask(pub u32);
impl Rxfgmask {
#[doc="Rx FIFO Global Mask Bits"]
   #[inline] pub fn fgm(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Rx FIFO Global Mask Bits"]
   #[inline] pub fn set_fgm<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rxfgmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rxfgmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Rx FIFO Information Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rxfir(pub u32);
impl Rxfir {
#[doc="Identifier Acceptance Filter Hit Indicator"]
   #[inline] pub fn idhit(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }
#[doc="Identifier Acceptance Filter Hit Indicator"]
   #[inline] pub fn set_idhit<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1ff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rxfir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rxfir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.idhit() != 0 { try!(write!(f, " idhit=0x{:x}", self.idhit()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CAN Bit Timing Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cbt(pub u32);
impl Cbt {
#[doc="Extended Phase Segment 2"]
   #[inline] pub fn epseg2(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="Extended Phase Segment 2"]
   #[inline] pub fn set_epseg2<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Extended Phase Segment 1"]
   #[inline] pub fn epseg1(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1f) as u8) } // [9:5]
   }
#[doc="Extended Phase Segment 1"]
   #[inline] pub fn set_epseg1<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Extended Propagation Segment"]
   #[inline] pub fn epropseg(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3f) as u8) } // [15:10]
   }
#[doc="Extended Propagation Segment"]
   #[inline] pub fn set_epropseg<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Extended Resync Jump Width"]
   #[inline] pub fn erjw(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
   }
#[doc="Extended Resync Jump Width"]
   #[inline] pub fn set_erjw<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Extended Prescaler Division Factor"]
   #[inline] pub fn epresdiv(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3ff) as u16) } // [30:21]
   }
#[doc="Extended Prescaler Division Factor"]
   #[inline] pub fn set_epresdiv<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Bit Timing Format Enable"]
   #[inline] pub fn btf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Bit Timing Format Enable"]
   #[inline] pub fn set_btf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Cbt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cbt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.epseg2() != 0 { try!(write!(f, " epseg2=0x{:x}", self.epseg2()))}
      if self.epseg1() != 0 { try!(write!(f, " epseg1=0x{:x}", self.epseg1()))}
      if self.epropseg() != 0 { try!(write!(f, " epropseg=0x{:x}", self.epropseg()))}
      if self.erjw() != 0 { try!(write!(f, " erjw=0x{:x}", self.erjw()))}
      if self.epresdiv() != 0 { try!(write!(f, " epresdiv=0x{:x}", self.epresdiv()))}
      if self.btf() != 0 { try!(write!(f, " btf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Embedded RAM"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ram(pub u32);
impl Ram {
#[doc="Data byte 3 of Rx/Tx frame."]
   #[inline] pub fn byte<I: Into<bits::R4>>(&self, index: I) -> bits::U8 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 3);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xff) as u8) } // [7:0]
   }
#[doc="Data byte 3 of Rx/Tx frame."]
   #[inline] pub fn set_byte<I: Into<bits::R4>, V: Into<bits::U8>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + (index << 3);
      self.0 &= !(0xff << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Ram {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ram {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.byte(0) != 0 { try!(write!(f, " byte[0]=0x{:x}", self.byte(0)))}
      if self.byte(1) != 0 { try!(write!(f, " byte[1]=0x{:x}", self.byte(1)))}
      if self.byte(2) != 0 { try!(write!(f, " byte[2]=0x{:x}", self.byte(2)))}
      if self.byte(3) != 0 { try!(write!(f, " byte[3]=0x{:x}", self.byte(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Message Buffer Header Word 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mb8h0(pub u32);
impl Mb8h0 {
   #[inline] pub fn edl(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
   #[inline] pub fn set_edl<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

   #[inline] pub fn brs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
   #[inline] pub fn set_brs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

   #[inline] pub fn esi(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
   #[inline] pub fn set_esi<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

   #[inline] pub fn code(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }
   #[inline] pub fn set_code<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

   #[inline] pub fn srr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
   #[inline] pub fn set_srr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

   #[inline] pub fn ide(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
   #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

   #[inline] pub fn rtr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
   #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

   #[inline] pub fn dlc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
   #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

   #[inline] pub fn time_stamp(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
   #[inline] pub fn set_time_stamp<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Mb8h0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mb8h0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.edl() != 0 { try!(write!(f, " edl"))}
      if self.brs() != 0 { try!(write!(f, " brs"))}
      if self.esi() != 0 { try!(write!(f, " esi"))}
      if self.code() != 0 { try!(write!(f, " code=0x{:x}", self.code()))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.time_stamp() != 0 { try!(write!(f, " time_stamp=0x{:x}", self.time_stamp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Message Buffer Header Word 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mb8h1(pub u32);
impl Mb8h1 {
   #[inline] pub fn prio(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
   }
   #[inline] pub fn set_prio<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 29);
      self.0 |= value << 29;
      self
   }

   #[inline] pub fn id_std(&self) -> bits::U11 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7ff) as u16) } // [28:18]
   }
   #[inline] pub fn set_id_std<V: Into<bits::U11>>(mut self, value: V) -> Self {
      let value: bits::U11 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7ff << 18);
      self.0 |= value << 18;
      self
   }

   #[inline] pub fn id_ext(&self) -> bits::U29 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fffffff) as u32) } // [28:0]
   }
   #[inline] pub fn set_id_ext<V: Into<bits::U29>>(mut self, value: V) -> Self {
      let value: bits::U29 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Mb8h1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mb8h1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prio() != 0 { try!(write!(f, " prio=0x{:x}", self.prio()))}
      if self.id_std() != 0 { try!(write!(f, " id_std=0x{:x}", self.id_std()))}
      if self.id_ext() != 0 { try!(write!(f, " id_ext=0x{:x}", self.id_ext()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Message Buffer Data Word"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mb8d0(pub u32);
impl Mb8d0 {
   #[inline] pub fn byte<I: Into<bits::R4>>(&self, index: I) -> bits::U8 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xff) as u8) } // [7:0]
   }
   #[inline] pub fn set_byte<I: Into<bits::R4>, V: Into<bits::U8>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0xff << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Mb8d0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mb8d0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.byte(0) != 0 { try!(write!(f, " byte[0]=0x{:x}", self.byte(0)))}
      if self.byte(1) != 0 { try!(write!(f, " byte[1]=0x{:x}", self.byte(1)))}
      if self.byte(2) != 0 { try!(write!(f, " byte[2]=0x{:x}", self.byte(2)))}
      if self.byte(3) != 0 { try!(write!(f, " byte[3]=0x{:x}", self.byte(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Message Buffer Data Word"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mb8d1(pub u32);
impl Mb8d1 {
   #[inline] pub fn byte<I: Into<bits::R4>>(&self, index: I) -> bits::U8 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xff) as u8) } // [7:0]
   }
   #[inline] pub fn set_byte<I: Into<bits::R4>, V: Into<bits::U8>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0xff << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Mb8d1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mb8d1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.byte(0) != 0 { try!(write!(f, " byte[0]=0x{:x}", self.byte(0)))}
      if self.byte(1) != 0 { try!(write!(f, " byte[1]=0x{:x}", self.byte(1)))}
      if self.byte(2) != 0 { try!(write!(f, " byte[2]=0x{:x}", self.byte(2)))}
      if self.byte(3) != 0 { try!(write!(f, " byte[3]=0x{:x}", self.byte(3)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Rx Individual Mask Registers"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rximr(pub u32);
impl Rximr {
#[doc="Individual Mask Bits"]
   #[inline] pub fn mi(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Individual Mask Bits"]
   #[inline] pub fn set_mi<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rximr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rximr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking Control 1 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl1Pn(pub u32);
impl Ctrl1Pn {
#[doc="Filtering Combination Selection"]
   #[inline] pub fn fcs(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Filtering Combination Selection"]
   #[inline] pub fn set_fcs<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ID Filtering Selection"]
   #[inline] pub fn idfs(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="ID Filtering Selection"]
   #[inline] pub fn set_idfs<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Payload Filtering Selection"]
   #[inline] pub fn plfs(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Payload Filtering Selection"]
   #[inline] pub fn set_plfs<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Number of Messages Matching the Same Filtering Criteria"]
   #[inline] pub fn nmatch(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Number of Messages Matching the Same Filtering Criteria"]
   #[inline] pub fn set_nmatch<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Wake Up by Match Flag Mask Bit"]
   #[inline] pub fn wumf_msk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Wake Up by Match Flag Mask Bit"]
   #[inline] pub fn set_wumf_msk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Wake Up by Timeout Flag Mask Bit"]
   #[inline] pub fn wtof_msk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Wake Up by Timeout Flag Mask Bit"]
   #[inline] pub fn set_wtof_msk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

}
impl ::core::fmt::Display for Ctrl1Pn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrl1Pn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fcs() != 0 { try!(write!(f, " fcs=0x{:x}", self.fcs()))}
      if self.idfs() != 0 { try!(write!(f, " idfs=0x{:x}", self.idfs()))}
      if self.plfs() != 0 { try!(write!(f, " plfs=0x{:x}", self.plfs()))}
      if self.nmatch() != 0 { try!(write!(f, " nmatch=0x{:x}", self.nmatch()))}
      if self.wumf_msk() != 0 { try!(write!(f, " wumf_msk"))}
      if self.wtof_msk() != 0 { try!(write!(f, " wtof_msk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking Control 2 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctrl2Pn(pub u32);
impl Ctrl2Pn {
#[doc="Timeout for No Message Matching the Filtering Criteria"]
   #[inline] pub fn matchto(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Timeout for No Message Matching the Filtering Criteria"]
   #[inline] pub fn set_matchto<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ctrl2Pn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrl2Pn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.matchto() != 0 { try!(write!(f, " matchto=0x{:x}", self.matchto()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking Wake Up Match Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WuMtc(pub u32);
impl WuMtc {
#[doc="Number of Matches while in Pretended Networking"]
   #[inline] pub fn mcounter(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Number of Matches while in Pretended Networking"]
   #[inline] pub fn set_mcounter<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Wake Up by Match Flag Bit"]
   #[inline] pub fn wumf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Wake Up by Match Flag Bit"]
   #[inline] pub fn set_wumf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Wake Up by Timeout Flag Bit"]
   #[inline] pub fn wtof(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Wake Up by Timeout Flag Bit"]
   #[inline] pub fn set_wtof<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

}
impl ::core::fmt::Display for WuMtc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for WuMtc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mcounter() != 0 { try!(write!(f, " mcounter=0x{:x}", self.mcounter()))}
      if self.wumf() != 0 { try!(write!(f, " wumf"))}
      if self.wtof() != 0 { try!(write!(f, " wtof"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking ID Filter 1 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FltId1(pub u32);
impl FltId1 {
#[doc="ID Filter 1 for Pretended Networking filtering"]
   #[inline] pub fn flt_id1(&self) -> bits::U29 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fffffff) as u32) } // [28:0]
   }
#[doc="ID Filter 1 for Pretended Networking filtering"]
   #[inline] pub fn set_flt_id1<V: Into<bits::U29>>(mut self, value: V) -> Self {
      let value: bits::U29 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fffffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Remote Transmission Request Filter"]
   #[inline] pub fn flt_rtr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="Remote Transmission Request Filter"]
   #[inline] pub fn set_flt_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="ID Extended Filter"]
   #[inline] pub fn flt_ide(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="ID Extended Filter"]
   #[inline] pub fn set_flt_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

}
impl ::core::fmt::Display for FltId1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for FltId1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flt_id1() != 0 { try!(write!(f, " flt_id1=0x{:x}", self.flt_id1()))}
      if self.flt_rtr() != 0 { try!(write!(f, " flt_rtr"))}
      if self.flt_ide() != 0 { try!(write!(f, " flt_ide"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking DLC Filter Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FltDlc(pub u32);
impl FltDlc {
#[doc="Upper Limit for Length of Data Bytes Filter"]
   #[inline] pub fn flt_dlc_hi(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Upper Limit for Length of Data Bytes Filter"]
   #[inline] pub fn set_flt_dlc_hi<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Lower Limit for Length of Data Bytes Filter"]
   #[inline] pub fn flt_dlc_lo(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="Lower Limit for Length of Data Bytes Filter"]
   #[inline] pub fn set_flt_dlc_lo<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for FltDlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for FltDlc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flt_dlc_hi() != 0 { try!(write!(f, " flt_dlc_hi=0x{:x}", self.flt_dlc_hi()))}
      if self.flt_dlc_lo() != 0 { try!(write!(f, " flt_dlc_lo=0x{:x}", self.flt_dlc_lo()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking Payload Low Filter 1 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pl1Lo(pub u32);
impl Pl1Lo {
#[doc="Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 3."]
   #[inline] pub fn data_byte_3(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 3."]
   #[inline] pub fn set_data_byte_3<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 2."]
   #[inline] pub fn data_byte_2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 2."]
   #[inline] pub fn set_data_byte_2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 1."]
   #[inline] pub fn data_byte_1(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 1."]
   #[inline] pub fn set_data_byte_1<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 0."]
   #[inline] pub fn data_byte_0(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 0."]
   #[inline] pub fn set_data_byte_0<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Pl1Lo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pl1Lo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking Payload High Filter 1 Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pl1Hi(pub u32);
impl Pl1Hi {
#[doc="Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
   #[inline] pub fn data_byte_7(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
   #[inline] pub fn set_data_byte_7<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
   #[inline] pub fn data_byte_6(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
   #[inline] pub fn set_data_byte_6<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
   #[inline] pub fn data_byte_5(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
   #[inline] pub fn set_data_byte_5<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
   #[inline] pub fn data_byte_4(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
   #[inline] pub fn set_data_byte_4<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Pl1Hi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pl1Hi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking ID Filter 2 Register / ID Mask Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FltId2Idmask(pub u32);
impl FltId2Idmask {
#[doc="ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
   #[inline] pub fn flt_id2_idmask(&self) -> bits::U29 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fffffff) as u32) } // [28:0]
   }
#[doc="ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
   #[inline] pub fn set_flt_id2_idmask<V: Into<bits::U29>>(mut self, value: V) -> Self {
      let value: bits::U29 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fffffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Remote Transmission Request Mask Bit"]
   #[inline] pub fn rtr_msk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="Remote Transmission Request Mask Bit"]
   #[inline] pub fn set_rtr_msk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="ID Extended Mask Bit"]
   #[inline] pub fn ide_msk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="ID Extended Mask Bit"]
   #[inline] pub fn set_ide_msk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

}
impl ::core::fmt::Display for FltId2Idmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for FltId2Idmask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.flt_id2_idmask() != 0 { try!(write!(f, " flt_id2_idmask=0x{:x}", self.flt_id2_idmask()))}
      if self.rtr_msk() != 0 { try!(write!(f, " rtr_msk"))}
      if self.ide_msk() != 0 { try!(write!(f, " ide_msk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking Payload Low Filter 2 Register / Payload Low Mask Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pl2PlmaskLo(pub u32);
impl Pl2PlmaskLo {
#[doc="Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 3."]
   #[inline] pub fn data_byte_3(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 3."]
   #[inline] pub fn set_data_byte_3<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 2."]
   #[inline] pub fn data_byte_2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 2."]
   #[inline] pub fn set_data_byte_2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 1."]
   #[inline] pub fn data_byte_1(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 1."]
   #[inline] pub fn set_data_byte_1<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 0."]
   #[inline] pub fn data_byte_0(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 0."]
   #[inline] pub fn set_data_byte_0<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Pl2PlmaskLo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pl2PlmaskLo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pretended Networking Payload High Filter 2 low order bits / Payload High Mask Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pl2PlmaskHi(pub u32);
impl Pl2PlmaskHi {
#[doc="Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
   #[inline] pub fn data_byte_7(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
   #[inline] pub fn set_data_byte_7<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
   #[inline] pub fn data_byte_6(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
   #[inline] pub fn set_data_byte_6<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
   #[inline] pub fn data_byte_5(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
   #[inline] pub fn set_data_byte_5<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
   #[inline] pub fn data_byte_4(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
   #[inline] pub fn set_data_byte_4<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Pl2PlmaskHi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pl2PlmaskHi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for C/S"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb0Cs(pub u32);
impl Wmb0Cs {
#[doc="Length of Data in Bytes"]
   #[inline] pub fn dlc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="Length of Data in Bytes"]
   #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Remote Transmission Request Bit"]
   #[inline] pub fn rtr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Remote Transmission Request Bit"]
   #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="ID Extended Bit"]
   #[inline] pub fn ide(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="ID Extended Bit"]
   #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Substitute Remote Request"]
   #[inline] pub fn srr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Substitute Remote Request"]
   #[inline] pub fn set_srr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

}
impl ::core::fmt::Display for Wmb0Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb0Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for ID"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb0Id(pub u32);
impl Wmb0Id {
#[doc="Received ID under Pretended Networking mode"]
   #[inline] pub fn id(&self) -> bits::U29 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fffffff) as u32) } // [28:0]
   }
#[doc="Received ID under Pretended Networking mode"]
   #[inline] pub fn set_id<V: Into<bits::U29>>(mut self, value: V) -> Self {
      let value: bits::U29 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Wmb0Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb0Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for Data 0-3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb0D03(pub u32);
impl Wmb0D03 {
#[doc="Received payload corresponding to the data byte 3 under Pretended Networking mode"]
   #[inline] pub fn data_byte_3(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Received payload corresponding to the data byte 3 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_3<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Received payload corresponding to the data byte 2 under Pretended Networking mode"]
   #[inline] pub fn data_byte_2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Received payload corresponding to the data byte 2 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Received payload corresponding to the data byte 1 under Pretended Networking mode"]
   #[inline] pub fn data_byte_1(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Received payload corresponding to the data byte 1 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_1<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Received payload corresponding to the data byte 0 under Pretended Networking mode"]
   #[inline] pub fn data_byte_0(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Received payload corresponding to the data byte 0 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_0<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Wmb0D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb0D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register Data 4-7"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb0D47(pub u32);
impl Wmb0D47 {
#[doc="Received payload corresponding to the data byte 7 under Pretended Networking mode"]
   #[inline] pub fn data_byte_7(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Received payload corresponding to the data byte 7 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_7<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Received payload corresponding to the data byte 6 under Pretended Networking mode"]
   #[inline] pub fn data_byte_6(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Received payload corresponding to the data byte 6 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_6<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Received payload corresponding to the data byte 5 under Pretended Networking mode"]
   #[inline] pub fn data_byte_5(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Received payload corresponding to the data byte 5 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_5<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Received payload corresponding to the data byte 4 under Pretended Networking mode"]
   #[inline] pub fn data_byte_4(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Received payload corresponding to the data byte 4 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_4<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Wmb0D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb0D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for C/S"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb1Cs(pub u32);
impl Wmb1Cs {
#[doc="Length of Data in Bytes"]
   #[inline] pub fn dlc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="Length of Data in Bytes"]
   #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Remote Transmission Request Bit"]
   #[inline] pub fn rtr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Remote Transmission Request Bit"]
   #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="ID Extended Bit"]
   #[inline] pub fn ide(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="ID Extended Bit"]
   #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Substitute Remote Request"]
   #[inline] pub fn srr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Substitute Remote Request"]
   #[inline] pub fn set_srr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

}
impl ::core::fmt::Display for Wmb1Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb1Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for ID"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb1Id(pub u32);
impl Wmb1Id {
#[doc="Received ID under Pretended Networking mode"]
   #[inline] pub fn id(&self) -> bits::U29 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fffffff) as u32) } // [28:0]
   }
#[doc="Received ID under Pretended Networking mode"]
   #[inline] pub fn set_id<V: Into<bits::U29>>(mut self, value: V) -> Self {
      let value: bits::U29 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Wmb1Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb1Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for Data 0-3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb1D03(pub u32);
impl Wmb1D03 {
#[doc="Received payload corresponding to the data byte 3 under Pretended Networking mode"]
   #[inline] pub fn data_byte_3(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Received payload corresponding to the data byte 3 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_3<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Received payload corresponding to the data byte 2 under Pretended Networking mode"]
   #[inline] pub fn data_byte_2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Received payload corresponding to the data byte 2 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Received payload corresponding to the data byte 1 under Pretended Networking mode"]
   #[inline] pub fn data_byte_1(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Received payload corresponding to the data byte 1 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_1<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Received payload corresponding to the data byte 0 under Pretended Networking mode"]
   #[inline] pub fn data_byte_0(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Received payload corresponding to the data byte 0 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_0<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Wmb1D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb1D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register Data 4-7"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb1D47(pub u32);
impl Wmb1D47 {
#[doc="Received payload corresponding to the data byte 7 under Pretended Networking mode"]
   #[inline] pub fn data_byte_7(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Received payload corresponding to the data byte 7 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_7<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Received payload corresponding to the data byte 6 under Pretended Networking mode"]
   #[inline] pub fn data_byte_6(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Received payload corresponding to the data byte 6 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_6<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Received payload corresponding to the data byte 5 under Pretended Networking mode"]
   #[inline] pub fn data_byte_5(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Received payload corresponding to the data byte 5 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_5<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Received payload corresponding to the data byte 4 under Pretended Networking mode"]
   #[inline] pub fn data_byte_4(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Received payload corresponding to the data byte 4 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_4<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Wmb1D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb1D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for C/S"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb2Cs(pub u32);
impl Wmb2Cs {
#[doc="Length of Data in Bytes"]
   #[inline] pub fn dlc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="Length of Data in Bytes"]
   #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Remote Transmission Request Bit"]
   #[inline] pub fn rtr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Remote Transmission Request Bit"]
   #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="ID Extended Bit"]
   #[inline] pub fn ide(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="ID Extended Bit"]
   #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Substitute Remote Request"]
   #[inline] pub fn srr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Substitute Remote Request"]
   #[inline] pub fn set_srr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

}
impl ::core::fmt::Display for Wmb2Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb2Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for ID"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb2Id(pub u32);
impl Wmb2Id {
#[doc="Received ID under Pretended Networking mode"]
   #[inline] pub fn id(&self) -> bits::U29 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fffffff) as u32) } // [28:0]
   }
#[doc="Received ID under Pretended Networking mode"]
   #[inline] pub fn set_id<V: Into<bits::U29>>(mut self, value: V) -> Self {
      let value: bits::U29 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Wmb2Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb2Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for Data 0-3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb2D03(pub u32);
impl Wmb2D03 {
#[doc="Received payload corresponding to the data byte 3 under Pretended Networking mode"]
   #[inline] pub fn data_byte_3(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Received payload corresponding to the data byte 3 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_3<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Received payload corresponding to the data byte 2 under Pretended Networking mode"]
   #[inline] pub fn data_byte_2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Received payload corresponding to the data byte 2 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Received payload corresponding to the data byte 1 under Pretended Networking mode"]
   #[inline] pub fn data_byte_1(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Received payload corresponding to the data byte 1 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_1<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Received payload corresponding to the data byte 0 under Pretended Networking mode"]
   #[inline] pub fn data_byte_0(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Received payload corresponding to the data byte 0 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_0<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Wmb2D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb2D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register Data 4-7"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb2D47(pub u32);
impl Wmb2D47 {
#[doc="Received payload corresponding to the data byte 7 under Pretended Networking mode"]
   #[inline] pub fn data_byte_7(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Received payload corresponding to the data byte 7 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_7<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Received payload corresponding to the data byte 6 under Pretended Networking mode"]
   #[inline] pub fn data_byte_6(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Received payload corresponding to the data byte 6 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_6<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Received payload corresponding to the data byte 5 under Pretended Networking mode"]
   #[inline] pub fn data_byte_5(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Received payload corresponding to the data byte 5 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_5<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Received payload corresponding to the data byte 4 under Pretended Networking mode"]
   #[inline] pub fn data_byte_4(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Received payload corresponding to the data byte 4 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_4<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Wmb2D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb2D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for C/S"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb3Cs(pub u32);
impl Wmb3Cs {
#[doc="Length of Data in Bytes"]
   #[inline] pub fn dlc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="Length of Data in Bytes"]
   #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Remote Transmission Request Bit"]
   #[inline] pub fn rtr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Remote Transmission Request Bit"]
   #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="ID Extended Bit"]
   #[inline] pub fn ide(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="ID Extended Bit"]
   #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Substitute Remote Request"]
   #[inline] pub fn srr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Substitute Remote Request"]
   #[inline] pub fn set_srr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

}
impl ::core::fmt::Display for Wmb3Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb3Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
      if self.rtr() != 0 { try!(write!(f, " rtr"))}
      if self.ide() != 0 { try!(write!(f, " ide"))}
      if self.srr() != 0 { try!(write!(f, " srr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for ID"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb3Id(pub u32);
impl Wmb3Id {
#[doc="Received ID under Pretended Networking mode"]
   #[inline] pub fn id(&self) -> bits::U29 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fffffff) as u32) } // [28:0]
   }
#[doc="Received ID under Pretended Networking mode"]
   #[inline] pub fn set_id<V: Into<bits::U29>>(mut self, value: V) -> Self {
      let value: bits::U29 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Wmb3Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb3Id {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register for Data 0-3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb3D03(pub u32);
impl Wmb3D03 {
#[doc="Received payload corresponding to the data byte 3 under Pretended Networking mode"]
   #[inline] pub fn data_byte_3(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Received payload corresponding to the data byte 3 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_3<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Received payload corresponding to the data byte 2 under Pretended Networking mode"]
   #[inline] pub fn data_byte_2(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Received payload corresponding to the data byte 2 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_2<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Received payload corresponding to the data byte 1 under Pretended Networking mode"]
   #[inline] pub fn data_byte_1(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Received payload corresponding to the data byte 1 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_1<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Received payload corresponding to the data byte 0 under Pretended Networking mode"]
   #[inline] pub fn data_byte_0(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Received payload corresponding to the data byte 0 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_0<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Wmb3D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb3D03 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_3() != 0 { try!(write!(f, " data_byte_3=0x{:x}", self.data_byte_3()))}
      if self.data_byte_2() != 0 { try!(write!(f, " data_byte_2=0x{:x}", self.data_byte_2()))}
      if self.data_byte_1() != 0 { try!(write!(f, " data_byte_1=0x{:x}", self.data_byte_1()))}
      if self.data_byte_0() != 0 { try!(write!(f, " data_byte_0=0x{:x}", self.data_byte_0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Wake Up Message Buffer Register Data 4-7"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wmb3D47(pub u32);
impl Wmb3D47 {
#[doc="Received payload corresponding to the data byte 7 under Pretended Networking mode"]
   #[inline] pub fn data_byte_7(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Received payload corresponding to the data byte 7 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_7<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Received payload corresponding to the data byte 6 under Pretended Networking mode"]
   #[inline] pub fn data_byte_6(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Received payload corresponding to the data byte 6 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_6<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Received payload corresponding to the data byte 5 under Pretended Networking mode"]
   #[inline] pub fn data_byte_5(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Received payload corresponding to the data byte 5 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_5<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Received payload corresponding to the data byte 4 under Pretended Networking mode"]
   #[inline] pub fn data_byte_4(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Received payload corresponding to the data byte 4 under Pretended Networking mode"]
   #[inline] pub fn set_data_byte_4<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Wmb3D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wmb3D47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data_byte_7() != 0 { try!(write!(f, " data_byte_7=0x{:x}", self.data_byte_7()))}
      if self.data_byte_6() != 0 { try!(write!(f, " data_byte_6=0x{:x}", self.data_byte_6()))}
      if self.data_byte_5() != 0 { try!(write!(f, " data_byte_5=0x{:x}", self.data_byte_5()))}
      if self.data_byte_4() != 0 { try!(write!(f, " data_byte_4=0x{:x}", self.data_byte_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CAN FD Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fdctrl(pub u32);
impl Fdctrl {
#[doc="Transceiver Delay Compensation Value"]
   #[inline] pub fn tdcval(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Transceiver Delay Compensation Value"]
   #[inline] pub fn set_tdcval<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transceiver Delay Compensation Offset"]
   #[inline] pub fn tdcoff(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
   }
#[doc="Transceiver Delay Compensation Offset"]
   #[inline] pub fn set_tdcoff<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Transceiver Delay Compensation Fail"]
   #[inline] pub fn tdcfail(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Transceiver Delay Compensation Fail"]
   #[inline] pub fn set_tdcfail<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Transceiver Delay Compensation Enable"]
   #[inline] pub fn tdcen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Transceiver Delay Compensation Enable"]
   #[inline] pub fn set_tdcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Message Buffer Data Size for Region 0"]
   #[inline] pub fn mbdsr0(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }
#[doc="Message Buffer Data Size for Region 0"]
   #[inline] pub fn set_mbdsr0<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Bit Rate Switch Enable"]
   #[inline] pub fn fdrate(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Bit Rate Switch Enable"]
   #[inline] pub fn set_fdrate<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Fdctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fdctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tdcval() != 0 { try!(write!(f, " tdcval=0x{:x}", self.tdcval()))}
      if self.tdcoff() != 0 { try!(write!(f, " tdcoff=0x{:x}", self.tdcoff()))}
      if self.tdcfail() != 0 { try!(write!(f, " tdcfail"))}
      if self.tdcen() != 0 { try!(write!(f, " tdcen"))}
      if self.mbdsr0() != 0 { try!(write!(f, " mbdsr0=0x{:x}", self.mbdsr0()))}
      if self.fdrate() != 0 { try!(write!(f, " fdrate"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CAN FD Bit Timing Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fdcbt(pub u32);
impl Fdcbt {
#[doc="Fast Phase Segment 2"]
   #[inline] pub fn fpseg2(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Fast Phase Segment 2"]
   #[inline] pub fn set_fpseg2<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Fast Phase Segment 1"]
   #[inline] pub fn fpseg1(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
   }
#[doc="Fast Phase Segment 1"]
   #[inline] pub fn set_fpseg1<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Fast Propagation Segment"]
   #[inline] pub fn fpropseg(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
   }
#[doc="Fast Propagation Segment"]
   #[inline] pub fn set_fpropseg<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Fast Resync Jump Width"]
   #[inline] pub fn frjw(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
   }
#[doc="Fast Resync Jump Width"]
   #[inline] pub fn set_frjw<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Fast Prescaler Division Factor"]
   #[inline] pub fn fpresdiv(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3ff) as u16) } // [29:20]
   }
#[doc="Fast Prescaler Division Factor"]
   #[inline] pub fn set_fpresdiv<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 20);
      self.0 |= value << 20;
      self
   }

}
impl ::core::fmt::Display for Fdcbt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fdcbt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fpseg2() != 0 { try!(write!(f, " fpseg2=0x{:x}", self.fpseg2()))}
      if self.fpseg1() != 0 { try!(write!(f, " fpseg1=0x{:x}", self.fpseg1()))}
      if self.fpropseg() != 0 { try!(write!(f, " fpropseg=0x{:x}", self.fpropseg()))}
      if self.frjw() != 0 { try!(write!(f, " frjw=0x{:x}", self.frjw()))}
      if self.fpresdiv() != 0 { try!(write!(f, " fpresdiv=0x{:x}", self.fpresdiv()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CAN FD CRC Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fdcrc(pub u32);
impl Fdcrc {
#[doc="Extended Transmitted CRC value"]
   #[inline] pub fn fd_txcrc(&self) -> bits::U21 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fffff) as u32) } // [20:0]
   }
#[doc="Extended Transmitted CRC value"]
   #[inline] pub fn set_fd_txcrc<V: Into<bits::U21>>(mut self, value: V) -> Self {
      let value: bits::U21 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1fffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CRC Mailbox Number for FD_TXCRC"]
   #[inline] pub fn fd_mbcrc(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7f) as u8) } // [30:24]
   }
#[doc="CRC Mailbox Number for FD_TXCRC"]
   #[inline] pub fn set_fd_mbcrc<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Fdcrc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fdcrc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fd_txcrc() != 0 { try!(write!(f, " fd_txcrc=0x{:x}", self.fd_txcrc()))}
      if self.fd_mbcrc() != 0 { try!(write!(f, " fd_mbcrc=0x{:x}", self.fd_mbcrc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

