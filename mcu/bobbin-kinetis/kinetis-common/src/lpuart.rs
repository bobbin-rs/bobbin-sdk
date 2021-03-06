
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPUART Peripheral"]
pub struct LpuartPeriph(pub usize); 

impl LpuartPeriph {
    #[doc="Get the VERID Register."]
    #[inline] pub fn verid_reg(&self) -> ::bobbin_mcu::register::Register<Verid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Verid, 0x0)
    }

    #[doc="Get the *mut pointer for the VERID register."]
    #[inline] pub fn verid_mut(&self) -> *mut Verid { 
        self.verid_reg().ptr()
    }

    #[doc="Get the *const pointer for the VERID register."]
    #[inline] pub fn verid_ptr(&self) -> *const Verid { 
        self.verid_reg().ptr()
    }

    #[doc="Read the VERID register."]
    #[inline] pub fn verid(&self) -> Verid { 
        self.verid_reg().read()
    }

    #[doc="Get the PARAM Register."]
    #[inline] pub fn param_reg(&self) -> ::bobbin_mcu::register::Register<Param> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Param, 0x4)
    }

    #[doc="Get the *mut pointer for the PARAM register."]
    #[inline] pub fn param_mut(&self) -> *mut Param { 
        self.param_reg().ptr()
    }

    #[doc="Get the *const pointer for the PARAM register."]
    #[inline] pub fn param_ptr(&self) -> *const Param { 
        self.param_reg().ptr()
    }

    #[doc="Read the PARAM register."]
    #[inline] pub fn param(&self) -> Param { 
        self.param_reg().read()
    }

    #[doc="Get the GLOBAL Register."]
    #[inline] pub fn global_reg(&self) -> ::bobbin_mcu::register::Register<Global> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Global, 0x8)
    }

    #[doc="Get the *mut pointer for the GLOBAL register."]
    #[inline] pub fn global_mut(&self) -> *mut Global { 
        self.global_reg().ptr()
    }

    #[doc="Get the *const pointer for the GLOBAL register."]
    #[inline] pub fn global_ptr(&self) -> *const Global { 
        self.global_reg().ptr()
    }

    #[doc="Read the GLOBAL register."]
    #[inline] pub fn global(&self) -> Global { 
        self.global_reg().read()
    }

    #[doc="Write the GLOBAL register."]
    #[inline] pub fn write_global(&self, value: Global) -> &Self { 
        self.global_reg().write(value);
        self
    }

    #[doc="Set the GLOBAL register."]
    #[inline] pub fn set_global<F: FnOnce(Global) -> Global>(&self, f: F) -> &Self {
        self.global_reg().set(f);
        self
    }

    #[doc="Modify the GLOBAL register."]
    #[inline] pub fn with_global<F: FnOnce(Global) -> Global>(&self, f: F) -> &Self {
        self.global_reg().with(f);
        self
    }

    #[doc="Get the PINCFG Register."]
    #[inline] pub fn pincfg_reg(&self) -> ::bobbin_mcu::register::Register<Pincfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pincfg, 0xc)
    }

    #[doc="Get the *mut pointer for the PINCFG register."]
    #[inline] pub fn pincfg_mut(&self) -> *mut Pincfg { 
        self.pincfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the PINCFG register."]
    #[inline] pub fn pincfg_ptr(&self) -> *const Pincfg { 
        self.pincfg_reg().ptr()
    }

    #[doc="Read the PINCFG register."]
    #[inline] pub fn pincfg(&self) -> Pincfg { 
        self.pincfg_reg().read()
    }

    #[doc="Write the PINCFG register."]
    #[inline] pub fn write_pincfg(&self, value: Pincfg) -> &Self { 
        self.pincfg_reg().write(value);
        self
    }

    #[doc="Set the PINCFG register."]
    #[inline] pub fn set_pincfg<F: FnOnce(Pincfg) -> Pincfg>(&self, f: F) -> &Self {
        self.pincfg_reg().set(f);
        self
    }

    #[doc="Modify the PINCFG register."]
    #[inline] pub fn with_pincfg<F: FnOnce(Pincfg) -> Pincfg>(&self, f: F) -> &Self {
        self.pincfg_reg().with(f);
        self
    }

    #[doc="Get the BAUD Register."]
    #[inline] pub fn baud_reg(&self) -> ::bobbin_mcu::register::Register<Baud> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Baud, 0x10)
    }

    #[doc="Get the *mut pointer for the BAUD register."]
    #[inline] pub fn baud_mut(&self) -> *mut Baud { 
        self.baud_reg().ptr()
    }

    #[doc="Get the *const pointer for the BAUD register."]
    #[inline] pub fn baud_ptr(&self) -> *const Baud { 
        self.baud_reg().ptr()
    }

    #[doc="Read the BAUD register."]
    #[inline] pub fn baud(&self) -> Baud { 
        self.baud_reg().read()
    }

    #[doc="Write the BAUD register."]
    #[inline] pub fn write_baud(&self, value: Baud) -> &Self { 
        self.baud_reg().write(value);
        self
    }

    #[doc="Set the BAUD register."]
    #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        self.baud_reg().set(f);
        self
    }

    #[doc="Modify the BAUD register."]
    #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        self.baud_reg().with(f);
        self
    }

    #[doc="Get the STAT Register."]
    #[inline] pub fn stat_reg(&self) -> ::bobbin_mcu::register::Register<Stat> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Stat, 0x14)
    }

    #[doc="Get the *mut pointer for the STAT register."]
    #[inline] pub fn stat_mut(&self) -> *mut Stat { 
        self.stat_reg().ptr()
    }

    #[doc="Get the *const pointer for the STAT register."]
    #[inline] pub fn stat_ptr(&self) -> *const Stat { 
        self.stat_reg().ptr()
    }

    #[doc="Read the STAT register."]
    #[inline] pub fn stat(&self) -> Stat { 
        self.stat_reg().read()
    }

    #[doc="Write the STAT register."]
    #[inline] pub fn write_stat(&self, value: Stat) -> &Self { 
        self.stat_reg().write(value);
        self
    }

    #[doc="Set the STAT register."]
    #[inline] pub fn set_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
        self.stat_reg().set(f);
        self
    }

    #[doc="Modify the STAT register."]
    #[inline] pub fn with_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
        self.stat_reg().with(f);
        self
    }

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x18)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        self.ctrl_reg().read()
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

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().with(f);
        self
    }

    #[doc="Get the DATA Register."]
    #[inline] pub fn data_reg(&self) -> ::bobbin_mcu::register::Register<Data> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Data, 0x1c)
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut Data { 
        self.data_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const Data { 
        self.data_reg().ptr()
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        self.data_reg().read()
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn write_data(&self, value: Data) -> &Self { 
        self.data_reg().write(value);
        self
    }

    #[doc="Set the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().set(f);
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().with(f);
        self
    }

    #[doc="Get the MATCH Register."]
    #[inline] pub fn match_reg(&self) -> ::bobbin_mcu::register::Register<Match> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Match, 0x20)
    }

    #[doc="Get the *mut pointer for the MATCH register."]
    #[inline] pub fn match_mut(&self) -> *mut Match { 
        self.match_reg().ptr()
    }

    #[doc="Get the *const pointer for the MATCH register."]
    #[inline] pub fn match_ptr(&self) -> *const Match { 
        self.match_reg().ptr()
    }

    #[doc="Read the MATCH register."]
    #[inline] pub fn _match(&self) -> Match { 
        self.match_reg().read()
    }

    #[doc="Write the MATCH register."]
    #[inline] pub fn write_match(&self, value: Match) -> &Self { 
        self.match_reg().write(value);
        self
    }

    #[doc="Set the MATCH register."]
    #[inline] pub fn set_match<F: FnOnce(Match) -> Match>(&self, f: F) -> &Self {
        self.match_reg().set(f);
        self
    }

    #[doc="Modify the MATCH register."]
    #[inline] pub fn with_match<F: FnOnce(Match) -> Match>(&self, f: F) -> &Self {
        self.match_reg().with(f);
        self
    }

    #[doc="Get the MODIR Register."]
    #[inline] pub fn modir_reg(&self) -> ::bobbin_mcu::register::Register<Modir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Modir, 0x24)
    }

    #[doc="Get the *mut pointer for the MODIR register."]
    #[inline] pub fn modir_mut(&self) -> *mut Modir { 
        self.modir_reg().ptr()
    }

    #[doc="Get the *const pointer for the MODIR register."]
    #[inline] pub fn modir_ptr(&self) -> *const Modir { 
        self.modir_reg().ptr()
    }

    #[doc="Read the MODIR register."]
    #[inline] pub fn modir(&self) -> Modir { 
        self.modir_reg().read()
    }

    #[doc="Write the MODIR register."]
    #[inline] pub fn write_modir(&self, value: Modir) -> &Self { 
        self.modir_reg().write(value);
        self
    }

    #[doc="Set the MODIR register."]
    #[inline] pub fn set_modir<F: FnOnce(Modir) -> Modir>(&self, f: F) -> &Self {
        self.modir_reg().set(f);
        self
    }

    #[doc="Modify the MODIR register."]
    #[inline] pub fn with_modir<F: FnOnce(Modir) -> Modir>(&self, f: F) -> &Self {
        self.modir_reg().with(f);
        self
    }

    #[doc="Get the FIFO Register."]
    #[inline] pub fn fifo_reg(&self) -> ::bobbin_mcu::register::Register<Fifo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fifo, 0x28)
    }

    #[doc="Get the *mut pointer for the FIFO register."]
    #[inline] pub fn fifo_mut(&self) -> *mut Fifo { 
        self.fifo_reg().ptr()
    }

    #[doc="Get the *const pointer for the FIFO register."]
    #[inline] pub fn fifo_ptr(&self) -> *const Fifo { 
        self.fifo_reg().ptr()
    }

    #[doc="Read the FIFO register."]
    #[inline] pub fn fifo(&self) -> Fifo { 
        self.fifo_reg().read()
    }

    #[doc="Write the FIFO register."]
    #[inline] pub fn write_fifo(&self, value: Fifo) -> &Self { 
        self.fifo_reg().write(value);
        self
    }

    #[doc="Set the FIFO register."]
    #[inline] pub fn set_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
        self.fifo_reg().set(f);
        self
    }

    #[doc="Modify the FIFO register."]
    #[inline] pub fn with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
        self.fifo_reg().with(f);
        self
    }

    #[doc="Get the WATER Register."]
    #[inline] pub fn water_reg(&self) -> ::bobbin_mcu::register::Register<Water> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Water, 0x2c)
    }

    #[doc="Get the *mut pointer for the WATER register."]
    #[inline] pub fn water_mut(&self) -> *mut Water { 
        self.water_reg().ptr()
    }

    #[doc="Get the *const pointer for the WATER register."]
    #[inline] pub fn water_ptr(&self) -> *const Water { 
        self.water_reg().ptr()
    }

    #[doc="Read the WATER register."]
    #[inline] pub fn water(&self) -> Water { 
        self.water_reg().read()
    }

    #[doc="Write the WATER register."]
    #[inline] pub fn write_water(&self, value: Water) -> &Self { 
        self.water_reg().write(value);
        self
    }

    #[doc="Set the WATER register."]
    #[inline] pub fn set_water<F: FnOnce(Water) -> Water>(&self, f: F) -> &Self {
        self.water_reg().set(f);
        self
    }

    #[doc="Modify the WATER register."]
    #[inline] pub fn with_water<F: FnOnce(Water) -> Water>(&self, f: F) -> &Self {
        self.water_reg().with(f);
        self
    }

}

