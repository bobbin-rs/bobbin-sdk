//! Nested Vectored Interrupt Controller
#[allow(unused_imports)] use bobbin_common::*;

periph!(NVIC, Nvic, 0xe000e000);

#[doc="Nested Vectored Interrupt Controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Nvic(pub usize);
impl Nvic {
    #[doc="Get the *mut pointer for the ISER register."]
    #[inline] pub fn iser_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Iser { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x100 + (index << 2)) as *mut Iser
    }

    #[doc="Get the *const pointer for the ISER register."]
    #[inline] pub fn iser_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Iser { 
           self.iser_mut(index)
    }

    #[doc="Read the ISER register."]
    #[inline] pub fn iser<I: Into<bits::R8>>(&self, index: I) -> Iser { 
        unsafe {
            read_volatile(self.iser_ptr(index))
        }
    }

    #[doc="Write the ISER register."]
    #[inline] pub fn set_iser<I: Into<bits::R8>, F: FnOnce(Iser) -> Iser>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.iser_mut(index), f(Iser(0)));
        }
        self
    }

    #[doc="Modify the ISER register."]
    #[inline] pub fn with_iser<I: Into<bits::R8> + Copy, F: FnOnce(Iser) -> Iser>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.iser_mut(index), f(self.iser(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICER register."]
    #[inline] pub fn icer_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Icer { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x180 + (index << 2)) as *mut Icer
    }

    #[doc="Get the *const pointer for the ICER register."]
    #[inline] pub fn icer_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Icer { 
           self.icer_mut(index)
    }

    #[doc="Read the ICER register."]
    #[inline] pub fn icer<I: Into<bits::R8>>(&self, index: I) -> Icer { 
        unsafe {
            read_volatile(self.icer_ptr(index))
        }
    }

    #[doc="Write the ICER register."]
    #[inline] pub fn set_icer<I: Into<bits::R8>, F: FnOnce(Icer) -> Icer>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.icer_mut(index), f(Icer(0)));
        }
        self
    }

    #[doc="Modify the ICER register."]
    #[inline] pub fn with_icer<I: Into<bits::R8> + Copy, F: FnOnce(Icer) -> Icer>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.icer_mut(index), f(self.icer(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISPR register."]
    #[inline] pub fn ispr_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Ispr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x200 + (index << 2)) as *mut Ispr
    }

    #[doc="Get the *const pointer for the ISPR register."]
    #[inline] pub fn ispr_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Ispr { 
           self.ispr_mut(index)
    }

    #[doc="Read the ISPR register."]
    #[inline] pub fn ispr<I: Into<bits::R8>>(&self, index: I) -> Ispr { 
        unsafe {
            read_volatile(self.ispr_ptr(index))
        }
    }

    #[doc="Write the ISPR register."]
    #[inline] pub fn set_ispr<I: Into<bits::R8>, F: FnOnce(Ispr) -> Ispr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ispr_mut(index), f(Ispr(0)));
        }
        self
    }

    #[doc="Modify the ISPR register."]
    #[inline] pub fn with_ispr<I: Into<bits::R8> + Copy, F: FnOnce(Ispr) -> Ispr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ispr_mut(index), f(self.ispr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICPR register."]
    #[inline] pub fn icpr_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Icpr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x280 + (index << 2)) as *mut Icpr
    }

    #[doc="Get the *const pointer for the ICPR register."]
    #[inline] pub fn icpr_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Icpr { 
           self.icpr_mut(index)
    }

    #[doc="Read the ICPR register."]
    #[inline] pub fn icpr<I: Into<bits::R8>>(&self, index: I) -> Icpr { 
        unsafe {
            read_volatile(self.icpr_ptr(index))
        }
    }

    #[doc="Write the ICPR register."]
    #[inline] pub fn set_icpr<I: Into<bits::R8>, F: FnOnce(Icpr) -> Icpr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.icpr_mut(index), f(Icpr(0)));
        }
        self
    }

    #[doc="Modify the ICPR register."]
    #[inline] pub fn with_icpr<I: Into<bits::R8> + Copy, F: FnOnce(Icpr) -> Icpr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.icpr_mut(index), f(self.icpr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IABR register."]
    #[inline] pub fn iabr_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Iabr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x280 + (index << 2)) as *mut Iabr
    }

    #[doc="Get the *const pointer for the IABR register."]
    #[inline] pub fn iabr_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Iabr { 
           self.iabr_mut(index)
    }

    #[doc="Read the IABR register."]
    #[inline] pub fn iabr<I: Into<bits::R8>>(&self, index: I) -> Iabr { 
        unsafe {
            read_volatile(self.iabr_ptr(index))
        }
    }

    #[doc="Write the IABR register."]
    #[inline] pub fn set_iabr<I: Into<bits::R8>, F: FnOnce(Iabr) -> Iabr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.iabr_mut(index), f(Iabr(0)));
        }
        self
    }

    #[doc="Modify the IABR register."]
    #[inline] pub fn with_iabr<I: Into<bits::R8> + Copy, F: FnOnce(Iabr) -> Iabr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.iabr_mut(index), f(self.iabr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR register."]
    #[inline] pub fn ipr_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Ipr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x400 + (index << 2)) as *mut Ipr
    }

    #[doc="Get the *const pointer for the IPR register."]
    #[inline] pub fn ipr_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Ipr { 
           self.ipr_mut(index)
    }

    #[doc="Read the IPR register."]
    #[inline] pub fn ipr<I: Into<bits::R8>>(&self, index: I) -> Ipr { 
        unsafe {
            read_volatile(self.ipr_ptr(index))
        }
    }

    #[doc="Write the IPR register."]
    #[inline] pub fn set_ipr<I: Into<bits::R8>, F: FnOnce(Ipr) -> Ipr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr_mut(index), f(Ipr(0)));
        }
        self
    }

    #[doc="Modify the IPR register."]
    #[inline] pub fn with_ipr<I: Into<bits::R8> + Copy, F: FnOnce(Ipr) -> Ipr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr_mut(index), f(self.ipr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the STIR register."]
    #[inline] pub fn stir_mut(&self) -> *mut Stir { 
        (self.0 + 0xf00) as *mut Stir
    }

    #[doc="Get the *const pointer for the STIR register."]
    #[inline] pub fn stir_ptr(&self) -> *const Stir { 
           self.stir_mut()
    }

    #[doc="Read the STIR register."]
    #[inline] pub fn stir(&self) -> Stir { 
        unsafe {
            read_volatile(self.stir_ptr())
        }
    }

    #[doc="Write the STIR register."]
    #[inline] pub fn set_stir<F: FnOnce(Stir) -> Stir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stir_mut(), f(Stir(0)));
        }
        self
    }

    #[doc="Modify the STIR register."]
    #[inline] pub fn with_stir<F: FnOnce(Stir) -> Stir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stir_mut(), f(self.stir()));
        }
        self
    }

}

