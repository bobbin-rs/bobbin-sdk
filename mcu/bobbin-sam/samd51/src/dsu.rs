::bobbin_mcu::periph!( DSU, Dsu, DSU_PERIPH, DsuPeriph, DSU_OWNED, DSU_REF_COUNT, 0x41002000, 0x00, 0x09);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DSU Peripheral"]
pub struct DsuPeriph(pub usize); 

impl DsuPeriph {
    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Get the STATUSA Register."]
    #[inline] pub fn statusa_reg(&self) -> ::bobbin_mcu::register::Register<Statusa> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Statusa, 0x1)
    }

    #[doc="Get the *mut pointer for the STATUSA register."]
    #[inline] pub fn statusa_mut(&self) -> *mut Statusa { 
        self.statusa_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUSA register."]
    #[inline] pub fn statusa_ptr(&self) -> *const Statusa { 
        self.statusa_reg().ptr()
    }

    #[doc="Read the STATUSA register."]
    #[inline] pub fn statusa(&self) -> Statusa { 
        self.statusa_reg().read()
    }

    #[doc="Write the STATUSA register."]
    #[inline] pub fn write_statusa(&self, value: Statusa) -> &Self { 
        self.statusa_reg().write(value);
        self
    }

    #[doc="Set the STATUSA register."]
    #[inline] pub fn set_statusa<F: FnOnce(Statusa) -> Statusa>(&self, f: F) -> &Self {
        self.statusa_reg().set(f);
        self
    }

    #[doc="Modify the STATUSA register."]
    #[inline] pub fn with_statusa<F: FnOnce(Statusa) -> Statusa>(&self, f: F) -> &Self {
        self.statusa_reg().with(f);
        self
    }

    #[doc="Get the STATUSB Register."]
    #[inline] pub fn statusb_reg(&self) -> ::bobbin_mcu::register::Register<Statusb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Statusb, 0x2)
    }

    #[doc="Get the *mut pointer for the STATUSB register."]
    #[inline] pub fn statusb_mut(&self) -> *mut Statusb { 
        self.statusb_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUSB register."]
    #[inline] pub fn statusb_ptr(&self) -> *const Statusb { 
        self.statusb_reg().ptr()
    }

    #[doc="Read the STATUSB register."]
    #[inline] pub fn statusb(&self) -> Statusb { 
        self.statusb_reg().read()
    }

    #[doc="Get the ADDR Register."]
    #[inline] pub fn addr_reg(&self) -> ::bobbin_mcu::register::Register<Addr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr, 0x4)
    }

    #[doc="Get the *mut pointer for the ADDR register."]
    #[inline] pub fn addr_mut(&self) -> *mut Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR register."]
    #[inline] pub fn addr_ptr(&self) -> *const Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Read the ADDR register."]
    #[inline] pub fn addr(&self) -> Addr { 
        self.addr_reg().read()
    }

    #[doc="Write the ADDR register."]
    #[inline] pub fn write_addr(&self, value: Addr) -> &Self { 
        self.addr_reg().write(value);
        self
    }

    #[doc="Set the ADDR register."]
    #[inline] pub fn set_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().set(f);
        self
    }

    #[doc="Modify the ADDR register."]
    #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().with(f);
        self
    }

    #[doc="Get the LENGTH Register."]
    #[inline] pub fn length_reg(&self) -> ::bobbin_mcu::register::Register<Length> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Length, 0x8)
    }

    #[doc="Get the *mut pointer for the LENGTH register."]
    #[inline] pub fn length_mut(&self) -> *mut Length { 
        self.length_reg().ptr()
    }

    #[doc="Get the *const pointer for the LENGTH register."]
    #[inline] pub fn length_ptr(&self) -> *const Length { 
        self.length_reg().ptr()
    }

    #[doc="Read the LENGTH register."]
    #[inline] pub fn length(&self) -> Length { 
        self.length_reg().read()
    }

    #[doc="Write the LENGTH register."]
    #[inline] pub fn write_length(&self, value: Length) -> &Self { 
        self.length_reg().write(value);
        self
    }

    #[doc="Set the LENGTH register."]
    #[inline] pub fn set_length<F: FnOnce(Length) -> Length>(&self, f: F) -> &Self {
        self.length_reg().set(f);
        self
    }

    #[doc="Modify the LENGTH register."]
    #[inline] pub fn with_length<F: FnOnce(Length) -> Length>(&self, f: F) -> &Self {
        self.length_reg().with(f);
        self
    }

    #[doc="Get the DATA Register."]
    #[inline] pub fn data_reg(&self) -> ::bobbin_mcu::register::Register<Data> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Data, 0xc)
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut Data { 
        self.data_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const Data { 
        self.data_reg().ptr()
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        self.data_reg().read()
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn write_data(&self, value: Data) -> &Self { 
        self.data_reg().write(value);
        self
    }

    #[doc="Set the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().set(f);
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().with(f);
        self
    }

    #[doc="Get the DCC Register."]
    #[inline] pub fn dcc_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dcc, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dcc, 0x10, 0x4)
    }

    #[doc="Get the *mut pointer for the DCC register."]
    #[inline] pub fn dcc_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dcc { 
        self.dcc_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DCC register."]
    #[inline] pub fn dcc_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dcc { 
        self.dcc_reg().ptr(index.into())
    }

    #[doc="Read the DCC register."]
    #[inline] pub fn dcc<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dcc { 
        self.dcc_reg().read(index.into())
    }

    #[doc="Write the DCC register."]
    #[inline] pub fn write_dcc<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dcc) -> &Self {
        self.dcc_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DCC register."]
    #[inline] pub fn set_dcc<I: Into<::bobbin_bits::R2>, F: FnOnce(Dcc) -> Dcc>(&self, index: I, f: F) -> &Self {
        self.dcc_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DCC register."]
    #[inline] pub fn with_dcc<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dcc) -> Dcc>(&self, index: I, f: F) -> &Self {
        self.dcc_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DID Register."]
    #[inline] pub fn did_reg(&self) -> ::bobbin_mcu::register::Register<Did> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Did, 0x18)
    }

    #[doc="Get the *mut pointer for the DID register."]
    #[inline] pub fn did_mut(&self) -> *mut Did { 
        self.did_reg().ptr()
    }

    #[doc="Get the *const pointer for the DID register."]
    #[inline] pub fn did_ptr(&self) -> *const Did { 
        self.did_reg().ptr()
    }

    #[doc="Read the DID register."]
    #[inline] pub fn did(&self) -> Did { 
        self.did_reg().read()
    }

    #[doc="Get the CFG Register."]
    #[inline] pub fn cfg_reg(&self) -> ::bobbin_mcu::register::Register<Cfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfg, 0x1c)
    }

    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut(&self) -> *mut Cfg { 
        self.cfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr(&self) -> *const Cfg { 
        self.cfg_reg().ptr()
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg(&self) -> Cfg { 
        self.cfg_reg().read()
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn write_cfg(&self, value: Cfg) -> &Self { 
        self.cfg_reg().write(value);
        self
    }

    #[doc="Set the CFG register."]
    #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        self.cfg_reg().set(f);
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        self.cfg_reg().with(f);
        self
    }

    #[doc="Get the DCFG Register."]
    #[inline] pub fn dcfg_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dcfg, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dcfg, 0xf0, 0x4)
    }

    #[doc="Get the *mut pointer for the DCFG register."]
    #[inline] pub fn dcfg_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dcfg { 
        self.dcfg_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DCFG register."]
    #[inline] pub fn dcfg_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dcfg { 
        self.dcfg_reg().ptr(index.into())
    }

    #[doc="Read the DCFG register."]
    #[inline] pub fn dcfg<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dcfg { 
        self.dcfg_reg().read(index.into())
    }

    #[doc="Write the DCFG register."]
    #[inline] pub fn write_dcfg<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dcfg) -> &Self {
        self.dcfg_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DCFG register."]
    #[inline] pub fn set_dcfg<I: Into<::bobbin_bits::R2>, F: FnOnce(Dcfg) -> Dcfg>(&self, index: I, f: F) -> &Self {
        self.dcfg_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DCFG register."]
    #[inline] pub fn with_dcfg<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dcfg) -> Dcfg>(&self, index: I, f: F) -> &Self {
        self.dcfg_reg().with(index.into(), f);
        self
    }

    #[doc="Get the ENTRY0 Register."]
    #[inline] pub fn entry0_reg(&self) -> ::bobbin_mcu::register::Register<Entry0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Entry0, 0x1000)
    }

    #[doc="Get the *mut pointer for the ENTRY0 register."]
    #[inline] pub fn entry0_mut(&self) -> *mut Entry0 { 
        self.entry0_reg().ptr()
    }

    #[doc="Get the *const pointer for the ENTRY0 register."]
    #[inline] pub fn entry0_ptr(&self) -> *const Entry0 { 
        self.entry0_reg().ptr()
    }

    #[doc="Read the ENTRY0 register."]
    #[inline] pub fn entry0(&self) -> Entry0 { 
        self.entry0_reg().read()
    }

    #[doc="Get the ENTRY1 Register."]
    #[inline] pub fn entry1_reg(&self) -> ::bobbin_mcu::register::Register<Entry1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Entry1, 0x1004)
    }

    #[doc="Get the *mut pointer for the ENTRY1 register."]
    #[inline] pub fn entry1_mut(&self) -> *mut Entry1 { 
        self.entry1_reg().ptr()
    }

    #[doc="Get the *const pointer for the ENTRY1 register."]
    #[inline] pub fn entry1_ptr(&self) -> *const Entry1 { 
        self.entry1_reg().ptr()
    }

    #[doc="Read the ENTRY1 register."]
    #[inline] pub fn entry1(&self) -> Entry1 { 
        self.entry1_reg().read()
    }

    #[doc="Get the END Register."]
    #[inline] pub fn end_reg(&self) -> ::bobbin_mcu::register::Register<End> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut End, 0x1008)
    }

    #[doc="Get the *mut pointer for the END register."]
    #[inline] pub fn end_mut(&self) -> *mut End { 
        self.end_reg().ptr()
    }

    #[doc="Get the *const pointer for the END register."]
    #[inline] pub fn end_ptr(&self) -> *const End { 
        self.end_reg().ptr()
    }

    #[doc="Read the END register."]
    #[inline] pub fn end(&self) -> End { 
        self.end_reg().read()
    }

    #[doc="Get the MEMTYPE Register."]
    #[inline] pub fn memtype_reg(&self) -> ::bobbin_mcu::register::Register<Memtype> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Memtype, 0x1fcc)
    }

    #[doc="Get the *mut pointer for the MEMTYPE register."]
    #[inline] pub fn memtype_mut(&self) -> *mut Memtype { 
        self.memtype_reg().ptr()
    }

    #[doc="Get the *const pointer for the MEMTYPE register."]
    #[inline] pub fn memtype_ptr(&self) -> *const Memtype { 
        self.memtype_reg().ptr()
    }

    #[doc="Read the MEMTYPE register."]
    #[inline] pub fn memtype(&self) -> Memtype { 
        self.memtype_reg().read()
    }

    #[doc="Get the PID4 Register."]
    #[inline] pub fn pid4_reg(&self) -> ::bobbin_mcu::register::Register<Pid4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pid4, 0x1fd0)
    }

    #[doc="Get the *mut pointer for the PID4 register."]
    #[inline] pub fn pid4_mut(&self) -> *mut Pid4 { 
        self.pid4_reg().ptr()
    }

    #[doc="Get the *const pointer for the PID4 register."]
    #[inline] pub fn pid4_ptr(&self) -> *const Pid4 { 
        self.pid4_reg().ptr()
    }

    #[doc="Read the PID4 register."]
    #[inline] pub fn pid4(&self) -> Pid4 { 
        self.pid4_reg().read()
    }

    #[doc="Get the PID5 Register."]
    #[inline] pub fn pid5_reg(&self) -> ::bobbin_mcu::register::Register<Pid5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pid5, 0x1fd4)
    }

    #[doc="Get the *mut pointer for the PID5 register."]
    #[inline] pub fn pid5_mut(&self) -> *mut Pid5 { 
        self.pid5_reg().ptr()
    }

    #[doc="Get the *const pointer for the PID5 register."]
    #[inline] pub fn pid5_ptr(&self) -> *const Pid5 { 
        self.pid5_reg().ptr()
    }

    #[doc="Read the PID5 register."]
    #[inline] pub fn pid5(&self) -> Pid5 { 
        self.pid5_reg().read()
    }

    #[doc="Get the PID6 Register."]
    #[inline] pub fn pid6_reg(&self) -> ::bobbin_mcu::register::Register<Pid6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pid6, 0x1fd8)
    }

    #[doc="Get the *mut pointer for the PID6 register."]
    #[inline] pub fn pid6_mut(&self) -> *mut Pid6 { 
        self.pid6_reg().ptr()
    }

    #[doc="Get the *const pointer for the PID6 register."]
    #[inline] pub fn pid6_ptr(&self) -> *const Pid6 { 
        self.pid6_reg().ptr()
    }

    #[doc="Read the PID6 register."]
    #[inline] pub fn pid6(&self) -> Pid6 { 
        self.pid6_reg().read()
    }

    #[doc="Get the PID7 Register."]
    #[inline] pub fn pid7_reg(&self) -> ::bobbin_mcu::register::Register<Pid7> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pid7, 0x1fdc)
    }

    #[doc="Get the *mut pointer for the PID7 register."]
    #[inline] pub fn pid7_mut(&self) -> *mut Pid7 { 
        self.pid7_reg().ptr()
    }

    #[doc="Get the *const pointer for the PID7 register."]
    #[inline] pub fn pid7_ptr(&self) -> *const Pid7 { 
        self.pid7_reg().ptr()
    }

    #[doc="Read the PID7 register."]
    #[inline] pub fn pid7(&self) -> Pid7 { 
        self.pid7_reg().read()
    }

    #[doc="Get the PID0 Register."]
    #[inline] pub fn pid0_reg(&self) -> ::bobbin_mcu::register::Register<Pid0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pid0, 0x1fe0)
    }

    #[doc="Get the *mut pointer for the PID0 register."]
    #[inline] pub fn pid0_mut(&self) -> *mut Pid0 { 
        self.pid0_reg().ptr()
    }

    #[doc="Get the *const pointer for the PID0 register."]
    #[inline] pub fn pid0_ptr(&self) -> *const Pid0 { 
        self.pid0_reg().ptr()
    }

    #[doc="Read the PID0 register."]
    #[inline] pub fn pid0(&self) -> Pid0 { 
        self.pid0_reg().read()
    }

    #[doc="Get the PID1 Register."]
    #[inline] pub fn pid1_reg(&self) -> ::bobbin_mcu::register::Register<Pid1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pid1, 0x1fe4)
    }

    #[doc="Get the *mut pointer for the PID1 register."]
    #[inline] pub fn pid1_mut(&self) -> *mut Pid1 { 
        self.pid1_reg().ptr()
    }

    #[doc="Get the *const pointer for the PID1 register."]
    #[inline] pub fn pid1_ptr(&self) -> *const Pid1 { 
        self.pid1_reg().ptr()
    }

    #[doc="Read the PID1 register."]
    #[inline] pub fn pid1(&self) -> Pid1 { 
        self.pid1_reg().read()
    }

    #[doc="Get the PID2 Register."]
    #[inline] pub fn pid2_reg(&self) -> ::bobbin_mcu::register::Register<Pid2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pid2, 0x1fe8)
    }

    #[doc="Get the *mut pointer for the PID2 register."]
    #[inline] pub fn pid2_mut(&self) -> *mut Pid2 { 
        self.pid2_reg().ptr()
    }

    #[doc="Get the *const pointer for the PID2 register."]
    #[inline] pub fn pid2_ptr(&self) -> *const Pid2 { 
        self.pid2_reg().ptr()
    }

    #[doc="Read the PID2 register."]
    #[inline] pub fn pid2(&self) -> Pid2 { 
        self.pid2_reg().read()
    }

    #[doc="Get the PID3 Register."]
    #[inline] pub fn pid3_reg(&self) -> ::bobbin_mcu::register::Register<Pid3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pid3, 0x1fec)
    }

    #[doc="Get the *mut pointer for the PID3 register."]
    #[inline] pub fn pid3_mut(&self) -> *mut Pid3 { 
        self.pid3_reg().ptr()
    }

    #[doc="Get the *const pointer for the PID3 register."]
    #[inline] pub fn pid3_ptr(&self) -> *const Pid3 { 
        self.pid3_reg().ptr()
    }

    #[doc="Read the PID3 register."]
    #[inline] pub fn pid3(&self) -> Pid3 { 
        self.pid3_reg().read()
    }

    #[doc="Get the CID0 Register."]
    #[inline] pub fn cid0_reg(&self) -> ::bobbin_mcu::register::Register<Cid0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cid0, 0x1ff0)
    }

    #[doc="Get the *mut pointer for the CID0 register."]
    #[inline] pub fn cid0_mut(&self) -> *mut Cid0 { 
        self.cid0_reg().ptr()
    }

    #[doc="Get the *const pointer for the CID0 register."]
    #[inline] pub fn cid0_ptr(&self) -> *const Cid0 { 
        self.cid0_reg().ptr()
    }

    #[doc="Read the CID0 register."]
    #[inline] pub fn cid0(&self) -> Cid0 { 
        self.cid0_reg().read()
    }

    #[doc="Get the CID1 Register."]
    #[inline] pub fn cid1_reg(&self) -> ::bobbin_mcu::register::Register<Cid1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cid1, 0x1ff4)
    }

    #[doc="Get the *mut pointer for the CID1 register."]
    #[inline] pub fn cid1_mut(&self) -> *mut Cid1 { 
        self.cid1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CID1 register."]
    #[inline] pub fn cid1_ptr(&self) -> *const Cid1 { 
        self.cid1_reg().ptr()
    }

    #[doc="Read the CID1 register."]
    #[inline] pub fn cid1(&self) -> Cid1 { 
        self.cid1_reg().read()
    }

    #[doc="Get the CID2 Register."]
    #[inline] pub fn cid2_reg(&self) -> ::bobbin_mcu::register::Register<Cid2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cid2, 0x1ff8)
    }

    #[doc="Get the *mut pointer for the CID2 register."]
    #[inline] pub fn cid2_mut(&self) -> *mut Cid2 { 
        self.cid2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CID2 register."]
    #[inline] pub fn cid2_ptr(&self) -> *const Cid2 { 
        self.cid2_reg().ptr()
    }

    #[doc="Read the CID2 register."]
    #[inline] pub fn cid2(&self) -> Cid2 { 
        self.cid2_reg().read()
    }

    #[doc="Get the CID3 Register."]
    #[inline] pub fn cid3_reg(&self) -> ::bobbin_mcu::register::Register<Cid3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cid3, 0x1ffc)
    }

    #[doc="Get the *mut pointer for the CID3 register."]
    #[inline] pub fn cid3_mut(&self) -> *mut Cid3 { 
        self.cid3_reg().ptr()
    }

    #[doc="Get the *const pointer for the CID3 register."]
    #[inline] pub fn cid3_ptr(&self) -> *const Cid3 { 
        self.cid3_reg().ptr()
    }

    #[doc="Read the CID3 register."]
    #[inline] pub fn cid3(&self) -> Cid3 { 
        self.cid3_reg().read()
    }

}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
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

    #[doc="32-bit Cyclic Redundancy Code"]
    #[inline] pub fn crc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CRC != 0"]
    #[inline] pub fn test_crc(&self) -> bool {
        self.crc() != 0
    }

    #[doc="Sets the CRC field."]
    #[inline] pub fn set_crc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Memory built-in self-test"]
    #[inline] pub fn mbist(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MBIST != 0"]
    #[inline] pub fn test_mbist(&self) -> bool {
        self.mbist() != 0
    }

    #[doc="Sets the MBIST field."]
    #[inline] pub fn set_mbist<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Chip-Erase"]
    #[inline] pub fn ce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CE != 0"]
    #[inline] pub fn test_ce(&self) -> bool {
        self.ce() != 0
    }

    #[doc="Sets the CE field."]
    #[inline] pub fn set_ce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Auxiliary Row Read"]
    #[inline] pub fn arr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ARR != 0"]
    #[inline] pub fn test_arr(&self) -> bool {
        self.arr() != 0
    }

    #[doc="Sets the ARR field."]
    #[inline] pub fn set_arr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Start Memory Stream Access"]
    #[inline] pub fn smsa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SMSA != 0"]
    #[inline] pub fn test_smsa(&self) -> bool {
        self.smsa() != 0
    }

    #[doc="Sets the SMSA field."]
    #[inline] pub fn set_smsa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ctrl {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.crc() != 0 { try!(write!(f, " crc"))}
        if self.mbist() != 0 { try!(write!(f, " mbist"))}
        if self.ce() != 0 { try!(write!(f, " ce"))}
        if self.arr() != 0 { try!(write!(f, " arr"))}
        if self.smsa() != 0 { try!(write!(f, " smsa"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusa(pub u8);
impl Statusa {
    #[doc="Done"]
    #[inline] pub fn done(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DONE != 0"]
    #[inline] pub fn test_done(&self) -> bool {
        self.done() != 0
    }

    #[doc="Sets the DONE field."]
    #[inline] pub fn set_done<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CPU Reset Phase Extension"]
    #[inline] pub fn crstext(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CRSTEXT != 0"]
    #[inline] pub fn test_crstext(&self) -> bool {
        self.crstext() != 0
    }

    #[doc="Sets the CRSTEXT field."]
    #[inline] pub fn set_crstext<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Bus Error"]
    #[inline] pub fn berr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BERR != 0"]
    #[inline] pub fn test_berr(&self) -> bool {
        self.berr() != 0
    }

    #[doc="Sets the BERR field."]
    #[inline] pub fn set_berr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Failure"]
    #[inline] pub fn fail(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FAIL != 0"]
    #[inline] pub fn test_fail(&self) -> bool {
        self.fail() != 0
    }

    #[doc="Sets the FAIL field."]
    #[inline] pub fn set_fail<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Protection Error"]
    #[inline] pub fn perr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Statusa {
    #[inline]
    fn from(other: u8) -> Self {
         Statusa(other)
    }
}