#[doc="Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="Feature Identification Number"]
    #[inline] pub fn feature(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FEATURE != 0"]
    #[inline] pub fn test_feature(&self) -> bool {
        self.feature() != 0
    }

    #[doc="Sets the FEATURE field."]
    #[inline] pub fn set_feature<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn minor(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MINOR != 0"]
    #[inline] pub fn test_minor(&self) -> bool {
        self.minor() != 0
    }

    #[doc="Sets the MINOR field."]
    #[inline] pub fn set_minor<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Major Version Number"]
    #[inline] pub fn major(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MAJOR != 0"]
    #[inline] pub fn test_major(&self) -> bool {
        self.major() != 0
    }

    #[doc="Sets the MAJOR field."]
    #[inline] pub fn set_major<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Verid {
    #[inline]
    fn from(other: u32) -> Self {
         Verid(other)
    }
}

impl ::core::fmt::Display for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
        if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
        if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="Transmit FIFO Size"]
    #[inline] pub fn txfifo(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TXFIFO != 0"]
    #[inline] pub fn test_txfifo(&self) -> bool {
        self.txfifo() != 0
    }

    #[doc="Sets the TXFIFO field."]
    #[inline] pub fn set_txfifo<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Size"]
    #[inline] pub fn rxfifo(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if RXFIFO != 0"]
    #[inline] pub fn test_rxfifo(&self) -> bool {
        self.rxfifo() != 0
    }

    #[doc="Sets the RXFIFO field."]
    #[inline] pub fn set_rxfifo<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Param {
    #[inline]
    fn from(other: u32) -> Self {
         Param(other)
    }
}

