
::bobbin_mcu::periph!( ETHERNET_MMC, EthernetMmc, ETHERNET_MMC_PERIPH, EthernetMmcPeriph, ETHERNET_MMC_OWNED, ETHERNET_MMC_REF_COUNT, 0x40028100, 0x00, 0x06);


#[doc="Ethernet: MAC management counters"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetMmcPeriph(pub usize);
impl EthernetMmcPeriph {
    #[doc="Get the MMCCR Register."]
    #[inline] pub fn mmccr_reg(&self) -> ::bobbin_mcu::register::Register<Mmccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmccr, 0x0)
    }

    #[doc="Get the *mut pointer for the MMCCR register."]
    #[inline] pub fn mmccr_mut(&self) -> *mut Mmccr { 
        self.mmccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCCR register."]
    #[inline] pub fn mmccr_ptr(&self) -> *const Mmccr { 
        self.mmccr_reg().ptr()
    }

    #[doc="Read the MMCCR register."]
    #[inline] pub fn mmccr(&self) -> Mmccr { 
        self.mmccr_reg().read()
    }

    #[doc="Write the MMCCR register."]
    #[inline] pub fn write_mmccr(&self, value: Mmccr) -> &Self { 
        self.mmccr_reg().write(value);
        self
    }

    #[doc="Set the MMCCR register."]
    #[inline] pub fn set_mmccr<F: FnOnce(Mmccr) -> Mmccr>(&self, f: F) -> &Self {
        self.mmccr_reg().set(f);
        self
    }

    #[doc="Modify the MMCCR register."]
    #[inline] pub fn with_mmccr<F: FnOnce(Mmccr) -> Mmccr>(&self, f: F) -> &Self {
        self.mmccr_reg().with(f);
        self
    }

    #[doc="Get the MMCRIR Register."]
    #[inline] pub fn mmcrir_reg(&self) -> ::bobbin_mcu::register::Register<Mmcrir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmcrir, 0x4)
    }

    #[doc="Get the *mut pointer for the MMCRIR register."]
    #[inline] pub fn mmcrir_mut(&self) -> *mut Mmcrir { 
        self.mmcrir_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCRIR register."]
    #[inline] pub fn mmcrir_ptr(&self) -> *const Mmcrir { 
        self.mmcrir_reg().ptr()
    }

    #[doc="Read the MMCRIR register."]
    #[inline] pub fn mmcrir(&self) -> Mmcrir { 
        self.mmcrir_reg().read()
    }

    #[doc="Write the MMCRIR register."]
    #[inline] pub fn write_mmcrir(&self, value: Mmcrir) -> &Self { 
        self.mmcrir_reg().write(value);
        self
    }

    #[doc="Set the MMCRIR register."]
    #[inline] pub fn set_mmcrir<F: FnOnce(Mmcrir) -> Mmcrir>(&self, f: F) -> &Self {
        self.mmcrir_reg().set(f);
        self
    }

    #[doc="Modify the MMCRIR register."]
    #[inline] pub fn with_mmcrir<F: FnOnce(Mmcrir) -> Mmcrir>(&self, f: F) -> &Self {
        self.mmcrir_reg().with(f);
        self
    }

    #[doc="Get the MMCTIR Register."]
    #[inline] pub fn mmctir_reg(&self) -> ::bobbin_mcu::register::Register<Mmctir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmctir, 0x8)
    }

    #[doc="Get the *mut pointer for the MMCTIR register."]
    #[inline] pub fn mmctir_mut(&self) -> *mut Mmctir { 
        self.mmctir_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCTIR register."]
    #[inline] pub fn mmctir_ptr(&self) -> *const Mmctir { 
        self.mmctir_reg().ptr()
    }

    #[doc="Read the MMCTIR register."]
    #[inline] pub fn mmctir(&self) -> Mmctir { 
        self.mmctir_reg().read()
    }

    #[doc="Get the MMCRIMR Register."]
    #[inline] pub fn mmcrimr_reg(&self) -> ::bobbin_mcu::register::Register<Mmcrimr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmcrimr, 0xc)
    }

    #[doc="Get the *mut pointer for the MMCRIMR register."]
    #[inline] pub fn mmcrimr_mut(&self) -> *mut Mmcrimr { 
        self.mmcrimr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCRIMR register."]
    #[inline] pub fn mmcrimr_ptr(&self) -> *const Mmcrimr { 
        self.mmcrimr_reg().ptr()
    }

    #[doc="Read the MMCRIMR register."]
    #[inline] pub fn mmcrimr(&self) -> Mmcrimr { 
        self.mmcrimr_reg().read()
    }

    #[doc="Write the MMCRIMR register."]
    #[inline] pub fn write_mmcrimr(&self, value: Mmcrimr) -> &Self { 
        self.mmcrimr_reg().write(value);
        self
    }

    #[doc="Set the MMCRIMR register."]
    #[inline] pub fn set_mmcrimr<F: FnOnce(Mmcrimr) -> Mmcrimr>(&self, f: F) -> &Self {
        self.mmcrimr_reg().set(f);
        self
    }

    #[doc="Modify the MMCRIMR register."]
    #[inline] pub fn with_mmcrimr<F: FnOnce(Mmcrimr) -> Mmcrimr>(&self, f: F) -> &Self {
        self.mmcrimr_reg().with(f);
        self
    }

    #[doc="Get the MMCTIMR Register."]
    #[inline] pub fn mmctimr_reg(&self) -> ::bobbin_mcu::register::Register<Mmctimr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmctimr, 0x10)
    }

    #[doc="Get the *mut pointer for the MMCTIMR register."]
    #[inline] pub fn mmctimr_mut(&self) -> *mut Mmctimr { 
        self.mmctimr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCTIMR register."]
    #[inline] pub fn mmctimr_ptr(&self) -> *const Mmctimr { 
        self.mmctimr_reg().ptr()
    }

    #[doc="Read the MMCTIMR register."]
    #[inline] pub fn mmctimr(&self) -> Mmctimr { 
        self.mmctimr_reg().read()
    }

    #[doc="Write the MMCTIMR register."]
    #[inline] pub fn write_mmctimr(&self, value: Mmctimr) -> &Self { 
        self.mmctimr_reg().write(value);
        self
    }

    #[doc="Set the MMCTIMR register."]
    #[inline] pub fn set_mmctimr<F: FnOnce(Mmctimr) -> Mmctimr>(&self, f: F) -> &Self {
        self.mmctimr_reg().set(f);
        self
    }

    #[doc="Modify the MMCTIMR register."]
    #[inline] pub fn with_mmctimr<F: FnOnce(Mmctimr) -> Mmctimr>(&self, f: F) -> &Self {
        self.mmctimr_reg().with(f);
        self
    }

    #[doc="Get the MMCTGFSCCR Register."]
    #[inline] pub fn mmctgfsccr_reg(&self) -> ::bobbin_mcu::register::Register<Mmctgfsccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmctgfsccr, 0x4c)
    }

    #[doc="Get the *mut pointer for the MMCTGFSCCR register."]
    #[inline] pub fn mmctgfsccr_mut(&self) -> *mut Mmctgfsccr { 
        self.mmctgfsccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCTGFSCCR register."]
    #[inline] pub fn mmctgfsccr_ptr(&self) -> *const Mmctgfsccr { 
        self.mmctgfsccr_reg().ptr()
    }

    #[doc="Read the MMCTGFSCCR register."]
    #[inline] pub fn mmctgfsccr(&self) -> Mmctgfsccr { 
        self.mmctgfsccr_reg().read()
    }

    #[doc="Get the MMCTGFMSCCR Register."]
    #[inline] pub fn mmctgfmsccr_reg(&self) -> ::bobbin_mcu::register::Register<Mmctgfmsccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmctgfmsccr, 0x50)
    }

    #[doc="Get the *mut pointer for the MMCTGFMSCCR register."]
    #[inline] pub fn mmctgfmsccr_mut(&self) -> *mut Mmctgfmsccr { 
        self.mmctgfmsccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCTGFMSCCR register."]
    #[inline] pub fn mmctgfmsccr_ptr(&self) -> *const Mmctgfmsccr { 
        self.mmctgfmsccr_reg().ptr()
    }

    #[doc="Read the MMCTGFMSCCR register."]
    #[inline] pub fn mmctgfmsccr(&self) -> Mmctgfmsccr { 
        self.mmctgfmsccr_reg().read()
    }

    #[doc="Get the MMCTGFCR Register."]
    #[inline] pub fn mmctgfcr_reg(&self) -> ::bobbin_mcu::register::Register<Mmctgfcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmctgfcr, 0x68)
    }

    #[doc="Get the *mut pointer for the MMCTGFCR register."]
    #[inline] pub fn mmctgfcr_mut(&self) -> *mut Mmctgfcr { 
        self.mmctgfcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCTGFCR register."]
    #[inline] pub fn mmctgfcr_ptr(&self) -> *const Mmctgfcr { 
        self.mmctgfcr_reg().ptr()
    }

    #[doc="Read the MMCTGFCR register."]
    #[inline] pub fn mmctgfcr(&self) -> Mmctgfcr { 
        self.mmctgfcr_reg().read()
    }

    #[doc="Get the MMCRFCECR Register."]
    #[inline] pub fn mmcrfcecr_reg(&self) -> ::bobbin_mcu::register::Register<Mmcrfcecr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmcrfcecr, 0x94)
    }

    #[doc="Get the *mut pointer for the MMCRFCECR register."]
    #[inline] pub fn mmcrfcecr_mut(&self) -> *mut Mmcrfcecr { 
        self.mmcrfcecr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCRFCECR register."]
    #[inline] pub fn mmcrfcecr_ptr(&self) -> *const Mmcrfcecr { 
        self.mmcrfcecr_reg().ptr()
    }

    #[doc="Read the MMCRFCECR register."]
    #[inline] pub fn mmcrfcecr(&self) -> Mmcrfcecr { 
        self.mmcrfcecr_reg().read()
    }

    #[doc="Get the MMCRFAECR Register."]
    #[inline] pub fn mmcrfaecr_reg(&self) -> ::bobbin_mcu::register::Register<Mmcrfaecr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmcrfaecr, 0x98)
    }

    #[doc="Get the *mut pointer for the MMCRFAECR register."]
    #[inline] pub fn mmcrfaecr_mut(&self) -> *mut Mmcrfaecr { 
        self.mmcrfaecr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCRFAECR register."]
    #[inline] pub fn mmcrfaecr_ptr(&self) -> *const Mmcrfaecr { 
        self.mmcrfaecr_reg().ptr()
    }

    #[doc="Read the MMCRFAECR register."]
    #[inline] pub fn mmcrfaecr(&self) -> Mmcrfaecr { 
        self.mmcrfaecr_reg().read()
    }

    #[doc="Get the MMCRGUFCR Register."]
    #[inline] pub fn mmcrgufcr_reg(&self) -> ::bobbin_mcu::register::Register<Mmcrgufcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mmcrgufcr, 0xc4)
    }

    #[doc="Get the *mut pointer for the MMCRGUFCR register."]
    #[inline] pub fn mmcrgufcr_mut(&self) -> *mut Mmcrgufcr { 
        self.mmcrgufcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MMCRGUFCR register."]
    #[inline] pub fn mmcrgufcr_ptr(&self) -> *const Mmcrgufcr { 
        self.mmcrgufcr_reg().ptr()
    }

    #[doc="Read the MMCRGUFCR register."]
    #[inline] pub fn mmcrgufcr(&self) -> Mmcrgufcr { 
        self.mmcrgufcr_reg().read()
    }

}

