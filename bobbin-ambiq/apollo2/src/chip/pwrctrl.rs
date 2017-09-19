//! PWR Controller Register Bank
#[allow(unused_imports)] use bobbin_common::*;

periph!(PWRCTRL, Pwrctrl, 0x40021000);

#[doc="PWR Controller Register Bank"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pwrctrl(pub usize);
impl Pwrctrl {
    #[doc="Get the *mut pointer for the SUPPLYSRC register."]
    #[inline] pub fn supplysrc_mut(&self) -> *mut Supplysrc { 
        (self.0 + 0x0) as *mut Supplysrc
    }

    #[doc="Get the *const pointer for the SUPPLYSRC register."]
    #[inline] pub fn supplysrc_ptr(&self) -> *const Supplysrc { 
           self.supplysrc_mut()
    }

    #[doc="Read the SUPPLYSRC register."]
    #[inline] pub fn supplysrc(&self) -> Supplysrc { 
        unsafe {
            read_volatile(self.supplysrc_ptr())
        }
    }

    #[doc="Write the SUPPLYSRC register."]
    #[inline] pub fn set_supplysrc<F: FnOnce(Supplysrc) -> Supplysrc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.supplysrc_mut(), f(Supplysrc(0)));
        }
        self
    }

    #[doc="Modify the SUPPLYSRC register."]
    #[inline] pub fn with_supplysrc<F: FnOnce(Supplysrc) -> Supplysrc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.supplysrc_mut(), f(self.supplysrc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the POWERSTATUS register."]
    #[inline] pub fn powerstatus_mut(&self) -> *mut Powerstatus { 
        (self.0 + 0x4) as *mut Powerstatus
    }

    #[doc="Get the *const pointer for the POWERSTATUS register."]
    #[inline] pub fn powerstatus_ptr(&self) -> *const Powerstatus { 
           self.powerstatus_mut()
    }

    #[doc="Read the POWERSTATUS register."]
    #[inline] pub fn powerstatus(&self) -> Powerstatus { 
        unsafe {
            read_volatile(self.powerstatus_ptr())
        }
    }

    #[doc="Write the POWERSTATUS register."]
    #[inline] pub fn set_powerstatus<F: FnOnce(Powerstatus) -> Powerstatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.powerstatus_mut(), f(Powerstatus(0)));
        }
        self
    }

    #[doc="Modify the POWERSTATUS register."]
    #[inline] pub fn with_powerstatus<F: FnOnce(Powerstatus) -> Powerstatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.powerstatus_mut(), f(self.powerstatus()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DEVICEEN register."]
    #[inline] pub fn deviceen_mut(&self) -> *mut Deviceen { 
        (self.0 + 0x8) as *mut Deviceen
    }

    #[doc="Get the *const pointer for the DEVICEEN register."]
    #[inline] pub fn deviceen_ptr(&self) -> *const Deviceen { 
           self.deviceen_mut()
    }

    #[doc="Read the DEVICEEN register."]
    #[inline] pub fn deviceen(&self) -> Deviceen { 
        unsafe {
            read_volatile(self.deviceen_ptr())
        }
    }

    #[doc="Write the DEVICEEN register."]
    #[inline] pub fn set_deviceen<F: FnOnce(Deviceen) -> Deviceen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.deviceen_mut(), f(Deviceen(0)));
        }
        self
    }

    #[doc="Modify the DEVICEEN register."]
    #[inline] pub fn with_deviceen<F: FnOnce(Deviceen) -> Deviceen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.deviceen_mut(), f(self.deviceen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SRAMPWDINSLEEP register."]
    #[inline] pub fn srampwdinsleep_mut(&self) -> *mut Srampwdinsleep { 
        (self.0 + 0xc) as *mut Srampwdinsleep
    }

    #[doc="Get the *const pointer for the SRAMPWDINSLEEP register."]
    #[inline] pub fn srampwdinsleep_ptr(&self) -> *const Srampwdinsleep { 
           self.srampwdinsleep_mut()
    }

    #[doc="Read the SRAMPWDINSLEEP register."]
    #[inline] pub fn srampwdinsleep(&self) -> Srampwdinsleep { 
        unsafe {
            read_volatile(self.srampwdinsleep_ptr())
        }
    }

    #[doc="Write the SRAMPWDINSLEEP register."]
    #[inline] pub fn set_srampwdinsleep<F: FnOnce(Srampwdinsleep) -> Srampwdinsleep>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.srampwdinsleep_mut(), f(Srampwdinsleep(0)));
        }
        self
    }

    #[doc="Modify the SRAMPWDINSLEEP register."]
    #[inline] pub fn with_srampwdinsleep<F: FnOnce(Srampwdinsleep) -> Srampwdinsleep>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.srampwdinsleep_mut(), f(self.srampwdinsleep()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MEMEN register."]
    #[inline] pub fn memen_mut(&self) -> *mut Memen { 
        (self.0 + 0x10) as *mut Memen
    }

    #[doc="Get the *const pointer for the MEMEN register."]
    #[inline] pub fn memen_ptr(&self) -> *const Memen { 
           self.memen_mut()
    }

    #[doc="Read the MEMEN register."]
    #[inline] pub fn memen(&self) -> Memen { 
        unsafe {
            read_volatile(self.memen_ptr())
        }
    }

    #[doc="Write the MEMEN register."]
    #[inline] pub fn set_memen<F: FnOnce(Memen) -> Memen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.memen_mut(), f(Memen(0)));
        }
        self
    }

    #[doc="Modify the MEMEN register."]
    #[inline] pub fn with_memen<F: FnOnce(Memen) -> Memen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.memen_mut(), f(self.memen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PWRONSTATUS register."]
    #[inline] pub fn pwronstatus_mut(&self) -> *mut Pwronstatus { 
        (self.0 + 0x14) as *mut Pwronstatus
    }

    #[doc="Get the *const pointer for the PWRONSTATUS register."]
    #[inline] pub fn pwronstatus_ptr(&self) -> *const Pwronstatus { 
           self.pwronstatus_mut()
    }

    #[doc="Read the PWRONSTATUS register."]
    #[inline] pub fn pwronstatus(&self) -> Pwronstatus { 
        unsafe {
            read_volatile(self.pwronstatus_ptr())
        }
    }

    #[doc="Write the PWRONSTATUS register."]
    #[inline] pub fn set_pwronstatus<F: FnOnce(Pwronstatus) -> Pwronstatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pwronstatus_mut(), f(Pwronstatus(0)));
        }
        self
    }

    #[doc="Modify the PWRONSTATUS register."]
    #[inline] pub fn with_pwronstatus<F: FnOnce(Pwronstatus) -> Pwronstatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pwronstatus_mut(), f(self.pwronstatus()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SRAMCTRL register."]
    #[inline] pub fn sramctrl_mut(&self) -> *mut Sramctrl { 
        (self.0 + 0x18) as *mut Sramctrl
    }

    #[doc="Get the *const pointer for the SRAMCTRL register."]
    #[inline] pub fn sramctrl_ptr(&self) -> *const Sramctrl { 
           self.sramctrl_mut()
    }

    #[doc="Read the SRAMCTRL register."]
    #[inline] pub fn sramctrl(&self) -> Sramctrl { 
        unsafe {
            read_volatile(self.sramctrl_ptr())
        }
    }

    #[doc="Write the SRAMCTRL register."]
    #[inline] pub fn set_sramctrl<F: FnOnce(Sramctrl) -> Sramctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sramctrl_mut(), f(Sramctrl(0)));
        }
        self
    }

    #[doc="Modify the SRAMCTRL register."]
    #[inline] pub fn with_sramctrl<F: FnOnce(Sramctrl) -> Sramctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sramctrl_mut(), f(self.sramctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCSTATUS register."]
    #[inline] pub fn adcstatus_mut(&self) -> *mut Adcstatus { 
        (self.0 + 0x1c) as *mut Adcstatus
    }

    #[doc="Get the *const pointer for the ADCSTATUS register."]
    #[inline] pub fn adcstatus_ptr(&self) -> *const Adcstatus { 
           self.adcstatus_mut()
    }

    #[doc="Read the ADCSTATUS register."]
    #[inline] pub fn adcstatus(&self) -> Adcstatus { 
        unsafe {
            read_volatile(self.adcstatus_ptr())
        }
    }

    #[doc="Write the ADCSTATUS register."]
    #[inline] pub fn set_adcstatus<F: FnOnce(Adcstatus) -> Adcstatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcstatus_mut(), f(Adcstatus(0)));
        }
        self
    }

    #[doc="Modify the ADCSTATUS register."]
    #[inline] pub fn with_adcstatus<F: FnOnce(Adcstatus) -> Adcstatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcstatus_mut(), f(self.adcstatus()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MISCOPT register."]
    #[inline] pub fn miscopt_mut(&self) -> *mut Miscopt { 
        (self.0 + 0x20) as *mut Miscopt
    }

    #[doc="Get the *const pointer for the MISCOPT register."]
    #[inline] pub fn miscopt_ptr(&self) -> *const Miscopt { 
           self.miscopt_mut()
    }

    #[doc="Read the MISCOPT register."]
    #[inline] pub fn miscopt(&self) -> Miscopt { 
        unsafe {
            read_volatile(self.miscopt_ptr())
        }
    }

    #[doc="Write the MISCOPT register."]
    #[inline] pub fn set_miscopt<F: FnOnce(Miscopt) -> Miscopt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.miscopt_mut(), f(Miscopt(0)));
        }
        self
    }

    #[doc="Modify the MISCOPT register."]
    #[inline] pub fn with_miscopt<F: FnOnce(Miscopt) -> Miscopt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.miscopt_mut(), f(self.miscopt()));
        }
        self
    }

}