impl ::core::fmt::Display for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfifo() != 0 { try!(write!(f, " txfifo=0x{:x}", self.txfifo()))}
        if self.rxfifo() != 0 { try!(write!(f, " rxfifo=0x{:x}", self.rxfifo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Global Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Global(pub u32);
impl Global {
    #[doc="Software Reset"]
    #[inline] pub fn rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RST != 0"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Sets the RST field."]
    #[inline] pub fn set_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Global {
    #[inline]
    fn from(other: u32) -> Self {
         Global(other)
    }
}

impl ::core::fmt::Display for Global {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Global {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rst() != 0 { try!(write!(f, " rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Pin Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pincfg(pub u32);
impl Pincfg {
    #[doc="Trigger Select"]
    #[inline] pub fn trgsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if TRGSEL != 0"]
    #[inline] pub fn test_trgsel(&self) -> bool {
        self.trgsel() != 0
    }

    #[doc="Sets the TRGSEL field."]
    #[inline] pub fn set_trgsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pincfg {
    #[inline]
    fn from(other: u32) -> Self {
         Pincfg(other)
    }
}

impl ::core::fmt::Display for Pincfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pincfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trgsel() != 0 { try!(write!(f, " trgsel=0x{:x}", self.trgsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Baud Rate Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u32);
impl Baud {
    #[doc="Baud Rate Modulo Divisor."]
    #[inline] pub fn sbr(&self) -> ::bobbin_bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if SBR != 0"]
    #[inline] pub fn test_sbr(&self) -> bool {
        self.sbr() != 0
    }

    #[doc="Sets the SBR field."]
    #[inline] pub fn set_sbr<V: Into<::bobbin_bits::U13>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Stop Bit Number Select"]
    #[inline] pub fn sbns(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SBNS != 0"]
    #[inline] pub fn test_sbns(&self) -> bool {
        self.sbns() != 0
    }

    #[doc="Sets the SBNS field."]
    #[inline] pub fn set_sbns<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="RX Input Active Edge Interrupt Enable"]
    #[inline] pub fn rxedgie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RXEDGIE != 0"]
    #[inline] pub fn test_rxedgie(&self) -> bool {
        self.rxedgie() != 0
    }

    #[doc="Sets the RXEDGIE field."]
    #[inline] pub fn set_rxedgie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="LIN Break Detect Interrupt Enable"]
    #[inline] pub fn lbkdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if LBKDIE != 0"]
    #[inline] pub fn test_lbkdie(&self) -> bool {
        self.lbkdie() != 0
    }

    #[doc="Sets the LBKDIE field."]
    #[inline] pub fn set_lbkdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Resynchronization Disable"]
    #[inline] pub fn resyncdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RESYNCDIS != 0"]
    #[inline] pub fn test_resyncdis(&self) -> bool {
        self.resyncdis() != 0
    }

    #[doc="Sets the RESYNCDIS field."]
    #[inline] pub fn set_resyncdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Both Edge Sampling"]
    #[inline] pub fn bothedge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if BOTHEDGE != 0"]
    #[inline] pub fn test_bothedge(&self) -> bool {
        self.bothedge() != 0
    }

    #[doc="Sets the BOTHEDGE field."]
    #[inline] pub fn set_bothedge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Match Configuration"]
    #[inline] pub fn matcfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if MATCFG != 0"]
    #[inline] pub fn test_matcfg(&self) -> bool {
        self.matcfg() != 0
    }

    #[doc="Sets the MATCFG field."]
    #[inline] pub fn set_matcfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Receiver Idle DMA Enable"]
    #[inline] pub fn ridmae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if RIDMAE != 0"]
    #[inline] pub fn test_ridmae(&self) -> bool {
        self.ridmae() != 0
    }

    #[doc="Sets the RIDMAE field."]
    #[inline] pub fn set_ridmae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Receiver Full DMA Enable"]
    #[inline] pub fn rdmae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RDMAE != 0"]
    #[inline] pub fn test_rdmae(&self) -> bool {
        self.rdmae() != 0
    }

    #[doc="Sets the RDMAE field."]
    #[inline] pub fn set_rdmae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Transmitter DMA Enable"]
    #[inline] pub fn tdmae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TDMAE != 0"]
    #[inline] pub fn test_tdmae(&self) -> bool {
        self.tdmae() != 0
    }

    #[doc="Sets the TDMAE field."]
    #[inline] pub fn set_tdmae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Oversampling Ratio"]
    #[inline] pub fn osr(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if OSR != 0"]
    #[inline] pub fn test_osr(&self) -> bool {
        self.osr() != 0
    }

    #[doc="Sets the OSR field."]
    #[inline] pub fn set_osr<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="10-bit Mode select"]
    #[inline] pub fn m10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if M10 != 0"]
    #[inline] pub fn test_m10(&self) -> bool {
        self.m10() != 0
    }

    #[doc="Sets the M10 field."]
    #[inline] pub fn set_m10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Match Address Mode Enable 2"]
    #[inline] pub fn maen2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if MAEN2 != 0"]
    #[inline] pub fn test_maen2(&self) -> bool {
        self.maen2() != 0
    }

    #[doc="Sets the MAEN2 field."]
    #[inline] pub fn set_maen2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Match Address Mode Enable 1"]
    #[inline] pub fn maen1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if MAEN1 != 0"]
    #[inline] pub fn test_maen1(&self) -> bool {
        self.maen1() != 0
    }

    #[doc="Sets the MAEN1 field."]
    #[inline] pub fn set_maen1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Baud {
    #[inline]
    fn from(other: u32) -> Self {
         Baud(other)
    }
}