#[doc="Ethernet MMC control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmccr(pub u32);
impl Mmccr {
    #[doc="no description available"]
    #[inline] pub fn cr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CR != 0"]
    #[inline] pub fn test_cr(&self) -> bool {
        self.cr() != 0
    }

    #[doc="Sets the CR field."]
    #[inline] pub fn set_cr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn csr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CSR != 0"]
    #[inline] pub fn test_csr(&self) -> bool {
        self.csr() != 0
    }

    #[doc="Sets the CSR field."]
    #[inline] pub fn set_csr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ror(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ROR != 0"]
    #[inline] pub fn test_ror(&self) -> bool {
        self.ror() != 0
    }

    #[doc="Sets the ROR field."]
    #[inline] pub fn set_ror<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MCF != 0"]
    #[inline] pub fn test_mcf(&self) -> bool {
        self.mcf() != 0
    }

    #[doc="Sets the MCF field."]
    #[inline] pub fn set_mcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mcp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MCP != 0"]
    #[inline] pub fn test_mcp(&self) -> bool {
        self.mcp() != 0
    }

    #[doc="Sets the MCP field."]
    #[inline] pub fn set_mcp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mcfhp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MCFHP != 0"]
    #[inline] pub fn test_mcfhp(&self) -> bool {
        self.mcfhp() != 0
    }

    #[doc="Sets the MCFHP field."]
    #[inline] pub fn set_mcfhp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Mmccr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmccr(other)
    }
}

