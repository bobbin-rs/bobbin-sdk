::bobbin_mcu::periph!( AC, Ac, AC_PERIPH, AcPeriph, AC_OWNED, AC_REF_COUNT, 0x42002000, 0x00, 0x01);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="AC Peripheral"]
pub struct AcPeriph(pub usize); 

impl AcPeriph {
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x1)
    }

    #[doc="Get the *mut pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_mut(&self) -> *mut Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_ptr(&self) -> *const Ctrlb { 
        self.ctrlb_reg().ptr()
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

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x2)
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        self.evctrl_reg().read()
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn write_evctrl(&self, value: Evctrl) -> &Self { 
        self.evctrl_reg().write(value);
        self
    }

    #[doc="Set the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().set(f);
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x4)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x5)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x6)
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

    #[doc="Get the STATUSA Register."]
    #[inline] pub fn statusa_reg(&self) -> ::bobbin_mcu::register::Register<Statusa> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Statusa, 0x7)
    }

    #[doc="Get the *mut pointer for the STATUSA register."]
    #[inline] pub fn statusa_mut(&self) -> *mut Statusa { 
        self.statusa_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUSA register."]
    #[inline] pub fn statusa_ptr(&self) -> *const Statusa { 
        self.statusa_reg().ptr()
    }

    #[doc="Read the STATUSA register."]
    #[inline] pub fn statusa(&self) -> Statusa { 
        self.statusa_reg().read()
    }

    #[doc="Get the STATUSB Register."]
    #[inline] pub fn statusb_reg(&self) -> ::bobbin_mcu::register::Register<Statusb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Statusb, 0x8)
    }

    #[doc="Get the *mut pointer for the STATUSB register."]
    #[inline] pub fn statusb_mut(&self) -> *mut Statusb { 
        self.statusb_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUSB register."]
    #[inline] pub fn statusb_ptr(&self) -> *const Statusb { 
        self.statusb_reg().ptr()
    }

    #[doc="Read the STATUSB register."]
    #[inline] pub fn statusb(&self) -> Statusb { 
        self.statusb_reg().read()
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x9)
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut Dbgctrl { 
        self.dbgctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_ptr(&self) -> *const Dbgctrl { 
        self.dbgctrl_reg().ptr()
    }

    #[doc="Read the DBGCTRL register."]
    #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
        self.dbgctrl_reg().read()
    }

    #[doc="Write the DBGCTRL register."]
    #[inline] pub fn write_dbgctrl(&self, value: Dbgctrl) -> &Self { 
        self.dbgctrl_reg().write(value);
        self
    }

    #[doc="Set the DBGCTRL register."]
    #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        self.dbgctrl_reg().set(f);
        self
    }

    #[doc="Modify the DBGCTRL register."]
    #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        self.dbgctrl_reg().with(f);
        self
    }

    #[doc="Get the WINCTRL Register."]
    #[inline] pub fn winctrl_reg(&self) -> ::bobbin_mcu::register::Register<Winctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Winctrl, 0xa)
    }

    #[doc="Get the *mut pointer for the WINCTRL register."]
    #[inline] pub fn winctrl_mut(&self) -> *mut Winctrl { 
        self.winctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the WINCTRL register."]
    #[inline] pub fn winctrl_ptr(&self) -> *const Winctrl { 
        self.winctrl_reg().ptr()
    }

    #[doc="Read the WINCTRL register."]
    #[inline] pub fn winctrl(&self) -> Winctrl { 
        self.winctrl_reg().read()
    }

    #[doc="Write the WINCTRL register."]
    #[inline] pub fn write_winctrl(&self, value: Winctrl) -> &Self { 
        self.winctrl_reg().write(value);
        self
    }

    #[doc="Set the WINCTRL register."]
    #[inline] pub fn set_winctrl<F: FnOnce(Winctrl) -> Winctrl>(&self, f: F) -> &Self {
        self.winctrl_reg().set(f);
        self
    }

    #[doc="Modify the WINCTRL register."]
    #[inline] pub fn with_winctrl<F: FnOnce(Winctrl) -> Winctrl>(&self, f: F) -> &Self {
        self.winctrl_reg().with(f);
        self
    }

    #[doc="Get the SCALER Register."]
    #[inline] pub fn scaler_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Scaler, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Scaler, 0xc, 0x1)
    }

    #[doc="Get the *mut pointer for the SCALER register."]
    #[inline] pub fn scaler_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Scaler { 
        self.scaler_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the SCALER register."]
    #[inline] pub fn scaler_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Scaler { 
        self.scaler_reg().ptr(index.into())
    }

    #[doc="Read the SCALER register."]
    #[inline] pub fn scaler<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Scaler { 
        self.scaler_reg().read(index.into())
    }

    #[doc="Write the SCALER register."]
    #[inline] pub fn write_scaler<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Scaler) -> &Self {
        self.scaler_reg().write(index.into(), value);
        self
    }

    #[doc="Set the SCALER register."]
    #[inline] pub fn set_scaler<I: Into<::bobbin_bits::R2>, F: FnOnce(Scaler) -> Scaler>(&self, index: I, f: F) -> &Self {
        self.scaler_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the SCALER register."]
    #[inline] pub fn with_scaler<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Scaler) -> Scaler>(&self, index: I, f: F) -> &Self {
        self.scaler_reg().with(index.into(), f);
        self
    }

    #[doc="Get the COMPCTRL Register."]
    #[inline] pub fn compctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Compctrl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Compctrl, 0x10, 0x4)
    }

    #[doc="Get the *mut pointer for the COMPCTRL register."]
    #[inline] pub fn compctrl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Compctrl { 
        self.compctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the COMPCTRL register."]
    #[inline] pub fn compctrl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Compctrl { 
        self.compctrl_reg().ptr(index.into())
    }

    #[doc="Read the COMPCTRL register."]
    #[inline] pub fn compctrl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Compctrl { 
        self.compctrl_reg().read(index.into())
    }

    #[doc="Write the COMPCTRL register."]
    #[inline] pub fn write_compctrl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Compctrl) -> &Self {
        self.compctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the COMPCTRL register."]
    #[inline] pub fn set_compctrl<I: Into<::bobbin_bits::R2>, F: FnOnce(Compctrl) -> Compctrl>(&self, index: I, f: F) -> &Self {
        self.compctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the COMPCTRL register."]
    #[inline] pub fn with_compctrl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Compctrl) -> Compctrl>(&self, index: I, f: F) -> &Self {
        self.compctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x20)
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

    #[doc="Get the CALIB Register."]
    #[inline] pub fn calib_reg(&self) -> ::bobbin_mcu::register::Register<Calib> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Calib, 0x24)
    }

    #[doc="Get the *mut pointer for the CALIB register."]
    #[inline] pub fn calib_mut(&self) -> *mut Calib { 
        self.calib_reg().ptr()
    }

    #[doc="Get the *const pointer for the CALIB register."]
    #[inline] pub fn calib_ptr(&self) -> *const Calib { 
        self.calib_reg().ptr()
    }

    #[doc="Read the CALIB register."]
    #[inline] pub fn calib(&self) -> Calib { 
        self.calib_reg().read()
    }

    #[doc="Write the CALIB register."]
    #[inline] pub fn write_calib(&self, value: Calib) -> &Self { 
        self.calib_reg().write(value);
        self
    }

    #[doc="Set the CALIB register."]
    #[inline] pub fn set_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        self.calib_reg().set(f);
        self
    }

    #[doc="Modify the CALIB register."]
    #[inline] pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        self.calib_reg().with(f);
        self
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u8);
impl Ctrlb {
    #[doc="Comparator 0 Start Comparison"]
    #[inline] pub fn start0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if START0 != 0"]
    #[inline] pub fn test_start0(&self) -> bool {
        self.start0() != 0
    }

    #[doc="Sets the START0 field."]
    #[inline] pub fn set_start0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator 1 Start Comparison"]
    #[inline] pub fn start1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if START1 != 0"]
    #[inline] pub fn test_start1(&self) -> bool {
        self.start1() != 0
    }

    #[doc="Sets the START1 field."]
    #[inline] pub fn set_start1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Ctrlb {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.start0() != 0 { try!(write!(f, " start0"))}
        if self.start1() != 0 { try!(write!(f, " start1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
    #[doc="Comparator 0 Event Output Enable"]
    #[inline] pub fn compeo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPEO0 != 0"]
    #[inline] pub fn test_compeo0(&self) -> bool {
        self.compeo0() != 0
    }

    #[doc="Sets the COMPEO0 field."]
    #[inline] pub fn set_compeo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator 1 Event Output Enable"]
    #[inline] pub fn compeo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPEO1 != 0"]
    #[inline] pub fn test_compeo1(&self) -> bool {
        self.compeo1() != 0
    }

    #[doc="Sets the COMPEO1 field."]
    #[inline] pub fn set_compeo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window 0 Event Output Enable"]
    #[inline] pub fn wineo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WINEO0 != 0"]
    #[inline] pub fn test_wineo0(&self) -> bool {
        self.wineo0() != 0
    }

    #[doc="Sets the WINEO0 field."]
    #[inline] pub fn set_wineo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Comparator 0 Event Input Enable"]
    #[inline] pub fn compei0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if COMPEI0 != 0"]
    #[inline] pub fn test_compei0(&self) -> bool {
        self.compei0() != 0
    }

    #[doc="Sets the COMPEI0 field."]
    #[inline] pub fn set_compei0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Comparator 1 Event Input Enable"]
    #[inline] pub fn compei1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if COMPEI1 != 0"]
    #[inline] pub fn test_compei1(&self) -> bool {
        self.compei1() != 0
    }

    #[doc="Sets the COMPEI1 field."]
    #[inline] pub fn set_compei1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Comparator 0 Input Event Invert Enable"]
    #[inline] pub fn invei0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if INVEI0 != 0"]
    #[inline] pub fn test_invei0(&self) -> bool {
        self.invei0() != 0
    }

    #[doc="Sets the INVEI0 field."]
    #[inline] pub fn set_invei0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Comparator 1 Input Event Invert Enable"]
    #[inline] pub fn invei1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if INVEI1 != 0"]
    #[inline] pub fn test_invei1(&self) -> bool {
        self.invei1() != 0
    }

    #[doc="Sets the INVEI1 field."]
    #[inline] pub fn set_invei1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Evctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.compeo0() != 0 { try!(write!(f, " compeo0"))}
        if self.compeo1() != 0 { try!(write!(f, " compeo1"))}
        if self.wineo0() != 0 { try!(write!(f, " wineo0"))}
        if self.compei0() != 0 { try!(write!(f, " compei0"))}
        if self.compei1() != 0 { try!(write!(f, " compei1"))}
        if self.invei0() != 0 { try!(write!(f, " invei0"))}
        if self.invei1() != 0 { try!(write!(f, " invei1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Comparator 0 Interrupt Enable"]
    #[inline] pub fn comp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMP0 != 0"]
    #[inline] pub fn test_comp0(&self) -> bool {
        self.comp0() != 0
    }

    #[doc="Sets the COMP0 field."]
    #[inline] pub fn set_comp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator 1 Interrupt Enable"]
    #[inline] pub fn comp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMP1 != 0"]
    #[inline] pub fn test_comp1(&self) -> bool {
        self.comp1() != 0
    }

    #[doc="Sets the COMP1 field."]
    #[inline] pub fn set_comp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window 0 Interrupt Enable"]
    #[inline] pub fn win0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WIN0 != 0"]
    #[inline] pub fn test_win0(&self) -> bool {
        self.win0() != 0
    }

    #[doc="Sets the WIN0 field."]
    #[inline] pub fn set_win0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.comp0() != 0 { try!(write!(f, " comp0"))}
        if self.comp1() != 0 { try!(write!(f, " comp1"))}
        if self.win0() != 0 { try!(write!(f, " win0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Comparator 0 Interrupt Enable"]
    #[inline] pub fn comp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMP0 != 0"]
    #[inline] pub fn test_comp0(&self) -> bool {
        self.comp0() != 0
    }

    #[doc="Sets the COMP0 field."]
    #[inline] pub fn set_comp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator 1 Interrupt Enable"]
    #[inline] pub fn comp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMP1 != 0"]
    #[inline] pub fn test_comp1(&self) -> bool {
        self.comp1() != 0
    }

    #[doc="Sets the COMP1 field."]
    #[inline] pub fn set_comp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window 0 Interrupt Enable"]
    #[inline] pub fn win0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WIN0 != 0"]
    #[inline] pub fn test_win0(&self) -> bool {
        self.win0() != 0
    }

    #[doc="Sets the WIN0 field."]
    #[inline] pub fn set_win0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.comp0() != 0 { try!(write!(f, " comp0"))}
        if self.comp1() != 0 { try!(write!(f, " comp1"))}
        if self.win0() != 0 { try!(write!(f, " win0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Comparator 0"]
    #[inline] pub fn comp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMP0 != 0"]
    #[inline] pub fn test_comp0(&self) -> bool {
        self.comp0() != 0
    }

    #[doc="Sets the COMP0 field."]
    #[inline] pub fn set_comp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator 1"]
    #[inline] pub fn comp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMP1 != 0"]
    #[inline] pub fn test_comp1(&self) -> bool {
        self.comp1() != 0
    }

    #[doc="Sets the COMP1 field."]
    #[inline] pub fn set_comp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window 0"]
    #[inline] pub fn win0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WIN0 != 0"]
    #[inline] pub fn test_win0(&self) -> bool {
        self.win0() != 0
    }

    #[doc="Sets the WIN0 field."]
    #[inline] pub fn set_win0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.comp0() != 0 { try!(write!(f, " comp0"))}
        if self.comp1() != 0 { try!(write!(f, " comp1"))}
        if self.win0() != 0 { try!(write!(f, " win0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusa(pub u8);
impl Statusa {
    #[doc="Comparator 0 Current State"]
    #[inline] pub fn state0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STATE0 != 0"]
    #[inline] pub fn test_state0(&self) -> bool {
        self.state0() != 0
    }

    #[doc="Sets the STATE0 field."]
    #[inline] pub fn set_state0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator 1 Current State"]
    #[inline] pub fn state1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STATE1 != 0"]
    #[inline] pub fn test_state1(&self) -> bool {
        self.state1() != 0
    }

    #[doc="Sets the STATE1 field."]
    #[inline] pub fn set_state1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window 0 Current State"]
    #[inline] pub fn wstate0(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if WSTATE0 != 0"]
    #[inline] pub fn test_wstate0(&self) -> bool {
        self.wstate0() != 0
    }

    #[doc="Sets the WSTATE0 field."]
    #[inline] pub fn set_wstate0<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Statusa {
    #[inline]
    fn from(other: u8) -> Self {
         Statusa(other)
    }
}

