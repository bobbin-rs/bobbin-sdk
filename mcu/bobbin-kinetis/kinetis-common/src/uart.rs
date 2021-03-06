
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UART Peripheral"]
pub struct UartPeriph(pub usize); 

impl UartPeriph {
    #[doc="Get the BDH Register."]
    #[inline] pub fn bdh_reg(&self) -> ::bobbin_mcu::register::Register<Bdh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdh, 0x0)
    }

    #[doc="Get the *mut pointer for the BDH register."]
    #[inline] pub fn bdh_mut(&self) -> *mut Bdh { 
        self.bdh_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDH register."]
    #[inline] pub fn bdh_ptr(&self) -> *const Bdh { 
        self.bdh_reg().ptr()
    }

    #[doc="Read the BDH register."]
    #[inline] pub fn bdh(&self) -> Bdh { 
        self.bdh_reg().read()
    }

    #[doc="Write the BDH register."]
    #[inline] pub fn write_bdh(&self, value: Bdh) -> &Self { 
        self.bdh_reg().write(value);
        self
    }

    #[doc="Set the BDH register."]
    #[inline] pub fn set_bdh<F: FnOnce(Bdh) -> Bdh>(&self, f: F) -> &Self {
        self.bdh_reg().set(f);
        self
    }

    #[doc="Modify the BDH register."]
    #[inline] pub fn with_bdh<F: FnOnce(Bdh) -> Bdh>(&self, f: F) -> &Self {
        self.bdh_reg().with(f);
        self
    }

    #[doc="Get the BDL Register."]
    #[inline] pub fn bdl_reg(&self) -> ::bobbin_mcu::register::Register<Bdl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdl, 0x1)
    }

    #[doc="Get the *mut pointer for the BDL register."]
    #[inline] pub fn bdl_mut(&self) -> *mut Bdl { 
        self.bdl_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDL register."]
    #[inline] pub fn bdl_ptr(&self) -> *const Bdl { 
        self.bdl_reg().ptr()
    }

    #[doc="Read the BDL register."]
    #[inline] pub fn bdl(&self) -> Bdl { 
        self.bdl_reg().read()
    }

    #[doc="Write the BDL register."]
    #[inline] pub fn write_bdl(&self, value: Bdl) -> &Self { 
        self.bdl_reg().write(value);
        self
    }

    #[doc="Set the BDL register."]
    #[inline] pub fn set_bdl<F: FnOnce(Bdl) -> Bdl>(&self, f: F) -> &Self {
        self.bdl_reg().set(f);
        self
    }

    #[doc="Modify the BDL register."]
    #[inline] pub fn with_bdl<F: FnOnce(Bdl) -> Bdl>(&self, f: F) -> &Self {
        self.bdl_reg().with(f);
        self
    }

    #[doc="Get the C1 Register."]
    #[inline] pub fn c1_reg(&self) -> ::bobbin_mcu::register::Register<C1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1, 0x2)
    }

    #[doc="Get the *mut pointer for the C1 register."]
    #[inline] pub fn c1_mut(&self) -> *mut C1 { 
        self.c1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1 register."]
    #[inline] pub fn c1_ptr(&self) -> *const C1 { 
        self.c1_reg().ptr()
    }

    #[doc="Read the C1 register."]
    #[inline] pub fn c1(&self) -> C1 { 
        self.c1_reg().read()
    }

    #[doc="Write the C1 register."]
    #[inline] pub fn write_c1(&self, value: C1) -> &Self { 
        self.c1_reg().write(value);
        self
    }

    #[doc="Set the C1 register."]
    #[inline] pub fn set_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        self.c1_reg().set(f);
        self
    }

    #[doc="Modify the C1 register."]
    #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        self.c1_reg().with(f);
        self
    }

    #[doc="Get the C2 Register."]
    #[inline] pub fn c2_reg(&self) -> ::bobbin_mcu::register::Register<C2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2, 0x3)
    }

    #[doc="Get the *mut pointer for the C2 register."]
    #[inline] pub fn c2_mut(&self) -> *mut C2 { 
        self.c2_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2 register."]
    #[inline] pub fn c2_ptr(&self) -> *const C2 { 
        self.c2_reg().ptr()
    }

    #[doc="Read the C2 register."]
    #[inline] pub fn c2(&self) -> C2 { 
        self.c2_reg().read()
    }

    #[doc="Write the C2 register."]
    #[inline] pub fn write_c2(&self, value: C2) -> &Self { 
        self.c2_reg().write(value);
        self
    }

    #[doc="Set the C2 register."]
    #[inline] pub fn set_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        self.c2_reg().set(f);
        self
    }

    #[doc="Modify the C2 register."]
    #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        self.c2_reg().with(f);
        self
    }

    #[doc="Get the S1 Register."]
    #[inline] pub fn s1_reg(&self) -> ::bobbin_mcu::register::Register<S1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut S1, 0x4)
    }

    #[doc="Get the *mut pointer for the S1 register."]
    #[inline] pub fn s1_mut(&self) -> *mut S1 { 
        self.s1_reg().ptr()
    }

    #[doc="Get the *const pointer for the S1 register."]
    #[inline] pub fn s1_ptr(&self) -> *const S1 { 
        self.s1_reg().ptr()
    }

    #[doc="Read the S1 register."]
    #[inline] pub fn s1(&self) -> S1 { 
        self.s1_reg().read()
    }

    #[doc="Get the S2 Register."]
    #[inline] pub fn s2_reg(&self) -> ::bobbin_mcu::register::Register<S2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut S2, 0x5)
    }

    #[doc="Get the *mut pointer for the S2 register."]
    #[inline] pub fn s2_mut(&self) -> *mut S2 { 
        self.s2_reg().ptr()
    }

    #[doc="Get the *const pointer for the S2 register."]
    #[inline] pub fn s2_ptr(&self) -> *const S2 { 
        self.s2_reg().ptr()
    }

    #[doc="Read the S2 register."]
    #[inline] pub fn s2(&self) -> S2 { 
        self.s2_reg().read()
    }

    #[doc="Write the S2 register."]
    #[inline] pub fn write_s2(&self, value: S2) -> &Self { 
        self.s2_reg().write(value);
        self
    }

    #[doc="Set the S2 register."]
    #[inline] pub fn set_s2<F: FnOnce(S2) -> S2>(&self, f: F) -> &Self {
        self.s2_reg().set(f);
        self
    }

    #[doc="Modify the S2 register."]
    #[inline] pub fn with_s2<F: FnOnce(S2) -> S2>(&self, f: F) -> &Self {
        self.s2_reg().with(f);
        self
    }

    #[doc="Get the C3 Register."]
    #[inline] pub fn c3_reg(&self) -> ::bobbin_mcu::register::Register<C3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C3, 0x6)
    }

    #[doc="Get the *mut pointer for the C3 register."]
    #[inline] pub fn c3_mut(&self) -> *mut C3 { 
        self.c3_reg().ptr()
    }

    #[doc="Get the *const pointer for the C3 register."]
    #[inline] pub fn c3_ptr(&self) -> *const C3 { 
        self.c3_reg().ptr()
    }

    #[doc="Read the C3 register."]
    #[inline] pub fn c3(&self) -> C3 { 
        self.c3_reg().read()
    }

    #[doc="Write the C3 register."]
    #[inline] pub fn write_c3(&self, value: C3) -> &Self { 
        self.c3_reg().write(value);
        self
    }

    #[doc="Set the C3 register."]
    #[inline] pub fn set_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
        self.c3_reg().set(f);
        self
    }

    #[doc="Modify the C3 register."]
    #[inline] pub fn with_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
        self.c3_reg().with(f);
        self
    }

    #[doc="Get the D Register."]
    #[inline] pub fn d_reg(&self) -> ::bobbin_mcu::register::Register<D> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut D, 0x7)
    }

    #[doc="Get the *mut pointer for the D register."]
    #[inline] pub fn d_mut(&self) -> *mut D { 
        self.d_reg().ptr()
    }

    #[doc="Get the *const pointer for the D register."]
    #[inline] pub fn d_ptr(&self) -> *const D { 
        self.d_reg().ptr()
    }

    #[doc="Read the D register."]
    #[inline] pub fn d(&self) -> D { 
        self.d_reg().read()
    }

    #[doc="Write the D register."]
    #[inline] pub fn write_d(&self, value: D) -> &Self { 
        self.d_reg().write(value);
        self
    }

    #[doc="Set the D register."]
    #[inline] pub fn set_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
        self.d_reg().set(f);
        self
    }

    #[doc="Modify the D register."]
    #[inline] pub fn with_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
        self.d_reg().with(f);
        self
    }

    #[doc="Get the MA1 Register."]
    #[inline] pub fn ma1_reg(&self) -> ::bobbin_mcu::register::Register<Ma1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ma1, 0x8)
    }

    #[doc="Get the *mut pointer for the MA1 register."]
    #[inline] pub fn ma1_mut(&self) -> *mut Ma1 { 
        self.ma1_reg().ptr()
    }

    #[doc="Get the *const pointer for the MA1 register."]
    #[inline] pub fn ma1_ptr(&self) -> *const Ma1 { 
        self.ma1_reg().ptr()
    }

    #[doc="Read the MA1 register."]
    #[inline] pub fn ma1(&self) -> Ma1 { 
        self.ma1_reg().read()
    }

    #[doc="Write the MA1 register."]
    #[inline] pub fn write_ma1(&self, value: Ma1) -> &Self { 
        self.ma1_reg().write(value);
        self
    }

    #[doc="Set the MA1 register."]
    #[inline] pub fn set_ma1<F: FnOnce(Ma1) -> Ma1>(&self, f: F) -> &Self {
        self.ma1_reg().set(f);
        self
    }

    #[doc="Modify the MA1 register."]
    #[inline] pub fn with_ma1<F: FnOnce(Ma1) -> Ma1>(&self, f: F) -> &Self {
        self.ma1_reg().with(f);
        self
    }

    #[doc="Get the MA2 Register."]
    #[inline] pub fn ma2_reg(&self) -> ::bobbin_mcu::register::Register<Ma2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ma2, 0x9)
    }

    #[doc="Get the *mut pointer for the MA2 register."]
    #[inline] pub fn ma2_mut(&self) -> *mut Ma2 { 
        self.ma2_reg().ptr()
    }

    #[doc="Get the *const pointer for the MA2 register."]
    #[inline] pub fn ma2_ptr(&self) -> *const Ma2 { 
        self.ma2_reg().ptr()
    }

    #[doc="Read the MA2 register."]
    #[inline] pub fn ma2(&self) -> Ma2 { 
        self.ma2_reg().read()
    }

    #[doc="Write the MA2 register."]
    #[inline] pub fn write_ma2(&self, value: Ma2) -> &Self { 
        self.ma2_reg().write(value);
        self
    }

    #[doc="Set the MA2 register."]
    #[inline] pub fn set_ma2<F: FnOnce(Ma2) -> Ma2>(&self, f: F) -> &Self {
        self.ma2_reg().set(f);
        self
    }

    #[doc="Modify the MA2 register."]
    #[inline] pub fn with_ma2<F: FnOnce(Ma2) -> Ma2>(&self, f: F) -> &Self {
        self.ma2_reg().with(f);
        self
    }

    #[doc="Get the C4 Register."]
    #[inline] pub fn c4_reg(&self) -> ::bobbin_mcu::register::Register<C4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C4, 0xa)
    }

    #[doc="Get the *mut pointer for the C4 register."]
    #[inline] pub fn c4_mut(&self) -> *mut C4 { 
        self.c4_reg().ptr()
    }

    #[doc="Get the *const pointer for the C4 register."]
    #[inline] pub fn c4_ptr(&self) -> *const C4 { 
        self.c4_reg().ptr()
    }

    #[doc="Read the C4 register."]
    #[inline] pub fn c4(&self) -> C4 { 
        self.c4_reg().read()
    }

    #[doc="Write the C4 register."]
    #[inline] pub fn write_c4(&self, value: C4) -> &Self { 
        self.c4_reg().write(value);
        self
    }

    #[doc="Set the C4 register."]
    #[inline] pub fn set_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
        self.c4_reg().set(f);
        self
    }

    #[doc="Modify the C4 register."]
    #[inline] pub fn with_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
        self.c4_reg().with(f);
        self
    }

    #[doc="Get the C5 Register."]
    #[inline] pub fn c5_reg(&self) -> ::bobbin_mcu::register::Register<C5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C5, 0xb)
    }

    #[doc="Get the *mut pointer for the C5 register."]
    #[inline] pub fn c5_mut(&self) -> *mut C5 { 
        self.c5_reg().ptr()
    }

    #[doc="Get the *const pointer for the C5 register."]
    #[inline] pub fn c5_ptr(&self) -> *const C5 { 
        self.c5_reg().ptr()
    }

    #[doc="Read the C5 register."]
    #[inline] pub fn c5(&self) -> C5 { 
        self.c5_reg().read()
    }

    #[doc="Write the C5 register."]
    #[inline] pub fn write_c5(&self, value: C5) -> &Self { 
        self.c5_reg().write(value);
        self
    }

    #[doc="Set the C5 register."]
    #[inline] pub fn set_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
        self.c5_reg().set(f);
        self
    }

    #[doc="Modify the C5 register."]
    #[inline] pub fn with_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
        self.c5_reg().with(f);
        self
    }

    #[doc="Get the ED Register."]
    #[inline] pub fn ed_reg(&self) -> ::bobbin_mcu::register::Register<Ed> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ed, 0xc)
    }

    #[doc="Get the *mut pointer for the ED register."]
    #[inline] pub fn ed_mut(&self) -> *mut Ed { 
        self.ed_reg().ptr()
    }

    #[doc="Get the *const pointer for the ED register."]
    #[inline] pub fn ed_ptr(&self) -> *const Ed { 
        self.ed_reg().ptr()
    }

    #[doc="Read the ED register."]
    #[inline] pub fn ed(&self) -> Ed { 
        self.ed_reg().read()
    }

    #[doc="Get the MODEM Register."]
    #[inline] pub fn modem_reg(&self) -> ::bobbin_mcu::register::Register<Modem> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Modem, 0xd)
    }

    #[doc="Get the *mut pointer for the MODEM register."]
    #[inline] pub fn modem_mut(&self) -> *mut Modem { 
        self.modem_reg().ptr()
    }

    #[doc="Get the *const pointer for the MODEM register."]
    #[inline] pub fn modem_ptr(&self) -> *const Modem { 
        self.modem_reg().ptr()
    }

    #[doc="Read the MODEM register."]
    #[inline] pub fn modem(&self) -> Modem { 
        self.modem_reg().read()
    }

    #[doc="Write the MODEM register."]
    #[inline] pub fn write_modem(&self, value: Modem) -> &Self { 
        self.modem_reg().write(value);
        self
    }

    #[doc="Set the MODEM register."]
    #[inline] pub fn set_modem<F: FnOnce(Modem) -> Modem>(&self, f: F) -> &Self {
        self.modem_reg().set(f);
        self
    }

    #[doc="Modify the MODEM register."]
    #[inline] pub fn with_modem<F: FnOnce(Modem) -> Modem>(&self, f: F) -> &Self {
        self.modem_reg().with(f);
        self
    }

    #[doc="Get the IR Register."]
    #[inline] pub fn ir_reg(&self) -> ::bobbin_mcu::register::Register<Ir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ir, 0xe)
    }

    #[doc="Get the *mut pointer for the IR register."]
    #[inline] pub fn ir_mut(&self) -> *mut Ir { 
        self.ir_reg().ptr()
    }

    #[doc="Get the *const pointer for the IR register."]
    #[inline] pub fn ir_ptr(&self) -> *const Ir { 
        self.ir_reg().ptr()
    }

    #[doc="Read the IR register."]
    #[inline] pub fn ir(&self) -> Ir { 
        self.ir_reg().read()
    }

    #[doc="Write the IR register."]
    #[inline] pub fn write_ir(&self, value: Ir) -> &Self { 
        self.ir_reg().write(value);
        self
    }

    #[doc="Set the IR register."]
    #[inline] pub fn set_ir<F: FnOnce(Ir) -> Ir>(&self, f: F) -> &Self {
        self.ir_reg().set(f);
        self
    }

    #[doc="Modify the IR register."]
    #[inline] pub fn with_ir<F: FnOnce(Ir) -> Ir>(&self, f: F) -> &Self {
        self.ir_reg().with(f);
        self
    }

    #[doc="Get the PFIFO Register."]
    #[inline] pub fn pfifo_reg(&self) -> ::bobbin_mcu::register::Register<Pfifo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pfifo, 0x10)
    }

    #[doc="Get the *mut pointer for the PFIFO register."]
    #[inline] pub fn pfifo_mut(&self) -> *mut Pfifo { 
        self.pfifo_reg().ptr()
    }

    #[doc="Get the *const pointer for the PFIFO register."]
    #[inline] pub fn pfifo_ptr(&self) -> *const Pfifo { 
        self.pfifo_reg().ptr()
    }

    #[doc="Read the PFIFO register."]
    #[inline] pub fn pfifo(&self) -> Pfifo { 
        self.pfifo_reg().read()
    }

    #[doc="Write the PFIFO register."]
    #[inline] pub fn write_pfifo(&self, value: Pfifo) -> &Self { 
        self.pfifo_reg().write(value);
        self
    }

    #[doc="Set the PFIFO register."]
    #[inline] pub fn set_pfifo<F: FnOnce(Pfifo) -> Pfifo>(&self, f: F) -> &Self {
        self.pfifo_reg().set(f);
        self
    }

    #[doc="Modify the PFIFO register."]
    #[inline] pub fn with_pfifo<F: FnOnce(Pfifo) -> Pfifo>(&self, f: F) -> &Self {
        self.pfifo_reg().with(f);
        self
    }

    #[doc="Get the CFIFO Register."]
    #[inline] pub fn cfifo_reg(&self) -> ::bobbin_mcu::register::Register<Cfifo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfifo, 0x11)
    }

    #[doc="Get the *mut pointer for the CFIFO register."]
    #[inline] pub fn cfifo_mut(&self) -> *mut Cfifo { 
        self.cfifo_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFIFO register."]
    #[inline] pub fn cfifo_ptr(&self) -> *const Cfifo { 
        self.cfifo_reg().ptr()
    }

    #[doc="Read the CFIFO register."]
    #[inline] pub fn cfifo(&self) -> Cfifo { 
        self.cfifo_reg().read()
    }

    #[doc="Write the CFIFO register."]
    #[inline] pub fn write_cfifo(&self, value: Cfifo) -> &Self { 
        self.cfifo_reg().write(value);
        self
    }

    #[doc="Set the CFIFO register."]
    #[inline] pub fn set_cfifo<F: FnOnce(Cfifo) -> Cfifo>(&self, f: F) -> &Self {
        self.cfifo_reg().set(f);
        self
    }

    #[doc="Modify the CFIFO register."]
    #[inline] pub fn with_cfifo<F: FnOnce(Cfifo) -> Cfifo>(&self, f: F) -> &Self {
        self.cfifo_reg().with(f);
        self
    }

    #[doc="Get the SFIFO Register."]
    #[inline] pub fn sfifo_reg(&self) -> ::bobbin_mcu::register::Register<Sfifo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sfifo, 0x12)
    }

    #[doc="Get the *mut pointer for the SFIFO register."]
    #[inline] pub fn sfifo_mut(&self) -> *mut Sfifo { 
        self.sfifo_reg().ptr()
    }

    #[doc="Get the *const pointer for the SFIFO register."]
    #[inline] pub fn sfifo_ptr(&self) -> *const Sfifo { 
        self.sfifo_reg().ptr()
    }

    #[doc="Read the SFIFO register."]
    #[inline] pub fn sfifo(&self) -> Sfifo { 
        self.sfifo_reg().read()
    }

    #[doc="Write the SFIFO register."]
    #[inline] pub fn write_sfifo(&self, value: Sfifo) -> &Self { 
        self.sfifo_reg().write(value);
        self
    }

    #[doc="Set the SFIFO register."]
    #[inline] pub fn set_sfifo<F: FnOnce(Sfifo) -> Sfifo>(&self, f: F) -> &Self {
        self.sfifo_reg().set(f);
        self
    }

    #[doc="Modify the SFIFO register."]
    #[inline] pub fn with_sfifo<F: FnOnce(Sfifo) -> Sfifo>(&self, f: F) -> &Self {
        self.sfifo_reg().with(f);
        self
    }

    #[doc="Get the TWFIFO Register."]
    #[inline] pub fn twfifo_reg(&self) -> ::bobbin_mcu::register::Register<Twfifo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Twfifo, 0x13)
    }

    #[doc="Get the *mut pointer for the TWFIFO register."]
    #[inline] pub fn twfifo_mut(&self) -> *mut Twfifo { 
        self.twfifo_reg().ptr()
    }

    #[doc="Get the *const pointer for the TWFIFO register."]
    #[inline] pub fn twfifo_ptr(&self) -> *const Twfifo { 
        self.twfifo_reg().ptr()
    }

    #[doc="Read the TWFIFO register."]
    #[inline] pub fn twfifo(&self) -> Twfifo { 
        self.twfifo_reg().read()
    }

    #[doc="Write the TWFIFO register."]
    #[inline] pub fn write_twfifo(&self, value: Twfifo) -> &Self { 
        self.twfifo_reg().write(value);
        self
    }

    #[doc="Set the TWFIFO register."]
    #[inline] pub fn set_twfifo<F: FnOnce(Twfifo) -> Twfifo>(&self, f: F) -> &Self {
        self.twfifo_reg().set(f);
        self
    }

    #[doc="Modify the TWFIFO register."]
    #[inline] pub fn with_twfifo<F: FnOnce(Twfifo) -> Twfifo>(&self, f: F) -> &Self {
        self.twfifo_reg().with(f);
        self
    }

    #[doc="Get the TCFIFO Register."]
    #[inline] pub fn tcfifo_reg(&self) -> ::bobbin_mcu::register::Register<Tcfifo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tcfifo, 0x14)
    }

    #[doc="Get the *mut pointer for the TCFIFO register."]
    #[inline] pub fn tcfifo_mut(&self) -> *mut Tcfifo { 
        self.tcfifo_reg().ptr()
    }

    #[doc="Get the *const pointer for the TCFIFO register."]
    #[inline] pub fn tcfifo_ptr(&self) -> *const Tcfifo { 
        self.tcfifo_reg().ptr()
    }

    #[doc="Read the TCFIFO register."]
    #[inline] pub fn tcfifo(&self) -> Tcfifo { 
        self.tcfifo_reg().read()
    }

    #[doc="Get the RWFIFO Register."]
    #[inline] pub fn rwfifo_reg(&self) -> ::bobbin_mcu::register::Register<Rwfifo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rwfifo, 0x15)
    }

    #[doc="Get the *mut pointer for the RWFIFO register."]
    #[inline] pub fn rwfifo_mut(&self) -> *mut Rwfifo { 
        self.rwfifo_reg().ptr()
    }

    #[doc="Get the *const pointer for the RWFIFO register."]
    #[inline] pub fn rwfifo_ptr(&self) -> *const Rwfifo { 
        self.rwfifo_reg().ptr()
    }

    #[doc="Read the RWFIFO register."]
    #[inline] pub fn rwfifo(&self) -> Rwfifo { 
        self.rwfifo_reg().read()
    }

    #[doc="Write the RWFIFO register."]
    #[inline] pub fn write_rwfifo(&self, value: Rwfifo) -> &Self { 
        self.rwfifo_reg().write(value);
        self
    }

    #[doc="Set the RWFIFO register."]
    #[inline] pub fn set_rwfifo<F: FnOnce(Rwfifo) -> Rwfifo>(&self, f: F) -> &Self {
        self.rwfifo_reg().set(f);
        self
    }

    #[doc="Modify the RWFIFO register."]
    #[inline] pub fn with_rwfifo<F: FnOnce(Rwfifo) -> Rwfifo>(&self, f: F) -> &Self {
        self.rwfifo_reg().with(f);
        self
    }

    #[doc="Get the RCFIFO Register."]
    #[inline] pub fn rcfifo_reg(&self) -> ::bobbin_mcu::register::Register<Rcfifo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rcfifo, 0x16)
    }

    #[doc="Get the *mut pointer for the RCFIFO register."]
    #[inline] pub fn rcfifo_mut(&self) -> *mut Rcfifo { 
        self.rcfifo_reg().ptr()
    }

    #[doc="Get the *const pointer for the RCFIFO register."]
    #[inline] pub fn rcfifo_ptr(&self) -> *const Rcfifo { 
        self.rcfifo_reg().ptr()
    }

    #[doc="Read the RCFIFO register."]
    #[inline] pub fn rcfifo(&self) -> Rcfifo { 
        self.rcfifo_reg().read()
    }

    #[doc="Get the C7816 Register."]
    #[inline] pub fn c7816_reg(&self) -> ::bobbin_mcu::register::Register<C7816> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C7816, 0x18)
    }

    #[doc="Get the *mut pointer for the C7816 register."]
    #[inline] pub fn c7816_mut(&self) -> *mut C7816 { 
        self.c7816_reg().ptr()
    }

    #[doc="Get the *const pointer for the C7816 register."]
    #[inline] pub fn c7816_ptr(&self) -> *const C7816 { 
        self.c7816_reg().ptr()
    }

    #[doc="Read the C7816 register."]
    #[inline] pub fn c7816(&self) -> C7816 { 
        self.c7816_reg().read()
    }

    #[doc="Write the C7816 register."]
    #[inline] pub fn write_c7816(&self, value: C7816) -> &Self { 
        self.c7816_reg().write(value);
        self
    }

    #[doc="Set the C7816 register."]
    #[inline] pub fn set_c7816<F: FnOnce(C7816) -> C7816>(&self, f: F) -> &Self {
        self.c7816_reg().set(f);
        self
    }

    #[doc="Modify the C7816 register."]
    #[inline] pub fn with_c7816<F: FnOnce(C7816) -> C7816>(&self, f: F) -> &Self {
        self.c7816_reg().with(f);
        self
    }

    #[doc="Get the IE7816 Register."]
    #[inline] pub fn ie7816_reg(&self) -> ::bobbin_mcu::register::Register<Ie7816> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ie7816, 0x19)
    }

    #[doc="Get the *mut pointer for the IE7816 register."]
    #[inline] pub fn ie7816_mut(&self) -> *mut Ie7816 { 
        self.ie7816_reg().ptr()
    }

    #[doc="Get the *const pointer for the IE7816 register."]
    #[inline] pub fn ie7816_ptr(&self) -> *const Ie7816 { 
        self.ie7816_reg().ptr()
    }

    #[doc="Read the IE7816 register."]
    #[inline] pub fn ie7816(&self) -> Ie7816 { 
        self.ie7816_reg().read()
    }

    #[doc="Write the IE7816 register."]
    #[inline] pub fn write_ie7816(&self, value: Ie7816) -> &Self { 
        self.ie7816_reg().write(value);
        self
    }

    #[doc="Set the IE7816 register."]
    #[inline] pub fn set_ie7816<F: FnOnce(Ie7816) -> Ie7816>(&self, f: F) -> &Self {
        self.ie7816_reg().set(f);
        self
    }

    #[doc="Modify the IE7816 register."]
    #[inline] pub fn with_ie7816<F: FnOnce(Ie7816) -> Ie7816>(&self, f: F) -> &Self {
        self.ie7816_reg().with(f);
        self
    }

    #[doc="Get the IS7816 Register."]
    #[inline] pub fn is7816_reg(&self) -> ::bobbin_mcu::register::Register<Is7816> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Is7816, 0x1a)
    }

    #[doc="Get the *mut pointer for the IS7816 register."]
    #[inline] pub fn is7816_mut(&self) -> *mut Is7816 { 
        self.is7816_reg().ptr()
    }

    #[doc="Get the *const pointer for the IS7816 register."]
    #[inline] pub fn is7816_ptr(&self) -> *const Is7816 { 
        self.is7816_reg().ptr()
    }

    #[doc="Read the IS7816 register."]
    #[inline] pub fn is7816(&self) -> Is7816 { 
        self.is7816_reg().read()
    }

    #[doc="Write the IS7816 register."]
    #[inline] pub fn write_is7816(&self, value: Is7816) -> &Self { 
        self.is7816_reg().write(value);
        self
    }

    #[doc="Set the IS7816 register."]
    #[inline] pub fn set_is7816<F: FnOnce(Is7816) -> Is7816>(&self, f: F) -> &Self {
        self.is7816_reg().set(f);
        self
    }

    #[doc="Modify the IS7816 register."]
    #[inline] pub fn with_is7816<F: FnOnce(Is7816) -> Is7816>(&self, f: F) -> &Self {
        self.is7816_reg().with(f);
        self
    }

    #[doc="Get the WP7816T0 Register."]
    #[inline] pub fn wp7816t0_reg(&self) -> ::bobbin_mcu::register::Register<Wp7816t0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wp7816t0, 0x1b)
    }

    #[doc="Get the *mut pointer for the WP7816T0 register."]
    #[inline] pub fn wp7816t0_mut(&self) -> *mut Wp7816t0 { 
        self.wp7816t0_reg().ptr()
    }

    #[doc="Get the *const pointer for the WP7816T0 register."]
    #[inline] pub fn wp7816t0_ptr(&self) -> *const Wp7816t0 { 
        self.wp7816t0_reg().ptr()
    }

    #[doc="Read the WP7816T0 register."]
    #[inline] pub fn wp7816t0(&self) -> Wp7816t0 { 
        self.wp7816t0_reg().read()
    }

    #[doc="Write the WP7816T0 register."]
    #[inline] pub fn write_wp7816t0(&self, value: Wp7816t0) -> &Self { 
        self.wp7816t0_reg().write(value);
        self
    }

    #[doc="Set the WP7816T0 register."]
    #[inline] pub fn set_wp7816t0<F: FnOnce(Wp7816t0) -> Wp7816t0>(&self, f: F) -> &Self {
        self.wp7816t0_reg().set(f);
        self
    }

    #[doc="Modify the WP7816T0 register."]
    #[inline] pub fn with_wp7816t0<F: FnOnce(Wp7816t0) -> Wp7816t0>(&self, f: F) -> &Self {
        self.wp7816t0_reg().with(f);
        self
    }

    #[doc="Get the WP7816T1 Register."]
    #[inline] pub fn wp7816t1_reg(&self) -> ::bobbin_mcu::register::Register<Wp7816t1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wp7816t1, 0x1b)
    }

    #[doc="Get the *mut pointer for the WP7816T1 register."]
    #[inline] pub fn wp7816t1_mut(&self) -> *mut Wp7816t1 { 
        self.wp7816t1_reg().ptr()
    }

    #[doc="Get the *const pointer for the WP7816T1 register."]
    #[inline] pub fn wp7816t1_ptr(&self) -> *const Wp7816t1 { 
        self.wp7816t1_reg().ptr()
    }

    #[doc="Read the WP7816T1 register."]
    #[inline] pub fn wp7816t1(&self) -> Wp7816t1 { 
        self.wp7816t1_reg().read()
    }

    #[doc="Write the WP7816T1 register."]
    #[inline] pub fn write_wp7816t1(&self, value: Wp7816t1) -> &Self { 
        self.wp7816t1_reg().write(value);
        self
    }

    #[doc="Set the WP7816T1 register."]
    #[inline] pub fn set_wp7816t1<F: FnOnce(Wp7816t1) -> Wp7816t1>(&self, f: F) -> &Self {
        self.wp7816t1_reg().set(f);
        self
    }

    #[doc="Modify the WP7816T1 register."]
    #[inline] pub fn with_wp7816t1<F: FnOnce(Wp7816t1) -> Wp7816t1>(&self, f: F) -> &Self {
        self.wp7816t1_reg().with(f);
        self
    }

    #[doc="Get the WN7816 Register."]
    #[inline] pub fn wn7816_reg(&self) -> ::bobbin_mcu::register::Register<Wn7816> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wn7816, 0x1c)
    }

    #[doc="Get the *mut pointer for the WN7816 register."]
    #[inline] pub fn wn7816_mut(&self) -> *mut Wn7816 { 
        self.wn7816_reg().ptr()
    }

    #[doc="Get the *const pointer for the WN7816 register."]
    #[inline] pub fn wn7816_ptr(&self) -> *const Wn7816 { 
        self.wn7816_reg().ptr()
    }

    #[doc="Read the WN7816 register."]
    #[inline] pub fn wn7816(&self) -> Wn7816 { 
        self.wn7816_reg().read()
    }

    #[doc="Write the WN7816 register."]
    #[inline] pub fn write_wn7816(&self, value: Wn7816) -> &Self { 
        self.wn7816_reg().write(value);
        self
    }

    #[doc="Set the WN7816 register."]
    #[inline] pub fn set_wn7816<F: FnOnce(Wn7816) -> Wn7816>(&self, f: F) -> &Self {
        self.wn7816_reg().set(f);
        self
    }

    #[doc="Modify the WN7816 register."]
    #[inline] pub fn with_wn7816<F: FnOnce(Wn7816) -> Wn7816>(&self, f: F) -> &Self {
        self.wn7816_reg().with(f);
        self
    }

    #[doc="Get the WF7816 Register."]
    #[inline] pub fn wf7816_reg(&self) -> ::bobbin_mcu::register::Register<Wf7816> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wf7816, 0x1d)
    }

    #[doc="Get the *mut pointer for the WF7816 register."]
    #[inline] pub fn wf7816_mut(&self) -> *mut Wf7816 { 
        self.wf7816_reg().ptr()
    }

    #[doc="Get the *const pointer for the WF7816 register."]
    #[inline] pub fn wf7816_ptr(&self) -> *const Wf7816 { 
        self.wf7816_reg().ptr()
    }

    #[doc="Read the WF7816 register."]
    #[inline] pub fn wf7816(&self) -> Wf7816 { 
        self.wf7816_reg().read()
    }

    #[doc="Write the WF7816 register."]
    #[inline] pub fn write_wf7816(&self, value: Wf7816) -> &Self { 
        self.wf7816_reg().write(value);
        self
    }

    #[doc="Set the WF7816 register."]
    #[inline] pub fn set_wf7816<F: FnOnce(Wf7816) -> Wf7816>(&self, f: F) -> &Self {
        self.wf7816_reg().set(f);
        self
    }

    #[doc="Modify the WF7816 register."]
    #[inline] pub fn with_wf7816<F: FnOnce(Wf7816) -> Wf7816>(&self, f: F) -> &Self {
        self.wf7816_reg().with(f);
        self
    }

    #[doc="Get the ET7816 Register."]
    #[inline] pub fn et7816_reg(&self) -> ::bobbin_mcu::register::Register<Et7816> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Et7816, 0x1e)
    }

    #[doc="Get the *mut pointer for the ET7816 register."]
    #[inline] pub fn et7816_mut(&self) -> *mut Et7816 { 
        self.et7816_reg().ptr()
    }

    #[doc="Get the *const pointer for the ET7816 register."]
    #[inline] pub fn et7816_ptr(&self) -> *const Et7816 { 
        self.et7816_reg().ptr()
    }

    #[doc="Read the ET7816 register."]
    #[inline] pub fn et7816(&self) -> Et7816 { 
        self.et7816_reg().read()
    }

    #[doc="Write the ET7816 register."]
    #[inline] pub fn write_et7816(&self, value: Et7816) -> &Self { 
        self.et7816_reg().write(value);
        self
    }

    #[doc="Set the ET7816 register."]
    #[inline] pub fn set_et7816<F: FnOnce(Et7816) -> Et7816>(&self, f: F) -> &Self {
        self.et7816_reg().set(f);
        self
    }

    #[doc="Modify the ET7816 register."]
    #[inline] pub fn with_et7816<F: FnOnce(Et7816) -> Et7816>(&self, f: F) -> &Self {
        self.et7816_reg().with(f);
        self
    }

    #[doc="Get the TL7816 Register."]
    #[inline] pub fn tl7816_reg(&self) -> ::bobbin_mcu::register::Register<Tl7816> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tl7816, 0x1f)
    }

    #[doc="Get the *mut pointer for the TL7816 register."]
    #[inline] pub fn tl7816_mut(&self) -> *mut Tl7816 { 
        self.tl7816_reg().ptr()
    }

    #[doc="Get the *const pointer for the TL7816 register."]
    #[inline] pub fn tl7816_ptr(&self) -> *const Tl7816 { 
        self.tl7816_reg().ptr()
    }

    #[doc="Read the TL7816 register."]
    #[inline] pub fn tl7816(&self) -> Tl7816 { 
        self.tl7816_reg().read()
    }

    #[doc="Write the TL7816 register."]
    #[inline] pub fn write_tl7816(&self, value: Tl7816) -> &Self { 
        self.tl7816_reg().write(value);
        self
    }

    #[doc="Set the TL7816 register."]
    #[inline] pub fn set_tl7816<F: FnOnce(Tl7816) -> Tl7816>(&self, f: F) -> &Self {
        self.tl7816_reg().set(f);
        self
    }

    #[doc="Modify the TL7816 register."]
    #[inline] pub fn with_tl7816<F: FnOnce(Tl7816) -> Tl7816>(&self, f: F) -> &Self {
        self.tl7816_reg().with(f);
        self
    }

}

