::bobbin_mcu::periph!( DAC, Dac, DAC_PERIPH, DacPeriph, DAC_OWNED, DAC_REF_COUNT, 0x43002400, 0x00, 0x06);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DAC Peripheral"]
pub struct DacPeriph(pub usize); 

impl DacPeriph {
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

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x7)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x8)
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

    #[doc="Get the DACCTRL Register."]
    #[inline] pub fn dacctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dacctrl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dacctrl, 0xc, 0x2)
    }

    #[doc="Get the *mut pointer for the DACCTRL register."]
    #[inline] pub fn dacctrl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dacctrl { 
        self.dacctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DACCTRL register."]
    #[inline] pub fn dacctrl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dacctrl { 
        self.dacctrl_reg().ptr(index.into())
    }

    #[doc="Read the DACCTRL register."]
    #[inline] pub fn dacctrl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dacctrl { 
        self.dacctrl_reg().read(index.into())
    }

    #[doc="Write the DACCTRL register."]
    #[inline] pub fn write_dacctrl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dacctrl) -> &Self {
        self.dacctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DACCTRL register."]
    #[inline] pub fn set_dacctrl<I: Into<::bobbin_bits::R2>, F: FnOnce(Dacctrl) -> Dacctrl>(&self, index: I, f: F) -> &Self {
        self.dacctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DACCTRL register."]
    #[inline] pub fn with_dacctrl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dacctrl) -> Dacctrl>(&self, index: I, f: F) -> &Self {
        self.dacctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DATA Register."]
    #[inline] pub fn data_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Data, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Data, 0x10, 0x2)
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Data { 
        self.data_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Data { 
        self.data_reg().ptr(index.into())
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn write_data<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Data) -> &Self {
        self.data_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DATA register."]
    #[inline] pub fn set_data<I: Into<::bobbin_bits::R2>, F: FnOnce(Data) -> Data>(&self, index: I, f: F) -> &Self {
        self.data_reg().set(index.into(), f);
        self
    }

    #[doc="Get the DATABUF Register."]
    #[inline] pub fn databuf_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Databuf, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Databuf, 0x14, 0x2)
    }

    #[doc="Get the *mut pointer for the DATABUF register."]
    #[inline] pub fn databuf_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Databuf { 
        self.databuf_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DATABUF register."]
    #[inline] pub fn databuf_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Databuf { 
        self.databuf_reg().ptr(index.into())
    }

    #[doc="Write the DATABUF register."]
    #[inline] pub fn write_databuf<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Databuf) -> &Self {
        self.databuf_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DATABUF register."]
    #[inline] pub fn set_databuf<I: Into<::bobbin_bits::R2>, F: FnOnce(Databuf) -> Databuf>(&self, index: I, f: F) -> &Self {
        self.databuf_reg().set(index.into(), f);
        self
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x18)
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

    #[doc="Get the RESULT Register."]
    #[inline] pub fn result_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Result, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Result, 0x1c, 0x2)
    }

    #[doc="Get the *mut pointer for the RESULT register."]
    #[inline] pub fn result_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Result { 
        self.result_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the RESULT register."]
    #[inline] pub fn result_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Result { 
        self.result_reg().ptr(index.into())
    }

    #[doc="Read the RESULT register."]
    #[inline] pub fn result<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Result { 
        self.result_reg().read(index.into())
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

    #[doc="Enable DAC Controller"]
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
    #[doc="Differential mode enable"]
    #[inline] pub fn diff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIFF != 0"]
    #[inline] pub fn test_diff(&self) -> bool {
        self.diff() != 0
    }

    #[doc="Sets the DIFF field."]
    #[inline] pub fn set_diff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Reference Selection for DAC0/1"]
    #[inline] pub fn refsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if REFSEL != 0"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel() != 0
    }

    #[doc="Sets the REFSEL field."]
    #[inline] pub fn set_refsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 1);
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
        if self.diff() != 0 { try!(write!(f, " diff"))}
        if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u8);
