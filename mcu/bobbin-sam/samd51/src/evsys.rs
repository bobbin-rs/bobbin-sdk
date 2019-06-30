::bobbin_mcu::periph!( EVSYS, Evsys, EVSYS_PERIPH, EvsysPeriph, EVSYS_OWNED, EVSYS_REF_COUNT, 0x4100e000, 0x00, 0x0b);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="EVSYS Peripheral"]
pub struct EvsysPeriph(pub usize); 

impl EvsysPeriph {
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

    #[doc="Get the SWEVT Register."]
    #[inline] pub fn swevt_reg(&self) -> ::bobbin_mcu::register::Register<Swevt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swevt, 0x4)
    }

    #[doc="Get the *mut pointer for the SWEVT register."]
    #[inline] pub fn swevt_mut(&self) -> *mut Swevt { 
        self.swevt_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWEVT register."]
    #[inline] pub fn swevt_ptr(&self) -> *const Swevt { 
        self.swevt_reg().ptr()
    }

    #[doc="Write the SWEVT register."]
    #[inline] pub fn write_swevt(&self, value: Swevt) -> &Self { 
        self.swevt_reg().write(value);
        self
    }

    #[doc="Set the SWEVT register."]
    #[inline] pub fn set_swevt<F: FnOnce(Swevt) -> Swevt>(&self, f: F) -> &Self {
        self.swevt_reg().set(f);
        self
    }

    #[doc="Get the PRICTRL Register."]
    #[inline] pub fn prictrl_reg(&self) -> ::bobbin_mcu::register::Register<Prictrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Prictrl, 0x8)
    }

    #[doc="Get the *mut pointer for the PRICTRL register."]
    #[inline] pub fn prictrl_mut(&self) -> *mut Prictrl { 
        self.prictrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the PRICTRL register."]
    #[inline] pub fn prictrl_ptr(&self) -> *const Prictrl { 
        self.prictrl_reg().ptr()
    }

    #[doc="Read the PRICTRL register."]
    #[inline] pub fn prictrl(&self) -> Prictrl { 
        self.prictrl_reg().read()
    }

    #[doc="Write the PRICTRL register."]
    #[inline] pub fn write_prictrl(&self, value: Prictrl) -> &Self { 
        self.prictrl_reg().write(value);
        self
    }

    #[doc="Set the PRICTRL register."]
    #[inline] pub fn set_prictrl<F: FnOnce(Prictrl) -> Prictrl>(&self, f: F) -> &Self {
        self.prictrl_reg().set(f);
        self
    }

    #[doc="Modify the PRICTRL register."]
    #[inline] pub fn with_prictrl<F: FnOnce(Prictrl) -> Prictrl>(&self, f: F) -> &Self {
        self.prictrl_reg().with(f);
        self
    }

    #[doc="Get the INTPEND Register."]
    #[inline] pub fn intpend_reg(&self) -> ::bobbin_mcu::register::Register<Intpend> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intpend, 0x10)
    }

    #[doc="Get the *mut pointer for the INTPEND register."]
    #[inline] pub fn intpend_mut(&self) -> *mut Intpend { 
        self.intpend_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTPEND register."]
    #[inline] pub fn intpend_ptr(&self) -> *const Intpend { 
        self.intpend_reg().ptr()
    }

    #[doc="Read the INTPEND register."]
    #[inline] pub fn intpend(&self) -> Intpend { 
        self.intpend_reg().read()
    }

    #[doc="Write the INTPEND register."]
    #[inline] pub fn write_intpend(&self, value: Intpend) -> &Self { 
        self.intpend_reg().write(value);
        self
    }

    #[doc="Set the INTPEND register."]
    #[inline] pub fn set_intpend<F: FnOnce(Intpend) -> Intpend>(&self, f: F) -> &Self {
        self.intpend_reg().set(f);
        self
    }

    #[doc="Modify the INTPEND register."]
    #[inline] pub fn with_intpend<F: FnOnce(Intpend) -> Intpend>(&self, f: F) -> &Self {
        self.intpend_reg().with(f);
        self
    }

    #[doc="Get the INTSTATUS Register."]
    #[inline] pub fn intstatus_reg(&self) -> ::bobbin_mcu::register::Register<Intstatus> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intstatus, 0x14)
    }

    #[doc="Get the *mut pointer for the INTSTATUS register."]
    #[inline] pub fn intstatus_mut(&self) -> *mut Intstatus { 
        self.intstatus_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTSTATUS register."]
    #[inline] pub fn intstatus_ptr(&self) -> *const Intstatus { 
        self.intstatus_reg().ptr()
    }

    #[doc="Read the INTSTATUS register."]
    #[inline] pub fn intstatus(&self) -> Intstatus { 
        self.intstatus_reg().read()
    }

    #[doc="Get the BUSYCH Register."]
    #[inline] pub fn busych_reg(&self) -> ::bobbin_mcu::register::Register<Busych> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Busych, 0x18)
    }

    #[doc="Get the *mut pointer for the BUSYCH register."]
    #[inline] pub fn busych_mut(&self) -> *mut Busych { 
        self.busych_reg().ptr()
    }

    #[doc="Get the *const pointer for the BUSYCH register."]
    #[inline] pub fn busych_ptr(&self) -> *const Busych { 
        self.busych_reg().ptr()
    }

    #[doc="Read the BUSYCH register."]
    #[inline] pub fn busych(&self) -> Busych { 
        self.busych_reg().read()
    }

    #[doc="Get the READYUSR Register."]
    #[inline] pub fn readyusr_reg(&self) -> ::bobbin_mcu::register::Register<Readyusr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Readyusr, 0x1c)
    }

    #[doc="Get the *mut pointer for the READYUSR register."]
    #[inline] pub fn readyusr_mut(&self) -> *mut Readyusr { 
        self.readyusr_reg().ptr()
    }

    #[doc="Get the *const pointer for the READYUSR register."]
    #[inline] pub fn readyusr_ptr(&self) -> *const Readyusr { 
        self.readyusr_reg().ptr()
    }

    #[doc="Read the READYUSR register."]
    #[inline] pub fn readyusr(&self) -> Readyusr { 
        self.readyusr_reg().read()
    }

    #[doc="Get the CHANNEL Register."]
    #[inline] pub fn channel_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Channel, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Channel, 0x20, 0x8)
    }

    #[doc="Get the *mut pointer for the CHANNEL register."]
    #[inline] pub fn channel_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Channel { 
        self.channel_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHANNEL register."]
    #[inline] pub fn channel_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Channel { 
        self.channel_reg().ptr(index.into())
    }

    #[doc="Read the CHANNEL register."]
    #[inline] pub fn channel<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Channel { 
        self.channel_reg().read(index.into())
    }

    #[doc="Write the CHANNEL register."]
    #[inline] pub fn write_channel<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Channel) -> &Self {
        self.channel_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHANNEL register."]
    #[inline] pub fn set_channel<I: Into<::bobbin_bits::R32>, F: FnOnce(Channel) -> Channel>(&self, index: I, f: F) -> &Self {
        self.channel_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHANNEL register."]
    #[inline] pub fn with_channel<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Channel) -> Channel>(&self, index: I, f: F) -> &Self {
        self.channel_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHINTENCLR Register."]
    #[inline] pub fn chintenclr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chintenclr, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chintenclr, 0x24, 0x8)
    }

    #[doc="Get the *mut pointer for the CHINTENCLR register."]
    #[inline] pub fn chintenclr_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chintenclr { 
        self.chintenclr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHINTENCLR register."]
    #[inline] pub fn chintenclr_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chintenclr { 
        self.chintenclr_reg().ptr(index.into())
    }

    #[doc="Read the CHINTENCLR register."]
    #[inline] pub fn chintenclr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chintenclr { 
        self.chintenclr_reg().read(index.into())
    }

    #[doc="Write the CHINTENCLR register."]
    #[inline] pub fn write_chintenclr<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chintenclr) -> &Self {
        self.chintenclr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHINTENCLR register."]
    #[inline] pub fn set_chintenclr<I: Into<::bobbin_bits::R32>, F: FnOnce(Chintenclr) -> Chintenclr>(&self, index: I, f: F) -> &Self {
        self.chintenclr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHINTENCLR register."]
    #[inline] pub fn with_chintenclr<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chintenclr) -> Chintenclr>(&self, index: I, f: F) -> &Self {
        self.chintenclr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHINTENSET Register."]
    #[inline] pub fn chintenset_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chintenset, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chintenset, 0x25, 0x8)
    }

    #[doc="Get the *mut pointer for the CHINTENSET register."]
    #[inline] pub fn chintenset_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chintenset { 
        self.chintenset_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHINTENSET register."]
    #[inline] pub fn chintenset_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chintenset { 
        self.chintenset_reg().ptr(index.into())
    }

    #[doc="Read the CHINTENSET register."]
    #[inline] pub fn chintenset<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chintenset { 
        self.chintenset_reg().read(index.into())
    }

    #[doc="Write the CHINTENSET register."]
    #[inline] pub fn write_chintenset<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chintenset) -> &Self {
        self.chintenset_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHINTENSET register."]
    #[inline] pub fn set_chintenset<I: Into<::bobbin_bits::R32>, F: FnOnce(Chintenset) -> Chintenset>(&self, index: I, f: F) -> &Self {
        self.chintenset_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHINTENSET register."]
    #[inline] pub fn with_chintenset<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chintenset) -> Chintenset>(&self, index: I, f: F) -> &Self {
        self.chintenset_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHINTFLAG Register."]
    #[inline] pub fn chintflag_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chintflag, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chintflag, 0x26, 0x8)
    }

    #[doc="Get the *mut pointer for the CHINTFLAG register."]
    #[inline] pub fn chintflag_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chintflag { 
        self.chintflag_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHINTFLAG register."]
    #[inline] pub fn chintflag_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chintflag { 
        self.chintflag_reg().ptr(index.into())
    }

    #[doc="Read the CHINTFLAG register."]
    #[inline] pub fn chintflag<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chintflag { 
        self.chintflag_reg().read(index.into())
    }

    #[doc="Write the CHINTFLAG register."]
    #[inline] pub fn write_chintflag<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chintflag) -> &Self {
        self.chintflag_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHINTFLAG register."]
    #[inline] pub fn set_chintflag<I: Into<::bobbin_bits::R32>, F: FnOnce(Chintflag) -> Chintflag>(&self, index: I, f: F) -> &Self {
        self.chintflag_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHINTFLAG register."]
    #[inline] pub fn with_chintflag<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chintflag) -> Chintflag>(&self, index: I, f: F) -> &Self {
        self.chintflag_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHSTATUS Register."]
    #[inline] pub fn chstatus_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chstatus, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chstatus, 0x27, 0x8)
    }

    #[doc="Get the *mut pointer for the CHSTATUS register."]
    #[inline] pub fn chstatus_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chstatus { 
        self.chstatus_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHSTATUS register."]
    #[inline] pub fn chstatus_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chstatus { 
        self.chstatus_reg().ptr(index.into())
    }

    #[doc="Read the CHSTATUS register."]
    #[inline] pub fn chstatus<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chstatus { 
        self.chstatus_reg().read(index.into())
    }

}