impl ::core::fmt::Display for Statusa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Statusa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.done() != 0 { try!(write!(f, " done"))}
        if self.crstext() != 0 { try!(write!(f, " crstext"))}
        if self.berr() != 0 { try!(write!(f, " berr"))}
        if self.fail() != 0 { try!(write!(f, " fail"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Statusb(pub u8);
impl Statusb {
    #[doc="Protected"]
    #[inline] pub fn prot(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PROT != 0"]
    #[inline] pub fn test_prot(&self) -> bool {
        self.prot() != 0
    }

    #[doc="Sets the PROT field."]
    #[inline] pub fn set_prot<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Debugger Present"]
    #[inline] pub fn dbgpres(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DBGPRES != 0"]
    #[inline] pub fn test_dbgpres(&self) -> bool {
        self.dbgpres() != 0
    }

    #[doc="Sets the DBGPRES field."]
    #[inline] pub fn set_dbgpres<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Debug Communication Channel 0 Dirty"]
    #[inline] pub fn dccd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCCD0 != 0"]
    #[inline] pub fn test_dccd0(&self) -> bool {
        self.dccd0() != 0
    }

    #[doc="Sets the DCCD0 field."]
    #[inline] pub fn set_dccd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Debug Communication Channel 1 Dirty"]
    #[inline] pub fn dccd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DCCD1 != 0"]
    #[inline] pub fn test_dccd1(&self) -> bool {
        self.dccd1() != 0
    }

    #[doc="Sets the DCCD1 field."]
    #[inline] pub fn set_dccd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Hot-Plugging Enable"]
    #[inline] pub fn hpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HPE != 0"]
    #[inline] pub fn test_hpe(&self) -> bool {
        self.hpe() != 0
    }

    #[doc="Sets the HPE field."]
    #[inline] pub fn set_hpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Chip Erase Locked"]
    #[inline] pub fn celck(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CELCK != 0"]
    #[inline] pub fn test_celck(&self) -> bool {
        self.celck() != 0
    }

    #[doc="Sets the CELCK field."]
    #[inline] pub fn set_celck<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Test Debug Communication Channel 0 Dirty"]
    #[inline] pub fn tdccd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TDCCD0 != 0"]
    #[inline] pub fn test_tdccd0(&self) -> bool {
        self.tdccd0() != 0
    }

    #[doc="Sets the TDCCD0 field."]
    #[inline] pub fn set_tdccd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Test Debug Communication Channel 1 Dirty"]
    #[inline] pub fn tdccd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TDCCD1 != 0"]
    #[inline] pub fn test_tdccd1(&self) -> bool {
        self.tdccd1() != 0
    }

    #[doc="Sets the TDCCD1 field."]
    #[inline] pub fn set_tdccd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Statusb {
    #[inline]
    fn from(other: u8) -> Self {
         Statusb(other)
    }
}

