#[allow(unused_imports)] use bobbin_common::*;

periph!( USART0, Usart0, _USART0, UsartPeriph, 0x40010000);
periph!( USART1, Usart1, _USART1, UsartPeriph, 0x40010400);
periph!( USART2, Usart2, _USART2, UsartPeriph, 0x40010800);
periph!( USART3, Usart3, _USART3, UsartPeriph, 0x40010c00);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USART Peripheral"]
pub struct UsartPeriph(pub usize); 






impl UsartPeriph {
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

    #[doc="Get the *mut pointer for the FRAME register."]
    #[inline] pub fn frame_mut(&self) -> *mut Frame { 
        (self.0 + 0x4) as *mut Frame
    }

    #[doc="Get the *const pointer for the FRAME register."]
    #[inline] pub fn frame_ptr(&self) -> *const Frame { 
           self.frame_mut()
    }

    #[doc="Read the FRAME register."]
    #[inline] pub fn frame(&self) -> Frame { 
        unsafe {
            read_volatile(self.frame_ptr())
        }
    }

    #[doc="Write the FRAME register."]
    #[inline] pub fn set_frame<F: FnOnce(Frame) -> Frame>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.frame_mut(), f(Frame(0)));
        }
        self
    }

    #[doc="Modify the FRAME register."]
    #[inline] pub fn with_frame<F: FnOnce(Frame) -> Frame>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.frame_mut(), f(self.frame()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TRIGCTRL register."]
    #[inline] pub fn trigctrl_mut(&self) -> *mut Trigctrl { 
        (self.0 + 0x8) as *mut Trigctrl
    }

    #[doc="Get the *const pointer for the TRIGCTRL register."]
    #[inline] pub fn trigctrl_ptr(&self) -> *const Trigctrl { 
           self.trigctrl_mut()
    }

    #[doc="Read the TRIGCTRL register."]
    #[inline] pub fn trigctrl(&self) -> Trigctrl { 
        unsafe {
            read_volatile(self.trigctrl_ptr())
        }
    }

    #[doc="Write the TRIGCTRL register."]
    #[inline] pub fn set_trigctrl<F: FnOnce(Trigctrl) -> Trigctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.trigctrl_mut(), f(Trigctrl(0)));
        }
        self
    }

    #[doc="Modify the TRIGCTRL register."]
    #[inline] pub fn with_trigctrl<F: FnOnce(Trigctrl) -> Trigctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.trigctrl_mut(), f(self.trigctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMD register."]
    #[inline] pub fn cmd_mut(&self) -> *mut Cmd { 
        (self.0 + 0xc) as *mut Cmd
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
        (self.0 + 0x10) as *mut Status
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
        (self.0 + 0x14) as *mut Clkdiv
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

    #[doc="Get the *mut pointer for the RXDOUBLEX register."]
    #[inline] pub fn rxdoublex_mut(&self) -> *mut Rxdoublex { 
        (self.0 + 0x20) as *mut Rxdoublex
    }

    #[doc="Get the *const pointer for the RXDOUBLEX register."]
    #[inline] pub fn rxdoublex_ptr(&self) -> *const Rxdoublex { 
           self.rxdoublex_mut()
    }

    #[doc="Read the RXDOUBLEX register."]
    #[inline] pub fn rxdoublex(&self) -> Rxdoublex { 
        unsafe {
            read_volatile(self.rxdoublex_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RXDOUBLE register."]
    #[inline] pub fn rxdouble_mut(&self) -> *mut Rxdouble { 
        (self.0 + 0x24) as *mut Rxdouble
    }

    #[doc="Get the *const pointer for the RXDOUBLE register."]
    #[inline] pub fn rxdouble_ptr(&self) -> *const Rxdouble { 
           self.rxdouble_mut()
    }

    #[doc="Read the RXDOUBLE register."]
    #[inline] pub fn rxdouble(&self) -> Rxdouble { 
        unsafe {
            read_volatile(self.rxdouble_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RXDATAXP register."]
    #[inline] pub fn rxdataxp_mut(&self) -> *mut Rxdataxp { 
        (self.0 + 0x28) as *mut Rxdataxp
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

    #[doc="Get the *mut pointer for the RXDOUBLEXP register."]
    #[inline] pub fn rxdoublexp_mut(&self) -> *mut Rxdoublexp { 
        (self.0 + 0x2c) as *mut Rxdoublexp
    }

    #[doc="Get the *const pointer for the RXDOUBLEXP register."]
    #[inline] pub fn rxdoublexp_ptr(&self) -> *const Rxdoublexp { 
           self.rxdoublexp_mut()
    }

    #[doc="Read the RXDOUBLEXP register."]
    #[inline] pub fn rxdoublexp(&self) -> Rxdoublexp { 
        unsafe {
            read_volatile(self.rxdoublexp_ptr())
        }
    }

    #[doc="Get the *mut pointer for the TXDATAX register."]
    #[inline] pub fn txdatax_mut(&self) -> *mut Txdatax { 
        (self.0 + 0x30) as *mut Txdatax
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
        (self.0 + 0x34) as *mut Txdata
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

    #[doc="Get the *mut pointer for the TXDOUBLEX register."]
    #[inline] pub fn txdoublex_mut(&self) -> *mut Txdoublex { 
        (self.0 + 0x38) as *mut Txdoublex
    }

    #[doc="Get the *const pointer for the TXDOUBLEX register."]
    #[inline] pub fn txdoublex_ptr(&self) -> *const Txdoublex { 
           self.txdoublex_mut()
    }

    #[doc="Read the TXDOUBLEX register."]
    #[inline] pub fn txdoublex(&self) -> Txdoublex { 
        unsafe {
            read_volatile(self.txdoublex_ptr())
        }
    }

    #[doc="Write the TXDOUBLEX register."]
    #[inline] pub fn set_txdoublex<F: FnOnce(Txdoublex) -> Txdoublex>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdoublex_mut(), f(Txdoublex(0)));
        }
        self
    }

    #[doc="Modify the TXDOUBLEX register."]
    #[inline] pub fn with_txdoublex<F: FnOnce(Txdoublex) -> Txdoublex>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdoublex_mut(), f(self.txdoublex()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TXDOUBLE register."]
    #[inline] pub fn txdouble_mut(&self) -> *mut Txdouble { 
        (self.0 + 0x3c) as *mut Txdouble
    }

    #[doc="Get the *const pointer for the TXDOUBLE register."]
    #[inline] pub fn txdouble_ptr(&self) -> *const Txdouble { 
           self.txdouble_mut()
    }

    #[doc="Read the TXDOUBLE register."]
    #[inline] pub fn txdouble(&self) -> Txdouble { 
        unsafe {
            read_volatile(self.txdouble_ptr())
        }
    }

    #[doc="Write the TXDOUBLE register."]
    #[inline] pub fn set_txdouble<F: FnOnce(Txdouble) -> Txdouble>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdouble_mut(), f(Txdouble(0)));
        }
        self
    }

    #[doc="Modify the TXDOUBLE register."]
    #[inline] pub fn with_txdouble<F: FnOnce(Txdouble) -> Txdouble>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.txdouble_mut(), f(self.txdouble()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IF register."]
    #[inline] pub fn if_mut(&self) -> *mut If { 
        (self.0 + 0x40) as *mut If
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
        (self.0 + 0x44) as *mut Ifs
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
        (self.0 + 0x48) as *mut Ifc
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
        (self.0 + 0x4c) as *mut Ien
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

    #[doc="Get the *mut pointer for the IRCTRL register."]
    #[inline] pub fn irctrl_mut(&self) -> *mut Irctrl { 
        (self.0 + 0x50) as *mut Irctrl
    }

    #[doc="Get the *const pointer for the IRCTRL register."]
    #[inline] pub fn irctrl_ptr(&self) -> *const Irctrl { 
           self.irctrl_mut()
    }

    #[doc="Read the IRCTRL register."]
    #[inline] pub fn irctrl(&self) -> Irctrl { 
        unsafe {
            read_volatile(self.irctrl_ptr())
        }
    }

    #[doc="Write the IRCTRL register."]
    #[inline] pub fn set_irctrl<F: FnOnce(Irctrl) -> Irctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.irctrl_mut(), f(Irctrl(0)));
        }
        self
    }

    #[doc="Modify the IRCTRL register."]
    #[inline] pub fn with_irctrl<F: FnOnce(Irctrl) -> Irctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.irctrl_mut(), f(self.irctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INPUT register."]
    #[inline] pub fn input_mut(&self) -> *mut Input { 
        (self.0 + 0x58) as *mut Input
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

    #[doc="Get the *mut pointer for the I2SCTRL register."]
    #[inline] pub fn i2sctrl_mut(&self) -> *mut I2sctrl { 
        (self.0 + 0x5c) as *mut I2sctrl
    }

    #[doc="Get the *const pointer for the I2SCTRL register."]
    #[inline] pub fn i2sctrl_ptr(&self) -> *const I2sctrl { 
           self.i2sctrl_mut()
    }

    #[doc="Read the I2SCTRL register."]
    #[inline] pub fn i2sctrl(&self) -> I2sctrl { 
        unsafe {
            read_volatile(self.i2sctrl_ptr())
        }
    }

    #[doc="Write the I2SCTRL register."]
    #[inline] pub fn set_i2sctrl<F: FnOnce(I2sctrl) -> I2sctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.i2sctrl_mut(), f(I2sctrl(0)));
        }
        self
    }

    #[doc="Modify the I2SCTRL register."]
    #[inline] pub fn with_i2sctrl<F: FnOnce(I2sctrl) -> I2sctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.i2sctrl_mut(), f(self.i2sctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TIMING register."]
    #[inline] pub fn timing_mut(&self) -> *mut Timing { 
        (self.0 + 0x60) as *mut Timing
    }

    #[doc="Get the *const pointer for the TIMING register."]
    #[inline] pub fn timing_ptr(&self) -> *const Timing { 
           self.timing_mut()
    }

    #[doc="Read the TIMING register."]
    #[inline] pub fn timing(&self) -> Timing { 
        unsafe {
            read_volatile(self.timing_ptr())
        }
    }

    #[doc="Write the TIMING register."]
    #[inline] pub fn set_timing<F: FnOnce(Timing) -> Timing>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timing_mut(), f(Timing(0)));
        }
        self
    }

    #[doc="Modify the TIMING register."]
    #[inline] pub fn with_timing<F: FnOnce(Timing) -> Timing>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timing_mut(), f(self.timing()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLX register."]
    #[inline] pub fn ctrlx_mut(&self) -> *mut Ctrlx { 
        (self.0 + 0x64) as *mut Ctrlx
    }

    #[doc="Get the *const pointer for the CTRLX register."]
    #[inline] pub fn ctrlx_ptr(&self) -> *const Ctrlx { 
           self.ctrlx_mut()
    }

    #[doc="Read the CTRLX register."]
    #[inline] pub fn ctrlx(&self) -> Ctrlx { 
        unsafe {
            read_volatile(self.ctrlx_ptr())
        }
    }

    #[doc="Write the CTRLX register."]
    #[inline] pub fn set_ctrlx<F: FnOnce(Ctrlx) -> Ctrlx>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlx_mut(), f(Ctrlx(0)));
        }
        self
    }

    #[doc="Modify the CTRLX register."]
    #[inline] pub fn with_ctrlx<F: FnOnce(Ctrlx) -> Ctrlx>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlx_mut(), f(self.ctrlx()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TIMECMP0 register."]
    #[inline] pub fn timecmp0_mut(&self) -> *mut Timecmp0 { 
        (self.0 + 0x68) as *mut Timecmp0
    }

    #[doc="Get the *const pointer for the TIMECMP0 register."]
    #[inline] pub fn timecmp0_ptr(&self) -> *const Timecmp0 { 
           self.timecmp0_mut()
    }

    #[doc="Read the TIMECMP0 register."]
    #[inline] pub fn timecmp0(&self) -> Timecmp0 { 
        unsafe {
            read_volatile(self.timecmp0_ptr())
        }
    }

    #[doc="Write the TIMECMP0 register."]
    #[inline] pub fn set_timecmp0<F: FnOnce(Timecmp0) -> Timecmp0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timecmp0_mut(), f(Timecmp0(0)));
        }
        self
    }

    #[doc="Modify the TIMECMP0 register."]
    #[inline] pub fn with_timecmp0<F: FnOnce(Timecmp0) -> Timecmp0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timecmp0_mut(), f(self.timecmp0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TIMECMP1 register."]
    #[inline] pub fn timecmp1_mut(&self) -> *mut Timecmp1 { 
        (self.0 + 0x6c) as *mut Timecmp1
    }

    #[doc="Get the *const pointer for the TIMECMP1 register."]
    #[inline] pub fn timecmp1_ptr(&self) -> *const Timecmp1 { 
           self.timecmp1_mut()
    }

    #[doc="Read the TIMECMP1 register."]
    #[inline] pub fn timecmp1(&self) -> Timecmp1 { 
        unsafe {
            read_volatile(self.timecmp1_ptr())
        }
    }

    #[doc="Write the TIMECMP1 register."]
    #[inline] pub fn set_timecmp1<F: FnOnce(Timecmp1) -> Timecmp1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timecmp1_mut(), f(Timecmp1(0)));
        }
        self
    }

    #[doc="Modify the TIMECMP1 register."]
    #[inline] pub fn with_timecmp1<F: FnOnce(Timecmp1) -> Timecmp1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timecmp1_mut(), f(self.timecmp1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TIMECMP2 register."]
    #[inline] pub fn timecmp2_mut(&self) -> *mut Timecmp2 { 
        (self.0 + 0x70) as *mut Timecmp2
    }

    #[doc="Get the *const pointer for the TIMECMP2 register."]
    #[inline] pub fn timecmp2_ptr(&self) -> *const Timecmp2 { 
           self.timecmp2_mut()
    }

    #[doc="Read the TIMECMP2 register."]
    #[inline] pub fn timecmp2(&self) -> Timecmp2 { 
        unsafe {
            read_volatile(self.timecmp2_ptr())
        }
    }

    #[doc="Write the TIMECMP2 register."]
    #[inline] pub fn set_timecmp2<F: FnOnce(Timecmp2) -> Timecmp2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timecmp2_mut(), f(Timecmp2(0)));
        }
        self
    }

    #[doc="Modify the TIMECMP2 register."]
    #[inline] pub fn with_timecmp2<F: FnOnce(Timecmp2) -> Timecmp2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.timecmp2_mut(), f(self.timecmp2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ROUTEPEN register."]
    #[inline] pub fn routepen_mut(&self) -> *mut Routepen { 
        (self.0 + 0x74) as *mut Routepen
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
        (self.0 + 0x78) as *mut Routeloc0
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

    #[doc="Get the *mut pointer for the ROUTELOC1 register."]
    #[inline] pub fn routeloc1_mut(&self) -> *mut Routeloc1 { 
        (self.0 + 0x7c) as *mut Routeloc1
    }

    #[doc="Get the *const pointer for the ROUTELOC1 register."]
    #[inline] pub fn routeloc1_ptr(&self) -> *const Routeloc1 { 
           self.routeloc1_mut()
    }

    #[doc="Read the ROUTELOC1 register."]
    #[inline] pub fn routeloc1(&self) -> Routeloc1 { 
        unsafe {
            read_volatile(self.routeloc1_ptr())
        }
    }

    #[doc="Write the ROUTELOC1 register."]
    #[inline] pub fn set_routeloc1<F: FnOnce(Routeloc1) -> Routeloc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc1_mut(), f(Routeloc1(0)));
        }
        self
    }

    #[doc="Modify the ROUTELOC1 register."]
    #[inline] pub fn with_routeloc1<F: FnOnce(Routeloc1) -> Routeloc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc1_mut(), f(self.routeloc1()));
        }
        self
    }

}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="USART Synchronous Mode"]
    #[inline] pub fn sync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYNC != 0"]
    #[inline] pub fn test_sync(&self) -> bool {
        self.sync() != 0
    }

    #[doc="Sets the SYNC field."]
    #[inline] pub fn set_sync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Loopback Enable"]
    #[inline] pub fn loopbk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LOOPBK != 0"]
    #[inline] pub fn test_loopbk(&self) -> bool {
        self.loopbk() != 0
    }

    #[doc="Sets the LOOPBK field."]
    #[inline] pub fn set_loopbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Collision Check Enable"]
    #[inline] pub fn ccen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CCEN != 0"]
    #[inline] pub fn test_ccen(&self) -> bool {
        self.ccen() != 0
    }

    #[doc="Sets the CCEN field."]
    #[inline] pub fn set_ccen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Multi-Processor Mode"]
    #[inline] pub fn mpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MPM != 0"]
    #[inline] pub fn test_mpm(&self) -> bool {
        self.mpm() != 0
    }

    #[doc="Sets the MPM field."]
    #[inline] pub fn set_mpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Multi-Processor Address-Bit"]
    #[inline] pub fn mpab(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MPAB != 0"]
    #[inline] pub fn test_mpab(&self) -> bool {
        self.mpab() != 0
    }

    #[doc="Sets the MPAB field."]
    #[inline] pub fn set_mpab<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Oversampling"]
    #[inline] pub fn ovs(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if OVS != 0"]
    #[inline] pub fn test_ovs(&self) -> bool {
        self.ovs() != 0
    }

    #[doc="Sets the OVS field."]
    #[inline] pub fn set_ovs<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn clkpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CLKPOL != 0"]
    #[inline] pub fn test_clkpol(&self) -> bool {
        self.clkpol() != 0
    }

    #[doc="Sets the CLKPOL field."]
    #[inline] pub fn set_clkpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock Edge For Setup/Sample"]
    #[inline] pub fn clkpha(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CLKPHA != 0"]
    #[inline] pub fn test_clkpha(&self) -> bool {
        self.clkpha() != 0
    }

    #[doc="Sets the CLKPHA field."]
    #[inline] pub fn set_clkpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Most Significant Bit First"]
    #[inline] pub fn msbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if MSBF != 0"]
    #[inline] pub fn test_msbf(&self) -> bool {
        self.msbf() != 0
    }

    #[doc="Sets the MSBF field."]
    #[inline] pub fn set_msbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Action On Slave-Select In Master Mode"]
    #[inline] pub fn csma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CSMA != 0"]
    #[inline] pub fn test_csma(&self) -> bool {
        self.csma() != 0
    }

    #[doc="Sets the CSMA field."]
    #[inline] pub fn set_csma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TX Buffer Interrupt Level"]
    #[inline] pub fn txbil(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXBIL != 0"]
    #[inline] pub fn test_txbil(&self) -> bool {
        self.txbil() != 0
    }

    #[doc="Sets the TXBIL field."]
    #[inline] pub fn set_txbil<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receiver Input Invert"]
    #[inline] pub fn rxinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if RXINV != 0"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Sets the RXINV field."]
    #[inline] pub fn set_rxinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Transmitter output Invert"]
    #[inline] pub fn txinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TXINV != 0"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Sets the TXINV field."]
    #[inline] pub fn set_txinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Chip Select Invert"]
    #[inline] pub fn csinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CSINV != 0"]
    #[inline] pub fn test_csinv(&self) -> bool {
        self.csinv() != 0
    }

    #[doc="Sets the CSINV field."]
    #[inline] pub fn set_csinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Automatic Chip Select"]
    #[inline] pub fn autocs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if AUTOCS != 0"]
    #[inline] pub fn test_autocs(&self) -> bool {
        self.autocs() != 0
    }

    #[doc="Sets the AUTOCS field."]
    #[inline] pub fn set_autocs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Automatic TX Tristate"]
    #[inline] pub fn autotri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if AUTOTRI != 0"]
    #[inline] pub fn test_autotri(&self) -> bool {
        self.autotri() != 0
    }

    #[doc="Sets the AUTOTRI field."]
    #[inline] pub fn set_autotri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SmartCard Mode"]
    #[inline] pub fn scmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if SCMODE != 0"]
    #[inline] pub fn test_scmode(&self) -> bool {
        self.scmode() != 0
    }

    #[doc="Sets the SCMODE field."]
    #[inline] pub fn set_scmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SmartCard Retransmit"]
    #[inline] pub fn scretrans(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if SCRETRANS != 0"]
    #[inline] pub fn test_scretrans(&self) -> bool {
        self.scretrans() != 0
    }

    #[doc="Sets the SCRETRANS field."]
    #[inline] pub fn set_scretrans<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Skip Parity Error Frames"]
    #[inline] pub fn skipperrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SKIPPERRF != 0"]
    #[inline] pub fn test_skipperrf(&self) -> bool {
        self.skipperrf() != 0
    }

    #[doc="Sets the SKIPPERRF field."]
    #[inline] pub fn set_skipperrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Bit 8 Default Value"]
    #[inline] pub fn bit8dv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if BIT8DV != 0"]
    #[inline] pub fn test_bit8dv(&self) -> bool {
        self.bit8dv() != 0
    }

    #[doc="Sets the BIT8DV field."]
    #[inline] pub fn set_bit8dv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Halt DMA On Error"]
    #[inline] pub fn errsdma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if ERRSDMA != 0"]
    #[inline] pub fn test_errsdma(&self) -> bool {
        self.errsdma() != 0
    }

    #[doc="Sets the ERRSDMA field."]
    #[inline] pub fn set_errsdma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Disable RX On Error"]
    #[inline] pub fn errsrx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if ERRSRX != 0"]
    #[inline] pub fn test_errsrx(&self) -> bool {
        self.errsrx() != 0
    }

    #[doc="Sets the ERRSRX field."]
    #[inline] pub fn set_errsrx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Disable TX On Error"]
    #[inline] pub fn errstx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ERRSTX != 0"]
    #[inline] pub fn test_errstx(&self) -> bool {
        self.errstx() != 0
    }

    #[doc="Sets the ERRSTX field."]
    #[inline] pub fn set_errstx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Synchronous Slave Setup Early"]
    #[inline] pub fn sssearly(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if SSSEARLY != 0"]
    #[inline] pub fn test_sssearly(&self) -> bool {
        self.sssearly() != 0
    }

    #[doc="Sets the SSSEARLY field."]
    #[inline] pub fn set_sssearly<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Byteswap In Double Accesses"]
    #[inline] pub fn byteswap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if BYTESWAP != 0"]
    #[inline] pub fn test_byteswap(&self) -> bool {
        self.byteswap() != 0
    }

    #[doc="Sets the BYTESWAP field."]
    #[inline] pub fn set_byteswap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Always Transmit When RX Not Full"]
    #[inline] pub fn autotx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if AUTOTX != 0"]
    #[inline] pub fn test_autotx(&self) -> bool {
        self.autotx() != 0
    }

    #[doc="Sets the AUTOTX field."]
    #[inline] pub fn set_autotx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Majority Vote Disable"]
    #[inline] pub fn mvdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if MVDIS != 0"]
    #[inline] pub fn test_mvdis(&self) -> bool {
        self.mvdis() != 0
    }

    #[doc="Sets the MVDIS field."]
    #[inline] pub fn set_mvdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Synchronous Master Sample Delay"]
    #[inline] pub fn smsdelay(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SMSDELAY != 0"]
    #[inline] pub fn test_smsdelay(&self) -> bool {
        self.smsdelay() != 0
    }

    #[doc="Sets the SMSDELAY field."]
    #[inline] pub fn set_smsdelay<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.sync() != 0 { try!(write!(f, " sync"))}
        if self.loopbk() != 0 { try!(write!(f, " loopbk"))}
        if self.ccen() != 0 { try!(write!(f, " ccen"))}
        if self.mpm() != 0 { try!(write!(f, " mpm"))}
        if self.mpab() != 0 { try!(write!(f, " mpab"))}
        if self.ovs() != 0 { try!(write!(f, " ovs=0x{:x}", self.ovs()))}
        if self.clkpol() != 0 { try!(write!(f, " clkpol"))}
        if self.clkpha() != 0 { try!(write!(f, " clkpha"))}
        if self.msbf() != 0 { try!(write!(f, " msbf"))}
        if self.csma() != 0 { try!(write!(f, " csma"))}
        if self.txbil() != 0 { try!(write!(f, " txbil"))}
        if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
        if self.txinv() != 0 { try!(write!(f, " txinv"))}
        if self.csinv() != 0 { try!(write!(f, " csinv"))}
        if self.autocs() != 0 { try!(write!(f, " autocs"))}
        if self.autotri() != 0 { try!(write!(f, " autotri"))}
        if self.scmode() != 0 { try!(write!(f, " scmode"))}
        if self.scretrans() != 0 { try!(write!(f, " scretrans"))}
        if self.skipperrf() != 0 { try!(write!(f, " skipperrf"))}
        if self.bit8dv() != 0 { try!(write!(f, " bit8dv"))}
        if self.errsdma() != 0 { try!(write!(f, " errsdma"))}
        if self.errsrx() != 0 { try!(write!(f, " errsrx"))}
        if self.errstx() != 0 { try!(write!(f, " errstx"))}
        if self.sssearly() != 0 { try!(write!(f, " sssearly"))}
        if self.byteswap() != 0 { try!(write!(f, " byteswap"))}
        if self.autotx() != 0 { try!(write!(f, " autotx"))}
        if self.mvdis() != 0 { try!(write!(f, " mvdis"))}
        if self.smsdelay() != 0 { try!(write!(f, " smsdelay"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Frame Format Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Frame(pub u32);
impl Frame {
    #[doc="Data-Bit Mode"]
    #[inline] pub fn databits(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DATABITS != 0"]
    #[inline] pub fn test_databits(&self) -> bool {
        self.databits() != 0
    }

    #[doc="Sets the DATABITS field."]
    #[inline] pub fn set_databits<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Parity-Bit Mode"]
    #[inline] pub fn parity(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if PARITY != 0"]
    #[inline] pub fn test_parity(&self) -> bool {
        self.parity() != 0
    }

    #[doc="Sets the PARITY field."]
    #[inline] pub fn set_parity<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Stop-Bit Mode"]
    #[inline] pub fn stopbits(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STOPBITS != 0"]
    #[inline] pub fn test_stopbits(&self) -> bool {
        self.stopbits() != 0
    }

    #[doc="Sets the STOPBITS field."]
    #[inline] pub fn set_stopbits<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Frame {
    #[inline]
    fn from(other: u32) -> Self {
         Frame(other)
    }
}