#[doc="UART Baud Rate Registers: High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdh(pub u8);
impl Bdh {
    #[doc="UART Baud Rate Bits"]
    #[inline] pub fn sbr(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SBR != 0"]
    #[inline] pub fn test_sbr(&self) -> bool {
        self.sbr() != 0
    }

    #[doc="Sets the SBR field."]
    #[inline] pub fn set_sbr<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Stop Bit Number Select"]
    #[inline] pub fn sbns(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SBNS != 0"]
    #[inline] pub fn test_sbns(&self) -> bool {
        self.sbns() != 0
    }

    #[doc="Sets the SBNS field."]
    #[inline] pub fn set_sbns<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RxD Input Active Edge Interrupt Enable"]
    #[inline] pub fn rxedgie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXEDGIE != 0"]
    #[inline] pub fn test_rxedgie(&self) -> bool {
        self.rxedgie() != 0
    }

    #[doc="Sets the RXEDGIE field."]
    #[inline] pub fn set_rxedgie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LIN Break Detect Interrupt or DMA Request Enable"]
    #[inline] pub fn lbkdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LBKDIE != 0"]
    #[inline] pub fn test_lbkdie(&self) -> bool {
        self.lbkdie() != 0
    }

    #[doc="Sets the LBKDIE field."]
    #[inline] pub fn set_lbkdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Bdh {
    #[inline]
    fn from(other: u8) -> Self {
         Bdh(other)
    }
}

