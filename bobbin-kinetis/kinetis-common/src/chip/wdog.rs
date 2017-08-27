#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "WDOG", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("WDOG"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "STCTRLH", offset: 0, size: Some(16), access: Some(ReadWrite), reset_value: Some(467), reset_mask: Some(65535), description: Some("Watchdog Status and Control Register High"), fields: [Field { name: "WDOGEN", bit_offset: 0, bit_width: 1, access: Some(ReadWrite), description: Some("Enables or disables the WDOG\\\'s operation"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("WDOG is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("WDOG is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CLKSRC", bit_offset: 1, bit_width: 1, access: Some(ReadWrite), description: Some("Selects clock source for the WDOG timer and other internal timing operations."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("WDOG clock sourced from LPO .") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("WDOG clock sourced from alternate clock source.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "IRQRSTEN", bit_offset: 2, bit_width: 1, access: Some(ReadWrite), description: Some("Used to enable the debug breadcrumbs feature"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("WDOG time-out generates reset only.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("WDOG time-out initially generates an interrupt. After WCT, it generates a reset.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WINEN", bit_offset: 3, bit_width: 1, access: Some(ReadWrite), description: Some("Enables Windowing mode."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Windowing mode is disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Windowing mode is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ALLOWUPDATE", bit_offset: 4, bit_width: 1, access: Some(ReadWrite), description: Some("Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No further updates allowed to WDOG write-once registers.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("WDOG write-once registers can be unlocked for updating.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DBGEN", bit_offset: 5, bit_width: 1, access: Some(ReadWrite), description: Some("Enables or disables WDOG in Debug mode."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("WDOG is disabled in CPU Debug mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("WDOG is enabled in CPU Debug mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "STOPEN", bit_offset: 6, bit_width: 1, access: Some(ReadWrite), description: Some("Enables or disables WDOG in Stop mode."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("WDOG is disabled in CPU Stop mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("WDOG is enabled in CPU Stop mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "WAITEN", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Enables or disables WDOG in Wait mode."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("WDOG is disabled in CPU Wait mode.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("WDOG is enabled in CPU Wait mode.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TESTWDOG", bit_offset: 10, bit_width: 1, access: Some(ReadWrite), description: Some("Puts the watchdog in the functional test mode"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "TESTSEL", bit_offset: 11, bit_width: 1, access: Some(ReadWrite), description: Some("Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Quick test. The timer runs in normal operation. You can load a small time-out value to do a quick test.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Byte test. Puts the timer in the byte test mode where individual bytes of the timer are enabled for operation and are compared for time-out against the corresponding byte of the programmed time-out value. Select the byte through BYTESEL[1:0] for testing.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BYTESEL", bit_offset: 12, bit_width: 2, access: Some(ReadWrite), description: Some("This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("Byte 0 selected") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Byte 1 selected") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Byte 2 selected") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Byte 3 selected") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DISTESTWDOG", bit_offset: 14, bit_width: 1, access: Some(ReadWrite), description: Some("Allows the WDOG\\\'s functional test mode to be disabled permanently"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("WDOG functional test mode is not disabled.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("WDOG functional test mode is disabled permanently until reset.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "STCTRLL", offset: 2, size: Some(16), access: Some(ReadWrite), reset_value: Some(1), reset_mask: Some(65535), description: Some("Watchdog Status and Control Register Low"), fields: [Field { name: "INTFLG", bit_offset: 15, bit_width: 1, access: Some(ReadWrite), description: Some("Interrupt flag"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "TOVALH", offset: 4, size: Some(16), access: Some(ReadWrite), reset_value: Some(76), reset_mask: Some(65535), description: Some("Watchdog Time-out Value Register High"), fields: [Field { name: "TOVALHIGH", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "TOVALL", offset: 6, size: Some(16), access: Some(ReadWrite), reset_value: Some(19276), reset_mask: Some(65535), description: Some("Watchdog Time-out Value Register Low"), fields: [Field { name: "TOVALLOW", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "WINH", offset: 8, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(65535), description: Some("Watchdog Window Register High"), fields: [Field { name: "WINHIGH", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "WINL", offset: 10, size: Some(16), access: Some(ReadWrite), reset_value: Some(16), reset_mask: Some(65535), description: Some("Watchdog Window Register Low"), fields: [Field { name: "WINLOW", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "REFRESH", offset: 12, size: Some(16), access: Some(ReadWrite), reset_value: Some(46208), reset_mask: Some(65535), description: Some("Watchdog Refresh register"), fields: [Field { name: "WDOGREFRESH", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Watchdog refresh register"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "UNLOCK", offset: 14, size: Some(16), access: Some(ReadWrite), reset_value: Some(55592), reset_mask: Some(65535), description: Some("Watchdog Unlock register"), fields: [Field { name: "WDOGUNLOCK", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "TMROUTH", offset: 16, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(65535), description: Some("Watchdog Timer Output Register High"), fields: [Field { name: "TIMEROUTHIGH", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Shows the value of the upper 16 bits of the watchdog timer."), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "TMROUTL", offset: 18, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(65535), description: Some("Watchdog Timer Output Register Low"), fields: [Field { name: "TIMEROUTLOW", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Shows the value of the lower 16 bits of the watchdog timer."), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "RSTCNT", offset: 20, size: Some(16), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(65535), description: Some("Watchdog Reset Count register"), fields: [Field { name: "RSTCNT", bit_offset: 0, bit_width: 16, access: Some(ReadWrite), description: Some("Counts the number of times the watchdog resets the system"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }, Register { name: "PRESC", offset: 22, size: Some(16), access: Some(ReadWrite), reset_value: Some(1024), reset_mask: Some(65535), description: Some("Watchdog Prescaler register"), fields: [Field { name: "PRESCVAL", bit_offset: 8, bit_width: 3, access: Some(ReadWrite), description: Some("3-bit prescaler for the watchdog clock source"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: None, dim_increment: None, dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: false, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WDOG Peripheral"]
pub struct WdogPeriph(pub usize); 