impl ::core::fmt::Display for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
        if self.sbns() != 0 { try!(write!(f, " sbns"))}
        if self.rxedgie() != 0 { try!(write!(f, " rxedgie"))}
        if self.lbkdie() != 0 { try!(write!(f, " lbkdie"))}
        if self.resyncdis() != 0 { try!(write!(f, " resyncdis"))}
        if self.bothedge() != 0 { try!(write!(f, " bothedge"))}
        if self.matcfg() != 0 { try!(write!(f, " matcfg=0x{:x}", self.matcfg()))}
        if self.ridmae() != 0 { try!(write!(f, " ridmae"))}
        if self.rdmae() != 0 { try!(write!(f, " rdmae"))}
        if self.tdmae() != 0 { try!(write!(f, " tdmae"))}
        if self.osr() != 0 { try!(write!(f, " osr=0x{:x}", self.osr()))}
        if self.m10() != 0 { try!(write!(f, " m10"))}
        if self.maen2() != 0 { try!(write!(f, " maen2"))}
        if self.maen1() != 0 { try!(write!(f, " maen1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc="Match 2 Flag"]
    #[inline] pub fn ma2f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if MA2F != 0"]
    #[inline] pub fn test_ma2f(&self) -> bool {
        self.ma2f() != 0
    }

    #[doc="Sets the MA2F field."]
    #[inline] pub fn set_ma2f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Match 1 Flag"]
    #[inline] pub fn ma1f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MA1F != 0"]
    #[inline] pub fn test_ma1f(&self) -> bool {
        self.ma1f() != 0
    }

    #[doc="Sets the MA1F field."]
    #[inline] pub fn set_ma1f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Parity Error Flag"]
    #[inline] pub fn pf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PF != 0"]
    #[inline] pub fn test_pf(&self) -> bool {
        self.pf() != 0
    }

    #[doc="Sets the PF field."]
    #[inline] pub fn set_pf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Framing Error Flag"]
    #[inline] pub fn fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FE != 0"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Sets the FE field."]
    #[inline] pub fn set_fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Noise Flag"]
    #[inline] pub fn nf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if NF != 0"]
    #[inline] pub fn test_nf(&self) -> bool {
        self.nf() != 0
    }

    #[doc="Sets the NF field."]
    #[inline] pub fn set_nf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Receiver Overrun Flag"]
    #[inline] pub fn or(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if OR != 0"]
    #[inline] pub fn test_or(&self) -> bool {
        self.or() != 0
    }

    #[doc="Sets the OR field."]
    #[inline] pub fn set_or<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Idle Line Flag"]
    #[inline] pub fn idle(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IDLE != 0"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Sets the IDLE field."]
    #[inline] pub fn set_idle<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Receive Data Register Full Flag"]
    #[inline] pub fn rdrf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RDRF != 0"]
    #[inline] pub fn test_rdrf(&self) -> bool {
        self.rdrf() != 0
    }

    #[doc="Sets the RDRF field."]
    #[inline] pub fn set_rdrf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Transmission Complete Flag"]
    #[inline] pub fn tc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if TC != 0"]
    #[inline] pub fn test_tc(&self) -> bool {
        self.tc() != 0
    }

    #[doc="Sets the TC field."]
    #[inline] pub fn set_tc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Transmit Data Register Empty Flag"]
    #[inline] pub fn tdre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TDRE != 0"]
    #[inline] pub fn test_tdre(&self) -> bool {
        self.tdre() != 0
    }

    #[doc="Sets the TDRE field."]
    #[inline] pub fn set_tdre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Receiver Active Flag"]
    #[inline] pub fn raf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RAF != 0"]
    #[inline] pub fn test_raf(&self) -> bool {
        self.raf() != 0
    }

    #[doc="Sets the RAF field."]
    #[inline] pub fn set_raf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="LIN Break Detection Enable"]
    #[inline] pub fn lbkde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if LBKDE != 0"]
    #[inline] pub fn test_lbkde(&self) -> bool {
        self.lbkde() != 0
    }

    #[doc="Sets the LBKDE field."]
    #[inline] pub fn set_lbkde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Break Character Generation Length"]
    #[inline] pub fn brk13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if BRK13 != 0"]
    #[inline] pub fn test_brk13(&self) -> bool {
        self.brk13() != 0
    }

    #[doc="Sets the BRK13 field."]
    #[inline] pub fn set_brk13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Receive Wake Up Idle Detect"]
    #[inline] pub fn rwuid(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if RWUID != 0"]
    #[inline] pub fn test_rwuid(&self) -> bool {
        self.rwuid() != 0
    }

    #[doc="Sets the RWUID field."]
    #[inline] pub fn set_rwuid<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Receive Data Inversion"]
    #[inline] pub fn rxinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if RXINV != 0"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Sets the RXINV field."]
    #[inline] pub fn set_rxinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="MSB First"]
    #[inline] pub fn msbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if MSBF != 0"]
    #[inline] pub fn test_msbf(&self) -> bool {
        self.msbf() != 0
    }

    #[doc="Sets the MSBF field."]
    #[inline] pub fn set_msbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="RXD Pin Active Edge Interrupt Flag"]
    #[inline] pub fn rxedgif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if RXEDGIF != 0"]
    #[inline] pub fn test_rxedgif(&self) -> bool {
        self.rxedgif() != 0
    }

    #[doc="Sets the RXEDGIF field."]
    #[inline] pub fn set_rxedgif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="LIN Break Detect Interrupt Flag"]
    #[inline] pub fn lbkdif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LBKDIF != 0"]
    #[inline] pub fn test_lbkdif(&self) -> bool {
        self.lbkdif() != 0
    }

    #[doc="Sets the LBKDIF field."]
    #[inline] pub fn set_lbkdif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Stat {
    #[inline]
    fn from(other: u32) -> Self {
         Stat(other)
    }
}

