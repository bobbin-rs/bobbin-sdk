::bobbin_mcu::periph!( CMCC, Cmcc, CMCC_PERIPH, CmccPeriph, CMCC_OWNED, CMCC_REF_COUNT, 0x41006000, 0x00, 0x05);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CMCC Peripheral"]
pub struct CmccPeriph(pub usize); 

impl CmccPeriph {
    #[doc="Get the TYPE Register."]
    #[inline] pub fn type_reg(&self) -> ::bobbin_mcu::register::Register<Type> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Type, 0x0)
    }

    #[doc="Get the *mut pointer for the TYPE register."]
    #[inline] pub fn type_mut(&self) -> *mut Type { 
        self.type_reg().ptr()
    }

    #[doc="Get the *const pointer for the TYPE register."]
    #[inline] pub fn type_ptr(&self) -> *const Type { 
        self.type_reg().ptr()
    }

    #[doc="Read the TYPE register."]
    #[inline] pub fn _type(&self) -> Type { 
        self.type_reg().read()
    }

    #[doc="Get the CFG Register."]
    #[inline] pub fn cfg_reg(&self) -> ::bobbin_mcu::register::Register<Cfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfg, 0x4)
    }

    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut(&self) -> *mut Cfg { 
        self.cfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr(&self) -> *const Cfg { 
        self.cfg_reg().ptr()
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg(&self) -> Cfg { 
        self.cfg_reg().read()
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn write_cfg(&self, value: Cfg) -> &Self { 
        self.cfg_reg().write(value);
        self
    }

    #[doc="Set the CFG register."]
    #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        self.cfg_reg().set(f);
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        self.cfg_reg().with(f);
        self
    }

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x8)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0xc)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Get the LCKWAY Register."]
    #[inline] pub fn lckway_reg(&self) -> ::bobbin_mcu::register::Register<Lckway> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Lckway, 0x10)
    }

    #[doc="Get the *mut pointer for the LCKWAY register."]
    #[inline] pub fn lckway_mut(&self) -> *mut Lckway { 
        self.lckway_reg().ptr()
    }

    #[doc="Get the *const pointer for the LCKWAY register."]
    #[inline] pub fn lckway_ptr(&self) -> *const Lckway { 
        self.lckway_reg().ptr()
    }

    #[doc="Read the LCKWAY register."]
    #[inline] pub fn lckway(&self) -> Lckway { 
        self.lckway_reg().read()
    }

    #[doc="Write the LCKWAY register."]
    #[inline] pub fn write_lckway(&self, value: Lckway) -> &Self { 
        self.lckway_reg().write(value);
        self
    }

    #[doc="Set the LCKWAY register."]
    #[inline] pub fn set_lckway<F: FnOnce(Lckway) -> Lckway>(&self, f: F) -> &Self {
        self.lckway_reg().set(f);
        self
    }

    #[doc="Modify the LCKWAY register."]
    #[inline] pub fn with_lckway<F: FnOnce(Lckway) -> Lckway>(&self, f: F) -> &Self {
        self.lckway_reg().with(f);
        self
    }

    #[doc="Get the MAINT0 Register."]
    #[inline] pub fn maint0_reg(&self) -> ::bobbin_mcu::register::Register<Maint0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maint0, 0x20)
    }

    #[doc="Get the *mut pointer for the MAINT0 register."]
    #[inline] pub fn maint0_mut(&self) -> *mut Maint0 { 
        self.maint0_reg().ptr()
    }

    #[doc="Get the *const pointer for the MAINT0 register."]
    #[inline] pub fn maint0_ptr(&self) -> *const Maint0 { 
        self.maint0_reg().ptr()
    }

    #[doc="Write the MAINT0 register."]
    #[inline] pub fn write_maint0(&self, value: Maint0) -> &Self { 
        self.maint0_reg().write(value);
        self
    }

    #[doc="Set the MAINT0 register."]
    #[inline] pub fn set_maint0<F: FnOnce(Maint0) -> Maint0>(&self, f: F) -> &Self {
        self.maint0_reg().set(f);
        self
    }

    #[doc="Get the MAINT1 Register."]
    #[inline] pub fn maint1_reg(&self) -> ::bobbin_mcu::register::Register<Maint1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maint1, 0x24)
    }

    #[doc="Get the *mut pointer for the MAINT1 register."]
    #[inline] pub fn maint1_mut(&self) -> *mut Maint1 { 
        self.maint1_reg().ptr()
    }

    #[doc="Get the *const pointer for the MAINT1 register."]
    #[inline] pub fn maint1_ptr(&self) -> *const Maint1 { 
        self.maint1_reg().ptr()
    }

    #[doc="Write the MAINT1 register."]
    #[inline] pub fn write_maint1(&self, value: Maint1) -> &Self { 
        self.maint1_reg().write(value);
        self
    }

    #[doc="Set the MAINT1 register."]
    #[inline] pub fn set_maint1<F: FnOnce(Maint1) -> Maint1>(&self, f: F) -> &Self {
        self.maint1_reg().set(f);
        self
    }

    #[doc="Get the MCFG Register."]
    #[inline] pub fn mcfg_reg(&self) -> ::bobbin_mcu::register::Register<Mcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mcfg, 0x28)
    }

    #[doc="Get the *mut pointer for the MCFG register."]
    #[inline] pub fn mcfg_mut(&self) -> *mut Mcfg { 
        self.mcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCFG register."]
    #[inline] pub fn mcfg_ptr(&self) -> *const Mcfg { 
        self.mcfg_reg().ptr()
    }

    #[doc="Read the MCFG register."]
    #[inline] pub fn mcfg(&self) -> Mcfg { 
        self.mcfg_reg().read()
    }

    #[doc="Write the MCFG register."]
    #[inline] pub fn write_mcfg(&self, value: Mcfg) -> &Self { 
        self.mcfg_reg().write(value);
        self
    }

    #[doc="Set the MCFG register."]
    #[inline] pub fn set_mcfg<F: FnOnce(Mcfg) -> Mcfg>(&self, f: F) -> &Self {
        self.mcfg_reg().set(f);
        self
    }

    #[doc="Modify the MCFG register."]
    #[inline] pub fn with_mcfg<F: FnOnce(Mcfg) -> Mcfg>(&self, f: F) -> &Self {
        self.mcfg_reg().with(f);
        self
    }

    #[doc="Get the MEN Register."]
    #[inline] pub fn men_reg(&self) -> ::bobbin_mcu::register::Register<Men> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Men, 0x2c)
    }

    #[doc="Get the *mut pointer for the MEN register."]
    #[inline] pub fn men_mut(&self) -> *mut Men { 
        self.men_reg().ptr()
    }

    #[doc="Get the *const pointer for the MEN register."]
    #[inline] pub fn men_ptr(&self) -> *const Men { 
        self.men_reg().ptr()
    }

    #[doc="Read the MEN register."]
    #[inline] pub fn men(&self) -> Men { 
        self.men_reg().read()
    }

    #[doc="Write the MEN register."]
    #[inline] pub fn write_men(&self, value: Men) -> &Self { 
        self.men_reg().write(value);
        self
    }

    #[doc="Set the MEN register."]
    #[inline] pub fn set_men<F: FnOnce(Men) -> Men>(&self, f: F) -> &Self {
        self.men_reg().set(f);
        self
    }

    #[doc="Modify the MEN register."]
    #[inline] pub fn with_men<F: FnOnce(Men) -> Men>(&self, f: F) -> &Self {
        self.men_reg().with(f);
        self
    }

    #[doc="Get the MCTRL Register."]
    #[inline] pub fn mctrl_reg(&self) -> ::bobbin_mcu::register::Register<Mctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mctrl, 0x30)
    }

    #[doc="Get the *mut pointer for the MCTRL register."]
    #[inline] pub fn mctrl_mut(&self) -> *mut Mctrl { 
        self.mctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCTRL register."]
    #[inline] pub fn mctrl_ptr(&self) -> *const Mctrl { 
        self.mctrl_reg().ptr()
    }

    #[doc="Write the MCTRL register."]
    #[inline] pub fn write_mctrl(&self, value: Mctrl) -> &Self { 
        self.mctrl_reg().write(value);
        self
    }

    #[doc="Set the MCTRL register."]
    #[inline] pub fn set_mctrl<F: FnOnce(Mctrl) -> Mctrl>(&self, f: F) -> &Self {
        self.mctrl_reg().set(f);
        self
    }

    #[doc="Get the MSR Register."]
    #[inline] pub fn msr_reg(&self) -> ::bobbin_mcu::register::Register<Msr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Msr, 0x34)
    }

    #[doc="Get the *mut pointer for the MSR register."]
    #[inline] pub fn msr_mut(&self) -> *mut Msr { 
        self.msr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MSR register."]
    #[inline] pub fn msr_ptr(&self) -> *const Msr { 
        self.msr_reg().ptr()
    }

    #[doc="Read the MSR register."]
    #[inline] pub fn msr(&self) -> Msr { 
        self.msr_reg().read()
    }

}

