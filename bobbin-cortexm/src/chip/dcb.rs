pub const DCB: Dcb = Dcb(0xe000edf0);

#[doc="Debug Core Block"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcb(pub u32);
impl Dcb {
#[doc="Get the *const pointer for the DHCSR register."]
  #[inline] pub fn dhcsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the DHCSR register."]
  #[inline] pub fn dhcsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the DHCSR register."]
  #[inline] pub fn dhcsr(&self) -> Dhcsr { 
     unsafe {
        Dhcsr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the DHCSR register."]
  #[inline] pub fn set_dhcsr(&self, value: Dhcsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DHCSR register."]
  #[inline] pub fn with_dhcsr<F: FnOnce(Dhcsr) -> Dhcsr>(&self, f: F) -> &Self {
     let tmp = self.dhcsr();
     self.set_dhcsr(f(tmp))
  }

#[doc="Get the *const pointer for the DCRSR register."]
  #[inline] pub fn dcrsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the DCRSR register."]
  #[inline] pub fn dcrsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the DCRSR register."]
  #[inline] pub fn dcrsr(&self) -> Dcrsr { 
     unsafe {
        Dcrsr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the DCRSR register."]
  #[inline] pub fn set_dcrsr(&self, value: Dcrsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DCRSR register."]
  #[inline] pub fn with_dcrsr<F: FnOnce(Dcrsr) -> Dcrsr>(&self, f: F) -> &Self {
     let tmp = self.dcrsr();
     self.set_dcrsr(f(tmp))
  }

#[doc="Get the *const pointer for the DERDR register."]
  #[inline] pub fn derdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the DERDR register."]
  #[inline] pub fn derdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the DERDR register."]
  #[inline] pub fn derdr(&self) -> Derdr { 
     unsafe {
        Derdr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the DERDR register."]
  #[inline] pub fn set_derdr(&self, value: Derdr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DERDR register."]
  #[inline] pub fn with_derdr<F: FnOnce(Derdr) -> Derdr>(&self, f: F) -> &Self {
     let tmp = self.derdr();
     self.set_derdr(f(tmp))
  }

#[doc="Get the *const pointer for the DEMCR register."]
  #[inline] pub fn demcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the DEMCR register."]
  #[inline] pub fn demcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the DEMCR register."]
  #[inline] pub fn demcr(&self) -> Demcr { 
     unsafe {
        Demcr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the DEMCR register."]
  #[inline] pub fn set_demcr(&self, value: Demcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DEMCR register."]
  #[inline] pub fn with_demcr<F: FnOnce(Demcr) -> Demcr>(&self, f: F) -> &Self {
     let tmp = self.demcr();
     self.set_demcr(f(tmp))
  }

}

