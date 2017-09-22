#[allow(unused_imports)] use bobbin_common::*;

periph!( LEUART0, Leuart0, _LEUART0, LeuartPeriph, 0x4004a000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LEUART Peripheral"]
pub struct LeuartPeriph(pub usize); 



impl LeuartPeriph {
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

    #[doc="Get the *mut pointer for the CMD register."]
    #[inline] pub fn cmd_mut(&self) -> *mut Cmd { 
        (self.0 + 0x4) as *mut Cmd
    }

    #[doc="Get the *const pointer for the CMD register."]
    #[inline] pub fn cmd_ptr(&self) -> *const Cmd { 
           self.cmd_mut()
    }

    #[doc="Write the CMD register."]
    #[inline] pub fn set_cmd<F: FnOnce(Cmd) -> Cmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmd_mut(), f(Cmd(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0x8) as *mut Status
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
           self.status_mut()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile(self.status_ptr())
        }
    }

    #[doc="Get the *mut pointer for the CLKDIV register."]
    #[inline] pub fn clkdiv_mut(&self) -> *mut Clkdiv { 
        (self.0 + 0xc) as *mut Clkdiv
    }

    #[doc="Get the *const pointer for the CLKDIV register."]
    #[inline] pub fn clkdiv_ptr(&self) -> *const Clkdiv { 
           self.clkdiv_mut()
    }

    #[doc="Read the CLKDIV register."]
    #[inline] pub fn clkdiv(&self) -> Clkdiv { 
        unsafe {
            read_volatile(self.clkdiv_ptr())
        }
    }

    #[doc="Write the CLKDIV register."]
    #[inline] pub fn set_clkdiv<F: FnOnce(Clkdiv) -> Clkdiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkdiv_mut(), f(Clkdiv(0)));
        }
        self
    }

    #[doc="Modify the CLKDIV register."]
    #[inline] pub fn with_clkdiv<F: FnOnce(Clkdiv) -> Clkdiv>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkdiv_mut(), f(self.clkdiv()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STARTFRAME register."]
    #[inline] pub fn startframe_mut(&self) -> *mut Startframe { 
        (self.0 + 0x10) as *mut Startframe
    }

    #[doc="Get the *const pointer for the STARTFRAME register."]
    #[inline] pub fn startframe_ptr(&self) -> *const Startframe { 
           self.startframe_mut()
    }

    #[doc="Read the STARTFRAME register."]
    #[inline] pub fn startframe(&self) -> Startframe { 
        unsafe {
            read_volatile(self.startframe_ptr())
        }
    }

    #[doc="Write the STARTFRAME register."]
    #[inline] pub fn set_startframe<F: FnOnce(Startframe) -> Startframe>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.startframe_mut(), f(Startframe(0)));
        }
        self
    }

    #[doc="Modify the STARTFRAME register."]
    #[inline] pub fn with_startframe<F: FnOnce(Startframe) -> Startframe>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.startframe_mut(), f(self.startframe()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SIGFRAME register."]
    #[inline] pub fn sigframe_mut(&self) -> *mut Sigframe { 
        (self.0 + 0x14) as *mut Sigframe
    }

    #[doc="Get the *const pointer for the SIGFRAME register."]
    #[inline] pub fn sigframe_ptr(&self) -> *const Sigframe { 
           self.sigframe_mut()
    }

    #[doc="Read the SIGFRAME register."]
    #[inline] pub fn sigframe(&self) -> Sigframe { 
        unsafe {
            read_volatile(self.sigframe_ptr())
        }
    }

    #[doc="Write the SIGFRAME register."]
    #[inline] pub fn set_sigframe<F: FnOnce(Sigframe) -> Sigframe>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sigframe_mut(), f(Sigframe(0)));
        }
        self
    }

    #[doc="Modify the SIGFRAME register."]
    #[inline] pub fn with_sigframe<F: FnOnce(Sigframe) -> Sigframe>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sigframe_mut(), f(self.sigframe()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RXDATAX register."]
    #[inline] pub fn rxdatax_mut(&self) -> *mut Rxdatax { 
        (self.0 + 0x18) as *mut Rxdatax
    }

    #[doc="Get the *const pointer for the RXDATAX register."]
    #[inline] pub fn rxdatax_ptr(&self) -> *const Rxdatax { 
           self.rxdatax_mut()
    }

    #[doc="Read the RXDATAX register."]
    #[inline] pub fn rxdatax(&self) -> Rxdatax { 
        unsafe {
            read_volatile(self.rxdatax_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RXDATA register."]
    #[inline] pub fn rxdata_mut(&self) -> *mut Rxdata { 
        (self.0 + 0x1c) as *mut Rxdata
    }

    #[doc="Get the *const pointer for the RXDATA register."]
    #[inline] pub fn rxdata_ptr(&self) -> *const Rxdata { 
           self.rxdata_mut()
    }

    #[doc="Read the RXDATA register."]
    #[inline] pub fn rxdata(&self) -> Rxdata { 
        unsafe {
            read_volatile(self.rxdata_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RXDATAXP register."]
    #[inline] pub fn rxdataxp_mut(&self) -> *mut Rxdataxp { 
        (self.0 + 0x20) as *mut Rxdataxp
    }

    #[doc="Get the *const pointer for the RXDATAXP register."]
    #[inline] pub fn rxdataxp_ptr(&self) -> *const Rxdataxp { 
           self.rxdataxp_mut()
    }

    #[doc="Read the RXDATAXP register."]
    #[inline] pub fn rxdataxp(&self) -> Rxdataxp { 
        unsafe {
            read_volatile(self.rxdataxp_ptr())
        }
    }

    #[doc="Get the *mut pointer for the TXDATAX register."]
    #[inline] pub fn txdatax_mut(&self) -> *mut Txdatax { 
        (self.0 + 0x24) as *mut Txdatax
    }

    #[doc="Get the *const pointer for the TXDATAX register."]
    #[inline] pub fn txdatax_ptr(&self) -> *const Txdatax { 
           self.txdatax_mut()
    }

    #[doc="Read the TXDATAX register."]
    #[inline] pub fn txdatax(&self) -> Txdatax { 
        unsafe {
            read_volatile(self.txdatax_ptr())
        }
    }

    #[doc="Write the TXDATAX register."]
    #[inline] pub fn set_txdatax<F: FnOnce(Txdatax) -> Txdatax>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdatax_mut(), f(Txdatax(0)));
        }
        self
    }

    #[doc="Modify the TXDATAX register."]
    #[inline] pub fn with_txdatax<F: FnOnce(Txdatax) -> Txdatax>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdatax_mut(), f(self.txdatax()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TXDATA register."]
    #[inline] pub fn txdata_mut(&self) -> *mut Txdata { 
        (self.0 + 0x28) as *mut Txdata
    }

    #[doc="Get the *const pointer for the TXDATA register."]
    #[inline] pub fn txdata_ptr(&self) -> *const Txdata { 
           self.txdata_mut()
    }

    #[doc="Read the TXDATA register."]
    #[inline] pub fn txdata(&self) -> Txdata { 
        unsafe {
            read_volatile(self.txdata_ptr())
        }
    }

    #[doc="Write the TXDATA register."]
    #[inline] pub fn set_txdata<F: FnOnce(Txdata) -> Txdata>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdata_mut(), f(Txdata(0)));
        }
        self
    }

    #[doc="Modify the TXDATA register."]
    #[inline] pub fn with_txdata<F: FnOnce(Txdata) -> Txdata>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdata_mut(), f(self.txdata()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IF register."]
    #[inline] pub fn if_mut(&self) -> *mut If { 
        (self.0 + 0x2c) as *mut If
    }

    #[doc="Get the *const pointer for the IF register."]
    #[inline] pub fn if_ptr(&self) -> *const If { 
           self.if_mut()
    }

    #[doc="Read the IF register."]
    #[inline] pub fn _if(&self) -> If { 
        unsafe {
            read_volatile(self.if_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IFS register."]
    #[inline] pub fn ifs_mut(&self) -> *mut Ifs { 
        (self.0 + 0x30) as *mut Ifs
    }

    #[doc="Get the *const pointer for the IFS register."]
    #[inline] pub fn ifs_ptr(&self) -> *const Ifs { 
           self.ifs_mut()
    }

    #[doc="Write the IFS register."]
    #[inline] pub fn set_ifs<F: FnOnce(Ifs) -> Ifs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifs_mut(), f(Ifs(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IFC register."]
    #[inline] pub fn ifc_mut(&self) -> *mut Ifc { 
        (self.0 + 0x34) as *mut Ifc
    }

    #[doc="Get the *const pointer for the IFC register."]
    #[inline] pub fn ifc_ptr(&self) -> *const Ifc { 
           self.ifc_mut()
    }

    #[doc="Write the IFC register."]
    #[inline] pub fn set_ifc<F: FnOnce(Ifc) -> Ifc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifc_mut(), f(Ifc(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IEN register."]
    #[inline] pub fn ien_mut(&self) -> *mut Ien { 
        (self.0 + 0x38) as *mut Ien
    }

    #[doc="Get the *const pointer for the IEN register."]
    #[inline] pub fn ien_ptr(&self) -> *const Ien { 
           self.ien_mut()
    }

    #[doc="Read the IEN register."]
    #[inline] pub fn ien(&self) -> Ien { 
        unsafe {
            read_volatile(self.ien_ptr())
        }
    }

    #[doc="Write the IEN register."]
    #[inline] pub fn set_ien<F: FnOnce(Ien) -> Ien>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ien_mut(), f(Ien(0)));
        }
        self
    }

    #[doc="Modify the IEN register."]
    #[inline] pub fn with_ien<F: FnOnce(Ien) -> Ien>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ien_mut(), f(self.ien()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PULSECTRL register."]
    #[inline] pub fn pulsectrl_mut(&self) -> *mut Pulsectrl { 
        (self.0 + 0x3c) as *mut Pulsectrl
    }

    #[doc="Get the *const pointer for the PULSECTRL register."]
    #[inline] pub fn pulsectrl_ptr(&self) -> *const Pulsectrl { 
           self.pulsectrl_mut()
    }

    #[doc="Read the PULSECTRL register."]
    #[inline] pub fn pulsectrl(&self) -> Pulsectrl { 
        unsafe {
            read_volatile(self.pulsectrl_ptr())
        }
    }

    #[doc="Write the PULSECTRL register."]
    #[inline] pub fn set_pulsectrl<F: FnOnce(Pulsectrl) -> Pulsectrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pulsectrl_mut(), f(Pulsectrl(0)));
        }
        self
    }

    #[doc="Modify the PULSECTRL register."]
    #[inline] pub fn with_pulsectrl<F: FnOnce(Pulsectrl) -> Pulsectrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pulsectrl_mut(), f(self.pulsectrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FREEZE register."]
    #[inline] pub fn freeze_mut(&self) -> *mut Freeze { 
        (self.0 + 0x40) as *mut Freeze
    }

    #[doc="Get the *const pointer for the FREEZE register."]
    #[inline] pub fn freeze_ptr(&self) -> *const Freeze { 
           self.freeze_mut()
    }

    #[doc="Read the FREEZE register."]
    #[inline] pub fn freeze(&self) -> Freeze { 
        unsafe {
            read_volatile(self.freeze_ptr())
        }
    }

    #[doc="Write the FREEZE register."]
    #[inline] pub fn set_freeze<F: FnOnce(Freeze) -> Freeze>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.freeze_mut(), f(Freeze(0)));
        }
        self
    }

    #[doc="Modify the FREEZE register."]
    #[inline] pub fn with_freeze<F: FnOnce(Freeze) -> Freeze>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.freeze_mut(), f(self.freeze()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        (self.0 + 0x44) as *mut Syncbusy
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
           self.syncbusy_mut()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        unsafe {
            read_volatile(self.syncbusy_ptr())
        }
    }

    #[doc="Get the *mut pointer for the ROUTEPEN register."]
    #[inline] pub fn routepen_mut(&self) -> *mut Routepen { 
        (self.0 + 0x54) as *mut Routepen
    }

    #[doc="Get the *const pointer for the ROUTEPEN register."]
    #[inline] pub fn routepen_ptr(&self) -> *const Routepen { 
           self.routepen_mut()
    }

    #[doc="Read the ROUTEPEN register."]
    #[inline] pub fn routepen(&self) -> Routepen { 
        unsafe {
            read_volatile(self.routepen_ptr())
        }
    }

    #[doc="Write the ROUTEPEN register."]
    #[inline] pub fn set_routepen<F: FnOnce(Routepen) -> Routepen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routepen_mut(), f(Routepen(0)));
        }
        self
    }

    #[doc="Modify the ROUTEPEN register."]
    #[inline] pub fn with_routepen<F: FnOnce(Routepen) -> Routepen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routepen_mut(), f(self.routepen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ROUTELOC0 register."]
    #[inline] pub fn routeloc0_mut(&self) -> *mut Routeloc0 { 
        (self.0 + 0x58) as *mut Routeloc0
    }

    #[doc="Get the *const pointer for the ROUTELOC0 register."]
    #[inline] pub fn routeloc0_ptr(&self) -> *const Routeloc0 { 
           self.routeloc0_mut()
    }

    #[doc="Read the ROUTELOC0 register."]
    #[inline] pub fn routeloc0(&self) -> Routeloc0 { 
        unsafe {
            read_volatile(self.routeloc0_ptr())
        }
    }

    #[doc="Write the ROUTELOC0 register."]
    #[inline] pub fn set_routeloc0<F: FnOnce(Routeloc0) -> Routeloc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc0_mut(), f(Routeloc0(0)));
        }
        self
    }

    #[doc="Modify the ROUTELOC0 register."]
    #[inline] pub fn with_routeloc0<F: FnOnce(Routeloc0) -> Routeloc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc0_mut(), f(self.routeloc0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INPUT register."]
    #[inline] pub fn input_mut(&self) -> *mut Input { 
        (self.0 + 0x64) as *mut Input
    }

    #[doc="Get the *const pointer for the INPUT register."]
    #[inline] pub fn input_ptr(&self) -> *const Input { 
           self.input_mut()
    }

    #[doc="Read the INPUT register."]
    #[inline] pub fn input(&self) -> Input { 
        unsafe {
            read_volatile(self.input_ptr())
        }
    }

    #[doc="Write the INPUT register."]
    #[inline] pub fn set_input<F: FnOnce(Input) -> Input>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.input_mut(), f(Input(0)));
        }
        self
    }

    #[doc="Modify the INPUT register."]
    #[inline] pub fn with_input<F: FnOnce(Input) -> Input>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.input_mut(), f(self.input()));
        }
        self
    }

}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Automatic Transmitter Tristate"]
    #[inline] pub fn autotri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if AUTOTRI != 0"]
    #[inline] pub fn test_autotri(&self) -> bool {
        self.autotri() != 0
    }

    #[doc="Sets the AUTOTRI field."]
    #[inline] pub fn set_autotri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data-Bit Mode"]
    #[inline] pub fn databits(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DATABITS != 0"]
    #[inline] pub fn test_databits(&self) -> bool {
        self.databits() != 0
    }

    #[doc="Sets the DATABITS field."]
    #[inline] pub fn set_databits<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Parity-Bit Mode"]
    #[inline] pub fn parity(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if PARITY != 0"]
    #[inline] pub fn test_parity(&self) -> bool {
        self.parity() != 0
    }

    #[doc="Sets the PARITY field."]
    #[inline] pub fn set_parity<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Stop-Bit Mode"]
    #[inline] pub fn stopbits(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STOPBITS != 0"]
    #[inline] pub fn test_stopbits(&self) -> bool {
        self.stopbits() != 0
    }

    #[doc="Sets the STOPBITS field."]
    #[inline] pub fn set_stopbits<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Invert Input And Output"]
    #[inline] pub fn inv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if INV != 0"]
    #[inline] pub fn test_inv(&self) -> bool {
        self.inv() != 0
    }

    #[doc="Sets the INV field."]
    #[inline] pub fn set_inv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear RX DMA On Error"]
    #[inline] pub fn errsdma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ERRSDMA != 0"]
    #[inline] pub fn test_errsdma(&self) -> bool {
        self.errsdma() != 0
    }

    #[doc="Sets the ERRSDMA field."]
    #[inline] pub fn set_errsdma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loopback Enable"]
    #[inline] pub fn loopbk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOOPBK != 0"]
    #[inline] pub fn test_loopbk(&self) -> bool {
        self.loopbk() != 0
    }

    #[doc="Sets the LOOPBK field."]
    #[inline] pub fn set_loopbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Start-Frame UnBlock RX"]
    #[inline] pub fn sfubrx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SFUBRX != 0"]
    #[inline] pub fn test_sfubrx(&self) -> bool {
        self.sfubrx() != 0
    }

    #[doc="Sets the SFUBRX field."]
    #[inline] pub fn set_sfubrx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Multi-Processor Mode"]
    #[inline] pub fn mpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if MPM != 0"]
    #[inline] pub fn test_mpm(&self) -> bool {
        self.mpm() != 0
    }

    #[doc="Sets the MPM field."]
    #[inline] pub fn set_mpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Multi-Processor Address-Bit"]
    #[inline] pub fn mpab(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if MPAB != 0"]
    #[inline] pub fn test_mpab(&self) -> bool {
        self.mpab() != 0
    }

    #[doc="Sets the MPAB field."]
    #[inline] pub fn set_mpab<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Bit 8 Default Value"]
    #[inline] pub fn bit8dv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BIT8DV != 0"]
    #[inline] pub fn test_bit8dv(&self) -> bool {
        self.bit8dv() != 0
    }

    #[doc="Sets the BIT8DV field."]
    #[inline] pub fn set_bit8dv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="RX DMA Wakeup"]
    #[inline] pub fn rxdmawu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if RXDMAWU != 0"]
    #[inline] pub fn test_rxdmawu(&self) -> bool {
        self.rxdmawu() != 0
    }

    #[doc="Sets the RXDMAWU field."]
    #[inline] pub fn set_rxdmawu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TX DMA Wakeup"]
    #[inline] pub fn txdmawu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXDMAWU != 0"]
    #[inline] pub fn test_txdmawu(&self) -> bool {
        self.txdmawu() != 0
    }

    #[doc="Sets the TXDMAWU field."]
    #[inline] pub fn set_txdmawu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TX Delay Transmission"]
    #[inline] pub fn txdelay(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if TXDELAY != 0"]
    #[inline] pub fn test_txdelay(&self) -> bool {
        self.txdelay() != 0
    }

    #[doc="Sets the TXDELAY field."]
    #[inline] pub fn set_txdelay<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
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
        if self.autotri() != 0 { try!(write!(f, " autotri"))}
        if self.databits() != 0 { try!(write!(f, " databits"))}
        if self.parity() != 0 { try!(write!(f, " parity=0x{:x}", self.parity()))}
        if self.stopbits() != 0 { try!(write!(f, " stopbits"))}
        if self.inv() != 0 { try!(write!(f, " inv"))}
        if self.errsdma() != 0 { try!(write!(f, " errsdma"))}
        if self.loopbk() != 0 { try!(write!(f, " loopbk"))}
        if self.sfubrx() != 0 { try!(write!(f, " sfubrx"))}
        if self.mpm() != 0 { try!(write!(f, " mpm"))}
        if self.mpab() != 0 { try!(write!(f, " mpab"))}
        if self.bit8dv() != 0 { try!(write!(f, " bit8dv"))}
        if self.rxdmawu() != 0 { try!(write!(f, " rxdmawu"))}
        if self.txdmawu() != 0 { try!(write!(f, " txdmawu"))}
        if self.txdelay() != 0 { try!(write!(f, " txdelay=0x{:x}", self.txdelay()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Command Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc="Receiver Enable"]
    #[inline] pub fn rxen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXEN != 0"]
    #[inline] pub fn test_rxen(&self) -> bool {
        self.rxen() != 0
    }

    #[doc="Sets the RXEN field."]
    #[inline] pub fn set_rxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receiver Disable"]
    #[inline] pub fn rxdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RXDIS != 0"]
    #[inline] pub fn test_rxdis(&self) -> bool {
        self.rxdis() != 0
    }

    #[doc="Sets the RXDIS field."]
    #[inline] pub fn set_rxdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn txen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXEN != 0"]
    #[inline] pub fn test_txen(&self) -> bool {
        self.txen() != 0
    }

    #[doc="Sets the TXEN field."]
    #[inline] pub fn set_txen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmitter Disable"]
    #[inline] pub fn txdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TXDIS != 0"]
    #[inline] pub fn test_txdis(&self) -> bool {
        self.txdis() != 0
    }

    #[doc="Sets the TXDIS field."]
    #[inline] pub fn set_txdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receiver Block Enable"]
    #[inline] pub fn rxblocken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXBLOCKEN != 0"]
    #[inline] pub fn test_rxblocken(&self) -> bool {
        self.rxblocken() != 0
    }

    #[doc="Sets the RXBLOCKEN field."]
    #[inline] pub fn set_rxblocken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receiver Block Disable"]
    #[inline] pub fn rxblockdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXBLOCKDIS != 0"]
    #[inline] pub fn test_rxblockdis(&self) -> bool {
        self.rxblockdis() != 0
    }

    #[doc="Sets the RXBLOCKDIS field."]
    #[inline] pub fn set_rxblockdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear TX"]
    #[inline] pub fn cleartx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CLEARTX != 0"]
    #[inline] pub fn test_cleartx(&self) -> bool {
        self.cleartx() != 0
    }

    #[doc="Sets the CLEARTX field."]
    #[inline] pub fn set_cleartx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear RX"]
    #[inline] pub fn clearrx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CLEARRX != 0"]
    #[inline] pub fn test_clearrx(&self) -> bool {
        self.clearrx() != 0
    }

    #[doc="Sets the CLEARRX field."]
    #[inline] pub fn set_clearrx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Cmd {
    #[inline]
    fn from(other: u32) -> Self {
         Cmd(other)
    }
}

