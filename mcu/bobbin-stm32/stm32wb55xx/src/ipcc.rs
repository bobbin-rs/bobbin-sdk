::bobbin_mcu::periph!( IPCC, Ipcc, IPCC_PERIPH, IpccPeriph, IPCC_OWNED, IPCC_REF_COUNT, 0x58000c00, 0x00, 0x2d);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="IPCC Peripheral"]
pub struct IpccPeriph(pub usize); 

impl IpccPeriph {
    #[doc="Get the C1CR Register."]
    #[inline] pub fn c1cr_reg(&self) -> ::bobbin_mcu::register::Register<C1cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1cr, 0x0)
    }

    #[doc="Get the *mut pointer for the C1CR register."]
    #[inline] pub fn c1cr_mut(&self) -> *mut C1cr { 
        self.c1cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1CR register."]
    #[inline] pub fn c1cr_ptr(&self) -> *const C1cr { 
        self.c1cr_reg().ptr()
    }

    #[doc="Read the C1CR register."]
    #[inline] pub fn c1cr(&self) -> C1cr { 
        self.c1cr_reg().read()
    }

    #[doc="Write the C1CR register."]
    #[inline] pub fn write_c1cr(&self, value: C1cr) -> &Self { 
        self.c1cr_reg().write(value);
        self
    }

    #[doc="Set the C1CR register."]
    #[inline] pub fn set_c1cr<F: FnOnce(C1cr) -> C1cr>(&self, f: F) -> &Self {
        self.c1cr_reg().set(f);
        self
    }

    #[doc="Modify the C1CR register."]
    #[inline] pub fn with_c1cr<F: FnOnce(C1cr) -> C1cr>(&self, f: F) -> &Self {
        self.c1cr_reg().with(f);
        self
    }

    #[doc="Get the C1MR Register."]
    #[inline] pub fn c1mr_reg(&self) -> ::bobbin_mcu::register::Register<C1mr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1mr, 0x4)
    }

    #[doc="Get the *mut pointer for the C1MR register."]
    #[inline] pub fn c1mr_mut(&self) -> *mut C1mr { 
        self.c1mr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1MR register."]
    #[inline] pub fn c1mr_ptr(&self) -> *const C1mr { 
        self.c1mr_reg().ptr()
    }

    #[doc="Read the C1MR register."]
    #[inline] pub fn c1mr(&self) -> C1mr { 
        self.c1mr_reg().read()
    }

    #[doc="Write the C1MR register."]
    #[inline] pub fn write_c1mr(&self, value: C1mr) -> &Self { 
        self.c1mr_reg().write(value);
        self
    }

    #[doc="Set the C1MR register."]
    #[inline] pub fn set_c1mr<F: FnOnce(C1mr) -> C1mr>(&self, f: F) -> &Self {
        self.c1mr_reg().set(f);
        self
    }

    #[doc="Modify the C1MR register."]
    #[inline] pub fn with_c1mr<F: FnOnce(C1mr) -> C1mr>(&self, f: F) -> &Self {
        self.c1mr_reg().with(f);
        self
    }

    #[doc="Get the C1SCR Register."]
    #[inline] pub fn c1scr_reg(&self) -> ::bobbin_mcu::register::Register<C1scr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1scr, 0x8)
    }

    #[doc="Get the *mut pointer for the C1SCR register."]
    #[inline] pub fn c1scr_mut(&self) -> *mut C1scr { 
        self.c1scr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1SCR register."]
    #[inline] pub fn c1scr_ptr(&self) -> *const C1scr { 
        self.c1scr_reg().ptr()
    }

    #[doc="Write the C1SCR register."]
    #[inline] pub fn write_c1scr(&self, value: C1scr) -> &Self { 
        self.c1scr_reg().write(value);
        self
    }

    #[doc="Set the C1SCR register."]
    #[inline] pub fn set_c1scr<F: FnOnce(C1scr) -> C1scr>(&self, f: F) -> &Self {
        self.c1scr_reg().set(f);
        self
    }

    #[doc="Get the C1TO2SR Register."]
    #[inline] pub fn c1to2sr_reg(&self) -> ::bobbin_mcu::register::Register<C1to2sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1to2sr, 0xc)
    }

    #[doc="Get the *mut pointer for the C1TO2SR register."]
    #[inline] pub fn c1to2sr_mut(&self) -> *mut C1to2sr { 
        self.c1to2sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1TO2SR register."]
    #[inline] pub fn c1to2sr_ptr(&self) -> *const C1to2sr { 
        self.c1to2sr_reg().ptr()
    }

    #[doc="Read the C1TO2SR register."]
    #[inline] pub fn c1to2sr(&self) -> C1to2sr { 
        self.c1to2sr_reg().read()
    }

    #[doc="Get the C2CR Register."]
    #[inline] pub fn c2cr_reg(&self) -> ::bobbin_mcu::register::Register<C2cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2cr, 0x10)
    }

    #[doc="Get the *mut pointer for the C2CR register."]
    #[inline] pub fn c2cr_mut(&self) -> *mut C2cr { 
        self.c2cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2CR register."]
    #[inline] pub fn c2cr_ptr(&self) -> *const C2cr { 
        self.c2cr_reg().ptr()
    }

    #[doc="Read the C2CR register."]
    #[inline] pub fn c2cr(&self) -> C2cr { 
        self.c2cr_reg().read()
    }

    #[doc="Write the C2CR register."]
    #[inline] pub fn write_c2cr(&self, value: C2cr) -> &Self { 
        self.c2cr_reg().write(value);
        self
    }

    #[doc="Set the C2CR register."]
    #[inline] pub fn set_c2cr<F: FnOnce(C2cr) -> C2cr>(&self, f: F) -> &Self {
        self.c2cr_reg().set(f);
        self
    }

    #[doc="Modify the C2CR register."]
    #[inline] pub fn with_c2cr<F: FnOnce(C2cr) -> C2cr>(&self, f: F) -> &Self {
        self.c2cr_reg().with(f);
        self
    }

    #[doc="Get the C2MR Register."]
    #[inline] pub fn c2mr_reg(&self) -> ::bobbin_mcu::register::Register<C2mr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2mr, 0x14)
    }

    #[doc="Get the *mut pointer for the C2MR register."]
    #[inline] pub fn c2mr_mut(&self) -> *mut C2mr { 
        self.c2mr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2MR register."]
    #[inline] pub fn c2mr_ptr(&self) -> *const C2mr { 
        self.c2mr_reg().ptr()
    }

    #[doc="Read the C2MR register."]
    #[inline] pub fn c2mr(&self) -> C2mr { 
        self.c2mr_reg().read()
    }

    #[doc="Write the C2MR register."]
    #[inline] pub fn write_c2mr(&self, value: C2mr) -> &Self { 
        self.c2mr_reg().write(value);
        self
    }

    #[doc="Set the C2MR register."]
    #[inline] pub fn set_c2mr<F: FnOnce(C2mr) -> C2mr>(&self, f: F) -> &Self {
        self.c2mr_reg().set(f);
        self
    }

    #[doc="Modify the C2MR register."]
    #[inline] pub fn with_c2mr<F: FnOnce(C2mr) -> C2mr>(&self, f: F) -> &Self {
        self.c2mr_reg().with(f);
        self
    }

    #[doc="Get the C2SCR Register."]
    #[inline] pub fn c2scr_reg(&self) -> ::bobbin_mcu::register::Register<C2scr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2scr, 0x18)
    }

    #[doc="Get the *mut pointer for the C2SCR register."]
    #[inline] pub fn c2scr_mut(&self) -> *mut C2scr { 
        self.c2scr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2SCR register."]
    #[inline] pub fn c2scr_ptr(&self) -> *const C2scr { 
        self.c2scr_reg().ptr()
    }

    #[doc="Write the C2SCR register."]
    #[inline] pub fn write_c2scr(&self, value: C2scr) -> &Self { 
        self.c2scr_reg().write(value);
        self
    }

    #[doc="Set the C2SCR register."]
    #[inline] pub fn set_c2scr<F: FnOnce(C2scr) -> C2scr>(&self, f: F) -> &Self {
        self.c2scr_reg().set(f);
        self
    }

    #[doc="Get the C2TOC1SR Register."]
    #[inline] pub fn c2toc1sr_reg(&self) -> ::bobbin_mcu::register::Register<C2toc1sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2toc1sr, 0x1c)
    }

    #[doc="Get the *mut pointer for the C2TOC1SR register."]
    #[inline] pub fn c2toc1sr_mut(&self) -> *mut C2toc1sr { 
        self.c2toc1sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2TOC1SR register."]
    #[inline] pub fn c2toc1sr_ptr(&self) -> *const C2toc1sr { 
        self.c2toc1sr_reg().ptr()
    }

    #[doc="Read the C2TOC1SR register."]
    #[inline] pub fn c2toc1sr(&self) -> C2toc1sr { 
        self.c2toc1sr_reg().read()
    }

    #[doc="Get the HWCFGR Register."]
    #[inline] pub fn hwcfgr_reg(&self) -> ::bobbin_mcu::register::Register<Hwcfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hwcfgr, 0x3f0)
    }

    #[doc="Get the *mut pointer for the HWCFGR register."]
    #[inline] pub fn hwcfgr_mut(&self) -> *mut Hwcfgr { 
        self.hwcfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the HWCFGR register."]
    #[inline] pub fn hwcfgr_ptr(&self) -> *const Hwcfgr { 
        self.hwcfgr_reg().ptr()
    }

    #[doc="Read the HWCFGR register."]
    #[inline] pub fn hwcfgr(&self) -> Hwcfgr { 
        self.hwcfgr_reg().read()
    }

    #[doc="Get the VERR Register."]
    #[inline] pub fn verr_reg(&self) -> ::bobbin_mcu::register::Register<Verr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Verr, 0x3f4)
    }

    #[doc="Get the *mut pointer for the VERR register."]
    #[inline] pub fn verr_mut(&self) -> *mut Verr { 
        self.verr_reg().ptr()
    }

    #[doc="Get the *const pointer for the VERR register."]
    #[inline] pub fn verr_ptr(&self) -> *const Verr { 
        self.verr_reg().ptr()
    }

    #[doc="Read the VERR register."]
    #[inline] pub fn verr(&self) -> Verr { 
        self.verr_reg().read()
    }

    #[doc="Get the IPIDR Register."]
    #[inline] pub fn ipidr_reg(&self) -> ::bobbin_mcu::register::Register<Ipidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ipidr, 0x3f8)
    }

    #[doc="Get the *mut pointer for the IPIDR register."]
    #[inline] pub fn ipidr_mut(&self) -> *mut Ipidr { 
        self.ipidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IPIDR register."]
    #[inline] pub fn ipidr_ptr(&self) -> *const Ipidr { 
        self.ipidr_reg().ptr()
    }

    #[doc="Read the IPIDR register."]
    #[inline] pub fn ipidr(&self) -> Ipidr { 
        self.ipidr_reg().read()
    }

    #[doc="Get the SIDR Register."]
    #[inline] pub fn sidr_reg(&self) -> ::bobbin_mcu::register::Register<Sidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sidr, 0x3fc)
    }

    #[doc="Get the *mut pointer for the SIDR register."]
    #[inline] pub fn sidr_mut(&self) -> *mut Sidr { 
        self.sidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SIDR register."]
    #[inline] pub fn sidr_ptr(&self) -> *const Sidr { 
        self.sidr_reg().ptr()
    }

    #[doc="Read the SIDR register."]
    #[inline] pub fn sidr(&self) -> Sidr { 
        self.sidr_reg().read()
    }

}

