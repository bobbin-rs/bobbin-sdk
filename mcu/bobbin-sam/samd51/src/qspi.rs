::bobbin_mcu::periph!( QSPI, Qspi, QSPI_PERIPH, QspiPeriph, QSPI_OWNED, QSPI_REF_COUNT, 0x42003400, 0x00, 0x1a);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="QSPI Peripheral"]
pub struct QspiPeriph(pub usize); 

impl QspiPeriph {
    #[doc="Get the CTRLA Register."]
    #[inline] pub fn ctrla_reg(&self) -> ::bobbin_mcu::register::Register<Ctrla> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrla, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        self.ctrla_reg().read()
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn write_ctrla(&self, value: Ctrla) -> &Self { 
        self.ctrla_reg().write(value);
        self
    }

    #[doc="Set the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().set(f);
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().with(f);
        self
    }

    #[doc="Get the CTRLB Register."]
    #[inline] pub fn ctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x4)
    }

    #[doc="Get the *mut pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_mut(&self) -> *mut Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_ptr(&self) -> *const Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Read the CTRLB register."]
    #[inline] pub fn ctrlb(&self) -> Ctrlb { 
        self.ctrlb_reg().read()
    }

    #[doc="Write the CTRLB register."]
    #[inline] pub fn write_ctrlb(&self, value: Ctrlb) -> &Self { 
        self.ctrlb_reg().write(value);
        self
    }

    #[doc="Set the CTRLB register."]
    #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        self.ctrlb_reg().set(f);
        self
    }

    #[doc="Modify the CTRLB register."]
    #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        self.ctrlb_reg().with(f);
        self
    }

    #[doc="Get the BAUD Register."]
    #[inline] pub fn baud_reg(&self) -> ::bobbin_mcu::register::Register<Baud> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Baud, 0x8)
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

    #[doc="Get the RXDATA Register."]
    #[inline] pub fn rxdata_reg(&self) -> ::bobbin_mcu::register::Register<Rxdata> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rxdata, 0xc)
    }

    #[doc="Get the *mut pointer for the RXDATA register."]
    #[inline] pub fn rxdata_mut(&self) -> *mut Rxdata { 
        self.rxdata_reg().ptr()
    }

    #[doc="Get the *const pointer for the RXDATA register."]
    #[inline] pub fn rxdata_ptr(&self) -> *const Rxdata { 
        self.rxdata_reg().ptr()
    }

    #[doc="Read the RXDATA register."]
    #[inline] pub fn rxdata(&self) -> Rxdata { 
        self.rxdata_reg().read()
    }

    #[doc="Get the TXDATA Register."]
    #[inline] pub fn txdata_reg(&self) -> ::bobbin_mcu::register::Register<Txdata> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Txdata, 0x10)
    }

    #[doc="Get the *mut pointer for the TXDATA register."]
    #[inline] pub fn txdata_mut(&self) -> *mut Txdata { 
        self.txdata_reg().ptr()
    }

    #[doc="Get the *const pointer for the TXDATA register."]
    #[inline] pub fn txdata_ptr(&self) -> *const Txdata { 
        self.txdata_reg().ptr()
    }

    #[doc="Write the TXDATA register."]
    #[inline] pub fn write_txdata(&self, value: Txdata) -> &Self { 
        self.txdata_reg().write(value);
        self
    }

    #[doc="Set the TXDATA register."]
    #[inline] pub fn set_txdata<F: FnOnce(Txdata) -> Txdata>(&self, f: F) -> &Self {
        self.txdata_reg().set(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x14)
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        self.intenclr_reg().read()
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn write_intenclr(&self, value: Intenclr) -> &Self { 
        self.intenclr_reg().write(value);
        self
    }

    #[doc="Set the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().set(f);
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().with(f);
        self
    }

    #[doc="Get the INTENSET Register."]
    #[inline] pub fn intenset_reg(&self) -> ::bobbin_mcu::register::Register<Intenset> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x18)
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        self.intenset_reg().read()
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn write_intenset(&self, value: Intenset) -> &Self { 
        self.intenset_reg().write(value);
        self
    }

    #[doc="Set the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().set(f);
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().with(f);
        self
    }

    #[doc="Get the INTFLAG Register."]
    #[inline] pub fn intflag_reg(&self) -> ::bobbin_mcu::register::Register<Intflag> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x1c)
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        self.intflag_reg().read()
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn write_intflag(&self, value: Intflag) -> &Self { 
        self.intflag_reg().write(value);
        self
    }

    #[doc="Set the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x20)
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        self.status_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        self.status_reg().ptr()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        self.status_reg().read()
    }

    #[doc="Get the INSTRADDR Register."]
    #[inline] pub fn instraddr_reg(&self) -> ::bobbin_mcu::register::Register<Instraddr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Instraddr, 0x30)
    }

    #[doc="Get the *mut pointer for the INSTRADDR register."]
    #[inline] pub fn instraddr_mut(&self) -> *mut Instraddr { 
        self.instraddr_reg().ptr()
    }

    #[doc="Get the *const pointer for the INSTRADDR register."]
    #[inline] pub fn instraddr_ptr(&self) -> *const Instraddr { 
        self.instraddr_reg().ptr()
    }

    #[doc="Read the INSTRADDR register."]
    #[inline] pub fn instraddr(&self) -> Instraddr { 
        self.instraddr_reg().read()
    }

    #[doc="Write the INSTRADDR register."]
    #[inline] pub fn write_instraddr(&self, value: Instraddr) -> &Self { 
        self.instraddr_reg().write(value);
        self
    }

    #[doc="Set the INSTRADDR register."]
    #[inline] pub fn set_instraddr<F: FnOnce(Instraddr) -> Instraddr>(&self, f: F) -> &Self {
        self.instraddr_reg().set(f);
        self
    }

    #[doc="Modify the INSTRADDR register."]
    #[inline] pub fn with_instraddr<F: FnOnce(Instraddr) -> Instraddr>(&self, f: F) -> &Self {
        self.instraddr_reg().with(f);
        self
    }

    #[doc="Get the INSTRCTRL Register."]
    #[inline] pub fn instrctrl_reg(&self) -> ::bobbin_mcu::register::Register<Instrctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Instrctrl, 0x34)
    }

    #[doc="Get the *mut pointer for the INSTRCTRL register."]
    #[inline] pub fn instrctrl_mut(&self) -> *mut Instrctrl { 
        self.instrctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the INSTRCTRL register."]
    #[inline] pub fn instrctrl_ptr(&self) -> *const Instrctrl { 
        self.instrctrl_reg().ptr()
    }

    #[doc="Read the INSTRCTRL register."]
    #[inline] pub fn instrctrl(&self) -> Instrctrl { 
        self.instrctrl_reg().read()
    }

    #[doc="Write the INSTRCTRL register."]
    #[inline] pub fn write_instrctrl(&self, value: Instrctrl) -> &Self { 
        self.instrctrl_reg().write(value);
        self
    }

    #[doc="Set the INSTRCTRL register."]
    #[inline] pub fn set_instrctrl<F: FnOnce(Instrctrl) -> Instrctrl>(&self, f: F) -> &Self {
        self.instrctrl_reg().set(f);
        self
    }

    #[doc="Modify the INSTRCTRL register."]
    #[inline] pub fn with_instrctrl<F: FnOnce(Instrctrl) -> Instrctrl>(&self, f: F) -> &Self {
        self.instrctrl_reg().with(f);
        self
    }

    #[doc="Get the INSTRFRAME Register."]
    #[inline] pub fn instrframe_reg(&self) -> ::bobbin_mcu::register::Register<Instrframe> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Instrframe, 0x38)
    }

    #[doc="Get the *mut pointer for the INSTRFRAME register."]
    #[inline] pub fn instrframe_mut(&self) -> *mut Instrframe { 
        self.instrframe_reg().ptr()
    }

    #[doc="Get the *const pointer for the INSTRFRAME register."]
    #[inline] pub fn instrframe_ptr(&self) -> *const Instrframe { 
        self.instrframe_reg().ptr()
    }

    #[doc="Read the INSTRFRAME register."]
    #[inline] pub fn instrframe(&self) -> Instrframe { 
        self.instrframe_reg().read()
    }

    #[doc="Write the INSTRFRAME register."]
    #[inline] pub fn write_instrframe(&self, value: Instrframe) -> &Self { 
        self.instrframe_reg().write(value);
        self
    }

    #[doc="Set the INSTRFRAME register."]
    #[inline] pub fn set_instrframe<F: FnOnce(Instrframe) -> Instrframe>(&self, f: F) -> &Self {
        self.instrframe_reg().set(f);
        self
    }

    #[doc="Modify the INSTRFRAME register."]
    #[inline] pub fn with_instrframe<F: FnOnce(Instrframe) -> Instrframe>(&self, f: F) -> &Self {
        self.instrframe_reg().with(f);
        self
    }

    #[doc="Get the SCRAMBCTRL Register."]
    #[inline] pub fn scrambctrl_reg(&self) -> ::bobbin_mcu::register::Register<Scrambctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Scrambctrl, 0x40)
    }

    #[doc="Get the *mut pointer for the SCRAMBCTRL register."]
    #[inline] pub fn scrambctrl_mut(&self) -> *mut Scrambctrl { 
        self.scrambctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the SCRAMBCTRL register."]
    #[inline] pub fn scrambctrl_ptr(&self) -> *const Scrambctrl { 
        self.scrambctrl_reg().ptr()
    }

    #[doc="Read the SCRAMBCTRL register."]
    #[inline] pub fn scrambctrl(&self) -> Scrambctrl { 
        self.scrambctrl_reg().read()
    }

    #[doc="Write the SCRAMBCTRL register."]
    #[inline] pub fn write_scrambctrl(&self, value: Scrambctrl) -> &Self { 
        self.scrambctrl_reg().write(value);
        self
    }

    #[doc="Set the SCRAMBCTRL register."]
    #[inline] pub fn set_scrambctrl<F: FnOnce(Scrambctrl) -> Scrambctrl>(&self, f: F) -> &Self {
        self.scrambctrl_reg().set(f);
        self
    }

    #[doc="Modify the SCRAMBCTRL register."]
    #[inline] pub fn with_scrambctrl<F: FnOnce(Scrambctrl) -> Scrambctrl>(&self, f: F) -> &Self {
        self.scrambctrl_reg().with(f);
        self
    }

    #[doc="Get the SCRAMBKEY Register."]
    #[inline] pub fn scrambkey_reg(&self) -> ::bobbin_mcu::register::Register<Scrambkey> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Scrambkey, 0x44)
    }

    #[doc="Get the *mut pointer for the SCRAMBKEY register."]
    #[inline] pub fn scrambkey_mut(&self) -> *mut Scrambkey { 
        self.scrambkey_reg().ptr()
    }

    #[doc="Get the *const pointer for the SCRAMBKEY register."]
    #[inline] pub fn scrambkey_ptr(&self) -> *const Scrambkey { 
        self.scrambkey_reg().ptr()
    }

    #[doc="Write the SCRAMBKEY register."]
    #[inline] pub fn write_scrambkey(&self, value: Scrambkey) -> &Self { 
        self.scrambkey_reg().write(value);
        self
    }

    #[doc="Set the SCRAMBKEY register."]
    #[inline] pub fn set_scrambkey<F: FnOnce(Scrambkey) -> Scrambkey>(&self, f: F) -> &Self {
        self.scrambkey_reg().set(f);
        self
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
    #[doc="Software Reset"]
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

    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Last Transfer"]
    #[inline] pub fn lastxfer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if LASTXFER != 0"]
    #[inline] pub fn test_lastxfer(&self) -> bool {
        self.lastxfer() != 0
    }

    #[doc="Sets the LASTXFER field."]
    #[inline] pub fn set_lastxfer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ctrla {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.lastxfer() != 0 { try!(write!(f, " lastxfer"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
    #[doc="Serial Memory Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Local Loopback Enable"]
    #[inline] pub fn loopen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LOOPEN != 0"]
    #[inline] pub fn test_loopen(&self) -> bool {
        self.loopen() != 0
    }

    #[doc="Sets the LOOPEN field."]
    #[inline] pub fn set_loopen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Wait Data Read Before Transfer"]
    #[inline] pub fn wdrbt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WDRBT != 0"]
    #[inline] pub fn test_wdrbt(&self) -> bool {
        self.wdrbt() != 0
    }

    #[doc="Sets the WDRBT field."]
    #[inline] pub fn set_wdrbt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Serial Memory reg"]
    #[inline] pub fn smemreg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SMEMREG != 0"]
    #[inline] pub fn test_smemreg(&self) -> bool {
        self.smemreg() != 0
    }

    #[doc="Sets the SMEMREG field."]
    #[inline] pub fn set_smemreg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Chip Select Mode"]
    #[inline] pub fn csmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if CSMODE != 0"]
    #[inline] pub fn test_csmode(&self) -> bool {
        self.csmode() != 0
    }

    #[doc="Sets the CSMODE field."]
    #[inline] pub fn set_csmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Length"]
    #[inline] pub fn datalen(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if DATALEN != 0"]
    #[inline] pub fn test_datalen(&self) -> bool {
        self.datalen() != 0
    }

    #[doc="Sets the DATALEN field."]
    #[inline] pub fn set_datalen<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Delay Between Consecutive Transfers"]
    #[inline] pub fn dlybct(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DLYBCT != 0"]
    #[inline] pub fn test_dlybct(&self) -> bool {
        self.dlybct() != 0
    }

    #[doc="Sets the DLYBCT field."]
    #[inline] pub fn set_dlybct<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Minimum Inactive CS Delay"]
    #[inline] pub fn dlycs(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DLYCS != 0"]
    #[inline] pub fn test_dlycs(&self) -> bool {
        self.dlycs() != 0
    }

    #[doc="Sets the DLYCS field."]
    #[inline] pub fn set_dlycs<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ctrlb {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrlb(other)
    }
}

