::bobbin_mcu::periph!( SUPC, Supc, SUPC_PERIPH, SupcPeriph, SUPC_OWNED, SUPC_REF_COUNT, 0x40001800, 0x00, 0x25);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SUPC Peripheral"]
pub struct SupcPeriph(pub usize); 

impl SupcPeriph {
    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x0)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x4)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x8)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0xc)
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

    #[doc="Get the BOD33 Register."]
    #[inline] pub fn bod33_reg(&self) -> ::bobbin_mcu::register::Register<Bod33> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bod33, 0x10)
    }

    #[doc="Get the *mut pointer for the BOD33 register."]
    #[inline] pub fn bod33_mut(&self) -> *mut Bod33 { 
        self.bod33_reg().ptr()
    }

    #[doc="Get the *const pointer for the BOD33 register."]
    #[inline] pub fn bod33_ptr(&self) -> *const Bod33 { 
        self.bod33_reg().ptr()
    }

    #[doc="Read the BOD33 register."]
    #[inline] pub fn bod33(&self) -> Bod33 { 
        self.bod33_reg().read()
    }

    #[doc="Write the BOD33 register."]
    #[inline] pub fn write_bod33(&self, value: Bod33) -> &Self { 
        self.bod33_reg().write(value);
        self
    }

    #[doc="Set the BOD33 register."]
    #[inline] pub fn set_bod33<F: FnOnce(Bod33) -> Bod33>(&self, f: F) -> &Self {
        self.bod33_reg().set(f);
        self
    }

    #[doc="Modify the BOD33 register."]
    #[inline] pub fn with_bod33<F: FnOnce(Bod33) -> Bod33>(&self, f: F) -> &Self {
        self.bod33_reg().with(f);
        self
    }

    #[doc="Get the BOD12 Register."]
    #[inline] pub fn bod12_reg(&self) -> ::bobbin_mcu::register::Register<Bod12> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bod12, 0x14)
    }

    #[doc="Get the *mut pointer for the BOD12 register."]
    #[inline] pub fn bod12_mut(&self) -> *mut Bod12 { 
        self.bod12_reg().ptr()
    }

    #[doc="Get the *const pointer for the BOD12 register."]
    #[inline] pub fn bod12_ptr(&self) -> *const Bod12 { 
        self.bod12_reg().ptr()
    }

    #[doc="Read the BOD12 register."]
    #[inline] pub fn bod12(&self) -> Bod12 { 
        self.bod12_reg().read()
    }

    #[doc="Write the BOD12 register."]
    #[inline] pub fn write_bod12(&self, value: Bod12) -> &Self { 
        self.bod12_reg().write(value);
        self
    }

    #[doc="Set the BOD12 register."]
    #[inline] pub fn set_bod12<F: FnOnce(Bod12) -> Bod12>(&self, f: F) -> &Self {
        self.bod12_reg().set(f);
        self
    }

    #[doc="Modify the BOD12 register."]
    #[inline] pub fn with_bod12<F: FnOnce(Bod12) -> Bod12>(&self, f: F) -> &Self {
        self.bod12_reg().with(f);
        self
    }

    #[doc="Get the VREG Register."]
    #[inline] pub fn vreg_reg(&self) -> ::bobbin_mcu::register::Register<Vreg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Vreg, 0x18)
    }

    #[doc="Get the *mut pointer for the VREG register."]
    #[inline] pub fn vreg_mut(&self) -> *mut Vreg { 
        self.vreg_reg().ptr()
    }

    #[doc="Get the *const pointer for the VREG register."]
    #[inline] pub fn vreg_ptr(&self) -> *const Vreg { 
        self.vreg_reg().ptr()
    }

    #[doc="Read the VREG register."]
    #[inline] pub fn vreg(&self) -> Vreg { 
        self.vreg_reg().read()
    }

    #[doc="Write the VREG register."]
    #[inline] pub fn write_vreg(&self, value: Vreg) -> &Self { 
        self.vreg_reg().write(value);
        self
    }

    #[doc="Set the VREG register."]
    #[inline] pub fn set_vreg<F: FnOnce(Vreg) -> Vreg>(&self, f: F) -> &Self {
        self.vreg_reg().set(f);
        self
    }

    #[doc="Modify the VREG register."]
    #[inline] pub fn with_vreg<F: FnOnce(Vreg) -> Vreg>(&self, f: F) -> &Self {
        self.vreg_reg().with(f);
        self
    }

    #[doc="Get the VREF Register."]
    #[inline] pub fn vref_reg(&self) -> ::bobbin_mcu::register::Register<Vref> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Vref, 0x1c)
    }

    #[doc="Get the *mut pointer for the VREF register."]
    #[inline] pub fn vref_mut(&self) -> *mut Vref { 
        self.vref_reg().ptr()
    }

    #[doc="Get the *const pointer for the VREF register."]
    #[inline] pub fn vref_ptr(&self) -> *const Vref { 
        self.vref_reg().ptr()
    }

    #[doc="Read the VREF register."]
    #[inline] pub fn vref(&self) -> Vref { 
        self.vref_reg().read()
    }

    #[doc="Write the VREF register."]
    #[inline] pub fn write_vref(&self, value: Vref) -> &Self { 
        self.vref_reg().write(value);
        self
    }

    #[doc="Set the VREF register."]
    #[inline] pub fn set_vref<F: FnOnce(Vref) -> Vref>(&self, f: F) -> &Self {
        self.vref_reg().set(f);
        self
    }

    #[doc="Modify the VREF register."]
    #[inline] pub fn with_vref<F: FnOnce(Vref) -> Vref>(&self, f: F) -> &Self {
        self.vref_reg().with(f);
        self
    }

    #[doc="Get the BBPS Register."]
    #[inline] pub fn bbps_reg(&self) -> ::bobbin_mcu::register::Register<Bbps> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bbps, 0x20)
    }

    #[doc="Get the *mut pointer for the BBPS register."]
    #[inline] pub fn bbps_mut(&self) -> *mut Bbps { 
        self.bbps_reg().ptr()
    }

    #[doc="Get the *const pointer for the BBPS register."]
    #[inline] pub fn bbps_ptr(&self) -> *const Bbps { 
        self.bbps_reg().ptr()
    }

    #[doc="Read the BBPS register."]
    #[inline] pub fn bbps(&self) -> Bbps { 
        self.bbps_reg().read()
    }

    #[doc="Write the BBPS register."]
    #[inline] pub fn write_bbps(&self, value: Bbps) -> &Self { 
        self.bbps_reg().write(value);
        self
    }

    #[doc="Set the BBPS register."]
    #[inline] pub fn set_bbps<F: FnOnce(Bbps) -> Bbps>(&self, f: F) -> &Self {
        self.bbps_reg().set(f);
        self
    }

    #[doc="Modify the BBPS register."]
    #[inline] pub fn with_bbps<F: FnOnce(Bbps) -> Bbps>(&self, f: F) -> &Self {
        self.bbps_reg().with(f);
        self
    }

    #[doc="Get the BKOUT Register."]
    #[inline] pub fn bkout_reg(&self) -> ::bobbin_mcu::register::Register<Bkout> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bkout, 0x24)
    }

    #[doc="Get the *mut pointer for the BKOUT register."]
    #[inline] pub fn bkout_mut(&self) -> *mut Bkout { 
        self.bkout_reg().ptr()
    }

    #[doc="Get the *const pointer for the BKOUT register."]
    #[inline] pub fn bkout_ptr(&self) -> *const Bkout { 
        self.bkout_reg().ptr()
    }

    #[doc="Read the BKOUT register."]
    #[inline] pub fn bkout(&self) -> Bkout { 
        self.bkout_reg().read()
    }

    #[doc="Write the BKOUT register."]
    #[inline] pub fn write_bkout(&self, value: Bkout) -> &Self { 
        self.bkout_reg().write(value);
        self
    }

    #[doc="Set the BKOUT register."]
    #[inline] pub fn set_bkout<F: FnOnce(Bkout) -> Bkout>(&self, f: F) -> &Self {
        self.bkout_reg().set(f);
        self
    }

    #[doc="Modify the BKOUT register."]
    #[inline] pub fn with_bkout<F: FnOnce(Bkout) -> Bkout>(&self, f: F) -> &Self {
        self.bkout_reg().with(f);
        self
    }

    #[doc="Get the BKIN Register."]
    #[inline] pub fn bkin_reg(&self) -> ::bobbin_mcu::register::Register<Bkin> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bkin, 0x28)
    }

    #[doc="Get the *mut pointer for the BKIN register."]
    #[inline] pub fn bkin_mut(&self) -> *mut Bkin { 
        self.bkin_reg().ptr()
    }

    #[doc="Get the *const pointer for the BKIN register."]
    #[inline] pub fn bkin_ptr(&self) -> *const Bkin { 
        self.bkin_reg().ptr()
    }

    #[doc="Read the BKIN register."]
    #[inline] pub fn bkin(&self) -> Bkin { 
        self.bkin_reg().read()
    }

}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc="BOD33 Ready"]
    #[inline] pub fn bod33rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BOD33RDY != 0"]
    #[inline] pub fn test_bod33rdy(&self) -> bool {
        self.bod33rdy() != 0
    }

    #[doc="Sets the BOD33RDY field."]
    #[inline] pub fn set_bod33rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BOD33 Detection"]
    #[inline] pub fn bod33det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BOD33DET != 0"]
    #[inline] pub fn test_bod33det(&self) -> bool {
        self.bod33det() != 0
    }

    #[doc="Sets the BOD33DET field."]
    #[inline] pub fn set_bod33det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BOD33 Synchronization Ready"]
    #[inline] pub fn b33srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if B33SRDY != 0"]
    #[inline] pub fn test_b33srdy(&self) -> bool {
        self.b33srdy() != 0
    }

    #[doc="Sets the B33SRDY field."]
    #[inline] pub fn set_b33srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BOD12 Ready"]
    #[inline] pub fn bod12rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BOD12RDY != 0"]
    #[inline] pub fn test_bod12rdy(&self) -> bool {
        self.bod12rdy() != 0
    }

    #[doc="Sets the BOD12RDY field."]
    #[inline] pub fn set_bod12rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BOD12 Detection"]
    #[inline] pub fn bod12det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BOD12DET != 0"]
    #[inline] pub fn test_bod12det(&self) -> bool {
        self.bod12det() != 0
    }

    #[doc="Sets the BOD12DET field."]
    #[inline] pub fn set_bod12det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="BOD12 Synchronization Ready"]
    #[inline] pub fn b12srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if B12SRDY != 0"]
    #[inline] pub fn test_b12srdy(&self) -> bool {
        self.b12srdy() != 0
    }

    #[doc="Sets the B12SRDY field."]
    #[inline] pub fn set_b12srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Voltage Regulator Ready"]
    #[inline] pub fn vregrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if VREGRDY != 0"]
    #[inline] pub fn test_vregrdy(&self) -> bool {
        self.vregrdy() != 0
    }

    #[doc="Sets the VREGRDY field."]
    #[inline] pub fn set_vregrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="VDDCORE Ready"]
    #[inline] pub fn vcorerdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VCORERDY != 0"]
    #[inline] pub fn test_vcorerdy(&self) -> bool {
        self.vcorerdy() != 0
    }

    #[doc="Sets the VCORERDY field."]
    #[inline] pub fn set_vcorerdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
        if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
        if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
        if self.bod12rdy() != 0 { try!(write!(f, " bod12rdy"))}
        if self.bod12det() != 0 { try!(write!(f, " bod12det"))}
        if self.b12srdy() != 0 { try!(write!(f, " b12srdy"))}
        if self.vregrdy() != 0 { try!(write!(f, " vregrdy"))}
        if self.vcorerdy() != 0 { try!(write!(f, " vcorerdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc="BOD33 Ready"]
    #[inline] pub fn bod33rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BOD33RDY != 0"]
    #[inline] pub fn test_bod33rdy(&self) -> bool {
        self.bod33rdy() != 0
    }

    #[doc="Sets the BOD33RDY field."]
    #[inline] pub fn set_bod33rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BOD33 Detection"]
    #[inline] pub fn bod33det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BOD33DET != 0"]
    #[inline] pub fn test_bod33det(&self) -> bool {
        self.bod33det() != 0
    }

    #[doc="Sets the BOD33DET field."]
    #[inline] pub fn set_bod33det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BOD33 Synchronization Ready"]
    #[inline] pub fn b33srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if B33SRDY != 0"]
    #[inline] pub fn test_b33srdy(&self) -> bool {
        self.b33srdy() != 0
    }

    #[doc="Sets the B33SRDY field."]
    #[inline] pub fn set_b33srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BOD12 Ready"]
    #[inline] pub fn bod12rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BOD12RDY != 0"]
    #[inline] pub fn test_bod12rdy(&self) -> bool {
        self.bod12rdy() != 0
    }

    #[doc="Sets the BOD12RDY field."]
    #[inline] pub fn set_bod12rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BOD12 Detection"]
    #[inline] pub fn bod12det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BOD12DET != 0"]
    #[inline] pub fn test_bod12det(&self) -> bool {
        self.bod12det() != 0
    }

    #[doc="Sets the BOD12DET field."]
    #[inline] pub fn set_bod12det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="BOD12 Synchronization Ready"]
    #[inline] pub fn b12srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if B12SRDY != 0"]
    #[inline] pub fn test_b12srdy(&self) -> bool {
        self.b12srdy() != 0
    }

    #[doc="Sets the B12SRDY field."]
    #[inline] pub fn set_b12srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Voltage Regulator Ready"]
    #[inline] pub fn vregrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if VREGRDY != 0"]
    #[inline] pub fn test_vregrdy(&self) -> bool {
        self.vregrdy() != 0
    }

    #[doc="Sets the VREGRDY field."]
    #[inline] pub fn set_vregrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="VDDCORE Ready"]
    #[inline] pub fn vcorerdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VCORERDY != 0"]
    #[inline] pub fn test_vcorerdy(&self) -> bool {
        self.vcorerdy() != 0
    }

    #[doc="Sets the VCORERDY field."]
    #[inline] pub fn set_vcorerdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
        if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
        if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
        if self.bod12rdy() != 0 { try!(write!(f, " bod12rdy"))}
        if self.bod12det() != 0 { try!(write!(f, " bod12det"))}
        if self.b12srdy() != 0 { try!(write!(f, " b12srdy"))}
        if self.vregrdy() != 0 { try!(write!(f, " vregrdy"))}
        if self.vcorerdy() != 0 { try!(write!(f, " vcorerdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
    #[doc="BOD33 Ready"]
    #[inline] pub fn bod33rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BOD33RDY != 0"]
    #[inline] pub fn test_bod33rdy(&self) -> bool {
        self.bod33rdy() != 0
    }

    #[doc="Sets the BOD33RDY field."]
    #[inline] pub fn set_bod33rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BOD33 Detection"]
    #[inline] pub fn bod33det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BOD33DET != 0"]
    #[inline] pub fn test_bod33det(&self) -> bool {
        self.bod33det() != 0
    }

    #[doc="Sets the BOD33DET field."]
    #[inline] pub fn set_bod33det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BOD33 Synchronization Ready"]
    #[inline] pub fn b33srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if B33SRDY != 0"]
    #[inline] pub fn test_b33srdy(&self) -> bool {
        self.b33srdy() != 0
    }

    #[doc="Sets the B33SRDY field."]
    #[inline] pub fn set_b33srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BOD12 Ready"]
    #[inline] pub fn bod12rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BOD12RDY != 0"]
    #[inline] pub fn test_bod12rdy(&self) -> bool {
        self.bod12rdy() != 0
    }

    #[doc="Sets the BOD12RDY field."]
    #[inline] pub fn set_bod12rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BOD12 Detection"]
    #[inline] pub fn bod12det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BOD12DET != 0"]
    #[inline] pub fn test_bod12det(&self) -> bool {
        self.bod12det() != 0
    }

    #[doc="Sets the BOD12DET field."]
    #[inline] pub fn set_bod12det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="BOD12 Synchronization Ready"]
    #[inline] pub fn b12srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if B12SRDY != 0"]
    #[inline] pub fn test_b12srdy(&self) -> bool {
        self.b12srdy() != 0
    }

    #[doc="Sets the B12SRDY field."]
    #[inline] pub fn set_b12srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Voltage Regulator Ready"]
    #[inline] pub fn vregrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if VREGRDY != 0"]
    #[inline] pub fn test_vregrdy(&self) -> bool {
        self.vregrdy() != 0
    }

    #[doc="Sets the VREGRDY field."]
    #[inline] pub fn set_vregrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="VDDCORE Ready"]
    #[inline] pub fn vcorerdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VCORERDY != 0"]
    #[inline] pub fn test_vcorerdy(&self) -> bool {
        self.vcorerdy() != 0
    }

    #[doc="Sets the VCORERDY field."]
    #[inline] pub fn set_vcorerdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
        if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
        if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
        if self.bod12rdy() != 0 { try!(write!(f, " bod12rdy"))}
        if self.bod12det() != 0 { try!(write!(f, " bod12det"))}
        if self.b12srdy() != 0 { try!(write!(f, " b12srdy"))}
        if self.vregrdy() != 0 { try!(write!(f, " vregrdy"))}
        if self.vcorerdy() != 0 { try!(write!(f, " vcorerdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power and Clocks Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="BOD33 Ready"]
    #[inline] pub fn bod33rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BOD33RDY != 0"]
    #[inline] pub fn test_bod33rdy(&self) -> bool {
        self.bod33rdy() != 0
    }

    #[doc="Sets the BOD33RDY field."]
    #[inline] pub fn set_bod33rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BOD33 Detection"]
    #[inline] pub fn bod33det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BOD33DET != 0"]
    #[inline] pub fn test_bod33det(&self) -> bool {
        self.bod33det() != 0
    }

    #[doc="Sets the BOD33DET field."]
    #[inline] pub fn set_bod33det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BOD33 Synchronization Ready"]
    #[inline] pub fn b33srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if B33SRDY != 0"]
    #[inline] pub fn test_b33srdy(&self) -> bool {
        self.b33srdy() != 0
    }

    #[doc="Sets the B33SRDY field."]
    #[inline] pub fn set_b33srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BOD12 Ready"]
    #[inline] pub fn bod12rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BOD12RDY != 0"]
    #[inline] pub fn test_bod12rdy(&self) -> bool {
        self.bod12rdy() != 0
    }

    #[doc="Sets the BOD12RDY field."]
    #[inline] pub fn set_bod12rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BOD12 Detection"]
    #[inline] pub fn bod12det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BOD12DET != 0"]
    #[inline] pub fn test_bod12det(&self) -> bool {
        self.bod12det() != 0
    }

    #[doc="Sets the BOD12DET field."]
    #[inline] pub fn set_bod12det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="BOD12 Synchronization Ready"]
    #[inline] pub fn b12srdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if B12SRDY != 0"]
    #[inline] pub fn test_b12srdy(&self) -> bool {
        self.b12srdy() != 0
    }

    #[doc="Sets the B12SRDY field."]
    #[inline] pub fn set_b12srdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Voltage Regulator Ready"]
    #[inline] pub fn vregrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if VREGRDY != 0"]
    #[inline] pub fn test_vregrdy(&self) -> bool {
        self.vregrdy() != 0
    }

    #[doc="Sets the VREGRDY field."]
    #[inline] pub fn set_vregrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="VDDCORE Ready"]
    #[inline] pub fn vcorerdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VCORERDY != 0"]
    #[inline] pub fn test_vcorerdy(&self) -> bool {
        self.vcorerdy() != 0
    }

    #[doc="Sets the VCORERDY field."]
    #[inline] pub fn set_vcorerdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
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
        if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
        if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
        if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
        if self.bod12rdy() != 0 { try!(write!(f, " bod12rdy"))}
        if self.bod12det() != 0 { try!(write!(f, " bod12det"))}
        if self.b12srdy() != 0 { try!(write!(f, " b12srdy"))}
        if self.vregrdy() != 0 { try!(write!(f, " vregrdy"))}
        if self.vcorerdy() != 0 { try!(write!(f, " vcorerdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BOD33 Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bod33(pub u32);
impl Bod33 {
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

    #[doc="Action when Threshold Crossed"]
    #[inline] pub fn action(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if ACTION != 0"]
    #[inline] pub fn test_action(&self) -> bool {
        self.action() != 0
    }

    #[doc="Sets the ACTION field."]
    #[inline] pub fn set_action<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Configuration in Standby mode"]
    #[inline] pub fn stdbycfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STDBYCFG != 0"]
    #[inline] pub fn test_stdbycfg(&self) -> bool {
        self.stdbycfg() != 0
    }

    #[doc="Sets the STDBYCFG field."]
    #[inline] pub fn set_stdbycfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Run in Standby mode"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Run in Hibernate mode"]
    #[inline] pub fn runhib(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNHIB != 0"]
    #[inline] pub fn test_runhib(&self) -> bool {
        self.runhib() != 0
    }

    #[doc="Sets the RUNHIB field."]
    #[inline] pub fn set_runhib<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Run in Backup mode"]
    #[inline] pub fn runbkup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RUNBKUP != 0"]
    #[inline] pub fn test_runbkup(&self) -> bool {
        self.runbkup() != 0
    }

    #[doc="Sets the RUNBKUP field."]
    #[inline] pub fn set_runbkup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Hysteresis value"]
    #[inline] pub fn hyst(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if HYST != 0"]
    #[inline] pub fn test_hyst(&self) -> bool {
        self.hyst() != 0
    }

    #[doc="Sets the HYST field."]
    #[inline] pub fn set_hyst<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Prescaler Select"]
    #[inline] pub fn psel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if PSEL != 0"]
    #[inline] pub fn test_psel(&self) -> bool {
        self.psel() != 0
    }

    #[doc="Sets the PSEL field."]
    #[inline] pub fn set_psel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Threshold Level for VDD"]
    #[inline] pub fn level(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if LEVEL != 0"]
    #[inline] pub fn test_level(&self) -> bool {
        self.level() != 0
    }

    #[doc="Sets the LEVEL field."]
    #[inline] pub fn set_level<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Threshold Level in battery backup sleep mode for VBAT"]
    #[inline] pub fn vbatlevel(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if VBATLEVEL != 0"]
    #[inline] pub fn test_vbatlevel(&self) -> bool {
        self.vbatlevel() != 0
    }

    #[doc="Sets the VBATLEVEL field."]
    #[inline] pub fn set_vbatlevel<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Bod33 {
    #[inline]
    fn from(other: u32) -> Self {
         Bod33(other)
    }
}