#[doc="Control"]
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software Event"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swevt(pub u32);
impl Swevt {
    #[doc="Channel 0 Software Selection"]
    #[inline] pub fn channel0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHANNEL0 != 0"]
    #[inline] pub fn test_channel0(&self) -> bool {
        self.channel0() != 0
    }

    #[doc="Sets the CHANNEL0 field."]
    #[inline] pub fn set_channel0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Software Selection"]
    #[inline] pub fn channel1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHANNEL1 != 0"]
    #[inline] pub fn test_channel1(&self) -> bool {
        self.channel1() != 0
    }

    #[doc="Sets the CHANNEL1 field."]
    #[inline] pub fn set_channel1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Software Selection"]
    #[inline] pub fn channel2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CHANNEL2 != 0"]
    #[inline] pub fn test_channel2(&self) -> bool {
        self.channel2() != 0
    }

    #[doc="Sets the CHANNEL2 field."]
    #[inline] pub fn set_channel2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Software Selection"]
    #[inline] pub fn channel3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CHANNEL3 != 0"]
    #[inline] pub fn test_channel3(&self) -> bool {
        self.channel3() != 0
    }

    #[doc="Sets the CHANNEL3 field."]
    #[inline] pub fn set_channel3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Software Selection"]
    #[inline] pub fn channel4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CHANNEL4 != 0"]
    #[inline] pub fn test_channel4(&self) -> bool {
        self.channel4() != 0
    }

    #[doc="Sets the CHANNEL4 field."]
    #[inline] pub fn set_channel4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Software Selection"]
    #[inline] pub fn channel5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CHANNEL5 != 0"]
    #[inline] pub fn test_channel5(&self) -> bool {
        self.channel5() != 0
    }

    #[doc="Sets the CHANNEL5 field."]
    #[inline] pub fn set_channel5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Software Selection"]
    #[inline] pub fn channel6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CHANNEL6 != 0"]
    #[inline] pub fn test_channel6(&self) -> bool {
        self.channel6() != 0
    }

    #[doc="Sets the CHANNEL6 field."]
    #[inline] pub fn set_channel6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Software Selection"]
    #[inline] pub fn channel7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHANNEL7 != 0"]
    #[inline] pub fn test_channel7(&self) -> bool {
        self.channel7() != 0
    }

    #[doc="Sets the CHANNEL7 field."]
    #[inline] pub fn set_channel7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Channel 8 Software Selection"]
    #[inline] pub fn channel8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHANNEL8 != 0"]
    #[inline] pub fn test_channel8(&self) -> bool {
        self.channel8() != 0
    }

    #[doc="Sets the CHANNEL8 field."]
    #[inline] pub fn set_channel8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Channel 9 Software Selection"]
    #[inline] pub fn channel9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CHANNEL9 != 0"]
    #[inline] pub fn test_channel9(&self) -> bool {
        self.channel9() != 0
    }

    #[doc="Sets the CHANNEL9 field."]
    #[inline] pub fn set_channel9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Channel 10 Software Selection"]
    #[inline] pub fn channel10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CHANNEL10 != 0"]
    #[inline] pub fn test_channel10(&self) -> bool {
        self.channel10() != 0
    }

    #[doc="Sets the CHANNEL10 field."]
    #[inline] pub fn set_channel10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Channel 11 Software Selection"]
    #[inline] pub fn channel11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CHANNEL11 != 0"]
    #[inline] pub fn test_channel11(&self) -> bool {
        self.channel11() != 0
    }

    #[doc="Sets the CHANNEL11 field."]
    #[inline] pub fn set_channel11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Channel 12 Software Selection"]
    #[inline] pub fn channel12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CHANNEL12 != 0"]
    #[inline] pub fn test_channel12(&self) -> bool {
        self.channel12() != 0
    }

    #[doc="Sets the CHANNEL12 field."]
    #[inline] pub fn set_channel12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Channel 13 Software Selection"]
    #[inline] pub fn channel13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CHANNEL13 != 0"]
    #[inline] pub fn test_channel13(&self) -> bool {
        self.channel13() != 0
    }

    #[doc="Sets the CHANNEL13 field."]
    #[inline] pub fn set_channel13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Channel 14 Software Selection"]
    #[inline] pub fn channel14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CHANNEL14 != 0"]
    #[inline] pub fn test_channel14(&self) -> bool {
        self.channel14() != 0
    }

    #[doc="Sets the CHANNEL14 field."]
    #[inline] pub fn set_channel14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Channel 15 Software Selection"]
    #[inline] pub fn channel15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CHANNEL15 != 0"]
    #[inline] pub fn test_channel15(&self) -> bool {
        self.channel15() != 0
    }

    #[doc="Sets the CHANNEL15 field."]
    #[inline] pub fn set_channel15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Channel 16 Software Selection"]
    #[inline] pub fn channel16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CHANNEL16 != 0"]
    #[inline] pub fn test_channel16(&self) -> bool {
        self.channel16() != 0
    }

    #[doc="Sets the CHANNEL16 field."]
    #[inline] pub fn set_channel16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Channel 17 Software Selection"]
    #[inline] pub fn channel17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CHANNEL17 != 0"]
    #[inline] pub fn test_channel17(&self) -> bool {
        self.channel17() != 0
    }

    #[doc="Sets the CHANNEL17 field."]
    #[inline] pub fn set_channel17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Channel 18 Software Selection"]
    #[inline] pub fn channel18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CHANNEL18 != 0"]
    #[inline] pub fn test_channel18(&self) -> bool {
        self.channel18() != 0
    }

    #[doc="Sets the CHANNEL18 field."]
    #[inline] pub fn set_channel18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Channel 19 Software Selection"]
    #[inline] pub fn channel19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CHANNEL19 != 0"]
    #[inline] pub fn test_channel19(&self) -> bool {
        self.channel19() != 0
    }

    #[doc="Sets the CHANNEL19 field."]
    #[inline] pub fn set_channel19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Channel 20 Software Selection"]
    #[inline] pub fn channel20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CHANNEL20 != 0"]
    #[inline] pub fn test_channel20(&self) -> bool {
        self.channel20() != 0
    }

    #[doc="Sets the CHANNEL20 field."]
    #[inline] pub fn set_channel20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Channel 21 Software Selection"]
    #[inline] pub fn channel21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CHANNEL21 != 0"]
    #[inline] pub fn test_channel21(&self) -> bool {
        self.channel21() != 0
    }

    #[doc="Sets the CHANNEL21 field."]
    #[inline] pub fn set_channel21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Channel 22 Software Selection"]
    #[inline] pub fn channel22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if CHANNEL22 != 0"]
    #[inline] pub fn test_channel22(&self) -> bool {
        self.channel22() != 0
    }

    #[doc="Sets the CHANNEL22 field."]
    #[inline] pub fn set_channel22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Channel 23 Software Selection"]
    #[inline] pub fn channel23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if CHANNEL23 != 0"]
    #[inline] pub fn test_channel23(&self) -> bool {
        self.channel23() != 0
    }

    #[doc="Sets the CHANNEL23 field."]
    #[inline] pub fn set_channel23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Channel 24 Software Selection"]
    #[inline] pub fn channel24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CHANNEL24 != 0"]
    #[inline] pub fn test_channel24(&self) -> bool {
        self.channel24() != 0
    }

    #[doc="Sets the CHANNEL24 field."]
    #[inline] pub fn set_channel24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Channel 25 Software Selection"]
    #[inline] pub fn channel25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CHANNEL25 != 0"]
    #[inline] pub fn test_channel25(&self) -> bool {
        self.channel25() != 0
    }

    #[doc="Sets the CHANNEL25 field."]
    #[inline] pub fn set_channel25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Channel 26 Software Selection"]
    #[inline] pub fn channel26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CHANNEL26 != 0"]
    #[inline] pub fn test_channel26(&self) -> bool {
        self.channel26() != 0
    }

    #[doc="Sets the CHANNEL26 field."]
    #[inline] pub fn set_channel26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Channel 27 Software Selection"]
    #[inline] pub fn channel27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if CHANNEL27 != 0"]
    #[inline] pub fn test_channel27(&self) -> bool {
        self.channel27() != 0
    }

    #[doc="Sets the CHANNEL27 field."]
    #[inline] pub fn set_channel27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Channel 28 Software Selection"]
    #[inline] pub fn channel28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CHANNEL28 != 0"]
    #[inline] pub fn test_channel28(&self) -> bool {
        self.channel28() != 0
    }

    #[doc="Sets the CHANNEL28 field."]
    #[inline] pub fn set_channel28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Channel 29 Software Selection"]
    #[inline] pub fn channel29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if CHANNEL29 != 0"]
    #[inline] pub fn test_channel29(&self) -> bool {
        self.channel29() != 0
    }

    #[doc="Sets the CHANNEL29 field."]
    #[inline] pub fn set_channel29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel 30 Software Selection"]
    #[inline] pub fn channel30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHANNEL30 != 0"]
    #[inline] pub fn test_channel30(&self) -> bool {
        self.channel30() != 0
    }

    #[doc="Sets the CHANNEL30 field."]
    #[inline] pub fn set_channel30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel 31 Software Selection"]
    #[inline] pub fn channel31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHANNEL31 != 0"]
    #[inline] pub fn test_channel31(&self) -> bool {
        self.channel31() != 0
    }

    #[doc="Sets the CHANNEL31 field."]
    #[inline] pub fn set_channel31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Swevt {
    #[inline]
    fn from(other: u32) -> Self {
         Swevt(other)
    }
}