impl ::core::fmt::Display for Mmccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmccr {
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

#[doc="Ethernet MMC receive interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrir(pub u32);
impl Mmcrir {
    #[doc="no description available"]
    #[inline] pub fn rfces(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RFCES != 0"]
    #[inline] pub fn test_rfces(&self) -> bool {
        self.rfces() != 0
    }

    #[doc="Sets the RFCES field."]
    #[inline] pub fn set_rfces<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rfaes(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RFAES != 0"]
    #[inline] pub fn test_rfaes(&self) -> bool {
        self.rfaes() != 0
    }

    #[doc="Sets the RFAES field."]
    #[inline] pub fn set_rfaes<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rgufs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RGUFS != 0"]
    #[inline] pub fn test_rgufs(&self) -> bool {
        self.rgufs() != 0
    }

    #[doc="Sets the RGUFS field."]
    #[inline] pub fn set_rgufs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Mmcrir {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrir(other)
    }
}

impl ::core::fmt::Display for Mmcrir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfces() != 0 { try!(write!(f, " rfces"))}
        if self.rfaes() != 0 { try!(write!(f, " rfaes"))}
        if self.rgufs() != 0 { try!(write!(f, " rgufs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmit interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctir(pub u32);
impl Mmctir {
    #[doc="no description available"]
    #[inline] pub fn tgfscs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TGFSCS != 0"]
    #[inline] pub fn test_tgfscs(&self) -> bool {
        self.tgfscs() != 0
    }

    #[doc="Sets the TGFSCS field."]
    #[inline] pub fn set_tgfscs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tgfmscs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TGFMSCS != 0"]
    #[inline] pub fn test_tgfmscs(&self) -> bool {
        self.tgfmscs() != 0
    }

    #[doc="Sets the TGFMSCS field."]
    #[inline] pub fn set_tgfmscs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tgfs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if TGFS != 0"]
    #[inline] pub fn test_tgfs(&self) -> bool {
        self.tgfs() != 0
    }

    #[doc="Sets the TGFS field."]
    #[inline] pub fn set_tgfs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

}