#[doc="Control register CPU1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1cr(pub u32);
impl C1cr {
    #[doc="processor 1 Transmit channel free interrupt enable"]
    #[inline] pub fn txfie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TXFIE != 0"]
    #[inline] pub fn test_txfie(&self) -> bool {
        self.txfie() != 0
    }

    #[doc="Sets the TXFIE field."]
    #[inline] pub fn set_txfie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="processor 1 Receive channel occupied interrupt enable"]
    #[inline] pub fn rxoie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXOIE != 0"]
    #[inline] pub fn test_rxoie(&self) -> bool {
        self.rxoie() != 0
    }

    #[doc="Sets the RXOIE field."]
    #[inline] pub fn set_rxoie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1cr {
    #[inline]
    fn from(other: u32) -> Self {
         C1cr(other)
    }
}

impl ::core::fmt::Display for C1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfie() != 0 { try!(write!(f, " txfie"))}
        if self.rxoie() != 0 { try!(write!(f, " rxoie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Mask register CPU1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1mr(pub u32);
impl C1mr {
    #[doc="processor 1 Transmit channel 6 free interrupt mask"]
    #[inline] pub fn ch6fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CH6FM != 0"]
    #[inline] pub fn test_ch6fm(&self) -> bool {
        self.ch6fm() != 0
    }

    #[doc="Sets the CH6FM field."]
    #[inline] pub fn set_ch6fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="processor 1 Transmit channel 5 free interrupt mask"]
    #[inline] pub fn ch5fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CH5FM != 0"]
    #[inline] pub fn test_ch5fm(&self) -> bool {
        self.ch5fm() != 0
    }

    #[doc="Sets the CH5FM field."]
    #[inline] pub fn set_ch5fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="processor 1 Transmit channel 4 free interrupt mask"]
    #[inline] pub fn ch4fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CH4FM != 0"]
    #[inline] pub fn test_ch4fm(&self) -> bool {
        self.ch4fm() != 0
    }

    #[doc="Sets the CH4FM field."]
    #[inline] pub fn set_ch4fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="processor 1 Transmit channel 3 free interrupt mask"]
    #[inline] pub fn ch3fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CH3FM != 0"]
    #[inline] pub fn test_ch3fm(&self) -> bool {
        self.ch3fm() != 0
    }

    #[doc="Sets the CH3FM field."]
    #[inline] pub fn set_ch3fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="processor 1 Transmit channel 2 free interrupt mask"]
    #[inline] pub fn ch2fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CH2FM != 0"]
    #[inline] pub fn test_ch2fm(&self) -> bool {
        self.ch2fm() != 0
    }

    #[doc="Sets the CH2FM field."]
    #[inline] pub fn set_ch2fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="processor 1 Transmit channel 1 free interrupt mask"]
    #[inline] pub fn ch1fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CH1FM != 0"]
    #[inline] pub fn test_ch1fm(&self) -> bool {
        self.ch1fm() != 0
    }

    #[doc="Sets the CH1FM field."]
    #[inline] pub fn set_ch1fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="processor 1 Receive channel 6 occupied interrupt enable"]
    #[inline] pub fn ch6om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH6OM != 0"]
    #[inline] pub fn test_ch6om(&self) -> bool {
        self.ch6om() != 0
    }

    #[doc="Sets the CH6OM field."]
    #[inline] pub fn set_ch6om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="processor 1 Receive channel 5 occupied interrupt enable"]
    #[inline] pub fn ch5om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH5OM != 0"]
    #[inline] pub fn test_ch5om(&self) -> bool {
        self.ch5om() != 0
    }

    #[doc="Sets the CH5OM field."]
    #[inline] pub fn set_ch5om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="processor 1 Receive channel 4 occupied interrupt enable"]
    #[inline] pub fn ch4om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH4OM != 0"]
    #[inline] pub fn test_ch4om(&self) -> bool {
        self.ch4om() != 0
    }

    #[doc="Sets the CH4OM field."]
    #[inline] pub fn set_ch4om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="processor 1 Receive channel 3 occupied interrupt enable"]
    #[inline] pub fn ch3om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH3OM != 0"]
    #[inline] pub fn test_ch3om(&self) -> bool {
        self.ch3om() != 0
    }

    #[doc="Sets the CH3OM field."]
    #[inline] pub fn set_ch3om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="processor 1 Receive channel 2 occupied interrupt enable"]
    #[inline] pub fn ch2om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH2OM != 0"]
    #[inline] pub fn test_ch2om(&self) -> bool {
        self.ch2om() != 0
    }

    #[doc="Sets the CH2OM field."]
    #[inline] pub fn set_ch2om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="processor 1 Receive channel 1 occupied interrupt enable"]
    #[inline] pub fn ch1om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH1OM != 0"]
    #[inline] pub fn test_ch1om(&self) -> bool {
        self.ch1om() != 0
    }

    #[doc="Sets the CH1OM field."]
    #[inline] pub fn set_ch1om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1mr {
    #[inline]
    fn from(other: u32) -> Self {
         C1mr(other)
    }
}