impl ::core::fmt::Display for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ma2f() != 0 { try!(write!(f, " ma2f"))}
        if self.ma1f() != 0 { try!(write!(f, " ma1f"))}
        if self.pf() != 0 { try!(write!(f, " pf"))}
        if self.fe() != 0 { try!(write!(f, " fe"))}
        if self.nf() != 0 { try!(write!(f, " nf"))}
        if self.or() != 0 { try!(write!(f, " or"))}
        if self.idle() != 0 { try!(write!(f, " idle"))}
        if self.rdrf() != 0 { try!(write!(f, " rdrf"))}
        if self.tc() != 0 { try!(write!(f, " tc"))}
        if self.tdre() != 0 { try!(write!(f, " tdre"))}
        if self.raf() != 0 { try!(write!(f, " raf"))}
        if self.lbkde() != 0 { try!(write!(f, " lbkde"))}
        if self.brk13() != 0 { try!(write!(f, " brk13"))}
        if self.rwuid() != 0 { try!(write!(f, " rwuid"))}
        if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
        if self.msbf() != 0 { try!(write!(f, " msbf"))}
        if self.rxedgif() != 0 { try!(write!(f, " rxedgif"))}
        if self.lbkdif() != 0 { try!(write!(f, " lbkdif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Parity Type"]
    #[inline] pub fn pt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PT != 0"]
    #[inline] pub fn test_pt(&self) -> bool {
        self.pt() != 0
    }

    #[doc="Sets the PT field."]
    #[inline] pub fn set_pt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Parity Enable"]
    #[inline] pub fn pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Idle Line Type Select"]
    #[inline] pub fn ilt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ILT != 0"]
    #[inline] pub fn test_ilt(&self) -> bool {
        self.ilt() != 0
    }

    #[doc="Sets the ILT field."]
    #[inline] pub fn set_ilt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver Wakeup Method Select"]
    #[inline] pub fn wake(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WAKE != 0"]
    #[inline] pub fn test_wake(&self) -> bool {
        self.wake() != 0
    }

    #[doc="Sets the WAKE field."]
    #[inline] pub fn set_wake<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="9-Bit or 8-Bit Mode Select"]
    #[inline] pub fn m(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if M != 0"]
    #[inline] pub fn test_m(&self) -> bool {
        self.m() != 0
    }

    #[doc="Sets the M field."]
    #[inline] pub fn set_m<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receiver Source Select"]
    #[inline] pub fn rsrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RSRC != 0"]
    #[inline] pub fn test_rsrc(&self) -> bool {
        self.rsrc() != 0
    }

    #[doc="Sets the RSRC field."]
    #[inline] pub fn set_rsrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Doze Enable"]
    #[inline] pub fn dozeen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DOZEEN != 0"]
    #[inline] pub fn test_dozeen(&self) -> bool {
        self.dozeen() != 0
    }

    #[doc="Sets the DOZEEN field."]
    #[inline] pub fn set_dozeen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loop Mode Select"]
    #[inline] pub fn loops(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOOPS != 0"]
    #[inline] pub fn test_loops(&self) -> bool {
        self.loops() != 0
    }

    #[doc="Sets the LOOPS field."]
    #[inline] pub fn set_loops<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Idle Configuration"]
    #[inline] pub fn idlecfg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if IDLECFG != 0"]
    #[inline] pub fn test_idlecfg(&self) -> bool {
        self.idlecfg() != 0
    }

    #[doc="Sets the IDLECFG field."]
    #[inline] pub fn set_idlecfg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="7-Bit Mode Select"]
    #[inline] pub fn m7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if M7 != 0"]
    #[inline] pub fn test_m7(&self) -> bool {
        self.m7() != 0
    }

    #[doc="Sets the M7 field."]
    #[inline] pub fn set_m7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Match 2 Interrupt Enable"]
    #[inline] pub fn ma2ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if MA2IE != 0"]
    #[inline] pub fn test_ma2ie(&self) -> bool {
        self.ma2ie() != 0
    }

    #[doc="Sets the MA2IE field."]
    #[inline] pub fn set_ma2ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Match 1 Interrupt Enable"]
    #[inline] pub fn ma1ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MA1IE != 0"]
    #[inline] pub fn test_ma1ie(&self) -> bool {
        self.ma1ie() != 0
    }

    #[doc="Sets the MA1IE field."]
    #[inline] pub fn set_ma1ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Send Break"]
    #[inline] pub fn sbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SBK != 0"]
    #[inline] pub fn test_sbk(&self) -> bool {
        self.sbk() != 0
    }

    #[doc="Sets the SBK field."]
    #[inline] pub fn set_sbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receiver Wakeup Control"]
    #[inline] pub fn rwu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RWU != 0"]
    #[inline] pub fn test_rwu(&self) -> bool {
        self.rwu() != 0
    }

    #[doc="Sets the RWU field."]
    #[inline] pub fn set_rwu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn re(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RE != 0"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Sets the RE field."]
    #[inline] pub fn set_re<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn te(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if TE != 0"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te() != 0
    }

    #[doc="Sets the TE field."]
    #[inline] pub fn set_te<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Idle Line Interrupt Enable"]
    #[inline] pub fn ilie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ILIE != 0"]
    #[inline] pub fn test_ilie(&self) -> bool {
        self.ilie() != 0
    }

    #[doc="Sets the ILIE field."]
    #[inline] pub fn set_ilie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Receiver Interrupt Enable"]
    #[inline] pub fn rie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RIE != 0"]
    #[inline] pub fn test_rie(&self) -> bool {
        self.rie() != 0
    }

    #[doc="Sets the RIE field."]
    #[inline] pub fn set_rie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Transmission Complete Interrupt Enable for"]
    #[inline] pub fn tcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Transmit Interrupt Enable"]
    #[inline] pub fn tie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Parity Error Interrupt Enable"]
    #[inline] pub fn peie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PEIE != 0"]
    #[inline] pub fn test_peie(&self) -> bool {
        self.peie() != 0
    }

    #[doc="Sets the PEIE field."]
    #[inline] pub fn set_peie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Framing Error Interrupt Enable"]
    #[inline] pub fn feie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FEIE != 0"]
    #[inline] pub fn test_feie(&self) -> bool {
        self.feie() != 0
    }

    #[doc="Sets the FEIE field."]
    #[inline] pub fn set_feie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Noise Error Interrupt Enable"]
    #[inline] pub fn neie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if NEIE != 0"]
    #[inline] pub fn test_neie(&self) -> bool {
        self.neie() != 0
    }

    #[doc="Sets the NEIE field."]
    #[inline] pub fn set_neie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn orie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if ORIE != 0"]
    #[inline] pub fn test_orie(&self) -> bool {
        self.orie() != 0
    }

    #[doc="Sets the ORIE field."]
    #[inline] pub fn set_orie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Transmit Data Inversion"]
    #[inline] pub fn txinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TXINV != 0"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Sets the TXINV field."]
    #[inline] pub fn set_txinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="TXD Pin Direction in Single-Wire Mode"]
    #[inline] pub fn txdir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if TXDIR != 0"]
    #[inline] pub fn test_txdir(&self) -> bool {
        self.txdir() != 0
    }

    #[doc="Sets the TXDIR field."]
    #[inline] pub fn set_txdir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Receive Bit 9 / Transmit Bit 8"]
    #[inline] pub fn r9t8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if R9T8 != 0"]
    #[inline] pub fn test_r9t8(&self) -> bool {
        self.r9t8() != 0
    }

    #[doc="Sets the R9T8 field."]
    #[inline] pub fn set_r9t8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Receive Bit 8 / Transmit Bit 9"]
    #[inline] pub fn r8t9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if R8T9 != 0"]
    #[inline] pub fn test_r8t9(&self) -> bool {
        self.r8t9() != 0
    }

    #[doc="Sets the R8T9 field."]
    #[inline] pub fn set_r8t9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pt() != 0 { try!(write!(f, " pt"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        if self.ilt() != 0 { try!(write!(f, " ilt"))}
        if self.wake() != 0 { try!(write!(f, " wake"))}
        if self.m() != 0 { try!(write!(f, " m"))}
        if self.rsrc() != 0 { try!(write!(f, " rsrc"))}
        if self.dozeen() != 0 { try!(write!(f, " dozeen"))}
        if self.loops() != 0 { try!(write!(f, " loops"))}
        if self.idlecfg() != 0 { try!(write!(f, " idlecfg=0x{:x}", self.idlecfg()))}
        if self.m7() != 0 { try!(write!(f, " m7"))}
        if self.ma2ie() != 0 { try!(write!(f, " ma2ie"))}
        if self.ma1ie() != 0 { try!(write!(f, " ma1ie"))}
        if self.sbk() != 0 { try!(write!(f, " sbk"))}
        if self.rwu() != 0 { try!(write!(f, " rwu"))}
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.te() != 0 { try!(write!(f, " te"))}
        if self.ilie() != 0 { try!(write!(f, " ilie"))}
        if self.rie() != 0 { try!(write!(f, " rie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.peie() != 0 { try!(write!(f, " peie"))}
        if self.feie() != 0 { try!(write!(f, " feie"))}
        if self.neie() != 0 { try!(write!(f, " neie"))}
        if self.orie() != 0 { try!(write!(f, " orie"))}
        if self.txinv() != 0 { try!(write!(f, " txinv"))}
        if self.txdir() != 0 { try!(write!(f, " txdir"))}
        if self.r9t8() != 0 { try!(write!(f, " r9t8"))}
        if self.r8t9() != 0 { try!(write!(f, " r8t9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="RT"]
    #[inline] pub fn rt(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if RT != 0"]
    #[inline] pub fn test_rt(&self) -> bool {
        self.rt() != 0
    }

    #[doc="Sets the RT field."]
    #[inline] pub fn set_rt<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Idle Line"]
    #[inline] pub fn idline(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if IDLINE != 0"]
    #[inline] pub fn test_idline(&self) -> bool {
        self.idline() != 0
    }

    #[doc="Sets the IDLINE field."]
    #[inline] pub fn set_idline<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Receive Buffer Empty"]
    #[inline] pub fn rxempt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if RXEMPT != 0"]
    #[inline] pub fn test_rxempt(&self) -> bool {
        self.rxempt() != 0
    }

    #[doc="Sets the RXEMPT field."]
    #[inline] pub fn set_rxempt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Frame Error / Transmit Special Character"]
    #[inline] pub fn fretsc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FRETSC != 0"]
    #[inline] pub fn test_fretsc(&self) -> bool {
        self.fretsc() != 0
    }

    #[doc="Sets the FRETSC field."]
    #[inline] pub fn set_fretsc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="PARITYE"]
    #[inline] pub fn paritye(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PARITYE != 0"]
    #[inline] pub fn test_paritye(&self) -> bool {
        self.paritye() != 0
    }

    #[doc="Sets the PARITYE field."]
    #[inline] pub fn set_paritye<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="NOISY"]
    #[inline] pub fn noisy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if NOISY != 0"]
    #[inline] pub fn test_noisy(&self) -> bool {
        self.noisy() != 0
    }

    #[doc="Sets the NOISY field."]
    #[inline] pub fn set_noisy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Data {
    #[inline]
    fn from(other: u32) -> Self {
         Data(other)
    }
}