impl ::core::fmt::Display for Statusa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Statusa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.state0() != 0 { try!(write!(f, " state0"))}
        if self.state1() != 0 { try!(write!(f, " state1"))}
        if self.wstate0() != 0 { try!(write!(f, " wstate0=0x{:x}", self.wstate0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusb(pub u8);
impl Statusb {
    #[doc="Comparator 0 Ready"]
    #[inline] pub fn ready0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if READY0 != 0"]
    #[inline] pub fn test_ready0(&self) -> bool {
        self.ready0() != 0
    }

    #[doc="Sets the READY0 field."]
    #[inline] pub fn set_ready0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator 1 Ready"]
    #[inline] pub fn ready1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if READY1 != 0"]
    #[inline] pub fn test_ready1(&self) -> bool {
        self.ready1() != 0
    }

    #[doc="Sets the READY1 field."]
    #[inline] pub fn set_ready1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Statusb {
    #[inline]
    fn from(other: u8) -> Self {
         Statusb(other)
    }
}

impl ::core::fmt::Display for Statusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Statusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ready0() != 0 { try!(write!(f, " ready0"))}
        if self.ready1() != 0 { try!(write!(f, " ready1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Run"]
    #[inline] pub fn dbgrun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGRUN != 0"]
    #[inline] pub fn test_dbgrun(&self) -> bool {
        self.dbgrun() != 0
    }

    #[doc="Sets the DBGRUN field."]
    #[inline] pub fn set_dbgrun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Dbgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Dbgctrl(other)
    }
}