impl ::core::fmt::Display for Ctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode() != 0 { try!(write!(f, " mode"))}
        if self.loopen() != 0 { try!(write!(f, " loopen"))}
        if self.wdrbt() != 0 { try!(write!(f, " wdrbt"))}
        if self.smemreg() != 0 { try!(write!(f, " smemreg"))}
        if self.csmode() != 0 { try!(write!(f, " csmode=0x{:x}", self.csmode()))}
        if self.datalen() != 0 { try!(write!(f, " datalen=0x{:x}", self.datalen()))}
        if self.dlybct() != 0 { try!(write!(f, " dlybct=0x{:x}", self.dlybct()))}
        if self.dlycs() != 0 { try!(write!(f, " dlycs=0x{:x}", self.dlycs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Baud Rate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u32);
impl Baud {
    #[doc="Clock Polarity"]
    #[inline] pub fn cpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Phase"]
    #[inline] pub fn cpha(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Serial Clock Baud Rate"]
    #[inline] pub fn baud(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if BAUD != 0"]
    #[inline] pub fn test_baud(&self) -> bool {
        self.baud() != 0
    }

    #[doc="Sets the BAUD field."]
    #[inline] pub fn set_baud<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Delay Before SCK"]
    #[inline] pub fn dlybs(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DLYBS != 0"]
    #[inline] pub fn test_dlybs(&self) -> bool {
        self.dlybs() != 0
    }

    #[doc="Sets the DLYBS field."]
    #[inline] pub fn set_dlybs<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
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
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
        if self.dlybs() != 0 { try!(write!(f, " dlybs=0x{:x}", self.dlybs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdata(pub u32);
impl Rxdata {
    #[doc="Receive Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rxdata {
    #[inline]
    fn from(other: u32) -> Self {
         Rxdata(other)
    }
}

impl ::core::fmt::Display for Rxdata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdata(pub u32);
impl Txdata {
    #[doc="Transmit Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Txdata {
    #[inline]
    fn from(other: u32) -> Self {
         Txdata(other)
    }
}

impl ::core::fmt::Display for Txdata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txdata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc="Receive Data Register Full Interrupt Disable"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Data Register Empty Interrupt Disable"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmission Complete Interrupt Disable"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun Error Interrupt Disable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Chip Select Rise Interrupt Disable"]
    #[inline] pub fn csrise(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSRISE != 0"]
    #[inline] pub fn test_csrise(&self) -> bool {
        self.csrise() != 0
    }

    #[doc="Sets the CSRISE field."]
    #[inline] pub fn set_csrise<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Instruction End Interrupt Disable"]
    #[inline] pub fn instrend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if INSTREND != 0"]
    #[inline] pub fn test_instrend(&self) -> bool {
        self.instrend() != 0
    }

    #[doc="Sets the INSTREND field."]
    #[inline] pub fn set_instrend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Intenclr {
    #[inline]
    fn from(other: u32) -> Self {
         Intenclr(other)
    }
}

impl ::core::fmt::Display for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        if self.csrise() != 0 { try!(write!(f, " csrise"))}
        if self.instrend() != 0 { try!(write!(f, " instrend"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc="Receive Data Register Full Interrupt Enable"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Data Register Empty Interrupt Enable"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmission Complete Interrupt Enable"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun Error Interrupt Enable"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Chip Select Rise Interrupt Enable"]
    #[inline] pub fn csrise(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSRISE != 0"]
    #[inline] pub fn test_csrise(&self) -> bool {
        self.csrise() != 0
    }

    #[doc="Sets the CSRISE field."]
    #[inline] pub fn set_csrise<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Instruction End Interrupt Enable"]
    #[inline] pub fn instrend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if INSTREND != 0"]
    #[inline] pub fn test_instrend(&self) -> bool {
        self.instrend() != 0
    }

    #[doc="Sets the INSTREND field."]
    #[inline] pub fn set_instrend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Intenset {
    #[inline]
    fn from(other: u32) -> Self {
         Intenset(other)
    }
}