impl ::core::fmt::Display for Statusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Statusb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prot() != 0 { try!(write!(f, " prot"))}
        if self.dbgpres() != 0 { try!(write!(f, " dbgpres"))}
        if self.dccd0() != 0 { try!(write!(f, " dccd0"))}
        if self.dccd1() != 0 { try!(write!(f, " dccd1"))}
        if self.hpe() != 0 { try!(write!(f, " hpe"))}
        if self.celck() != 0 { try!(write!(f, " celck"))}
        if self.tdccd0() != 0 { try!(write!(f, " tdccd0"))}
        if self.tdccd1() != 0 { try!(write!(f, " tdccd1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
    #[doc="Access Mode"]
    #[inline] pub fn amod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if AMOD != 0"]
    #[inline] pub fn test_amod(&self) -> bool {
        self.amod() != 0
    }

    #[doc="Sets the AMOD field."]
    #[inline] pub fn set_amod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U30 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3fffffff) as u32) } // [31:2]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U30>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U30 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fffffff << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Addr {
    #[inline]
    fn from(other: u32) -> Self {
         Addr(other)
    }
}

impl ::core::fmt::Display for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.amod() != 0 { try!(write!(f, " amod=0x{:x}", self.amod()))}
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Length(pub u32);
impl Length {
    #[doc="Length"]
    #[inline] pub fn length(&self) -> ::bobbin_bits::U30 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3fffffff) as u32) } // [31:2]
    }

    #[doc="Returns true if LENGTH != 0"]
    #[inline] pub fn test_length(&self) -> bool {
        self.length() != 0
    }

    #[doc="Sets the LENGTH field."]
    #[inline] pub fn set_length<V: Into<::bobbin_bits::U30>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U30 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fffffff << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Length {
    #[inline]
    fn from(other: u32) -> Self {
         Length(other)
    }
}