impl WdogPeriph {
#[doc="Get the *const pointer for the STCTRLH register."]
   #[inline] pub fn stctrlh_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x0) as *const u16
   }
#[doc="Get the *mut pointer for the STCTRLH register."]
   #[inline] pub fn stctrlh_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x0) as *mut u16
   }
#[doc="Read the STCTRLH register."]
   #[inline] pub fn stctrlh(&self) -> Stctrlh { 
      unsafe {
         Stctrlh(::core::ptr::read_volatile((self.0 + 0x0) as *const u16))
      }
   }
#[doc="Write the STCTRLH register."]
   #[inline] pub fn set_stctrlh<F: FnOnce(Stctrlh) -> Stctrlh>(&self, f: F) -> &Self {
      let value = f(Stctrlh(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the STCTRLH register."]
   #[inline] pub fn with_stctrlh<F: FnOnce(Stctrlh) -> Stctrlh>(&self, f: F) -> &Self {
      let tmp = self.stctrlh();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STCTRLL register."]
   #[inline] pub fn stctrll_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x2) as *const u16
   }
#[doc="Get the *mut pointer for the STCTRLL register."]
   #[inline] pub fn stctrll_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x2) as *mut u16
   }
#[doc="Read the STCTRLL register."]
   #[inline] pub fn stctrll(&self) -> Stctrll { 
      unsafe {
         Stctrll(::core::ptr::read_volatile((self.0 + 0x2) as *const u16))
      }
   }
#[doc="Write the STCTRLL register."]
   #[inline] pub fn set_stctrll<F: FnOnce(Stctrll) -> Stctrll>(&self, f: F) -> &Self {
      let value = f(Stctrll(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the STCTRLL register."]
   #[inline] pub fn with_stctrll<F: FnOnce(Stctrll) -> Stctrll>(&self, f: F) -> &Self {
      let tmp = self.stctrll();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TOVALH register."]
   #[inline] pub fn tovalh_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x4) as *const u16
   }
#[doc="Get the *mut pointer for the TOVALH register."]
   #[inline] pub fn tovalh_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x4) as *mut u16
   }
