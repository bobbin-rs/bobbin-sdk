
periph!( DCB, Dcb, DCB_PERIPH, DcbPeriph, DCB_OWNED, DCB_REF_COUNT, 0xe000edf0, 0x00, 0x05);


#[doc="Debug Core Block"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DcbPeriph(pub usize);
impl DcbPeriph {
    #[doc="Get the DHCSR Register."]
    #[inline] pub fn dhcsr_reg(&self) -> ::bobbin_mcu::register::Register<Dhcsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhcsr, 0x0)
    }

    #[doc="Get the *mut pointer for the DHCSR register."]
    #[inline] pub fn dhcsr_mut(&self) -> *mut Dhcsr { 
        self.dhcsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHCSR register."]
    #[inline] pub fn dhcsr_ptr(&self) -> *const Dhcsr { 
        self.dhcsr_reg().ptr()
    }

    #[doc="Read the DHCSR register."]
    #[inline] pub fn dhcsr(&self) -> Dhcsr { 
        self.dhcsr_reg().read()
    }

    #[doc="Write the DHCSR register."]
    #[inline] pub fn write_dhcsr(&self, value: Dhcsr) -> &Self { 
        self.dhcsr_reg().write(value);
        self
    }

    #[doc="Set the DHCSR register."]
    #[inline] pub fn set_dhcsr<F: FnOnce(Dhcsr) -> Dhcsr>(&self, f: F) -> &Self {
        self.dhcsr_reg().set(f);
        self
    }

    #[doc="Modify the DHCSR register."]
    #[inline] pub fn with_dhcsr<F: FnOnce(Dhcsr) -> Dhcsr>(&self, f: F) -> &Self {
        self.dhcsr_reg().with(f);
        self
    }

    #[doc="Get the DCRSR Register."]
    #[inline] pub fn dcrsr_reg(&self) -> ::bobbin_mcu::register::Register<Dcrsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dcrsr, 0x4)
    }

    #[doc="Get the *mut pointer for the DCRSR register."]
    #[inline] pub fn dcrsr_mut(&self) -> *mut Dcrsr { 
        self.dcrsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DCRSR register."]
    #[inline] pub fn dcrsr_ptr(&self) -> *const Dcrsr { 
        self.dcrsr_reg().ptr()
    }

    #[doc="Read the DCRSR register."]
    #[inline] pub fn dcrsr(&self) -> Dcrsr { 
        self.dcrsr_reg().read()
    }

    #[doc="Write the DCRSR register."]
    #[inline] pub fn write_dcrsr(&self, value: Dcrsr) -> &Self { 
        self.dcrsr_reg().write(value);
        self
    }

    #[doc="Set the DCRSR register."]
    #[inline] pub fn set_dcrsr<F: FnOnce(Dcrsr) -> Dcrsr>(&self, f: F) -> &Self {
        self.dcrsr_reg().set(f);
        self
    }

    #[doc="Modify the DCRSR register."]
    #[inline] pub fn with_dcrsr<F: FnOnce(Dcrsr) -> Dcrsr>(&self, f: F) -> &Self {
        self.dcrsr_reg().with(f);
        self
    }

    #[doc="Get the DERDR Register."]
    #[inline] pub fn derdr_reg(&self) -> ::bobbin_mcu::register::Register<Derdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Derdr, 0x8)
    }

    #[doc="Get the *mut pointer for the DERDR register."]
    #[inline] pub fn derdr_mut(&self) -> *mut Derdr { 
        self.derdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DERDR register."]
    #[inline] pub fn derdr_ptr(&self) -> *const Derdr { 
        self.derdr_reg().ptr()
    }

    #[doc="Read the DERDR register."]
    #[inline] pub fn derdr(&self) -> Derdr { 
        self.derdr_reg().read()
    }

    #[doc="Write the DERDR register."]
    #[inline] pub fn write_derdr(&self, value: Derdr) -> &Self { 
        self.derdr_reg().write(value);
        self
    }

    #[doc="Set the DERDR register."]
    #[inline] pub fn set_derdr<F: FnOnce(Derdr) -> Derdr>(&self, f: F) -> &Self {
        self.derdr_reg().set(f);
        self
    }

    #[doc="Modify the DERDR register."]
    #[inline] pub fn with_derdr<F: FnOnce(Derdr) -> Derdr>(&self, f: F) -> &Self {
        self.derdr_reg().with(f);
        self
    }

    #[doc="Get the DEMCR Register."]
    #[inline] pub fn demcr_reg(&self) -> ::bobbin_mcu::register::Register<Demcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Demcr, 0xc)
    }

    #[doc="Get the *mut pointer for the DEMCR register."]
    #[inline] pub fn demcr_mut(&self) -> *mut Demcr { 
        self.demcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DEMCR register."]
    #[inline] pub fn demcr_ptr(&self) -> *const Demcr { 
        self.demcr_reg().ptr()
    }

    #[doc="Read the DEMCR register."]
    #[inline] pub fn demcr(&self) -> Demcr { 
        self.demcr_reg().read()
    }

    #[doc="Write the DEMCR register."]
    #[inline] pub fn write_demcr(&self, value: Demcr) -> &Self { 
        self.demcr_reg().write(value);
        self
    }

    #[doc="Set the DEMCR register."]
    #[inline] pub fn set_demcr<F: FnOnce(Demcr) -> Demcr>(&self, f: F) -> &Self {
        self.demcr_reg().set(f);
        self
    }

    #[doc="Modify the DEMCR register."]
    #[inline] pub fn with_demcr<F: FnOnce(Demcr) -> Demcr>(&self, f: F) -> &Self {
        self.demcr_reg().with(f);
        self
    }

}