impl ::core::fmt::Display for C1mr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1mr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch6fm() != 0 { try!(write!(f, " ch6fm"))}
        if self.ch5fm() != 0 { try!(write!(f, " ch5fm"))}
        if self.ch4fm() != 0 { try!(write!(f, " ch4fm"))}
        if self.ch3fm() != 0 { try!(write!(f, " ch3fm"))}
        if self.ch2fm() != 0 { try!(write!(f, " ch2fm"))}
        if self.ch1fm() != 0 { try!(write!(f, " ch1fm"))}
        if self.ch6om() != 0 { try!(write!(f, " ch6om"))}
        if self.ch5om() != 0 { try!(write!(f, " ch5om"))}
        if self.ch4om() != 0 { try!(write!(f, " ch4om"))}
        if self.ch3om() != 0 { try!(write!(f, " ch3om"))}
        if self.ch2om() != 0 { try!(write!(f, " ch2om"))}
        if self.ch1om() != 0 { try!(write!(f, " ch1om"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Set or Clear register CPU1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1scr(pub u32);
impl C1scr {
    #[doc="processor 1 Transmit channel 6 status set"]
    #[inline] pub fn ch6s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CH6S != 0"]
    #[inline] pub fn test_ch6s(&self) -> bool {
        self.ch6s() != 0
    }

    #[doc="Sets the CH6S field."]
    #[inline] pub fn set_ch6s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="processor 1 Transmit channel 5 status set"]
    #[inline] pub fn ch5s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CH5S != 0"]
    #[inline] pub fn test_ch5s(&self) -> bool {
        self.ch5s() != 0
    }

    #[doc="Sets the CH5S field."]
    #[inline] pub fn set_ch5s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="processor 1 Transmit channel 4 status set"]
    #[inline] pub fn ch4s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CH4S != 0"]
    #[inline] pub fn test_ch4s(&self) -> bool {
        self.ch4s() != 0
    }

    #[doc="Sets the CH4S field."]
    #[inline] pub fn set_ch4s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="processor 1 Transmit channel 3 status set"]
    #[inline] pub fn ch3s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CH3S != 0"]
    #[inline] pub fn test_ch3s(&self) -> bool {
        self.ch3s() != 0
    }

    #[doc="Sets the CH3S field."]
    #[inline] pub fn set_ch3s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="processor 1 Transmit channel 2 status set"]
    #[inline] pub fn ch2s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CH2S != 0"]
    #[inline] pub fn test_ch2s(&self) -> bool {
        self.ch2s() != 0
    }

    #[doc="Sets the CH2S field."]
    #[inline] pub fn set_ch2s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="processor 1 Transmit channel 1 status set"]
    #[inline] pub fn ch1s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CH1S != 0"]
    #[inline] pub fn test_ch1s(&self) -> bool {
        self.ch1s() != 0
    }

    #[doc="Sets the CH1S field."]
    #[inline] pub fn set_ch1s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="processor 1 Receive channel 6 status clear"]
    #[inline] pub fn ch6c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH6C != 0"]
    #[inline] pub fn test_ch6c(&self) -> bool {
        self.ch6c() != 0
    }

    #[doc="Sets the CH6C field."]
    #[inline] pub fn set_ch6c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="processor 1 Receive channel 5 status clear"]
    #[inline] pub fn ch5c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH5C != 0"]
    #[inline] pub fn test_ch5c(&self) -> bool {
        self.ch5c() != 0
    }

    #[doc="Sets the CH5C field."]
    #[inline] pub fn set_ch5c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="processor 1 Receive channel 4 status clear"]
    #[inline] pub fn ch4c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH4C != 0"]
    #[inline] pub fn test_ch4c(&self) -> bool {
        self.ch4c() != 0
    }

    #[doc="Sets the CH4C field."]
    #[inline] pub fn set_ch4c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="processor 1 Receive channel 3 status clear"]
    #[inline] pub fn ch3c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH3C != 0"]
    #[inline] pub fn test_ch3c(&self) -> bool {
        self.ch3c() != 0
    }

    #[doc="Sets the CH3C field."]
    #[inline] pub fn set_ch3c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="processor 1 Receive channel 2 status clear"]
    #[inline] pub fn ch2c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH2C != 0"]
    #[inline] pub fn test_ch2c(&self) -> bool {
        self.ch2c() != 0
    }

    #[doc="Sets the CH2C field."]
    #[inline] pub fn set_ch2c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="processor 1 Receive channel 1 status clear"]
    #[inline] pub fn ch1c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH1C != 0"]
    #[inline] pub fn test_ch1c(&self) -> bool {
        self.ch1c() != 0
    }

    #[doc="Sets the CH1C field."]
    #[inline] pub fn set_ch1c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1scr {
    #[inline]
    fn from(other: u32) -> Self {
         C1scr(other)
    }
}