impl ::core::fmt::Display for Frame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Frame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.databits() != 0 { try!(write!(f, " databits=0x{:x}", self.databits()))}
        if self.parity() != 0 { try!(write!(f, " parity=0x{:x}", self.parity()))}
        if self.stopbits() != 0 { try!(write!(f, " stopbits=0x{:x}", self.stopbits()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Trigger Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Trigctrl(pub u32);
impl Trigctrl {
    #[doc="Receive Trigger Enable"]
    #[inline] pub fn rxten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXTEN != 0"]
    #[inline] pub fn test_rxten(&self) -> bool {
        self.rxten() != 0
    }

    #[doc="Sets the RXTEN field."]
    #[inline] pub fn set_rxten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit Trigger Enable"]
    #[inline] pub fn txten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXTEN != 0"]
    #[inline] pub fn test_txten(&self) -> bool {
        self.txten() != 0
    }

    #[doc="Sets the TXTEN field."]
    #[inline] pub fn set_txten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="AUTOTX Trigger Enable"]
    #[inline] pub fn autotxten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if AUTOTXTEN != 0"]
    #[inline] pub fn test_autotxten(&self) -> bool {
        self.autotxten() != 0
    }

    #[doc="Sets the AUTOTXTEN field."]
    #[inline] pub fn set_autotxten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Enable Transmit Trigger after RX End of Frame plus TCMP0VAL"]
    #[inline] pub fn txarx0en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXARX0EN != 0"]
    #[inline] pub fn test_txarx0en(&self) -> bool {
        self.txarx0en() != 0
    }

    #[doc="Sets the TXARX0EN field."]
    #[inline] pub fn set_txarx0en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Enable Transmit Trigger after RX End of Frame plus TCMP1VAL"]
    #[inline] pub fn txarx1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXARX1EN != 0"]
    #[inline] pub fn test_txarx1en(&self) -> bool {
        self.txarx1en() != 0
    }

    #[doc="Sets the TXARX1EN field."]
    #[inline] pub fn set_txarx1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enable Transmit Trigger after RX End of Frame plus TCMP2VAL"]
    #[inline] pub fn txarx2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXARX2EN != 0"]
    #[inline] pub fn test_txarx2en(&self) -> bool {
        self.txarx2en() != 0
    }

    #[doc="Sets the TXARX2EN field."]
    #[inline] pub fn set_txarx2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable Receive Trigger after TX end of frame plus TCMPVAL0 baud-times"]
    #[inline] pub fn rxatx0en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RXATX0EN != 0"]
    #[inline] pub fn test_rxatx0en(&self) -> bool {
        self.rxatx0en() != 0
    }

    #[doc="Sets the RXATX0EN field."]
    #[inline] pub fn set_rxatx0en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Enable Receive Trigger after TX end of frame plus TCMPVAL1 baud-times"]
    #[inline] pub fn rxatx1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RXATX1EN != 0"]
    #[inline] pub fn test_rxatx1en(&self) -> bool {
        self.rxatx1en() != 0
    }

    #[doc="Sets the RXATX1EN field."]
    #[inline] pub fn set_rxatx1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Enable Receive Trigger after TX end of frame plus TCMPVAL2 baud-times"]
    #[inline] pub fn rxatx2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if RXATX2EN != 0"]
    #[inline] pub fn test_rxatx2en(&self) -> bool {
        self.rxatx2en() != 0
    }

    #[doc="Sets the RXATX2EN field."]
    #[inline] pub fn set_rxatx2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Trigger PRS Channel Select"]
    #[inline] pub fn tsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if TSEL != 0"]
    #[inline] pub fn test_tsel(&self) -> bool {
        self.tsel() != 0
    }

    #[doc="Sets the TSEL field."]
    #[inline] pub fn set_tsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Trigctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Trigctrl(other)
    }
}

