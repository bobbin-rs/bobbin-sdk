::bobbin_mcu::periph!( ICM, Icm, ICM_PERIPH, IcmPeriph, ICM_OWNED, ICM_REF_COUNT, 0x42002c00, 0x00, 0x0e);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ICM Peripheral"]
pub struct IcmPeriph(pub usize); 

impl IcmPeriph {
    #[doc="Get the CFG Register."]
    #[inline] pub fn cfg_reg(&self) -> ::bobbin_mcu::register::Register<Cfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfg, 0x0)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x4)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x8)
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

    #[doc="Get the IER Register."]
    #[inline] pub fn ier_reg(&self) -> ::bobbin_mcu::register::Register<Ier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ier, 0x10)
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Write the IER register."]
    #[inline] pub fn write_ier(&self, value: Ier) -> &Self { 
        self.ier_reg().write(value);
        self
    }

    #[doc="Set the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().set(f);
        self
    }

    #[doc="Get the IDR Register."]
    #[inline] pub fn idr_reg(&self) -> ::bobbin_mcu::register::Register<Idr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Idr, 0x14)
    }

    #[doc="Get the *mut pointer for the IDR register."]
    #[inline] pub fn idr_mut(&self) -> *mut Idr { 
        self.idr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IDR register."]
    #[inline] pub fn idr_ptr(&self) -> *const Idr { 
        self.idr_reg().ptr()
    }

    #[doc="Write the IDR register."]
    #[inline] pub fn write_idr(&self, value: Idr) -> &Self { 
        self.idr_reg().write(value);
        self
    }

    #[doc="Set the IDR register."]
    #[inline] pub fn set_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Self {
        self.idr_reg().set(f);
        self
    }

    #[doc="Get the IMR Register."]
    #[inline] pub fn imr_reg(&self) -> ::bobbin_mcu::register::Register<Imr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Imr, 0x18)
    }

    #[doc="Get the *mut pointer for the IMR register."]
    #[inline] pub fn imr_mut(&self) -> *mut Imr { 
        self.imr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IMR register."]
    #[inline] pub fn imr_ptr(&self) -> *const Imr { 
        self.imr_reg().ptr()
    }

    #[doc="Read the IMR register."]
    #[inline] pub fn imr(&self) -> Imr { 
        self.imr_reg().read()
    }

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x1c)
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        self.isr_reg().read()
    }

    #[doc="Get the UASR Register."]
    #[inline] pub fn uasr_reg(&self) -> ::bobbin_mcu::register::Register<Uasr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Uasr, 0x20)
    }

    #[doc="Get the *mut pointer for the UASR register."]
    #[inline] pub fn uasr_mut(&self) -> *mut Uasr { 
        self.uasr_reg().ptr()
    }

    #[doc="Get the *const pointer for the UASR register."]
    #[inline] pub fn uasr_ptr(&self) -> *const Uasr { 
        self.uasr_reg().ptr()
    }

    #[doc="Read the UASR register."]
    #[inline] pub fn uasr(&self) -> Uasr { 
        self.uasr_reg().read()
    }

    #[doc="Get the DSCR Register."]
    #[inline] pub fn dscr_reg(&self) -> ::bobbin_mcu::register::Register<Dscr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dscr, 0x30)
    }

    #[doc="Get the *mut pointer for the DSCR register."]
    #[inline] pub fn dscr_mut(&self) -> *mut Dscr { 
        self.dscr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DSCR register."]
    #[inline] pub fn dscr_ptr(&self) -> *const Dscr { 
        self.dscr_reg().ptr()
    }

    #[doc="Read the DSCR register."]
    #[inline] pub fn dscr(&self) -> Dscr { 
        self.dscr_reg().read()
    }

    #[doc="Write the DSCR register."]
    #[inline] pub fn write_dscr(&self, value: Dscr) -> &Self { 
        self.dscr_reg().write(value);
        self
    }

    #[doc="Set the DSCR register."]
    #[inline] pub fn set_dscr<F: FnOnce(Dscr) -> Dscr>(&self, f: F) -> &Self {
        self.dscr_reg().set(f);
        self
    }

    #[doc="Modify the DSCR register."]
    #[inline] pub fn with_dscr<F: FnOnce(Dscr) -> Dscr>(&self, f: F) -> &Self {
        self.dscr_reg().with(f);
        self
    }

    #[doc="Get the HASH Register."]
    #[inline] pub fn hash_reg(&self) -> ::bobbin_mcu::register::Register<Hash> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hash, 0x34)
    }

    #[doc="Get the *mut pointer for the HASH register."]
    #[inline] pub fn hash_mut(&self) -> *mut Hash { 
        self.hash_reg().ptr()
    }

    #[doc="Get the *const pointer for the HASH register."]
    #[inline] pub fn hash_ptr(&self) -> *const Hash { 
        self.hash_reg().ptr()
    }

    #[doc="Read the HASH register."]
    #[inline] pub fn hash(&self) -> Hash { 
        self.hash_reg().read()
    }

    #[doc="Write the HASH register."]
    #[inline] pub fn write_hash(&self, value: Hash) -> &Self { 
        self.hash_reg().write(value);
        self
    }

    #[doc="Set the HASH register."]
    #[inline] pub fn set_hash<F: FnOnce(Hash) -> Hash>(&self, f: F) -> &Self {
        self.hash_reg().set(f);
        self
    }

    #[doc="Modify the HASH register."]
    #[inline] pub fn with_hash<F: FnOnce(Hash) -> Hash>(&self, f: F) -> &Self {
        self.hash_reg().with(f);
        self
    }

    #[doc="Get the UIHVAL Register."]
    #[inline] pub fn uihval_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Uihval, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Uihval, 0x38, 0x4)
    }

    #[doc="Get the *mut pointer for the UIHVAL register."]
    #[inline] pub fn uihval_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Uihval { 
        self.uihval_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the UIHVAL register."]
    #[inline] pub fn uihval_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Uihval { 
        self.uihval_reg().ptr(index.into())
    }

    #[doc="Write the UIHVAL register."]
    #[inline] pub fn write_uihval<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Uihval) -> &Self {
        self.uihval_reg().write(index.into(), value);
        self
    }

    #[doc="Set the UIHVAL register."]
    #[inline] pub fn set_uihval<I: Into<::bobbin_bits::R8>, F: FnOnce(Uihval) -> Uihval>(&self, index: I, f: F) -> &Self {
        self.uihval_reg().set(index.into(), f);
        self
    }

}