impl ::core::fmt::Display for C1scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch6s() != 0 { try!(write!(f, " ch6s"))}
        if self.ch5s() != 0 { try!(write!(f, " ch5s"))}
        if self.ch4s() != 0 { try!(write!(f, " ch4s"))}
        if self.ch3s() != 0 { try!(write!(f, " ch3s"))}
        if self.ch2s() != 0 { try!(write!(f, " ch2s"))}
        if self.ch1s() != 0 { try!(write!(f, " ch1s"))}
        if self.ch6c() != 0 { try!(write!(f, " ch6c"))}
        if self.ch5c() != 0 { try!(write!(f, " ch5c"))}
        if self.ch4c() != 0 { try!(write!(f, " ch4c"))}
        if self.ch3c() != 0 { try!(write!(f, " ch3c"))}
        if self.ch2c() != 0 { try!(write!(f, " ch2c"))}
        if self.ch1c() != 0 { try!(write!(f, " ch1c"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU1 to CPU2 status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1to2sr(pub u32);
impl C1to2sr {
    #[doc="processor 1 transmit to process 2 Receive channel 6 status flag"]
    #[inline] pub fn ch6f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH6F != 0"]
    #[inline] pub fn test_ch6f(&self) -> bool {
        self.ch6f() != 0
    }

    #[doc="Sets the CH6F field."]
    #[inline] pub fn set_ch6f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="processor 1 transmit to process 2 Receive channel 5 status flag"]
    #[inline] pub fn ch5f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH5F != 0"]
    #[inline] pub fn test_ch5f(&self) -> bool {
        self.ch5f() != 0
    }

    #[doc="Sets the CH5F field."]
    #[inline] pub fn set_ch5f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="processor 1 transmit to process 2 Receive channel 4 status flag"]
    #[inline] pub fn ch4f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH4F != 0"]
    #[inline] pub fn test_ch4f(&self) -> bool {
        self.ch4f() != 0
    }

    #[doc="Sets the CH4F field."]
    #[inline] pub fn set_ch4f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="processor 1 transmit to process 2 Receive channel 3 status flag"]
    #[inline] pub fn ch3f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH3F != 0"]
    #[inline] pub fn test_ch3f(&self) -> bool {
        self.ch3f() != 0
    }

    #[doc="Sets the CH3F field."]
    #[inline] pub fn set_ch3f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="processor 1 transmit to process 2 Receive channel 2 status flag"]
    #[inline] pub fn ch2f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH2F != 0"]
    #[inline] pub fn test_ch2f(&self) -> bool {
        self.ch2f() != 0
    }

    #[doc="Sets the CH2F field."]
    #[inline] pub fn set_ch2f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="processor 1 transmit to process 2 Receive channel 1 status flag"]
    #[inline] pub fn ch1f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH1F != 0"]
    #[inline] pub fn test_ch1f(&self) -> bool {
        self.ch1f() != 0
    }

    #[doc="Sets the CH1F field."]
    #[inline] pub fn set_ch1f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1to2sr {
    #[inline]
    fn from(other: u32) -> Self {
         C1to2sr(other)
    }
}