impl ::core::fmt::Display for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        if self.csrise() != 0 { try!(write!(f, " csrise"))}
        if self.instrend() != 0 { try!(write!(f, " instrend"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
    #[doc="Receive Data Register Full"]
    #[inline] pub fn rxc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXC != 0"]
    #[inline] pub fn test_rxc(&self) -> bool {
        self.rxc() != 0
    }

    #[doc="Sets the RXC field."]
    #[inline] pub fn set_rxc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Data Register Empty"]
    #[inline] pub fn dre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DRE != 0"]
    #[inline] pub fn test_dre(&self) -> bool {
        self.dre() != 0
    }

    #[doc="Sets the DRE field."]
    #[inline] pub fn set_dre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmission Complete"]
    #[inline] pub fn txc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun Error"]
    #[inline] pub fn error(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Chip Select Rise"]
    #[inline] pub fn csrise(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSRISE != 0"]
    #[inline] pub fn test_csrise(&self) -> bool {
        self.csrise() != 0
    }

    #[doc="Sets the CSRISE field."]
    #[inline] pub fn set_csrise<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Instruction End"]
    #[inline] pub fn instrend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if INSTREND != 0"]
    #[inline] pub fn test_instrend(&self) -> bool {
        self.instrend() != 0
    }

    #[doc="Sets the INSTREND field."]
    #[inline] pub fn set_instrend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Intflag {
    #[inline]
    fn from(other: u32) -> Self {
         Intflag(other)
    }
}

