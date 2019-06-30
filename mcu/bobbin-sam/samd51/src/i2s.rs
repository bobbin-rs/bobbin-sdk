::bobbin_mcu::periph!( I2S, I2s, I2S_PERIPH, I2sPeriph, I2S_OWNED, I2S_REF_COUNT, 0x43002800, 0x00, 0x10);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2S Peripheral"]
pub struct I2sPeriph(pub usize); 

impl I2sPeriph {
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

    #[doc="Get the CLKCTRL Register."]
    #[inline] pub fn clkctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Clkctrl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Clkctrl, 0x4, 0x4)
    }

    #[doc="Get the *mut pointer for the CLKCTRL register."]
    #[inline] pub fn clkctrl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Clkctrl { 
        self.clkctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CLKCTRL register."]
    #[inline] pub fn clkctrl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Clkctrl { 
        self.clkctrl_reg().ptr(index.into())
    }

    #[doc="Read the CLKCTRL register."]
    #[inline] pub fn clkctrl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Clkctrl { 
        self.clkctrl_reg().read(index.into())
    }

    #[doc="Write the CLKCTRL register."]
    #[inline] pub fn write_clkctrl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Clkctrl) -> &Self {
        self.clkctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CLKCTRL register."]
    #[inline] pub fn set_clkctrl<I: Into<::bobbin_bits::R2>, F: FnOnce(Clkctrl) -> Clkctrl>(&self, index: I, f: F) -> &Self {
        self.clkctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CLKCTRL register."]
    #[inline] pub fn with_clkctrl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Clkctrl) -> Clkctrl>(&self, index: I, f: F) -> &Self {
        self.clkctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0xc)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x10)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x14)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x18)
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        self.syncbusy_reg().read()
    }

    #[doc="Get the TXCTRL Register."]
    #[inline] pub fn txctrl_reg(&self) -> ::bobbin_mcu::register::Register<Txctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Txctrl, 0x20)
    }

    #[doc="Get the *mut pointer for the TXCTRL register."]
    #[inline] pub fn txctrl_mut(&self) -> *mut Txctrl { 
        self.txctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the TXCTRL register."]
    #[inline] pub fn txctrl_ptr(&self) -> *const Txctrl { 
        self.txctrl_reg().ptr()
    }

    #[doc="Read the TXCTRL register."]
    #[inline] pub fn txctrl(&self) -> Txctrl { 
        self.txctrl_reg().read()
    }

    #[doc="Write the TXCTRL register."]
    #[inline] pub fn write_txctrl(&self, value: Txctrl) -> &Self { 
        self.txctrl_reg().write(value);
        self
    }

    #[doc="Set the TXCTRL register."]
    #[inline] pub fn set_txctrl<F: FnOnce(Txctrl) -> Txctrl>(&self, f: F) -> &Self {
        self.txctrl_reg().set(f);
        self
    }

    #[doc="Modify the TXCTRL register."]
    #[inline] pub fn with_txctrl<F: FnOnce(Txctrl) -> Txctrl>(&self, f: F) -> &Self {
        self.txctrl_reg().with(f);
        self
    }

    #[doc="Get the RXCTRL Register."]
    #[inline] pub fn rxctrl_reg(&self) -> ::bobbin_mcu::register::Register<Rxctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rxctrl, 0x24)
    }

    #[doc="Get the *mut pointer for the RXCTRL register."]
    #[inline] pub fn rxctrl_mut(&self) -> *mut Rxctrl { 
        self.rxctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the RXCTRL register."]
    #[inline] pub fn rxctrl_ptr(&self) -> *const Rxctrl { 
        self.rxctrl_reg().ptr()
    }

    #[doc="Read the RXCTRL register."]
    #[inline] pub fn rxctrl(&self) -> Rxctrl { 
        self.rxctrl_reg().read()
    }

    #[doc="Write the RXCTRL register."]
    #[inline] pub fn write_rxctrl(&self, value: Rxctrl) -> &Self { 
        self.rxctrl_reg().write(value);
        self
    }

    #[doc="Set the RXCTRL register."]
    #[inline] pub fn set_rxctrl<F: FnOnce(Rxctrl) -> Rxctrl>(&self, f: F) -> &Self {
        self.rxctrl_reg().set(f);
        self
    }

    #[doc="Modify the RXCTRL register."]
    #[inline] pub fn with_rxctrl<F: FnOnce(Rxctrl) -> Rxctrl>(&self, f: F) -> &Self {
        self.rxctrl_reg().with(f);
        self
    }

    #[doc="Get the TXDATA Register."]
    #[inline] pub fn txdata_reg(&self) -> ::bobbin_mcu::register::Register<Txdata> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Txdata, 0x30)
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

    #[doc="Get the RXDATA Register."]
    #[inline] pub fn rxdata_reg(&self) -> ::bobbin_mcu::register::Register<Rxdata> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rxdata, 0x34)
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

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u8);
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
        let value: u8 = value.into();
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock Unit 0 Enable"]
    #[inline] pub fn cken0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CKEN0 != 0"]
    #[inline] pub fn test_cken0(&self) -> bool {
        self.cken0() != 0
    }

    #[doc="Sets the CKEN0 field."]
    #[inline] pub fn set_cken0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clock Unit 1 Enable"]
    #[inline] pub fn cken1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CKEN1 != 0"]
    #[inline] pub fn test_cken1(&self) -> bool {
        self.cken1() != 0
    }

    #[doc="Sets the CKEN1 field."]
    #[inline] pub fn set_cken1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Tx Serializer Enable"]
    #[inline] pub fn txen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXEN != 0"]
    #[inline] pub fn test_txen(&self) -> bool {
        self.txen() != 0
    }

    #[doc="Sets the TXEN field."]
    #[inline] pub fn set_txen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Rx Serializer Enable"]
    #[inline] pub fn rxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXEN != 0"]
    #[inline] pub fn test_rxen(&self) -> bool {
        self.rxen() != 0
    }

    #[doc="Sets the RXEN field."]
    #[inline] pub fn set_rxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Ctrla {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.cken0() != 0 { try!(write!(f, " cken0"))}
        if self.cken1() != 0 { try!(write!(f, " cken1"))}
        if self.txen() != 0 { try!(write!(f, " txen"))}
        if self.rxen() != 0 { try!(write!(f, " rxen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Unit n Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkctrl(pub u32);
impl Clkctrl {
    #[doc="Slot Size"]
    #[inline] pub fn slotsize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SLOTSIZE != 0"]
    #[inline] pub fn test_slotsize(&self) -> bool {
        self.slotsize() != 0
    }

    #[doc="Sets the SLOTSIZE field."]
    #[inline] pub fn set_slotsize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of Slots in Frame"]
    #[inline] pub fn nbslots(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if NBSLOTS != 0"]
    #[inline] pub fn test_nbslots(&self) -> bool {
        self.nbslots() != 0
    }

    #[doc="Sets the NBSLOTS field."]
    #[inline] pub fn set_nbslots<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Frame Sync Width"]
    #[inline] pub fn fswidth(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if FSWIDTH != 0"]
    #[inline] pub fn test_fswidth(&self) -> bool {
        self.fswidth() != 0
    }

    #[doc="Sets the FSWIDTH field."]
    #[inline] pub fn set_fswidth<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Data Delay from Frame Sync"]
    #[inline] pub fn bitdelay(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BITDELAY != 0"]
    #[inline] pub fn test_bitdelay(&self) -> bool {
        self.bitdelay() != 0
    }

    #[doc="Sets the BITDELAY field."]
    #[inline] pub fn set_bitdelay<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Frame Sync Select"]
    #[inline] pub fn fssel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FSSEL != 0"]
    #[inline] pub fn test_fssel(&self) -> bool {
        self.fssel() != 0
    }

    #[doc="Sets the FSSEL field."]
    #[inline] pub fn set_fssel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame Sync Invert"]
    #[inline] pub fn fsinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FSINV != 0"]
    #[inline] pub fn test_fsinv(&self) -> bool {
        self.fsinv() != 0
    }

    #[doc="Sets the FSINV field."]
    #[inline] pub fn set_fsinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Frame Sync Output Invert"]
    #[inline] pub fn fsoutinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FSOUTINV != 0"]
    #[inline] pub fn test_fsoutinv(&self) -> bool {
        self.fsoutinv() != 0
    }

    #[doc="Sets the FSOUTINV field."]
    #[inline] pub fn set_fsoutinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Serial Clock Select"]
    #[inline] pub fn scksel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SCKSEL != 0"]
    #[inline] pub fn test_scksel(&self) -> bool {
        self.scksel() != 0
    }

    #[doc="Sets the SCKSEL field."]
    #[inline] pub fn set_scksel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Serial Clock Output Invert"]
    #[inline] pub fn sckoutinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SCKOUTINV != 0"]
    #[inline] pub fn test_sckoutinv(&self) -> bool {
        self.sckoutinv() != 0
    }

    #[doc="Sets the SCKOUTINV field."]
    #[inline] pub fn set_sckoutinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Master Clock Select"]
    #[inline] pub fn mcksel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MCKSEL != 0"]
    #[inline] pub fn test_mcksel(&self) -> bool {
        self.mcksel() != 0
    }

    #[doc="Sets the MCKSEL field."]
    #[inline] pub fn set_mcksel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Master Clock Enable"]
    #[inline] pub fn mcken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if MCKEN != 0"]
    #[inline] pub fn test_mcken(&self) -> bool {
        self.mcken() != 0
    }

    #[doc="Sets the MCKEN field."]
    #[inline] pub fn set_mcken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Master Clock Output Invert"]
    #[inline] pub fn mckoutinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MCKOUTINV != 0"]
    #[inline] pub fn test_mckoutinv(&self) -> bool {
        self.mckoutinv() != 0
    }

    #[doc="Sets the MCKOUTINV field."]
    #[inline] pub fn set_mckoutinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Master Clock Division Factor"]
    #[inline] pub fn mckdiv(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if MCKDIV != 0"]
    #[inline] pub fn test_mckdiv(&self) -> bool {
        self.mckdiv() != 0
    }

    #[doc="Sets the MCKDIV field."]
    #[inline] pub fn set_mckdiv<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Master Clock Output Division Factor"]
    #[inline] pub fn mckoutdiv(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if MCKOUTDIV != 0"]
    #[inline] pub fn test_mckoutdiv(&self) -> bool {
        self.mckoutdiv() != 0
    }

    #[doc="Sets the MCKOUTDIV field."]
    #[inline] pub fn set_mckoutdiv<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Clkctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Clkctrl(other)
    }
}