#[doc="Interrupt Set-Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iser(pub u32);
impl Iser {
    #[doc="Interrupt set-enable bits"]
    #[inline] pub fn setena<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Interrupt set-enable bits"]
    #[inline] pub fn test_setena<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.setena(index) != 0
    }

    #[doc="Interrupt set-enable bits"]
    #[inline] pub fn set_setena<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Iser {
    #[inline]
    fn from(other: u32) -> Self {
         Iser(other)
    }
}

impl ::core::fmt::Display for Iser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.setena(0) != 0 { try!(write!(f, " setena[0]"))}
        if self.setena(1) != 0 { try!(write!(f, " setena[1]"))}
        if self.setena(2) != 0 { try!(write!(f, " setena[2]"))}
        if self.setena(3) != 0 { try!(write!(f, " setena[3]"))}
        if self.setena(4) != 0 { try!(write!(f, " setena[4]"))}
        if self.setena(5) != 0 { try!(write!(f, " setena[5]"))}
        if self.setena(6) != 0 { try!(write!(f, " setena[6]"))}
        if self.setena(7) != 0 { try!(write!(f, " setena[7]"))}
        if self.setena(8) != 0 { try!(write!(f, " setena[8]"))}
        if self.setena(9) != 0 { try!(write!(f, " setena[9]"))}
        if self.setena(10) != 0 { try!(write!(f, " setena[10]"))}
        if self.setena(11) != 0 { try!(write!(f, " setena[11]"))}
        if self.setena(12) != 0 { try!(write!(f, " setena[12]"))}
        if self.setena(13) != 0 { try!(write!(f, " setena[13]"))}
        if self.setena(14) != 0 { try!(write!(f, " setena[14]"))}
        if self.setena(15) != 0 { try!(write!(f, " setena[15]"))}
        if self.setena(16) != 0 { try!(write!(f, " setena[16]"))}
        if self.setena(17) != 0 { try!(write!(f, " setena[17]"))}
        if self.setena(18) != 0 { try!(write!(f, " setena[18]"))}
        if self.setena(19) != 0 { try!(write!(f, " setena[19]"))}
        if self.setena(20) != 0 { try!(write!(f, " setena[20]"))}
        if self.setena(21) != 0 { try!(write!(f, " setena[21]"))}
        if self.setena(22) != 0 { try!(write!(f, " setena[22]"))}
        if self.setena(23) != 0 { try!(write!(f, " setena[23]"))}
        if self.setena(24) != 0 { try!(write!(f, " setena[24]"))}
        if self.setena(25) != 0 { try!(write!(f, " setena[25]"))}
        if self.setena(26) != 0 { try!(write!(f, " setena[26]"))}
        if self.setena(27) != 0 { try!(write!(f, " setena[27]"))}
        if self.setena(28) != 0 { try!(write!(f, " setena[28]"))}
        if self.setena(29) != 0 { try!(write!(f, " setena[29]"))}
        if self.setena(30) != 0 { try!(write!(f, " setena[30]"))}
        if self.setena(31) != 0 { try!(write!(f, " setena[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear-Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icer(pub u32);
impl Icer {
    #[doc="Interrupt clear-enable bits"]
    #[inline] pub fn clrena<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Interrupt clear-enable bits"]
    #[inline] pub fn test_clrena<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.clrena(index) != 0
    }

    #[doc="Interrupt clear-enable bits"]
    #[inline] pub fn set_clrena<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Icer {
    #[inline]
    fn from(other: u32) -> Self {
         Icer(other)
    }
}

impl ::core::fmt::Display for Icer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrena(0) != 0 { try!(write!(f, " clrena[0]"))}
        if self.clrena(1) != 0 { try!(write!(f, " clrena[1]"))}
        if self.clrena(2) != 0 { try!(write!(f, " clrena[2]"))}
        if self.clrena(3) != 0 { try!(write!(f, " clrena[3]"))}
        if self.clrena(4) != 0 { try!(write!(f, " clrena[4]"))}
        if self.clrena(5) != 0 { try!(write!(f, " clrena[5]"))}
        if self.clrena(6) != 0 { try!(write!(f, " clrena[6]"))}
        if self.clrena(7) != 0 { try!(write!(f, " clrena[7]"))}
        if self.clrena(8) != 0 { try!(write!(f, " clrena[8]"))}
        if self.clrena(9) != 0 { try!(write!(f, " clrena[9]"))}
        if self.clrena(10) != 0 { try!(write!(f, " clrena[10]"))}
        if self.clrena(11) != 0 { try!(write!(f, " clrena[11]"))}
        if self.clrena(12) != 0 { try!(write!(f, " clrena[12]"))}
        if self.clrena(13) != 0 { try!(write!(f, " clrena[13]"))}
        if self.clrena(14) != 0 { try!(write!(f, " clrena[14]"))}
        if self.clrena(15) != 0 { try!(write!(f, " clrena[15]"))}
        if self.clrena(16) != 0 { try!(write!(f, " clrena[16]"))}
        if self.clrena(17) != 0 { try!(write!(f, " clrena[17]"))}
        if self.clrena(18) != 0 { try!(write!(f, " clrena[18]"))}
        if self.clrena(19) != 0 { try!(write!(f, " clrena[19]"))}
        if self.clrena(20) != 0 { try!(write!(f, " clrena[20]"))}
        if self.clrena(21) != 0 { try!(write!(f, " clrena[21]"))}
        if self.clrena(22) != 0 { try!(write!(f, " clrena[22]"))}
        if self.clrena(23) != 0 { try!(write!(f, " clrena[23]"))}
        if self.clrena(24) != 0 { try!(write!(f, " clrena[24]"))}
        if self.clrena(25) != 0 { try!(write!(f, " clrena[25]"))}
        if self.clrena(26) != 0 { try!(write!(f, " clrena[26]"))}
        if self.clrena(27) != 0 { try!(write!(f, " clrena[27]"))}
        if self.clrena(28) != 0 { try!(write!(f, " clrena[28]"))}
        if self.clrena(29) != 0 { try!(write!(f, " clrena[29]"))}
        if self.clrena(30) != 0 { try!(write!(f, " clrena[30]"))}
        if self.clrena(31) != 0 { try!(write!(f, " clrena[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Set-Pending Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ispr(pub u32);
impl Ispr {
    #[doc="Interrupt set-pending bits"]
    #[inline] pub fn setpend<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Interrupt set-pending bits"]
    #[inline] pub fn test_setpend<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.setpend(index) != 0
    }

    #[doc="Interrupt set-pending bits"]
    #[inline] pub fn set_setpend<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ispr {
    #[inline]
    fn from(other: u32) -> Self {
         Ispr(other)
    }
}

impl ::core::fmt::Display for Ispr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ispr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.setpend(0) != 0 { try!(write!(f, " setpend[0]"))}
        if self.setpend(1) != 0 { try!(write!(f, " setpend[1]"))}
        if self.setpend(2) != 0 { try!(write!(f, " setpend[2]"))}
        if self.setpend(3) != 0 { try!(write!(f, " setpend[3]"))}
        if self.setpend(4) != 0 { try!(write!(f, " setpend[4]"))}
        if self.setpend(5) != 0 { try!(write!(f, " setpend[5]"))}
        if self.setpend(6) != 0 { try!(write!(f, " setpend[6]"))}
        if self.setpend(7) != 0 { try!(write!(f, " setpend[7]"))}
        if self.setpend(8) != 0 { try!(write!(f, " setpend[8]"))}
        if self.setpend(9) != 0 { try!(write!(f, " setpend[9]"))}
        if self.setpend(10) != 0 { try!(write!(f, " setpend[10]"))}
        if self.setpend(11) != 0 { try!(write!(f, " setpend[11]"))}
        if self.setpend(12) != 0 { try!(write!(f, " setpend[12]"))}
        if self.setpend(13) != 0 { try!(write!(f, " setpend[13]"))}
        if self.setpend(14) != 0 { try!(write!(f, " setpend[14]"))}
        if self.setpend(15) != 0 { try!(write!(f, " setpend[15]"))}
        if self.setpend(16) != 0 { try!(write!(f, " setpend[16]"))}
        if self.setpend(17) != 0 { try!(write!(f, " setpend[17]"))}
        if self.setpend(18) != 0 { try!(write!(f, " setpend[18]"))}
        if self.setpend(19) != 0 { try!(write!(f, " setpend[19]"))}
        if self.setpend(20) != 0 { try!(write!(f, " setpend[20]"))}
        if self.setpend(21) != 0 { try!(write!(f, " setpend[21]"))}
        if self.setpend(22) != 0 { try!(write!(f, " setpend[22]"))}
        if self.setpend(23) != 0 { try!(write!(f, " setpend[23]"))}
        if self.setpend(24) != 0 { try!(write!(f, " setpend[24]"))}
        if self.setpend(25) != 0 { try!(write!(f, " setpend[25]"))}
        if self.setpend(26) != 0 { try!(write!(f, " setpend[26]"))}
        if self.setpend(27) != 0 { try!(write!(f, " setpend[27]"))}
        if self.setpend(28) != 0 { try!(write!(f, " setpend[28]"))}
        if self.setpend(29) != 0 { try!(write!(f, " setpend[29]"))}
        if self.setpend(30) != 0 { try!(write!(f, " setpend[30]"))}
        if self.setpend(31) != 0 { try!(write!(f, " setpend[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear-Pending Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icpr(pub u32);
impl Icpr {
    #[doc="Interrupt clear-pending bits"]
    #[inline] pub fn clrpend<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Interrupt clear-pending bits"]
    #[inline] pub fn test_clrpend<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.clrpend(index) != 0
    }

    #[doc="Interrupt clear-pending bits"]
    #[inline] pub fn set_clrpend<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Icpr {
    #[inline]
    fn from(other: u32) -> Self {
         Icpr(other)
    }
}

impl ::core::fmt::Display for Icpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrpend(0) != 0 { try!(write!(f, " clrpend[0]"))}
        if self.clrpend(1) != 0 { try!(write!(f, " clrpend[1]"))}
        if self.clrpend(2) != 0 { try!(write!(f, " clrpend[2]"))}
        if self.clrpend(3) != 0 { try!(write!(f, " clrpend[3]"))}
        if self.clrpend(4) != 0 { try!(write!(f, " clrpend[4]"))}
        if self.clrpend(5) != 0 { try!(write!(f, " clrpend[5]"))}
        if self.clrpend(6) != 0 { try!(write!(f, " clrpend[6]"))}
        if self.clrpend(7) != 0 { try!(write!(f, " clrpend[7]"))}
        if self.clrpend(8) != 0 { try!(write!(f, " clrpend[8]"))}
        if self.clrpend(9) != 0 { try!(write!(f, " clrpend[9]"))}
        if self.clrpend(10) != 0 { try!(write!(f, " clrpend[10]"))}
        if self.clrpend(11) != 0 { try!(write!(f, " clrpend[11]"))}
        if self.clrpend(12) != 0 { try!(write!(f, " clrpend[12]"))}
        if self.clrpend(13) != 0 { try!(write!(f, " clrpend[13]"))}
        if self.clrpend(14) != 0 { try!(write!(f, " clrpend[14]"))}
        if self.clrpend(15) != 0 { try!(write!(f, " clrpend[15]"))}
        if self.clrpend(16) != 0 { try!(write!(f, " clrpend[16]"))}
        if self.clrpend(17) != 0 { try!(write!(f, " clrpend[17]"))}
        if self.clrpend(18) != 0 { try!(write!(f, " clrpend[18]"))}
        if self.clrpend(19) != 0 { try!(write!(f, " clrpend[19]"))}
        if self.clrpend(20) != 0 { try!(write!(f, " clrpend[20]"))}
        if self.clrpend(21) != 0 { try!(write!(f, " clrpend[21]"))}
        if self.clrpend(22) != 0 { try!(write!(f, " clrpend[22]"))}
        if self.clrpend(23) != 0 { try!(write!(f, " clrpend[23]"))}
        if self.clrpend(24) != 0 { try!(write!(f, " clrpend[24]"))}
        if self.clrpend(25) != 0 { try!(write!(f, " clrpend[25]"))}
        if self.clrpend(26) != 0 { try!(write!(f, " clrpend[26]"))}
        if self.clrpend(27) != 0 { try!(write!(f, " clrpend[27]"))}
        if self.clrpend(28) != 0 { try!(write!(f, " clrpend[28]"))}
        if self.clrpend(29) != 0 { try!(write!(f, " clrpend[29]"))}
        if self.clrpend(30) != 0 { try!(write!(f, " clrpend[30]"))}
        if self.clrpend(31) != 0 { try!(write!(f, " clrpend[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Active Bit Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iabr(pub u32);
impl Iabr {
    #[doc="Interrupt clear-pending bits"]
    #[inline] pub fn active<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Interrupt clear-pending bits"]
    #[inline] pub fn test_active<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.active(index) != 0
    }

    #[doc="Interrupt clear-pending bits"]
    #[inline] pub fn set_active<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Iabr {
    #[inline]
    fn from(other: u32) -> Self {
         Iabr(other)
    }
}

impl ::core::fmt::Display for Iabr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iabr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.active(0) != 0 { try!(write!(f, " active[0]"))}
        if self.active(1) != 0 { try!(write!(f, " active[1]"))}
        if self.active(2) != 0 { try!(write!(f, " active[2]"))}
        if self.active(3) != 0 { try!(write!(f, " active[3]"))}
        if self.active(4) != 0 { try!(write!(f, " active[4]"))}
        if self.active(5) != 0 { try!(write!(f, " active[5]"))}
        if self.active(6) != 0 { try!(write!(f, " active[6]"))}
        if self.active(7) != 0 { try!(write!(f, " active[7]"))}
        if self.active(8) != 0 { try!(write!(f, " active[8]"))}
        if self.active(9) != 0 { try!(write!(f, " active[9]"))}
        if self.active(10) != 0 { try!(write!(f, " active[10]"))}
        if self.active(11) != 0 { try!(write!(f, " active[11]"))}
        if self.active(12) != 0 { try!(write!(f, " active[12]"))}
        if self.active(13) != 0 { try!(write!(f, " active[13]"))}
        if self.active(14) != 0 { try!(write!(f, " active[14]"))}
        if self.active(15) != 0 { try!(write!(f, " active[15]"))}
        if self.active(16) != 0 { try!(write!(f, " active[16]"))}
        if self.active(17) != 0 { try!(write!(f, " active[17]"))}
        if self.active(18) != 0 { try!(write!(f, " active[18]"))}
        if self.active(19) != 0 { try!(write!(f, " active[19]"))}
        if self.active(20) != 0 { try!(write!(f, " active[20]"))}
        if self.active(21) != 0 { try!(write!(f, " active[21]"))}
        if self.active(22) != 0 { try!(write!(f, " active[22]"))}
        if self.active(23) != 0 { try!(write!(f, " active[23]"))}
        if self.active(24) != 0 { try!(write!(f, " active[24]"))}
        if self.active(25) != 0 { try!(write!(f, " active[25]"))}
        if self.active(26) != 0 { try!(write!(f, " active[26]"))}
        if self.active(27) != 0 { try!(write!(f, " active[27]"))}
        if self.active(28) != 0 { try!(write!(f, " active[28]"))}
        if self.active(29) != 0 { try!(write!(f, " active[29]"))}
        if self.active(30) != 0 { try!(write!(f, " active[30]"))}
        if self.active(31) != 0 { try!(write!(f, " active[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register x"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr(pub u32);
impl Ipr {
    #[doc="Interrupt Priority"]
    #[inline] pub fn pri<I: Into<bits::R4>>(&self, index: I) -> bits::U8 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xff) as u8) } // [7:0]
    }

    #[doc="Interrupt Priority"]
    #[inline] pub fn test_pri<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.pri(index) != 0
    }

    #[doc="Interrupt Priority"]
    #[inline] pub fn set_pri<I: Into<bits::R4>, V: Into<bits::U8>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0xff << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ipr {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr(other)
    }
}

impl ::core::fmt::Display for Ipr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pri(0) != 0 { try!(write!(f, " pri[0]=0x{:x}", self.pri(0)))}
        if self.pri(1) != 0 { try!(write!(f, " pri[1]=0x{:x}", self.pri(1)))}
        if self.pri(2) != 0 { try!(write!(f, " pri[2]=0x{:x}", self.pri(2)))}
        if self.pri(3) != 0 { try!(write!(f, " pri[3]=0x{:x}", self.pri(3)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software Trigger Interrupt Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stir(pub u32);
impl Stir {
    #[doc="Interrupt ID of the interrupt to trigger, in the range 0-239."]
    #[inline] pub fn intid(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Interrupt ID of the interrupt to trigger, in the range 0-239."]
    #[inline] pub fn test_intid(&self) -> bool {
        self.intid() != 0
    }

    #[doc="Interrupt ID of the interrupt to trigger, in the range 0-239."]
    #[inline] pub fn set_intid<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stir {
    #[inline]
    fn from(other: u32) -> Self {
         Stir(other)
    }
}

impl ::core::fmt::Display for Stir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.intid() != 0 { try!(write!(f, " intid=0x{:x}", self.intid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