impl ::core::fmt::Display for Swevt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swevt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.channel0() != 0 { try!(write!(f, " channel0"))}
        if self.channel1() != 0 { try!(write!(f, " channel1"))}
        if self.channel2() != 0 { try!(write!(f, " channel2"))}
        if self.channel3() != 0 { try!(write!(f, " channel3"))}
        if self.channel4() != 0 { try!(write!(f, " channel4"))}
        if self.channel5() != 0 { try!(write!(f, " channel5"))}
        if self.channel6() != 0 { try!(write!(f, " channel6"))}
        if self.channel7() != 0 { try!(write!(f, " channel7"))}
        if self.channel8() != 0 { try!(write!(f, " channel8"))}
        if self.channel9() != 0 { try!(write!(f, " channel9"))}
        if self.channel10() != 0 { try!(write!(f, " channel10"))}
        if self.channel11() != 0 { try!(write!(f, " channel11"))}
        if self.channel12() != 0 { try!(write!(f, " channel12"))}
        if self.channel13() != 0 { try!(write!(f, " channel13"))}
        if self.channel14() != 0 { try!(write!(f, " channel14"))}
        if self.channel15() != 0 { try!(write!(f, " channel15"))}
        if self.channel16() != 0 { try!(write!(f, " channel16"))}
        if self.channel17() != 0 { try!(write!(f, " channel17"))}
        if self.channel18() != 0 { try!(write!(f, " channel18"))}
        if self.channel19() != 0 { try!(write!(f, " channel19"))}
        if self.channel20() != 0 { try!(write!(f, " channel20"))}
        if self.channel21() != 0 { try!(write!(f, " channel21"))}
        if self.channel22() != 0 { try!(write!(f, " channel22"))}
        if self.channel23() != 0 { try!(write!(f, " channel23"))}
        if self.channel24() != 0 { try!(write!(f, " channel24"))}
        if self.channel25() != 0 { try!(write!(f, " channel25"))}
        if self.channel26() != 0 { try!(write!(f, " channel26"))}
        if self.channel27() != 0 { try!(write!(f, " channel27"))}
        if self.channel28() != 0 { try!(write!(f, " channel28"))}
        if self.channel29() != 0 { try!(write!(f, " channel29"))}
        if self.channel30() != 0 { try!(write!(f, " channel30"))}
        if self.channel31() != 0 { try!(write!(f, " channel31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Priority Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prictrl(pub u8);
impl Prictrl {
    #[doc="Channel Priority Number"]
    #[inline] pub fn pri(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PRI != 0"]
    #[inline] pub fn test_pri(&self) -> bool {
        self.pri() != 0
    }

    #[doc="Sets the PRI field."]
    #[inline] pub fn set_pri<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Round-Robin Scheduling Enable"]
    #[inline] pub fn rren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RREN != 0"]
    #[inline] pub fn test_rren(&self) -> bool {
        self.rren() != 0
    }

    #[doc="Sets the RREN field."]
    #[inline] pub fn set_rren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Prictrl {
    #[inline]
    fn from(other: u8) -> Self {
         Prictrl(other)
    }
}

impl ::core::fmt::Display for Prictrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prictrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pri() != 0 { try!(write!(f, " pri=0x{:x}", self.pri()))}
        if self.rren() != 0 { try!(write!(f, " rren"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Pending Interrupt"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intpend(pub u16);
impl Intpend {
    #[doc="Channel ID"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Overrun"]
    #[inline] pub fn ovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Channel Event Detected"]
    #[inline] pub fn evd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EVD != 0"]
    #[inline] pub fn test_evd(&self) -> bool {
        self.evd() != 0
    }

    #[doc="Sets the EVD field."]
    #[inline] pub fn set_evd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Ready"]
    #[inline] pub fn ready(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if READY != 0"]
    #[inline] pub fn test_ready(&self) -> bool {
        self.ready() != 0
    }

    #[doc="Sets the READY field."]
    #[inline] pub fn set_ready<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intpend {
    #[inline]
    fn from(other: u16) -> Self {
         Intpend(other)
    }
}

impl ::core::fmt::Display for Intpend {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intpend {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.evd() != 0 { try!(write!(f, " evd"))}
        if self.ready() != 0 { try!(write!(f, " ready"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstatus(pub u32);
impl Intstatus {
    #[doc="Channel 0 Pending Interrupt"]
    #[inline] pub fn chint0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHINT0 != 0"]
    #[inline] pub fn test_chint0(&self) -> bool {
        self.chint0() != 0
    }

    #[doc="Sets the CHINT0 field."]
    #[inline] pub fn set_chint0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Pending Interrupt"]
    #[inline] pub fn chint1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHINT1 != 0"]
    #[inline] pub fn test_chint1(&self) -> bool {
        self.chint1() != 0
    }

    #[doc="Sets the CHINT1 field."]
    #[inline] pub fn set_chint1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Pending Interrupt"]
    #[inline] pub fn chint2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CHINT2 != 0"]
    #[inline] pub fn test_chint2(&self) -> bool {
        self.chint2() != 0
    }

    #[doc="Sets the CHINT2 field."]
    #[inline] pub fn set_chint2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Pending Interrupt"]
    #[inline] pub fn chint3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CHINT3 != 0"]
    #[inline] pub fn test_chint3(&self) -> bool {
        self.chint3() != 0
    }

    #[doc="Sets the CHINT3 field."]
    #[inline] pub fn set_chint3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Pending Interrupt"]
    #[inline] pub fn chint4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CHINT4 != 0"]
    #[inline] pub fn test_chint4(&self) -> bool {
        self.chint4() != 0
    }

    #[doc="Sets the CHINT4 field."]
    #[inline] pub fn set_chint4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Pending Interrupt"]
    #[inline] pub fn chint5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CHINT5 != 0"]
    #[inline] pub fn test_chint5(&self) -> bool {
        self.chint5() != 0
    }

    #[doc="Sets the CHINT5 field."]
    #[inline] pub fn set_chint5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Pending Interrupt"]
    #[inline] pub fn chint6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CHINT6 != 0"]
    #[inline] pub fn test_chint6(&self) -> bool {
        self.chint6() != 0
    }

    #[doc="Sets the CHINT6 field."]
    #[inline] pub fn set_chint6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Pending Interrupt"]
    #[inline] pub fn chint7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHINT7 != 0"]
    #[inline] pub fn test_chint7(&self) -> bool {
        self.chint7() != 0
    }

    #[doc="Sets the CHINT7 field."]
    #[inline] pub fn set_chint7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Channel 8 Pending Interrupt"]
    #[inline] pub fn chint8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINT8 != 0"]
    #[inline] pub fn test_chint8(&self) -> bool {
        self.chint8() != 0
    }

    #[doc="Sets the CHINT8 field."]
    #[inline] pub fn set_chint8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Channel 9 Pending Interrupt"]
    #[inline] pub fn chint9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CHINT9 != 0"]
    #[inline] pub fn test_chint9(&self) -> bool {
        self.chint9() != 0
    }

    #[doc="Sets the CHINT9 field."]
    #[inline] pub fn set_chint9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Channel 10 Pending Interrupt"]
    #[inline] pub fn chint10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CHINT10 != 0"]
    #[inline] pub fn test_chint10(&self) -> bool {
        self.chint10() != 0
    }

    #[doc="Sets the CHINT10 field."]
    #[inline] pub fn set_chint10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Channel 11 Pending Interrupt"]
    #[inline] pub fn chint11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CHINT11 != 0"]
    #[inline] pub fn test_chint11(&self) -> bool {
        self.chint11() != 0
    }

    #[doc="Sets the CHINT11 field."]
    #[inline] pub fn set_chint11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Intstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Intstatus(other)
    }
}