impl Evctrl {
    #[doc="Start Conversion Event Input DAC 0"]
    #[inline] pub fn startei0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STARTEI0 != 0"]
    #[inline] pub fn test_startei0(&self) -> bool {
        self.startei0() != 0
    }

    #[doc="Sets the STARTEI0 field."]
    #[inline] pub fn set_startei0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Conversion Event Input DAC 1"]
    #[inline] pub fn startei1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STARTEI1 != 0"]
    #[inline] pub fn test_startei1(&self) -> bool {
        self.startei1() != 0
    }

    #[doc="Sets the STARTEI1 field."]
    #[inline] pub fn set_startei1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Buffer Empty Event Output DAC 0"]
    #[inline] pub fn emptyeo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EMPTYEO0 != 0"]
    #[inline] pub fn test_emptyeo0(&self) -> bool {
        self.emptyeo0() != 0
    }

    #[doc="Sets the EMPTYEO0 field."]
    #[inline] pub fn set_emptyeo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data Buffer Empty Event Output DAC 1"]
    #[inline] pub fn emptyeo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EMPTYEO1 != 0"]
    #[inline] pub fn test_emptyeo1(&self) -> bool {
        self.emptyeo1() != 0
    }

    #[doc="Sets the EMPTYEO1 field."]
    #[inline] pub fn set_emptyeo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enable Invertion of DAC 0 input event"]
    #[inline] pub fn invei0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if INVEI0 != 0"]
    #[inline] pub fn test_invei0(&self) -> bool {
        self.invei0() != 0
    }

    #[doc="Sets the INVEI0 field."]
    #[inline] pub fn set_invei0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enable Invertion of DAC 1 input event"]
    #[inline] pub fn invei1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if INVEI1 != 0"]
    #[inline] pub fn test_invei1(&self) -> bool {
        self.invei1() != 0
    }

    #[doc="Sets the INVEI1 field."]
    #[inline] pub fn set_invei1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Result Ready Event Output 0"]
    #[inline] pub fn resrdyeo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RESRDYEO0 != 0"]
    #[inline] pub fn test_resrdyeo0(&self) -> bool {
        self.resrdyeo0() != 0
    }

    #[doc="Sets the RESRDYEO0 field."]
    #[inline] pub fn set_resrdyeo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Result Ready Event Output 1"]
    #[inline] pub fn resrdyeo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RESRDYEO1 != 0"]
    #[inline] pub fn test_resrdyeo1(&self) -> bool {
        self.resrdyeo1() != 0
    }

    #[doc="Sets the RESRDYEO1 field."]
    #[inline] pub fn set_resrdyeo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Evctrl {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.startei0() != 0 { try!(write!(f, " startei0"))}
        if self.startei1() != 0 { try!(write!(f, " startei1"))}
        if self.emptyeo0() != 0 { try!(write!(f, " emptyeo0"))}
        if self.emptyeo1() != 0 { try!(write!(f, " emptyeo1"))}
        if self.invei0() != 0 { try!(write!(f, " invei0"))}
        if self.invei1() != 0 { try!(write!(f, " invei1"))}
        if self.resrdyeo0() != 0 { try!(write!(f, " resrdyeo0"))}
        if self.resrdyeo1() != 0 { try!(write!(f, " resrdyeo1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Underrun 0 Interrupt Enable"]
    #[inline] pub fn underrun0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UNDERRUN0 != 0"]
    #[inline] pub fn test_underrun0(&self) -> bool {
        self.underrun0() != 0
    }

    #[doc="Sets the UNDERRUN0 field."]
    #[inline] pub fn set_underrun0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Underrun 1 Interrupt Enable"]
    #[inline] pub fn underrun1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if UNDERRUN1 != 0"]
    #[inline] pub fn test_underrun1(&self) -> bool {
        self.underrun1() != 0
    }

    #[doc="Sets the UNDERRUN1 field."]
    #[inline] pub fn set_underrun1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Buffer 0 Empty Interrupt Enable"]
    #[inline] pub fn empty0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EMPTY0 != 0"]
    #[inline] pub fn test_empty0(&self) -> bool {
        self.empty0() != 0
    }

    #[doc="Sets the EMPTY0 field."]
    #[inline] pub fn set_empty0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data Buffer 1 Empty Interrupt Enable"]
    #[inline] pub fn empty1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EMPTY1 != 0"]
    #[inline] pub fn test_empty1(&self) -> bool {
        self.empty1() != 0
    }

    #[doc="Sets the EMPTY1 field."]
    #[inline] pub fn set_empty1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Result 0 Ready Interrupt Enable"]
    #[inline] pub fn resrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RESRDY0 != 0"]
    #[inline] pub fn test_resrdy0(&self) -> bool {
        self.resrdy0() != 0
    }

    #[doc="Sets the RESRDY0 field."]
    #[inline] pub fn set_resrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Result 1 Ready Interrupt Enable"]
    #[inline] pub fn resrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RESRDY1 != 0"]
    #[inline] pub fn test_resrdy1(&self) -> bool {
        self.resrdy1() != 0
    }

    #[doc="Sets the RESRDY1 field."]
    #[inline] pub fn set_resrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Overrun 0 Interrupt Enable"]
    #[inline] pub fn overrun0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if OVERRUN0 != 0"]
    #[inline] pub fn test_overrun0(&self) -> bool {
        self.overrun0() != 0
    }

    #[doc="Sets the OVERRUN0 field."]
    #[inline] pub fn set_overrun0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overrun 1 Interrupt Enable"]
    #[inline] pub fn overrun1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVERRUN1 != 0"]
    #[inline] pub fn test_overrun1(&self) -> bool {
        self.overrun1() != 0
    }

    #[doc="Sets the OVERRUN1 field."]
    #[inline] pub fn set_overrun1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.underrun0() != 0 { try!(write!(f, " underrun0"))}
        if self.underrun1() != 0 { try!(write!(f, " underrun1"))}
        if self.empty0() != 0 { try!(write!(f, " empty0"))}
        if self.empty1() != 0 { try!(write!(f, " empty1"))}
        if self.resrdy0() != 0 { try!(write!(f, " resrdy0"))}
        if self.resrdy1() != 0 { try!(write!(f, " resrdy1"))}
        if self.overrun0() != 0 { try!(write!(f, " overrun0"))}
        if self.overrun1() != 0 { try!(write!(f, " overrun1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Underrun 0 Interrupt Enable"]
    #[inline] pub fn underrun0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UNDERRUN0 != 0"]
    #[inline] pub fn test_underrun0(&self) -> bool {
        self.underrun0() != 0
    }

    #[doc="Sets the UNDERRUN0 field."]
    #[inline] pub fn set_underrun0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Underrun 1 Interrupt Enable"]
    #[inline] pub fn underrun1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if UNDERRUN1 != 0"]
    #[inline] pub fn test_underrun1(&self) -> bool {
        self.underrun1() != 0
    }

    #[doc="Sets the UNDERRUN1 field."]
    #[inline] pub fn set_underrun1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Buffer 0 Empty Interrupt Enable"]
    #[inline] pub fn empty0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EMPTY0 != 0"]
    #[inline] pub fn test_empty0(&self) -> bool {
        self.empty0() != 0
    }

    #[doc="Sets the EMPTY0 field."]
    #[inline] pub fn set_empty0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data Buffer 1 Empty Interrupt Enable"]
    #[inline] pub fn empty1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EMPTY1 != 0"]
    #[inline] pub fn test_empty1(&self) -> bool {
        self.empty1() != 0
    }

    #[doc="Sets the EMPTY1 field."]
    #[inline] pub fn set_empty1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Result 0 Ready Interrupt Enable"]
    #[inline] pub fn resrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RESRDY0 != 0"]
    #[inline] pub fn test_resrdy0(&self) -> bool {
        self.resrdy0() != 0
    }

    #[doc="Sets the RESRDY0 field."]
    #[inline] pub fn set_resrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Result 1 Ready Interrupt Enable"]
    #[inline] pub fn resrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RESRDY1 != 0"]
    #[inline] pub fn test_resrdy1(&self) -> bool {
        self.resrdy1() != 0
    }

    #[doc="Sets the RESRDY1 field."]
    #[inline] pub fn set_resrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Overrun 0 Interrupt Enable"]
    #[inline] pub fn overrun0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if OVERRUN0 != 0"]
    #[inline] pub fn test_overrun0(&self) -> bool {
        self.overrun0() != 0
    }

    #[doc="Sets the OVERRUN0 field."]
    #[inline] pub fn set_overrun0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overrun 1 Interrupt Enable"]
    #[inline] pub fn overrun1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVERRUN1 != 0"]
    #[inline] pub fn test_overrun1(&self) -> bool {
        self.overrun1() != 0
    }

    #[doc="Sets the OVERRUN1 field."]
    #[inline] pub fn set_overrun1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.underrun0() != 0 { try!(write!(f, " underrun0"))}
        if self.underrun1() != 0 { try!(write!(f, " underrun1"))}
        if self.empty0() != 0 { try!(write!(f, " empty0"))}
        if self.empty1() != 0 { try!(write!(f, " empty1"))}
        if self.resrdy0() != 0 { try!(write!(f, " resrdy0"))}
        if self.resrdy1() != 0 { try!(write!(f, " resrdy1"))}
        if self.overrun0() != 0 { try!(write!(f, " overrun0"))}
        if self.overrun1() != 0 { try!(write!(f, " overrun1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Result 0 Underrun"]
    #[inline] pub fn underrun0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UNDERRUN0 != 0"]
    #[inline] pub fn test_underrun0(&self) -> bool {
        self.underrun0() != 0
    }

    #[doc="Sets the UNDERRUN0 field."]
    #[inline] pub fn set_underrun0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Result 1 Underrun"]
    #[inline] pub fn underrun1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if UNDERRUN1 != 0"]
    #[inline] pub fn test_underrun1(&self) -> bool {
        self.underrun1() != 0
    }

    #[doc="Sets the UNDERRUN1 field."]
    #[inline] pub fn set_underrun1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Buffer 0 Empty"]
    #[inline] pub fn empty0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EMPTY0 != 0"]
    #[inline] pub fn test_empty0(&self) -> bool {
        self.empty0() != 0
    }

    #[doc="Sets the EMPTY0 field."]
    #[inline] pub fn set_empty0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data Buffer 1 Empty"]
    #[inline] pub fn empty1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EMPTY1 != 0"]
    #[inline] pub fn test_empty1(&self) -> bool {
        self.empty1() != 0
    }

    #[doc="Sets the EMPTY1 field."]
    #[inline] pub fn set_empty1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Result 0 Ready"]
    #[inline] pub fn resrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RESRDY0 != 0"]
    #[inline] pub fn test_resrdy0(&self) -> bool {
        self.resrdy0() != 0
    }

    #[doc="Sets the RESRDY0 field."]
    #[inline] pub fn set_resrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Result 1 Ready"]
    #[inline] pub fn resrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RESRDY1 != 0"]
    #[inline] pub fn test_resrdy1(&self) -> bool {
        self.resrdy1() != 0
    }

    #[doc="Sets the RESRDY1 field."]
    #[inline] pub fn set_resrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Result 0 Overrun"]
    #[inline] pub fn overrun0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if OVERRUN0 != 0"]
    #[inline] pub fn test_overrun0(&self) -> bool {
        self.overrun0() != 0
    }

    #[doc="Sets the OVERRUN0 field."]
    #[inline] pub fn set_overrun0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Result 1 Overrun"]
    #[inline] pub fn overrun1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OVERRUN1 != 0"]
    #[inline] pub fn test_overrun1(&self) -> bool {
        self.overrun1() != 0
    }

    #[doc="Sets the OVERRUN1 field."]
    #[inline] pub fn set_overrun1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.underrun0() != 0 { try!(write!(f, " underrun0"))}
        if self.underrun1() != 0 { try!(write!(f, " underrun1"))}
        if self.empty0() != 0 { try!(write!(f, " empty0"))}
        if self.empty1() != 0 { try!(write!(f, " empty1"))}
        if self.resrdy0() != 0 { try!(write!(f, " resrdy0"))}
        if self.resrdy1() != 0 { try!(write!(f, " resrdy1"))}
        if self.overrun0() != 0 { try!(write!(f, " overrun0"))}
        if self.overrun1() != 0 { try!(write!(f, " overrun1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="DAC 0 Startup Ready"]
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

    #[doc="DAC 1 Startup Ready"]
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

    #[doc="DAC 0 End of Conversion"]
    #[inline] pub fn eoc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EOC0 != 0"]
    #[inline] pub fn test_eoc0(&self) -> bool {
        self.eoc0() != 0
    }

    #[doc="Sets the EOC0 field."]
    #[inline] pub fn set_eoc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DAC 1 End of Conversion"]
    #[inline] pub fn eoc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EOC1 != 0"]
    #[inline] pub fn test_eoc1(&self) -> bool {
        self.eoc1() != 0
    }

    #[doc="Sets the EOC1 field."]
    #[inline] pub fn set_eoc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.ready0() != 0 { try!(write!(f, " ready0"))}
        if self.ready1() != 0 { try!(write!(f, " ready1"))}
        if self.eoc0() != 0 { try!(write!(f, " eoc0"))}
        if self.eoc1() != 0 { try!(write!(f, " eoc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
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

    #[doc="DAC Enable Status"]
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

    #[doc="Data DAC 0"]
    #[inline] pub fn data0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data DAC 1"]
    #[inline] pub fn data1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data Buffer DAC 0"]
    #[inline] pub fn databuf0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DATABUF0 != 0"]
    #[inline] pub fn test_databuf0(&self) -> bool {
        self.databuf0() != 0
    }

    #[doc="Sets the DATABUF0 field."]
    #[inline] pub fn set_databuf0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Buffer DAC 1"]
    #[inline] pub fn databuf1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DATABUF1 != 0"]
    #[inline] pub fn test_databuf1(&self) -> bool {
        self.databuf1() != 0
    }

    #[doc="Sets the DATABUF1 field."]
    #[inline] pub fn set_databuf1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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
        if self.data0() != 0 { try!(write!(f, " data0"))}
        if self.data1() != 0 { try!(write!(f, " data1"))}
        if self.databuf0() != 0 { try!(write!(f, " databuf0"))}
        if self.databuf1() != 0 { try!(write!(f, " databuf1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DAC n Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dacctrl(pub u16);
impl Dacctrl {
    #[doc="Left Adjusted Data"]
    #[inline] pub fn leftadj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LEFTADJ != 0"]
    #[inline] pub fn test_leftadj(&self) -> bool {
        self.leftadj() != 0
    }

    #[doc="Sets the LEFTADJ field."]
    #[inline] pub fn set_leftadj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable DAC0"]
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

    #[doc="Current Control"]
    #[inline] pub fn cctrl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if CCTRL != 0"]
    #[inline] pub fn test_cctrl(&self) -> bool {
        self.cctrl() != 0
    }

    #[doc="Sets the CCTRL field."]
    #[inline] pub fn set_cctrl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Standalone Filter"]
    #[inline] pub fn fext(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FEXT != 0"]
    #[inline] pub fn test_fext(&self) -> bool {
        self.fext() != 0
    }

    #[doc="Sets the FEXT field."]
    #[inline] pub fn set_fext<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Dithering Mode"]
    #[inline] pub fn dither(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DITHER != 0"]
    #[inline] pub fn test_dither(&self) -> bool {
        self.dither() != 0
    }

    #[doc="Sets the DITHER field."]
    #[inline] pub fn set_dither<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Refresh period"]
    #[inline] pub fn refresh(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if REFRESH != 0"]
    #[inline] pub fn test_refresh(&self) -> bool {
        self.refresh() != 0
    }

    #[doc="Sets the REFRESH field."]
    #[inline] pub fn set_refresh<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Sampling Rate"]
    #[inline] pub fn osr(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if OSR != 0"]
    #[inline] pub fn test_osr(&self) -> bool {
        self.osr() != 0
    }

    #[doc="Sets the OSR field."]
    #[inline] pub fn set_osr<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Dacctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Dacctrl(other)
    }
}