#[doc="Read the TOVALH register."]
   #[inline] pub fn tovalh(&self) -> Tovalh { 
      unsafe {
         Tovalh(::core::ptr::read_volatile((self.0 + 0x4) as *const u16))
      }
   }
#[doc="Write the TOVALH register."]
   #[inline] pub fn set_tovalh<F: FnOnce(Tovalh) -> Tovalh>(&self, f: F) -> &Self {
      let value = f(Tovalh(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TOVALH register."]
   #[inline] pub fn with_tovalh<F: FnOnce(Tovalh) -> Tovalh>(&self, f: F) -> &Self {
      let tmp = self.tovalh();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TOVALL register."]
   #[inline] pub fn tovall_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x6) as *const u16
   }
#[doc="Get the *mut pointer for the TOVALL register."]
   #[inline] pub fn tovall_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x6) as *mut u16
   }
#[doc="Read the TOVALL register."]
   #[inline] pub fn tovall(&self) -> Tovall { 
      unsafe {
         Tovall(::core::ptr::read_volatile((self.0 + 0x6) as *const u16))
      }
   }
#[doc="Write the TOVALL register."]
   #[inline] pub fn set_tovall<F: FnOnce(Tovall) -> Tovall>(&self, f: F) -> &Self {
      let value = f(Tovall(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TOVALL register."]
   #[inline] pub fn with_tovall<F: FnOnce(Tovall) -> Tovall>(&self, f: F) -> &Self {
      let tmp = self.tovall();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x6) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WINH register."]
   #[inline] pub fn winh_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x8) as *const u16
   }
#[doc="Get the *mut pointer for the WINH register."]
   #[inline] pub fn winh_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x8) as *mut u16
   }
#[doc="Read the WINH register."]
   #[inline] pub fn winh(&self) -> Winh { 
      unsafe {
         Winh(::core::ptr::read_volatile((self.0 + 0x8) as *const u16))
      }
   }
#[doc="Write the WINH register."]
   #[inline] pub fn set_winh<F: FnOnce(Winh) -> Winh>(&self, f: F) -> &Self {
      let value = f(Winh(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the WINH register."]
   #[inline] pub fn with_winh<F: FnOnce(Winh) -> Winh>(&self, f: F) -> &Self {
      let tmp = self.winh();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x8) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the WINL register."]
   #[inline] pub fn winl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0xa) as *const u16
   }
#[doc="Get the *mut pointer for the WINL register."]
   #[inline] pub fn winl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0xa) as *mut u16
   }
#[doc="Read the WINL register."]
   #[inline] pub fn winl(&self) -> Winl { 
      unsafe {
         Winl(::core::ptr::read_volatile((self.0 + 0xa) as *const u16))
      }
   }
#[doc="Write the WINL register."]
   #[inline] pub fn set_winl<F: FnOnce(Winl) -> Winl>(&self, f: F) -> &Self {
      let value = f(Winl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the WINL register."]
   #[inline] pub fn with_winl<F: FnOnce(Winl) -> Winl>(&self, f: F) -> &Self {
      let tmp = self.winl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the REFRESH register."]
   #[inline] pub fn refresh_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0xc) as *const u16
   }
#[doc="Get the *mut pointer for the REFRESH register."]
   #[inline] pub fn refresh_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0xc) as *mut u16
   }
#[doc="Read the REFRESH register."]
   #[inline] pub fn refresh(&self) -> Refresh { 
      unsafe {
         Refresh(::core::ptr::read_volatile((self.0 + 0xc) as *const u16))
      }
   }