impl ::core::fmt::Display for Trigctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Trigctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxten() != 0 { try!(write!(f, " rxten"))}
        if self.txten() != 0 { try!(write!(f, " txten"))}
        if self.autotxten() != 0 { try!(write!(f, " autotxten"))}
        if self.txarx0en() != 0 { try!(write!(f, " txarx0en"))}
        if self.txarx1en() != 0 { try!(write!(f, " txarx1en"))}
        if self.txarx2en() != 0 { try!(write!(f, " txarx2en"))}
        if self.rxatx0en() != 0 { try!(write!(f, " rxatx0en"))}
        if self.rxatx1en() != 0 { try!(write!(f, " rxatx1en"))}
        if self.rxatx2en() != 0 { try!(write!(f, " rxatx2en"))}
        if self.tsel() != 0 { try!(write!(f, " tsel=0x{:x}", self.tsel()))}
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

    #[doc="Master Enable"]
    #[inline] pub fn masteren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MASTEREN != 0"]
    #[inline] pub fn test_masteren(&self) -> bool {
        self.masteren() != 0
    }

    #[doc="Sets the MASTEREN field."]
    #[inline] pub fn set_masteren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Master Disable"]
    #[inline] pub fn masterdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MASTERDIS != 0"]
    #[inline] pub fn test_masterdis(&self) -> bool {
        self.masterdis() != 0
    }

    #[doc="Sets the MASTERDIS field."]
    #[inline] pub fn set_masterdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receiver Block Enable"]
    #[inline] pub fn rxblocken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXBLOCKEN != 0"]
    #[inline] pub fn test_rxblocken(&self) -> bool {
        self.rxblocken() != 0
    }

    #[doc="Sets the RXBLOCKEN field."]
    #[inline] pub fn set_rxblocken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Receiver Block Disable"]
    #[inline] pub fn rxblockdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RXBLOCKDIS != 0"]
    #[inline] pub fn test_rxblockdis(&self) -> bool {
        self.rxblockdis() != 0
    }

    #[doc="Sets the RXBLOCKDIS field."]
    #[inline] pub fn set_rxblockdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmitter Tristate Enable"]
    #[inline] pub fn txtrien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXTRIEN != 0"]
    #[inline] pub fn test_txtrien(&self) -> bool {
        self.txtrien() != 0
    }

    #[doc="Sets the TXTRIEN field."]
    #[inline] pub fn set_txtrien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmitter Tristate Disable"]
    #[inline] pub fn txtridis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXTRIDIS != 0"]
    #[inline] pub fn test_txtridis(&self) -> bool {
        self.txtridis() != 0
    }

    #[doc="Sets the TXTRIDIS field."]
    #[inline] pub fn set_txtridis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clear TX"]
    #[inline] pub fn cleartx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CLEARTX != 0"]
    #[inline] pub fn test_cleartx(&self) -> bool {
        self.cleartx() != 0
    }

    #[doc="Sets the CLEARTX field."]
    #[inline] pub fn set_cleartx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clear RX"]
    #[inline] pub fn clearrx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CLEARRX != 0"]
    #[inline] pub fn test_clearrx(&self) -> bool {
        self.clearrx() != 0
    }

    #[doc="Sets the CLEARRX field."]
    #[inline] pub fn set_clearrx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
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
        if self.masteren() != 0 { try!(write!(f, " masteren"))}
        if self.masterdis() != 0 { try!(write!(f, " masterdis"))}
        if self.rxblocken() != 0 { try!(write!(f, " rxblocken"))}
        if self.rxblockdis() != 0 { try!(write!(f, " rxblockdis"))}
        if self.txtrien() != 0 { try!(write!(f, " txtrien"))}
        if self.txtridis() != 0 { try!(write!(f, " txtridis"))}
        if self.cleartx() != 0 { try!(write!(f, " cleartx"))}
        if self.clearrx() != 0 { try!(write!(f, " clearrx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Status Register"]
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

    #[doc="SPI Master Mode"]
    #[inline] pub fn master(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MASTER != 0"]
    #[inline] pub fn test_master(&self) -> bool {
        self.master() != 0
    }

    #[doc="Sets the MASTER field."]
    #[inline] pub fn set_master<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Block Incoming Data"]
    #[inline] pub fn rxblock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXBLOCK != 0"]
    #[inline] pub fn test_rxblock(&self) -> bool {
        self.rxblock() != 0
    }

    #[doc="Sets the RXBLOCK field."]
    #[inline] pub fn set_rxblock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmitter Tristated"]
    #[inline] pub fn txtri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXTRI != 0"]
    #[inline] pub fn test_txtri(&self) -> bool {
        self.txtri() != 0
    }

    #[doc="Sets the TXTRI field."]
    #[inline] pub fn set_txtri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TX Complete"]
    #[inline] pub fn txc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXC != 0"]
    #[inline] pub fn test_txc(&self) -> bool {
        self.txc() != 0
    }

    #[doc="Sets the TXC field."]
    #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TX Buffer Level"]
    #[inline] pub fn txbl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXBL != 0"]
    #[inline] pub fn test_txbl(&self) -> bool {
        self.txbl() != 0
    }

    #[doc="Sets the TXBL field."]
    #[inline] pub fn set_txbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="RX Data Valid"]
    #[inline] pub fn rxdatav(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RXDATAV != 0"]
    #[inline] pub fn test_rxdatav(&self) -> bool {
        self.rxdatav() != 0
    }

    #[doc="Sets the RXDATAV field."]
    #[inline] pub fn set_rxdatav<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="RX FIFO Full"]
    #[inline] pub fn rxfull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RXFULL != 0"]
    #[inline] pub fn test_rxfull(&self) -> bool {
        self.rxfull() != 0
    }

    #[doc="Sets the RXFULL field."]
    #[inline] pub fn set_rxfull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TX Buffer Expects Double Right Data"]
    #[inline] pub fn txbdright(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXBDRIGHT != 0"]
    #[inline] pub fn test_txbdright(&self) -> bool {
        self.txbdright() != 0
    }

    #[doc="Sets the TXBDRIGHT field."]
    #[inline] pub fn set_txbdright<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TX Buffer Expects Single Right Data"]
    #[inline] pub fn txbsright(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TXBSRIGHT != 0"]
    #[inline] pub fn test_txbsright(&self) -> bool {
        self.txbsright() != 0
    }

    #[doc="Sets the TXBSRIGHT field."]
    #[inline] pub fn set_txbsright<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="RX Data Right"]
    #[inline] pub fn rxdatavright(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RXDATAVRIGHT != 0"]
    #[inline] pub fn test_rxdatavright(&self) -> bool {
        self.rxdatavright() != 0
    }

    #[doc="Sets the RXDATAVRIGHT field."]
    #[inline] pub fn set_rxdatavright<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="RX Full of Right Data"]
    #[inline] pub fn rxfullright(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if RXFULLRIGHT != 0"]
    #[inline] pub fn test_rxfullright(&self) -> bool {
        self.rxfullright() != 0
    }

    #[doc="Sets the RXFULLRIGHT field."]
    #[inline] pub fn set_rxfullright<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TX Idle"]
    #[inline] pub fn txidle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXIDLE != 0"]
    #[inline] pub fn test_txidle(&self) -> bool {
        self.txidle() != 0
    }

    #[doc="Sets the TXIDLE field."]
    #[inline] pub fn set_txidle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="The USART Timer restarted itself"]
    #[inline] pub fn timerrestarted(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TIMERRESTARTED != 0"]
    #[inline] pub fn test_timerrestarted(&self) -> bool {
        self.timerrestarted() != 0
    }

    #[doc="Sets the TIMERRESTARTED field."]
    #[inline] pub fn set_timerrestarted<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TX Buffer Count"]
    #[inline] pub fn txbufcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if TXBUFCNT != 0"]
    #[inline] pub fn test_txbufcnt(&self) -> bool {
        self.txbufcnt() != 0
    }

    #[doc="Sets the TXBUFCNT field."]
    #[inline] pub fn set_txbufcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
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
        if self.master() != 0 { try!(write!(f, " master"))}
        if self.rxblock() != 0 { try!(write!(f, " rxblock"))}
        if self.txtri() != 0 { try!(write!(f, " txtri"))}
        if self.txc() != 0 { try!(write!(f, " txc"))}
        if self.txbl() != 0 { try!(write!(f, " txbl"))}
        if self.rxdatav() != 0 { try!(write!(f, " rxdatav"))}
        if self.rxfull() != 0 { try!(write!(f, " rxfull"))}
        if self.txbdright() != 0 { try!(write!(f, " txbdright"))}
        if self.txbsright() != 0 { try!(write!(f, " txbsright"))}
        if self.rxdatavright() != 0 { try!(write!(f, " rxdatavright"))}
        if self.rxfullright() != 0 { try!(write!(f, " rxfullright"))}
        if self.txidle() != 0 { try!(write!(f, " txidle"))}
        if self.timerrestarted() != 0 { try!(write!(f, " timerrestarted"))}
        if self.txbufcnt() != 0 { try!(write!(f, " txbufcnt=0x{:x}", self.txbufcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkdiv(pub u32);
impl Clkdiv {
    #[doc="Fractional Clock Divider"]
    #[inline] pub fn div(&self) -> bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0xfffff) as u32) } // [22:3]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<bits::U20>>(mut self, value: V) -> Self {
        let value: bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="AUTOBAUD detection enable"]
    #[inline] pub fn autobauden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if AUTOBAUDEN != 0"]
    #[inline] pub fn test_autobauden(&self) -> bool {
        self.autobauden() != 0
    }

    #[doc="Sets the AUTOBAUDEN field."]
    #[inline] pub fn set_autobauden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.autobauden() != 0 { try!(write!(f, " autobauden"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RX Buffer Data Extended Register"]
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

    #[doc="Data Parity Error"]
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

    #[doc="Data Framing Error"]
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

#[doc="RX Buffer Data Register"]
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

#[doc="RX Buffer Double Data Extended Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdoublex(pub u32);
impl Rxdoublex {
    #[doc="RX Data 0"]
    #[inline] pub fn rxdata0(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if RXDATA0 != 0"]
    #[inline] pub fn test_rxdata0(&self) -> bool {
        self.rxdata0() != 0
    }

    #[doc="Sets the RXDATA0 field."]
    #[inline] pub fn set_rxdata0<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Parity Error 0"]
    #[inline] pub fn perr0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PERR0 != 0"]
    #[inline] pub fn test_perr0(&self) -> bool {
        self.perr0() != 0
    }

    #[doc="Sets the PERR0 field."]
    #[inline] pub fn set_perr0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Data Framing Error 0"]
    #[inline] pub fn ferr0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FERR0 != 0"]
    #[inline] pub fn test_ferr0(&self) -> bool {
        self.ferr0() != 0
    }

    #[doc="Sets the FERR0 field."]
    #[inline] pub fn set_ferr0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="RX Data 1"]
    #[inline] pub fn rxdata1(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1ff) as u16) } // [24:16]
    }

    #[doc="Returns true if RXDATA1 != 0"]
    #[inline] pub fn test_rxdata1(&self) -> bool {
        self.rxdata1() != 0
    }

    #[doc="Sets the RXDATA1 field."]
    #[inline] pub fn set_rxdata1<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data Parity Error 1"]
    #[inline] pub fn perr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PERR1 != 0"]
    #[inline] pub fn test_perr1(&self) -> bool {
        self.perr1() != 0
    }

    #[doc="Sets the PERR1 field."]
    #[inline] pub fn set_perr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Data Framing Error 1"]
    #[inline] pub fn ferr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if FERR1 != 0"]
    #[inline] pub fn test_ferr1(&self) -> bool {
        self.ferr1() != 0
    }

    #[doc="Sets the FERR1 field."]
    #[inline] pub fn set_ferr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Rxdoublex {
    #[inline]
    fn from(other: u32) -> Self {
         Rxdoublex(other)
    }
}