#[doc="Debug Halting Control and Status Register"]
#[derive(PartialEq, Eq)]
pub struct Dhcsr(pub u32);
impl Dhcsr {
#[doc="Debug Key. 0xA05F must be written whenever this register is written. Reads back as status bits [25:16]. If not written as Key, the write operation is ignored and no bits are written into the register."]
  #[inline] pub fn dbgkey(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
#[doc="Debug Key. 0xA05F must be written whenever this register is written. Reads back as status bits [25:16]. If not written as Key, the write operation is ignored and no bits are written into the register."]
  #[inline] pub fn set_dbgkey(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still)."]
  #[inline] pub fn s_reset_st(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still)."]
  #[inline] pub fn set_s_reset_st(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch."]
  #[inline] pub fn s_retire_st(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch."]
  #[inline] pub fn set_s_retire_st(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Reads as one if the core is running (not halted) and a lockup condition is present."]
  #[inline] pub fn s_lockup(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Reads as one if the core is running (not halted) and a lockup condition is present."]
  #[inline] pub fn set_s_lockup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="ndicates that the core is sleeping (WFI, WFE or SLEEP-ON-EXIT). Must use C_HALT to gain control or wait for interrupt to wake-up."]
  #[inline] pub fn s_sleep(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="ndicates that the core is sleeping (WFI, WFE or SLEEP-ON-EXIT). Must use C_HALT to gain control or wait for interrupt to wake-up."]
  #[inline] pub fn set_s_sleep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="The core is in debug state when S_HALT is set."]
  #[inline] pub fn s_halt(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="The core is in debug state when S_HALT is set."]
  #[inline] pub fn set_s_halt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete."]
  #[inline] pub fn s_regrdy(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete."]
  #[inline] pub fn set_s_regrdy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 C_HALT = 1 The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE can detect core stalls on load/store operations."]
  #[inline] pub fn c_snapstall(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 C_HALT = 1 The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE can detect core stalls on load/store operations."]
  #[inline] pub fn set_c_snapstall(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Mask interrupts when stepping or running in halted debug. Does not affect NMI, which is not maskable. Must only be modified when the processor is halted (S_HALT == 1)."]
  #[inline] pub fn c_maskints(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Mask interrupts when stepping or running in halted debug. Does not affect NMI, which is not maskable. Must only be modified when the processor is halted (S_HALT == 1)."]
  #[inline] pub fn set_c_maskints(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1)."]
  #[inline] pub fn c_step(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1)."]
  #[inline] pub fn set_c_step(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset. This bit can only be written if C_DEBUGEN is 1, otherwise it is ignored. When setting this bit to 1, C_DEBUGEN must also be written to 1 in the same value (value[1:0] is 2’b11). The core can halt itself, but only if C_DEBUGEN is already 1 and only if it writes with b11)."]
  #[inline] pub fn c_halt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset. This bit can only be written if C_DEBUGEN is 1, otherwise it is ignored. When setting this bit to 1, C_DEBUGEN must also be written to 1 in the same value (value[1:0] is 2’b11). The core can halt itself, but only if C_DEBUGEN is already 1 and only if it writes with b11)."]
  #[inline] pub fn set_c_halt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it.
The core must write a 1 to it when writing C_HALT to halt itself."]
  #[inline] pub fn c_debugen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it.
The core must write a 1 to it when writing C_HALT to halt itself."]
  #[inline] pub fn set_c_debugen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dhcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgkey() != 0 { try!(write!(f, " dbgkey=0x{:x}", self.dbgkey()))}
      if self.s_reset_st() != 0 { try!(write!(f, " s_reset_st"))}
      if self.s_retire_st() != 0 { try!(write!(f, " s_retire_st"))}
      if self.s_lockup() != 0 { try!(write!(f, " s_lockup"))}
      if self.s_sleep() != 0 { try!(write!(f, " s_sleep"))}
      if self.s_halt() != 0 { try!(write!(f, " s_halt"))}
      if self.s_regrdy() != 0 { try!(write!(f, " s_regrdy"))}
      if self.c_snapstall() != 0 { try!(write!(f, " c_snapstall"))}
      if self.c_maskints() != 0 { try!(write!(f, " c_maskints"))}
      if self.c_step() != 0 { try!(write!(f, " c_step"))}
      if self.c_halt() != 0 { try!(write!(f, " c_halt"))}
      if self.c_debugen() != 0 { try!(write!(f, " c_debugen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug Core Register Selector Register"]
#[derive(PartialEq, Eq)]
pub struct Dcrsr(pub u32);
impl Dcrsr {
#[doc="Write = 1, Read = 0"]
  #[inline] pub fn regwnr(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Write = 1, Read = 0"]
  #[inline] pub fn set_regwnr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Register Select"]
  #[inline] pub fn regsel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Register Select"]
  #[inline] pub fn set_regsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dcrsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcrsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.regwnr() != 0 { try!(write!(f, " regwnr"))}
      if self.regsel() != 0 { try!(write!(f, " regsel=0x{:x}", self.regsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug Core Register Data Register"]
#[derive(PartialEq, Eq)]
pub struct Derdr(pub u32);
impl Derdr {
#[doc="This is the data value written to the register selected by the Debug Register Selector Register."]
  #[inline] pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="This is the data value written to the register selected by the Debug Register Selector Register."]
  #[inline] pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Derdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Derdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug Exception and Monitor Control Register"]
#[derive(PartialEq, Eq)]
pub struct Demcr(pub u32);
impl Demcr {
#[doc="This bit must be set to 1 to enable use of the trace and debug blocks."]
  #[inline] pub fn trcena(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="This bit must be set to 1 to enable use of the trace and debug blocks."]
  #[inline] pub fn set_trcena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="This enables the monitor to identify how it wakes up: 1 = woken up by MON_PEND, 0 = woken up by debug exception."]
  #[inline] pub fn mon_req(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="This enables the monitor to identify how it wakes up: 1 = woken up by MON_PEND, 0 = woken up by debug exception."]
  #[inline] pub fn set_mon_req(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
  #[inline] pub fn mon_step(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
  #[inline] pub fn set_mon_step(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Pend the monitor to activate when priority permits."]
  #[inline] pub fn mon_pend(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Pend the monitor to activate when priority permits."]
  #[inline] pub fn set_mon_pend(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Enable the debug monitor."]
  #[inline] pub fn mon_en(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Enable the debug monitor."]
  #[inline] pub fn set_mon_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Debug trap on Hard Fault."]
  #[inline] pub fn vc_harderr(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Debug trap on Hard Fault."]
  #[inline] pub fn set_vc_harderr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Debug Trap on interrupt/exception service errors. These are a subset of other faults and catches before BUSERR or HARDERR."]
  #[inline] pub fn vc_interr(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Debug Trap on interrupt/exception service errors. These are a subset of other faults and catches before BUSERR or HARDERR."]
  #[inline] pub fn set_vc_interr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Debug Trap on normal Bus error."]
  #[inline] pub fn vc_buserr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Debug Trap on normal Bus error."]
  #[inline] pub fn set_vc_buserr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Debug trap on Usage Fault state errors."]
  #[inline] pub fn vc_staterr(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Debug trap on Usage Fault state errors."]
  #[inline] pub fn set_vc_staterr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Debug trap on Usage Fault enabled checking errors."]
  #[inline] pub fn vc_chkerr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Debug trap on Usage Fault enabled checking errors."]
  #[inline] pub fn set_vc_chkerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Debug trap on Usage Fault access to Coprocessor which is not present or marked as not present in CAR register."]
  #[inline] pub fn vc_nocperr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Debug trap on Usage Fault access to Coprocessor which is not present or marked as not present in CAR register."]
  #[inline] pub fn set_vc_nocperr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Debug trap on Memory Management faults."]
  #[inline] pub fn vc_mmerr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Debug trap on Memory Management faults."]
  #[inline] pub fn set_vc_mmerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Reset Vector Catch. Halt running system if Core reset occurs."]
  #[inline] pub fn vc_corereset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Reset Vector Catch. Halt running system if Core reset occurs."]
  #[inline] pub fn set_vc_corereset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Demcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Demcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.trcena() != 0 { try!(write!(f, " trcena"))}
      if self.mon_req() != 0 { try!(write!(f, " mon_req"))}
      if self.mon_step() != 0 { try!(write!(f, " mon_step"))}
      if self.mon_pend() != 0 { try!(write!(f, " mon_pend"))}
      if self.mon_en() != 0 { try!(write!(f, " mon_en"))}
      if self.vc_harderr() != 0 { try!(write!(f, " vc_harderr"))}
      if self.vc_interr() != 0 { try!(write!(f, " vc_interr"))}
      if self.vc_buserr() != 0 { try!(write!(f, " vc_buserr"))}
      if self.vc_staterr() != 0 { try!(write!(f, " vc_staterr"))}
      if self.vc_chkerr() != 0 { try!(write!(f, " vc_chkerr"))}
      if self.vc_nocperr() != 0 { try!(write!(f, " vc_nocperr"))}
      if self.vc_mmerr() != 0 { try!(write!(f, " vc_mmerr"))}
      if self.vc_corereset() != 0 { try!(write!(f, " vc_corereset"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

