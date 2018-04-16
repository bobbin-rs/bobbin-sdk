
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USART_F24 Peripheral"]
pub struct UsartPeriph(pub usize); 

impl UsartPeriph {
    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x0)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

    #[doc="Get the DR Register."]
    #[inline] pub fn dr_reg(&self) -> ::bobbin_mcu::register::Register<Dr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dr, 0x4)
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        self.dr_reg().read()
    }

    #[doc="Write the DR register."]
    #[inline] pub fn write_dr(&self, value: Dr) -> &Self { 
        self.dr_reg().write(value);
        self
    }

    #[doc="Set the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        self.dr_reg().set(f);
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        self.dr_reg().with(f);
        self
    }

    #[doc="Get the BRR Register."]
    #[inline] pub fn brr_reg(&self) -> ::bobbin_mcu::register::Register<Brr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Brr, 0x8)
    }

    #[doc="Get the *mut pointer for the BRR register."]
    #[inline] pub fn brr_mut(&self) -> *mut Brr { 
        self.brr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BRR register."]
    #[inline] pub fn brr_ptr(&self) -> *const Brr { 
        self.brr_reg().ptr()
    }

    #[doc="Read the BRR register."]
    #[inline] pub fn brr(&self) -> Brr { 
        self.brr_reg().read()
    }

    #[doc="Write the BRR register."]
    #[inline] pub fn write_brr(&self, value: Brr) -> &Self { 
        self.brr_reg().write(value);
        self
    }

    #[doc="Set the BRR register."]
    #[inline] pub fn set_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
        self.brr_reg().set(f);
        self
    }

    #[doc="Modify the BRR register."]
    #[inline] pub fn with_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
        self.brr_reg().with(f);
        self
    }

    #[doc="Get the CR1 Register."]
    #[inline] pub fn cr1_reg(&self) -> ::bobbin_mcu::register::Register<Cr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr1, 0xc)
    }

    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        self.cr1_reg().read()
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn write_cr1(&self, value: Cr1) -> &Self { 
        self.cr1_reg().write(value);
        self
    }

    #[doc="Set the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().set(f);
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().with(f);
        self
    }

    #[doc="Get the CR2 Register."]
    #[inline] pub fn cr2_reg(&self) -> ::bobbin_mcu::register::Register<Cr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr2, 0x10)
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        self.cr2_reg().read()
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn write_cr2(&self, value: Cr2) -> &Self { 
        self.cr2_reg().write(value);
        self
    }

    #[doc="Set the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().set(f);
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().with(f);
        self
    }

    #[doc="Get the CR3 Register."]
    #[inline] pub fn cr3_reg(&self) -> ::bobbin_mcu::register::Register<Cr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr3, 0x14)
    }

    #[doc="Get the *mut pointer for the CR3 register."]
    #[inline] pub fn cr3_mut(&self) -> *mut Cr3 { 
        self.cr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR3 register."]
    #[inline] pub fn cr3_ptr(&self) -> *const Cr3 { 
        self.cr3_reg().ptr()
    }

    #[doc="Read the CR3 register."]
    #[inline] pub fn cr3(&self) -> Cr3 { 
        self.cr3_reg().read()
    }

    #[doc="Write the CR3 register."]
    #[inline] pub fn write_cr3(&self, value: Cr3) -> &Self { 
        self.cr3_reg().write(value);
        self
    }

    #[doc="Set the CR3 register."]
    #[inline] pub fn set_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        self.cr3_reg().set(f);
        self
    }

    #[doc="Modify the CR3 register."]
    #[inline] pub fn with_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        self.cr3_reg().with(f);
        self
    }

    #[doc="Get the GTPR Register."]
    #[inline] pub fn gtpr_reg(&self) -> ::bobbin_mcu::register::Register<Gtpr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gtpr, 0x18)
    }

    #[doc="Get the *mut pointer for the GTPR register."]
    #[inline] pub fn gtpr_mut(&self) -> *mut Gtpr { 
        self.gtpr_reg().ptr()
    }

    #[doc="Get the *const pointer for the GTPR register."]
    #[inline] pub fn gtpr_ptr(&self) -> *const Gtpr { 
        self.gtpr_reg().ptr()
    }

    #[doc="Read the GTPR register."]
    #[inline] pub fn gtpr(&self) -> Gtpr { 
        self.gtpr_reg().read()
    }

    #[doc="Write the GTPR register."]
    #[inline] pub fn write_gtpr(&self, value: Gtpr) -> &Self { 
        self.gtpr_reg().write(value);
        self
    }

    #[doc="Set the GTPR register."]
    #[inline] pub fn set_gtpr<F: FnOnce(Gtpr) -> Gtpr>(&self, f: F) -> &Self {
        self.gtpr_reg().set(f);
        self
    }

    #[doc="Modify the GTPR register."]
    #[inline] pub fn with_gtpr<F: FnOnce(Gtpr) -> Gtpr>(&self, f: F) -> &Self {
        self.gtpr_reg().with(f);
        self
    }

}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="CTS flag"]
    #[inline] pub fn cts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTS != 0"]
    #[inline] pub fn test_cts(&self) -> bool {
        self.cts() != 0
    }

    #[doc="Sets the CTS field."]
    #[inline] pub fn set_cts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="LIN break detection flag"]
    #[inline] pub fn lbd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LBD != 0"]
    #[inline] pub fn test_lbd(&self) -> bool {
        self.lbd() != 0
    }

    #[doc="Sets the LBD field."]
    #[inline] pub fn set_lbd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmit data register empty"]
    #[inline] pub fn txe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmission complete"]
    #[inline] pub fn tc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC != 0"]
    #[inline] pub fn test_tc(&self) -> bool {
        self.tc() != 0
    }

    #[doc="Sets the TC field."]
    #[inline] pub fn set_tc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Read data register not empty"]
    #[inline] pub fn rxne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXNE != 0"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne() != 0
    }

    #[doc="Sets the RXNE field."]
    #[inline] pub fn set_rxne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IDLE line detected"]
    #[inline] pub fn idle(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLE != 0"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Sets the IDLE field."]
    #[inline] pub fn set_idle<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Overrun error"]
    #[inline] pub fn ore(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ORE != 0"]
    #[inline] pub fn test_ore(&self) -> bool {
        self.ore() != 0
    }

    #[doc="Sets the ORE field."]
    #[inline] pub fn set_ore<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Noise detected flag"]
    #[inline] pub fn nf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NF != 0"]
    #[inline] pub fn test_nf(&self) -> bool {
        self.nf() != 0
    }

    #[doc="Sets the NF field."]
    #[inline] pub fn set_nf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Framing error"]
    #[inline] pub fn fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FE != 0"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Sets the FE field."]
    #[inline] pub fn set_fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Parity error"]
    #[inline] pub fn pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cts() != 0 { try!(write!(f, " cts"))}
        if self.lbd() != 0 { try!(write!(f, " lbd"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.tc() != 0 { try!(write!(f, " tc"))}
        if self.rxne() != 0 { try!(write!(f, " rxne"))}
        if self.idle() != 0 { try!(write!(f, " idle"))}
        if self.ore() != 0 { try!(write!(f, " ore"))}
        if self.nf() != 0 { try!(write!(f, " nf"))}
        if self.fe() != 0 { try!(write!(f, " fe"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="Data value"]
    #[inline] pub fn dr(&self) -> ::bobbin_bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if DR != 0"]
    #[inline] pub fn test_dr(&self) -> bool {
        self.dr() != 0
    }

    #[doc="Sets the DR field."]
    #[inline] pub fn set_dr<V: Into<::bobbin_bits::U9>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dr {
    #[inline]
    fn from(other: u32) -> Self {
         Dr(other)
    }
}

impl ::core::fmt::Display for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dr() != 0 { try!(write!(f, " dr=0x{:x}", self.dr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Baud rate register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc="mantissa of USARTDIV"]
    #[inline] pub fn div_mantissa(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if DIV_Mantissa != 0"]
    #[inline] pub fn test_div_mantissa(&self) -> bool {
        self.div_mantissa() != 0
    }

    #[doc="Sets the DIV_Mantissa field."]
    #[inline] pub fn set_div_mantissa<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="fraction of USARTDIV"]
    #[inline] pub fn div_fraction(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DIV_Fraction != 0"]
    #[inline] pub fn test_div_fraction(&self) -> bool {
        self.div_fraction() != 0
    }

    #[doc="Sets the DIV_Fraction field."]
    #[inline] pub fn set_div_fraction<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Brr {
    #[inline]
    fn from(other: u32) -> Self {
         Brr(other)
    }
}

impl ::core::fmt::Display for Brr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Brr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.div_mantissa() != 0 { try!(write!(f, " div_mantissa=0x{:x}", self.div_mantissa()))}
        if self.div_fraction() != 0 { try!(write!(f, " div_fraction=0x{:x}", self.div_fraction()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Oversampling mode"]
    #[inline] pub fn over8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVER8 != 0"]
    #[inline] pub fn test_over8(&self) -> bool {
        self.over8() != 0
    }

    #[doc="Sets the OVER8 field."]
    #[inline] pub fn set_over8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="USART enable"]
    #[inline] pub fn ue(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if UE != 0"]
    #[inline] pub fn test_ue(&self) -> bool {
        self.ue() != 0
    }

    #[doc="Sets the UE field."]
    #[inline] pub fn set_ue<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Word length"]
    #[inline] pub fn m(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if M != 0"]
    #[inline] pub fn test_m(&self) -> bool {
        self.m() != 0
    }

    #[doc="Sets the M field."]
    #[inline] pub fn set_m<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Wakeup method"]
    #[inline] pub fn wake(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WAKE != 0"]
    #[inline] pub fn test_wake(&self) -> bool {
        self.wake() != 0
    }

    #[doc="Sets the WAKE field."]
    #[inline] pub fn set_wake<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Parity control enable"]
    #[inline] pub fn pce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PCE != 0"]
    #[inline] pub fn test_pce(&self) -> bool {
        self.pce() != 0
    }

    #[doc="Sets the PCE field."]
    #[inline] pub fn set_pce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Parity selection"]
    #[inline] pub fn ps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="PE interrupt enable"]
    #[inline] pub fn peie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEIE != 0"]
    #[inline] pub fn test_peie(&self) -> bool {
        self.peie() != 0
    }

    #[doc="Sets the PEIE field."]
    #[inline] pub fn set_peie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TXE interrupt enable"]
    #[inline] pub fn txeie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXEIE != 0"]
    #[inline] pub fn test_txeie(&self) -> bool {
        self.txeie() != 0
    }

    #[doc="Sets the TXEIE field."]
    #[inline] pub fn set_txeie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmission complete interrupt enable"]
    #[inline] pub fn tcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="RXNE interrupt enable"]
    #[inline] pub fn rxneie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXNEIE != 0"]
    #[inline] pub fn test_rxneie(&self) -> bool {
        self.rxneie() != 0
    }

    #[doc="Sets the RXNEIE field."]
    #[inline] pub fn set_rxneie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IDLE interrupt enable"]
    #[inline] pub fn idleie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLEIE != 0"]
    #[inline] pub fn test_idleie(&self) -> bool {
        self.idleie() != 0
    }

    #[doc="Sets the IDLEIE field."]
    #[inline] pub fn set_idleie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmitter enable"]
    #[inline] pub fn te(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TE != 0"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te() != 0
    }

    #[doc="Sets the TE field."]
    #[inline] pub fn set_te<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receiver enable"]
    #[inline] pub fn re(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RE != 0"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Sets the RE field."]
    #[inline] pub fn set_re<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver wakeup"]
    #[inline] pub fn rwu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RWU != 0"]
    #[inline] pub fn test_rwu(&self) -> bool {
        self.rwu() != 0
    }

    #[doc="Sets the RWU field."]
    #[inline] pub fn set_rwu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Send break"]
    #[inline] pub fn sbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SBK != 0"]
    #[inline] pub fn test_sbk(&self) -> bool {
        self.sbk() != 0
    }

    #[doc="Sets the SBK field."]
    #[inline] pub fn set_sbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
    }
}

