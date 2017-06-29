//! Direct Memory Access Controller
pub const DMAC: Dmac = Periph(0x41004800, DmacId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMAC Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct DmacId {}
pub type Dmac = Periph<DmacId>;



impl<T> Periph<T> {
#[doc="Get the *const pointer for the ACTIVE register."]
  #[inline] pub fn active_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the ACTIVE register."]
  #[inline] pub fn active_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the ACTIVE register."]
  #[inline] pub fn active(&self) -> Active { 
     unsafe {
        Active(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }

#[doc="Get the *const pointer for the BASEADDR register."]
  #[inline] pub fn baseaddr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the BASEADDR register."]
  #[inline] pub fn baseaddr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Read the BASEADDR register."]
  #[inline] pub fn baseaddr(&self) -> Baseaddr { 
     unsafe {
        Baseaddr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
#[doc="Write the BASEADDR register."]
  #[inline] pub fn set_baseaddr(&self, value: Baseaddr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BASEADDR register."]
  #[inline] pub fn with_baseaddr<F: FnOnce(Baseaddr) -> Baseaddr>(&self, f: F) -> &Self {
     let tmp = self.baseaddr();
     self.set_baseaddr(f(tmp))
  }

#[doc="Get the *const pointer for the BUSYCH register."]
  #[inline] pub fn busych_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the BUSYCH register."]
  #[inline] pub fn busych_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the BUSYCH register."]
  #[inline] pub fn busych(&self) -> Busych { 
     unsafe {
        Busych(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CHCTRLA register."]
  #[inline] pub fn chctrla_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x40) as *const u8
  }
#[doc="Get the *mut pointer for the CHCTRLA register."]
  #[inline] pub fn chctrla_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x40) as *mut u8
  }
#[doc="Read the CHCTRLA register."]
  #[inline] pub fn chctrla(&self) -> Chctrla { 
     unsafe {
        Chctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u8))
     }
  }
#[doc="Write the CHCTRLA register."]
  #[inline] pub fn set_chctrla(&self, value: Chctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CHCTRLA register."]
  #[inline] pub fn with_chctrla<F: FnOnce(Chctrla) -> Chctrla>(&self, f: F) -> &Self {
     let tmp = self.chctrla();
     self.set_chctrla(f(tmp))
  }

#[doc="Get the *const pointer for the CHCTRLB register."]
  #[inline] pub fn chctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the CHCTRLB register."]
  #[inline] pub fn chctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the CHCTRLB register."]
  #[inline] pub fn chctrlb(&self) -> Chctrlb { 
     unsafe {
        Chctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the CHCTRLB register."]
  #[inline] pub fn set_chctrlb(&self, value: Chctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CHCTRLB register."]
  #[inline] pub fn with_chctrlb<F: FnOnce(Chctrlb) -> Chctrlb>(&self, f: F) -> &Self {
     let tmp = self.chctrlb();
     self.set_chctrlb(f(tmp))
  }

#[doc="Get the *const pointer for the CHID register."]
  #[inline] pub fn chid_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x3f) as *const u8
  }
#[doc="Get the *mut pointer for the CHID register."]
  #[inline] pub fn chid_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x3f) as *mut u8
  }
#[doc="Read the CHID register."]
  #[inline] pub fn chid(&self) -> Chid { 
     unsafe {
        Chid(::core::ptr::read_volatile(((self.0 as usize) + 0x3f) as *const u8))
     }
  }
#[doc="Write the CHID register."]
  #[inline] pub fn set_chid(&self, value: Chid) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3f) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CHID register."]
  #[inline] pub fn with_chid<F: FnOnce(Chid) -> Chid>(&self, f: F) -> &Self {
     let tmp = self.chid();
     self.set_chid(f(tmp))
  }

#[doc="Get the *const pointer for the CHINTENCLR register."]
  #[inline] pub fn chintenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4c) as *const u8
  }
#[doc="Get the *mut pointer for the CHINTENCLR register."]
  #[inline] pub fn chintenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4c) as *mut u8
  }
#[doc="Read the CHINTENCLR register."]
  #[inline] pub fn chintenclr(&self) -> Chintenclr { 
     unsafe {
        Chintenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u8))
     }
  }
#[doc="Write the CHINTENCLR register."]
  #[inline] pub fn set_chintenclr(&self, value: Chintenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CHINTENCLR register."]
  #[inline] pub fn with_chintenclr<F: FnOnce(Chintenclr) -> Chintenclr>(&self, f: F) -> &Self {
     let tmp = self.chintenclr();
     self.set_chintenclr(f(tmp))
  }

#[doc="Get the *const pointer for the CHINTENSET register."]
  #[inline] pub fn chintenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4d) as *const u8
  }
#[doc="Get the *mut pointer for the CHINTENSET register."]
  #[inline] pub fn chintenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4d) as *mut u8
  }
#[doc="Read the CHINTENSET register."]
  #[inline] pub fn chintenset(&self) -> Chintenset { 
     unsafe {
        Chintenset(::core::ptr::read_volatile(((self.0 as usize) + 0x4d) as *const u8))
     }
  }
#[doc="Write the CHINTENSET register."]
  #[inline] pub fn set_chintenset(&self, value: Chintenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4d) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CHINTENSET register."]
  #[inline] pub fn with_chintenset<F: FnOnce(Chintenset) -> Chintenset>(&self, f: F) -> &Self {
     let tmp = self.chintenset();
     self.set_chintenset(f(tmp))
  }