impl ::core::fmt::Display for C1to2sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1to2sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch6f() != 0 { try!(write!(f, " ch6f"))}
        if self.ch5f() != 0 { try!(write!(f, " ch5f"))}
        if self.ch4f() != 0 { try!(write!(f, " ch4f"))}
        if self.ch3f() != 0 { try!(write!(f, " ch3f"))}
        if self.ch2f() != 0 { try!(write!(f, " ch2f"))}
        if self.ch1f() != 0 { try!(write!(f, " ch1f"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register CPU2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2cr(pub u32);
impl C2cr {
    #[doc="processor 2 Transmit channel free interrupt enable"]
    #[inline] pub fn txfie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TXFIE != 0"]
    #[inline] pub fn test_txfie(&self) -> bool {
        self.txfie() != 0
    }

    #[doc="Sets the TXFIE field."]
    #[inline] pub fn set_txfie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="processor 2 Receive channel occupied interrupt enable"]
    #[inline] pub fn rxoie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXOIE != 0"]
    #[inline] pub fn test_rxoie(&self) -> bool {
        self.rxoie() != 0
    }

    #[doc="Sets the RXOIE field."]
    #[inline] pub fn set_rxoie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2cr {
    #[inline]
    fn from(other: u32) -> Self {
         C2cr(other)
    }
}

impl ::core::fmt::Display for C2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfie() != 0 { try!(write!(f, " txfie"))}
        if self.rxoie() != 0 { try!(write!(f, " rxoie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Mask register CPU2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2mr(pub u32);
impl C2mr {
    #[doc="processor 2 Transmit channel 6 free interrupt mask"]
    #[inline] pub fn ch6fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CH6FM != 0"]
    #[inline] pub fn test_ch6fm(&self) -> bool {
        self.ch6fm() != 0
    }

    #[doc="Sets the CH6FM field."]
    #[inline] pub fn set_ch6fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="processor 2 Transmit channel 5 free interrupt mask"]
    #[inline] pub fn ch5fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CH5FM != 0"]
    #[inline] pub fn test_ch5fm(&self) -> bool {
        self.ch5fm() != 0
    }

    #[doc="Sets the CH5FM field."]
    #[inline] pub fn set_ch5fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="processor 2 Transmit channel 4 free interrupt mask"]
    #[inline] pub fn ch4fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CH4FM != 0"]
    #[inline] pub fn test_ch4fm(&self) -> bool {
        self.ch4fm() != 0
    }

    #[doc="Sets the CH4FM field."]
    #[inline] pub fn set_ch4fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="processor 2 Transmit channel 3 free interrupt mask"]
    #[inline] pub fn ch3fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CH3FM != 0"]
    #[inline] pub fn test_ch3fm(&self) -> bool {
        self.ch3fm() != 0
    }

    #[doc="Sets the CH3FM field."]
    #[inline] pub fn set_ch3fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="processor 2 Transmit channel 2 free interrupt mask"]
    #[inline] pub fn ch2fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CH2FM != 0"]
    #[inline] pub fn test_ch2fm(&self) -> bool {
        self.ch2fm() != 0
    }

    #[doc="Sets the CH2FM field."]
    #[inline] pub fn set_ch2fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="processor 2 Transmit channel 1 free interrupt mask"]
    #[inline] pub fn ch1fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CH1FM != 0"]
    #[inline] pub fn test_ch1fm(&self) -> bool {
        self.ch1fm() != 0
    }

    #[doc="Sets the CH1FM field."]
    #[inline] pub fn set_ch1fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="processor 2 Receive channel 6 occupied interrupt enable"]
    #[inline] pub fn ch6om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH6OM != 0"]
    #[inline] pub fn test_ch6om(&self) -> bool {
        self.ch6om() != 0
    }

    #[doc="Sets the CH6OM field."]
    #[inline] pub fn set_ch6om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="processor 2 Receive channel 5 occupied interrupt enable"]
    #[inline] pub fn ch5om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH5OM != 0"]
    #[inline] pub fn test_ch5om(&self) -> bool {
        self.ch5om() != 0
    }

    #[doc="Sets the CH5OM field."]
    #[inline] pub fn set_ch5om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="processor 2 Receive channel 4 occupied interrupt enable"]
    #[inline] pub fn ch4om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH4OM != 0"]
    #[inline] pub fn test_ch4om(&self) -> bool {
        self.ch4om() != 0
    }

    #[doc="Sets the CH4OM field."]
    #[inline] pub fn set_ch4om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="processor 2 Receive channel 3 occupied interrupt enable"]
    #[inline] pub fn ch3om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH3OM != 0"]
    #[inline] pub fn test_ch3om(&self) -> bool {
        self.ch3om() != 0
    }

    #[doc="Sets the CH3OM field."]
    #[inline] pub fn set_ch3om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="processor 2 Receive channel 2 occupied interrupt enable"]
    #[inline] pub fn ch2om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH2OM != 0"]
    #[inline] pub fn test_ch2om(&self) -> bool {
        self.ch2om() != 0
    }

    #[doc="Sets the CH2OM field."]
    #[inline] pub fn set_ch2om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="processor 2 Receive channel 1 occupied interrupt enable"]
    #[inline] pub fn ch1om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH1OM != 0"]
    #[inline] pub fn test_ch1om(&self) -> bool {
        self.ch1om() != 0
    }

    #[doc="Sets the CH1OM field."]
    #[inline] pub fn set_ch1om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2mr {
    #[inline]
    fn from(other: u32) -> Self {
         C2mr(other)
    }
}