#[doc="Write the REFRESH register."]
   #[inline] pub fn set_refresh<F: FnOnce(Refresh) -> Refresh>(&self, f: F) -> &Self {
      let value = f(Refresh(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the REFRESH register."]
   #[inline] pub fn with_refresh<F: FnOnce(Refresh) -> Refresh>(&self, f: F) -> &Self {
      let tmp = self.refresh();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xc) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the UNLOCK register."]
   #[inline] pub fn unlock_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0xe) as *const u16
   }
#[doc="Get the *mut pointer for the UNLOCK register."]
   #[inline] pub fn unlock_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0xe) as *mut u16
   }
#[doc="Read the UNLOCK register."]
   #[inline] pub fn unlock(&self) -> Unlock { 
      unsafe {
         Unlock(::core::ptr::read_volatile((self.0 + 0xe) as *const u16))
      }
   }
#[doc="Write the UNLOCK register."]
   #[inline] pub fn set_unlock<F: FnOnce(Unlock) -> Unlock>(&self, f: F) -> &Self {
      let value = f(Unlock(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the UNLOCK register."]
   #[inline] pub fn with_unlock<F: FnOnce(Unlock) -> Unlock>(&self, f: F) -> &Self {
      let tmp = self.unlock();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xe) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TMROUTH register."]
   #[inline] pub fn tmrouth_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x10) as *const u16
   }
#[doc="Get the *mut pointer for the TMROUTH register."]
   #[inline] pub fn tmrouth_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x10) as *mut u16
   }
#[doc="Read the TMROUTH register."]
   #[inline] pub fn tmrouth(&self) -> Tmrouth { 
      unsafe {
         Tmrouth(::core::ptr::read_volatile((self.0 + 0x10) as *const u16))
      }
   }
#[doc="Write the TMROUTH register."]
   #[inline] pub fn set_tmrouth<F: FnOnce(Tmrouth) -> Tmrouth>(&self, f: F) -> &Self {
      let value = f(Tmrouth(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TMROUTH register."]
   #[inline] pub fn with_tmrouth<F: FnOnce(Tmrouth) -> Tmrouth>(&self, f: F) -> &Self {
      let tmp = self.tmrouth();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TMROUTL register."]
   #[inline] pub fn tmroutl_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x12) as *const u16
   }
#[doc="Get the *mut pointer for the TMROUTL register."]
   #[inline] pub fn tmroutl_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x12) as *mut u16
   }
#[doc="Read the TMROUTL register."]
   #[inline] pub fn tmroutl(&self) -> Tmroutl { 
      unsafe {
         Tmroutl(::core::ptr::read_volatile((self.0 + 0x12) as *const u16))
      }
   }
#[doc="Write the TMROUTL register."]
   #[inline] pub fn set_tmroutl<F: FnOnce(Tmroutl) -> Tmroutl>(&self, f: F) -> &Self {
      let value = f(Tmroutl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x12) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the TMROUTL register."]
   #[inline] pub fn with_tmroutl<F: FnOnce(Tmroutl) -> Tmroutl>(&self, f: F) -> &Self {
      let tmp = self.tmroutl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x12) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RSTCNT register."]
   #[inline] pub fn rstcnt_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x14) as *const u16
   }
#[doc="Get the *mut pointer for the RSTCNT register."]
   #[inline] pub fn rstcnt_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x14) as *mut u16
   }
#[doc="Read the RSTCNT register."]
   #[inline] pub fn rstcnt(&self) -> Rstcnt { 
      unsafe {
         Rstcnt(::core::ptr::read_volatile((self.0 + 0x14) as *const u16))
      }
   }
#[doc="Write the RSTCNT register."]
   #[inline] pub fn set_rstcnt<F: FnOnce(Rstcnt) -> Rstcnt>(&self, f: F) -> &Self {
      let value = f(Rstcnt(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the RSTCNT register."]
   #[inline] pub fn with_rstcnt<F: FnOnce(Rstcnt) -> Rstcnt>(&self, f: F) -> &Self {
      let tmp = self.rstcnt();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x14) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRESC register."]
   #[inline] pub fn presc_ptr(&self) -> *const u16 { 
      ((self.0 as usize) + 0x16) as *const u16
   }