impl ::core::fmt::Display for Intstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chint0() != 0 { try!(write!(f, " chint0"))}
        if self.chint1() != 0 { try!(write!(f, " chint1"))}
        if self.chint2() != 0 { try!(write!(f, " chint2"))}
        if self.chint3() != 0 { try!(write!(f, " chint3"))}
        if self.chint4() != 0 { try!(write!(f, " chint4"))}
        if self.chint5() != 0 { try!(write!(f, " chint5"))}
        if self.chint6() != 0 { try!(write!(f, " chint6"))}
        if self.chint7() != 0 { try!(write!(f, " chint7"))}
        if self.chint8() != 0 { try!(write!(f, " chint8"))}
        if self.chint9() != 0 { try!(write!(f, " chint9"))}
        if self.chint10() != 0 { try!(write!(f, " chint10"))}
        if self.chint11() != 0 { try!(write!(f, " chint11"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Busy Channels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Busych(pub u32);
impl Busych {
    #[doc="Busy Channel 0"]
    #[inline] pub fn busych0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BUSYCH0 != 0"]
    #[inline] pub fn test_busych0(&self) -> bool {
        self.busych0() != 0
    }

    #[doc="Sets the BUSYCH0 field."]
    #[inline] pub fn set_busych0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Busy Channel 1"]
    #[inline] pub fn busych1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BUSYCH1 != 0"]
    #[inline] pub fn test_busych1(&self) -> bool {
        self.busych1() != 0
    }

    #[doc="Sets the BUSYCH1 field."]
    #[inline] pub fn set_busych1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Busy Channel 2"]
    #[inline] pub fn busych2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BUSYCH2 != 0"]
    #[inline] pub fn test_busych2(&self) -> bool {
        self.busych2() != 0
    }

    #[doc="Sets the BUSYCH2 field."]
    #[inline] pub fn set_busych2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Busy Channel 3"]
    #[inline] pub fn busych3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BUSYCH3 != 0"]
    #[inline] pub fn test_busych3(&self) -> bool {
        self.busych3() != 0
    }

    #[doc="Sets the BUSYCH3 field."]
    #[inline] pub fn set_busych3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Busy Channel 4"]
    #[inline] pub fn busych4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BUSYCH4 != 0"]
    #[inline] pub fn test_busych4(&self) -> bool {
        self.busych4() != 0
    }

    #[doc="Sets the BUSYCH4 field."]
    #[inline] pub fn set_busych4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Busy Channel 5"]
    #[inline] pub fn busych5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BUSYCH5 != 0"]
    #[inline] pub fn test_busych5(&self) -> bool {
        self.busych5() != 0
    }

    #[doc="Sets the BUSYCH5 field."]
    #[inline] pub fn set_busych5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Busy Channel 6"]
    #[inline] pub fn busych6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BUSYCH6 != 0"]
    #[inline] pub fn test_busych6(&self) -> bool {
        self.busych6() != 0
    }

    #[doc="Sets the BUSYCH6 field."]
    #[inline] pub fn set_busych6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Busy Channel 7"]
    #[inline] pub fn busych7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BUSYCH7 != 0"]
    #[inline] pub fn test_busych7(&self) -> bool {
        self.busych7() != 0
    }

    #[doc="Sets the BUSYCH7 field."]
    #[inline] pub fn set_busych7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Busy Channel 8"]
    #[inline] pub fn busych8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BUSYCH8 != 0"]
    #[inline] pub fn test_busych8(&self) -> bool {
        self.busych8() != 0
    }

    #[doc="Sets the BUSYCH8 field."]
    #[inline] pub fn set_busych8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Busy Channel 9"]
    #[inline] pub fn busych9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BUSYCH9 != 0"]
    #[inline] pub fn test_busych9(&self) -> bool {
        self.busych9() != 0
    }

    #[doc="Sets the BUSYCH9 field."]
    #[inline] pub fn set_busych9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Busy Channel 10"]
    #[inline] pub fn busych10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BUSYCH10 != 0"]
    #[inline] pub fn test_busych10(&self) -> bool {
        self.busych10() != 0
    }

    #[doc="Sets the BUSYCH10 field."]
    #[inline] pub fn set_busych10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Busy Channel 11"]
    #[inline] pub fn busych11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BUSYCH11 != 0"]
    #[inline] pub fn test_busych11(&self) -> bool {
        self.busych11() != 0
    }

    #[doc="Sets the BUSYCH11 field."]
    #[inline] pub fn set_busych11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Busych {
    #[inline]
    fn from(other: u32) -> Self {
         Busych(other)
    }
}