impl ::core::fmt::Display for Bdh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
        if self.sbns() != 0 { try!(write!(f, " sbns"))}
        if self.rxedgie() != 0 { try!(write!(f, " rxedgie"))}
        if self.lbkdie() != 0 { try!(write!(f, " lbkdie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Baud Rate Registers: Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdl(pub u8);
impl Bdl {
    #[doc="UART Baud Rate Bits"]
    #[inline] pub fn sbr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SBR != 0"]
    #[inline] pub fn test_sbr(&self) -> bool {
        self.sbr() != 0
    }

    #[doc="Sets the SBR field."]
    #[inline] pub fn set_sbr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Bdl {
    #[inline]
    fn from(other: u8) -> Self {
         Bdl(other)
    }
}

impl ::core::fmt::Display for Bdl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
    #[doc="Parity Type"]
    #[inline] pub fn pt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PT != 0"]
    #[inline] pub fn test_pt(&self) -> bool {
        self.pt() != 0
    }

    #[doc="Sets the PT field."]
    #[inline] pub fn set_pt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Parity Enable"]
    #[inline] pub fn pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Idle Line Type Select"]
    #[inline] pub fn ilt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ILT != 0"]
    #[inline] pub fn test_ilt(&self) -> bool {
        self.ilt() != 0
    }

    #[doc="Sets the ILT field."]
    #[inline] pub fn set_ilt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver Wakeup Method Select"]
    #[inline] pub fn wake(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WAKE != 0"]
    #[inline] pub fn test_wake(&self) -> bool {
        self.wake() != 0
    }

    #[doc="Sets the WAKE field."]
    #[inline] pub fn set_wake<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="9-bit or 8-bit Mode Select"]
    #[inline] pub fn m(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if M != 0"]
    #[inline] pub fn test_m(&self) -> bool {
        self.m() != 0
    }

    #[doc="Sets the M field."]
    #[inline] pub fn set_m<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receiver Source Select"]
    #[inline] pub fn rsrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RSRC != 0"]
    #[inline] pub fn test_rsrc(&self) -> bool {
        self.rsrc() != 0
    }

    #[doc="Sets the RSRC field."]
    #[inline] pub fn set_rsrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Stops in Wait Mode"]
    #[inline] pub fn uartswai(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if UARTSWAI != 0"]
    #[inline] pub fn test_uartswai(&self) -> bool {
        self.uartswai() != 0
    }

    #[doc="Sets the UARTSWAI field."]
    #[inline] pub fn set_uartswai<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loop Mode Select"]
    #[inline] pub fn loops(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOOPS != 0"]
    #[inline] pub fn test_loops(&self) -> bool {
        self.loops() != 0
    }

    #[doc="Sets the LOOPS field."]
    #[inline] pub fn set_loops<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C1 {
    #[inline]
    fn from(other: u8) -> Self {
         C1(other)
    }
}