#[doc="Memory and Core Voltage Supply Source Select Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Supplysrc(pub u32);
impl Supplysrc {
    #[doc="Switches the CORE DOMAIN from BUCK mode (if enabled) to LDO when CPU is in DEEP SLEEP. If all the devices are off then this does not matter and LDO (low power mode) is used"]
    #[inline] pub fn switch_ldo_in_sleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SWITCH_LDO_IN_SLEEP != 0"]
    #[inline] pub fn test_switch_ldo_in_sleep(&self) -> bool {
        self.switch_ldo_in_sleep() != 0
    }

    #[doc="Sets the SWITCH_LDO_IN_SLEEP field."]
    #[inline] pub fn set_switch_ldo_in_sleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enables and Selects the Core Buck as the supply for the low-voltage power domain."]
    #[inline] pub fn corebucken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COREBUCKEN != 0"]
    #[inline] pub fn test_corebucken(&self) -> bool {
        self.corebucken() != 0
    }

    #[doc="Sets the COREBUCKEN field."]
    #[inline] pub fn set_corebucken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enables and select the Memory Buck as the supply for the Flash and SRAM power domain."]
    #[inline] pub fn membucken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MEMBUCKEN != 0"]
    #[inline] pub fn test_membucken(&self) -> bool {
        self.membucken() != 0
    }

    #[doc="Sets the MEMBUCKEN field."]
    #[inline] pub fn set_membucken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Supplysrc {
    #[inline]
    fn from(other: u32) -> Self {
         Supplysrc(other)
    }
}