impl ::core::fmt::Display for Busych {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Busych {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.busych0() != 0 { try!(write!(f, " busych0"))}
        if self.busych1() != 0 { try!(write!(f, " busych1"))}
        if self.busych2() != 0 { try!(write!(f, " busych2"))}
        if self.busych3() != 0 { try!(write!(f, " busych3"))}
        if self.busych4() != 0 { try!(write!(f, " busych4"))}
        if self.busych5() != 0 { try!(write!(f, " busych5"))}
        if self.busych6() != 0 { try!(write!(f, " busych6"))}
        if self.busych7() != 0 { try!(write!(f, " busych7"))}
        if self.busych8() != 0 { try!(write!(f, " busych8"))}
        if self.busych9() != 0 { try!(write!(f, " busych9"))}
        if self.busych10() != 0 { try!(write!(f, " busych10"))}
        if self.busych11() != 0 { try!(write!(f, " busych11"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ready Users"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Readyusr(pub u32);
impl Readyusr {
    #[doc="Ready User for Channel 0"]
    #[inline] pub fn readyusr0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if READYUSR0 != 0"]
    #[inline] pub fn test_readyusr0(&self) -> bool {
        self.readyusr0() != 0
    }

    #[doc="Sets the READYUSR0 field."]
    #[inline] pub fn set_readyusr0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ready User for Channel 1"]
    #[inline] pub fn readyusr1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if READYUSR1 != 0"]
    #[inline] pub fn test_readyusr1(&self) -> bool {
        self.readyusr1() != 0
    }

    #[doc="Sets the READYUSR1 field."]
    #[inline] pub fn set_readyusr1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Ready User for Channel 2"]
    #[inline] pub fn readyusr2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if READYUSR2 != 0"]
    #[inline] pub fn test_readyusr2(&self) -> bool {
        self.readyusr2() != 0
    }

    #[doc="Sets the READYUSR2 field."]
    #[inline] pub fn set_readyusr2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Ready User for Channel 3"]
    #[inline] pub fn readyusr3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if READYUSR3 != 0"]
    #[inline] pub fn test_readyusr3(&self) -> bool {
        self.readyusr3() != 0
    }