impl ::core::fmt::Display for C2mr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2mr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch6fm() != 0 { try!(write!(f, " ch6fm"))}
        if self.ch5fm() != 0 { try!(write!(f, " ch5fm"))}
        if self.ch4fm() != 0 { try!(write!(f, " ch4fm"))}
        if self.ch3fm() != 0 { try!(write!(f, " ch3fm"))}
        if self.ch2fm() != 0 { try!(write!(f, " ch2fm"))}
        if self.ch1fm() != 0 { try!(write!(f, " ch1fm"))}
        if self.ch6om() != 0 { try!(write!(f, " ch6om"))}
        if self.ch5om() != 0 { try!(write!(f, " ch5om"))}
        if self.ch4om() != 0 { try!(write!(f, " ch4om"))}
        if self.ch3om() != 0 { try!(write!(f, " ch3om"))}
        if self.ch2om() != 0 { try!(write!(f, " ch2om"))}
        if self.ch1om() != 0 { try!(write!(f, " ch1om"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Set or Clear register CPU2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2scr(pub u32);
impl C2scr {
    #[doc="processor 2 Transmit channel 6 status set"]
    #[inline] pub fn ch6s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CH6S != 0"]
    #[inline] pub fn test_ch6s(&self) -> bool {
        self.ch6s() != 0
    }

    #[doc="Sets the CH6S field."]
    #[inline] pub fn set_ch6s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="processor 2 Transmit channel 5 status set"]
    #[inline] pub fn ch5s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CH5S != 0"]
    #[inline] pub fn test_ch5s(&self) -> bool {
        self.ch5s() != 0
    }

    #[doc="Sets the CH5S field."]
    #[inline] pub fn set_ch5s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="processor 2 Transmit channel 4 status set"]
    #[inline] pub fn ch4s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CH4S != 0"]
    #[inline] pub fn test_ch4s(&self) -> bool {
        self.ch4s() != 0
    }

    #[doc="Sets the CH4S field."]
    #[inline] pub fn set_ch4s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="processor 2 Transmit channel 3 status set"]
    #[inline] pub fn ch3s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CH3S != 0"]
    #[inline] pub fn test_ch3s(&self) -> bool {
        self.ch3s() != 0
    }

    #[doc="Sets the CH3S field."]
    #[inline] pub fn set_ch3s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="processor 2 Transmit channel 2 status set"]
    #[inline] pub fn ch2s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CH2S != 0"]
    #[inline] pub fn test_ch2s(&self) -> bool {
        self.ch2s() != 0
    }

    #[doc="Sets the CH2S field."]
    #[inline] pub fn set_ch2s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="processor 2 Transmit channel 1 status set"]
    #[inline] pub fn ch1s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CH1S != 0"]
    #[inline] pub fn test_ch1s(&self) -> bool {
        self.ch1s() != 0
    }

    #[doc="Sets the CH1S field."]
    #[inline] pub fn set_ch1s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="processor 2 Receive channel 6 status clear"]
    #[inline] pub fn ch6c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH6C != 0"]
    #[inline] pub fn test_ch6c(&self) -> bool {
        self.ch6c() != 0
    }

    #[doc="Sets the CH6C field."]
    #[inline] pub fn set_ch6c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="processor 2 Receive channel 5 status clear"]
    #[inline] pub fn ch5c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH5C != 0"]
    #[inline] pub fn test_ch5c(&self) -> bool {
        self.ch5c() != 0
    }

    #[doc="Sets the CH5C field."]
    #[inline] pub fn set_ch5c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="processor 2 Receive channel 4 status clear"]
    #[inline] pub fn ch4c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH4C != 0"]
    #[inline] pub fn test_ch4c(&self) -> bool {
        self.ch4c() != 0
    }

    #[doc="Sets the CH4C field."]
    #[inline] pub fn set_ch4c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="processor 2 Receive channel 3 status clear"]
    #[inline] pub fn ch3c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH3C != 0"]
    #[inline] pub fn test_ch3c(&self) -> bool {
        self.ch3c() != 0
    }

    #[doc="Sets the CH3C field."]
    #[inline] pub fn set_ch3c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="processor 2 Receive channel 2 status clear"]
    #[inline] pub fn ch2c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH2C != 0"]
    #[inline] pub fn test_ch2c(&self) -> bool {
        self.ch2c() != 0
    }

    #[doc="Sets the CH2C field."]
    #[inline] pub fn set_ch2c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="processor 2 Receive channel 1 status clear"]
    #[inline] pub fn ch1c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH1C != 0"]
    #[inline] pub fn test_ch1c(&self) -> bool {
        self.ch1c() != 0
    }

    #[doc="Sets the CH1C field."]
    #[inline] pub fn set_ch1c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2scr {
    #[inline]
    fn from(other: u32) -> Self {
         C2scr(other)
    }
}