impl ::core::fmt::Display for Supplysrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Supplysrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.switch_ldo_in_sleep() != 0 { try!(write!(f, " switch_ldo_in_sleep"))}
        if self.corebucken() != 0 { try!(write!(f, " corebucken"))}
        if self.membucken() != 0 { try!(write!(f, " membucken"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Status Register for MCU supplies and peripherals"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Powerstatus(pub u32);
impl Powerstatus {
    #[doc="Indicates whether the Core low-voltage domain is supplied from the LDO or the Buck."]
    #[inline] pub fn corebuckon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COREBUCKON != 0"]
    #[inline] pub fn test_corebuckon(&self) -> bool {
        self.corebuckon() != 0
    }

    #[doc="Sets the COREBUCKON field."]
    #[inline] pub fn set_corebuckon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Indicate whether the Memory power domain is supplied from the LDO or the Buck."]
    #[inline] pub fn membuckon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MEMBUCKON != 0"]
    #[inline] pub fn test_membuckon(&self) -> bool {
        self.membuckon() != 0
    }

    #[doc="Sets the MEMBUCKON field."]
    #[inline] pub fn set_membuckon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Powerstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Powerstatus(other)
    }
}

impl ::core::fmt::Display for Powerstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Powerstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.corebuckon() != 0 { try!(write!(f, " corebuckon"))}
        if self.membuckon() != 0 { try!(write!(f, " membuckon"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE ENABLES for SHELBY"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Deviceen(pub u32);
impl Deviceen {
    #[doc="Enable PDM Digital Block"]
    #[inline] pub fn pdm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PDM != 0"]
    #[inline] pub fn test_pdm(&self) -> bool {
        self.pdm() != 0
    }

    #[doc="Sets the PDM field."]
    #[inline] pub fn set_pdm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Enable ADC Digital Block"]
    #[inline] pub fn adc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADC != 0"]
    #[inline] pub fn test_adc(&self) -> bool {
        self.adc() != 0
    }

    #[doc="Sets the ADC field."]
    #[inline] pub fn set_adc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable UART 1"]
    #[inline] pub fn uart1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if UART1 != 0"]
    #[inline] pub fn test_uart1(&self) -> bool {
        self.uart1() != 0
    }

    #[doc="Sets the UART1 field."]
    #[inline] pub fn set_uart1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enable UART 0"]
    #[inline] pub fn uart0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if UART0 != 0"]
    #[inline] pub fn test_uart0(&self) -> bool {
        self.uart0() != 0
    }

    #[doc="Sets the UART0 field."]
    #[inline] pub fn set_uart0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Enable IO MASTER 5"]
    #[inline] pub fn io_master5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IO_MASTER5 != 0"]
    #[inline] pub fn test_io_master5(&self) -> bool {
        self.io_master5() != 0
    }

    #[doc="Sets the IO_MASTER5 field."]
    #[inline] pub fn set_io_master5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Enable IO MASTER 4"]
    #[inline] pub fn io_master4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IO_MASTER4 != 0"]
    #[inline] pub fn test_io_master4(&self) -> bool {
        self.io_master4() != 0
    }

    #[doc="Sets the IO_MASTER4 field."]
    #[inline] pub fn set_io_master4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Enable IO MASTER 3"]
    #[inline] pub fn io_master3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IO_MASTER3 != 0"]
    #[inline] pub fn test_io_master3(&self) -> bool {
        self.io_master3() != 0
    }

    #[doc="Sets the IO_MASTER3 field."]
    #[inline] pub fn set_io_master3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enable IO MASTER 2"]
    #[inline] pub fn io_master2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IO_MASTER2 != 0"]
    #[inline] pub fn test_io_master2(&self) -> bool {
        self.io_master2() != 0
    }

    #[doc="Sets the IO_MASTER2 field."]
    #[inline] pub fn set_io_master2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enable IO MASTER 1"]
    #[inline] pub fn io_master1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IO_MASTER1 != 0"]
    #[inline] pub fn test_io_master1(&self) -> bool {
        self.io_master1() != 0
    }

    #[doc="Sets the IO_MASTER1 field."]
    #[inline] pub fn set_io_master1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enable IO MASTER 0"]
    #[inline] pub fn io_master0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IO_MASTER0 != 0"]
    #[inline] pub fn test_io_master0(&self) -> bool {
        self.io_master0() != 0
    }

    #[doc="Sets the IO_MASTER0 field."]
    #[inline] pub fn set_io_master0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable IO SLAVE"]
    #[inline] pub fn io_slave(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IO_SLAVE != 0"]
    #[inline] pub fn test_io_slave(&self) -> bool {
        self.io_slave() != 0
    }

    #[doc="Sets the IO_SLAVE field."]
    #[inline] pub fn set_io_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Deviceen {
    #[inline]
    fn from(other: u32) -> Self {
         Deviceen(other)
    }
}