impl ::core::fmt::Display for C1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pt() != 0 { try!(write!(f, " pt"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        if self.ilt() != 0 { try!(write!(f, " ilt"))}
        if self.wake() != 0 { try!(write!(f, " wake"))}
        if self.m() != 0 { try!(write!(f, " m"))}
        if self.rsrc() != 0 { try!(write!(f, " rsrc"))}
        if self.uartswai() != 0 { try!(write!(f, " uartswai"))}
        if self.loops() != 0 { try!(write!(f, " loops"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
    #[doc="Send Break"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receiver Wakeup Control"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receiver Enable"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmitter Enable"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Idle Line Interrupt DMA Transfer Enable"]
    #[inline] pub fn ilie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ILIE != 0"]
    #[inline] pub fn test_ilie(&self) -> bool {
        self.ilie() != 0
    }

    #[doc="Sets the ILIE field."]
    #[inline] pub fn set_ilie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline] pub fn rie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RIE != 0"]
    #[inline] pub fn test_rie(&self) -> bool {
        self.rie() != 0
    }

    #[doc="Sets the RIE field."]
    #[inline] pub fn set_rie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmission Complete Interrupt or DMA Transfer Enable"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmitter Interrupt or DMA Transfer Enable."]
    #[inline] pub fn tie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C2 {
    #[inline]
    fn from(other: u8) -> Self {
         C2(other)
    }
}

impl ::core::fmt::Display for C2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sbk() != 0 { try!(write!(f, " sbk"))}
        if self.rwu() != 0 { try!(write!(f, " rwu"))}
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.te() != 0 { try!(write!(f, " te"))}
        if self.ilie() != 0 { try!(write!(f, " ilie"))}
        if self.rie() != 0 { try!(write!(f, " rie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Status Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct S1(pub u8);
impl S1 {
    #[doc="Parity Error Flag"]
    #[inline] pub fn pf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PF != 0"]
    #[inline] pub fn test_pf(&self) -> bool {
        self.pf() != 0
    }

    #[doc="Sets the PF field."]
    #[inline] pub fn set_pf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Framing Error Flag"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Noise Flag"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver Overrun Flag"]
    #[inline] pub fn or(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OR != 0"]
    #[inline] pub fn test_or(&self) -> bool {
        self.or() != 0
    }

    #[doc="Sets the OR field."]
    #[inline] pub fn set_or<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Idle Line Flag"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive Data Register Full Flag"]
    #[inline] pub fn rdrf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RDRF != 0"]
    #[inline] pub fn test_rdrf(&self) -> bool {
        self.rdrf() != 0
    }

    #[doc="Sets the RDRF field."]
    #[inline] pub fn set_rdrf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmit Complete Flag"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit Data Register Empty Flag"]
    #[inline] pub fn tdre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TDRE != 0"]
    #[inline] pub fn test_tdre(&self) -> bool {
        self.tdre() != 0
    }

    #[doc="Sets the TDRE field."]
    #[inline] pub fn set_tdre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for S1 {
    #[inline]
    fn from(other: u8) -> Self {
         S1(other)
    }
}

impl ::core::fmt::Display for S1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for S1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pf() != 0 { try!(write!(f, " pf"))}
        if self.fe() != 0 { try!(write!(f, " fe"))}
        if self.nf() != 0 { try!(write!(f, " nf"))}
        if self.or() != 0 { try!(write!(f, " or"))}
        if self.idle() != 0 { try!(write!(f, " idle"))}
        if self.rdrf() != 0 { try!(write!(f, " rdrf"))}
        if self.tc() != 0 { try!(write!(f, " tc"))}
        if self.tdre() != 0 { try!(write!(f, " tdre"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Status Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct S2(pub u8);
impl S2 {
    #[doc="Receiver Active Flag"]
    #[inline] pub fn raf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RAF != 0"]
    #[inline] pub fn test_raf(&self) -> bool {
        self.raf() != 0
    }

    #[doc="Sets the RAF field."]
    #[inline] pub fn set_raf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="LIN Break Detection Enable"]
    #[inline] pub fn lbkde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LBKDE != 0"]
    #[inline] pub fn test_lbkde(&self) -> bool {
        self.lbkde() != 0
    }

    #[doc="Sets the LBKDE field."]
    #[inline] pub fn set_lbkde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Break Transmit Character Length"]
    #[inline] pub fn brk13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BRK13 != 0"]
    #[inline] pub fn test_brk13(&self) -> bool {
        self.brk13() != 0
    }

    #[doc="Sets the BRK13 field."]
    #[inline] pub fn set_brk13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive Wakeup Idle Detect"]
    #[inline] pub fn rwuid(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RWUID != 0"]
    #[inline] pub fn test_rwuid(&self) -> bool {
        self.rwuid() != 0
    }

    #[doc="Sets the RWUID field."]
    #[inline] pub fn set_rwuid<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receive Data Inversion"]
    #[inline] pub fn rxinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXINV != 0"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Sets the RXINV field."]
    #[inline] pub fn set_rxinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Most Significant Bit First"]
    #[inline] pub fn msbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MSBF != 0"]
    #[inline] pub fn test_msbf(&self) -> bool {
        self.msbf() != 0
    }