impl ::core::fmt::Display for Rxdoublex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdoublex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdata0() != 0 { try!(write!(f, " rxdata0=0x{:x}", self.rxdata0()))}
        if self.perr0() != 0 { try!(write!(f, " perr0"))}
        if self.ferr0() != 0 { try!(write!(f, " ferr0"))}
        if self.rxdata1() != 0 { try!(write!(f, " rxdata1=0x{:x}", self.rxdata1()))}
        if self.perr1() != 0 { try!(write!(f, " perr1"))}
        if self.ferr1() != 0 { try!(write!(f, " ferr1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RX FIFO Double Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdouble(pub u32);
impl Rxdouble {
    #[doc="RX Data 0"]
    #[inline] pub fn rxdata0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RXDATA0 != 0"]
    #[inline] pub fn test_rxdata0(&self) -> bool {
        self.rxdata0() != 0
    }

    #[doc="Sets the RXDATA0 field."]
    #[inline] pub fn set_rxdata0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RX Data 1"]
    #[inline] pub fn rxdata1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if RXDATA1 != 0"]
    #[inline] pub fn test_rxdata1(&self) -> bool {
        self.rxdata1() != 0
    }

    #[doc="Sets the RXDATA1 field."]
    #[inline] pub fn set_rxdata1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Rxdouble {
    #[inline]
    fn from(other: u32) -> Self {
         Rxdouble(other)
    }
}

impl ::core::fmt::Display for Rxdouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdata0() != 0 { try!(write!(f, " rxdata0=0x{:x}", self.rxdata0()))}
        if self.rxdata1() != 0 { try!(write!(f, " rxdata1=0x{:x}", self.rxdata1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RX Buffer Data Extended Peek Register"]
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

    #[doc="Data Parity Error Peek"]
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

    #[doc="Data Framing Error Peek"]
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

#[doc="RX Buffer Double Data Extended Peek Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdoublexp(pub u32);
impl Rxdoublexp {
    #[doc="RX Data 0 Peek"]
    #[inline] pub fn rxdatap0(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if RXDATAP0 != 0"]
    #[inline] pub fn test_rxdatap0(&self) -> bool {
        self.rxdatap0() != 0
    }

    #[doc="Sets the RXDATAP0 field."]
    #[inline] pub fn set_rxdatap0<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Parity Error 0 Peek"]
    #[inline] pub fn perrp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PERRP0 != 0"]
    #[inline] pub fn test_perrp0(&self) -> bool {
        self.perrp0() != 0
    }

    #[doc="Sets the PERRP0 field."]
    #[inline] pub fn set_perrp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Data Framing Error 0 Peek"]
    #[inline] pub fn ferrp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FERRP0 != 0"]
    #[inline] pub fn test_ferrp0(&self) -> bool {
        self.ferrp0() != 0
    }

    #[doc="Sets the FERRP0 field."]
    #[inline] pub fn set_ferrp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="RX Data 1 Peek"]
    #[inline] pub fn rxdatap1(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1ff) as u16) } // [24:16]
    }

    #[doc="Returns true if RXDATAP1 != 0"]
    #[inline] pub fn test_rxdatap1(&self) -> bool {
        self.rxdatap1() != 0
    }

    #[doc="Sets the RXDATAP1 field."]
    #[inline] pub fn set_rxdatap1<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data Parity Error 1 Peek"]
    #[inline] pub fn perrp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PERRP1 != 0"]
    #[inline] pub fn test_perrp1(&self) -> bool {
        self.perrp1() != 0
    }

    #[doc="Sets the PERRP1 field."]
    #[inline] pub fn set_perrp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Data Framing Error 1 Peek"]
    #[inline] pub fn ferrp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if FERRP1 != 0"]
    #[inline] pub fn test_ferrp1(&self) -> bool {
        self.ferrp1() != 0
    }

    #[doc="Sets the FERRP1 field."]
    #[inline] pub fn set_ferrp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Rxdoublexp {
    #[inline]
    fn from(other: u32) -> Self {
         Rxdoublexp(other)
    }
}