#[doc="Get the *mut pointer for the PRESC register."]
   #[inline] pub fn presc_mut(&self) -> *mut u16 { 
      ((self.0 as usize) + 0x16) as *mut u16
   }
#[doc="Read the PRESC register."]
   #[inline] pub fn presc(&self) -> Presc { 
      unsafe {
         Presc(::core::ptr::read_volatile((self.0 + 0x16) as *const u16))
      }
   }
#[doc="Write the PRESC register."]
   #[inline] pub fn set_presc<F: FnOnce(Presc) -> Presc>(&self, f: F) -> &Self {
      let value = f(Presc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x16) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the PRESC register."]
   #[inline] pub fn with_presc<F: FnOnce(Presc) -> Presc>(&self, f: F) -> &Self {
      let tmp = self.presc();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x16) as *mut u16, value.0);
      }
      self
   }

}

#[doc="Watchdog Status and Control Register High"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stctrlh(pub u16);
impl Stctrlh {
#[doc="Enables or disables the WDOG\'s operation"]
   #[inline] pub fn wdogen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Enables or disables the WDOG\'s operation"]
   #[inline] pub fn set_wdogen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Selects clock source for the WDOG timer and other internal timing operations."]
   #[inline] pub fn clksrc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Selects clock source for the WDOG timer and other internal timing operations."]
   #[inline] pub fn set_clksrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Used to enable the debug breadcrumbs feature"]
   #[inline] pub fn irqrsten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Used to enable the debug breadcrumbs feature"]
   #[inline] pub fn set_irqrsten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Enables Windowing mode."]
   #[inline] pub fn winen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Enables Windowing mode."]
   #[inline] pub fn set_winen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
   #[inline] pub fn allowupdate(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
   #[inline] pub fn set_allowupdate<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Enables or disables WDOG in Debug mode."]
   #[inline] pub fn dbgen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Enables or disables WDOG in Debug mode."]
   #[inline] pub fn set_dbgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Enables or disables WDOG in Stop mode."]
   #[inline] pub fn stopen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Enables or disables WDOG in Stop mode."]
   #[inline] pub fn set_stopen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Enables or disables WDOG in Wait mode."]
   #[inline] pub fn waiten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Enables or disables WDOG in Wait mode."]
   #[inline] pub fn set_waiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Puts the watchdog in the functional test mode"]
   #[inline] pub fn testwdog(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Puts the watchdog in the functional test mode"]
   #[inline] pub fn set_testwdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
   #[inline] pub fn testsel(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
   #[inline] pub fn set_testsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
   #[inline] pub fn bytesel(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
   #[inline] pub fn set_bytesel<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Allows the WDOG\'s functional test mode to be disabled permanently"]
   #[inline] pub fn distestwdog(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Allows the WDOG\'s functional test mode to be disabled permanently"]
   #[inline] pub fn set_distestwdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Stctrlh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stctrlh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdogen() != 0 { try!(write!(f, " wdogen"))}
      if self.clksrc() != 0 { try!(write!(f, " clksrc"))}
      if self.irqrsten() != 0 { try!(write!(f, " irqrsten"))}
      if self.winen() != 0 { try!(write!(f, " winen"))}
      if self.allowupdate() != 0 { try!(write!(f, " allowupdate"))}
      if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
      if self.stopen() != 0 { try!(write!(f, " stopen"))}
      if self.waiten() != 0 { try!(write!(f, " waiten"))}
      if self.testwdog() != 0 { try!(write!(f, " testwdog"))}
      if self.testsel() != 0 { try!(write!(f, " testsel"))}
      if self.bytesel() != 0 { try!(write!(f, " bytesel=0x{:x}", self.bytesel()))}
      if self.distestwdog() != 0 { try!(write!(f, " distestwdog"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Status and Control Register Low"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stctrll(pub u16);
impl Stctrll {
#[doc="Interrupt flag"]
   #[inline] pub fn intflg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Interrupt flag"]
   #[inline] pub fn set_intflg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for Stctrll {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stctrll {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intflg() != 0 { try!(write!(f, " intflg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Time-out Value Register High"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tovalh(pub u16);
impl Tovalh {
#[doc="Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
   #[inline] pub fn tovalhigh(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
   #[inline] pub fn set_tovalhigh<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tovalh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tovalh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tovalhigh() != 0 { try!(write!(f, " tovalhigh=0x{:x}", self.tovalhigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Time-out Value Register Low"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tovall(pub u16);
impl Tovall {
#[doc="Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
   #[inline] pub fn tovallow(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
   #[inline] pub fn set_tovallow<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tovall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tovall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tovallow() != 0 { try!(write!(f, " tovallow=0x{:x}", self.tovallow()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Window Register High"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Winh(pub u16);
impl Winh {
#[doc="Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
   #[inline] pub fn winhigh(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
   #[inline] pub fn set_winhigh<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Winh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Winh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.winhigh() != 0 { try!(write!(f, " winhigh=0x{:x}", self.winhigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Window Register Low"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Winl(pub u16);
impl Winl {
#[doc="Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
   #[inline] pub fn winlow(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
   #[inline] pub fn set_winlow<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Winl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Winl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.winlow() != 0 { try!(write!(f, " winlow=0x{:x}", self.winlow()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Refresh register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Refresh(pub u16);
impl Refresh {
#[doc="Watchdog refresh register"]
   #[inline] pub fn wdogrefresh(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Watchdog refresh register"]
   #[inline] pub fn set_wdogrefresh<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Refresh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Refresh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdogrefresh() != 0 { try!(write!(f, " wdogrefresh=0x{:x}", self.wdogrefresh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Unlock register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Unlock(pub u16);
impl Unlock {
#[doc="Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
   #[inline] pub fn wdogunlock(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
   #[inline] pub fn set_wdogunlock<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Unlock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Unlock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdogunlock() != 0 { try!(write!(f, " wdogunlock=0x{:x}", self.wdogunlock()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Output Register High"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tmrouth(pub u16);
impl Tmrouth {
#[doc="Shows the value of the upper 16 bits of the watchdog timer."]
   #[inline] pub fn timerouthigh(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Shows the value of the upper 16 bits of the watchdog timer."]
   #[inline] pub fn set_timerouthigh<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tmrouth {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tmrouth {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.timerouthigh() != 0 { try!(write!(f, " timerouthigh=0x{:x}", self.timerouthigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Output Register Low"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tmroutl(pub u16);
impl Tmroutl {
#[doc="Shows the value of the lower 16 bits of the watchdog timer."]
   #[inline] pub fn timeroutlow(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Shows the value of the lower 16 bits of the watchdog timer."]
   #[inline] pub fn set_timeroutlow<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tmroutl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tmroutl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.timeroutlow() != 0 { try!(write!(f, " timeroutlow=0x{:x}", self.timeroutlow()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Reset Count register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rstcnt(pub u16);
impl Rstcnt {
#[doc="Counts the number of times the watchdog resets the system"]
   #[inline] pub fn rstcnt(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Counts the number of times the watchdog resets the system"]
   #[inline] pub fn set_rstcnt<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rstcnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rstcnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rstcnt() != 0 { try!(write!(f, " rstcnt=0x{:x}", self.rstcnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Prescaler register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Presc(pub u16);
impl Presc {
#[doc="3-bit prescaler for the watchdog clock source"]
   #[inline] pub fn prescval(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="3-bit prescaler for the watchdog clock source"]
   #[inline] pub fn set_prescval<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Presc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Presc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prescval() != 0 { try!(write!(f, " prescval=0x{:x}", self.prescval()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