impl ::core::fmt::Display for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.over8() != 0 { try!(write!(f, " over8"))}
        if self.ue() != 0 { try!(write!(f, " ue"))}
        if self.m() != 0 { try!(write!(f, " m"))}
        if self.wake() != 0 { try!(write!(f, " wake"))}
        if self.pce() != 0 { try!(write!(f, " pce"))}
        if self.ps() != 0 { try!(write!(f, " ps"))}
        if self.peie() != 0 { try!(write!(f, " peie"))}
        if self.txeie() != 0 { try!(write!(f, " txeie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
        if self.idleie() != 0 { try!(write!(f, " idleie"))}
        if self.te() != 0 { try!(write!(f, " te"))}
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.rwu() != 0 { try!(write!(f, " rwu"))}
        if self.sbk() != 0 { try!(write!(f, " sbk"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="LIN mode enable"]
    #[inline] pub fn linen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LINEN != 0"]
    #[inline] pub fn test_linen(&self) -> bool {
        self.linen() != 0
    }

    #[doc="Sets the LINEN field."]
    #[inline] pub fn set_linen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="STOP bits"]
    #[inline] pub fn stop(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clock enable"]
    #[inline] pub fn clken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CLKEN != 0"]
    #[inline] pub fn test_clken(&self) -> bool {
        self.clken() != 0
    }

    #[doc="Sets the CLKEN field."]
    #[inline] pub fn set_clken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clock polarity"]
    #[inline] pub fn cpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clock phase"]
    #[inline] pub fn cpha(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Last bit clock pulse"]
    #[inline] pub fn lbcl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LBCL != 0"]
    #[inline] pub fn test_lbcl(&self) -> bool {
        self.lbcl() != 0
    }

    #[doc="Sets the LBCL field."]
    #[inline] pub fn set_lbcl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LIN break detection interrupt enable"]
    #[inline] pub fn lbdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LBDIE != 0"]
    #[inline] pub fn test_lbdie(&self) -> bool {
        self.lbdie() != 0
    }

    #[doc="Sets the LBDIE field."]
    #[inline] pub fn set_lbdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="lin break detection length"]
    #[inline] pub fn lbdl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LBDL != 0"]
    #[inline] pub fn test_lbdl(&self) -> bool {
        self.lbdl() != 0
    }

    #[doc="Sets the LBDL field."]
    #[inline] pub fn set_lbdl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Address of the USART node"]
    #[inline] pub fn add(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADD != 0"]
    #[inline] pub fn test_add(&self) -> bool {
        self.add() != 0
    }

    #[doc="Sets the ADD field."]
    #[inline] pub fn set_add<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr2(other)
    }
}