    #[doc="Sets the MSBF field."]
    #[inline] pub fn set_msbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RxD Pin Active Edge Interrupt Flag"]
    #[inline] pub fn rxedgif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXEDGIF != 0"]
    #[inline] pub fn test_rxedgif(&self) -> bool {
        self.rxedgif() != 0
    }

    #[doc="Sets the RXEDGIF field."]
    #[inline] pub fn set_rxedgif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LIN Break Detect Interrupt Flag"]
    #[inline] pub fn lbkdif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LBKDIF != 0"]
    #[inline] pub fn test_lbkdif(&self) -> bool {
        self.lbkdif() != 0
    }

    #[doc="Sets the LBKDIF field."]
    #[inline] pub fn set_lbkdif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for S2 {
    #[inline]
    fn from(other: u8) -> Self {
         S2(other)
    }
}

impl ::core::fmt::Display for S2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for S2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.raf() != 0 { try!(write!(f, " raf"))}
        if self.lbkde() != 0 { try!(write!(f, " lbkde"))}
        if self.brk13() != 0 { try!(write!(f, " brk13"))}
        if self.rwuid() != 0 { try!(write!(f, " rwuid"))}
        if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
        if self.msbf() != 0 { try!(write!(f, " msbf"))}
        if self.rxedgif() != 0 { try!(write!(f, " rxedgif"))}
        if self.lbkdif() != 0 { try!(write!(f, " lbkdif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C3(pub u8);
impl C3 {
    #[doc="Parity Error Interrupt Enable"]
    #[inline] pub fn peie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PEIE != 0"]
    #[inline] pub fn test_peie(&self) -> bool {
        self.peie() != 0
    }

    #[doc="Sets the PEIE field."]
    #[inline] pub fn set_peie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Framing Error Interrupt Enable"]
    #[inline] pub fn feie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FEIE != 0"]
    #[inline] pub fn test_feie(&self) -> bool {
        self.feie() != 0
    }

    #[doc="Sets the FEIE field."]
    #[inline] pub fn set_feie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Noise Error Interrupt Enable"]
    #[inline] pub fn neie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NEIE != 0"]
    #[inline] pub fn test_neie(&self) -> bool {
        self.neie() != 0
    }

    #[doc="Sets the NEIE field."]
    #[inline] pub fn set_neie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun Error Interrupt Enable"]
    #[inline] pub fn orie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ORIE != 0"]
    #[inline] pub fn test_orie(&self) -> bool {
        self.orie() != 0
    }

    #[doc="Sets the ORIE field."]
    #[inline] pub fn set_orie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit Data Inversion."]
    #[inline] pub fn txinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXINV != 0"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Sets the TXINV field."]
    #[inline] pub fn set_txinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmitter Pin Data Direction in Single-Wire mode"]
    #[inline] pub fn txdir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXDIR != 0"]
    #[inline] pub fn test_txdir(&self) -> bool {
        self.txdir() != 0
    }

    #[doc="Sets the TXDIR field."]
    #[inline] pub fn set_txdir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmit Bit 8"]
    #[inline] pub fn t8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if T8 != 0"]
    #[inline] pub fn test_t8(&self) -> bool {
        self.t8() != 0
    }

    #[doc="Sets the T8 field."]
    #[inline] pub fn set_t8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Received Bit 8"]
    #[inline] pub fn r8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if R8 != 0"]
    #[inline] pub fn test_r8(&self) -> bool {
        self.r8() != 0
    }

    #[doc="Sets the R8 field."]
    #[inline] pub fn set_r8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C3 {
    #[inline]
    fn from(other: u8) -> Self {
         C3(other)
    }
}