impl ::core::fmt::Display for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxc() != 0 { try!(write!(f, " rxc"))}
        if self.dre() != 0 { try!(write!(f, " dre"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        if self.csrise() != 0 { try!(write!(f, " csrise"))}
        if self.instrend() != 0 { try!(write!(f, " instrend"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Chip Select"]
    #[inline] pub fn csstatus(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CSSTATUS != 0"]
    #[inline] pub fn test_csstatus(&self) -> bool {
        self.csstatus() != 0
    }

    #[doc="Sets the CSSTATUS field."]
    #[inline] pub fn set_csstatus<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Status {
    #[inline]
    fn from(other: u32) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.csstatus() != 0 { try!(write!(f, " csstatus"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Instruction Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Instraddr(pub u32);
impl Instraddr {
    #[doc="Instruction Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Instraddr {
    #[inline]
    fn from(other: u32) -> Self {
         Instraddr(other)
    }
}

impl ::core::fmt::Display for Instraddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Instraddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Instruction Code"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Instrctrl(pub u32);
impl Instrctrl {
    #[doc="Instruction Code"]
    #[inline] pub fn instr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if INSTR != 0"]
    #[inline] pub fn test_instr(&self) -> bool {
        self.instr() != 0
    }

    #[doc="Sets the INSTR field."]
    #[inline] pub fn set_instr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Option Code"]
    #[inline] pub fn optcode(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if OPTCODE != 0"]
    #[inline] pub fn test_optcode(&self) -> bool {
        self.optcode() != 0
    }

    #[doc="Sets the OPTCODE field."]
    #[inline] pub fn set_optcode<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Instrctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Instrctrl(other)
    }
}

impl ::core::fmt::Display for Instrctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Instrctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.instr() != 0 { try!(write!(f, " instr=0x{:x}", self.instr()))}
        if self.optcode() != 0 { try!(write!(f, " optcode=0x{:x}", self.optcode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Instruction Frame"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Instrframe(pub u32);
impl Instrframe {
    #[doc="Instruction Code, Address, Option Code and Data Width"]
    #[inline] pub fn width(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if WIDTH != 0"]
    #[inline] pub fn test_width(&self) -> bool {
        self.width() != 0
    }

    #[doc="Sets the WIDTH field."]
    #[inline] pub fn set_width<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Instruction Enable"]
    #[inline] pub fn instren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if INSTREN != 0"]
    #[inline] pub fn test_instren(&self) -> bool {
        self.instren() != 0
    }

    #[doc="Sets the INSTREN field."]
    #[inline] pub fn set_instren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Address Enable"]
    #[inline] pub fn addren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ADDREN != 0"]
    #[inline] pub fn test_addren(&self) -> bool {
        self.addren() != 0
    }

    #[doc="Sets the ADDREN field."]
    #[inline] pub fn set_addren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Option Enable"]
    #[inline] pub fn optcodeen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if OPTCODEEN != 0"]
    #[inline] pub fn test_optcodeen(&self) -> bool {
        self.optcodeen() != 0
    }

    #[doc="Sets the OPTCODEEN field."]
    #[inline] pub fn set_optcodeen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Data Enable"]
    #[inline] pub fn dataen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DATAEN != 0"]
    #[inline] pub fn test_dataen(&self) -> bool {
        self.dataen() != 0
    }

    #[doc="Sets the DATAEN field."]
    #[inline] pub fn set_dataen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Option Code Length"]
    #[inline] pub fn optcodelen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if OPTCODELEN != 0"]
    #[inline] pub fn test_optcodelen(&self) -> bool {
        self.optcodelen() != 0
    }

    #[doc="Sets the OPTCODELEN field."]
    #[inline] pub fn set_optcodelen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Address Length"]
    #[inline] pub fn addrlen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ADDRLEN != 0"]
    #[inline] pub fn test_addrlen(&self) -> bool {
        self.addrlen() != 0
    }

    #[doc="Sets the ADDRLEN field."]
    #[inline] pub fn set_addrlen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Data Transfer Type"]
    #[inline] pub fn tfrtype(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if TFRTYPE != 0"]
    #[inline] pub fn test_tfrtype(&self) -> bool {
        self.tfrtype() != 0
    }

    #[doc="Sets the TFRTYPE field."]
    #[inline] pub fn set_tfrtype<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Continuous Read Mode"]
    #[inline] pub fn crmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CRMODE != 0"]
    #[inline] pub fn test_crmode(&self) -> bool {
        self.crmode() != 0
    }

    #[doc="Sets the CRMODE field."]
    #[inline] pub fn set_crmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Double Data Rate Enable"]
    #[inline] pub fn ddren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DDREN != 0"]
    #[inline] pub fn test_ddren(&self) -> bool {
        self.ddren() != 0
    }

    #[doc="Sets the DDREN field."]
    #[inline] pub fn set_ddren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Dummy Cycles Length"]
    #[inline] pub fn dummylen(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if DUMMYLEN != 0"]
    #[inline] pub fn test_dummylen(&self) -> bool {
        self.dummylen() != 0
    }

    #[doc="Sets the DUMMYLEN field."]
    #[inline] pub fn set_dummylen<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Instrframe {
    #[inline]
    fn from(other: u32) -> Self {
         Instrframe(other)
    }
}