impl From<u32> for Mmctir {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctir(other)
    }
}

impl ::core::fmt::Display for Mmctir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tgfscs() != 0 { try!(write!(f, " tgfscs"))}
        if self.tgfmscs() != 0 { try!(write!(f, " tgfmscs"))}
        if self.tgfs() != 0 { try!(write!(f, " tgfs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC receive interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrimr(pub u32);
impl Mmcrimr {
    #[doc="no description available"]
    #[inline] pub fn rfcem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RFCEM != 0"]
    #[inline] pub fn test_rfcem(&self) -> bool {
        self.rfcem() != 0
    }

    #[doc="Sets the RFCEM field."]
    #[inline] pub fn set_rfcem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rfaem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RFAEM != 0"]
    #[inline] pub fn test_rfaem(&self) -> bool {
        self.rfaem() != 0
    }

    #[doc="Sets the RFAEM field."]
    #[inline] pub fn set_rfaem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rgufm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RGUFM != 0"]
    #[inline] pub fn test_rgufm(&self) -> bool {
        self.rgufm() != 0
    }

    #[doc="Sets the RGUFM field."]
    #[inline] pub fn set_rgufm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Mmcrimr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrimr(other)
    }
}

impl ::core::fmt::Display for Mmcrimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfcem() != 0 { try!(write!(f, " rfcem"))}
        if self.rfaem() != 0 { try!(write!(f, " rfaem"))}
        if self.rgufm() != 0 { try!(write!(f, " rgufm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmit interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctimr(pub u32);
impl Mmctimr {
    #[doc="no description available"]
    #[inline] pub fn tgfscm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TGFSCM != 0"]
    #[inline] pub fn test_tgfscm(&self) -> bool {
        self.tgfscm() != 0
    }

    #[doc="Sets the TGFSCM field."]
    #[inline] pub fn set_tgfscm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tgfmscm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TGFMSCM != 0"]
    #[inline] pub fn test_tgfmscm(&self) -> bool {
        self.tgfmscm() != 0
    }

    #[doc="Sets the TGFMSCM field."]
    #[inline] pub fn set_tgfmscm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tgfm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TGFM != 0"]
    #[inline] pub fn test_tgfm(&self) -> bool {
        self.tgfm() != 0
    }

    #[doc="Sets the TGFM field."]
    #[inline] pub fn set_tgfm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Mmctimr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctimr(other)
    }
}