    #[doc="Sets the READYUSR3 field."]
    #[inline] pub fn set_readyusr3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Ready User for Channel 4"]
    #[inline] pub fn readyusr4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if READYUSR4 != 0"]
    #[inline] pub fn test_readyusr4(&self) -> bool {
        self.readyusr4() != 0
    }

    #[doc="Sets the READYUSR4 field."]
    #[inline] pub fn set_readyusr4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Ready User for Channel 5"]
    #[inline] pub fn readyusr5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if READYUSR5 != 0"]
    #[inline] pub fn test_readyusr5(&self) -> bool {
        self.readyusr5() != 0
    }

    #[doc="Sets the READYUSR5 field."]
    #[inline] pub fn set_readyusr5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Ready User for Channel 6"]
    #[inline] pub fn readyusr6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if READYUSR6 != 0"]
    #[inline] pub fn test_readyusr6(&self) -> bool {
        self.readyusr6() != 0
    }

    #[doc="Sets the READYUSR6 field."]
    #[inline] pub fn set_readyusr6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Ready User for Channel 7"]
    #[inline] pub fn readyusr7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if READYUSR7 != 0"]
    #[inline] pub fn test_readyusr7(&self) -> bool {
        self.readyusr7() != 0
    }

    #[doc="Sets the READYUSR7 field."]
    #[inline] pub fn set_readyusr7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Ready User for Channel 8"]
    #[inline] pub fn readyusr8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if READYUSR8 != 0"]
    #[inline] pub fn test_readyusr8(&self) -> bool {
        self.readyusr8() != 0
    }

    #[doc="Sets the READYUSR8 field."]
    #[inline] pub fn set_readyusr8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Ready User for Channel 9"]
    #[inline] pub fn readyusr9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if READYUSR9 != 0"]
    #[inline] pub fn test_readyusr9(&self) -> bool {
        self.readyusr9() != 0
    }

    #[doc="Sets the READYUSR9 field."]
    #[inline] pub fn set_readyusr9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Ready User for Channel 10"]
    #[inline] pub fn readyusr10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if READYUSR10 != 0"]
    #[inline] pub fn test_readyusr10(&self) -> bool {
        self.readyusr10() != 0
    }

    #[doc="Sets the READYUSR10 field."]
    #[inline] pub fn set_readyusr10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Ready User for Channel 11"]
    #[inline] pub fn readyusr11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if READYUSR11 != 0"]
    #[inline] pub fn test_readyusr11(&self) -> bool {
        self.readyusr11() != 0
    }

    #[doc="Sets the READYUSR11 field."]
    #[inline] pub fn set_readyusr11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Readyusr {
    #[inline]
    fn from(other: u32) -> Self {
         Readyusr(other)
    }
}