#[doc="Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="Write Back Disable"]
    #[inline] pub fn wbdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WBDIS != 0"]
    #[inline] pub fn test_wbdis(&self) -> bool {
        self.wbdis() != 0
    }

    #[doc="Sets the WBDIS field."]
    #[inline] pub fn set_wbdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="End of Monitoring Disable"]
    #[inline] pub fn eomdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EOMDIS != 0"]
    #[inline] pub fn test_eomdis(&self) -> bool {
        self.eomdis() != 0
    }

    #[doc="Sets the EOMDIS field."]
    #[inline] pub fn set_eomdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Secondary List Branching Disable"]
    #[inline] pub fn slbdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SLBDIS != 0"]
    #[inline] pub fn test_slbdis(&self) -> bool {
        self.slbdis() != 0
    }

    #[doc="Sets the SLBDIS field."]
    #[inline] pub fn set_slbdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Bus Burden Control"]
    #[inline] pub fn bbc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if BBC != 0"]
    #[inline] pub fn test_bbc(&self) -> bool {
        self.bbc() != 0
    }

    #[doc="Sets the BBC field."]
    #[inline] pub fn set_bbc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Automatic Switch To Compare Digest"]
    #[inline] pub fn ascd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ASCD != 0"]
    #[inline] pub fn test_ascd(&self) -> bool {
        self.ascd() != 0
    }

    #[doc="Sets the ASCD field."]
    #[inline] pub fn set_ascd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Dual Input Buffer"]
    #[inline] pub fn dualbuff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DUALBUFF != 0"]
    #[inline] pub fn test_dualbuff(&self) -> bool {
        self.dualbuff() != 0
    }

    #[doc="Sets the DUALBUFF field."]
    #[inline] pub fn set_dualbuff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="User Initial Hash Value"]
    #[inline] pub fn uihash(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if UIHASH != 0"]
    #[inline] pub fn test_uihash(&self) -> bool {
        self.uihash() != 0
    }

    #[doc="Sets the UIHASH field."]
    #[inline] pub fn set_uihash<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="User SHA Algorithm"]
    #[inline] pub fn ualgo(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if UALGO != 0"]
    #[inline] pub fn test_ualgo(&self) -> bool {
        self.ualgo() != 0
    }

    #[doc="Sets the UALGO field."]
    #[inline] pub fn set_ualgo<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Region Hash Area Protection"]
    #[inline] pub fn haprot(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if HAPROT != 0"]
    #[inline] pub fn test_haprot(&self) -> bool {
        self.haprot() != 0
    }

    #[doc="Sets the HAPROT field."]
    #[inline] pub fn set_haprot<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Region Descriptor Area Protection"]
    #[inline] pub fn daprot(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if DAPROT != 0"]
    #[inline] pub fn test_daprot(&self) -> bool {
        self.daprot() != 0
    }

    #[doc="Sets the DAPROT field."]
    #[inline] pub fn set_daprot<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
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
        if self.wbdis() != 0 { try!(write!(f, " wbdis"))}
        if self.eomdis() != 0 { try!(write!(f, " eomdis"))}
        if self.slbdis() != 0 { try!(write!(f, " slbdis"))}
        if self.bbc() != 0 { try!(write!(f, " bbc=0x{:x}", self.bbc()))}
        if self.ascd() != 0 { try!(write!(f, " ascd"))}
        if self.dualbuff() != 0 { try!(write!(f, " dualbuff"))}
        if self.uihash() != 0 { try!(write!(f, " uihash"))}
        if self.ualgo() != 0 { try!(write!(f, " ualgo=0x{:x}", self.ualgo()))}
        if self.haprot() != 0 { try!(write!(f, " haprot=0x{:x}", self.haprot()))}
        if self.daprot() != 0 { try!(write!(f, " daprot=0x{:x}", self.daprot()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="ICM Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ICM Disable Register"]
    #[inline] pub fn disable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DISABLE != 0"]
    #[inline] pub fn test_disable(&self) -> bool {
        self.disable() != 0
    }

    #[doc="Sets the DISABLE field."]
    #[inline] pub fn set_disable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Recompute Internal Hash"]
    #[inline] pub fn rehash(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if REHASH != 0"]
    #[inline] pub fn test_rehash(&self) -> bool {
        self.rehash() != 0
    }

    #[doc="Sets the REHASH field."]
    #[inline] pub fn set_rehash<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Region Monitoring Disable"]
    #[inline] pub fn rmdis(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if RMDIS != 0"]
    #[inline] pub fn test_rmdis(&self) -> bool {
        self.rmdis() != 0
    }

    #[doc="Sets the RMDIS field."]
    #[inline] pub fn set_rmdis<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Region Monitoring Enable"]
    #[inline] pub fn rmen(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if RMEN != 0"]
    #[inline] pub fn test_rmen(&self) -> bool {
        self.rmen() != 0
    }

    #[doc="Sets the RMEN field."]
    #[inline] pub fn set_rmen<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
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
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.disable() != 0 { try!(write!(f, " disable"))}
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.rehash() != 0 { try!(write!(f, " rehash=0x{:x}", self.rehash()))}
        if self.rmdis() != 0 { try!(write!(f, " rmdis=0x{:x}", self.rmdis()))}
        if self.rmen() != 0 { try!(write!(f, " rmen=0x{:x}", self.rmen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="ICM Controller Enable Register"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RAW Region Monitoring Disabled Status"]
    #[inline] pub fn rawrmdis(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if RAWRMDIS != 0"]
    #[inline] pub fn test_rawrmdis(&self) -> bool {
        self.rawrmdis() != 0
    }

    #[doc="Sets the RAWRMDIS field."]
    #[inline] pub fn set_rawrmdis<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Region Monitoring Disabled Status"]
    #[inline] pub fn rmdis(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if RMDIS != 0"]
    #[inline] pub fn test_rmdis(&self) -> bool {
        self.rmdis() != 0
    }

    #[doc="Sets the RMDIS field."]
    #[inline] pub fn set_rmdis<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
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
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.rawrmdis() != 0 { try!(write!(f, " rawrmdis=0x{:x}", self.rawrmdis()))}
        if self.rmdis() != 0 { try!(write!(f, " rmdis=0x{:x}", self.rmdis()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Region Hash Completed Interrupt Enable"]
    #[inline] pub fn rhc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if RHC != 0"]
    #[inline] pub fn test_rhc(&self) -> bool {
        self.rhc() != 0
    }

    #[doc="Sets the RHC field."]
    #[inline] pub fn set_rhc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Region Digest Mismatch Interrupt Enable"]
    #[inline] pub fn rdm(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if RDM != 0"]
    #[inline] pub fn test_rdm(&self) -> bool {
        self.rdm() != 0
    }

    #[doc="Sets the RDM field."]
    #[inline] pub fn set_rdm<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Region Bus Error Interrupt Enable"]
    #[inline] pub fn rbe(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if RBE != 0"]
    #[inline] pub fn test_rbe(&self) -> bool {
        self.rbe() != 0
    }

    #[doc="Sets the RBE field."]
    #[inline] pub fn set_rbe<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Region Wrap Condition detected Interrupt Enable"]
    #[inline] pub fn rwc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if RWC != 0"]
    #[inline] pub fn test_rwc(&self) -> bool {
        self.rwc() != 0
    }

    #[doc="Sets the RWC field."]
    #[inline] pub fn set_rwc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Region End bit Condition Detected Interrupt Enable"]
    #[inline] pub fn rec(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if REC != 0"]
    #[inline] pub fn test_rec(&self) -> bool {
        self.rec() != 0
    }

    #[doc="Sets the REC field."]
    #[inline] pub fn set_rec<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Region Status Updated Interrupt Disable"]
    #[inline] pub fn rsu(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if RSU != 0"]
    #[inline] pub fn test_rsu(&self) -> bool {
        self.rsu() != 0
    }

    #[doc="Sets the RSU field."]
    #[inline] pub fn set_rsu<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Undefined Register Access Detection Interrupt Enable"]
    #[inline] pub fn urad(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if URAD != 0"]
    #[inline] pub fn test_urad(&self) -> bool {
        self.urad() != 0
    }

    #[doc="Sets the URAD field."]
    #[inline] pub fn set_urad<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rhc() != 0 { try!(write!(f, " rhc=0x{:x}", self.rhc()))}
        if self.rdm() != 0 { try!(write!(f, " rdm=0x{:x}", self.rdm()))}
        if self.rbe() != 0 { try!(write!(f, " rbe=0x{:x}", self.rbe()))}
        if self.rwc() != 0 { try!(write!(f, " rwc=0x{:x}", self.rwc()))}
        if self.rec() != 0 { try!(write!(f, " rec=0x{:x}", self.rec()))}
        if self.rsu() != 0 { try!(write!(f, " rsu=0x{:x}", self.rsu()))}
        if self.urad() != 0 { try!(write!(f, " urad"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Disable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc="Region Hash Completed Interrupt Disable"]
    #[inline] pub fn rhc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if RHC != 0"]
    #[inline] pub fn test_rhc(&self) -> bool {
        self.rhc() != 0
    }

    #[doc="Sets the RHC field."]
    #[inline] pub fn set_rhc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Region Digest Mismatch Interrupt Disable"]
    #[inline] pub fn rdm(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if RDM != 0"]
    #[inline] pub fn test_rdm(&self) -> bool {
        self.rdm() != 0
    }

    #[doc="Sets the RDM field."]
    #[inline] pub fn set_rdm<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Region Bus Error Interrupt Disable"]
    #[inline] pub fn rbe(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if RBE != 0"]
    #[inline] pub fn test_rbe(&self) -> bool {
        self.rbe() != 0
    }

    #[doc="Sets the RBE field."]
    #[inline] pub fn set_rbe<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Region Wrap Condition Detected Interrupt Disable"]
    #[inline] pub fn rwc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if RWC != 0"]
    #[inline] pub fn test_rwc(&self) -> bool {
        self.rwc() != 0
    }

    #[doc="Sets the RWC field."]
    #[inline] pub fn set_rwc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Region End bit Condition detected Interrupt Disable"]
    #[inline] pub fn rec(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if REC != 0"]
    #[inline] pub fn test_rec(&self) -> bool {
        self.rec() != 0
    }

    #[doc="Sets the REC field."]
    #[inline] pub fn set_rec<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Region Status Updated Interrupt Disable"]
    #[inline] pub fn rsu(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if RSU != 0"]
    #[inline] pub fn test_rsu(&self) -> bool {
        self.rsu() != 0
    }

    #[doc="Sets the RSU field."]
    #[inline] pub fn set_rsu<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Undefined Register Access Detection Interrupt Disable"]
    #[inline] pub fn urad(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if URAD != 0"]
    #[inline] pub fn test_urad(&self) -> bool {
        self.urad() != 0
    }

    #[doc="Sets the URAD field."]
    #[inline] pub fn set_urad<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Idr {
    #[inline]
    fn from(other: u32) -> Self {
         Idr(other)
    }
}