impl ::core::fmt::Display for Instrframe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Instrframe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.width() != 0 { try!(write!(f, " width=0x{:x}", self.width()))}
        if self.instren() != 0 { try!(write!(f, " instren"))}
        if self.addren() != 0 { try!(write!(f, " addren"))}
        if self.optcodeen() != 0 { try!(write!(f, " optcodeen"))}
        if self.dataen() != 0 { try!(write!(f, " dataen"))}
        if self.optcodelen() != 0 { try!(write!(f, " optcodelen=0x{:x}", self.optcodelen()))}
        if self.addrlen() != 0 { try!(write!(f, " addrlen"))}
        if self.tfrtype() != 0 { try!(write!(f, " tfrtype=0x{:x}", self.tfrtype()))}
        if self.crmode() != 0 { try!(write!(f, " crmode"))}
        if self.ddren() != 0 { try!(write!(f, " ddren"))}
        if self.dummylen() != 0 { try!(write!(f, " dummylen=0x{:x}", self.dummylen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Scrambling Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scrambctrl(pub u32);
impl Scrambctrl {
    #[doc="Scrambling/Unscrambling Enable"]
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

    #[doc="Scrambling/Unscrambling Random Value Disable"]
    #[inline] pub fn randomdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RANDOMDIS != 0"]
    #[inline] pub fn test_randomdis(&self) -> bool {
        self.randomdis() != 0
    }

    #[doc="Sets the RANDOMDIS field."]
    #[inline] pub fn set_randomdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Scrambctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Scrambctrl(other)
    }
}

impl ::core::fmt::Display for Scrambctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scrambctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.randomdis() != 0 { try!(write!(f, " randomdis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Scrambling Key"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scrambkey(pub u32);
impl Scrambkey {
    #[doc="Scrambling User Key"]
    #[inline] pub fn key(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scrambkey {
    #[inline]
    fn from(other: u32) -> Self {
         Scrambkey(other)
    }
}

impl ::core::fmt::Display for Scrambkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scrambkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