impl ::core::fmt::Display for Bod33 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bod33 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.action() != 0 { try!(write!(f, " action=0x{:x}", self.action()))}
        if self.stdbycfg() != 0 { try!(write!(f, " stdbycfg"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.runhib() != 0 { try!(write!(f, " runhib"))}
        if self.runbkup() != 0 { try!(write!(f, " runbkup"))}
        if self.hyst() != 0 { try!(write!(f, " hyst=0x{:x}", self.hyst()))}
        if self.psel() != 0 { try!(write!(f, " psel=0x{:x}", self.psel()))}
        if self.level() != 0 { try!(write!(f, " level=0x{:x}", self.level()))}
        if self.vbatlevel() != 0 { try!(write!(f, " vbatlevel=0x{:x}", self.vbatlevel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BOD12 Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bod12(pub u32);
impl Bod12 {
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

    #[doc="Hysteresis Enable"]
    #[inline] pub fn hyst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HYST != 0"]
    #[inline] pub fn test_hyst(&self) -> bool {
        self.hyst() != 0
    }

    #[doc="Sets the HYST field."]
    #[inline] pub fn set_hyst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Action when Threshold Crossed"]
    #[inline] pub fn action(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if ACTION != 0"]
    #[inline] pub fn test_action(&self) -> bool {
        self.action() != 0
    }

    #[doc="Sets the ACTION field."]
    #[inline] pub fn set_action<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Configuration in Standby mode"]
    #[inline] pub fn stdbycfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STDBYCFG != 0"]
    #[inline] pub fn test_stdbycfg(&self) -> bool {
        self.stdbycfg() != 0
    }

    #[doc="Sets the STDBYCFG field."]
    #[inline] pub fn set_stdbycfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Run during Standby"]
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

    #[doc="Configuration in Active mode"]
    #[inline] pub fn actcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ACTCFG != 0"]
    #[inline] pub fn test_actcfg(&self) -> bool {
        self.actcfg() != 0
    }

    #[doc="Sets the ACTCFG field."]
    #[inline] pub fn set_actcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Prescaler Select"]
    #[inline] pub fn psel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if PSEL != 0"]
    #[inline] pub fn test_psel(&self) -> bool {
        self.psel() != 0
    }

    #[doc="Sets the PSEL field."]
    #[inline] pub fn set_psel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Threshold Level"]
    #[inline] pub fn level(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if LEVEL != 0"]
    #[inline] pub fn test_level(&self) -> bool {
        self.level() != 0
    }

    #[doc="Sets the LEVEL field."]
    #[inline] pub fn set_level<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Bod12 {
    #[inline]
    fn from(other: u32) -> Self {
         Bod12(other)
    }
}