impl ::core::fmt::Display for C2scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch6s() != 0 { try!(write!(f, " ch6s"))}
        if self.ch5s() != 0 { try!(write!(f, " ch5s"))}
        if self.ch4s() != 0 { try!(write!(f, " ch4s"))}
        if self.ch3s() != 0 { try!(write!(f, " ch3s"))}
        if self.ch2s() != 0 { try!(write!(f, " ch2s"))}
        if self.ch1s() != 0 { try!(write!(f, " ch1s"))}
        if self.ch6c() != 0 { try!(write!(f, " ch6c"))}
        if self.ch5c() != 0 { try!(write!(f, " ch5c"))}
        if self.ch4c() != 0 { try!(write!(f, " ch4c"))}
        if self.ch3c() != 0 { try!(write!(f, " ch3c"))}
        if self.ch2c() != 0 { try!(write!(f, " ch2c"))}
        if self.ch1c() != 0 { try!(write!(f, " ch1c"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 to CPU1 status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2toc1sr(pub u32);
impl C2toc1sr {
    #[doc="processor 2 transmit to process 1 Receive channel 6 status flag"]
    #[inline] pub fn ch6f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH6F != 0"]
    #[inline] pub fn test_ch6f(&self) -> bool {
        self.ch6f() != 0
    }

    #[doc="Sets the CH6F field."]
    #[inline] pub fn set_ch6f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="processor 2 transmit to process 1 Receive channel 5 status flag"]
    #[inline] pub fn ch5f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH5F != 0"]
    #[inline] pub fn test_ch5f(&self) -> bool {
        self.ch5f() != 0
    }

    #[doc="Sets the CH5F field."]
    #[inline] pub fn set_ch5f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="processor 2 transmit to process 1 Receive channel 4 status flag"]
    #[inline] pub fn ch4f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH4F != 0"]
    #[inline] pub fn test_ch4f(&self) -> bool {
        self.ch4f() != 0
    }

    #[doc="Sets the CH4F field."]
    #[inline] pub fn set_ch4f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="processor 2 transmit to process 1 Receive channel 3 status flag"]
    #[inline] pub fn ch3f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH3F != 0"]
    #[inline] pub fn test_ch3f(&self) -> bool {
        self.ch3f() != 0
    }

    #[doc="Sets the CH3F field."]
    #[inline] pub fn set_ch3f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="processor 2 transmit to process 1 Receive channel 2 status flag"]
    #[inline] pub fn ch2f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH2F != 0"]
    #[inline] pub fn test_ch2f(&self) -> bool {
        self.ch2f() != 0
    }

    #[doc="Sets the CH2F field."]
    #[inline] pub fn set_ch2f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="processor 2 transmit to process 1 Receive channel 1 status flag"]
    #[inline] pub fn ch1f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH1F != 0"]
    #[inline] pub fn test_ch1f(&self) -> bool {
        self.ch1f() != 0
    }

    #[doc="Sets the CH1F field."]
    #[inline] pub fn set_ch1f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2toc1sr {
    #[inline]
    fn from(other: u32) -> Self {
         C2toc1sr(other)
    }
}