impl ::core::fmt::Display for Deviceen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Deviceen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pdm() != 0 { try!(write!(f, " pdm"))}
        if self.adc() != 0 { try!(write!(f, " adc"))}
        if self.uart1() != 0 { try!(write!(f, " uart1"))}
        if self.uart0() != 0 { try!(write!(f, " uart0"))}
        if self.io_master5() != 0 { try!(write!(f, " io_master5"))}
        if self.io_master4() != 0 { try!(write!(f, " io_master4"))}
        if self.io_master3() != 0 { try!(write!(f, " io_master3"))}
        if self.io_master2() != 0 { try!(write!(f, " io_master2"))}
        if self.io_master1() != 0 { try!(write!(f, " io_master1"))}
        if self.io_master0() != 0 { try!(write!(f, " io_master0"))}
        if self.io_slave() != 0 { try!(write!(f, " io_slave"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Powerdown an SRAM Banks in Deep Sleep mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srampwdinsleep(pub u32);
impl Srampwdinsleep {
    #[doc="Enable CACHE BANKS to power down in deep sleep"]
    #[inline] pub fn cache_pwd_slp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CACHE_PWD_SLP != 0"]
    #[inline] pub fn test_cache_pwd_slp(&self) -> bool {
        self.cache_pwd_slp() != 0
    }

    #[doc="Sets the CACHE_PWD_SLP field."]
    #[inline] pub fn set_cache_pwd_slp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline] pub fn sramsleeppowerdown(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if SRAMSLEEPPOWERDOWN != 0"]
    #[inline] pub fn test_sramsleeppowerdown(&self) -> bool {
        self.sramsleeppowerdown() != 0
    }

    #[doc="Sets the SRAMSLEEPPOWERDOWN field."]
    #[inline] pub fn set_sramsleeppowerdown<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srampwdinsleep {
    #[inline]
    fn from(other: u32) -> Self {
         Srampwdinsleep(other)
    }
}

impl ::core::fmt::Display for Srampwdinsleep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srampwdinsleep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cache_pwd_slp() != 0 { try!(write!(f, " cache_pwd_slp"))}
        if self.sramsleeppowerdown() != 0 { try!(write!(f, " sramsleeppowerdown=0x{:x}", self.sramsleeppowerdown()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Disables individual banks of the MEMORY array"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Memen(pub u32);
impl Memen {
    #[doc="Enable CACHE BANK 2"]
    #[inline] pub fn cacheb2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CACHEB2 != 0"]
    #[inline] pub fn test_cacheb2(&self) -> bool {
        self.cacheb2() != 0
    }

    #[doc="Sets the CACHEB2 field."]
    #[inline] pub fn set_cacheb2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Enable CACHE BANK 0"]
    #[inline] pub fn cacheb0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if CACHEB0 != 0"]
    #[inline] pub fn test_cacheb0(&self) -> bool {
        self.cacheb0() != 0
    }

    #[doc="Sets the CACHEB0 field."]
    #[inline] pub fn set_cacheb0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Enable FLASH1"]
    #[inline] pub fn flash1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FLASH1 != 0"]
    #[inline] pub fn test_flash1(&self) -> bool {
        self.flash1() != 0
    }

    #[doc="Sets the FLASH1 field."]
    #[inline] pub fn set_flash1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable FLASH 0"]
    #[inline] pub fn flash0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FLASH0 != 0"]
    #[inline] pub fn test_flash0(&self) -> bool {
        self.flash0() != 0
    }

    #[doc="Sets the FLASH0 field."]
    #[inline] pub fn set_flash0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Enables power for selected SRAM banks (else an access to its address space to generate a Hard Fault)."]
    #[inline] pub fn sramen(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if SRAMEN != 0"]
    #[inline] pub fn test_sramen(&self) -> bool {
        self.sramen() != 0
    }

    #[doc="Sets the SRAMEN field."]
    #[inline] pub fn set_sramen<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Memen {
    #[inline]
    fn from(other: u32) -> Self {
         Memen(other)
    }
}