impl ::core::fmt::Display for Cmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxen() != 0 { try!(write!(f, " rxen"))}
        if self.rxdis() != 0 { try!(write!(f, " rxdis"))}
        if self.txen() != 0 { try!(write!(f, " txen"))}
        if self.txdis() != 0 { try!(write!(f, " txdis"))}
        if self.rxblocken() != 0 { try!(write!(f, " rxblocken"))}
        if self.rxblockdis() != 0 { try!(write!(f, " rxblockdis"))}
        if self.cleartx() != 0 { try!(write!(f, " cleartx"))}
        if self.clearrx() != 0 { try!(write!(f, " clearrx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="Receiver Enable Status"]
    #[inline] pub fn rxens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXENS != 0"]
    #[inline] pub fn test_rxens(&self) -> bool {
        self.rxens() != 0
    }

    #[doc="Sets the RXENS field."]
    #[inline] pub fn set_rxens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmitter Enable Status"]
    #[inline] pub fn txens(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXENS != 0"]
    #[inline] pub fn test_txens(&self) -> bool {
        self.txens() != 0
    }

    #[doc="Sets the TXENS field."]
    #[inline] pub fn set_txens<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Block Incoming Data"]
    #[inline] pub fn rxblock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXBLOCK != 0"]
    #[inline] pub fn test_rxblock(&self) -> bool {
        self.rxblock() != 0
    }

    #[doc="Sets the RXBLOCK field."]
    #[inline] pub fn set_rxblock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TX Complete"]
    #[inline] pub fn txc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TX Buffer Level"]
    #[inline] pub fn txbl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXBL != 0"]
    #[inline] pub fn test_txbl(&self) -> bool {
        self.txbl() != 0
    }

    #[doc="Sets the TXBL field."]
    #[inline] pub fn set_txbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RX Data Valid"]
    #[inline] pub fn rxdatav(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXDATAV != 0"]
    #[inline] pub fn test_rxdatav(&self) -> bool {
        self.rxdatav() != 0
    }

    #[doc="Sets the RXDATAV field."]
    #[inline] pub fn set_rxdatav<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TX Idle"]
    #[inline] pub fn txidle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXIDLE != 0"]
    #[inline] pub fn test_txidle(&self) -> bool {
        self.txidle() != 0
    }

    #[doc="Sets the TXIDLE field."]
    #[inline] pub fn set_txidle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
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
        if self.rxens() != 0 { try!(write!(f, " rxens"))}
        if self.txens() != 0 { try!(write!(f, " txens"))}
        if self.rxblock() != 0 { try!(write!(f, " rxblock"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.txbl() != 0 { try!(write!(f, " txbl"))}
        if self.rxdatav() != 0 { try!(write!(f, " rxdatav"))}
        if self.txidle() != 0 { try!(write!(f, " txidle"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkdiv(pub u32);
impl Clkdiv {
    #[doc="Fractional Clock Divider"]
    #[inline] pub fn div(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3fff) as u16) } // [16:3]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Clkdiv {
    #[inline]
    fn from(other: u32) -> Self {
         Clkdiv(other)
    }
}

impl ::core::fmt::Display for Clkdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Start Frame Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Startframe(pub u32);
impl Startframe {
    #[doc="Start Frame"]
    #[inline] pub fn startframe(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if STARTFRAME != 0"]
    #[inline] pub fn test_startframe(&self) -> bool {
        self.startframe() != 0
    }

    #[doc="Sets the STARTFRAME field."]
    #[inline] pub fn set_startframe<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Startframe {
    #[inline]
    fn from(other: u32) -> Self {
         Startframe(other)
    }
}

impl ::core::fmt::Display for Startframe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Startframe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.startframe() != 0 { try!(write!(f, " startframe=0x{:x}", self.startframe()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Signal Frame Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sigframe(pub u32);
impl Sigframe {
    #[doc="Signal Frame"]
    #[inline] pub fn sigframe(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if SIGFRAME != 0"]
    #[inline] pub fn test_sigframe(&self) -> bool {
        self.sigframe() != 0
    }

    #[doc="Sets the SIGFRAME field."]
    #[inline] pub fn set_sigframe<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sigframe {
    #[inline]
    fn from(other: u32) -> Self {
         Sigframe(other)
    }
}

impl ::core::fmt::Display for Sigframe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sigframe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sigframe() != 0 { try!(write!(f, " sigframe=0x{:x}", self.sigframe()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Buffer Data Extended Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdatax(pub u32);
impl Rxdatax {
    #[doc="RX Data"]
    #[inline] pub fn rxdata(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if RXDATA != 0"]
    #[inline] pub fn test_rxdata(&self) -> bool {
        self.rxdata() != 0
    }

    #[doc="Sets the RXDATA field."]
    #[inline] pub fn set_rxdata<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Parity Error"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Receive Data Framing Error"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Rxdatax {
    #[inline]
    fn from(other: u32) -> Self {
         Rxdatax(other)
    }
}

impl ::core::fmt::Display for Rxdatax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdatax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdata() != 0 { try!(write!(f, " rxdata=0x{:x}", self.rxdata()))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Buffer Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdata(pub u32);
impl Rxdata {
    #[doc="RX Data"]
    #[inline] pub fn rxdata(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RXDATA != 0"]
    #[inline] pub fn test_rxdata(&self) -> bool {
        self.rxdata() != 0
    }

    #[doc="Sets the RXDATA field."]
    #[inline] pub fn set_rxdata<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
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
        if self.rxdata() != 0 { try!(write!(f, " rxdata=0x{:x}", self.rxdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Buffer Data Extended Peek Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdataxp(pub u32);
impl Rxdataxp {
    #[doc="RX Data Peek"]
    #[inline] pub fn rxdatap(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if RXDATAP != 0"]
    #[inline] pub fn test_rxdatap(&self) -> bool {
        self.rxdatap() != 0
    }

    #[doc="Sets the RXDATAP field."]
    #[inline] pub fn set_rxdatap<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Parity Error Peek"]
    #[inline] pub fn perrp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PERRP != 0"]
    #[inline] pub fn test_perrp(&self) -> bool {
        self.perrp() != 0
    }

    #[doc="Sets the PERRP field."]
    #[inline] pub fn set_perrp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Receive Data Framing Error Peek"]
    #[inline] pub fn ferrp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FERRP != 0"]
    #[inline] pub fn test_ferrp(&self) -> bool {
        self.ferrp() != 0
    }

    #[doc="Sets the FERRP field."]
    #[inline] pub fn set_ferrp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Rxdataxp {
    #[inline]
    fn from(other: u32) -> Self {
         Rxdataxp(other)
    }
}

impl ::core::fmt::Display for Rxdataxp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdataxp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdatap() != 0 { try!(write!(f, " rxdatap=0x{:x}", self.rxdatap()))}
        if self.perrp() != 0 { try!(write!(f, " perrp"))}
        if self.ferrp() != 0 { try!(write!(f, " ferrp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Buffer Data Extended Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdatax(pub u32);
impl Txdatax {
    #[doc="TX Data"]
    #[inline] pub fn txdata(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if TXDATA != 0"]
    #[inline] pub fn test_txdata(&self) -> bool {
        self.txdata() != 0
    }

    #[doc="Sets the TXDATA field."]
    #[inline] pub fn set_txdata<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Data As Break"]
    #[inline] pub fn txbreak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXBREAK != 0"]
    #[inline] pub fn test_txbreak(&self) -> bool {
        self.txbreak() != 0
    }

    #[doc="Sets the TXBREAK field."]
    #[inline] pub fn set_txbreak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Disable TX After Transmission"]
    #[inline] pub fn txdisat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TXDISAT != 0"]
    #[inline] pub fn test_txdisat(&self) -> bool {
        self.txdisat() != 0
    }

    #[doc="Sets the TXDISAT field."]
    #[inline] pub fn set_txdisat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Enable RX After Transmission"]
    #[inline] pub fn rxenat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RXENAT != 0"]
    #[inline] pub fn test_rxenat(&self) -> bool {
        self.rxenat() != 0
    }

    #[doc="Sets the RXENAT field."]
    #[inline] pub fn set_rxenat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Txdatax {
    #[inline]
    fn from(other: u32) -> Self {
         Txdatax(other)
    }
}

impl ::core::fmt::Display for Txdatax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txdatax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
        if self.txbreak() != 0 { try!(write!(f, " txbreak"))}
        if self.txdisat() != 0 { try!(write!(f, " txdisat"))}
        if self.rxenat() != 0 { try!(write!(f, " rxenat"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Buffer Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdata(pub u32);
impl Txdata {
    #[doc="TX Data"]
    #[inline] pub fn txdata(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TXDATA != 0"]
    #[inline] pub fn test_txdata(&self) -> bool {
        self.txdata() != 0
    }

    #[doc="Sets the TXDATA field."]
    #[inline] pub fn set_txdata<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
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
        if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct If(pub u32);
impl If {
    #[doc="TX Complete Interrupt Flag"]
    #[inline] pub fn txc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TX Buffer Level Interrupt Flag"]
    #[inline] pub fn txbl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXBL != 0"]
    #[inline] pub fn test_txbl(&self) -> bool {
        self.txbl() != 0
    }

    #[doc="Sets the TXBL field."]
    #[inline] pub fn set_txbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RX Data Valid Interrupt Flag"]
    #[inline] pub fn rxdatav(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXDATAV != 0"]
    #[inline] pub fn test_rxdatav(&self) -> bool {
        self.rxdatav() != 0
    }

    #[doc="Sets the RXDATAV field."]
    #[inline] pub fn set_rxdatav<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RX Overflow Interrupt Flag"]
    #[inline] pub fn rxof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RX Underflow Interrupt Flag"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TX Overflow Interrupt Flag"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Parity Error Interrupt Flag"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Framing Error Interrupt Flag"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Multi-Processor Address Frame Interrupt Flag"]
    #[inline] pub fn mpaf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MPAF != 0"]
    #[inline] pub fn test_mpaf(&self) -> bool {
        self.mpaf() != 0
    }

    #[doc="Sets the MPAF field."]
    #[inline] pub fn set_mpaf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Start Frame Interrupt Flag"]
    #[inline] pub fn startf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STARTF != 0"]
    #[inline] pub fn test_startf(&self) -> bool {
        self.startf() != 0
    }

    #[doc="Sets the STARTF field."]
    #[inline] pub fn set_startf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Signal Frame Interrupt Flag"]
    #[inline] pub fn sigf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SIGF != 0"]
    #[inline] pub fn test_sigf(&self) -> bool {
        self.sigf() != 0
    }

    #[doc="Sets the SIGF field."]
    #[inline] pub fn set_sigf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for If {
    #[inline]
    fn from(other: u32) -> Self {
         If(other)
    }
}

impl ::core::fmt::Display for If {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for If {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.txbl() != 0 { try!(write!(f, " txbl"))}
        if self.rxdatav() != 0 { try!(write!(f, " rxdatav"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.mpaf() != 0 { try!(write!(f, " mpaf"))}
        if self.startf() != 0 { try!(write!(f, " startf"))}
        if self.sigf() != 0 { try!(write!(f, " sigf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Set Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc="Set TXC Interrupt Flag"]
    #[inline] pub fn txc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Set RXOF Interrupt Flag"]
    #[inline] pub fn rxof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Set RXUF Interrupt Flag"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Set TXOF Interrupt Flag"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Set PERR Interrupt Flag"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Set FERR Interrupt Flag"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Set MPAF Interrupt Flag"]
    #[inline] pub fn mpaf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MPAF != 0"]
    #[inline] pub fn test_mpaf(&self) -> bool {
        self.mpaf() != 0
    }

    #[doc="Sets the MPAF field."]
    #[inline] pub fn set_mpaf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Set STARTF Interrupt Flag"]
    #[inline] pub fn startf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STARTF != 0"]
    #[inline] pub fn test_startf(&self) -> bool {
        self.startf() != 0
    }

    #[doc="Sets the STARTF field."]
    #[inline] pub fn set_startf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Set SIGF Interrupt Flag"]
    #[inline] pub fn sigf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SIGF != 0"]
    #[inline] pub fn test_sigf(&self) -> bool {
        self.sigf() != 0
    }

    #[doc="Sets the SIGF field."]
    #[inline] pub fn set_sigf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Ifs {
    #[inline]
    fn from(other: u32) -> Self {
         Ifs(other)
    }
}

impl ::core::fmt::Display for Ifs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.mpaf() != 0 { try!(write!(f, " mpaf"))}
        if self.startf() != 0 { try!(write!(f, " startf"))}
        if self.sigf() != 0 { try!(write!(f, " sigf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Clear Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifc(pub u32);
impl Ifc {
    #[doc="Clear TXC Interrupt Flag"]
    #[inline] pub fn txc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear RXOF Interrupt Flag"]
    #[inline] pub fn rxof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear RXUF Interrupt Flag"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear TXOF Interrupt Flag"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear PERR Interrupt Flag"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear FERR Interrupt Flag"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Clear MPAF Interrupt Flag"]
    #[inline] pub fn mpaf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MPAF != 0"]
    #[inline] pub fn test_mpaf(&self) -> bool {
        self.mpaf() != 0
    }

    #[doc="Sets the MPAF field."]
    #[inline] pub fn set_mpaf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear STARTF Interrupt Flag"]
    #[inline] pub fn startf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STARTF != 0"]
    #[inline] pub fn test_startf(&self) -> bool {
        self.startf() != 0
    }

    #[doc="Sets the STARTF field."]
    #[inline] pub fn set_startf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clear SIGF Interrupt Flag"]
    #[inline] pub fn sigf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SIGF != 0"]
    #[inline] pub fn test_sigf(&self) -> bool {
        self.sigf() != 0
    }

    #[doc="Sets the SIGF field."]
    #[inline] pub fn set_sigf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Ifc {
    #[inline]
    fn from(other: u32) -> Self {
         Ifc(other)
    }
}

impl ::core::fmt::Display for Ifc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.mpaf() != 0 { try!(write!(f, " mpaf"))}
        if self.startf() != 0 { try!(write!(f, " startf"))}
        if self.sigf() != 0 { try!(write!(f, " sigf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc="TXC Interrupt Enable"]
    #[inline] pub fn txc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TXBL Interrupt Enable"]
    #[inline] pub fn txbl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXBL != 0"]
    #[inline] pub fn test_txbl(&self) -> bool {
        self.txbl() != 0
    }

    #[doc="Sets the TXBL field."]
    #[inline] pub fn set_txbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RXDATAV Interrupt Enable"]
    #[inline] pub fn rxdatav(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXDATAV != 0"]
    #[inline] pub fn test_rxdatav(&self) -> bool {
        self.rxdatav() != 0
    }

    #[doc="Sets the RXDATAV field."]
    #[inline] pub fn set_rxdatav<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RXOF Interrupt Enable"]
    #[inline] pub fn rxof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RXUF Interrupt Enable"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TXOF Interrupt Enable"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PERR Interrupt Enable"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="FERR Interrupt Enable"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="MPAF Interrupt Enable"]
    #[inline] pub fn mpaf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MPAF != 0"]
    #[inline] pub fn test_mpaf(&self) -> bool {
        self.mpaf() != 0
    }

    #[doc="Sets the MPAF field."]
    #[inline] pub fn set_mpaf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="STARTF Interrupt Enable"]
    #[inline] pub fn startf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STARTF != 0"]
    #[inline] pub fn test_startf(&self) -> bool {
        self.startf() != 0
    }

    #[doc="Sets the STARTF field."]
    #[inline] pub fn set_startf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SIGF Interrupt Enable"]
    #[inline] pub fn sigf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SIGF != 0"]
    #[inline] pub fn test_sigf(&self) -> bool {
        self.sigf() != 0
    }

    #[doc="Sets the SIGF field."]
    #[inline] pub fn set_sigf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Ien {
    #[inline]
    fn from(other: u32) -> Self {
         Ien(other)
    }
}

impl ::core::fmt::Display for Ien {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ien {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.txbl() != 0 { try!(write!(f, " txbl"))}
        if self.rxdatav() != 0 { try!(write!(f, " rxdatav"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.mpaf() != 0 { try!(write!(f, " mpaf"))}
        if self.startf() != 0 { try!(write!(f, " startf"))}
        if self.sigf() != 0 { try!(write!(f, " sigf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pulse Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pulsectrl(pub u32);
impl Pulsectrl {
    #[doc="Pulse Width"]
    #[inline] pub fn pulsew(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PULSEW != 0"]
    #[inline] pub fn test_pulsew(&self) -> bool {
        self.pulsew() != 0
    }

    #[doc="Sets the PULSEW field."]
    #[inline] pub fn set_pulsew<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pulse Generator/Extender Enable"]
    #[inline] pub fn pulseen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PULSEEN != 0"]
    #[inline] pub fn test_pulseen(&self) -> bool {
        self.pulseen() != 0
    }

    #[doc="Sets the PULSEEN field."]
    #[inline] pub fn set_pulseen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pulse Filter"]
    #[inline] pub fn pulsefilt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PULSEFILT != 0"]
    #[inline] pub fn test_pulsefilt(&self) -> bool {
        self.pulsefilt() != 0
    }

    #[doc="Sets the PULSEFILT field."]
    #[inline] pub fn set_pulsefilt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Pulsectrl {
    #[inline]
    fn from(other: u32) -> Self {
         Pulsectrl(other)
    }
}

impl ::core::fmt::Display for Pulsectrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pulsectrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pulsew() != 0 { try!(write!(f, " pulsew=0x{:x}", self.pulsew()))}
        if self.pulseen() != 0 { try!(write!(f, " pulseen"))}
        if self.pulsefilt() != 0 { try!(write!(f, " pulsefilt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Freeze Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Freeze(pub u32);
impl Freeze {
    #[doc="Register Update Freeze"]
    #[inline] pub fn regfreeze(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if REGFREEZE != 0"]
    #[inline] pub fn test_regfreeze(&self) -> bool {
        self.regfreeze() != 0
    }

    #[doc="Sets the REGFREEZE field."]
    #[inline] pub fn set_regfreeze<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Freeze {
    #[inline]
    fn from(other: u32) -> Self {
         Freeze(other)
    }
}

impl ::core::fmt::Display for Freeze {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Freeze {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.regfreeze() != 0 { try!(write!(f, " regfreeze"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="CTRL Register Busy"]
    #[inline] pub fn ctrl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTRL != 0"]
    #[inline] pub fn test_ctrl(&self) -> bool {
        self.ctrl() != 0
    }

    #[doc="Sets the CTRL field."]
    #[inline] pub fn set_ctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CMD Register Busy"]
    #[inline] pub fn cmd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CLKDIV Register Busy"]
    #[inline] pub fn clkdiv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="STARTFRAME Register Busy"]
    #[inline] pub fn startframe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STARTFRAME != 0"]
    #[inline] pub fn test_startframe(&self) -> bool {
        self.startframe() != 0
    }

    #[doc="Sets the STARTFRAME field."]
    #[inline] pub fn set_startframe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SIGFRAME Register Busy"]
    #[inline] pub fn sigframe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SIGFRAME != 0"]
    #[inline] pub fn test_sigframe(&self) -> bool {
        self.sigframe() != 0
    }

    #[doc="Sets the SIGFRAME field."]
    #[inline] pub fn set_sigframe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TXDATAX Register Busy"]
    #[inline] pub fn txdatax(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXDATAX != 0"]
    #[inline] pub fn test_txdatax(&self) -> bool {
        self.txdatax() != 0
    }

    #[doc="Sets the TXDATAX field."]
    #[inline] pub fn set_txdatax<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TXDATA Register Busy"]
    #[inline] pub fn txdata(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXDATA != 0"]
    #[inline] pub fn test_txdata(&self) -> bool {
        self.txdata() != 0
    }

    #[doc="Sets the TXDATA field."]
    #[inline] pub fn set_txdata<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PULSECTRL Register Busy"]
    #[inline] pub fn pulsectrl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PULSECTRL != 0"]
    #[inline] pub fn test_pulsectrl(&self) -> bool {
        self.pulsectrl() != 0
    }

    #[doc="Sets the PULSECTRL field."]
    #[inline] pub fn set_pulsectrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.ctrl() != 0 { try!(write!(f, " ctrl"))}
        if self.cmd() != 0 { try!(write!(f, " cmd"))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv"))}
        if self.startframe() != 0 { try!(write!(f, " startframe"))}
        if self.sigframe() != 0 { try!(write!(f, " sigframe"))}
        if self.txdatax() != 0 { try!(write!(f, " txdatax"))}
        if self.txdata() != 0 { try!(write!(f, " txdata"))}
        if self.pulsectrl() != 0 { try!(write!(f, " pulsectrl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Pin Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc="RX Pin Enable"]
    #[inline] pub fn rxpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXPEN != 0"]
    #[inline] pub fn test_rxpen(&self) -> bool {
        self.rxpen() != 0
    }

    #[doc="Sets the RXPEN field."]
    #[inline] pub fn set_rxpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TX Pin Enable"]
    #[inline] pub fn txpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXPEN != 0"]
    #[inline] pub fn test_txpen(&self) -> bool {
        self.txpen() != 0
    }

    #[doc="Sets the TXPEN field."]
    #[inline] pub fn set_txpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Routepen {
    #[inline]
    fn from(other: u32) -> Self {
         Routepen(other)
    }
}

impl ::core::fmt::Display for Routepen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routepen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxpen() != 0 { try!(write!(f, " rxpen"))}
        if self.txpen() != 0 { try!(write!(f, " txpen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Location Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routeloc0(pub u32);
impl Routeloc0 {
    #[doc="I/O Location"]
    #[inline] pub fn rxloc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if RXLOC != 0"]
    #[inline] pub fn test_rxloc(&self) -> bool {
        self.rxloc() != 0
    }

    #[doc="Sets the RXLOC field."]
    #[inline] pub fn set_rxloc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I/O Location"]
    #[inline] pub fn txloc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if TXLOC != 0"]
    #[inline] pub fn test_txloc(&self) -> bool {
        self.txloc() != 0
    }

    #[doc="Sets the TXLOC field."]
    #[inline] pub fn set_txloc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Routeloc0 {
    #[inline]
    fn from(other: u32) -> Self {
         Routeloc0(other)
    }
}

impl ::core::fmt::Display for Routeloc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routeloc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxloc() != 0 { try!(write!(f, " rxloc=0x{:x}", self.rxloc()))}
        if self.txloc() != 0 { try!(write!(f, " txloc=0x{:x}", self.txloc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LEUART Input Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Input(pub u32);
impl Input {
    #[doc="RX PRS Channel Select"]
    #[inline] pub fn rxprssel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if RXPRSSEL != 0"]
    #[inline] pub fn test_rxprssel(&self) -> bool {
        self.rxprssel() != 0
    }

    #[doc="Sets the RXPRSSEL field."]
    #[inline] pub fn set_rxprssel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PRS RX Enable"]
    #[inline] pub fn rxprs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXPRS != 0"]
    #[inline] pub fn test_rxprs(&self) -> bool {
        self.rxprs() != 0
    }

    #[doc="Sets the RXPRS field."]
    #[inline] pub fn set_rxprs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Input {
    #[inline]
    fn from(other: u32) -> Self {
         Input(other)
    }
}

impl ::core::fmt::Display for Input {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Input {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxprssel() != 0 { try!(write!(f, " rxprssel=0x{:x}", self.rxprssel()))}
        if self.rxprs() != 0 { try!(write!(f, " rxprs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