impl ::core::fmt::Display for Bod12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bod12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.hyst() != 0 { try!(write!(f, " hyst"))}
        if self.action() != 0 { try!(write!(f, " action=0x{:x}", self.action()))}
        if self.stdbycfg() != 0 { try!(write!(f, " stdbycfg"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.actcfg() != 0 { try!(write!(f, " actcfg"))}
        if self.psel() != 0 { try!(write!(f, " psel=0x{:x}", self.psel()))}
        if self.level() != 0 { try!(write!(f, " level=0x{:x}", self.level()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="VREG Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vreg(pub u32);
impl Vreg {
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

    #[doc="Voltage Regulator Selection"]
    #[inline] pub fn sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SEL != 0"]
    #[inline] pub fn test_sel(&self) -> bool {
        self.sel() != 0
    }

    #[doc="Sets the SEL field."]
    #[inline] pub fn set_sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Run in Backup mode"]
    #[inline] pub fn runbkup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RUNBKUP != 0"]
    #[inline] pub fn test_runbkup(&self) -> bool {
        self.runbkup() != 0
    }

    #[doc="Sets the RUNBKUP field."]
    #[inline] pub fn set_runbkup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Voltage Scaling Enable"]
    #[inline] pub fn vsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if VSEN != 0"]
    #[inline] pub fn test_vsen(&self) -> bool {
        self.vsen() != 0
    }

    #[doc="Sets the VSEN field."]
    #[inline] pub fn set_vsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Voltage Scaling Period"]
    #[inline] pub fn vsper(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if VSPER != 0"]
    #[inline] pub fn test_vsper(&self) -> bool {
        self.vsper() != 0
    }

    #[doc="Sets the VSPER field."]
    #[inline] pub fn set_vsper<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Vreg {
    #[inline]
    fn from(other: u32) -> Self {
         Vreg(other)
    }
}

impl ::core::fmt::Display for Vreg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vreg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.sel() != 0 { try!(write!(f, " sel"))}
        if self.runbkup() != 0 { try!(write!(f, " runbkup"))}
        if self.vsen() != 0 { try!(write!(f, " vsen"))}
        if self.vsper() != 0 { try!(write!(f, " vsper=0x{:x}", self.vsper()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="VREF Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vref(pub u32);
impl Vref {
    #[doc="Temperature Sensor Output Enable"]
    #[inline] pub fn tsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TSEN != 0"]
    #[inline] pub fn test_tsen(&self) -> bool {
        self.tsen() != 0
    }

    #[doc="Sets the TSEN field."]
    #[inline] pub fn set_tsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Voltage Reference Output Enable"]
    #[inline] pub fn vrefoe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if VREFOE != 0"]
    #[inline] pub fn test_vrefoe(&self) -> bool {
        self.vrefoe() != 0
    }

    #[doc="Sets the VREFOE field."]
    #[inline] pub fn set_vrefoe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Temperature Sensor Selection"]
    #[inline] pub fn tssel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TSSEL != 0"]
    #[inline] pub fn test_tssel(&self) -> bool {
        self.tssel() != 0
    }

    #[doc="Sets the TSSEL field."]
    #[inline] pub fn set_tssel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Run during Standby"]
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

    #[doc="On Demand Contrl"]
    #[inline] pub fn ondemand(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Voltage Reference Selection"]
    #[inline] pub fn sel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if SEL != 0"]
    #[inline] pub fn test_sel(&self) -> bool {
        self.sel() != 0
    }

    #[doc="Sets the SEL field."]
    #[inline] pub fn set_sel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Vref {
    #[inline]
    fn from(other: u32) -> Self {
         Vref(other)
    }
}

impl ::core::fmt::Display for Vref {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vref {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsen() != 0 { try!(write!(f, " tsen"))}
        if self.vrefoe() != 0 { try!(write!(f, " vrefoe"))}
        if self.tssel() != 0 { try!(write!(f, " tssel"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.sel() != 0 { try!(write!(f, " sel=0x{:x}", self.sel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Battery Backup Power Switch"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bbps(pub u32);
impl Bbps {
    #[doc="Battery Backup Configuration"]
    #[inline] pub fn conf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CONF != 0"]
    #[inline] pub fn test_conf(&self) -> bool {
        self.conf() != 0
    }

    #[doc="Sets the CONF field."]
    #[inline] pub fn set_conf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Wake Enable"]
    #[inline] pub fn wakeen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WAKEEN != 0"]
    #[inline] pub fn test_wakeen(&self) -> bool {
        self.wakeen() != 0
    }

    #[doc="Sets the WAKEEN field."]
    #[inline] pub fn set_wakeen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Bbps {
    #[inline]
    fn from(other: u32) -> Self {
         Bbps(other)
    }
}

impl ::core::fmt::Display for Bbps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bbps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.conf() != 0 { try!(write!(f, " conf"))}
        if self.wakeen() != 0 { try!(write!(f, " wakeen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup Output Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bkout(pub u32);
impl Bkout {
    #[doc="Enable Output"]
    #[inline] pub fn en(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear Output"]
    #[inline] pub fn clr(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if CLR != 0"]
    #[inline] pub fn test_clr(&self) -> bool {
        self.clr() != 0
    }

    #[doc="Sets the CLR field."]
    #[inline] pub fn set_clr<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Set Output"]
    #[inline] pub fn set(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if SET != 0"]
    #[inline] pub fn test_set(&self) -> bool {
        self.set() != 0
    }

    #[doc="Sets the SET field."]
    #[inline] pub fn set_set<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="RTC Toggle Output"]
    #[inline] pub fn rtctgl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if RTCTGL != 0"]
    #[inline] pub fn test_rtctgl(&self) -> bool {
        self.rtctgl() != 0
    }

    #[doc="Sets the RTCTGL field."]
    #[inline] pub fn set_rtctgl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Bkout {
    #[inline]
    fn from(other: u32) -> Self {
         Bkout(other)
    }
}

impl ::core::fmt::Display for Bkout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bkout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.en() != 0 { try!(write!(f, " en=0x{:x}", self.en()))}
        if self.clr() != 0 { try!(write!(f, " clr=0x{:x}", self.clr()))}
        if self.set() != 0 { try!(write!(f, " set=0x{:x}", self.set()))}
        if self.rtctgl() != 0 { try!(write!(f, " rtctgl=0x{:x}", self.rtctgl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup Input Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bkin(pub u32);
impl Bkin {
    #[doc="Backup Input Value"]
    #[inline] pub fn bkin(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BKIN != 0"]
    #[inline] pub fn test_bkin(&self) -> bool {
        self.bkin() != 0
    }

    #[doc="Sets the BKIN field."]
    #[inline] pub fn set_bkin<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bkin {
    #[inline]
    fn from(other: u32) -> Self {
         Bkin(other)
    }
}

impl ::core::fmt::Display for Bkin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bkin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bkin() != 0 { try!(write!(f, " bkin=0x{:x}", self.bkin()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