impl ::core::fmt::Display for Memen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Memen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cacheb2() != 0 { try!(write!(f, " cacheb2"))}
        if self.cacheb0() != 0 { try!(write!(f, " cacheb0"))}
        if self.flash1() != 0 { try!(write!(f, " flash1"))}
        if self.flash0() != 0 { try!(write!(f, " flash0"))}
        if self.sramen() != 0 { try!(write!(f, " sramen=0x{:x}", self.sramen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="POWER ON Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pwronstatus(pub u32);
impl Pwronstatus {
    #[doc="This bit is 1 if power is supplied to CACHE BANK 2"]
    #[inline] pub fn pd_cacheb2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PD_CACHEB2 != 0"]
    #[inline] pub fn test_pd_cacheb2(&self) -> bool {
        self.pd_cacheb2() != 0
    }

    #[doc="Sets the PD_CACHEB2 field."]
    #[inline] pub fn set_pd_cacheb2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="This bit is 1 if power is supplied to CACHE BANK 0"]
    #[inline] pub fn pd_cacheb0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PD_CACHEB0 != 0"]
    #[inline] pub fn test_pd_cacheb0(&self) -> bool {
        self.pd_cacheb0() != 0
    }

    #[doc="Sets the PD_CACHEB0 field."]
    #[inline] pub fn set_pd_cacheb0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_GRP7"]
    #[inline] pub fn pd_grp7_sram(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PD_GRP7_SRAM != 0"]
    #[inline] pub fn test_pd_grp7_sram(&self) -> bool {
        self.pd_grp7_sram() != 0
    }

    #[doc="Sets the PD_GRP7_SRAM field."]
    #[inline] pub fn set_pd_grp7_sram<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_GRP6"]
    #[inline] pub fn pd_grp6_sram(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PD_GRP6_SRAM != 0"]
    #[inline] pub fn test_pd_grp6_sram(&self) -> bool {
        self.pd_grp6_sram() != 0
    }

    #[doc="Sets the PD_GRP6_SRAM field."]
    #[inline] pub fn set_pd_grp6_sram<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_GRP5"]
    #[inline] pub fn pd_grp5_sram(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PD_GRP5_SRAM != 0"]
    #[inline] pub fn test_pd_grp5_sram(&self) -> bool {
        self.pd_grp5_sram() != 0
    }

    #[doc="Sets the PD_GRP5_SRAM field."]
    #[inline] pub fn set_pd_grp5_sram<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_GRP4"]
    #[inline] pub fn pd_grp4_sram(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD_GRP4_SRAM != 0"]
    #[inline] pub fn test_pd_grp4_sram(&self) -> bool {
        self.pd_grp4_sram() != 0
    }

    #[doc="Sets the PD_GRP4_SRAM field."]
    #[inline] pub fn set_pd_grp4_sram<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_GRP3"]
    #[inline] pub fn pd_grp3_sram(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PD_GRP3_SRAM != 0"]
    #[inline] pub fn test_pd_grp3_sram(&self) -> bool {
        self.pd_grp3_sram() != 0
    }

    #[doc="Sets the PD_GRP3_SRAM field."]
    #[inline] pub fn set_pd_grp3_sram<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_GRP2"]
    #[inline] pub fn pd_grp2_sram(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PD_GRP2_SRAM != 0"]
    #[inline] pub fn test_pd_grp2_sram(&self) -> bool {
        self.pd_grp2_sram() != 0
    }

    #[doc="Sets the PD_GRP2_SRAM field."]
    #[inline] pub fn set_pd_grp2_sram<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_GRP1"]
    #[inline] pub fn pd_grp1_sram(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PD_GRP1_SRAM != 0"]
    #[inline] pub fn test_pd_grp1_sram(&self) -> bool {
        self.pd_grp1_sram() != 0
    }

    #[doc="Sets the PD_GRP1_SRAM field."]
    #[inline] pub fn set_pd_grp1_sram<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_SRAM0_3"]
    #[inline] pub fn pd_grp0_sram3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PD_GRP0_SRAM3 != 0"]
    #[inline] pub fn test_pd_grp0_sram3(&self) -> bool {
        self.pd_grp0_sram3() != 0
    }

    #[doc="Sets the PD_GRP0_SRAM3 field."]
    #[inline] pub fn set_pd_grp0_sram3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain PD_SRAM0_2"]
    #[inline] pub fn pd_grp0_sram2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PD_GRP0_SRAM2 != 0"]
    #[inline] pub fn test_pd_grp0_sram2(&self) -> bool {
        self.pd_grp0_sram2() != 0
    }

    #[doc="Sets the PD_GRP0_SRAM2 field."]
    #[inline] pub fn set_pd_grp0_sram2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain SRAM0_1"]
    #[inline] pub fn pd_grp0_sram1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PD_GRP0_SRAM1 != 0"]
    #[inline] pub fn test_pd_grp0_sram1(&self) -> bool {
        self.pd_grp0_sram1() != 0
    }

    #[doc="Sets the PD_GRP0_SRAM1 field."]
    #[inline] pub fn set_pd_grp0_sram1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This bit is 1 if power is supplied to SRAM domain SRAM0_0"]
    #[inline] pub fn pd_grp0_sram0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PD_GRP0_SRAM0 != 0"]
    #[inline] pub fn test_pd_grp0_sram0(&self) -> bool {
        self.pd_grp0_sram0() != 0
    }

    #[doc="Sets the PD_GRP0_SRAM0 field."]
    #[inline] pub fn set_pd_grp0_sram0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit is 1 if power is supplied to domain PD_ADC"]
    #[inline] pub fn pdadc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PDADC != 0"]
    #[inline] pub fn test_pdadc(&self) -> bool {
        self.pdadc() != 0
    }

    #[doc="Sets the PDADC field."]
    #[inline] pub fn set_pdadc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This bit is 1 if power is supplied to domain PD_FLAM1"]
    #[inline] pub fn pd_flam1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PD_FLAM1 != 0"]
    #[inline] pub fn test_pd_flam1(&self) -> bool {
        self.pd_flam1() != 0
    }

    #[doc="Sets the PD_FLAM1 field."]
    #[inline] pub fn set_pd_flam1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This bit is 1 if power is supplied to domain PD_FLAM0"]
    #[inline] pub fn pd_flam0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PD_FLAM0 != 0"]
    #[inline] pub fn test_pd_flam0(&self) -> bool {
        self.pd_flam0() != 0
    }

    #[doc="Sets the PD_FLAM0 field."]
    #[inline] pub fn set_pd_flam0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit is 1 if power is supplied to domain PD_PDM"]
    #[inline] pub fn pd_pdm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PD_PDM != 0"]
    #[inline] pub fn test_pd_pdm(&self) -> bool {
        self.pd_pdm() != 0
    }

    #[doc="Sets the PD_PDM field."]
    #[inline] pub fn set_pd_pdm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit is 1 if power is supplied to power domain C, which supplies IOM3-5."]
    #[inline] pub fn pdc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PDC != 0"]
    #[inline] pub fn test_pdc(&self) -> bool {
        self.pdc() != 0
    }

    #[doc="Sets the PDC field."]
    #[inline] pub fn set_pdc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit is 1 if power is supplied to power domain B, which supplies IOM0-2."]
    #[inline] pub fn pdb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PDB != 0"]
    #[inline] pub fn test_pdb(&self) -> bool {
        self.pdb() != 0
    }

    #[doc="Sets the PDB field."]
    #[inline] pub fn set_pdb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit is 1 if power is supplied to power domain A, which supplies IOS and UART0,1."]
    #[inline] pub fn pda(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PDA != 0"]
    #[inline] pub fn test_pda(&self) -> bool {
        self.pda() != 0
    }

    #[doc="Sets the PDA field."]
    #[inline] pub fn set_pda<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Pwronstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Pwronstatus(other)
    }
}