impl ::core::fmt::Display for Mmctimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tgfscm() != 0 { try!(write!(f, " tgfscm"))}
        if self.tgfmscm() != 0 { try!(write!(f, " tgfmscm"))}
        if self.tgfm() != 0 { try!(write!(f, " tgfm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmitted good frames after a single collision counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctgfsccr(pub u32);
impl Mmctgfsccr {
    #[doc="no description available"]
    #[inline] pub fn tgfscc(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TGFSCC != 0"]
    #[inline] pub fn test_tgfscc(&self) -> bool {
        self.tgfscc() != 0
    }

    #[doc="Sets the TGFSCC field."]
    #[inline] pub fn set_tgfscc<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmctgfsccr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctgfsccr(other)
    }
}

impl ::core::fmt::Display for Mmctgfsccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctgfsccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmitted good frames after more than a single collision"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctgfmsccr(pub u32);
impl Mmctgfmsccr {
    #[doc="no description available"]
    #[inline] pub fn tgfmscc(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TGFMSCC != 0"]
    #[inline] pub fn test_tgfmscc(&self) -> bool {
        self.tgfmscc() != 0
    }

    #[doc="Sets the TGFMSCC field."]
    #[inline] pub fn set_tgfmscc<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmctgfmsccr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctgfmsccr(other)
    }
}

impl ::core::fmt::Display for Mmctgfmsccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctgfmsccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmitted good frames counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctgfcr(pub u32);
impl Mmctgfcr {
    #[doc="HTL"]
    #[inline] pub fn tgfc(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TGFC != 0"]
    #[inline] pub fn test_tgfc(&self) -> bool {
        self.tgfc() != 0
    }

    #[doc="Sets the TGFC field."]
    #[inline] pub fn set_tgfc<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmctgfcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctgfcr(other)
    }
}

impl ::core::fmt::Display for Mmctgfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctgfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC received frames with CRC error counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrfcecr(pub u32);
impl Mmcrfcecr {
    #[doc="no description available"]
    #[inline] pub fn rfcfc(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RFCFC != 0"]
    #[inline] pub fn test_rfcfc(&self) -> bool {
        self.rfcfc() != 0
    }

    #[doc="Sets the RFCFC field."]
    #[inline] pub fn set_rfcfc<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmcrfcecr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrfcecr(other)
    }
}

impl ::core::fmt::Display for Mmcrfcecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrfcecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC received frames with alignment error counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrfaecr(pub u32);
impl Mmcrfaecr {
    #[doc="no description available"]
    #[inline] pub fn rfaec(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RFAEC != 0"]
    #[inline] pub fn test_rfaec(&self) -> bool {
        self.rfaec() != 0
    }

    #[doc="Sets the RFAEC field."]
    #[inline] pub fn set_rfaec<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmcrfaecr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrfaecr(other)
    }
}

impl ::core::fmt::Display for Mmcrfaecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrfaecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MMC received good unicast frames counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrgufcr(pub u32);
impl Mmcrgufcr {
    #[doc="no description available"]
    #[inline] pub fn rgufc(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RGUFC != 0"]
    #[inline] pub fn test_rgufc(&self) -> bool {
        self.rgufc() != 0
    }

    #[doc="Sets the RGUFC field."]
    #[inline] pub fn set_rgufc<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmcrgufcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrgufcr(other)
    }
}

impl ::core::fmt::Display for Mmcrgufcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrgufcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