#[doc="Cache Type Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Type(pub u32);
impl Type {
    #[doc="dynamic Clock Gating supported"]
    #[inline] pub fn gclk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GCLK != 0"]
    #[inline] pub fn test_gclk(&self) -> bool {
        self.gclk() != 0
    }

    #[doc="Sets the GCLK field."]
    #[inline] pub fn set_gclk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Round Robin Policy supported"]
    #[inline] pub fn rrp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RRP != 0"]
    #[inline] pub fn test_rrp(&self) -> bool {
        self.rrp() != 0
    }

    #[doc="Sets the RRP field."]
    #[inline] pub fn set_rrp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Number of Way"]
    #[inline] pub fn waynum(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if WAYNUM != 0"]
    #[inline] pub fn test_waynum(&self) -> bool {
        self.waynum() != 0
    }

    #[doc="Sets the WAYNUM field."]
    #[inline] pub fn set_waynum<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Lock Down supported"]
    #[inline] pub fn lckdown(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LCKDOWN != 0"]
    #[inline] pub fn test_lckdown(&self) -> bool {
        self.lckdown() != 0
    }

    #[doc="Sets the LCKDOWN field."]
    #[inline] pub fn set_lckdown<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Cache Size"]
    #[inline] pub fn csize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if CSIZE != 0"]
    #[inline] pub fn test_csize(&self) -> bool {
        self.csize() != 0
    }

    #[doc="Sets the CSIZE field."]
    #[inline] pub fn set_csize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Cache Line Size"]
    #[inline] pub fn clsize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if CLSIZE != 0"]
    #[inline] pub fn test_clsize(&self) -> bool {
        self.clsize() != 0
    }

    #[doc="Sets the CLSIZE field."]
    #[inline] pub fn set_clsize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Type {
    #[inline]
    fn from(other: u32) -> Self {
         Type(other)
    }
}