impl ::core::fmt::Display for Pwronstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pwronstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pd_cacheb2() != 0 { try!(write!(f, " pd_cacheb2"))}
        if self.pd_cacheb0() != 0 { try!(write!(f, " pd_cacheb0"))}
        if self.pd_grp7_sram() != 0 { try!(write!(f, " pd_grp7_sram"))}
        if self.pd_grp6_sram() != 0 { try!(write!(f, " pd_grp6_sram"))}
        if self.pd_grp5_sram() != 0 { try!(write!(f, " pd_grp5_sram"))}
        if self.pd_grp4_sram() != 0 { try!(write!(f, " pd_grp4_sram"))}
        if self.pd_grp3_sram() != 0 { try!(write!(f, " pd_grp3_sram"))}
        if self.pd_grp2_sram() != 0 { try!(write!(f, " pd_grp2_sram"))}
        if self.pd_grp1_sram() != 0 { try!(write!(f, " pd_grp1_sram"))}
        if self.pd_grp0_sram3() != 0 { try!(write!(f, " pd_grp0_sram3"))}
        if self.pd_grp0_sram2() != 0 { try!(write!(f, " pd_grp0_sram2"))}
        if self.pd_grp0_sram1() != 0 { try!(write!(f, " pd_grp0_sram1"))}
        if self.pd_grp0_sram0() != 0 { try!(write!(f, " pd_grp0_sram0"))}
        if self.pdadc() != 0 { try!(write!(f, " pdadc"))}
        if self.pd_flam1() != 0 { try!(write!(f, " pd_flam1"))}
        if self.pd_flam0() != 0 { try!(write!(f, " pd_flam0"))}
        if self.pd_pdm() != 0 { try!(write!(f, " pd_pdm"))}
        if self.pdc() != 0 { try!(write!(f, " pdc"))}
        if self.pdb() != 0 { try!(write!(f, " pdb"))}
        if self.pda() != 0 { try!(write!(f, " pda"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sramctrl(pub u32);
impl Sramctrl {
    #[doc="Enables top-level clock gating in the SRAM block. This bit should be enabled for lowest power operation."]
    #[inline] pub fn sram_master_clkgate(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SRAM_MASTER_CLKGATE != 0"]
    #[inline] pub fn test_sram_master_clkgate(&self) -> bool {
        self.sram_master_clkgate() != 0
    }

    #[doc="Sets the SRAM_MASTER_CLKGATE field."]
    #[inline] pub fn set_sram_master_clkgate<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enables individual per-RAM clock gating in the SRAM block. This bit should be enabled for lowest power operation."]
    #[inline] pub fn sram_clkgate(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SRAM_CLKGATE != 0"]
    #[inline] pub fn test_sram_clkgate(&self) -> bool {
        self.sram_clkgate() != 0
    }

    #[doc="Sets the SRAM_CLKGATE field."]
    #[inline] pub fn set_sram_clkgate<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable LS (light sleep) of cache RAMs. When this bit is set, the RAMS will be put into light sleep mode while inactive. NOTE: if the SRAM is actively used, this may have an adverse affect on power since entering/exiting LS mode may consume more power than would be saved."]
    #[inline] pub fn sram_light_sleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SRAM_LIGHT_SLEEP != 0"]
    #[inline] pub fn test_sram_light_sleep(&self) -> bool {
        self.sram_light_sleep() != 0
    }

    #[doc="Sets the SRAM_LIGHT_SLEEP field."]
    #[inline] pub fn set_sram_light_sleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sramctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Sramctrl(other)
    }
}