impl ::core::fmt::Display for Length {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Length {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.length() != 0 { try!(write!(f, " length=0x{:x}", self.length()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Data {
    #[inline]
    fn from(other: u32) -> Self {
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Communication Channel n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcc(pub u32);
impl Dcc {
    #[doc="Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcc {
    #[inline]
    fn from(other: u32) -> Self {
         Dcc(other)
    }
}

impl ::core::fmt::Display for Dcc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Device Identification"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Did(pub u32);
impl Did {
    #[doc="Device Select"]
    #[inline] pub fn devsel(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DEVSEL != 0"]
    #[inline] pub fn test_devsel(&self) -> bool {
        self.devsel() != 0
    }

    #[doc="Sets the DEVSEL field."]
    #[inline] pub fn set_devsel<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Revision Number"]
    #[inline] pub fn revision(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if REVISION != 0"]
    #[inline] pub fn test_revision(&self) -> bool {
        self.revision() != 0
    }

    #[doc="Sets the REVISION field."]
    #[inline] pub fn set_revision<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Die Number"]
    #[inline] pub fn die(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if DIE != 0"]
    #[inline] pub fn test_die(&self) -> bool {
        self.die() != 0
    }

    #[doc="Sets the DIE field."]
    #[inline] pub fn set_die<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Series"]
    #[inline] pub fn series(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if SERIES != 0"]
    #[inline] pub fn test_series(&self) -> bool {
        self.series() != 0
    }

    #[doc="Sets the SERIES field."]
    #[inline] pub fn set_series<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Family"]
    #[inline] pub fn family(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1f) as u8) } // [27:23]
    }