impl ::core::fmt::Display for Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gclk() != 0 { try!(write!(f, " gclk"))}
        if self.rrp() != 0 { try!(write!(f, " rrp"))}
        if self.waynum() != 0 { try!(write!(f, " waynum=0x{:x}", self.waynum()))}
        if self.lckdown() != 0 { try!(write!(f, " lckdown"))}
        if self.csize() != 0 { try!(write!(f, " csize=0x{:x}", self.csize()))}
        if self.clsize() != 0 { try!(write!(f, " clsize=0x{:x}", self.clsize()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="Instruction Cache Disable"]
    #[inline] pub fn icdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ICDIS != 0"]
    #[inline] pub fn test_icdis(&self) -> bool {
        self.icdis() != 0
    }

    #[doc="Sets the ICDIS field."]
    #[inline] pub fn set_icdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Cache Disable"]
    #[inline] pub fn dcdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDIS != 0"]
    #[inline] pub fn test_dcdis(&self) -> bool {
        self.dcdis() != 0
    }

    #[doc="Sets the DCDIS field."]
    #[inline] pub fn set_dcdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Cache size configured by software"]
    #[inline] pub fn csizesw(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if CSIZESW != 0"]
    #[inline] pub fn test_csizesw(&self) -> bool {
        self.csizesw() != 0
    }

    #[doc="Sets the CSIZESW field."]
    #[inline] pub fn set_csizesw<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg(other)
    }
}