impl ::core::fmt::Display for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rhc() != 0 { try!(write!(f, " rhc=0x{:x}", self.rhc()))}
        if self.rdm() != 0 { try!(write!(f, " rdm=0x{:x}", self.rdm()))}
        if self.rbe() != 0 { try!(write!(f, " rbe=0x{:x}", self.rbe()))}
        if self.rwc() != 0 { try!(write!(f, " rwc=0x{:x}", self.rwc()))}
        if self.rec() != 0 { try!(write!(f, " rec=0x{:x}", self.rec()))}
        if self.rsu() != 0 { try!(write!(f, " rsu=0x{:x}", self.rsu()))}
        if self.urad() != 0 { try!(write!(f, " urad"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc="Region Hash Completed Interrupt Mask"]
    #[inline] pub fn rhc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if RHC != 0"]
    #[inline] pub fn test_rhc(&self) -> bool {
        self.rhc() != 0
    }

    #[doc="Sets the RHC field."]
    #[inline] pub fn set_rhc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Region Digest Mismatch Interrupt Mask"]
    #[inline] pub fn rdm(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if RDM != 0"]
    #[inline] pub fn test_rdm(&self) -> bool {
        self.rdm() != 0
    }

    #[doc="Sets the RDM field."]
    #[inline] pub fn set_rdm<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Region Bus Error Interrupt Mask"]
    #[inline] pub fn rbe(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if RBE != 0"]
    #[inline] pub fn test_rbe(&self) -> bool {
        self.rbe() != 0
    }

    #[doc="Sets the RBE field."]
    #[inline] pub fn set_rbe<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Region Wrap Condition Detected Interrupt Mask"]
    #[inline] pub fn rwc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if RWC != 0"]
    #[inline] pub fn test_rwc(&self) -> bool {
        self.rwc() != 0
    }

    #[doc="Sets the RWC field."]
    #[inline] pub fn set_rwc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Region End bit Condition Detected Interrupt Mask"]
    #[inline] pub fn rec(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if REC != 0"]
    #[inline] pub fn test_rec(&self) -> bool {
        self.rec() != 0
    }

    #[doc="Sets the REC field."]
    #[inline] pub fn set_rec<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Region Status Updated Interrupt Mask"]
    #[inline] pub fn rsu(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if RSU != 0"]
    #[inline] pub fn test_rsu(&self) -> bool {
        self.rsu() != 0
    }

    #[doc="Sets the RSU field."]
    #[inline] pub fn set_rsu<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Undefined Register Access Detection Interrupt Mask"]
    #[inline] pub fn urad(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if URAD != 0"]
    #[inline] pub fn test_urad(&self) -> bool {
        self.urad() != 0
    }

    #[doc="Sets the URAD field."]
    #[inline] pub fn set_urad<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Imr {
    #[inline]
    fn from(other: u32) -> Self {
         Imr(other)
    }
}