impl ::core::fmt::Display for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rt() != 0 { try!(write!(f, " rt=0x{:x}", self.rt()))}
        if self.idline() != 0 { try!(write!(f, " idline"))}
        if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
        if self.fretsc() != 0 { try!(write!(f, " fretsc"))}
        if self.paritye() != 0 { try!(write!(f, " paritye"))}
        if self.noisy() != 0 { try!(write!(f, " noisy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Match Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Match(pub u32);
impl Match {
    #[doc="Match Address 1"]
    #[inline] pub fn ma1(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if MA1 != 0"]
    #[inline] pub fn test_ma1(&self) -> bool {
        self.ma1() != 0
    }

    #[doc="Sets the MA1 field."]
    #[inline] pub fn set_ma1<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Match Address 2"]
    #[inline] pub fn ma2(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if MA2 != 0"]
    #[inline] pub fn test_ma2(&self) -> bool {
        self.ma2() != 0
    }

    #[doc="Sets the MA2 field."]
    #[inline] pub fn set_ma2<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Match {
    #[inline]
    fn from(other: u32) -> Self {
         Match(other)
    }
}

impl ::core::fmt::Display for Match {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Match {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ma1() != 0 { try!(write!(f, " ma1=0x{:x}", self.ma1()))}
        if self.ma2() != 0 { try!(write!(f, " ma2=0x{:x}", self.ma2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Modem IrDA Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Modir(pub u32);
impl Modir {
    #[doc="Transmitter clear-to-send enable"]
    #[inline] pub fn txctse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXCTSE != 0"]
    #[inline] pub fn test_txctse(&self) -> bool {
        self.txctse() != 0
    }

    #[doc="Sets the TXCTSE field."]
    #[inline] pub fn set_txctse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmitter request-to-send enable"]
    #[inline] pub fn txrtse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXRTSE != 0"]
    #[inline] pub fn test_txrtse(&self) -> bool {
        self.txrtse() != 0
    }

    #[doc="Sets the TXRTSE field."]
    #[inline] pub fn set_txrtse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmitter request-to-send polarity"]
    #[inline] pub fn txrtspol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXRTSPOL != 0"]
    #[inline] pub fn test_txrtspol(&self) -> bool {
        self.txrtspol() != 0
    }

    #[doc="Sets the TXRTSPOL field."]
    #[inline] pub fn set_txrtspol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver request-to-send enable"]
    #[inline] pub fn rxrtse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXRTSE != 0"]
    #[inline] pub fn test_rxrtse(&self) -> bool {
        self.rxrtse() != 0
    }

    #[doc="Sets the RXRTSE field."]
    #[inline] pub fn set_rxrtse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit CTS Configuration"]
    #[inline] pub fn txctsc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXCTSC != 0"]
    #[inline] pub fn test_txctsc(&self) -> bool {
        self.txctsc() != 0
    }

    #[doc="Sets the TXCTSC field."]
    #[inline] pub fn set_txctsc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit CTS Source"]
    #[inline] pub fn txctssrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXCTSSRC != 0"]
    #[inline] pub fn test_txctssrc(&self) -> bool {
        self.txctssrc() != 0
    }

    #[doc="Sets the TXCTSSRC field."]
    #[inline] pub fn set_txctssrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive RTS Configuration"]
    #[inline] pub fn rtswater(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if RTSWATER != 0"]
    #[inline] pub fn test_rtswater(&self) -> bool {
        self.rtswater() != 0
    }

    #[doc="Sets the RTSWATER field."]
    #[inline] pub fn set_rtswater<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmitter narrow pulse"]
    #[inline] pub fn tnp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if TNP != 0"]
    #[inline] pub fn test_tnp(&self) -> bool {
        self.tnp() != 0
    }

    #[doc="Sets the TNP field."]
    #[inline] pub fn set_tnp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Infrared enable"]
    #[inline] pub fn iren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Modir {
    #[inline]
    fn from(other: u32) -> Self {
         Modir(other)
    }
}