#[doc="Debug Halting Control and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhcsr(pub u32);
impl Dhcsr {
    #[doc="Debug Key. 0xA05F must be written whenever this register is written. Reads back as status bits [25:16]. If not written as Key, the write operation is ignored and no bits are written into the register."]
    #[inline] pub fn dbgkey(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if DBGKEY != 0"]
    #[inline] pub fn test_dbgkey(&self) -> bool {
        self.dbgkey() != 0
    }

    #[doc="Sets the DBGKEY field."]
    #[inline] pub fn set_dbgkey<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still)."]
    #[inline] pub fn s_reset_st(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if S_RESET_ST != 0"]
    #[inline] pub fn test_s_reset_st(&self) -> bool {
        self.s_reset_st() != 0
    }

    #[doc="Sets the S_RESET_ST field."]
    #[inline] pub fn set_s_reset_st<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch."]
    #[inline] pub fn s_retire_st(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if S_RETIRE_ST != 0"]
    #[inline] pub fn test_s_retire_st(&self) -> bool {
        self.s_retire_st() != 0
    }

    #[doc="Sets the S_RETIRE_ST field."]
    #[inline] pub fn set_s_retire_st<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Reads as one if the core is running (not halted) and a lockup condition is present."]
    #[inline] pub fn s_lockup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if S_LOCKUP != 0"]
    #[inline] pub fn test_s_lockup(&self) -> bool {
        self.s_lockup() != 0
    }

    #[doc="Sets the S_LOCKUP field."]
    #[inline] pub fn set_s_lockup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ndicates that the core is sleeping (WFI, WFE or SLEEP-ON-EXIT). Must use C_HALT to gain control or wait for interrupt to wake-up."]
    #[inline] pub fn s_sleep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if S_SLEEP != 0"]
    #[inline] pub fn test_s_sleep(&self) -> bool {
        self.s_sleep() != 0
    }

    #[doc="Sets the S_SLEEP field."]
    #[inline] pub fn set_s_sleep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="The core is in debug state when S_HALT is set."]
    #[inline] pub fn s_halt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if S_HALT != 0"]
    #[inline] pub fn test_s_halt(&self) -> bool {
        self.s_halt() != 0
    }

    #[doc="Sets the S_HALT field."]
    #[inline] pub fn set_s_halt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete."]
    #[inline] pub fn s_regrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if S_REGRDY != 0"]
    #[inline] pub fn test_s_regrdy(&self) -> bool {
        self.s_regrdy() != 0
    }

    #[doc="Sets the S_REGRDY field."]
    #[inline] pub fn set_s_regrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 C_HALT = 1 The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE can detect core stalls on load/store operations."]
    #[inline] pub fn c_snapstall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if C_SNAPSTALL != 0"]
    #[inline] pub fn test_c_snapstall(&self) -> bool {
        self.c_snapstall() != 0
    }

    #[doc="Sets the C_SNAPSTALL field."]
    #[inline] pub fn set_c_snapstall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Mask interrupts when stepping or running in halted debug. Does not affect NMI, which is not maskable. Must only be modified when the processor is halted (S_HALT == 1)."]
    #[inline] pub fn c_maskints(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if C_MASKINTS != 0"]
    #[inline] pub fn test_c_maskints(&self) -> bool {
        self.c_maskints() != 0
    }

    #[doc="Sets the C_MASKINTS field."]
    #[inline] pub fn set_c_maskints<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1)."]
    #[inline] pub fn c_step(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if C_STEP != 0"]
    #[inline] pub fn test_c_step(&self) -> bool {
        self.c_step() != 0
    }

    #[doc="Sets the C_STEP field."]
    #[inline] pub fn set_c_step<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset. This bit can only be written if C_DEBUGEN is 1, otherwise it is ignored. When setting this bit to 1, C_DEBUGEN must also be written to 1 in the same value (value[1:0] is 2’b11). The core can halt itself, but only if C_DEBUGEN is already 1 and only if it writes with b11)."]
    #[inline] pub fn c_halt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if C_HALT != 0"]
    #[inline] pub fn test_c_halt(&self) -> bool {
        self.c_halt() != 0
    }

    #[doc="Sets the C_HALT field."]
    #[inline] pub fn set_c_halt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it.