impl ::core::fmt::Display for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rhc() != 0 { try!(write!(f, " rhc=0x{:x}", self.rhc()))}
        if self.rdm() != 0 { try!(write!(f, " rdm=0x{:x}", self.rdm()))}
        if self.rbe() != 0 { try!(write!(f, " rbe=0x{:x}", self.rbe()))}
        if self.rwc() != 0 { try!(write!(f, " rwc=0x{:x}", self.rwc()))}
        if self.rec() != 0 { try!(write!(f, " rec=0x{:x}", self.rec()))}
        if self.rsu() != 0 { try!(write!(f, " rsu=0x{:x}", self.rsu()))}
        if self.urad() != 0 { try!(write!(f, " urad"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Region Hash Completed"]
    #[inline] pub fn rhc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if RHC != 0"]
    #[inline] pub fn test_rhc(&self) -> bool {
        self.rhc() != 0
    }

    #[doc="Sets the RHC field."]
    #[inline] pub fn set_rhc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Region Digest Mismatch"]
    #[inline] pub fn rdm(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if RDM != 0"]
    #[inline] pub fn test_rdm(&self) -> bool {
        self.rdm() != 0
    }

    #[doc="Sets the RDM field."]
    #[inline] pub fn set_rdm<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Region Bus Error"]
    #[inline] pub fn rbe(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if RBE != 0"]
    #[inline] pub fn test_rbe(&self) -> bool {
        self.rbe() != 0
    }

    #[doc="Sets the RBE field."]
    #[inline] pub fn set_rbe<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Region Wrap Condition Detected"]
    #[inline] pub fn rwc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if RWC != 0"]
    #[inline] pub fn test_rwc(&self) -> bool {
        self.rwc() != 0
    }

    #[doc="Sets the RWC field."]
    #[inline] pub fn set_rwc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Region End bit Condition Detected"]
    #[inline] pub fn rec(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if REC != 0"]
    #[inline] pub fn test_rec(&self) -> bool {
        self.rec() != 0
    }

    #[doc="Sets the REC field."]
    #[inline] pub fn set_rec<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Region Status Updated Detected"]
    #[inline] pub fn rsu(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if RSU != 0"]
    #[inline] pub fn test_rsu(&self) -> bool {
        self.rsu() != 0
    }

    #[doc="Sets the RSU field."]
    #[inline] pub fn set_rsu<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Undefined Register Access Detection Status"]
    #[inline] pub fn urad(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if URAD != 0"]
    #[inline] pub fn test_urad(&self) -> bool {
        self.urad() != 0
    }

    #[doc="Sets the URAD field."]
    #[inline] pub fn set_urad<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Isr(other)
    }
}