impl ::core::fmt::Display for Clkctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.slotsize() != 0 { try!(write!(f, " slotsize=0x{:x}", self.slotsize()))}
        if self.nbslots() != 0 { try!(write!(f, " nbslots=0x{:x}", self.nbslots()))}
        if self.fswidth() != 0 { try!(write!(f, " fswidth=0x{:x}", self.fswidth()))}
        if self.bitdelay() != 0 { try!(write!(f, " bitdelay"))}
        if self.fssel() != 0 { try!(write!(f, " fssel"))}
        if self.fsinv() != 0 { try!(write!(f, " fsinv"))}
        if self.fsoutinv() != 0 { try!(write!(f, " fsoutinv"))}
        if self.scksel() != 0 { try!(write!(f, " scksel"))}
        if self.sckoutinv() != 0 { try!(write!(f, " sckoutinv"))}
        if self.mcksel() != 0 { try!(write!(f, " mcksel"))}
        if self.mcken() != 0 { try!(write!(f, " mcken"))}
        if self.mckoutinv() != 0 { try!(write!(f, " mckoutinv"))}
        if self.mckdiv() != 0 { try!(write!(f, " mckdiv=0x{:x}", self.mckdiv()))}
        if self.mckoutdiv() != 0 { try!(write!(f, " mckoutdiv=0x{:x}", self.mckoutdiv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u16);
impl Intenclr {
    #[doc="Receive Ready 0 Interrupt Enable"]
    #[inline] pub fn rxrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXRDY0 != 0"]
    #[inline] pub fn test_rxrdy0(&self) -> bool {
        self.rxrdy0() != 0
    }

    #[doc="Sets the RXRDY0 field."]
    #[inline] pub fn set_rxrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Ready 1 Interrupt Enable"]
    #[inline] pub fn rxrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RXRDY1 != 0"]
    #[inline] pub fn test_rxrdy1(&self) -> bool {
        self.rxrdy1() != 0
    }

    #[doc="Sets the RXRDY1 field."]
    #[inline] pub fn set_rxrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Overrun 0 Interrupt Enable"]
    #[inline] pub fn rxor0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXOR0 != 0"]
    #[inline] pub fn test_rxor0(&self) -> bool {
        self.rxor0() != 0
    }

    #[doc="Sets the RXOR0 field."]
    #[inline] pub fn set_rxor0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive Overrun 1 Interrupt Enable"]
    #[inline] pub fn rxor1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXOR1 != 0"]
    #[inline] pub fn test_rxor1(&self) -> bool {
        self.rxor1() != 0
    }

    #[doc="Sets the RXOR1 field."]
    #[inline] pub fn set_rxor1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmit Ready 0 Interrupt Enable"]
    #[inline] pub fn txrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXRDY0 != 0"]
    #[inline] pub fn test_txrdy0(&self) -> bool {
        self.txrdy0() != 0
    }

    #[doc="Sets the TXRDY0 field."]
    #[inline] pub fn set_txrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmit Ready 1 Interrupt Enable"]
    #[inline] pub fn txrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXRDY1 != 0"]
    #[inline] pub fn test_txrdy1(&self) -> bool {
        self.txrdy1() != 0
    }

    #[doc="Sets the TXRDY1 field."]
    #[inline] pub fn set_txrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit Underrun 0 Interrupt Enable"]
    #[inline] pub fn txur0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXUR0 != 0"]
    #[inline] pub fn test_txur0(&self) -> bool {
        self.txur0() != 0
    }

    #[doc="Sets the TXUR0 field."]
    #[inline] pub fn set_txur0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Transmit Underrun 1 Interrupt Enable"]
    #[inline] pub fn txur1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXUR1 != 0"]
    #[inline] pub fn test_txur1(&self) -> bool {
        self.txur1() != 0
    }

    #[doc="Sets the TXUR1 field."]
    #[inline] pub fn set_txur1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Intenclr {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.rxrdy0() != 0 { try!(write!(f, " rxrdy0"))}
        if self.rxrdy1() != 0 { try!(write!(f, " rxrdy1"))}
        if self.rxor0() != 0 { try!(write!(f, " rxor0"))}
        if self.rxor1() != 0 { try!(write!(f, " rxor1"))}
        if self.txrdy0() != 0 { try!(write!(f, " txrdy0"))}
        if self.txrdy1() != 0 { try!(write!(f, " txrdy1"))}
        if self.txur0() != 0 { try!(write!(f, " txur0"))}
        if self.txur1() != 0 { try!(write!(f, " txur1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u16);
impl Intenset {
    #[doc="Receive Ready 0 Interrupt Enable"]
    #[inline] pub fn rxrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXRDY0 != 0"]
    #[inline] pub fn test_rxrdy0(&self) -> bool {
        self.rxrdy0() != 0
    }

    #[doc="Sets the RXRDY0 field."]
    #[inline] pub fn set_rxrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Ready 1 Interrupt Enable"]
    #[inline] pub fn rxrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RXRDY1 != 0"]
    #[inline] pub fn test_rxrdy1(&self) -> bool {
        self.rxrdy1() != 0
    }

    #[doc="Sets the RXRDY1 field."]
    #[inline] pub fn set_rxrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Overrun 0 Interrupt Enable"]
    #[inline] pub fn rxor0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXOR0 != 0"]
    #[inline] pub fn test_rxor0(&self) -> bool {
        self.rxor0() != 0
    }

    #[doc="Sets the RXOR0 field."]
    #[inline] pub fn set_rxor0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive Overrun 1 Interrupt Enable"]
    #[inline] pub fn rxor1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXOR1 != 0"]
    #[inline] pub fn test_rxor1(&self) -> bool {
        self.rxor1() != 0
    }

    #[doc="Sets the RXOR1 field."]
    #[inline] pub fn set_rxor1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmit Ready 0 Interrupt Enable"]
    #[inline] pub fn txrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXRDY0 != 0"]
    #[inline] pub fn test_txrdy0(&self) -> bool {
        self.txrdy0() != 0
    }

    #[doc="Sets the TXRDY0 field."]
    #[inline] pub fn set_txrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmit Ready 1 Interrupt Enable"]
    #[inline] pub fn txrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXRDY1 != 0"]
    #[inline] pub fn test_txrdy1(&self) -> bool {
        self.txrdy1() != 0
    }

    #[doc="Sets the TXRDY1 field."]
    #[inline] pub fn set_txrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit Underrun 0 Interrupt Enable"]
    #[inline] pub fn txur0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXUR0 != 0"]
    #[inline] pub fn test_txur0(&self) -> bool {
        self.txur0() != 0
    }

    #[doc="Sets the TXUR0 field."]
    #[inline] pub fn set_txur0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Transmit Underrun 1 Interrupt Enable"]
    #[inline] pub fn txur1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXUR1 != 0"]
    #[inline] pub fn test_txur1(&self) -> bool {
        self.txur1() != 0
    }

    #[doc="Sets the TXUR1 field."]
    #[inline] pub fn set_txur1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Intenset {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.rxrdy0() != 0 { try!(write!(f, " rxrdy0"))}
        if self.rxrdy1() != 0 { try!(write!(f, " rxrdy1"))}
        if self.rxor0() != 0 { try!(write!(f, " rxor0"))}
        if self.rxor1() != 0 { try!(write!(f, " rxor1"))}
        if self.txrdy0() != 0 { try!(write!(f, " txrdy0"))}
        if self.txrdy1() != 0 { try!(write!(f, " txrdy1"))}
        if self.txur0() != 0 { try!(write!(f, " txur0"))}
        if self.txur1() != 0 { try!(write!(f, " txur1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u16);
impl Intflag {
    #[doc="Receive Ready 0"]
    #[inline] pub fn rxrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXRDY0 != 0"]
    #[inline] pub fn test_rxrdy0(&self) -> bool {
        self.rxrdy0() != 0
    }

    #[doc="Sets the RXRDY0 field."]
    #[inline] pub fn set_rxrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Ready 1"]
    #[inline] pub fn rxrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RXRDY1 != 0"]
    #[inline] pub fn test_rxrdy1(&self) -> bool {
        self.rxrdy1() != 0
    }

    #[doc="Sets the RXRDY1 field."]
    #[inline] pub fn set_rxrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive Overrun 0"]
    #[inline] pub fn rxor0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXOR0 != 0"]
    #[inline] pub fn test_rxor0(&self) -> bool {
        self.rxor0() != 0
    }

    #[doc="Sets the RXOR0 field."]
    #[inline] pub fn set_rxor0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive Overrun 1"]
    #[inline] pub fn rxor1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXOR1 != 0"]
    #[inline] pub fn test_rxor1(&self) -> bool {
        self.rxor1() != 0
    }

    #[doc="Sets the RXOR1 field."]
    #[inline] pub fn set_rxor1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmit Ready 0"]
    #[inline] pub fn txrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXRDY0 != 0"]
    #[inline] pub fn test_txrdy0(&self) -> bool {
        self.txrdy0() != 0
    }

    #[doc="Sets the TXRDY0 field."]
    #[inline] pub fn set_txrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmit Ready 1"]
    #[inline] pub fn txrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXRDY1 != 0"]
    #[inline] pub fn test_txrdy1(&self) -> bool {
        self.txrdy1() != 0
    }

    #[doc="Sets the TXRDY1 field."]
    #[inline] pub fn set_txrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit Underrun 0"]
    #[inline] pub fn txur0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXUR0 != 0"]
    #[inline] pub fn test_txur0(&self) -> bool {
        self.txur0() != 0
    }

    #[doc="Sets the TXUR0 field."]
    #[inline] pub fn set_txur0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Transmit Underrun 1"]
    #[inline] pub fn txur1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXUR1 != 0"]
    #[inline] pub fn test_txur1(&self) -> bool {
        self.txur1() != 0
    }

    #[doc="Sets the TXUR1 field."]
    #[inline] pub fn set_txur1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Intflag {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.rxrdy0() != 0 { try!(write!(f, " rxrdy0"))}
        if self.rxrdy1() != 0 { try!(write!(f, " rxrdy1"))}
        if self.rxor0() != 0 { try!(write!(f, " rxor0"))}
        if self.rxor1() != 0 { try!(write!(f, " rxor1"))}
        if self.txrdy0() != 0 { try!(write!(f, " txrdy0"))}
        if self.txrdy1() != 0 { try!(write!(f, " txrdy1"))}
        if self.txur0() != 0 { try!(write!(f, " txur0"))}
        if self.txur1() != 0 { try!(write!(f, " txur1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u16);
impl Syncbusy {
    #[doc="Software Reset Synchronization Status"]
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable Synchronization Status"]
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock Unit 0 Enable Synchronization Status"]
    #[inline] pub fn cken0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CKEN0 != 0"]
    #[inline] pub fn test_cken0(&self) -> bool {
        self.cken0() != 0
    }

    #[doc="Sets the CKEN0 field."]
    #[inline] pub fn set_cken0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clock Unit 1 Enable Synchronization Status"]
    #[inline] pub fn cken1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CKEN1 != 0"]
    #[inline] pub fn test_cken1(&self) -> bool {
        self.cken1() != 0
    }

    #[doc="Sets the CKEN1 field."]
    #[inline] pub fn set_cken1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Tx Serializer Enable Synchronization Status"]
    #[inline] pub fn txen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXEN != 0"]
    #[inline] pub fn test_txen(&self) -> bool {
        self.txen() != 0
    }

    #[doc="Sets the TXEN field."]
    #[inline] pub fn set_txen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Rx Serializer Enable Synchronization Status"]
    #[inline] pub fn rxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXEN != 0"]
    #[inline] pub fn test_rxen(&self) -> bool {
        self.rxen() != 0
    }

    #[doc="Sets the RXEN field."]
    #[inline] pub fn set_rxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Tx Data Synchronization Status"]
    #[inline] pub fn txdata(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXDATA != 0"]
    #[inline] pub fn test_txdata(&self) -> bool {
        self.txdata() != 0
    }

    #[doc="Sets the TXDATA field."]
    #[inline] pub fn set_txdata<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Rx Data Synchronization Status"]
    #[inline] pub fn rxdata(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXDATA != 0"]
    #[inline] pub fn test_rxdata(&self) -> bool {
        self.rxdata() != 0
    }

    #[doc="Sets the RXDATA field."]
    #[inline] pub fn set_rxdata<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u16> for Syncbusy {
    #[inline]
    fn from(other: u16) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.cken0() != 0 { try!(write!(f, " cken0"))}
        if self.cken1() != 0 { try!(write!(f, " cken1"))}
        if self.txen() != 0 { try!(write!(f, " txen"))}
        if self.rxen() != 0 { try!(write!(f, " rxen"))}
        if self.txdata() != 0 { try!(write!(f, " txdata"))}
        if self.rxdata() != 0 { try!(write!(f, " rxdata"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Serializer Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txctrl(pub u32);
impl Txctrl {
    #[doc="Line Default Line when Slot Disabled"]
    #[inline] pub fn txdefault(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if TXDEFAULT != 0"]
    #[inline] pub fn test_txdefault(&self) -> bool {
        self.txdefault() != 0
    }

    #[doc="Sets the TXDEFAULT field."]
    #[inline] pub fn set_txdefault<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit Data when Underrun"]
    #[inline] pub fn txsame(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXSAME != 0"]
    #[inline] pub fn test_txsame(&self) -> bool {
        self.txsame() != 0
    }

    #[doc="Sets the TXSAME field."]
    #[inline] pub fn set_txsame<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Slot Formatting Adjust"]
    #[inline] pub fn slotadj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SLOTADJ != 0"]
    #[inline] pub fn test_slotadj(&self) -> bool {
        self.slotadj() != 0
    }

    #[doc="Sets the SLOTADJ field."]
    #[inline] pub fn set_slotadj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Data Word Size"]
    #[inline] pub fn datasize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if DATASIZE != 0"]
    #[inline] pub fn test_datasize(&self) -> bool {
        self.datasize() != 0
    }

    #[doc="Sets the DATASIZE field."]
    #[inline] pub fn set_datasize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data Word Formatting Adjust"]
    #[inline] pub fn wordadj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WORDADJ != 0"]
    #[inline] pub fn test_wordadj(&self) -> bool {
        self.wordadj() != 0
    }

    #[doc="Sets the WORDADJ field."]
    #[inline] pub fn set_wordadj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Formatting Bit Extension"]
    #[inline] pub fn extend(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if EXTEND != 0"]
    #[inline] pub fn test_extend(&self) -> bool {
        self.extend() != 0
    }

    #[doc="Sets the EXTEND field."]
    #[inline] pub fn set_extend<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Data Formatting Bit Reverse"]
    #[inline] pub fn bitrev(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BITREV != 0"]
    #[inline] pub fn test_bitrev(&self) -> bool {
        self.bitrev() != 0
    }

    #[doc="Sets the BITREV field."]
    #[inline] pub fn set_bitrev<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Slot 0 Disabled for this Serializer"]
    #[inline] pub fn slotdis0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SLOTDIS0 != 0"]
    #[inline] pub fn test_slotdis0(&self) -> bool {
        self.slotdis0() != 0
    }

    #[doc="Sets the SLOTDIS0 field."]
    #[inline] pub fn set_slotdis0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Slot 1 Disabled for this Serializer"]
    #[inline] pub fn slotdis1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SLOTDIS1 != 0"]
    #[inline] pub fn test_slotdis1(&self) -> bool {
        self.slotdis1() != 0
    }

    #[doc="Sets the SLOTDIS1 field."]
    #[inline] pub fn set_slotdis1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Slot 2 Disabled for this Serializer"]
    #[inline] pub fn slotdis2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if SLOTDIS2 != 0"]
    #[inline] pub fn test_slotdis2(&self) -> bool {
        self.slotdis2() != 0
    }

    #[doc="Sets the SLOTDIS2 field."]
    #[inline] pub fn set_slotdis2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Slot 3 Disabled for this Serializer"]
    #[inline] pub fn slotdis3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if SLOTDIS3 != 0"]
    #[inline] pub fn test_slotdis3(&self) -> bool {
        self.slotdis3() != 0
    }

    #[doc="Sets the SLOTDIS3 field."]
    #[inline] pub fn set_slotdis3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Slot 4 Disabled for this Serializer"]
    #[inline] pub fn slotdis4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SLOTDIS4 != 0"]
    #[inline] pub fn test_slotdis4(&self) -> bool {
        self.slotdis4() != 0
    }

    #[doc="Sets the SLOTDIS4 field."]
    #[inline] pub fn set_slotdis4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Slot 5 Disabled for this Serializer"]
    #[inline] pub fn slotdis5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SLOTDIS5 != 0"]
    #[inline] pub fn test_slotdis5(&self) -> bool {
        self.slotdis5() != 0
    }

    #[doc="Sets the SLOTDIS5 field."]
    #[inline] pub fn set_slotdis5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Slot 6 Disabled for this Serializer"]
    #[inline] pub fn slotdis6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SLOTDIS6 != 0"]
    #[inline] pub fn test_slotdis6(&self) -> bool {
        self.slotdis6() != 0
    }

    #[doc="Sets the SLOTDIS6 field."]
    #[inline] pub fn set_slotdis6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Slot 7 Disabled for this Serializer"]
    #[inline] pub fn slotdis7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if SLOTDIS7 != 0"]
    #[inline] pub fn test_slotdis7(&self) -> bool {
        self.slotdis7() != 0
    }

    #[doc="Sets the SLOTDIS7 field."]
    #[inline] pub fn set_slotdis7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Mono Mode"]
    #[inline] pub fn mono(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if MONO != 0"]
    #[inline] pub fn test_mono(&self) -> bool {
        self.mono() != 0
    }

    #[doc="Sets the MONO field."]
    #[inline] pub fn set_mono<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Single or Multiple DMA Channels"]
    #[inline] pub fn dma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

}