impl ::core::fmt::Display for Rxdoublexp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdoublexp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdatap0() != 0 { try!(write!(f, " rxdatap0=0x{:x}", self.rxdatap0()))}
        if self.perrp0() != 0 { try!(write!(f, " perrp0"))}
        if self.ferrp0() != 0 { try!(write!(f, " ferrp0"))}
        if self.rxdatap1() != 0 { try!(write!(f, " rxdatap1=0x{:x}", self.rxdatap1()))}
        if self.perrp1() != 0 { try!(write!(f, " perrp1"))}
        if self.ferrp1() != 0 { try!(write!(f, " ferrp1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TX Buffer Data Extended Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdatax(pub u32);
impl Txdatax {
    #[doc="TX Data"]
    #[inline] pub fn txdatax(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if TXDATAX != 0"]
    #[inline] pub fn test_txdatax(&self) -> bool {
        self.txdatax() != 0
    }

    #[doc="Sets the TXDATAX field."]
    #[inline] pub fn set_txdatax<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Unblock RX After Transmission"]
    #[inline] pub fn ubrxat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if UBRXAT != 0"]
    #[inline] pub fn test_ubrxat(&self) -> bool {
        self.ubrxat() != 0
    }

    #[doc="Sets the UBRXAT field."]
    #[inline] pub fn set_ubrxat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Set TXTRI After Transmission"]
    #[inline] pub fn txtriat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXTRIAT != 0"]
    #[inline] pub fn test_txtriat(&self) -> bool {
        self.txtriat() != 0
    }

    #[doc="Sets the TXTRIAT field."]
    #[inline] pub fn set_txtriat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
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

    #[doc="Clear TXEN After Transmission"]
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
        if self.txdatax() != 0 { try!(write!(f, " txdatax=0x{:x}", self.txdatax()))}
        if self.ubrxat() != 0 { try!(write!(f, " ubrxat"))}
        if self.txtriat() != 0 { try!(write!(f, " txtriat"))}
        if self.txbreak() != 0 { try!(write!(f, " txbreak"))}
        if self.txdisat() != 0 { try!(write!(f, " txdisat"))}
        if self.rxenat() != 0 { try!(write!(f, " rxenat"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TX Buffer Data Register"]
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

#[doc="TX Buffer Double Data Extended Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdoublex(pub u32);
impl Txdoublex {
    #[doc="TX Data"]
    #[inline] pub fn txdata0(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if TXDATA0 != 0"]
    #[inline] pub fn test_txdata0(&self) -> bool {
        self.txdata0() != 0
    }

    #[doc="Sets the TXDATA0 field."]
    #[inline] pub fn set_txdata0<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Unblock RX After Transmission"]
    #[inline] pub fn ubrxat0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if UBRXAT0 != 0"]
    #[inline] pub fn test_ubrxat0(&self) -> bool {
        self.ubrxat0() != 0
    }

    #[doc="Sets the UBRXAT0 field."]
    #[inline] pub fn set_ubrxat0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Set TXTRI After Transmission"]
    #[inline] pub fn txtriat0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXTRIAT0 != 0"]
    #[inline] pub fn test_txtriat0(&self) -> bool {
        self.txtriat0() != 0
    }

    #[doc="Sets the TXTRIAT0 field."]
    #[inline] pub fn set_txtriat0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Transmit Data As Break"]
    #[inline] pub fn txbreak0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXBREAK0 != 0"]
    #[inline] pub fn test_txbreak0(&self) -> bool {
        self.txbreak0() != 0
    }

    #[doc="Sets the TXBREAK0 field."]
    #[inline] pub fn set_txbreak0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Clear TXEN After Transmission"]
    #[inline] pub fn txdisat0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TXDISAT0 != 0"]
    #[inline] pub fn test_txdisat0(&self) -> bool {
        self.txdisat0() != 0
    }

    #[doc="Sets the TXDISAT0 field."]
    #[inline] pub fn set_txdisat0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Enable RX After Transmission"]
    #[inline] pub fn rxenat0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RXENAT0 != 0"]
    #[inline] pub fn test_rxenat0(&self) -> bool {
        self.rxenat0() != 0
    }

    #[doc="Sets the RXENAT0 field."]
    #[inline] pub fn set_rxenat0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="TX Data"]
    #[inline] pub fn txdata1(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1ff) as u16) } // [24:16]
    }

    #[doc="Returns true if TXDATA1 != 0"]
    #[inline] pub fn test_txdata1(&self) -> bool {
        self.txdata1() != 0
    }

    #[doc="Sets the TXDATA1 field."]
    #[inline] pub fn set_txdata1<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Unblock RX After Transmission"]
    #[inline] pub fn ubrxat1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if UBRXAT1 != 0"]
    #[inline] pub fn test_ubrxat1(&self) -> bool {
        self.ubrxat1() != 0
    }

    #[doc="Sets the UBRXAT1 field."]
    #[inline] pub fn set_ubrxat1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Set TXTRI After Transmission"]
    #[inline] pub fn txtriat1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TXTRIAT1 != 0"]
    #[inline] pub fn test_txtriat1(&self) -> bool {
        self.txtriat1() != 0
    }

    #[doc="Sets the TXTRIAT1 field."]
    #[inline] pub fn set_txtriat1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Transmit Data As Break"]
    #[inline] pub fn txbreak1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if TXBREAK1 != 0"]
    #[inline] pub fn test_txbreak1(&self) -> bool {
        self.txbreak1() != 0
    }

    #[doc="Sets the TXBREAK1 field."]
    #[inline] pub fn set_txbreak1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Clear TXEN After Transmission"]
    #[inline] pub fn txdisat1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if TXDISAT1 != 0"]
    #[inline] pub fn test_txdisat1(&self) -> bool {
        self.txdisat1() != 0
    }

    #[doc="Sets the TXDISAT1 field."]
    #[inline] pub fn set_txdisat1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Enable RX After Transmission"]
    #[inline] pub fn rxenat1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if RXENAT1 != 0"]
    #[inline] pub fn test_rxenat1(&self) -> bool {
        self.rxenat1() != 0
    }

    #[doc="Sets the RXENAT1 field."]
    #[inline] pub fn set_rxenat1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Txdoublex {
    #[inline]
    fn from(other: u32) -> Self {
         Txdoublex(other)
    }
}