impl ::core::fmt::Display for Dacctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dacctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.leftadj() != 0 { try!(write!(f, " leftadj"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.cctrl() != 0 { try!(write!(f, " cctrl=0x{:x}", self.cctrl()))}
        if self.fext() != 0 { try!(write!(f, " fext"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.dither() != 0 { try!(write!(f, " dither"))}
        if self.refresh() != 0 { try!(write!(f, " refresh=0x{:x}", self.refresh()))}
        if self.osr() != 0 { try!(write!(f, " osr=0x{:x}", self.osr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DAC n Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u16);
impl Data {
    #[doc="DAC0 Data"]
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
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Data {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DAC n Data Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Databuf(pub u16);
impl Databuf {
    #[doc="DAC0 Data Buffer"]
    #[inline] pub fn databuf(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATABUF != 0"]
    #[inline] pub fn test_databuf(&self) -> bool {
        self.databuf() != 0
    }

    #[doc="Sets the DATABUF field."]
    #[inline] pub fn set_databuf<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Databuf {
    #[inline]
    fn from(other: u16) -> Self {
         Databuf(other)
    }
}

impl ::core::fmt::Display for Databuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Databuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.databuf() != 0 { try!(write!(f, " databuf=0x{:x}", self.databuf()))}
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

#[doc="Filter Result"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Result(pub u16);
impl Result {
    #[doc="Filter Result"]
    #[inline] pub fn result(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RESULT != 0"]
    #[inline] pub fn test_result(&self) -> bool {
        self.result() != 0
    }

    #[doc="Sets the RESULT field."]
    #[inline] pub fn set_result<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Result {
    #[inline]
    fn from(other: u16) -> Self {
         Result(other)
    }
}

impl ::core::fmt::Display for Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.result() != 0 { try!(write!(f, " result=0x{:x}", self.result()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