impl ::core::fmt::Display for C3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.peie() != 0 { try!(write!(f, " peie"))}
        if self.feie() != 0 { try!(write!(f, " feie"))}
        if self.neie() != 0 { try!(write!(f, " neie"))}
        if self.orie() != 0 { try!(write!(f, " orie"))}
        if self.txinv() != 0 { try!(write!(f, " txinv"))}
        if self.txdir() != 0 { try!(write!(f, " txdir"))}
        if self.t8() != 0 { try!(write!(f, " t8"))}
        if self.r8() != 0 { try!(write!(f, " r8"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct D(pub u8);
impl D {
    #[doc="Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
    #[inline] pub fn rt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RT != 0"]
    #[inline] pub fn test_rt(&self) -> bool {
        self.rt() != 0
    }

    #[doc="Sets the RT field."]
    #[inline] pub fn set_rt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for D {
    #[inline]
    fn from(other: u8) -> Self {
         D(other)
    }
}

impl ::core::fmt::Display for D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rt() != 0 { try!(write!(f, " rt=0x{:x}", self.rt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Match Address Registers 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ma1(pub u8);
impl Ma1 {
    #[doc="Match Address"]
    #[inline] pub fn ma(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MA != 0"]
    #[inline] pub fn test_ma(&self) -> bool {
        self.ma() != 0
    }

    #[doc="Sets the MA field."]
    #[inline] pub fn set_ma<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Ma1 {
    #[inline]
    fn from(other: u8) -> Self {
         Ma1(other)
    }
}

impl ::core::fmt::Display for Ma1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ma1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ma() != 0 { try!(write!(f, " ma=0x{:x}", self.ma()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Match Address Registers 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ma2(pub u8);
impl Ma2 {
    #[doc="Match Address"]
    #[inline] pub fn ma(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MA != 0"]
    #[inline] pub fn test_ma(&self) -> bool {
        self.ma() != 0
    }

    #[doc="Sets the MA field."]
    #[inline] pub fn set_ma<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Ma2 {
    #[inline]
    fn from(other: u8) -> Self {
         Ma2(other)
    }
}

impl ::core::fmt::Display for Ma2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ma2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ma() != 0 { try!(write!(f, " ma=0x{:x}", self.ma()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control Register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C4(pub u8);
impl C4 {
    #[doc="Baud Rate Fine Adjust"]
    #[inline] pub fn brfa(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if BRFA != 0"]
    #[inline] pub fn test_brfa(&self) -> bool {
        self.brfa() != 0
    }

    #[doc="Sets the BRFA field."]
    #[inline] pub fn set_brfa<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="10-bit Mode select"]
    #[inline] pub fn m10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if M10 != 0"]
    #[inline] pub fn test_m10(&self) -> bool {
        self.m10() != 0
    }

    #[doc="Sets the M10 field."]
    #[inline] pub fn set_m10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Match Address Mode Enable 2"]
    #[inline] pub fn maen2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if MAEN2 != 0"]
    #[inline] pub fn test_maen2(&self) -> bool {
        self.maen2() != 0
    }

    #[doc="Sets the MAEN2 field."]
    #[inline] pub fn set_maen2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Match Address Mode Enable 1"]
    #[inline] pub fn maen1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MAEN1 != 0"]
    #[inline] pub fn test_maen1(&self) -> bool {
        self.maen1() != 0
    }

    #[doc="Sets the MAEN1 field."]
    #[inline] pub fn set_maen1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C4 {
    #[inline]
    fn from(other: u8) -> Self {
         C4(other)
    }
}

impl ::core::fmt::Display for C4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.brfa() != 0 { try!(write!(f, " brfa=0x{:x}", self.brfa()))}
        if self.m10() != 0 { try!(write!(f, " m10"))}
        if self.maen2() != 0 { try!(write!(f, " maen2"))}
        if self.maen1() != 0 { try!(write!(f, " maen1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control Register 5"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C5(pub u8);
impl C5 {
    #[doc="LIN Break Detect DMA Select Bit"]
    #[inline] pub fn lbkddmas(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LBKDDMAS != 0"]
    #[inline] pub fn test_lbkddmas(&self) -> bool {
        self.lbkddmas() != 0
    }

    #[doc="Sets the LBKDDMAS field."]
    #[inline] pub fn set_lbkddmas<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Idle Line DMA Select"]
    #[inline] pub fn ildmas(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ILDMAS != 0"]
    #[inline] pub fn test_ildmas(&self) -> bool {
        self.ildmas() != 0
    }

    #[doc="Sets the ILDMAS field."]
    #[inline] pub fn set_ildmas<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receiver Full DMA Select"]
    #[inline] pub fn rdmas(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RDMAS != 0"]
    #[inline] pub fn test_rdmas(&self) -> bool {
        self.rdmas() != 0
    }

    #[doc="Sets the RDMAS field."]
    #[inline] pub fn set_rdmas<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmission Complete DMA Select"]
    #[inline] pub fn tcdmas(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TCDMAS != 0"]
    #[inline] pub fn test_tcdmas(&self) -> bool {
        self.tcdmas() != 0
    }

    #[doc="Sets the TCDMAS field."]
    #[inline] pub fn set_tcdmas<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmitter DMA Select"]
    #[inline] pub fn tdmas(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TDMAS != 0"]
    #[inline] pub fn test_tdmas(&self) -> bool {
        self.tdmas() != 0
    }

    #[doc="Sets the TDMAS field."]
    #[inline] pub fn set_tdmas<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C5 {
    #[inline]
    fn from(other: u8) -> Self {
         C5(other)
    }
}

impl ::core::fmt::Display for C5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lbkddmas() != 0 { try!(write!(f, " lbkddmas"))}
        if self.ildmas() != 0 { try!(write!(f, " ildmas"))}
        if self.rdmas() != 0 { try!(write!(f, " rdmas"))}
        if self.tcdmas() != 0 { try!(write!(f, " tcdmas"))}
        if self.tdmas() != 0 { try!(write!(f, " tdmas"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Extended Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ed(pub u8);
impl Ed {
    #[doc="The current received dataword contained in D and C3[R8] was received with a parity error."]
    #[inline] pub fn paritye(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PARITYE != 0"]
    #[inline] pub fn test_paritye(&self) -> bool {
        self.paritye() != 0
    }

    #[doc="Sets the PARITYE field."]
    #[inline] pub fn set_paritye<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="The current received dataword contained in D and C3[R8] was received with noise."]
    #[inline] pub fn noisy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOISY != 0"]
    #[inline] pub fn test_noisy(&self) -> bool {
        self.noisy() != 0
    }

    #[doc="Sets the NOISY field."]
    #[inline] pub fn set_noisy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ed {
    #[inline]
    fn from(other: u8) -> Self {
         Ed(other)
    }
}

impl ::core::fmt::Display for Ed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.paritye() != 0 { try!(write!(f, " paritye"))}
        if self.noisy() != 0 { try!(write!(f, " noisy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Modem Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Modem(pub u8);
impl Modem {
    #[doc="Transmitter clear-to-send enable"]
    #[inline] pub fn txctse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXCTSE != 0"]
    #[inline] pub fn test_txctse(&self) -> bool {
        self.txctse() != 0
    }

    #[doc="Sets the TXCTSE field."]
    #[inline] pub fn set_txctse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmitter request-to-send enable"]
    #[inline] pub fn txrtse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXRTSE != 0"]
    #[inline] pub fn test_txrtse(&self) -> bool {
        self.txrtse() != 0
    }

    #[doc="Sets the TXRTSE field."]
    #[inline] pub fn set_txrtse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmitter request-to-send polarity"]
    #[inline] pub fn txrtspol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXRTSPOL != 0"]
    #[inline] pub fn test_txrtspol(&self) -> bool {
        self.txrtspol() != 0
    }

    #[doc="Sets the TXRTSPOL field."]
    #[inline] pub fn set_txrtspol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver request-to-send enable"]
    #[inline] pub fn rxrtse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXRTSE != 0"]
    #[inline] pub fn test_rxrtse(&self) -> bool {
        self.rxrtse() != 0
    }

    #[doc="Sets the RXRTSE field."]
    #[inline] pub fn set_rxrtse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Modem {
    #[inline]
    fn from(other: u8) -> Self {
         Modem(other)
    }
}

impl ::core::fmt::Display for Modem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Modem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txctse() != 0 { try!(write!(f, " txctse"))}
        if self.txrtse() != 0 { try!(write!(f, " txrtse"))}
        if self.txrtspol() != 0 { try!(write!(f, " txrtspol"))}
        if self.rxrtse() != 0 { try!(write!(f, " rxrtse"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Infrared Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ir(pub u8);
impl Ir {
    #[doc="Transmitter narrow pulse"]
    #[inline] pub fn tnp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if TNP != 0"]
    #[inline] pub fn test_tnp(&self) -> bool {
        self.tnp() != 0
    }

    #[doc="Sets the TNP field."]
    #[inline] pub fn set_tnp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Infrared enable"]
    #[inline] pub fn iren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Ir {
    #[inline]
    fn from(other: u8) -> Self {
         Ir(other)
    }
}

impl ::core::fmt::Display for Ir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tnp() != 0 { try!(write!(f, " tnp=0x{:x}", self.tnp()))}
        if self.iren() != 0 { try!(write!(f, " iren"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART FIFO Parameters"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pfifo(pub u8);
impl Pfifo {
    #[doc="Receive FIFO. Buffer Depth"]
    #[inline] pub fn rxfifosize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RXFIFOSIZE != 0"]
    #[inline] pub fn test_rxfifosize(&self) -> bool {
        self.rxfifosize() != 0
    }

    #[doc="Sets the RXFIFOSIZE field."]
    #[inline] pub fn set_rxfifosize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Enable"]
    #[inline] pub fn rxfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFE != 0"]
    #[inline] pub fn test_rxfe(&self) -> bool {
        self.rxfe() != 0
    }

    #[doc="Sets the RXFE field."]
    #[inline] pub fn set_rxfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit FIFO. Buffer Depth"]
    #[inline] pub fn txfifosize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if TXFIFOSIZE != 0"]
    #[inline] pub fn test_txfifosize(&self) -> bool {
        self.txfifosize() != 0
    }

    #[doc="Sets the TXFIFOSIZE field."]
    #[inline] pub fn set_txfifosize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit FIFO Enable"]
    #[inline] pub fn txfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Pfifo {
    #[inline]
    fn from(other: u8) -> Self {
         Pfifo(other)
    }
}

impl ::core::fmt::Display for Pfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxfifosize() != 0 { try!(write!(f, " rxfifosize=0x{:x}", self.rxfifosize()))}
        if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
        if self.txfifosize() != 0 { try!(write!(f, " txfifosize=0x{:x}", self.txfifosize()))}
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART FIFO Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfifo(pub u8);
impl Cfifo {
    #[doc="Receive FIFO Underflow Interrupt Enable"]
    #[inline] pub fn rxufe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXUFE != 0"]
    #[inline] pub fn test_rxufe(&self) -> bool {
        self.rxufe() != 0
    }

    #[doc="Sets the RXUFE field."]
    #[inline] pub fn set_rxufe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit FIFO Overflow Interrupt Enable"]
    #[inline] pub fn txofe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXOFE != 0"]
    #[inline] pub fn test_txofe(&self) -> bool {
        self.txofe() != 0
    }

    #[doc="Sets the TXOFE field."]
    #[inline] pub fn set_txofe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive FIFO Overflow Interrupt Enable"]
    #[inline] pub fn rxofe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXOFE != 0"]
    #[inline] pub fn test_rxofe(&self) -> bool {
        self.rxofe() != 0
    }

    #[doc="Sets the RXOFE field."]
    #[inline] pub fn set_rxofe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive FIFO/Buffer Flush"]
    #[inline] pub fn rxflush(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXFLUSH != 0"]
    #[inline] pub fn test_rxflush(&self) -> bool {
        self.rxflush() != 0
    }

    #[doc="Sets the RXFLUSH field."]
    #[inline] pub fn set_rxflush<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO/Buffer Flush"]
    #[inline] pub fn txflush(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFLUSH != 0"]
    #[inline] pub fn test_txflush(&self) -> bool {
        self.txflush() != 0
    }

    #[doc="Sets the TXFLUSH field."]
    #[inline] pub fn set_txflush<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cfifo {
    #[inline]
    fn from(other: u8) -> Self {
         Cfifo(other)
    }
}