impl ::core::fmt::Display for Sramctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sramctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sram_master_clkgate() != 0 { try!(write!(f, " sram_master_clkgate"))}
        if self.sram_clkgate() != 0 { try!(write!(f, " sram_clkgate"))}
        if self.sram_light_sleep() != 0 { try!(write!(f, " sram_light_sleep"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Status Register for ADC Block"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adcstatus(pub u32);
impl Adcstatus {
    #[doc="This bit indicates that the ADC REFBUF is powered down"]
    #[inline] pub fn adc_refbuf_pwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ADC_REFBUF_PWD != 0"]
    #[inline] pub fn test_adc_refbuf_pwd(&self) -> bool {
        self.adc_refbuf_pwd() != 0
    }

    #[doc="Sets the ADC_REFBUF_PWD field."]
    #[inline] pub fn set_adc_refbuf_pwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit indicates that the ADC REFKEEP is powered down"]
    #[inline] pub fn adc_refkeep_pwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ADC_REFKEEP_PWD != 0"]
    #[inline] pub fn test_adc_refkeep_pwd(&self) -> bool {
        self.adc_refkeep_pwd() != 0
    }

    #[doc="Sets the ADC_REFKEEP_PWD field."]
    #[inline] pub fn set_adc_refkeep_pwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline] pub fn adc_vbat_pwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADC_VBAT_PWD != 0"]
    #[inline] pub fn test_adc_vbat_pwd(&self) -> bool {
        self.adc_vbat_pwd() != 0
    }

    #[doc="Sets the ADC_VBAT_PWD field."]
    #[inline] pub fn set_adc_vbat_pwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline] pub fn adc_vptat_pwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ADC_VPTAT_PWD != 0"]
    #[inline] pub fn test_adc_vptat_pwd(&self) -> bool {
        self.adc_vptat_pwd() != 0
    }

    #[doc="Sets the ADC_VPTAT_PWD field."]
    #[inline] pub fn set_adc_vptat_pwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit indicates that the ADC Band Gap is powered down"]
    #[inline] pub fn adc_bgt_pwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ADC_BGT_PWD != 0"]
    #[inline] pub fn test_adc_bgt_pwd(&self) -> bool {
        self.adc_bgt_pwd() != 0
    }

    #[doc="Sets the ADC_BGT_PWD field."]
    #[inline] pub fn set_adc_bgt_pwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit indicates that the ADC is powered down"]
    #[inline] pub fn adc_pwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADC_PWD != 0"]
    #[inline] pub fn test_adc_pwd(&self) -> bool {
        self.adc_pwd() != 0
    }

    #[doc="Sets the ADC_PWD field."]
    #[inline] pub fn set_adc_pwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Adcstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Adcstatus(other)
    }
}