impl ::core::fmt::Display for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.linen() != 0 { try!(write!(f, " linen"))}
        if self.stop() != 0 { try!(write!(f, " stop=0x{:x}", self.stop()))}
        if self.clken() != 0 { try!(write!(f, " clken"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        if self.lbcl() != 0 { try!(write!(f, " lbcl"))}
        if self.lbdie() != 0 { try!(write!(f, " lbdie"))}
        if self.lbdl() != 0 { try!(write!(f, " lbdl"))}
        if self.add() != 0 { try!(write!(f, " add=0x{:x}", self.add()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc="One sample bit method enable"]
    #[inline] pub fn onebit(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ONEBIT != 0"]
    #[inline] pub fn test_onebit(&self) -> bool {
        self.onebit() != 0
    }

    #[doc="Sets the ONEBIT field."]
    #[inline] pub fn set_onebit<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CTS interrupt enable"]
    #[inline] pub fn ctsie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTSIE != 0"]
    #[inline] pub fn test_ctsie(&self) -> bool {
        self.ctsie() != 0
    }

    #[doc="Sets the CTSIE field."]
    #[inline] pub fn set_ctsie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CTS enable"]
    #[inline] pub fn ctse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTSE != 0"]
    #[inline] pub fn test_ctse(&self) -> bool {
        self.ctse() != 0
    }

    #[doc="Sets the CTSE field."]
    #[inline] pub fn set_ctse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="RTS enable"]
    #[inline] pub fn rtse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RTSE != 0"]
    #[inline] pub fn test_rtse(&self) -> bool {
        self.rtse() != 0
    }

    #[doc="Sets the RTSE field."]
    #[inline] pub fn set_rtse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA enable transmitter"]
    #[inline] pub fn dmat(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DMAT != 0"]
    #[inline] pub fn test_dmat(&self) -> bool {
        self.dmat() != 0
    }

    #[doc="Sets the DMAT field."]
    #[inline] pub fn set_dmat<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DMA enable receiver"]
    #[inline] pub fn dmar(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DMAR != 0"]
    #[inline] pub fn test_dmar(&self) -> bool {
        self.dmar() != 0
    }

    #[doc="Sets the DMAR field."]
    #[inline] pub fn set_dmar<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Smartcard mode enable"]
    #[inline] pub fn scen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCEN != 0"]
    #[inline] pub fn test_scen(&self) -> bool {
        self.scen() != 0
    }

    #[doc="Sets the SCEN field."]
    #[inline] pub fn set_scen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Smartcard NACK enable"]
    #[inline] pub fn nack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACK != 0"]
    #[inline] pub fn test_nack(&self) -> bool {
        self.nack() != 0
    }

    #[doc="Sets the NACK field."]
    #[inline] pub fn set_nack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Half-duplex selection"]
    #[inline] pub fn hdsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HDSEL != 0"]
    #[inline] pub fn test_hdsel(&self) -> bool {
        self.hdsel() != 0
    }

    #[doc="Sets the HDSEL field."]
    #[inline] pub fn set_hdsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IrDA low-power"]
    #[inline] pub fn irlp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IRLP != 0"]
    #[inline] pub fn test_irlp(&self) -> bool {
        self.irlp() != 0
    }

    #[doc="Sets the IRLP field."]
    #[inline] pub fn set_irlp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IrDA mode enable"]
    #[inline] pub fn iren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn eie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EIE != 0"]
    #[inline] pub fn test_eie(&self) -> bool {
        self.eie() != 0
    }

    #[doc="Sets the EIE field."]
    #[inline] pub fn set_eie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr3(other)
    }
}