impl ::core::fmt::Display for Cfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxufe() != 0 { try!(write!(f, " rxufe"))}
        if self.txofe() != 0 { try!(write!(f, " txofe"))}
        if self.rxofe() != 0 { try!(write!(f, " rxofe"))}
        if self.rxflush() != 0 { try!(write!(f, " rxflush"))}
        if self.txflush() != 0 { try!(write!(f, " txflush"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART FIFO Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sfifo(pub u8);
impl Sfifo {
    #[doc="Receiver Buffer Underflow Flag"]
    #[inline] pub fn rxuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXUF != 0"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Sets the RXUF field."]
    #[inline] pub fn set_rxuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmitter Buffer Overflow Flag"]
    #[inline] pub fn txof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXOF != 0"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Sets the TXOF field."]
    #[inline] pub fn set_txof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receiver Buffer Overflow Flag"]
    #[inline] pub fn rxof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXOF != 0"]
    #[inline] pub fn test_rxof(&self) -> bool {
        self.rxof() != 0
    }

    #[doc="Sets the RXOF field."]
    #[inline] pub fn set_rxof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive Buffer/FIFO Empty"]
    #[inline] pub fn rxempt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXEMPT != 0"]
    #[inline] pub fn test_rxempt(&self) -> bool {
        self.rxempt() != 0
    }

    #[doc="Sets the RXEMPT field."]
    #[inline] pub fn set_rxempt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit Buffer/FIFO Empty"]
    #[inline] pub fn txempt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXEMPT != 0"]
    #[inline] pub fn test_txempt(&self) -> bool {
        self.txempt() != 0
    }

    #[doc="Sets the TXEMPT field."]
    #[inline] pub fn set_txempt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Sfifo {
    #[inline]
    fn from(other: u8) -> Self {
         Sfifo(other)
    }
}

impl ::core::fmt::Display for Sfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.rxof() != 0 { try!(write!(f, " rxof"))}
        if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
        if self.txempt() != 0 { try!(write!(f, " txempt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART FIFO Transmit Watermark"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Twfifo(pub u8);
impl Twfifo {
    #[doc="Transmit Watermark"]
    #[inline] pub fn txwater(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TXWATER != 0"]
    #[inline] pub fn test_txwater(&self) -> bool {
        self.txwater() != 0
    }

    #[doc="Sets the TXWATER field."]
    #[inline] pub fn set_txwater<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Twfifo {
    #[inline]
    fn from(other: u8) -> Self {
         Twfifo(other)
    }
}

impl ::core::fmt::Display for Twfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Twfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART FIFO Transmit Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcfifo(pub u8);
impl Tcfifo {
    #[doc="Transmit Counter"]
    #[inline] pub fn txcount(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TXCOUNT != 0"]
    #[inline] pub fn test_txcount(&self) -> bool {
        self.txcount() != 0
    }

    #[doc="Sets the TXCOUNT field."]
    #[inline] pub fn set_txcount<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Tcfifo {
    #[inline]
    fn from(other: u8) -> Self {
         Tcfifo(other)
    }
}

impl ::core::fmt::Display for Tcfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tcfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART FIFO Receive Watermark"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rwfifo(pub u8);
impl Rwfifo {
    #[doc="Receive Watermark"]
    #[inline] pub fn rxwater(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RXWATER != 0"]
    #[inline] pub fn test_rxwater(&self) -> bool {
        self.rxwater() != 0
    }

    #[doc="Sets the RXWATER field."]
    #[inline] pub fn set_rxwater<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rwfifo {
    #[inline]
    fn from(other: u8) -> Self {
         Rwfifo(other)
    }
}

impl ::core::fmt::Display for Rwfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rwfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART FIFO Receive Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcfifo(pub u8);
impl Rcfifo {
    #[doc="Receive Counter"]
    #[inline] pub fn rxcount(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RXCOUNT != 0"]
    #[inline] pub fn test_rxcount(&self) -> bool {
        self.rxcount() != 0
    }

    #[doc="Sets the RXCOUNT field."]
    #[inline] pub fn set_rxcount<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rcfifo {
    #[inline]
    fn from(other: u8) -> Self {
         Rcfifo(other)
    }
}

impl ::core::fmt::Display for Rcfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C7816(pub u8);
impl C7816 {
    #[doc="ISO-7816 Functionality Enabled"]
    #[inline] pub fn iso_7816e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ISO_7816E != 0"]
    #[inline] pub fn test_iso_7816e(&self) -> bool {
        self.iso_7816e() != 0
    }

    #[doc="Sets the ISO_7816E field."]
    #[inline] pub fn set_iso_7816e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Type"]
    #[inline] pub fn ttype(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TTYPE != 0"]
    #[inline] pub fn test_ttype(&self) -> bool {
        self.ttype() != 0
    }

    #[doc="Sets the TTYPE field."]
    #[inline] pub fn set_ttype<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Detect Initial Character"]
    #[inline] pub fn init(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Generate NACK on Error"]
    #[inline] pub fn anack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ANACK != 0"]
    #[inline] pub fn test_anack(&self) -> bool {
        self.anack() != 0
    }

    #[doc="Sets the ANACK field."]
    #[inline] pub fn set_anack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Generate NACK on Overflow"]
    #[inline] pub fn onack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ONACK != 0"]
    #[inline] pub fn test_onack(&self) -> bool {
        self.onack() != 0
    }

    #[doc="Sets the ONACK field."]
    #[inline] pub fn set_onack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for C7816 {
    #[inline]
    fn from(other: u8) -> Self {
         C7816(other)
    }
}