    #[doc="Returns true if FAMILY != 0"]
    #[inline] pub fn test_family(&self) -> bool {
        self.family() != 0
    }

    #[doc="Sets the FAMILY field."]
    #[inline] pub fn set_family<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Processor"]
    #[inline] pub fn processor(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if PROCESSOR != 0"]
    #[inline] pub fn test_processor(&self) -> bool {
        self.processor() != 0
    }

    #[doc="Sets the PROCESSOR field."]
    #[inline] pub fn set_processor<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Did {
    #[inline]
    fn from(other: u32) -> Self {
         Did(other)
    }
}

impl ::core::fmt::Display for Did {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Did {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.devsel() != 0 { try!(write!(f, " devsel=0x{:x}", self.devsel()))}
        if self.revision() != 0 { try!(write!(f, " revision=0x{:x}", self.revision()))}
        if self.die() != 0 { try!(write!(f, " die=0x{:x}", self.die()))}
        if self.series() != 0 { try!(write!(f, " series=0x{:x}", self.series()))}
        if self.family() != 0 { try!(write!(f, " family=0x{:x}", self.family()))}
        if self.processor() != 0 { try!(write!(f, " processor=0x{:x}", self.processor()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="Latency Quality Of Service"]
    #[inline] pub fn lqos(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if LQOS != 0"]
    #[inline] pub fn test_lqos(&self) -> bool {
        self.lqos() != 0
    }

    #[doc="Sets the LQOS field."]
    #[inline] pub fn set_lqos<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA Trigger Level"]
    #[inline] pub fn dccdmalevel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if DCCDMALEVEL != 0"]
    #[inline] pub fn test_dccdmalevel(&self) -> bool {
        self.dccdmalevel() != 0
    }