impl ::core::fmt::Display for Readyusr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Readyusr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.readyusr0() != 0 { try!(write!(f, " readyusr0"))}
        if self.readyusr1() != 0 { try!(write!(f, " readyusr1"))}
        if self.readyusr2() != 0 { try!(write!(f, " readyusr2"))}
        if self.readyusr3() != 0 { try!(write!(f, " readyusr3"))}
        if self.readyusr4() != 0 { try!(write!(f, " readyusr4"))}
        if self.readyusr5() != 0 { try!(write!(f, " readyusr5"))}
        if self.readyusr6() != 0 { try!(write!(f, " readyusr6"))}
        if self.readyusr7() != 0 { try!(write!(f, " readyusr7"))}
        if self.readyusr8() != 0 { try!(write!(f, " readyusr8"))}
        if self.readyusr9() != 0 { try!(write!(f, " readyusr9"))}
        if self.readyusr10() != 0 { try!(write!(f, " readyusr10"))}
        if self.readyusr11() != 0 { try!(write!(f, " readyusr11"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Channel(pub u32);
impl Channel {
    #[doc="Event Generator Selection"]
    #[inline] pub fn evgen(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if EVGEN != 0"]
    #[inline] pub fn test_evgen(&self) -> bool {
        self.evgen() != 0
    }

    #[doc="Sets the EVGEN field."]
    #[inline] pub fn set_evgen<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Path Selection"]
    #[inline] pub fn path(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if PATH != 0"]
    #[inline] pub fn test_path(&self) -> bool {
        self.path() != 0
    }

    #[doc="Sets the PATH field."]
    #[inline] pub fn set_path<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Edge Detection Selection"]
    #[inline] pub fn edgsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if EDGSEL != 0"]
    #[inline] pub fn test_edgsel(&self) -> bool {
        self.edgsel() != 0
    }

    #[doc="Sets the EDGSEL field."]
    #[inline] pub fn set_edgsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Run in standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Generic Clock On Demand"]
    #[inline] pub fn ondemand(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Channel {
    #[inline]
    fn from(other: u32) -> Self {
         Channel(other)
    }
}