impl ::core::fmt::Display for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rhc() != 0 { try!(write!(f, " rhc=0x{:x}", self.rhc()))}
        if self.rdm() != 0 { try!(write!(f, " rdm=0x{:x}", self.rdm()))}
        if self.rbe() != 0 { try!(write!(f, " rbe=0x{:x}", self.rbe()))}
        if self.rwc() != 0 { try!(write!(f, " rwc=0x{:x}", self.rwc()))}
        if self.rec() != 0 { try!(write!(f, " rec=0x{:x}", self.rec()))}
        if self.rsu() != 0 { try!(write!(f, " rsu=0x{:x}", self.rsu()))}
        if self.urad() != 0 { try!(write!(f, " urad"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Undefined Access Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uasr(pub u32);
impl Uasr {
    #[doc="Undefined Register Access Trace"]
    #[inline] pub fn urat(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if URAT != 0"]
    #[inline] pub fn test_urat(&self) -> bool {
        self.urat() != 0
    }

    #[doc="Sets the URAT field."]
    #[inline] pub fn set_urat<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uasr {
    #[inline]
    fn from(other: u32) -> Self {
         Uasr(other)
    }
}

impl ::core::fmt::Display for Uasr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uasr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.urat() != 0 { try!(write!(f, " urat=0x{:x}", self.urat()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Region Descriptor Area Start Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dscr(pub u32);
impl Dscr {
    #[doc="Descriptor Area Start Address"]
    #[inline] pub fn dasa(&self) -> ::bobbin_bits::U26 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3ffffff) as u32) } // [31:6]
    }

    #[doc="Returns true if DASA != 0"]
    #[inline] pub fn test_dasa(&self) -> bool {
        self.dasa() != 0
    }

    #[doc="Sets the DASA field."]
    #[inline] pub fn set_dasa<V: Into<::bobbin_bits::U26>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U26 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffffff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Dscr {
    #[inline]
    fn from(other: u32) -> Self {
         Dscr(other)
    }
}

impl ::core::fmt::Display for Dscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dasa() != 0 { try!(write!(f, " dasa=0x{:x}", self.dasa()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Region Hash Area Start Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hash(pub u32);
impl Hash {
    #[doc="Hash Area Start Address"]
    #[inline] pub fn hasa(&self) -> ::bobbin_bits::U25 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1ffffff) as u32) } // [31:7]
    }

    #[doc="Returns true if HASA != 0"]
    #[inline] pub fn test_hasa(&self) -> bool {
        self.hasa() != 0
    }

    #[doc="Sets the HASA field."]
    #[inline] pub fn set_hasa<V: Into<::bobbin_bits::U25>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U25 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ffffff << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Hash {
    #[inline]
    fn from(other: u32) -> Self {
         Hash(other)
    }
}

impl ::core::fmt::Display for Hash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hasa() != 0 { try!(write!(f, " hasa=0x{:x}", self.hasa()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="User Initial Hash Value n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uihval(pub u32);
impl Uihval {
    #[doc="Initial Hash Value"]
    #[inline] pub fn val(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VAL != 0"]
    #[inline] pub fn test_val(&self) -> bool {
        self.val() != 0
    }

    #[doc="Sets the VAL field."]
    #[inline] pub fn set_val<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uihval {
    #[inline]
    fn from(other: u32) -> Self {
         Uihval(other)
    }
}

impl ::core::fmt::Display for Uihval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uihval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