    #[doc="Sets the DCCDMALEVEL field."]
    #[inline] pub fn set_dccdmalevel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Trace Control"]
    #[inline] pub fn etbramen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ETBRAMEN != 0"]
    #[inline] pub fn test_etbramen(&self) -> bool {
        self.etbramen() != 0
    }

    #[doc="Sets the ETBRAMEN field."]
    #[inline] pub fn set_etbramen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg(other)
    }
}

impl ::core::fmt::Display for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lqos() != 0 { try!(write!(f, " lqos=0x{:x}", self.lqos()))}
        if self.dccdmalevel() != 0 { try!(write!(f, " dccdmalevel=0x{:x}", self.dccdmalevel()))}
        if self.etbramen() != 0 { try!(write!(f, " etbramen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Device Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcfg(pub u32);
impl Dcfg {
    #[doc="Device Configuration"]
    #[inline] pub fn dcfg(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DCFG != 0"]
    #[inline] pub fn test_dcfg(&self) -> bool {
        self.dcfg() != 0
    }

    #[doc="Sets the DCFG field."]
    #[inline] pub fn set_dcfg<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Dcfg(other)
    }
}

impl ::core::fmt::Display for Dcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CoreSight ROM Table Entry 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Entry0(pub u32);
impl Entry0 {
    #[doc="Entry Present"]
    #[inline] pub fn epres(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EPRES != 0"]
    #[inline] pub fn test_epres(&self) -> bool {
        self.epres() != 0
    }

    #[doc="Sets the EPRES field."]
    #[inline] pub fn set_epres<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Format"]
    #[inline] pub fn fmt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FMT != 0"]
    #[inline] pub fn test_fmt(&self) -> bool {
        self.fmt() != 0
    }

    #[doc="Sets the FMT field."]
    #[inline] pub fn set_fmt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Address Offset"]
    #[inline] pub fn addoff(&self) -> ::bobbin_bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xfffff) as u32) } // [31:12]
    }

    #[doc="Returns true if ADDOFF != 0"]
    #[inline] pub fn test_addoff(&self) -> bool {
        self.addoff() != 0
    }

    #[doc="Sets the ADDOFF field."]
    #[inline] pub fn set_addoff<V: Into<::bobbin_bits::U20>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Entry0 {
    #[inline]
    fn from(other: u32) -> Self {
         Entry0(other)
    }
}