impl ::core::fmt::Display for Channel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Channel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.evgen() != 0 { try!(write!(f, " evgen=0x{:x}", self.evgen()))}
        if self.path() != 0 { try!(write!(f, " path=0x{:x}", self.path()))}
        if self.edgsel() != 0 { try!(write!(f, " edgsel=0x{:x}", self.edgsel()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintenclr(pub u8);
impl Chintenclr {
    #[doc="Channel Overrun Interrupt Disable"]
    #[inline] pub fn ovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Event Detected Interrupt Disable"]
    #[inline] pub fn evd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EVD != 0"]
    #[inline] pub fn test_evd(&self) -> bool {
        self.evd() != 0
    }

    #[doc="Sets the EVD field."]
    #[inline] pub fn set_evd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Chintenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Chintenclr(other)
    }
}

impl ::core::fmt::Display for Chintenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chintenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.evd() != 0 { try!(write!(f, " evd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintenset(pub u8);
impl Chintenset {
    #[doc="Channel Overrun Interrupt Enable"]
    #[inline] pub fn ovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Event Detected Interrupt Enable"]
    #[inline] pub fn evd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EVD != 0"]
    #[inline] pub fn test_evd(&self) -> bool {
        self.evd() != 0
    }

    #[doc="Sets the EVD field."]
    #[inline] pub fn set_evd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Chintenset {
    #[inline]
    fn from(other: u8) -> Self {
         Chintenset(other)
    }
}

impl ::core::fmt::Display for Chintenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chintenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.evd() != 0 { try!(write!(f, " evd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintflag(pub u8);
impl Chintflag {
    #[doc="Channel Overrun"]
    #[inline] pub fn ovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Event Detected"]
    #[inline] pub fn evd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EVD != 0"]
    #[inline] pub fn test_evd(&self) -> bool {
        self.evd() != 0
    }

    #[doc="Sets the EVD field."]
    #[inline] pub fn set_evd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Chintflag {
    #[inline]
    fn from(other: u8) -> Self {
         Chintflag(other)
    }
}

impl ::core::fmt::Display for Chintflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chintflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.evd() != 0 { try!(write!(f, " evd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chstatus(pub u8);
impl Chstatus {
    #[doc="Ready User"]
    #[inline] pub fn rdyusr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RDYUSR != 0"]
    #[inline] pub fn test_rdyusr(&self) -> bool {
        self.rdyusr() != 0
    }

    #[doc="Sets the RDYUSR field."]
    #[inline] pub fn set_rdyusr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Busy Channel"]
    #[inline] pub fn busych(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BUSYCH != 0"]
    #[inline] pub fn test_busych(&self) -> bool {
        self.busych() != 0
    }

    #[doc="Sets the BUSYCH field."]
    #[inline] pub fn set_busych<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Chstatus {
    #[inline]
    fn from(other: u8) -> Self {
         Chstatus(other)
    }
}

impl ::core::fmt::Display for Chstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdyusr() != 0 { try!(write!(f, " rdyusr"))}
        if self.busych() != 0 { try!(write!(f, " busych"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