The core must write a 1 to it when writing C_HALT to halt itself."]
    #[inline] pub fn c_debugen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if C_DEBUGEN != 0"]
    #[inline] pub fn test_c_debugen(&self) -> bool {
        self.c_debugen() != 0
    }

    #[doc="Sets the C_DEBUGEN field."]
    #[inline] pub fn set_c_debugen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhcsr {
    #[inline]
    fn from(other: u32) -> Self {
         Dhcsr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcrsr(pub u32);
impl Dcrsr {
    #[doc="Write = 1, Read = 0"]
    #[inline] pub fn regwnr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if REGWNR != 0"]
    #[inline] pub fn test_regwnr(&self) -> bool {
        self.regwnr() != 0
    }

    #[doc="Sets the REGWNR field."]
    #[inline] pub fn set_regwnr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Register Select"]
    #[inline] pub fn regsel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if REGSEL != 0"]
    #[inline] pub fn test_regsel(&self) -> bool {
        self.regsel() != 0
    }

    #[doc="Sets the REGSEL field."]
    #[inline] pub fn set_regsel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcrsr {
    #[inline]
    fn from(other: u32) -> Self {
         Dcrsr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Derdr(pub u32);
impl Derdr {
    #[doc="This is the data value written to the register selected by the Debug Register Selector Register."]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Derdr {
    #[inline]
    fn from(other: u32) -> Self {
         Derdr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Demcr(pub u32);
impl Demcr {
    #[doc="This bit must be set to 1 to enable use of the trace and debug blocks."]
    #[inline] pub fn trcena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if TRCENA != 0"]
    #[inline] pub fn test_trcena(&self) -> bool {
        self.trcena() != 0
    }

    #[doc="Sets the TRCENA field."]
    #[inline] pub fn set_trcena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="This enables the monitor to identify how it wakes up: 1 = woken up by MON_PEND, 0 = woken up by debug exception."]
    #[inline] pub fn mon_req(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MON_REQ != 0"]
    #[inline] pub fn test_mon_req(&self) -> bool {
        self.mon_req() != 0
    }

    #[doc="Sets the MON_REQ field."]
    #[inline] pub fn set_mon_req<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[inline] pub fn mon_step(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if MON_STEP != 0"]
    #[inline] pub fn test_mon_step(&self) -> bool {
        self.mon_step() != 0
    }

    #[doc="Sets the MON_STEP field."]
    #[inline] pub fn set_mon_step<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pend the monitor to activate when priority permits."]
    #[inline] pub fn mon_pend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if MON_PEND != 0"]
    #[inline] pub fn test_mon_pend(&self) -> bool {
        self.mon_pend() != 0
    }

    #[doc="Sets the MON_PEND field."]
    #[inline] pub fn set_mon_pend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Enable the debug monitor."]
    #[inline] pub fn mon_en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MON_EN != 0"]
    #[inline] pub fn test_mon_en(&self) -> bool {
        self.mon_en() != 0
    }

    #[doc="Sets the MON_EN field."]
    #[inline] pub fn set_mon_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Debug trap on Hard Fault."]
    #[inline] pub fn vc_harderr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VC_HARDERR != 0"]
    #[inline] pub fn test_vc_harderr(&self) -> bool {
        self.vc_harderr() != 0
    }

    #[doc="Sets the VC_HARDERR field."]
    #[inline] pub fn set_vc_harderr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Debug Trap on interrupt/exception service errors. These are a subset of other faults and catches before BUSERR or HARDERR."]
    #[inline] pub fn vc_interr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if VC_INTERR != 0"]
    #[inline] pub fn test_vc_interr(&self) -> bool {
        self.vc_interr() != 0
    }

    #[doc="Sets the VC_INTERR field."]
    #[inline] pub fn set_vc_interr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Debug Trap on normal Bus error."]
    #[inline] pub fn vc_buserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if VC_BUSERR != 0"]
    #[inline] pub fn test_vc_buserr(&self) -> bool {
        self.vc_buserr() != 0
    }

    #[doc="Sets the VC_BUSERR field."]
    #[inline] pub fn set_vc_buserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Debug trap on Usage Fault state errors."]
    #[inline] pub fn vc_staterr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if VC_STATERR != 0"]
    #[inline] pub fn test_vc_staterr(&self) -> bool {
        self.vc_staterr() != 0
    }

    #[doc="Sets the VC_STATERR field."]
    #[inline] pub fn set_vc_staterr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Debug trap on Usage Fault enabled checking errors."]
    #[inline] pub fn vc_chkerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if VC_CHKERR != 0"]
    #[inline] pub fn test_vc_chkerr(&self) -> bool {
        self.vc_chkerr() != 0
    }

    #[doc="Sets the VC_CHKERR field."]
    #[inline] pub fn set_vc_chkerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Debug trap on Usage Fault access to Coprocessor which is not present or marked as not present in CAR register."]
    #[inline] pub fn vc_nocperr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if VC_NOCPERR != 0"]
    #[inline] pub fn test_vc_nocperr(&self) -> bool {
        self.vc_nocperr() != 0
    }

    #[doc="Sets the VC_NOCPERR field."]
    #[inline] pub fn set_vc_nocperr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Debug trap on Memory Management faults."]
    #[inline] pub fn vc_mmerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if VC_MMERR != 0"]
    #[inline] pub fn test_vc_mmerr(&self) -> bool {
        self.vc_mmerr() != 0
    }

    #[doc="Sets the VC_MMERR field."]
    #[inline] pub fn set_vc_mmerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Reset Vector Catch. Halt running system if Core reset occurs."]
    #[inline] pub fn vc_corereset(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VC_CORERESET != 0"]
    #[inline] pub fn test_vc_corereset(&self) -> bool {
        self.vc_corereset() != 0
    }

    #[doc="Sets the VC_CORERESET field."]
    #[inline] pub fn set_vc_corereset<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Demcr {
    #[inline]
    fn from(other: u32) -> Self {
         Demcr(other)
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