impl ::core::fmt::Display for Txdoublex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txdoublex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdata0() != 0 { try!(write!(f, " txdata0=0x{:x}", self.txdata0()))}
        if self.ubrxat0() != 0 { try!(write!(f, " ubrxat0"))}
        if self.txtriat0() != 0 { try!(write!(f, " txtriat0"))}
        if self.txbreak0() != 0 { try!(write!(f, " txbreak0"))}
        if self.txdisat0() != 0 { try!(write!(f, " txdisat0"))}
        if self.rxenat0() != 0 { try!(write!(f, " rxenat0"))}
        if self.txdata1() != 0 { try!(write!(f, " txdata1=0x{:x}", self.txdata1()))}
        if self.ubrxat1() != 0 { try!(write!(f, " ubrxat1"))}
        if self.txtriat1() != 0 { try!(write!(f, " txtriat1"))}
        if self.txbreak1() != 0 { try!(write!(f, " txbreak1"))}
        if self.txdisat1() != 0 { try!(write!(f, " txdisat1"))}
        if self.rxenat1() != 0 { try!(write!(f, " rxenat1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TX Buffer Double Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdouble(pub u32);
impl Txdouble {
    #[doc="TX Data"]
    #[inline] pub fn txdata0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TXDATA0 != 0"]
    #[inline] pub fn test_txdata0(&self) -> bool {
        self.txdata0() != 0
    }

    #[doc="Sets the TXDATA0 field."]
    #[inline] pub fn set_txdata0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TX Data"]
    #[inline] pub fn txdata1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if TXDATA1 != 0"]
    #[inline] pub fn test_txdata1(&self) -> bool {
        self.txdata1() != 0
    }

    #[doc="Sets the TXDATA1 field."]
    #[inline] pub fn set_txdata1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Txdouble {
    #[inline]
    fn from(other: u32) -> Self {
         Txdouble(other)
    }
}

impl ::core::fmt::Display for Txdouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txdouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdata0() != 0 { try!(write!(f, " txdata0=0x{:x}", self.txdata0()))}
        if self.txdata1() != 0 { try!(write!(f, " txdata1=0x{:x}", self.txdata1()))}
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

    #[doc="RX Buffer Full Interrupt Flag"]
    #[inline] pub fn rxfull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFULL != 0"]
    #[inline] pub fn test_rxfull(&self) -> bool {
        self.rxfull() != 0
    }

    #[doc="Sets the RXFULL field."]
    #[inline] pub fn set_rxfull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RX Overflow Interrupt Flag"]
    #[inline] pub fn rxof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RX Underflow Interrupt Flag"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TX Overflow Interrupt Flag"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TX Underflow Interrupt Flag"]
    #[inline] pub fn txuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXUF != 0"]
    #[inline] pub fn test_txuf(&self) -> bool {
        self.txuf() != 0
    }

    #[doc="Sets the TXUF field."]
    #[inline] pub fn set_txuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Parity Error Interrupt Flag"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Framing Error Interrupt Flag"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Multi-Processor Address Frame Interrupt Flag"]
    #[inline] pub fn mpaf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if MPAF != 0"]
    #[inline] pub fn test_mpaf(&self) -> bool {
        self.mpaf() != 0
    }

    #[doc="Sets the MPAF field."]
    #[inline] pub fn set_mpaf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Slave-Select In Master Mode Interrupt Flag"]
    #[inline] pub fn ssm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SSM != 0"]
    #[inline] pub fn test_ssm(&self) -> bool {
        self.ssm() != 0
    }

    #[doc="Sets the SSM field."]
    #[inline] pub fn set_ssm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Collision Check Fail Interrupt Flag"]
    #[inline] pub fn ccf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CCF != 0"]
    #[inline] pub fn test_ccf(&self) -> bool {
        self.ccf() != 0
    }

    #[doc="Sets the CCF field."]
    #[inline] pub fn set_ccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TX Idle Interrupt Flag"]
    #[inline] pub fn txidle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXIDLE != 0"]
    #[inline] pub fn test_txidle(&self) -> bool {
        self.txidle() != 0
    }

    #[doc="Sets the TXIDLE field."]
    #[inline] pub fn set_txidle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Timer comparator 0 Interrupt Flag"]
    #[inline] pub fn tcmp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TCMP0 != 0"]
    #[inline] pub fn test_tcmp0(&self) -> bool {
        self.tcmp0() != 0
    }

    #[doc="Sets the TCMP0 field."]
    #[inline] pub fn set_tcmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Timer comparator 1 Interrupt Flag"]
    #[inline] pub fn tcmp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TCMP1 != 0"]
    #[inline] pub fn test_tcmp1(&self) -> bool {
        self.tcmp1() != 0
    }

    #[doc="Sets the TCMP1 field."]
    #[inline] pub fn set_tcmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Timer comparator 2 Interrupt Flag"]
    #[inline] pub fn tcmp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TCMP2 != 0"]
    #[inline] pub fn test_tcmp2(&self) -> bool {
        self.tcmp2() != 0
    }

    #[doc="Sets the TCMP2 field."]
    #[inline] pub fn set_tcmp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
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
        if self.rxfull() != 0 { try!(write!(f, " rxfull"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.txuf() != 0 { try!(write!(f, " txuf"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.mpaf() != 0 { try!(write!(f, " mpaf"))}
        if self.ssm() != 0 { try!(write!(f, " ssm"))}
        if self.ccf() != 0 { try!(write!(f, " ccf"))}
        if self.txidle() != 0 { try!(write!(f, " txidle"))}
        if self.tcmp0() != 0 { try!(write!(f, " tcmp0"))}
        if self.tcmp1() != 0 { try!(write!(f, " tcmp1"))}
        if self.tcmp2() != 0 { try!(write!(f, " tcmp2"))}
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

    #[doc="Set RXFULL Interrupt Flag"]
    #[inline] pub fn rxfull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFULL != 0"]
    #[inline] pub fn test_rxfull(&self) -> bool {
        self.rxfull() != 0
    }

    #[doc="Sets the RXFULL field."]
    #[inline] pub fn set_rxfull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Set RXOF Interrupt Flag"]
    #[inline] pub fn rxof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Set RXUF Interrupt Flag"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Set TXOF Interrupt Flag"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Set TXUF Interrupt Flag"]
    #[inline] pub fn txuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXUF != 0"]
    #[inline] pub fn test_txuf(&self) -> bool {
        self.txuf() != 0
    }

    #[doc="Sets the TXUF field."]
    #[inline] pub fn set_txuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Set PERR Interrupt Flag"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Set FERR Interrupt Flag"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Set MPAF Interrupt Flag"]
    #[inline] pub fn mpaf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if MPAF != 0"]
    #[inline] pub fn test_mpaf(&self) -> bool {
        self.mpaf() != 0
    }

    #[doc="Sets the MPAF field."]
    #[inline] pub fn set_mpaf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Set SSM Interrupt Flag"]
    #[inline] pub fn ssm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SSM != 0"]
    #[inline] pub fn test_ssm(&self) -> bool {
        self.ssm() != 0
    }

    #[doc="Sets the SSM field."]
    #[inline] pub fn set_ssm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Set CCF Interrupt Flag"]
    #[inline] pub fn ccf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CCF != 0"]
    #[inline] pub fn test_ccf(&self) -> bool {
        self.ccf() != 0
    }

    #[doc="Sets the CCF field."]
    #[inline] pub fn set_ccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Set TXIDLE Interrupt Flag"]
    #[inline] pub fn txidle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXIDLE != 0"]
    #[inline] pub fn test_txidle(&self) -> bool {
        self.txidle() != 0
    }

    #[doc="Sets the TXIDLE field."]
    #[inline] pub fn set_txidle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Set TCMP0 Interrupt Flag"]
    #[inline] pub fn tcmp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TCMP0 != 0"]
    #[inline] pub fn test_tcmp0(&self) -> bool {
        self.tcmp0() != 0
    }

    #[doc="Sets the TCMP0 field."]
    #[inline] pub fn set_tcmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Set TCMP1 Interrupt Flag"]
    #[inline] pub fn tcmp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TCMP1 != 0"]
    #[inline] pub fn test_tcmp1(&self) -> bool {
        self.tcmp1() != 0
    }

    #[doc="Sets the TCMP1 field."]
    #[inline] pub fn set_tcmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Set TCMP2 Interrupt Flag"]
    #[inline] pub fn tcmp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TCMP2 != 0"]
    #[inline] pub fn test_tcmp2(&self) -> bool {
        self.tcmp2() != 0
    }

    #[doc="Sets the TCMP2 field."]
    #[inline] pub fn set_tcmp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
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
        if self.rxfull() != 0 { try!(write!(f, " rxfull"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.txuf() != 0 { try!(write!(f, " txuf"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.mpaf() != 0 { try!(write!(f, " mpaf"))}
        if self.ssm() != 0 { try!(write!(f, " ssm"))}
        if self.ccf() != 0 { try!(write!(f, " ccf"))}
        if self.txidle() != 0 { try!(write!(f, " txidle"))}
        if self.tcmp0() != 0 { try!(write!(f, " tcmp0"))}
        if self.tcmp1() != 0 { try!(write!(f, " tcmp1"))}
        if self.tcmp2() != 0 { try!(write!(f, " tcmp2"))}
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

    #[doc="Clear RXFULL Interrupt Flag"]
    #[inline] pub fn rxfull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFULL != 0"]
    #[inline] pub fn test_rxfull(&self) -> bool {
        self.rxfull() != 0
    }

    #[doc="Sets the RXFULL field."]
    #[inline] pub fn set_rxfull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear RXOF Interrupt Flag"]
    #[inline] pub fn rxof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear RXUF Interrupt Flag"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear TXOF Interrupt Flag"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear TXUF Interrupt Flag"]
    #[inline] pub fn txuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXUF != 0"]
    #[inline] pub fn test_txuf(&self) -> bool {
        self.txuf() != 0
    }

    #[doc="Sets the TXUF field."]
    #[inline] pub fn set_txuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Clear PERR Interrupt Flag"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear FERR Interrupt Flag"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clear MPAF Interrupt Flag"]
    #[inline] pub fn mpaf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if MPAF != 0"]
    #[inline] pub fn test_mpaf(&self) -> bool {
        self.mpaf() != 0
    }

    #[doc="Sets the MPAF field."]
    #[inline] pub fn set_mpaf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clear SSM Interrupt Flag"]
    #[inline] pub fn ssm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SSM != 0"]
    #[inline] pub fn test_ssm(&self) -> bool {
        self.ssm() != 0
    }

    #[doc="Sets the SSM field."]
    #[inline] pub fn set_ssm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clear CCF Interrupt Flag"]
    #[inline] pub fn ccf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CCF != 0"]
    #[inline] pub fn test_ccf(&self) -> bool {
        self.ccf() != 0
    }

    #[doc="Sets the CCF field."]
    #[inline] pub fn set_ccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clear TXIDLE Interrupt Flag"]
    #[inline] pub fn txidle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXIDLE != 0"]
    #[inline] pub fn test_txidle(&self) -> bool {
        self.txidle() != 0
    }

    #[doc="Sets the TXIDLE field."]
    #[inline] pub fn set_txidle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Clear TCMP0 Interrupt Flag"]
    #[inline] pub fn tcmp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TCMP0 != 0"]
    #[inline] pub fn test_tcmp0(&self) -> bool {
        self.tcmp0() != 0
    }

    #[doc="Sets the TCMP0 field."]
    #[inline] pub fn set_tcmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Clear TCMP1 Interrupt Flag"]
    #[inline] pub fn tcmp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TCMP1 != 0"]
    #[inline] pub fn test_tcmp1(&self) -> bool {
        self.tcmp1() != 0
    }

    #[doc="Sets the TCMP1 field."]
    #[inline] pub fn set_tcmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Clear TCMP2 Interrupt Flag"]
    #[inline] pub fn tcmp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TCMP2 != 0"]
    #[inline] pub fn test_tcmp2(&self) -> bool {
        self.tcmp2() != 0
    }

    #[doc="Sets the TCMP2 field."]
    #[inline] pub fn set_tcmp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
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
        if self.rxfull() != 0 { try!(write!(f, " rxfull"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.txuf() != 0 { try!(write!(f, " txuf"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.mpaf() != 0 { try!(write!(f, " mpaf"))}
        if self.ssm() != 0 { try!(write!(f, " ssm"))}
        if self.ccf() != 0 { try!(write!(f, " ccf"))}
        if self.txidle() != 0 { try!(write!(f, " txidle"))}
        if self.tcmp0() != 0 { try!(write!(f, " tcmp0"))}
        if self.tcmp1() != 0 { try!(write!(f, " tcmp1"))}
        if self.tcmp2() != 0 { try!(write!(f, " tcmp2"))}
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

    #[doc="RXFULL Interrupt Enable"]
    #[inline] pub fn rxfull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFULL != 0"]
    #[inline] pub fn test_rxfull(&self) -> bool {
        self.rxfull() != 0
    }

    #[doc="Sets the RXFULL field."]
    #[inline] pub fn set_rxfull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RXOF Interrupt Enable"]
    #[inline] pub fn rxof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RXUF Interrupt Enable"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TXOF Interrupt Enable"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TXUF Interrupt Enable"]
    #[inline] pub fn txuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXUF != 0"]
    #[inline] pub fn test_txuf(&self) -> bool {
        self.txuf() != 0
    }

    #[doc="Sets the TXUF field."]
    #[inline] pub fn set_txuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="PERR Interrupt Enable"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FERR Interrupt Enable"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="MPAF Interrupt Enable"]
    #[inline] pub fn mpaf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if MPAF != 0"]
    #[inline] pub fn test_mpaf(&self) -> bool {
        self.mpaf() != 0
    }

    #[doc="Sets the MPAF field."]
    #[inline] pub fn set_mpaf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="SSM Interrupt Enable"]
    #[inline] pub fn ssm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SSM != 0"]
    #[inline] pub fn test_ssm(&self) -> bool {
        self.ssm() != 0
    }

    #[doc="Sets the SSM field."]
    #[inline] pub fn set_ssm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CCF Interrupt Enable"]
    #[inline] pub fn ccf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CCF != 0"]
    #[inline] pub fn test_ccf(&self) -> bool {
        self.ccf() != 0
    }

    #[doc="Sets the CCF field."]
    #[inline] pub fn set_ccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TXIDLE Interrupt Enable"]
    #[inline] pub fn txidle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TXIDLE != 0"]
    #[inline] pub fn test_txidle(&self) -> bool {
        self.txidle() != 0
    }

    #[doc="Sets the TXIDLE field."]
    #[inline] pub fn set_txidle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TCMP0 Interrupt Enable"]
    #[inline] pub fn tcmp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TCMP0 != 0"]
    #[inline] pub fn test_tcmp0(&self) -> bool {
        self.tcmp0() != 0
    }

    #[doc="Sets the TCMP0 field."]
    #[inline] pub fn set_tcmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TCMP1 Interrupt Enable"]
    #[inline] pub fn tcmp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TCMP1 != 0"]
    #[inline] pub fn test_tcmp1(&self) -> bool {
        self.tcmp1() != 0
    }

    #[doc="Sets the TCMP1 field."]
    #[inline] pub fn set_tcmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="TCMP2 Interrupt Enable"]
    #[inline] pub fn tcmp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TCMP2 != 0"]
    #[inline] pub fn test_tcmp2(&self) -> bool {
        self.tcmp2() != 0
    }

    #[doc="Sets the TCMP2 field."]
    #[inline] pub fn set_tcmp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
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
        if self.rxfull() != 0 { try!(write!(f, " rxfull"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.txuf() != 0 { try!(write!(f, " txuf"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.mpaf() != 0 { try!(write!(f, " mpaf"))}
        if self.ssm() != 0 { try!(write!(f, " ssm"))}
        if self.ccf() != 0 { try!(write!(f, " ccf"))}
        if self.txidle() != 0 { try!(write!(f, " txidle"))}
        if self.tcmp0() != 0 { try!(write!(f, " tcmp0"))}
        if self.tcmp1() != 0 { try!(write!(f, " tcmp1"))}
        if self.tcmp2() != 0 { try!(write!(f, " tcmp2"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IrDA Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Irctrl(pub u32);
impl Irctrl {
    #[doc="Enable IrDA Module"]
    #[inline] pub fn iren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IrDA TX Pulse Width"]
    #[inline] pub fn irpw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if IRPW != 0"]
    #[inline] pub fn test_irpw(&self) -> bool {
        self.irpw() != 0
    }

    #[doc="Sets the IRPW field."]
    #[inline] pub fn set_irpw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IrDA RX Filter"]
    #[inline] pub fn irfilt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IRFILT != 0"]
    #[inline] pub fn test_irfilt(&self) -> bool {
        self.irfilt() != 0
    }

    #[doc="Sets the IRFILT field."]
    #[inline] pub fn set_irfilt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IrDA PRS Channel Enable"]
    #[inline] pub fn irprsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IRPRSEN != 0"]
    #[inline] pub fn test_irprsen(&self) -> bool {
        self.irprsen() != 0
    }

    #[doc="Sets the IRPRSEN field."]
    #[inline] pub fn set_irprsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="IrDA PRS Channel Select"]
    #[inline] pub fn irprssel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if IRPRSSEL != 0"]
    #[inline] pub fn test_irprssel(&self) -> bool {
        self.irprssel() != 0
    }

    #[doc="Sets the IRPRSSEL field."]
    #[inline] pub fn set_irprssel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Irctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Irctrl(other)
    }
}

impl ::core::fmt::Display for Irctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Irctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iren() != 0 { try!(write!(f, " iren"))}
        if self.irpw() != 0 { try!(write!(f, " irpw=0x{:x}", self.irpw()))}
        if self.irfilt() != 0 { try!(write!(f, " irfilt"))}
        if self.irprsen() != 0 { try!(write!(f, " irprsen"))}
        if self.irprssel() != 0 { try!(write!(f, " irprssel=0x{:x}", self.irprssel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USART Input Register"]
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
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RXPRS != 0"]
    #[inline] pub fn test_rxprs(&self) -> bool {
        self.rxprs() != 0
    }

    #[doc="Sets the RXPRS field."]
    #[inline] pub fn set_rxprs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CLK PRS Channel Select"]
    #[inline] pub fn clkprssel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CLKPRSSEL != 0"]
    #[inline] pub fn test_clkprssel(&self) -> bool {
        self.clkprssel() != 0
    }

    #[doc="Sets the CLKPRSSEL field."]
    #[inline] pub fn set_clkprssel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PRS CLK Enable"]
    #[inline] pub fn clkprs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CLKPRS != 0"]
    #[inline] pub fn test_clkprs(&self) -> bool {
        self.clkprs() != 0
    }

    #[doc="Sets the CLKPRS field."]
    #[inline] pub fn set_clkprs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
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
        if self.clkprssel() != 0 { try!(write!(f, " clkprssel=0x{:x}", self.clkprssel()))}
        if self.clkprs() != 0 { try!(write!(f, " clkprs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2S Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct I2sctrl(pub u32);
impl I2sctrl {
    #[doc="Enable I2S Mode"]
    #[inline] pub fn en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Stero or Mono"]
    #[inline] pub fn mono(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MONO != 0"]
    #[inline] pub fn test_mono(&self) -> bool {
        self.mono() != 0
    }

    #[doc="Sets the MONO field."]
    #[inline] pub fn set_mono<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Justification of I2S Data"]
    #[inline] pub fn justify(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JUSTIFY != 0"]
    #[inline] pub fn test_justify(&self) -> bool {
        self.justify() != 0
    }

    #[doc="Sets the JUSTIFY field."]
    #[inline] pub fn set_justify<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Separate DMA Request For Left/Right Data"]
    #[inline] pub fn dmasplit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMASPLIT != 0"]
    #[inline] pub fn test_dmasplit(&self) -> bool {
        self.dmasplit() != 0
    }

    #[doc="Sets the DMASPLIT field."]
    #[inline] pub fn set_dmasplit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Delay on I2S data"]
    #[inline] pub fn delay(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DELAY != 0"]
    #[inline] pub fn test_delay(&self) -> bool {
        self.delay() != 0
    }

    #[doc="Sets the DELAY field."]
    #[inline] pub fn set_delay<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2S Word Format"]
    #[inline] pub fn format(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if FORMAT != 0"]
    #[inline] pub fn test_format(&self) -> bool {
        self.format() != 0
    }

    #[doc="Sets the FORMAT field."]
    #[inline] pub fn set_format<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for I2sctrl {
    #[inline]
    fn from(other: u32) -> Self {
         I2sctrl(other)
    }
}

impl ::core::fmt::Display for I2sctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for I2sctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.en() != 0 { try!(write!(f, " en"))}
        if self.mono() != 0 { try!(write!(f, " mono"))}
        if self.justify() != 0 { try!(write!(f, " justify"))}
        if self.dmasplit() != 0 { try!(write!(f, " dmasplit"))}
        if self.delay() != 0 { try!(write!(f, " delay"))}
        if self.format() != 0 { try!(write!(f, " format=0x{:x}", self.format()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timing Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timing(pub u32);
impl Timing {
    #[doc="TX frame start delay"]
    #[inline] pub fn txdelay(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if TXDELAY != 0"]
    #[inline] pub fn test_txdelay(&self) -> bool {
        self.txdelay() != 0
    }

    #[doc="Sets the TXDELAY field."]
    #[inline] pub fn set_txdelay<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Chip Select Setup"]
    #[inline] pub fn cssetup(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if CSSETUP != 0"]
    #[inline] pub fn test_cssetup(&self) -> bool {
        self.cssetup() != 0
    }

    #[doc="Sets the CSSETUP field."]
    #[inline] pub fn set_cssetup<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Inter-character spacing"]
    #[inline] pub fn ics(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ICS != 0"]
    #[inline] pub fn test_ics(&self) -> bool {
        self.ics() != 0
    }

    #[doc="Sets the ICS field."]
    #[inline] pub fn set_ics<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Chip Select Hold"]
    #[inline] pub fn cshold(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if CSHOLD != 0"]
    #[inline] pub fn test_cshold(&self) -> bool {
        self.cshold() != 0
    }

    #[doc="Sets the CSHOLD field."]
    #[inline] pub fn set_cshold<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Timing {
    #[inline]
    fn from(other: u32) -> Self {
         Timing(other)
    }
}

impl ::core::fmt::Display for Timing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdelay() != 0 { try!(write!(f, " txdelay=0x{:x}", self.txdelay()))}
        if self.cssetup() != 0 { try!(write!(f, " cssetup=0x{:x}", self.cssetup()))}
        if self.ics() != 0 { try!(write!(f, " ics=0x{:x}", self.ics()))}
        if self.cshold() != 0 { try!(write!(f, " cshold=0x{:x}", self.cshold()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control Register Extended"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlx(pub u32);
impl Ctrlx {
    #[doc="Debug halt"]
    #[inline] pub fn dbghalt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGHALT != 0"]
    #[inline] pub fn test_dbghalt(&self) -> bool {
        self.dbghalt() != 0
    }

    #[doc="Sets the DBGHALT field."]
    #[inline] pub fn set_dbghalt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CTS Pin Inversion"]
    #[inline] pub fn ctsinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSINV != 0"]
    #[inline] pub fn test_ctsinv(&self) -> bool {
        self.ctsinv() != 0
    }

    #[doc="Sets the CTSINV field."]
    #[inline] pub fn set_ctsinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CTS Function enabled"]
    #[inline] pub fn ctsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTSEN != 0"]
    #[inline] pub fn test_ctsen(&self) -> bool {
        self.ctsen() != 0
    }

    #[doc="Sets the CTSEN field."]
    #[inline] pub fn set_ctsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTS Pin Inversion"]
    #[inline] pub fn rtsinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTSINV != 0"]
    #[inline] pub fn test_rtsinv(&self) -> bool {
        self.rtsinv() != 0
    }

    #[doc="Sets the RTSINV field."]
    #[inline] pub fn set_rtsinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Ctrlx {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrlx(other)
    }
}