impl ::core::fmt::Display for Modir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Modir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txctse() != 0 { try!(write!(f, " txctse"))}
        if self.txrtse() != 0 { try!(write!(f, " txrtse"))}
        if self.txrtspol() != 0 { try!(write!(f, " txrtspol"))}
        if self.rxrtse() != 0 { try!(write!(f, " rxrtse"))}
        if self.txctsc() != 0 { try!(write!(f, " txctsc"))}
        if self.txctssrc() != 0 { try!(write!(f, " txctssrc"))}
        if self.rtswater() != 0 { try!(write!(f, " rtswater=0x{:x}", self.rtswater()))}
        if self.tnp() != 0 { try!(write!(f, " tnp=0x{:x}", self.tnp()))}
        if self.iren() != 0 { try!(write!(f, " iren"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART FIFO Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc="Receive FIFO. Buffer Depth"]
    #[inline] pub fn rxfifosize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RXFIFOSIZE != 0"]
    #[inline] pub fn test_rxfifosize(&self) -> bool {
        self.rxfifosize() != 0
    }

    #[doc="Sets the RXFIFOSIZE field."]
    #[inline] pub fn set_rxfifosize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Enable"]
    #[inline] pub fn rxfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFE != 0"]
    #[inline] pub fn test_rxfe(&self) -> bool {
        self.rxfe() != 0
    }

    #[doc="Sets the RXFE field."]
    #[inline] pub fn set_rxfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit FIFO. Buffer Depth"]
    #[inline] pub fn txfifosize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if TXFIFOSIZE != 0"]
    #[inline] pub fn test_txfifosize(&self) -> bool {
        self.txfifosize() != 0
    }

    #[doc="Sets the TXFIFOSIZE field."]
    #[inline] pub fn set_txfifosize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit FIFO Enable"]
    #[inline] pub fn txfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Underflow Interrupt Enable"]
    #[inline] pub fn rxufe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RXUFE != 0"]
    #[inline] pub fn test_rxufe(&self) -> bool {
        self.rxufe() != 0
    }

    #[doc="Sets the RXUFE field."]
    #[inline] pub fn set_rxufe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmit FIFO Overflow Interrupt Enable"]
    #[inline] pub fn txofe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXOFE != 0"]
    #[inline] pub fn test_txofe(&self) -> bool {
        self.txofe() != 0
    }

    #[doc="Sets the TXOFE field."]
    #[inline] pub fn set_txofe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Receiver Idle Empty Enable"]
    #[inline] pub fn rxiden(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x7) as u8) } // [12:10]
    }

    #[doc="Returns true if RXIDEN != 0"]
    #[inline] pub fn test_rxiden(&self) -> bool {
        self.rxiden() != 0
    }

    #[doc="Sets the RXIDEN field."]
    #[inline] pub fn set_rxiden<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO/Buffer Flush"]
    #[inline] pub fn rxflush(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RXFLUSH != 0"]
    #[inline] pub fn test_rxflush(&self) -> bool {
        self.rxflush() != 0
    }

    #[doc="Sets the RXFLUSH field."]
    #[inline] pub fn set_rxflush<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Transmit FIFO/Buffer Flush"]
    #[inline] pub fn txflush(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TXFLUSH != 0"]
    #[inline] pub fn test_txflush(&self) -> bool {
        self.txflush() != 0
    }

    #[doc="Sets the TXFLUSH field."]
    #[inline] pub fn set_txflush<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Receiver Buffer Underflow Flag"]
    #[inline] pub fn rxuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Transmitter Buffer Overflow Flag"]
    #[inline] pub fn txof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Receive Buffer/FIFO Empty"]
    #[inline] pub fn rxempt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if RXEMPT != 0"]
    #[inline] pub fn test_rxempt(&self) -> bool {
        self.rxempt() != 0
    }

    #[doc="Sets the RXEMPT field."]
    #[inline] pub fn set_rxempt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Transmit Buffer/FIFO Empty"]
    #[inline] pub fn txempt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TXEMPT != 0"]
    #[inline] pub fn test_txempt(&self) -> bool {
        self.txempt() != 0
    }

    #[doc="Sets the TXEMPT field."]
    #[inline] pub fn set_txempt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Fifo {
    #[inline]
    fn from(other: u32) -> Self {
         Fifo(other)
    }
}