impl ::core::fmt::Display for C2toc1sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2toc1sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch6f() != 0 { try!(write!(f, " ch6f"))}
        if self.ch5f() != 0 { try!(write!(f, " ch5f"))}
        if self.ch4f() != 0 { try!(write!(f, " ch4f"))}
        if self.ch3f() != 0 { try!(write!(f, " ch3f"))}
        if self.ch2f() != 0 { try!(write!(f, " ch2f"))}
        if self.ch1f() != 0 { try!(write!(f, " ch1f"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IPCC Hardware configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hwcfgr(pub u32);
impl Hwcfgr {
    #[doc="Number of channels per CPU supported by the IP, range 1 to 16"]
    #[inline] pub fn channels(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CHANNELS != 0"]
    #[inline] pub fn test_channels(&self) -> bool {
        self.channels() != 0
    }

    #[doc="Sets the CHANNELS field."]
    #[inline] pub fn set_channels<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hwcfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Hwcfgr(other)
    }
}

impl ::core::fmt::Display for Hwcfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hwcfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.channels() != 0 { try!(write!(f, " channels=0x{:x}", self.channels()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IPCC version register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verr(pub u32);
impl Verr {
    #[doc="Major Revision"]
    #[inline] pub fn majrev(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if MAJREV != 0"]
    #[inline] pub fn test_majrev(&self) -> bool {
        self.majrev() != 0
    }

    #[doc="Sets the MAJREV field."]
    #[inline] pub fn set_majrev<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Minor Revision"]
    #[inline] pub fn minrev(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MINREV != 0"]
    #[inline] pub fn test_minrev(&self) -> bool {
        self.minrev() != 0
    }

    #[doc="Sets the MINREV field."]
    #[inline] pub fn set_minrev<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Verr {
    #[inline]
    fn from(other: u32) -> Self {
         Verr(other)
    }
}

impl ::core::fmt::Display for Verr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.majrev() != 0 { try!(write!(f, " majrev=0x{:x}", self.majrev()))}
        if self.minrev() != 0 { try!(write!(f, " minrev=0x{:x}", self.minrev()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IPCC indentification register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipidr(pub u32);
impl Ipidr {
    #[doc="Identification Code"]
    #[inline] pub fn ipid(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if IPID != 0"]
    #[inline] pub fn test_ipid(&self) -> bool {
        self.ipid() != 0
    }

    #[doc="Sets the IPID field."]
    #[inline] pub fn set_ipid<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ipidr {
    #[inline]
    fn from(other: u32) -> Self {
         Ipidr(other)
    }
}

impl ::core::fmt::Display for Ipidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IPCC size indentification register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sidr(pub u32);
impl Sidr {
    #[doc="Size Identification Code"]
    #[inline] pub fn sid(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SID != 0"]
    #[inline] pub fn test_sid(&self) -> bool {
        self.sid() != 0
    }

    #[doc="Sets the SID field."]
    #[inline] pub fn set_sid<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sidr {
    #[inline]
    fn from(other: u32) -> Self {
         Sidr(other)
    }
}

impl ::core::fmt::Display for Sidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