impl ::core::fmt::Display for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winctrl(pub u8);
impl Winctrl {
    #[doc="Window 0 Mode Enable"]
    #[inline] pub fn wen0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WEN0 != 0"]
    #[inline] pub fn test_wen0(&self) -> bool {
        self.wen0() != 0
    }

    #[doc="Sets the WEN0 field."]
    #[inline] pub fn set_wen0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Window 0 Interrupt Selection"]
    #[inline] pub fn wintsel0(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if WINTSEL0 != 0"]
    #[inline] pub fn test_wintsel0(&self) -> bool {
        self.wintsel0() != 0
    }

    #[doc="Sets the WINTSEL0 field."]
    #[inline] pub fn set_wintsel0<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Winctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Winctrl(other)
    }
}

impl ::core::fmt::Display for Winctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wen0() != 0 { try!(write!(f, " wen0"))}
        if self.wintsel0() != 0 { try!(write!(f, " wintsel0=0x{:x}", self.wintsel0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Scaler n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scaler(pub u8);
impl Scaler {
    #[doc="Scaler Value"]
    #[inline] pub fn value(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Scaler {
    #[inline]
    fn from(other: u8) -> Self {
         Scaler(other)
    }
}

impl ::core::fmt::Display for Scaler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scaler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.value() != 0 { try!(write!(f, " value=0x{:x}", self.value()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Comparator Control n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Compctrl(pub u32);
impl Compctrl {
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

    #[doc="Single-Shot Mode"]
    #[inline] pub fn single(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SINGLE != 0"]
    #[inline] pub fn test_single(&self) -> bool {
        self.single() != 0
    }

    #[doc="Sets the SINGLE field."]
    #[inline] pub fn set_single<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Interrupt Selection"]
    #[inline] pub fn intsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if INTSEL != 0"]
    #[inline] pub fn test_intsel(&self) -> bool {
        self.intsel() != 0
    }

    #[doc="Sets the INTSEL field."]
    #[inline] pub fn set_intsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Negative Input Mux Selection"]
    #[inline] pub fn muxneg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if MUXNEG != 0"]
    #[inline] pub fn test_muxneg(&self) -> bool {
        self.muxneg() != 0
    }

    #[doc="Sets the MUXNEG field."]
    #[inline] pub fn set_muxneg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Positive Input Mux Selection"]
    #[inline] pub fn muxpos(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if MUXPOS != 0"]
    #[inline] pub fn test_muxpos(&self) -> bool {
        self.muxpos() != 0
    }

    #[doc="Sets the MUXPOS field."]
    #[inline] pub fn set_muxpos<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Swap Inputs and Invert"]
    #[inline] pub fn swap(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SWAP != 0"]
    #[inline] pub fn test_swap(&self) -> bool {
        self.swap() != 0
    }

    #[doc="Sets the SWAP field."]
    #[inline] pub fn set_swap<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Speed Selection"]
    #[inline] pub fn speed(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if SPEED != 0"]
    #[inline] pub fn test_speed(&self) -> bool {
        self.speed() != 0
    }

    #[doc="Sets the SPEED field."]
    #[inline] pub fn set_speed<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Hysteresis Enable"]
    #[inline] pub fn hysten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HYSTEN != 0"]
    #[inline] pub fn test_hysten(&self) -> bool {
        self.hysten() != 0
    }

    #[doc="Sets the HYSTEN field."]
    #[inline] pub fn set_hysten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Hysteresis Level"]
    #[inline] pub fn hyst(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if HYST != 0"]
    #[inline] pub fn test_hyst(&self) -> bool {
        self.hyst() != 0
    }

    #[doc="Sets the HYST field."]
    #[inline] pub fn set_hyst<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Filter Length"]
    #[inline] pub fn flen(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if FLEN != 0"]
    #[inline] pub fn test_flen(&self) -> bool {
        self.flen() != 0
    }

    #[doc="Sets the FLEN field."]
    #[inline] pub fn set_flen<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Output"]
    #[inline] pub fn out(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if OUT != 0"]
    #[inline] pub fn test_out(&self) -> bool {
        self.out() != 0
    }

    #[doc="Sets the OUT field."]
    #[inline] pub fn set_out<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Compctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Compctrl(other)
    }
}