impl ::core::fmt::Display for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxfifosize() != 0 { try!(write!(f, " rxfifosize=0x{:x}", self.rxfifosize()))}
        if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
        if self.txfifosize() != 0 { try!(write!(f, " txfifosize=0x{:x}", self.txfifosize()))}
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.rxufe() != 0 { try!(write!(f, " rxufe"))}
        if self.txofe() != 0 { try!(write!(f, " txofe"))}
        if self.rxiden() != 0 { try!(write!(f, " rxiden=0x{:x}", self.rxiden()))}
        if self.rxflush() != 0 { try!(write!(f, " rxflush"))}
        if self.txflush() != 0 { try!(write!(f, " txflush"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
        if self.txempt() != 0 { try!(write!(f, " txempt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Watermark Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Water(pub u32);
impl Water {
    #[doc="Transmit Watermark"]
    #[inline] pub fn txwater(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if TXWATER != 0"]
    #[inline] pub fn test_txwater(&self) -> bool {
        self.txwater() != 0
    }

    #[doc="Sets the TXWATER field."]
    #[inline] pub fn set_txwater<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Counter"]
    #[inline] pub fn txcount(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if TXCOUNT != 0"]
    #[inline] pub fn test_txcount(&self) -> bool {
        self.txcount() != 0
    }

    #[doc="Sets the TXCOUNT field."]
    #[inline] pub fn set_txcount<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive Watermark"]
    #[inline] pub fn rxwater(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if RXWATER != 0"]
    #[inline] pub fn test_rxwater(&self) -> bool {
        self.rxwater() != 0
    }

    #[doc="Sets the RXWATER field."]
    #[inline] pub fn set_rxwater<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receive Counter"]
    #[inline] pub fn rxcount(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if RXCOUNT != 0"]
    #[inline] pub fn test_rxcount(&self) -> bool {
        self.rxcount() != 0
    }

    #[doc="Sets the RXCOUNT field."]
    #[inline] pub fn set_rxcount<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Water {
    #[inline]
    fn from(other: u32) -> Self {
         Water(other)
    }
}

impl ::core::fmt::Display for Water {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Water {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
        if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
        if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
        if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