impl ::core::fmt::Display for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.icdis() != 0 { try!(write!(f, " icdis"))}
        if self.dcdis() != 0 { try!(write!(f, " dcdis"))}
        if self.csizesw() != 0 { try!(write!(f, " csizesw=0x{:x}", self.csizesw()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Cache Controller Enable"]
    #[inline] pub fn cen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CEN != 0"]
    #[inline] pub fn test_cen(&self) -> bool {
        self.cen() != 0
    }

    #[doc="Sets the CEN field."]
    #[inline] pub fn set_cen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrl(other)
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
        if self.cen() != 0 { try!(write!(f, " cen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Cache Controller Status"]
    #[inline] pub fn csts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CSTS != 0"]
    #[inline] pub fn test_csts(&self) -> bool {
        self.csts() != 0
    }

    #[doc="Sets the CSTS field."]
    #[inline] pub fn set_csts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.csts() != 0 { try!(write!(f, " csts"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Lock per Way Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lckway(pub u32);
impl Lckway {
    #[doc="Lockdown way Register"]
    #[inline] pub fn lckway(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if LCKWAY != 0"]
    #[inline] pub fn test_lckway(&self) -> bool {
        self.lckway() != 0
    }

    #[doc="Sets the LCKWAY field."]
    #[inline] pub fn set_lckway<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lckway {
    #[inline]
    fn from(other: u32) -> Self {
         Lckway(other)
    }
}

impl ::core::fmt::Display for Lckway {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lckway {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lckway() != 0 { try!(write!(f, " lckway=0x{:x}", self.lckway()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Maintenance Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maint0(pub u32);
impl Maint0 {
    #[doc="Cache Controller invalidate All"]
    #[inline] pub fn invall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INVALL != 0"]
    #[inline] pub fn test_invall(&self) -> bool {
        self.invall() != 0
    }

    #[doc="Sets the INVALL field."]
    #[inline] pub fn set_invall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Maint0 {
    #[inline]
    fn from(other: u32) -> Self {
         Maint0(other)
    }
}

impl ::core::fmt::Display for Maint0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maint0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.invall() != 0 { try!(write!(f, " invall"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Maintenance Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maint1(pub u32);
impl Maint1 {
    #[doc="Invalidate Index"]
    #[inline] pub fn index(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xff) as u8) } // [11:4]
    }

    #[doc="Returns true if INDEX != 0"]
    #[inline] pub fn test_index(&self) -> bool {
        self.index() != 0
    }

    #[doc="Sets the INDEX field."]
    #[inline] pub fn set_index<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Invalidate Way"]
    #[inline] pub fn way(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if WAY != 0"]
    #[inline] pub fn test_way(&self) -> bool {
        self.way() != 0
    }

    #[doc="Sets the WAY field."]
    #[inline] pub fn set_way<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Maint1 {
    #[inline]
    fn from(other: u32) -> Self {
         Maint1(other)
    }
}

impl ::core::fmt::Display for Maint1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maint1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.index() != 0 { try!(write!(f, " index=0x{:x}", self.index()))}
        if self.way() != 0 { try!(write!(f, " way=0x{:x}", self.way()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Monitor Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcfg(pub u32);
impl Mcfg {
    #[doc="Cache Controller Monitor Counter Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Mcfg(other)
    }
}

impl ::core::fmt::Display for Mcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Monitor Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Men(pub u32);
impl Men {
    #[doc="Cache Controller Monitor Enable"]
    #[inline] pub fn menable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MENABLE != 0"]
    #[inline] pub fn test_menable(&self) -> bool {
        self.menable() != 0
    }

    #[doc="Sets the MENABLE field."]
    #[inline] pub fn set_menable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Men {
    #[inline]
    fn from(other: u32) -> Self {
         Men(other)
    }
}

impl ::core::fmt::Display for Men {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Men {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.menable() != 0 { try!(write!(f, " menable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Monitor Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mctrl(pub u32);
impl Mctrl {
    #[doc="Cache Controller Software Reset"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Mctrl(other)
    }
}

impl ::core::fmt::Display for Mctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Monitor Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc="Monitor Event Counter"]
    #[inline] pub fn event_cnt(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if EVENT_CNT != 0"]
    #[inline] pub fn test_event_cnt(&self) -> bool {
        self.event_cnt() != 0
    }

    #[doc="Sets the EVENT_CNT field."]
    #[inline] pub fn set_event_cnt<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Msr {
    #[inline]
    fn from(other: u32) -> Self {
         Msr(other)
    }
}

impl ::core::fmt::Display for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

