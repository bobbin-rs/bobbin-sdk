#[allow(unused_imports)] use bobbin_common::*;

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x4000a000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x4000a030);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x4000a060);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x4000a090);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x4000a0c0);
periph!( GPIOF, Gpiof, _GPIOF, GpioPeriph, 0x4000a0f0);
periph!( GPIOG, Gpiog, _GPIOG, GpioPeriph, 0x4000a120);
periph!( GPIOH, Gpioh, _GPIOH, GpioPeriph, 0x4000a150);
periph!( GPIOI, Gpioi, _GPIOI, GpioPeriph, 0x4000a180);
periph!( GPIOJ, Gpioj, _GPIOJ, GpioPeriph, 0x4000a1b0);
periph!( GPIOK, Gpiok, _GPIOK, GpioPeriph, 0x4000a1e0);
periph!( GPIOL, Gpiol, _GPIOL, GpioPeriph, 0x4000a210);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 














impl GpioPeriph {
    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        (self.0 + 0x0) as *mut Ctrl
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
           self.ctrl_mut()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        unsafe {
            read_volatile(self.ctrl_ptr())
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(Ctrl(0)));
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(self.ctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MODEL register."]
    #[inline] pub fn model_mut(&self) -> *mut Model { 
        (self.0 + 0x4) as *mut Model
    }

    #[doc="Get the *const pointer for the MODEL register."]
    #[inline] pub fn model_ptr(&self) -> *const Model { 
           self.model_mut()
    }

    #[doc="Read the MODEL register."]
    #[inline] pub fn model(&self) -> Model { 
        unsafe {
            read_volatile(self.model_ptr())
        }
    }

    #[doc="Write the MODEL register."]
    #[inline] pub fn set_model<F: FnOnce(Model) -> Model>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.model_mut(), f(Model(0)));
        }
        self
    }

    #[doc="Modify the MODEL register."]
    #[inline] pub fn with_model<F: FnOnce(Model) -> Model>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.model_mut(), f(self.model()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MODEH register."]
    #[inline] pub fn modeh_mut(&self) -> *mut Modeh { 
        (self.0 + 0x8) as *mut Modeh
    }

    #[doc="Get the *const pointer for the MODEH register."]
    #[inline] pub fn modeh_ptr(&self) -> *const Modeh { 
           self.modeh_mut()
    }

    #[doc="Read the MODEH register."]
    #[inline] pub fn modeh(&self) -> Modeh { 
        unsafe {
            read_volatile(self.modeh_ptr())
        }
    }

    #[doc="Write the MODEH register."]
    #[inline] pub fn set_modeh<F: FnOnce(Modeh) -> Modeh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.modeh_mut(), f(Modeh(0)));
        }
        self
    }

    #[doc="Modify the MODEH register."]
    #[inline] pub fn with_modeh<F: FnOnce(Modeh) -> Modeh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.modeh_mut(), f(self.modeh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOUT register."]
    #[inline] pub fn dout_mut(&self) -> *mut Dout { 
        (self.0 + 0xc) as *mut Dout
    }

    #[doc="Get the *const pointer for the DOUT register."]
    #[inline] pub fn dout_ptr(&self) -> *const Dout { 
           self.dout_mut()
    }

    #[doc="Read the DOUT register."]
    #[inline] pub fn dout(&self) -> Dout { 
        unsafe {
            read_volatile(self.dout_ptr())
        }
    }

    #[doc="Write the DOUT register."]
    #[inline] pub fn set_dout<F: FnOnce(Dout) -> Dout>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dout_mut(), f(Dout(0)));
        }
        self
    }

    #[doc="Modify the DOUT register."]
    #[inline] pub fn with_dout<F: FnOnce(Dout) -> Dout>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dout_mut(), f(self.dout()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOUTTGL register."]
    #[inline] pub fn douttgl_mut(&self) -> *mut Douttgl { 
        (self.0 + 0x18) as *mut Douttgl
    }

    #[doc="Get the *const pointer for the DOUTTGL register."]
    #[inline] pub fn douttgl_ptr(&self) -> *const Douttgl { 
           self.douttgl_mut()
    }

    #[doc="Write the DOUTTGL register."]
    #[inline] pub fn set_douttgl<F: FnOnce(Douttgl) -> Douttgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.douttgl_mut(), f(Douttgl(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIN register."]
    #[inline] pub fn din_mut(&self) -> *mut Din { 
        (self.0 + 0x1c) as *mut Din
    }

    #[doc="Get the *const pointer for the DIN register."]
    #[inline] pub fn din_ptr(&self) -> *const Din { 
           self.din_mut()
    }

    #[doc="Read the DIN register."]
    #[inline] pub fn din(&self) -> Din { 
        unsafe {
            read_volatile(self.din_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PINLOCKN register."]
    #[inline] pub fn pinlockn_mut(&self) -> *mut Pinlockn { 
        (self.0 + 0x20) as *mut Pinlockn
    }

    #[doc="Get the *const pointer for the PINLOCKN register."]
    #[inline] pub fn pinlockn_ptr(&self) -> *const Pinlockn { 
           self.pinlockn_mut()
    }

    #[doc="Read the PINLOCKN register."]
    #[inline] pub fn pinlockn(&self) -> Pinlockn { 
        unsafe {
            read_volatile(self.pinlockn_ptr())
        }
    }

    #[doc="Write the PINLOCKN register."]
    #[inline] pub fn set_pinlockn<F: FnOnce(Pinlockn) -> Pinlockn>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pinlockn_mut(), f(Pinlockn(0)));
        }
        self
    }

    #[doc="Modify the PINLOCKN register."]
    #[inline] pub fn with_pinlockn<F: FnOnce(Pinlockn) -> Pinlockn>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pinlockn_mut(), f(self.pinlockn()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OVTDIS register."]
    #[inline] pub fn ovtdis_mut(&self) -> *mut Ovtdis { 
        (self.0 + 0x28) as *mut Ovtdis
    }

    #[doc="Get the *const pointer for the OVTDIS register."]
    #[inline] pub fn ovtdis_ptr(&self) -> *const Ovtdis { 
           self.ovtdis_mut()
    }

    #[doc="Read the OVTDIS register."]
    #[inline] pub fn ovtdis(&self) -> Ovtdis { 
        unsafe {
            read_volatile(self.ovtdis_ptr())
        }
    }

    #[doc="Write the OVTDIS register."]
    #[inline] pub fn set_ovtdis<F: FnOnce(Ovtdis) -> Ovtdis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ovtdis_mut(), f(Ovtdis(0)));
        }
        self
    }

    #[doc="Modify the OVTDIS register."]
    #[inline] pub fn with_ovtdis<F: FnOnce(Ovtdis) -> Ovtdis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ovtdis_mut(), f(self.ovtdis()));
        }
        self
    }

}