impl ::core::fmt::Display for Compctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Compctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.single() != 0 { try!(write!(f, " single"))}
        if self.intsel() != 0 { try!(write!(f, " intsel=0x{:x}", self.intsel()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.muxneg() != 0 { try!(write!(f, " muxneg=0x{:x}", self.muxneg()))}
        if self.muxpos() != 0 { try!(write!(f, " muxpos=0x{:x}", self.muxpos()))}
        if self.swap() != 0 { try!(write!(f, " swap"))}
        if self.speed() != 0 { try!(write!(f, " speed=0x{:x}", self.speed()))}
        if self.hysten() != 0 { try!(write!(f, " hysten"))}
        if self.hyst() != 0 { try!(write!(f, " hyst=0x{:x}", self.hyst()))}
        if self.flen() != 0 { try!(write!(f, " flen=0x{:x}", self.flen()))}
        if self.out() != 0 { try!(write!(f, " out=0x{:x}", self.out()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Software Reset Synchronization Busy"]
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

    #[doc="Enable Synchronization Busy"]
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

    #[doc="WINCTRL Synchronization Busy"]
    #[inline] pub fn winctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WINCTRL != 0"]
    #[inline] pub fn test_winctrl(&self) -> bool {
        self.winctrl() != 0
    }

    #[doc="Sets the WINCTRL field."]
    #[inline] pub fn set_winctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COMPCTRL 0 Synchronization Busy"]
    #[inline] pub fn compctrl0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPCTRL0 != 0"]
    #[inline] pub fn test_compctrl0(&self) -> bool {
        self.compctrl0() != 0
    }

    #[doc="Sets the COMPCTRL0 field."]
    #[inline] pub fn set_compctrl0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COMPCTRL 1 Synchronization Busy"]
    #[inline] pub fn compctrl1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPCTRL1 != 0"]
    #[inline] pub fn test_compctrl1(&self) -> bool {
        self.compctrl1() != 0
    }

    #[doc="Sets the COMPCTRL1 field."]
    #[inline] pub fn set_compctrl1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.winctrl() != 0 { try!(write!(f, " winctrl"))}
        if self.compctrl0() != 0 { try!(write!(f, " compctrl0"))}
        if self.compctrl1() != 0 { try!(write!(f, " compctrl1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Calibration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calib(pub u16);
impl Calib {
    #[doc="COMP0/1 Bias Scaling"]
    #[inline] pub fn bias0(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if BIAS0 != 0"]
    #[inline] pub fn test_bias0(&self) -> bool {
        self.bias0() != 0
    }

    #[doc="Sets the BIAS0 field."]
    #[inline] pub fn set_bias0<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Calib {
    #[inline]
    fn from(other: u16) -> Self {
         Calib(other)
    }
}

impl ::core::fmt::Display for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bias0() != 0 { try!(write!(f, " bias0=0x{:x}", self.bias0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