#[doc="Get the *const pointer for the CHINTFLAG register."]
  #[inline] pub fn chintflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4e) as *const u8
  }
#[doc="Get the *mut pointer for the CHINTFLAG register."]
  #[inline] pub fn chintflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4e) as *mut u8
  }
#[doc="Read the CHINTFLAG register."]
  #[inline] pub fn chintflag(&self) -> Chintflag { 
     unsafe {
        Chintflag(::core::ptr::read_volatile(((self.0 as usize) + 0x4e) as *const u8))
     }
  }
#[doc="Write the CHINTFLAG register."]
  #[inline] pub fn set_chintflag(&self, value: Chintflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4e) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CHINTFLAG register."]
  #[inline] pub fn with_chintflag<F: FnOnce(Chintflag) -> Chintflag>(&self, f: F) -> &Self {
     let tmp = self.chintflag();
     self.set_chintflag(f(tmp))
  }

#[doc="Get the *const pointer for the CHSTATUS register."]
  #[inline] pub fn chstatus_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4f) as *const u8
  }
#[doc="Get the *mut pointer for the CHSTATUS register."]
  #[inline] pub fn chstatus_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4f) as *mut u8
  }
#[doc="Read the CHSTATUS register."]
  #[inline] pub fn chstatus(&self) -> Chstatus { 
     unsafe {
        Chstatus(::core::ptr::read_volatile(((self.0 as usize) + 0x4f) as *const u8))
     }
  }

#[doc="Get the *const pointer for the CRCCHKSUM register."]
  #[inline] pub fn crcchksum_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CRCCHKSUM register."]
  #[inline] pub fn crcchksum_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CRCCHKSUM register."]
  #[inline] pub fn crcchksum(&self) -> Crcchksum { 
     unsafe {
        Crcchksum(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CRCCHKSUM register."]
  #[inline] pub fn set_crcchksum(&self, value: Crcchksum) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CRCCHKSUM register."]
  #[inline] pub fn with_crcchksum<F: FnOnce(Crcchksum) -> Crcchksum>(&self, f: F) -> &Self {
     let tmp = self.crcchksum();
     self.set_crcchksum(f(tmp))
  }

#[doc="Get the *const pointer for the CRCCTRL register."]
  #[inline] pub fn crcctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
#[doc="Get the *mut pointer for the CRCCTRL register."]
  #[inline] pub fn crcctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
#[doc="Read the CRCCTRL register."]
  #[inline] pub fn crcctrl(&self) -> Crcctrl { 
     unsafe {
        Crcctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
#[doc="Write the CRCCTRL register."]
  #[inline] pub fn set_crcctrl(&self, value: Crcctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CRCCTRL register."]
  #[inline] pub fn with_crcctrl<F: FnOnce(Crcctrl) -> Crcctrl>(&self, f: F) -> &Self {
     let tmp = self.crcctrl();
     self.set_crcctrl(f(tmp))
  }

#[doc="Get the *const pointer for the CRCDATAIN register."]
  #[inline] pub fn crcdatain_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CRCDATAIN register."]
  #[inline] pub fn crcdatain_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CRCDATAIN register."]
  #[inline] pub fn crcdatain(&self) -> Crcdatain { 
     unsafe {
        Crcdatain(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CRCDATAIN register."]
  #[inline] pub fn set_crcdatain(&self, value: Crcdatain) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CRCDATAIN register."]
  #[inline] pub fn with_crcdatain<F: FnOnce(Crcdatain) -> Crcdatain>(&self, f: F) -> &Self {
     let tmp = self.crcdatain();
     self.set_crcdatain(f(tmp))
  }

#[doc="Get the *const pointer for the CRCSTATUS register."]
  #[inline] pub fn crcstatus_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
#[doc="Get the *mut pointer for the CRCSTATUS register."]
  #[inline] pub fn crcstatus_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
#[doc="Read the CRCSTATUS register."]
  #[inline] pub fn crcstatus(&self) -> Crcstatus { 
     unsafe {
        Crcstatus(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
#[doc="Write the CRCSTATUS register."]
  #[inline] pub fn set_crcstatus(&self, value: Crcstatus) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CRCSTATUS register."]
  #[inline] pub fn with_crcstatus<F: FnOnce(Crcstatus) -> Crcstatus>(&self, f: F) -> &Self {
     let tmp = self.crcstatus();
     self.set_crcstatus(f(tmp))
  }

#[doc="Get the *const pointer for the CTRL register."]
  #[inline] pub fn ctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
#[doc="Get the *mut pointer for the CTRL register."]
  #[inline] pub fn ctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
#[doc="Read the CTRL register."]
  #[inline] pub fn ctrl(&self) -> Ctrl { 
     unsafe {
        Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
#[doc="Write the CTRL register."]
  #[inline] pub fn set_ctrl(&self, value: Ctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CTRL register."]
  #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

#[doc="Get the *const pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
#[doc="Get the *mut pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
#[doc="Read the DBGCTRL register."]
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

#[doc="Get the *const pointer for the INTPEND register."]
  #[inline] pub fn intpend_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x20) as *const u16
  }
#[doc="Get the *mut pointer for the INTPEND register."]
  #[inline] pub fn intpend_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x20) as *mut u16
  }
#[doc="Read the INTPEND register."]
  #[inline] pub fn intpend(&self) -> Intpend { 
     unsafe {
        Intpend(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u16))
     }
  }