#[doc="Port Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Drive strength for port"]
    #[inline] pub fn drivestrength(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DRIVESTRENGTH != 0"]
    #[inline] pub fn test_drivestrength(&self) -> bool {
        self.drivestrength() != 0
    }

    #[doc="Sets the DRIVESTRENGTH field."]
    #[inline] pub fn set_drivestrength<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Slewrate limit for port"]
    #[inline] pub fn slewrate(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if SLEWRATE != 0"]
    #[inline] pub fn test_slewrate(&self) -> bool {
        self.slewrate() != 0
    }

    #[doc="Sets the SLEWRATE field."]
    #[inline] pub fn set_slewrate<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data In Disable"]
    #[inline] pub fn dindis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DINDIS != 0"]
    #[inline] pub fn test_dindis(&self) -> bool {
        self.dindis() != 0
    }

    #[doc="Sets the DINDIS field."]
    #[inline] pub fn set_dindis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Alternate drive strength for port"]
    #[inline] pub fn drivestrengthalt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DRIVESTRENGTHALT != 0"]
    #[inline] pub fn test_drivestrengthalt(&self) -> bool {
        self.drivestrengthalt() != 0
    }

    #[doc="Sets the DRIVESTRENGTHALT field."]
    #[inline] pub fn set_drivestrengthalt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Alternate slewrate limit for port"]
    #[inline] pub fn slewratealt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if SLEWRATEALT != 0"]
    #[inline] pub fn test_slewratealt(&self) -> bool {
        self.slewratealt() != 0
    }

    #[doc="Sets the SLEWRATEALT field."]
    #[inline] pub fn set_slewratealt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Alternate Data In Disable"]
    #[inline] pub fn dindisalt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if DINDISALT != 0"]
    #[inline] pub fn test_dindisalt(&self) -> bool {
        self.dindisalt() != 0
    }

    #[doc="Sets the DINDISALT field."]
    #[inline] pub fn set_dindisalt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
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
        if self.drivestrength() != 0 { try!(write!(f, " drivestrength"))}
        if self.slewrate() != 0 { try!(write!(f, " slewrate=0x{:x}", self.slewrate()))}
        if self.dindis() != 0 { try!(write!(f, " dindis"))}
        if self.drivestrengthalt() != 0 { try!(write!(f, " drivestrengthalt"))}
        if self.slewratealt() != 0 { try!(write!(f, " slewratealt=0x{:x}", self.slewratealt()))}
        if self.dindisalt() != 0 { try!(write!(f, " dindisalt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Pin Mode Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Model(pub u32);
impl Model {
    #[doc="Pin n Mode"]
    #[inline] pub fn mode<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mode(index) != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Model {
    #[inline]
    fn from(other: u32) -> Self {
         Model(other)
    }
}

impl ::core::fmt::Display for Model {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Model {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode(0) != 0 { try!(write!(f, " mode[0]=0x{:x}", self.mode(0)))}
        if self.mode(1) != 0 { try!(write!(f, " mode[1]=0x{:x}", self.mode(1)))}
        if self.mode(2) != 0 { try!(write!(f, " mode[2]=0x{:x}", self.mode(2)))}
        if self.mode(3) != 0 { try!(write!(f, " mode[3]=0x{:x}", self.mode(3)))}
        if self.mode(4) != 0 { try!(write!(f, " mode[4]=0x{:x}", self.mode(4)))}
        if self.mode(5) != 0 { try!(write!(f, " mode[5]=0x{:x}", self.mode(5)))}
        if self.mode(6) != 0 { try!(write!(f, " mode[6]=0x{:x}", self.mode(6)))}
        if self.mode(7) != 0 { try!(write!(f, " mode[7]=0x{:x}", self.mode(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Pin Mode High Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Modeh(pub u32);
impl Modeh {
    #[doc="Pin 8 Mode"]
    #[inline] pub fn mode<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mode(index) != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Modeh {
    #[inline]
    fn from(other: u32) -> Self {
         Modeh(other)
    }
}

impl ::core::fmt::Display for Modeh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Modeh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode(0) != 0 { try!(write!(f, " mode[0]=0x{:x}", self.mode(0)))}
        if self.mode(1) != 0 { try!(write!(f, " mode[1]=0x{:x}", self.mode(1)))}
        if self.mode(2) != 0 { try!(write!(f, " mode[2]=0x{:x}", self.mode(2)))}
        if self.mode(3) != 0 { try!(write!(f, " mode[3]=0x{:x}", self.mode(3)))}
        if self.mode(4) != 0 { try!(write!(f, " mode[4]=0x{:x}", self.mode(4)))}
        if self.mode(5) != 0 { try!(write!(f, " mode[5]=0x{:x}", self.mode(5)))}
        if self.mode(6) != 0 { try!(write!(f, " mode[6]=0x{:x}", self.mode(6)))}
        if self.mode(7) != 0 { try!(write!(f, " mode[7]=0x{:x}", self.mode(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Out Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dout(pub u32);
impl Dout {
    #[doc="Data Out"]
    #[inline] pub fn dout<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DOUT != 0"]
    #[inline] pub fn test_dout<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.dout(index) != 0
    }

    #[doc="Sets the DOUT field."]
    #[inline] pub fn set_dout<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dout {
    #[inline]
    fn from(other: u32) -> Self {
         Dout(other)
    }
}

impl ::core::fmt::Display for Dout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dout(0) != 0 { try!(write!(f, " dout[0]"))}
        if self.dout(1) != 0 { try!(write!(f, " dout[1]"))}
        if self.dout(2) != 0 { try!(write!(f, " dout[2]"))}
        if self.dout(3) != 0 { try!(write!(f, " dout[3]"))}
        if self.dout(4) != 0 { try!(write!(f, " dout[4]"))}
        if self.dout(5) != 0 { try!(write!(f, " dout[5]"))}
        if self.dout(6) != 0 { try!(write!(f, " dout[6]"))}
        if self.dout(7) != 0 { try!(write!(f, " dout[7]"))}
        if self.dout(8) != 0 { try!(write!(f, " dout[8]"))}
        if self.dout(9) != 0 { try!(write!(f, " dout[9]"))}
        if self.dout(10) != 0 { try!(write!(f, " dout[10]"))}
        if self.dout(11) != 0 { try!(write!(f, " dout[11]"))}
        if self.dout(12) != 0 { try!(write!(f, " dout[12]"))}
        if self.dout(13) != 0 { try!(write!(f, " dout[13]"))}
        if self.dout(14) != 0 { try!(write!(f, " dout[14]"))}
        if self.dout(15) != 0 { try!(write!(f, " dout[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Out Toggle Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Douttgl(pub u32);
impl Douttgl {
    #[doc="Data Out Toggle"]
    #[inline] pub fn douttgl<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DOUTTGL != 0"]
    #[inline] pub fn test_douttgl<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.douttgl(index) != 0
    }

    #[doc="Sets the DOUTTGL field."]
    #[inline] pub fn set_douttgl<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Douttgl {
    #[inline]
    fn from(other: u32) -> Self {
         Douttgl(other)
    }
}

impl ::core::fmt::Display for Douttgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Douttgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.douttgl(0) != 0 { try!(write!(f, " douttgl[0]"))}
        if self.douttgl(1) != 0 { try!(write!(f, " douttgl[1]"))}
        if self.douttgl(2) != 0 { try!(write!(f, " douttgl[2]"))}
        if self.douttgl(3) != 0 { try!(write!(f, " douttgl[3]"))}
        if self.douttgl(4) != 0 { try!(write!(f, " douttgl[4]"))}
        if self.douttgl(5) != 0 { try!(write!(f, " douttgl[5]"))}
        if self.douttgl(6) != 0 { try!(write!(f, " douttgl[6]"))}
        if self.douttgl(7) != 0 { try!(write!(f, " douttgl[7]"))}
        if self.douttgl(8) != 0 { try!(write!(f, " douttgl[8]"))}
        if self.douttgl(9) != 0 { try!(write!(f, " douttgl[9]"))}
        if self.douttgl(10) != 0 { try!(write!(f, " douttgl[10]"))}
        if self.douttgl(11) != 0 { try!(write!(f, " douttgl[11]"))}
        if self.douttgl(12) != 0 { try!(write!(f, " douttgl[12]"))}
        if self.douttgl(13) != 0 { try!(write!(f, " douttgl[13]"))}
        if self.douttgl(14) != 0 { try!(write!(f, " douttgl[14]"))}
        if self.douttgl(15) != 0 { try!(write!(f, " douttgl[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data In Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Din(pub u32);
impl Din {
    #[doc="Data In"]
    #[inline] pub fn din<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIN != 0"]
    #[inline] pub fn test_din<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.din(index) != 0
    }

    #[doc="Sets the DIN field."]
    #[inline] pub fn set_din<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Din {
    #[inline]
    fn from(other: u32) -> Self {
         Din(other)
    }
}

impl ::core::fmt::Display for Din {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Din {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.din(0) != 0 { try!(write!(f, " din[0]"))}
        if self.din(1) != 0 { try!(write!(f, " din[1]"))}
        if self.din(2) != 0 { try!(write!(f, " din[2]"))}
        if self.din(3) != 0 { try!(write!(f, " din[3]"))}
        if self.din(4) != 0 { try!(write!(f, " din[4]"))}
        if self.din(5) != 0 { try!(write!(f, " din[5]"))}
        if self.din(6) != 0 { try!(write!(f, " din[6]"))}
        if self.din(7) != 0 { try!(write!(f, " din[7]"))}
        if self.din(8) != 0 { try!(write!(f, " din[8]"))}
        if self.din(9) != 0 { try!(write!(f, " din[9]"))}
        if self.din(10) != 0 { try!(write!(f, " din[10]"))}
        if self.din(11) != 0 { try!(write!(f, " din[11]"))}
        if self.din(12) != 0 { try!(write!(f, " din[12]"))}
        if self.din(13) != 0 { try!(write!(f, " din[13]"))}
        if self.din(14) != 0 { try!(write!(f, " din[14]"))}
        if self.din(15) != 0 { try!(write!(f, " din[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Unlocked Pins Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pinlockn(pub u32);
impl Pinlockn {
    #[doc="Unlocked Pins"]
    #[inline] pub fn pinlockn<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PINLOCKN != 0"]
    #[inline] pub fn test_pinlockn<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.pinlockn(index) != 0
    }

    #[doc="Sets the PINLOCKN field."]
    #[inline] pub fn set_pinlockn<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pinlockn {
    #[inline]
    fn from(other: u32) -> Self {
         Pinlockn(other)
    }
}

impl ::core::fmt::Display for Pinlockn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pinlockn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pinlockn(0) != 0 { try!(write!(f, " pinlockn[0]"))}
        if self.pinlockn(1) != 0 { try!(write!(f, " pinlockn[1]"))}
        if self.pinlockn(2) != 0 { try!(write!(f, " pinlockn[2]"))}
        if self.pinlockn(3) != 0 { try!(write!(f, " pinlockn[3]"))}
        if self.pinlockn(4) != 0 { try!(write!(f, " pinlockn[4]"))}
        if self.pinlockn(5) != 0 { try!(write!(f, " pinlockn[5]"))}
        if self.pinlockn(6) != 0 { try!(write!(f, " pinlockn[6]"))}
        if self.pinlockn(7) != 0 { try!(write!(f, " pinlockn[7]"))}
        if self.pinlockn(8) != 0 { try!(write!(f, " pinlockn[8]"))}
        if self.pinlockn(9) != 0 { try!(write!(f, " pinlockn[9]"))}
        if self.pinlockn(10) != 0 { try!(write!(f, " pinlockn[10]"))}
        if self.pinlockn(11) != 0 { try!(write!(f, " pinlockn[11]"))}
        if self.pinlockn(12) != 0 { try!(write!(f, " pinlockn[12]"))}
        if self.pinlockn(13) != 0 { try!(write!(f, " pinlockn[13]"))}
        if self.pinlockn(14) != 0 { try!(write!(f, " pinlockn[14]"))}
        if self.pinlockn(15) != 0 { try!(write!(f, " pinlockn[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Over Voltage Disable for all modes"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ovtdis(pub u32);
impl Ovtdis {
    #[doc="Disable Over Voltage capability"]
    #[inline] pub fn ovtdis<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVTDIS != 0"]
    #[inline] pub fn test_ovtdis<I: Into<bits::R16>>(&self, index: I) -> bool{
        self.ovtdis(index) != 0
    }

    #[doc="Sets the OVTDIS field."]
    #[inline] pub fn set_ovtdis<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ovtdis {
    #[inline]
    fn from(other: u32) -> Self {
         Ovtdis(other)
    }
}

impl ::core::fmt::Display for Ovtdis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ovtdis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovtdis(0) != 0 { try!(write!(f, " ovtdis[0]"))}
        if self.ovtdis(1) != 0 { try!(write!(f, " ovtdis[1]"))}
        if self.ovtdis(2) != 0 { try!(write!(f, " ovtdis[2]"))}
        if self.ovtdis(3) != 0 { try!(write!(f, " ovtdis[3]"))}
        if self.ovtdis(4) != 0 { try!(write!(f, " ovtdis[4]"))}
        if self.ovtdis(5) != 0 { try!(write!(f, " ovtdis[5]"))}
        if self.ovtdis(6) != 0 { try!(write!(f, " ovtdis[6]"))}
        if self.ovtdis(7) != 0 { try!(write!(f, " ovtdis[7]"))}
        if self.ovtdis(8) != 0 { try!(write!(f, " ovtdis[8]"))}
        if self.ovtdis(9) != 0 { try!(write!(f, " ovtdis[9]"))}
        if self.ovtdis(10) != 0 { try!(write!(f, " ovtdis[10]"))}
        if self.ovtdis(11) != 0 { try!(write!(f, " ovtdis[11]"))}
        if self.ovtdis(12) != 0 { try!(write!(f, " ovtdis[12]"))}
        if self.ovtdis(13) != 0 { try!(write!(f, " ovtdis[13]"))}
        if self.ovtdis(14) != 0 { try!(write!(f, " ovtdis[14]"))}
        if self.ovtdis(15) != 0 { try!(write!(f, " ovtdis[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