impl ::core::fmt::Display for Adcstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adcstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adc_refbuf_pwd() != 0 { try!(write!(f, " adc_refbuf_pwd"))}
        if self.adc_refkeep_pwd() != 0 { try!(write!(f, " adc_refkeep_pwd"))}
        if self.adc_vbat_pwd() != 0 { try!(write!(f, " adc_vbat_pwd"))}
        if self.adc_vptat_pwd() != 0 { try!(write!(f, " adc_vptat_pwd"))}
        if self.adc_bgt_pwd() != 0 { try!(write!(f, " adc_bgt_pwd"))}
        if self.adc_pwd() != 0 { try!(write!(f, " adc_pwd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power Optimization Control Bits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Miscopt(pub u32);
impl Miscopt {
    #[doc="Setting this bit will enable the MEM LDO to be in LPMODE during deep sleep even when the ctimers or stimers are running"]
    #[inline] pub fn dis_ldolpmode_timers(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DIS_LDOLPMODE_TIMERS != 0"]
    #[inline] pub fn test_dis_ldolpmode_timers(&self) -> bool {
        self.dis_ldolpmode_timers() != 0
    }

    #[doc="Sets the DIS_LDOLPMODE_TIMERS field."]
    #[inline] pub fn set_dis_ldolpmode_timers<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Setting this bit will enable the Core LDO to be in LPMODE during deep sleep even when HFRC is enabled."]
    #[inline] pub fn dis_ldolpmode_hfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DIS_LDOLPMODE_HFRC != 0"]
    #[inline] pub fn test_dis_ldolpmode_hfrc(&self) -> bool {
        self.dis_ldolpmode_hfrc() != 0
    }

    #[doc="Sets the DIS_LDOLPMODE_HFRC field."]
    #[inline] pub fn set_dis_ldolpmode_hfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Control Bit to mask the ADC_EN in the adc_pwr_down equation."]
    #[inline] pub fn adc_en_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADC_EN_MASK != 0"]
    #[inline] pub fn test_adc_en_mask(&self) -> bool {
        self.adc_en_mask() != 0
    }

    #[doc="Sets the ADC_EN_MASK field."]
    #[inline] pub fn set_adc_en_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Miscopt {
    #[inline]
    fn from(other: u32) -> Self {
         Miscopt(other)
    }
}

impl ::core::fmt::Display for Miscopt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Miscopt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dis_ldolpmode_timers() != 0 { try!(write!(f, " dis_ldolpmode_timers"))}
        if self.dis_ldolpmode_hfrc() != 0 { try!(write!(f, " dis_ldolpmode_hfrc"))}
        if self.adc_en_mask() != 0 { try!(write!(f, " adc_en_mask"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