#[doc="Write the INTPEND register."]
  #[inline] pub fn set_intpend(&self, value: Intpend) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the INTPEND register."]
  #[inline] pub fn with_intpend<F: FnOnce(Intpend) -> Intpend>(&self, f: F) -> &Self {
     let tmp = self.intpend();
     self.set_intpend(f(tmp))
  }

#[doc="Get the *const pointer for the INTSTATUS register."]
  #[inline] pub fn intstatus_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the INTSTATUS register."]
  #[inline] pub fn intstatus_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the INTSTATUS register."]
  #[inline] pub fn intstatus(&self) -> Intstatus { 
     unsafe {
        Intstatus(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PENDCH register."]
  #[inline] pub fn pendch_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the PENDCH register."]
  #[inline] pub fn pendch_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Read the PENDCH register."]
  #[inline] pub fn pendch(&self) -> Pendch { 
     unsafe {
        Pendch(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PRICTRL register."]
  #[inline] pub fn prictrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the PRICTRL register."]
  #[inline] pub fn prictrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the PRICTRL register."]
  #[inline] pub fn prictrl(&self) -> Prictrl { 
     unsafe {
        Prictrl(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the PRICTRL register."]
  #[inline] pub fn set_prictrl(&self, value: Prictrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PRICTRL register."]
  #[inline] pub fn with_prictrl<F: FnOnce(Prictrl) -> Prictrl>(&self, f: F) -> &Self {
     let tmp = self.prictrl();
     self.set_prictrl(f(tmp))
  }

#[doc="Get the *const pointer for the SWTRIGCTRL register."]
  #[inline] pub fn swtrigctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the SWTRIGCTRL register."]
  #[inline] pub fn swtrigctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the SWTRIGCTRL register."]
  #[inline] pub fn swtrigctrl(&self) -> Swtrigctrl { 
     unsafe {
        Swtrigctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the SWTRIGCTRL register."]
  #[inline] pub fn set_swtrigctrl(&self, value: Swtrigctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SWTRIGCTRL register."]
  #[inline] pub fn with_swtrigctrl<F: FnOnce(Swtrigctrl) -> Swtrigctrl>(&self, f: F) -> &Self {
     let tmp = self.swtrigctrl();
     self.set_swtrigctrl(f(tmp))
  }

#[doc="Get the *const pointer for the WRBADDR register."]
  #[inline] pub fn wrbaddr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
#[doc="Get the *mut pointer for the WRBADDR register."]
  #[inline] pub fn wrbaddr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
#[doc="Read the WRBADDR register."]
  #[inline] pub fn wrbaddr(&self) -> Wrbaddr { 
     unsafe {
        Wrbaddr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
#[doc="Write the WRBADDR register."]
  #[inline] pub fn set_wrbaddr(&self, value: Wrbaddr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the WRBADDR register."]
  #[inline] pub fn with_wrbaddr<F: FnOnce(Wrbaddr) -> Wrbaddr>(&self, f: F) -> &Self {
     let tmp = self.wrbaddr();
     self.set_wrbaddr(f(tmp))
  }

}

#[doc="Active Channel and Levels"]
#[derive(PartialEq, Eq)]
pub struct Active(pub u32);
impl Active {
#[doc="Level n Channel Trigger Request Executing"]
  #[inline] pub fn lvlex(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Level n Channel Trigger Request Executing"]
  #[inline] pub fn set_lvlex(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Active Channel ID"]
  #[inline] pub fn id(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
#[doc="Active Channel ID"]
  #[inline] pub fn set_id(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Active Channel Busy"]
  #[inline] pub fn abusy(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Active Channel Busy"]
  #[inline] pub fn set_abusy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Active Channel Block Transfer Count"]
  #[inline] pub fn btcnt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
#[doc="Active Channel Block Transfer Count"]
  #[inline] pub fn set_btcnt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Active {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Active {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lvlex(0) != 0 { try!(write!(f, " lvlex[0]"))}
      if self.lvlex(1) != 0 { try!(write!(f, " lvlex[1]"))}
      if self.lvlex(2) != 0 { try!(write!(f, " lvlex[2]"))}
      if self.lvlex(3) != 0 { try!(write!(f, " lvlex[3]"))}
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      if self.abusy() != 0 { try!(write!(f, " abusy"))}
      if self.btcnt() != 0 { try!(write!(f, " btcnt=0x{:x}", self.btcnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Descriptor Memory Section Base Address"]
#[derive(PartialEq, Eq)]
pub struct Baseaddr(pub u32);
impl Baseaddr {
#[doc="Descriptor Memory Base Address"]
  #[inline] pub fn baseaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Descriptor Memory Base Address"]
  #[inline] pub fn set_baseaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Baseaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Baseaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Busy Channels"]
#[derive(PartialEq, Eq)]
pub struct Busych(pub u32);
impl Busych {
#[doc="Busy Channel n"]
  #[inline] pub fn busych(&self, index: usize) -> u32 {
     assert!(index < 12);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Busy Channel n"]
  #[inline] pub fn set_busych(mut self, index: usize, value: u32) -> Self {
     assert!(index < 12);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Busych {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Busych {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.busych(0) != 0 { try!(write!(f, " busych[0]"))}
      if self.busych(1) != 0 { try!(write!(f, " busych[1]"))}
      if self.busych(2) != 0 { try!(write!(f, " busych[2]"))}
      if self.busych(3) != 0 { try!(write!(f, " busych[3]"))}
      if self.busych(4) != 0 { try!(write!(f, " busych[4]"))}
      if self.busych(5) != 0 { try!(write!(f, " busych[5]"))}
      if self.busych(6) != 0 { try!(write!(f, " busych[6]"))}
      if self.busych(7) != 0 { try!(write!(f, " busych[7]"))}
      if self.busych(8) != 0 { try!(write!(f, " busych[8]"))}
      if self.busych(9) != 0 { try!(write!(f, " busych[9]"))}
      if self.busych(10) != 0 { try!(write!(f, " busych[10]"))}
      if self.busych(11) != 0 { try!(write!(f, " busych[11]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel Control A"]
#[derive(PartialEq, Eq)]
pub struct Chctrla(pub u8);
impl Chctrla {
#[doc="Channel Software Reset"]
  #[inline] pub fn swrst(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Channel Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Channel Enable"]
  #[inline] pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Channel Enable"]
  #[inline] pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Chctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel Control B"]
#[derive(PartialEq, Eq)]
pub struct Chctrlb(pub u32);
impl Chctrlb {
#[doc="Event Input Action"]
  #[inline] pub fn evact(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
#[doc="Event Input Action"]
  #[inline] pub fn set_evact(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Channel Event Input Enable"]
  #[inline] pub fn evie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Channel Event Input Enable"]
  #[inline] pub fn set_evie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Channel Event Output Enable"]
  #[inline] pub fn evoe(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Channel Event Output Enable"]
  #[inline] pub fn set_evoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Channel Arbitration Level"]
  #[inline] pub fn lvl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
#[doc="Channel Arbitration Level"]
  #[inline] pub fn set_lvl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Peripheral Trigger Source"]
  #[inline] pub fn trigsrc(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3f // [13:8]
  }
#[doc="Peripheral Trigger Source"]
  #[inline] pub fn set_trigsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Trigger Action"]
  #[inline] pub fn trigact(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
#[doc="Trigger Action"]
  #[inline] pub fn set_trigact(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Software Command"]
  #[inline] pub fn cmd(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
#[doc="Software Command"]
  #[inline] pub fn set_cmd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Chctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
      if self.evie() != 0 { try!(write!(f, " evie"))}
      if self.evoe() != 0 { try!(write!(f, " evoe"))}
      if self.lvl() != 0 { try!(write!(f, " lvl=0x{:x}", self.lvl()))}
      if self.trigsrc() != 0 { try!(write!(f, " trigsrc=0x{:x}", self.trigsrc()))}
      if self.trigact() != 0 { try!(write!(f, " trigact=0x{:x}", self.trigact()))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel ID"]
#[derive(PartialEq, Eq)]
pub struct Chid(pub u8);
impl Chid {
#[doc="Channel ID"]
  #[inline] pub fn id(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
#[doc="Channel ID"]
  #[inline] pub fn set_id(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Chid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Chintenclr(pub u8);
impl Chintenclr {
#[doc="Transfer Error Interrupt Enable"]
  #[inline] pub fn terr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Transfer Error Interrupt Enable"]
  #[inline] pub fn set_terr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transfer Complete Interrupt Enable"]
  #[inline] pub fn tcmpl(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transfer Complete Interrupt Enable"]
  #[inline] pub fn set_tcmpl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Channel Suspend Interrupt Enable"]
  #[inline] pub fn susp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Channel Suspend Interrupt Enable"]
  #[inline] pub fn set_susp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Chintenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chintenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.terr() != 0 { try!(write!(f, " terr"))}
      if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
      if self.susp() != 0 { try!(write!(f, " susp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Chintenset(pub u8);
impl Chintenset {
#[doc="Transfer Error Interrupt Enable"]
  #[inline] pub fn terr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Transfer Error Interrupt Enable"]
  #[inline] pub fn set_terr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transfer Complete Interrupt Enable"]
  #[inline] pub fn tcmpl(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transfer Complete Interrupt Enable"]
  #[inline] pub fn set_tcmpl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Channel Suspend Interrupt Enable"]
  #[inline] pub fn susp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Channel Suspend Interrupt Enable"]
  #[inline] pub fn set_susp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Chintenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chintenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.terr() != 0 { try!(write!(f, " terr"))}
      if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
      if self.susp() != 0 { try!(write!(f, " susp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Chintflag(pub u8);
impl Chintflag {
#[doc="Transfer Error"]
  #[inline] pub fn terr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Transfer Error"]
  #[inline] pub fn set_terr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transfer Complete"]
  #[inline] pub fn tcmpl(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transfer Complete"]
  #[inline] pub fn set_tcmpl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Channel Suspend"]
  #[inline] pub fn susp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Channel Suspend"]
  #[inline] pub fn set_susp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Chintflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chintflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.terr() != 0 { try!(write!(f, " terr"))}
      if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
      if self.susp() != 0 { try!(write!(f, " susp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Channel Status"]
#[derive(PartialEq, Eq)]
pub struct Chstatus(pub u8);
impl Chstatus {
#[doc="Channel Pending"]
  #[inline] pub fn pend(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Channel Pending"]
  #[inline] pub fn set_pend(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Channel Busy"]
  #[inline] pub fn busy(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Channel Busy"]
  #[inline] pub fn set_busy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Fetch Error"]
  #[inline] pub fn ferr(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Fetch Error"]
  #[inline] pub fn set_ferr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Chstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pend() != 0 { try!(write!(f, " pend"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.ferr() != 0 { try!(write!(f, " ferr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC Checksum"]
#[derive(PartialEq, Eq)]
pub struct Crcchksum(pub u32);
impl Crcchksum {
#[doc="CRC Checksum"]
  #[inline] pub fn crcchksum(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="CRC Checksum"]
  #[inline] pub fn set_crcchksum(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Crcchksum {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcchksum {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC Control"]
#[derive(PartialEq, Eq)]
pub struct Crcctrl(pub u16);
impl Crcctrl {
#[doc="CRC Beat Size"]
  #[inline] pub fn crcbeatsize(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x3 // [1:0]
  }
#[doc="CRC Beat Size"]
  #[inline] pub fn set_crcbeatsize(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="CRC Polynomial Type"]
  #[inline] pub fn crcpoly(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
#[doc="CRC Polynomial Type"]
  #[inline] pub fn set_crcpoly(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="CRC Input Source"]
  #[inline] pub fn crcsrc(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x3f // [13:8]
  }
#[doc="CRC Input Source"]
  #[inline] pub fn set_crcsrc(mut self, value: u16) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Crcctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.crcbeatsize() != 0 { try!(write!(f, " crcbeatsize=0x{:x}", self.crcbeatsize()))}
      if self.crcpoly() != 0 { try!(write!(f, " crcpoly=0x{:x}", self.crcpoly()))}
      if self.crcsrc() != 0 { try!(write!(f, " crcsrc=0x{:x}", self.crcsrc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC Data Input"]
#[derive(PartialEq, Eq)]
pub struct Crcdatain(pub u32);
impl Crcdatain {
#[doc="CRC Data Input"]
  #[inline] pub fn crcdatain(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="CRC Data Input"]
  #[inline] pub fn set_crcdatain(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Crcdatain {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcdatain {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC Status"]
#[derive(PartialEq, Eq)]
pub struct Crcstatus(pub u8);
impl Crcstatus {
#[doc="CRC Module Busy"]
  #[inline] pub fn crcbusy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="CRC Module Busy"]
  #[inline] pub fn set_crcbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="CRC Zero"]
  #[inline] pub fn crczero(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="CRC Zero"]
  #[inline] pub fn set_crczero(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Crcstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.crcbusy() != 0 { try!(write!(f, " crcbusy"))}
      if self.crczero() != 0 { try!(write!(f, " crczero"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control"]
#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="DMA Enable"]
  #[inline] pub fn dmaenable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="DMA Enable"]
  #[inline] pub fn set_dmaenable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="CRC Enable"]
  #[inline] pub fn crcenable(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="CRC Enable"]
  #[inline] pub fn set_crcenable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Priority Level n Enable"]
  #[inline] pub fn lvlen(&self, index: usize) -> u16 {
     assert!(index < 4);
     let shift: usize = 8 + index;
     ((self.0 as u16) >> shift) & 0x1 // [8]
  }
#[doc="Priority Level n Enable"]
  #[inline] pub fn set_lvlen(mut self, index: usize, value: u16) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 8 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.dmaenable() != 0 { try!(write!(f, " dmaenable"))}
      if self.crcenable() != 0 { try!(write!(f, " crcenable"))}
      if self.lvlen(0) != 0 { try!(write!(f, " lvlen[0]"))}
      if self.lvlen(1) != 0 { try!(write!(f, " lvlen[1]"))}
      if self.lvlen(2) != 0 { try!(write!(f, " lvlen[2]"))}
      if self.lvlen(3) != 0 { try!(write!(f, " lvlen[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug Control"]
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run"]
  #[inline] pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Run"]
  #[inline] pub fn set_dbgrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Pending"]
#[derive(PartialEq, Eq)]
pub struct Intpend(pub u16);
impl Intpend {
#[doc="Channel ID"]
  #[inline] pub fn id(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xf // [3:0]
  }
#[doc="Channel ID"]
  #[inline] pub fn set_id(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transfer Error"]
  #[inline] pub fn terr(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
#[doc="Transfer Error"]
  #[inline] pub fn set_terr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Transfer Complete"]
  #[inline] pub fn tcmpl(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
#[doc="Transfer Complete"]
  #[inline] pub fn set_tcmpl(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Channel Suspend"]
  #[inline] pub fn susp(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
#[doc="Channel Suspend"]
  #[inline] pub fn set_susp(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Fetch Error"]
  #[inline] pub fn ferr(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
#[doc="Fetch Error"]
  #[inline] pub fn set_ferr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Busy"]
  #[inline] pub fn busy(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
#[doc="Busy"]
  #[inline] pub fn set_busy(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Pending"]
  #[inline] pub fn pend(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
#[doc="Pending"]
  #[inline] pub fn set_pend(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Intpend {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intpend {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      if self.terr() != 0 { try!(write!(f, " terr"))}
      if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
      if self.susp() != 0 { try!(write!(f, " susp"))}
      if self.ferr() != 0 { try!(write!(f, " ferr"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.pend() != 0 { try!(write!(f, " pend"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Status"]
#[derive(PartialEq, Eq)]
pub struct Intstatus(pub u32);
impl Intstatus {
#[doc="Channel n Pending Interrupt"]
  #[inline] pub fn chint(&self, index: usize) -> u32 {
     assert!(index < 12);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Channel n Pending Interrupt"]
  #[inline] pub fn set_chint(mut self, index: usize, value: u32) -> Self {
     assert!(index < 12);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Intstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chint(0) != 0 { try!(write!(f, " chint[0]"))}
      if self.chint(1) != 0 { try!(write!(f, " chint[1]"))}
      if self.chint(2) != 0 { try!(write!(f, " chint[2]"))}
      if self.chint(3) != 0 { try!(write!(f, " chint[3]"))}
      if self.chint(4) != 0 { try!(write!(f, " chint[4]"))}
      if self.chint(5) != 0 { try!(write!(f, " chint[5]"))}
      if self.chint(6) != 0 { try!(write!(f, " chint[6]"))}
      if self.chint(7) != 0 { try!(write!(f, " chint[7]"))}
      if self.chint(8) != 0 { try!(write!(f, " chint[8]"))}
      if self.chint(9) != 0 { try!(write!(f, " chint[9]"))}
      if self.chint(10) != 0 { try!(write!(f, " chint[10]"))}
      if self.chint(11) != 0 { try!(write!(f, " chint[11]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pending Channels"]
#[derive(PartialEq, Eq)]
pub struct Pendch(pub u32);
impl Pendch {
#[doc="Pending Channel n"]
  #[inline] pub fn pendch(&self, index: usize) -> u32 {
     assert!(index < 12);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Pending Channel n"]
  #[inline] pub fn set_pendch(mut self, index: usize, value: u32) -> Self {
     assert!(index < 12);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Pendch {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pendch {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pendch(0) != 0 { try!(write!(f, " pendch[0]"))}
      if self.pendch(1) != 0 { try!(write!(f, " pendch[1]"))}
      if self.pendch(2) != 0 { try!(write!(f, " pendch[2]"))}
      if self.pendch(3) != 0 { try!(write!(f, " pendch[3]"))}
      if self.pendch(4) != 0 { try!(write!(f, " pendch[4]"))}
      if self.pendch(5) != 0 { try!(write!(f, " pendch[5]"))}
      if self.pendch(6) != 0 { try!(write!(f, " pendch[6]"))}
      if self.pendch(7) != 0 { try!(write!(f, " pendch[7]"))}
      if self.pendch(8) != 0 { try!(write!(f, " pendch[8]"))}
      if self.pendch(9) != 0 { try!(write!(f, " pendch[9]"))}
      if self.pendch(10) != 0 { try!(write!(f, " pendch[10]"))}
      if self.pendch(11) != 0 { try!(write!(f, " pendch[11]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Priority Control 0"]
#[derive(PartialEq, Eq)]
pub struct Prictrl(pub u32);
impl Prictrl {
#[doc="Level n Channel Priority Number"]
  #[inline] pub fn lvlpri(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
#[doc="Level n Channel Priority Number"]
  #[inline] pub fn set_lvlpri(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Level 0 Round-Robin Scheduling Enable"]
  #[inline] pub fn rrlvlen(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 7 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [7]
  }
#[doc="Level 0 Round-Robin Scheduling Enable"]
  #[inline] pub fn set_rrlvlen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 7 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Prictrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prictrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lvlpri(0) != 0 { try!(write!(f, " lvlpri[0]=0x{:x}", self.lvlpri(0)))}
      if self.lvlpri(1) != 0 { try!(write!(f, " lvlpri[1]=0x{:x}", self.lvlpri(1)))}
      if self.lvlpri(2) != 0 { try!(write!(f, " lvlpri[2]=0x{:x}", self.lvlpri(2)))}
      if self.lvlpri(3) != 0 { try!(write!(f, " lvlpri[3]=0x{:x}", self.lvlpri(3)))}
      if self.rrlvlen(0) != 0 { try!(write!(f, " rrlvlen[0]"))}
      if self.rrlvlen(1) != 0 { try!(write!(f, " rrlvlen[1]"))}
      if self.rrlvlen(2) != 0 { try!(write!(f, " rrlvlen[2]"))}
      if self.rrlvlen(3) != 0 { try!(write!(f, " rrlvlen[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Software Trigger Control"]
#[derive(PartialEq, Eq)]
pub struct Swtrigctrl(pub u32);
impl Swtrigctrl {
#[doc="Channel n Software Trigger"]
  #[inline] pub fn swtrig(&self, index: usize) -> u32 {
     assert!(index < 12);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Channel n Software Trigger"]
  #[inline] pub fn set_swtrig(mut self, index: usize, value: u32) -> Self {
     assert!(index < 12);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Swtrigctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Swtrigctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swtrig(0) != 0 { try!(write!(f, " swtrig[0]"))}
      if self.swtrig(1) != 0 { try!(write!(f, " swtrig[1]"))}
      if self.swtrig(2) != 0 { try!(write!(f, " swtrig[2]"))}
      if self.swtrig(3) != 0 { try!(write!(f, " swtrig[3]"))}
      if self.swtrig(4) != 0 { try!(write!(f, " swtrig[4]"))}
      if self.swtrig(5) != 0 { try!(write!(f, " swtrig[5]"))}
      if self.swtrig(6) != 0 { try!(write!(f, " swtrig[6]"))}
      if self.swtrig(7) != 0 { try!(write!(f, " swtrig[7]"))}
      if self.swtrig(8) != 0 { try!(write!(f, " swtrig[8]"))}
      if self.swtrig(9) != 0 { try!(write!(f, " swtrig[9]"))}
      if self.swtrig(10) != 0 { try!(write!(f, " swtrig[10]"))}
      if self.swtrig(11) != 0 { try!(write!(f, " swtrig[11]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Write-Back Memory Section Base Address"]
#[derive(PartialEq, Eq)]
pub struct Wrbaddr(pub u32);
impl Wrbaddr {
#[doc="Write-Back Memory Base Address"]
  #[inline] pub fn wrbaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Write-Back Memory Base Address"]
  #[inline] pub fn set_wrbaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Wrbaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wrbaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="DMAC Block Transfer Descriptor"]
pub struct Transfer(pub [u8; 16]);

impl Transfer {
#[doc="Read the BTCTRL register."]
   #[inline] pub fn btctrl(&self) -> Btctrl { 
      unsafe {
         Btctrl(::core::ptr::read_volatile(self.0.as_ptr().offset(0x0) as *const u16))
      }
   }
#[doc="Write the BTCTRL register."]
   #[inline] pub fn set_btctrl(&mut self, value: Btctrl) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut u16, value.0);
      }
      self
  }
#[doc="Modfy the BTCTRL register."]
   #[inline] pub fn with_btctrl<F: FnOnce(Btctrl) -> Btctrl>(&mut self, f: F) -> &mut Self {
      let tmp = self.btctrl();
      self.set_btctrl(f(tmp))
   }

#[doc="Read the BTCNT register."]
   #[inline] pub fn btcnt(&self) -> Btcnt { 
      unsafe {
         Btcnt(::core::ptr::read_volatile(self.0.as_ptr().offset(0x2) as *const u16))
      }
   }
#[doc="Write the BTCNT register."]
   #[inline] pub fn set_btcnt(&mut self, value: Btcnt) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x2) as *mut u16, value.0);
      }
      self
  }
#[doc="Modfy the BTCNT register."]
   #[inline] pub fn with_btcnt<F: FnOnce(Btcnt) -> Btcnt>(&mut self, f: F) -> &mut Self {
      let tmp = self.btcnt();
      self.set_btcnt(f(tmp))
   }

#[doc="Read the SRCADDR register."]
   #[inline] pub fn srcaddr(&self) -> Srcaddr { 
      unsafe {
         Srcaddr(::core::ptr::read_volatile(self.0.as_ptr().offset(0x4) as *const u32))
      }
   }
#[doc="Write the SRCADDR register."]
   #[inline] pub fn set_srcaddr(&mut self, value: Srcaddr) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x4) as *mut u32, value.0);
      }
      self
  }
#[doc="Modfy the SRCADDR register."]
   #[inline] pub fn with_srcaddr<F: FnOnce(Srcaddr) -> Srcaddr>(&mut self, f: F) -> &mut Self {
      let tmp = self.srcaddr();
      self.set_srcaddr(f(tmp))
   }

#[doc="Read the DSTADDR register."]
   #[inline] pub fn dstaddr(&self) -> Dstaddr { 
      unsafe {
         Dstaddr(::core::ptr::read_volatile(self.0.as_ptr().offset(0x8) as *const u32))
      }
   }
#[doc="Write the DSTADDR register."]
   #[inline] pub fn set_dstaddr(&mut self, value: Dstaddr) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x8) as *mut u32, value.0);
      }
      self
  }
#[doc="Modfy the DSTADDR register."]
   #[inline] pub fn with_dstaddr<F: FnOnce(Dstaddr) -> Dstaddr>(&mut self, f: F) -> &mut Self {
      let tmp = self.dstaddr();
      self.set_dstaddr(f(tmp))
   }

#[doc="Read the DESCADDR register."]
   #[inline] pub fn descaddr(&self) -> Descaddr { 
      unsafe {
         Descaddr(::core::ptr::read_volatile(self.0.as_ptr().offset(0xc) as *const u32))
      }
   }
#[doc="Write the DESCADDR register."]
   #[inline] pub fn set_descaddr(&mut self, value: Descaddr) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0xc) as *mut u32, value.0);
      }
      self
  }
#[doc="Modfy the DESCADDR register."]
   #[inline] pub fn with_descaddr<F: FnOnce(Descaddr) -> Descaddr>(&mut self, f: F) -> &mut Self {
      let tmp = self.descaddr();
      self.set_descaddr(f(tmp))
   }

}
#[doc="Block Transfer Control"]
#[derive(PartialEq, Eq)]
pub struct Btctrl(pub u16);
impl Btctrl {
#[doc="Descriptor Valid"]
  #[inline] pub fn valid(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Descriptor Valid"]
  #[inline] pub fn set_valid(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Event Output Selection"]
  #[inline] pub fn evosel(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x3 // [2:1]
  }
#[doc="Event Output Selection"]
  #[inline] pub fn set_evosel(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Block Action"]
  #[inline] pub fn blockact(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x3 // [4:3]
  }
#[doc="Block Action"]
  #[inline] pub fn set_blockact(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Beat Size"]
  #[inline] pub fn beatsize(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x3 // [9:8]
  }
#[doc="Beat Size"]
  #[inline] pub fn set_beatsize(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Source Address Increment Enable"]
  #[inline] pub fn srcinc(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
#[doc="Source Address Increment Enable"]
  #[inline] pub fn set_srcinc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Destination Address Increment Enable"]
  #[inline] pub fn dstinc(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
#[doc="Destination Address Increment Enable"]
  #[inline] pub fn set_dstinc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Step Selection"]
  #[inline] pub fn stepsel(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
#[doc="Step Selection"]
  #[inline] pub fn set_stepsel(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Address Increment Step Size"]
  #[inline] pub fn stepsize(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x7 // [15:13]
  }
#[doc="Address Increment Step Size"]
  #[inline] pub fn set_stepsize(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Btctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Btctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.valid() != 0 { try!(write!(f, " valid"))}
      if self.evosel() != 0 { try!(write!(f, " evosel=0x{:x}", self.evosel()))}
      if self.blockact() != 0 { try!(write!(f, " blockact=0x{:x}", self.blockact()))}
      if self.beatsize() != 0 { try!(write!(f, " beatsize=0x{:x}", self.beatsize()))}
      if self.srcinc() != 0 { try!(write!(f, " srcinc"))}
      if self.dstinc() != 0 { try!(write!(f, " dstinc"))}
      if self.stepsel() != 0 { try!(write!(f, " stepsel"))}
      if self.stepsize() != 0 { try!(write!(f, " stepsize=0x{:x}", self.stepsize()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Block Transfer Count"]
#[derive(PartialEq, Eq)]
pub struct Btcnt(pub u16);
impl Btcnt {
#[doc="Block Transfer Count"]
  #[inline] pub fn btcnt(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Block Transfer Count"]
  #[inline] pub fn set_btcnt(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Btcnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Btcnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.btcnt() != 0 { try!(write!(f, " btcnt=0x{:x}", self.btcnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Block Transfer Source Address"]
#[derive(PartialEq, Eq)]
pub struct Srcaddr(pub u32);
impl Srcaddr {
#[doc="Transfer Source Address"]
  #[inline] pub fn srcaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Transfer Source Address"]
  #[inline] pub fn set_srcaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Srcaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srcaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Block Transfer Source Address"]
#[derive(PartialEq, Eq)]
pub struct Dstaddr(pub u32);
impl Dstaddr {
#[doc="Transfer Destination Address"]
  #[inline] pub fn dstaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Transfer Destination Address"]
  #[inline] pub fn set_dstaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dstaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dstaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Block Transfer Destination Address"]
#[derive(PartialEq, Eq)]
pub struct Descaddr(pub u32);
impl Descaddr {
#[doc="Next Descriptor Address"]
  #[inline] pub fn descaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Next Descriptor Address"]
  #[inline] pub fn set_descaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Descaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Descaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="DMAC Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

pub const DMAC_CH0: Channel<DmacCh0Id, DmacId> = Channel { periph: DMAC, index: 0, id: DmacCh0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh0Id {}
pub type DmacCh0 = Channel<DmacCh0Id, DmacId>;

pub const DMAC_CH1: Channel<DmacCh1Id, DmacId> = Channel { periph: DMAC, index: 1, id: DmacCh1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh1Id {}
pub type DmacCh1 = Channel<DmacCh1Id, DmacId>;

pub const DMAC_CH2: Channel<DmacCh2Id, DmacId> = Channel { periph: DMAC, index: 2, id: DmacCh2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh2Id {}
pub type DmacCh2 = Channel<DmacCh2Id, DmacId>;

pub const DMAC_CH3: Channel<DmacCh3Id, DmacId> = Channel { periph: DMAC, index: 3, id: DmacCh3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh3Id {}
pub type DmacCh3 = Channel<DmacCh3Id, DmacId>;

pub const DMAC_CH4: Channel<DmacCh4Id, DmacId> = Channel { periph: DMAC, index: 4, id: DmacCh4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh4Id {}
pub type DmacCh4 = Channel<DmacCh4Id, DmacId>;

pub const DMAC_CH5: Channel<DmacCh5Id, DmacId> = Channel { periph: DMAC, index: 5, id: DmacCh5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh5Id {}
pub type DmacCh5 = Channel<DmacCh5Id, DmacId>;

pub const DMAC_CH6: Channel<DmacCh6Id, DmacId> = Channel { periph: DMAC, index: 6, id: DmacCh6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh6Id {}
pub type DmacCh6 = Channel<DmacCh6Id, DmacId>;

pub const DMAC_CH7: Channel<DmacCh7Id, DmacId> = Channel { periph: DMAC, index: 7, id: DmacCh7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh7Id {}
pub type DmacCh7 = Channel<DmacCh7Id, DmacId>;

pub const DMAC_CH8: Channel<DmacCh8Id, DmacId> = Channel { periph: DMAC, index: 8, id: DmacCh8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh8Id {}
pub type DmacCh8 = Channel<DmacCh8Id, DmacId>;

pub const DMAC_CH9: Channel<DmacCh9Id, DmacId> = Channel { periph: DMAC, index: 9, id: DmacCh9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh9Id {}
pub type DmacCh9 = Channel<DmacCh9Id, DmacId>;

pub const DMAC_CH10: Channel<DmacCh10Id, DmacId> = Channel { periph: DMAC, index: 10, id: DmacCh10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh10Id {}
pub type DmacCh10 = Channel<DmacCh10Id, DmacId>;

pub const DMAC_CH11: Channel<DmacCh11Id, DmacId> = Channel { periph: DMAC, index: 11, id: DmacCh11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct DmacCh11Id {}
pub type DmacCh11 = Channel<DmacCh11Id, DmacId>;

pub trait IrqDma<T> {
   fn irq_dma(&self) -> super::irq::Irq<T>;
}

pub trait RegisterDmaHandler {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleDma {
   fn handle_dma(&self);
}

impl IrqDma<super::irq::DmacId> for Dmac {
   fn irq_dma(&self) -> super::irq::IrqDmac { super::irq::IRQ_DMAC }
}

impl RegisterDmaHandler for Dmac {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(6, Some(wrapper::<F>));
       super::irq::IrqGuard::new(6)
   }
}