impl From<u32> for Txctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Txctrl(other)
    }
}

impl ::core::fmt::Display for Txctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdefault() != 0 { try!(write!(f, " txdefault=0x{:x}", self.txdefault()))}
        if self.txsame() != 0 { try!(write!(f, " txsame"))}
        if self.slotadj() != 0 { try!(write!(f, " slotadj"))}
        if self.datasize() != 0 { try!(write!(f, " datasize=0x{:x}", self.datasize()))}
        if self.wordadj() != 0 { try!(write!(f, " wordadj"))}
        if self.extend() != 0 { try!(write!(f, " extend=0x{:x}", self.extend()))}
        if self.bitrev() != 0 { try!(write!(f, " bitrev"))}
        if self.slotdis0() != 0 { try!(write!(f, " slotdis0"))}
        if self.slotdis1() != 0 { try!(write!(f, " slotdis1"))}
        if self.slotdis2() != 0 { try!(write!(f, " slotdis2"))}
        if self.slotdis3() != 0 { try!(write!(f, " slotdis3"))}
        if self.slotdis4() != 0 { try!(write!(f, " slotdis4"))}
        if self.slotdis5() != 0 { try!(write!(f, " slotdis5"))}
        if self.slotdis6() != 0 { try!(write!(f, " slotdis6"))}
        if self.slotdis7() != 0 { try!(write!(f, " slotdis7"))}
        if self.mono() != 0 { try!(write!(f, " mono"))}
        if self.dma() != 0 { try!(write!(f, " dma"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Serializer Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxctrl(pub u32);
impl Rxctrl {
    #[doc="Serializer Mode"]
    #[inline] pub fn sermode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SERMODE != 0"]
    #[inline] pub fn test_sermode(&self) -> bool {
        self.sermode() != 0
    }

    #[doc="Sets the SERMODE field."]
    #[inline] pub fn set_sermode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Unit Selection"]
    #[inline] pub fn clksel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CLKSEL != 0"]
    #[inline] pub fn test_clksel(&self) -> bool {
        self.clksel() != 0
    }

    #[doc="Sets the CLKSEL field."]
    #[inline] pub fn set_clksel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Data Slot Formatting Adjust"]
    #[inline] pub fn slotadj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SLOTADJ != 0"]
    #[inline] pub fn test_slotadj(&self) -> bool {
        self.slotadj() != 0
    }

    #[doc="Sets the SLOTADJ field."]
    #[inline] pub fn set_slotadj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Data Word Size"]
    #[inline] pub fn datasize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if DATASIZE != 0"]
    #[inline] pub fn test_datasize(&self) -> bool {
        self.datasize() != 0
    }

    #[doc="Sets the DATASIZE field."]
    #[inline] pub fn set_datasize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data Word Formatting Adjust"]
    #[inline] pub fn wordadj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WORDADJ != 0"]
    #[inline] pub fn test_wordadj(&self) -> bool {
        self.wordadj() != 0
    }

    #[doc="Sets the WORDADJ field."]
    #[inline] pub fn set_wordadj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Formatting Bit Extension"]
    #[inline] pub fn extend(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if EXTEND != 0"]
    #[inline] pub fn test_extend(&self) -> bool {
        self.extend() != 0
    }

    #[doc="Sets the EXTEND field."]
    #[inline] pub fn set_extend<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Data Formatting Bit Reverse"]
    #[inline] pub fn bitrev(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BITREV != 0"]
    #[inline] pub fn test_bitrev(&self) -> bool {
        self.bitrev() != 0
    }

    #[doc="Sets the BITREV field."]
    #[inline] pub fn set_bitrev<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Slot 0 Disabled for this Serializer"]
    #[inline] pub fn slotdis0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SLOTDIS0 != 0"]
    #[inline] pub fn test_slotdis0(&self) -> bool {
        self.slotdis0() != 0
    }

    #[doc="Sets the SLOTDIS0 field."]
    #[inline] pub fn set_slotdis0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Slot 1 Disabled for this Serializer"]
    #[inline] pub fn slotdis1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SLOTDIS1 != 0"]
    #[inline] pub fn test_slotdis1(&self) -> bool {
        self.slotdis1() != 0
    }

    #[doc="Sets the SLOTDIS1 field."]
    #[inline] pub fn set_slotdis1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Slot 2 Disabled for this Serializer"]
    #[inline] pub fn slotdis2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if SLOTDIS2 != 0"]
    #[inline] pub fn test_slotdis2(&self) -> bool {
        self.slotdis2() != 0
    }

    #[doc="Sets the SLOTDIS2 field."]
    #[inline] pub fn set_slotdis2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Slot 3 Disabled for this Serializer"]
    #[inline] pub fn slotdis3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if SLOTDIS3 != 0"]
    #[inline] pub fn test_slotdis3(&self) -> bool {
        self.slotdis3() != 0
    }

    #[doc="Sets the SLOTDIS3 field."]
    #[inline] pub fn set_slotdis3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Slot 4 Disabled for this Serializer"]
    #[inline] pub fn slotdis4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SLOTDIS4 != 0"]
    #[inline] pub fn test_slotdis4(&self) -> bool {
        self.slotdis4() != 0
    }

    #[doc="Sets the SLOTDIS4 field."]
    #[inline] pub fn set_slotdis4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Slot 5 Disabled for this Serializer"]
    #[inline] pub fn slotdis5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SLOTDIS5 != 0"]
    #[inline] pub fn test_slotdis5(&self) -> bool {
        self.slotdis5() != 0
    }

    #[doc="Sets the SLOTDIS5 field."]
    #[inline] pub fn set_slotdis5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Slot 6 Disabled for this Serializer"]
    #[inline] pub fn slotdis6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SLOTDIS6 != 0"]
    #[inline] pub fn test_slotdis6(&self) -> bool {
        self.slotdis6() != 0
    }

    #[doc="Sets the SLOTDIS6 field."]
    #[inline] pub fn set_slotdis6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Slot 7 Disabled for this Serializer"]
    #[inline] pub fn slotdis7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if SLOTDIS7 != 0"]
    #[inline] pub fn test_slotdis7(&self) -> bool {
        self.slotdis7() != 0
    }

    #[doc="Sets the SLOTDIS7 field."]
    #[inline] pub fn set_slotdis7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Mono Mode"]
    #[inline] pub fn mono(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if MONO != 0"]
    #[inline] pub fn test_mono(&self) -> bool {
        self.mono() != 0
    }

    #[doc="Sets the MONO field."]
    #[inline] pub fn set_mono<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Single or Multiple DMA Channels"]
    #[inline] pub fn dma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Loop-back Test Mode"]
    #[inline] pub fn rxloop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if RXLOOP != 0"]
    #[inline] pub fn test_rxloop(&self) -> bool {
        self.rxloop() != 0
    }

    #[doc="Sets the RXLOOP field."]
    #[inline] pub fn set_rxloop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Rxctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Rxctrl(other)
    }
}