impl ::core::fmt::Display for C7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iso_7816e() != 0 { try!(write!(f, " iso_7816e"))}
        if self.ttype() != 0 { try!(write!(f, " ttype"))}
        if self.init() != 0 { try!(write!(f, " init"))}
        if self.anack() != 0 { try!(write!(f, " anack"))}
        if self.onack() != 0 { try!(write!(f, " onack"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ie7816(pub u8);
impl Ie7816 {
    #[doc="Receive Threshold Exceeded Interrupt Enable"]
    #[inline] pub fn rxte(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXTE != 0"]
    #[inline] pub fn test_rxte(&self) -> bool {
        self.rxte() != 0
    }

    #[doc="Sets the RXTE field."]
    #[inline] pub fn set_rxte<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Threshold Exceeded Interrupt Enable"]
    #[inline] pub fn txte(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXTE != 0"]
    #[inline] pub fn test_txte(&self) -> bool {
        self.txte() != 0
    }

    #[doc="Sets the TXTE field."]
    #[inline] pub fn set_txte<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Guard Timer Violated Interrupt Enable"]
    #[inline] pub fn gtve(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GTVE != 0"]
    #[inline] pub fn test_gtve(&self) -> bool {
        self.gtve() != 0
    }

    #[doc="Sets the GTVE field."]
    #[inline] pub fn set_gtve<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Initial Character Detected Interrupt Enable"]
    #[inline] pub fn initde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if INITDE != 0"]
    #[inline] pub fn test_initde(&self) -> bool {
        self.initde() != 0
    }

    #[doc="Sets the INITDE field."]
    #[inline] pub fn set_initde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Block Wait Timer Interrupt Enable"]
    #[inline] pub fn bwte(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BWTE != 0"]
    #[inline] pub fn test_bwte(&self) -> bool {
        self.bwte() != 0
    }

    #[doc="Sets the BWTE field."]
    #[inline] pub fn set_bwte<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Character Wait Timer Interrupt Enable"]
    #[inline] pub fn cwte(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CWTE != 0"]
    #[inline] pub fn test_cwte(&self) -> bool {
        self.cwte() != 0
    }

    #[doc="Sets the CWTE field."]
    #[inline] pub fn set_cwte<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Wait Timer Interrupt Enable"]
    #[inline] pub fn wte(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WTE != 0"]
    #[inline] pub fn test_wte(&self) -> bool {
        self.wte() != 0
    }

    #[doc="Sets the WTE field."]
    #[inline] pub fn set_wte<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ie7816 {
    #[inline]
    fn from(other: u8) -> Self {
         Ie7816(other)
    }
}

impl ::core::fmt::Display for Ie7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ie7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxte() != 0 { try!(write!(f, " rxte"))}
        if self.txte() != 0 { try!(write!(f, " txte"))}
        if self.gtve() != 0 { try!(write!(f, " gtve"))}
        if self.initde() != 0 { try!(write!(f, " initde"))}
        if self.bwte() != 0 { try!(write!(f, " bwte"))}
        if self.cwte() != 0 { try!(write!(f, " cwte"))}
        if self.wte() != 0 { try!(write!(f, " wte"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Interrupt Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Is7816(pub u8);
impl Is7816 {
    #[doc="Receive Threshold Exceeded Interrupt"]
    #[inline] pub fn rxt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXT != 0"]
    #[inline] pub fn test_rxt(&self) -> bool {
        self.rxt() != 0
    }

    #[doc="Sets the RXT field."]
    #[inline] pub fn set_rxt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Threshold Exceeded Interrupt"]
    #[inline] pub fn txt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXT != 0"]
    #[inline] pub fn test_txt(&self) -> bool {
        self.txt() != 0
    }

    #[doc="Sets the TXT field."]
    #[inline] pub fn set_txt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Guard Timer Violated Interrupt"]
    #[inline] pub fn gtv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GTV != 0"]
    #[inline] pub fn test_gtv(&self) -> bool {
        self.gtv() != 0
    }

    #[doc="Sets the GTV field."]
    #[inline] pub fn set_gtv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Initial Character Detected Interrupt"]
    #[inline] pub fn initd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if INITD != 0"]
    #[inline] pub fn test_initd(&self) -> bool {
        self.initd() != 0
    }

    #[doc="Sets the INITD field."]
    #[inline] pub fn set_initd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Block Wait Timer Interrupt"]
    #[inline] pub fn bwt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BWT != 0"]
    #[inline] pub fn test_bwt(&self) -> bool {
        self.bwt() != 0
    }

    #[doc="Sets the BWT field."]
    #[inline] pub fn set_bwt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Character Wait Timer Interrupt"]
    #[inline] pub fn cwt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CWT != 0"]
    #[inline] pub fn test_cwt(&self) -> bool {
        self.cwt() != 0
    }

    #[doc="Sets the CWT field."]
    #[inline] pub fn set_cwt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Wait Timer Interrupt"]
    #[inline] pub fn wt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WT != 0"]
    #[inline] pub fn test_wt(&self) -> bool {
        self.wt() != 0
    }

    #[doc="Sets the WT field."]
    #[inline] pub fn set_wt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Is7816 {
    #[inline]
    fn from(other: u8) -> Self {
         Is7816(other)
    }
}

impl ::core::fmt::Display for Is7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Is7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxt() != 0 { try!(write!(f, " rxt"))}
        if self.txt() != 0 { try!(write!(f, " txt"))}
        if self.gtv() != 0 { try!(write!(f, " gtv"))}
        if self.initd() != 0 { try!(write!(f, " initd"))}
        if self.bwt() != 0 { try!(write!(f, " bwt"))}
        if self.cwt() != 0 { try!(write!(f, " cwt"))}
        if self.wt() != 0 { try!(write!(f, " wt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Wait Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wp7816t0(pub u8);
impl Wp7816t0 {
    #[doc="Wait Time Integer (C7816[TTYPE] = 0)"]
    #[inline] pub fn wi(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WI != 0"]
    #[inline] pub fn test_wi(&self) -> bool {
        self.wi() != 0
    }

    #[doc="Sets the WI field."]
    #[inline] pub fn set_wi<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Wp7816t0 {
    #[inline]
    fn from(other: u8) -> Self {
         Wp7816t0(other)
    }
}

impl ::core::fmt::Display for Wp7816t0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wp7816t0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wi() != 0 { try!(write!(f, " wi=0x{:x}", self.wi()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Wait Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wp7816t1(pub u8);
impl Wp7816t1 {
    #[doc="Block Wait Time Integer(C7816[TTYPE] = 1)"]
    #[inline] pub fn bwi(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BWI != 0"]
    #[inline] pub fn test_bwi(&self) -> bool {
        self.bwi() != 0
    }

    #[doc="Sets the BWI field."]
    #[inline] pub fn set_bwi<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Character Wait Time Integer (C7816[TTYPE] = 1)"]
    #[inline] pub fn cwi(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if CWI != 0"]
    #[inline] pub fn test_cwi(&self) -> bool {
        self.cwi() != 0
    }

    #[doc="Sets the CWI field."]
    #[inline] pub fn set_cwi<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Wp7816t1 {
    #[inline]
    fn from(other: u8) -> Self {
         Wp7816t1(other)
    }
}

impl ::core::fmt::Display for Wp7816t1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wp7816t1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bwi() != 0 { try!(write!(f, " bwi=0x{:x}", self.bwi()))}
        if self.cwi() != 0 { try!(write!(f, " cwi=0x{:x}", self.cwi()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Wait N Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wn7816(pub u8);
impl Wn7816 {
    #[doc="Guard Band N"]
    #[inline] pub fn gtn(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if GTN != 0"]
    #[inline] pub fn test_gtn(&self) -> bool {
        self.gtn() != 0
    }

    #[doc="Sets the GTN field."]
    #[inline] pub fn set_gtn<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Wn7816 {
    #[inline]
    fn from(other: u8) -> Self {
         Wn7816(other)
    }
}

impl ::core::fmt::Display for Wn7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wn7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gtn() != 0 { try!(write!(f, " gtn=0x{:x}", self.gtn()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Wait FD Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wf7816(pub u8);
impl Wf7816 {
    #[doc="FD Multiplier"]
    #[inline] pub fn gtfd(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if GTFD != 0"]
    #[inline] pub fn test_gtfd(&self) -> bool {
        self.gtfd() != 0
    }

    #[doc="Sets the GTFD field."]
    #[inline] pub fn set_gtfd<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Wf7816 {
    #[inline]
    fn from(other: u8) -> Self {
         Wf7816(other)
    }
}

impl ::core::fmt::Display for Wf7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wf7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gtfd() != 0 { try!(write!(f, " gtfd=0x{:x}", self.gtfd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Error Threshold Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Et7816(pub u8);
impl Et7816 {
    #[doc="Receive NACK Threshold"]
    #[inline] pub fn rxthreshold(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if RXTHRESHOLD != 0"]
    #[inline] pub fn test_rxthreshold(&self) -> bool {
        self.rxthreshold() != 0
    }

    #[doc="Sets the RXTHRESHOLD field."]
    #[inline] pub fn set_rxthreshold<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit NACK Threshold"]
    #[inline] pub fn txthreshold(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if TXTHRESHOLD != 0"]
    #[inline] pub fn test_txthreshold(&self) -> bool {
        self.txthreshold() != 0
    }

    #[doc="Sets the TXTHRESHOLD field."]
    #[inline] pub fn set_txthreshold<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Et7816 {
    #[inline]
    fn from(other: u8) -> Self {
         Et7816(other)
    }
}

impl ::core::fmt::Display for Et7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Et7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxthreshold() != 0 { try!(write!(f, " rxthreshold=0x{:x}", self.rxthreshold()))}
        if self.txthreshold() != 0 { try!(write!(f, " txthreshold=0x{:x}", self.txthreshold()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 7816 Transmit Length Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tl7816(pub u8);
impl Tl7816 {
    #[doc="Transmit Length"]
    #[inline] pub fn tlen(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TLEN != 0"]
    #[inline] pub fn test_tlen(&self) -> bool {
        self.tlen() != 0
    }

    #[doc="Sets the TLEN field."]
    #[inline] pub fn set_tlen<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Tl7816 {
    #[inline]
    fn from(other: u8) -> Self {
         Tl7816(other)
    }
}

impl ::core::fmt::Display for Tl7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tl7816 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tlen() != 0 { try!(write!(f, " tlen=0x{:x}", self.tlen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