impl ::core::fmt::Display for Cr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.onebit() != 0 { try!(write!(f, " onebit"))}
        if self.ctsie() != 0 { try!(write!(f, " ctsie"))}
        if self.ctse() != 0 { try!(write!(f, " ctse"))}
        if self.rtse() != 0 { try!(write!(f, " rtse"))}
        if self.dmat() != 0 { try!(write!(f, " dmat"))}
        if self.dmar() != 0 { try!(write!(f, " dmar"))}
        if self.scen() != 0 { try!(write!(f, " scen"))}
        if self.nack() != 0 { try!(write!(f, " nack"))}
        if self.hdsel() != 0 { try!(write!(f, " hdsel"))}
        if self.irlp() != 0 { try!(write!(f, " irlp"))}
        if self.iren() != 0 { try!(write!(f, " iren"))}
        if self.eie() != 0 { try!(write!(f, " eie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Guard time and prescaler register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gtpr(pub u32);
impl Gtpr {
    #[doc="Guard time value"]
    #[inline] pub fn gt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GT != 0"]
    #[inline] pub fn test_gt(&self) -> bool {
        self.gt() != 0
    }

    #[doc="Sets the GT field."]
    #[inline] pub fn set_gt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Prescaler value"]
    #[inline] pub fn psc(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PSC != 0"]
    #[inline] pub fn test_psc(&self) -> bool {
        self.psc() != 0
    }

    #[doc="Sets the PSC field."]
    #[inline] pub fn set_psc<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gtpr {
    #[inline]
    fn from(other: u32) -> Self {
         Gtpr(other)
    }
}

impl ::core::fmt::Display for Gtpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gtpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gt() != 0 { try!(write!(f, " gt=0x{:x}", self.gt()))}
        if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