impl ::core::fmt::Display for Entry0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Entry0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epres() != 0 { try!(write!(f, " epres"))}
        if self.fmt() != 0 { try!(write!(f, " fmt"))}
        if self.addoff() != 0 { try!(write!(f, " addoff=0x{:x}", self.addoff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CoreSight ROM Table Entry 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Entry1(pub u32);
impl Entry1 {
}

impl From<u32> for Entry1 {
    #[inline]
    fn from(other: u32) -> Self {
         Entry1(other)
    }
}

impl ::core::fmt::Display for Entry1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Entry1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CoreSight ROM Table End"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct End(pub u32);
impl End {
    #[doc="End Marker"]
    #[inline] pub fn end(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if END != 0"]
    #[inline] pub fn test_end(&self) -> bool {
        self.end() != 0
    }

    #[doc="Sets the END field."]
    #[inline] pub fn set_end<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for End {
    #[inline]
    fn from(other: u32) -> Self {
         End(other)
    }
}

impl ::core::fmt::Display for End {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for End {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CoreSight ROM Table Memory Type"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Memtype(pub u32);
impl Memtype {
    #[doc="System Memory Present"]
    #[inline] pub fn smemp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SMEMP != 0"]
    #[inline] pub fn test_smemp(&self) -> bool {
        self.smemp() != 0
    }

    #[doc="Sets the SMEMP field."]
    #[inline] pub fn set_smemp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Memtype {
    #[inline]
    fn from(other: u32) -> Self {
         Memtype(other)
    }
}

impl ::core::fmt::Display for Memtype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Memtype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.smemp() != 0 { try!(write!(f, " smemp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Identification 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pid4(pub u32);
impl Pid4 {
    #[doc="JEP-106 Continuation Code"]
    #[inline] pub fn jepcc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if JEPCC != 0"]
    #[inline] pub fn test_jepcc(&self) -> bool {
        self.jepcc() != 0
    }

    #[doc="Sets the JEPCC field."]
    #[inline] pub fn set_jepcc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="4KB count"]
    #[inline] pub fn fkbc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if FKBC != 0"]
    #[inline] pub fn test_fkbc(&self) -> bool {
        self.fkbc() != 0
    }

    #[doc="Sets the FKBC field."]
    #[inline] pub fn set_fkbc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Pid4 {
    #[inline]
    fn from(other: u32) -> Self {
         Pid4(other)
    }
}

impl ::core::fmt::Display for Pid4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pid4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jepcc() != 0 { try!(write!(f, " jepcc=0x{:x}", self.jepcc()))}
        if self.fkbc() != 0 { try!(write!(f, " fkbc=0x{:x}", self.fkbc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Identification 5"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pid5(pub u32);
impl Pid5 {
}

impl From<u32> for Pid5 {
    #[inline]
    fn from(other: u32) -> Self {
         Pid5(other)
    }
}

impl ::core::fmt::Display for Pid5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pid5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Identification 6"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pid6(pub u32);
impl Pid6 {
}

impl From<u32> for Pid6 {
    #[inline]
    fn from(other: u32) -> Self {
         Pid6(other)
    }
}

impl ::core::fmt::Display for Pid6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pid6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Identification 7"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pid7(pub u32);
impl Pid7 {
}

impl From<u32> for Pid7 {
    #[inline]
    fn from(other: u32) -> Self {
         Pid7(other)
    }
}

impl ::core::fmt::Display for Pid7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pid7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Identification 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pid0(pub u32);
impl Pid0 {
    #[doc="Part Number Low"]
    #[inline] pub fn partnbl(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PARTNBL != 0"]
    #[inline] pub fn test_partnbl(&self) -> bool {
        self.partnbl() != 0
    }

    #[doc="Sets the PARTNBL field."]
    #[inline] pub fn set_partnbl<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pid0 {
    #[inline]
    fn from(other: u32) -> Self {
         Pid0(other)
    }
}

impl ::core::fmt::Display for Pid0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pid0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.partnbl() != 0 { try!(write!(f, " partnbl=0x{:x}", self.partnbl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Identification 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pid1(pub u32);
impl Pid1 {
    #[doc="Part Number High"]
    #[inline] pub fn partnbh(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PARTNBH != 0"]
    #[inline] pub fn test_partnbh(&self) -> bool {
        self.partnbh() != 0
    }

    #[doc="Sets the PARTNBH field."]
    #[inline] pub fn set_partnbh<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low part of the JEP-106 Identity Code"]
    #[inline] pub fn jepidcl(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if JEPIDCL != 0"]
    #[inline] pub fn test_jepidcl(&self) -> bool {
        self.jepidcl() != 0
    }

    #[doc="Sets the JEPIDCL field."]
    #[inline] pub fn set_jepidcl<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Pid1 {
    #[inline]
    fn from(other: u32) -> Self {
         Pid1(other)
    }
}

impl ::core::fmt::Display for Pid1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pid1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.partnbh() != 0 { try!(write!(f, " partnbh=0x{:x}", self.partnbh()))}
        if self.jepidcl() != 0 { try!(write!(f, " jepidcl=0x{:x}", self.jepidcl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Identification 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pid2(pub u32);
impl Pid2 {
    #[doc="JEP-106 Identity Code High"]
    #[inline] pub fn jepidch(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if JEPIDCH != 0"]
    #[inline] pub fn test_jepidch(&self) -> bool {
        self.jepidch() != 0
    }

    #[doc="Sets the JEPIDCH field."]
    #[inline] pub fn set_jepidch<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="JEP-106 Identity Code is used"]
    #[inline] pub fn jepu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JEPU != 0"]
    #[inline] pub fn test_jepu(&self) -> bool {
        self.jepu() != 0
    }

    #[doc="Sets the JEPU field."]
    #[inline] pub fn set_jepu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Revision Number"]
    #[inline] pub fn revision(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if REVISION != 0"]
    #[inline] pub fn test_revision(&self) -> bool {
        self.revision() != 0
    }

    #[doc="Sets the REVISION field."]
    #[inline] pub fn set_revision<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Pid2 {
    #[inline]
    fn from(other: u32) -> Self {
         Pid2(other)
    }
}

impl ::core::fmt::Display for Pid2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pid2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jepidch() != 0 { try!(write!(f, " jepidch=0x{:x}", self.jepidch()))}
        if self.jepu() != 0 { try!(write!(f, " jepu"))}
        if self.revision() != 0 { try!(write!(f, " revision=0x{:x}", self.revision()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Identification 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pid3(pub u32);
impl Pid3 {
    #[doc="ARM CUSMOD"]
    #[inline] pub fn cusmod(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CUSMOD != 0"]
    #[inline] pub fn test_cusmod(&self) -> bool {
        self.cusmod() != 0
    }

    #[doc="Sets the CUSMOD field."]
    #[inline] pub fn set_cusmod<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Revision Number"]
    #[inline] pub fn revand(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if REVAND != 0"]
    #[inline] pub fn test_revand(&self) -> bool {
        self.revand() != 0
    }

    #[doc="Sets the REVAND field."]
    #[inline] pub fn set_revand<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Pid3 {
    #[inline]
    fn from(other: u32) -> Self {
         Pid3(other)
    }
}

impl ::core::fmt::Display for Pid3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pid3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cusmod() != 0 { try!(write!(f, " cusmod=0x{:x}", self.cusmod()))}
        if self.revand() != 0 { try!(write!(f, " revand=0x{:x}", self.revand()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Component Identification 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cid0(pub u32);
impl Cid0 {
    #[doc="Preamble Byte 0"]
    #[inline] pub fn preambleb0(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PREAMBLEB0 != 0"]
    #[inline] pub fn test_preambleb0(&self) -> bool {
        self.preambleb0() != 0
    }

    #[doc="Sets the PREAMBLEB0 field."]
    #[inline] pub fn set_preambleb0<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cid0 {
    #[inline]
    fn from(other: u32) -> Self {
         Cid0(other)
    }
}

impl ::core::fmt::Display for Cid0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cid0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.preambleb0() != 0 { try!(write!(f, " preambleb0=0x{:x}", self.preambleb0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Component Identification 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cid1(pub u32);
impl Cid1 {
    #[doc="Preamble"]
    #[inline] pub fn preamble(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PREAMBLE != 0"]
    #[inline] pub fn test_preamble(&self) -> bool {
        self.preamble() != 0
    }

    #[doc="Sets the PREAMBLE field."]
    #[inline] pub fn set_preamble<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Component Class"]
    #[inline] pub fn cclass(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if CCLASS != 0"]
    #[inline] pub fn test_cclass(&self) -> bool {
        self.cclass() != 0
    }

    #[doc="Sets the CCLASS field."]
    #[inline] pub fn set_cclass<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Cid1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cid1(other)
    }
}

impl ::core::fmt::Display for Cid1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cid1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.preamble() != 0 { try!(write!(f, " preamble=0x{:x}", self.preamble()))}
        if self.cclass() != 0 { try!(write!(f, " cclass=0x{:x}", self.cclass()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Component Identification 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cid2(pub u32);
impl Cid2 {
    #[doc="Preamble Byte 2"]
    #[inline] pub fn preambleb2(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PREAMBLEB2 != 0"]
    #[inline] pub fn test_preambleb2(&self) -> bool {
        self.preambleb2() != 0
    }

    #[doc="Sets the PREAMBLEB2 field."]
    #[inline] pub fn set_preambleb2<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cid2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cid2(other)
    }
}

impl ::core::fmt::Display for Cid2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cid2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.preambleb2() != 0 { try!(write!(f, " preambleb2=0x{:x}", self.preambleb2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Component Identification 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cid3(pub u32);
impl Cid3 {
    #[doc="Preamble Byte 3"]
    #[inline] pub fn preambleb3(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PREAMBLEB3 != 0"]
    #[inline] pub fn test_preambleb3(&self) -> bool {
        self.preambleb3() != 0
    }

    #[doc="Sets the PREAMBLEB3 field."]
    #[inline] pub fn set_preambleb3<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cid3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cid3(other)
    }
}

impl ::core::fmt::Display for Cid3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cid3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.preambleb3() != 0 { try!(write!(f, " preambleb3=0x{:x}", self.preambleb3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