impl ::core::fmt::Display for Ctrlx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbghalt() != 0 { try!(write!(f, " dbghalt"))}
        if self.ctsinv() != 0 { try!(write!(f, " ctsinv"))}
        if self.ctsen() != 0 { try!(write!(f, " ctsen"))}
        if self.rtsinv() != 0 { try!(write!(f, " rtsinv"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Used to generate interrupts and various delays"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timecmp0(pub u32);
impl Timecmp0 {
    #[doc="Timer comparator 0."]
    #[inline] pub fn tcmpval(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TCMPVAL != 0"]
    #[inline] pub fn test_tcmpval(&self) -> bool {
        self.tcmpval() != 0
    }

    #[doc="Sets the TCMPVAL field."]
    #[inline] pub fn set_tcmpval<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer start source"]
    #[inline] pub fn tstart(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if TSTART != 0"]
    #[inline] pub fn test_tstart(&self) -> bool {
        self.tstart() != 0
    }

    #[doc="Sets the TSTART field."]
    #[inline] pub fn set_tstart<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Source used to disable comparator 0"]
    #[inline] pub fn tstop(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if TSTOP != 0"]
    #[inline] pub fn test_tstop(&self) -> bool {
        self.tstop() != 0
    }

    #[doc="Sets the TSTOP field."]
    #[inline] pub fn set_tstop<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Restart Timer on TCMP0"]
    #[inline] pub fn restarten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RESTARTEN != 0"]
    #[inline] pub fn test_restarten(&self) -> bool {
        self.restarten() != 0
    }

    #[doc="Sets the RESTARTEN field."]
    #[inline] pub fn set_restarten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Timecmp0 {
    #[inline]
    fn from(other: u32) -> Self {
         Timecmp0(other)
    }
}