impl ::core::fmt::Display for Rxctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sermode() != 0 { try!(write!(f, " sermode=0x{:x}", self.sermode()))}
        if self.clksel() != 0 { try!(write!(f, " clksel"))}
        if self.slotadj() != 0 { try!(write!(f, " slotadj"))}
        if self.datasize() != 0 { try!(write!(f, " datasize=0x{:x}", self.datasize()))}
        if self.wordadj() != 0 { try!(write!(f, " wordadj"))}
        if self.extend() != 0 { try!(write!(f, " extend=0x{:x}", self.extend()))}
        if self.bitrev() != 0 { try!(write!(f, " bitrev"))}
        if self.slotdis0() != 0 { try!(write!(f, " slotdis0"))}
        if self.slotdis1() != 0 { try!(write!(f, " slotdis1"))}
        if self.slotdis2() != 0 { try!(write!(f, " slotdis2"))}
        if self.slotdis3() != 0 { try!(write!(f, " slotdis3"))}
        if self.slotdis4() != 0 { try!(write!(f, " slotdis4"))}
        if self.slotdis5() != 0 { try!(write!(f, " slotdis5"))}
        if self.slotdis6() != 0 { try!(write!(f, " slotdis6"))}
        if self.slotdis7() != 0 { try!(write!(f, " slotdis7"))}
        if self.mono() != 0 { try!(write!(f, " mono"))}
        if self.dma() != 0 { try!(write!(f, " dma"))}
        if self.rxloop() != 0 { try!(write!(f, " rxloop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdata(pub u32);
impl Txdata {
    #[doc="Sample Data"]
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdata(pub u32);
impl Rxdata {
    #[doc="Sample Data"]
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
        try!(write!(f, "]"));
        Ok(())
    }
}