impl ::core::fmt::Display for Timecmp0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timecmp0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcmpval() != 0 { try!(write!(f, " tcmpval=0x{:x}", self.tcmpval()))}
        if self.tstart() != 0 { try!(write!(f, " tstart=0x{:x}", self.tstart()))}
        if self.tstop() != 0 { try!(write!(f, " tstop=0x{:x}", self.tstop()))}
        if self.restarten() != 0 { try!(write!(f, " restarten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Used to generate interrupts and various delays"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timecmp1(pub u32);
impl Timecmp1 {
    #[doc="Timer comparator 1."]
    #[inline] pub fn tcmpval(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TCMPVAL != 0"]
    #[inline] pub fn test_tcmpval(&self) -> bool {
        self.tcmpval() != 0
    }

    #[doc="Sets the TCMPVAL field."]
    #[inline] pub fn set_tcmpval<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer start source"]
    #[inline] pub fn tstart(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if TSTART != 0"]
    #[inline] pub fn test_tstart(&self) -> bool {
        self.tstart() != 0
    }

    #[doc="Sets the TSTART field."]
    #[inline] pub fn set_tstart<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Source used to disable comparator 1"]
    #[inline] pub fn tstop(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if TSTOP != 0"]
    #[inline] pub fn test_tstop(&self) -> bool {
        self.tstop() != 0
    }

    #[doc="Sets the TSTOP field."]
    #[inline] pub fn set_tstop<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Restart Timer on TCMP1"]
    #[inline] pub fn restarten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RESTARTEN != 0"]
    #[inline] pub fn test_restarten(&self) -> bool {
        self.restarten() != 0
    }

    #[doc="Sets the RESTARTEN field."]
    #[inline] pub fn set_restarten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Timecmp1 {
    #[inline]
    fn from(other: u32) -> Self {
         Timecmp1(other)
    }
}

impl ::core::fmt::Display for Timecmp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timecmp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcmpval() != 0 { try!(write!(f, " tcmpval=0x{:x}", self.tcmpval()))}
        if self.tstart() != 0 { try!(write!(f, " tstart=0x{:x}", self.tstart()))}
        if self.tstop() != 0 { try!(write!(f, " tstop=0x{:x}", self.tstop()))}
        if self.restarten() != 0 { try!(write!(f, " restarten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Used to generate interrupts and various delays"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timecmp2(pub u32);
impl Timecmp2 {
    #[doc="Timer comparator 2."]
    #[inline] pub fn tcmpval(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TCMPVAL != 0"]
    #[inline] pub fn test_tcmpval(&self) -> bool {
        self.tcmpval() != 0
    }

    #[doc="Sets the TCMPVAL field."]
    #[inline] pub fn set_tcmpval<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer start source"]
    #[inline] pub fn tstart(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if TSTART != 0"]
    #[inline] pub fn test_tstart(&self) -> bool {
        self.tstart() != 0
    }

    #[doc="Sets the TSTART field."]
    #[inline] pub fn set_tstart<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Source used to disable comparator 2"]
    #[inline] pub fn tstop(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if TSTOP != 0"]
    #[inline] pub fn test_tstop(&self) -> bool {
        self.tstop() != 0
    }

    #[doc="Sets the TSTOP field."]
    #[inline] pub fn set_tstop<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Restart Timer on TCMP2"]
    #[inline] pub fn restarten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RESTARTEN != 0"]
    #[inline] pub fn test_restarten(&self) -> bool {
        self.restarten() != 0
    }

    #[doc="Sets the RESTARTEN field."]
    #[inline] pub fn set_restarten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Timecmp2 {
    #[inline]
    fn from(other: u32) -> Self {
         Timecmp2(other)
    }
}

impl ::core::fmt::Display for Timecmp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Timecmp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcmpval() != 0 { try!(write!(f, " tcmpval=0x{:x}", self.tcmpval()))}
        if self.tstart() != 0 { try!(write!(f, " tstart=0x{:x}", self.tstart()))}
        if self.tstop() != 0 { try!(write!(f, " tstop=0x{:x}", self.tstop()))}
        if self.restarten() != 0 { try!(write!(f, " restarten"))}
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

    #[doc="CS Pin Enable"]
    #[inline] pub fn cspen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CSPEN != 0"]
    #[inline] pub fn test_cspen(&self) -> bool {
        self.cspen() != 0
    }

    #[doc="Sets the CSPEN field."]
    #[inline] pub fn set_cspen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CLK Pin Enable"]
    #[inline] pub fn clkpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CLKPEN != 0"]
    #[inline] pub fn test_clkpen(&self) -> bool {
        self.clkpen() != 0
    }

    #[doc="Sets the CLKPEN field."]
    #[inline] pub fn set_clkpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="CTS Pin Enable"]
    #[inline] pub fn ctspen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTSPEN != 0"]
    #[inline] pub fn test_ctspen(&self) -> bool {
        self.ctspen() != 0
    }

    #[doc="Sets the CTSPEN field."]
    #[inline] pub fn set_ctspen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RTS Pin Enable"]
    #[inline] pub fn rtspen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RTSPEN != 0"]
    #[inline] pub fn test_rtspen(&self) -> bool {
        self.rtspen() != 0
    }

    #[doc="Sets the RTSPEN field."]
    #[inline] pub fn set_rtspen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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
        if self.cspen() != 0 { try!(write!(f, " cspen"))}
        if self.clkpen() != 0 { try!(write!(f, " clkpen"))}
        if self.ctspen() != 0 { try!(write!(f, " ctspen"))}
        if self.rtspen() != 0 { try!(write!(f, " rtspen"))}
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

    #[doc="I/O Location"]
    #[inline] pub fn csloc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if CSLOC != 0"]
    #[inline] pub fn test_csloc(&self) -> bool {
        self.csloc() != 0
    }

    #[doc="Sets the CSLOC field."]
    #[inline] pub fn set_csloc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="I/O Location"]
    #[inline] pub fn clkloc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if CLKLOC != 0"]
    #[inline] pub fn test_clkloc(&self) -> bool {
        self.clkloc() != 0
    }

    #[doc="Sets the CLKLOC field."]
    #[inline] pub fn set_clkloc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
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
        if self.csloc() != 0 { try!(write!(f, " csloc=0x{:x}", self.csloc()))}
        if self.clkloc() != 0 { try!(write!(f, " clkloc=0x{:x}", self.clkloc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Location Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routeloc1(pub u32);
impl Routeloc1 {
    #[doc="I/O Location"]
    #[inline] pub fn ctsloc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CTSLOC != 0"]
    #[inline] pub fn test_ctsloc(&self) -> bool {
        self.ctsloc() != 0
    }

    #[doc="Sets the CTSLOC field."]
    #[inline] pub fn set_ctsloc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I/O Location"]
    #[inline] pub fn rtsloc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if RTSLOC != 0"]
    #[inline] pub fn test_rtsloc(&self) -> bool {
        self.rtsloc() != 0
    }

    #[doc="Sets the RTSLOC field."]
    #[inline] pub fn set_rtsloc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Routeloc1 {
    #[inline]
    fn from(other: u32) -> Self {
         Routeloc1(other)
    }
}

impl ::core::fmt::Display for Routeloc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routeloc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctsloc() != 0 { try!(write!(f, " ctsloc=0x{:x}", self.ctsloc()))}
        if self.rtsloc() != 0 { try!(write!(f, " rtsloc=0x{:x}", self.rtsloc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


